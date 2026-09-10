use crate::db::DatabaseState;
use crate::git_engine::{self, GitSnapshotRequest, RepositoryHealth};
use crate::github_tracking::{self, RemoteTrackingSnapshot};
use crate::projects::{fetch_project, refresh_repository_metadata, ProjectRecord};
use crate::{project_dashboard, task_intelligence, workflow};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub const CONTROL_PLANE_SCHEMA: &str = "hiveai-project-control-plane/v1";
pub const PROJECT_JSON: &str = ".hiveai/PROJECT.json";
pub const RULES_MD: &str = ".hiveai/RULES.md";
pub const STATE_JSON: &str = ".hiveai/STATE.json";
pub const HANDOFF_MD: &str = ".hiveai/HANDOFF.md";
pub const EVENTS_JSONL: &str = ".hiveai/EVENTS.jsonl";
pub const EVENT_INDEX_JSON: &str = ".hiveai/EVENT_INDEX.json";
const MAX_FILE_BYTES: usize = 256 * 1024;
const MAX_EVENTS: usize = 128;
const MAX_EVENT_BYTES: usize = 16 * 1024;
const MAX_EVENT_TAIL_BYTES: u64 = 2 * 1024 * 1024;
const MAX_EVENT_INDEX_IDS: usize = 4096;
pub const EVENT_INDEX_HORIZON: &str =
    "newest 4096 event IDs, reconciled from a bounded 2 MiB EVENTS tail";
const WORKFLOW_STATES: &[&str] = &[
    "IDLE",
    "READY",
    "IN_PROGRESS",
    "AWAITING_AUDIT",
    "CHANGES_REQUIRED",
    "BLOCKED",
    "WAITING_OWNER",
    "COMPLETE",
    "NEEDS_RECONCILIATION",
    "BACKLOG",
    "PLANNING_REQUIRED",
    "PROMPT_REQUIRED",
    "PROMPT_READY",
    "READY_FOR_IMPLEMENTATION",
    "BUILDER_RUNNING",
    "IMPLEMENTATION_COMPLETE",
    "AUDIT_REQUIRED",
    "AUDIT_RUNNING",
    "AUDIT_PASSED",
    "VERIFY_REQUIRED",
    "VERIFY_RUNNING",
    "TASK_COMPLETE",
    "AUDIT_FAILED",
    "FIX_REQUIRED",
    "RE_AUDIT_REQUIRED",
    "WAITING_HUMAN",
    "WAITING_EXTERNAL",
    "DESIGN_GATE",
];

