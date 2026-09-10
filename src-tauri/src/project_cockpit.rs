use crate::command_center::{self, ProjectOperationSummary};
use crate::control_plane::{self, ControlPlaneSnapshot, GitControlPlaneState, ProjectTruth};
use crate::db::DatabaseState;
use crate::git_engine::{
    self, GitDiff, GitDiffRequest, GitDiffScope, GitSnapshot, GitSnapshotRequest,
};
use crate::github_tracking::{self, RemoteTrackingSnapshot};
use crate::project_dashboard::{self, ProjectDashboardResolution};
use crate::projects::{fetch_project, ProjectRecord};
use crate::task_intelligence::TaskIntelligenceSnapshot;
use crate::task_sources::{self, DiscoveredProjectSource};
use crate::workflow::{self, WorkflowEvent, WorkflowProjectList};
use rusqlite::Connection;
use serde::Serialize;
use std::collections::BTreeMap;

const MAX_COCKPIT_TASKS: usize = 128;
const MAX_COCKPIT_HISTORY: usize = 200;
const MAX_COCKPIT_RECORDS: usize = 100;
const MAX_COCKPIT_ACTIVITY: usize = 150;
const MAX_COCKPIT_FILES: usize = 256;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitTestRun {
    pub id: String,
    pub task_id: Option<String>,
    pub command: String,
    pub result: String,
    pub output_metadata: Option<String>,
    pub started_at: String,
    pub finished_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitAuditFinding {
    pub id: String,
    pub severity: String,
    pub title: String,
    pub detail: Option<String>,
    pub file_path: Option<String>,
    pub line_number: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitAudit {
    pub id: String,
    pub task_id: Option<String>,
    pub result: String,
    pub summary: Option<String>,
    pub confidence: Option<f64>,
    pub created_at: String,
    pub findings: Vec<CockpitAuditFinding>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitAgentSession {
    pub id: String,
    pub task_id: Option<String>,
    pub provider: String,
    pub state: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitPermission {
    pub id: String,
    pub session_id: Option<String>,
    pub permission_kind: String,
    pub requested_resource: Option<String>,
    pub state: String,
    pub decided_by: Option<String>,
    pub created_at: String,
    pub decided_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitActivity {
    pub id: String,
    pub kind: String,
    pub event: String,
    pub state: Option<String>,
    pub actor: Option<String>,
    pub occurred_at: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CockpitFileEntry {
    pub path: String,
    pub role: String,
    pub status: String,
    pub source_kind: Option<String>,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectCockpitSnapshot {
    pub project: ProjectRecord,
    pub project_summary: ProjectOperationSummary,
    pub dashboard: ProjectDashboardResolution,
    pub control_plane: ControlPlaneSnapshot,
    pub truth: ProjectTruth,
    pub task_intelligence: Option<TaskIntelligenceSnapshot>,
    pub task_intelligence_error: Option<String>,
    pub workflow: WorkflowProjectList,
    pub workflow_history: Vec<WorkflowEvent>,
    pub git: Option<GitSnapshot>,
    pub git_error: Option<String>,
    pub git_diff: Option<GitDiff>,
    pub git_diff_error: Option<String>,
    pub sources: Vec<DiscoveredProjectSource>,
    pub sources_error: Option<String>,
    pub tests: Vec<CockpitTestRun>,
    pub audits: Vec<CockpitAudit>,
    pub agent_sessions: Vec<CockpitAgentSession>,
    pub permissions: Vec<CockpitPermission>,
    pub activity: Vec<CockpitActivity>,
    pub files: Vec<CockpitFileEntry>,
    pub warnings: Vec<String>,
    pub generated_at: String,
    pub github_tracking: Option<RemoteTrackingSnapshot>,
    pub remote_primary: Option<RemoteCockpitPrimary>,
    pub local_workspace_telemetry: Option<LocalWorkspaceTelemetry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteCockpitPrimary {
    pub repository: String,
    pub branch: String,
    pub remote_head: Option<String>,
    pub remote_health: String,
    pub fetched_at: String,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
    pub current_milestone: Option<String>,
    pub current_sprint: Option<String>,
    pub current_task_id: Option<String>,
    pub current_task_title: Option<String>,
    pub current_task_status: Option<String>,
    pub workflow_state: Option<String>,
    pub required_actor: Option<String>,
    pub next_action: Option<String>,
    pub blockers: Vec<String>,
    pub progress_scope_type: Option<String>,
    pub progress_scope_id: Option<String>,
    pub progress_completed: Option<u64>,
    pub progress_total: Option<u64>,
    pub progress_percent: Option<f64>,
    pub last_completed_task_id: Option<String>,
    pub last_completed_task_title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalWorkspaceTelemetry {
    pub local_path: String,
    pub local_branch: Option<String>,
    pub local_head: Option<String>,
    pub local_health: Option<String>,
    pub warnings: Vec<String>,
    pub git_error: Option<String>,
    pub git_diff_error: Option<String>,
}

pub fn snapshot(
    database: &DatabaseState,
    project_id: &str,
) -> Result<ProjectCockpitSnapshot, String> {
    let project = fetch_project(database, project_id)?;
    if github_tracking::is_github_v3_project(&project) {
        return snapshot_remote_primary(database, project);
    }
    let github_tracking = github_tracking::refresh_project(database, &project).ok();
    let dashboard = project_dashboard::resolve(database, project_id)?;
    let truth = control_plane::ProjectTruthResolver::resolve(database, project_id)?;
    let control_plane = control_plane::snapshot(database, project_id)?;
    let project_summary = match github_tracking.as_ref() {
        Some(remote) => command_center::remote_project_summary(&project, remote),
        None => command_center::summarize_project_for_cockpit(database, &project)?,
    };
    let (task_intelligence, task_intelligence_error) =
        match crate::task_intelligence::list(database, project_id) {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        };
    let workflow = workflow::project_list(
        database,
        workflow::WorkflowProjectListQuery {
            project_id: project_id.to_string(),
            limit: Some(MAX_COCKPIT_TASKS),
        },
    )?;
    let workflow_history = workflow::project_history(database, project_id, MAX_COCKPIT_HISTORY)?;
    let mut warnings = dashboard.warnings.clone();
    warnings.extend(control_plane.warnings.iter().cloned());
    if matches!(
        control_plane.git.last_remote_observation_status.as_str(),
        "DEGRADED" | "FAILED"
    ) {
        push_warning(
            &mut warnings,
            control_plane
                .git
                .last_remote_observation_error
                .as_deref()
                .unwrap_or("remote observation is degraded"),
        );
    }
    let local_warnings = warnings.clone();
    if let Some(remote) = github_tracking.as_ref() {
        warnings.clear();
        if remote.remote_health != "CURRENT" || github_tracking::remote_health(remote) != "HEALTHY"
        {
            warnings.push(remote.error.clone().unwrap_or_else(|| {
                format!(
                    "GitHub remote status is {}",
                    github_tracking::remote_health(remote)
                )
            }));
        }
    }

    let (git, git_error) = match git_engine::snapshot(
        database,
        GitSnapshotRequest {
            project_id: project_id.to_string(),
            persist: Some(false),
        },
    ) {
        Ok(value) => (Some(value), None),
        Err(error) => (None, Some(error)),
    };
    let (git_diff, git_diff_error) = if git.is_some() {
        match git_engine::diff(
            database,
            GitDiffRequest {
                project_id: project_id.to_string(),
                scope: GitDiffScope::WorkingTree,
                base_ref: None,
                head_sha: None,
            },
        ) {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error)),
        }
    } else {
        (None, None)
    };
    let (sources, sources_error) = match task_sources::list(database, project_id) {
        Ok(value) => (value, None),
        Err(error) => (Vec::new(), Some(error)),
    };
    let connection = database.open_connection()?;
    let tests = read_tests(&connection, project_id)?;
    let audits = read_audits(&connection, project_id)?;
    let agent_sessions = read_agent_sessions(&connection, project_id)?;
    let permissions = read_permissions(&connection, project_id)?;
    let activity = build_activity(
        &workflow_history,
        &tests,
        &audits,
        &agent_sessions,
        &dashboard,
    );
    let files = build_files(&sources, &dashboard);
    let local_path = project.original_path.clone();
    let local_branch = git.as_ref().and_then(|value| value.current_branch.clone());
    let local_head = git.as_ref().and_then(|value| value.head_sha.clone());
    let local_health = git.as_ref().map(|value| format!("{:?}", value.health));
    let local_git_error = git_error.clone();
    let local_git_diff_error = git_diff_error.clone();
    for error in [
        task_intelligence_error.as_deref(),
        git_error.as_deref(),
        git_diff_error.as_deref(),
        sources_error.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        push_warning(&mut warnings, error);
    }
    Ok(ProjectCockpitSnapshot {
        project,
        project_summary,
        dashboard,
        control_plane,
        truth,
        task_intelligence,
        task_intelligence_error,
        workflow,
        workflow_history,
        git,
        git_error,
        git_diff,
        git_diff_error,
        sources,
        sources_error,
        tests,
        audits,
        agent_sessions,
        permissions,
        activity,
        files,
        warnings,
        generated_at: crate::time::utc_timestamp(),
        remote_primary: github_tracking.as_ref().map(remote_projection),
        local_workspace_telemetry: if github_tracking.is_some() {
            Some(LocalWorkspaceTelemetry {
                local_path,
                local_branch,
                local_head,
                local_health,
                warnings: local_warnings,
                git_error: local_git_error,
                git_diff_error: local_git_diff_error,
            })
        } else {
            None
        },
        github_tracking,
    })
}

fn snapshot_remote_primary(
    database: &DatabaseState,
    project: ProjectRecord,
) -> Result<ProjectCockpitSnapshot, String> {
    let remote = github_tracking::refresh_project(database, &project)
        .unwrap_or_else(|error| github_tracking::unavailable_for_project(&project, error));
    let mut local_warnings = Vec::new();

    match project_dashboard::resolve(database, &project.id) {
        Ok(value) => local_warnings.extend(value.warnings),
        Err(error) => local_warnings.push(format!("local dashboard: {error}")),
    }
    match control_plane::snapshot(database, &project.id) {
        Ok(value) => local_warnings.extend(value.warnings),
        Err(error) => local_warnings.push(format!("local control plane: {error}")),
    }
    match control_plane::ProjectTruthResolver::resolve(database, &project.id) {
        Ok(value) => local_warnings.extend(value.warnings),
        Err(error) => local_warnings.push(format!("local truth resolver: {error}")),
    }
    let (task_intelligence, task_intelligence_error) =
        match crate::task_intelligence::list(database, &project.id) {
            Ok(value) => (Some(value), None),
            Err(error) => {
                local_warnings.push(format!("local task intelligence: {error}"));
                (None, Some(error))
            }
        };
    let workflow = match workflow::project_list(
        database,
        workflow::WorkflowProjectListQuery {
            project_id: project.id.clone(),
            limit: Some(MAX_COCKPIT_TASKS),
        },
    ) {
        Ok(value) => value,
        Err(error) => {
            local_warnings.push(format!("local workflow: {error}"));
            WorkflowProjectList {
                project_id: project.id.clone(),
                tasks: Vec::new(),
            }
        }
    };
    let workflow_history =
        match workflow::project_history(database, &project.id, MAX_COCKPIT_HISTORY) {
            Ok(value) => value,
            Err(error) => {
                local_warnings.push(format!("local workflow history: {error}"));
                Vec::new()
            }
        };

    let (git, git_error) = match git_engine::snapshot(
        database,
        GitSnapshotRequest {
            project_id: project.id.clone(),
            persist: Some(false),
        },
    ) {
        Ok(value) => (Some(value), None),
        Err(error) => {
            local_warnings.push(format!("local git: {error}"));
            (None, Some(error))
        }
    };
    let (git_diff, git_diff_error) = if git.is_some() {
        match git_engine::diff(
            database,
            GitDiffRequest {
                project_id: project.id.clone(),
                scope: GitDiffScope::WorkingTree,
                base_ref: None,
                head_sha: None,
            },
        ) {
            Ok(value) => (Some(value), None),
            Err(error) => {
                local_warnings.push(format!("local git diff: {error}"));
                (None, Some(error))
            }
        }
    } else {
        (None, None)
    };
    let sources = match task_sources::list(database, &project.id) {
        Ok(value) => value,
        Err(error) => {
            local_warnings.push(format!("local task sources: {error}"));
            Vec::new()
        }
    };

    let (tests, audits, agent_sessions, permissions) = match database.open_connection() {
        Ok(connection) => {
            let tests = match read_tests(&connection, &project.id) {
                Ok(value) => value,
                Err(error) => {
                    local_warnings.push(format!("local test history: {error}"));
                    Vec::new()
                }
            };
            let audits = match read_audits(&connection, &project.id) {
                Ok(value) => value,
                Err(error) => {
                    local_warnings.push(format!("local audit history: {error}"));
                    Vec::new()
                }
            };
            let agent_sessions = match read_agent_sessions(&connection, &project.id) {
                Ok(value) => value,
                Err(error) => {
                    local_warnings.push(format!("local agent history: {error}"));
                    Vec::new()
                }
            };
            let permissions = match read_permissions(&connection, &project.id) {
                Ok(value) => value,
                Err(error) => {
                    local_warnings.push(format!("local permission history: {error}"));
                    Vec::new()
                }
            };
            (tests, audits, agent_sessions, permissions)
        }
        Err(error) => {
            local_warnings.push(format!("local cockpit database: {error}"));
            (Vec::new(), Vec::new(), Vec::new(), Vec::new())
        }
    };

    let local_branch = git.as_ref().and_then(|value| value.current_branch.clone());
    let local_head = git.as_ref().and_then(|value| value.head_sha.clone());
    let local_health = git.as_ref().map(|value| format!("{:?}", value.health));
    let local_git_error = git_error.clone();
    let local_git_diff_error = git_diff_error.clone();
    let dashboard = remote_dashboard(&project, &remote);
    let control_plane = remote_control_plane(&project, &remote);
    let truth = remote_truth(&project, &remote);
    let project_summary = command_center::remote_project_summary(&project, &remote);
    let warnings = remote_warnings(&remote);
    let local_path = project.original_path.clone();

    Ok(ProjectCockpitSnapshot {
        project,
        project_summary,
        dashboard,
        control_plane,
        truth,
        task_intelligence,
        task_intelligence_error,
        workflow,
        workflow_history,
        git,
        git_error,
        git_diff,
        git_diff_error,
        sources,
        sources_error: None,
        tests,
        audits,
        agent_sessions,
        permissions,
        activity: remote_activity(&remote),
        files: remote_files(&remote),
        warnings,
        generated_at: crate::time::utc_timestamp(),
        remote_primary: Some(remote_projection(&remote)),
        local_workspace_telemetry: Some(LocalWorkspaceTelemetry {
            local_path,
            local_branch,
            local_head,
            local_health,
            warnings: local_warnings,
            git_error: local_git_error,
            git_diff_error: local_git_diff_error,
        }),
        github_tracking: Some(remote),
    })
}

fn remote_warnings(remote: &RemoteTrackingSnapshot) -> Vec<String> {
    if remote.remote_health == "CURRENT" && github_tracking::remote_health(remote) == "HEALTHY" {
        Vec::new()
    } else {
        vec![remote.error.clone().unwrap_or_else(|| {
            format!(
                "GitHub remote status is {}",
                github_tracking::remote_health(remote)
            )
        })]
    }
}

fn remote_dashboard(
    project: &ProjectRecord,
    remote: &RemoteTrackingSnapshot,
) -> ProjectDashboardResolution {
    ProjectDashboardResolution {
        project_id: project.id.clone(),
        manifest_status: match remote.remote_health.as_str() {
            "CURRENT" => project_dashboard::ManifestStatus::Valid,
            "STALE" | "STALE_REMOTE_SNAPSHOT" => project_dashboard::ManifestStatus::Stale,
            "ERROR" => project_dashboard::ManifestStatus::Malformed,
            _ => project_dashboard::ManifestStatus::Unavailable,
        },
        manifest_path: "TASKS.md".into(),
        schema: Some("hiveai-task-tracker/root-v1".into()),
        project_key: Some(remote.project_key.clone()),
        repository: Some(remote.repository.clone()),
        branch_policy: Some(remote.branch.clone()),
        dashboard_mode: Some("GITHUB_TASKS_ONLY".into()),
        tracking_mode: Some("GITHUB_TASKS_ONLY".into()),
        refresh_policy: Some("GITHUB_POLL".into()),
        task_authority: project_dashboard::TaskAuthorityState::Canonical,
        canonical_task_source: Some("TASKS.md".into()),
        roles: BTreeMap::new(),
        provenance_mode: "GITHUB_ROOT_TASKS".into(),
        materialized: project_dashboard::MaterializedDashboardStatus {
            project_status: Some("REMOTE".into()),
            health: Some(github_tracking::remote_health(remote).into()),
            current_milestone: remote.current_milestone.clone(),
            current_task_title: remote.current_task_title.clone(),
            current_task_id: remote.current_task_id.clone(),
            declared_workflow_state: remote.workflow_state.clone(),
            progress_raw: remote.progress_percent.map(|value| format!("{value}%")),
            progress_percent: remote.progress_percent.map(|value| value.round() as u32),
            required_actor: remote.required_actor.clone(),
            next_action: remote.next_action.clone(),
            waiting_on: None,
            last_meaningful_update: remote.updated_at.clone(),
            current_work: Vec::new(),
            blockers_waiting: remote.blockers.clone(),
            milestone_summary: Vec::new(),
            quality_verification: Vec::new(),
            recent_meaningful_activity: Vec::new(),
            provenance: vec![project_dashboard::MaterializedFact {
                label: "Source".into(),
                value: format!("GitHub {}@{}", remote.repository, remote.branch),
            }],
        },
        warnings: Vec::new(),
    }
}

fn remote_control_plane(
    project: &ProjectRecord,
    remote: &RemoteTrackingSnapshot,
) -> ControlPlaneSnapshot {
    ControlPlaneSnapshot {
        schema: "github-remote-v3".into(),
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
        truth_sync: Default::default(),
        source_precedence: vec!["GITHUB_TASKS_ONLY".into()],
        warnings: Vec::new(),
    }
}

fn remote_truth(project: &ProjectRecord, remote: &RemoteTrackingSnapshot) -> ProjectTruth {
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

fn remote_activity(remote: &RemoteTrackingSnapshot) -> Vec<CockpitActivity> {
    remote
        .recent_events
        .iter()
        .map(|event| CockpitActivity {
            id: format!("github:{}", event.id),
            kind: event
                .event_type
                .clone()
                .unwrap_or_else(|| "GITHUB_EVENT".into()),
            event: format!(
                "{} -> {}",
                event.from.as_deref().unwrap_or("INITIAL"),
                event.to.as_deref().unwrap_or("UNKNOWN")
            ),
            state: event.to.clone(),
            actor: event.actor.clone(),
            occurred_at: event
                .timestamp
                .clone()
                .unwrap_or_else(|| remote.fetched_at.clone()),
            source: format!("{}@{}", remote.repository, remote.branch),
        })
        .collect()
}

fn remote_files(remote: &RemoteTrackingSnapshot) -> Vec<CockpitFileEntry> {
    ["TASKS.md"]
        .into_iter()
        .map(|path| CockpitFileEntry {
            path: path.into(),
            role: "GITHUB_CANONICAL_TASKS".into(),
            status: "REMOTE".into(),
            source_kind: Some("GITHUB".into()),
            evidence: format!("{}@{}", remote.repository, remote.branch),
        })
        .collect()
}

fn remote_projection(remote: &RemoteTrackingSnapshot) -> RemoteCockpitPrimary {
    RemoteCockpitPrimary {
        repository: remote.repository.clone(),
        branch: remote.branch.clone(),
        remote_head: remote.remote_head.clone(),
        remote_health: github_tracking::remote_health(remote).into(),
        fetched_at: remote.fetched_at.clone(),
        updated_at: remote.updated_at.clone(),
        updated_by: remote.updated_by.clone(),
        current_milestone: remote.current_milestone.clone(),
        current_sprint: remote.current_sprint.clone(),
        current_task_id: remote.current_task_id.clone(),
        current_task_title: remote.current_task_title.clone(),
        current_task_status: remote.current_task_status.clone(),
        workflow_state: remote.workflow_state.clone(),
        required_actor: remote.required_actor.clone(),
        next_action: remote.next_action.clone(),
        blockers: remote.blockers.clone(),
        progress_scope_type: remote.progress_scope_type.clone(),
        progress_scope_id: remote.progress_scope_id.clone(),
        progress_completed: remote.progress_completed,
        progress_total: remote.progress_total,
        progress_percent: remote.progress_percent,
        last_completed_task_id: remote.last_completed_task_id.clone(),
        last_completed_task_title: remote.last_completed_task_title.clone(),
    }
}

fn read_tests(connection: &Connection, project_id: &str) -> Result<Vec<CockpitTestRun>, String> {
    let mut statement = connection.prepare("SELECT id, task_id, command, result, output_metadata_json, started_at, finished_at FROM test_runs WHERE project_id=?1 ORDER BY started_at DESC, id DESC LIMIT ?2").map_err(db_error)?;
    let rows = statement
        .query_map(
            rusqlite::params![project_id, MAX_COCKPIT_RECORDS as i64],
            |row| {
                Ok(CockpitTestRun {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    command: row.get(2)?,
                    result: row.get(3)?,
                    output_metadata: row.get(4)?,
                    started_at: row.get(5)?,
                    finished_at: row.get(6)?,
                })
            },
        )
        .map_err(db_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_error)
}

fn read_audits(connection: &Connection, project_id: &str) -> Result<Vec<CockpitAudit>, String> {
    let mut statement = connection.prepare("SELECT id, task_id, result, summary, confidence, created_at FROM audits WHERE project_id=?1 ORDER BY created_at DESC, id DESC LIMIT ?2").map_err(db_error)?;
    let rows = statement
        .query_map(
            rusqlite::params![project_id, MAX_COCKPIT_RECORDS as i64],
            |row| {
                Ok(CockpitAudit {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    result: row.get(2)?,
                    summary: row.get(3)?,
                    confidence: row.get(4)?,
                    created_at: row.get(5)?,
                    findings: Vec::new(),
                })
            },
        )
        .map_err(db_error)?;
    let mut audits = rows.collect::<Result<Vec<_>, _>>().map_err(db_error)?;
    let mut finding_statement = connection.prepare("SELECT id, audit_id, severity, title, detail, file_path, line_number, created_at FROM audit_findings WHERE audit_id=?1 ORDER BY created_at ASC, id ASC LIMIT ?2").map_err(db_error)?;
    for audit in &mut audits {
        let findings = finding_statement
            .query_map(
                rusqlite::params![audit.id, MAX_COCKPIT_RECORDS as i64],
                |row| {
                    Ok(CockpitAuditFinding {
                        id: row.get(0)?,
                        severity: row.get(2)?,
                        title: row.get(3)?,
                        detail: row.get(4)?,
                        file_path: row.get(5)?,
                        line_number: row.get(6)?,
                        created_at: row.get(7)?,
                    })
                },
            )
            .map_err(db_error)?;
        audit.findings = findings.collect::<Result<Vec<_>, _>>().map_err(db_error)?;
    }
    Ok(audits)
}

fn read_agent_sessions(
    connection: &Connection,
    project_id: &str,
) -> Result<Vec<CockpitAgentSession>, String> {
    let mut statement = connection.prepare("SELECT id, task_id, provider, state, started_at, ended_at, created_at FROM agent_sessions WHERE project_id=?1 ORDER BY COALESCE(started_at, created_at) DESC, id DESC LIMIT ?2").map_err(db_error)?;
    let rows = statement
        .query_map(
            rusqlite::params![project_id, MAX_COCKPIT_RECORDS as i64],
            |row| {
                Ok(CockpitAgentSession {
                    id: row.get(0)?,
                    task_id: row.get(1)?,
                    provider: row.get(2)?,
                    state: row.get(3)?,
                    started_at: row.get(4)?,
                    ended_at: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        )
        .map_err(db_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_error)
}

fn read_permissions(
    connection: &Connection,
    project_id: &str,
) -> Result<Vec<CockpitPermission>, String> {
    let mut statement = connection.prepare("SELECT p.id, p.session_id, p.permission_kind, p.requested_resource, p.state, p.decided_by, p.created_at, p.decided_at FROM permission_requests p LEFT JOIN agent_sessions s ON s.id=p.session_id WHERE s.project_id=?1 ORDER BY p.created_at DESC, p.id DESC LIMIT ?2").map_err(db_error)?;
    let rows = statement
        .query_map(
            rusqlite::params![project_id, MAX_COCKPIT_RECORDS as i64],
            |row| {
                Ok(CockpitPermission {
                    id: row.get(0)?,
                    session_id: row.get(1)?,
                    permission_kind: row.get(2)?,
                    requested_resource: row.get(3)?,
                    state: row.get(4)?,
                    decided_by: row.get(5)?,
                    created_at: row.get(6)?,
                    decided_at: row.get(7)?,
                })
            },
        )
        .map_err(db_error)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_error)
}

fn build_activity(
    workflow_history: &[WorkflowEvent],
    tests: &[CockpitTestRun],
    audits: &[CockpitAudit],
    sessions: &[CockpitAgentSession],
    dashboard: &ProjectDashboardResolution,
) -> Vec<CockpitActivity> {
    let mut activity = Vec::new();
    activity.extend(workflow_history.iter().map(|event| CockpitActivity {
        id: format!("workflow:{}", event.id),
        kind: "WORKFLOW".into(),
        event: event.summary.clone(),
        state: event.to_state.map(|state| state.to_string()),
        actor: event.actor_type.map(|actor| actor.to_string()),
        occurred_at: event.occurred_at.clone(),
        source: "M10 task_events".into(),
    }));
    activity.extend(tests.iter().map(|test| CockpitActivity {
        id: format!("test:{}", test.id),
        kind: "TEST_RUN".into(),
        event: format!("{}: {}", test.command, test.result),
        state: Some(test.result.clone()),
        actor: None,
        occurred_at: test.started_at.clone(),
        source: "test_runs".into(),
    }));
    activity.extend(audits.iter().map(|audit| {
        CockpitActivity {
            id: format!("audit:{}", audit.id),
            kind: "AUDIT".into(),
            event: audit
                .summary
                .clone()
                .unwrap_or_else(|| format!("Audit {}", audit.result)),
            state: Some(audit.result.clone()),
            actor: None,
            occurred_at: audit.created_at.clone(),
            source: "audits".into(),
        }
    }));
    activity.extend(sessions.iter().map(|session| {
        CockpitActivity {
            id: format!("agent:{}", session.id),
            kind: "AGENT".into(),
            event: format!("{} session {}", session.provider, session.state),
            state: Some(session.state.clone()),
            actor: Some(session.provider.clone()),
            occurred_at: session
                .started_at
                .clone()
                .unwrap_or_else(|| session.created_at.clone()),
            source: "agent_sessions".into(),
        }
    }));
    activity.extend(
        dashboard
            .materialized
            .recent_meaningful_activity
            .iter()
            .enumerate()
            .map(|(index, event)| CockpitActivity {
                id: format!("dashboard-activity:{index}"),
                kind: "PROJECT_DASHBOARD".into(),
                event: event.clone(),
                state: None,
                actor: None,
                occurred_at: "UNDATED".into(),
                source: project_dashboard::MANIFEST_RELATIVE_PATH.into(),
            }),
    );
    activity.sort_by(|left, right| {
        let left_undated = left.occurred_at == "UNDATED";
        let right_undated = right.occurred_at == "UNDATED";
        left_undated
            .cmp(&right_undated)
            .then(right.occurred_at.cmp(&left.occurred_at))
            .then(left.id.cmp(&right.id))
    });
    activity.truncate(MAX_COCKPIT_ACTIVITY);
    activity
}

fn build_files(
    sources: &[DiscoveredProjectSource],
    dashboard: &ProjectDashboardResolution,
) -> Vec<CockpitFileEntry> {
    let mut files = Vec::new();
    for source in sources {
        files.push(CockpitFileEntry {
            path: source.relative_path.clone(),
            role: source.authority_class.clone(),
            status: source.status.clone(),
            source_kind: Some(source.source_kind.clone()),
            evidence: format!("M08 source inventory; discovered {}", source.discovered_at),
        });
    }
    for (role, resolved) in &dashboard.roles {
        for source in resolved {
            let status = match source.status {
                project_dashboard::SourceStatus::Available => "AVAILABLE",
                project_dashboard::SourceStatus::Missing => "MISSING",
                project_dashboard::SourceStatus::Rejected => "REJECTED",
            };
            files.push(CockpitFileEntry {
                path: source.path.clone(),
                role: role.clone(),
                status: status.into(),
                source_kind: None,
                evidence: "Project Dashboard authority map".into(),
            });
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path).then(left.role.cmp(&right.role)));
    files.dedup_by(|left, right| left.path == right.path && left.role == right.role);
    files.truncate(MAX_COCKPIT_FILES);
    files
}

fn push_warning(warnings: &mut Vec<String>, message: &str) {
    if !warnings.iter().any(|existing| existing == message) {
        warnings.push(message.to_string());
    }
}

fn db_error(error: rusqlite::Error) -> String {
    format!("project cockpit database error: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use tempfile::tempdir;

    fn seed_workflow_task(database: &DatabaseState, project_id: &str, task_id: &str, title: &str) {
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO tasks (id, project_id, title, state, metadata_json, created_at, updated_at) VALUES (?1, ?2, ?3, 'READY_FOR_IMPLEMENTATION', '{}', '2026-08-27T00:00:00Z', '2026-08-27T00:00:00Z')",
                rusqlite::params![task_id, project_id, title],
            )
            .unwrap();
    }

    fn seed_workflow_event(
        database: &DatabaseState,
        task_id: &str,
        event_id: &str,
        occurred_at: &str,
        summary: &str,
    ) {
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO task_events (id, task_id, event_type, from_state, to_state, actor_type, summary, evidence_json, occurred_at) VALUES (?1, ?2, 'WORKFLOW_TRANSITION', 'PROMPT_READY', 'READY_FOR_IMPLEMENTATION', 'CODEX', ?3, '{\"evidenceRefs\":[]}', ?4)",
                rusqlite::params![event_id, task_id, summary, occurred_at],
            )
            .unwrap();
    }

    fn remote_fixture(health: &str) -> RemoteTrackingSnapshot {
        RemoteTrackingSnapshot {
            project_key: "h-veai".into(),
            display_name: "H-veAI".into(),
            repository: "Sekiph82/H-veAI".into(),
            branch: "main".into(),
            remote_head: Some("remote-head-m16q".into()),
            project_blob_sha: Some("project-blob".into()),
            tasks_blob_sha: Some("tasks-blob".into()),
            rules_blob_sha: Some("rules-blob".into()),
            events_blob_sha: Some("events-blob".into()),
            current_milestone: Some("PAG-M05".into()),
            current_sprint: Some("PAG-M05-C001".into()),
            current_task_id: Some("PAG-M05-001".into()),
            current_task_title: Some("Remote canonical task".into()),
            current_task_status: Some("IN_PROGRESS".into()),
            workflow_state: Some("IN_PROGRESS".into()),
            required_actor: Some("CODEX".into()),
            next_action: Some("Continue from GitHub".into()),
            next_task_id: None,
            next_task_title: None,
            blockers: Vec::new(),
            progress_scope_type: Some("MILESTONE".into()),
            progress_scope_id: Some("PAG-M05".into()),
            progress_completed: Some(2),
            progress_total: Some(8),
            progress_percent: Some(25.0),
            last_completed_task_id: Some("PAG-M04-099".into()),
            last_completed_task_title: Some("Remote completed task".into()),
            updated_at: Some("2026-09-10T12:00:00Z".into()),
            updated_by: Some("CODEX".into()),
            total_tasks: Some(8),
            completed_tasks: Some(2),
            latest_commit_message: Some("Remote commit".into()),
            latest_commit_author: Some("CODEX".into()),
            latest_commit_at: Some("2026-09-10T12:00:00Z".into()),
            fetched_at: "2026-09-10T12:00:00Z".into(),
            remote_health: health.into(),
            error: (health != "CURRENT").then(|| format!("remote health is {health}")),
            recent_events: Vec::new(),
        }
    }

    fn seed_remote_project(
        database: &DatabaseState,
        health: &str,
    ) -> (String, RemoteTrackingSnapshot) {
        crate::github_tracking::ensure_portfolio(database).unwrap();
        let project_id = "github:Sekiph82/H-veAI@main".to_string();
        let project = crate::projects::fetch_project(database, &project_id).unwrap();
        let remote = remote_fixture(health);
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO github_sync_state (id,project_id,resource_kind,resource_cursor,last_synced_at,metadata_json) VALUES (?1,?2,'GITHUB_TRACKING_V3',?3,?4,?5) ON CONFLICT(project_id,resource_kind) DO UPDATE SET resource_cursor=excluded.resource_cursor,last_synced_at=excluded.last_synced_at,metadata_json=excluded.metadata_json",
                rusqlite::params![
                    format!("{}:GITHUB_TRACKING_V3", project.id),
                    project.id,
                    remote.remote_head.clone(),
                    remote.fetched_at.clone(),
                    serde_json::to_string(&remote).unwrap(),
                ],
            )
            .unwrap();
        (project_id, remote)
    }

    #[test]
    fn m16q_remote_primary_survives_broken_local_telemetry_and_firewalls_warnings() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let (project_id, remote) = seed_remote_project(&database, "CURRENT");
        let cockpit = snapshot(&database, &project_id).unwrap();

        assert_eq!(
            cockpit.remote_primary.as_ref().unwrap().current_milestone,
            remote.current_milestone
        );
        assert_eq!(
            cockpit.remote_primary.as_ref().unwrap().current_task_title,
            remote.current_task_title
        );
        assert_eq!(cockpit.project_summary.next_action, remote.next_action);
        assert_eq!(cockpit.project_summary.progress_percent, Some(25));
        assert_eq!(cockpit.project_summary.health, "HEALTHY");
        assert!(cockpit.warnings.is_empty());
        assert!(cockpit
            .local_workspace_telemetry
            .as_ref()
            .unwrap()
            .git_error
            .is_some());
        assert!(cockpit
            .local_workspace_telemetry
            .as_ref()
            .unwrap()
            .warnings
            .iter()
            .any(|warning| warning.contains("local dashboard") || warning.contains("local git")));
    }

    #[test]
    fn m16q_stale_and_invalid_remote_truth_never_promotes_local_values() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let (project_id, remote) = seed_remote_project(&database, "STALE");
        let stale = snapshot(&database, &project_id).unwrap();
        assert_eq!(
            stale.remote_primary.as_ref().unwrap().remote_health,
            "STALE"
        );
        assert_eq!(
            stale.remote_primary.as_ref().unwrap().current_milestone,
            remote.current_milestone
        );
        assert!(stale
            .warnings
            .iter()
            .any(|warning| warning.contains("remote health")));

        let connection = database.open_connection().unwrap();
        let invalid = remote_fixture("ERROR");
        connection
            .execute(
                "UPDATE github_sync_state SET metadata_json=?1, resource_cursor=?2 WHERE project_id=?3 AND resource_kind='GITHUB_TRACKING_V3'",
                rusqlite::params![
                    serde_json::to_string(&invalid).unwrap(),
                    invalid.remote_head.clone(),
                    project_id,
                ],
            )
            .unwrap();
        let invalid_snapshot = snapshot(&database, &project_id).unwrap();
        assert_eq!(
            invalid_snapshot
                .remote_primary
                .as_ref()
                .unwrap()
                .remote_health,
            "ERROR"
        );
        assert_eq!(
            invalid_snapshot.dashboard.materialized.current_milestone,
            invalid.current_milestone
        );
        assert!(invalid_snapshot
            .warnings
            .iter()
            .all(|warning| !warning.contains("local") && !warning.contains("control plane")));
    }

    #[test]
    fn m16q_malformed_local_control_plane_and_stale_dashboard_cannot_contaminate_remote_view() {
        let db_dir = tempdir().unwrap();
        let local_dir = tempdir().unwrap();
        std::fs::create_dir_all(local_dir.path().join(".hiveai")).unwrap();
        std::fs::write(
            local_dir.path().join(".hiveai/PROJECT.json"),
            "{ malformed local control-plane json",
        )
        .unwrap();
        std::fs::write(
            local_dir.path().join(".hiveai/PROJECT_DASHBOARD.md"),
            "Current milestone: PAG-M02\nCurrent task: stale local task\nProgress: 1%\n",
        )
        .unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let (project_id, remote) = seed_remote_project(&database, "CURRENT");
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "UPDATE projects SET local_path=?1, original_path=?1, normalized_path=?1 WHERE id=?2",
                rusqlite::params![local_dir.path().to_string_lossy(), project_id],
            )
            .unwrap();

        let cockpit = snapshot(&database, &project_id).unwrap();
        assert_eq!(
            cockpit.remote_primary.as_ref().unwrap().current_milestone,
            remote.current_milestone
        );
        assert_eq!(
            cockpit.dashboard.materialized.current_milestone,
            Some("PAG-M05".into())
        );
        assert!(cockpit.warnings.is_empty());
        assert!(cockpit
            .local_workspace_telemetry
            .as_ref()
            .unwrap()
            .warnings
            .iter()
            .any(|warning| warning.contains("malformed") || warning.contains("PAG-M02")));
    }

    #[test]
    fn m16q_command_center_and_cockpit_share_remote_primary_semantics() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let (project_id, remote) = seed_remote_project(&database, "CURRENT");
        let cockpit = snapshot(&database, &project_id).unwrap();
        let center = crate::command_center::snapshot(&database).unwrap();
        let summary = center
            .projects
            .iter()
            .find(|project| project.project_id == project_id)
            .unwrap();

        assert_eq!(summary.current_state, remote.workflow_state);
        assert_eq!(summary.next_action, remote.next_action);
        assert_eq!(
            summary.progress_percent,
            cockpit.project_summary.progress_percent
        );
        assert_eq!(summary.health, cockpit.project_summary.health);
        assert_eq!(
            summary.current_task.as_ref().map(|task| &task.title),
            remote.current_task_title.as_ref()
        );
    }

    #[test]
    fn m12_snapshot_is_project_scoped_and_unknowns_are_explicit() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_a_dir = tempdir().unwrap();
        let project_b_dir = tempdir().unwrap();
        let project_a = register_project(
            &database,
            RegisterProjectRequest {
                path: project_a_dir.path().to_string_lossy().into_owned(),
                name: Some("Project A".into()),
            },
        )
        .unwrap();
        let project_b = register_project(
            &database,
            RegisterProjectRequest {
                path: project_b_dir.path().to_string_lossy().into_owned(),
                name: Some("Project B".into()),
            },
        )
        .unwrap();
        let snapshot = snapshot(&database, &project_a.id).unwrap();
        assert_eq!(snapshot.project.id, project_a.id);
        assert_ne!(snapshot.project.id, project_b.id);
        assert_eq!(snapshot.project_summary.project_id, project_a.id);
        assert!(snapshot.git.is_none());
        assert!(snapshot
            .git_error
            .as_deref()
            .unwrap_or_default()
            .contains("NON_GIT_PROJECT"));
        assert!(snapshot
            .activity
            .iter()
            .all(|item| !item.event.contains("Project B")));
    }

    #[test]
    fn m12b_registered_active_project_loads_snapshot_for_exact_registry_id() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_dir = tempdir().unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Native Route Project".into()),
            },
        )
        .unwrap();
        let registered = crate::projects::fetch_project(&database, &project.id).unwrap();
        let cockpit = snapshot(&database, &registered.id).unwrap();
        assert_eq!(registered.status, "ACTIVE");
        assert_eq!(cockpit.project.id, registered.id);
        assert_eq!(cockpit.project_summary.project_id, registered.id);
    }

    #[test]
    fn m12_git_loading_does_not_persist_a_snapshot() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_dir = tempdir().unwrap();
        let output = std::process::Command::new("git")
            .args(["init"])
            .current_dir(project_dir.path())
            .output()
            .unwrap();
        assert!(output.status.success());
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Git Project".into()),
            },
        )
        .unwrap();
        let snapshot = snapshot(&database, &project.id).unwrap();
        assert!(snapshot.git.is_some());
        let connection = database.open_connection().unwrap();
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM git_snapshots", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn m12_project_dashboard_authority_maps_never_cross_projects() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_a_dir = tempdir().unwrap();
        let project_b_dir = tempdir().unwrap();
        for (directory, key, task) in [
            (&project_a_dir, "project-a", "Project A canonical task"),
            (&project_b_dir, "project-b", "Project B canonical task"),
        ] {
            std::fs::create_dir_all(directory.path().join(".hiveai")).unwrap();
            std::fs::write(
                directory.path().join(project_dashboard::MANIFEST_RELATIVE_PATH),
                format!(
                    "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\nprojectKey: {key}\n## Source authorities\n- Canonical task source: `TASKS.md`\n## H!veAI live status\n| Field | Value |\n| --- | --- |\n| Project status | ACTIVE |\n| Current milestone | M12 |\n| Current task | {task} |\n| Current task ID | {key}-task |\n| Current workflow state | IN_PROGRESS |\n| Progress | 25% |\n| Required actor | CODEX |\n| Next action | Continue {key} |\n| Last meaningful update | 2026-08-27 |\n## Recent meaningful activity\n- {key} activity\n"
                ),
            )
            .unwrap();
        }
        let project_a = register_project(
            &database,
            RegisterProjectRequest {
                path: project_a_dir.path().to_string_lossy().into_owned(),
                name: Some("Project A".into()),
            },
        )
        .unwrap();
        let _project_b = register_project(
            &database,
            RegisterProjectRequest {
                path: project_b_dir.path().to_string_lossy().into_owned(),
                name: Some("Project B".into()),
            },
        )
        .unwrap();
        let snapshot = snapshot(&database, &project_a.id).unwrap();
        assert_eq!(snapshot.dashboard.project_key.as_deref(), Some("project-a"));
        assert_eq!(
            snapshot
                .dashboard
                .materialized
                .current_task_title
                .as_deref(),
            Some("Project A canonical task")
        );
        assert!(snapshot
            .dashboard
            .materialized
            .recent_meaningful_activity
            .iter()
            .all(|event| !event.contains("project-b") && !event.contains("Project B")));
        assert!(snapshot
            .files
            .iter()
            .all(|file| !file.evidence.contains("Project B")));
    }

    #[test]
    fn m12_missing_and_archived_states_remain_explicit() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let missing_dir = tempdir().unwrap();
        let missing_project = register_project(
            &database,
            RegisterProjectRequest {
                path: missing_dir.path().to_string_lossy().into_owned(),
                name: Some("Missing Project".into()),
            },
        )
        .unwrap();
        let missing_path = missing_dir.path().to_path_buf();
        drop(missing_dir);
        let missing_snapshot = snapshot(&database, &missing_project.id).unwrap();
        assert_eq!(missing_snapshot.project.status, "MISSING");
        assert!(missing_snapshot.git.is_none());
        assert!(missing_snapshot
            .warnings
            .iter()
            .any(|warning| warning.contains("MISSING") || warning.contains("unavailable")));

        let archived_dir = tempdir().unwrap();
        let archived_project = register_project(
            &database,
            RegisterProjectRequest {
                path: archived_dir.path().to_string_lossy().into_owned(),
                name: Some("Archived Project".into()),
            },
        )
        .unwrap();
        crate::projects::archive_project(&database, &archived_project.id).unwrap();
        let archived_snapshot = snapshot(&database, &archived_project.id).unwrap();
        assert_eq!(archived_snapshot.project.status, "ARCHIVED");
        assert!(archived_snapshot.task_intelligence.is_none());
        assert!(archived_snapshot
            .task_intelligence_error
            .as_deref()
            .unwrap_or_default()
            .contains("not been parsed"));
        assert!(!archived_snapshot
            .warnings
            .iter()
            .any(|warning| warning.contains(missing_path.to_string_lossy().as_ref())));
    }

    #[test]
    fn m12_project_history_prevents_cross_task_starvation_and_orders_globally() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_dir = tempdir().unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("History Project".into()),
            },
        )
        .unwrap();
        seed_workflow_task(&database, &project.id, "task-a", "Older task A");
        seed_workflow_task(&database, &project.id, "task-b", "Newer task B");
        for index in 0..(MAX_COCKPIT_HISTORY + 5) {
            seed_workflow_event(
                &database,
                "task-a",
                &format!("a-event-{index:03}"),
                &format!("2026-01-01T00:{:02}:00Z", index % 60),
                "old task A event",
            );
        }
        seed_workflow_event(
            &database,
            "task-b",
            "b-new-event",
            "2026-08-27T12:00:00Z",
            "new task B event",
        );
        let snapshot = snapshot(&database, &project.id).unwrap();
        assert_eq!(snapshot.workflow_history.len(), MAX_COCKPIT_HISTORY);
        assert!(snapshot
            .workflow_history
            .iter()
            .any(|event| event.task_id == "task-b" && event.id == "b-new-event"));
        assert!(snapshot.workflow_history.windows(2).all(|events| {
            (events[0].occurred_at.as_str(), events[0].id.as_str())
                >= (events[1].occurred_at.as_str(), events[1].id.as_str())
        }));
        assert!(snapshot
            .activity
            .iter()
            .any(|event| event.id == "workflow:b-new-event"));
    }

    #[test]
    fn m12_project_history_tie_order_is_deterministic() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_dir = tempdir().unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Tie Project".into()),
            },
        )
        .unwrap();
        seed_workflow_task(&database, &project.id, "task-a", "Task A");
        seed_workflow_task(&database, &project.id, "task-b", "Task B");
        seed_workflow_event(
            &database,
            "task-a",
            "tie-a",
            "2026-08-27T12:00:00Z",
            "tie A",
        );
        seed_workflow_event(
            &database,
            "task-b",
            "tie-b",
            "2026-08-27T12:00:00Z",
            "tie B",
        );
        let first = snapshot(&database, &project.id).unwrap();
        let second = snapshot(&database, &project.id).unwrap();
        let first_ids: Vec<_> = first
            .workflow_history
            .iter()
            .map(|event| event.id.as_str())
            .collect();
        let second_ids: Vec<_> = second
            .workflow_history
            .iter()
            .map(|event| event.id.as_str())
            .collect();
        assert_eq!(first_ids, second_ids);
        assert_eq!(first_ids[0], "tie-b");
    }

    #[test]
    fn m12_project_history_and_activity_exclude_other_project_events() {
        let db_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project_a_dir = tempdir().unwrap();
        let project_b_dir = tempdir().unwrap();
        let project_a = register_project(
            &database,
            RegisterProjectRequest {
                path: project_a_dir.path().to_string_lossy().into_owned(),
                name: Some("Project A".into()),
            },
        )
        .unwrap();
        let project_b = register_project(
            &database,
            RegisterProjectRequest {
                path: project_b_dir.path().to_string_lossy().into_owned(),
                name: Some("Project B".into()),
            },
        )
        .unwrap();
        seed_workflow_task(&database, &project_a.id, "task-a", "A task");
        seed_workflow_task(&database, &project_b.id, "task-b", "B task");
        seed_workflow_event(
            &database,
            "task-a",
            "a-event",
            "2026-08-27T10:00:00Z",
            "A event",
        );
        seed_workflow_event(
            &database,
            "task-b",
            "b-newer-event",
            "2026-08-27T12:00:00Z",
            "B event",
        );
        let snapshot = snapshot(&database, &project_a.id).unwrap();
        assert!(snapshot
            .workflow_history
            .iter()
            .all(|event| event.task_id != "task-b" && event.id != "b-newer-event"));
        assert!(snapshot
            .activity
            .iter()
            .all(|event| !event.event.contains("B event") && event.id != "workflow:b-newer-event"));
    }
}
