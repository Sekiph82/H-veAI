use crate::db::DatabaseState;
use crate::projects::{list_projects, ProjectListQuery, ProjectRecord};
use crate::time::utc_timestamp;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[cfg(not(test))]
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::process::Stdio;
#[cfg(not(test))]
use std::sync::mpsc;
#[cfg(not(test))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;
#[cfg(not(test))]
use tauri::Emitter;

pub const GITHUB_TASKS_ONLY_POLICY: &str = "GITHUB_TASKS_ONLY";
const REMOTE_TASKS_RESOURCE_KIND: &str = "GITHUB_TASKS_REMOTE";
pub const SELECTED_PROJECT_REFRESH_SECONDS: u64 = 10;
pub const PORTFOLIO_REFRESH_SECONDS: u64 = 30;

#[derive(Debug, Default)]
pub struct RefreshRequestGate(std::sync::atomic::AtomicBool);

impl RefreshRequestGate {
    pub fn request(&self) -> bool {
        !self.0.swap(true, std::sync::atomic::Ordering::AcqRel)
    }

    pub fn take(&self) -> bool {
        self.0.swap(false, std::sync::atomic::Ordering::AcqRel)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteTrackingSnapshot {
    pub project_key: String,
    pub display_name: String,
    pub repository: String,
    pub branch: String,
    pub remote_head: Option<String>,
    pub project_blob_sha: Option<String>,
    pub tasks_blob_sha: Option<String>,
    pub rules_blob_sha: Option<String>,
    pub events_blob_sha: Option<String>,
    pub current_milestone: Option<String>,
    pub current_sprint: Option<String>,
    pub current_task_id: Option<String>,
    pub current_task_title: Option<String>,
    #[serde(default)]
    pub current_task_status: Option<String>,
    pub workflow_state: Option<String>,
    pub required_actor: Option<String>,
    pub next_action: Option<String>,
    #[serde(default)]
    pub next_task_id: Option<String>,
    #[serde(default)]
    pub next_task_title: Option<String>,
    pub blockers: Vec<String>,
    pub progress_scope_type: Option<String>,
    pub progress_scope_id: Option<String>,
    pub progress_completed: Option<u64>,
    pub progress_total: Option<u64>,
    pub progress_percent: Option<f64>,
    pub last_completed_task_id: Option<String>,
    pub last_completed_task_title: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
    #[serde(default)]
    pub total_tasks: Option<u64>,
    #[serde(default)]
    pub completed_tasks: Option<u64>,
    #[serde(default)]
    pub latest_commit_message: Option<String>,
    #[serde(default)]
    pub latest_commit_author: Option<String>,
    #[serde(default)]
    pub latest_commit_at: Option<String>,
    pub fetched_at: String,
    pub remote_health: String,
    pub error: Option<String>,
    #[serde(default)]
    pub recent_events: Vec<RemoteTrackingEvent>,
    #[serde(default)]
    pub task_rows: Vec<RemoteTaskRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteTrackingEvent {
    pub id: String,
    pub event_type: Option<String>,
    pub task_id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub actor: Option<String>,
    pub timestamp: Option<String>,
    pub commit_sha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RemoteTaskRow {
    pub id: String,
    pub title: String,
    pub status: String,
    pub source_path: String,
    pub source_line: usize,
}

pub fn ensure_portfolio(database: &DatabaseState) -> Result<(), String> {
    const TARGETS: [(&str, &str, &str, &str); 8] = [
        ("h-veai", "H-veAI", "Sekiph82/H-veAI", "main"),
        ("bulk-edit", "Bulk-Edit", "Sekiph82/Bulk-Edit", "main"),
        (
            "fmcg-erp-system",
            "fmcg-erp-system",
            "Sekiph82/fmcg-erp-system",
            "main",
        ),
        (
            "formulab",
            "FormuLab",
            "Sekiph82/FormuLab",
            "feature/laboratory-stability",
        ),
        ("packlab", "PackLab", "Sekiph82/PackLab", "main"),
        ("packlab-3d", "PackLab 3D", "Sekiph82/PackLab-3D", "main"),
        ("scrubbots", "ScrubBots", "Sekiph82/Scrubbots", "main"),
        (
            "scrubbots-level-factory",
            "ScrubBots - Pixel Art Generator",
            "Sekiph82/ScrubBots-Level-Factory",
            "main",
        ),
    ];
    let connection = database.open_connection()?;
    let transaction = connection.unchecked_transaction().map_err(db_error)?;
    let now = utc_timestamp();
    let mut target_project_ids = HashSet::new();
    for (_id, name, repository, branch) in TARGETS {
        let mut parts = repository.splitn(2, '/');
        let owner = parts.next().unwrap_or_default();
        let repo = parts.next().unwrap_or_default();
        let target_id = format!("github:{repository}@{branch}");
        let excluded: bool = transaction
            .query_row(
                "SELECT 1 FROM github_project_exclusions WHERE lower(repository)=lower(?1) AND branch=?2",
                params![repository, branch],
                |row| row.get::<_, i64>(0),
            )
            .optional()
            .map_err(db_error)?
            .is_some();
        if excluded {
            continue;
        }
        let project_id: String = transaction
            .query_row(
                "SELECT p.id FROM projects p JOIN repositories r ON r.project_id=p.id WHERE p.id=?1 LIMIT 1",
                [&target_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(db_error)?
            .or_else(|| {
                transaction
                    .query_row(
                        "SELECT p.id FROM projects p JOIN repositories r ON r.project_id=p.id WHERE lower(r.github_owner)=lower(?1) AND lower(r.github_repo)=lower(?2) AND p.task_source_policy=?3 ORDER BY p.id LIMIT 1",
                        params![owner, repo, GITHUB_TASKS_ONLY_POLICY],
                        |row| row.get(0),
                    )
                    .optional()
                    .ok()
                    .flatten()
            })
            .or_else(|| {
                transaction
                    .query_row(
                        "SELECT p.id FROM projects p JOIN repositories r ON r.project_id=p.id WHERE lower(r.github_owner)=lower(?1) AND lower(r.github_repo)=lower(?2) ORDER BY p.id LIMIT 1",
                        params![owner, repo],
                        |row| row.get(0),
                    )
                    .optional()
                    .ok()
                    .flatten()
            })
            .unwrap_or(target_id);
        let branch = branch.to_string();
        target_project_ids.insert(project_id.clone());
        let existing = transaction
            .query_row("SELECT 1 FROM projects WHERE id=?1", [&project_id], |row| {
                row.get::<_, i64>(0)
            })
            .optional()
            .map_err(db_error)?;
        if existing.is_none() {
            transaction.execute(
                "INSERT INTO projects (id,name,local_path,default_branch,status,priority,metadata_json,created_at,updated_at,original_path,normalized_path,registered_at,last_validated_at,task_source_policy) VALUES (?1,?2,NULL,?3,'ACTIVE',0,?4,?5,?5,NULL,NULL,?5,NULL,?6)",
                params![project_id, name, branch, r#"{"tracking":"github-tasks-only"}"#, now, GITHUB_TASKS_ONLY_POLICY],
            ).map_err(db_error)?;
            transaction.execute(
                "INSERT INTO repositories (id,project_id,remote_url,github_owner,github_repo,default_branch,created_at,updated_at,is_git_repository) VALUES (?1,?2,?3,?4,?5,?6,?7,?7,1)",
                params![format!("{project_id}:repository"), project_id, format!("https://github.com/{repository}.git"), owner, repo, branch, now],
            ).map_err(db_error)?;
        } else {
            transaction
                .execute(
                    "UPDATE projects SET name=?2, default_branch=?3, task_source_policy=?4, status=CASE WHEN status='ARCHIVED' THEN 'ACTIVE' ELSE status END, archived_at=NULL, updated_at=?5 WHERE id=?1",
                    params![project_id, name, branch, GITHUB_TASKS_ONLY_POLICY, now],
                )
                .map_err(db_error)?;
            let repository_updated = transaction
                .execute(
                    "UPDATE repositories SET remote_url=?2, github_owner=?3, github_repo=?4, default_branch=?5, is_git_repository=1, updated_at=?6 WHERE project_id=?1",
                    params![project_id, format!("https://github.com/{repository}.git"), owner, repo, branch, now],
                )
                .map_err(db_error)?;
            if repository_updated == 0 {
                transaction
                    .execute(
                        "INSERT INTO repositories (id,project_id,remote_url,github_owner,github_repo,default_branch,created_at,updated_at,is_git_repository) VALUES (?1,?2,?3,?4,?5,?6,?7,?7,1)",
                        params![format!("{project_id}:repository"), project_id, format!("https://github.com/{repository}.git"), owner, repo, branch, now],
                    )
                    .map_err(db_error)?;
            }
        }
        let duplicate_ids = transaction
            .prepare("SELECT p.id FROM projects p JOIN repositories r ON r.project_id=p.id WHERE lower(r.github_owner)=lower(?1) AND lower(r.github_repo)=lower(?2) AND p.id<>?3 ORDER BY p.id")
            .map_err(db_error)?
            .query_map(params![owner, repo, project_id], |row| row.get::<_, String>(0))
            .map_err(db_error)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(db_error)?;
        for duplicate_id in duplicate_ids {
            merge_duplicate_project(&transaction, &project_id, &duplicate_id, &now)?;
        }
    }
    let stale_project_ids = transaction
        .prepare("SELECT id FROM projects WHERE status <> 'ARCHIVED'")
        .map_err(db_error)?
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(db_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(db_error)?;
    for stale_project_id in stale_project_ids {
        if target_project_ids.contains(&stale_project_id) {
            continue;
        }
        transaction
            .execute(
                "UPDATE projects SET status='ARCHIVED', archived_at=COALESCE(archived_at, ?2), updated_at=?2 WHERE id=?1",
                params![stale_project_id, now],
            )
            .map_err(db_error)?;
        transaction
            .execute(
                "DELETE FROM github_sync_state WHERE project_id=?1",
                [&stale_project_id],
            )
            .map_err(db_error)?;
    }
    transaction
        .execute(
            "DELETE FROM github_sync_state WHERE resource_kind <> ?1",
            [REMOTE_TASKS_RESOURCE_KIND],
        )
        .map_err(db_error)?;
    transaction.commit().map_err(db_error)?;
    Ok(())
}

fn merge_duplicate_project(
    transaction: &rusqlite::Transaction<'_>,
    canonical_id: &str,
    duplicate_id: &str,
    now: &str,
) -> Result<(), String> {
    let duplicate: Option<(
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        i64,
    )> = transaction
        .query_row(
            "SELECT local_path, original_path, normalized_path, registered_at, last_validated_at, preferred_builder, preferred_auditor, priority FROM projects WHERE id=?1",
            [duplicate_id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                ))
            },
        )
        .optional()
        .map_err(db_error)?;
    let Some((
        local_path,
        original_path,
        normalized_path,
        registered_at,
        last_validated_at,
        preferred_builder,
        preferred_auditor,
        priority,
    )) = duplicate
    else {
        return Ok(());
    };
    let canonical_has_path: bool = transaction
        .query_row(
            "SELECT COALESCE(NULLIF(TRIM(COALESCE(normalized_path, '')), ''), NULL) IS NOT NULL FROM projects WHERE id=?1",
            [canonical_id],
            |row| row.get(0),
        )
        .map_err(db_error)?;
    if !canonical_has_path
        && normalized_path
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
    {
        transaction
            .execute(
                "UPDATE projects SET local_path=?2, original_path=?3, normalized_path=?4, registered_at=COALESCE(registered_at, ?5), last_validated_at=?6, updated_at=?7 WHERE id=?1",
                params![canonical_id, local_path, original_path, normalized_path, registered_at, last_validated_at, now],
            )
            .map_err(db_error)?;
    }
    transaction
        .execute(
            "UPDATE projects SET preferred_builder=COALESCE(preferred_builder, ?2), preferred_auditor=COALESCE(preferred_auditor, ?3), priority=CASE WHEN priority=0 THEN ?4 ELSE priority END WHERE id=?1",
            params![canonical_id, preferred_builder, preferred_auditor, priority],
        )
        .map_err(db_error)?;

    transaction
        .execute(
            "DELETE FROM github_sync_state WHERE project_id=?1",
            [duplicate_id],
        )
        .map_err(db_error)?;
    for table in [
        "project_sources",
        "task_sources",
        "tasks",
        "prompts",
        "agent_sessions",
        "audits",
        "test_runs",
        "alerts",
        "decisions",
        "project_snapshots",
    ] {
        transaction
            .execute(
                &format!("UPDATE {table} SET project_id=?1 WHERE project_id=?2"),
                params![canonical_id, duplicate_id],
            )
            .map_err(db_error)?;
    }
    transaction
        .execute(
            "DELETE FROM repositories WHERE project_id=?1",
            [duplicate_id],
        )
        .map_err(db_error)?;
    transaction
        .execute("DELETE FROM projects WHERE id=?1", [duplicate_id])
        .map_err(db_error)?;
    Ok(())
}

pub fn refresh_all(database: &DatabaseState) -> Result<Vec<RemoteTrackingSnapshot>, String> {
    ensure_portfolio(database)?;
    let projects = list_projects(
        database,
        ProjectListQuery {
            include_archived: Some(false),
            ..Default::default()
        },
    )?;
    Ok(projects
        .iter()
        .filter_map(|project| refresh_project(database, project).ok())
        .collect())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteObservationChange {
    Unchanged,
    Changed,
}

pub fn refresh_interval_seconds(selected: bool) -> u64 {
    if selected {
        SELECTED_PROJECT_REFRESH_SECONDS
    } else {
        PORTFOLIO_REFRESH_SECONDS
    }
}

pub fn is_github_tasks_project(project: &ProjectRecord) -> bool {
    project.task_source_policy.as_deref() == Some(GITHUB_TASKS_ONLY_POLICY)
        && project
            .repository
            .as_ref()
            .and_then(|repository| repository.github_owner.as_ref())
            .is_some()
        && project
            .repository
            .as_ref()
            .and_then(|repository| repository.github_repo.as_ref())
            .is_some()
}

pub fn observe_project(
    database: &DatabaseState,
    project: &ProjectRecord,
) -> Result<(RemoteTrackingSnapshot, RemoteObservationChange), String> {
    let repository = project
        .repository
        .as_ref()
        .ok_or_else(|| "GitHub repository identity is unavailable".to_string())?;
    let owner = repository
        .github_owner
        .clone()
        .ok_or_else(|| "GitHub owner is unavailable".to_string())?;
    let repo = repository
        .github_repo
        .clone()
        .ok_or_else(|| "GitHub repository is unavailable".to_string())?;
    let branch = repository
        .default_branch
        .clone()
        .or_else(|| repository.current_branch.clone())
        .ok_or_else(|| "GitHub tracked branch is unavailable".to_string())?;
    if cfg!(test) {
        return Err("remote observation is disabled for local unit fixtures".into());
    }
    let repository_name = format!("{owner}/{repo}");
    let fetched_at = utc_timestamp();
    let previous = cached(database, project, "")?;
    match fetch_github_head(&repository_name, &branch) {
        Ok(head) => {
            if let Some(snapshot) = previous.as_ref() {
                if snapshot.remote_head.as_deref() == Some(head.as_str())
                    && snapshot.remote_health == "CURRENT"
                {
                    return Ok((snapshot.clone(), RemoteObservationChange::Unchanged));
                }
            }
            match fetch_github_root_tasks(&repository_name, &branch, &head).and_then(|raw| {
                parse_root_tasks(&raw, &repository_name, &branch, fetched_at.clone())
            }) {
                Ok(snapshot) => {
                    let changed = previous
                        .as_ref()
                        .map(|old| old.remote_head != snapshot.remote_head)
                        .unwrap_or(true);
                    persist(database, project, &snapshot)?;
                    Ok((
                        snapshot,
                        if changed {
                            RemoteObservationChange::Changed
                        } else {
                            RemoteObservationChange::Unchanged
                        },
                    ))
                }
                Err(error) => {
                    let snapshot =
                        remote_error(&repository_name, &branch, Some(head), error, fetched_at);
                    persist(database, project, &snapshot)?;
                    Ok((snapshot, RemoteObservationChange::Changed))
                }
            }
        }
        Err(error) => match previous {
            Some(mut snapshot) => {
                snapshot.remote_health = "STALE".into();
                snapshot.error = Some(error);
                persist(database, project, &snapshot)?;
                Ok((snapshot, RemoteObservationChange::Unchanged))
            }
            None => {
                let snapshot = unavailable(&repository_name, &branch, error, fetched_at);
                persist(database, project, &snapshot)?;
                Ok((snapshot, RemoteObservationChange::Changed))
            }
        },
    }
}

pub fn refresh_project(
    database: &DatabaseState,
    project: &ProjectRecord,
) -> Result<RemoteTrackingSnapshot, String> {
    let repository = project
        .repository
        .as_ref()
        .ok_or_else(|| "GitHub repository identity is unavailable".to_string())?;
    let repository_name = match (&repository.github_owner, &repository.github_repo) {
        (Some(owner), Some(repo)) => format!("{owner}/{repo}"),
        _ => return Err("GitHub repository identity is unavailable".into()),
    };
    let branch = repository
        .default_branch
        .clone()
        .or_else(|| repository.current_branch.clone())
        .ok_or_else(|| "GitHub tracked branch is unavailable".to_string())?;
    if let Some(snapshot) = cached(database, project, "")? {
        return Ok(snapshot);
    }
    Ok(unavailable(
        &repository_name,
        &branch,
        "GitHub refresh is pending; no cached remote snapshot is available".into(),
        utc_timestamp(),
    ))
}

pub fn unavailable_for_project(project: &ProjectRecord, error: String) -> RemoteTrackingSnapshot {
    let repository = project.repository.as_ref();
    let owner = repository
        .and_then(|value| value.github_owner.as_deref())
        .unwrap_or("unknown");
    let repo = repository
        .and_then(|value| value.github_repo.as_deref())
        .unwrap_or("unknown");
    let branch = repository
        .and_then(|value| value.default_branch.as_deref())
        .or_else(|| repository.and_then(|value| value.current_branch.as_deref()))
        .unwrap_or("unknown");
    unavailable(&format!("{owner}/{repo}"), branch, error, utc_timestamp())
}

pub fn remote_health(snapshot: &RemoteTrackingSnapshot) -> &'static str {
    match snapshot.remote_health.as_str() {
        "CURRENT" => {
            let workflow = snapshot
                .workflow_state
                .as_deref()
                .unwrap_or_default()
                .to_ascii_uppercase();
            let actor = snapshot
                .required_actor
                .as_deref()
                .unwrap_or_default()
                .to_ascii_uppercase();
            if workflow == "BLOCKED" || !snapshot.blockers.is_empty() {
                "BLOCKED"
            } else if actor == "OWNER" && workflow.contains("WAIT") {
                "WAITING_OWNER"
            } else if actor.contains("AUDIT") && workflow.contains("WAIT") {
                "WAITING_AUDIT"
            } else {
                "HEALTHY"
            }
        }
        "STALE" | "STALE_REMOTE_SNAPSHOT" => "STALE",
        "ERROR" => "ERROR",
        _ => "UNAVAILABLE",
    }
}

struct RootTasksRemote {
    head: String,
    tasks: String,
    tasks_blob_sha: String,
    latest_commit_message: Option<String>,
    latest_commit_author: Option<String>,
    latest_commit_at: Option<String>,
}

fn github_url_part(value: &str) -> Result<String, String> {
    if value.is_empty()
        || value.contains("..")
        || value.contains('?')
        || value.contains('#')
        || value.contains('\\')
    {
        return Err("unsafe GitHub repository or branch identifier".into());
    }
    Ok(value.to_string())
}

fn run_http(url: &str) -> Result<String, String> {
    let mut child = crate::process_policy::background_command("curl.exe")
        .args([
            "-L",
            "--fail",
            "--silent",
            "--show-error",
            "--max-time",
            "30",
            url,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("GitHub HTTP observation failed to start: {e}"))?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| "GitHub HTTP observation stdout was not captured".to_string())?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| "GitHub HTTP observation stderr was not captured".to_string())?;
    let stdout_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        let result = stdout.read_to_end(&mut bytes);
        (result, bytes)
    });
    let stderr_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        let result = stderr.read_to_end(&mut bytes);
        (result, bytes)
    });
    let deadline = std::time::Instant::now() + Duration::from_secs(35);
    let status;
    loop {
        match child
            .try_wait()
            .map_err(|e| format!("GitHub HTTP observation failed: {e}"))?
        {
            Some(value) => {
                status = value;
                break;
            }
            None if std::time::Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Err("GitHub HTTP observation timed out after 35 seconds".into());
            }
            None => thread::sleep(Duration::from_millis(50)),
        }
    }
    let (stdout_result, stdout_bytes) = stdout_reader
        .join()
        .map_err(|_| "GitHub HTTP stdout reader failed".to_string())?;
    stdout_result.map_err(|e| format!("GitHub HTTP stdout read failed: {e}"))?;
    let (stderr_result, stderr_bytes) = stderr_reader
        .join()
        .map_err(|_| "GitHub HTTP stderr reader failed".to_string())?;
    stderr_result.map_err(|e| format!("GitHub HTTP stderr read failed: {e}"))?;
    if !status.success() {
        return Err(String::from_utf8_lossy(&stderr_bytes).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&stdout_bytes).to_string())
}

