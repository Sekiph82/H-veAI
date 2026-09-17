use crate::db::DatabaseState;
use crate::projects::{list_projects, ProjectListQuery, ProjectRecord};
use crate::time::utc_timestamp;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Read;
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};
#[cfg(not(test))]
use std::sync::mpsc;
use std::sync::Condvar;
use std::sync::Mutex;
#[cfg(not(test))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;
#[cfg(not(test))]
use tauri::Emitter;

pub const GITHUB_TASKS_ONLY_POLICY: &str = "GITHUB_TASKS_ONLY";
const REMOTE_TASKS_RESOURCE_KIND: &str = "GITHUB_TASKS_REMOTE";
/// Selected-owner validation is bounded to twelve observations/hour. This is
/// a scheduler target only; M19 uses one portfolio hard freshness horizon.
pub const SELECTED_PROJECT_REFRESH_SECONDS: u64 = 300;
/// Background projects receive one HEAD validation/hour. Same-HEAD
/// validation is one HTTP stage; a changed HEAD adds raw TASKS + commit-feed
/// stages, all subject to the shared tracking governor.
pub const PORTFOLIO_REFRESH_SECONDS: u64 = 3600;
pub const TRACKING_REQUESTS_PER_HOUR: usize = 40;
pub const TRACKING_HTTP_TIMEOUT_SECONDS: u64 = 20;
pub const TRACKING_HTTP_PROCESS_GRACE_SECONDS: u64 = 5;
pub const TRACKING_MAX_SCOPED_STAGES: u64 = 3;
pub const TRACKING_WORKER_CAPACITY: usize = 4;
pub const TRACKING_SCHEDULER_TICK_SECONDS: u64 = 1;
pub const TRACKING_REFRESH_COMPLETION_TIMEOUT_SECONDS: u64 = TRACKING_MAX_SCOPED_STAGES
    * (TRACKING_HTTP_TIMEOUT_SECONDS + TRACKING_HTTP_PROCESS_GRACE_SECONDS)
    + TRACKING_HTTP_PROCESS_GRACE_SECONDS;
pub const M19_HARD_VALIDATION_HORIZON_SECONDS: u64 =
    PORTFOLIO_REFRESH_SECONDS + TRACKING_REFRESH_COMPLETION_TIMEOUT_SECONDS;
pub const TRACKING_REFRESH_ADMISSION_TIMEOUT_SECONDS: u64 =
    TRACKING_REFRESH_COMPLETION_TIMEOUT_SECONDS + TRACKING_SCHEDULER_TICK_SECONDS;
const TRACKING_HTTP_TIMEOUT: Duration = Duration::from_secs(TRACKING_HTTP_TIMEOUT_SECONDS);
const MAX_PENDING_REFRESH_GENERATIONS: usize = 32;
const FAILURE_BACKOFF_MAX_SECONDS: u64 = 24 * 60 * 60;
static TRACKING_REQUEST_GOVERNOR: std::sync::OnceLock<Mutex<TrackingRequestGovernor>> =
    std::sync::OnceLock::new();

#[derive(Debug)]
struct TrackingRequestGovernor {
    window_started: std::time::Instant,
    requests: usize,
}

impl Default for TrackingRequestGovernor {
    fn default() -> Self {
        Self {
            window_started: std::time::Instant::now(),
            requests: 0,
        }
    }
}

impl TrackingRequestGovernor {
    fn admit_at(&mut self, now: std::time::Instant) -> bool {
        if now.duration_since(self.window_started) >= Duration::from_secs(3600) {
            self.window_started = now;
            self.requests = 0;
        }
        if self.requests >= TRACKING_REQUESTS_PER_HOUR {
            return false;
        }
        self.requests += 1;
        true
    }
}

