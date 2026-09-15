use crate::db::DatabaseState;
use crate::git_engine::{self, GitSnapshot, RepositoryHealth};
use crate::projects::ProjectRecord;
use crate::time::utc_timestamp;
use chrono::DateTime;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use serde_json::json;
use serde_json::Value;
#[cfg(not(test))]
use std::io::Read;
#[cfg(not(test))]
use std::process::Stdio;
#[cfg(not(test))]
use std::thread;
use std::time::Duration;
#[cfg(not(test))]
use std::time::Instant;

const CACHE_SCHEMA_VERSION: u32 = 1;
const MAX_BRANCHES: usize = 25;
const MAX_COMMITS: usize = 25;
const MAX_PULL_REQUESTS: usize = 10;
const MAX_PR_ENRICHED: usize = 5;
const MAX_PR_FILES: usize = 25;
const MAX_PR_REVIEWS: usize = 10;
const MAX_PR_CHECKS: usize = 20;
const MAX_ISSUES: usize = 20;
const MAX_ACTION_RUNS: usize = 20;
const MAX_ACTION_RUNS_ENRICHED: usize = 5;
const MAX_ACTION_JOBS: usize = 20;
const MAX_ACTION_STEPS: usize = 25;
const MAX_ACTION_LOG_CHARS: usize = 512;
const MAX_SUBRESOURCE_REQUESTS: usize = 80;
const MAX_RELEASES: usize = 10;
const MAX_TAGS: usize = 25;
const MAX_COMMENTS: usize = 10;
const MAX_BODY_CHARS: usize = 512;
const MAX_RESPONSE_BYTES: usize = 512 * 1024;
const CACHE_MAX_AGE_SECONDS: i64 = 30;
const HTTP_TIMEOUT: Duration = Duration::from_secs(20);
const HTTP_STATUS_MARKER: &str = "__HIVEAI_HTTP_STATUS__";