fn atom_value(entry: &str, tag: &str) -> Option<String> {
    let start = entry.find(&format!("<{tag}"))?;
    let start = entry[start..].find('>')? + start + 1;
    let end = entry[start..].find(&format!("</{tag}>"))? + start;
    Some(
        entry[start..end]
            .trim()
            .replace("\n", " ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" "),
    )
}

fn fetch_github_head(repository: &str, branch: &str) -> Result<String, String> {
    let repository = github_url_part(repository)?;
    let branch = github_url_part(branch)?;
    let feed = run_http(&format!(
        "https://github.com/{repository}/commits/{branch}.atom"
    ))?;
    let entry = feed
        .split("<entry>")
        .nth(1)
        .ok_or_else(|| format!("GitHub branch {repository}@{branch} returned no commits"))?;
    atom_value(entry, "id")
        .and_then(|value| value.rsplit_once('/').map(|(_, sha)| sha.to_string()))
        .filter(|sha| sha.len() == 40 && sha.chars().all(|c| c.is_ascii_hexdigit()))
        .ok_or_else(|| format!("GitHub branch {repository}@{branch} returned no valid HEAD"))
}

fn fetch_github_root_tasks(
    repository: &str,
    branch: &str,
    head: &str,
) -> Result<RootTasksRemote, String> {
    let repository = github_url_part(repository)?;
    let branch = github_url_part(branch)?;
    let tasks = run_http(&format!(
        "https://raw.githubusercontent.com/{repository}/{branch}/TASKS.md"
    ))?;
    if tasks.trim().is_empty() {
        return Err("GitHub root TASKS.md is empty".into());
    }
    let feed = run_http(&format!(
        "https://github.com/{repository}/commits/{branch}.atom"
    ))?;
    let entry = feed.split("<entry>").nth(1).unwrap_or_default();
    Ok(RootTasksRemote {
        head: head.to_string(),
        tasks_blob_sha: format!("sha256:{:x}", Sha256::digest(tasks.as_bytes())),
        tasks,
        latest_commit_message: atom_value(entry, "title"),
        latest_commit_author: atom_value(entry, "name"),
        latest_commit_at: atom_value(entry, "updated"),
    })
}

fn markdown_field(text: &str, labels: &[&str]) -> Option<String> {
    text.lines().find_map(|line| {
        let normalized = line.trim().trim_start_matches('-').trim();
        labels.iter().find_map(|label| {
            normalized
                .strip_prefix(label)
                .map(|value| value.trim().trim_matches('`').to_string())
                .filter(|value| !value.is_empty())
        })
    })
}

fn task_row(line: &str) -> Option<(char, String, String)> {
    let value = line.trim_start().strip_prefix("-")?.trim_start();
    let mut chars = value.chars();
    if chars.next()? != '[' {
        return None;
    }
    let status = chars.next()?;
    if chars.next()? != ']' {
        return None;
    }
    let rest = chars.as_str().trim();
    if rest.is_empty() {
        return None;
    }
    let (id, title) = if let Some(value) = rest.strip_prefix("**") {
        value
            .split_once("**")
            .map(|(id, title)| (id, title.trim()))
            .unwrap_or((rest, rest))
    } else {
        rest.split_once('—')
            .or_else(|| rest.split_once(" - "))
            .or_else(|| rest.split_once(char::is_whitespace))
            .unwrap_or((rest, rest))
    };
    let id = id.trim().trim_matches('*').trim_matches('`').to_string();
    let title = title.trim().trim_matches('*').to_string();
    Some((status, id, title))
}

fn parse_root_tasks(
    raw: &RootTasksRemote,
    repository: &str,
    branch: &str,
    fetched_at: String,
) -> Result<RemoteTrackingSnapshot, String> {
    let display_name = repository
        .split('/')
        .nth(1)
        .unwrap_or(repository)
        .to_string();
    let current = markdown_field(&raw.tasks, &["Current Task:"])
        .filter(|value| {
            let normalized = value.to_ascii_lowercase();
            !normalized.contains("no exact current task")
                && !normalized.contains("no active task")
                && !normalized.starts_with("none")
        })
        .and_then(|value| {
            let (id, title) = value
                .split_once('—')
                .unwrap_or((value.as_str(), value.as_str()));
            Some((id.trim().to_string(), title.trim().to_string()))
        });
    let current_task_id = current.as_ref().map(|value| value.0.clone());
    let current_task_title = current.as_ref().map(|value| value.1.clone());
    let current_task_status = markdown_field(&raw.tasks, &["Current Task Status:"]);
    let workflow_state = markdown_field(&raw.tasks, &["Workflow:", "Workflow State:"])
        .or_else(|| current_task_status.clone());
    let next_value = markdown_field(
        &raw.tasks,
        &["Next Task/Action:", "Next Action:", "Next Task:"],
    );
    let next_value = next_value.filter(|value| {
        let normalized = value.to_ascii_lowercase();
        !normalized.starts_with("none") && !normalized.contains("no future task")
    });
    let next_task = next_value
        .as_ref()
        .and_then(|value| value.split_once('—'))
        .map(|(id, title)| (id.trim().to_string(), title.trim().to_string()));
    let mut total = 0_u64;
    let mut completed = 0_u64;
    let mut last_completed = None;
    let mut task_rows = Vec::new();
    for (line_index, line) in raw.tasks.lines().enumerate() {
        if let Some((status, id, title)) = task_row(line) {
            total += 1;
            if matches!(status, 'x' | 'X') {
                completed += 1;
                last_completed = Some((id.clone(), title.clone()));
            }
            if task_rows.len() < 4096 {
                let normalized_status = match status {
                    'x' | 'X' => "TASK_COMPLETE",
                    '~' => "IN_PROGRESS",
                    '!' => "BLOCKED",
                    _ => "BACKLOG",
                };
                task_rows.push(RemoteTaskRow {
                    id,
                    title,
                    status: normalized_status.into(),
                    source_path: "TASKS.md".into(),
                    source_line: line_index + 1,
                });
            }
        }
    }
    let mut blockers = raw
        .tasks
        .lines()
        .skip_while(|line| !line.trim().eq_ignore_ascii_case("## Blockers/Waits"))
        .skip(1)
        .take_while(|line| !line.trim_start().starts_with("## "))
        .filter_map(|line| line.trim().strip_prefix('-').map(str::trim))
        .filter(|line| !line.is_empty() && !line.eq_ignore_ascii_case("none"))
        .map(str::to_string)
        .collect::<Vec<_>>();
    if blockers.is_empty() {
        blockers = markdown_field(&raw.tasks, &["Blockers/Waits:"])
            .into_iter()
            .flat_map(|value| {
                value
                    .split(';')
                    .map(str::trim)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .filter(|value| !value.is_empty() && !value.eq_ignore_ascii_case("none"))
            .collect();
    }
    let current_milestone = markdown_field(&raw.tasks, &["Current Milestone:"]);
    let percent = if total == 0 {
        None
    } else {
        Some(completed as f64 * 100.0 / total as f64)
    };
    Ok(RemoteTrackingSnapshot {
        project_key: format!("github:{repository}@{branch}"),
        display_name,
        repository: repository.into(),
        branch: branch.into(),
        remote_head: Some(raw.head.clone()),
        project_blob_sha: None,
        tasks_blob_sha: Some(raw.tasks_blob_sha.clone()),
        rules_blob_sha: None,
        events_blob_sha: None,
        current_milestone: current_milestone.clone(),
        current_sprint: markdown_field(&raw.tasks, &["Current Sprint:"]),
        current_task_id,
        current_task_title,
        current_task_status,
        workflow_state,
        required_actor: markdown_field(&raw.tasks, &["Required Actor:"]),
        next_action: next_value,
        next_task_id: next_task.as_ref().map(|value| value.0.clone()),
        next_task_title: next_task.map(|value| value.1),
        blockers,
        progress_scope_type: Some("TASKS".into()),
        progress_scope_id: current_milestone,
        progress_completed: Some(completed),
        progress_total: Some(total),
        progress_percent: percent,
        last_completed_task_id: last_completed.as_ref().map(|value| value.0.clone()),
        last_completed_task_title: last_completed.map(|value| value.1),
        updated_at: raw.latest_commit_at.clone(),
        updated_by: raw.latest_commit_author.clone(),
        total_tasks: Some(total),
        completed_tasks: Some(completed),
        latest_commit_message: raw.latest_commit_message.clone(),
        latest_commit_author: raw.latest_commit_author.clone(),
        latest_commit_at: raw.latest_commit_at.clone(),
        fetched_at,
        remote_health: "CURRENT".into(),
        error: None,
        recent_events: Vec::new(),
        task_rows,
    })
}

fn persist(
    database: &DatabaseState,
    project: &ProjectRecord,
    snapshot: &RemoteTrackingSnapshot,
) -> Result<(), String> {
    let connection = database.open_connection()?;
    let metadata = serde_json::to_string(snapshot).map_err(|e| e.to_string())?;
    connection.execute("INSERT INTO github_sync_state (id,project_id,resource_kind,resource_cursor,last_synced_at,metadata_json) VALUES (?1,?2,?3,?4,?5,?6) ON CONFLICT(project_id,resource_kind) DO UPDATE SET resource_cursor=excluded.resource_cursor,last_synced_at=excluded.last_synced_at,metadata_json=excluded.metadata_json", params![format!("{}:{REMOTE_TASKS_RESOURCE_KIND}", project.id), project.id, REMOTE_TASKS_RESOURCE_KIND, snapshot.remote_head, snapshot.fetched_at, metadata]).map_err(db_error)?;
    Ok(())
}

fn cached(
    database: &DatabaseState,
    project: &ProjectRecord,
    error: &str,
) -> Result<Option<RemoteTrackingSnapshot>, String> {
    let connection = database.open_connection()?;
    let value: Option<String> = connection
        .query_row(
            "SELECT metadata_json FROM github_sync_state WHERE project_id=?1 AND resource_kind=?2",
            params![project.id, REMOTE_TASKS_RESOURCE_KIND],
            |row| row.get(0),
        )
        .optional()
        .map_err(db_error)?;
    Ok(value
        .and_then(|json| serde_json::from_str::<RemoteTrackingSnapshot>(&json).ok())
        .map(|mut snapshot| {
            if !error.is_empty() {
                snapshot.error = Some(error.into());
            }
            snapshot
        }))
}

/// Returns the last validated GitHub snapshot without consulting the local workspace.
/// Source inventory uses this same remote cache so local prompts and documentation
/// cannot become live project-state inputs for GitHub-tracked projects.
pub fn cached_snapshot(
    database: &DatabaseState,
    project: &ProjectRecord,
) -> Result<Option<RemoteTrackingSnapshot>, String> {
    cached(database, project, "")
}

fn unavailable(
    repository: &str,
    branch: &str,
    error: String,
    fetched_at: String,
) -> RemoteTrackingSnapshot {
    RemoteTrackingSnapshot {
        project_key: format!("github:{repository}@{branch}"),
        display_name: repository.into(),
        repository: repository.into(),
        branch: branch.into(),
        remote_head: None,
        project_blob_sha: None,
        tasks_blob_sha: None,
        rules_blob_sha: None,
        events_blob_sha: None,
        current_milestone: None,
        current_sprint: None,
        current_task_id: None,
        current_task_title: None,
        current_task_status: None,
        workflow_state: None,
        required_actor: None,
        next_action: None,
        next_task_id: None,
        next_task_title: None,
        blockers: Vec::new(),
        progress_scope_type: None,
        progress_scope_id: None,
        progress_completed: None,
        progress_total: None,
        progress_percent: None,
        last_completed_task_id: None,
        last_completed_task_title: None,
        updated_at: None,
        updated_by: None,
        total_tasks: None,
        completed_tasks: None,
        latest_commit_message: None,
        latest_commit_author: None,
        latest_commit_at: None,
        fetched_at,
        remote_health: "UNAVAILABLE".into(),
        error: Some(error),
        recent_events: Vec::new(),
        task_rows: Vec::new(),
    }
}

fn remote_error(
    repository: &str,
    branch: &str,
    head: Option<String>,
    error: String,
    fetched_at: String,
) -> RemoteTrackingSnapshot {
    let mut snapshot = unavailable(repository, branch, error, fetched_at);
    snapshot.remote_head = head;
    snapshot.remote_health = "ERROR".into();
    snapshot
}

fn db_error(error: rusqlite::Error) -> String {
    format!("GitHub tracking cache database error: {error}")
}

#[cfg(not(test))]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubTrackingUpdateEvent {
    pub project_id: String,
    pub remote_head: Option<String>,
    pub remote_health: String,
    pub generated_at: String,
}

#[cfg(not(test))]
pub struct GitHubTrackingManager {
    database: DatabaseState,
    app_handle: tauri::AppHandle,
    selected_project: Arc<Mutex<Option<String>>>,
    refresh_requested: Arc<RefreshRequestGate>,
    stop: Arc<AtomicBool>,
    worker: Mutex<Option<thread::JoinHandle<()>>>,
}

#[cfg(not(test))]
impl GitHubTrackingManager {
    pub fn start(database: DatabaseState, app_handle: tauri::AppHandle) -> Result<Self, String> {
        ensure_portfolio(&database)?;
        let manager = Self {
            database,
            app_handle,
            selected_project: Arc::new(Mutex::new(None)),
            refresh_requested: Arc::new(RefreshRequestGate::default()),
            stop: Arc::new(AtomicBool::new(false)),
            worker: Mutex::new(None),
        };
        let database = manager.database.clone();
        let app_handle = manager.app_handle.clone();
        let selected_project = Arc::clone(&manager.selected_project);
        let refresh_requested = Arc::clone(&manager.refresh_requested);
        let stop = Arc::clone(&manager.stop);
        let worker = thread::Builder::new()
            .name("hiveai-github-tracking".into())
            .spawn(move || {
                polling_loop(
                    database,
                    app_handle,
                    selected_project,
                    refresh_requested,
                    stop,
                )
            })
            .map_err(|error| format!("start GitHub tracking scheduler: {error}"))?;
        *manager
            .worker
            .lock()
            .map_err(|_| "GitHub tracking worker lock poisoned")? = Some(worker);
        Ok(manager)
    }

    pub fn select_project(&self, project_id: Option<String>) {
        if let Ok(mut selected) = self.selected_project.lock() {
            *selected = project_id;
        }
        self.refresh_requested.request();
    }

    pub fn refresh_now(&self) -> Result<usize, String> {
        self.refresh_requested.request();
        Ok(0)
    }
}

#[cfg(not(test))]
impl Drop for GitHubTrackingManager {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Ok(mut worker) = self.worker.lock() {
            if let Some(handle) = worker.take() {
                let _ = handle.join();
            }
        }
    }
}

#[cfg(not(test))]
fn polling_loop(
    database: DatabaseState,
    app_handle: tauri::AppHandle,
    selected_project: Arc<Mutex<Option<String>>>,
    refresh_requested: Arc<RefreshRequestGate>,
    stop: Arc<AtomicBool>,
) {
    if let Err(error) = ensure_portfolio(&database) {
        log::error!("H!veAI GitHub portfolio reconciliation failed: {error}");
    } else {
        if let Ok(projects) = list_projects(
            &database,
            ProjectListQuery {
                include_archived: Some(false),
                ..Default::default()
            },
        ) {
            for project in projects {
                if let Err(error) =
                    crate::task_sources::reconcile_github_remote_sources(&database, &project.id)
                {
                    log::warn!(
                        "H!veAI GitHub source reconciliation skipped for {}: {error}",
                        project.id
                    );
                }
            }
        }
        let _ = app_handle.emit("hiveai-command-center-refresh", ());
    }
    let (job_tx, job_rx) = mpsc::channel::<(DatabaseState, ProjectRecord)>();
    let (result_tx, result_rx) = mpsc::channel::<(
        String,
        Result<(RemoteTrackingSnapshot, RemoteObservationChange), String>,
    )>();
    let shared_job_rx = Arc::new(Mutex::new(job_rx));
    let mut workers = Vec::new();
    for index in 0..4 {
        let worker_rx = Arc::clone(&shared_job_rx);
        let worker_tx = result_tx.clone();
        let worker_stop = Arc::clone(&stop);
        let worker = thread::Builder::new()
            .name(format!("hiveai-github-worker-{index}"))
            .spawn(move || {
                while !worker_stop.load(Ordering::Acquire) {
                    let job = worker_rx
                        .lock()
                        .ok()
                        .and_then(|receiver| receiver.recv_timeout(Duration::from_secs(1)).ok());
                    let Some((database, project)) = job else {
                        continue;
                    };
                    let project_id = project.id.clone();
                    let result = observe_project(&database, &project);
                    let _ = worker_tx.send((project_id, result));
                }
            })
            .expect("start GitHub tracking worker");
        workers.push(worker);
    }
    drop(result_tx);
    let mut next_due = HashMap::<String, std::time::Instant>::new();
    let mut failures = HashMap::<String, u32>::new();
    let mut signatures = HashMap::<String, (Option<String>, String)>::new();
    let mut in_flight = HashSet::<String>::new();
    while !stop.load(Ordering::Acquire) {
        while let Ok((project_id, result)) = result_rx.try_recv() {
            in_flight.remove(&project_id);
            match result {
                Ok((snapshot, _change)) => {
                    if snapshot.remote_health == "CURRENT" {
                        failures.remove(&project_id);
                    } else {
                        let failure = failures.entry(project_id.clone()).or_insert(0);
                        *failure = failure.saturating_add(1);
                        let multiplier = 2u64.saturating_pow((*failure).min(4));
                        let retry = Duration::from_secs(PORTFOLIO_REFRESH_SECONDS)
                            .checked_mul(multiplier as u32)
                            .unwrap_or(Duration::from_secs(300));
                        next_due.insert(
                            project_id.clone(),
                            std::time::Instant::now()
                                + std::cmp::min(retry, Duration::from_secs(300)),
                        );
                    }
                    let signature = (snapshot.remote_head.clone(), snapshot.remote_health.clone());
                    let should_emit = signatures.get(&project_id) != Some(&signature);
                    signatures.insert(project_id.clone(), signature);
                    if should_emit {
                        emit_update(&app_handle, &project_id, &snapshot);
                    }
                }
                Err(error) => {
                    let failure = failures.entry(project_id.clone()).or_insert(0);
                    *failure = failure.saturating_add(1);
                    let multiplier = 2u64.saturating_pow((*failure).min(4));
                    let next = Duration::from_secs(PORTFOLIO_REFRESH_SECONDS)
                        .checked_mul(multiplier as u32)
                        .unwrap_or(Duration::from_secs(300));
                    next_due.insert(
                        project_id.clone(),
                        std::time::Instant::now() + std::cmp::min(next, Duration::from_secs(300)),
                    );
                    log::warn!("GitHub tracking scheduler failed for {project_id}: {error}");
                }
            }
        }
        let projects = match list_projects(
            &database,
            ProjectListQuery {
                include_archived: Some(false),
                ..Default::default()
            },
        ) {
            Ok(projects) => projects,
            Err(error) => {
                log::warn!("GitHub tracking scheduler cannot list projects: {error}");
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };
        let selected = selected_project.lock().ok().and_then(|value| value.clone());
        let now = std::time::Instant::now();
        if refresh_requested.take() {
            for due in next_due.values_mut() {
                *due = now;
            }
        }
        let mut live_ids = std::collections::HashSet::new();
        for project in projects {
            if !is_github_tasks_project(&project) {
                continue;
            }
            live_ids.insert(project.id.clone());
            let interval = Duration::from_secs(refresh_interval_seconds(
                selected.as_deref() == Some(project.id.as_str()),
            ));
            let due = next_due.entry(project.id.clone()).or_insert(now);
            if *due > now {
                continue;
            }
            if in_flight.len() >= 4 || in_flight.contains(&project.id) {
                continue;
            }
            if job_tx.send((database.clone(), project.clone())).is_err() {
                break;
            }
            in_flight.insert(project.id.clone());
            *due = now + interval;
        }
        next_due.retain(|id, _| live_ids.contains(id));
        failures.retain(|id, _| live_ids.contains(id));
        signatures.retain(|id, _| live_ids.contains(id));
        thread::sleep(Duration::from_millis(100));
    }
    drop(job_tx);
    for worker in workers {
        let _ = worker.join();
    }
}

#[cfg(not(test))]
fn emit_update(app_handle: &tauri::AppHandle, project_id: &str, snapshot: &RemoteTrackingSnapshot) {
    let event = GitHubTrackingUpdateEvent {
        project_id: project_id.into(),
        remote_head: snapshot.remote_head.clone(),
        remote_health: snapshot.remote_health.clone(),
        generated_at: utc_timestamp(),
    };
    let _ = app_handle.emit("github-tracking-updated", &event);
    let _ = app_handle.emit("hiveai-command-center-refresh", &event);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DatabaseState;
    use crate::projects::{register_project, RegisterProjectRequest};
    use tempfile::tempdir;

    #[test]
    fn root_tasks_parser_materializes_current_state_and_exact_counts() {
        let raw = RootTasksRemote {
            head: "0123456789012345678901234567890123456789".into(),
            tasks: "# Demo\n\n## Project Status\n- Current Milestone: M2\n- Current Sprint: M2-S1\n- Current Task: TASK-2 — Current work\n- Current Task Status: IN_PROGRESS\n- Next Task/Action: TASK-3 — Next work\n- Required Actor: CODEX\n\n## Tasks\n- [x] TASK-1 — Done\n- [~] TASK-2 — Current work\n- [ ] TASK-3 — Next work\n".into(),
            tasks_blob_sha: "sha256:test".into(),
            latest_commit_message: Some("update tracker".into()),
            latest_commit_author: Some("owner".into()),
            latest_commit_at: Some("2026-09-10T00:00:00Z".into()),
        };
        let snapshot = parse_root_tasks(&raw, "Sekiph82/demo", "main", "now".into()).unwrap();
        assert_eq!(snapshot.current_task_id.as_deref(), Some("TASK-2"));
        assert_eq!(snapshot.current_task_status.as_deref(), Some("IN_PROGRESS"));
        assert_eq!(snapshot.next_task_id.as_deref(), Some("TASK-3"));
        assert_eq!(snapshot.completed_tasks, Some(1));
        assert_eq!(snapshot.total_tasks, Some(3));
        assert_eq!(snapshot.task_rows.len(), 3);
        assert_eq!(snapshot.task_rows[1].id, "TASK-2");
        assert_eq!(snapshot.task_rows[1].status, "IN_PROGRESS");
        assert_eq!(snapshot.task_rows[1].source_path, "TASKS.md");
        assert!((snapshot.progress_percent.unwrap() - 33.333333333333336).abs() < 0.0001);
        assert_eq!(
            snapshot.latest_commit_message.as_deref(),
            Some("update tracker")
        );
    }

    #[test]
    fn bulk_edit_blocked_owner_fixture_is_not_healthy() {
        let raw = RootTasksRemote {
            head: "0123456789012345678901234567890123456789".into(),
            tasks: "# Bulk-Edit\n\n## Project Status\n- Current Milestone: M13\n- Current Task: M13.03 — Real Etsy video upload architecture\n- Current Task Status: BLOCKED\n- Next Task/Action: Owner approval — Run live acceptance\n- Required Actor: OWNER\n\n## Tasks\n- [!] M13.03 — Real Etsy video upload architecture\n".into(),
            tasks_blob_sha: "sha256:test".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let snapshot = parse_root_tasks(&raw, "Sekiph82/Bulk-Edit", "main", "now".into()).unwrap();
        assert_eq!(remote_health(&snapshot), "BLOCKED");
    }

    #[test]
    fn scheduler_cadence_is_ten_seconds_selected_and_thirty_seconds_portfolio() {
        assert_eq!(
            refresh_interval_seconds(true),
            SELECTED_PROJECT_REFRESH_SECONDS
        );
        assert_eq!(refresh_interval_seconds(false), PORTFOLIO_REFRESH_SECONDS);
        assert_eq!(SELECTED_PROJECT_REFRESH_SECONDS, 10);
        assert_eq!(PORTFOLIO_REFRESH_SECONDS, 30);
    }

    #[test]
    fn manual_and_scheduled_refresh_requests_are_coalesced() {
        let gate = RefreshRequestGate::default();
        assert!(gate.request());
        assert!(!gate.request());
        assert!(gate.take());
        assert!(!gate.take());
    }

    #[test]
    fn ensure_portfolio_merges_legacy_repository_rows_into_one_remote_identity() {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        ensure_portfolio(&database).unwrap();

        let local_dir = tempdir().unwrap();
        let duplicate = register_project(
            &database,
            RegisterProjectRequest {
                path: local_dir.path().to_string_lossy().into_owned(),
                name: Some("legacy duplicate".into()),
            },
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "UPDATE repositories SET github_owner='Sekiph82', github_repo='Scrubbots', default_branch='main', remote_url='https://github.com/Sekiph82/Scrubbots.git', is_git_repository=1 WHERE project_id=?1",
                [duplicate.id.as_str()],
            )
            .unwrap();

        ensure_portfolio(&database).unwrap();
        let projects = list_projects(
            &database,
            ProjectListQuery {
                include_archived: Some(false),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(projects.len(), 8);
        assert!(!projects.iter().any(|project| project.id == duplicate.id));
        assert!(!projects.iter().any(|project| {
            project
                .repository
                .as_ref()
                .and_then(|repository| repository.github_repo.as_deref())
                == Some("AI-Commerce-HQ")
        }));
        let scrubbots = projects
            .iter()
            .find(|project| project.id == "github:Sekiph82/Scrubbots@main")
            .unwrap();
        assert_eq!(
            scrubbots.task_source_policy.as_deref(),
            Some(GITHUB_TASKS_ONLY_POLICY)
        );
        assert_eq!(scrubbots.normalized_path, duplicate.normalized_path);
    }

    #[test]
    fn ensure_portfolio_archives_non_portfolio_persisted_rows() {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        ensure_portfolio(&database).unwrap();

        let local_dir = tempdir().unwrap();
        let extra = register_project(
            &database,
            RegisterProjectRequest {
                path: local_dir.path().to_string_lossy().into_owned(),
                name: Some("stale persisted project".into()),
            },
        )
        .unwrap();
        assert_eq!(
            list_projects(
                &database,
                ProjectListQuery {
                    include_archived: Some(false),
                    ..Default::default()
                },
            )
            .unwrap()
            .len(),
            9
        );
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO github_sync_state (id, project_id, resource_kind, resource_cursor, last_synced_at, metadata_json) VALUES ('legacy-cache', 'github:Sekiph82/H-veAI@main', 'GITHUB_TRACKING_V3', 'legacy', 'now', '{}')",
                [],
            )
            .unwrap();

        ensure_portfolio(&database).unwrap();
        let active = list_projects(
            &database,
            ProjectListQuery {
                include_archived: Some(false),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(active.len(), 8);
        assert!(!active.iter().any(|project| project.id == extra.id));
        assert!(!active.iter().any(|project| {
            project
                .repository
                .as_ref()
                .and_then(|repository| repository.github_repo.as_deref())
                == Some("AI-Commerce-HQ")
        }));
        assert_eq!(
            crate::projects::fetch_project(&database, &extra.id)
                .unwrap()
                .status,
            "ARCHIVED"
        );
        assert_eq!(
            database
                .open_connection()
                .unwrap()
                .query_row(
                    "SELECT COUNT(*) FROM github_sync_state WHERE resource_kind='GITHUB_TRACKING_V3'",
                    [],
                    |row| row.get::<_, i64>(0),
                )
                .unwrap(),
            0
        );
    }
}