fn admit_tracking_request() -> Result<(), String> {
    let lock =
        TRACKING_REQUEST_GOVERNOR.get_or_init(|| Mutex::new(TrackingRequestGovernor::default()));
    let mut governor = lock
        .lock()
        .map_err(|_| "GitHub tracking request governor lock poisoned".to_string())?;
    if !governor.admit_at(std::time::Instant::now()) {
        return Err("GITHUB_TRACKING_BUDGET_WAIT".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackingHourlyMath {
    pub project_count: usize,
    pub background_observations: usize,
    pub selected_observations: usize,
    pub same_head_http_stages: usize,
    pub changed_head_http_stages: usize,
    pub admitted_http_stages: usize,
}

pub fn tracking_hourly_math(project_count: usize, selected: bool) -> TrackingHourlyMath {
    let projects = project_count.max(1);
    let selected_observations = if selected {
        (3600 / SELECTED_PROJECT_REFRESH_SECONDS) as usize
    } else {
        0
    };
    let selected_project_count = if selected { 1 } else { 0 };
    let background_observations = projects.saturating_sub(selected_project_count)
        * (3600 / PORTFOLIO_REFRESH_SECONDS) as usize;
    let observations = background_observations + selected_observations;
    TrackingHourlyMath {
        project_count: projects,
        background_observations,
        selected_observations,
        same_head_http_stages: observations,
        changed_head_http_stages: observations * 3,
        admitted_http_stages: TRACKING_REQUESTS_PER_HOUR.min(observations * 3),
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct RefreshScopeRequest {
    generation: u64,
    project_id: String,
    accepted_epoch: u64,
}

#[derive(Debug, Default)]
pub struct RefreshScopeGate(Mutex<std::collections::VecDeque<RefreshScopeRequest>>);

impl RefreshScopeGate {
    /// Compatibility helper for direct gate tests; production callers use
    /// `request_for_generation` so scope is never detached from its generation.
    pub fn request(&self, project_id: String) {
        self.request_for_generation(0, project_id)
            .expect("refresh scope test queue is bounded");
    }

    fn request_for_generation(&self, generation: u64, project_id: String) -> Result<(), String> {
        if let Ok(mut scope) = self.0.lock() {
            if scope.len() >= MAX_PENDING_REFRESH_GENERATIONS {
                return Err("GITHUB_REFRESH_QUEUE_FULL".into());
            }
            scope.push_back(RefreshScopeRequest {
                generation,
                project_id,
                accepted_epoch: generation,
            });
            return Ok(());
        }
        Err("GitHub refresh scope lock poisoned".into())
    }

    fn take(&self) -> Option<String> {
        self.0
            .lock()
            .ok()
            .and_then(|mut scope| scope.pop_front())
            .map(|request| request.project_id)
    }

    fn take_all(&self) -> Vec<RefreshScopeRequest> {
        self.0
            .lock()
            .map(|mut scope| scope.drain(..).collect())
            .unwrap_or_default()
    }
}

#[derive(Debug, Default)]
struct RefreshGenerationCoordinator {
    pending: Vec<RefreshScopeRequest>,
}

impl RefreshGenerationCoordinator {
    fn enqueue(&mut self, request: RefreshScopeRequest) {
        if !self.pending.iter().any(|pending| {
            pending.generation == request.generation && pending.project_id == request.project_id
        }) {
            self.pending.push(request);
        }
    }

    fn is_pending_for(&self, project_id: &str) -> bool {
        self.pending
            .iter()
            .any(|request| request.project_id == project_id)
    }

    fn take_for_project<F>(
        &mut self,
        project_id: &str,
        mut is_active: F,
    ) -> Vec<RefreshScopeRequest>
    where
        F: FnMut(u64) -> bool,
    {
        let mut requests = Vec::new();
        let mut retained = Vec::new();
        for request in self.pending.drain(..) {
            if request.project_id == project_id && is_active(request.generation) {
                requests.push(request);
            } else if is_active(request.generation) {
                retained.push(request);
            }
        }
        self.pending = retained;
        requests
    }

    #[allow(dead_code)]
    fn settle_project(&mut self, project_id: &str) -> Vec<u64> {
        self.take_for_project(project_id, |_| true)
            .into_iter()
            .map(|request| request.generation)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RefreshGenerationState {
    Pending,
    Admitted { observation_id: u64 },
    Settled,
}

/// Coordinates an intentional refresh generation with the scheduler. The
/// frontend command resolves only after a qualifying observation settles. The
/// state also exposes a separate admission wait so queue delay cannot consume
/// the observation execution budget.
#[derive(Debug, Default)]
pub struct RefreshCompletion {
    requested: AtomicU64,
    completed: Mutex<std::collections::HashMap<u64, (RefreshGenerationState, Result<(), String>)>>,
    changed: Condvar,
}

impl RefreshCompletion {
    pub fn request(&self) -> u64 {
        self.request_bounded()
            .expect("refresh generation queue is bounded")
    }

    fn request_bounded(&self) -> Result<u64, String> {
        let mut completed = self
            .completed
            .lock()
            .map_err(|_| "GitHub refresh completion lock poisoned".to_string())?;
        if completed.len() >= MAX_PENDING_REFRESH_GENERATIONS {
            return Err("GITHUB_REFRESH_QUEUE_FULL".into());
        }
        let generation = self.requested.fetch_add(1, AtomicOrdering::AcqRel) + 1;
        completed.insert(generation, (RefreshGenerationState::Pending, Ok(())));
        Ok(generation)
    }

    fn mark_admitted(&self, generation: u64, observation_id: u64) {
        if let Ok(mut completed) = self.completed.lock() {
            if let Some((state, result)) = completed.get_mut(&generation) {
                if *state == RefreshGenerationState::Pending {
                    *state = RefreshGenerationState::Admitted { observation_id };
                    *result = Ok(());
                    self.changed.notify_all();
                }
            }
        }
    }

    fn complete(&self, generation: u64) {
        self.complete_with_result(generation, Ok(()));
    }

    fn complete_with_result(&self, generation: u64, result: Result<(), String>) {
        if let Ok(mut completed) = self.completed.lock() {
            if let Some((state, stored)) = completed.get_mut(&generation) {
                if matches!(
                    *state,
                    RefreshGenerationState::Pending | RefreshGenerationState::Admitted { .. }
                ) {
                    *state = RefreshGenerationState::Settled;
                    *stored = result;
                    self.changed.notify_all();
                }
            }
        }
    }

    fn cancel(&self, generation: u64, error: String) {
        if let Ok(mut completed) = self.completed.lock() {
            if completed.get(&generation).is_some_and(|(state, _)| {
                matches!(
                    state,
                    RefreshGenerationState::Pending | RefreshGenerationState::Admitted { .. }
                )
            }) {
                completed.remove(&generation);
                self.changed.notify_all();
                log::debug!("GitHub refresh generation {generation} cancelled: {error}");
            }
        }
    }

    fn is_active(&self, generation: u64) -> bool {
        self.completed
            .lock()
            .ok()
            .and_then(|completed| completed.get(&generation).map(|(state, _)| *state))
            .is_some_and(|state| {
                matches!(
                    state,
                    RefreshGenerationState::Pending | RefreshGenerationState::Admitted { .. }
                )
            })
    }

    fn wait_for_admission(&self, generation: u64, timeout: Duration) -> Result<(), String> {
        let completed = self
            .completed
            .lock()
            .map_err(|_| "GitHub refresh completion lock poisoned".to_string())?;
        let (completed, result) = self
            .changed
            .wait_timeout_while(completed, timeout, |value| {
                value
                    .get(&generation)
                    .is_some_and(|(state, _)| *state == RefreshGenerationState::Pending)
            })
            .map_err(|_| "GitHub refresh admission wait poisoned".to_string())?;
        match completed
            .get(&generation)
            .map(|(state, result)| (state, result))
        {
            Some((RefreshGenerationState::Admitted { .. }, _)) => Ok(()),
            Some((RefreshGenerationState::Settled, result)) => result.clone(),
            Some((RefreshGenerationState::Pending, _)) if result.timed_out() => {
                Err("GITHUB_REFRESH_BACKPRESSURE".into())
            }
            _ => Err("GITHUB_REFRESH_ADMISSION_ENDED".into()),
        }
    }

    fn was_admitted(&self, generation: u64) -> bool {
        self.completed
            .lock()
            .ok()
            .and_then(|completed| completed.get(&generation).map(|(state, _)| *state))
            .is_some_and(|state| matches!(state, RefreshGenerationState::Admitted { .. }))
    }

    pub fn wait(&self, generation: u64, timeout: Duration) -> Result<(), String> {
        let mut completed = self
            .completed
            .lock()
            .map_err(|_| "GitHub refresh completion lock poisoned".to_string())?;
        let (next, result) = self
            .changed
            .wait_timeout_while(completed, timeout, |value| {
                value
                    .get(&generation)
                    .is_some_and(|(state, _)| *state != RefreshGenerationState::Settled)
            })
            .map_err(|_| "GitHub refresh completion wait poisoned".to_string())?;
        completed = next;
        match completed.remove(&generation) {
            Some((RefreshGenerationState::Settled, result)) => result,
            Some(_) if result.timed_out() => Err(
                "GitHub refresh completion timed out before the requested generation settled"
                    .into(),
            ),
            _ => Err(
                "GitHub refresh completion ended before the requested generation settled".into(),
            ),
        }
    }

    fn pending_state(&self, generation: u64) -> Option<RefreshGenerationState> {
        self.completed
            .lock()
            .ok()
            .and_then(|completed| completed.get(&generation).map(|(state, _)| *state))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ObservationIdentity {
    observation_id: u64,
    project_id: String,
    started_epoch: u64,
}

#[derive(Debug, Clone)]
struct TrackingObservationJob {
    database: DatabaseState,
    project: ProjectRecord,
    identity: ObservationIdentity,
    refreshes: Vec<RefreshScopeRequest>,
}

#[derive(Debug)]
struct TrackingObservationResult {
    job: TrackingObservationJob,
    result: Result<(RemoteTrackingSnapshot, RemoteObservationChange), String>,
    completed_epoch: u64,
}

/// Shared admission/result state machine used by the production polling loop
/// and deterministic native scheduler evidence tests.
#[derive(Debug, Default)]
struct SchedulerLifecycleState {
    next_observation_id: u64,
    next_epoch: u64,
    in_flight: HashMap<String, ObservationIdentity>,
    pending_refreshes: RefreshGenerationCoordinator,
}

impl SchedulerLifecycleState {
    fn enqueue_refresh(&mut self, request: RefreshScopeRequest) {
        self.pending_refreshes.enqueue(request);
    }

    fn is_in_flight(&self, project_id: &str) -> bool {
        self.in_flight.contains_key(project_id)
    }

    fn in_flight_len(&self) -> usize {
        self.in_flight.len()
    }

    fn has_pending_refresh(&self, project_id: &str) -> bool {
        self.pending_refreshes.is_pending_for(project_id)
    }

    fn admit(
        &mut self,
        database: DatabaseState,
        project: ProjectRecord,
        completion: &RefreshCompletion,
    ) -> Option<TrackingObservationJob> {
        if self.in_flight.len() >= TRACKING_WORKER_CAPACITY || self.is_in_flight(&project.id) {
            return None;
        }
        self.next_observation_id = self.next_observation_id.saturating_add(1);
        self.next_epoch = self.next_epoch.saturating_add(1);
        let identity = ObservationIdentity {
            observation_id: self.next_observation_id,
            project_id: project.id.clone(),
            started_epoch: self.next_epoch,
        };
        let refreshes = self
            .pending_refreshes
            .take_for_project(&project.id, |generation| completion.is_active(generation));
        for refresh in &refreshes {
            completion.mark_admitted(refresh.generation, identity.observation_id);
        }
        self.in_flight.insert(project.id.clone(), identity.clone());
        Some(TrackingObservationJob {
            database,
            project,
            identity,
            refreshes,
        })
    }

    fn abort(
        &mut self,
        job: &TrackingObservationJob,
        completion: &RefreshCompletion,
        error: String,
    ) {
        let identity = &job.identity;
        if self
            .in_flight
            .get(&identity.project_id)
            .is_some_and(|current| current == identity)
        {
            self.in_flight.remove(&identity.project_id);
            for generation in job.refreshes.iter().map(|request| request.generation) {
                completion.complete_with_result(generation, Err(error.clone()));
            }
        }
    }

    fn complete(
        &mut self,
        result: TrackingObservationResult,
        completion: &RefreshCompletion,
    ) -> bool {
        let identity = &result.job.identity;
        if self
            .in_flight
            .get(&identity.project_id)
            .is_none_or(|current| current != identity)
        {
            return false;
        }
        self.in_flight.remove(&identity.project_id);
        let outcome = result
            .result
            .as_ref()
            .map(|(snapshot, _)| {
                if snapshot.remote_health == "CURRENT" {
                    Ok(())
                } else {
                    Err(snapshot
                        .error
                        .clone()
                        .unwrap_or_else(|| "GITHUB_REMOTE_VALIDATION_FAILED".into()))
                }
            })
            .unwrap_or_else(|error| Err(error.clone()));
        for refresh in &result.job.refreshes {
            completion.complete_with_result(refresh.generation, outcome.clone());
        }
        true
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
    /// Backward-compatible alias for the content materialization timestamp.
    #[serde(default)]
    pub content_fetched_at: Option<String>,
    /// Last successful identity/branch/HEAD validation timestamp.
    #[serde(default)]
    pub validated_at: Option<String>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RemoteTaskRow {
    pub id: String,
    /// Stable identity for this physical root-TASKS row. `id` remains the
    /// owner-authored explicit task ID used by dependency references/UI.
    #[serde(default)]
    pub canonical_row_id: String,
    pub title: String,
    pub status: String,
    pub source_path: String,
    pub source_line: usize,
    #[serde(default)]
    pub required_actor: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub owner_gate: Option<String>,
    #[serde(default)]
    pub external_wait: Option<String>,
    #[serde(default)]
    pub priority: Option<i64>,
    #[serde(default)]
    pub content_hash: Option<String>,
    #[serde(default = "default_remote_metadata_complete")]
    pub metadata_complete: bool,
}

fn default_remote_metadata_complete() -> bool {
    false
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
        ("formulab", "FormuLab", "Sekiph82/FormuLab", "main"),
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
                    // Seed metadata is migration/bootstrap input only. Never
                    // reactivate or rewrite the user-owned Registry identity.
                    "UPDATE projects SET default_branch=COALESCE(default_branch, ?2), task_source_policy=COALESCE(task_source_policy, ?3), updated_at=?4 WHERE id=?1",
                    params![project_id, branch, GITHUB_TASKS_ONLY_POLICY, now],
                )
                .map_err(db_error)?;
            let repository_updated = transaction
                .execute(
                    "UPDATE repositories SET remote_url=COALESCE(remote_url, ?2), github_owner=COALESCE(github_owner, ?3), github_repo=COALESCE(github_repo, ?4), default_branch=COALESCE(default_branch, ?5), updated_at=?6 WHERE project_id=?1",
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
    // The seed set is not a portfolio allow-list. Registered projects are
    // owned by the Registry and remain visible until an explicit lifecycle
    // operation archives or removes them.
    transaction
        .execute(
            // Preserve M18 resource caches. Only obsolete pre-M18 tracking
            // rows are retired here; branch changes above still invalidate
            // every resource for the affected Registry identity.
            "DELETE FROM github_sync_state WHERE resource_kind LIKE 'GITHUB_TRACKING_%' AND resource_kind <> ?1",
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

/// One M19 hard acceptance horizon for the portfolio. Selected cadence is a
/// scheduler target and does not create a second M19 eligibility contract.
pub fn m19_validation_horizon_seconds() -> u64 {
    M19_HARD_VALIDATION_HORIZON_SECONDS
}

pub fn tracking_refresh_completion_timeout() -> Duration {
    Duration::from_secs(TRACKING_REFRESH_COMPLETION_TIMEOUT_SECONDS)
}

pub fn tracking_failure_backoff_seconds(failure_count: u32, selected: bool) -> u64 {
    let base = refresh_interval_seconds(selected);
    let exponent = failure_count.saturating_sub(1).min(5);
    base.saturating_mul(1_u64 << exponent)
        .min(FAILURE_BACKOFF_MAX_SECONDS)
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
    let content_fetched_at = utc_timestamp();
    let previous = cached(database, project, "")?;
    match fetch_github_head(&repository_name, &branch) {
        Ok(head) => {
            if let Some(snapshot) = previous.as_ref() {
                if same_head_cache_is_reusable(snapshot, &head, &branch) {
                    let refreshed = refresh_same_head_validation(snapshot, utc_timestamp());
                    persist(database, project, &refreshed)?;
                    return Ok((refreshed, RemoteObservationChange::Unchanged));
                }
            }
            match fetch_github_root_tasks(&repository_name, &branch, &head).and_then(|raw| {
                parse_root_tasks_with_lifecycle(
                    &raw,
                    &repository_name,
                    &branch,
                    content_fetched_at.clone(),
                    utc_timestamp(),
                )
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
                    let snapshot = remote_error_from_previous(
                        previous.as_ref(),
                        &repository_name,
                        &branch,
                        Some(head),
                        error,
                        content_fetched_at,
                    );
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
                let snapshot = unavailable(&repository_name, &branch, error, content_fetched_at);
                persist(database, project, &snapshot)?;
                Ok((snapshot, RemoteObservationChange::Changed))
            }
        },
    }
}

/// Same-head reuse is valid only when the durable snapshot contains the
/// materialized task rows required by the current remote-primary contract.
/// Historical snapshots deserialize with an empty `task_rows` default, so a
/// populated cache must be reparsed even when GitHub HEAD has not changed.
pub(crate) fn same_head_cache_is_reusable(
    snapshot: &RemoteTrackingSnapshot,
    fetched_head: &str,
    expected_branch: &str,
) -> bool {
    if snapshot.branch != expected_branch
        || snapshot.remote_head.as_deref() != Some(fetched_head)
        || snapshot.remote_health != "CURRENT"
    {
        return false;
    }

    match snapshot.total_tasks {
        Some(0) => snapshot.task_rows.is_empty(),
        Some(_) => !snapshot.task_rows.is_empty(),
        None => false,
    }
}

pub(crate) fn refresh_same_head_validation(
    snapshot: &RemoteTrackingSnapshot,
    validated_at: String,
) -> RemoteTrackingSnapshot {
    let mut refreshed = snapshot.clone();
    refreshed.validated_at = Some(validated_at.clone());
    refreshed.error = None;
    refreshed.remote_health = "CURRENT".into();
    refreshed
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
    admit_tracking_request()?;
    let mut child = crate::process_policy::background_command("curl.exe")
        .args([
            "-L",
            "--fail",
            "--silent",
            "--show-error",
            "--max-time",
            &TRACKING_HTTP_TIMEOUT.as_secs().to_string(),
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
    let deadline = std::time::Instant::now() + TRACKING_HTTP_TIMEOUT + Duration::from_secs(5);
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
                return Err(format!(
                    "GitHub HTTP observation timed out after {} seconds",
                    TRACKING_HTTP_TIMEOUT_SECONDS + TRACKING_HTTP_PROCESS_GRACE_SECONDS
                ));
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

#[derive(Debug, Default)]
struct RemoteTaskMetadata {
    required_actor: Option<String>,
    dependencies: Vec<String>,
    blockers: Vec<String>,
    owner_gate: Option<String>,
    external_wait: Option<String>,
    priority: Option<i64>,
    complete: bool,
}

fn remote_task_metadata(lines: &[&str], start: usize, end: usize) -> RemoteTaskMetadata {
    let mut metadata = RemoteTaskMetadata {
        complete: true,
        ..Default::default()
    };
    let mut active_label: Option<String> = None;
    for raw in lines
        .iter()
        .skip(start + 1)
        .take(end.saturating_sub(start + 1))
    {
        let trimmed = raw.trim();
        if trimmed.starts_with('#') || task_row(trimmed).is_some() {
            break;
        }
        let indented = raw.chars().next().is_some_and(char::is_whitespace);
        let value = trimmed.trim_start_matches('-').trim();
        if let Some((label, content)) = value.split_once(':') {
            let label = label.trim().to_ascii_lowercase();
            let content = content.trim().trim_matches('`').to_string();
            if content.is_empty() {
                active_label = Some(label);
                continue;
            }
            active_label = None;
            remote_add_metadata(&mut metadata, &label, content);
        } else if indented && active_label.is_some() && !value.is_empty() {
            remote_add_metadata(
                &mut metadata,
                active_label.as_deref().unwrap_or_default(),
                value.to_string(),
            );
        } else if !indented {
            active_label = None;
        }
    }
    metadata
}

fn remote_add_metadata(metadata: &mut RemoteTaskMetadata, label: &str, content: String) {
    match label {
        "owner" | "actor" | "required actor" => {
            metadata.required_actor = normalize_remote_actor(&content);
            if metadata.required_actor.is_none() {
                metadata.complete = false;
            }
        }
        "depends on" | "dependency" | "dependencies" => {
            metadata.dependencies.extend(split_remote_values(&content));
        }
        "blocker" | "blockers" | "blocked by" => {
            metadata.blockers.extend(split_remote_values(&content));
        }
        "owner gate" | "owner decision" | "decision gate" | "gate" => {
            metadata.owner_gate = Some(content);
        }
        "waiting for" | "external" | "external wait" => {
            metadata.external_wait = Some(content);
        }
        "priority" | "task priority" => match content.parse::<i64>() {
            Ok(value) => metadata.priority = Some(value.clamp(-100, 100)),
            Err(_) => metadata.complete = false,
        },
        _ => {}
    }
}

fn normalize_remote_actor(value: &str) -> Option<String> {
    ["Human", "Codex", "Claude", "GPT Audit", "CI", "External"]
        .iter()
        .find(|actor| actor.eq_ignore_ascii_case(value.trim()))
        .map(|actor| (*actor).into())
}

fn split_remote_values(value: &str) -> Vec<String> {
    value
        .split([',', ';'])
        .map(str::trim)
        .filter(|value| !value.is_empty() && !value.eq_ignore_ascii_case("none"))
        .map(ToString::to_string)
        .collect()
}

fn parse_root_tasks(
    raw: &RootTasksRemote,
    repository: &str,
    branch: &str,
    fetched_at: String,
) -> Result<RemoteTrackingSnapshot, String> {
    parse_root_tasks_with_lifecycle(raw, repository, branch, fetched_at.clone(), fetched_at)
}

fn parse_root_tasks_with_lifecycle(
    raw: &RootTasksRemote,
    repository: &str,
    branch: &str,
    content_fetched_at_value: String,
    validated_at_value: String,
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
    let lines = raw.tasks.lines().collect::<Vec<_>>();
    let task_positions = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| task_row(line).map(|_| index))
        .collect::<Vec<_>>();
    for (position, line_index) in task_positions.iter().enumerate() {
        let line = lines[*line_index];
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
                let metadata = remote_task_metadata(
                    &lines,
                    *line_index,
                    task_positions
                        .get(position + 1)
                        .copied()
                        .unwrap_or(lines.len()),
                );
                task_rows.push(RemoteTaskRow {
                    id,
                    canonical_row_id: format!(
                        "{repository}@{branch}:{}:{}:{}",
                        raw.tasks_blob_sha,
                        *line_index + 1,
                        lines[*line_index].trim().to_ascii_lowercase()
                    ),
                    title,
                    status: normalized_status.into(),
                    source_path: "TASKS.md".into(),
                    source_line: *line_index + 1,
                    required_actor: metadata.required_actor,
                    dependencies: metadata.dependencies,
                    blockers: metadata.blockers,
                    owner_gate: metadata.owner_gate,
                    external_wait: metadata.external_wait,
                    priority: metadata.priority,
                    content_hash: Some(raw.tasks_blob_sha.clone()),
                    metadata_complete: metadata.complete,
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
    let content_fetched_at = Some(content_fetched_at_value);
    let fetched_at = content_fetched_at.clone().unwrap_or_default();
    let validated_at = Some(validated_at_value);
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
        content_fetched_at,
        validated_at,
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
            if snapshot.content_fetched_at.is_none() {
                snapshot.content_fetched_at = Some(snapshot.fetched_at.clone());
            }
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
        content_fetched_at: None,
        validated_at: None,
        remote_health: "UNAVAILABLE".into(),
        error: Some(error),
        recent_events: Vec::new(),
        task_rows: Vec::new(),
    }
}

fn remote_error_from_previous(
    previous: Option<&RemoteTrackingSnapshot>,
    repository: &str,
    branch: &str,
    head: Option<String>,
    error: String,
    observed_at: String,
) -> RemoteTrackingSnapshot {
    let mut snapshot = previous
        .cloned()
        .unwrap_or_else(|| unavailable(repository, branch, error.clone(), observed_at.clone()));
    snapshot.remote_head = head.or(snapshot.remote_head);
    snapshot.fetched_at = observed_at;
    snapshot.remote_health = "ERROR".into();
    snapshot.error = Some(error);
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
    refresh_scope: Arc<RefreshScopeGate>,
    refresh_completion: Arc<RefreshCompletion>,
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
            refresh_scope: Arc::new(RefreshScopeGate::default()),
            refresh_completion: Arc::new(RefreshCompletion::default()),
            stop: Arc::new(AtomicBool::new(false)),
            worker: Mutex::new(None),
        };
        let database = manager.database.clone();
        let app_handle = manager.app_handle.clone();
        let selected_project = Arc::clone(&manager.selected_project);
        let refresh_requested = Arc::clone(&manager.refresh_requested);
        let refresh_scope = Arc::clone(&manager.refresh_scope);
        let refresh_completion = Arc::clone(&manager.refresh_completion);
        let stop = Arc::clone(&manager.stop);
        let worker = thread::Builder::new()
            .name("hiveai-github-tracking".into())
            .spawn(move || {
                polling_loop(
                    database,
                    app_handle,
                    selected_project,
                    refresh_requested,
                    refresh_scope,
                    refresh_completion,
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
    }

    pub fn refresh_selected(&self, project_id: String) -> Result<usize, String> {
        if project_id.trim().is_empty() {
            return Err("GITHUB_REFRESH_PROJECT_REQUIRED".into());
        }
        let generation = self.refresh_completion.request_bounded()?;
        if let Err(error) = self
            .refresh_scope
            .request_for_generation(generation, project_id)
        {
            self.refresh_completion
                .complete_with_result(generation, Err(error.clone()));
            return Err(error);
        }
        self.refresh_requested.request();
        if let Err(error) = self.refresh_completion.wait_for_admission(
            generation,
            Duration::from_secs(TRACKING_REFRESH_ADMISSION_TIMEOUT_SECONDS),
        ) {
            self.refresh_completion.cancel(generation, error.clone());
            return Err(error);
        }
        if let Err(error) = self
            .refresh_completion
            .wait(generation, tracking_refresh_completion_timeout())
        {
            self.refresh_completion.cancel(generation, error.clone());
            return Err(error);
        }
        Ok(generation as usize)
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
    refresh_scope: Arc<RefreshScopeGate>,
    refresh_completion: Arc<RefreshCompletion>,
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
    let (job_tx, job_rx) = mpsc::channel::<TrackingObservationJob>();
    let (result_tx, result_rx) = mpsc::channel::<TrackingObservationResult>();
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
                    let Some(job) = job else {
                        continue;
                    };
                    let result = observe_project(&job.database, &job.project);
                    let _ = worker_tx.send(TrackingObservationResult {
                        job,
                        result,
                        completed_epoch: 0,
                    });
                }
            })
            .expect("start GitHub tracking worker");
        workers.push(worker);
    }
    drop(result_tx);
    let mut next_due = HashMap::<String, std::time::Instant>::new();
    let mut failures = HashMap::<String, u32>::new();
    let mut signatures = HashMap::<String, (Option<String>, String)>::new();
    let mut lifecycle = SchedulerLifecycleState::default();
    let mut rotation_cursor = 0usize;
    while !stop.load(Ordering::Acquire) {
        while let Ok(result) = result_rx.try_recv() {
            let project_id = result.job.identity.project_id.clone();
            let snapshot = result
                .result
                .as_ref()
                .ok()
                .map(|(snapshot, _)| snapshot.clone());
            let observation_error = result.result.as_ref().err().cloned();
            if !lifecycle.complete(result, &refresh_completion) {
                continue;
            }
            let selected_for_backoff = selected_project
                .lock()
                .ok()
                .and_then(|value| value.clone())
                .as_deref()
                == Some(project_id.as_str());
            match snapshot {
                Some(snapshot) => {
                    if snapshot.remote_health == "CURRENT" {
                        failures.remove(&project_id);
                    } else {
                        let failure = failures.entry(project_id.clone()).or_insert(0);
                        *failure = failure.saturating_add(1);
                        next_due.insert(
                            project_id.clone(),
                            std::time::Instant::now()
                                + Duration::from_secs(tracking_failure_backoff_seconds(
                                    *failure,
                                    selected_for_backoff,
                                )),
                        );
                    }
                    let signature = (snapshot.remote_head.clone(), snapshot.remote_health.clone());
                    let should_emit = signatures.get(&project_id) != Some(&signature);
                    signatures.insert(project_id.clone(), signature);
                    if should_emit {
                        emit_update(&app_handle, &project_id, &snapshot);
                    }
                    let completion = if snapshot.remote_health == "CURRENT" {
                        Ok(())
                    } else {
                        Err(snapshot
                            .error
                            .unwrap_or_else(|| "GITHUB_REMOTE_VALIDATION_FAILED".into()))
                    };
                    if completion.is_ok() {
                        next_due.insert(
                            project_id.clone(),
                            std::time::Instant::now()
                                + Duration::from_secs(refresh_interval_seconds(
                                    selected_for_backoff,
                                )),
                        );
                    }
                }
                None => {
                    let failure = failures.entry(project_id.clone()).or_insert(0);
                    *failure = failure.saturating_add(1);
                    next_due.insert(
                        project_id.clone(),
                        std::time::Instant::now()
                            + Duration::from_secs(tracking_failure_backoff_seconds(
                                *failure,
                                selected_for_backoff,
                            )),
                    );
                    log::warn!(
                        "GitHub tracking scheduler failed for {project_id}: {}",
                        observation_error
                            .unwrap_or_else(|| "GITHUB_REMOTE_OBSERVATION_FAILED".into())
                    );
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
            for request in refresh_scope.take_all() {
                lifecycle.enqueue_refresh(request.clone());
                next_due.insert(request.project_id, now);
            }
        }
        let mut live_ids = std::collections::HashSet::new();
        let mut projects = projects;
        projects.sort_by(|left, right| left.id.cmp(&right.id));
        if !projects.is_empty() {
            let offset = rotation_cursor % projects.len();
            projects.rotate_left(offset);
            rotation_cursor = rotation_cursor.wrapping_add(1);
        }
        projects.sort_by_key(|project| {
            (
                !lifecycle.has_pending_refresh(&project.id),
                project.id.clone(),
            )
        });
        for project in projects {
            if !is_github_tasks_project(&project) {
                continue;
            }
            live_ids.insert(project.id.clone());
            let interval = Duration::from_secs(refresh_interval_seconds(
                selected.as_deref() == Some(project.id.as_str()),
            ));
            let due = next_due.entry(project.id.clone()).or_insert(now);
            if lifecycle.has_pending_refresh(&project.id) {
                *due = now;
            }
            if *due > now {
                continue;
            }
            if lifecycle.in_flight_len() >= TRACKING_WORKER_CAPACITY
                || lifecycle.is_in_flight(&project.id)
            {
                continue;
            }
            let Some(job) = lifecycle.admit(database.clone(), project.clone(), &refresh_completion)
            else {
                continue;
            };
            if job_tx.send(job.clone()).is_err() {
                lifecycle.abort(
                    &job,
                    &refresh_completion,
                    "GITHUB_REFRESH_WORKER_UNAVAILABLE".into(),
                );
                break;
            }
            *due = now + interval;
        }
        next_due.retain(|id, _| live_ids.contains(id));
        failures.retain(|id, _| live_ids.contains(id));
        signatures.retain(|id, _| live_ids.contains(id));
        thread::sleep(Duration::from_secs(1));
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
    use crate::projects::{
        archive_project, register_project, remove_project, RegisterProjectRequest,
    };
    use tempfile::tempdir;

    #[test]
    fn root_tasks_parser_materializes_current_state_and_exact_counts() {
        let raw = RootTasksRemote {
            head: "0123456789012345678901234567890123456789".into(),
            tasks: "# Demo\n\n## Project Status\n- Current Milestone: M2\n- Current Sprint: M2-S1\n- Current Task: TASK-2 — Current work\n- Current Task Status: IN_PROGRESS\n- Next Task/Action: TASK-3 — Next work\n- Required Actor: CODEX\n\n## Tasks\n- [x] TASK-1 — Done\n  Owner: CODEX\n- [~] TASK-2 — Current work\n  Owner: CLAUDE\n  Depends on: TASK-1\n  Priority: 7\n- [ ] TASK-3 — Next work\n  Owner: HUMAN\n  Waiting for: approval\n".into(),
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
        assert_eq!(
            snapshot.task_rows[1].required_actor.as_deref(),
            Some("Claude")
        );
        assert_eq!(snapshot.task_rows[1].dependencies, vec!["TASK-1"]);
        assert_eq!(snapshot.task_rows[1].priority, Some(7));
        assert_eq!(
            snapshot.task_rows[2].required_actor.as_deref(),
            Some("Human")
        );
        assert_eq!(
            snapshot.task_rows[2].external_wait.as_deref(),
            Some("approval")
        );
        assert!(snapshot.task_rows.iter().all(|row| row.metadata_complete));
        assert!((snapshot.progress_percent.unwrap() - 33.333333333333336).abs() < 0.0001);
        assert_eq!(
            snapshot.latest_commit_message.as_deref(),
            Some("update tracker")
        );
    }

    #[test]
    fn remote_task_metadata_is_per_task_and_malformed_fails_closed() {
        let raw = RootTasksRemote {
            head: "0123456789012345678901234567890123456789".into(),
            tasks: "# Remote\n- Required Actor: CODEX\n\n## Tasks\n- [ ] TASK-A — prerequisite\n  Owner: CODEX\n- [ ] TASK-B — blocked dependent\n  Owner: CLAUDE\n  Depends on: TASK-A\n  Blocker: waiting on TASK-A\n  Waiting for: review\n- [ ] TASK-C — malformed metadata\n  Owner: CODEX\n  Priority: high\n".into(),
            tasks_blob_sha: "sha256:remote".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let snapshot = parse_root_tasks(&raw, "Sekiph82/Bulk-Edit", "main", "now".into()).unwrap();
        assert_eq!(snapshot.task_rows.len(), 3);
        assert_eq!(
            snapshot.task_rows[0].required_actor.as_deref(),
            Some("Codex")
        );
        assert_eq!(
            snapshot.task_rows[1].required_actor.as_deref(),
            Some("Claude")
        );
        assert_eq!(snapshot.task_rows[1].dependencies, vec!["TASK-A"]);
        assert_eq!(snapshot.task_rows[1].blockers, vec!["waiting on TASK-A"]);
        assert_eq!(
            snapshot.task_rows[1].external_wait.as_deref(),
            Some("review")
        );
        assert_eq!(
            snapshot.task_rows[2].required_actor.as_deref(),
            Some("Codex")
        );
        assert!(!snapshot.task_rows[2].metadata_complete);
        assert!(snapshot
            .task_rows
            .iter()
            .all(|row| row.content_hash.as_deref() == Some("sha256:remote")));
    }

    #[test]
    fn remote_duplicate_explicit_ids_keep_distinct_canonical_rows() {
        let raw = RootTasksRemote {
            head: "duplicate-head".into(),
            tasks: "## Tasks\n- [ ] TASK-A — first spelling\n  Owner: CODEX\n- [x] task-a — conflicting completion\n  Owner: CODEX\n- [ ] TASK-B — dependent\n  Owner: CODEX\n  Depends on: TASK-A\n".into(),
            tasks_blob_sha: "sha256:duplicate".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let snapshot = parse_root_tasks(&raw, "Sekiph82/Bulk-Edit", "main", "T0".into()).unwrap();
        assert_eq!(snapshot.task_rows.len(), 3);
        assert_ne!(
            snapshot.task_rows[0].canonical_row_id,
            snapshot.task_rows[1].canonical_row_id
        );
        assert_eq!(snapshot.task_rows[0].id.to_ascii_lowercase(), "task-a");
        assert_eq!(snapshot.task_rows[1].id.to_ascii_lowercase(), "task-a");
        assert_eq!(snapshot.task_rows[1].status, "TASK_COMPLETE");
        assert_eq!(snapshot.validated_at.as_deref(), Some("T0"));
        assert_eq!(snapshot.content_fetched_at.as_deref(), Some("T0"));
    }

    #[test]
    fn same_head_validation_refreshes_only_validation_timestamp_and_clears_error() {
        let raw = RootTasksRemote {
            head: "same-head".into(),
            tasks: "## Tasks\n- [ ] TASK-A — stable task\n  Owner: CODEX\n".into(),
            tasks_blob_sha: "sha256:stable".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let mut snapshot = parse_root_tasks(
            &raw,
            "Sekiph82/H-veAI",
            "main",
            "2026-09-15T00:00:00Z".into(),
        )
        .unwrap();
        snapshot.error = Some("transient old error".into());
        let refreshed = refresh_same_head_validation(&snapshot, "2026-09-16T00:00:01Z".into());
        assert_eq!(
            refreshed.content_fetched_at.as_deref(),
            Some("2026-09-15T00:00:00Z")
        );
        assert_eq!(
            refreshed.validated_at.as_deref(),
            Some("2026-09-16T00:00:01Z")
        );
        assert_eq!(refreshed.fetched_at, snapshot.fetched_at);
        assert_eq!(refreshed.error, None);
    }

    #[test]
    fn ensure_portfolio_bootstraps_exactly_eight_canonical_targets() {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();

        ensure_portfolio(&database).unwrap();
        let projects = list_projects(
            &database,
            ProjectListQuery {
                include_archived: Some(false),
                ..Default::default()
            },
        )
        .unwrap();

        let expected = [
            ("Sekiph82/H-veAI", "main"),
            ("Sekiph82/Bulk-Edit", "main"),
            ("Sekiph82/fmcg-erp-system", "main"),
            ("Sekiph82/FormuLab", "main"),
            ("Sekiph82/PackLab", "main"),
            ("Sekiph82/PackLab-3D", "main"),
            ("Sekiph82/Scrubbots", "main"),
            ("Sekiph82/ScrubBots-Level-Factory", "main"),
        ];
        assert_eq!(projects.len(), expected.len());
        for (repository, branch) in expected {
            let matches = projects
                .iter()
                .filter(|project| {
                    project
                        .repository
                        .as_ref()
                        .map(|value| {
                            format!(
                                "{}/{}",
                                value.github_owner.as_deref().unwrap_or_default(),
                                value.github_repo.as_deref().unwrap_or_default()
                            ) == repository
                                && value.default_branch.as_deref() == Some(branch)
                        })
                        .unwrap_or(false)
                })
                .count();
            assert_eq!(matches, 1, "expected exactly one {repository}@{branch}");
        }
        assert_eq!(
            projects
                .iter()
                .filter(|project| {
                    project
                        .repository
                        .as_ref()
                        .and_then(|value| value.github_repo.as_deref())
                        == Some("FormuLab")
                })
                .count(),
            1
        );
    }

    #[test]
    fn ensure_portfolio_preserves_existing_formulab_identity_and_cache() {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        let local_dir = tempdir().unwrap();
        let old_branch = "feature/laboratory-stability";
        let old_project = register_project(
            &database,
            RegisterProjectRequest {
                path: local_dir.path().to_string_lossy().into_owned(),
                name: Some("FormuLab local workspace".into()),
            },
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "UPDATE projects SET default_branch=?1, task_source_policy=?2 WHERE id=?3",
                params![old_branch, GITHUB_TASKS_ONLY_POLICY, old_project.id],
            )
            .unwrap();
        connection
            .execute(
                "UPDATE repositories SET remote_url=?1, github_owner='Sekiph82', github_repo='FormuLab', default_branch=?2, is_git_repository=1 WHERE project_id=?3",
                params![
                    "https://github.com/Sekiph82/FormuLab.git",
                    old_branch,
                    old_project.id
                ],
            )
            .unwrap();

        let persisted = crate::projects::fetch_project(&database, &old_project.id).unwrap();
        let old_snapshot = parse_root_tasks(
            &RootTasksRemote {
                head: "same-formulab-head".into(),
                tasks: "# FormuLab\n\n## Tasks\n".into(),
                tasks_blob_sha: "sha256:old-branch".into(),
                latest_commit_message: None,
                latest_commit_author: None,
                latest_commit_at: None,
            },
            "Sekiph82/FormuLab",
            old_branch,
            "now".into(),
        )
        .unwrap();
        persist(&database, &persisted, &old_snapshot).unwrap();
        assert_eq!(
            cached_snapshot(&database, &persisted)
                .unwrap()
                .unwrap()
                .branch,
            old_branch
        );
        assert!(!same_head_cache_is_reusable(
            &old_snapshot,
            "same-formulab-head",
            "main"
        ));

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
        let formulab = projects
            .iter()
            .filter(|project| {
                project
                    .repository
                    .as_ref()
                    .and_then(|value| value.github_repo.as_deref())
                    == Some("FormuLab")
            })
            .collect::<Vec<_>>();
        assert_eq!(formulab.len(), 1);
        assert_eq!(formulab[0].id, old_project.id);
        assert_eq!(
            formulab[0]
                .repository
                .as_ref()
                .unwrap()
                .default_branch
                .as_deref(),
            Some(old_branch)
        );
        assert_eq!(formulab[0].normalized_path, persisted.normalized_path);
        assert_eq!(formulab[0].original_path, persisted.original_path);
        assert!(cached_snapshot(&database, formulab[0]).unwrap().is_some());

        let other_repositories = [
            "H-veAI",
            "Bulk-Edit",
            "fmcg-erp-system",
            "PackLab",
            "PackLab-3D",
            "Scrubbots",
            "ScrubBots-Level-Factory",
        ];
        for repository in other_repositories {
            assert_eq!(
                projects
                    .iter()
                    .filter(|project| {
                        project
                            .repository
                            .as_ref()
                            .and_then(|value| value.github_repo.as_deref())
                            == Some(repository)
                    })
                    .count(),
                1,
                "expected exactly one {repository}@main"
            );
        }
    }

    #[test]
    fn legacy_remote_snapshot_without_task_rows_is_not_reusable_at_same_head() {
        let raw = RootTasksRemote {
            head: "0123456789012345678901234567890123456789".into(),
            tasks: "## Project Status\n- Current Task: TASK-1 — Current work\n\n## Tasks\n- [~] TASK-1 — Current work\n".into(),
            tasks_blob_sha: "sha256:test".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let current = parse_root_tasks(&raw, "Sekiph82/demo", "main", "now".into()).unwrap();
        let mut historical_json = serde_json::to_value(&current).unwrap();
        historical_json.as_object_mut().unwrap().remove("taskRows");
        let historical: RemoteTrackingSnapshot = serde_json::from_value(historical_json).unwrap();

        assert_eq!(historical.total_tasks, Some(1));
        assert!(historical.task_rows.is_empty());
        assert!(!same_head_cache_is_reusable(
            &historical,
            raw.head.as_str(),
            "main"
        ));
    }

    #[test]
    fn same_head_cache_reuse_accepts_materialized_populated_and_empty_snapshots() {
        let populated_raw = RootTasksRemote {
            head: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
            tasks: "## Tasks\n- [ ] TASK-1 — Backlog\n".into(),
            tasks_blob_sha: "sha256:populated".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let populated =
            parse_root_tasks(&populated_raw, "Sekiph82/demo", "main", "now".into()).unwrap();
        assert!(same_head_cache_is_reusable(
            &populated,
            populated_raw.head.as_str(),
            "main"
        ));

        let empty_raw = RootTasksRemote {
            head: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".into(),
            tasks: "# Empty project\n\n## Tasks\n".into(),
            tasks_blob_sha: "sha256:empty".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let empty = parse_root_tasks(&empty_raw, "Sekiph82/demo", "main", "now".into()).unwrap();
        assert_eq!(empty.total_tasks, Some(0));
        assert!(empty.task_rows.is_empty());
        assert!(same_head_cache_is_reusable(
            &empty,
            empty_raw.head.as_str(),
            "main"
        ));
    }

    #[test]
    fn reparsing_and_persisting_legacy_snapshot_materializes_rows() {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        ensure_portfolio(&database).unwrap();
        let project =
            crate::projects::fetch_project(&database, "github:Sekiph82/H-veAI@main").unwrap();
        let raw = RootTasksRemote {
            head: "cccccccccccccccccccccccccccccccccccccccc".into(),
            tasks: "## Project Status\n- Current Task: TASK-1 — Current work\n\n## Tasks\n- [~] TASK-1 — Current work\n- [ ] TASK-2 — Next work\n".into(),
            tasks_blob_sha: "sha256:repair".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let reparsed = parse_root_tasks(&raw, "Sekiph82/H-veAI", "main", "now".into()).unwrap();
        let mut historical_json = serde_json::to_value(&reparsed).unwrap();
        historical_json.as_object_mut().unwrap().remove("taskRows");
        let historical_json = serde_json::to_string(&historical_json).unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO github_sync_state (id, project_id, resource_kind, resource_cursor, last_synced_at, metadata_json) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    format!("{}:{REMOTE_TASKS_RESOURCE_KIND}", project.id),
                    &project.id,
                    REMOTE_TASKS_RESOURCE_KIND,
                    raw.head,
                    "now",
                    historical_json,
                ],
            )
            .unwrap();
        let legacy = cached(&database, &project, "").unwrap().unwrap();
        assert!(!same_head_cache_is_reusable(
            &legacy,
            "cccccccccccccccccccccccccccccccccccccccc",
            "main"
        ));

        persist(&database, &project, &reparsed).unwrap();
        let repaired = cached(&database, &project, "").unwrap().unwrap();
        assert_eq!(repaired.task_rows.len(), 2);
        assert!(same_head_cache_is_reusable(
            &repaired,
            "cccccccccccccccccccccccccccccccccccccccc",
            "main"
        ));
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
    fn scheduler_cadence_is_bounded_selected_and_portfolio() {
        assert_eq!(
            refresh_interval_seconds(true),
            SELECTED_PROJECT_REFRESH_SECONDS
        );
        assert_eq!(refresh_interval_seconds(false), PORTFOLIO_REFRESH_SECONDS);
        assert_eq!(SELECTED_PROJECT_REFRESH_SECONDS, 300);
        assert_eq!(PORTFOLIO_REFRESH_SECONDS, 3600);
        assert_eq!(m19_validation_horizon_seconds(), 3680);
        assert_eq!(SELECTED_PROJECT_REFRESH_SECONDS, 300);
    }

    #[test]
    fn failure_backoff_is_monotonic_bounded_and_success_reset_is_zero() {
        let background = (1..=8)
            .map(|failure| tracking_failure_backoff_seconds(failure, false))
            .collect::<Vec<_>>();
        assert!(background.windows(2).all(|pair| pair[1] >= pair[0]));
        assert!(background[..5].windows(2).all(|pair| pair[1] > pair[0]));
        assert_eq!(background[0], PORTFOLIO_REFRESH_SECONDS);
        assert_eq!(
            tracking_failure_backoff_seconds(100, false),
            FAILURE_BACKOFF_MAX_SECONDS
        );
        let selected = tracking_failure_backoff_seconds(1, true);
        assert_eq!(selected, SELECTED_PROJECT_REFRESH_SECONDS);
        assert!(tracking_failure_backoff_seconds(2, true) > selected);
        assert_eq!(
            tracking_failure_backoff_seconds(0, false),
            PORTFOLIO_REFRESH_SECONDS
        );
    }

    #[test]
    fn failed_projects_keep_fair_retry_budget_for_healthy_projects() {
        let failing = (1..=20)
            .map(|failure| tracking_failure_backoff_seconds(failure, false))
            .collect::<Vec<_>>();
        assert_eq!(failing[0], PORTFOLIO_REFRESH_SECONDS);
        assert_eq!(failing[1], PORTFOLIO_REFRESH_SECONDS * 2);
        assert!(failing
            .iter()
            .all(|delay| *delay >= PORTFOLIO_REFRESH_SECONDS));
        assert_eq!(tracking_hourly_math(20, false).admitted_http_stages, 40);
    }

    #[test]
    fn changed_head_refresh_deadline_covers_three_sequential_tracking_stages() {
        assert_eq!(TRACKING_HTTP_TIMEOUT_SECONDS, 20);
        assert_eq!(TRACKING_HTTP_PROCESS_GRACE_SECONDS, 5);
        assert_eq!(TRACKING_MAX_SCOPED_STAGES, 3);
        assert_eq!(TRACKING_REFRESH_COMPLETION_TIMEOUT_SECONDS, 80);
    }

    #[test]
    fn slow_changed_head_work_is_bounded_and_late_completion_cannot_resurrect_timeout() {
        let stage_budget = TRACKING_HTTP_TIMEOUT_SECONDS + TRACKING_HTTP_PROCESS_GRACE_SECONDS;
        let declared_work = Duration::from_secs(TRACKING_MAX_SCOPED_STAGES * stage_budget);
        assert!(declared_work < tracking_refresh_completion_timeout());
        let over_deadline = tracking_refresh_completion_timeout() + Duration::from_secs(1);
        assert!(over_deadline > tracking_refresh_completion_timeout());

        let completion = RefreshCompletion::default();
        let generation = completion.request();
        assert!(completion
            .wait(generation, Duration::from_millis(1))
            .is_err());
        completion.complete(generation);
        assert!(completion.wait(generation, Duration::ZERO).is_err());
    }

    fn scheduler_fixture_project(id: &str) -> ProjectRecord {
        ProjectRecord {
            id: id.into(),
            name: id.into(),
            original_path: id.into(),
            normalized_path: id.into(),
            status: "ACTIVE".into(),
            priority: 0,
            preferred_builder: None,
            preferred_auditor: None,
            task_source_policy: Some(GITHUB_TASKS_ONLY_POLICY.into()),
            preferred_agent_provider: None,
            registered_at: "T0".into(),
            last_validated_at: None,
            repository: None,
        }
    }

    fn scheduler_fixture_result(
        job: TrackingObservationJob,
        health: &str,
        completed_epoch: u64,
    ) -> TrackingObservationResult {
        let raw = RootTasksRemote {
            head: "scheduler-head".into(),
            tasks: "## Tasks\n- [ ] TASK-1 — Scheduler task\n".into(),
            tasks_blob_sha: "sha256:scheduler".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let mut snapshot = parse_root_tasks_with_lifecycle(
            &raw,
            "Sekiph82/scheduler",
            "main",
            "T0".into(),
            "T1".into(),
        )
        .unwrap();
        snapshot.remote_health = health.into();
        snapshot.error = (health == "ERROR").then(|| "sanitized scheduler failure".into());
        TrackingObservationResult {
            job,
            result: Ok((snapshot, RemoteObservationChange::Unchanged)),
            completed_epoch,
        }
    }

    #[test]
    fn changed_head_validation_uses_completion_boundary_across_two_slow_cycles() {
        let raw = RootTasksRemote {
            head: "slow-head".into(),
            tasks: "## Tasks\n- [ ] TASK-1 — Slow task\n".into(),
            tasks_blob_sha: "sha256:slow".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let first = parse_root_tasks_with_lifecycle(
            &raw,
            "Sekiph82/H-veAI",
            "main",
            "T0".into(),
            "T0+80".into(),
        )
        .unwrap();
        assert_eq!(first.content_fetched_at.as_deref(), Some("T0"));
        assert_eq!(first.validated_at.as_deref(), Some("T0+80"));

        let next_due = 80 + PORTFOLIO_REFRESH_SECONDS;
        assert_eq!(next_due, M19_HARD_VALIDATION_HORIZON_SECONDS);
        assert!(next_due - 80 <= M19_HARD_VALIDATION_HORIZON_SECONDS);
        let second = parse_root_tasks_with_lifecycle(
            &raw,
            "Sekiph82/H-veAI",
            "main",
            "T0+3680".into(),
            "T0+3760".into(),
        )
        .unwrap();
        assert_eq!(second.validated_at.as_deref(), Some("T0+3760"));
        assert_eq!(second.content_fetched_at.as_deref(), Some("T0+3680"));
    }

    #[test]
    fn failed_changed_head_validation_preserves_previous_validation_boundary() {
        let raw = RootTasksRemote {
            head: "prior-head".into(),
            tasks: "## Tasks\n- [ ] TASK-1 — Prior task\n".into(),
            tasks_blob_sha: "sha256:prior".into(),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
        };
        let previous = parse_root_tasks_with_lifecycle(
            &raw,
            "Sekiph82/H-veAI",
            "main",
            "T0".into(),
            "T0+80".into(),
        )
        .unwrap();
        let failed = remote_error_from_previous(
            Some(&previous),
            "Sekiph82/H-veAI",
            "main",
            Some("new-head".into()),
            "sanitized failure".into(),
            "T0+3680".into(),
        );
        assert_eq!(failed.remote_health, "ERROR");
        assert_eq!(failed.validated_at, previous.validated_at);
        assert_eq!(failed.content_fetched_at, previous.content_fetched_at);
    }

    #[test]
    fn production_scheduler_preexisting_inflight_observation_cannot_settle_manual_generation() {
        let database = DatabaseState::initialize(tempdir().unwrap().path().to_path_buf()).unwrap();
        let project = scheduler_fixture_project("project-a");
        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let old_job = scheduler
            .admit(database.clone(), project.clone(), &completion)
            .unwrap();
        let generation = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation,
            project_id: project.id.clone(),
            accepted_epoch: generation,
        });
        assert!(scheduler.complete(scheduler_fixture_result(old_job, "CURRENT", 2), &completion));
        assert_eq!(
            completion.pending_state(generation),
            Some(RefreshGenerationState::Pending)
        );
        let new_job = scheduler.admit(database, project, &completion).unwrap();
        assert_eq!(
            new_job
                .refreshes
                .iter()
                .map(|item| item.generation)
                .collect::<Vec<_>>(),
            vec![generation]
        );
        assert!(completion.was_admitted(generation));
        assert!(scheduler.complete(scheduler_fixture_result(new_job, "CURRENT", 4), &completion));
        assert!(completion.wait(generation, Duration::ZERO).is_ok());
    }

    #[test]
    fn production_scheduler_isolates_a_b_and_same_scope_coalesces_truthfully() {
        let database = DatabaseState::initialize(tempdir().unwrap().path().to_path_buf()).unwrap();
        let project_a = scheduler_fixture_project("project-a");
        let project_b = scheduler_fixture_project("project-b");
        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let generation_a = completion.request();
        let generation_b = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation: generation_a,
            project_id: project_a.id.clone(),
            accepted_epoch: generation_a,
        });
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation: generation_b,
            project_id: project_b.id.clone(),
            accepted_epoch: generation_b,
        });
        let job_a = scheduler
            .admit(database.clone(), project_a.clone(), &completion)
            .unwrap();
        let job_b = scheduler
            .admit(database.clone(), project_b.clone(), &completion)
            .unwrap();
        assert_eq!(job_a.refreshes[0].generation, generation_a);
        assert_eq!(job_b.refreshes[0].generation, generation_b);
        assert!(scheduler.complete(scheduler_fixture_result(job_b, "ERROR", 3), &completion));
        assert!(completion.wait(generation_b, Duration::ZERO).is_err());
        assert!(completion.pending_state(generation_a).is_some());
        assert!(scheduler.complete(scheduler_fixture_result(job_a, "CURRENT", 4), &completion));
        assert!(completion.wait(generation_a, Duration::ZERO).is_ok());

        let generation_one = completion.request();
        let generation_two = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation: generation_one,
            project_id: project_a.id.clone(),
            accepted_epoch: generation_one,
        });
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation: generation_two,
            project_id: project_a.id.clone(),
            accepted_epoch: generation_two,
        });
        let coalesced = scheduler.admit(database, project_a, &completion).unwrap();
        assert_eq!(coalesced.refreshes.len(), 2);
        assert!(scheduler.complete(
            scheduler_fixture_result(coalesced, "CURRENT", 6),
            &completion
        ));
        assert!(completion.wait(generation_one, Duration::ZERO).is_ok());
        assert!(completion.wait(generation_two, Duration::ZERO).is_ok());
    }

    #[test]
    fn production_scheduler_saturated_workers_bound_manual_admission_and_execution_budget() {
        let database = DatabaseState::initialize(tempdir().unwrap().path().to_path_buf()).unwrap();
        let projects = (0..TRACKING_WORKER_CAPACITY)
            .map(|index| scheduler_fixture_project(&format!("background-{index}")))
            .collect::<Vec<_>>();
        let manual = scheduler_fixture_project("manual-project");
        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let mut in_flight = projects
            .into_iter()
            .map(|project| {
                scheduler
                    .admit(database.clone(), project, &completion)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(scheduler.in_flight_len(), TRACKING_WORKER_CAPACITY);
        let generation = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation,
            project_id: manual.id.clone(),
            accepted_epoch: generation,
        });
        assert!(scheduler
            .admit(database.clone(), manual.clone(), &completion)
            .is_none());
        for (index, job) in in_flight.drain(..).enumerate() {
            assert!(scheduler.complete(
                scheduler_fixture_result(job, "CURRENT", index as u64 + 2),
                &completion
            ));
        }
        let admitted = scheduler.admit(database, manual, &completion).unwrap();
        assert_eq!(admitted.refreshes[0].generation, generation);
        assert!(completion.was_admitted(generation));
        assert!(
            TRACKING_REFRESH_ADMISSION_TIMEOUT_SECONDS
                >= TRACKING_REFRESH_COMPLETION_TIMEOUT_SECONDS
        );
        assert!(scheduler.complete(
            scheduler_fixture_result(admitted, "CURRENT", 10),
            &completion
        ));
        assert!(completion.wait(generation, Duration::ZERO).is_ok());
    }

    fn lifecycle_evidence_row(
        generation: u64,
        project_id: &str,
        job: &TrackingObservationJob,
        completed_epoch: u64,
        pre_existing_in_flight: bool,
        queue_outcome: &str,
        result_health: &str,
        generation_result: &str,
        history_recording_permitted: bool,
        diagnostic: Option<&str>,
    ) -> serde_json::Value {
        serde_json::json!({
            "generationId": generation,
            "projectId": project_id,
            "requestAcceptedEpoch": generation,
            "qualifyingObservationId": job.identity.observation_id,
            "observationStartEpoch": job.identity.started_epoch,
            "observationCompletionEpoch": completed_epoch,
            "workerAdmissionState": "ADMITTED",
            "workerAdmissionEpoch": job.identity.started_epoch,
            "preExistingInFlight": pre_existing_in_flight,
            "queueAdmissionOutcome": queue_outcome,
            "resultHealth": result_health,
            "validatedAtBefore": "T0+80",
            "validatedAtAfter": (result_health == "CURRENT").then_some("T1+80"),
            "nextDueEpoch": completed_epoch + PORTFOLIO_REFRESH_SECONDS,
            "freshnessHorizonSeconds": M19_HARD_VALIDATION_HORIZON_SECONDS,
            "generationResult": generation_result,
            "historyRecordingPermitted": history_recording_permitted,
            "diagnostic": diagnostic,
        })
    }

    #[test]
    fn v07_r02_production_scheduler_lifecycle_evidence_is_measured() {
        let database = DatabaseState::initialize(tempdir().unwrap().path().to_path_buf()).unwrap();
        let project_a = scheduler_fixture_project("project-a");
        let project_b = scheduler_fixture_project("project-b");
        let mut rows = Vec::new();

        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let old_job = scheduler
            .admit(database.clone(), project_a.clone(), &completion)
            .unwrap();
        let generation = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation,
            project_id: project_a.id.clone(),
            accepted_epoch: generation,
        });
        assert!(scheduler.complete(scheduler_fixture_result(old_job, "CURRENT", 2), &completion));
        let qualifying = scheduler
            .admit(database.clone(), project_a.clone(), &completion)
            .unwrap();
        rows.push(lifecycle_evidence_row(
            generation,
            &project_a.id,
            &qualifying,
            4,
            true,
            "WAITED_FOR_PREEXISTING_JOB",
            "CURRENT",
            "COMPLETED",
            true,
            None,
        ));
        assert!(scheduler.complete(
            scheduler_fixture_result(qualifying, "CURRENT", 4),
            &completion
        ));
        assert!(completion.wait(generation, Duration::ZERO).is_ok());

        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let generation_a = completion.request();
        let generation_b = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation: generation_a,
            project_id: project_a.id.clone(),
            accepted_epoch: generation_a,
        });
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation: generation_b,
            project_id: project_b.id.clone(),
            accepted_epoch: generation_b,
        });
        let job_a = scheduler
            .admit(database.clone(), project_a.clone(), &completion)
            .unwrap();
        let job_b = scheduler
            .admit(database.clone(), project_b.clone(), &completion)
            .unwrap();
        rows.push(lifecycle_evidence_row(
            generation_a,
            &project_a.id,
            &job_a,
            6,
            false,
            "ADMITTED_INDEPENDENT_SCOPE",
            "CURRENT",
            "COMPLETED",
            true,
            None,
        ));
        rows.push(lifecycle_evidence_row(
            generation_b,
            &project_b.id,
            &job_b,
            7,
            false,
            "ADMITTED_INDEPENDENT_SCOPE",
            "ERROR",
            "FAILED",
            false,
            Some("sanitized scheduler failure"),
        ));
        assert!(scheduler.complete(scheduler_fixture_result(job_b, "ERROR", 7), &completion));
        assert!(scheduler.complete(scheduler_fixture_result(job_a, "CURRENT", 6), &completion));
        assert!(completion.wait(generation_a, Duration::ZERO).is_ok());
        assert!(completion.wait(generation_b, Duration::ZERO).is_err());

        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let generation_one = completion.request();
        let generation_two = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation: generation_one,
            project_id: project_a.id.clone(),
            accepted_epoch: generation_one,
        });
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation: generation_two,
            project_id: project_a.id.clone(),
            accepted_epoch: generation_two,
        });
        let job = scheduler
            .admit(database.clone(), project_a.clone(), &completion)
            .unwrap();
        rows.push(lifecycle_evidence_row(
            generation_one,
            &project_a.id,
            &job,
            9,
            false,
            "COALESCED_SAME_QUALIFYING_OBSERVATION",
            "CURRENT",
            "COMPLETED",
            true,
            None,
        ));
        rows.push(lifecycle_evidence_row(
            generation_two,
            &project_a.id,
            &job,
            9,
            false,
            "COALESCED_SAME_QUALIFYING_OBSERVATION",
            "CURRENT",
            "COMPLETED",
            true,
            None,
        ));
        assert_eq!(job.refreshes.len(), 2);
        assert!(scheduler.complete(scheduler_fixture_result(job, "CURRENT", 9), &completion));
        assert!(completion.wait(generation_one, Duration::ZERO).is_ok());
        assert!(completion.wait(generation_two, Duration::ZERO).is_ok());

        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let background = (0..TRACKING_WORKER_CAPACITY)
            .map(|index| scheduler_fixture_project(&format!("saturated-{index}")))
            .collect::<Vec<_>>();
        let mut jobs = background
            .into_iter()
            .map(|project| {
                scheduler
                    .admit(database.clone(), project, &completion)
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let generation = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation,
            project_id: "manual-project".into(),
            accepted_epoch: generation,
        });
        let manual = scheduler_fixture_project("manual-project");
        assert!(scheduler
            .admit(database.clone(), manual.clone(), &completion)
            .is_none());
        for (index, job) in jobs.drain(..).enumerate() {
            assert!(scheduler.complete(
                scheduler_fixture_result(job, "CURRENT", 20 + index as u64),
                &completion
            ));
        }
        let manual_job = scheduler
            .admit(database.clone(), manual.clone(), &completion)
            .unwrap();
        rows.push(lifecycle_evidence_row(
            generation,
            &manual.id,
            &manual_job,
            25,
            true,
            "ADMITTED_AFTER_BOUNDED_QUEUE_WAIT",
            "CURRENT",
            "COMPLETED",
            true,
            None,
        ));
        assert!(scheduler.complete(
            scheduler_fixture_result(manual_job, "CURRENT", 25),
            &completion
        ));
        assert!(completion.wait(generation, Duration::ZERO).is_ok());

        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let slow_project = scheduler_fixture_project("slow-changed-head");
        let generation = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation,
            project_id: slow_project.id.clone(),
            accepted_epoch: generation,
        });
        let slow_job = scheduler
            .admit(database.clone(), slow_project.clone(), &completion)
            .unwrap();
        rows.push(lifecycle_evidence_row(
            generation,
            &slow_project.id,
            &slow_job,
            1080,
            false,
            "ADMITTED",
            "CURRENT",
            "COMPLETED_WITHIN_80S_EXECUTION_BUDGET",
            true,
            None,
        ));
        assert!(scheduler.complete(
            scheduler_fixture_result(slow_job, "CURRENT", 1080),
            &completion
        ));
        assert!(completion.wait(generation, Duration::ZERO).is_ok());

        let completion = RefreshCompletion::default();
        let mut scheduler = SchedulerLifecycleState::default();
        let overdue_project = scheduler_fixture_project("over-deadline");
        let generation = completion.request();
        scheduler.enqueue_refresh(RefreshScopeRequest {
            generation,
            project_id: overdue_project.id.clone(),
            accepted_epoch: generation,
        });
        let overdue_job = scheduler
            .admit(database, overdue_project.clone(), &completion)
            .unwrap();
        completion.cancel(generation, "GITHUB_REFRESH_EXECUTION_TIMEOUT".into());
        rows.push(lifecycle_evidence_row(
            generation,
            &overdue_project.id,
            &overdue_job,
            1081,
            false,
            "ADMITTED",
            "CURRENT",
            "TIMED_OUT_EXECUTION",
            false,
            Some("GITHUB_REFRESH_EXECUTION_TIMEOUT"),
        ));
        assert!(scheduler.complete(
            scheduler_fixture_result(overdue_job, "CURRENT", 1081),
            &completion
        ));
        assert!(completion.wait(generation, Duration::ZERO).is_err());

        assert_eq!(rows.len(), 8);
        println!("M19_V07_R02_LIFECYCLE_JSON={}", serde_json::to_string(&serde_json::json!({
            "schema": "M19_V07_R02_TRACKING_LIFECYCLE_MATRIX_V1",
            "source": "production SchedulerLifecycleState admission/in-flight/result transitions",
            "workerCapacity": TRACKING_WORKER_CAPACITY,
            "executionBudgetSeconds": TRACKING_REFRESH_COMPLETION_TIMEOUT_SECONDS,
            "admissionBudgetSeconds": TRACKING_REFRESH_ADMISSION_TIMEOUT_SECONDS,
            "freshnessHorizonSeconds": M19_HARD_VALIDATION_HORIZON_SECONDS,
            "selectedSchedulerTargetSeconds": SELECTED_PROJECT_REFRESH_SECONDS,
            "rows": rows
        })).unwrap());
    }

    #[test]
    fn hourly_math_matches_real_eight_nine_ten_twenty_portfolios() {
        for projects in [8usize, 9, 10, 20] {
            let idle = tracking_hourly_math(projects, false);
            assert_eq!(idle.background_observations, projects);
            assert_eq!(idle.same_head_http_stages, projects);
            assert_eq!(idle.changed_head_http_stages, projects * 3);
            assert_eq!(idle.admitted_http_stages, (projects * 3).min(40));
        }
        let selected = tracking_hourly_math(20, true);
        assert_eq!(selected.selected_observations, 12);
        assert_eq!(selected.background_observations, 19);
        assert_eq!(selected.changed_head_http_stages, 31 * 3);
        assert_eq!(selected.admitted_http_stages, 40);
    }

    #[test]
    fn tracking_governor_resets_only_at_injected_hour_boundary() {
        let start = std::time::Instant::now();
        let mut governor = TrackingRequestGovernor {
            window_started: start,
            requests: 0,
        };
        for _ in 0..TRACKING_REQUESTS_PER_HOUR {
            assert!(governor.admit_at(start));
        }
        assert!(!governor.admit_at(start + Duration::from_secs(3599)));
        assert!(governor.admit_at(start + Duration::from_secs(3600)));
        assert_eq!(governor.requests, 1);
    }

    #[test]
    fn manual_refresh_scope_targets_one_project() {
        let gate = RefreshScopeGate::default();
        gate.request("github:Sekiph82/Bulk-Edit@main".into());
        assert_eq!(
            gate.take().as_deref(),
            Some("github:Sekiph82/Bulk-Edit@main")
        );
        assert_eq!(gate.take(), None);
    }

    #[test]
    fn refresh_scope_queue_preserves_generation_project_binding() {
        let gate = RefreshScopeGate::default();
        gate.request_for_generation(11, "project-a".into()).unwrap();
        gate.request_for_generation(12, "project-b".into()).unwrap();
        assert_eq!(
            gate.take_all(),
            vec![
                RefreshScopeRequest {
                    generation: 11,
                    project_id: "project-a".into(),
                    accepted_epoch: 11,
                },
                RefreshScopeRequest {
                    generation: 12,
                    project_id: "project-b".into(),
                    accepted_epoch: 12,
                }
            ]
        );
    }

    #[test]
    fn scheduler_generation_coordinator_settles_only_declared_project_scope() {
        let completion = std::sync::Arc::new(RefreshCompletion::default());
        let generation_a = completion.request();
        let generation_b = completion.request();
        let mut coordinator = RefreshGenerationCoordinator::default();
        coordinator.enqueue(RefreshScopeRequest {
            generation: generation_a,
            project_id: "project-a".into(),
            accepted_epoch: generation_a,
        });
        coordinator.enqueue(RefreshScopeRequest {
            generation: generation_b,
            project_id: "project-b".into(),
            accepted_epoch: generation_b,
        });
        assert_eq!(coordinator.settle_project("project-b"), vec![generation_b]);
        completion.complete(generation_b);
        assert!(completion.wait(generation_b, Duration::ZERO).is_ok());
        assert!(coordinator.is_pending_for("project-a"));
        assert!(completion
            .completed
            .lock()
            .unwrap()
            .get(&generation_a)
            .is_some_and(|(_, result)| result.is_ok()));
        assert_eq!(coordinator.settle_project("project-a"), vec![generation_a]);
        completion.complete(generation_a);
        assert!(completion.wait(generation_a, Duration::ZERO).is_ok());
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
    fn refresh_generations_wait_for_completion_and_preserve_order() {
        let completion = std::sync::Arc::new(RefreshCompletion::default());
        let first = completion.request();
        let second = completion.request();
        assert_eq!((first, second), (1, 2));
        let waiter = std::sync::Arc::clone(&completion);
        let handle = std::thread::spawn(move || waiter.wait(second, Duration::from_secs(2)));
        std::thread::sleep(Duration::from_millis(40));
        assert!(
            !handle.is_finished(),
            "queued refresh must not look completed"
        );
        completion.complete(first);
        std::thread::sleep(Duration::from_millis(40));
        assert!(
            !handle.is_finished(),
            "first generation cannot settle the second"
        );
        completion.complete(second);
        assert!(handle.join().unwrap().is_ok());
    }

    #[test]
    fn refresh_generation_wait_has_a_bounded_timeout() {
        let completion = RefreshCompletion::default();
        let generation = completion.request();
        let result = completion.wait(generation, Duration::from_millis(20));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("timed out"));
    }

    #[test]
    fn refresh_generation_failure_is_returned_to_the_correct_waiter() {
        let completion = RefreshCompletion::default();
        let first = completion.request();
        let second = completion.request();
        completion.complete_with_result(first, Err("project-a failed".into()));
        assert_eq!(
            completion.wait(first, Duration::ZERO).unwrap_err(),
            "project-a failed"
        );
        assert!(completion.wait(second, Duration::ZERO).is_err());
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
    fn ensure_portfolio_preserves_non_seed_projects_until_explicit_archive() {
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
        assert_eq!(active.len(), 9);
        assert!(active.iter().any(|project| project.id == extra.id));
        assert_eq!(
            crate::projects::fetch_project(&database, &extra.id)
                .unwrap()
                .status,
            "ACTIVE"
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

        crate::projects::archive_project(&database, &extra.id).unwrap();
        ensure_portfolio(&database).unwrap();
        assert_eq!(
            crate::projects::fetch_project(&database, &extra.id)
                .unwrap()
                .status,
            "ARCHIVED"
        );
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
            8
        );
    }

    #[test]
    fn ensure_portfolio_keeps_ninth_and_tenth_projects_across_refresh_and_restart() {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        ensure_portfolio(&database).unwrap();
        let ninth_dir = tempdir().unwrap();
        let tenth_dir = tempdir().unwrap();
        let ninth = register_project(
            &database,
            RegisterProjectRequest {
                path: ninth_dir.path().to_string_lossy().into_owned(),
                name: Some("Ninth project".into()),
            },
        )
        .unwrap();
        let tenth = register_project(
            &database,
            RegisterProjectRequest {
                path: tenth_dir.path().to_string_lossy().into_owned(),
                name: Some("Tenth project".into()),
            },
        )
        .unwrap();
        let duplicate = register_project(
            &database,
            RegisterProjectRequest {
                path: ninth_dir.path().to_string_lossy().into_owned(),
                name: Some("Duplicate ninth project".into()),
            },
        )
        .unwrap();
        assert_eq!(duplicate.id, ninth.id);

        ensure_portfolio(&database).unwrap();
        let active = list_projects(
            &database,
            ProjectListQuery {
                include_archived: Some(false),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(active.len(), 10);
        assert!(active.iter().any(|project| project.id == ninth.id));
        assert!(active.iter().any(|project| project.id == tenth.id));

        drop(database);
        let restarted = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        ensure_portfolio(&restarted).unwrap();
        let after_restart = list_projects(
            &restarted,
            ProjectListQuery {
                include_archived: Some(false),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(after_restart.len(), 10);
        assert!(after_restart.iter().any(|project| project.id == ninth.id));
        assert!(after_restart.iter().any(|project| project.id == tenth.id));
    }

    #[test]
    fn ensure_portfolio_preserves_m18_resource_caches() {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        ensure_portfolio(&database).unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO github_sync_state (id, project_id, resource_kind, resource_cursor, last_synced_at, metadata_json) VALUES ('m18-cache', 'github:Sekiph82/H-veAI@main', 'GITHUB_REPOSITORY', 'main', 'now', '{}')",
                [],
            )
            .unwrap();
        ensure_portfolio(&database).unwrap();
        assert_eq!(
            database
                .open_connection()
                .unwrap()
                .query_row(
                    "SELECT COUNT(*) FROM github_sync_state WHERE resource_kind='GITHUB_REPOSITORY'",
                    [],
                    |row| row.get::<_, i64>(0),
                )
                .unwrap(),
            1
        );
    }

    #[test]
    fn ensure_portfolio_preserves_seed_settings_identity_archive_and_remove() {
        let database_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(database_dir.path().to_path_buf()).unwrap();
        ensure_portfolio(&database).unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "UPDATE projects SET task_source_policy='REGISTRY_CUSTOM', default_branch='release' WHERE id='github:Sekiph82/H-veAI@main'",
                [],
            )
            .unwrap();
        connection
            .execute(
                "UPDATE repositories SET remote_url='https://example.invalid/owner/project.git', github_owner='owner', github_repo='project', default_branch='release', is_git_repository=0 WHERE project_id='github:Sekiph82/H-veAI@main'",
                [],
            )
            .unwrap();
        ensure_portfolio(&database).unwrap();
        let preserved =
            crate::projects::fetch_project(&database, "github:Sekiph82/H-veAI@main").unwrap();
        assert_eq!(
            preserved.task_source_policy.as_deref(),
            Some("REGISTRY_CUSTOM")
        );
        assert_eq!(
            preserved
                .repository
                .as_ref()
                .and_then(|repo| repo.github_owner.as_deref()),
            Some("owner")
        );
        assert_eq!(
            preserved
                .repository
                .as_ref()
                .and_then(|repo| repo.github_repo.as_deref()),
            Some("project")
        );
        assert_eq!(
            preserved
                .repository
                .as_ref()
                .and_then(|repo| repo.default_branch.as_deref()),
            Some("release")
        );
        assert!(!preserved.repository.as_ref().unwrap().is_git_repository);

        archive_project(&database, "github:Sekiph82/H-veAI@main").unwrap();
        ensure_portfolio(&database).unwrap();
        assert_eq!(
            crate::projects::fetch_project(&database, "github:Sekiph82/H-veAI@main")
                .unwrap()
                .status,
            "ARCHIVED"
        );

        remove_project(&database, "github:Sekiph82/Bulk-Edit@main").unwrap();
        ensure_portfolio(&database).unwrap();
        assert!(
            crate::projects::fetch_project(&database, "github:Sekiph82/Bulk-Edit@main").is_err()
        );
        let exclusion: i64 = database.open_connection().unwrap().query_row(
            "SELECT COUNT(*) FROM github_project_exclusions WHERE lower(repository)=lower('Sekiph82/Bulk-Edit') AND branch='main'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(exclusion, 1);
    }
}
