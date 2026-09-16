use crate::control_plane;
use crate::db::DatabaseState;
use crate::github_tracking::{self, RemoteTrackingSnapshot};
use crate::next_best_task;
use crate::project_dashboard::{self, ProjectDashboardResolution, TaskAuthorityState};
use crate::projects::{list_projects, ProjectListQuery, ProjectRecord};
use crate::task_intelligence::{self, ParsedTask, TaskIntelligenceSnapshot};
use crate::watcher::{read_task_refresh_health, TaskRefreshHealth};
use crate::workflow::{self, WorkflowState, WorkflowTask};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

pub const MAX_VISIBLE_PROJECTS: usize = 128;
pub const DEFAULT_ACTIVITY_LIMIT: usize = 50;
pub const MAX_ACTIVITY_LIMIT: usize = 200;
pub const MAX_ATTENTION_ITEMS: usize = 100;
pub const MAX_QUEUE_ITEMS: usize = 100;
pub const MAX_PROJECT_WARNINGS: usize = 64;
pub const MAX_PORTFOLIO_WARNINGS: usize = 256;
pub const MAX_WARNING_SCALAR_BYTES: usize = 1024;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandCenterSnapshot {
    pub generated_at: String,
    pub projects: Vec<ProjectOperationSummary>,
    pub kpis: PortfolioKpis,
    pub attention: Vec<AttentionItem>,
    pub work_queue: Vec<WorkQueueItem>,
    pub recent_activity: Vec<ActivityItem>,
    pub engineering_brief: EngineeringBrief,
    #[serde(default)]
    pub m19: Option<next_best_task::M19Snapshot>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioKpis {
    pub projects: usize,
    pub active_tasks: Option<usize>,
    pub needs_attention: Option<usize>,
    pub running: Option<usize>,
    pub completed_tasks: Option<usize>,
    pub healthy: usize,
    pub health_detail: String,
    pub authority_detail: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOperationSummary {
    pub project_id: String,
    pub name: String,
    pub registry_status: String,
    pub health: String,
    pub manifest_status: String,
    pub tracking_mode: Option<String>,
    pub task_authority: String,
    pub provenance_mode: String,
    pub materialized: project_dashboard::MaterializedDashboardStatus,
    pub canonical_task_source: Option<String>,
    pub current_task: Option<TaskSummary>,
    pub current_state: Option<String>,
    pub current_milestone: Option<String>,
    pub last_action: Option<ActionSummary>,
    pub next_action: Option<String>,
    pub required_actor: Option<String>,
    pub blockers: Vec<String>,
    pub allowed_actors: Vec<String>,
    pub total_tasks: Option<usize>,
    pub active_tasks: Option<usize>,
    pub completed_tasks: Option<usize>,
    pub progress_percent: Option<u8>,
    pub progress_scope: Option<String>,
    pub authority_source: String,
    pub provenance: Vec<String>,
    pub reconciliation_state: String,
    pub warnings: Vec<String>,
    pub refresh_status: Option<String>,
    pub refresh_at: Option<String>,
    pub refresh_error: Option<String>,
    pub control_plane: Option<control_plane::ControlPlaneSummary>,
    pub truth_sync: control_plane::TruthSyncState,
    pub github_tracking: Option<RemoteTrackingSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSummary {
    pub task_id: String,
    pub title: String,
    pub source_path: String,
    pub parsed_status: String,
    pub workflow_state: Option<String>,
    pub required_actor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionSummary {
    pub summary: String,
    pub occurred_at: String,
    pub actor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AttentionItem {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub task_id: Option<String>,
    pub title: String,
    pub state: String,
    pub detail: String,
    pub category: String,
    operational_identity: Option<AttentionIdentity>,
}

impl Serialize for AttentionItem {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut output = serializer.serialize_struct("AttentionItem", 9)?;
        output.serialize_field("id", &self.id)?;
        output.serialize_field("projectId", &self.project_id)?;
        output.serialize_field("projectName", &self.project_name)?;
        output.serialize_field("taskId", &self.task_id)?;
        output.serialize_field("title", &self.title)?;
        output.serialize_field("state", &self.state)?;
        output.serialize_field("detail", &self.detail)?;
        output.serialize_field("category", &self.category)?;
        let issue_key = format!(
            "{}:{}:{}",
            self.project_id,
            self.task_id.as_deref().unwrap_or("project"),
            normalize_operational_identity(
                self.operational_identity
                    .as_ref()
                    .map(|identity| identity.source.as_str())
                    .unwrap_or(&self.detail)
            )
        );
        output.serialize_field("issueKey", &issue_key)?;
        output.end()
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkQueueItem {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub task_id: String,
    pub task: String,
    pub stage: String,
    pub state: String,
    pub actor: Option<String>,
    pub updated_at: Option<String>,
    pub attention: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityItem {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub kind: String,
    pub event: String,
    pub state: Option<String>,
    pub actor: Option<String>,
    pub occurred_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BriefFact {
    pub label: String,
    pub value: String,
    pub source: String,
    pub provenance: BriefProvenance,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BriefProvenance {
    pub source_class: String,
    pub project_id: Option<String>,
    pub source_path: Option<String>,
    pub evidence_type: Option<String>,
    pub evidence_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineeringBrief {
    pub facts: Vec<BriefFact>,
    pub recommendation: Option<String>,
    #[serde(default)]
    pub m19: Option<next_best_task::M19Snapshot>,
    #[serde(default)]
    pub changes_since_last: Option<next_best_task::M19Comparison>,
    #[serde(default)]
    pub attention: Vec<next_best_task::M19Attention>,
    #[serde(default)]
    pub unavailable_inputs: Vec<String>,
    #[serde(default)]
    pub actor_readiness: Vec<next_best_task::M19ActorReadiness>,
}

pub fn snapshot(database: &DatabaseState) -> Result<CommandCenterSnapshot, String> {
    let mut projects = list_projects(
        database,
        ProjectListQuery {
            include_archived: Some(false),
            ..Default::default()
        },
    )?;
    projects.sort_by(|left, right| {
        left.name
            .to_ascii_lowercase()
            .cmp(&right.name.to_ascii_lowercase())
            .then(left.id.cmp(&right.id))
    });
    let mut warnings = Vec::new();
    if projects.len() > MAX_VISIBLE_PROJECTS {
        push_warning(
            &mut warnings,
            format!("visible project limit reached ({MAX_VISIBLE_PROJECTS})"),
        );
        projects.truncate(MAX_VISIBLE_PROJECTS);
    }
    let mut summaries = Vec::new();
    let mut attention = Vec::new();
    let mut queue = Vec::new();
    let mut activity = Vec::new();
    let mut has_legacy_projects = false;
    for project in projects {
        if github_tracking::is_github_tasks_project(&project) {
            match github_tracking::refresh_project(database, &project) {
                Ok(remote) => {
                    let summary = remote_project_summary(&project, &remote);
                    if let Some(item) = remote_attention(&project, &summary, &remote) {
                        attention.push(item);
                    }
                    if let Some(item) = remote_queue(&project, &remote) {
                        queue.push(item);
                    }
                    activity.extend(remote_activity(&project, &remote));
                    if let Some(error) = remote.error.clone() {
                        push_portfolio_warning(&mut warnings, format!("{}: {error}", project.name));
                    }
                    summaries.push(summary);
                }
                Err(error) => {
                    let remote = github_tracking::unavailable_for_project(&project, error.clone());
                    let summary = remote_project_summary(&project, &remote);
                    if let Some(item) = remote_attention(&project, &summary, &remote) {
                        attention.push(item);
                    }
                    push_portfolio_warning(&mut warnings, format!("{}: {error}", project.name));
                    summaries.push(summary);
                }
            }
        } else {
            has_legacy_projects = true;
            let (summary, project_attention, project_queue, mut project_warnings) =
                match summarize_project(database, &project) {
                    Ok(value) => value,
                    Err(error) => {
                        let detail = format!("Project reconciliation degraded: {error}");
                        push_portfolio_warning(&mut warnings, detail.clone());
                        (
                            degraded_project_summary(&project, &detail),
                            vec![AttentionItem {
                                id: format!("project-degraded:{}", project.id),
                                project_id: project.id.clone(),
                                project_name: project.name.clone(),
                                task_id: None,
                                title: "Project requires reconciliation".into(),
                                state: "NEEDS_ATTENTION".into(),
                                detail,
                                category: "ROOT_TASKS".into(),
                                operational_identity: None,
                            }],
                            Vec::new(),
                            Vec::new(),
                        )
                    }
                };
            attention.extend(project_attention);
            queue.extend(project_queue);
            for warning in project_warnings.drain(..) {
                push_portfolio_warning(&mut warnings, warning);
            }
            summaries.push(summary);
        }
    }
    if has_legacy_projects {
        activity = read_activity(database, DEFAULT_ACTIVITY_LIMIT)?;
    }
    deduplicate_materialized_attention(&mut attention);
    deduplicate_materialized_queue(&mut queue);
    if has_legacy_projects {
        append_materialized_activity(&summaries, &mut activity);
    }
    attention.sort_by(|left, right| left.id.cmp(&right.id));
    attention.truncate(MAX_ATTENTION_ITEMS);
    queue.sort_by(|left, right| {
        right
            .updated_at
            .cmp(&left.updated_at)
            .then(left.id.cmp(&right.id))
    });
    queue.truncate(MAX_QUEUE_ITEMS);
    activity.truncate(MAX_ACTIVITY_LIMIT);
    let active_tasks = summaries
        .iter()
        .filter_map(|summary| summary.active_tasks)
        .sum::<usize>();
    let completed_tasks = summaries
        .iter()
        .filter_map(|summary| summary.completed_tasks)
        .sum::<usize>();
    let canonical_projects = summaries
        .iter()
        .filter(|summary| {
            summary.task_authority == "GITHUB_TASKS_ONLY" || summary.task_authority == "CANONICAL"
        })
        .count();
    let fallback_projects = summaries
        .iter()
        .filter(|summary| summary.task_authority == "FALLBACK_M08_M09")
        .count();
    let not_canonicalized_projects = summaries
        .iter()
        .filter(|summary| summary.task_authority == "NOT_CANONICALIZED")
        .count();
    let healthy = summaries
        .iter()
        .filter(|summary| summary.health == "HEALTHY")
        .count();
    let active_tasks_known = summaries
        .iter()
        .all(|summary| summary.active_tasks.is_some());
    let completed_tasks_known = summaries
        .iter()
        .all(|summary| summary.completed_tasks.is_some());
    let workflow_known = summaries.iter().all(|summary| {
        matches!(
            summary.task_authority.as_str(),
            "GITHUB_TASKS_ONLY" | "CANONICAL" | "ROOT_TASKS_UNAVAILABLE"
        )
    });
    let authority_detail = if summaries
        .iter()
        .all(|summary| summary.task_authority == "GITHUB_TASKS_ONLY")
    {
        "GitHub + root TASKS.md".into()
    } else {
        format!(
            "{canonical_projects} canonical, {fallback_projects} fallback, {not_canonicalized_projects} not canonicalized"
        )
    };
    let health_detail = format!("{healthy} / {} healthy", summaries.len());
    let brief_source_class = if summaries
        .iter()
        .all(|summary| summary.task_authority == "GITHUB_TASKS_ONLY")
    {
        "GITHUB_TASKS_ONLY"
    } else if summaries
        .iter()
        .all(|summary| summary.task_authority == "CANONICAL")
    {
        "ROOT_TASKS"
    } else {
        "TASKS_AUTHORITY"
    };
    let brief_source = if brief_source_class == "GITHUB_TASKS_ONLY" {
        "GitHub tracked-branch root TASKS.md"
    } else {
        "Repository-root TASKS.md"
    };
    let mut facts = vec![
        BriefFact {
            label: "Registered projects".into(),
            value: summaries.len().to_string(),
            source: "Project Registry".into(),
            provenance: BriefProvenance {
                source_class: "REGISTRY".into(),
                project_id: None,
                source_path: None,
                evidence_type: Some("PROJECTS".into()),
                evidence_id: None,
            },
        },
        BriefFact {
            label: "Authoritative active tasks".into(),
            value: if active_tasks_known {
                active_tasks.to_string()
            } else {
                "Unavailable".into()
            },
            source: brief_source.into(),
            provenance: BriefProvenance {
                source_class: brief_source_class.into(),
                project_id: summaries.first().map(|summary| summary.project_id.clone()),
                source_path: summaries
                    .first()
                    .and_then(|summary| summary.canonical_task_source.clone()),
                evidence_type: Some("TASK_INTELLIGENCE_SNAPSHOT".into()),
                evidence_id: None,
            },
        },
        BriefFact {
            label: "Needs attention".into(),
            value: if workflow_known {
                attention.len().to_string()
            } else {
                "Unavailable".into()
            },
            source: brief_source.into(),
            provenance: BriefProvenance {
                source_class: brief_source_class.into(),
                project_id: attention.first().map(|item| item.project_id.clone()),
                source_path: None,
                evidence_type: attention.first().map(|item| item.category.clone()),
                evidence_id: attention.first().map(|item| item.id.clone()),
            },
        },
        BriefFact {
            label: "Running workflow tasks".into(),
            value: if workflow_known {
                queue
                    .iter()
                    .filter(|item| is_running_state(&item.state))
                    .count()
                    .to_string()
            } else {
                "Unavailable".into()
            },
            source: brief_source.into(),
            provenance: BriefProvenance {
                source_class: brief_source_class.into(),
                project_id: queue.first().map(|item| item.project_id.clone()),
                source_path: None,
                evidence_type: queue.first().map(|item| item.stage.clone()),
                evidence_id: queue.first().map(|item| item.id.clone()),
            },
        },
    ];
    for summary in &summaries {
        if summary.task_authority == "GITHUB_TASKS_ONLY" {
            continue;
        }
        if !is_single_dashboard_summary(summary) {
            continue;
        }
        for fact in summary.materialized.quality_verification.iter().take(10) {
            facts.push(BriefFact {
                label: format!("{} quality: {}", summary.name, fact.label),
                value: fact.value.clone(),
                source: "Project Dashboard".into(),
                provenance: BriefProvenance {
                    source_class: "PROJECT_DASHBOARD".into(),
                    project_id: Some(summary.project_id.clone()),
                    source_path: Some(project_dashboard::MANIFEST_RELATIVE_PATH.into()),
                    evidence_type: Some("QUALITY_VERIFICATION".into()),
                    evidence_id: Some(fact.label.clone()),
                },
            });
        }
    }
    let m19 = match next_best_task::snapshot(database) {
        Ok(value) => Some(value),
        Err(error) => {
            push_portfolio_warning(
                &mut warnings,
                format!("M19 Engineering Brief unavailable: {error}"),
            );
            None
        }
    };
    if let Some(m19_snapshot) = &m19 {
        for fact in m19_snapshot.facts.iter().take(24) {
            facts.push(BriefFact {
                label: format!("M19 — {}", fact.label),
                value: fact.value.clone(),
                source: fact.source.clone(),
                provenance: BriefProvenance {
                    source_class: "M19_ENGINEERING_BRIEF".into(),
                    project_id: None,
                    source_path: Some("TASKS.md / Registry / native M19 evidence".into()),
                    evidence_type: Some(fact.freshness.clone()),
                    evidence_id: Some(fact.label.clone()),
                },
            });
        }
    }
    facts.truncate(32);
    let project_count = summaries.len();
    Ok(CommandCenterSnapshot {
        generated_at: crate::time::utc_timestamp(),
        projects: summaries,
        kpis: PortfolioKpis {
            projects: project_count,
            active_tasks: active_tasks_known.then_some(active_tasks),
            needs_attention: workflow_known.then_some(attention.len()),
            running: workflow_known.then_some(
                queue
                    .iter()
                    .filter(|item| is_running_state(&item.state))
                    .count(),
            ),
            completed_tasks: completed_tasks_known.then_some(completed_tasks),
            healthy,
            health_detail,
            authority_detail,
        },
        attention,
        work_queue: queue,
        recent_activity: activity,
        engineering_brief: EngineeringBrief {
            facts,
            recommendation: m19
                .as_ref()
                .and_then(|value| value.recommended.as_ref())
                .map(|value| {
                    format!(
                        "Rank 1: {} / {} (score {})",
                        value.project_name, value.task_title, value.score
                    )
                }),
            changes_since_last: m19.as_ref().map(|value| value.comparison.clone()),
            attention: m19
                .as_ref()
                .map(|value| value.attention.clone())
                .unwrap_or_default(),
            unavailable_inputs: m19
                .as_ref()
                .map(|value| value.unavailable_inputs.clone())
                .unwrap_or_default(),
            actor_readiness: m19
                .as_ref()
                .map(|value| value.actor_readiness.clone())
                .unwrap_or_default(),
            m19: m19.clone(),
        },
        m19,
        warnings,
    })
}

fn degraded_project_summary(project: &ProjectRecord, detail: &str) -> ProjectOperationSummary {
    ProjectOperationSummary {
        project_id: project.id.clone(),
        name: project.name.clone(),
        registry_status: project.status.clone(),
        health: "NEEDS_ATTENTION".into(),
        manifest_status: "UNAVAILABLE".into(),
        tracking_mode: Some("ROOT_TASKS_ONLY".into()),
        task_authority: "ROOT_TASKS_UNAVAILABLE".into(),
        provenance_mode: "ROOT_TASKS_UNAVAILABLE".into(),
        materialized: project_dashboard::MaterializedDashboardStatus::default(),
        canonical_task_source: Some("TASKS.md".into()),
        current_task: None,
        current_state: None,
        current_milestone: None,
        last_action: None,
        next_action: Some("Restore a readable repository-root TASKS.md".into()),
        required_actor: None,
        blockers: Vec::new(),
        allowed_actors: Vec::new(),
        total_tasks: None,
        active_tasks: None,
        completed_tasks: None,
        progress_percent: None,
        progress_scope: None,
        authority_source: "ROOT_TASKS_UNAVAILABLE".into(),
        provenance: Vec::new(),
        reconciliation_state: "NEEDS_RECONCILIATION".into(),
        warnings: vec![detail.into()],
        refresh_status: None,
        refresh_at: None,
        refresh_error: Some(detail.into()),
        control_plane: None,
        truth_sync: control_plane::TruthSyncState::default(),
        github_tracking: None,
    }
}

pub(crate) fn remote_project_summary(
    project: &ProjectRecord,
    remote: &RemoteTrackingSnapshot,
) -> ProjectOperationSummary {
    let mut summary = ProjectOperationSummary {
        project_id: project.id.clone(),
        name: remote.display_name.clone(),
        registry_status: "GITHUB_REMOTE".into(),
        health: "UNAVAILABLE".into(),
        manifest_status: "UNAVAILABLE".into(),
        tracking_mode: Some("github-poll".into()),
        task_authority: "GITHUB_TASKS_ONLY".into(),
        provenance_mode: "GITHUB_REMOTE".into(),
        materialized: project_dashboard::MaterializedDashboardStatus::default(),
        canonical_task_source: Some("TASKS.md".into()),
        current_task: None,
        current_state: None,
        current_milestone: None,
        last_action: None,
        next_action: None,
        required_actor: None,
        blockers: Vec::new(),
        allowed_actors: Vec::new(),
        total_tasks: None,
        active_tasks: None,
        completed_tasks: None,
        progress_percent: None,
        progress_scope: None,
        authority_source: String::new(),
        provenance: Vec::new(),
        reconciliation_state: "REMOTE_PRIMARY".into(),
        warnings: Vec::new(),
        refresh_status: None,
        refresh_at: None,
        refresh_error: None,
        control_plane: None,
        truth_sync: control_plane::TruthSyncState::default(),
        github_tracking: None,
    };
    apply_remote_tracking(&mut summary, remote);
    summary
}

fn apply_remote_tracking(summary: &mut ProjectOperationSummary, remote: &RemoteTrackingSnapshot) {
    summary.github_tracking = Some(remote.clone());
    summary.registry_status = "GITHUB_REMOTE".into();
    summary.tracking_mode = Some("github-poll".into());
    summary.task_authority = "GITHUB_TASKS_ONLY".into();
    summary.provenance_mode = "GITHUB_ROOT_TASKS".into();
    summary.manifest_status = match remote.remote_health.as_str() {
        "CURRENT" => "VALID",
        "STALE" | "STALE_REMOTE_SNAPSHOT" => "STALE",
        "ERROR" => "ERROR",
        _ => "UNAVAILABLE",
    }
    .into();
    summary.health = github_tracking::remote_health(remote).into();
    summary.current_state = remote
        .current_task_status
        .clone()
        .or_else(|| remote.workflow_state.clone());
    summary.current_milestone = remote.current_milestone.clone();
    summary.next_action = remote.next_action.clone();
    summary.required_actor = remote.required_actor.clone();
    summary.blockers = remote.blockers.clone();
    summary.allowed_actors = remote.required_actor.clone().into_iter().collect();
    summary.progress_percent = remote.progress_percent.map(|value| value.round() as u8);
    summary.progress_scope = remote.progress_scope_id.clone();
    summary.total_tasks = remote.progress_total.map(|value| value as usize);
    summary.completed_tasks = remote.progress_completed.map(|value| value as usize);
    summary.active_tasks = match (summary.total_tasks, summary.completed_tasks) {
        (Some(total), Some(done)) => Some(total.saturating_sub(done)),
        _ => None,
    };
    summary.current_task = remote
        .current_task_id
        .clone()
        .zip(remote.current_task_title.clone())
        .map(|(task_id, title)| TaskSummary {
            task_id,
            title,
            source_path: "TASKS.md".into(),
            parsed_status: remote
                .current_task_status
                .clone()
                .or_else(|| remote.workflow_state.clone())
                .unwrap_or_else(|| "REMOTE".into()),
            workflow_state: remote.workflow_state.clone(),
            required_actor: remote.required_actor.clone(),
        });
    summary.authority_source = format!(
        "GitHub remote {}/{}@{}",
        remote.repository,
        remote.branch,
        remote.remote_head.as_deref().unwrap_or("unknown")
    );
    summary.provenance = vec![
        "GitHub branch HEAD".into(),
        "TASKS.md".into(),
        format!(
            "https://github.com/{}/commits/{}",
            remote.repository, remote.branch
        ),
    ];
    summary.refresh_status = Some(remote.remote_health.clone());
    summary.refresh_at = Some(remote.fetched_at.clone());
    summary.refresh_error = remote.error.clone();
    summary.reconciliation_state = "REMOTE_PRIMARY".into();
    summary.warnings.clear();
    if remote.remote_health != "CURRENT" || summary.health != "HEALTHY" {
        push_warning(
            &mut summary.warnings,
            remote
                .error
                .clone()
                .unwrap_or_else(|| format!("GitHub remote status is {}", summary.health)),
        );
    }
}

fn remote_attention(
    project: &ProjectRecord,
    summary: &ProjectOperationSummary,
    remote: &RemoteTrackingSnapshot,
) -> Option<AttentionItem> {
    if summary.health == "HEALTHY" {
        return None;
    }
    let title = remote
        .current_task_title
        .clone()
        .unwrap_or_else(|| "GitHub project requires attention".into());
    let detail = if !remote.blockers.is_empty() {
        remote.blockers.join("; ")
    } else {
        format!(
            "Remote workflow {} / health {}",
            remote.workflow_state.as_deref().unwrap_or("UNKNOWN"),
            summary.health
        )
    };
    Some(AttentionItem {
        id: format!(
            "github-remote:{}:{}",
            project.id,
            remote.remote_head.as_deref().unwrap_or("unknown")
        ),
        project_id: project.id.clone(),
        project_name: project.name.clone(),
        task_id: remote.current_task_id.clone(),
        title,
        state: summary.health.clone(),
        detail,
        category: "GITHUB_REMOTE".into(),
        operational_identity: None,
    })
}

fn remote_queue(project: &ProjectRecord, remote: &RemoteTrackingSnapshot) -> Option<WorkQueueItem> {
    let task_id = remote.current_task_id.clone().or_else(|| {
        remote
            .next_action
            .as_ref()
            .map(|_| format!("remote:{}", remote.project_key))
    })?;
    let task = remote
        .current_task_title
        .clone()
        .or_else(|| remote.next_action.clone())
        .unwrap_or_else(|| "Current remote task unavailable".into());
    let stage = remote
        .current_milestone
        .clone()
        .or_else(|| remote.current_sprint.clone())
        .unwrap_or_else(|| "GitHub remote".into());
    Some(WorkQueueItem {
        id: format!("github-queue:{}:{}", project.id, task_id),
        project_id: project.id.clone(),
        project_name: project.name.clone(),
        task_id,
        task,
        stage,
        state: remote
            .current_task_status
            .clone()
            .or_else(|| remote.workflow_state.clone())
            .unwrap_or_else(|| "UNKNOWN".into()),
        actor: remote.required_actor.clone(),
        updated_at: remote
            .updated_at
            .clone()
            .or_else(|| Some(remote.fetched_at.clone())),
        attention: github_tracking::remote_health(remote) != "HEALTHY",
    })
}

fn remote_activity(project: &ProjectRecord, remote: &RemoteTrackingSnapshot) -> Vec<ActivityItem> {
    remote
        .recent_events
        .iter()
        .map(|event| ActivityItem {
            id: format!("github-event:{}:{}", project.id, event.id),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            kind: "GITHUB_REMOTE_EVENT".into(),
            event: event
                .event_type
                .clone()
                .unwrap_or_else(|| "Remote tracker event".into()),
            state: event.to.clone(),
            actor: event.actor.clone(),
            occurred_at: event
                .timestamp
                .clone()
                .unwrap_or_else(|| remote.fetched_at.clone()),
        })
        .collect()
}

pub fn resolve_project(
    database: &DatabaseState,
    project_id: &str,
) -> Result<ProjectDashboardResolution, String> {
    project_dashboard::resolve(database, project_id)
}

fn summarize_project(
    database: &DatabaseState,
    project: &ProjectRecord,
) -> Result<
    (
        ProjectOperationSummary,
        Vec<AttentionItem>,
        Vec<WorkQueueItem>,
        Vec<String>,
    ),
    String,
> {
    let dashboard = project_dashboard::resolve(database, &project.id)?;
    let truth = control_plane::ProjectTruthResolver::resolve(database, &project.id)?;
    let root_tasks_only = dashboard.provenance_mode.starts_with("ROOT_TASKS");
    let control_plane_snapshot = (!root_tasks_only)
        .then(|| control_plane::snapshot(database, &project.id).ok())
        .flatten();
    let mut warnings = dashboard.warnings.clone();
    warnings.extend(truth.warnings.iter().cloned());
    if let Some(control_plane) = control_plane_snapshot.as_ref() {
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
    }
    let intelligence = task_intelligence::list(database, &project.id).ok();
    if intelligence.is_none() && project.status == "ACTIVE" {
        push_warning(
            &mut warnings,
            "M09 task intelligence has not been parsed for this project",
        );
    }
    let workflow_result = workflow::project_list(
        database,
        workflow::WorkflowProjectListQuery {
            project_id: project.id.clone(),
            limit: Some(workflow::MAX_HISTORY_LIMIT),
        },
    );
    let workflows_available = workflow_result.is_ok();
    let workflows = workflow_result.map(|value| value.tasks).unwrap_or_default();
    if !workflows_available {
        push_warning(
            &mut warnings,
            "M10 workflow evidence unavailable; workflow-derived state is unknown",
        );
    }
    let tasks = authoritative_tasks(&dashboard, intelligence.as_ref());
    let task_ids = tasks
        .iter()
        .map(|task| task.id.as_str())
        .collect::<HashSet<_>>();
    let workflow_tasks = workflows
        .into_iter()
        .filter(|task| {
            task_ids.contains(task.task_id.as_str()) && task.source_active && task.workflow_managed
        })
        .collect::<Vec<_>>();
    let completed = tasks
        .iter()
        .filter(|task| {
            task_is_complete(
                task,
                (!root_tasks_only)
                    .then(|| {
                        workflow_tasks
                            .iter()
                            .find(|workflow| workflow.task_id == task.id)
                    })
                    .flatten(),
            )
        })
        .count();
    let active = tasks.len().saturating_sub(completed);
    let task_authority = task_authority_name(&dashboard.task_authority);
    let task_truth_available = !matches!(
        dashboard.task_authority,
        TaskAuthorityState::NotCanonicalized
    ) && intelligence.is_some();
    let total_tasks = task_truth_available.then_some(tasks.len());
    let completed_tasks = total_tasks.map(|_| completed);
    let active_tasks = total_tasks.map(|_| active);
    let current_task = truth
        .current_task_id
        .as_deref()
        .and_then(|task_id| tasks.iter().find(|task| task.id == task_id));
    let current_workflow = (!root_tasks_only)
        .then(|| {
            truth
                .current_task_id
                .as_deref()
                .and_then(|task_id| workflow_tasks.iter().find(|task| task.task_id == task_id))
        })
        .flatten();
    let current_state = truth.workflow_state.clone();
    let materialized_conflict = current_workflow.as_ref().and_then(|workflow| {
        dashboard
            .materialized
            .declared_workflow_state
            .as_ref()
            .filter(|declared| declared.as_str() != workflow.current_state.to_string())
    });
    if materialized_conflict.is_some() {
        push_warning(
            &mut warnings,
            "materialized dashboard workflow state conflicts with stronger M10 workflow truth",
        );
    }
    let last_action = current_workflow
        .as_ref()
        .and_then(|task| task.latest_event.as_ref())
        .map(|event| ActionSummary {
            summary: event.summary.clone(),
            occurred_at: event.occurred_at.clone(),
            actor: event.actor_type.map(|actor| actor.to_string()),
        });
    let allowed_actors: Vec<String> = if root_tasks_only {
        truth.required_actor.clone().into_iter().collect()
    } else {
        current_workflow
            .as_ref()
            .map(|task| {
                task.allowed_actors
                    .iter()
                    .map(ToString::to_string)
                    .collect()
            })
            .unwrap_or_default()
    };
    let next_action = truth.next_action.clone();
    let refresh_health = read_task_refresh_health(database, &project.id)?;
    if let Some(refresh) = refresh_health.as_ref() {
        if refresh.status == "DEGRADED" {
            push_warning(
                &mut warnings,
                refresh
                    .error
                    .clone()
                    .unwrap_or_else(|| "M09 task refresh is degraded".into()),
            );
        }
    }
    let health = project_health(
        project,
        &dashboard,
        &workflow_tasks,
        workflows_available,
        refresh_health.as_ref(),
        materialized_conflict.is_some(),
        &truth,
        control_plane_snapshot.as_ref(),
    );
    let mut project_attention = Vec::new();
    let mut queue = Vec::new();
    for workflow in &workflow_tasks {
        if root_tasks_only {
            continue;
        }
        if attention_state(workflow.current_state) {
            project_attention.push(AttentionItem {
                id: format!("workflow:{}", workflow.task_id),
                project_id: project.id.clone(),
                project_name: project.name.clone(),
                task_id: Some(workflow.task_id.clone()),
                title: workflow.title.clone(),
                state: workflow.current_state.to_string(),
                detail: workflow
                    .latest_event
                    .as_ref()
                    .map(|event| event.summary.clone())
                    .unwrap_or_else(|| "Workflow state requires attention".into()),
                category: "WORKFLOW".into(),
                operational_identity: Some(structured_attention_identity(
                    "WORKFLOW",
                    Some(workflow.task_id.clone()),
                    workflow
                        .latest_event
                        .as_ref()
                        .map(|event| event.summary.as_str())
                        .unwrap_or("Workflow state requires attention"),
                )),
            });
        }
        if workflow.current_state.is_running()
            || workflow.current_state == WorkflowState::VerifyRequired
            || workflow.current_state.is_suspension()
        {
            queue.push(WorkQueueItem {
                id: format!("queue:{}", workflow.task_id),
                project_id: project.id.clone(),
                project_name: project.name.clone(),
                task_id: workflow.task_id.clone(),
                task: workflow.title.clone(),
                stage: workflow.current_state.to_string(),
                state: workflow.current_state.to_string(),
                actor: workflow.required_actor.clone().or_else(|| {
                    workflow
                        .latest_event
                        .as_ref()
                        .and_then(|event| event.actor_type.map(|actor| actor.to_string()))
                }),
                updated_at: workflow
                    .latest_event
                    .as_ref()
                    .map(|event| event.occurred_at.clone()),
                attention: workflow.attention_required,
            });
        }
    }
    if !root_tasks_only && is_single_dashboard_resolution(&dashboard) {
        let (dashboard_attention, dashboard_queue) =
            materialized_operational_evidence(&project, &dashboard);
        project_attention.extend(dashboard_attention);
        queue.extend(dashboard_queue);
    }
    if root_tasks_only {
        let (root_attention, root_queue) = root_tasks_current_items(&project, &truth);
        project_attention.extend(root_attention);
        queue.extend(root_queue);
    }
    let has_remote_identity = project
        .repository
        .as_ref()
        .and_then(|repository| repository.github_owner.as_ref())
        .is_some();
    if project.status == "MISSING" && !has_remote_identity {
        project_attention.push(AttentionItem {
            id: format!("project:{}", project.id),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id: None,
            title: "Registered project root unavailable".into(),
            state: "MISSING".into(),
            detail: "Repair the registered path before project operations resume.".into(),
            category: "REGISTRY".into(),
            operational_identity: None,
        });
    }
    for (index, warning) in dashboard
        .warnings
        .iter()
        .filter(|warning| {
            !root_tasks_only && dashboard_warning_requires_attention(&dashboard, warning)
        })
        .take(4)
        .enumerate()
    {
        project_attention.push(AttentionItem {
            id: format!("manifest:{}:{index}", project.id),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id: None,
            title: "Project Dashboard authority warning".into(),
            state: dashboard.manifest_status.clone().into_serialized(),
            detail: warning.clone(),
            category: "PROJECT_DASHBOARD".into(),
            operational_identity: None,
        });
    }
    let mut bounded_warnings = Vec::new();
    for warning in warnings.drain(..) {
        push_warning(&mut bounded_warnings, warning);
    }
    let summary = ProjectOperationSummary {
        project_id: project.id.clone(),
        name: project.name.clone(),
        registry_status: project.status.clone(),
        health,
        manifest_status: dashboard.manifest_status.clone().into_serialized(),
        tracking_mode: dashboard.tracking_mode.clone(),
        task_authority,
        provenance_mode: dashboard.provenance_mode.clone(),
        materialized: dashboard.materialized.clone(),
        canonical_task_source: dashboard.canonical_task_source.clone(),
        current_task: current_task.map(|task| TaskSummary {
            task_id: task.id.clone(),
            title: task.title.clone(),
            source_path: task.source_path.clone(),
            parsed_status: task.parsed_status.clone(),
            workflow_state: if root_tasks_only {
                truth.workflow_state.clone()
            } else {
                workflow_tasks
                    .iter()
                    .find(|workflow| workflow.task_id == task.id)
                    .map(|workflow| workflow.current_state.to_string())
            },
            required_actor: task.required_actor.clone(),
        }),
        current_state,
        current_milestone: truth.current_milestone.clone(),
        last_action,
        next_action,
        required_actor: truth.required_actor.clone(),
        blockers: truth.blockers.clone(),
        allowed_actors: if root_tasks_only || !allowed_actors.is_empty() {
            allowed_actors
        } else {
            dashboard
                .materialized
                .required_actor
                .clone()
                .into_iter()
                .collect()
        },
        total_tasks,
        active_tasks,
        completed_tasks,
        progress_percent: truth.progress_percent,
        progress_scope: truth.progress_scope.clone(),
        authority_source: truth.authority_source.clone(),
        provenance: truth.provenance.clone(),
        reconciliation_state: truth.reconciliation_state.clone(),
        warnings: bounded_warnings,
        refresh_status: refresh_health.as_ref().map(|health| health.status.clone()),
        refresh_at: refresh_health
            .as_ref()
            .map(|health| health.refreshed_at.clone()),
        refresh_error: refresh_health.and_then(|health| health.error),
        control_plane: control_plane_snapshot.as_ref().map(control_plane::summary),
        truth_sync: control_plane_snapshot
            .as_ref()
            .map(|value| value.truth_sync.clone())
            .unwrap_or_default(),
        github_tracking: None,
    };
    Ok((summary, project_attention, queue, Vec::new()))
}

pub(crate) fn summarize_project_for_cockpit(
    database: &DatabaseState,
    project: &ProjectRecord,
) -> Result<ProjectOperationSummary, String> {
    summarize_project(database, project).map(|(summary, _, _, _)| summary)
}

fn authoritative_tasks<'a>(
    dashboard: &ProjectDashboardResolution,
    intelligence: Option<&'a TaskIntelligenceSnapshot>,
) -> Vec<&'a ParsedTask> {
    let Some(snapshot) = intelligence else {
        return Vec::new();
    };
    let mut ids = HashSet::new();
    snapshot
        .tasks
        .iter()
        .filter(|task| match dashboard.task_authority {
            TaskAuthorityState::NotCanonicalized => false,
            TaskAuthorityState::Canonical => dashboard
                .canonical_task_source
                .as_deref()
                .map(|path| same_path(path, &task.source_path))
                .unwrap_or(false),
            TaskAuthorityState::FallbackM08M09 => true,
        })
        .filter(|task| ids.insert(task.id.clone()))
        .collect()
}

fn task_is_complete(task: &ParsedTask, workflow: Option<&WorkflowTask>) -> bool {
    workflow
        .map(|workflow| workflow.current_state == WorkflowState::TaskComplete)
        .unwrap_or(false)
        || matches!(
            task.parsed_status.to_ascii_uppercase().as_str(),
            "COMPLETE" | "COMPLETED" | "DONE" | "TASK_COMPLETE"
        )
}

fn project_health(
    project: &ProjectRecord,
    dashboard: &ProjectDashboardResolution,
    workflows: &[WorkflowTask],
    workflows_available: bool,
    refresh_health: Option<&TaskRefreshHealth>,
    materialized_conflict: bool,
    truth: &control_plane::ProjectTruth,
    control_plane: Option<&control_plane::ControlPlaneSnapshot>,
) -> String {
    if project.status == "MISSING" {
        return "MISSING".into();
    }
    if dashboard.provenance_mode.starts_with("ROOT_TASKS") {
        if truth.reconciliation_state == "NEEDS_RECONCILIATION" {
            return "NEEDS_RECONCILIATION".into();
        }
        return match truth.workflow_state.as_deref() {
            Some("BLOCKED") => "BLOCKED".into(),
            Some("WAITING_HUMAN") | Some("WAITING_EXTERNAL") => "ATTENTION".into(),
            Some("IN_PROGRESS") | Some("RUNNING") => "RUNNING".into(),
            Some("COMPLETE") | Some("COMPLETED") | Some("DONE") | Some("TASK_COMPLETE") => {
                "HEALTHY".into()
            }
            _ => "UNKNOWN".into(),
        };
    }
    if control_plane.is_some_and(|snapshot| snapshot.adopted)
        && truth.reconciliation_state == "NEEDS_RECONCILIATION"
    {
        return "NEEDS_RECONCILIATION".into();
    }
    if control_plane.is_some_and(|snapshot| {
        matches!(
            snapshot.git.last_remote_observation_status.as_str(),
            "DEGRADED" | "FAILED"
        )
    }) {
        return "ATTENTION".into();
    }
    if workflows.iter().any(|task| {
        matches!(
            task.current_state,
            WorkflowState::Blocked | WorkflowState::AuditFailed | WorkflowState::FixRequired
        )
    }) {
        return "BLOCKED".into();
    }
    if workflows
        .iter()
        .any(|task| attention_state(task.current_state))
        || materialized_conflict
        || dashboard
            .warnings
            .iter()
            .any(|warning| dashboard_warning_requires_attention(dashboard, warning))
    {
        return "ATTENTION".into();
    }
    if workflows.iter().any(|task| task.current_state.is_running()) {
        return "RUNNING".into();
    }
    if refresh_health.is_some_and(|health| health.status == "DEGRADED") {
        return "ATTENTION".into();
    }
    if !workflows_available {
        return "UNKNOWN".into();
    }
    if dashboard.task_authority == TaskAuthorityState::FallbackM08M09
        && dashboard.manifest_status != project_dashboard::ManifestStatus::Absent
    {
        return "UNKNOWN".into();
    }
    if let Some(health) = dashboard.materialized.health.as_deref() {
        if matches!(health, "HEALTHY" | "ATTENTION" | "BLOCKED") {
            return health.to_string();
        }
    }
    if workflows.is_empty() {
        return "UNKNOWN".into();
    }
    "HEALTHY".into()
}

fn dashboard_warning_requires_attention(
    dashboard: &ProjectDashboardResolution,
    warning: &str,
) -> bool {
    match dashboard.manifest_status {
        project_dashboard::ManifestStatus::Absent => false,
        project_dashboard::ManifestStatus::Malformed
        | project_dashboard::ManifestStatus::Stale
        | project_dashboard::ManifestStatus::Unavailable => true,
        project_dashboard::ManifestStatus::Partial | project_dashboard::ManifestStatus::Valid => {
            let lower = warning.to_ascii_lowercase();
            lower.contains("conflict")
                || lower.contains("degraded")
                || lower.contains("rejected")
                || lower.contains("canonical task source is unavailable")
        }
    }
}

fn is_single_dashboard_resolution(dashboard: &ProjectDashboardResolution) -> bool {
    matches!(
        dashboard.manifest_status,
        project_dashboard::ManifestStatus::Valid | project_dashboard::ManifestStatus::Partial
    ) && dashboard.tracking_mode.as_deref() == Some("single-dashboard-watch")
}

fn is_single_dashboard_summary(summary: &ProjectOperationSummary) -> bool {
    matches!(summary.manifest_status.as_str(), "VALID" | "PARTIAL")
        && summary.tracking_mode.as_deref() == Some("single-dashboard-watch")
}

fn materialized_operational_evidence(
    project: &ProjectRecord,
    dashboard: &ProjectDashboardResolution,
) -> (Vec<AttentionItem>, Vec<WorkQueueItem>) {
    let materialized = &dashboard.materialized;
    let mut attention = Vec::new();
    let mut queue = Vec::new();
    let dashboard_task_id =
        meaningful_optional_value(materialized.current_task_id.as_deref().unwrap_or_default());
    let meaningful_blockers = materialized
        .blockers_waiting
        .iter()
        .filter(|value| is_meaningful_materialized_value(value))
        .cloned()
        .collect::<Vec<_>>();
    let mut blocker_keys = HashSet::new();
    for blocker in &meaningful_blockers {
        let blocker_key = normalize_operational_identity(blocker);
        if !blocker_keys.insert(blocker_key.clone()) {
            continue;
        }
        attention.push(AttentionItem {
            id: stable_materialized_id(&project.id, "BLOCKER", &blocker_key, 0),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id: dashboard_task_id.clone(),
            title: "Project Dashboard blocker or wait".into(),
            state: if materialized
                .project_status
                .as_deref()
                .is_some_and(|status| status == "BLOCKED")
            {
                "BLOCKED"
            } else {
                "WAITING"
            }
            .into(),
            detail: blocker.clone(),
            category: "PROJECT_DASHBOARD".into(),
            operational_identity: Some(structured_attention_identity(
                "PROJECT_DASHBOARD_BLOCKER",
                dashboard_task_id.clone(),
                blocker,
            )),
        });
    }
    if let Some(waiting_on) = materialized
        .waiting_on
        .as_deref()
        .filter(|value| is_meaningful_materialized_value(value))
        .filter(|value| {
            !meaningful_blockers
                .iter()
                .any(|blocker| materialized_values_overlap(blocker, value))
        })
    {
        attention.push(AttentionItem {
            id: stable_materialized_id(
                &project.id,
                "WAITING",
                &normalize_operational_identity(waiting_on),
                0,
            ),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id: dashboard_task_id.clone(),
            title: "Project Dashboard waiting".into(),
            state: "WAITING".into(),
            detail: format!("Waiting on: {waiting_on}"),
            category: "PROJECT_DASHBOARD".into(),
            operational_identity: Some(structured_attention_identity(
                "PROJECT_DASHBOARD_WAITING",
                dashboard_task_id.clone(),
                waiting_on,
            )),
        });
    }
    let status_attention = materialized
        .project_status
        .as_deref()
        .is_some_and(|status| status == "BLOCKED")
        || materialized
            .health
            .as_deref()
            .is_some_and(|health| matches!(health, "BLOCKED" | "ATTENTION"));
    if status_attention && attention.is_empty() {
        let state = materialized
            .project_status
            .as_deref()
            .filter(|status| *status == "BLOCKED")
            .or_else(|| materialized.health.as_deref())
            .unwrap_or("ATTENTION");
        attention.push(AttentionItem {
            id: stable_materialized_id(
                &project.id,
                "STATUS",
                &format!(
                    "{}:{}",
                    materialized.project_status.as_deref().unwrap_or("UNKNOWN"),
                    materialized.health.as_deref().unwrap_or("UNKNOWN")
                ),
                0,
            ),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id: dashboard_task_id.clone(),
            title: "Project Dashboard status requires attention".into(),
            state: state.into(),
            detail: format!(
                "Project status {}, health {}",
                materialized.project_status.as_deref().unwrap_or("UNKNOWN"),
                materialized.health.as_deref().unwrap_or("UNKNOWN")
            ),
            category: "PROJECT_DASHBOARD".into(),
            operational_identity: Some(structured_attention_identity(
                "PROJECT_DASHBOARD_STATUS",
                dashboard_task_id.clone(),
                &format!(
                    "{}:{}",
                    materialized.project_status.as_deref().unwrap_or("UNKNOWN"),
                    materialized.health.as_deref().unwrap_or("UNKNOWN")
                ),
            )),
        });
    }
    let mut quality_occurrences = HashMap::new();
    for fact in &materialized.quality_verification {
        if explicit_materialized_failure(&fact.value) {
            let quality_key = format!(
                "{}:{}",
                normalize_operational_identity(&fact.label),
                normalize_operational_identity(&fact.value)
            );
            let occurrence = materialized_occurrence(&mut quality_occurrences, &quality_key);
            attention.push(AttentionItem {
                id: stable_materialized_id(&project.id, "QUALITY", &quality_key, occurrence),
                project_id: project.id.clone(),
                project_name: project.name.clone(),
                task_id: dashboard_task_id.clone(),
                title: "Project Dashboard verification failed".into(),
                state: "FAILED".into(),
                detail: format!("{}: {}", fact.label, fact.value),
                category: "PROJECT_DASHBOARD".into(),
                operational_identity: Some(structured_attention_identity(
                    "PROJECT_DASHBOARD_QUALITY",
                    dashboard_task_id.clone(),
                    &fact.label,
                )),
            });
        }
    }
    let mut work_occurrences = HashMap::new();
    for work in &materialized.current_work {
        let Some(state) = materialized_queue_state(&work.status) else {
            continue;
        };
        let work_key = format!(
            "{}:{}:{}:{}",
            normalize_operational_identity(&work.item),
            normalize_operational_identity(&work.status),
            normalize_operational_identity(&work.owner_actor),
            normalize_operational_identity(&work.evidence_source)
        );
        let occurrence = materialized_occurrence(&mut work_occurrences, &work_key);
        let task_id = if work.id.is_empty() {
            stable_materialized_id(&project.id, "TASK", &work_key, occurrence)
        } else {
            work.id.clone()
        };
        queue.push(WorkQueueItem {
            id: format!("PROJECT_DASHBOARD:WORK:{}:{task_id}", project.id),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id,
            task: work.item.clone(),
            stage: state.to_string(),
            state: state.to_string(),
            actor: meaningful_optional_value(&work.owner_actor),
            updated_at: None,
            attention: matches!(state, "WAITING" | "BLOCKED" | "VERIFYING"),
        });
    }
    (attention, queue)
}

fn is_meaningful_materialized_value(value: &str) -> bool {
    !matches!(
        value.trim().to_ascii_uppercase().as_str(),
        "" | "NONE" | "UNKNOWN" | "NOT_VERIFIED" | "NONE VERIFIED"
    )
}

fn meaningful_optional_value(value: &str) -> Option<String> {
    is_meaningful_materialized_value(value).then(|| value.to_string())
}

fn stable_materialized_id(
    project_id: &str,
    evidence_class: &str,
    identity: &str,
    occurrence: usize,
) -> String {
    let key =
        format!("PROJECT_DASHBOARD\0{project_id}\0{evidence_class}\0{identity}\0{occurrence}");
    let digest = format!("{:x}", Sha256::digest(key.as_bytes()));
    format!("PROJECT_DASHBOARD:{evidence_class}:{}", &digest[..16])
}

fn materialized_occurrence(counts: &mut HashMap<String, usize>, identity: &str) -> usize {
    let occurrence = counts.get(identity).copied().unwrap_or_default();
    counts.insert(identity.to_string(), occurrence.saturating_add(1));
    occurrence
}

fn normalize_operational_identity(value: &str) -> String {
    // Preserve the full parser-bounded UTF-8 scalar; fold whitespace and Unicode case only.
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn materialized_values_overlap(left: &str, right: &str) -> bool {
    let left = normalize_materialized_text(left);
    let right = normalize_materialized_text(right);
    left == right || left.contains(&right) || right.contains(&left)
}

fn normalize_materialized_text(value: &str) -> String {
    normalize_operational_identity(value)
}

fn materialized_queue_state(value: &str) -> Option<&'static str> {
    match value
        .trim()
        .to_ascii_uppercase()
        .replace(['-', ' '], "_")
        .as_str()
    {
        "ACTIVE" | "IN_PROGRESS" | "RUNNING" | "IMPLEMENTING" => Some("RUNNING"),
        "AUDITING" => Some("AUDITING"),
        "VERIFYING" | "VERIFICATION" => Some("VERIFYING"),
        "WAITING" => Some("WAITING"),
        "BLOCKED" => Some("BLOCKED"),
        "COMPLETE_PENDING_AUDIT" | "IMPLEMENTATION_COMPLETE_PENDING_AUDIT" => Some("VERIFYING"),
        _ => None,
    }
}

fn explicit_materialized_failure(value: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| {
            matches!(
                token.to_ascii_uppercase().as_str(),
                "FAIL" | "FAILED" | "ERROR" | "BLOCKED"
            )
        })
}

fn deduplicate_materialized_attention(items: &mut Vec<AttentionItem>) {
    let stronger = items
        .iter()
        .filter(|item| item.category != "PROJECT_DASHBOARD")
        .cloned()
        .collect::<Vec<_>>();
    let mut seen_materialized_ids = HashSet::new();
    items.retain(|item| {
        if item.category != "PROJECT_DASHBOARD" {
            return true;
        }
        if !seen_materialized_ids.insert(item.id.clone()) {
            return false;
        }
        !stronger
            .iter()
            .any(|candidate| attention_identities_match(item, candidate))
    });
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AttentionIdentity {
    evidence_class: String,
    task_id: Option<String>,
    source: String,
}

fn structured_attention_identity(
    evidence_class: &str,
    task_id: Option<String>,
    source: &str,
) -> AttentionIdentity {
    AttentionIdentity {
        evidence_class: evidence_class.into(),
        task_id,
        source: normalize_operational_identity(source),
    }
}

fn attention_identity(item: &AttentionItem) -> Option<AttentionIdentity> {
    item.operational_identity.clone()
}

fn attention_identities_match(dashboard: &AttentionItem, stronger: &AttentionItem) -> bool {
    if dashboard.category != "PROJECT_DASHBOARD"
        || stronger.category == "PROJECT_DASHBOARD"
        || dashboard.project_id != stronger.project_id
    {
        return false;
    }
    let Some(dashboard_identity) = attention_identity(dashboard) else {
        return false;
    };
    let Some(stronger_identity) = attention_identity(stronger) else {
        return false;
    };
    let task_matches = match (&dashboard_identity.task_id, &stronger_identity.task_id) {
        (Some(left), Some(right)) => left == right,
        _ => false,
    };
    let source_matches = dashboard_identity.source == stronger_identity.source;
    if !source_matches {
        return false;
    }
    match dashboard_identity.evidence_class.as_str() {
        "PROJECT_DASHBOARD_QUALITY" => {
            task_matches
                && matches!(
                    stronger_identity.evidence_class.as_str(),
                    "TEST_RUN" | "AUDIT"
                )
        }
        "PROJECT_DASHBOARD_WAITING" | "PROJECT_DASHBOARD_BLOCKER" => {
            task_matches
                && matches!(
                    stronger_identity.evidence_class.as_str(),
                    "WORKFLOW" | "PERMISSION"
                )
                || dashboard_identity.task_id.is_none()
                    && stronger_identity.task_id.is_none()
                    && matches!(
                        stronger_identity.evidence_class.as_str(),
                        "WORKFLOW" | "PERMISSION"
                    )
        }
        _ => false,
    }
}

fn deduplicate_materialized_queue(items: &mut Vec<WorkQueueItem>) {
    let stronger = items
        .iter()
        .filter(|item| !item.id.starts_with("PROJECT_DASHBOARD:"))
        .cloned()
        .collect::<Vec<_>>();
    items.retain(|item| {
        !item.id.starts_with("PROJECT_DASHBOARD:")
            || !stronger.iter().any(|candidate| {
                candidate.project_id == item.project_id
                    && (candidate.task_id == item.task_id
                        || materialized_values_overlap(&candidate.task, &item.task))
            })
    });
}

fn append_materialized_activity(
    summaries: &[ProjectOperationSummary],
    activity: &mut Vec<ActivityItem>,
) {
    for summary in summaries {
        if !is_single_dashboard_summary(summary) {
            continue;
        }
        for event in summary.materialized.recent_meaningful_activity.iter() {
            if activity
                .iter()
                .any(|item| item.project_id == summary.project_id && item.event == *event)
            {
                continue;
            }
            let identity = normalize_operational_identity(event);
            let occurrence = activity
                .iter()
                .filter(|item| {
                    item.project_id == summary.project_id
                        && item.kind == "PROJECT_DASHBOARD"
                        && normalize_operational_identity(&item.event) == identity
                })
                .count();
            activity.push(ActivityItem {
                id: stable_materialized_id(&summary.project_id, "ACTIVITY", &identity, occurrence),
                project_id: summary.project_id.clone(),
                project_name: summary.name.clone(),
                kind: "PROJECT_DASHBOARD".into(),
                event: event.clone(),
                state: Some("DASHBOARD".into()),
                actor: None,
                occurred_at: "UNDATED".into(),
            });
        }
    }
}

fn attention_state(state: WorkflowState) -> bool {
    matches!(
        state,
        WorkflowState::WaitingHuman
            | WorkflowState::WaitingExternal
            | WorkflowState::DesignGate
            | WorkflowState::Blocked
            | WorkflowState::AuditFailed
            | WorkflowState::FixRequired
            | WorkflowState::AuditRequired
            | WorkflowState::VerifyRequired
    )
}

fn root_tasks_current_items(
    project: &ProjectRecord,
    truth: &control_plane::ProjectTruth,
) -> (Vec<AttentionItem>, Vec<WorkQueueItem>) {
    let state = truth
        .workflow_state
        .as_deref()
        .unwrap_or("NEEDS_RECONCILIATION");
    let needs_attention = truth.reconciliation_state == "NEEDS_RECONCILIATION"
        || matches!(state, "BLOCKED" | "WAITING_HUMAN" | "WAITING_EXTERNAL");
    let mut attention = Vec::new();
    if needs_attention {
        let detail = if !truth.blockers.is_empty() {
            truth.blockers.join("; ")
        } else {
            truth
                .next_action
                .clone()
                .unwrap_or_else(|| "Repository-root TASKS.md requires reconciliation".into())
        };
        attention.push(AttentionItem {
            id: format!("root-tasks:attention:{}", project.id),
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            task_id: truth.current_task_id.clone(),
            title: truth
                .current_task_title
                .clone()
                .unwrap_or_else(|| "Repository-root TASKS.md requires reconciliation".into()),
            state: state.into(),
            detail: detail.clone(),
            category: "ROOT_TASKS".into(),
            operational_identity: Some(structured_attention_identity(
                "ROOT_TASKS",
                truth.current_task_id.clone(),
                &detail,
            )),
        });
    }
    let running = matches!(state, "RUNNING" | "IN_PROGRESS");
    let queue = if running {
        truth
            .current_task_id
            .clone()
            .zip(truth.current_task_title.clone())
            .map(|(task_id, task)| {
                vec![WorkQueueItem {
                    id: format!("root-tasks:queue:{}", project.id),
                    project_id: project.id.clone(),
                    project_name: project.name.clone(),
                    task_id,
                    task,
                    stage: state.into(),
                    state: state.into(),
                    actor: truth.required_actor.clone(),
                    updated_at: None,
                    attention: false,
                }]
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    (attention, queue)
}

fn is_running_state(state: &str) -> bool {
    let normalized = state.trim().to_ascii_uppercase();
    matches!(
        normalized.as_str(),
        "RUNNING" | "IN_PROGRESS" | "IMPLEMENTING" | "EXECUTING" | "WORKING"
    ) || normalized.ends_with("_RUNNING")
        || normalized.ends_with("_IN_PROGRESS")
        || normalized.ends_with("_IMPLEMENTING")
        || normalized.ends_with("_EXECUTING")
        || normalized.ends_with("_WORKING")
}

fn read_evidence_items(
    database: &DatabaseState,
) -> Result<(Vec<AttentionItem>, Vec<WorkQueueItem>), String> {
    let connection = database.open_connection()?;
    let mut attention = Vec::new();
    let mut queue = Vec::new();
    {
        let mut statement = connection.prepare("SELECT t.id, t.project_id, COALESCE(p.name, 'Unassigned'), t.task_id, t.result, t.command FROM test_runs t LEFT JOIN projects p ON p.id=t.project_id WHERE (p.status IS NULL OR p.status != 'ARCHIVED') AND t.finished_at IS NOT NULL AND upper(t.result) IN ('FAIL','FAILED','ERROR') ORDER BY COALESCE(t.finished_at, t.started_at) DESC, t.id DESC LIMIT ?1").map_err(|e| format!("read failed test evidence: {e}"))?;
        let rows = statement
            .query_map([MAX_ATTENTION_ITEMS as i64], |row| {
                Ok(AttentionItem {
                    id: format!("test-failure:{}", row.get::<_, String>(0)?),
                    project_id: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    project_name: row.get(2)?,
                    task_id: row.get(3)?,
                    title: "Verification/test failed".into(),
                    state: row.get(4)?,
                    detail: format!("Test check: {}", row.get::<_, String>(5)?),
                    category: "TEST_RUN".into(),
                    operational_identity: Some(structured_attention_identity(
                        "TEST_RUN",
                        row.get(3)?,
                        &row.get::<_, String>(5)?,
                    )),
                })
            })
            .map_err(|e| format!("read failed test evidence: {e}"))?;
        attention.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read failed test evidence: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT a.id, a.project_id, COALESCE(p.name, 'Unassigned'), a.task_id, a.result, a.summary FROM audits a LEFT JOIN projects p ON p.id=a.project_id WHERE (p.status IS NULL OR p.status != 'ARCHIVED') AND upper(a.result) IN ('FAIL','FAILED','ERROR') ORDER BY a.created_at DESC, a.id DESC LIMIT ?1").map_err(|e| format!("read failed audit evidence: {e}"))?;
        let rows = statement
            .query_map([MAX_ATTENTION_ITEMS as i64], |row| {
                Ok(AttentionItem {
                    id: format!("audit-failure:{}", row.get::<_, String>(0)?),
                    project_id: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    project_name: row.get(2)?,
                    task_id: row.get(3)?,
                    title: "Audit failed".into(),
                    state: row.get(4)?,
                    detail: format!(
                        "Audit check: {}",
                        row.get::<_, Option<String>>(5)?.unwrap_or_default()
                    ),
                    category: "AUDIT".into(),
                    operational_identity: Some(structured_attention_identity(
                        "AUDIT",
                        row.get(3)?,
                        &row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                    )),
                })
            })
            .map_err(|e| format!("read failed audit evidence: {e}"))?;
        attention.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read failed audit evidence: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT r.id, s.project_id, COALESCE(p.name, 'Unassigned'), s.task_id, r.permission_kind, r.state FROM permission_requests r LEFT JOIN agent_sessions s ON s.id=r.session_id LEFT JOIN projects p ON p.id=s.project_id WHERE (p.status IS NULL OR p.status != 'ARCHIVED') AND upper(r.state) IN ('PENDING','OPEN','REQUESTED') AND r.decided_at IS NULL ORDER BY r.created_at DESC, r.id DESC LIMIT ?1").map_err(|e| format!("read permission requests: {e}"))?;
        let rows = statement
            .query_map([MAX_ATTENTION_ITEMS as i64], |row| {
                Ok(AttentionItem {
                    id: format!("permission:{}", row.get::<_, String>(0)?),
                    project_id: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    project_name: row.get(2)?,
                    task_id: row.get(3)?,
                    title: "Permission request pending".into(),
                    state: row.get(5)?,
                    detail: row.get::<_, String>(4)?,
                    category: "PERMISSION".into(),
                    operational_identity: Some(structured_attention_identity(
                        "PERMISSION",
                        row.get(3)?,
                        &row.get::<_, String>(4)?,
                    )),
                })
            })
            .map_err(|e| format!("read permission requests: {e}"))?;
        attention.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read permission requests: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT s.id, s.project_id, COALESCE(p.name, 'Unassigned'), s.task_id, s.provider, s.state, COALESCE(s.started_at, s.created_at) FROM agent_sessions s LEFT JOIN projects p ON p.id=s.project_id WHERE (p.status IS NULL OR p.status != 'ARCHIVED') AND upper(s.state) IN ('RUNNING','STARTING','WAITING_PERMISSION','WAITING_USER') AND s.task_id IS NOT NULL ORDER BY COALESCE(s.started_at, s.created_at) DESC, s.id DESC LIMIT ?1").map_err(|e| format!("read active agent sessions: {e}"))?;
        let rows = statement
            .query_map([MAX_QUEUE_ITEMS as i64], |row| {
                Ok(WorkQueueItem {
                    id: format!("agent:{}", row.get::<_, String>(0)?),
                    project_id: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    project_name: row.get(2)?,
                    task_id: row.get::<_, String>(3)?,
                    task: "Agent session evidence".into(),
                    stage: row.get(4)?,
                    state: row.get(5)?,
                    actor: row.get(4)?,
                    updated_at: row.get(6)?,
                    attention: matches!(
                        row.get::<_, String>(5)?.as_str(),
                        "WAITING_PERMISSION" | "WAITING_USER"
                    ),
                })
            })
            .map_err(|e| format!("read active agent sessions: {e}"))?;
        queue.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read active agent sessions: {e}"))?,
        );
    }
    Ok((attention, queue))
}
fn task_authority_name(state: &TaskAuthorityState) -> String {
    match state {
        TaskAuthorityState::Canonical => "CANONICAL",
        TaskAuthorityState::NotCanonicalized => "NOT_CANONICALIZED",
        TaskAuthorityState::FallbackM08M09 => "FALLBACK_M08_M09",
    }
    .into()
}
fn same_path(left: &str, right: &str) -> bool {
    left.replace('\\', "/")
        .trim_matches('/')
        .to_ascii_lowercase()
        == right
            .replace('\\', "/")
            .trim_matches('/')
            .to_ascii_lowercase()
}

fn read_activity(database: &DatabaseState, limit: usize) -> Result<Vec<ActivityItem>, String> {
    let limit = limit.clamp(1, MAX_ACTIVITY_LIMIT);
    let connection = database.open_connection()?;
    let mut items = Vec::new();
    {
        let mut statement = connection.prepare("SELECT e.id, t.project_id, p.name, e.event_type, e.summary, e.to_state, e.actor_type, e.occurred_at FROM task_events e JOIN tasks t ON t.id=e.task_id JOIN projects p ON p.id=t.project_id WHERE p.status != 'ARCHIVED' ORDER BY e.occurred_at DESC, e.id DESC LIMIT ?1").map_err(|e| format!("read workflow activity: {e}"))?;
        let rows = statement
            .query_map([limit as i64], |row| {
                Ok(ActivityItem {
                    id: format!("WORKFLOW:{}", row.get::<_, String>(0)?),
                    project_id: row.get(1)?,
                    project_name: row.get(2)?,
                    kind: "WORKFLOW".into(),
                    event: row.get(4)?,
                    state: row.get(5)?,
                    actor: row.get(6)?,
                    occurred_at: row.get(7)?,
                })
            })
            .map_err(|e| format!("read workflow activity: {e}"))?;
        items.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read workflow activity: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT e.id, s.project_id, COALESCE(p.name, 'Unassigned'), e.event_type, e.payload_json, s.state, s.provider, e.occurred_at FROM agent_events e JOIN agent_sessions s ON s.id=e.session_id LEFT JOIN projects p ON p.id=s.project_id WHERE p.status IS NULL OR p.status != 'ARCHIVED' ORDER BY e.occurred_at DESC, e.id DESC LIMIT ?1").map_err(|e| format!("read agent activity: {e}"))?;
        let rows = statement
            .query_map([limit as i64], |row| {
                Ok(ActivityItem {
                    id: format!("AGENT_EVENT:{}", row.get::<_, String>(0)?),
                    project_id: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    project_name: row.get(2)?,
                    kind: "AGENT_EVENT".into(),
                    event: format!("Agent event: {}", row.get::<_, String>(3)?),
                    state: row.get(5)?,
                    actor: row.get(6)?,
                    occurred_at: row.get(7)?,
                })
            })
            .map_err(|e| format!("read agent activity: {e}"))?;
        items.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read agent activity: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT s.id, s.project_id, COALESCE(p.name, 'Unassigned'), s.provider, s.state, s.started_at, s.created_at FROM agent_sessions s LEFT JOIN projects p ON p.id=s.project_id WHERE p.status IS NULL OR p.status != 'ARCHIVED' ORDER BY COALESCE(s.started_at, s.created_at) DESC, s.id DESC LIMIT ?1").map_err(|e| format!("read agent sessions: {e}"))?;
        let rows = statement
            .query_map([limit as i64], |row| {
                Ok(ActivityItem {
                    id: format!("AGENT_SESSION:{}", row.get::<_, String>(0)?),
                    project_id: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    project_name: row.get(2)?,
                    kind: "AGENT_SESSION".into(),
                    event: format!(
                        "{} session {}",
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?
                    ),
                    state: row.get(4)?,
                    actor: row.get(3)?,
                    occurred_at: row
                        .get::<_, Option<String>>(5)?
                        .or_else(|| row.get(6).ok())
                        .unwrap_or_default(),
                })
            })
            .map_err(|e| format!("read agent sessions: {e}"))?;
        items.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read agent sessions: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT a.id, a.project_id, COALESCE(p.name, 'Unassigned'), a.result, a.summary, a.created_at FROM audits a LEFT JOIN projects p ON p.id=a.project_id WHERE p.status IS NULL OR p.status != 'ARCHIVED' ORDER BY a.created_at DESC, a.id DESC LIMIT ?1").map_err(|e| format!("read audit activity: {e}"))?;
        let rows = statement
            .query_map([limit as i64], |row| {
                Ok(ActivityItem {
                    id: format!("AUDIT:{}", row.get::<_, String>(0)?),
                    project_id: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    project_name: row.get(2)?,
                    kind: "AUDIT".into(),
                    event: row
                        .get::<_, Option<String>>(4)?
                        .unwrap_or_else(|| "Audit evidence recorded".into()),
                    state: row.get(3)?,
                    actor: None,
                    occurred_at: row.get(5)?,
                })
            })
            .map_err(|e| format!("read audit activity: {e}"))?;
        items.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read audit activity: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT t.id, t.project_id, COALESCE(p.name, 'Unassigned'), t.result, t.command, t.finished_at, t.started_at FROM test_runs t LEFT JOIN projects p ON p.id=t.project_id WHERE p.status IS NULL OR p.status != 'ARCHIVED' ORDER BY COALESCE(t.finished_at, t.started_at) DESC, t.id DESC LIMIT ?1").map_err(|e| format!("read test activity: {e}"))?;
        let rows = statement
            .query_map([limit as i64], |row| {
                Ok(ActivityItem {
                    id: format!("TEST:{}", row.get::<_, String>(0)?),
                    project_id: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    project_name: row.get(2)?,
                    kind: "TEST_RUN".into(),
                    event: format!("Verification {}", row.get::<_, String>(3)?),
                    state: row.get(3)?,
                    actor: None,
                    occurred_at: row
                        .get::<_, Option<String>>(5)?
                        .or_else(|| row.get(6).ok())
                        .unwrap_or_default(),
                })
            })
            .map_err(|e| format!("read test activity: {e}"))?;
        items.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read test activity: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT g.id, r.project_id, p.name, g.status_json, g.captured_at FROM git_snapshots g JOIN repositories r ON r.id=g.repository_id JOIN projects p ON p.id=r.project_id WHERE p.status != 'ARCHIVED' ORDER BY g.captured_at DESC, g.id DESC LIMIT ?1").map_err(|e| format!("read Git activity: {e}"))?;
        let rows = statement
            .query_map([limit as i64], |row| {
                Ok(ActivityItem {
                    id: format!("GIT:{}", row.get::<_, String>(0)?),
                    project_id: row.get(1)?,
                    project_name: row.get(2)?,
                    kind: "GIT_SNAPSHOT".into(),
                    event: "Git snapshot captured".into(),
                    state: Some("SNAPSHOT".into()),
                    actor: Some("Git Engine".into()),
                    occurred_at: row.get(4)?,
                })
            })
            .map_err(|e| format!("read Git activity: {e}"))?;
        items.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read Git activity: {e}"))?,
        );
    }
    {
        let mut statement = connection.prepare("SELECT s.id, s.project_id, p.name, s.watcher_health, s.evidence_generated_at FROM project_snapshots s JOIN projects p ON p.id=s.project_id WHERE p.status != 'ARCHIVED' ORDER BY s.evidence_generated_at DESC, s.id DESC LIMIT ?1").map_err(|e| format!("read project snapshot activity: {e}"))?;
        let rows = statement
            .query_map([limit as i64], |row| {
                Ok(ActivityItem {
                    id: format!("PROJECT_SNAPSHOT:{}", row.get::<_, String>(0)?),
                    project_id: row.get(1)?,
                    project_name: row.get(2)?,
                    kind: "PROJECT_SNAPSHOT".into(),
                    event: "Project watcher snapshot refreshed".into(),
                    state: row.get(3)?,
                    actor: Some("WATCHER".into()),
                    occurred_at: row.get(4)?,
                })
            })
            .map_err(|e| format!("read project snapshot activity: {e}"))?;
        items.extend(
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("read project snapshot activity: {e}"))?,
        );
    }
    items.sort_by(|left, right| {
        right
            .occurred_at
            .cmp(&left.occurred_at)
            .then(left.kind.cmp(&right.kind))
            .then(left.id.cmp(&right.id))
    });
    items.truncate(limit);
    Ok(items)
}

fn push_warning(warnings: &mut Vec<String>, message: impl AsRef<str>) {
    let mut bounded = message.as_ref().to_string();
    while bounded.len() > MAX_WARNING_SCALAR_BYTES {
        bounded.pop();
    }
    if warnings.iter().any(|existing| existing == &bounded) {
        return;
    }
    if warnings.len() < MAX_PROJECT_WARNINGS.saturating_sub(1) {
        warnings.push(bounded);
    } else {
        warnings.truncate(MAX_PROJECT_WARNINGS.saturating_sub(1));
        warnings.push(format!(
            "WARNING_LIMIT_REACHED: project warning limit reached ({MAX_PROJECT_WARNINGS})"
        ));
    }
}

fn push_portfolio_warning(warnings: &mut Vec<String>, message: impl AsRef<str>) {
    let mut bounded = message.as_ref().to_string();
    while bounded.len() > MAX_WARNING_SCALAR_BYTES {
        bounded.pop();
    }
    if warnings.iter().any(|existing| existing == &bounded) {
        return;
    }
    if warnings.len() < MAX_PORTFOLIO_WARNINGS.saturating_sub(1) {
        warnings.push(bounded);
    } else {
        warnings.truncate(MAX_PORTFOLIO_WARNINGS.saturating_sub(1));
        warnings.push(format!(
            "WARNING_LIMIT_REACHED: portfolio warning limit reached ({MAX_PORTFOLIO_WARNINGS})"
        ));
    }
}

trait SerializedStatus {
    fn into_serialized(&self) -> String;
}
impl SerializedStatus for project_dashboard::ManifestStatus {
    fn into_serialized(&self) -> String {
        match self {
            project_dashboard::ManifestStatus::Valid => "VALID",
            project_dashboard::ManifestStatus::Partial => "PARTIAL",
            project_dashboard::ManifestStatus::Absent => "ABSENT",
            project_dashboard::ManifestStatus::Malformed => "MALFORMED",
            project_dashboard::ManifestStatus::Stale => "STALE",
            project_dashboard::ManifestStatus::Unavailable => "UNAVAILABLE",
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use crate::task_intelligence;
    use std::fs;
    use tempfile::{tempdir, TempDir};

    fn fixture(
        contents: &str,
        manifest: Option<&str>,
    ) -> (TempDir, TempDir, DatabaseState, String, Vec<ParsedTask>) {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(project_dir.path().join("TASKS.md"), contents).unwrap();
        if let Some(manifest) = manifest {
            fs::create_dir_all(project_dir.path().join(".hiveai")).unwrap();
            fs::write(
                project_dir
                    .path()
                    .join(project_dashboard::MANIFEST_RELATIVE_PATH),
                manifest,
            )
            .unwrap();
        }
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("M11A Fixture".into()),
            },
        )
        .unwrap();
        let parsed = task_intelligence::parse(&database, &project.id).unwrap();
        (db_dir, project_dir, database, project.id, parsed.tasks)
    }

    fn canonical_manifest() -> &'static str {
        "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\n## Source authorities\nCanonical task source: `TASKS.md`\n"
    }

    fn seed_workflow(
        database: &DatabaseState,
        project_id: &str,
        task_id: &str,
        state: &str,
        event_id: &str,
        occurred_at: &str,
        summary: &str,
    ) {
        database.open_connection().unwrap().execute_batch(&format!("INSERT OR IGNORE INTO tasks (id, project_id, source_id, title, state, required_actor, milestone, metadata_json, created_at, updated_at) VALUES ('{task_id}','{project_id}',NULL,'Fixture workflow task','{state}',NULL,NULL,'{{\"sourceActive\":true}}','{occurred_at}','{occurred_at}'); UPDATE tasks SET state='{state}', updated_at='{occurred_at}' WHERE id='{task_id}'; INSERT INTO task_events (id, task_id, event_type, from_state, to_state, actor_type, summary, evidence_json, occurred_at) VALUES ('{event_id}','{task_id}','WORKFLOW_TRANSITION','READY_FOR_IMPLEMENTATION','{state}','CODEX','{summary}','{{\"resumeState\":\"READY_FOR_IMPLEMENTATION\"}}','{occurred_at}');")).unwrap();
        let _: String = database
            .open_connection()
            .unwrap()
            .query_row("SELECT id FROM projects WHERE id=?1", [project_id], |row| {
                row.get(0)
            })
            .unwrap();
    }

    #[test]
    fn m11_current_task_selection_is_deterministic() {
        assert!(is_running_state("BUILDER_RUNNING"));
        assert!(is_running_state("IN_PROGRESS"));
        assert!(!is_running_state("READY_FOR_IMPLEMENTATION"));
    }

    #[test]
    fn m11_project_health_is_categorical_and_evidence_based() {
        assert_eq!(attention_state(WorkflowState::WaitingHuman), true);
        assert_eq!(attention_state(WorkflowState::TaskComplete), false);
    }

    fn dashboard_for_attention(
        status: project_dashboard::ManifestStatus,
    ) -> ProjectDashboardResolution {
        ProjectDashboardResolution {
            project_id: "p".into(),
            manifest_status: status,
            manifest_path: ".hiveai/PROJECT_DASHBOARD.md".into(),
            schema: None,
            project_key: None,
            repository: None,
            branch_policy: None,
            dashboard_mode: None,
            tracking_mode: None,
            refresh_policy: None,
            task_authority: TaskAuthorityState::FallbackM08M09,
            canonical_task_source: None,
            roles: std::collections::BTreeMap::new(),
            provenance_mode: "FALLBACK_M08_M09".into(),
            materialized: project_dashboard::MaterializedDashboardStatus::default(),
            warnings: Vec::new(),
        }
    }

    #[test]
    fn m11a_r14_absent_is_informational_but_malformed_and_stale_need_attention() {
        let mut absent = dashboard_for_attention(project_dashboard::ManifestStatus::Absent);
        absent
            .warnings
            .push(".hiveai/PROJECT_DASHBOARD.md is absent".into());
        assert!(!dashboard_warning_requires_attention(
            &absent,
            &absent.warnings[0]
        ));
        let malformed = dashboard_for_attention(project_dashboard::ManifestStatus::Malformed);
        assert!(dashboard_warning_requires_attention(
            &malformed,
            "manifest parse failed"
        ));
        let stale = dashboard_for_attention(project_dashboard::ManifestStatus::Stale);
        assert!(dashboard_warning_requires_attention(
            &stale,
            "repository identity conflict"
        ));
        let partial = dashboard_for_attention(project_dashboard::ManifestStatus::Partial);
        assert!(!dashboard_warning_requires_attention(
            &partial,
            "secondary provenance is missing"
        ));
        assert!(dashboard_warning_requires_attention(
            &partial,
            "materialized dashboard conflict"
        ));
    }

    #[test]
    fn m11_portfolio_counts_use_authoritative_tasks_only() {
        assert_eq!(same_path("TASKS.md", "tasks.md"), true);
    }

    #[test]
    fn m11a_r01_snapshot_reads_real_m10_workflow_rows_with_bound() {
        let (_db_dir, _project_dir, database, project_id, tasks) = fixture(
            "# Work\n- [ ] active task\n- [ ] waiting task\n",
            Some(canonical_manifest()),
        );
        let task = &tasks[0];
        let waiting = &tasks[1];
        seed_workflow(
            &database,
            &project_id,
            &task.id,
            "BUILDER_RUNNING",
            "workflow-live",
            "2026-08-26T10:00:00Z",
            "builder started",
        );
        seed_workflow(
            &database,
            &project_id,
            &waiting.id,
            "WAITING_HUMAN",
            "workflow-waiting",
            "2026-08-26T11:00:00Z",
            "owner decision required",
        );
        let snapshot = snapshot(&database).unwrap();
        let project = &snapshot.projects[0];
        assert_eq!(
            project.current_state.as_deref(),
            Some("NEEDS_RECONCILIATION")
        );
        assert!(project.last_action.is_none());
        assert!(!project
            .warnings
            .iter()
            .any(|warning| warning.contains("multiple active canonical workflow tasks")));
        assert!(!snapshot
            .attention
            .iter()
            .any(|item| item.project_id == project_id && item.state == "WAITING_HUMAN"));
        assert!(!snapshot
            .work_queue
            .iter()
            .any(|item| item.task_id == task.id && item.state == "BUILDER_RUNNING"));
        assert!(!snapshot
            .warnings
            .iter()
            .any(|warning| warning.starts_with("M10 workflow evidence unavailable")));
    }

    #[test]
    fn m11a_r02_missing_m09_is_unknown_and_empty_parsed_is_zero() {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(project_dir.path().join("TASKS.md"), "# Empty\n").unwrap();
        fs::create_dir_all(project_dir.path().join(".hiveai")).unwrap();
        fs::write(
            project_dir
                .path()
                .join(project_dashboard::MANIFEST_RELATIVE_PATH),
            canonical_manifest(),
        )
        .unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into_owned(),
                name: Some("Unknown Fixture".into()),
            },
        )
        .unwrap();
        let missing = snapshot(&database).unwrap();
        assert_eq!(missing.projects[0].total_tasks, None);
        assert_eq!(missing.projects[0].active_tasks, None);
        task_intelligence::parse(&database, &project.id).unwrap();
        let empty = snapshot(&database).unwrap();
        assert_eq!(empty.projects[0].total_tasks, Some(0));
        assert_eq!(empty.projects[0].active_tasks, Some(0));
    }

    #[test]
    fn m11a_r03_m10_complete_task_is_never_selected() {
        let (_db_dir, _project_dir, database, project_id, tasks) = fixture(
            "# Work\n- [ ] task A\n- [ ] task B\n",
            Some(canonical_manifest()),
        );
        seed_workflow(
            &database,
            &project_id,
            &tasks[0].id,
            "TASK_COMPLETE",
            "workflow-complete",
            "2026-08-26T12:00:00Z",
            "completed A",
        );
        seed_workflow(
            &database,
            &project_id,
            &tasks[1].id,
            "BUILDER_RUNNING",
            "workflow-active",
            "2026-08-26T11:00:00Z",
            "started B",
        );
        let snapshot = snapshot(&database).unwrap();
        assert!(snapshot.projects[0].current_task.is_none());
        assert_eq!(
            snapshot.projects[0].reconciliation_state,
            "NEEDS_RECONCILIATION"
        );
    }

    #[test]
    fn m11a_p2_materialized_dashboard_current_task_is_primary_without_double_counting_m09() {
        let (_db_dir, _project_dir, database, _project_id, _tasks) = fixture(
            "# Work\n- [ ] internal supporting task\n",
            Some("hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\n| Field | Value |\n| --- | --- |\n| Health | UNKNOWN |\n| Current task | Materialized dashboard task |\n| Current task ID | DASHBOARD-TASK |\n| Progress | 50% |\n| Next action | Run the dashboard gate |\n"),
        );
        let snapshot = snapshot(&database).unwrap();
        let project = &snapshot.projects[0];
        assert_eq!(project.total_tasks, Some(1));
        assert_eq!(project.progress_percent, None);
        assert!(project.current_task.is_none());
        assert_eq!(project.reconciliation_state, "NEEDS_RECONCILIATION");
        assert_eq!(project.tracking_mode.as_deref(), Some("ROOT_TASKS_ONLY"));
        assert_eq!(project.materialized, Default::default());
    }

    #[test]
    fn m11a_r16_materialized_dashboard_feeds_attention_queue_brief_and_undated_activity() {
        let manifest = "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\n| Field | Value |\n| --- | --- |\n| Project status | ACTIVE |\n| Health | unknown |\n| Required actor | human |\n| Waiting on | Human approval |\n## Current work\n| ID | Item | Status | Owner/actor | Evidence/source |\n| --- | --- | --- | --- | --- |\n| active-row | External implementation | IN_PROGRESS | CODEX | TASKS.md |\n| closed-row | Historical work | COMPLETE | CODEX | TASKS.md |\n| unknown-row | Unclear prose | MAYBE | CODEX | TASKS.md |\n## Blockers and waiting\n- Human approval\n## Quality and verification\n| Check | Result |\n| --- | --- |\n| Native tests | FAIL |\n| Frontend build | PASS |\n## Recent meaningful activity\n- Dashboard status was materialized\n";
        let (_db_dir, _project_dir, database, _project_id, _tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(manifest));
        let snapshot = snapshot(&database).unwrap();
        let project = &snapshot.projects[0];
        assert_eq!(project.total_tasks, Some(1));
        assert_eq!(project.materialized, Default::default());
        assert!(snapshot
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(snapshot
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
        assert!(snapshot
            .engineering_brief
            .facts
            .iter()
            .all(|fact| fact.source != "Project Dashboard"));
        assert!(snapshot
            .recent_activity
            .iter()
            .all(|item| item.kind != "PROJECT_DASHBOARD"));
    }

    #[test]
    fn m11a_r16_stronger_m10_workflow_suppresses_matching_dashboard_queue_row() {
        let manifest = "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## Current work\n| ID | Item | Status | Owner/actor | Evidence/source |\n| --- | --- | --- | --- | --- |\n| workflow-row | Same operational task | RUNNING | CODEX | TASKS.md |\n";
        let (_db_dir, _project_dir, database, _project_id, tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(manifest));
        let task_id = tasks[0].id.clone();
        let connection = database.open_connection().unwrap();
        connection
            .execute_batch(&format!(
                "UPDATE tasks SET title='Same operational task', state='BUILDER_RUNNING', updated_at='2026-08-26T10:00:00Z' WHERE id='{task_id}'; INSERT INTO task_events (id, task_id, event_type, from_state, to_state, actor_type, summary, evidence_json, occurred_at) VALUES ('workflow-row-event','{task_id}','WORKFLOW_TRANSITION','READY_FOR_IMPLEMENTATION','BUILDER_RUNNING','CODEX','builder running','{{}}','2026-08-26T10:00:00Z');"
            ))
            .unwrap();
        let snapshot = snapshot(&database).unwrap();
        assert!(!snapshot
            .work_queue
            .iter()
            .any(|item| item.task == "Same operational task"));
        assert!(!snapshot.work_queue.iter().any(|item| {
            item.task == "Same operational task" && item.id.starts_with("PROJECT_DASHBOARD:")
        }));
    }

    #[test]
    fn m11a_r23_full_scalar_blocker_and_activity_identity_is_collision_safe() {
        let prefix = "x".repeat(256);
        let blocker_a = format!("{prefix} suffix-alpha");
        let blocker_b = format!("{prefix} suffix-beta");
        let first_id = stable_materialized_id(
            "project",
            "BLOCKER",
            &normalize_operational_identity(&blocker_a),
            0,
        );
        let second_id = stable_materialized_id(
            "project",
            "BLOCKER",
            &normalize_operational_identity(&blocker_b),
            0,
        );
        assert_ne!(first_id, second_id);
        assert_eq!(
            first_id,
            stable_materialized_id(
                "project",
                "BLOCKER",
                &normalize_operational_identity(&blocker_a),
                0,
            )
        );
        assert!(first_id.len() <= "PROJECT_DASHBOARD:BLOCKER:".len() + 16);
    }

    #[test]
    fn m11a_r23_root_tasks_materialized_identity_is_not_current_evidence() {
        let manifest = "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## Blockers and waiting\n- poisoned dashboard blocker\n";
        let (_db_dir, _project_dir, database, _project_id, _tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(manifest));
        let first = snapshot(&database).unwrap();
        assert!(first
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(first
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
    }

    #[test]
    fn m11a_r23_long_quality_identity_is_historical_for_root_tasks() {
        let prefix = "quality".repeat(37);
        let dashboard_check = format!("{prefix} dashboard-suffix");
        let prefix_only_test = format!("{prefix} persisted-test-suffix");
        let prefix_only_audit = format!("{prefix} persisted-audit-suffix");
        let (_db_dir, project_dir, database, project_id, tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(canonical_manifest()));
        let task_id = tasks[0].id.clone();
        let manifest = format!(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nCurrent task ID: {task_id}\n## Quality and verification\n| Check | Result | Evidence |\n| --- | --- | --- |\n| {dashboard_check} | FAIL | dashboard |\n"
        );
        fs::write(
            project_dir
                .path()
                .join(project_dashboard::MANIFEST_RELATIVE_PATH),
            manifest,
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO test_runs (id, project_id, task_id, command, result, started_at, finished_at) VALUES ('long-prefix-test', ?1, ?2, ?3, 'FAIL', '2026-08-26T10:00:00Z', '2026-08-26T10:01:00Z')",
                rusqlite::params![project_id, task_id, prefix_only_test],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audits (id, project_id, task_id, result, summary, created_at) VALUES ('long-prefix-audit', ?1, ?2, 'FAIL', ?3, '2026-08-26T10:02:00Z')",
                rusqlite::params![project_id, task_id, prefix_only_audit],
            )
            .unwrap();

        let prefix_only = snapshot(&database).unwrap();
        assert!(prefix_only
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(prefix_only
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
        assert!(prefix_only
            .attention
            .iter()
            .all(|item| !matches!(item.category.as_str(), "TEST_RUN" | "AUDIT")));
        assert_eq!(
            prefix_only.kpis.needs_attention,
            Some(prefix_only.attention.len())
        );
    }

    #[test]
    fn m11a_r24_unicode_blocker_activity_and_scalar_identity_are_collision_safe() {
        let blocker_a = "build ğ blocker";
        let blocker_b = "build ü blocker";
        let activity_a = "deploy ç release";
        let activity_b = "deploy ş release";
        let long_unicode = format!("bounded {} fact", "λ".repeat(480));
        let manifest = format!(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nWaiting on: owner ğ\n## Current work\n| ID | Item | Status | Owner/actor | Evidence/source |\n| --- | --- | --- | --- | --- |\n| | work ğ | ACTIVE | CODEX | dashboard |\n| | work ü | ACTIVE | CODEX | dashboard |\n## Blockers and waiting\n- {blocker_a}\n- {blocker_a}\n- {blocker_b}\n- {long_unicode}\n## Recent meaningful activity\n- {activity_a}\n- {activity_b}\n"
        );
        let (_db_dir, project_dir, database, _project_id, _tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(&manifest));
        let first = snapshot(&database).unwrap();
        assert!(first
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(first
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
        assert_eq!(normalize_operational_identity(blocker_a), "build ğ blocker");
        assert_ne!(
            structured_attention_identity("PROJECT_DASHBOARD_BLOCKER", None, blocker_a),
            structured_attention_identity("PROJECT_DASHBOARD_BLOCKER", None, blocker_b)
        );
        assert_ne!(activity_a, activity_b);
        assert!(long_unicode.len() > 900);
        assert!(first
            .recent_activity
            .iter()
            .all(|item| item.kind != "PROJECT_DASHBOARD"));
    }

    #[test]
    fn m11a_r25_structured_quality_identity_preserves_colons_and_display_independence() {
        let dashboard_label = "build: windows: release";
        let (_db_dir, project_dir, database, project_id, tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(canonical_manifest()));
        let task_id = tasks[0].id.clone();
        let manifest = format!(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nCurrent task ID: {task_id}\n## Quality and verification\n| Check | Result | Evidence |\n| --- | --- | --- |\n| {dashboard_label} | FAIL | dashboard |\n"
        );
        fs::write(
            project_dir
                .path()
                .join(project_dashboard::MANIFEST_RELATIVE_PATH),
            manifest,
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO test_runs (id, project_id, task_id, command, result, started_at, finished_at) VALUES ('colon-test', ?1, ?2, 'build', 'FAIL', '2026-08-27T10:00:00Z', '2026-08-27T10:01:00Z')",
                rusqlite::params![project_id, task_id],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audits (id, project_id, task_id, result, summary, created_at) VALUES ('colon-audit', ?1, ?2, 'FAIL', 'build', '2026-08-27T10:02:00Z')",
                rusqlite::params![project_id, task_id],
            )
            .unwrap();

        let distinct = snapshot(&database).unwrap();
        assert!(distinct
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(distinct
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
        assert!(distinct
            .attention
            .iter()
            .all(|item| !matches!(item.category.as_str(), "TEST_RUN" | "AUDIT")));
        assert_eq!(
            distinct.kpis.needs_attention,
            Some(distinct.attention.len())
        );

        let mut dashboard = AttentionItem {
            id: "PROJECT_DASHBOARD:QUALITY:display-independent".into(),
            project_id: "project".into(),
            project_name: "Project".into(),
            task_id: Some("task".into()),
            title: "Dashboard failure".into(),
            state: "FAILED".into(),
            detail: "display label: FAILED".into(),
            category: "PROJECT_DASHBOARD".into(),
            operational_identity: Some(structured_attention_identity(
                "PROJECT_DASHBOARD_QUALITY",
                Some("task".into()),
                dashboard_label,
            )),
        };
        let mut stronger = AttentionItem {
            id: "test-failure:display-independent".into(),
            project_id: "project".into(),
            project_name: "Project".into(),
            task_id: Some("task".into()),
            title: "Test failure".into(),
            state: "FAIL".into(),
            detail: "Test check: an unrelated rendering".into(),
            category: "TEST_RUN".into(),
            operational_identity: Some(structured_attention_identity(
                "TEST_RUN",
                Some("task".into()),
                dashboard_label,
            )),
        };
        assert!(attention_identities_match(&dashboard, &stronger));
        dashboard.detail = "completely different display punctuation".into();
        stronger.detail = "another display string".into();
        assert!(attention_identities_match(&dashboard, &stronger));
    }

    #[test]
    fn m11a_r24_r25_unicode_colon_identity_remains_conservative() {
        let dashboard_label = "dağıtım: türkiye";
        let (_db_dir, project_dir, database, project_id, tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(canonical_manifest()));
        let task_id = tasks[0].id.clone();
        let manifest = format!(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nCurrent task ID: {task_id}\n## Quality and verification\n| Check | Result | Evidence |\n| --- | --- | --- |\n| {dashboard_label} | FAIL | dashboard |\n"
        );
        fs::write(
            project_dir
                .path()
                .join(project_dashboard::MANIFEST_RELATIVE_PATH),
            manifest,
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO test_runs (id, project_id, task_id, command, result, started_at, finished_at) VALUES ('unicode-test', ?1, ?2, 'dagitim', 'FAIL', '2026-08-27T11:00:00Z', '2026-08-27T11:01:00Z')",
                rusqlite::params![project_id, task_id],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audits (id, project_id, task_id, result, summary, created_at) VALUES ('unicode-audit', ?1, ?2, 'FAIL', 'dağıtım', '2026-08-27T11:02:00Z')",
                rusqlite::params![project_id, task_id],
            )
            .unwrap();
        let distinct = snapshot(&database).unwrap();
        assert!(distinct
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(distinct
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
        assert!(distinct
            .attention
            .iter()
            .all(|item| !matches!(item.category.as_str(), "TEST_RUN" | "AUDIT")));
        assert_eq!(
            distinct.kpis.needs_attention,
            Some(distinct.attention.len())
        );
    }

    #[test]
    fn m11a_r19_waiting_without_real_wait_fact_stays_out_of_attention() {
        for waiting_value in ["NONE", "UNKNOWN"] {
            let manifest = format!(
                "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nProject status: WAITING\nHealth: UNKNOWN\nWaiting on: {waiting_value}\n"
            );
            let (_db_dir, _project_dir, database, _project_id, _tasks) =
                fixture("# Work\n- [ ] internal task\n", Some(&manifest));
            let snapshot = snapshot(&database).unwrap();
            assert!(!snapshot
                .attention
                .iter()
                .any(|item| item.category == "PROJECT_DASHBOARD"));
        }
    }

    #[test]
    fn m11a_r19_waiting_requires_one_real_fact_and_blocked_is_independent() {
        let waiting = "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nProject status: WAITING\nHealth: UNKNOWN\nWaiting on: Human approval\n## Blockers and waiting\n- Human approval\n- Human approval\n";
        let (_db_dir, _project_dir, database, _project_id, _tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(waiting));
        let waiting_snapshot = snapshot(&database).unwrap();
        assert_eq!(
            waiting_snapshot
                .attention
                .iter()
                .filter(|item| item.category == "PROJECT_DASHBOARD")
                .count(),
            0
        );

        let blocked = "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nProject status: BLOCKED\nHealth: UNKNOWN\n";
        let (_db_dir, _project_dir, database, _project_id, _tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(blocked));
        let snapshot = snapshot(&database).unwrap();
        assert_eq!(
            snapshot
                .attention
                .iter()
                .filter(|item| item.category == "PROJECT_DASHBOARD")
                .count(),
            0
        );
    }

    #[test]
    fn m11a_r19_health_attention_and_blocked_remain_actionable() {
        for health in ["ATTENTION", "BLOCKED"] {
            let manifest = format!(
                "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nProject status: ACTIVE\nHealth: {health}\n"
            );
            let (_db_dir, _project_dir, database, _project_id, _tasks) =
                fixture("# Work\n- [ ] internal task\n", Some(&manifest));
            let snapshot = snapshot(&database).unwrap();
            assert_eq!(
                snapshot
                    .attention
                    .iter()
                    .filter(|item| item.category == "PROJECT_DASHBOARD")
                    .count(),
                0
            );
        }
    }

    #[test]
    fn m11a_r20_matching_test_and_audit_quality_suppress_only_dashboard_duplicates() {
        let (_db_dir, project_dir, database, project_id, tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(canonical_manifest()));
        let task_id = tasks[0].id.clone();
        let manifest = format!(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nCurrent task ID: {task_id}\n## Quality and verification\n| Check | Result | Evidence |\n| --- | --- | --- |\n| Native tests | FAIL | persisted test |\n| Security audit | FAIL | persisted audit |\n"
        );
        fs::write(
            project_dir
                .path()
                .join(project_dashboard::MANIFEST_RELATIVE_PATH),
            manifest,
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute(
                "INSERT INTO test_runs (id, project_id, task_id, command, result, started_at, finished_at) VALUES (?1, ?2, ?3, ?4, 'FAIL', '2026-08-26T10:00:00Z', '2026-08-26T10:01:00Z')",
                rusqlite::params!["matching-test", project_id, task_id, "Native tests"],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audits (id, project_id, task_id, result, summary, created_at) VALUES (?1, ?2, ?3, 'FAIL', ?4, '2026-08-26T10:02:00Z')",
                rusqlite::params!["matching-audit", project_id, task_id, "Security audit"],
            )
            .unwrap();
        let snapshot = snapshot(&database).unwrap();
        assert_eq!(
            snapshot
                .attention
                .iter()
                .filter(|item| item.category == "PROJECT_DASHBOARD")
                .count(),
            0
        );
        assert_eq!(
            snapshot
                .attention
                .iter()
                .filter(|item| item.category == "TEST_RUN")
                .count(),
            0
        );
        assert_eq!(
            snapshot
                .attention
                .iter()
                .filter(|item| item.category == "AUDIT")
                .count(),
            0
        );
        assert_eq!(
            snapshot.kpis.needs_attention,
            Some(snapshot.attention.len())
        );
    }

    #[test]
    fn m11a_r20_unproven_failures_remain_distinct_and_snapshot_ids_repeat() {
        let (_db_dir, project_dir, database, project_id, tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(canonical_manifest()));
        let task_id = tasks[0].id.clone();
        let manifest = format!(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nCurrent task ID: {task_id}\n## Quality and verification\n| Check | Result | Evidence |\n| --- | --- | --- |\n| Native tests | FAIL | dashboard |\n"
        );
        fs::write(
            project_dir
                .path()
                .join(project_dashboard::MANIFEST_RELATIVE_PATH),
            manifest,
        )
        .unwrap();
        database
            .open_connection()
            .unwrap()
            .execute(
                "INSERT INTO test_runs (id, project_id, task_id, command, result, started_at, finished_at) VALUES ('unrelated-test', ?1, ?2, 'cargo fmt', 'FAIL', '2026-08-26T10:00:00Z', '2026-08-26T10:01:00Z')",
                rusqlite::params![project_id, task_id],
            )
            .unwrap();
        let first = snapshot(&database).unwrap();
        assert!(first
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(first
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
        let second = snapshot(&database).unwrap();
        assert!(second
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(second
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
        assert_eq!(
            first
                .attention
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
            second
                .attention
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>()
        );
        assert_eq!(first.kpis.needs_attention, Some(first.attention.len()));
        assert_eq!(
            first
                .attention
                .iter()
                .filter(|item| item.category == "PROJECT_DASHBOARD")
                .count(),
            0
        );
        assert_eq!(
            first
                .attention
                .iter()
                .filter(|item| item.category == "TEST_RUN")
                .count(),
            0
        );
    }

    #[test]
    fn m11a_r20_matching_wait_uses_task_and_source_identity() {
        let (_db_dir, project_dir, database, project_id, tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(canonical_manifest()));
        let task_id = tasks[0].id.clone();
        let manifest = format!(
            "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nCurrent task ID: {task_id}\nWaiting on: FILESYSTEM\n"
        );
        fs::write(
            project_dir
                .path()
                .join(project_dashboard::MANIFEST_RELATIVE_PATH),
            manifest,
        )
        .unwrap();
        let connection = database.open_connection().unwrap();
        connection
            .execute_batch(&format!(
                "INSERT INTO agent_sessions (id,project_id,task_id,provider,state,created_at) VALUES ('permission-session','{project_id}','{task_id}','CODEX','WAITING_PERMISSION','2026-08-26T10:00:00Z'); INSERT INTO permission_requests (id,session_id,permission_kind,state,created_at) VALUES ('matching-permission','permission-session','FILESYSTEM','PENDING','2026-08-26T10:01:00Z');"
            ))
            .unwrap();
        let snapshot = snapshot(&database).unwrap();
        assert_eq!(
            snapshot
                .attention
                .iter()
                .filter(|item| item.category == "PROJECT_DASHBOARD")
                .count(),
            0
        );
        assert_eq!(
            snapshot
                .attention
                .iter()
                .filter(|item| item.category == "PERMISSION")
                .count(),
            0
        );
    }

    #[test]
    fn m11a_r22_root_tasks_ignores_materialized_ids_and_duplicate_facts() {
        let (_db_dir, project_dir, database, _project_id, _tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(canonical_manifest()));
        let first_manifest = "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## Blockers and waiting\n- Later blocker\n## Quality and verification\n| Check | Result | Evidence |\n| --- | --- | --- |\n| Same check | FAIL | one |\n| Same check | FAIL | two |\n## Recent meaningful activity\n- Later activity\n";
        fs::write(
            project_dir
                .path()
                .join(project_dashboard::MANIFEST_RELATIVE_PATH),
            first_manifest,
        )
        .unwrap();
        let first = snapshot(&database).unwrap();
        assert!(first
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(first
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("PROJECT_DASHBOARD:")));
        let second = snapshot(&database).unwrap();
        assert!(second
            .attention
            .iter()
            .all(|item| item.category != "PROJECT_DASHBOARD"));
        assert!(second
            .recent_activity
            .iter()
            .all(|item| item.kind != "PROJECT_DASHBOARD"));
    }

    #[test]
    fn m11a_r18_invalid_materialized_health_stays_unknown_in_command_center() {
        let manifest = "hiveaiDashboardSchema: hiveai-project-dashboard/v1\ndashboardMode: source-map\ntrackingMode: single-dashboard-watch\n## Source authorities\nCanonical task source: `TASKS.md`\n## H!veAI live status\nProject status: SUPER_ACTIVE\nHealth: BROKENISH\nRequired actor: mystery-agent\n";
        let (_db_dir, _project_dir, database, _project_id, _tasks) =
            fixture("# Work\n- [ ] internal task\n", Some(manifest));
        let project = &snapshot(&database).unwrap().projects[0];
        assert_eq!(project.health, "NEEDS_RECONCILIATION");
        assert_eq!(project.materialized, Default::default());
    }

    #[test]
    fn m11a_r06_mixed_evidence_attention_queue_and_activity_are_real_and_bounded() {
        let (_db_dir, _project_dir, database, project_id, tasks) =
            fixture("# Work\n- [ ] task\n", Some(canonical_manifest()));
        let task_id = &tasks[0].id;
        seed_workflow(
            &database,
            &project_id,
            task_id,
            "WAITING_HUMAN",
            "workflow-waiting",
            "2026-08-26T10:00:00Z",
            "owner decision required",
        );
        let connection = database.open_connection().unwrap();
        connection.execute_batch(&format!("INSERT INTO agent_sessions (id,project_id,task_id,provider,state,started_at,created_at) VALUES ('session-1','{project_id}','{task_id}','CODEX','RUNNING','2026-08-26T09:00:00Z','2026-08-26T09:00:00Z'); INSERT INTO agent_events (id,session_id,event_type,payload_json,occurred_at) VALUES ('agent-event-1','session-1','OUTPUT','{{}}','2026-08-26T09:01:00Z'); INSERT INTO audits (id,project_id,task_id,result,summary,created_at) VALUES ('audit-1','{project_id}','{task_id}','FAIL','audit failed','2026-08-26T09:02:00Z'); INSERT INTO test_runs (id,project_id,task_id,command,result,started_at,finished_at) VALUES ('test-1','{project_id}','{task_id}','cargo test','FAIL','2026-08-26T09:03:00Z','2026-08-26T09:04:00Z'); INSERT INTO permission_requests (id,session_id,permission_kind,requested_resource,state,created_at) VALUES ('permission-1','session-1','FILESYSTEM','bounded fixture','PENDING','2026-08-26T09:05:00Z'); INSERT INTO repositories (id,project_id,remote_url,created_at,updated_at) VALUES ('repo-activity','{project_id}',NULL,'2026-08-26T09:06:00Z','2026-08-26T09:06:00Z'); INSERT INTO git_snapshots (id,repository_id,status_json,captured_at) VALUES ('git-1','repo-activity','{{}}','2026-08-26T09:07:00Z'); INSERT INTO project_snapshots (id,project_id,availability,evidence_generated_at,watcher_health,created_at) VALUES ('snapshot-1','{project_id}','AVAILABLE','2026-08-26T09:08:00Z','HEALTHY','2026-08-26T09:08:00Z');")).unwrap();
        let first = snapshot(&database).unwrap();
        let second = snapshot(&database).unwrap();
        assert!(first
            .attention
            .iter()
            .all(|item| item.category == "ROOT_TASKS"));
        assert!(first
            .attention
            .iter()
            .all(|item| !matches!(item.category.as_str(), "TEST_RUN" | "AUDIT" | "PERMISSION")));
        assert!(first
            .work_queue
            .iter()
            .all(|item| !item.id.starts_with("agent:")));
        assert_eq!(first.kpis.needs_attention, Some(first.attention.len()));
        assert_eq!(
            first.kpis.running,
            Some(
                first
                    .work_queue
                    .iter()
                    .filter(|item| is_running_state(&item.state))
                    .count()
            )
        );
        assert!(first.engineering_brief.facts.iter().all(|fact| {
            fact.label != "Needs attention" || fact.provenance.source_class != "GITHUB_TASKS_ONLY"
        }));
        assert!(first
            .recent_activity
            .iter()
            .filter(|item| matches!(item.kind.as_str(), "AUDIT" | "TEST_RUN"))
            .all(|item| item.actor.is_none()));
        for kind in [
            "WORKFLOW",
            "AGENT_EVENT",
            "AGENT_SESSION",
            "AUDIT",
            "TEST_RUN",
            "GIT_SNAPSHOT",
            "PROJECT_SNAPSHOT",
        ] {
            assert_eq!(
                first
                    .recent_activity
                    .iter()
                    .filter(|item| item.kind == kind)
                    .count(),
                1,
                "missing or duplicate {kind}"
            );
        }
        assert_eq!(
            first
                .recent_activity
                .iter()
                .map(|item| &item.id)
                .collect::<Vec<_>>(),
            second
                .recent_activity
                .iter()
                .map(|item| &item.id)
                .collect::<Vec<_>>()
        );
        assert!(first.recent_activity.len() <= MAX_ACTIVITY_LIMIT);
    }

    #[test]
    fn x04_v05_root_tasks_normalizes_current_fields_and_authoritative_items() {
        let contents = "# X05-MILESTONE\n- [~] Canonical running task\n  Required actor: Codex\n  Next: Run the canonical action\n  Blocker: Canonical blocker\n- [x] Completed task\n";
        let (_db_dir, _project_dir, database, _project_id, _tasks) =
            fixture(contents, Some(canonical_manifest()));
        let current = snapshot(&database).unwrap();
        let project = &current.projects[0];
        assert_eq!(project.current_milestone.as_deref(), Some("X05-MILESTONE"));
        assert_eq!(project.required_actor.as_deref(), Some("Codex"));
        assert_eq!(project.blockers, vec!["Canonical blocker"]);
        assert_eq!(
            project.next_action.as_deref(),
            Some("Run the canonical action")
        );
        assert_eq!(project.authority_source, "TASKS.md");
        assert_eq!(project.reconciliation_state, "RESOLVED");
        assert!(current
            .attention
            .iter()
            .all(|item| item.category != "TEST_RUN" && item.category != "AUDIT"));
        assert_eq!(current.work_queue.len(), 1);
        assert_eq!(current.work_queue[0].task, "Canonical running task");
        assert_eq!(current.work_queue[0].actor.as_deref(), Some("Codex"));
        assert_eq!(
            current.work_queue[0].id,
            format!("root-tasks:queue:{}", project.project_id)
        );
    }

    #[test]
    fn x04_v05_root_tasks_blocked_truth_is_the_only_current_attention() {
        let contents = "# X05-MILESTONE\n- [!] Canonical blocked task\n  Required actor: Human\n  Next: Review the blocker\n  Blocker: Owner decision required\n";
        let (_db_dir, _project_dir, database, _project_id, _tasks) =
            fixture(contents, Some(canonical_manifest()));
        let current = snapshot(&database).unwrap();
        assert_eq!(current.attention.len(), 1);
        assert_eq!(current.attention[0].category, "ROOT_TASKS");
        assert_eq!(current.attention[0].state, "BLOCKED");
        assert_eq!(current.attention[0].detail, "Owner decision required");
        assert!(current.work_queue.is_empty());
        assert_eq!(current.kpis.needs_attention, Some(1));
    }

    #[test]
    fn x04_v05_degraded_summary_never_revives_control_plane() {
        let (_db_dir, _project_dir, database, project_id, _tasks) =
            fixture("# Work\n- [ ] task\n", Some(canonical_manifest()));
        let project = crate::projects::fetch_project(&database, &project_id).unwrap();
        let summary = degraded_project_summary(&project, "forced TASKS read failure");
        assert_eq!(summary.task_authority, "ROOT_TASKS_UNAVAILABLE");
        assert_eq!(summary.provenance_mode, "ROOT_TASKS_UNAVAILABLE");
        assert_eq!(summary.authority_source, "ROOT_TASKS_UNAVAILABLE");
        assert_eq!(summary.reconciliation_state, "NEEDS_RECONCILIATION");
        assert_eq!(
            summary.next_action.as_deref(),
            Some("Restore a readable repository-root TASKS.md")
        );
        assert!(summary.control_plane.is_none());
        assert!(!summary
            .warnings
            .iter()
            .any(|warning| warning.contains("CONTROL_PLANE")));
    }

    #[test]
    fn m11a_r08_warning_bounds_are_deterministic() {
        let mut project = Vec::new();
        for index in 0..(MAX_PROJECT_WARNINGS * 3) {
            push_warning(&mut project, format!("warning-{index}"));
        }
        assert_eq!(project.len(), MAX_PROJECT_WARNINGS);
        assert!(project.last().unwrap().starts_with("WARNING_LIMIT_REACHED"));
        let mut portfolio = Vec::new();
        for index in 0..(MAX_PORTFOLIO_WARNINGS * 3) {
            push_portfolio_warning(&mut portfolio, format!("warning-{index}"));
        }
        assert_eq!(portfolio.len(), MAX_PORTFOLIO_WARNINGS);
        assert!(portfolio
            .last()
            .unwrap()
            .starts_with("WARNING_LIMIT_REACHED"));
        assert!(portfolio
            .iter()
            .all(|warning| warning.len() <= MAX_WARNING_SCALAR_BYTES));
    }
}
