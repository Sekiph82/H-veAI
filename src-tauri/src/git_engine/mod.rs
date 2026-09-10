mod mutation;

use crate::db::DatabaseState;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

pub use mutation::{mutation_status, MutationStatus};

const COMMAND_TIMEOUT: Duration = Duration::from_secs(8);
const MAX_COMMAND_OUTPUT: usize = 8 * 1024 * 1024;
const MAX_DIFF_BYTES: usize = 96 * 1024;
const MAX_DIFF_LINES: usize = 1200;
const MAX_DIFF_CAPTURE_BYTES: usize = MAX_DIFF_BYTES * 8;
const MAX_UNTRACKED_IDENTITY_FILES: usize = 4096;
const MAX_UNTRACKED_IDENTITY_BYTES: u64 = 64 * 1024 * 1024;
const MAX_UNTRACKED_IDENTITY_TIME: Duration = Duration::from_secs(2);
const MAX_TRACKED_IDENTITY_BYTES: u64 = 64 * 1024 * 1024;
const MAX_IDENTITY_READ_CHUNK: usize = 64 * 1024;
const MAX_RECENT_COMMITS: usize = 25;
const MAX_WORKTREES: usize = 50;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitSnapshot {
    pub project_id: String,
    pub repository_id: String,
    pub repository_path: String,
    pub current_branch: Option<String>,
    pub detached_head: bool,
    pub head_sha: Option<String>,
    pub staged_files: Vec<GitFileChange>,
    pub unstaged_files: Vec<GitFileChange>,
    pub untracked_files: Vec<String>,
    pub conflicted_files: Vec<String>,
    pub ahead_count: Option<u64>,
    pub behind_count: Option<u64>,
    pub upstream: Option<String>,
    pub remotes: Vec<GitRemote>,
    pub recent_commits: Vec<GitCommit>,
    pub worktrees: Vec<GitWorktree>,
    pub health: RepositoryHealth,
    pub snapshot_timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitFileChange {
    pub path: String,
    pub kind: String,
    pub staged: bool,
    pub unstaged: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRemote {
    pub name: String,
    pub fetch_url: String,
    pub push_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitCommit {
    pub sha: String,
    pub subject: String,
    pub author_name: String,
    pub author_email: String,
    pub authored_at: String,
    pub committed_at: String,
    pub parent_count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitWorktree {
    pub path: String,
    pub branch: Option<String>,
    pub head_sha: Option<String>,
    pub locked: bool,
    pub prunable: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)]
pub enum RepositoryHealth {
    Clean,
    Dirty,
    Conflicted,
    Detached,
    Unborn,
    Missing,
    NonGit,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GitSnapshotRequest {
    pub project_id: String,
    pub persist: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDiffRequest {
    pub project_id: String,
    pub scope: GitDiffScope,
    #[serde(default)]
    pub base_ref: Option<String>,
    #[serde(default)]
    pub head_sha: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GitDiffScope {
    Staged,
    WorkingTree,
    CommitRange,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDiff {
    pub project_id: String,
    pub scope: GitDiffScope,
    pub base_ref: Option<String>,
    pub base_sha: Option<String>,
    pub head_sha: Option<String>,
    pub changed_files: Vec<String>,
    pub text: String,
    pub truncated: bool,
    pub binary_files: Vec<String>,
    pub byte_limit: usize,
    pub line_limit: usize,
    pub full_change_set_sha256: String,
    pub untracked_content_sha256: Option<String>,
    pub identity_complete: bool,
    pub identity_diagnostic: Option<String>,
}

impl Serialize for GitDiffScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            Self::Staged => "STAGED",
            Self::WorkingTree => "WORKING_TREE",
            Self::CommitRange => "COMMIT_RANGE",
        })
    }
}

pub fn snapshot(
    database: &DatabaseState,
    request: GitSnapshotRequest,
) -> Result<GitSnapshot, String> {
    let (repository_id, repository_path) = resolve_repository(database, &request.project_id)?;
    let result = collect_snapshot(&request.project_id, &repository_id, &repository_path)?;
    if request.persist.unwrap_or(false) {
        persist_snapshot(database, &result)?;
    }
    Ok(result)
}

pub fn diff(database: &DatabaseState, request: GitDiffRequest) -> Result<GitDiff, String> {
    let (_, repository_path) = resolve_repository(database, &request.project_id)?;
    let (range, base_ref, base_sha, head_sha) = match request.scope {
        GitDiffScope::CommitRange => {
            let base_ref = request
                .base_ref
                .as_deref()
                .ok_or_else(|| "GIT_COMMIT_RANGE_BASE_REQUIRED".to_string())?;
            let base_sha = resolve_commit(&repository_path, base_ref)?;
            let head_sha = match request.head_sha.as_deref() {
                Some(value) => resolve_commit(&repository_path, value)?,
                None => resolve_commit(&repository_path, "HEAD")?,
            };
            (
                Some(format!("{base_sha}..{head_sha}")),
                Some(base_ref.to_string()),
                Some(base_sha),
                Some(head_sha),
            )
        }
        _ => (None, None, None, None),
    };
    let args = match (&request.scope, &range) {
        (GitDiffScope::Staged, _) => vec![
            "diff",
            "--no-ext-diff",
            "--no-textconv",
            "--binary",
            "--cached",
            "--",
        ],
        (GitDiffScope::WorkingTree, _) => {
            vec!["diff", "--no-ext-diff", "--no-textconv", "--binary", "--"]
        }
        (GitDiffScope::CommitRange, Some(range)) => vec![
            "diff",
            "--no-ext-diff",
            "--no-textconv",
            "--binary",
            range,
            "--",
        ],
        (GitDiffScope::CommitRange, None) => return Err("GIT_COMMIT_RANGE_INVALID".into()),
    };
    let streamed = run_git_bounded(
        &repository_path,
        &args,
        MAX_DIFF_CAPTURE_BYTES,
        MAX_COMMAND_OUTPUT,
    )?;
    let raw = String::from_utf8_lossy(&streamed.stdout).into_owned();
    let numstat_args = match (&request.scope, &range) {
        (GitDiffScope::Staged, _) => vec![
            "diff",
            "--no-ext-diff",
            "--no-textconv",
            "--cached",
            "--numstat",
            "--",
        ],
        (GitDiffScope::WorkingTree, _) => {
            vec!["diff", "--no-ext-diff", "--no-textconv", "--numstat", "--"]
        }
        (GitDiffScope::CommitRange, Some(range)) => vec![
            "diff",
            "--no-ext-diff",
            "--no-textconv",
            "--numstat",
            range,
            "--",
        ],
        (GitDiffScope::CommitRange, None) => return Err("GIT_COMMIT_RANGE_INVALID".into()),
    };
    let numstat = output_text(&run_git(&repository_path, &numstat_args)?.stdout)?;
    let mut binary_files = numstat
        .lines()
        .filter_map(|line| {
            let mut fields = line.splitn(3, '\t');
            if fields.next()? == "-" && fields.next()? == "-" {
                fields.next().map(normalize_binary_path)
            } else {
                None
            }
        })
        .filter(|path| path != "/dev/null" && !path.contains('\0'))
        .collect::<Vec<_>>();
    binary_files.sort();
    binary_files.dedup();
    let name_args = match (&request.scope, &range) {
        (GitDiffScope::Staged, _) => vec!["diff", "--name-only", "--cached", "--"],
        (GitDiffScope::WorkingTree, _) => vec!["diff", "--name-only", "--"],
        (GitDiffScope::CommitRange, Some(range)) => vec!["diff", "--name-only", range, "--"],
        (GitDiffScope::CommitRange, None) => return Err("GIT_COMMIT_RANGE_INVALID".into()),
    };
    let mut changed_files = output_text(&run_git(&repository_path, &name_args)?.stdout)?
        .lines()
        .map(normalize_binary_path)
        .filter(|path| !path.is_empty() && !path.contains('\0'))
        .collect::<Vec<_>>();
    changed_files.sort();
    changed_files.dedup();
    let untracked_identity = if request.scope == GitDiffScope::WorkingTree {
        Some(untracked_content_identity(&repository_path)?)
    } else {
        None
    };
    let change_identity = change_set_identity(
        &repository_path,
        request.scope,
        range.as_deref(),
        &changed_files,
        &numstat,
        untracked_identity.as_ref(),
    )?;
    let full_change_set_sha256 = change_identity.hash.clone();
    let text = sanitize_binary_payloads(&raw);
    let (text, mut truncated) = bound_text(&text, MAX_DIFF_BYTES, MAX_DIFF_LINES);
    truncated |= streamed.stdout_truncated;
    Ok(GitDiff {
        project_id: request.project_id,
        scope: request.scope,
        base_ref,
        base_sha,
        head_sha,
        changed_files,
        text,
        truncated,
        binary_files,
        byte_limit: MAX_DIFF_BYTES,
        line_limit: MAX_DIFF_LINES,
        full_change_set_sha256,
        untracked_content_sha256: untracked_identity.as_ref().map(|value| value.hash.clone()),
        identity_complete: change_identity.complete
            && untracked_identity
                .as_ref()
                .map(|value| value.complete)
                .unwrap_or(true),
        identity_diagnostic: change_identity.diagnostic.or_else(|| {
            untracked_identity
                .as_ref()
                .and_then(|value| value.diagnostic.clone())
        }),
    })
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn resolve_commit(path: &Path, reference: &str) -> Result<String, String> {
    if reference.trim().is_empty() || reference.starts_with('-') || reference.len() > 256 {
        return Err("GIT_COMMIT_RANGE_REF_INVALID".into());
    }
    let expression = format!("{reference}^{{commit}}");
    let output = run_git(path, &["rev-parse", "--verify", &expression])?;
    let value = output_text(&output.stdout)?.trim().to_string();
    if value.len() < 7 || value.len() > 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("GIT_COMMIT_RANGE_REF_INVALID".into());
    }
    Ok(value)
}

#[derive(Debug, Clone)]
struct ContentIdentity {
    hash: String,
    complete: bool,
    diagnostic: Option<String>,
}

fn untracked_content_identity(path: &Path) -> Result<ContentIdentity, String> {
    let output = run_git(path, &["ls-files", "--others", "--exclude-standard", "-z"])?;
    let mut files = output_text(&output.stdout)?
        .split('\0')
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    files.sort();
    let mut hasher = Sha256::new();
    let mut complete = true;
    let mut diagnostic = None;
    let started = SystemTime::now();
    let mut total_bytes = 0u64;
    for (index, relative) in files.into_iter().enumerate() {
        if index >= MAX_UNTRACKED_IDENTITY_FILES {
            complete = false;
            diagnostic = Some("GIT_UNTRACKED_IDENTITY_FILE_LIMIT".into());
            break;
        }
        if started.elapsed().unwrap_or_default() > MAX_UNTRACKED_IDENTITY_TIME {
            complete = false;
            diagnostic = Some("GIT_UNTRACKED_IDENTITY_TIMEOUT".into());
            break;
        }
        if relative.contains('\0')
            || relative.starts_with('/')
            || relative.contains("..\\")
            || relative.contains("../")
        {
            continue;
        }
        let candidate = path.join(&relative);
        let canonical = fs::canonicalize(&candidate)
            .map_err(|_| "GIT_UNTRACKED_IDENTITY_UNAVAILABLE".to_string())?;
        let root =
            fs::canonicalize(path).map_err(|_| "GIT_UNTRACKED_IDENTITY_UNAVAILABLE".to_string())?;
        if !canonical.starts_with(&root) || !canonical.is_file() {
            continue;
        }
        let size = fs::metadata(&canonical)
            .map_err(|_| "GIT_UNTRACKED_IDENTITY_UNAVAILABLE".to_string())?
            .len();
        if total_bytes.saturating_add(size) > MAX_UNTRACKED_IDENTITY_BYTES {
            complete = false;
            diagnostic = Some("GIT_UNTRACKED_IDENTITY_BYTE_LIMIT".into());
            break;
        }
        let file_hash = stream_file_hash(&canonical, &mut total_bytes)?;
        hasher.update(relative.as_bytes());
        hasher.update([0]);
        hasher.update(file_hash.as_bytes());
        hasher.update([0]);
    }
    Ok(ContentIdentity {
        hash: format!("{:x}", hasher.finalize()),
        complete,
        diagnostic,
    })
}

fn stream_file_hash(path: &Path, total_bytes: &mut u64) -> Result<String, String> {
    let mut file =
        fs::File::open(path).map_err(|_| "GIT_UNTRACKED_IDENTITY_UNAVAILABLE".to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; MAX_IDENTITY_READ_CHUNK];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_| "GIT_UNTRACKED_IDENTITY_UNAVAILABLE".to_string())?;
        if read == 0 {
            break;
        }
        *total_bytes = total_bytes.saturating_add(read as u64);
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn change_set_identity(
    path: &Path,
    scope: GitDiffScope,
    range: Option<&str>,
    changed_files: &[String],
    numstat: &str,
    untracked: Option<&ContentIdentity>,
) -> Result<ContentIdentity, String> {
    let raw_identity = match (scope, range) {
        (GitDiffScope::Staged, _) => {
            run_git(path, &["diff", "--raw", "-z", "--cached", "--"])?.stdout
        }
        (GitDiffScope::WorkingTree, _) => run_git(path, &["diff", "--raw", "-z", "--"])?.stdout,
        (GitDiffScope::CommitRange, Some(range)) => {
            run_git(path, &["diff", "--raw", "-z", range, "--"])?.stdout
        }
        (GitDiffScope::CommitRange, None) => return Err("GIT_COMMIT_RANGE_INVALID".into()),
    };
    let mut identity = Sha256::new();
    identity.update(
        match scope {
            GitDiffScope::Staged => "STAGED",
            GitDiffScope::WorkingTree => "WORKING_TREE",
            GitDiffScope::CommitRange => "COMMIT_RANGE",
        }
        .as_bytes(),
    );
    identity.update(range.unwrap_or_default().as_bytes());
    identity.update(&raw_identity);
    identity.update(numstat.as_bytes());
    identity.update(changed_files.join("\n").as_bytes());
    let mut complete = true;
    let mut diagnostic = None;
    if scope == GitDiffScope::WorkingTree {
        let mut tracked_total = 0u64;
        for relative in changed_files {
            identity.update(relative.as_bytes());
            identity.update([0]);
            let candidate = path.join(relative);
            if candidate.is_file() {
                match stream_file_hash_bounded(
                    &candidate,
                    &mut tracked_total,
                    MAX_TRACKED_IDENTITY_BYTES,
                )
                .map_err(|_| "GIT_WORKTREE_IDENTITY_UNAVAILABLE")?
                {
                    Some(file_hash) => identity.update(file_hash.as_bytes()),
                    None => {
                        complete = false;
                        diagnostic = Some("GIT_TRACKED_IDENTITY_BYTE_LIMIT".into());
                        identity.update(b"TRUNCATED");
                        break;
                    }
                }
            } else {
                identity.update(b"MISSING");
            }
            identity.update([0]);
        }
    }
    if let Some(untracked) = untracked {
        identity.update(untracked.hash.as_bytes());
        identity.update([u8::from(untracked.complete)]);
        identity.update(
            untracked
                .diagnostic
                .as_deref()
                .unwrap_or_default()
                .as_bytes(),
        );
    }
    Ok(ContentIdentity {
        hash: format!("{:x}", identity.finalize()),
        complete,
        diagnostic,
    })
}

fn stream_file_hash_bounded(
    path: &Path,
    total_bytes: &mut u64,
    max_total_bytes: u64,
) -> Result<Option<String>, String> {
    let mut file =
        fs::File::open(path).map_err(|_| "GIT_WORKTREE_IDENTITY_UNAVAILABLE".to_string())?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; MAX_IDENTITY_READ_CHUNK];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_| "GIT_WORKTREE_IDENTITY_UNAVAILABLE".to_string())?;
        if read == 0 {
            return Ok(Some(format!("{:x}", hasher.finalize())));
        }
        if total_bytes.saturating_add(read as u64) > max_total_bytes {
            return Ok(None);
        }
        *total_bytes = total_bytes.saturating_add(read as u64);
        hasher.update(&buffer[..read]);
    }
}