const RESOURCE_REPOSITORY: &str = "GITHUB_REPOSITORY";
const RESOURCE_BRANCHES: &str = "GITHUB_BRANCHES";
const RESOURCE_COMMITS: &str = "GITHUB_COMMITS";
const RESOURCE_PULL_REQUESTS: &str = "GITHUB_PULL_REQUESTS";
const RESOURCE_ISSUES: &str = "GITHUB_ISSUES";
const RESOURCE_ACTIONS: &str = "GITHUB_ACTIONS";
const RESOURCE_RELEASES: &str = "GITHUB_RELEASES";
const RESOURCE_TAGS: &str = "GITHUB_TAGS";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubIntegrationSnapshot {
    pub project_id: String,
    pub repository: GitHubRepository,
    pub branches: Vec<GitHubBranch>,
    pub commits: Vec<GitHubCommit>,
    pub pull_requests: Vec<GitHubPullRequest>,
    pub issues: Vec<GitHubIssue>,
    pub actions: Vec<GitHubActionRun>,
    pub releases: Vec<GitHubRelease>,
    pub tags: Vec<GitHubTag>,
    pub local: LocalGitHubEvidence,
    pub reconciliation: GitHubReconciliation,
    pub remote_health: String,
    pub fetched_at: String,
    pub cache: GitHubCacheStatus,
    pub mutation_policy: GitHubMutationPolicy,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepository {
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub default_branch: String,
    pub tracked_branch: String,
    pub remote_head: Option<String>,
    pub private: Option<bool>,
    pub archived: Option<bool>,
    pub html_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubBranch {
    pub name: String,
    pub sha: Option<String>,
    pub protected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubCommit {
    pub sha: String,
    pub message: String,
    pub author: Option<String>,
    pub authored_at: Option<String>,
    pub committed_at: Option<String>,
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubPullRequest {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub draft: Option<bool>,
    pub merged: bool,
    pub author: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub source_branch: Option<String>,
    pub target_branch: Option<String>,
    pub head_sha: Option<String>,
    pub base_sha: Option<String>,
    pub html_url: Option<String>,
    pub changed_files: Option<u64>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
    pub review_status: Option<String>,
    pub check_status: Option<String>,
    pub detail_state: String,
    pub files_state: String,
    pub reviews_state: String,
    pub comments_state: String,
    pub checks_state: String,
    pub files: Vec<GitHubPullRequestFile>,
    pub reviews: Vec<GitHubPullRequestReview>,
    pub checks: Vec<GitHubCheck>,
    pub comments: Vec<String>,
    pub task_links: Vec<String>,
    pub session_links: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubPullRequestFile {
    pub filename: String,
    pub status: Option<String>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
    pub changes: Option<u64>,
    pub patch_excerpt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubPullRequestReview {
    pub id: u64,
    pub user: Option<String>,
    pub state: Option<String>,
    pub submitted_at: Option<String>,
    pub body_excerpt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubCheck {
    pub id: u64,
    pub name: Option<String>,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub details_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubIssue {
    pub number: u64,
    pub title: String,
    pub state: String,
    pub labels: Vec<String>,
    pub author: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub html_url: Option<String>,
    pub body_excerpt: Option<String>,
    pub comments: Vec<String>,
    pub task_links: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubActionRun {
    pub id: u64,
    pub name: Option<String>,
    pub event: Option<String>,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub branch: Option<String>,
    pub head_sha: Option<String>,
    pub pull_request_numbers: Vec<u64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub failed_log_summary: Option<String>,
    pub jobs_state: String,
    pub logs_state: String,
    pub jobs: Vec<GitHubActionJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubActionJob {
    pub id: u64,
    pub name: Option<String>,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub steps: Vec<GitHubActionStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubActionStep {
    pub name: Option<String>,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub number: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRelease {
    pub id: u64,
    pub name: Option<String>,
    pub tag_name: Option<String>,
    pub target_commitish: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub published_at: Option<String>,
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubTag {
    pub name: String,
    pub commit_sha: Option<String>,
    pub protected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalGitHubEvidence {
    pub available: bool,
    pub branch: Option<String>,
    pub head_sha: Option<String>,
    pub upstream: Option<String>,
    pub ahead_count: Option<u64>,
    pub behind_count: Option<u64>,
    pub detached: bool,
    pub dirty: bool,
    pub health: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GitHubReconciliation {
    pub state: String,
    pub local_branch: Option<String>,
    pub local_head: Option<String>,
    pub remote_branch: String,
    pub remote_head: Option<String>,
    pub local_dirty: bool,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubCacheStatus {
    pub schema_version: u32,
    pub state: String,
    pub fetched_at: Option<String>,
    pub last_known_good_at: Option<String>,
    pub age_seconds: Option<i64>,
    pub provenance: String,
    pub resources: Vec<GitHubResourceCacheStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubResourceCacheStatus {
    pub kind: String,
    pub state: String,
    pub fetched_at: Option<String>,
    pub last_known_good_at: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubMutationPolicy {
    pub remote_mutations: String,
    pub pull_request_creation: String,
    pub workflow_retry: String,
    pub local_git_mutations: String,
    pub confirmation_required: bool,
}

#[derive(Debug, Clone)]
struct Identity {
    owner: String,
    repo: String,
    branch: String,
    full_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEnvelope {
    schema_version: u32,
    resource_kind: String,
    repository: String,
    branch: String,
    fetched_at: String,
    last_known_good_at: String,
    payload: Value,
}

#[derive(Debug, Clone)]
struct ResourceResult {
    kind: String,
    value: Option<Value>,
    state: String,
    fetched_at: Option<String>,
    last_known_good_at: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
struct ReadResponse {
    status: u16,
    body: String,
}

trait GitHubReadTransport {
    fn get(&mut self, url: &str) -> Result<ReadResponse, String>;
}

struct RequestBudget {
    remaining: usize,
}

impl RequestBudget {
    fn new() -> Self {
        Self {
            remaining: MAX_SUBRESOURCE_REQUESTS,
        }
    }

    fn take(&mut self) -> bool {
        if self.remaining == 0 {
            return false;
        }
        self.remaining -= 1;
        true
    }
}

#[cfg(test)]
#[derive(Default)]
struct FixtureTransport {
    requests: Vec<String>,
    responses: Vec<(String, ReadResponse)>,
}

#[cfg(test)]
impl FixtureTransport {
    fn response(mut self, url: &str, status: u16, body: &str) -> Self {
        self.responses.push((
            url.into(),
            ReadResponse {
                status,
                body: body.into(),
            },
        ));
        self
    }
}

#[cfg(test)]
impl GitHubReadTransport for FixtureTransport {
    fn get(&mut self, url: &str) -> Result<ReadResponse, String> {
        self.requests.push(url.into());
        if let Some((_, response)) = self.responses.iter().find(|(path, _)| path == url) {
            return Ok(response.clone());
        }
        let body = if url.ends_with("/actions/runs") || url.contains("/actions/runs?") {
            r#"{"workflow_runs":[]}"#
        } else if url.contains("/repos/") && !url.contains("/branches") && !url.contains("/commits") {
            r#"{}"#
        } else {
            "[]"
        };
        Ok(ReadResponse { status: 200, body: body.into() })
    }
}

pub fn snapshot(
    database: &DatabaseState,
    project: &ProjectRecord,
) -> Result<GitHubIntegrationSnapshot, String> {
    #[cfg(test)]
    let mut transport = FixtureTransport::default();
    #[cfg(not(test))]
    let mut transport = CurlTransport;
    snapshot_with_transport(database, project, &mut transport)
}

fn snapshot_with_transport<T: GitHubReadTransport>(
    database: &DatabaseState,
    project: &ProjectRecord,
    transport: &mut T,
) -> Result<GitHubIntegrationSnapshot, String> {
    let identity = identity_from_project(project).map(identity_from_view)?;
    let now = utc_timestamp();
    let resources = fetch_resources_with_transport(
        database,
        project,
        &identity,
        &now,
        transport,
    );

    let repository_result = resources.iter().find(|r| r.kind == RESOURCE_REPOSITORY);
    let repository_value = repository_result.and_then(|r| r.value.as_ref());
    let branches_value = resource_value(&resources, RESOURCE_BRANCHES);
    let commits_value = resource_value(&resources, RESOURCE_COMMITS);
    let prs_value = resource_value(&resources, RESOURCE_PULL_REQUESTS);
    let issues_value = resource_value(&resources, RESOURCE_ISSUES);
    let actions_value = resource_value(&resources, RESOURCE_ACTIONS);
    let releases_value = resource_value(&resources, RESOURCE_RELEASES);
    let tags_value = resource_value(&resources, RESOURCE_TAGS);

    let mut repository = parse_repository(repository_value, &identity);
    let branches = parse_branches(branches_value);
    let commits = parse_commits(commits_value);
    let mut pull_requests = parse_pull_requests(prs_value);
    enrich_pull_requests(&mut pull_requests, &resources);
    let issues = parse_issues(issues_value);
    let mut actions = parse_actions(actions_value);
    enrich_actions(&mut actions, &resources);
    let releases = parse_releases(releases_value);
    let tags = parse_tags(tags_value);
    repository.remote_head = branches
        .iter()
        .find(|branch| branch.name == identity.branch)
        .and_then(|branch| branch.sha.clone())
        .or_else(|| commits.first().map(|commit| commit.sha.clone()));

    let local = local_evidence(database, project);
    let remote_health = overall_health(&resources);
    let reconciliation = reconcile(
        &local,
        &identity.branch,
        repository.remote_head.as_deref(),
        &remote_health,
    );
    let cache = cache_status(&resources, &now);
    let mut warnings = resources
        .iter()
        .filter_map(|resource| resource.error.clone())
        .collect::<Vec<_>>();
    if local.error.is_some() {
        warnings.push(
            "Local Git Engine evidence is unavailable; remote integration remains observational."
                .into(),
        );
    }
    if remote_health != "CURRENT" {
        warnings.push(format!(
            "GitHub integration is {remote_health}; cached data is never presented as current."
        ));
    }
    warnings.truncate(32);

    // These collections are deliberately bounded again at the product boundary,
    // even if a future transport/parser changes its fixture input.
    pull_requests.truncate(MAX_PULL_REQUESTS);
    let mutation_policy = GitHubMutationPolicy {
        remote_mutations: "DENIED_BY_DEFAULT".into(),
        pull_request_creation: "UNAVAILABLE_AUTH_BOUNDARY".into(),
        workflow_retry: "UNAVAILABLE_AUTH_BOUNDARY".into(),
        local_git_mutations: "NOT_PERFORMED".into(),
        confirmation_required: true,
    };
    Ok(GitHubIntegrationSnapshot {
        project_id: project.id.clone(),
        repository,
        branches,
        commits,
        pull_requests,
        issues,
        actions,
        releases,
        tags,
        local,
        reconciliation,
        remote_health,
        fetched_at: now,
        cache,
        mutation_policy,
        warnings,
    })
}

pub fn identity_from_project(project: &ProjectRecord) -> Result<IdentityView, String> {
    let repository = project
        .repository
        .as_ref()
        .ok_or_else(|| "GITHUB_REPOSITORY_IDENTITY_UNAVAILABLE".to_string())?;
    let owner = repository
        .github_owner
        .as_deref()
        .ok_or_else(|| "GITHUB_REPOSITORY_OWNER_UNAVAILABLE".to_string())?;
    let repo = repository
        .github_repo
        .as_deref()
        .ok_or_else(|| "GITHUB_REPOSITORY_NAME_UNAVAILABLE".to_string())?;
    let branch = repository
        .default_branch
        .as_deref()
        .or(repository.current_branch.as_deref())
        .ok_or_else(|| "GITHUB_TRACKED_BRANCH_UNAVAILABLE".to_string())?;
    validate_segment(owner, "owner")?;
    validate_segment(repo, "repository")?;
    validate_branch(branch)?;
    Ok(IdentityView {
        owner: owner.into(),
        repo: repo.into(),
        branch: branch.into(),
        full_name: format!("{owner}/{repo}"),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IdentityView {
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub full_name: String,
}

fn identity_from_view(view: IdentityView) -> Identity {
    Identity {
        owner: view.owner,
        repo: view.repo,
        branch: view.branch,
        full_name: view.full_name,
    }
}

fn validate_segment(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 100
        || value.contains("..")
        || !value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Err(format!(
            "GITHUB_{}_IDENTITY_INVALID",
            label.to_ascii_uppercase()
        ));
    }
    Ok(())
}

fn validate_branch(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 256
        || value.contains("..")
        || value
            .chars()
            .any(|c| c.is_ascii_control() || matches!(c, ':' | '?' | '#' | '\\'))
    {
        return Err("GITHUB_BRANCH_IDENTITY_INVALID".into());
    }
    Ok(())
}

fn resource_value<'a>(resources: &'a [ResourceResult], kind: &str) -> Option<&'a Value> {
    resources
        .iter()
        .find(|resource| resource.kind == kind)
        .and_then(|resource| resource.value.as_ref())
}

fn fetch_resources_with_transport<T: GitHubReadTransport>(
    database: &DatabaseState,
    project: &ProjectRecord,
    identity: &Identity,
    now: &str,
    transport: &mut T,
) -> Vec<ResourceResult> {
    let repository_path = format!(
        "https://api.github.com/repos/{}/{}",
        identity.owner, identity.repo
    );
    let branch_query = percent_encode(&identity.branch);
    let specs = [
        (RESOURCE_REPOSITORY, repository_path.clone()),
        (
            RESOURCE_BRANCHES,
            format!("{repository_path}/branches?per_page={MAX_BRANCHES}"),
        ),
        (
            RESOURCE_COMMITS,
            format!("{repository_path}/commits?sha={branch_query}&per_page={MAX_COMMITS}"),
        ),
        (
            RESOURCE_PULL_REQUESTS,
            format!("{repository_path}/pulls?state=all&per_page={MAX_PULL_REQUESTS}"),
        ),
        (
            RESOURCE_ISSUES,
            format!("{repository_path}/issues?state=all&per_page={MAX_ISSUES}"),
        ),
        (
            RESOURCE_ACTIONS,
            format!("{repository_path}/actions/runs?per_page={MAX_ACTION_RUNS}"),
        ),
        (
            RESOURCE_RELEASES,
            format!("{repository_path}/releases?per_page={MAX_RELEASES}"),
        ),
        (
            RESOURCE_TAGS,
            format!("{repository_path}/tags?per_page={MAX_TAGS}"),
        ),
    ];
    let mut resources = specs
        .into_iter()
        .map(|(kind, url)| {
            load_or_fetch(
                database,
                project,
                &identity.full_name,
                &identity.branch,
                kind,
                &url,
                now,
                transport,
            )
        })
        .collect::<Vec<_>>();
    let mut budget = RequestBudget::new();
    let selected_prs = parse_pull_requests(resource_value(&resources, RESOURCE_PULL_REQUESTS));
    for pr in selected_prs.iter().take(MAX_PR_ENRICHED) {
        let base = format!("{repository_path}/pulls/{}", pr.number);
        push_enrichment(
            &mut resources,
            load_or_fetch(
                database,
                project,
                &identity.full_name,
                &identity.branch,
                &format!("GITHUB_PR_{}_DETAIL", pr.number),
                &base,
                now,
                transport,
            ),
            &mut budget,
        );
        push_enrichment(
            &mut resources,
            load_or_fetch(
                database,
                project,
                &identity.full_name,
                &identity.branch,
                &format!("GITHUB_PR_{}_FILES", pr.number),
                &format!("{base}/files?per_page={MAX_PR_FILES}"),
                now,
                transport,
            ),
            &mut budget,
        );
        push_enrichment(
            &mut resources,
            load_or_fetch(
                database,
                project,
                &identity.full_name,
                &identity.branch,
                &format!("GITHUB_PR_{}_REVIEWS", pr.number),
                &format!("{base}/reviews?per_page={MAX_PR_REVIEWS}"),
                now,
                transport,
            ),
            &mut budget,
        );
        push_enrichment(
            &mut resources,
            load_or_fetch(
                database,
                project,
                &identity.full_name,
                &identity.branch,
                &format!("GITHUB_PR_{}_COMMENTS", pr.number),
                &format!("{repository_path}/issues/{}/comments?per_page={MAX_COMMENTS}", pr.number),
                now,
                transport,
            ),
            &mut budget,
        );
        push_enrichment(
            &mut resources,
            load_or_fetch(
                database,
                project,
                &identity.full_name,
                &identity.branch,
                &format!("GITHUB_PR_{}_REVIEW_COMMENTS", pr.number),
                &format!("{base}/comments?per_page={MAX_COMMENTS}"),
                now,
                transport,
            ),
            &mut budget,
        );
        if let Some(head_sha) = pr.head_sha.as_deref() {
            push_enrichment(
                &mut resources,
                load_or_fetch(
                    database,
                    project,
                    &identity.full_name,
                    &identity.branch,
                    &format!("GITHUB_PR_{}_CHECKS", pr.number),
                    &format!("{repository_path}/commits/{head_sha}/check-runs?per_page={MAX_PR_CHECKS}"),
                    now,
                    transport,
                ),
                &mut budget,
            );
            push_enrichment(
                &mut resources,
                load_or_fetch(
                    database,
                    project,
                    &identity.full_name,
                    &identity.branch,
                    &format!("GITHUB_PR_{}_STATUS", pr.number),
                    &format!("{repository_path}/commits/{head_sha}/status?per_page={MAX_PR_CHECKS}"),
                    now,
                    transport,
                ),
                &mut budget,
            );
        }
    }
    let selected_runs = parse_actions(resource_value(&resources, RESOURCE_ACTIONS));
    for run in selected_runs.iter().take(MAX_ACTION_RUNS_ENRICHED) {
        let kind = format!("GITHUB_ACTION_{}_JOBS", run.id);
        let jobs = load_or_fetch(
            database,
            project,
            &identity.full_name,
            &identity.branch,
            &kind,
            &format!("{repository_path}/actions/runs/{}/jobs?per_page={MAX_ACTION_JOBS}", run.id),
            now,
            transport,
        );
        let jobs_for_logs = jobs.value.clone();
        push_enrichment(&mut resources, jobs, &mut budget);
        for job_id in parse_job_ids(jobs_for_logs).into_iter().take(2) {
            if !run.conclusion.as_deref().is_some_and(|value| {
                matches!(value, "FAILURE" | "CANCELLED" | "TIMED_OUT" | "ACTION_REQUIRED")
            }) {
                break;
            }
            let kind = format!("GITHUB_ACTION_JOB_{}_LOG", job_id);
            push_enrichment_text(
                &mut resources,
                load_or_fetch_text(
                    database,
                    project,
                    &identity.full_name,
                    &identity.branch,
                    &kind,
                    &format!("{repository_path}/actions/jobs/{job_id}/logs"),
                    now,
                    transport,
                ),
                &mut budget,
            );
        }
    }
    resources
}

fn push_enrichment(resources: &mut Vec<ResourceResult>, result: ResourceResult, budget: &mut RequestBudget) {
    if budget.take() {
        resources.push(result);
    } else {
        resources.push(ResourceResult {
            kind: result.kind,
            value: None,
            state: "UNAVAILABLE".into(),
            fetched_at: None,
            last_known_good_at: None,
            error: Some("GITHUB_SUBRESOURCE_REQUEST_BOUND".into()),
        });
    }
}

fn push_enrichment_text(resources: &mut Vec<ResourceResult>, result: ResourceResult, budget: &mut RequestBudget) {
    push_enrichment(resources, result, budget);
}

fn parse_job_ids(value: Option<Value>) -> Vec<u64> {
    let Some(value) = value else { return Vec::new() };
    let jobs = value
        .get("jobs")
        .and_then(Value::as_array)
        .or_else(|| value.as_array())
        .map(|items| items.to_vec())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|job| job.get("id").and_then(Value::as_u64))
        .take(MAX_ACTION_JOBS)
        .collect();
    jobs
}

fn load_or_fetch<T: GitHubReadTransport>(
    database: &DatabaseState,
    project: &ProjectRecord,
    repository: &str,
    branch: &str,
    kind: &str,
    url: &str,
    now: &str,
    transport: &mut T,
) -> ResourceResult {
    if let Ok(Some(cache)) = load_cache(database, &project.id, kind, repository, branch) {
        if cache_is_fresh(&cache.fetched_at, now) {
            return ResourceResult {
                kind: kind.into(),
                value: Some(cache.payload),
                state: "CURRENT".into(),
                fetched_at: Some(cache.fetched_at),
                last_known_good_at: Some(cache.last_known_good_at),
                error: None,
            };
        }
    }
    match request_json(transport, url) {
        Ok(value) => {
            let cache = CacheEnvelope {
                schema_version: CACHE_SCHEMA_VERSION,
                resource_kind: kind.into(),
                repository: repository.into(),
                branch: branch.into(),
                fetched_at: now.into(),
                last_known_good_at: now.into(),
                payload: sanitize_value(&value, None),
            };
            let persist_error = persist_cache(database, &project.id, &cache).err();
            ResourceResult {
                kind: kind.into(),
                value: Some(cache.payload.clone()),
                state: "CURRENT".into(),
                fetched_at: Some(now.into()),
                last_known_good_at: Some(now.into()),
                error: persist_error,
            }
        }
        Err(error) => match load_cache(database, &project.id, kind, repository, branch) {
            Ok(Some(cache)) => ResourceResult {
                kind: kind.into(),
                value: Some(cache.payload),
                state: classify_failure(&error),
                fetched_at: Some(cache.fetched_at),
                last_known_good_at: Some(cache.last_known_good_at),
                error: Some(error),
            },
            _ => ResourceResult {
                kind: kind.into(),
                value: None,
                state: classify_failure(&error),
                fetched_at: None,
                last_known_good_at: None,
                error: Some(error),
            },
        },
    }
}

fn load_or_fetch_text<T: GitHubReadTransport>(
    database: &DatabaseState,
    project: &ProjectRecord,
    repository: &str,
    branch: &str,
    kind: &str,
    url: &str,
    now: &str,
    transport: &mut T,
) -> ResourceResult {
    if let Ok(Some(cache)) = load_cache(database, &project.id, kind, repository, branch) {
        if cache_is_fresh(&cache.fetched_at, now) {
            return ResourceResult {
                kind: kind.into(),
                value: Some(cache.payload),
                state: "CURRENT".into(),
                fetched_at: Some(cache.fetched_at),
                last_known_good_at: Some(cache.last_known_good_at),
                error: None,
            };
        }
    }
    match transport.get(url).and_then(|response| {
        if (200..300).contains(&response.status) {
            Ok(sanitize_value(&Value::String(response.body), None))
        } else {
            Err(format!("GITHUB_HTTP_REMOTE_UNAVAILABLE: HTTP {}", response.status))
        }
    }) {
        Ok(value) => {
            let cache = CacheEnvelope {
                schema_version: CACHE_SCHEMA_VERSION,
                resource_kind: kind.into(),
                repository: repository.into(),
                branch: branch.into(),
                fetched_at: now.into(),
                last_known_good_at: now.into(),
                payload: value.clone(),
            };
            let persist_error = persist_cache(database, &project.id, &cache).err();
            ResourceResult {
                kind: kind.into(), value: Some(value), state: "CURRENT".into(),
                fetched_at: Some(now.into()), last_known_good_at: Some(now.into()), error: persist_error,
            }
        }
        Err(error) => match load_cache(database, &project.id, kind, repository, branch) {
            Ok(Some(cache)) => ResourceResult {
                kind: kind.into(), value: Some(cache.payload), state: classify_failure(&error),
                fetched_at: Some(cache.fetched_at), last_known_good_at: Some(cache.last_known_good_at), error: Some(sanitize_error(&error)),
            },
            _ => ResourceResult { kind: kind.into(), value: None, state: classify_failure(&error), fetched_at: None, last_known_good_at: None, error: Some(sanitize_error(&error)) },
        },
    }
}

fn persist_cache(
    database: &DatabaseState,
    project_id: &str,
    cache: &CacheEnvelope,
) -> Result<(), String> {
    let connection = database.open_connection()?;
    let metadata = serde_json::to_string(cache)
        .map_err(|error| format!("GITHUB_CACHE_SERIALIZE_FAILED: {error}"))?;
    connection
        .execute(
            "INSERT INTO github_sync_state (id, project_id, resource_kind, resource_cursor, last_synced_at, metadata_json) VALUES (?1, ?2, ?3, ?4, ?5, ?6) ON CONFLICT(project_id, resource_kind) DO UPDATE SET resource_cursor=excluded.resource_cursor, last_synced_at=excluded.last_synced_at, metadata_json=excluded.metadata_json",
            params![format!("{}:{}", project_id, cache.resource_kind), project_id, cache.resource_kind, cache.branch, cache.fetched_at, metadata],
        )
        .map_err(|error| format!("GITHUB_CACHE_PERSIST_FAILED: {error}"))?;
    Ok(())
}

fn load_cache(
    database: &DatabaseState,
    project_id: &str,
    kind: &str,
    repository: &str,
    branch: &str,
) -> Result<Option<CacheEnvelope>, String> {
    let connection = database.open_connection()?;
    let value: Option<String> = connection
        .query_row(
            "SELECT metadata_json FROM github_sync_state WHERE project_id=?1 AND resource_kind=?2",
            params![project_id, kind],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| format!("GITHUB_CACHE_READ_FAILED: {error}"))?;
    let Some(value) = value else { return Ok(None) };
    let cache = serde_json::from_str::<CacheEnvelope>(&value)
        .map_err(|_| "GITHUB_CACHE_MALFORMED".to_string())?;
    if cache.schema_version != CACHE_SCHEMA_VERSION
        || cache.repository != repository
        || cache.branch != branch
        || cache.resource_kind != kind
    {
        return Ok(None);
    }
    if sanitize_value(&cache.payload, None) != cache.payload {
        return Ok(None);
    }
    Ok(Some(cache))
}

fn cache_is_fresh(fetched_at: &str, now: &str) -> bool {
    let Ok(fetched) = DateTime::parse_from_rfc3339(fetched_at) else {
        return false;
    };
    let Ok(current) = DateTime::parse_from_rfc3339(now) else {
        return false;
    };
    (current - fetched).num_seconds().unsigned_abs() as i64 <= CACHE_MAX_AGE_SECONDS
}

fn cache_status(resources: &[ResourceResult], now: &str) -> GitHubCacheStatus {
    let fetched_at = resources.iter().filter_map(|r| r.fetched_at.clone()).max();
    let last_known_good_at = resources
        .iter()
        .filter_map(|r| r.last_known_good_at.clone())
        .max();
    let age_seconds = last_known_good_at.as_deref().and_then(|value| {
        let fetched = DateTime::parse_from_rfc3339(value).ok()?;
        let current = DateTime::parse_from_rfc3339(now).ok()?;
        Some((current - fetched).num_seconds().max(0))
    });
    let state = if resources.iter().any(|r| r.state == "CURRENT")
        && resources.iter().all(|r| r.state == "CURRENT")
    {
        "CURRENT"
    } else if resources.iter().any(|r| r.last_known_good_at.is_some()) {
        "STALE"
    } else {
        "EMPTY"
    };
    GitHubCacheStatus {
        schema_version: CACHE_SCHEMA_VERSION,
        state: state.into(),
        fetched_at,
        last_known_good_at,
        age_seconds,
        provenance: "GitHub API resource cache; registry identity scoped".into(),
        resources: resources
            .iter()
            .map(|r| GitHubResourceCacheStatus {
                kind: r.kind.clone(),
                state: r.state.clone(),
                fetched_at: r.fetched_at.clone(),
                last_known_good_at: r.last_known_good_at.clone(),
                error: r.error.clone(),
            })
            .collect(),
    }
}

fn overall_health(resources: &[ResourceResult]) -> String {
    for status in [
        "RATE_LIMITED",
        "AUTH_REQUIRED",
        "TIMEOUT",
        "OFFLINE",
        "MALFORMED",
        "UNAVAILABLE",
    ] {
        if resources.iter().any(|r| r.state == status) {
            return status.into();
        }
    }
    if resources.iter().any(|r| r.state == "STALE") {
        "STALE".into()
    } else {
        "CURRENT".into()
    }
}

fn classify_failure(error: &str) -> String {
    let upper = error.to_ascii_uppercase();
    if upper.contains("RATE_LIMIT") || upper.contains("HTTP_403") && upper.contains("API RATE") {
        "RATE_LIMITED".into()
    } else if upper.contains("HTTP_401") || upper.contains("AUTH") {
        "AUTH_REQUIRED".into()
    } else if upper.contains("TIMEOUT") {
        "TIMEOUT".into()
    } else if upper.contains("MALFORMED") {
        "MALFORMED".into()
    } else if upper.contains("NETWORK")
        || upper.contains("COULD NOT RESOLVE")
        || upper.contains("FAILED TO CONNECT")
    {
        "OFFLINE".into()
    } else {
        "UNAVAILABLE".into()
    }
}

fn request_json<T: GitHubReadTransport>(transport: &mut T, url: &str) -> Result<Value, String> {
    let response = transport.get(url)?;
    let status = response.status;
    let body = response.body;
    if !(200..300).contains(&status) {
        let body_hint = sanitize_error(&body);
        let category = if status == 401 {
            "GITHUB_HTTP_401_AUTH_REQUIRED"
        } else if status == 403 && body_hint.to_ascii_uppercase().contains("RATE LIMIT") {
            "GITHUB_HTTP_403_RATE_LIMITED"
        } else {
            "GITHUB_HTTP_REMOTE_UNAVAILABLE"
        };
        return Err(format!(
            "{category}: HTTP {status}{}",
            if body_hint.is_empty() {
                String::new()
            } else {
                format!(" — {body_hint}")
            }
        ));
    }
    serde_json::from_str(&body).map_err(|_| "GITHUB_MALFORMED_RESPONSE".into())
}

#[cfg(not(test))]
struct CurlTransport;

#[cfg(not(test))]
impl GitHubReadTransport for CurlTransport {
    fn get(&mut self, url: &str) -> Result<ReadResponse, String> {
        let (status, body) = run_http(url)?;
        Ok(ReadResponse { status, body })
    }
}

#[cfg(not(test))]
fn run_http(url: &str) -> Result<(u16, String), String> {
    let mut child = crate::process_policy::background_command("curl.exe")
        .args([
            "-L",
            "--silent",
            "--show-error",
            "--connect-timeout",
            "5",
            "--max-time",
            "15",
            "--header",
            "Accept: application/vnd.github+json",
            "--header",
            "X-GitHub-Api-Version: 2022-11-28",
            "--header",
            "User-Agent: H-veAI-GitHub-Integration",
            "--write-out",
            &format!("\n{HTTP_STATUS_MARKER}%{{http_code}}"),
            url,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("GITHUB_NETWORK_START_FAILED: {error}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "GITHUB_NETWORK_STDOUT_UNAVAILABLE".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "GITHUB_NETWORK_STDERR_UNAVAILABLE".to_string())?;
    let stdout_reader = thread::spawn(move || read_bounded(stdout, MAX_RESPONSE_BYTES));
    let stderr_reader = thread::spawn(move || read_bounded(stderr, 16 * 1024));
    let deadline = Instant::now() + HTTP_TIMEOUT;
    let status = loop {
        match child
            .try_wait()
            .map_err(|error| format!("GITHUB_NETWORK_WAIT_FAILED: {error}"))?
        {
            Some(status) => break status,
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Err("GITHUB_NETWORK_TIMEOUT".into());
            }
            None => thread::sleep(Duration::from_millis(50)),
        }
    };
    let (stdout, stdout_truncated) = stdout_reader
        .join()
        .map_err(|_| "GITHUB_NETWORK_STDOUT_READER_FAILED".to_string())?;
    let (stderr, _) = stderr_reader
        .join()
        .map_err(|_| "GITHUB_NETWORK_STDERR_READER_FAILED".to_string())?;
    if !status.success() {
        return Err(format!(
            "GITHUB_NETWORK_REQUEST_FAILED: {}",
            sanitize_error(&String::from_utf8_lossy(&stderr))
        ));
    }
    if stdout_truncated {
        return Err("GITHUB_MALFORMED_RESPONSE: response exceeded bounded size".into());
    }
    let output = String::from_utf8_lossy(&stdout);
    let Some((body, status_text)) = output.rsplit_once(HTTP_STATUS_MARKER) else {
        return Err("GITHUB_MALFORMED_RESPONSE: HTTP status marker missing".into());
    };
    let status_text = status_text.trim();
    let status = status_text
        .parse::<u16>()
        .map_err(|_| "GITHUB_MALFORMED_RESPONSE: HTTP status invalid".to_string())?;
    Ok((status, body.trim().to_string()))
}

#[cfg(not(test))]
fn read_bounded<R: Read>(mut reader: R, limit: usize) -> (Vec<u8>, bool) {
    let mut retained = Vec::with_capacity(limit.min(16 * 1024));
    let mut buffer = [0_u8; 8192];
    let mut truncated = false;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                if retained.len() < limit {
                    let keep = (limit - retained.len()).min(count);
                    retained.extend_from_slice(&buffer[..keep]);
                    if keep < count {
                        truncated = true;
                    }
                } else {
                    truncated = true;
                }
            }
            Err(_) => break,
        }
    }
    (retained, truncated)
}

fn sanitize_error(value: &str) -> String {
    let mut pending_bearer = false;
    let redacted = value
        .split_whitespace()
        .map(|token| {
            let lower = token.to_ascii_lowercase();
            let result = if pending_bearer {
                pending_bearer = false;
                "[REDACTED]".to_string()
            } else if lower == "bearer" {
                pending_bearer = true;
                "Bearer".to_string()
            } else if lower.starts_with("authorization:") {
                "Authorization: [REDACTED]".to_string()
            } else if ["token=", "access_token=", "api_key=", "apikey="]
                .iter()
                .any(|prefix| lower.starts_with(prefix))
            {
                format!("{}[REDACTED]", token.split('=').next().unwrap_or("secret"))
            } else if ["ghp_", "github_pat_", "gho_", "ghu_", "ghs_", "ghr_"]
                .iter()
                .any(|prefix| lower.starts_with(prefix))
            {
                "[REDACTED]".to_string()
            } else {
                token.to_string()
            };
            result
        })
        .collect::<Vec<_>>()
        .join(" ");
    redacted
        .chars()
        .filter(|c| !c.is_ascii_control() || *c == '\n')
        .collect::<String>()
        .trim()
        .chars()
        .take(512)
        .collect()
}

fn credential_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    lower.contains("token") || lower.contains("authorization") || lower.contains("api_key") || lower == "apikey"
}

fn sanitize_value(value: &Value, key: Option<&str>) -> Value {
    if key.is_some_and(credential_key) {
        return Value::String("[REDACTED]".into());
    }
    match value {
        Value::String(text) => Value::String(sanitize_error(text)),
        Value::Array(items) => Value::Array(
            items
                .iter()
                .take(128)
                .map(|item| sanitize_value(item, None))
                .collect(),
        ),
        Value::Object(object) => Value::Object(
            object
                .iter()
                .take(128)
                .map(|(key, item)| (key.clone(), sanitize_value(item, Some(key))))
                .collect(),
        ),
        _ => value.clone(),
    }
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}

fn parse_repository(value: Option<&Value>, identity: &Identity) -> GitHubRepository {
    GitHubRepository {
        owner: identity.owner.clone(),
        name: identity.repo.clone(),
        full_name: string(value, "full_name").unwrap_or_else(|| identity.full_name.clone()),
        default_branch: string(value, "default_branch").unwrap_or_else(|| identity.branch.clone()),
        tracked_branch: identity.branch.clone(),
        remote_head: None,
        private: value
            .and_then(|v| v.get("private"))
            .and_then(Value::as_bool),
        archived: value
            .and_then(|v| v.get("archived"))
            .and_then(Value::as_bool),
        html_url: string(value, "html_url"),
        description: string(value, "description").map(|v| bound_text(&v, MAX_BODY_CHARS)),
    }
}

fn parse_branches(value: Option<&Value>) -> Vec<GitHubBranch> {
    array(value)
        .into_iter()
        .take(MAX_BRANCHES)
        .filter_map(|v| {
            Some(GitHubBranch {
                name: string(Some(v), "name")?,
                sha: v.get("commit").and_then(|c| string(Some(c), "sha")),
                protected: v.get("protected").and_then(Value::as_bool),
            })
        })
        .collect()
}

fn parse_commits(value: Option<&Value>) -> Vec<GitHubCommit> {
    array(value)
        .into_iter()
        .take(MAX_COMMITS)
        .filter_map(|v| {
            let sha = string(Some(v), "sha")?;
            let commit = v.get("commit");
            Some(GitHubCommit {
                sha,
                message: bound_text(
                    &string(commit, "message").unwrap_or_default(),
                    MAX_BODY_CHARS,
                ),
                author: commit.and_then(|c| string(c.get("author"), "name")),
                authored_at: commit.and_then(|c| string(c.get("author"), "date")),
                committed_at: commit.and_then(|c| string(c.get("committer"), "date")),
                html_url: string(Some(v), "html_url"),
            })
        })
        .collect()
}

fn parse_pull_requests(value: Option<&Value>) -> Vec<GitHubPullRequest> {
    array(value)
        .into_iter()
        .take(MAX_PULL_REQUESTS)
        .filter_map(|v| {
            let number = v.get("number")?.as_u64()?;
            let title = string(Some(v), "title").unwrap_or_else(|| "Untitled pull request".into());
            let body = string(Some(v), "body").unwrap_or_default();
            let merged = v.get("merged_at").is_some_and(|value| !value.is_null());
            let mut task_links = explicit_links(&format!("{title} {body}"));
            task_links.retain(|link| link.starts_with("TASK-") || link.starts_with('M'));
            let session_links = explicit_links(&format!("{title} {body}"))
                .into_iter()
                .filter(|link| link.starts_with("SESSION-"))
                .collect();
            Some(GitHubPullRequest {
                number,
                title: bound_text(&title, MAX_BODY_CHARS),
                state: if merged {
                    "MERGED".into()
                } else {
                    string(Some(v), "state")
                        .unwrap_or_else(|| "UNKNOWN".into())
                        .to_ascii_uppercase()
                },
                draft: v.get("draft").and_then(Value::as_bool),
                merged,
                author: string(v.get("user"), "login"),
                created_at: string(Some(v), "created_at"),
                updated_at: string(Some(v), "updated_at"),
                source_branch: string(v.get("head"), "ref"),
                target_branch: string(v.get("base"), "ref"),
                head_sha: string(v.get("head"), "sha"),
                base_sha: string(v.get("base"), "sha"),
                html_url: string(Some(v), "html_url"),
                changed_files: v.get("changed_files").and_then(Value::as_u64),
                additions: v.get("additions").and_then(Value::as_u64),
                deletions: v.get("deletions").and_then(Value::as_u64),
                review_status: string(Some(v), "review_status"),
                check_status: string(Some(v), "check_status"),
                detail_state: "NOT_REQUESTED".into(),
                files_state: "NOT_REQUESTED".into(),
                reviews_state: "NOT_REQUESTED".into(),
                comments_state: "NOT_REQUESTED".into(),
                checks_state: "NOT_REQUESTED".into(),
                files: Vec::new(),
                reviews: Vec::new(),
                checks: Vec::new(),
                comments: bounded_comments(v.get("comments")),
                task_links,
                session_links,
            })
        })
        .collect()
}

fn parse_issues(value: Option<&Value>) -> Vec<GitHubIssue> {
    array(value)
        .into_iter()
        .filter(|v| v.get("pull_request").is_none())
        .take(MAX_ISSUES)
        .filter_map(|v| {
            Some(GitHubIssue {
                number: v.get("number")?.as_u64()?,
                title: bound_text(
                    &string(Some(v), "title").unwrap_or_else(|| "Untitled issue".into()),
                    MAX_BODY_CHARS,
                ),
                state: string(Some(v), "state")
                    .unwrap_or_else(|| "UNKNOWN".into())
                    .to_ascii_uppercase(),
                labels: v
                    .get("labels")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(|label| string(Some(label), "name"))
                    .take(20)
                    .collect(),
                author: string(v.get("user"), "login"),
                created_at: string(Some(v), "created_at"),
                updated_at: string(Some(v), "updated_at"),
                html_url: string(Some(v), "html_url"),
                body_excerpt: string(Some(v), "body").map(|body| bound_text(&body, MAX_BODY_CHARS)),
                comments: bounded_comments(v.get("comments")),
                task_links: explicit_links(&format!(
                    "{} {}",
                    string(Some(v), "title").unwrap_or_default(),
                    string(Some(v), "body").unwrap_or_default()
                ))
                .into_iter()
                .filter(|link| link.starts_with("TASK-") || link.starts_with('M'))
                .collect(),
            })
        })
        .collect()
}

fn parse_actions(value: Option<&Value>) -> Vec<GitHubActionRun> {
    let runs = value
        .and_then(|v| v.get("workflow_runs"))
        .and_then(Value::as_array)
        .or_else(|| value.and_then(Value::as_array));
    runs.into_iter()
        .flatten()
        .take(MAX_ACTION_RUNS)
        .filter_map(|v| {
            let jobs = v
                .get("jobs")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .take(10)
                .filter_map(|job| {
                    Some(GitHubActionJob {
                        id: job.get("id")?.as_u64()?,
                        name: string(Some(job), "name"),
                        status: string(Some(job), "status").map(|s| s.to_ascii_uppercase()),
                        conclusion: string(Some(job), "conclusion").map(|s| s.to_ascii_uppercase()),
                        steps: job
                            .get("steps")
                            .and_then(Value::as_array)
                            .into_iter()
                            .flatten()
                            .take(25)
                            .map(|step| GitHubActionStep {
                                name: string(Some(step), "name")
                                    .map(|s| bound_text(&s, MAX_BODY_CHARS)),
                                status: string(Some(step), "status")
                                    .map(|s| s.to_ascii_uppercase()),
                                conclusion: string(Some(step), "conclusion")
                                    .map(|s| s.to_ascii_uppercase()),
                                number: step.get("number").and_then(Value::as_u64),
                            })
                            .collect(),
                    })
                })
                .collect::<Vec<_>>();
            let failed_log_summary = string(Some(v), "failed_log_summary")
                .or_else(|| string(Some(v), "log_summary"))
                .map(|s| bound_text(&s, MAX_BODY_CHARS));
            Some(GitHubActionRun {
                id: v.get("id")?.as_u64()?,
                name: string(Some(v), "name"),
                event: string(Some(v), "event"),
                status: string(Some(v), "status").map(|s| s.to_ascii_uppercase()),
                conclusion: string(Some(v), "conclusion").map(|s| s.to_ascii_uppercase()),
                branch: string(Some(v), "head_branch"),
                head_sha: string(Some(v), "head_sha"),
                pull_request_numbers: v
                    .get("pull_requests")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .filter_map(|pr| pr.get("number").and_then(Value::as_u64))
                    .take(MAX_PULL_REQUESTS)
                    .collect(),
                created_at: string(Some(v), "created_at"),
                updated_at: string(Some(v), "updated_at"),
                failed_log_summary,
                jobs_state: "NOT_REQUESTED".into(),
                logs_state: "NOT_REQUESTED".into(),
                jobs,
            })
        })
        .collect()
}

fn resource_result<'a>(resources: &'a [ResourceResult], kind: &str) -> Option<&'a ResourceResult> {
    resources.iter().find(|resource| resource.kind == kind)
}

fn resource_state(resources: &[ResourceResult], kind: &str) -> String {
    resource_result(resources, kind)
        .map(|resource| resource.state.clone())
        .unwrap_or_else(|| "NOT_REQUESTED".into())
}

fn enrich_pull_requests(pull_requests: &mut [GitHubPullRequest], resources: &[ResourceResult]) {
    for pull_request in pull_requests.iter_mut().take(MAX_PR_ENRICHED) {
        let number = pull_request.number;
        let detail_kind = format!("GITHUB_PR_{number}_DETAIL");
        pull_request.detail_state = resource_state(resources, &detail_kind);
        if let Some(detail) = resource_result(resources, &detail_kind).and_then(|r| r.value.as_ref()) {
            pull_request.changed_files = detail.get("changed_files").and_then(Value::as_u64);
            pull_request.additions = detail.get("additions").and_then(Value::as_u64);
            pull_request.deletions = detail.get("deletions").and_then(Value::as_u64);
            pull_request.review_status = string(Some(detail), "review_status");
            pull_request.source_branch = string(detail.get("head"), "ref").or_else(|| pull_request.source_branch.clone());
            pull_request.target_branch = string(detail.get("base"), "ref").or_else(|| pull_request.target_branch.clone());
            pull_request.head_sha = string(detail.get("head"), "sha").or_else(|| pull_request.head_sha.clone());
            pull_request.base_sha = string(detail.get("base"), "sha").or_else(|| pull_request.base_sha.clone());
        }

        let files_kind = format!("GITHUB_PR_{number}_FILES");
        pull_request.files_state = resource_state(resources, &files_kind);
        pull_request.files = resource_result(resources, &files_kind)
            .and_then(|r| r.value.as_ref())
            .map(parse_pull_request_files)
            .unwrap_or_default();

        let reviews_kind = format!("GITHUB_PR_{number}_REVIEWS");
        pull_request.reviews_state = resource_state(resources, &reviews_kind);
        pull_request.reviews = resource_result(resources, &reviews_kind)
            .and_then(|r| r.value.as_ref())
            .map(parse_reviews)
            .unwrap_or_default();
        if pull_request.reviews_state == "CURRENT" {
            pull_request.review_status = Some(review_summary(&pull_request.reviews));
        }

        let issue_comments_kind = format!("GITHUB_PR_{number}_COMMENTS");
        let review_comments_kind = format!("GITHUB_PR_{number}_REVIEW_COMMENTS");
        let issue_state = resource_state(resources, &issue_comments_kind);
        let review_state = resource_state(resources, &review_comments_kind);
        pull_request.comments_state = if issue_state == "CURRENT" && review_state == "CURRENT" {
            "CURRENT".into()
        } else if issue_state == "CURRENT" || review_state == "CURRENT" {
            "PARTIAL".into()
        } else if issue_state == "NOT_REQUESTED" && review_state == "NOT_REQUESTED" {
            "NOT_REQUESTED".into()
        } else if issue_state != "CURRENT" {
            issue_state
        } else {
            review_state
        };
        let mut comments = resource_result(resources, &issue_comments_kind)
            .and_then(|r| r.value.as_ref())
            .map(|value| bounded_comments(Some(value)))
            .unwrap_or_default();
        if let Some(value) = resource_result(resources, &review_comments_kind).and_then(|r| r.value.as_ref()) {
            comments.extend(bounded_comments(Some(value)));
        }
        comments.truncate(MAX_COMMENTS);
        pull_request.comments = comments;

        let checks_kind = format!("GITHUB_PR_{number}_CHECKS");
        let status_kind = format!("GITHUB_PR_{number}_STATUS");
        let checks_state = resource_state(resources, &checks_kind);
        let status_state = resource_state(resources, &status_kind);
        pull_request.checks_state = if checks_state == "CURRENT" && status_state == "CURRENT" {
            "CURRENT".into()
        } else if checks_state == "CURRENT" || status_state == "CURRENT" {
            "PARTIAL".into()
        } else if checks_state != "NOT_REQUESTED" {
            checks_state
        } else {
            status_state
        };
        let mut checks = resource_result(resources, &checks_kind)
            .and_then(|r| r.value.as_ref())
            .map(parse_checks)
            .unwrap_or_default();
        if let Some(value) = resource_result(resources, &status_kind).and_then(|r| r.value.as_ref()) {
            checks.extend(parse_statuses(value));
        }
        checks.truncate(MAX_PR_CHECKS);
        pull_request.checks = checks;
        if pull_request.checks_state == "CURRENT" {
            pull_request.check_status = Some(check_summary(&pull_request.checks));
        }
    }
}

fn parse_pull_request_files(value: &Value) -> Vec<GitHubPullRequestFile> {
    array(Some(value))
        .into_iter()
        .take(MAX_PR_FILES)
        .filter_map(|file| Some(GitHubPullRequestFile {
            filename: bound_text(&string(Some(file), "filename")?, MAX_BODY_CHARS),
            status: string(Some(file), "status"),
            additions: file.get("additions").and_then(Value::as_u64),
            deletions: file.get("deletions").and_then(Value::as_u64),
            changes: file.get("changes").and_then(Value::as_u64),
            patch_excerpt: string(Some(file), "patch").map(|patch| bound_text(&patch, MAX_BODY_CHARS)),
        }))
        .collect()
}

fn parse_reviews(value: &Value) -> Vec<GitHubPullRequestReview> {
    array(Some(value))
        .into_iter()
        .take(MAX_PR_REVIEWS)
        .filter_map(|review| Some(GitHubPullRequestReview {
            id: review.get("id")?.as_u64()?,
            user: string(review.get("user"), "login"),
            state: string(Some(review), "state").map(|value| value.to_ascii_uppercase()),
            submitted_at: string(Some(review), "submitted_at"),
            body_excerpt: string(Some(review), "body").map(|value| bound_text(&value, MAX_BODY_CHARS)),
        }))
        .collect()
}

fn parse_checks(value: &Value) -> Vec<GitHubCheck> {
    let records = value.get("check_runs").and_then(Value::as_array).cloned().unwrap_or_default();
    records.into_iter().take(MAX_PR_CHECKS).filter_map(|check| Some(GitHubCheck {
        id: check.get("id")?.as_u64()?,
        name: string(Some(&check), "name").map(|value| bound_text(&value, MAX_BODY_CHARS)),
        status: string(Some(&check), "status").map(|value| value.to_ascii_uppercase()),
        conclusion: string(Some(&check), "conclusion").map(|value| value.to_ascii_uppercase()),
        details_url: string(Some(&check), "details_url"),
    })).collect()
}

fn parse_statuses(value: &Value) -> Vec<GitHubCheck> {
    value.get("statuses").and_then(Value::as_array).cloned().unwrap_or_default()
        .into_iter().take(MAX_PR_CHECKS).enumerate().map(|(index, status)| GitHubCheck {
            id: status.get("id").and_then(Value::as_u64).unwrap_or(index as u64),
            name: string(Some(&status), "context").map(|value| bound_text(&value, MAX_BODY_CHARS)),
            status: string(Some(&status), "state").map(|value| value.to_ascii_uppercase()),
            conclusion: string(Some(&status), "state").map(|value| value.to_ascii_uppercase()),
            details_url: string(Some(&status), "target_url"),
        }).collect()
}

fn review_summary(reviews: &[GitHubPullRequestReview]) -> String {
    if reviews.is_empty() { return "NO_REVIEWS".into(); }
    if reviews.iter().any(|review| review.state.as_deref() == Some("CHANGES_REQUESTED")) { return "CHANGES_REQUESTED".into(); }
    if reviews.iter().any(|review| review.state.as_deref() == Some("APPROVED")) { return "APPROVED".into(); }
    "REVIEWED".into()
}

fn check_summary(checks: &[GitHubCheck]) -> String {
    if checks.is_empty() { return "NO_CHECKS".into(); }
    if checks.iter().any(|check| matches!(check.conclusion.as_deref(), Some("FAILURE" | "ERROR"))) { return "FAILURE".into(); }
    if checks.iter().any(|check| matches!(check.status.as_deref(), Some("IN_PROGRESS" | "QUEUED"))) { return "PENDING".into(); }
    if checks.iter().all(|check| check.conclusion.as_deref() == Some("SUCCESS")) { return "SUCCESS".into(); }
    "COMPLETED".into()
}

fn enrich_actions(actions: &mut [GitHubActionRun], resources: &[ResourceResult]) {
    for run in actions.iter_mut().take(MAX_ACTION_RUNS_ENRICHED) {
        let jobs_kind = format!("GITHUB_ACTION_{}_JOBS", run.id);
        run.jobs_state = resource_state(resources, &jobs_kind);
        run.jobs = resource_result(resources, &jobs_kind)
            .and_then(|r| r.value.as_ref())
            .map(|value| parse_action_jobs(value))
            .unwrap_or_default();
        if run.jobs_state != "CURRENT" || run.jobs.is_empty() {
            run.logs_state = if run.jobs_state == "CURRENT" { "NOT_APPLICABLE".into() } else { run.jobs_state.clone() };
            continue;
        }
        let mut log_states = Vec::new();
        for job in run.jobs.iter().take(2) {
            let kind = format!("GITHUB_ACTION_JOB_{}_LOG", job.id);
            log_states.push(resource_state(resources, &kind));
            if run.failed_log_summary.is_none() {
                run.failed_log_summary = resource_result(resources, &kind)
                    .and_then(|r| r.value.as_ref())
                    .and_then(Value::as_str)
                    .map(|value| bound_text(value, MAX_ACTION_LOG_CHARS));
            }
        }
        run.logs_state = if log_states.iter().all(|state| state == "CURRENT") { "CURRENT".into() } else if log_states.iter().any(|state| state == "CURRENT") { "PARTIAL".into() } else { log_states.first().cloned().unwrap_or_else(|| "NOT_APPLICABLE".into()) };
    }
}

fn parse_action_jobs(value: &Value) -> Vec<GitHubActionJob> {
    value.get("jobs").and_then(Value::as_array).cloned().unwrap_or_default()
        .into_iter().take(MAX_ACTION_JOBS).filter_map(|job| Some(GitHubActionJob {
            id: job.get("id")?.as_u64()?,
            name: string(Some(&job), "name"),
            status: string(Some(&job), "status").map(|value| value.to_ascii_uppercase()),
            conclusion: string(Some(&job), "conclusion").map(|value| value.to_ascii_uppercase()),
            steps: job.get("steps").and_then(Value::as_array).cloned().unwrap_or_default().into_iter().take(MAX_ACTION_STEPS).map(|step| GitHubActionStep {
                name: string(Some(&step), "name").map(|value| bound_text(&value, MAX_BODY_CHARS)),
                status: string(Some(&step), "status").map(|value| value.to_ascii_uppercase()),
                conclusion: string(Some(&step), "conclusion").map(|value| value.to_ascii_uppercase()),
                number: step.get("number").and_then(Value::as_u64),
            }).collect(),
        })).collect()
}

fn parse_releases(value: Option<&Value>) -> Vec<GitHubRelease> {
    array(value)
        .into_iter()
        .take(MAX_RELEASES)
        .filter_map(|v| {
            Some(GitHubRelease {
                id: v.get("id")?.as_u64()?,
                name: string(Some(v), "name"),
                tag_name: string(Some(v), "tag_name"),
                target_commitish: string(Some(v), "target_commitish"),
                draft: v.get("draft").and_then(Value::as_bool).unwrap_or(false),
                prerelease: v
                    .get("prerelease")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                published_at: string(Some(v), "published_at"),
                html_url: string(Some(v), "html_url"),
            })
        })
        .collect()
}

fn parse_tags(value: Option<&Value>) -> Vec<GitHubTag> {
    array(value)
        .into_iter()
        .take(MAX_TAGS)
        .filter_map(|v| {
            Some(GitHubTag {
                name: string(Some(v), "name")?,
                commit_sha: string(v.get("commit"), "sha"),
                protected: v.get("protected").and_then(Value::as_bool),
            })
        })
        .collect()
}

fn array(value: Option<&Value>) -> Vec<&Value> {
    value
        .and_then(Value::as_array)
        .map(|values| values.iter().collect())
        .unwrap_or_default()
}

fn string(value: Option<&Value>, key: &str) -> Option<String> {
    value
        .and_then(|v| v.get(key))
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

fn bounded_comments(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .take(MAX_COMMENTS)
                .filter_map(|item| {
                    item.as_str()
                        .map(|text| bound_text(text, MAX_BODY_CHARS))
                        .or_else(|| {
                            string(Some(item), "body").map(|text| bound_text(&text, MAX_BODY_CHARS))
                        })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn bound_text(value: &str, max_chars: usize) -> String {
    sanitize_error(value).chars().take(max_chars).collect()
}

fn explicit_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    for token in text.split(|c: char| !(c.is_ascii_alphanumeric() || matches!(c, '-' | '.' | '_')))
    {
        let valid = if token.len() <= 64 && token.starts_with("TASK-") {
            valid_reference_suffix(&token[5..])
        } else if token.len() <= 64 && token.starts_with("SESSION-") {
            valid_reference_suffix(&token[8..])
        } else {
            valid_milestone(token)
        };
        if valid && !links.iter().any(|existing| existing == token) {
                links.push(token.to_string());
        }
    }
    links
}

fn valid_reference_suffix(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 56
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

fn valid_milestone(value: &str) -> bool {
    let Some(rest) = value.strip_prefix('M') else {
        return false;
    };
    if rest.is_empty() || rest.starts_with('.') || rest.ends_with('.') {
        return false;
    }
    rest.split('.').all(|part| {
        !part.is_empty() && part.chars().all(|c| c.is_ascii_digit())
    })
}

fn local_evidence(database: &DatabaseState, project: &ProjectRecord) -> LocalGitHubEvidence {
    match git_engine::snapshot(
        database,
        git_engine::GitSnapshotRequest {
            project_id: project.id.clone(),
            persist: Some(false),
        },
    ) {
        Ok(snapshot) => local_from_snapshot(&snapshot),
        Err(error) => LocalGitHubEvidence {
            available: false,
            branch: None,
            head_sha: None,
            upstream: None,
            ahead_count: None,
            behind_count: None,
            detached: false,
            dirty: false,
            health: None,
            error: Some(error),
        },
    }
}

fn local_from_snapshot(snapshot: &GitSnapshot) -> LocalGitHubEvidence {
    LocalGitHubEvidence {
        available: true,
        branch: snapshot.current_branch.clone(),
        head_sha: snapshot.head_sha.clone(),
        upstream: snapshot.upstream.clone(),
        ahead_count: snapshot.ahead_count,
        behind_count: snapshot.behind_count,
        detached: snapshot.detached_head,
        dirty: !snapshot.staged_files.is_empty()
            || !snapshot.unstaged_files.is_empty()
            || !snapshot.untracked_files.is_empty()
            || !snapshot.conflicted_files.is_empty(),
        health: Some(
            match snapshot.health {
                RepositoryHealth::Clean => "CLEAN",
                RepositoryHealth::Dirty => "DIRTY",
                RepositoryHealth::Conflicted => "CONFLICTED",
                RepositoryHealth::Detached => "DETACHED",
                RepositoryHealth::Unborn => "UNBORN",
                RepositoryHealth::Missing => "MISSING",
                RepositoryHealth::NonGit => "NON_GIT",
            }
            .into(),
        ),
        error: None,
    }
}

pub fn reconcile(
    local: &LocalGitHubEvidence,
    remote_branch: &str,
    remote_head: Option<&str>,
    remote_health: &str,
) -> GitHubReconciliation {
    let mut evidence = Vec::new();
    let state = if remote_health == "STALE" {
        evidence.push("remote fetched-at is outside the freshness window".into());
        "REMOTE_STALE"
    } else if matches!(
        remote_health,
        "UNAVAILABLE" | "AUTH_REQUIRED" | "RATE_LIMITED" | "OFFLINE" | "TIMEOUT" | "MALFORMED"
    ) {
        evidence.push(format!("remote health is {remote_health}"));
        "REMOTE_UNAVAILABLE"
    } else if remote_head.is_none() {
        evidence.push("tracked remote branch has no verified HEAD".into());
        "REMOTE_BRANCH_MISSING"
    } else if !local.available {
        evidence.push("local Git Engine snapshot is unavailable".into());
        "UNKNOWN"
    } else if local.detached || local.upstream.is_none() || local.branch.is_none() {
        evidence.push("local branch is detached or has no upstream".into());
        "DETACHED_OR_NO_UPSTREAM"
    } else if local.head_sha.as_deref() == remote_head {
        evidence.push("local HEAD equals the verified remote branch HEAD".into());
        "EQUAL"
    } else if local.ahead_count.unwrap_or(0) > 0 && local.behind_count.unwrap_or(0) == 0 {
        evidence.push("local upstream counts show commits only ahead of remote".into());
        "LOCAL_AHEAD"
    } else if local.ahead_count.unwrap_or(0) == 0 && local.behind_count.unwrap_or(0) > 0 {
        evidence.push("local upstream counts show commits only behind remote".into());
        "LOCAL_BEHIND"
    } else if local.ahead_count.unwrap_or(0) > 0 && local.behind_count.unwrap_or(0) > 0 {
        evidence.push("local upstream counts show both ahead and behind commits".into());
        "DIVERGED"
    } else {
        evidence.push("local and remote commit relationship is insufficiently evidenced".into());
        "UNKNOWN"
    };
    GitHubReconciliation {
        state: state.into(),
        local_branch: local.branch.clone(),
        local_head: local.head_sha.clone(),
        remote_branch: remote_branch.into(),
        remote_head: remote_head.map(ToString::to_string),
        local_dirty: local.dirty,
        evidence,
    }
}

fn github_repository_from_project(project: &ProjectRecord) -> Result<Identity, String> {
    identity_from_project(project).map(identity_from_view)
}

#[allow(dead_code)]
fn project_identity_for_scope(project: &ProjectRecord) -> Result<String, String> {
    Ok(github_repository_from_project(project)?.full_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn local(
        state: &str,
        head: &str,
        ahead: Option<u64>,
        behind: Option<u64>,
        dirty: bool,
    ) -> LocalGitHubEvidence {
        LocalGitHubEvidence {
            available: true,
            branch: Some("main".into()),
            head_sha: Some(head.into()),
            upstream: Some("origin/main".into()),
            ahead_count: ahead,
            behind_count: behind,
            detached: state == "DETACHED",
            dirty,
            health: Some(state.into()),
            error: None,
        }
    }

    #[test]
    fn reconciliation_matrix_keeps_dirty_separate_from_commit_relationship() {
        let cases = [
            (
                local("CLEAN", "remote", Some(0), Some(0), true),
                "remote",
                "EQUAL",
            ),
            (
                local("DIRTY", "ahead", Some(2), Some(0), true),
                "remote",
                "LOCAL_AHEAD",
            ),
            (
                local("CLEAN", "behind", Some(0), Some(3), false),
                "remote",
                "LOCAL_BEHIND",
            ),
            (
                local("DIRTY", "split", Some(1), Some(1), true),
                "remote",
                "DIVERGED",
            ),
            (
                local("DETACHED", "detached", None, None, true),
                "remote",
                "DETACHED_OR_NO_UPSTREAM",
            ),
        ];
        for (local, head, expected) in cases {
            let result = reconcile(&local, "main", Some(head), "CURRENT");
            assert_eq!(result.state, expected);
            assert_eq!(result.local_dirty, local.dirty);
        }
    }

    #[test]
    fn reconciliation_fails_closed_for_remote_states_and_missing_branch() {
        let local = local("CLEAN", "remote", Some(0), Some(0), false);
        assert_eq!(
            reconcile(&local, "main", Some("remote"), "STALE").state,
            "REMOTE_STALE"
        );
        assert_eq!(
            reconcile(&local, "main", Some("remote"), "RATE_LIMITED").state,
            "REMOTE_UNAVAILABLE"
        );
        assert_eq!(
            reconcile(&local, "main", None, "CURRENT").state,
            "REMOTE_BRANCH_MISSING"
        );
        let unavailable = LocalGitHubEvidence {
            available: false,
            ..local
        };
        assert_eq!(
            reconcile(&unavailable, "main", Some("remote"), "CURRENT").state,
            "UNKNOWN"
        );
    }

    #[test]
    fn payload_parsers_bound_and_preserve_explicit_links_only() {
        let value = json!([{"number": 7, "title": "M18.01 TASK-42", "body": "same title does not imply ownership SESSION-abc", "state": "open", "draft": false, "user": {"login": "owner"}, "head": {"ref": "feature", "sha": "abc"}, "base": {"ref": "main", "sha": "def"}, "comments": 99}]);
        let prs = parse_pull_requests(Some(&value));
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].task_links, vec!["M18.01", "TASK-42"]);
        assert_eq!(prs[0].session_links, vec!["SESSION-abc"]);
        assert!(prs[0].comments.is_empty());

        let issues = json!([{"number": 1, "title": "M18.03", "body": "TASK-8 is explicit", "state": "open", "labels": [{"name": "bug"}]}]);
        let parsed = parse_issues(Some(&issues));
        assert_eq!(parsed[0].task_links, vec!["M18.03", "TASK-8"]);
        assert_eq!(parsed[0].labels, vec!["bug"]);
    }

    #[test]
    fn strict_link_grammar_rejects_prose_and_accepts_canonical_refs() {
        let links = explicit_links("Model5 May2026 Mfoo7bar M18.10.01 TASK-42 SESSION-abc other-project-7");
        assert_eq!(links, vec!["M18.10.01", "TASK-42", "SESSION-abc"]);
        assert!(!explicit_links("M18.10.01x M.18 M18..1").contains(&"M18.10.01x".into()));
    }

    #[test]
    fn production_transport_seam_enriches_exact_pr_and_actions_paths() {
        let database_dir = tempfile::tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        crate::github_tracking::ensure_portfolio(&database).unwrap();
        let project = crate::projects::fetch_project(&database, "github:Sekiph82/H-veAI@main").unwrap();
        let repo = "https://api.github.com/repos/Sekiph82/H-veAI";
        let pr_list = format!("{repo}/pulls?state=all&per_page={MAX_PULL_REQUESTS}");
        let detail = format!("{repo}/pulls/12");
        let files = format!("{repo}/pulls/12/files?per_page={MAX_PR_FILES}");
        let reviews = format!("{repo}/pulls/12/reviews?per_page={MAX_PR_REVIEWS}");
        let comments = format!("{repo}/issues/12/comments?per_page={MAX_COMMENTS}");
        let review_comments = format!("{repo}/pulls/12/comments?per_page={MAX_COMMENTS}");
        let checks = format!("{repo}/commits/head-sha/check-runs?per_page={MAX_PR_CHECKS}");
        let status = format!("{repo}/commits/head-sha/status?per_page={MAX_PR_CHECKS}");
        let actions = format!("{repo}/actions/runs?per_page={MAX_ACTION_RUNS}");
        let jobs = format!("{repo}/actions/runs/55/jobs?per_page={MAX_ACTION_JOBS}");
        let logs = format!("{repo}/actions/jobs/56/logs");
        let mut transport = FixtureTransport::default()
            .response(&pr_list, 200, r#"[{"number":12,"title":"M18.10.01","body":"TASK-42","state":"open","head":{"ref":"feature","sha":"head-sha"},"base":{"ref":"main","sha":"base-sha"}}]"#)
            .response(&detail, 200, r#"{"number":12,"changed_files":2,"additions":4,"deletions":1,"head":{"ref":"feature","sha":"head-sha"},"base":{"ref":"main","sha":"base-sha"}}"#)
            .response(&files, 200, r#"[{"filename":"src/app.ts","status":"modified","additions":3,"deletions":1,"changes":4,"patch":"@@ bounded"}]"#)
            .response(&reviews, 200, r#"[{"id":13,"user":{"login":"reviewer"},"state":"APPROVED","body":"Looks good"}]"#)
            .response(&comments, 200, r#"[{"body":"TASK-42 comment"}]"#)
            .response(&review_comments, 200, r#"[{"body":"review comment"}]"#)
            .response(&checks, 200, r#"{"check_runs":[{"id":14,"name":"build","status":"completed","conclusion":"success"}]}"#)
            .response(&status, 200, r#"{"statuses":[]}"#)
            .response(&actions, 200, r#"{"workflow_runs":[{"id":55,"name":"CI","status":"completed","conclusion":"failure","head_sha":"head-sha","head_branch":"feature"}]}"#)
            .response(&jobs, 200, r#"{"jobs":[{"id":56,"name":"build","status":"completed","conclusion":"failure","steps":[{"number":1,"name":"compile","status":"completed","conclusion":"failure"}]}]}"#)
            .response(&logs, 200, "compile failed");
        let snapshot = snapshot_with_transport(&database, &project, &mut transport).unwrap();
        let pr = &snapshot.pull_requests[0];
        assert_eq!(pr.changed_files, Some(2));
        assert_eq!(pr.files[0].filename, "src/app.ts");
        assert_eq!(pr.review_status.as_deref(), Some("APPROVED"));
        assert_eq!(pr.check_status.as_deref(), Some("SUCCESS"));
        assert_eq!(pr.comments.len(), 2);
        assert_eq!(snapshot.actions[0].jobs[0].steps[0].name.as_deref(), Some("compile"));
        assert_eq!(snapshot.actions[0].failed_log_summary.as_deref(), Some("compile failed"));
        for expected in [detail, files, reviews, comments, review_comments, checks, status, jobs, logs] {
            assert!(transport.requests.iter().any(|request| request == &expected), "missing request {expected}");
        }
        assert!(transport.requests.len() <= MAX_SUBRESOURCE_REQUESTS + 8);
    }

    #[test]
    fn successful_remote_payloads_are_redacted_before_cache_and_frontend_projection() {
        let database_dir = tempfile::tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        crate::github_tracking::ensure_portfolio(&database).unwrap();
        let project = crate::projects::fetch_project(&database, "github:Sekiph82/H-veAI@main").unwrap();
        let url = format!("https://api.github.com/repos/Sekiph82/H-veAI/issues?state=all&per_page={MAX_ISSUES}");
        let secret_body = "Authorization: Bearer arbitrary-secret github_pat_test-secret token=query-secret";
        let mut transport = FixtureTransport::default().response(&url, 200, &format!(r#"[{{"number":3,"title":"M18.03","body":"{secret_body}","state":"open"}}]"#));
        let snapshot = snapshot_with_transport(&database, &project, &mut transport).unwrap();
        let issue = &snapshot.issues[0];
        let returned = serde_json::to_string(issue).unwrap();
        assert!(!returned.contains("arbitrary-secret"));
        assert!(!returned.contains("github_pat_test-secret"));
        assert!(!returned.contains("query-secret"));
        let metadata: Vec<String> = database.open_connection().unwrap().prepare("SELECT metadata_json FROM github_sync_state WHERE project_id=?1").unwrap().query_map([&project.id], |row| row.get(0)).unwrap().collect::<Result<_, _>>().unwrap();
        let persisted = metadata.join("\n");
        assert!(!persisted.contains("arbitrary-secret"));
        assert!(!persisted.contains("github_pat_test-secret"));
        assert!(!persisted.contains("query-secret"));
    }

    #[test]
    fn failure_classes_are_distinct_and_safe() {
        assert_eq!(
            classify_failure("GITHUB_HTTP_403_RATE_LIMITED"),
            "RATE_LIMITED"
        );
        assert_eq!(
            classify_failure("GITHUB_HTTP_401_AUTH_REQUIRED"),
            "AUTH_REQUIRED"
        );
        assert_eq!(classify_failure("GITHUB_NETWORK_TIMEOUT"), "TIMEOUT");
        assert_eq!(classify_failure("GITHUB_MALFORMED_RESPONSE"), "MALFORMED");
        assert_eq!(
            classify_failure("GITHUB_NETWORK_REQUEST_FAILED: Could not resolve host"),
            "OFFLINE"
        );
        assert_eq!(
            classify_failure("GITHUB_HTTP_REMOTE_UNAVAILABLE"),
            "UNAVAILABLE"
        );
    }

    #[test]
    fn repository_scope_validation_rejects_host_and_path_injection() {
        assert!(validate_segment("Sekiph82", "owner").is_ok());
        assert!(validate_segment("repo-name", "repository").is_ok());
        assert!(validate_segment("evil/owner", "owner").is_err());
        assert!(validate_segment("repo?x=1", "repository").is_err());
        assert!(validate_branch("feature/topic").is_ok());
        assert!(validate_branch("../main").is_err());
        assert!(validate_branch("https://evil.example").is_err());
    }

    #[test]
    fn percent_encoding_keeps_branch_query_bounded_and_non_authoritative() {
        assert_eq!(percent_encode("feature/topic"), "feature%2Ftopic");
        assert_eq!(percent_encode("main"), "main");
    }

    #[test]
    fn mutation_boundary_is_explicitly_denied() {
        let policy = GitHubMutationPolicy {
            remote_mutations: "DENIED_BY_DEFAULT".into(),
            pull_request_creation: "UNAVAILABLE_AUTH_BOUNDARY".into(),
            workflow_retry: "UNAVAILABLE_AUTH_BOUNDARY".into(),
            local_git_mutations: "NOT_PERFORMED".into(),
            confirmation_required: true,
        };
        assert_eq!(policy.remote_mutations, "DENIED_BY_DEFAULT");
        assert!(policy.confirmation_required);
    }

    #[test]
    fn actions_cover_statuses_and_bound_job_step_log_evidence() {
        let value = json!({"workflow_runs": [
            {"id": 1, "name": "queued", "status": "queued"},
            {"id": 2, "name": "running", "status": "in_progress"},
            {"id": 3, "name": "failed", "status": "completed", "conclusion": "failure", "failed_log_summary": "bounded failure", "jobs": [{"id": 4, "name": "build", "status": "completed", "conclusion": "failure", "steps": [{"number": 1, "name": "compile", "status": "completed", "conclusion": "failure"}]}]},
            {"id": 5, "name": "cancelled", "status": "completed", "conclusion": "cancelled"},
            {"id": 6, "name": "timed out", "status": "completed", "conclusion": "timed_out"},
            {"id": 7, "name": "skipped", "status": "completed", "conclusion": "skipped"}
        ]});
        let actions = parse_actions(Some(&value));
        assert_eq!(actions.len(), 6);
        assert_eq!(actions[0].status.as_deref(), Some("QUEUED"));
        assert_eq!(actions[1].status.as_deref(), Some("IN_PROGRESS"));
        assert_eq!(actions[2].conclusion.as_deref(), Some("FAILURE"));
        assert_eq!(
            actions[2].failed_log_summary.as_deref(),
            Some("bounded failure")
        );
        assert_eq!(actions[2].jobs[0].steps[0].name.as_deref(), Some("compile"));
        assert_eq!(actions[5].conclusion.as_deref(), Some("SKIPPED"));
    }

    #[test]
    fn cache_is_branch_scoped_and_stale_state_is_truthful() {
        let database_dir = tempfile::tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        crate::github_tracking::ensure_portfolio(&database).unwrap();
        let project =
            crate::projects::fetch_project(&database, "github:Sekiph82/H-veAI@main").unwrap();
        let cache = CacheEnvelope {
            schema_version: CACHE_SCHEMA_VERSION,
            resource_kind: RESOURCE_REPOSITORY.into(),
            repository: "Sekiph82/H-veAI".into(),
            branch: "main".into(),
            fetched_at: "2026-09-14T00:00:00Z".into(),
            last_known_good_at: "2026-09-14T00:00:00Z".into(),
            payload: json!({"full_name": "Sekiph82/H-veAI"}),
        };
        persist_cache(&database, &project.id, &cache).unwrap();
        assert!(load_cache(
            &database,
            &project.id,
            RESOURCE_REPOSITORY,
            "Sekiph82/H-veAI",
            "main"
        )
        .unwrap()
        .is_some());
        assert!(load_cache(
            &database,
            &project.id,
            RESOURCE_REPOSITORY,
            "Sekiph82/H-veAI",
            "develop"
        )
        .unwrap()
        .is_none());
        let resources = vec![ResourceResult {
            kind: RESOURCE_REPOSITORY.into(),
            value: Some(cache.payload),
            state: "OFFLINE".into(),
            fetched_at: Some(cache.fetched_at),
            last_known_good_at: Some(cache.last_known_good_at),
            error: Some("offline".into()),
        }];
        let status = cache_status(&resources, "2026-09-15T00:00:00Z");
        assert_eq!(status.state, "STALE");
        assert_eq!(
            status.provenance,
            "GitHub API resource cache; registry identity scoped"
        );
    }

    #[test]
    fn error_redaction_and_response_bounds_are_deterministic() {
        let redacted = sanitize_error("Authorization: Bearer arbitrary-secret token=abc123 access_token=other github_pat_secret gho_old");
        assert!(!redacted.contains("arbitrary-secret"));
        assert!(!redacted.contains("ghp_secret"));
        assert!(!redacted.contains("abc123"));
        assert!(!redacted.contains("other"));
        assert!(!redacted.contains("github_pat_secret"));
        assert!(!redacted.contains("gho_old"));
        assert_eq!(bound_text(&"0123456789", 4), "0123");
    }
}