#[cfg(test)]
thread_local! {
    static FAIL_NEXT_EVENT_INDEX_PERSISTENCE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
fn fail_next_event_index_persistence() {
    FAIL_NEXT_EVENT_INDEX_PERSISTENCE.with(|value| value.set(true));
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryIdentity {
    pub owner: String,
    pub name: String,
    #[serde(default)]
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDocument {
    pub schema: String,
    pub project_key: String,
    pub display_name: String,
    #[serde(deserialize_with = "deserialize_repository_identity")]
    pub repository: RepositoryIdentity,
    pub canonical_task_source: String,
    #[serde(default, alias = "eventSource")]
    pub events: String,
    #[serde(default, rename = "eventSources")]
    pub event_sources: Vec<String>,
    // Kept readable for backward compatibility, but never emitted into the
    // portable repository contract.
    #[serde(default, skip_serializing)]
    pub local_registry_id: Option<String>,
    #[serde(default)]
    pub governance: Option<ProjectGovernance>,
    #[serde(default, alias = "rulesSource")]
    pub rules: Option<String>,
    #[serde(default, alias = "stateSource")]
    pub state: Option<String>,
    #[serde(default, alias = "handoffSource")]
    pub handoff: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectGovernance {
    #[serde(default)]
    pub builder_may_mutate_handoff: Option<bool>,
    #[serde(default)]
    pub handoff_automation_allowed: Option<bool>,
    #[serde(default)]
    pub handoff_policy: Option<HandoffPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HandoffPolicy {
    Automated,
    Manual,
    OwnerOnly,
    OwnerApproved,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StateDocument {
    pub schema: String,
    pub project_key: String,
    pub workflow_state: String,
    #[serde(default)]
    pub milestone: Option<String>,
    #[serde(default)]
    pub cycle: Option<String>,
    #[serde(default)]
    pub current_task_id: Option<String>,
    #[serde(default)]
    pub current_task_title: Option<String>,
    #[serde(default)]
    pub required_actor: Option<String>,
    #[serde(default)]
    pub next_action: Option<String>,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub progress_percent: Option<u8>,
    #[serde(default)]
    pub progress_scope: Option<String>,
    #[serde(default)]
    pub updated_by: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HandoffDocument {
    pub schema: String,
    pub project_key: String,
    pub resume_pointer: String,
    #[serde(default)]
    pub current_task_id: Option<String>,
    #[serde(default)]
    pub current_task_title: Option<String>,
    #[serde(default)]
    pub current_milestone: Option<String>,
    #[serde(default)]
    pub current_cycle: Option<String>,
    #[serde(default)]
    pub required_actor: Option<String>,
    #[serde(default)]
    pub next_action: Option<String>,
    #[serde(default)]
    pub last_audit: Option<String>,
    #[serde(default)]
    pub last_agent_session: Option<String>,
    #[serde(default)]
    pub owner: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

fn deserialize_repository_identity<'de, D>(deserializer: D) -> Result<RepositoryIdentity, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;
    let raw = match value {
        serde_json::Value::String(value) => value,
        serde_json::Value::Object(_) => {
            return serde_json::from_value(value).map_err(serde::de::Error::custom)
        }
        _ => {
            return Err(serde::de::Error::custom(
                "repository must be a bounded string or object",
            ))
        }
    };
    parse_repository_identity(&raw)
        .ok_or_else(|| serde::de::Error::custom("repository must be owner/name or a GitHub URL"))
}

fn parse_repository_identity(raw: &str) -> Option<RepositoryIdentity> {
    let trimmed = raw.trim().trim_end_matches('/');
    let path = if let Some(value) = trimmed.strip_prefix("https://github.com/") {
        value
    } else if let Some(value) = trimmed.strip_prefix("http://github.com/") {
        value
    } else if let Some(value) = trimmed.strip_prefix("git@github.com:") {
        value
    } else {
        trimmed
    };
    let path = path.trim_end_matches(".git");
    let mut parts = path.split('/');
    let owner = parts.next()?.trim();
    let name = parts.next()?.trim();
    if parts.next().is_some()
        || owner.is_empty()
        || name.is_empty()
        || owner.contains([':', '\\', ' '])
        || name.contains([':', '\\', ' '])
    {
        return None;
    }
    Some(RepositoryIdentity {
        owner: owner.to_string(),
        name: name.to_string(),
        branch: Some("main".into()),
    })
}

fn normalize_state_value(value: serde_json::Value) -> Result<StateDocument, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "STATE.json must be an object".to_string())?;
    let get_string = |keys: &[&str]| {
        keys.iter()
            .find_map(|key| object.get(*key))
            .and_then(|value| value.as_str().map(str::to_string))
    };
    let progress_percent = object
        .get("progressPercent")
        .and_then(|value| value.as_u64())
        .or_else(|| {
            object
                .get("progress")
                .and_then(|value| value.get("percent"))
                .and_then(|value| value.as_f64())
                .map(|value| value.round() as u64)
        })
        .and_then(|value| u8::try_from(value).ok())
        .filter(|value| *value <= 100);
    Ok(StateDocument {
        schema: get_string(&["schema"]).unwrap_or_else(|| CONTROL_PLANE_SCHEMA.into()),
        project_key: get_string(&["projectKey"]).unwrap_or_default(),
        workflow_state: get_string(&["workflowState", "status"]).unwrap_or_else(|| "IDLE".into()),
        milestone: get_string(&["milestone", "currentMilestone"]),
        cycle: get_string(&["cycle", "currentSprint", "currentCycle"]),
        current_task_id: get_string(&["currentTaskId", "taskId"]),
        current_task_title: get_string(&["currentTaskTitle", "taskTitle"]),
        required_actor: get_string(&["requiredActor"]),
        next_action: get_string(&["nextAction"]),
        blockers: object
            .get("blockers")
            .and_then(|value| value.as_array())
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default(),
        progress_percent,
        progress_scope: get_string(&["progressScope"]),
        updated_by: get_string(&["updatedBy"]),
        updated_at: get_string(&["updatedAt"]),
    })
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EventRecord {
    #[serde(rename = "eventId", alias = "event_id")]
    pub event_id: String,
    #[serde(rename = "type", alias = "eventType", alias = "event_type")]
    pub event_type: String,
    pub project_key: String,
    #[serde(rename = "at", alias = "occurredAt", alias = "occurred_at")]
    pub occurred_at: String,
    pub actor: String,
    pub summary: String,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct CanonicalEventRecord {
    schema: String,
    #[serde(rename = "eventId")]
    event_id: String,
    project_key: String,
    #[serde(rename = "type")]
    event_type: String,
    #[serde(rename = "at")]
    occurred_at: String,
    actor: String,
    task_id: Option<String>,
    workflow_state: Option<String>,
    summary: String,
    commit: Option<String>,
    audit_id: Option<String>,
    session_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct EventIndexDocument {
    schema: String,
    event_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SessionResultClaim {
    pub provider: String,
    pub session_id: Option<String>,
    pub task_id: Option<String>,
    pub state: String,
    pub final_response: Option<String>,
    pub summary: Option<String>,
    pub changed_files: Vec<String>,
    pub tests: Vec<String>,
    pub commit: Option<String>,
    pub requested_workflow_transition: Option<String>,
    pub captured_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitControlPlaneState {
    pub local_status: String,
    pub remote_status: String,
    pub sync_status: String,
    pub branch: Option<String>,
    pub head_sha: Option<String>,
    pub upstream: Option<String>,
    pub ahead: Option<u64>,
    pub behind: Option<u64>,
    pub dirty: bool,
    pub conflicted: bool,
    pub last_remote_observation_at: Option<String>,
    pub last_remote_observation_status: String,
    pub last_remote_observation_error: Option<String>,
    pub last_remote_observed_upstream: Option<String>,
    pub last_remote_observed_ahead: Option<u64>,
    pub last_remote_observed_behind: Option<u64>,
    pub last_remote_observed_diverged: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TruthSyncState {
    pub generation: i64,
    pub materialized_generation: i64,
    pub status: String,
    pub revision: Option<String>,
    pub trigger: Option<String>,
    pub error: Option<String>,
    pub attempted_at: Option<String>,
    pub completed_at: Option<String>,
    pub retry_count: u32,
}

impl Default for TruthSyncState {
    fn default() -> Self {
        Self {
            generation: 0,
            materialized_generation: 0,
            status: "CURRENT".into(),
            revision: None,
            trigger: None,
            error: None,
            attempted_at: None,
            completed_at: None,
            retry_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlPlaneSnapshot {
    pub schema: String,
    pub project_id: String,
    pub project_key: Option<String>,
    pub display_name: String,
    pub adopted: bool,
    pub health: String,
    pub workflow_state: Option<String>,
    pub canonical_task_source: Option<String>,
    pub current_task_id: Option<String>,
    pub current_task_title: Option<String>,
    pub current_milestone: Option<String>,
    pub current_cycle: Option<String>,
    pub required_actor: Option<String>,
    pub remote_repository: Option<String>,
    pub session_result: Option<SessionResultClaim>,
    pub auto_fast_forward_enabled: bool,
    pub next_action: Option<String>,
    pub blockers: Vec<String>,
    pub progress_percent: Option<u8>,
    pub resume_pointer: Option<String>,
    pub event_count: usize,
    pub last_event_at: Option<String>,
    pub git: GitControlPlaneState,
    pub truth_sync: TruthSyncState,
    pub source_precedence: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectTruth {
    pub project_id: String,
    pub current_task_id: Option<String>,
    pub current_task_title: Option<String>,
    pub current_task_status: Option<String>,
    pub current_milestone: Option<String>,
    pub current_cycle: Option<String>,
    pub workflow_state: Option<String>,
    pub required_actor: Option<String>,
    pub next_action: Option<String>,
    pub blockers: Vec<String>,
    pub progress_percent: Option<u8>,
    pub progress_scope: Option<String>,
    pub authority_source: String,
    pub provenance: Vec<String>,
    pub reconciliation_state: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlPlaneSummary {
    pub status: String,
    pub health: String,
    pub project_key: Option<String>,
    pub workflow_state: Option<String>,
    pub current_task_id: Option<String>,
    pub next_action: Option<String>,
    pub sync_status: String,
    pub adopted: bool,
    pub truth_sync: TruthSyncState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GitSyncPlan {
    pub action: String,
    pub safe: bool,
    pub reason: String,
    pub fetch_required: bool,
    pub merge_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LocalRepairPlan {
    pub action: String,
    pub safe: bool,
    pub reason: String,
    pub preserves_local_data: bool,
    pub target_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalControlPlaneStatus {
    Missing,
    Malformed,
    UnsupportedSchema,
    Adopted,
}

#[derive(Debug, Clone)]
struct PhysicalControlPlaneEvidence {
    status: PhysicalControlPlaneStatus,
    schema: Option<String>,
    revision: Option<String>,
    last_event_at: Option<String>,
}

/// Probe the on-disk contract without consulting the registry projection.
/// The registry may be stale; the portable files are the adoption authority.
pub fn probe_physical_control_plane(project_root: &Path) -> PhysicalControlPlaneStatus {
    physical_control_plane_evidence(project_root).status
}

fn physical_control_plane_evidence(project_root: &Path) -> PhysicalControlPlaneEvidence {
    let missing = || PhysicalControlPlaneEvidence {
        status: PhysicalControlPlaneStatus::Missing,
        schema: None,
        revision: None,
        last_event_at: None,
    };
    if !project_root.is_dir() {
        return missing();
    }
    let project_path = project_root.join(PROJECT_JSON);
    let raw = match read_bounded(&project_path, MAX_FILE_BYTES) {
        Ok(raw) => raw,
        Err(error) if error == "NOT_FOUND" => return missing(),
        Err(_) => {
            return PhysicalControlPlaneEvidence {
                status: PhysicalControlPlaneStatus::Malformed,
                schema: None,
                revision: None,
                last_event_at: None,
            }
        }
    };
    let document: ProjectDocument = match serde_json::from_str(&raw) {
        Ok(value) => value,
        Err(_) => {
            return PhysicalControlPlaneEvidence {
                status: PhysicalControlPlaneStatus::Malformed,
                schema: None,
                revision: Some(format!("{:x}", Sha256::digest(raw.as_bytes()))),
                last_event_at: None,
            }
        }
    };
    if document.schema != CONTROL_PLANE_SCHEMA {
        return PhysicalControlPlaneEvidence {
            status: PhysicalControlPlaneStatus::UnsupportedSchema,
            schema: Some(document.schema),
            revision: Some(format!("{:x}", Sha256::digest(raw.as_bytes()))),
            last_event_at: None,
        };
    }
    let key = document.project_key.trim();
    let repository_valid = !document.repository.owner.trim().is_empty()
        && !document.repository.name.trim().is_empty()
        && !key.is_empty()
        && key.len() <= 128
        && !key.contains(['/', '\\', ':'])
        && key != "."
        && key != "..";
    let canonical = safe_relative_path(&document.canonical_task_source);
    let rules = document.rules.as_deref().unwrap_or(RULES_MD);
    let state = document.state.as_deref().unwrap_or(STATE_JSON);
    let handoff = document.handoff.as_deref().unwrap_or(HANDOFF_MD);
    let events = if document.events.trim().is_empty() {
        None
    } else {
        safe_relative_path(&document.events)
    };
    let pointers_valid = [rules, state, handoff]
        .into_iter()
        .all(|path| safe_relative_path(path).is_some())
        && events.is_some();
    let files_valid = canonical
        .as_deref()
        .is_some_and(|path| project_root.join(path).is_file())
        && [rules, state, handoff].into_iter().all(|path| {
            safe_relative_path(path).is_some_and(|value| project_root.join(value).is_file())
        })
        && events
            .as_deref()
            .is_some_and(|path| project_root.join(path).is_file());
    if !repository_valid || !pointers_valid || !files_valid {
        return PhysicalControlPlaneEvidence {
            status: PhysicalControlPlaneStatus::Malformed,
            schema: Some(CONTROL_PLANE_SCHEMA.into()),
            revision: Some(format!("{:x}", Sha256::digest(raw.as_bytes()))),
            last_event_at: None,
        };
    }
    let event_path = project_root.join(events.as_deref().unwrap_or(EVENTS_JSONL));
    let mut warnings = Vec::new();
    let last_event_at = read_events(&event_path, &mut warnings)
        .last()
        .map(|event| event.occurred_at.clone());
    PhysicalControlPlaneEvidence {
        status: PhysicalControlPlaneStatus::Adopted,
        schema: Some(CONTROL_PLANE_SCHEMA.into()),
        revision: Some(format!("{:x}", Sha256::digest(raw.as_bytes()))),
        last_event_at,
    }
}

/// Reconcile DB adoption metadata from the verified physical contract. This
/// is intentionally called by startup/reconcile maintenance, never by reads.
pub fn converge_physical_adoption(
    database: &DatabaseState,
    project_id: &str,
) -> Result<PhysicalControlPlaneStatus, String> {
    let project = fetch_project(database, project_id)?;
    if github_tracking::is_github_tasks_project(&project) {
        return Ok(PhysicalControlPlaneStatus::Missing);
    }
    if project.status == "ARCHIVED" {
        return Ok(PhysicalControlPlaneStatus::Missing);
    }
    let evidence = physical_control_plane_evidence(Path::new(&project.normalized_path));
    let (status, schema, revision, sync_status, last_event) = match &evidence.status {
        PhysicalControlPlaneStatus::Adopted => (
            "ADOPTED",
            evidence.schema.as_deref(),
            evidence.revision.as_deref(),
            "CURRENT",
            evidence.last_event_at.as_deref(),
        ),
        PhysicalControlPlaneStatus::Malformed | PhysicalControlPlaneStatus::UnsupportedSchema => (
            "MALFORMED",
            evidence.schema.as_deref(),
            evidence.revision.as_deref(),
            "DEGRADED",
            None,
        ),
        PhysicalControlPlaneStatus::Missing => ("UNADOPTED", None, None, "DEGRADED", None),
    };
    let mut connection = database.open_connection()?;
    let tx = connection
        .transaction()
        .map_err(|error| format!("begin physical adoption convergence: {error}"))?;
    tx.execute(
        "UPDATE projects SET control_plane_status=?2, control_plane_schema=?3, control_plane_revision=?4, control_plane_last_event_at=?5, control_plane_sync_status=?6, updated_at=?7 WHERE id=?1",
        rusqlite::params![project_id, status, schema, revision, last_event, sync_status, crate::time::utc_timestamp()],
    )
    .map_err(|error| format!("persist physical adoption projection: {error}"))?;
    if evidence.status == PhysicalControlPlaneStatus::Adopted {
        tx.execute(
            "UPDATE projects SET truth_generation=1, truth_sync_status='PENDING', truth_sync_trigger='GENERATION_BOOTSTRAP', truth_sync_error=NULL, truth_sync_attempted_at=NULL, truth_sync_completed_at=NULL, truth_sync_retry_count=0 WHERE id=?1 AND truth_generation=0 AND truth_materialized_generation=0 AND truth_sync_status='CURRENT'",
            [project_id],
        )
        .map_err(|error| format!("bootstrap physically adopted truth: {error}"))?;
    } else {
        // v22 could have pre-armed a row from stale DB metadata before this
        // physical verifier existed. Undo only an unmaterialized bootstrap;
        // never erase an already materialized generation.
        tx.execute(
            "UPDATE projects SET truth_generation=0, truth_materialized_generation=0, truth_sync_status='CURRENT', truth_sync_trigger='PHYSICAL_ADOPTION_REJECTED', truth_sync_error=NULL, truth_sync_attempted_at=NULL, truth_sync_completed_at=NULL, truth_sync_retry_count=0 WHERE id=?1 AND truth_generation=1 AND truth_materialized_generation=0 AND truth_sync_trigger='GENERATION_BOOTSTRAP'",
            [project_id],
        )
        .map_err(|error| format!("clear unverified generation bootstrap: {error}"))?;
    }
    tx.commit()
        .map_err(|error| format!("commit physical adoption convergence: {error}"))?;
    Ok(evidence.status)
}

pub fn snapshot(
    database: &DatabaseState,
    project_id: &str,
) -> Result<ControlPlaneSnapshot, String> {
    let project = fetch_project(database, project_id)?;
    if github_tracking::is_github_tasks_project(&project) {
        let remote = github_tracking::refresh_project(database, &project)
            .unwrap_or_else(|error| github_tracking::unavailable_for_project(&project, error));
        return Ok(remote_control_plane_snapshot(&project, &remote));
    }
    let initial_truth_sync = truth_sync_status(database, project_id);
    let materialization = if truth_sync_is_current(&initial_truth_sync) {
        Ok(None)
    } else {
        ProjectTruthMaterializer::materialize(database, project_id, "SNAPSHOT_RECOVERY").map(Some)
    };
    let mut result = resolve_project(&project)?;
    if let Err(error) = materialization {
        result
            .warnings
            .push(format!("truth materialization unavailable: {error}"));
    }
    let truth = ProjectTruthResolver::resolve(database, project_id)?;
    result.current_task_id = truth.current_task_id.clone();
    result.current_task_title = truth.current_task_title.clone();
    result.current_milestone = truth.current_milestone.clone();
    result.current_cycle = truth.current_cycle.clone();
    result.workflow_state = truth.workflow_state.clone();
    result.required_actor = truth.required_actor.clone();
    result.next_action = truth.next_action.clone();
    result.blockers = truth.blockers.clone();
    result.progress_percent = truth.progress_percent;
    result.warnings.extend(truth.warnings.clone());
    result.warnings.sort();
    result.warnings.dedup();
    if result.adopted && truth.reconciliation_state == "NEEDS_RECONCILIATION" {
        result.health = "NEEDS_RECONCILIATION".into();
    }
    result.auto_fast_forward_enabled = database
        .open_connection()
        .ok()
        .and_then(|connection| {
            connection
                .query_row(
                    "SELECT control_plane_auto_ff FROM projects WHERE id=?1",
                    [project_id],
                    |row| row.get::<_, i64>(0),
                )
                .ok()
        })
        .is_some_and(|value| value == 1);
    if let Ok(git) = git_engine::snapshot(
        database,
        GitSnapshotRequest {
            project_id: project_id.to_string(),
            persist: Some(false),
        },
    ) {
        result.git = git_state(&git);
    }
    overlay_remote_observation(database, project_id, &mut result.git);
    result.truth_sync = read_truth_sync(database, project_id).unwrap_or_default();
    result.health = health_for(&result, &result.git);
    Ok(result)
}

/// Upgrade only the bounded control-plane files. Canonical task files, handoff prose,
/// audits, prompts, logs, and project-specific governance are deliberately untouched.
pub fn upgrade_control_plane(project: &ProjectRecord) -> Result<bool, String> {
    if github_tracking::is_github_tasks_project(project) {
        return Ok(false);
    }
    let root = Path::new(&project.normalized_path);
    if project.status != "ACTIVE" || !root.is_dir() {
        return Ok(false);
    }
    let project_path = root.join(PROJECT_JSON);
    let raw = match read_bounded(&project_path, MAX_FILE_BYTES) {
        Ok(raw) => raw,
        Err(error) if error == "NOT_FOUND" => return Ok(false),
        Err(error) => return Err(error),
    };
    let mut document: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|error| format!("PROJECT.json is malformed: {error}"))?;
    let object = document
        .as_object_mut()
        .ok_or_else(|| "PROJECT.json must be an object".to_string())?;
    let already_current = object.get("schema").and_then(|value| value.as_str())
        == Some(CONTROL_PLANE_SCHEMA)
        && object
            .get("repository")
            .is_some_and(serde_json::Value::is_object)
        && object.get("events").is_some()
        && object.get("rules").is_some()
        && object.get("state").is_some()
        && object.get("handoff").is_some();
    if already_current {
        return reconcile_control_plane(project);
    }
    let repository = object
        .get("repository")
        .and_then(|value| value.as_str())
        .and_then(parse_repository_identity)
        .or_else(|| {
            project.repository.as_ref().and_then(|repository| {
                repository
                    .github_owner
                    .as_ref()
                    .zip(repository.github_repo.as_ref())
                    .map(|(owner, name)| RepositoryIdentity {
                        owner: owner.clone(),
                        name: name.clone(),
                        branch: repository
                            .current_branch
                            .clone()
                            .or_else(|| Some("main".into())),
                    })
            })
        })
        .ok_or_else(|| "PROJECT.json repository identity is ambiguous".to_string())?;
    let canonical = object
        .get("canonicalTaskSource")
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
        .or_else(|| {
            ["TASKS.md", "tasks.md", "docs/FORMULAB_V1_TASK_TRACKER.md"]
                .iter()
                .find(|candidate| root.join(candidate).is_file())
                .map(|candidate| (*candidate).to_string())
        })
        .ok_or_else(|| "canonical task source is ambiguous".to_string())?;
    object.insert("schema".into(), CONTROL_PLANE_SCHEMA.into());
    object.insert(
        "repository".into(),
        serde_json::to_value(repository).map_err(|error| error.to_string())?,
    );
    object.insert("canonicalTaskSource".into(), canonical.into());
    migrate_pointer(object, "rules", "rulesSource", RULES_MD);
    migrate_pointer(object, "state", "stateSource", STATE_JSON);
    migrate_pointer(object, "handoff", "handoffSource", HANDOFF_MD);
    if object.get("events").is_none() {
        if let Some(value) = object.remove("eventSource") {
            if value.is_string() {
                object.insert("events".into(), value);
            } else {
                return Err("eventSource must be a scalar path".into());
            }
        } else {
            object.insert("events".into(), EVENTS_JSONL.into());
        }
    } else if !object
        .get("events")
        .is_some_and(serde_json::Value::is_string)
    {
        return Err("events must be a scalar canonical event path".into());
    }
    if let Some(value) = object.get("eventSources") {
        if !value.is_array()
            || !value
                .as_array()
                .is_some_and(|items| items.iter().all(serde_json::Value::is_string))
        {
            return Err("eventSources must be an array of paths".into());
        }
    } else {
        object.insert("eventSources".into(), serde_json::json!([]));
    }
    let project_json =
        serde_json::to_string_pretty(&document).map_err(|error| error.to_string())?;
    atomic_replace(&project_path, &raw, &project_json)?;

    let state_path = root.join(STATE_JSON);
    if let Ok(state_raw) = read_bounded(&state_path, MAX_FILE_BYTES) {
        if let Ok(mut state) = serde_json::from_str::<serde_json::Value>(&state_raw) {
            if let Some(state_object) = state.as_object_mut() {
                state_object.insert("schema".into(), CONTROL_PLANE_SCHEMA.into());
                if state_object.get("workflowState").is_none() {
                    state_object.insert("workflowState".into(), "NEEDS_RECONCILIATION".into());
                }
                let normalized =
                    serde_json::to_string_pretty(&state).map_err(|error| error.to_string())?;
                if normalized != state_raw {
                    atomic_replace(&state_path, &state_raw, &normalized)?;
                }
            }
        }
    }
    reconcile_control_plane(project)?;
    Ok(true)
}

fn migrate_pointer(
    object: &mut serde_json::Map<String, serde_json::Value>,
    current: &str,
    legacy: &str,
    fallback: &str,
) {
    if object.get(current).is_none() {
        if let Some(value) = object.remove(legacy) {
            object.insert(current.into(), value);
        } else {
            object.insert(current.into(), fallback.into());
        }
    }
}

fn atomic_replace(path: &Path, expected: &str, contents: &str) -> Result<(), String> {
    let current = read_bounded(path, MAX_FILE_BYTES)?;
    if current != expected {
        return Err(format!(
            "control-plane file changed during migration: {}",
            path.display()
        ));
    }
    let temp = path.with_extension(format!("upgrade-{}", std::process::id()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
        .map_err(|error| format!("create migration temp file: {error}"))?;
    file.write_all(contents.as_bytes())
        .map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    drop(file);
    fs::rename(&temp, path).map_err(|error| {
        let _ = fs::remove_file(&temp);
        format!("replace migrated control-plane file: {error}")
    })
}

pub fn summary(snapshot: &ControlPlaneSnapshot) -> ControlPlaneSummary {
    ControlPlaneSummary {
        status: if snapshot.adopted {
            "ADOPTED"
        } else {
            "UNADOPTED"
        }
        .into(),
        health: snapshot.health.clone(),
        project_key: snapshot.project_key.clone(),
        workflow_state: snapshot.workflow_state.clone(),
        current_task_id: snapshot.current_task_id.clone(),
        next_action: snapshot.next_action.clone(),
        sync_status: snapshot.git.sync_status.clone(),
        adopted: snapshot.adopted,
        truth_sync: snapshot.truth_sync.clone(),
    }
}

pub fn reconcile_project(
    database: &DatabaseState,
    project_id: &str,
) -> Result<ControlPlaneSnapshot, String> {
    let project = refresh_repository_metadata(database, project_id)?;
    if github_tracking::is_github_tasks_project(&project) {
        return snapshot(database, project_id);
    }
    converge_physical_adoption(database, project_id)?;
    materialize_project_truth(database, project_id, "EXPLICIT_RECONCILE")?;
    snapshot(database, project_id)
}

fn bounded_text(value: &str, limit: usize) -> String {
    value.chars().take(limit).collect()
}

fn stable_project_key(project: &ProjectRecord) -> Result<String, String> {
    let path = Path::new(&project.normalized_path).join(PROJECT_JSON);
    if let Ok(raw) = read_bounded(&path, MAX_FILE_BYTES) {
        let document: ProjectDocument = serde_json::from_str(&raw)
            .map_err(|error| format!("PROJECT.json is malformed: {error}"))?;
        let key = document.project_key.trim();
        if !key.is_empty() && key != project.id {
            return Ok(bounded_text(key, 128));
        }
    }
    let source = project
        .repository
        .as_ref()
        .and_then(|repository| repository.github_repo.as_deref())
        .unwrap_or(&project.name);
    let mut slug = String::new();
    for character in source.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    if !slug.is_empty() {
        return Ok(bounded_text(slug, 128));
    }
    let digest = Sha256::digest(project.normalized_path.as_bytes());
    Ok(format!("project-{digest:x}").chars().take(128).collect())
}

fn read_truth_sync(database: &DatabaseState, project_id: &str) -> Result<TruthSyncState, String> {
    let connection = database.open_connection()?;
    connection
        .query_row(
            "SELECT truth_generation, truth_materialized_generation, truth_sync_status, truth_sync_revision, truth_sync_trigger, truth_sync_error, truth_sync_attempted_at, truth_sync_completed_at, truth_sync_retry_count FROM projects WHERE id=?1",
            [project_id],
            |row| {
                Ok(TruthSyncState {
                    generation: row.get(0)?,
                    materialized_generation: row.get(1)?,
                    status: row.get(2)?,
                    revision: row.get(3)?,
                    trigger: row.get(4)?,
                    error: row.get(5)?,
                    attempted_at: row.get(6)?,
                    completed_at: row.get(7)?,
                    retry_count: row.get::<_, i64>(8)?.max(0) as u32,
                })
            },
        )
        .map_err(|error| error.to_string())
}

fn truth_sync_is_current(state: &TruthSyncState) -> bool {
    state.status == "CURRENT"
        && state.generation == state.materialized_generation
        && state.error.is_none()
}

fn current_truth_generation(database: &DatabaseState, project_id: &str) -> Result<i64, String> {
    let connection = database.open_connection()?;
    connection
        .query_row(
            "SELECT truth_generation FROM projects WHERE id=?1",
            [project_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())
}

/// Arm durable truth synchronization in the same SQLite transaction as the
/// domain mutation. The generation is the authoritative dirty intent.
pub fn mark_truth_dirty_tx(
    tx: &rusqlite::Transaction<'_>,
    project_id: &str,
    trigger: &str,
) -> Result<i64, String> {
    let updated = tx
        .execute(
            "UPDATE projects SET truth_generation=truth_generation+1, truth_sync_status='PENDING', truth_sync_trigger=?2, truth_sync_error=NULL, truth_sync_attempted_at=?3 WHERE id=?1",
            rusqlite::params![project_id, bounded_text(trigger, 96), crate::time::utc_timestamp()],
        )
        .map_err(|error| error.to_string())?;
    if updated != 1 {
        return Err("PROJECT_NOT_FOUND_FOR_TRUTH_GENERATION".into());
    }
    tx.query_row(
        "SELECT truth_generation FROM projects WHERE id=?1",
        [project_id],
        |row| row.get(0),
    )
    .map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TruthSyncCasOutcome {
    Applied { attempt: i64 },
    AlreadyCurrent,
    Superseded,
    InvalidGeneration,
}

fn mark_truth_sync_pending(
    database: &DatabaseState,
    project_id: &str,
    generation: i64,
    revision: &str,
    trigger: &str,
) -> Result<TruthSyncCasOutcome, String> {
    let connection = database.open_connection()?;
    let updated = connection
        .execute(
            "UPDATE projects SET truth_sync_status='PENDING', truth_sync_revision=?3, truth_sync_trigger=?4, truth_sync_error=NULL, truth_sync_attempted_at=?5, truth_sync_retry_count=CASE WHEN truth_sync_revision=?3 THEN MIN(truth_sync_retry_count+1, 1024) ELSE 1 END WHERE id=?1 AND truth_generation=?2 AND truth_materialized_generation <= truth_generation",
            rusqlite::params![project_id, generation, bounded_text(revision, 128), bounded_text(trigger, 96), crate::time::utc_timestamp()],
        )
        .map_err(|error| error.to_string())?;
    if updated == 1 {
        let attempt = connection
            .query_row(
                "SELECT truth_sync_retry_count FROM projects WHERE id=?1",
                [project_id],
                |row| row.get(0),
            )
            .map_err(|error| error.to_string())?;
        return Ok(TruthSyncCasOutcome::Applied { attempt });
    }
    let state = connection
        .query_row(
            "SELECT truth_generation, truth_materialized_generation, truth_sync_status, truth_sync_error FROM projects WHERE id=?1",
            [project_id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                ))
            },
        )
        .optional()
        .map_err(|error| error.to_string())?;
    let Some((current_generation, materialized_generation, status, error)) = state else {
        return Ok(TruthSyncCasOutcome::InvalidGeneration);
    };
    if current_generation != generation {
        return Ok(TruthSyncCasOutcome::Superseded);
    }
    if status == "CURRENT" && materialized_generation == generation && error.is_none() {
        return Ok(TruthSyncCasOutcome::AlreadyCurrent);
    }
    Ok(TruthSyncCasOutcome::Superseded)
}

fn mark_truth_sync_current(
    database: &DatabaseState,
    project_id: &str,
    generation: i64,
    attempt: i64,
    revision: &str,
) -> Result<TruthSyncCasOutcome, String> {
    let connection = database.open_connection()?;
    let updated = connection
        .execute(
            "UPDATE projects SET truth_sync_status='CURRENT', truth_sync_revision=?3, truth_materialized_generation=?2, truth_sync_error=NULL, truth_sync_completed_at=?5 WHERE id=?1 AND truth_generation=?2 AND truth_sync_retry_count=?4 AND truth_materialized_generation <= truth_generation",
            rusqlite::params![project_id, generation, bounded_text(revision, 128), attempt, crate::time::utc_timestamp()],
        )
        .map_err(|error| error.to_string())
        ?;
    if updated == 1 {
        return Ok(TruthSyncCasOutcome::Applied { attempt });
    }
    let state = connection
        .query_row(
            "SELECT truth_generation, truth_materialized_generation, truth_sync_status, truth_sync_retry_count, truth_sync_error FROM projects WHERE id=?1",
            [project_id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, Option<String>>(4)?,
                ))
            },
        )
        .optional()
        .map_err(|error| error.to_string())?;
    let Some((current_generation, materialized_generation, status, current_attempt, error)) = state
    else {
        return Ok(TruthSyncCasOutcome::InvalidGeneration);
    };
    if current_generation != generation || current_attempt != attempt {
        return Ok(TruthSyncCasOutcome::Superseded);
    }
    if status == "CURRENT" && materialized_generation == generation && error.is_none() {
        return Ok(TruthSyncCasOutcome::AlreadyCurrent);
    }
    Ok(TruthSyncCasOutcome::Superseded)
}

fn truth_revision(truth: &ProjectTruth, project_key: &str) -> String {
    let bytes = serde_json::to_vec(&(project_key, truth)).unwrap_or_default();
    format!("{:x}", Sha256::digest(bytes))
}

fn mark_truth_sync_degraded(
    database: &DatabaseState,
    project_id: &str,
    generation: i64,
    attempt: i64,
    revision: &str,
    trigger: &str,
    error: &str,
) -> Result<TruthSyncCasOutcome, String> {
    if let Ok(connection) = database.open_connection() {
        let updated = connection.execute(
            "UPDATE projects SET truth_sync_status='DEGRADED', truth_sync_revision=?3, truth_sync_trigger=?4, truth_sync_error=?5, truth_sync_attempted_at=?6 WHERE id=?1 AND truth_generation=?2 AND truth_sync_retry_count=?7",
            rusqlite::params![project_id, generation, bounded_text(revision, 128), bounded_text(trigger, 96), bounded_text(error, 512), crate::time::utc_timestamp(), attempt],
        ).map_err(|error| error.to_string())?;
        return Ok(if updated == 1 {
            TruthSyncCasOutcome::Applied { attempt }
        } else {
            TruthSyncCasOutcome::Superseded
        });
    }
    Err("TRUTH_SYNC_DATABASE_UNAVAILABLE".into())
}

pub fn truth_sync_status(database: &DatabaseState, project_id: &str) -> TruthSyncState {
    read_truth_sync(database, project_id).unwrap_or_default()
}

pub fn retry_pending_truth_sync(database: &DatabaseState) -> usize {
    let Ok(connection) = database.open_connection() else {
        return 0;
    };
    let ids = connection
        .prepare("SELECT id FROM projects WHERE status='ACTIVE' AND (truth_materialized_generation < truth_generation OR truth_sync_status IN ('PENDING','DEGRADED')) ORDER BY truth_generation ASC LIMIT 8")
        .and_then(|mut statement| statement.query_map([], |row| row.get::<_, String>(0)).and_then(|rows| rows.collect::<Result<Vec<_>, _>>()))
        .unwrap_or_default();
    ids.into_iter()
        .filter(|id| {
            if fetch_project(database, id)
                .map(|project| github_tracking::is_github_tasks_project(&project))
                .unwrap_or(false)
            {
                return false;
            }
            materialize_project_truth(database, id, "TRUTH_SYNC_RETRY").is_ok()
                && truth_sync_is_current(&truth_sync_status(database, id))
        })
        .count()
}

pub fn materialize_best_effort(database: &DatabaseState, project_id: &str, trigger: &str) {
    let _ = materialize_project_truth(database, project_id, trigger);
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MaterializationResult {
    pub project_id: String,
    pub trigger: String,
    pub changed: bool,
    pub state_written: bool,
    pub handoff_written: bool,
    pub event_written: bool,
    pub handoff_governed: bool,
    pub reconciliation_state: String,
    pub warnings: Vec<String>,
    pub truth_sync: TruthSyncState,
}

/// Persist the resolver's already-bounded truth into the normalized state files.
/// This is deliberately compare-before-write: watcher events caused by this
/// operation converge on the same fingerprint and do not create a self-loop.
pub fn materialize_project_truth(
    database: &DatabaseState,
    project_id: &str,
    trigger: &str,
) -> Result<MaterializationResult, String> {
    let project = fetch_project(database, project_id)?;
    if github_tracking::is_github_tasks_project(&project) {
        return Err("GitHub-tracked project truth is sourced from remote TASKS.md".into());
    }
    let generation = current_truth_generation(database, project_id)?;
    let portable_key = match stable_project_key(&project) {
        Ok(value) => value,
        Err(error) => {
            if let TruthSyncCasOutcome::Applied { attempt } =
                mark_truth_sync_pending(database, project_id, generation, "UNAVAILABLE", trigger)?
            {
                let _ = mark_truth_sync_degraded(
                    database,
                    project_id,
                    generation,
                    attempt,
                    "UNAVAILABLE",
                    trigger,
                    &error,
                );
            }
            return Err(error);
        }
    };
    let root = Path::new(&project.normalized_path);
    if project.status != "ACTIVE" || !root.is_dir() {
        let attempt = match mark_truth_sync_pending(
            database,
            project_id,
            generation,
            "UNAVAILABLE",
            trigger,
        )? {
            TruthSyncCasOutcome::Applied { attempt } => attempt,
            TruthSyncCasOutcome::AlreadyCurrent => {
                return Ok(MaterializationResult {
                    project_id: project_id.into(),
                    trigger: trigger.into(),
                    changed: false,
                    state_written: false,
                    handoff_written: false,
                    event_written: false,
                    handoff_governed: false,
                    reconciliation_state: "CURRENT".into(),
                    warnings: Vec::new(),
                    truth_sync: truth_sync_status(database, project_id),
                })
            }
            TruthSyncCasOutcome::Superseded => {
                return Err("TRUTH_SYNC_SUPERSEDED_GENERATION".into())
            }
            TruthSyncCasOutcome::InvalidGeneration => {
                return Err("TRUTH_SYNC_INVALID_GENERATION".into())
            }
        };
        let _ = mark_truth_sync_degraded(
            database,
            project_id,
            generation,
            attempt,
            "UNAVAILABLE",
            trigger,
            "PROJECT_ROOT_UNAVAILABLE",
        );
        return Ok(MaterializationResult {
            project_id: project_id.into(),
            trigger: trigger.into(),
            changed: false,
            state_written: false,
            handoff_written: false,
            event_written: false,
            handoff_governed: false,
            reconciliation_state: "UNAVAILABLE".into(),
            warnings: vec!["project root is unavailable".into()],
            truth_sync: truth_sync_status(database, project_id),
        });
    }
    let truth = match resolve_truth_impl(database, project_id) {
        Ok(value) => value,
        Err(error) => {
            if let TruthSyncCasOutcome::Applied { attempt } =
                mark_truth_sync_pending(database, project_id, generation, "UNAVAILABLE", trigger)?
            {
                let _ = mark_truth_sync_degraded(
                    database,
                    project_id,
                    generation,
                    attempt,
                    "UNAVAILABLE",
                    trigger,
                    &error,
                );
            }
            return Err(error);
        }
    };
    let revision = truth_revision(&truth, &portable_key);
    let attempt =
        match mark_truth_sync_pending(database, project_id, generation, &revision, trigger)? {
            TruthSyncCasOutcome::Applied { attempt } => attempt,
            TruthSyncCasOutcome::AlreadyCurrent => {
                return Ok(MaterializationResult {
                    project_id: project_id.into(),
                    trigger: trigger.into(),
                    changed: false,
                    state_written: false,
                    handoff_written: false,
                    event_written: false,
                    handoff_governed: false,
                    reconciliation_state: truth.reconciliation_state,
                    warnings: truth.warnings,
                    truth_sync: truth_sync_status(database, project_id),
                })
            }
            TruthSyncCasOutcome::Superseded => {
                return Err("TRUTH_SYNC_SUPERSEDED_GENERATION".into())
            }
            TruthSyncCasOutcome::InvalidGeneration => {
                return Err("TRUTH_SYNC_INVALID_GENERATION".into())
            }
        };
    let result = (|| -> Result<MaterializationResult, String> {
        let state_path = root.join(STATE_JSON);
        let state_raw = read_bounded(&state_path, MAX_FILE_BYTES)?;
        let mut state_value: serde_json::Value = serde_json::from_str(&state_raw)
            .map_err(|error| format!("STATE.json is malformed: {error}"))?;
        let state_object = state_value
            .as_object_mut()
            .ok_or_else(|| "STATE.json must be an object".to_string())?;
        let metadata_missing = ["updatedAt", "updatedBy", "materializationTrigger"]
            .iter()
            .any(|key| !state_object.contains_key(*key));
        state_object.insert("updatedAt".into(), crate::time::utc_timestamp().into());
        state_object.insert("updatedBy".into(), "HIVEAI_SYSTEM".into());
        state_object.insert(
            "materializationTrigger".into(),
            bounded_text(trigger, 96).into(),
        );
        let set_optional = |object: &mut serde_json::Map<String, serde_json::Value>,
                            key: &str,
                            value: Option<&String>| {
            object.insert(
                key.into(),
                value.map_or(serde_json::Value::Null, |value| value.clone().into()),
            );
        };
        state_object.insert("schema".into(), CONTROL_PLANE_SCHEMA.into());
        state_object.insert("projectKey".into(), portable_key.clone().into());
        state_object.insert(
            "workflowState".into(),
            truth
                .workflow_state
                .clone()
                .unwrap_or_else(|| "NEEDS_RECONCILIATION".into())
                .into(),
        );
        set_optional(state_object, "milestone", truth.current_milestone.as_ref());
        set_optional(
            state_object,
            "currentMilestone",
            truth.current_milestone.as_ref(),
        );
        set_optional(state_object, "cycle", truth.current_cycle.as_ref());
        set_optional(state_object, "currentCycle", truth.current_cycle.as_ref());
        set_optional(
            state_object,
            "currentTaskId",
            truth.current_task_id.as_ref(),
        );
        set_optional(
            state_object,
            "currentTaskTitle",
            truth.current_task_title.as_ref(),
        );
        set_optional(state_object, "requiredActor", truth.required_actor.as_ref());
        set_optional(state_object, "nextAction", truth.next_action.as_ref());
        state_object.insert(
            "blockers".into(),
            serde_json::to_value(&truth.blockers).map_err(|error| error.to_string())?,
        );
        state_object.insert(
            "progressPercent".into(),
            truth
                .progress_percent
                .map_or(serde_json::Value::Null, |value| value.into()),
        );
        set_optional(state_object, "progressScope", truth.progress_scope.as_ref());
        state_object.insert(
            "authoritySource".into(),
            truth.authority_source.clone().into(),
        );
        state_object.insert(
            "provenance".into(),
            serde_json::to_value(&truth.provenance).map_err(|error| error.to_string())?,
        );
        state_object.insert(
            "reconciliationStatus".into(),
            truth.reconciliation_state.clone().into(),
        );
        state_object.insert(
            "reconciliationWarnings".into(),
            serde_json::to_value(&truth.warnings).map_err(|error| error.to_string())?,
        );
        let state_changed = metadata_missing
            || control_plane_value_fingerprint(&state_value)
                != control_plane_value_fingerprint(
                    &serde_json::from_str(&state_raw).map_err(|error| error.to_string())?,
                );
        let state_json =
            serde_json::to_string_pretty(&state_value).map_err(|error| error.to_string())?;
        if state_changed {
            atomic_replace(&state_path, &state_raw, &state_json)?;
        }

        let project_raw = read_bounded(&root.join(PROJECT_JSON), MAX_FILE_BYTES)?;
        let project_value: serde_json::Value = serde_json::from_str(&project_raw)
            .map_err(|error| format!("PROJECT.json is malformed: {error}"))?;
        let rules_raw = read_bounded(&root.join(RULES_MD), MAX_FILE_BYTES).unwrap_or_default();
        let governance = handoff_governance(&project_value, &rules_raw);
        let handoff_path = root.join(HANDOFF_MD);
        let handoff_raw = read_bounded(&handoff_path, MAX_FILE_BYTES).unwrap_or_default();
        let mut warnings = truth.warnings.clone();
        if let Some(note) = governance.note.clone() {
            warnings.push(note);
        }
        // An externally governed HANDOFF is evidence owned by the repository;
        // do not even parse its managed markers or let them block STATE.
        let handoff_changed = if governance.automation_allowed {
            let handoff_json =
                render_materialized_handoff(&project.name, &portable_key, &truth, &handoff_raw)?;
            if handoff_raw != handoff_json {
                atomic_replace(&handoff_path, &handoff_raw, &handoff_json)?;
                true
            } else {
                false
            }
        } else {
            false
        };
        let changed = state_changed || handoff_changed;
        let event_written = if changed {
            let event_id = format!(
                "truth:{}:{}",
                portable_key,
                &revision[..24.min(revision.len())]
            );
            append_canonical_event(
                &project,
                CanonicalEventRecord {
                    schema: "hiveai-event/v1".into(),
                    event_id,
                    project_key: portable_key.clone(),
                    event_type: "PROJECT_TRUTH_MATERIALIZED".into(),
                    occurred_at: crate::time::utc_timestamp(),
                    actor: "HIVEAI_SYSTEM".into(),
                    task_id: truth.current_task_id.clone(),
                    workflow_state: truth.workflow_state.clone(),
                    summary: format!("Project truth materialized after {trigger}"),
                    commit: None,
                    audit_id: None,
                    session_id: None,
                    evidence_refs: truth.provenance.iter().take(16).cloned().collect(),
                },
            )?
        } else {
            false
        };
        Ok(MaterializationResult {
            project_id: project_id.into(),
            trigger: trigger.into(),
            changed,
            state_written: state_changed,
            handoff_written: handoff_changed,
            event_written,
            handoff_governed: governance.automation_allowed,
            reconciliation_state: truth.reconciliation_state.clone(),
            warnings,
            truth_sync: TruthSyncState::default(),
        })
    })();
    match result {
        Ok(mut value) => {
            match mark_truth_sync_current(database, project_id, generation, attempt, &revision)? {
                TruthSyncCasOutcome::InvalidGeneration => {
                    return Err("TRUTH_SYNC_INVALID_GENERATION".into())
                }
                TruthSyncCasOutcome::Superseded => {
                    value
                        .warnings
                        .push("truth sync completion superseded by newer generation".into());
                }
                TruthSyncCasOutcome::Applied { .. } | TruthSyncCasOutcome::AlreadyCurrent => {}
            }
            value.truth_sync = truth_sync_status(database, project_id);
            Ok(value)
        }
        Err(error) => {
            let _ = mark_truth_sync_degraded(
                database, project_id, generation, attempt, &revision, trigger, &error,
            );
            Err(error)
        }
    }
}

pub struct ProjectTruthMaterializer;

impl ProjectTruthMaterializer {
    pub fn materialize(
        database: &DatabaseState,
        project_id: &str,
        trigger: &str,
    ) -> Result<MaterializationResult, String> {
        materialize_project_truth(database, project_id, trigger)
    }
}

fn control_plane_value_fingerprint(value: &serde_json::Value) -> String {
    let mut value = value.clone();
    if let Some(object) = value.as_object_mut() {
        object.remove("updatedAt");
        object.remove("updatedBy");
        object.remove("materializationTrigger");
    }
    let bytes = serde_json::to_vec(&value).unwrap_or_default();
    format!("{:x}", Sha256::digest(bytes))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandoffGovernance {
    automation_allowed: bool,
    note: Option<String>,
}

fn handoff_governance(project: &serde_json::Value, rules: &str) -> HandoffGovernance {
    let typed = project
        .get("governance")
        .cloned()
        .map(serde_json::from_value::<ProjectGovernance>);
    if matches!(typed, Some(Err(_))) {
        return HandoffGovernance {
            automation_allowed: false,
            note: Some("HANDOFF automation denied: PROJECT governance is malformed".into()),
        };
    }
    if let Some(Ok(governance)) = typed {
        if governance.builder_may_mutate_handoff == Some(false)
            || governance.handoff_automation_allowed == Some(false)
            || matches!(
                governance.handoff_policy,
                Some(
                    HandoffPolicy::Manual | HandoffPolicy::OwnerOnly | HandoffPolicy::OwnerApproved
                )
            )
        {
            return HandoffGovernance {
                automation_allowed: false,
                note: Some(
                    "HANDOFF externally governed by PROJECT governance; STATE remains materialized"
                        .into(),
                ),
            };
        }
    }
    if rules_forbid_handoff_automation(rules) {
        return HandoffGovernance {
            automation_allowed: false,
            note: Some("HANDOFF externally governed by RULES; STATE remains materialized".into()),
        };
    }
    HandoffGovernance {
        automation_allowed: true,
        note: None,
    }
}

fn rules_forbid_handoff_automation(rules: &str) -> bool {
    rules
        .lines()
        .map(|line| line.trim().to_ascii_lowercase())
        .any(|line| {
            line.contains("codex may not rewrite handoff")
                || line.contains("builder automation may not rewrite handoff")
                || (line.contains("handoff") && line.contains("owner-approved"))
                || (line.contains("handoff") && line.contains("owner-only"))
        })
}

fn handoff_automation_allowed(project: &serde_json::Value, rules: &str) -> bool {
    handoff_governance(project, rules).automation_allowed
}

const HANDOFF_BEGIN: &str = "<!-- HIVEAI:BEGIN MANAGED CURRENT -->";
const HANDOFF_END: &str = "<!-- HIVEAI:END MANAGED CURRENT -->";

fn render_materialized_handoff(
    project_name: &str,
    project_key: &str,
    truth: &ProjectTruth,
    existing: &str,
) -> Result<String, String> {
    let begin_count = existing.matches(HANDOFF_BEGIN).count();
    let end_count = existing.matches(HANDOFF_END).count();
    if begin_count > 1 || end_count > 1 || begin_count != end_count {
        return Err("HANDOFF managed markers are malformed or duplicated".into());
    }
    let managed = format!(
        "{HANDOFF_BEGIN}\n## Current\n\n- Project key: {project_key}\n- Current task ID: {}\n- Current task title: {}\n- Current milestone: {}\n- Current cycle: {}\n- Required actor: {}\n- Workflow state: {}\n\n## Next\n\n- Next action: {}\n- Resume pointer: Read STATE.json and the canonical task source.\n{HANDOFF_END}",
        truth.current_task_id.as_deref().unwrap_or("NEEDS_RECONCILIATION"),
        truth.current_task_title.as_deref().unwrap_or("NEEDS_RECONCILIATION"),
        truth.current_milestone.as_deref().unwrap_or("NEEDS_RECONCILIATION"),
        truth.current_cycle.as_deref().unwrap_or("NEEDS_RECONCILIATION"),
        truth.required_actor.as_deref().unwrap_or("NEEDS_RECONCILIATION"),
        truth.workflow_state.as_deref().unwrap_or("NEEDS_RECONCILIATION"),
        truth.next_action.as_deref().unwrap_or("Reconcile from canonical evidence"),
    );
    if begin_count == 1 {
        let start = existing.find(HANDOFF_BEGIN).unwrap();
        let end = existing.find(HANDOFF_END).unwrap() + HANDOFF_END.len();
        let mut output = String::with_capacity(existing.len() + managed.len());
        output.push_str(&existing[..start]);
        output.push_str(&managed);
        output.push_str(&existing[end..]);
        return Ok(output);
    }
    if existing.trim().is_empty() {
        return Ok(format!("# {project_name} H!veAI Handoff\n\n{managed}\n"));
    }
    Ok(format!("{managed}\n\n{existing}"))
}

fn overlay_remote_observation(
    database: &DatabaseState,
    project_id: &str,
    git: &mut GitControlPlaneState,
) {
    let Ok(connection) = database.open_connection() else {
        return;
    };
    let row = connection.query_row(
        "SELECT last_remote_observation_at, last_remote_observation_status, last_remote_observation_error, last_remote_observed_upstream, last_remote_observed_ahead, last_remote_observed_behind, last_remote_observed_diverged FROM projects WHERE id=?1",
        [project_id],
        |row| Ok((row.get::<_, Option<String>>(0)?, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, Option<String>>(3)?, row.get::<_, Option<i64>>(4)?, row.get::<_, Option<i64>>(5)?, row.get::<_, Option<i64>>(6)?)),
    );
    if let Ok((at, status, error, upstream, ahead, behind, diverged)) = row {
        git.last_remote_observation_at = at;
        git.last_remote_observation_status = status;
        git.last_remote_observation_error = error;
        git.last_remote_observed_upstream = upstream;
        git.last_remote_observed_ahead = ahead.and_then(|value| u64::try_from(value).ok());
        git.last_remote_observed_behind = behind.and_then(|value| u64::try_from(value).ok());
        git.last_remote_observed_diverged = diverged.map(|value| value != 0);
    }
}

fn persist_remote_observation(
    database: &DatabaseState,
    project_id: &str,
    status: &str,
    error: Option<&str>,
    git: Option<&git_engine::GitSnapshot>,
) -> Result<(), String> {
    let connection = database.open_connection()?;
    let mut bounded_error = error.map(|value| value.chars().take(512).collect::<String>());
    if status == "SUCCESS" {
        bounded_error = None;
    }
    connection.execute(
        "UPDATE projects SET last_remote_observation_at=?2, last_remote_observation_status=?3, last_remote_observation_error=?4, last_remote_observed_upstream=COALESCE(?5,last_remote_observed_upstream), last_remote_observed_ahead=COALESCE(?6,last_remote_observed_ahead), last_remote_observed_behind=COALESCE(?7,last_remote_observed_behind), last_remote_observed_diverged=COALESCE(?8,last_remote_observed_diverged) WHERE id=?1",
        rusqlite::params![project_id, crate::time::utc_timestamp(), status, bounded_error, git.and_then(|value| value.upstream.clone()), git.and_then(|value| value.ahead_count.map(|count| count as i64)), git.and_then(|value| value.behind_count.map(|count| count as i64)), git.map(|value| if value.ahead_count.unwrap_or(0) > 0 && value.behind_count.unwrap_or(0) > 0 { 1 } else { 0 })],
    ).map_err(|error| error.to_string())?;
    Ok(())
}

pub fn append_event(project: &ProjectRecord, mut event: EventRecord) -> Result<bool, String> {
    let portable_key = stable_project_key(project).unwrap_or_else(|_| event.project_key.clone());
    event.project_key = portable_key.clone();
    append_canonical_event(
        project,
        CanonicalEventRecord {
            schema: "hiveai-event/v1".into(),
            event_id: event.event_id,
            project_key: portable_key,
            event_type: event.event_type,
            occurred_at: event.occurred_at,
            actor: event.actor,
            task_id: event.task_id,
            workflow_state: None,
            summary: event.summary,
            commit: None,
            audit_id: None,
            session_id: None,
            evidence_refs: event.evidence_refs,
        },
    )
}

fn append_canonical_event(
    project: &ProjectRecord,
    event: CanonicalEventRecord,
) -> Result<bool, String> {
    let root = Path::new(&project.normalized_path);
    if project.status != "ACTIVE" || !root.is_dir() {
        return Err("project root is unavailable".into());
    }
    if event.event_id.trim().is_empty() || event.event_type.trim().is_empty() {
        return Err("event id and type are required".into());
    }
    let (path, index_path) = project_event_paths(&root);
    let mut event_ids = load_event_index(&index_path, &path)?;
    if event_ids
        .iter()
        .any(|candidate| candidate == &event.event_id)
    {
        // Reconcile a stale sidecar before acknowledging the duplicate. The
        // EVENTS tail is durable evidence after a crash between append/index.
        persist_event_index(&index_path, &event_ids)?;
        return Ok(false);
    }
    let line = serde_json::to_string(&event).map_err(|error| error.to_string())?;
    if line.len() > MAX_EVENT_BYTES {
        return Err("event exceeds the bounded event size".into());
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| format!("open EVENTS.jsonl: {error}"))?;
    writeln!(file, "{line}").map_err(|error| error.to_string())?;
    event_ids.push(event.event_id);
    if event_ids.len() > MAX_EVENT_INDEX_IDS {
        let remove = event_ids.len() - MAX_EVENT_INDEX_IDS;
        event_ids.drain(..remove);
    }
    persist_event_index(&index_path, &event_ids)?;
    Ok(true)
}

fn project_event_paths(root: &Path) -> (PathBuf, PathBuf) {
    let events = read_bounded(&root.join(PROJECT_JSON), MAX_FILE_BYTES)
        .ok()
        .and_then(|raw| serde_json::from_str::<ProjectDocument>(&raw).ok())
        .and_then(|document| safe_relative_path(&document.events))
        .map(|path| root.join(path))
        .unwrap_or_else(|| root.join(EVENTS_JSONL));
    (events, root.join(EVENT_INDEX_JSON))
}

fn load_event_index(index_path: &Path, events_path: &Path) -> Result<Vec<String>, String> {
    let mut event_ids = if let Ok(raw) = read_bounded(index_path, MAX_FILE_BYTES) {
        let document: EventIndexDocument = serde_json::from_str(&raw)
            .map_err(|error| format!("EVENT_INDEX.json is malformed: {error}"))?;
        if document.event_ids.len() > MAX_EVENT_INDEX_IDS {
            return Err("EVENT_INDEX.json exceeds the bounded id budget".into());
        }
        document.event_ids
    } else {
        Vec::new()
    };
    let tail = recent_event_tail(events_path)?;
    for line in tail.lines() {
        if line.len() > MAX_EVENT_BYTES {
            continue;
        }
        if let Ok(event) = serde_json::from_str::<EventRecord>(line) {
            if !event_ids
                .iter()
                .any(|candidate| candidate == &event.event_id)
            {
                event_ids.push(event.event_id);
            }
        }
    }
    if event_ids.len() > MAX_EVENT_INDEX_IDS {
        let remove = event_ids.len() - MAX_EVENT_INDEX_IDS;
        event_ids.drain(..remove);
    }
    Ok(event_ids)
}

fn recent_event_tail(events_path: &Path) -> Result<String, String> {
    let mut file = match File::open(events_path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(String::new()),
        Err(error) => return Err(format!("event index backfill unavailable: {error}")),
    };
    let length = file
        .metadata()
        .map_err(|error| format!("event tail metadata unavailable: {error}"))?
        .len();
    let offset = length.saturating_sub(MAX_EVENT_TAIL_BYTES);
    file.seek(SeekFrom::Start(offset))
        .map_err(|error| format!("event tail seek failed: {error}"))?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|error| format!("event tail read failed: {error}"))?;
    let mut text = String::from_utf8_lossy(&bytes).into_owned();
    if offset > 0 {
        if let Some(position) = text.find('\n') {
            text = text[position + 1..].to_string();
        } else {
            text.clear();
        }
    }
    Ok(text)
}

fn persist_event_index(index_path: &Path, event_ids: &[String]) -> Result<(), String> {
    #[cfg(test)]
    if FAIL_NEXT_EVENT_INDEX_PERSISTENCE.with(|value| value.replace(false)) {
        return Err("EVENT_INDEX_FAILPOINT: persistence failed after durable event append".into());
    }
    let document = EventIndexDocument {
        schema: "hiveai-event-index/v1".into(),
        event_ids: event_ids.to_vec(),
    };
    let contents = serde_json::to_string_pretty(&document).map_err(|error| error.to_string())?;
    match read_bounded(index_path, MAX_FILE_BYTES) {
        Ok(expected) => atomic_replace(index_path, &expected, &contents),
        Err(error) if error == "NOT_FOUND" => create_if_missing(index_path, &contents),
        Err(error) => Err(error),
    }
}

pub fn resolve_project(project: &ProjectRecord) -> Result<ControlPlaneSnapshot, String> {
    let root = PathBuf::from(&project.normalized_path);
    let mut warnings = Vec::new();
    if project.status != "ACTIVE" || !root.is_dir() {
        return Ok(empty_snapshot(
            project,
            "MISSING",
            "Registered project root is unavailable",
        ));
    }
    let project_path = root.join(PROJECT_JSON);
    let raw = match read_bounded(&project_path, MAX_FILE_BYTES) {
        Ok(raw) => raw,
        Err(error) if error == "NOT_FOUND" => {
            return Ok(empty_snapshot(
                project,
                "NEEDS_RECONCILIATION",
                "Project is registered but has not adopted the v1 control plane",
            ));
        }
        Err(error) => return Ok(empty_snapshot(project, "MALFORMED_CONTROL_PLANE", &error)),
    };
    let document: ProjectDocument = match serde_json::from_str(&raw) {
        Ok(value) => value,
        Err(error) => {
            return Ok(empty_snapshot(
                project,
                "MALFORMED_CONTROL_PLANE",
                &format!("PROJECT.json is malformed: {error}"),
            ))
        }
    };
    let mut adopted = document.schema == CONTROL_PLANE_SCHEMA;
    if !adopted {
        warnings.push("PROJECT.json schema is not the supported v1 control-plane schema".into());
    }
    if document.canonical_task_source.trim().is_empty()
        || Path::new(&document.canonical_task_source).is_absolute()
        || document.canonical_task_source.contains("..")
    {
        adopted = false;
        warnings.push("Canonical task source must be a bounded relative path".into());
    }
    let state = read_state(&root.join(STATE_JSON), &mut warnings);
    let handoff = read_handoff(&root.join(HANDOFF_MD), &mut warnings);
    let events_path = safe_relative_path(&document.events)
        .map(|path| root.join(path))
        .unwrap_or_else(|| root.join(EVENTS_JSONL));
    if document.events.trim().is_empty() || safe_relative_path(&document.events).is_none() {
        adopted = false;
        warnings.push("Canonical events path must be a bounded relative path".into());
    }
    let events = read_events(&events_path, &mut warnings);
    let session_result = read_optional_json::<SessionResultClaim>(
        &root.join(".hiveai/SESSION_RESULT.json"),
        &mut warnings,
    );
    if state.is_none()
        || handoff.is_none()
        || !root.join(RULES_MD).is_file()
        || !events_path.is_file()
    {
        adopted = false;
        warnings.push("Required control-plane files are incomplete".into());
    }
    let workflow_state = state.as_ref().map(|value| value.workflow_state.clone());
    if workflow_state
        .as_deref()
        .is_some_and(|value| !WORKFLOW_STATES.contains(&value))
    {
        adopted = false;
        warnings.push("STATE.json has an unsupported workflow state".into());
    }
    // `localRegistryId` is legacy machine-local metadata. It remains readable
    // for old manifests but is intentionally ignored for adoption and truth.
    let last_event_at = events.last().map(|event| event.occurred_at.clone());
    let mut result = ControlPlaneSnapshot {
        schema: document.schema,
        project_id: project.id.clone(),
        project_key: Some(document.project_key),
        display_name: document.display_name,
        adopted,
        health: if adopted {
            "HEALTHY".into()
        } else {
            "MALFORMED_CONTROL_PLANE".into()
        },
        workflow_state,
        canonical_task_source: Some(document.canonical_task_source),
        current_task_id: state
            .as_ref()
            .and_then(|value| value.current_task_id.clone())
            .or_else(|| {
                handoff
                    .as_ref()
                    .and_then(|value| value.current_task_id.clone())
            }),
        current_task_title: state
            .as_ref()
            .and_then(|value| value.current_task_title.clone())
            .or_else(|| {
                handoff
                    .as_ref()
                    .and_then(|value| value.current_task_title.clone())
            }),
        current_milestone: state
            .as_ref()
            .and_then(|value| value.milestone.clone())
            .or_else(|| {
                handoff
                    .as_ref()
                    .and_then(|value| value.current_milestone.clone())
            }),
        current_cycle: state
            .as_ref()
            .and_then(|value| value.cycle.clone())
            .or_else(|| {
                handoff
                    .as_ref()
                    .and_then(|value| value.current_cycle.clone())
            }),
        required_actor: state
            .as_ref()
            .and_then(|value| value.required_actor.clone())
            .or_else(|| {
                handoff
                    .as_ref()
                    .and_then(|value| value.required_actor.clone())
            }),
        remote_repository: Some(format!(
            "{}/{}",
            document.repository.owner, document.repository.name
        )),
        session_result,
        auto_fast_forward_enabled: false,
        next_action: state
            .as_ref()
            .and_then(|value| value.next_action.clone())
            .or_else(|| handoff.as_ref().and_then(|value| value.next_action.clone())),
        blockers: state
            .as_ref()
            .map(|value| value.blockers.clone())
            .unwrap_or_default(),
        progress_percent: state.as_ref().and_then(|value| value.progress_percent),
        resume_pointer: handoff.as_ref().map(|value| value.resume_pointer.clone()),
        event_count: events.len(),
        last_event_at,
        git: GitControlPlaneState {
            local_status: "UNKNOWN".into(),
            remote_status: "UNKNOWN".into(),
            sync_status: "UNKNOWN".into(),
            branch: None,
            head_sha: None,
            upstream: None,
            ahead: None,
            behind: None,
            dirty: false,
            conflicted: false,
            last_remote_observation_at: None,
            last_remote_observation_status: "UNKNOWN".into(),
            last_remote_observation_error: None,
            last_remote_observed_upstream: None,
            last_remote_observed_ahead: None,
            last_remote_observed_behind: None,
            last_remote_observed_diverged: None,
        },
        truth_sync: TruthSyncState::default(),
        source_precedence: vec![
            "PROJECT_REGISTRY".into(),
            "PROJECT.json".into(),
            "CANONICAL_TASK_SOURCE".into(),
            "STATE.json".into(),
            "HANDOFF.md".into(),
            "GIT_ENGINE".into(),
            "EVENTS.jsonl".into(),
        ],
        warnings,
    };
    result.health = health_for(&result, &result.git);
    Ok(result)
}

fn remote_control_plane_snapshot(
    project: &ProjectRecord,
    remote: &RemoteTrackingSnapshot,
) -> ControlPlaneSnapshot {
    ControlPlaneSnapshot {
        schema: "github-root-tasks-v1".into(),
        project_id: project.id.clone(),
        project_key: Some(remote.project_key.clone()),
        display_name: project.name.clone(),
        adopted: false,
        health: github_tracking::remote_health(remote).into(),
        workflow_state: remote.workflow_state.clone(),
        canonical_task_source: Some("TASKS.md".into()),
        current_task_id: remote.current_task_id.clone(),
        current_task_title: remote.current_task_title.clone(),
        current_milestone: remote.current_milestone.clone(),
        current_cycle: remote.current_sprint.clone(),
        required_actor: remote.required_actor.clone(),
        remote_repository: Some(remote.repository.clone()),
        session_result: None,
        auto_fast_forward_enabled: false,
        next_action: remote.next_action.clone(),
        blockers: remote.blockers.clone(),
        progress_percent: remote.progress_percent.map(|value| value.round() as u8),
        resume_pointer: None,
        event_count: remote.recent_events.len(),
        last_event_at: remote.updated_at.clone(),
        git: GitControlPlaneState {
            local_status: "SECONDARY_TELEMETRY".into(),
            remote_status: remote.remote_health.clone(),
            sync_status: "REMOTE_PRIMARY".into(),
            branch: Some(remote.branch.clone()),
            head_sha: remote.remote_head.clone(),
            upstream: Some(remote.repository.clone()),
            ahead: None,
            behind: None,
            dirty: false,
            conflicted: false,
            last_remote_observation_at: Some(remote.fetched_at.clone()),
            last_remote_observation_status: remote.remote_health.clone(),
            last_remote_observation_error: remote.error.clone(),
            last_remote_observed_upstream: Some(remote.remote_head.clone().unwrap_or_default()),
            last_remote_observed_ahead: None,
            last_remote_observed_behind: None,
            last_remote_observed_diverged: None,
        },
        truth_sync: TruthSyncState::default(),
        source_precedence: vec!["GITHUB_TASKS_ONLY".into()],
        warnings: Vec::new(),
    }
}

/// Resolve the one project truth contract consumed by reconciliation and both
/// project-facing summaries. Every current-task claim must be backed by the
/// canonical task source or an explicit, valid workflow/control-plane claim.
pub struct ProjectTruthResolver;

impl ProjectTruthResolver {
    pub fn resolve(database: &DatabaseState, project_id: &str) -> Result<ProjectTruth, String> {
        resolve_truth_impl(database, project_id)
    }
}

pub fn resolve_truth(database: &DatabaseState, project_id: &str) -> Result<ProjectTruth, String> {
    ProjectTruthResolver::resolve(database, project_id)
}

fn resolve_truth_impl(database: &DatabaseState, project_id: &str) -> Result<ProjectTruth, String> {
    let project = fetch_project(database, project_id)?;
    if github_tracking::is_github_tasks_project(&project) {
        let remote = github_tracking::refresh_project(database, &project)
            .unwrap_or_else(|error| github_tracking::unavailable_for_project(&project, error));
        return Ok(remote_project_truth(&project, &remote));
    }
    let control_plane = resolve_project(&project)?;
    let mut warnings = control_plane.warnings.clone();
    let mut provenance = vec![PROJECT_JSON.into(), STATE_JSON.into(), HANDOFF_MD.into()];
    let state = read_state(
        &Path::new(&project.normalized_path).join(STATE_JSON),
        &mut warnings,
    );
    let handoff = read_handoff(
        &Path::new(&project.normalized_path).join(HANDOFF_MD),
        &mut warnings,
    );
    let intelligence = task_intelligence::list(database, project_id).ok();
    let dashboard = project_dashboard::resolve(database, project_id)?;
    let canonical_tasks = intelligence
        .as_ref()
        .map(|snapshot| {
            snapshot
                .tasks
                .iter()
                .filter(|task| match dashboard.task_authority {
                    project_dashboard::TaskAuthorityState::NotCanonicalized => false,
                    project_dashboard::TaskAuthorityState::Canonical => dashboard
                        .canonical_task_source
                        .as_deref()
                        .map(|path| same_path(path, &task.source_path))
                        .unwrap_or(false),
                    project_dashboard::TaskAuthorityState::FallbackM08M09 => true,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if intelligence.is_some() {
        provenance.push("CANONICAL_TASK_SOURCE".into());
    } else {
        warnings.push("canonical task intelligence is unavailable".into());
    }
    let workflows = workflow::project_list(
        database,
        workflow::WorkflowProjectListQuery {
            project_id: project_id.to_string(),
            limit: Some(workflow::MAX_HISTORY_LIMIT),
        },
    )
    .map(|value| value.tasks)
    .unwrap_or_default();
    let task_by_id = canonical_tasks
        .iter()
        .map(|task| (task.id.as_str(), *task))
        .collect::<std::collections::HashMap<_, _>>();
    let workflow_candidates = workflows
        .iter()
        .filter(|workflow| {
            workflow.source_active
                && workflow.workflow_managed
                && task_by_id.contains_key(workflow.task_id.as_str())
                && !truth_task_complete(task_by_id[workflow.task_id.as_str()], Some(workflow))
        })
        .collect::<Vec<_>>();
    let explicit_state_id = state
        .as_ref()
        .and_then(|value| value.current_task_id.clone())
        .filter(|value| !is_placeholder(value));
    let explicit_handoff_id = handoff
        .as_ref()
        .and_then(|value| value.current_task_id.clone())
        .filter(|value| !is_placeholder(value));
    let active_explicit = |id: Option<&String>| {
        id.filter(|value| {
            task_by_id.contains_key(value.as_str())
                && truth_task_is_active(task_by_id[value.as_str()], &workflows)
        })
        .cloned()
    };
    let state_candidate = active_explicit(explicit_state_id.as_ref());
    let handoff_candidate = active_explicit(explicit_handoff_id.as_ref());
    let mut selected_id = None;
    let mut authority_source = None;
    let mut explicit_conflict = state_candidate.is_some()
        && handoff_candidate.is_some()
        && state_candidate != handoff_candidate;
    let mut workflow_ambiguous = false;
    if workflow_candidates.len() > 1 {
        let workflow_ids = workflow_candidates
            .iter()
            .map(|value| value.task_id.as_str())
            .collect::<Vec<_>>();
        let state_match = state_candidate
            .as_deref()
            .filter(|id| workflow_ids.contains(id));
        let handoff_match = handoff_candidate
            .as_deref()
            .filter(|id| workflow_ids.contains(id));
        if state_match.is_some() && handoff_match.is_some() && state_match != handoff_match {
            explicit_conflict = true;
        } else if let Some(id) = state_match.or(handoff_match) {
            selected_id = Some(id.to_string());
            authority_source = Some(
                if state_match.is_some() {
                    "STATE.json"
                } else {
                    "HANDOFF.md"
                }
                .into(),
            );
        } else {
            workflow_ambiguous = true;
            warnings.push(format!(
                "multiple active canonical workflow tasks require reconciliation: {}",
                workflow_candidates
                    .iter()
                    .map(|value| value.task_id.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    } else if let Some(workflow) = workflow_candidates.first() {
        if state_candidate
            .as_deref()
            .is_some_and(|id| id != workflow.task_id.as_str())
            || handoff_candidate
                .as_deref()
                .is_some_and(|id| id != workflow.task_id.as_str())
        {
            explicit_conflict = true;
            warnings.push(
                "explicit STATE/HANDOFF task identity conflicts with active workflow task".into(),
            );
        } else {
            selected_id = Some(workflow.task_id.clone());
            authority_source = Some("M10_WORKFLOW".into());
        }
    } else if state_candidate.is_some() || handoff_candidate.is_some() {
        if let (Some(state_id), Some(handoff_id)) = (&state_candidate, &handoff_candidate) {
            if state_id != handoff_id {
                explicit_conflict = true;
            }
        }
        if !explicit_conflict {
            selected_id = state_candidate.clone().or(handoff_candidate.clone());
            authority_source = Some(
                if explicit_state_id.is_some() {
                    "STATE.json"
                } else {
                    "HANDOFF.md"
                }
                .into(),
            );
        }
    }
    if let Some(task_id) = explicit_state_id
        .as_deref()
        .filter(|id| !state_candidate.as_deref().is_some_and(|value| value == *id))
    {
        if !task_by_id.contains_key(task_id)
            || !truth_task_is_active(task_by_id[task_id], &workflows)
        {
            warnings.push(format!(
                "STATE.json currentTaskId is not a valid active canonical task: {task_id}"
            ));
        }
    }
    if let Some(task_id) = explicit_handoff_id.as_deref().filter(|id| {
        !handoff_candidate
            .as_deref()
            .is_some_and(|value| value == *id)
    }) {
        if !task_by_id.contains_key(task_id)
            || !truth_task_is_active(task_by_id[task_id], &workflows)
        {
            warnings.push(format!(
                "HANDOFF.md currentTaskId is not a valid active canonical task: {task_id}"
            ));
        }
    }
    let scope_milestone = state
        .as_ref()
        .and_then(|value| value.milestone.clone())
        .or_else(|| {
            handoff
                .as_ref()
                .and_then(|value| value.current_milestone.clone())
        })
        .filter(|value| !is_placeholder(value));
    let scope_cycle = state
        .as_ref()
        .and_then(|value| value.cycle.clone())
        .or_else(|| {
            handoff
                .as_ref()
                .and_then(|value| value.current_cycle.clone())
        })
        .filter(|value| !is_placeholder(value));
    if selected_id.is_none() && !workflow_ambiguous && !explicit_conflict {
        if let Some(milestone) = scope_milestone.as_deref() {
            let scoped = canonical_tasks
                .iter()
                .filter(|task| {
                    task.milestone.as_deref() == Some(milestone)
                        && truth_task_is_active(task, &workflows)
                })
                .collect::<Vec<_>>();
            if scoped.len() == 1 {
                selected_id = Some(scoped[0].id.clone());
                authority_source = Some("CANONICAL_TASK_SOURCE".into());
            }
        }
    }
    if workflow_ambiguous || explicit_conflict {
        selected_id = None;
        authority_source = Some("NEEDS_RECONCILIATION".into());
    }
    let selected_task = selected_id
        .as_deref()
        .and_then(|task_id| task_by_id.get(task_id).copied());
    let selected_workflow = selected_task.and_then(|task| {
        workflows.iter().find(|workflow| {
            workflow.task_id == task.id
                && workflow.source_active
                && workflow.workflow_managed
                && workflow.current_state != workflow::WorkflowState::TaskComplete
        })
    });
    let current_milestone = scope_milestone.or_else(|| {
        selected_task
            .and_then(|task| task.milestone.clone())
            .or_else(|| selected_workflow.and_then(|workflow| workflow.milestone.clone()))
    });
    let current_cycle = scope_cycle;
    let requested_progress_scope = state
        .as_ref()
        .and_then(|value| value.progress_scope.clone())
        .filter(|value| !is_placeholder(value));
    let mut progress_scope = requested_progress_scope.clone();
    let mut progress_scope_valid = true;
    if let Some(scope) = requested_progress_scope.as_deref() {
        let valid = scope
            .strip_prefix("MILESTONE:")
            .is_some_and(|value| !value.is_empty() && current_milestone.as_deref() == Some(value))
            || scope
                .strip_prefix("CYCLE:")
                .is_some_and(|value| !value.is_empty() && current_cycle.as_deref() == Some(value));
        if !valid {
            progress_scope_valid = false;
            progress_scope = None;
            warnings.push(format!(
                "progress scope does not match resolved milestone/cycle: {scope}"
            ));
        }
    }
    if state
        .as_ref()
        .and_then(|value| value.progress_percent)
        .is_some()
        && requested_progress_scope.is_none()
    {
        progress_scope_valid = false;
        warnings.push("progress percent is present without an exact progress scope".into());
    }
    let explicit_progress_present = state
        .as_ref()
        .and_then(|value| value.progress_percent)
        .is_some();
    let explicit_progress = state
        .as_ref()
        .and_then(|value| value.progress_percent)
        .filter(|_| selected_task.is_some() && progress_scope.is_some() && progress_scope_valid);
    let mut progress_percent = explicit_progress;
    if progress_percent.is_none()
        && !explicit_progress_present
        && selected_task.is_some()
        && progress_scope.is_none()
        && current_milestone.is_some()
    {
        let milestone = current_milestone.as_deref().unwrap();
        let scoped = canonical_tasks
            .iter()
            .filter(|task| task.milestone.as_deref() == Some(milestone))
            .collect::<Vec<_>>();
        if !scoped.is_empty() {
            let completed = scoped
                .iter()
                .filter(|task| truth_task_complete_with_workflow(task, &workflows))
                .count();
            progress_scope = Some(format!("MILESTONE:{milestone}"));
            progress_percent = Some(((completed * 100) / scoped.len()) as u8);
        }
    }
    if progress_percent.is_some() && progress_scope.is_none() {
        progress_percent = None;
        warnings.push("progress percent was rejected because no exact scope resolved".into());
    }
    let reconciliation_state = if control_plane.adopted
        && selected_task.is_some()
        && progress_scope_valid
        && !workflow_ambiguous
        && !explicit_conflict
        && warnings.iter().all(|warning| !warning.contains("conflict"))
    {
        "RESOLVED"
    } else {
        "NEEDS_RECONCILIATION"
    };
    if selected_task.is_none() {
        authority_source = Some("NEEDS_RECONCILIATION".into());
    }
    let workflow_state = selected_workflow
        .map(|workflow| workflow.current_state.to_string())
        .or_else(|| {
            selected_task.and_then(|_| state.as_ref().map(|value| value.workflow_state.clone()))
        })
        .filter(|_value| selected_task.is_some())
        .or_else(|| {
            (reconciliation_state == "NEEDS_RECONCILIATION")
                .then_some("NEEDS_RECONCILIATION".into())
        });
    let required_actor = selected_workflow
        .and_then(|workflow| workflow.required_actor.clone())
        .or_else(|| selected_task.and_then(|task| task.required_actor.clone()))
        .or_else(|| {
            state
                .as_ref()
                .and_then(|value| value.required_actor.clone())
        })
        .or_else(|| {
            handoff
                .as_ref()
                .and_then(|value| value.required_actor.clone())
        });
    let next_action = selected_workflow
        .and_then(|workflow| workflow.allowed_next_states.first())
        .map(|state| format!("Advance to {state}"))
        .or_else(|| selected_task.and_then(|task| task.next_step.clone()))
        .or_else(|| state.as_ref().and_then(|value| value.next_action.clone()))
        .or_else(|| handoff.as_ref().and_then(|value| value.next_action.clone()))
        .or_else(|| {
            (selected_task.is_none())
                .then_some("Reconcile current task from canonical evidence".into())
        });
    let blockers = selected_task
        .map(|task| task.blockers.clone())
        .filter(|items: &Vec<String>| !items.is_empty())
        .or_else(|| state.as_ref().map(|value| value.blockers.clone()))
        .unwrap_or_default();
    let current_task_status = selected_workflow
        .map(|workflow| workflow.current_state.to_string())
        .or_else(|| selected_task.map(|task| task.parsed_status.clone()));
    let current_task_title = selected_task.map(|task| task.title.clone());
    provenance.extend(
        selected_task
            .map(|task| vec![task.source_path.clone(), task.evidence.content_hash.clone()])
            .unwrap_or_default(),
    );
    let mut deduped_warnings = Vec::new();
    for warning in warnings {
        if !deduped_warnings
            .iter()
            .any(|value: &String| value == &warning)
        {
            deduped_warnings.push(warning);
        }
    }
    Ok(ProjectTruth {
        project_id: project_id.into(),
        current_task_id: selected_id,
        current_task_title,
        current_task_status,
        current_milestone,
        current_cycle,
        workflow_state,
        required_actor,
        next_action,
        blockers,
        progress_percent,
        progress_scope,
        authority_source: authority_source.unwrap_or_else(|| "NEEDS_RECONCILIATION".into()),
        provenance,
        reconciliation_state: reconciliation_state.into(),
        warnings: deduped_warnings,
    })
}

fn remote_project_truth(project: &ProjectRecord, remote: &RemoteTrackingSnapshot) -> ProjectTruth {
    ProjectTruth {
        project_id: project.id.clone(),
        current_task_id: remote.current_task_id.clone(),
        current_task_title: remote.current_task_title.clone(),
        current_task_status: remote
            .current_task_status
            .clone()
            .or_else(|| remote.workflow_state.clone()),
        current_milestone: remote.current_milestone.clone(),
        current_cycle: remote.current_sprint.clone(),
        workflow_state: remote.workflow_state.clone(),
        required_actor: remote.required_actor.clone(),
        next_action: remote.next_action.clone(),
        blockers: remote.blockers.clone(),
        progress_percent: remote.progress_percent.map(|value| value.round() as u8),
        progress_scope: remote
            .progress_scope_type
            .clone()
            .or_else(|| remote.progress_scope_id.clone()),
        authority_source: "GITHUB_TASKS_ONLY".into(),
        provenance: vec![format!("{}@{}", remote.repository, remote.branch)],
        reconciliation_state: "REMOTE_PRIMARY".into(),
        warnings: Vec::new(),
    }
}

fn truth_task_complete(
    task: &task_intelligence::ParsedTask,
    workflow: Option<&workflow::WorkflowTask>,
) -> bool {
    workflow.is_some_and(|value| value.current_state == workflow::WorkflowState::TaskComplete)
        || matches!(
            task.parsed_status.to_ascii_uppercase().as_str(),
            "COMPLETE" | "COMPLETED" | "DONE" | "CLOSED" | "ARCHIVED" | "TASK_COMPLETE"
        )
}

fn truth_task_complete_with_workflow(
    task: &task_intelligence::ParsedTask,
    workflows: &[workflow::WorkflowTask],
) -> bool {
    truth_task_complete(
        task,
        workflows.iter().find(|value| value.task_id == task.id),
    )
}

fn truth_task_is_active(
    task: &task_intelligence::ParsedTask,
    workflows: &[workflow::WorkflowTask],
) -> bool {
    !truth_task_complete_with_workflow(task, workflows)
}

fn same_path(left: &str, right: &str) -> bool {
    left.replace('\\', "/")
        .trim_start_matches("./")
        .eq_ignore_ascii_case(right.replace('\\', "/").trim_start_matches("./"))
}

pub fn adopt(database: &DatabaseState, project_id: &str) -> Result<ControlPlaneSnapshot, String> {
    let project = fetch_project(database, project_id)?;
    if project.status != "ACTIVE" {
        return Err("CONTROL_PLANE_ADOPTION_REQUIRES_ACTIVE_PROJECT".into());
    }
    let root = PathBuf::from(&project.normalized_path);
    if !root.is_dir() {
        return Err("CONTROL_PLANE_ADOPTION_ROOT_UNAVAILABLE".into());
    }
    let control_root = root.join(".hiveai");
    fs::create_dir_all(&control_root)
        .map_err(|error| format!("create control-plane directory: {error}"))?;
    let canonical_task_source = canonical_task_source_for_adoption(&project, &root)?;
    let portable_key = stable_project_key(&project)?;
    let repository = project
        .repository
        .as_ref()
        .and_then(|value| value.github_owner.as_ref().zip(value.github_repo.as_ref()))
        .map(|(owner, name)| RepositoryIdentity {
            owner: owner.clone(),
            name: name.clone(),
            branch: project
                .repository
                .as_ref()
                .and_then(|value| value.current_branch.clone()),
        })
        .unwrap_or(RepositoryIdentity {
            owner: "LOCAL".into(),
            name: project.name.clone(),
            branch: None,
        });
    let project_json = serde_json::to_string_pretty(&ProjectDocument {
        schema: CONTROL_PLANE_SCHEMA.into(),
        project_key: portable_key.clone(),
        display_name: project.name.clone(),
        repository,
        canonical_task_source: canonical_task_source.clone(),
        events: EVENTS_JSONL.into(),
        event_sources: vec![
            EVENT_INDEX_JSON.into(),
            ".git/HEAD".into(),
            ".git/index".into(),
        ],
        local_registry_id: None,
        governance: None,
        rules: Some(RULES_MD.into()),
        state: Some(STATE_JSON.into()),
        handoff: Some(HANDOFF_MD.into()),
    })
    .map_err(|error| error.to_string())?;
    create_if_missing(&root.join(PROJECT_JSON), &project_json)?;
    let state_json = serde_json::to_string_pretty(&StateDocument {
        schema: CONTROL_PLANE_SCHEMA.into(),
        project_key: portable_key,
        workflow_state: "READY".into(),
        milestone: None,
        cycle: None,
        current_task_id: None,
        current_task_title: None,
        required_actor: None,
        next_action: Some("Read canonical task source".into()),
        blockers: Vec::new(),
        progress_percent: None,
        progress_scope: None,
        updated_by: Some("HIVEAI_SYSTEM".into()),
        updated_at: Some(crate::time::utc_timestamp()),
    })
    .map_err(|error| error.to_string())?;
    create_if_missing(&root.join(STATE_JSON), &state_json)?;
    create_if_missing(
        &root.join(HANDOFF_MD),
        &handoff_markdown_template(&project.name, &canonical_task_source),
    )?;
    create_if_missing(&root.join(EVENTS_JSONL), "")?;
    create_if_missing(
        &root.join(EVENT_INDEX_JSON),
        "{\n  \"schema\": \"hiveai-event-index/v1\",\n  \"eventIds\": []\n}\n",
    )?;
    create_if_missing(&root.join(RULES_MD), &default_rules())?;
    let evidence = physical_control_plane_evidence(&root);
    if evidence.status != PhysicalControlPlaneStatus::Adopted {
        return Err("CONTROL_PLANE_ADOPTION_PHYSICAL_CONTRACT_INVALID".into());
    }
    let mut connection = database.open_connection()?;
    let tx = connection
        .transaction()
        .map_err(|error| format!("begin control-plane adoption transaction: {error}"))?;
    tx.execute(
        "UPDATE projects SET control_plane_status='ADOPTED', control_plane_schema=?2, control_plane_revision=?3, control_plane_last_event_at=?4, control_plane_sync_status='CURRENT', updated_at=?5 WHERE id=?1",
        rusqlite::params![
            project_id,
            evidence.schema.as_deref(),
            evidence.revision.as_deref(),
            evidence.last_event_at.as_deref(),
            crate::time::utc_timestamp()
        ],
    )
    .map_err(|error| format!("persist control-plane adoption metadata: {error}"))?;
    mark_truth_dirty_tx(&tx, project_id, "ADOPTION")?;
    tx.commit()
        .map_err(|error| format!("commit control-plane adoption: {error}"))?;
    materialize_project_truth(database, project_id, "ADOPTION")?;
    snapshot(database, project_id)
}

fn canonical_task_source_for_adoption(
    project: &ProjectRecord,
    root: &Path,
) -> Result<String, String> {
    if let Ok(raw) = read_bounded(&root.join(PROJECT_JSON), MAX_FILE_BYTES) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(candidate) = value
                .get("canonicalTaskSource")
                .and_then(serde_json::Value::as_str)
                .and_then(safe_relative_path)
            {
                if root.join(&candidate).is_file() {
                    return Ok(candidate);
                }
            }
        }
    }
    if let Some(policy) = project
        .task_source_policy
        .as_deref()
        .and_then(safe_relative_path)
    {
        if root.join(&policy).is_file() {
            return Ok(policy);
        }
    }
    let name = project.name.to_ascii_lowercase();
    let known = if name == "formulab" {
        Some("docs/FORMULAB_V1_TASK_TRACKER.md")
    } else if name == "packlab 3d"
        || name == "packlab-3d"
        || name == "scrubbots"
        || name == "scrubbots-level-factory"
        || name == "scrubbots - pixel art generator"
    {
        Some("tasks.md")
    } else {
        None
    };
    if let Some(candidate) = known {
        if root.join(candidate).is_file() {
            return Ok(candidate.into());
        }
    }
    let mut candidates = Vec::new();
    for candidate in ["TASKS.md", "tasks.md", "docs/FORMULAB_V1_TASK_TRACKER.md"] {
        if root.join(candidate).is_file()
            && !candidates
                .iter()
                .any(|value: &String| value.eq_ignore_ascii_case(candidate))
        {
            candidates.push(candidate.to_string());
        }
    }
    match candidates.as_slice() {
        [candidate] => Ok(candidate.clone()),
        [] => Err("NEEDS_RECONCILIATION: no canonical task ledger was found".into()),
        _ => Err("NEEDS_RECONCILIATION: multiple canonical task ledgers conflict".into()),
    }
}

fn safe_relative_path(value: &str) -> Option<String> {
    let value = value.trim().replace('\\', "/");
    if value.is_empty()
        || value.starts_with('/')
        || value.contains(':')
        || value.split('/').any(|part| part.is_empty() || part == "..")
    {
        None
    } else {
        Some(value.trim_start_matches("./").to_string())
    }
}

fn handoff_markdown_template(project_name: &str, canonical_task_source: &str) -> String {
    format!(
        "# {project_name} H!veAI Handoff\n\n## Current\n\n- Canonical task source: `{canonical_task_source}`\n- Current task ID: NEEDS_RECONCILIATION\n- Current task title: NEEDS_RECONCILIATION\n- Current milestone: NEEDS_RECONCILIATION\n- Current cycle: NEEDS_RECONCILIATION\n- Required actor: NEEDS_RECONCILIATION\n- Workflow state: IDLE\n\n## Next\n\n- Next action: Reconcile from the canonical task source and latest workflow evidence.\n- Resume pointer: Read STATE.json and the canonical task source.\n\nThis human-readable resume pointer is never a machine-state substitute.\n"
    )
}

fn reconcile_control_plane(project: &ProjectRecord) -> Result<bool, String> {
    let root = Path::new(&project.normalized_path);
    let state_path = root.join(STATE_JSON);
    let state_raw = match read_bounded(&state_path, MAX_FILE_BYTES) {
        Ok(raw) => raw,
        Err(error) if error == "NOT_FOUND" => return Ok(false),
        Err(error) => return Err(error),
    };
    let mut state_value: serde_json::Value = serde_json::from_str(&state_raw)
        .map_err(|error| format!("STATE.json is malformed: {error}"))?;
    let state_object = state_value
        .as_object_mut()
        .ok_or_else(|| "STATE.json must be an object".to_string())?;
    let mut warnings = Vec::new();
    let handoff = read_handoff(&root.join(HANDOFF_MD), &mut warnings);
    let mut conflicts = Vec::new();
    let handoff_fields = handoff.as_ref().map(|value| {
        [
            ("currentTaskId", value.current_task_id.clone()),
            ("currentTaskTitle", value.current_task_title.clone()),
            ("currentMilestone", value.current_milestone.clone()),
            ("currentCycle", value.current_cycle.clone()),
            ("requiredActor", value.required_actor.clone()),
            ("nextAction", value.next_action.clone()),
        ]
    });
    if let Some(fields) = handoff_fields {
        for (key, handoff_value) in fields {
            let current = state_object
                .get(key)
                .and_then(serde_json::Value::as_str)
                .filter(|value| !is_placeholder(value))
                .map(ToOwned::to_owned);
            let current_missing = current.is_none();
            if let (Some(current), Some(handoff_value)) = (current, handoff_value.clone()) {
                if current != handoff_value && !is_placeholder(&handoff_value) {
                    conflicts.push(format!(
                        "{key}: STATE.json={current}; HANDOFF.md={handoff_value}"
                    ));
                }
            } else if current_missing
                && !is_placeholder(handoff_value.as_deref().unwrap_or_default())
            {
                state_object.insert(key.into(), handoff_value.into());
            }
        }
    }
    if let Some(task_id) = state_object
        .get("currentTaskId")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !is_placeholder(value))
        .map(ToOwned::to_owned)
    {
        let title_missing = state_object
            .get("currentTaskTitle")
            .and_then(serde_json::Value::as_str)
            .is_none_or(is_placeholder);
        if title_missing {
            if let Ok(project_document) = read_project_document(root) {
                if let Some(title) =
                    lookup_task_title(root, &project_document.canonical_task_source, &task_id)?
                {
                    state_object.insert("currentTaskTitle".into(), title.into());
                }
            }
        }
    }
    if !conflicts.is_empty() {
        state_object.insert("workflowState".into(), "NEEDS_RECONCILIATION".into());
        state_object.insert("reconciliationStatus".into(), "NEEDS_RECONCILIATION".into());
        state_object.insert(
            "reconciliationConflicts".into(),
            serde_json::Value::Array(
                conflicts
                    .into_iter()
                    .map(serde_json::Value::String)
                    .collect(),
            ),
        );
    }
    let normalized =
        serde_json::to_string_pretty(&state_value).map_err(|error| error.to_string())?;
    if normalized == state_raw {
        return Ok(false);
    }
    atomic_replace(&state_path, &state_raw, &normalized)?;
    Ok(true)
}

fn is_placeholder(value: &str) -> bool {
    let value = value.trim();
    value.is_empty()
        || value.eq_ignore_ascii_case("unknown")
        || value.eq_ignore_ascii_case("unavailable")
        || value.eq_ignore_ascii_case("needs_reconciliation")
        || value == "-"
}

fn read_project_document(root: &Path) -> Result<ProjectDocument, String> {
    let raw = read_bounded(&root.join(PROJECT_JSON), MAX_FILE_BYTES)?;
    serde_json::from_str(&raw).map_err(|error| format!("PROJECT.json is malformed: {error}"))
}

fn lookup_task_title(root: &Path, source: &str, task_id: &str) -> Result<Option<String>, String> {
    let Some(relative) = safe_relative_path(source) else {
        return Ok(None);
    };
    let raw = match read_bounded(&root.join(relative), MAX_FILE_BYTES) {
        Ok(raw) => raw,
        Err(error) if error == "NOT_FOUND" => return Ok(None),
        Err(error) => return Err(error),
    };
    let mut matches = Vec::new();
    for line in raw.lines().take(4096) {
        let trimmed = line.trim().trim_start_matches(['-', '*', '#', ' ']);
        if !trimmed.starts_with(task_id) {
            continue;
        }
        let title = trimmed
            .split_once(':')
            .map(|(_, value)| value.trim())
            .or_else(|| trimmed.split_once(" - ").map(|(_, value)| value.trim()))
            .unwrap_or(trimmed.trim_start_matches(task_id).trim());
        if !title.is_empty() && !matches.iter().any(|value| value == title) {
            matches.push(title.to_string());
        }
    }
    Ok((matches.len() == 1).then(|| matches.remove(0)))
}

pub fn git_sync_plan(
    git: Option<&git_engine::GitSnapshot>,
    auto_fast_forward_enabled: bool,
) -> GitSyncPlan {
    let Some(git) = git else {
        return GitSyncPlan {
            action: "LOCAL_GIT_NOT_CONNECTED".into(),
            safe: false,
            reason: "No valid local Git snapshot is available".into(),
            fetch_required: false,
            merge_required: false,
        };
    };
    if !matches!(git.health, RepositoryHealth::Clean) {
        return GitSyncPlan { action: "SYNC_ATTENTION".into(), safe: false, reason: "Dirty, conflicted, detached, missing, or unborn local state requires owner attention; remote observation remains fetch-only".into(), fetch_required: true, merge_required: false };
    }
    match (git.ahead_count, git.behind_count) {
        (Some(ahead), Some(behind)) if ahead > 0 && behind > 0 => GitSyncPlan { action: "SYNC_ATTENTION".into(), safe: false, reason: "Local and upstream histories diverged; no reset, rebase, stash, or merge is permitted".into(), fetch_required: true, merge_required: false },
        (Some(0), Some(behind)) if behind > 0 && auto_fast_forward_enabled => GitSyncPlan { action: "FAST_FORWARD_ALLOWED".into(), safe: true, reason: "Clean local branch is strictly behind its configured upstream".into(), fetch_required: true, merge_required: true },
        (Some(0), Some(behind)) if behind > 0 => GitSyncPlan { action: "SYNC_ATTENTION".into(), safe: false, reason: "Fast-forward is available but owner auto-sync is disabled".into(), fetch_required: true, merge_required: false },
        _ => GitSyncPlan { action: "NO_ACTION".into(), safe: true, reason: "No safe fast-forward is required".into(), fetch_required: false, merge_required: false },
    }
}

pub fn local_repair_plan(project: &ProjectRecord) -> LocalRepairPlan {
    let repository = project.repository.as_ref();
    if repository.is_some_and(|value| !value.is_git_repository) {
        return LocalRepairPlan { action: "REPAIR_OR_CONNECT".into(), safe: true, reason: "Remote identity may be shown, but local Git is not connected; choose a new clone path or backup plan explicitly".into(), preserves_local_data: true, target_path: Some(project.normalized_path.clone()) };
    }
    LocalRepairPlan {
        action: "NO_REPAIR_REQUIRED".into(),
        safe: true,
        reason: "Registered local Git identity is available".into(),
        preserves_local_data: true,
        target_path: Some(project.normalized_path.clone()),
    }
}

pub fn set_auto_fast_forward(
    database: &DatabaseState,
    project_id: &str,
    enabled: bool,
) -> Result<(), String> {
    let connection = database.open_connection()?;
    let changed = connection
        .execute(
            "UPDATE projects SET control_plane_auto_ff=?2, updated_at=?3 WHERE id=?1",
            rusqlite::params![
                project_id,
                if enabled { 1 } else { 0 },
                crate::time::utc_timestamp()
            ],
        )
        .map_err(|error| error.to_string())?;
    if changed == 0 {
        return Err("project is not registered".into());
    }
    Ok(())
}

pub fn sync_remote(
    database: &DatabaseState,
    project_id: &str,
    auto_fast_forward_enabled: bool,
) -> Result<GitSyncPlan, String> {
    let project = fetch_project(database, project_id)?;
    let repository = project
        .repository
        .as_ref()
        .filter(|value| value.is_git_repository)
        .ok_or_else(|| "LOCAL_GIT_NOT_CONNECTED".to_string())?;
    let root = Path::new(&project.normalized_path);
    let initial = match git_engine::snapshot(
        database,
        GitSnapshotRequest {
            project_id: project_id.to_string(),
            persist: Some(false),
        },
    ) {
        Ok(value) => value,
        Err(error) => {
            let reason = format!("SAFE_REMOTE_OBSERVATION_FAILED: {error}");
            persist_remote_observation(database, project_id, "DEGRADED", Some(&reason), None)?;
            materialize_best_effort(database, project_id, "REMOTE_OBSERVATION_FAILED");
            return Ok(GitSyncPlan {
                action: "SYNC_ATTENTION".into(),
                safe: false,
                reason,
                fetch_required: true,
                merge_required: false,
            });
        }
    };
    let remote = initial
        .upstream
        .as_deref()
        .and_then(|value| value.split('/').next())
        .filter(|value| {
            !value.is_empty()
                && value
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
        })
        .unwrap_or("origin");
    if let Err(error) = git_engine::run_git_bounded(root, &["fetch", "--prune", remote], 4096, 4096)
    {
        let reason = format!("SAFE_FETCH_FAILED: {error}");
        persist_remote_observation(database, project_id, "DEGRADED", Some(&reason), None)?;
        materialize_best_effort(database, project_id, "REMOTE_OBSERVATION_FAILED");
        return Ok(GitSyncPlan {
            action: "SYNC_ATTENTION".into(),
            safe: false,
            reason,
            fetch_required: true,
            merge_required: false,
        });
    }
    let refreshed = git_engine::snapshot(
        database,
        GitSnapshotRequest {
            project_id: project_id.to_string(),
            persist: Some(false),
        },
    )?;
    let refreshed_plan = git_sync_plan(Some(&refreshed), auto_fast_forward_enabled);
    persist_remote_observation(database, project_id, "SUCCESS", None, Some(&refreshed))?;
    materialize_best_effort(database, project_id, "REMOTE_OBSERVATION_SUCCESS");
    if !auto_fast_forward_enabled || refreshed_plan.action != "FAST_FORWARD_ALLOWED" {
        return Ok(refreshed_plan);
    }
    let repository_root = repository
        .repository_root
        .as_deref()
        .and_then(|value| Path::new(value).canonicalize().ok());
    let registered_root = root.canonicalize().ok();
    if repository_root.is_none() || repository_root != registered_root {
        return Err("SAFE_SYNC_REPOSITORY_ROOT_MISMATCH".into());
    }
    let mut connection = database.open_connection()?;
    let tx = connection
        .transaction()
        .map_err(|error| format!("begin remote fast-forward truth transaction: {error}"))?;
    mark_truth_dirty_tx(&tx, project_id, "REMOTE_FAST_FORWARD")?;
    tx.commit()
        .map_err(|error| format!("commit remote fast-forward truth intent: {error}"))?;
    git_engine::run_git_bounded(root, &["merge", "--ff-only", "@{upstream}"], 4096, 4096)
        .map_err(|error| format!("SAFE_FAST_FORWARD_FAILED: {error}"))?;
    materialize_best_effort(database, project_id, "REMOTE_FAST_FORWARD");
    let _ = snapshot(database, project_id)?;
    Ok(GitSyncPlan {
        action: "FAST_FORWARD_COMPLETE".into(),
        safe: true,
        reason: "Clean local branch was updated by a verified fast-forward only".into(),
        fetch_required: true,
        merge_required: true,
    })
}

pub fn watcher_sources(root: &Path, declared: &[String]) -> Vec<PathBuf> {
    let mut paths = vec![
        root.join(PROJECT_JSON),
        root.join(RULES_MD),
        root.join(STATE_JSON),
        root.join(HANDOFF_MD),
        root.join(EVENTS_JSONL),
        root.join(EVENT_INDEX_JSON),
        root.join(".git/HEAD"),
        root.join(".git/index"),
        root.join(".hiveai/SESSION_RESULT.json"),
    ];
    paths.extend(
        declared
            .iter()
            .filter(|path| !path.contains("..") && !Path::new(path).is_absolute())
            .map(|path| root.join(path)),
    );
    paths.sort();
    paths.dedup();
    paths
}

pub fn watcher_sources_for_project(root: &Path) -> Vec<PathBuf> {
    let mut declared = Vec::new();
    if let Ok(raw) = read_bounded(&root.join(PROJECT_JSON), MAX_FILE_BYTES) {
        if let Ok(document) = serde_json::from_str::<ProjectDocument>(&raw) {
            declared.push(document.canonical_task_source);
            declared.push(document.events);
            declared.extend(document.event_sources);
        }
    }
    let mut paths = watcher_sources(root, &declared);
    for relative in git_ref_paths(root) {
        paths.push(relative);
    }
    paths.sort();
    paths.dedup();
    paths
}

fn git_ref_paths(root: &Path) -> Vec<PathBuf> {
    let git = root.join(".git");
    let head = fs::read_to_string(git.join("HEAD")).ok();
    let mut paths = vec![git.join("HEAD"), git.join("index")];
    if let Some(reference) = head
        .as_deref()
        .and_then(|value| value.trim().strip_prefix("ref: "))
    {
        let safe = reference.starts_with("refs/")
            && !reference.contains("..")
            && !reference.contains('\\');
        if safe {
            paths.push(git.join(reference.replace('/', std::path::MAIN_SEPARATOR_STR)));
            if let Some(branch) = reference.strip_prefix("refs/heads/") {
                if !branch.is_empty() && !branch.contains("..") {
                    paths.push(
                        git.join(
                            format!("refs/remotes/origin/{branch}")
                                .replace('/', std::path::MAIN_SEPARATOR_STR),
                        ),
                    );
                }
            }
        }
    }
    if let Some(upstream) = read_git_config_value(&git, "branch") {
        let safe =
            upstream.starts_with("refs/") && !upstream.contains("..") && !upstream.contains('\\');
        if safe {
            paths.push(git.join(upstream.replace('/', std::path::MAIN_SEPARATOR_STR)));
        }
    }
    if git.join("packed-refs").is_file() {
        paths.push(git.join("packed-refs"));
    }
    paths
}

fn read_git_config_value(git: &Path, section: &str) -> Option<String> {
    let config = fs::read_to_string(git.join("config")).ok()?;
    let mut active = false;
    for line in config.lines() {
        let trimmed = line.trim();
        active =
            trimmed == format!("[{section}]") || trimmed.starts_with(&format!("[{section} \""));
        if active && trimmed.starts_with("merge = ") {
            return Some(trimmed.trim_start_matches("merge = ").trim().to_string());
        }
    }
    None
}

fn empty_snapshot(project: &ProjectRecord, health: &str, warning: &str) -> ControlPlaneSnapshot {
    ControlPlaneSnapshot {
        schema: CONTROL_PLANE_SCHEMA.into(),
        project_id: project.id.clone(),
        project_key: None,
        display_name: project.name.clone(),
        adopted: false,
        health: health.into(),
        workflow_state: None,
        canonical_task_source: None,
        current_task_id: None,
        current_task_title: None,
        current_milestone: None,
        current_cycle: None,
        required_actor: None,
        remote_repository: project.repository.as_ref().and_then(|repository| {
            repository
                .github_owner
                .as_ref()
                .zip(repository.github_repo.as_ref())
                .map(|(owner, name)| format!("{owner}/{name}"))
        }),
        session_result: None,
        auto_fast_forward_enabled: false,
        next_action: None,
        blockers: Vec::new(),
        progress_percent: None,
        resume_pointer: None,
        event_count: 0,
        last_event_at: None,
        git: GitControlPlaneState {
            local_status: "UNKNOWN".into(),
            remote_status: "UNKNOWN".into(),
            sync_status: "UNKNOWN".into(),
            branch: None,
            head_sha: None,
            upstream: None,
            ahead: None,
            behind: None,
            dirty: false,
            conflicted: false,
            last_remote_observation_at: None,
            last_remote_observation_status: "UNKNOWN".into(),
            last_remote_observation_error: None,
            last_remote_observed_upstream: None,
            last_remote_observed_ahead: None,
            last_remote_observed_behind: None,
            last_remote_observed_diverged: None,
        },
        truth_sync: TruthSyncState::default(),
        source_precedence: vec![
            "PROJECT_REGISTRY".into(),
            "PROJECT.json".into(),
            "CANONICAL_TASK_SOURCE".into(),
            "STATE.json".into(),
            "HANDOFF.md".into(),
            "GIT_ENGINE".into(),
            "EVENTS.jsonl".into(),
        ],
        warnings: vec![warning.into()],
    }
}

fn read_bounded(path: &Path, limit: usize) -> Result<String, String> {
    let mut file = File::open(path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            "NOT_FOUND".into()
        } else {
            error.to_string()
        }
    })?;
    let mut bytes = Vec::new();
    std::io::Read::by_ref(&mut file)
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > limit {
        return Err("Control-plane file exceeds the bounded read budget".into());
    }
    String::from_utf8(bytes).map_err(|_| "Control-plane file is not valid UTF-8".into())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path, warnings: &mut Vec<String>) -> Option<T> {
    match read_bounded(path, MAX_FILE_BYTES) {
        Ok(raw) => match serde_json::from_str(&raw) {
            Ok(value) => Some(value),
            Err(_) => {
                warnings.push(format!(
                    "{} is malformed",
                    path.file_name().unwrap_or_default().to_string_lossy()
                ));
                None
            }
        },
        Err(error) if error == "NOT_FOUND" => {
            warnings.push(format!(
                "{} is missing",
                path.file_name().unwrap_or_default().to_string_lossy()
            ));
            None
        }
        Err(error) => {
            warnings.push(error);
            None
        }
    }
}

fn read_optional_json<T: for<'de> Deserialize<'de>>(
    path: &Path,
    warnings: &mut Vec<String>,
) -> Option<T> {
    let raw = match read_bounded(path, MAX_FILE_BYTES) {
        Ok(raw) => raw,
        Err(error) if error == "NOT_FOUND" => return None,
        Err(error) => {
            warnings.push(error);
            return None;
        }
    };
    match serde_json::from_str(&raw) {
        Ok(value) => Some(value),
        Err(_) => {
            warnings.push("SESSION_RESULT.json is malformed".into());
            None
        }
    }
}

fn read_state(path: &Path, warnings: &mut Vec<String>) -> Option<StateDocument> {
    let raw = match read_bounded(path, MAX_FILE_BYTES) {
        Ok(raw) => raw,
        Err(error) if error == "NOT_FOUND" => {
            warnings.push("STATE.json is missing".into());
            return None;
        }
        Err(error) => {
            warnings.push(error);
            return None;
        }
    };
    match serde_json::from_str::<serde_json::Value>(&raw)
        .map_err(|error| error.to_string())
        .and_then(normalize_state_value)
    {
        Ok(state) => Some(state),
        Err(error) => {
            warnings.push(format!("STATE.json is malformed: {error}"));
            None
        }
    }
}

fn read_handoff(path: &Path, warnings: &mut Vec<String>) -> Option<HandoffDocument> {
    let raw = match read_bounded(path, MAX_FILE_BYTES) {
        Ok(raw) => raw,
        Err(error) if error == "NOT_FOUND" => {
            warnings.push("HANDOFF.md is missing".into());
            return None;
        }
        Err(error) => {
            warnings.push(error);
            return None;
        }
    };
    if let Ok(document) = serde_json::from_str::<HandoffDocument>(&raw) {
        return Some(document);
    }
    let mut current_task_id = None;
    let mut current_task_title = None;
    let mut current_milestone = None;
    let mut current_cycle = None;
    let mut required_actor = None;
    let mut next_action = None;
    let mut last_audit = None;
    let mut last_agent_session = None;
    let mut owner = None;
    let mut resume_pointer = None;
    for line in raw.lines().take(256) {
        let columns = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        let (label, value) = if columns.len() >= 2 {
            (
                normalize_handoff_label(columns[0]),
                columns[1]
                    .trim()
                    .trim_matches('`')
                    .trim_matches('*')
                    .to_string(),
            )
        } else if let Some((label, value)) = line.trim().split_once(':') {
            (
                normalize_handoff_label(label),
                value.trim().trim_matches('`').trim_matches('*').to_string(),
            )
        } else {
            continue;
        };
        if value.is_empty() || value == "-" || value.eq_ignore_ascii_case("unknown") {
            continue;
        }
        match label.as_str() {
            "current task id" => current_task_id = Some(value),
            "current task title" => current_task_title = Some(value),
            "current task" => {
                if looks_like_task_id(&value) {
                    current_task_id = Some(value);
                } else {
                    current_task_title = Some(value);
                }
            }
            "current milestone" => current_milestone = Some(value),
            "current cycle" | "active cycle" | "current sprint" => current_cycle = Some(value),
            "next action" | "next" => next_action = Some(value),
            "required actor" => required_actor = Some(value),
            "owner" => owner = Some(value),
            "resume pointer" | "current" => resume_pointer = Some(value),
            "last audit" => last_audit = Some(value),
            "last agent session" => last_agent_session = Some(value),
            _ => {}
        }
    }
    Some(HandoffDocument {
        schema: CONTROL_PLANE_SCHEMA.into(),
        project_key: String::new(),
        resume_pointer: resume_pointer.unwrap_or_else(|| "Read canonical task source".into()),
        current_task_id,
        current_task_title,
        current_milestone,
        current_cycle,
        required_actor,
        next_action,
        last_audit,
        last_agent_session,
        owner,
        updated_at: None,
    })
}

fn looks_like_task_id(value: &str) -> bool {
    value.len() <= 128
        && value.contains('-')
        && !value.chars().any(char::is_whitespace)
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
}

fn normalize_handoff_label(value: &str) -> String {
    value
        .trim()
        .trim_start_matches('-')
        .trim()
        .trim_matches('*')
        .trim()
        .to_ascii_lowercase()
}

fn read_events(path: &Path, warnings: &mut Vec<String>) -> Vec<EventRecord> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            warnings.push("EVENTS.jsonl is missing".into());
            return Vec::new();
        }
        Err(error) => {
            warnings.push(error.to_string());
            return Vec::new();
        }
    };
    let end = file.metadata().map(|metadata| metadata.len()).unwrap_or(0);
    let start = end.saturating_sub(MAX_EVENT_TAIL_BYTES);
    if file.seek(SeekFrom::Start(start)).is_err() {
        warnings.push("EVENTS.jsonl tail seek failed".into());
        return Vec::new();
    }
    let mut bytes = Vec::new();
    if file.read_to_end(&mut bytes).is_err() {
        warnings.push("EVENTS.jsonl tail read failed".into());
        return Vec::new();
    }
    let raw = String::from_utf8_lossy(&bytes);
    let mut lines = raw.lines().rev().take(MAX_EVENTS).collect::<Vec<_>>();
    lines.reverse();
    lines
        .into_iter()
        .filter_map(|line| {
            if line.len() > MAX_EVENT_BYTES {
                warnings.push("EVENTS.jsonl contains an oversized event".into());
                return None;
            }
            match serde_json::from_str(line) {
                Ok(event) => Some(event),
                Err(_) => {
                    warnings.push("EVENTS.jsonl contains malformed event JSON".into());
                    None
                }
            }
        })
        .collect()
}

fn create_if_missing(path: &Path, contents: &str) -> Result<(), String> {
    match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(mut file) => std::io::Write::write_all(&mut file, contents.as_bytes())
            .map_err(|error| format!("write {}: {error}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
        Err(error) => Err(format!("create {}: {error}", path.display())),
    }
}

fn default_rules() -> String {
    "# H!veAI Project Rules v1\n\nActors: OWNER, HIVEAI_SYSTEM, CODEX, CLAUDE, CHATGPT, INDEPENDENT_AUDITOR.\nCanonical task truth remains in the project task source. Stricter project-specific rules remain authoritative.\n"
        .into()
}

fn git_state(git: &git_engine::GitSnapshot) -> GitControlPlaneState {
    let dirty = !git.staged_files.is_empty()
        || !git.unstaged_files.is_empty()
        || !git.untracked_files.is_empty();
    let conflicted = !git.conflicted_files.is_empty();
    let local_status = if conflicted {
        "CONFLICTED"
    } else if dirty {
        "DIRTY"
    } else {
        "CONNECTED"
    };
    let remote_status = if git.upstream.is_some() {
        "CONNECTED"
    } else {
        "UNKNOWN"
    };
    let sync_status = match (git.ahead_count, git.behind_count) {
        (Some(ahead), Some(behind)) if ahead > 0 && behind > 0 => "DIVERGED",
        (Some(0), Some(behind)) if behind > 0 => "BEHIND",
        (Some(ahead), Some(0)) if ahead > 0 => "AHEAD",
        (Some(0), Some(0)) => "IN_SYNC",
        _ => "UNKNOWN",
    };
    GitControlPlaneState {
        local_status: local_status.into(),
        remote_status: remote_status.into(),
        sync_status: sync_status.into(),
        branch: git.current_branch.clone(),
        head_sha: git.head_sha.clone(),
        upstream: git.upstream.clone(),
        ahead: git.ahead_count,
        behind: git.behind_count,
        dirty,
        conflicted,
        last_remote_observation_at: None,
        last_remote_observation_status: "UNKNOWN".into(),
        last_remote_observation_error: None,
        last_remote_observed_upstream: None,
        last_remote_observed_ahead: None,
        last_remote_observed_behind: None,
        last_remote_observed_diverged: None,
    }
}

fn health_for(snapshot: &ControlPlaneSnapshot, git: &GitControlPlaneState) -> String {
    if matches!(
        snapshot.health.as_str(),
        "MISSING" | "MALFORMED" | "MALFORMED_CONTROL_PLANE" | "UNADOPTED"
    ) {
        return snapshot.health.clone();
    }
    if !snapshot.adopted {
        return snapshot.health.clone();
    }
    if snapshot.health == "NEEDS_RECONCILIATION"
        || snapshot.workflow_state.as_deref() == Some("NEEDS_RECONCILIATION")
    {
        return "NEEDS_RECONCILIATION".into();
    }
    if snapshot.truth_sync.materialized_generation < snapshot.truth_sync.generation {
        return "TRUTH_SYNC_PENDING".into();
    }
    if snapshot.truth_sync.status == "DEGRADED" {
        return "TRUTH_SYNC_DEGRADED".into();
    }
    if snapshot.truth_sync.status == "PENDING" {
        return "TRUTH_SYNC_PENDING".into();
    }
    if snapshot.workflow_state.as_deref() == Some("BLOCKED") || !snapshot.blockers.is_empty() {
        return "BLOCKED".into();
    }
    if git.local_status == "UNKNOWN" {
        return "LOCAL_GIT_NOT_CONNECTED".into();
    }
    if git.dirty
        || git.conflicted
        || matches!(git.sync_status.as_str(), "DIVERGED" | "BEHIND" | "AHEAD")
    {
        return "SYNC_ATTENTION".into();
    }
    if git.last_remote_observation_status == "DEGRADED"
        || git.last_remote_observation_status == "FAILED"
    {
        return "SYNC_ATTENTION".into();
    }
    "HEALTHY".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn project(root: &Path) -> ProjectRecord {
        ProjectRecord {
            id: "p1".into(),
            name: "Fixture".into(),
            original_path: root.to_string_lossy().into(),
            normalized_path: root.to_string_lossy().into(),
            status: "ACTIVE".into(),
            priority: 0,
            preferred_builder: None,
            preferred_auditor: None,
            task_source_policy: None,
            preferred_agent_provider: None,
            registered_at: "now".into(),
            last_validated_at: None,
            repository: None,
        }
    }

    #[test]
    fn missing_control_plane_is_explicitly_unadopted() {
        let root = tempdir().unwrap();
        let snapshot = resolve_project(&project(root.path())).unwrap();
        assert!(!snapshot.adopted);
        assert_eq!(snapshot.health, "NEEDS_RECONCILIATION");
    }

    #[test]
    fn parser_uses_explicit_json_and_does_not_promote_body_colons() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        fs::write(
            root.path().join(PROJECT_JSON),
            serde_json::to_string(&ProjectDocument {
                schema: CONTROL_PLANE_SCHEMA.into(),
                project_key: "p1".into(),
                display_name: "Fixture".into(),
                repository: RepositoryIdentity {
                    owner: "owner".into(),
                    name: "repo".into(),
                    branch: None,
                },
                canonical_task_source: "TASKS.md".into(),
                events: EVENTS_JSONL.into(),
                event_sources: Vec::new(),
                local_registry_id: None,
                governance: None,
                rules: None,
                state: None,
                handoff: None,
            })
            .unwrap(),
        )
        .unwrap();
        fs::write(
            root.path().join(STATE_JSON),
            serde_json::to_string(&StateDocument {
                schema: CONTROL_PLANE_SCHEMA.into(),
                project_key: "p1".into(),
                workflow_state: "READY".into(),
                milestone: None,
                cycle: None,
                current_task_id: Some("T1".into()),
                current_task_title: None,
                required_actor: None,
                next_action: None,
                blockers: Vec::new(),
                progress_percent: Some(10),
                progress_scope: Some("MILESTONE:M09".into()),
                updated_by: None,
                updated_at: None,
            })
            .unwrap(),
        )
        .unwrap();
        fs::write(
            root.path().join(HANDOFF_MD),
            serde_json::to_string(&HandoffDocument {
                schema: CONTROL_PLANE_SCHEMA.into(),
                project_key: "p1".into(),
                resume_pointer: "TASKS.md".into(),
                current_task_id: Some("T1".into()),
                current_task_title: None,
                current_milestone: None,
                current_cycle: None,
                required_actor: None,
                next_action: None,
                last_audit: None,
                last_agent_session: None,
                owner: None,
                updated_at: None,
            })
            .unwrap(),
        )
        .unwrap();
        fs::write(root.path().join(RULES_MD), "Actors: OWNER, HIVEAI_SYSTEM").unwrap();
        fs::write(root.path().join(EVENTS_JSONL), "").unwrap();
        let snapshot = resolve_project(&project(root.path())).unwrap();
        assert!(snapshot.adopted);
        assert_eq!(snapshot.current_task_id.as_deref(), Some("T1"));
    }

    #[test]
    fn watcher_sources_are_deduplicated_and_bounded_to_relative_paths() {
        let root = tempdir().unwrap();
        let paths = watcher_sources(
            root.path(),
            &[
                "TASKS.md".into(),
                "../secret".into(),
                ".hiveai/STATE.json".into(),
            ],
        );
        assert!(paths.iter().any(|path| path.ends_with("TASKS.md")));
        assert_eq!(
            paths
                .iter()
                .filter(|path| path.ends_with("TASKS.md"))
                .count(),
            1
        );
        assert!(!paths
            .iter()
            .any(|path| path.to_string_lossy().contains("secret")));
    }

    #[test]
    fn sync_plan_refuses_divergence_and_requires_clean_owner_enabled_ff() {
        let plan = git_sync_plan(None, true);
        assert_eq!(plan.action, "LOCAL_GIT_NOT_CONNECTED");
    }

    #[test]
    fn legacy_repository_forms_are_bounded_and_normalized() {
        for raw in [
            "Sekiph82/RepoName",
            "https://github.com/Sekiph82/RepoName",
            "https://github.com/Sekiph82/RepoName.git",
            "git@github.com:Sekiph82/RepoName.git",
        ] {
            let identity = parse_repository_identity(raw).unwrap();
            assert_eq!(identity.owner, "Sekiph82");
            assert_eq!(identity.name, "RepoName");
            assert_eq!(identity.branch.as_deref(), Some("main"));
        }
        assert!(parse_repository_identity("https://example.com/Sekiph82/RepoName").is_none());
    }

    #[test]
    fn markdown_handoff_is_preserved_as_supporting_metadata() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        fs::write(
            root.path().join(HANDOFF_MD),
            "# Handoff\n\n| Current task | `PAG-M00-C003` |\n| Next action | Run the next gate |\n",
        )
        .unwrap();
        let mut warnings = Vec::new();
        let handoff = read_handoff(&root.path().join(HANDOFF_MD), &mut warnings).unwrap();
        assert_eq!(handoff.current_task_id.as_deref(), Some("PAG-M00-C003"));
        assert_eq!(handoff.next_action.as_deref(), Some("Run the next gate"));
    }

    #[test]
    fn events_reader_returns_newest_tail_from_large_history() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        let path = root.path().join(EVENTS_JSONL);
        let mut body = String::new();
        for index in 0..300 {
            body.push_str(&format!("{{\"eventId\":\"e{index}\",\"type\":\"WORKFLOW_CHANGED\",\"projectKey\":\"p1\",\"at\":\"2026-09-08T00:{:02}:00Z\",\"actor\":\"SYSTEM\",\"summary\":\"event {index}\"}}\n", index % 60));
        }
        fs::write(&path, body).unwrap();
        let mut warnings = Vec::new();
        let events = read_events(&path, &mut warnings);
        assert_eq!(events.len(), MAX_EVENTS);
        assert_eq!(events.last().unwrap().event_id, "e299");
    }

    #[test]
    fn upgrade_preserves_unknown_project_fields_and_nested_task_source() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        fs::create_dir(root.path().join("H!veAI")).unwrap();
        fs::write(root.path().join("H!veAI/TASKS.md"), "# Tasks\n").unwrap();
        fs::write(root.path().join(RULES_MD), "project-specific rule").unwrap();
        fs::write(
            root.path().join(STATE_JSON),
            "{\"schema\":\"hiveai-state/v1\",\"workflowState\":\"READY\"}",
        )
        .unwrap();
        fs::write(
            root.path().join(HANDOFF_MD),
            "# Human handoff\nDo not rewrite this body.\n",
        )
        .unwrap();
        fs::write(root.path().join(EVENTS_JSONL), "").unwrap();
        fs::write(root.path().join(PROJECT_JSON), "{\"schema\":\"hiveai-project/v1\",\"projectKey\":\"p1\",\"displayName\":\"P\",\"repository\":\"Sekiph82/P\",\"canonicalTaskSource\":\"H!veAI/TASKS.md\",\"custom\":\"keep\"}").unwrap();
        let record = project(root.path());
        assert!(upgrade_control_plane(&record).unwrap());
        let upgraded: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(root.path().join(PROJECT_JSON)).unwrap())
                .unwrap();
        assert_eq!(upgraded["schema"], CONTROL_PLANE_SCHEMA);
        assert_eq!(upgraded["repository"]["name"], "P");
        assert_eq!(upgraded["custom"], "keep");
        assert_eq!(upgraded["events"], EVENTS_JSONL);
        assert!(upgraded["eventSources"].is_array());
        let reparsed: ProjectDocument = serde_json::from_value(upgraded).unwrap();
        assert_eq!(reparsed.canonical_task_source, "H!veAI/TASKS.md");
        assert!(fs::read_to_string(root.path().join(HANDOFF_MD))
            .unwrap()
            .contains("Do not rewrite"));
    }

    #[test]
    fn upgrade_reconciles_typed_handoff_without_conflating_cycle_and_task() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        fs::write(
            root.path().join("TASKS.md"),
            "- PAG-M00-C004: Bootstrap Reliability\n",
        )
        .unwrap();
        fs::write(root.path().join(RULES_MD), "owner-only").unwrap();
        fs::write(root.path().join(EVENTS_JSONL), "").unwrap();
        fs::write(
            root.path().join(STATE_JSON),
            "{\"schema\":\"hiveai-state/v1\",\"workflowState\":\"READY\"}",
        )
        .unwrap();
        fs::write(
            root.path().join(HANDOFF_MD),
            "# Handoff\n\n- Active cycle: `PAG-M00-C003`\n- Current task ID: `PAG-M00-C004`\n- Required actor: CODEX\n- Next action: Run the next gate\n",
        )
        .unwrap();
        fs::write(
            root.path().join(PROJECT_JSON),
            "{\"schema\":\"hiveai-project/v1\",\"projectKey\":\"p1\",\"displayName\":\"P\",\"repository\":\"Sekiph82/P\",\"canonicalTaskSource\":\"TASKS.md\"}",
        )
        .unwrap();

        assert!(upgrade_control_plane(&project(root.path())).unwrap());
        let state: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(root.path().join(STATE_JSON)).unwrap())
                .unwrap();
        assert_eq!(state["currentCycle"], "PAG-M00-C003");
        assert_eq!(state["currentTaskId"], "PAG-M00-C004");
        assert_eq!(state["currentTaskTitle"], "Bootstrap Reliability");
        assert_eq!(state["requiredActor"], "CODEX");
        assert_eq!(state["nextAction"], "Run the next gate");
    }

    #[test]
    fn all_target_project_shapes_parse_with_the_final_control_plane_contract() {
        let fixtures = [
            ("AI-Commerce-HQ", "H!veAI/TASKS.md"),
            ("Bulk-Edit", "TASKS.md"),
            ("FormuLab", "docs/FORMULAB_V1_TASK_TRACKER.md"),
            ("PackLab", "TASKS.md"),
            ("PackLab 3D", "tasks.md"),
            ("ScrubBots", "tasks.md"),
            ("ScrubBots-Level-Factory", "tasks.md"),
            ("fmcg-erp-system", "TASKS.md"),
        ];
        for (name, canonical) in fixtures {
            let value = serde_json::json!({
                "schema": CONTROL_PLANE_SCHEMA,
                "projectKey": name,
                "displayName": name,
                "repository": {"owner": "Sekiph82", "name": name, "branch": "main"},
                "canonicalTaskSource": canonical,
                "rules": RULES_MD,
                "state": STATE_JSON,
                "handoff": HANDOFF_MD,
                "events": EVENTS_JSONL,
                "eventSources": []
            });
            let document: ProjectDocument = serde_json::from_value(value).unwrap();
            assert_eq!(document.events, EVENTS_JSONL);
            assert!(document.event_sources.is_empty());
            assert_eq!(document.canonical_task_source, canonical);
        }
    }

    #[test]
    fn events_scalar_and_event_sources_array_round_trip_without_alias_confusion() {
        let raw = serde_json::json!({
            "schema": CONTROL_PLANE_SCHEMA,
            "projectKey": "p1",
            "displayName": "Fixture",
            "repository": "Sekiph82/Fixture",
            "canonicalTaskSource": "TASKS.md",
            "events": EVENTS_JSONL,
            "eventSources": ["tasks.md", ".git/HEAD"]
        });
        let document: ProjectDocument = serde_json::from_value(raw).unwrap();
        assert_eq!(document.events, EVENTS_JSONL);
        assert_eq!(document.event_sources, ["tasks.md", ".git/HEAD"]);
        assert!(
            serde_json::from_value::<ProjectDocument>(serde_json::json!({
                "schema": CONTROL_PLANE_SCHEMA,
                "projectKey": "p1",
                "displayName": "Fixture",
                "repository": "Sekiph82/Fixture",
                "canonicalTaskSource": "TASKS.md",
                "events": [EVENTS_JSONL]
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<ProjectDocument>(serde_json::json!({
                "schema": CONTROL_PLANE_SCHEMA,
                "projectKey": "p1",
                "displayName": "Fixture",
                "repository": "Sekiph82/Fixture",
                "canonicalTaskSource": "TASKS.md",
                "events": EVENTS_JSONL,
                "eventSources": EVENTS_JSONL
            }))
            .is_err()
        );
    }

    #[test]
    fn markdown_handoff_keeps_cycle_and_task_identity_separate() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        fs::write(
            root.path().join(HANDOFF_MD),
            "- Active cycle: `PAG-M00-C003`\n- Current task ID: `PAG-M00-C004`\n- Current task title: Bootstrap Reliability\n- Required actor: CODEX\n- Next action: Run the next gate\n",
        )
        .unwrap();
        let handoff = read_handoff(&root.path().join(HANDOFF_MD), &mut Vec::new()).unwrap();
        assert_eq!(handoff.current_cycle.as_deref(), Some("PAG-M00-C003"));
        assert_eq!(handoff.current_task_id.as_deref(), Some("PAG-M00-C004"));
        assert_eq!(
            handoff.current_task_title.as_deref(),
            Some("Bootstrap Reliability")
        );
        assert_eq!(handoff.required_actor.as_deref(), Some("CODEX"));
    }

    #[test]
    fn adoption_source_resolution_supports_nested_lowercase_and_ambiguity_refusal() {
        let formulab = tempdir().unwrap();
        fs::create_dir_all(formulab.path().join("docs")).unwrap();
        fs::write(
            formulab.path().join("docs/FORMULAB_V1_TASK_TRACKER.md"),
            "# tracker",
        )
        .unwrap();
        assert_eq!(
            canonical_task_source_for_adoption(
                &ProjectRecord {
                    name: "FormuLab".into(),
                    ..project(formulab.path())
                },
                formulab.path()
            )
            .unwrap(),
            "docs/FORMULAB_V1_TASK_TRACKER.md"
        );

        let packlab = tempdir().unwrap();
        fs::write(packlab.path().join("tasks.md"), "# tasks").unwrap();
        assert_eq!(
            canonical_task_source_for_adoption(
                &ProjectRecord {
                    name: "PackLab 3D".into(),
                    ..project(packlab.path())
                },
                packlab.path()
            )
            .unwrap(),
            "tasks.md"
        );

        let ambiguous = tempdir().unwrap();
        fs::write(ambiguous.path().join("TASKS.md"), "# tasks").unwrap();
        fs::create_dir_all(ambiguous.path().join("docs")).unwrap();
        fs::write(
            ambiguous.path().join("docs/FORMULAB_V1_TASK_TRACKER.md"),
            "# another",
        )
        .unwrap();
        assert!(
            canonical_task_source_for_adoption(&project(ambiguous.path()), ambiguous.path())
                .unwrap_err()
                .contains("multiple canonical task ledgers")
        );
    }

    #[test]
    fn event_replay_is_rejected_after_display_tail_is_exceeded() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        let mut history = String::new();
        for index in 0..300 {
            history.push_str(
                &serde_json::to_string(&EventRecord {
                    event_id: format!("event-{index}"),
                    event_type: "WORKFLOW_CHANGED".into(),
                    project_key: "p1".into(),
                    occurred_at: format!("2026-09-08T00:{:02}:00Z", index % 60),
                    actor: "SYSTEM".into(),
                    summary: format!("event {index}"),
                    task_id: None,
                    evidence_refs: Vec::new(),
                })
                .unwrap(),
            );
            history.push('\n');
        }
        fs::write(root.path().join(EVENTS_JSONL), history).unwrap();
        let record = project(root.path());
        let replay = EventRecord {
            event_id: "event-0".into(),
            event_type: "WORKFLOW_CHANGED".into(),
            project_key: "p1".into(),
            occurred_at: "2026-09-08T01:00:00Z".into(),
            actor: "SYSTEM".into(),
            summary: "replay".into(),
            task_id: None,
            evidence_refs: Vec::new(),
        };
        assert!(!append_event(&record, replay.clone()).unwrap());
        let fresh = EventRecord {
            event_id: "event-300".into(),
            ..replay
        };
        assert!(append_event(&record, fresh).unwrap());
        assert!(!append_event(
            &record,
            EventRecord {
                event_id: "event-0".into(),
                event_type: "WORKFLOW_CHANGED".into(),
                project_key: "p1".into(),
                occurred_at: "now".into(),
                actor: "SYSTEM".into(),
                summary: "replay".into(),
                task_id: None,
                evidence_refs: Vec::new(),
            }
        )
        .unwrap());
    }

    #[test]
    fn event_index_tail_closes_crash_window_after_append_before_index() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        fs::write(
            root.path().join(EVENT_INDEX_JSON),
            "{\"schema\":\"hiveai-event-index/v1\",\"eventIds\":[]}",
        )
        .unwrap();
        let event = EventRecord {
            event_id: "crash-window-1".into(),
            event_type: "WORKFLOW_CHANGED".into(),
            project_key: "p1".into(),
            occurred_at: "2026-09-09T00:00:00Z".into(),
            actor: "SYSTEM".into(),
            summary: "durable append".into(),
            task_id: None,
            evidence_refs: Vec::new(),
        };
        fail_next_event_index_persistence();
        let error = append_event(&project(root.path()), event.clone()).unwrap_err();
        assert!(error.contains("EVENT_INDEX_FAILPOINT"));
        assert!(fs::read_to_string(root.path().join(EVENTS_JSONL))
            .unwrap()
            .contains("crash-window-1"));
        assert!(!append_event(&project(root.path()), event).unwrap());
        let index: EventIndexDocument =
            serde_json::from_str(&fs::read_to_string(root.path().join(EVENT_INDEX_JSON)).unwrap())
                .unwrap();
        assert_eq!(index.event_ids, vec!["crash-window-1"]);
    }

    #[test]
    fn event_idempotency_horizon_is_explicit_after_more_than_4096_events() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        let mut history = String::new();
        for index in 0..=MAX_EVENT_INDEX_IDS {
            history.push_str(
                &serde_json::to_string(&EventRecord {
                    event_id: format!("horizon-{index}"),
                    event_type: "WORKFLOW_CHANGED".into(),
                    project_key: "p1".into(),
                    occurred_at: format!("2026-09-09T00:{:02}:00Z", index % 60),
                    actor: "SYSTEM".into(),
                    summary: "bounded history".into(),
                    task_id: None,
                    evidence_refs: Vec::new(),
                })
                .unwrap(),
            );
            history.push('\n');
        }
        fs::write(root.path().join(EVENTS_JSONL), history).unwrap();
        let index = load_event_index(
            &root.path().join(EVENT_INDEX_JSON),
            &root.path().join(EVENTS_JSONL),
        )
        .unwrap();
        assert_eq!(index.len(), MAX_EVENT_INDEX_IDS);
        assert!(!index.iter().any(|value| value == "horizon-0"));
        assert!(index.iter().any(|value| value == "horizon-4096"));
        assert!(EVENT_INDEX_HORIZON.contains("4096"));
    }

    #[test]
    fn project_truth_survives_materialization_and_database_restart() {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(
            project_dir.path().join("TASKS.md"),
            "# Work\n\n- [ ] TASK-A: First task\n- [ ] TASK-B: Second task\n",
        )
        .unwrap();
        let git = |args: &[&str]| {
            let output = std::process::Command::new("git")
                .args(args)
                .current_dir(project_dir.path())
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "git {:?}: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        };
        git(&["init", "-q"]);
        git(&["config", "user.name", "M16I Test"]);
        git(&["config", "user.email", "m16i@example.com"]);
        git(&["add", "TASKS.md"]);
        git(&["commit", "-qm", "seed task ledger"]);
        git(&[
            "remote",
            "add",
            "origin",
            "https://github.com/owner/repo.git",
        ]);
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("M16I Restart Fixture".into()),
            },
        )
        .unwrap();
        adopt(&database, &project.id).unwrap();
        task_intelligence::parse(&database, &project.id).unwrap();
        let task_b: String = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT id FROM tasks WHERE project_id=?1 AND title='Second task'",
                [&project.id],
                |row| row.get(0),
            )
            .unwrap();
        crate::workflow::transition(
            &database,
            crate::workflow::WorkflowTransitionRequest {
                task_id: task_b.clone(),
                expected_from_state: crate::workflow::WorkflowState::Backlog,
                to_state: crate::workflow::WorkflowState::PlanningRequired,
                actor_type: crate::workflow::ActorType::Human,
                request_id: "restart-workflow".into(),
                summary: "selected task B".into(),
                evidence_refs: Vec::new(),
            },
        )
        .unwrap();
        let mut state: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(project_dir.path().join(STATE_JSON)).unwrap())
                .unwrap();
        state["workflowState"] = "READY".into();
        fs::write(
            project_dir.path().join(STATE_JSON),
            serde_json::to_string_pretty(&state).unwrap(),
        )
        .unwrap();
        let result = materialize_project_truth(&database, &project.id, "RESTART_FIXTURE").unwrap();
        assert!(result.state_written);
        let persisted: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(project_dir.path().join(STATE_JSON)).unwrap())
                .unwrap();
        assert_eq!(persisted["currentTaskId"], task_b);
        assert_eq!(persisted["updatedBy"], "HIVEAI_SYSTEM");
        drop(database);

        let restarted = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let center = crate::command_center::snapshot(&restarted).unwrap();
        let summary = center
            .projects
            .iter()
            .find(|value| value.project_id == project.id)
            .unwrap();
        assert_eq!(summary.current_task.as_ref().unwrap().task_id, task_b);
        let cockpit = crate::project_cockpit::snapshot(&restarted, &project.id).unwrap();
        assert_eq!(
            cockpit.truth.current_task_id.as_deref(),
            Some(task_b.as_str())
        );
    }

    #[test]
    fn m16l_current_command_center_cockpit_and_control_reads_are_observational() {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(
            project_dir.path().join("TASKS.md"),
            "# Work\n\n- [ ] TASK-A: First task\n",
        )
        .unwrap();
        let git = |args: &[&str]| {
            let output = std::process::Command::new("git")
                .args(args)
                .current_dir(project_dir.path())
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "git {:?}: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        };
        git(&["init", "-q"]);
        git(&["config", "user.name", "M16L Purity"]);
        git(&["config", "user.email", "m16l@example.com"]);
        git(&["add", "TASKS.md"]);
        git(&["commit", "-qm", "seed"]);
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let registered = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("M16L Purity Fixture".into()),
            },
        )
        .unwrap();
        adopt(&database, &registered.id).unwrap();
        let tracked_files = [STATE_JSON, HANDOFF_MD, EVENTS_JSONL, EVENT_INDEX_JSON];
        let before_files = tracked_files
            .iter()
            .map(|file| fs::read(project_dir.path().join(file)).unwrap_or_default())
            .collect::<Vec<_>>();
        let before_sync = truth_sync_status(&database, &registered.id);
        assert!(truth_sync_is_current(&before_sync));
        let before_db = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT truth_generation, truth_materialized_generation, truth_sync_status, truth_sync_retry_count FROM projects WHERE id=?1",
                [&registered.id],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?, row.get::<_, String>(2)?, row.get::<_, i64>(3)?)),
            )
        .unwrap();

        let before_evidence = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT (SELECT COUNT(*) FROM git_snapshots), (SELECT COUNT(*) FROM task_events), (SELECT COUNT(*) FROM project_snapshots)",
                [],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?)),
            )
            .unwrap();

        for _ in 0..100 {
            snapshot(&database, &registered.id).unwrap();
        }
        for _ in 0..100 {
            let center = crate::command_center::snapshot(&database).unwrap();
            assert!(center
                .projects
                .iter()
                .any(|project| project.project_id == registered.id));
        }
        for _ in 0..100 {
            let cockpit = crate::project_cockpit::snapshot(&database, &registered.id).unwrap();
            assert_eq!(cockpit.project.id, registered.id);
            assert_eq!(cockpit.project_summary.project_id, registered.id);
        }

        let after_files = tracked_files
            .iter()
            .map(|file| fs::read(project_dir.path().join(file)).unwrap_or_default())
            .collect::<Vec<_>>();
        assert_eq!(after_files, before_files);
        let after_db = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT truth_generation, truth_materialized_generation, truth_sync_status, truth_sync_retry_count FROM projects WHERE id=?1",
                [&registered.id],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?, row.get::<_, String>(2)?, row.get::<_, i64>(3)?)),
            )
        .unwrap();
        assert_eq!(after_db, before_db);
        let after_evidence = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT (SELECT COUNT(*) FROM git_snapshots), (SELECT COUNT(*) FROM task_events), (SELECT COUNT(*) FROM project_snapshots)",
                [],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?)),
            )
            .unwrap();
        assert_eq!(after_evidence, before_evidence);

        let mut connection = database.open_connection().unwrap();
        let tx = connection.transaction().unwrap();
        let next_generation =
            mark_truth_dirty_tx(&tx, &registered.id, "M16L_PURPOSEFUL_MUTATION").unwrap();
        tx.commit().unwrap();
        assert_eq!(next_generation, before_sync.generation + 1);
        let recovered = snapshot(&database, &registered.id).unwrap();
        assert_eq!(recovered.truth_sync.status, "CURRENT");
        assert_eq!(
            recovered.truth_sync.generation,
            recovered.truth_sync.materialized_generation
        );
        assert!(recovered.truth_sync.generation > before_sync.generation);
    }

    #[test]
    fn m16m_physical_adoption_converges_stale_db_and_fails_closed() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let root = tempdir().unwrap();
        fs::write(root.path().join("TASKS.md"), "- [ ] TASK-A: Adopted task\n").unwrap();
        let registered = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: root.path().to_string_lossy().into_owned(),
                name: Some("Physical adoption fixture".into()),
            },
        )
        .unwrap();
        let before_adopt = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT control_plane_status, truth_generation FROM projects WHERE id=?1",
                [&registered.id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
            )
            .unwrap();
        assert_eq!(before_adopt, ("UNADOPTED".into(), 0));
        adopt(&database, &registered.id).unwrap();
        let after_adopt = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT control_plane_status, control_plane_schema, truth_generation, truth_sync_status FROM projects WHERE id=?1",
                [&registered.id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?, row.get::<_, i64>(2)?, row.get::<_, String>(3)?)),
            )
            .unwrap();
        assert_eq!(after_adopt.0, "ADOPTED");
        assert_eq!(after_adopt.1.as_deref(), Some(CONTROL_PLANE_SCHEMA));
        assert_eq!(after_adopt.2, 1);
        assert_eq!(after_adopt.3, "CURRENT");
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "UPDATE projects SET control_plane_status='UNADOPTED', control_plane_schema=NULL, control_plane_revision=NULL, truth_generation=0, truth_materialized_generation=0, truth_sync_status='CURRENT' WHERE id=?1",
                [&registered.id],
            )
            .unwrap();
        assert_eq!(
            probe_physical_control_plane(root.path()),
            PhysicalControlPlaneStatus::Adopted
        );
        assert_eq!(
            converge_physical_adoption(&database, &registered.id).unwrap(),
            PhysicalControlPlaneStatus::Adopted
        );
        let converged = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT control_plane_status, control_plane_schema, truth_generation, truth_materialized_generation, truth_sync_status, truth_sync_trigger FROM projects WHERE id=?1",
                [&registered.id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?, row.get::<_, i64>(2)?, row.get::<_, i64>(3)?, row.get::<_, String>(4)?, row.get::<_, Option<String>>(5)?)),
            )
            .unwrap();
        assert_eq!(converged.0, "ADOPTED");
        assert_eq!(converged.1.as_deref(), Some(CONTROL_PLANE_SCHEMA));
        assert_eq!(converged.2, 1);
        assert_eq!(converged.3, 0);
        assert_eq!(converged.4, "PENDING");
        assert_eq!(converged.5.as_deref(), Some("GENERATION_BOOTSTRAP"));
        materialize_project_truth(&database, &registered.id, "M16M_FIXTURE").unwrap();
        assert_eq!(
            converge_physical_adoption(&database, &registered.id).unwrap(),
            PhysicalControlPlaneStatus::Adopted
        );
        assert_eq!(truth_sync_status(&database, &registered.id).generation, 1);

        fs::write(root.path().join(PROJECT_JSON), "{ malformed").unwrap();
        connection
            .execute(
                "UPDATE projects SET control_plane_status='ADOPTED' WHERE id=?1",
                [&registered.id],
            )
            .unwrap();
        assert_eq!(
            converge_physical_adoption(&database, &registered.id).unwrap(),
            PhysicalControlPlaneStatus::Malformed
        );
        let malformed = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT control_plane_status, control_plane_sync_status, truth_generation FROM projects WHERE id=?1",
                [&registered.id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, i64>(2)?)),
            )
            .unwrap();
        assert_eq!(malformed.0, "MALFORMED");
        assert_eq!(malformed.1, "DEGRADED");
        assert_eq!(malformed.2, 1);

        fs::remove_file(root.path().join(PROJECT_JSON)).unwrap();
        connection
            .execute(
                "UPDATE projects SET control_plane_status='ADOPTED' WHERE id=?1",
                [&registered.id],
            )
            .unwrap();
        assert_eq!(
            converge_physical_adoption(&database, &registered.id).unwrap(),
            PhysicalControlPlaneStatus::Missing
        );
        assert_eq!(
            database
                .open_connection()
                .unwrap()
                .query_row(
                    "SELECT control_plane_status, control_plane_sync_status FROM projects WHERE id=?1",
                    [&registered.id],
                    |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
                )
                .unwrap(),
            ("UNADOPTED".into(), "DEGRADED".into())
        );
    }

    #[test]
    fn m16j_canonical_event_round_trip_uses_portable_identity() {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".hiveai")).unwrap();
        fs::write(
            root.path().join(PROJECT_JSON),
            serde_json::json!({
                "schema": CONTROL_PLANE_SCHEMA,
                "projectKey": "portable-project",
                "displayName": "Portable",
                "repository": "owner/portable",
                "canonicalTaskSource": "TASKS.md",
                "events": EVENTS_JSONL
            })
            .to_string(),
        )
        .unwrap();
        let event = EventRecord {
            event_id: "portable-event".into(),
            event_type: "WORKFLOW_CHANGED".into(),
            project_key: "local-registry-uuid".into(),
            occurred_at: "2026-09-09T00:00:00Z".into(),
            actor: "SYSTEM".into(),
            summary: "portable event".into(),
            task_id: Some("TASK-1".into()),
            evidence_refs: Vec::new(),
        };
        assert!(append_event(&project(root.path()), event).unwrap());
        let raw = fs::read_to_string(root.path().join(EVENTS_JSONL)).unwrap();
        let value: serde_json::Value = serde_json::from_str(raw.trim()).unwrap();
        assert_eq!(value["schema"], "hiveai-event/v1");
        assert_eq!(value["projectKey"], "portable-project");
        assert_eq!(value["type"], "WORKFLOW_CHANGED");
        assert_eq!(value["taskId"], "TASK-1");
        assert!(value.get("workflowState").is_some());
        assert!(value.get("commit").is_some());
        assert!(value.get("auditId").is_some());
        assert!(value.get("sessionId").is_some());
        assert_eq!(
            read_events(&root.path().join(EVENTS_JSONL), &mut Vec::new()).len(),
            1
        );
    }

    #[test]
    fn m16j_handoff_managed_block_is_lossless_and_fails_closed() {
        let truth = ProjectTruth {
            project_id: "local".into(),
            current_task_id: Some("TASK-1".into()),
            current_task_title: Some("Keep notes".into()),
            current_task_status: Some("OPEN".into()),
            current_milestone: Some("M16".into()),
            current_cycle: Some("M16J".into()),
            workflow_state: Some("IN_PROGRESS".into()),
            required_actor: Some("OWNER".into()),
            next_action: Some("Review".into()),
            blockers: Vec::new(),
            progress_percent: Some(50),
            progress_scope: Some("MILESTONE:M16".into()),
            authority_source: "STATE.json".into(),
            provenance: vec![STATE_JSON.into()],
            reconciliation_state: "RESOLVED".into(),
            warnings: Vec::new(),
        };
        let notes = "# Owner heading\n\nDo not rewrite this prose.\n";
        let first =
            render_materialized_handoff("Fixture", "portable-project", &truth, notes).unwrap();
        let second =
            render_materialized_handoff("Fixture", "portable-project", &truth, &first).unwrap();
        assert_eq!(second.matches(HANDOFF_BEGIN).count(), 1);
        assert_eq!(second.matches(HANDOFF_END).count(), 1);
        assert!(second.contains("# Owner heading"));
        assert!(second.contains("Do not rewrite this prose."));
        assert!(render_materialized_handoff(
            "Fixture",
            "portable-project",
            &truth,
            &format!("{HANDOFF_BEGIN}\nold\n{HANDOFF_BEGIN}\n{HANDOFF_END}")
        )
        .is_err());
        assert!(
            render_materialized_handoff("Fixture", "portable-project", &truth, notes)
                .unwrap()
                .contains("portable-project")
        );
        assert!(!handoff_automation_allowed(
            &serde_json::json!({"handoffAutomationAllowed": false}),
            "builder automation may not rewrite HANDOFF without owner approval"
        ));
    }

    #[test]
    fn m16j_truth_sync_failure_is_durable_and_restart_retry_recovers() {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(
            project_dir.path().join("TASKS.md"),
            "# Work\n\n- [ ] TASK-A: First task\n",
        )
        .unwrap();
        let git = |args: &[&str]| {
            let output = std::process::Command::new("git")
                .args(args)
                .current_dir(project_dir.path())
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "git {:?}: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        };
        git(&["init", "-q"]);
        git(&["config", "user.name", "M16J Test"]);
        git(&["config", "user.email", "m16j@example.com"]);
        git(&["add", "TASKS.md"]);
        git(&["commit", "-qm", "seed"]);
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let registered = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Portable Retry".into()),
            },
        )
        .unwrap();
        adopt(&database, &registered.id).unwrap();
        let state = fs::read_to_string(project_dir.path().join(STATE_JSON)).unwrap();
        fs::remove_file(project_dir.path().join(STATE_JSON)).unwrap();
        assert!(materialize_project_truth(&database, &registered.id, "M16J_FAILPOINT").is_err());
        assert_eq!(
            truth_sync_status(&database, &registered.id).status,
            "DEGRADED"
        );
        fs::write(project_dir.path().join(STATE_JSON), state).unwrap();
        assert_eq!(retry_pending_truth_sync(&database), 1);
        assert_eq!(
            truth_sync_status(&database, &registered.id).status,
            "CURRENT"
        );
    }

    #[test]
    fn remote_observation_fetches_when_auto_fast_forward_is_disabled() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let remote = tempdir().unwrap();
        let local = tempdir().unwrap();
        let other = tempdir().unwrap();
        let git = |root: &Path, args: &[&str]| {
            let output = std::process::Command::new("git")
                .args(args)
                .current_dir(root)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "git {:?}: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        };
        git(remote.path(), &["init", "--bare", "-q"]);
        git(
            local.path(),
            &["-c", "init.defaultBranch=main", "init", "-q"],
        );
        git(local.path(), &["config", "user.name", "M16H Test"]);
        git(local.path(), &["config", "user.email", "m16h@example.com"]);
        fs::write(local.path().join("README.md"), "one\n").unwrap();
        git(local.path(), &["add", "README.md"]);
        git(local.path(), &["commit", "-qm", "initial"]);
        git(
            local.path(),
            &[
                "remote",
                "add",
                "origin",
                remote.path().to_string_lossy().as_ref(),
            ],
        );
        git(local.path(), &["push", "-q", "-u", "origin", "main"]);
        git(
            other.path(),
            &[
                "clone",
                "-q",
                "-b",
                "main",
                remote.path().to_string_lossy().as_ref(),
                ".",
            ],
        );
        git(other.path(), &["config", "user.name", "M16H Other"]);
        git(other.path(), &["config", "user.email", "other@example.com"]);
        fs::write(other.path().join("README.md"), "two\n").unwrap();
        git(other.path(), &["add", "README.md"]);
        git(other.path(), &["commit", "-qm", "remote advance"]);
        git(other.path(), &["push", "-q", "origin", "main"]);
        let registered = crate::projects::register_project(
            &database,
            crate::projects::RegisterProjectRequest {
                path: local.path().to_string_lossy().into_owned(),
                name: Some("M16H Remote Fixture".into()),
            },
        )
        .unwrap();
        let before = git_engine::snapshot(
            &database,
            GitSnapshotRequest {
                project_id: registered.id.clone(),
                persist: Some(false),
            },
        )
        .unwrap();
        let observed = sync_remote(&database, &registered.id, false).unwrap();
        assert_eq!(observed.action, "SYNC_ATTENTION");
        assert!(observed.fetch_required);
        let after_observation = git_engine::snapshot(
            &database,
            GitSnapshotRequest {
                project_id: registered.id.clone(),
                persist: Some(false),
            },
        )
        .unwrap();
        assert_eq!(after_observation.head_sha, before.head_sha);
        assert_eq!(after_observation.behind_count, Some(1));
        let observation = database
            .open_connection()
            .unwrap()
            .query_row(
                "SELECT last_remote_observation_status,last_remote_observed_behind FROM projects WHERE id=?1",
                [&registered.id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<i64>>(1)?)),
            )
            .unwrap();
        assert_eq!(observation.0, "SUCCESS");
        assert_eq!(observation.1, Some(1));
        let merged = sync_remote(&database, &registered.id, true).unwrap();
        assert_eq!(merged.action, "FAST_FORWARD_COMPLETE");
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct LiteralPortfolioFixture {
        name: String,
        project_key: String,
        canonical_task_source: String,
        repository: String,
        branch: String,
        target_branch_head_sha: String,
        project_blob_sha: String,
        rules_blob_sha: String,
        rules: String,
        project: serde_json::Value,
        state: serde_json::Value,
        handoff: serde_json::Value,
        events: serde_json::Value,
        #[serde(default)]
        event_sources: Vec<String>,
        rules_text: String,
    }

    #[test]
    fn m16k_literal_eight_repository_contracts_are_checked_in_and_resolvable() {
        let fixtures: Vec<LiteralPortfolioFixture> =
            serde_json::from_str(include_str!("../fixtures/m16k/portfolio.json")).unwrap();
        assert_eq!(fixtures.len(), 8);
        let expected = [
            ("ai-commerce-hq", "H!veAI/TASKS.md"),
            ("bulk-edit", "TASKS.md"),
            ("fmcg-erp-system", "TASKS.md"),
            ("formulab", "docs/FORMULAB_V1_TASK_TRACKER.md"),
            ("packlab", "TASKS.md"),
            ("packlab-3d", "tasks.md"),
            ("scrubbots", "tasks.md"),
            ("scrubbots-level-factory", "tasks.md"),
        ];
        let actual_sources = [
            "H!veAI/TASKS.md",
            "TASKS.md",
            "TASKS.md",
            "docs/FORMULAB_V1_TASK_TRACKER.md",
            "TASKS.md",
            "tasks.md",
            "tasks.md",
            "tasks.md",
        ];
        let expected_heads = [
            "9233c7df49eaf43c58185f49d20ae607156836fa",
            "05a059ab5aff211be8a9cd8feccd3d5cba7845fd",
            "77aa33b2d18811e019d17609a4298929946e603a",
            "db2520d648a7b379af15e2516c6c220f91aabc04",
            "46cdf07c3c5138594301214ffb351792c378e125",
            "af5d83f089d753b82101369c303880d45b2ffc9e",
            "f44f1d50c6ea8f3427a4a62410887cc2b554945b",
            "f1a1c4f1df18cf8384733d7d372bd6c1aafffc8a",
        ];
        let expected_project_blobs = [
            "f0ffda434aa8e8b321b7582603a4fc9f8f0d7e5f",
            "6738f34092d3d1198cb798e514810bfd18389bc2",
            "5793d3cdaf076e46e6a889c6dd4488c3d5b04195",
            "8231abb9f1fe845318ee9c4de795683d9ceaf1b9",
            "73376f94e0892b6444b78b4024e92be2aa1bfd45",
            "d552a805862a7ba96b06ed1bddd4808050f881b6",
            "14a9d6592fb2d27fa39d753e5069782154a1c842",
            "8da21e67de8235d09a8078e7d45a872e1e5e401e",
        ];
        let expected_rules_blobs = [
            "fe47f40fa5276d8f371da52cd495767b5c0ed7fc",
            "27287ce352cc781101e090ac9105e83952b06845",
            "763d6365ff6aa15a6208d9c197cc58558dd042cc",
            "393d8c624a6b0517cf7ee266b1ee6c43e6bdf133",
            "f9953064eaaf935bfe88c0c7f716de4e2459094f",
            "05d8a810e1971362d66c2722c0117b418e73301c",
            "78e15149669a3c4a7191dd0b60202dc9e764c631",
            "e551ed83dd5b28f6d59bf934f6960c0bee6a09e9",
        ];
        let expected_repo_names = [
            "AI-Commerce-HQ",
            "Bulk-Edit",
            "fmcg-erp-system",
            "FormuLab",
            "PackLab",
            "PackLab-3D",
            "Scrubbots",
            "ScrubBots-Level-Factory",
        ];
        for (index, fixture) in fixtures.iter().enumerate() {
            let (key, source) = expected[index];
            let actual_source = actual_sources[index];
            assert_eq!(fixture.project_key, key);
            assert_eq!(fixture.canonical_task_source, source);
            assert!(!fixture.repository.is_empty());
            assert!(!fixture.branch.is_empty());
            assert_eq!(fixture.target_branch_head_sha, expected_heads[index]);
            assert_eq!(fixture.project_blob_sha, expected_project_blobs[index]);
            assert_eq!(fixture.rules_blob_sha, expected_rules_blobs[index]);
            assert_ne!(fixture.target_branch_head_sha, fixture.project_blob_sha);
            assert!(!fixture.name.is_empty());
            assert!(!fixture.rules.is_empty());
            assert_eq!(fixture.project["projectKey"], key);
            assert_eq!(fixture.project["canonicalTaskSource"], actual_source);
            let document: ProjectDocument = serde_json::from_value(fixture.project.clone())
                .expect("current fixture must parse through the production project parser");
            assert_eq!(document.schema, CONTROL_PLANE_SCHEMA);
            assert_eq!(document.project_key, key);
            assert_eq!(document.canonical_task_source, actual_source);
            assert_eq!(document.repository.owner, "Sekiph82");
            assert_eq!(document.repository.name, expected_repo_names[index]);
            assert_eq!(fixture.state["schema"], CONTROL_PLANE_SCHEMA);
            assert_eq!(fixture.state["projectKey"], key);
            assert_eq!(fixture.handoff["projectKey"], key);
            assert_eq!(fixture.events["schema"], "hiveai-event/v1");
            assert!(fixture.event_sources.is_empty());
            assert!(!fixture.rules_text.is_empty());
        }
        let level_factory = &fixtures[7];
        assert_eq!(
            level_factory.project["governance"]["builderMayMutateHandoff"],
            false
        );
        assert!(level_factory
            .rules_text
            .contains("owner-approved prompt explicitly changes governance"));
        let project = serde_json::json!({
            "schema": CONTROL_PLANE_SCHEMA,
            "projectKey": level_factory.project_key,
            "displayName": level_factory.name,
            "repository": level_factory.repository,
            "canonicalTaskSource": level_factory.canonical_task_source,
            "governance": {"builderMayMutateHandoff": false}
        });
        assert!(!handoff_automation_allowed(
            &project,
            "Codex may not rewrite HANDOFF unless an owner-approved prompt explicitly changes governance."
        ));
    }

    #[test]
    fn m16k_typed_governance_denies_actual_level_factory_and_ignores_malformed_handoff() {
        let project = serde_json::json!({
            "governance": {"builderMayMutateHandoff": false}
        });
        let decision = handoff_governance(
            &project,
            "Codex may not rewrite HANDOFF unless an owner-approved prompt explicitly changes governance.",
        );
        assert!(!decision.automation_allowed);
        assert!(decision.note.unwrap().contains("externally governed"));
        let malformed = format!("{HANDOFF_BEGIN}\nold\n{HANDOFF_BEGIN}\n{HANDOFF_END}");
        assert!(render_materialized_handoff(
            "Level Factory",
            "scrubbots-level-factory",
            &ProjectTruth {
                project_id: "level-factory".into(),
                current_task_id: None,
                current_task_title: None,
                current_task_status: None,
                current_milestone: None,
                current_cycle: None,
                workflow_state: None,
                required_actor: None,
                next_action: None,
                blockers: Vec::new(),
                progress_percent: None,
                progress_scope: None,
                authority_source: "TASKS.md".into(),
                provenance: Vec::new(),
                reconciliation_state: "RESOLVED".into(),
                warnings: Vec::new(),
            },
            &malformed,
        )
        .is_err());
        // The denied path never calls the renderer, so malformed markers do
        // not prevent STATE materialization.
        assert!(!decision.automation_allowed);
    }

    #[test]
    fn m16k_portable_project_serialization_omits_legacy_registry_identity() {
        let document = ProjectDocument {
            schema: CONTROL_PLANE_SCHEMA.into(),
            project_key: "portable-project".into(),
            display_name: "Portable".into(),
            repository: RepositoryIdentity {
                owner: "owner".into(),
                name: "repo".into(),
                branch: Some("main".into()),
            },
            canonical_task_source: "TASKS.md".into(),
            events: EVENTS_JSONL.into(),
            event_sources: Vec::new(),
            local_registry_id: Some("machine-local-uuid".into()),
            governance: None,
            rules: Some(RULES_MD.into()),
            state: Some(STATE_JSON.into()),
            handoff: Some(HANDOFF_MD.into()),
        };
        let value = serde_json::to_value(document).unwrap();
        assert!(value.get("localRegistryId").is_none());
        assert_eq!(value["projectKey"], "portable-project");
    }

    #[test]
    fn m16k_truth_generation_is_transactional_and_stale_completion_is_ignored() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO projects (id,name,status,created_at,updated_at) VALUES ('generation-project','Generation','ACTIVE','now','now')",
                [],
            )
            .unwrap();
        drop(connection);
        let mut connection = database.open_connection().unwrap();
        let tx = connection.transaction().unwrap();
        let generation_one =
            mark_truth_dirty_tx(&tx, "generation-project", "WORKFLOW_TRANSITION").unwrap();
        tx.commit().unwrap();
        assert_eq!(generation_one, 1);
        let attempt_one = match mark_truth_sync_pending(
            &database,
            "generation-project",
            generation_one,
            "rev-one",
            "A",
        )
        .unwrap()
        {
            TruthSyncCasOutcome::Applied { attempt } => attempt,
            outcome => panic!("unexpected pending outcome: {outcome:?}"),
        };
        assert_eq!(
            mark_truth_sync_current(
                &database,
                "generation-project",
                generation_one,
                attempt_one,
                "rev-one",
            )
            .unwrap(),
            TruthSyncCasOutcome::Applied {
                attempt: attempt_one
            }
        );
        let mut connection = database.open_connection().unwrap();
        let tx = connection.transaction().unwrap();
        let generation_two =
            mark_truth_dirty_tx(&tx, "generation-project", "AUDIT_LIFECYCLE").unwrap();
        tx.commit().unwrap();
        assert_eq!(generation_two, 2);
        let attempt_two = match mark_truth_sync_pending(
            &database,
            "generation-project",
            generation_two,
            "rev-two",
            "B",
        )
        .unwrap()
        {
            TruthSyncCasOutcome::Applied { attempt } => attempt,
            outcome => panic!("unexpected pending outcome: {outcome:?}"),
        };
        let _ = mark_truth_sync_degraded(
            &database,
            "generation-project",
            generation_one,
            99,
            "rev-one",
            "A",
            "stale",
        );
        assert_eq!(
            mark_truth_sync_current(
                &database,
                "generation-project",
                generation_one,
                attempt_one,
                "rev-one",
            )
            .unwrap(),
            TruthSyncCasOutcome::Superseded
        );
        let sync = truth_sync_status(&database, "generation-project");
        assert_eq!(sync.generation, 2);
        assert_eq!(sync.materialized_generation, 1);
        assert_eq!(sync.status, "PENDING");
        assert_eq!(
            mark_truth_sync_current(
                &database,
                "generation-project",
                generation_two,
                attempt_two,
                "rev-two",
            )
            .unwrap(),
            TruthSyncCasOutcome::Applied {
                attempt: attempt_two
            }
        );
        let sync = truth_sync_status(&database, "generation-project");
        assert_eq!(sync.materialized_generation, 2);
        assert_eq!(sync.status, "CURRENT");
    }
}