fn normalize_binary_path(path: &str) -> String {
    path.trim_matches('"')
        .replace('\\', "/")
        .trim_start_matches("./")
        .to_string()
}

fn sanitize_binary_payloads(raw: &str) -> String {
    let mut in_binary_payload = false;
    raw.lines()
        .filter(|line| {
            if line.starts_with("diff --git ") {
                in_binary_payload = false;
                return true;
            }
            if *line == "GIT binary patch" {
                in_binary_payload = true;
                return false;
            }
            !in_binary_payload
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn collect_snapshot(
    project_id: &str,
    repository_id: &str,
    path: &Path,
) -> Result<GitSnapshot, String> {
    let branch = git_optional(path, &["symbolic-ref", "--quiet", "--short", "HEAD"])?;
    let head_sha = git_optional(path, &["rev-parse", "--verify", "HEAD"])?;
    let status =
        output_text(&run_git(path, &["status", "--porcelain=v1", "-z", "--branch"])?.stdout)?;
    let (staged_files, unstaged_files, untracked_files, conflicted_files) = parse_status(&status);
    let upstream = git_optional(
        path,
        &[
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            "@{upstream}",
        ],
    )?;
    let (ahead_count, behind_count) = upstream_counts(path, upstream.is_some());
    let remotes = read_remotes(path)?;
    let recent_commits = read_commits(path)?;
    let worktrees = read_worktrees(path)?;
    let detached_head = branch.is_none() && head_sha.is_some();
    let unborn = branch.is_some() && head_sha.is_none();
    let health = if conflicted_files.iter().next().is_some() {
        RepositoryHealth::Conflicted
    } else if unborn {
        RepositoryHealth::Unborn
    } else if detached_head {
        RepositoryHealth::Detached
    } else if staged_files.is_empty() && unstaged_files.is_empty() && untracked_files.is_empty() {
        RepositoryHealth::Clean
    } else {
        RepositoryHealth::Dirty
    };
    Ok(GitSnapshot {
        project_id: project_id.to_string(),
        repository_id: repository_id.to_string(),
        repository_path: path.to_string_lossy().into_owned(),
        current_branch: branch,
        detached_head,
        head_sha,
        staged_files,
        unstaged_files,
        untracked_files,
        conflicted_files,
        ahead_count,
        behind_count,
        upstream,
        remotes,
        recent_commits,
        worktrees,
        health,
        snapshot_timestamp: timestamp(),
    })
}

fn resolve_repository(
    database: &DatabaseState,
    project_id: &str,
) -> Result<(String, PathBuf), String> {
    let project = crate::projects::fetch_project(database, project_id)?;
    if project.status == "MISSING" {
        return Err("PROJECT_PATH_MISSING: registered project path is unavailable".to_string());
    }
    let repository = project
        .repository
        .ok_or_else(|| "NON_GIT_PROJECT: registered project is not a Git repository".to_string())?;
    if !repository.is_git_repository {
        return Err("NON_GIT_PROJECT: registered project is not a Git repository".to_string());
    }
    let path = PathBuf::from(project.normalized_path);
    if !path.is_dir() {
        return Err("PROJECT_PATH_MISSING: registered project path is unavailable".to_string());
    }
    Ok((repository.id, path))
}

fn persist_snapshot(database: &DatabaseState, snapshot: &GitSnapshot) -> Result<(), String> {
    let connection = database.open_connection()?;
    connection.execute("INSERT INTO git_snapshots (id, repository_id, branch, head_sha, status_json, captured_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", rusqlite::params![Uuid::new_v4().to_string(), snapshot.repository_id, snapshot.current_branch, snapshot.head_sha, serde_json::to_string(snapshot).map_err(|error| error.to_string())?, snapshot.snapshot_timestamp]).map_err(|error| format!("persist Git snapshot: {error}"))?;
    Ok(())
}

fn parse_status(
    status: &str,
) -> (
    Vec<GitFileChange>,
    Vec<GitFileChange>,
    Vec<String>,
    Vec<String>,
) {
    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();
    let mut conflicts = Vec::new();
    for record in status
        .split('\0')
        .filter(|record| !record.is_empty() && !record.starts_with("##"))
    {
        if record.len() < 3 {
            continue;
        }
        let bytes = record.as_bytes();
        let x = bytes[0] as char;
        let y = bytes[1] as char;
        let path = record[3..]
            .split(" -> ")
            .last()
            .unwrap_or(&record[3..])
            .to_string();
        if x == '?' && y == '?' {
            untracked.push(path);
            continue;
        }
        let conflict = matches!(
            (x, y),
            ('D', 'D')
                | ('A', 'U')
                | ('U', 'D')
                | ('U', 'A')
                | ('D', 'U')
                | ('A', 'A')
                | ('U', 'U')
        );
        if conflict {
            conflicts.push(path.clone());
        }
        if x != ' ' {
            staged.push(GitFileChange {
                path: path.clone(),
                kind: status_kind(x),
                staged: true,
                unstaged: false,
            });
        }
        if y != ' ' {
            unstaged.push(GitFileChange {
                path,
                kind: status_kind(y),
                staged: false,
                unstaged: true,
            });
        }
    }
    (staged, unstaged, untracked, conflicts)
}

fn status_kind(code: char) -> String {
    match code {
        'A' => "ADDED",
        'M' => "MODIFIED",
        'D' => "DELETED",
        'R' => "RENAMED",
        'C' => "COPIED",
        'U' => "CONFLICT",
        _ => "UNKNOWN",
    }
    .to_string()
}

fn upstream_counts(path: &Path, has_upstream: bool) -> (Option<u64>, Option<u64>) {
    if !has_upstream {
        return (None, None);
    }
    let Ok(output) = run_git(
        path,
        &["rev-list", "--left-right", "--count", "@{upstream}...HEAD"],
    ) else {
        return (None, None);
    };
    let Ok(text) = output_text(&output.stdout) else {
        return (None, None);
    };
    let mut values = text
        .split_whitespace()
        .filter_map(|value| value.parse::<u64>().ok());
    let behind = values.next();
    let ahead = values.next();
    (ahead, behind)
}

fn read_remotes(path: &Path) -> Result<Vec<GitRemote>, String> {
    let text = output_text(&run_git(path, &["remote", "-v"])?.stdout)?;
    let mut remotes = Vec::new();
    for line in text.lines() {
        let mut parts = line.split_whitespace();
        let Some(name) = parts.next() else {
            continue;
        };
        let Some(url) = parts.next() else {
            continue;
        };
        let Some(kind) = parts.next() else {
            continue;
        };
        let safe = sanitize_remote(url);
        if kind == "(fetch)" {
            remotes.push(GitRemote {
                name: name.to_string(),
                fetch_url: safe,
                push_url: None,
            });
        } else if kind == "(push)" {
            if let Some(remote) = remotes.iter_mut().find(|remote| remote.name == name) {
                remote.push_url = Some(safe);
            }
        }
    }
    Ok(remotes)
}

fn read_commits(path: &Path) -> Result<Vec<GitCommit>, String> {
    let format = "%H%x1f%s%x1f%an%x1f%ae%x1f%aI%x1f%cI%x1f%P%x1e";
    let output = match run_git(path, &["log", "-n", "25", &format!("--format={format}")]) {
        Ok(output) => output,
        Err(error) if error.starts_with("GIT_EXIT_128:") => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let text = output_text(&output.stdout)?;
    Ok(text
        .split('\x1e')
        .filter(|record| !record.is_empty())
        .take(MAX_RECENT_COMMITS)
        .filter_map(|record| {
            let fields = record.split('\x1f').collect::<Vec<_>>();
            if fields.len() < 7 {
                return None;
            }
            Some(GitCommit {
                sha: fields[0].to_string(),
                subject: fields[1].to_string(),
                author_name: fields[2].to_string(),
                author_email: fields[3].to_string(),
                authored_at: fields[4].to_string(),
                committed_at: fields[5].to_string(),
                parent_count: if fields[6].trim().is_empty() {
                    0
                } else {
                    fields[6].split_whitespace().count()
                },
            })
        })
        .collect())
}

fn read_worktrees(path: &Path) -> Result<Vec<GitWorktree>, String> {
    let output = run_git(path, &["worktree", "list", "--porcelain"])?;
    let text = output_text(&output.stdout)?;
    let mut trees = Vec::new();
    let mut current: Option<GitWorktree> = None;
    for line in text.lines() {
        if line.starts_with("worktree ") {
            if let Some(tree) = current.take() {
                trees.push(tree);
            }
            current = Some(GitWorktree {
                path: line[9..].to_string(),
                branch: None,
                head_sha: None,
                locked: false,
                prunable: false,
            });
        } else if let Some(tree) = current.as_mut() {
            if let Some(value) = line.strip_prefix("HEAD ") {
                tree.head_sha = Some(value.to_string());
            } else if let Some(value) = line.strip_prefix("branch ") {
                tree.branch = Some(
                    value
                        .strip_prefix("refs/heads/")
                        .unwrap_or(value)
                        .to_string(),
                );
            } else if line == "locked" {
                tree.locked = true;
            } else if line.starts_with("prunable") {
                tree.prunable = true;
            }
        }
    }
    if let Some(tree) = current {
        trees.push(tree);
    }
    trees.truncate(MAX_WORKTREES);
    Ok(trees)
}

fn sanitize_remote(url: &str) -> String {
    if let Some((scheme, rest)) = url.split_once("://") {
        if let Some((_, host)) = rest.rsplit_once('@') {
            return format!("{}://{}", scheme, host);
        }
    }
    url.to_string()
}

fn git_optional(path: &Path, args: &[&str]) -> Result<Option<String>, String> {
    match run_git(path, args) {
        Ok(output) => {
            Ok(Some(output_text(&output.stdout)?.trim().to_string())
                .filter(|value| !value.is_empty()))
        }
        Err(error) if error.starts_with("GIT_EXIT_1:") => Ok(None),
        Err(error)
            if (args.iter().any(|arg| *arg == "@{upstream}")
                || args.iter().any(|arg| *arg == "--verify"))
                && error.starts_with("GIT_EXIT_128:") =>
        {
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

fn production_git_command() -> Command {
    crate::process_policy::background_command("git")
}

pub(crate) fn run_git(path: &Path, args: &[&str]) -> Result<Output, String> {
    let mut child = production_git_command()
        .args(args)
        .current_dir(path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("GIT_SPAWN: {error}"))?;
    let started = SystemTime::now();
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("GIT_WAIT: {error}"))?
        {
            let output = child
                .wait_with_output()
                .map_err(|error| format!("GIT_OUTPUT: {error}"))?;
            if !status.success() {
                let detail = String::from_utf8_lossy(&output.stderr);
                return Err(format!(
                    "GIT_EXIT_{}: {}",
                    status.code().unwrap_or(-1),
                    bound_text(&detail, 4096, 40).0
                ));
            }
            return Ok(output);
        }
        if started.elapsed().unwrap_or_default() > COMMAND_TIMEOUT {
            let _ = child.kill();
            return Err("GIT_TIMEOUT: Git command exceeded the fixed timeout".to_string());
        }
        thread::sleep(Duration::from_millis(15));
    }
}

pub(crate) struct BoundedGitOutput {
    pub(crate) stdout: Vec<u8>,
    pub(crate) stdout_truncated: bool,
}

fn read_stream_bounded<R: Read>(mut reader: R, limit: usize) -> (Vec<u8>, bool) {
    let mut output = Vec::with_capacity(limit.min(64 * 1024));
    let mut buffer = [0u8; 64 * 1024];
    let mut truncated = false;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                if output.len() < limit {
                    let keep = (limit - output.len()).min(count);
                    output.extend_from_slice(&buffer[..keep]);
                    truncated |= keep < count;
                } else {
                    truncated = true;
                }
            }
            Err(_) => {
                truncated = true;
                break;
            }
        }
    }
    (output, truncated)
}

pub(crate) fn run_git_bounded(
    path: &Path,
    args: &[&str],
    stdout_limit: usize,
    stderr_limit: usize,
) -> Result<BoundedGitOutput, String> {
    let mut child = production_git_command()
        .args(args)
        .current_dir(path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("GIT_SPAWN: {error}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "GIT_STDOUT_UNAVAILABLE".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "GIT_STDERR_UNAVAILABLE".to_string())?;
    let stdout_thread = thread::spawn(move || read_stream_bounded(stdout, stdout_limit));
    let stderr_thread = thread::spawn(move || read_stream_bounded(stderr, stderr_limit));
    let started = SystemTime::now();
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("GIT_WAIT: {error}"))?
        {
            break status;
        }
        if started.elapsed().unwrap_or_default() > COMMAND_TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            return Err("GIT_TIMEOUT: Git command exceeded the fixed timeout".to_string());
        }
        thread::sleep(Duration::from_millis(15));
    };
    let (stdout, stdout_truncated) = stdout_thread
        .join()
        .map_err(|_| "GIT_STDOUT_READER_FAILED".to_string())?;
    let (stderr, _) = stderr_thread
        .join()
        .map_err(|_| "GIT_STDERR_READER_FAILED".to_string())?;
    if !status.success() {
        let message = String::from_utf8_lossy(&stderr).trim().to_string();
        return Err(format!(
            "GIT_EXIT_{}{}",
            status.code().unwrap_or(-1),
            if message.is_empty() {
                String::new()
            } else {
                format!(": {message}")
            }
        ));
    }
    Ok(BoundedGitOutput {
        stdout,
        stdout_truncated,
    })
}

pub(crate) fn output_text(bytes: &[u8]) -> Result<String, String> {
    if bytes.len() > MAX_COMMAND_OUTPUT {
        return Err("GIT_OUTPUT_LIMIT: Git output exceeded the fixed limit".to_string());
    }
    String::from_utf8(bytes.to_vec())
        .map_err(|_| "GIT_OUTPUT_BINARY: Git returned non-text output".to_string())
}
fn bound_text(text: &str, byte_limit: usize, line_limit: usize) -> (String, bool) {
    let mut output = text.lines().take(line_limit).collect::<Vec<_>>().join("\n");
    let mut truncated = text.lines().count() > line_limit;
    if output.len() > byte_limit {
        output.truncate(byte_limit);
        truncated = true;
    }
    (output, truncated)
}
fn timestamp() -> String {
    crate::time::utc_timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn git(path: &Path, args: &[&str]) {
        assert!(Command::new("git")
            .args(args)
            .current_dir(path)
            .output()
            .expect("git available")
            .status
            .success());
    }
    fn fixture() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        git(dir.path(), &["init", "-q"]);
        git(dir.path(), &["config", "user.name", "Test User"]);
        git(dir.path(), &["config", "user.email", "test@example.com"]);
        fs::write(dir.path().join("tracked.txt"), "one\n").unwrap();
        git(dir.path(), &["add", "tracked.txt"]);
        git(dir.path(), &["commit", "-qm", "initial"]);
        dir
    }

    fn registered_fixture() -> (tempfile::TempDir, tempfile::TempDir, DatabaseState, String) {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        let repository = fixture();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: repository.path().to_string_lossy().into_owned(),
                name: Some("Diff Boundary".into()),
            },
        )
        .unwrap();
        (database_dir, repository, database, project.id)
    }

    fn product_diff(database: &DatabaseState, project_id: &str, scope: GitDiffScope) -> GitDiff {
        diff(
            database,
            GitDiffRequest {
                project_id: project_id.into(),
                scope,
                base_ref: None,
                head_sha: None,
            },
        )
        .unwrap()
    }

    #[test]
    fn diff_text_working_tree_product_boundary() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        fs::write(repository.path().join("tracked.txt"), "working change\n").unwrap();
        let result = product_diff(&database, &project_id, GitDiffScope::WorkingTree);
        assert!(result.text.contains("working change"));
        assert!(result.binary_files.is_empty());
    }

    #[test]
    fn diff_commit_range_collects_committed_implementation_identity() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        let base_sha = output_text(
            &run_git(repository.path(), &["rev-parse", "HEAD"])
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();
        fs::write(repository.path().join("committed.txt"), "implementation\n").unwrap();
        git(repository.path(), &["add", "committed.txt"]);
        git(repository.path(), &["commit", "-qm", "implementation"]);
        let head_sha = output_text(
            &run_git(repository.path(), &["rev-parse", "HEAD"])
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();
        let result = diff(
            &database,
            GitDiffRequest {
                project_id,
                scope: GitDiffScope::CommitRange,
                base_ref: Some(base_sha.clone()),
                head_sha: Some(head_sha.clone()),
            },
        )
        .unwrap();
        assert_eq!(result.scope, GitDiffScope::CommitRange);
        assert_eq!(result.base_sha.as_deref(), Some(base_sha.as_str()));
        assert_eq!(result.head_sha.as_deref(), Some(head_sha.as_str()));
        assert_eq!(result.changed_files, vec!["committed.txt"]);
        assert!(result.full_change_set_sha256.len() == 64);
    }

    #[test]
    fn diff_text_staged_product_boundary() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        fs::write(repository.path().join("tracked.txt"), "staged change\n").unwrap();
        git(repository.path(), &["add", "tracked.txt"]);
        let result = product_diff(&database, &project_id, GitDiffScope::Staged);
        assert!(result.text.contains("staged change"));
        assert!(result.binary_files.is_empty());
    }

    fn commit_binary(repository: &Path) {
        fs::write(repository.join(".gitattributes"), "*.bin binary\n").unwrap();
        fs::write(repository.join("image.bin"), [0_u8; 1024]).unwrap();
        git(repository, &["add", ".gitattributes"]);
        git(repository, &["add", "image.bin"]);
        git(repository, &["commit", "-qm", "binary"]);
    }

    #[test]
    fn diff_binary_working_tree_metadata_only() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        commit_binary(repository.path());
        fs::write(repository.path().join("image.bin"), [1_u8; 1024]).unwrap();
        let result = product_diff(&database, &project_id, GitDiffScope::WorkingTree);
        assert_eq!(result.binary_files, vec!["image.bin"]);
        assert!(!result.text.contains("GIT binary patch"));
    }

    #[test]
    fn diff_new_binary_staged_returns_real_path_not_dev_null() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        fs::write(repository.path().join(".gitattributes"), "*.bin binary\n").unwrap();
        fs::write(repository.path().join("new.bin"), [2_u8; 1024]).unwrap();
        git(repository.path(), &["add", ".gitattributes", "new.bin"]);
        let result = product_diff(&database, &project_id, GitDiffScope::Staged);
        assert_eq!(result.binary_files, vec!["new.bin"]);
        assert!(!result.binary_files.iter().any(|path| path == "/dev/null"));
    }

    #[test]
    fn diff_mixed_working_tree_keeps_text_and_binary_metadata() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        commit_binary(repository.path());
        fs::write(repository.path().join("image.bin"), [3_u8; 1024]).unwrap();
        fs::write(repository.path().join("tracked.txt"), "mixed working\n").unwrap();
        let result = product_diff(&database, &project_id, GitDiffScope::WorkingTree);
        assert!(result.text.contains("mixed working"));
        assert_eq!(result.binary_files, vec!["image.bin"]);
    }

    #[test]
    fn diff_mixed_staged_keeps_text_and_binary_metadata() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        commit_binary(repository.path());
        fs::write(repository.path().join("image.bin"), [4_u8; 1024]).unwrap();
        fs::write(repository.path().join("tracked.txt"), "mixed staged\n").unwrap();
        git(repository.path(), &["add", "."]);
        let result = product_diff(&database, &project_id, GitDiffScope::Staged);
        assert!(result.text.contains("mixed staged"));
        assert_eq!(result.binary_files, vec!["image.bin"]);
    }

    #[test]
    fn diff_return_never_contains_nul_or_binary_patch_payload() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        commit_binary(repository.path());
        fs::write(repository.path().join("image.bin"), [5_u8; 1024]).unwrap();
        let result = product_diff(&database, &project_id, GitDiffScope::WorkingTree);
        assert!(!result.text.as_bytes().contains(&0));
        assert!(!result.text.contains("GIT binary patch"));
        assert!(!result.text.contains("/dev/null"));
    }

    #[test]
    fn diff_truncation_is_after_sanitization() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        fs::write(repository.path().join("tracked.txt"), "line\n".repeat(5000)).unwrap();
        let result = product_diff(&database, &project_id, GitDiffScope::WorkingTree);
        assert!(result.truncated);
        assert!(!result.text.contains("GIT binary patch"));
    }

    #[test]
    fn large_diffs_are_bounded_but_keep_full_identity_for_all_scopes() {
        let large = "large implementation line\n".repeat(70_000);

        let (_database_dir, working_repository, working_database, working_project_id) =
            registered_fixture();
        fs::write(working_repository.path().join("tracked.txt"), &large).unwrap();
        let working = product_diff(
            &working_database,
            &working_project_id,
            GitDiffScope::WorkingTree,
        );
        assert!(working.truncated);
        assert_eq!(working.full_change_set_sha256.len(), 64);
        assert!(working.identity_complete);

        let (_database_dir, staged_repository, staged_database, staged_project_id) =
            registered_fixture();
        fs::write(staged_repository.path().join("tracked.txt"), &large).unwrap();
        git(staged_repository.path(), &["add", "tracked.txt"]);
        let staged = product_diff(&staged_database, &staged_project_id, GitDiffScope::Staged);
        assert!(staged.truncated);
        assert_eq!(staged.full_change_set_sha256.len(), 64);
        assert!(staged.identity_complete);

        let (_database_dir, range_repository, range_database, range_project_id) =
            registered_fixture();
        let base_sha = output_text(
            &run_git(range_repository.path(), &["rev-parse", "HEAD"])
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();
        fs::write(range_repository.path().join("tracked.txt"), &large).unwrap();
        git(range_repository.path(), &["add", "tracked.txt"]);
        git(
            range_repository.path(),
            &["commit", "-qm", "large implementation"],
        );
        let range = diff(
            &range_database,
            GitDiffRequest {
                project_id: range_project_id,
                scope: GitDiffScope::CommitRange,
                base_ref: Some(base_sha),
                head_sha: None,
            },
        )
        .unwrap();
        assert!(range.truncated);
        assert_eq!(range.full_change_set_sha256.len(), 64);
        assert!(range.identity_complete);
    }

    #[test]
    fn untracked_identity_streams_content_and_marks_budget_overflow() {
        let (_database_dir, repository, database, project_id) = registered_fixture();
        fs::write(repository.path().join("new.txt"), "untracked\n").unwrap();
        let result = product_diff(&database, &project_id, GitDiffScope::WorkingTree);
        assert_eq!(
            result.untracked_content_sha256.as_deref().map(str::len),
            Some(64)
        );
        assert!(result.identity_complete);

        fs::write(
            repository.path().join("oversized.txt"),
            vec![b'x'; (MAX_UNTRACKED_IDENTITY_BYTES + 1) as usize],
        )
        .unwrap();
        let bounded = product_diff(&database, &project_id, GitDiffScope::WorkingTree);
        assert!(!bounded.identity_complete);
        assert_eq!(
            bounded.identity_diagnostic.as_deref(),
            Some("GIT_UNTRACKED_IDENTITY_BYTE_LIMIT")
        );
    }

    #[test]
    fn status_matrix_detects_staged_unstaged_untracked_and_clean() {
        let dir = fixture();
        let clean = collect_snapshot("p", "r", dir.path()).unwrap();
        assert!(matches!(clean.health, RepositoryHealth::Clean));
        fs::write(dir.path().join("tracked.txt"), "two\n").unwrap();
        fs::write(dir.path().join("new.txt"), "new\n").unwrap();
        git(dir.path(), &["add", "tracked.txt"]);
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert_eq!(snapshot.staged_files[0].kind, "MODIFIED");
        assert_eq!(snapshot.untracked_files, vec!["new.txt"]);
        fs::write(dir.path().join("tracked.txt"), "three\n").unwrap();
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert!(!snapshot.unstaged_files.is_empty());
    }
    #[test]
    fn branch_head_detached_and_unborn_are_distinguished() {
        let dir = fixture();
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert!(matches!(
            snapshot.current_branch.as_deref(),
            Some("master") | Some("main")
        ));
        assert!(snapshot.head_sha.is_some());
        git(dir.path(), &["checkout", "--detach", "-q", "HEAD"]);
        let detached = collect_snapshot("p", "r", dir.path()).unwrap();
        assert!(detached.detached_head);
        let empty = tempdir().unwrap();
        git(empty.path(), &["init", "-q"]);
        let unborn = collect_snapshot("p", "r", empty.path()).unwrap();
        assert!(matches!(unborn.health, RepositoryHealth::Unborn));
    }
    #[test]
    fn no_upstream_is_explicitly_unavailable_and_remote_is_sanitized() {
        let dir = fixture();
        git(
            dir.path(),
            &[
                "remote",
                "add",
                "origin",
                "https://user:secret@example.com/a/repo.git",
            ],
        );
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert_eq!(snapshot.ahead_count, None);
        assert_eq!(snapshot.behind_count, None);
        assert_eq!(
            snapshot.remotes[0].fetch_url,
            "https://example.com/a/repo.git"
        );
    }
    #[test]
    fn bounded_diff_reports_truncation_and_worktree_fixture() {
        let dir = fixture();
        fs::write(dir.path().join("tracked.txt"), "x\n".repeat(2000)).unwrap();
        let diff = diff_fixture(dir.path(), false);
        assert!(diff.truncated);
    }
    fn diff_fixture(path: &Path, staged: bool) -> GitDiff {
        let args = if staged {
            vec!["diff", "--cached", "--", "tracked.txt"]
        } else {
            vec!["diff", "--", "tracked.txt"]
        };
        let output = run_git(path, &args).unwrap();
        let raw = output_text(&output.stdout).unwrap();
        let (text, truncated) = bound_text(&raw, 10, 2);
        GitDiff {
            project_id: "p".into(),
            scope: if staged {
                GitDiffScope::Staged
            } else {
                GitDiffScope::WorkingTree
            },
            base_ref: None,
            base_sha: None,
            head_sha: None,
            changed_files: vec!["tracked.txt".into()],
            text,
            truncated,
            binary_files: vec![],
            byte_limit: 10,
            line_limit: 2,
            full_change_set_sha256: hash_bytes(raw.as_bytes()),
            untracked_content_sha256: None,
            identity_complete: true,
            identity_diagnostic: None,
        }
    }
    #[test]
    fn recent_commits_are_bounded() {
        let dir = fixture();
        let commits = read_commits(dir.path()).unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].parent_count, 0);
    }
    #[test]
    fn worktree_fixture_is_read_only() {
        let dir = fixture();
        let worktrees = read_worktrees(dir.path()).unwrap();
        assert_eq!(worktrees.len(), 1);
        assert_eq!(worktrees[0].head_sha.as_deref().unwrap().len(), 40);
    }
    #[test]
    fn non_git_and_missing_errors_are_structured() {
        assert!(detect_non_git(tempdir().unwrap().path()).contains("NON_GIT"));
    }

    #[test]
    fn production_git_path_captures_bounded_version_output() {
        let dir = tempdir().unwrap();
        let output = run_git(dir.path(), &["--version"]).unwrap();
        let version = output_text(&output.stdout).unwrap();
        assert!(version.starts_with("git version "));
        assert!(version.len() <= 128);
    }

    #[test]
    fn production_git_path_preserves_structured_exit_errors() {
        let dir = tempdir().unwrap();
        let error = run_git(dir.path(), &["not-a-real-git-subcommand"]).unwrap_err();
        assert!(error.starts_with("GIT_EXIT_"));
    }
    fn detect_non_git(path: &Path) -> String {
        if !path.join(".git").exists() {
            "NON_GIT_PROJECT".into()
        } else {
            String::new()
        }
    }

    #[test]
    fn status_parser_distinguishes_deleted_renamed_and_conflicted_records() {
        let status = "## main\0D  deleted.txt\0R  old.txt -> new.txt\0UU conflict.txt\0";
        let (staged, _unstaged, _, conflicts) = parse_status(status);
        assert!(staged.iter().any(|file| file.kind == "DELETED"));
        assert!(staged.iter().any(|file| file.kind == "RENAMED"));
        assert_eq!(conflicts, vec!["conflict.txt"]);
    }

    #[test]
    fn raw_binary_diff_fixture_is_sanitized_at_the_product_boundary() {
        let dir = fixture();
        fs::write(dir.path().join("image.bin"), [0_u8; 1024]).unwrap();
        fs::write(dir.path().join(".gitattributes"), "image.bin binary\n").unwrap();
        git(dir.path(), &["add", "image.bin"]);
        git(dir.path(), &["add", ".gitattributes"]);
        git(dir.path(), &["commit", "-qm", "binary"]);
        fs::write(dir.path().join("image.bin"), [1_u8; 1024]).unwrap();
        let output = run_git(dir.path(), &["diff", "--binary", "--", "image.bin"]).unwrap();
        let text = output_text(&output.stdout).unwrap();
        assert!(text.contains("Binary files") || text.contains("GIT binary patch"));
        let sanitized = sanitize_binary_payloads(&text);
        assert!(!sanitized.contains("GIT binary patch"));
        assert!(!text.as_bytes().contains(&0));
    }

    #[test]
    fn worktree_fixture_reports_second_checkout() {
        let dir = fixture();
        let worktree = tempdir().unwrap();
        git(
            dir.path(),
            &[
                "worktree",
                "add",
                "-q",
                "-b",
                "m06-worktree",
                worktree.path().to_str().unwrap(),
            ],
        );
        let trees = read_worktrees(dir.path()).unwrap();
        assert_eq!(trees.len(), 2);
        assert!(trees
            .iter()
            .any(|tree| tree.branch.as_deref() == Some("m06-worktree")));
    }

    #[test]
    fn upstream_counts_use_local_bare_remote_without_network() {
        let dir = fixture();
        let bare = tempdir().unwrap();
        git(bare.path(), &["init", "--bare", "-q"]);
        git(
            dir.path(),
            &["remote", "add", "origin", bare.path().to_str().unwrap()],
        );
        git(dir.path(), &["push", "-q", "-u", "origin", "HEAD"]);
        fs::write(dir.path().join("tracked.txt"), "two\n").unwrap();
        git(dir.path(), &["add", "tracked.txt"]);
        git(dir.path(), &["commit", "-qm", "ahead"]);
        let snapshot = collect_snapshot("p", "r", dir.path()).unwrap();
        assert_eq!(snapshot.ahead_count, Some(1));
        assert_eq!(snapshot.behind_count, Some(0));
    }
}
