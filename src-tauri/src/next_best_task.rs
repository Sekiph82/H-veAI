use crate::agent_session_center::ProviderReadiness;
use crate::db::DatabaseState;
use crate::github_tracking;
use crate::projects::{list_projects, ProjectListQuery, ProjectRecord};
use crate::task_intelligence::{self, ParsedTask, TaskIntelligenceSnapshot};
use crate::time::utc_timestamp;
use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::Path;

pub const MAX_CANDIDATES: usize = 128;
pub const MAX_ATTENTION: usize = 64;
pub const MAX_FACTS: usize = 32;
const MAX_ROOT_TASK_BYTES: u64 = 2 * 1024 * 1024;
const RECENT_FAILURE_WINDOW: Duration = Duration::days(7);
const M19_FINGERPRINT_KEY: &str = "m19.engineering_brief.fingerprint";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M19Snapshot {
    pub generated_at: String,
    pub active_projects: usize,
    pub candidate_count: usize,
    pub recommended: Option<M19Recommendation>,
    pub alternatives: Vec<M19Recommendation>,
    pub attention: Vec<M19Attention>,
    pub facts: Vec<M19Fact>,
    pub unavailable_inputs: Vec<String>,
    pub comparison: M19Comparison,
    pub actor_readiness: Vec<M19ActorReadiness>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M19Comparison {
    pub state: String,
    pub previous_generated_at: Option<String>,
    pub changed: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M19ActorReadiness {
    pub actor: String,
    pub state: String,
    pub available: bool,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M19Recommendation {
    pub rank: usize,
    pub score_difference_from_top: i64,
    pub project_id: String,
    pub project_name: String,
    pub task_id: String,
    pub task_title: String,
    pub factual_state: String,
    pub eligibility_reason: String,
    pub score: i64,
    pub score_components: Vec<M19ScoreComponent>,
    pub dependencies: Vec<String>,
    pub blockers: Vec<String>,
    pub required_actor: Option<String>,
    pub actor_readiness: String,
    pub evidence: Vec<String>,
    pub uncertainty: Vec<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M19ScoreComponent {
    pub key: String,
    pub label: String,
    pub points: i64,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M19Attention {
    pub project_id: String,
    pub project_name: String,
    pub task_id: Option<String>,
    pub title: String,
    pub category: String,
    pub detail: String,
    pub evidence: Vec<String>,
    pub issue_key: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M19Fact {
    pub label: String,
    pub value: String,
    pub source: String,
    pub freshness: String,
}

#[derive(Debug, Clone)]
pub struct CandidateInput {
    pub project_id: String,
    pub project_name: String,
    pub project_priority: i64,
    pub task_id: String,
    /// Unique physical task-row identity used by the canonical graph.
    /// Explicit IDs remain aliases and are never sufficient as row keys.
    pub canonical_row_id: String,
    pub explicit_task_id: Option<String>,
    pub task_title: String,
    pub factual_state: String,
    pub task_priority: Option<i64>,
    pub dependencies: Vec<String>,
    pub dependency_task_ids: Vec<String>,
    pub blockers: Vec<String>,
    pub required_actor: Option<String>,
    pub external_wait: Option<String>,
    pub evidence: Vec<String>,
    pub evidence_freshness: String,
    pub evidence_uncertainty: Vec<String>,
    pub verified_failure_urgency: i64,
    pub failure_evidence: Vec<FailureEvidence>,
    pub context_switch_cost: i64,
    pub owner_focus_points: i64,
}

#[derive(Debug, Clone)]
pub struct FailureEvidence {
    pub evidence_id: String,
    pub source: String,
    pub result: String,
    pub occurred_at: String,
    pub age_seconds: i64,
    pub freshness: String,
}

impl FailureEvidence {
    fn summary(&self) -> String {
        format!(
            "{}#{}={} at {} ({}; age={}s)",
            self.source,
            self.evidence_id,
            self.result,
            self.occurred_at,
            self.freshness,
            self.age_seconds
        )
    }
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub input: CandidateInput,
    pub uncertainty: Vec<String>,
    pub actor_readiness: String,
    pub unblocks: i64,
}

#[derive(Debug, Default)]
pub struct CandidateSet {
    pub eligible: Vec<Candidate>,
    pub attention: Vec<M19Attention>,
    pub unavailable_inputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct ScoredCandidate {
    candidate: Candidate,
    score: i64,
    components: Vec<M19ScoreComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct M19Fingerprint {
    schema: u32,
    generated_at: String,
    active_projects: usize,
    candidate_count: usize,
    recommendation: Option<String>,
    attention: Vec<String>,
    actors: Vec<String>,
    fresh_failure_tasks: Vec<String>,
}

pub fn snapshot(database: &DatabaseState) -> Result<M19Snapshot, String> {
    let projects = list_projects(
        database,
        ProjectListQuery {
            include_archived: Some(false),
            ..Default::default()
        },
    )?;
    let readiness = crate::agent_session_center::readiness();
    let actor_readiness = actor_readiness_matrix(&readiness);
    let mut inputs = Vec::new();
    let mut attention = Vec::new();
    let mut unavailable_inputs = Vec::new();
    for project in projects.iter().filter(|project| project.status == "ACTIVE") {
        if github_tracking::is_github_tasks_project(project) {
            collect_remote_inputs(
                database,
                project,
                &mut inputs,
                &mut attention,
                &mut unavailable_inputs,
            );
        } else {
            collect_local_inputs(
                database,
                project,
                &mut inputs,
                &mut attention,
                &mut unavailable_inputs,
            );
        }
    }
    let mut candidates = build_candidate_set(inputs);
    apply_readiness(&mut candidates, &readiness);
    let scored = score_candidates(candidates.eligible);
    attention.extend(candidates.attention);
    attention.sort_by(|left, right| {
        left.project_id
            .cmp(&right.project_id)
            .then(left.task_id.cmp(&right.task_id))
            .then(left.category.cmp(&right.category))
            .then(left.title.cmp(&right.title))
    });
    attention.truncate(MAX_ATTENTION);
    unavailable_inputs.extend(candidates.unavailable_inputs);
    unavailable_inputs.sort();
    unavailable_inputs.dedup();
    unavailable_inputs.truncate(MAX_FACTS);
    let generated_at = utc_timestamp();
    let current_fingerprint = fingerprint(
        &generated_at,
        &projects,
        &scored,
        &attention,
        &actor_readiness,
    );
    let (comparison, comparison_error) = compare(database, &current_fingerprint);
    if let Some(error) = comparison_error {
        unavailable_inputs.push(format!(
            "M19 comparison persistence unavailable: {}",
            bounded(&error)
        ));
    }
    let top_score = scored.first().map(|candidate| candidate.score).unwrap_or(0);
    let recommendations = scored
        .iter()
        .take(MAX_CANDIDATES)
        .enumerate()
        .map(|(index, candidate)| to_recommendation(candidate, index + 1, top_score))
        .collect::<Vec<_>>();
    let recommended = recommendations.first().cloned();
    let alternatives = recommendations.iter().skip(1).take(3).cloned().collect();
    let active_projects = projects
        .iter()
        .filter(|project| project.status == "ACTIVE")
        .count();
    let mut facts = vec![
        M19Fact {
            label: "Active projects".into(),
            value: active_projects.to_string(),
            source: "Registry projects.status".into(),
            freshness: "CURRENT".into(),
        },
        M19Fact {
            label: "Eligible task candidates".into(),
            value: scored.len().to_string(),
            source: "M19 full canonical graph after scoring".into(),
            freshness: "CURRENT".into(),
        },
        M19Fact {
            label: "Attention items".into(),
            value: attention.len().to_string(),
            source: "M19 eligibility boundary".into(),
            freshness: "CURRENT".into(),
        },
        M19Fact {
            label: "Change since previous snapshot".into(),
            value: comparison.state.clone(),
            source: "Persisted M19 factual fingerprint".into(),
            freshness: if comparison.state.starts_with("UNAVAILABLE") {
                "UNAVAILABLE"
            } else {
                "CURRENT"
            }
            .into(),
        },
    ];
    for actor in &actor_readiness {
        facts.push(M19Fact {
            label: format!("{} readiness", actor.actor),
            value: actor.state.clone(),
            source: actor.evidence.clone(),
            freshness: "CURRENT".into(),
        });
    }
    for value in &unavailable_inputs {
        facts.push(M19Fact {
            label: "Unavailable input".into(),
            value: value.clone(),
            source: "Native evidence boundary".into(),
            freshness: "UNAVAILABLE".into(),
        });
    }
    facts.truncate(MAX_FACTS);
    Ok(M19Snapshot {
        generated_at,
        active_projects,
        candidate_count: scored.len(),
        recommended,
        alternatives,
        attention,
        facts,
        unavailable_inputs,
        comparison,
        actor_readiness,
    })
}

fn collect_local_inputs(
    database: &DatabaseState,
    project: &ProjectRecord,
    inputs: &mut Vec<CandidateInput>,
    attention: &mut Vec<M19Attention>,
    unavailable: &mut Vec<String>,
) {
    let root = match fs::canonicalize(Path::new(&project.normalized_path)) {
        Ok(root) => root,
        Err(error) => {
            unavailable.push(format!(
                "{} TASKS root unavailable: {}",
                project.name,
                bounded(&error.to_string())
            ));
            return;
        }
    };
    let tasks_path = root.join("TASKS.md");
    let (before_hash, _) = match read_root_tasks(&tasks_path) {
        Ok(value) => value,
        Err(error) => {
            unavailable.push(format!("{} TASKS.md {}", project.name, bounded(&error)));
            return;
        }
    };
    // Parse the canonical source at decision time; persisted M09 snapshots are telemetry, not authority.
    let parsed = match task_intelligence::parse_exact_root_tasks(&project.id, &project.name, &root)
    {
        Ok(value) => value,
        Err(error) => {
            unavailable.push(format!(
                "{} TASKS truth unavailable: {}",
                project.name,
                bounded(&error)
            ));
            return;
        }
    };
    let (after_hash, _) = match read_root_tasks(&tasks_path) {
        Ok(value) => value,
        Err(error) => {
            unavailable.push(format!(
                "{} TASKS.md changed or became unreadable: {}",
                project.name,
                bounded(&error)
            ));
            return;
        }
    };
    if before_hash != after_hash {
        unavailable.push(format!(
            "{} TASKS.md is STALE: changed during M19 decision",
            project.name
        ));
        return;
    }
    let root_tasks = parsed
        .tasks
        .iter()
        .filter(|task| is_exact_root_task(task) && task.evidence.content_hash == after_hash)
        .cloned()
        .collect::<Vec<_>>();
    if parsed
        .tasks
        .iter()
        .any(|task| is_exact_root_task(task) && task.evidence.content_hash != after_hash)
    {
        unavailable.push(format!(
            "{} TASKS.md is STALE: parsed evidence hash does not match current root",
            project.name
        ));
        return;
    }
    if parsed.warnings.iter().any(|warning| {
        warning
            .source_path
            .as_deref()
            .is_some_and(is_exact_root_path)
            && matches!(
                warning.code.as_str(),
                "SOURCE_READ_FAILED"
                    | "SOURCE_CHANGED_DURING_PARSE"
                    | "INVALID_UTF8"
                    | "MALFORMED_PRIORITY"
            )
    }) {
        unavailable.push(format!(
            "{} TASKS.md is MALFORMED or unreadable",
            project.name
        ));
        return;
    }
    collect_parsed_inputs(
        database,
        project,
        &root_tasks,
        &parsed,
        inputs,
        attention,
        unavailable,
        &after_hash,
    );
}

fn collect_parsed_inputs(
    database: &DatabaseState,
    project: &ProjectRecord,
    tasks: &[ParsedTask],
    _snapshot: &TaskIntelligenceSnapshot,
    inputs: &mut Vec<CandidateInput>,
    _attention: &mut Vec<M19Attention>,
    unavailable: &mut Vec<String>,
    root_hash: &str,
) {
    let explicit_ids: HashMap<String, Vec<String>> = tasks
        .iter()
        .filter_map(|task| {
            task.explicit_task_id
                .as_ref()
                .map(|id| (id.trim().to_ascii_lowercase(), task.id.clone()))
        })
        .fold(HashMap::new(), |mut map, (key, value)| {
            map.entry(key).or_default().push(value);
            map
        });
    for task in tasks {
        let dependencies = task.dependency_references.clone();
        let dependency_task_ids = dependencies
            .iter()
            .filter_map(|dependency| {
                let matches = explicit_ids.get(&dependency.trim().to_ascii_lowercase())?;
                (matches.len() == 1).then(|| matches[0].clone())
            })
            .collect::<Vec<_>>();
        let blockers = task.blockers.clone();
        let mut uncertainty = Vec::new();
        let (failure_evidence, failure_error) =
            match recent_failure_evidence(database, &project.id, &task.id) {
                Ok(value) => (value, None),
                Err(error) => (Vec::new(), Some(error)),
            };
        let verified_failure_urgency = verified_failure_urgency(&failure_evidence);
        if let Some(error) = failure_error {
            uncertainty.push(format!(
                "linked failure evidence unavailable: {}",
                bounded(&error)
            ));
            unavailable.push(format!(
                "{} {} failure evidence unavailable",
                project.name, task.id
            ));
        }
        let input = CandidateInput {
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            project_priority: project.priority,
            task_id: task.id.clone(),
            canonical_row_id: task.id.clone(),
            explicit_task_id: task.explicit_task_id.clone(),
            task_title: task.title.clone(),
            factual_state: task.parsed_status.clone(),
            task_priority: task.priority,
            dependencies,
            dependency_task_ids,
            blockers,
            required_actor: task.required_actor.clone(),
            external_wait: task
                .external_wait
                .clone()
                .or_else(|| task.owner_gate.clone()),
            evidence: vec![format!(
                "{}:{}-{} sha256:{}",
                task.evidence.source_path,
                task.evidence.start_line,
                task.evidence.end_line,
                root_hash
            )],
            evidence_freshness: "CURRENT".into(),
            evidence_uncertainty: uncertainty,
            verified_failure_urgency,
            failure_evidence,
            context_switch_cost: if task.parsed_status.eq_ignore_ascii_case("IN_PROGRESS") {
                0
            } else {
                5
            },
            owner_focus_points: 0,
        };
        inputs.push(input);
    }
}

fn collect_remote_inputs(
    database: &DatabaseState,
    project: &ProjectRecord,
    inputs: &mut Vec<CandidateInput>,
    attention: &mut Vec<M19Attention>,
    unavailable: &mut Vec<String>,
) {
    let remote = github_tracking::cached_snapshot(database, project)
        .ok()
        .flatten();
    let Some(remote) = remote else {
        unavailable.push(format!(
            "{} GitHub TASKS snapshot is UNAVAILABLE",
            project.name
        ));
        return;
    };
    let registry_repository = project.repository.as_ref().and_then(|repository| {
        Some(format!(
            "{}/{}",
            repository.github_owner.as_deref()?,
            repository.github_repo.as_deref()?
        ))
    });
    let expected_branch = project.repository.as_ref().and_then(|repository| {
        repository
            .default_branch
            .as_deref()
            .or(repository.current_branch.as_deref())
    });
    let now = DateTime::parse_from_rfc3339(&utc_timestamp())
        .ok()
        .map(|value| value.with_timezone(&Utc));
    let fresh = remote_validation_is_fresh(&remote, now);
    if remote.remote_health != "CURRENT"
        || !registry_repository
            .as_deref()
            .is_some_and(|expected| expected.eq_ignore_ascii_case(&remote.repository))
        || !expected_branch.is_some_and(|expected| expected == remote.branch)
        || !fresh
        || remote.remote_head.as_deref().is_none_or(str::is_empty)
        || remote.tasks_blob_sha.as_deref().is_none_or(str::is_empty)
    {
        unavailable.push(format!(
            "{} GitHub evidence is UNAVAILABLE (identity, branch, HEAD, root hash, health, or freshness proof failed)",
            project.name
        ));
        return;
    }
    if remote.task_rows.is_empty() {
        if remote.total_tasks != Some(0) {
            unavailable.push(format!(
                "{} GitHub TASKS rows are UNAVAILABLE",
                project.name
            ));
        }
        return;
    }
    for task in remote.task_rows {
        let evidence = vec![format!(
            "GitHub {}/{}:{} {} head:{}",
            remote.repository,
            task.source_path,
            task.source_line,
            task.content_hash.as_deref().unwrap_or("unavailable"),
            remote.remote_head.as_deref().unwrap_or("unavailable")
        )];
        if !task.metadata_complete
            || task.content_hash.as_deref() != remote.tasks_blob_sha.as_deref()
        {
            unavailable.push(format!(
                "{} task {} per-task TASKS evidence is incomplete or stale",
                project.name, task.id
            ));
            attention.push(M19Attention {
                project_id: project.id.clone(),
                project_name: project.name.clone(),
                task_id: Some(task.id.clone()),
                title: task.title.clone(),
                category: "REMOTE_TASK_EVIDENCE_UNAVAILABLE".into(),
                detail:
                    "Per-task actor/dependency/blocker/owner evidence or root hash is unavailable."
                        .into(),
                evidence,
                issue_key: format!(
                    "{}:{}:{}",
                    project.id,
                    task.id,
                    normalize_issue_text(&task.title)
                ),
            });
            continue;
        }
        let blockers = task.blockers.clone();
        let (failure_evidence, failure_error) =
            match recent_failure_evidence(database, &project.id, &task.id) {
                Ok(value) => (value, None),
                Err(error) => (Vec::new(), Some(error)),
            };
        let verified_failure_urgency = verified_failure_urgency(&failure_evidence);
        let mut evidence_uncertainty = Vec::new();
        if let Some(error) = failure_error {
            evidence_uncertainty.push(format!(
                "linked failure evidence unavailable: {}",
                bounded(&error)
            ));
            unavailable.push(format!(
                "{} {} failure evidence unavailable",
                project.name, task.id
            ));
        }
        inputs.push(CandidateInput {
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            project_priority: project.priority,
            task_id: task.id.clone(),
            canonical_row_id: if task.canonical_row_id.is_empty() {
                format!("{}:{}:{}", project.id, task.source_line, task.id)
            } else {
                task.canonical_row_id.clone()
            },
            explicit_task_id: Some(task.id.clone()),
            task_title: task.title.clone(),
            factual_state: task.status.clone(),
            task_priority: task.priority,
            dependencies: task.dependencies.clone(),
            dependency_task_ids: task.dependencies.clone(),
            blockers,
            required_actor: task.required_actor.clone(),
            external_wait: task.external_wait.clone().or(task.owner_gate.clone()),
            evidence,
            evidence_freshness: "CURRENT".into(),
            evidence_uncertainty,
            verified_failure_urgency,
            failure_evidence,
            context_switch_cost: if task.status.eq_ignore_ascii_case("IN_PROGRESS") {
                0
            } else {
                5
            },
            owner_focus_points: 0,
        });
    }
}

fn remote_validation_is_fresh(
    remote: &github_tracking::RemoteTrackingSnapshot,
    now: Option<DateTime<Utc>>,
) -> bool {
    let Some(validated_at) = remote.validated_at.as_deref() else {
        return false;
    };
    DateTime::parse_from_rfc3339(validated_at)
        .ok()
        .map(|value| value.with_timezone(&Utc))
        .zip(now)
        .is_some_and(|(validated, now)| {
            now.signed_duration_since(validated) >= Duration::zero()
                && now.signed_duration_since(validated)
                    <= Duration::seconds(
                        github_tracking::validation_horizon_seconds(false) as i64,
                    )
        })
}

fn build_candidate_set(inputs: Vec<CandidateInput>) -> CandidateSet {
    let mut set = CandidateSet::default();
    let all_states = inputs
        .iter()
        .map(|input| {
            (
                (input.project_id.clone(), input.canonical_row_id.clone()),
                input.factual_state.clone(),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut canonical_ids: HashMap<(String, String), Vec<String>> = HashMap::new();
    for input in &inputs {
        for alias in [
            Some(input.task_id.as_str()),
            input.explicit_task_id.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            let ids = canonical_ids
                .entry((input.project_id.clone(), alias.trim().to_ascii_lowercase()))
                .or_default();
            if !ids.contains(&input.canonical_row_id) {
                ids.push(input.canonical_row_id.clone());
            }
        }
    }
    let mut normalized = Vec::with_capacity(inputs.len());
    for mut input in inputs {
        let dependency_refs = if input.dependencies.is_empty() {
            input.dependency_task_ids.clone()
        } else {
            input.dependencies.clone()
        };
        input.dependency_task_ids.clear();
        let mut derived_blockers = Vec::new();
        let mut normalized_dependencies = HashSet::new();
        for dependency in dependency_refs {
            let normalized_dependency = dependency.trim().to_ascii_lowercase();
            if !normalized_dependencies.insert(normalized_dependency.clone()) {
                continue;
            }
            let matches = canonical_ids.get(&(input.project_id.clone(), normalized_dependency));
            match matches {
                Some(ids) if ids.len() == 1 => {
                    let prerequisite = ids[0].clone();
                    input.dependency_task_ids.push(prerequisite.clone());
                    if all_states
                        .get(&(input.project_id.clone(), prerequisite.clone()))
                        .is_some_and(|state| !is_completed(state))
                    {
                        derived_blockers.push(format!(
                            "dependency {dependency} is unfinished from canonical task state"
                        ));
                    }
                }
                Some(_) => derived_blockers.push(format!(
                    "dependency {dependency} is ambiguous in canonical TASKS.md"
                )),
                None => derived_blockers.push(format!(
                    "dependency {dependency} is not evidenced in canonical TASKS.md"
                )),
            }
        }
        let own_alias = input
            .explicit_task_id
            .as_deref()
            .unwrap_or(input.task_id.as_str())
            .trim()
            .to_ascii_lowercase();
        if canonical_ids
            .get(&(input.project_id.clone(), own_alias))
            .is_some_and(|rows| rows.len() > 1)
        {
            derived_blockers.push(format!(
                "explicit task ID {} is ambiguous in canonical TASKS.md",
                input.explicit_task_id.as_deref().unwrap_or(&input.task_id)
            ));
        }
        let generated_blocker_count = derived_blockers.len();
        input.blockers.extend(derived_blockers);
        normalized.push((input, generated_blocker_count));
    }
    let mut dependent_counts: HashMap<(String, String), HashSet<(String, String)>> = HashMap::new();
    for (dependent, generated_blocker_count) in &normalized {
        let has_independent_blocker = dependent.blockers.len() > *generated_blocker_count;
        if is_completed(&dependent.factual_state)
            || has_independent_blocker
            || dependent
                .external_wait
                .as_deref()
                .is_some_and(|wait| !wait.trim().is_empty())
            || !matches!(
                normalize_actor(dependent.required_actor.as_deref()).as_deref(),
                Some("CODEX") | Some("CLAUDE")
            )
        {
            continue;
        }
        let unmet = dependent
            .dependency_task_ids
            .iter()
            .filter(|prerequisite| {
                all_states
                    .get(&(dependent.project_id.clone(), prerequisite.to_string()))
                    .is_some_and(|state| !is_completed(state))
            })
            .collect::<HashSet<_>>();
        if unmet.len() != 1 {
            continue;
        }
        let prerequisite = unmet.into_iter().next().unwrap();
        dependent_counts
            .entry((dependent.project_id.clone(), prerequisite.to_string()))
            .or_default()
            .insert((
                dependent.project_id.clone(),
                dependent.canonical_row_id.clone(),
            ));
    }
    for (input, _) in normalized {
        if is_completed(&input.factual_state) {
            continue;
        }
        let key = (input.project_id.clone(), input.canonical_row_id.clone());
        let candidate = Candidate {
            unblocks: dependent_counts
                .get(&key)
                .map(|items| items.len() as i64)
                .unwrap_or_default(),
            actor_readiness: normalize_actor(input.required_actor.as_deref())
                .unwrap_or_else(|| "UNKNOWN".into()),
            uncertainty: input.evidence_uncertainty.clone(),
            input,
        };
        defer_or_retain(candidate, &mut set);
    }
    set
}

fn defer_or_retain(candidate: Candidate, set: &mut CandidateSet) {
    let input = &candidate.input;
    if !candidate.uncertainty.is_empty() || input.evidence_freshness != "CURRENT" {
        let mut detail = candidate.uncertainty.join("; ");
        if input.evidence_freshness != "CURRENT" {
            detail.push_str(&format!(" evidence is {}", input.evidence_freshness));
        }
        set.attention
            .push(attention_for(&candidate, "EVIDENCE_UNCERTAIN", detail));
        return;
    }
    if !input.blockers.is_empty() {
        set.attention.push(attention_for(
            &candidate,
            "BLOCKED_OR_DEPENDENCY",
            input.blockers.join("; "),
        ));
        return;
    }
    if let Some(wait) = input
        .external_wait
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        set.attention.push(attention_for(
            &candidate,
            "HUMAN_OR_EXTERNAL_WAIT",
            bounded(wait),
        ));
        return;
    }
    match normalize_actor(input.required_actor.as_deref()).as_deref() {
        Some("HUMAN") | Some("EXTERNAL") => set.attention.push(attention_for(
            &candidate,
            "HUMAN_OR_EXTERNAL_WAIT",
            format!(
                "required actor: {}",
                input.required_actor.as_deref().unwrap_or("unknown")
            ),
        )),
        Some("CI") | Some("GPT_AUDIT") => set.attention.push(attention_for(
            &candidate,
            "NON_EXECUTABLE_ACTOR",
            format!(
                "required actor {} is not an immediate builder",
                input.required_actor.as_deref().unwrap_or("unknown")
            ),
        )),
        Some("CODEX") | Some("CLAUDE") => set.eligible.push(candidate),
        _ => set.attention.push(attention_for(
            &candidate,
            "ACTOR_UNKNOWN",
            "required actor is unavailable or unknown".into(),
        )),
    }
}

fn attention_for(candidate: &Candidate, category: &str, detail: String) -> M19Attention {
    let issue_key = format!(
        "{}:{}:{}",
        candidate.input.project_id,
        candidate.input.task_id,
        normalize_issue_text(&detail)
    );
    M19Attention {
        project_id: candidate.input.project_id.clone(),
        project_name: candidate.input.project_name.clone(),
        task_id: Some(candidate.input.task_id.clone()),
        title: candidate.input.task_title.clone(),
        category: category.into(),
        detail,
        evidence: candidate.input.evidence.clone(),
        issue_key,
    }
}

fn normalize_issue_text(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn apply_readiness(set: &mut CandidateSet, readiness: &[ProviderReadiness]) {
    let states = readiness
        .iter()
        .map(|value| (value.provider.to_ascii_uppercase(), value))
        .collect::<HashMap<_, _>>();
    let mut keep = Vec::new();
    for mut candidate in set.eligible.drain(..) {
        let actor = normalize_actor(candidate.input.required_actor.as_deref())
            .unwrap_or_else(|| "UNKNOWN".into());
        match actor.as_str() {
            "CODEX" | "CLAUDE" => match states.get(actor.as_str()) {
                Some(provider) if provider.available => {
                    candidate.actor_readiness = format!("{} / AVAILABLE", actor);
                    keep.push(candidate);
                }
                Some(provider) => set.attention.push(attention_for(
                    &candidate,
                    "ACTOR_UNAVAILABLE",
                    provider
                        .diagnostic_message
                        .clone()
                        .unwrap_or_else(|| format!("{actor} is {}", provider.readiness_state)),
                )),
                None => set.attention.push(attention_for(
                    &candidate,
                    "ACTOR_UNAVAILABLE",
                    format!("{actor} readiness is unavailable"),
                )),
            },
            _ => set.attention.push(attention_for(
                &candidate,
                "ACTOR_UNKNOWN",
                "actor is not executable by the selected builder path".into(),
            )),
        }
    }
    set.eligible = keep;
}

fn score_candidates(candidates: Vec<Candidate>) -> Vec<ScoredCandidate> {
    let mut scored = candidates
        .into_iter()
        .map(|candidate| {
            let input = &candidate.input;
            let task_priority_points = input.task_priority.unwrap_or_default().clamp(-100, 100) * 5;
            let components = vec![
                M19ScoreComponent {
                    key: "project_priority".into(),
                    label: "Explicit project priority".into(),
                    points: input.project_priority.clamp(-100, 100) * 10,
                    evidence: "Registry priority".into(),
                },
                M19ScoreComponent {
                    key: "task_priority".into(),
                    label: "Authoritative task priority".into(),
                    points: task_priority_points,
                    evidence: input
                        .task_priority
                        .map(|_| "Canonical TASKS.md priority".into())
                        .unwrap_or_else(|| "No authoritative task priority".into()),
                },
                M19ScoreComponent {
                    key: "dependency_unlock".into(),
                    label: "Dependency unlock".into(),
                    points: candidate.unblocks.clamp(0, 20) * 15,
                    evidence: format!(
                        "{} unique unfinished canonical dependents",
                        candidate.unblocks
                    ),
                },
                M19ScoreComponent {
                    key: "verified_failure".into(),
                    label: "Fresh verified failure".into(),
                    points: input.verified_failure_urgency.clamp(0, 100),
                    evidence: if input.failure_evidence.is_empty() {
                        "No linked audit/test evidence for this task".into()
                    } else {
                        input
                            .failure_evidence
                            .iter()
                            .map(FailureEvidence::summary)
                            .collect::<Vec<_>>()
                            .join(" | ")
                    },
                },
                M19ScoreComponent {
                    key: "actor_readiness".into(),
                    label: "Required actor readiness".into(),
                    points: if input.required_actor.is_some() {
                        20
                    } else {
                        0
                    },
                    evidence: candidate.actor_readiness.clone(),
                },
                M19ScoreComponent {
                    key: "context_switch_cost".into(),
                    label: "Bounded context-switch cost".into(),
                    points: -input.context_switch_cost.clamp(0, 20),
                    evidence: "Factual task state; IN_PROGRESS has zero switch cost".into(),
                },
                M19ScoreComponent {
                    key: "owner_focus".into(),
                    label: "Authoritative owner focus".into(),
                    points: input.owner_focus_points.clamp(-100, 100),
                    evidence: "No owner-focus setting is configured".into(),
                },
                M19ScoreComponent {
                    key: "uncertainty_penalty".into(),
                    label: "Evidence uncertainty".into(),
                    points: -(candidate.uncertainty.len() as i64 * 10),
                    evidence: if candidate.uncertainty.is_empty() {
                        "No recorded uncertainty".into()
                    } else {
                        candidate.uncertainty.join("; ")
                    },
                },
            ];
            let score = components.iter().map(|component| component.points).sum();
            ScoredCandidate {
                candidate,
                score,
                components,
            }
        })
        .collect::<Vec<_>>();
    scored.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then(tie_key(&left.candidate).cmp(&tie_key(&right.candidate)))
            .then(
                left.candidate
                    .input
                    .project_id
                    .cmp(&right.candidate.input.project_id),
            )
            .then(
                left.candidate
                    .input
                    .task_id
                    .cmp(&right.candidate.input.task_id),
            )
    });
    scored
}

fn to_recommendation(scored: &ScoredCandidate, rank: usize, top_score: i64) -> M19Recommendation {
    let input = &scored.candidate.input;
    let difference = top_score.saturating_sub(scored.score);
    let positive = scored
        .components
        .iter()
        .filter(|component| component.points > 0)
        .map(|component| format!("{} +{}", component.label, component.points))
        .collect::<Vec<_>>();
    let explanation = if rank == 1 {
        if positive.is_empty() {
            "Ranked first among the currently eligible candidates; no favorable evidence inputs were available.".into()
        } else {
            format!(
                "Ranked first from the same factual score object: {}.",
                positive.join(", ")
            )
        }
    } else if difference == 0 {
        format!(
            "Ranked #{rank}; tied with rank 1 at score {} from the same factual score object.",
            scored.score
        )
    } else {
        format!(
            "Ranked #{rank}, {} points below rank 1 from the same factual score object: {}.",
            difference,
            positive.join(", ")
        )
    };
    M19Recommendation { rank, score_difference_from_top: difference, project_id: input.project_id.clone(), project_name: input.project_name.clone(), task_id: input.task_id.clone(), task_title: input.task_title.clone(), factual_state: input.factual_state.clone(), eligibility_reason: "ACTIVE canonical TASKS task with no evidenced blocker, dependency, human/external wait, or unavailable actor".into(), score: scored.score, score_components: scored.components.clone(), dependencies: input.dependencies.clone(), blockers: input.blockers.clone(), required_actor: input.required_actor.clone(), actor_readiness: scored.candidate.actor_readiness.clone(), evidence: input.evidence.clone(), uncertainty: scored.candidate.uncertainty.clone(), explanation }
}

fn actor_readiness_matrix(readiness: &[ProviderReadiness]) -> Vec<M19ActorReadiness> {
    let mut out = readiness
        .iter()
        .map(|provider| M19ActorReadiness {
            actor: provider.provider.clone(),
            state: if provider.available {
                "AVAILABLE"
            } else {
                provider.readiness_state.as_str()
            }
            .into(),
            available: provider.available,
            evidence: provider
                .diagnostic_message
                .clone()
                .unwrap_or_else(|| "Native provider readiness probe".into()),
        })
        .collect::<Vec<_>>();
    for actor in ["HUMAN", "CI", "GPT_AUDIT", "EXTERNAL", "UNKNOWN"] {
        out.push(M19ActorReadiness {
            actor: actor.into(),
            state: "NON_EXECUTABLE_OR_UNAVAILABLE".into(),
            available: false,
            evidence: "Canonical actor semantics; not an immediate builder provider".into(),
        });
    }
    out
}

fn normalize_actor(value: Option<&str>) -> Option<String> {
    let value = value?.trim().to_ascii_uppercase().replace(' ', "_");
    match value.as_str() {
        "CODEX" | "CLAUDE" | "HUMAN" | "CI" | "GPT_AUDIT" | "GPTAUDIT" | "EXTERNAL" => {
            Some(if value == "GPTAUDIT" {
                "GPT_AUDIT".into()
            } else {
                value
            })
        }
        _ => None,
    }
}

fn recent_failure_evidence(
    database: &DatabaseState,
    project_id: &str,
    task_id: &str,
) -> Result<Vec<FailureEvidence>, String> {
    let connection = database.open_connection()?;
    let now = DateTime::parse_from_rfc3339(&utc_timestamp())
        .map_err(|error| format!("invalid current timestamp: {error}"))?
        .with_timezone(&Utc);
    let mut evidence = Vec::new();
    for (table, timestamp) in [
        ("audits", "created_at"),
        ("test_runs", "COALESCE(finished_at, started_at)"),
    ] {
        let sql = format!("SELECT id, result, {timestamp} FROM {table} WHERE project_id=?1 AND task_id=?2 ORDER BY {timestamp} DESC LIMIT 1");
        let latest: Option<(String, String, Option<String>)> = connection
            .query_row(&sql, params![project_id, task_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .optional()
            .map_err(|error| format!("{table} failure evidence query failed: {error}"))?;
        let Some((evidence_id, result, at)) = latest else {
            continue;
        };
        let at = at.ok_or_else(|| format!("{table} failure timestamp is missing"))?;
        let parsed = DateTime::parse_from_rfc3339(&at)
            .map_err(|error| format!("{table} failure timestamp is malformed: {error}"))?
            .with_timezone(&Utc);
        let age = now.signed_duration_since(parsed);
        let result = result.trim().to_ascii_uppercase();
        let freshness = if age >= Duration::zero() && age <= RECENT_FAILURE_WINDOW {
            "CURRENT"
        } else {
            "HISTORICAL"
        };
        evidence.push(FailureEvidence {
            evidence_id,
            source: table.into(),
            result,
            occurred_at: at,
            age_seconds: age.num_seconds().max(0),
            freshness: freshness.into(),
        });
    }
    Ok(evidence)
}

fn verified_failure_urgency(evidence: &[FailureEvidence]) -> i64 {
    let latest = evidence
        .iter()
        .max_by(|left, right| left.occurred_at.cmp(&right.occurred_at));
    if latest.is_some_and(|item| {
        item.freshness == "CURRENT" && matches!(item.result.as_str(), "FAIL" | "FAILED" | "ERROR")
    }) {
        100
    } else {
        0
    }
}

fn is_exact_root_task(task: &ParsedTask) -> bool {
    task.source_kind.eq_ignore_ascii_case("TASKS") && is_exact_root_path(&task.source_path)
}
fn is_exact_root_path(path: &str) -> bool {
    path.replace('\\', "/").eq_ignore_ascii_case("TASKS.md")
}

fn read_root_tasks(path: &Path) -> Result<(String, String), String> {
    let metadata = fs::metadata(path).map_err(|error| format!("is unavailable: {error}"))?;
    if !metadata.is_file() {
        return Err("is not a regular file".into());
    }
    if metadata.len() > MAX_ROOT_TASK_BYTES {
        return Err("exceeds the canonical source size bound".into());
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|error| format!("cannot be read: {error}"))?
        .take(MAX_ROOT_TASK_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot be read: {error}"))?;
    if bytes.len() as u64 > MAX_ROOT_TASK_BYTES {
        return Err("exceeds the canonical source size bound".into());
    }
    let hash = hex_digest(&bytes);
    let text = String::from_utf8(bytes).map_err(|_| "contains invalid UTF-8".to_string())?;
    if text.trim().is_empty() {
        return Err("is empty".into());
    }
    Ok((hash, text))
}

fn compare(database: &DatabaseState, current: &M19Fingerprint) -> (M19Comparison, Option<String>) {
    let connection = match database.open_connection() {
        Ok(connection) => connection,
        Err(error) => {
            return (
                M19Comparison {
                    state: "UNAVAILABLE".into(),
                    previous_generated_at: None,
                    changed: Vec::new(),
                },
                Some(error),
            )
        }
    };
    let previous: Option<String> = match connection
        .query_row(
            "SELECT value_json FROM settings WHERE key=?1 AND scope='WORKSPACE'",
            [M19_FINGERPRINT_KEY],
            |row| row.get(0),
        )
        .optional()
    {
        Ok(value) => value,
        Err(error) => {
            return (
                M19Comparison {
                    state: "UNAVAILABLE".into(),
                    previous_generated_at: None,
                    changed: Vec::new(),
                },
                Some(format!("read persisted fingerprint failed: {error}")),
            )
        }
    };
    let comparison = match previous.as_deref() {
        None => M19Comparison {
            state: "UNAVAILABLE_FIRST_SNAPSHOT".into(),
            previous_generated_at: None,
            changed: Vec::new(),
        },
        Some(value) => {
            let previous = match serde_json::from_str::<M19Fingerprint>(value) {
                Ok(previous) if previous.schema == 1 => previous,
                Ok(previous) => {
                    return (
                        M19Comparison {
                            state: "UNAVAILABLE_INCOMPATIBLE_SCHEMA".into(),
                            previous_generated_at: Some(previous.generated_at),
                            changed: Vec::new(),
                        },
                        None,
                    )
                }
                Err(_) => {
                    return (
                        M19Comparison {
                            state: "UNAVAILABLE_INCOMPATIBLE_SCHEMA".into(),
                            previous_generated_at: None,
                            changed: Vec::new(),
                        },
                        None,
                    )
                }
            };
            let mut changed = Vec::new();
            if previous.recommendation != current.recommendation {
                changed.push("recommendation identity".into());
            }
            if previous.attention != current.attention {
                changed.push("attention set/state".into());
            }
            if previous.active_projects != current.active_projects
                || previous.candidate_count != current.candidate_count
            {
                changed.push("portfolio counts".into());
            }
            if previous.actors != current.actors {
                changed.push("provider/actor state".into());
            }
            if previous.fresh_failure_tasks != current.fresh_failure_tasks {
                changed.push("fresh failure identities".into());
            }
            M19Comparison {
                state: if changed.is_empty() {
                    "NO_COMPARABLE_CHANGE".into()
                } else {
                    "CHANGED".into()
                },
                previous_generated_at: Some(previous.generated_at),
                changed,
            }
        }
    };
    (comparison, None)
}

/// Capture the factual pre-refresh snapshot, await the scoped refresh, then
/// persist that captured snapshot exactly once before computing the current
/// comparison. A failed refresh never creates a misleading baseline.
pub fn refresh_and_compare<F>(
    database: &DatabaseState,
    refresh: F,
) -> Result<M19Snapshot, String>
where
    F: FnOnce() -> Result<(), String>,
{
    let previous = snapshot(database)?;
    refresh()?;
    record_history(database, &previous)?;
    snapshot(database)
}

/// Explicit history mutation, kept separate from observational snapshots.
pub fn record_history(database: &DatabaseState, snapshot: &M19Snapshot) -> Result<(), String> {
    let fingerprint = M19Fingerprint {
        schema: 1,
        generated_at: snapshot.generated_at.clone(),
        active_projects: snapshot.active_projects,
        candidate_count: snapshot.candidate_count,
        recommendation: snapshot
            .recommended
            .as_ref()
            .map(|item| format!("{}:{}", item.project_id, item.task_id)),
        attention: snapshot
            .attention
            .iter()
            .map(|item| {
                format!(
                    "{}:{}:{}:{}",
                    item.project_id,
                    item.task_id.as_deref().unwrap_or(""),
                    item.category,
                    item.detail
                )
            })
            .collect(),
        actors: snapshot
            .actor_readiness
            .iter()
            .map(|item| format!("{}:{}:{}", item.actor, item.state, item.available))
            .collect(),
        fresh_failure_tasks: snapshot
            .recommended
            .iter()
            .chain(snapshot.alternatives.iter())
            .filter(|item| {
                item.score_components
                    .iter()
                    .any(|component| component.key == "verified_failure" && component.points > 0)
            })
            .map(|item| format!("{}:{}", item.project_id, item.task_id))
            .collect(),
    };
    let json = serde_json::to_string(&fingerprint)
        .map_err(|error| format!("serialize fingerprint: {error}"))?;
    let connection = database.open_connection()?;
    connection.execute("INSERT INTO settings (key,value_json,scope,created_at,updated_at) VALUES (?1,?2,'WORKSPACE',?3,?3) ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json,updated_at=excluded.updated_at", params![M19_FINGERPRINT_KEY, json, snapshot.generated_at]).map_err(|error| format!("persist fingerprint failed: {error}"))?;
    Ok(())
}

fn fingerprint(
    generated_at: &str,
    projects: &[ProjectRecord],
    scored: &[ScoredCandidate],
    attention: &[M19Attention],
    actors: &[M19ActorReadiness],
) -> M19Fingerprint {
    M19Fingerprint {
        schema: 1,
        generated_at: generated_at.into(),
        active_projects: projects
            .iter()
            .filter(|project| project.status == "ACTIVE")
            .count(),
        candidate_count: scored.len(),
        recommendation: scored.first().map(|candidate| {
            format!(
                "{}:{}",
                candidate.candidate.input.project_id, candidate.candidate.input.task_id
            )
        }),
        attention: attention
            .iter()
            .map(|item| {
                format!(
                    "{}:{}:{}:{}",
                    item.project_id,
                    item.task_id.as_deref().unwrap_or(""),
                    item.category,
                    item.detail
                )
            })
            .collect(),
        actors: actors
            .iter()
            .map(|actor| format!("{}:{}:{}", actor.actor, actor.state, actor.available))
            .collect(),
        fresh_failure_tasks: scored
            .iter()
            .filter(|candidate| candidate.candidate.input.verified_failure_urgency > 0)
            .map(|candidate| {
                format!(
                    "{}:{}",
                    candidate.candidate.input.project_id, candidate.candidate.input.task_id
                )
            })
            .collect(),
    }
}

fn tie_key(candidate: &Candidate) -> String {
    hex_digest(format!("{}|{}", candidate.input.project_id, candidate.input.task_id).as_bytes())
}
fn is_completed(status: &str) -> bool {
    matches!(
        status.trim().to_ascii_uppercase().as_str(),
        "COMPLETED"
            | "COMPLETE"
            | "DONE"
            | "CLOSED"
            | "PASS"
            | "PASSED"
            | "CANCELLED"
            | "TASK_COMPLETE"
            | "[X]"
            | "X"
    )
}
fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn bounded(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_ascii_control() || *character == '\n')
        .take(512)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use crate::task_intelligence::{ParserAdapterIdentity, TaskConfidence, TaskEvidenceLocator};
    use std::fs;
    use tempfile::tempdir;

    fn input(project: &str, task: &str) -> CandidateInput {
        CandidateInput {
            project_id: project.into(),
            project_name: project.into(),
            project_priority: 1,
            task_id: task.into(),
            canonical_row_id: task.into(),
            explicit_task_id: Some(task.into()),
            task_title: format!("{task} title"),
            factual_state: "PLANNED".into(),
            task_priority: None,
            dependencies: Vec::new(),
            dependency_task_ids: Vec::new(),
            blockers: Vec::new(),
            required_actor: Some("CODEX".into()),
            external_wait: None,
            evidence: vec!["TASKS.md:1".into()],
            evidence_freshness: "CURRENT".into(),
            evidence_uncertainty: Vec::new(),
            verified_failure_urgency: 0,
            failure_evidence: Vec::new(),
            context_switch_cost: 0,
            owner_focus_points: 0,
        }
    }

    fn parsed(project: &str, path: &str, kind: &str, id: &str, hash: &str) -> ParsedTask {
        ParsedTask {
            id: format!("internal-{id}"),
            project_id: project.into(),
            source_id: "source".into(),
            source_path: path.into(),
            source_kind: kind.into(),
            title: id.into(),
            priority: None,
            parsed_status: "OPEN".into(),
            storage_state: "BACKLOG".into(),
            explicit_task_id: Some(id.into()),
            milestone: None,
            required_actor: Some("Codex".into()),
            blockers: Vec::new(),
            dependency_references: Vec::new(),
            next_step: None,
            owner_gate: None,
            external_wait: None,
            acceptance_criteria: Vec::new(),
            confidence: TaskConfidence {
                score: 1.0,
                reasons: Vec::new(),
            },
            evidence: TaskEvidenceLocator {
                source_path: path.into(),
                content_hash: hash.into(),
                start_line: 1,
                end_line: 1,
                heading_path: Vec::new(),
                locator_text: None,
            },
            adapter_id: "generic".into(),
            warnings: Vec::new(),
        }
    }

    #[test]
    fn root_filter_ignores_roadmap_handoff_and_custom_tasks() {
        let snapshot = TaskIntelligenceSnapshot {
            project_id: "p".into(),
            parsed_at: "now".into(),
            adapter: ParserAdapterIdentity {
                id: "generic".into(),
                evidence: "test".into(),
                convention_matched: false,
            },
            tasks: vec![
                parsed("p", "TASKS.md", "TASKS", "ROOT", "hash"),
                parsed("p", "ROADMAP.md", "ROADMAP", "ROADMAP", "hash"),
                parsed("p", "HANDOFF.md", "HANDOFF", "HANDOFF", "hash"),
                parsed("p", "custom.md", "CUSTOM", "CUSTOM", "hash"),
            ],
            handoff: None,
            warnings: Vec::new(),
        };
        let root = snapshot
            .tasks
            .iter()
            .filter(|task| is_exact_root_task(task) && task.evidence.content_hash == "hash")
            .collect::<Vec<_>>();
        assert_eq!(root.len(), 1);
        assert_eq!(root[0].explicit_task_id.as_deref(), Some("ROOT"));
    }

    #[test]
    fn full_graph_scores_real_prerequisite_and_keeps_dependent_attention() {
        let prerequisite = input("project", "A");
        let mut dependent = input("project", "B");
        dependent.dependencies = vec!["A".into()];
        dependent.dependency_task_ids = vec!["A".into()];
        let set = build_candidate_set(vec![prerequisite, dependent]);
        assert_eq!(set.eligible.len(), 1);
        assert_eq!(set.eligible[0].unblocks, 1);
        assert_eq!(set.attention.len(), 1);
        let mut duplicate = input("project", "B");
        duplicate.dependency_task_ids = vec!["A".into(), "A".into()];
        let deduped = build_candidate_set(vec![input("project", "A"), duplicate]);
        assert_eq!(deduped.eligible[0].unblocks, 1);
    }

    #[test]
    fn duplicate_remote_explicit_ids_are_ambiguous_even_with_conflicting_states() {
        let mut first = input("remote", "TASK-A");
        first.canonical_row_id = "remote:TASKS.md:10:first".into();
        first.factual_state = "PLANNED".into();
        let mut second = input("remote", "task-a");
        second.canonical_row_id = "remote:TASKS.md:11:second".into();
        second.factual_state = "TASK_COMPLETE".into();
        let mut dependent = input("remote", "TASK-B");
        dependent.canonical_row_id = "remote:TASKS.md:12:dependent".into();
        dependent.dependencies = vec!["TASK-A".into()];
        let set = build_candidate_set(vec![first, second, dependent]);
        assert!(set
            .eligible
            .iter()
            .all(|candidate| candidate.input.task_id != "TASK-B"));
        assert!(set.attention.iter().any(|item| {
            item.task_id.as_deref() == Some("TASK-B") && item.detail.contains("ambiguous")
        }));
    }

    #[test]
    fn duplicate_local_explicit_ids_are_ambiguous_without_row_collapse() {
        let mut first = input("local", "TASK-A");
        first.canonical_row_id = "local:TASKS.md:10:first".into();
        let mut second = input("local", "TASK-A");
        second.canonical_row_id = "local:TASKS.md:11:second".into();
        let mut dependent = input("local", "TASK-B");
        dependent.canonical_row_id = "local:TASKS.md:12:dependent".into();
        dependent.dependencies = vec!["TASK-A".into()];
        let set = build_candidate_set(vec![first, second, dependent]);
        assert!(set
            .eligible
            .iter()
            .all(|candidate| candidate.input.task_id != "TASK-B"));
        assert!(set.attention.iter().any(|item| {
            item.task_id.as_deref() == Some("TASK-B") && item.detail.contains("ambiguous")
        }));
    }

    #[test]
    fn duplicate_dependency_edges_are_one_canonical_edge_and_one_blocker() {
        let mut dependent = input("local", "B");
        dependent.dependencies = vec!["A".into(), "A".into(), "a".into()];
        let set = build_candidate_set(vec![input("local", "A"), dependent]);
        assert_eq!(set.attention.len(), 1);
        assert_eq!(
            set.attention[0]
                .detail
                .matches("dependency A is unfinished")
                .count(),
            1
        );
    }

    #[test]
    fn remote_validation_timestamp_uses_scheduler_compatible_hard_horizon() {
        let remote = github_tracking::RemoteTrackingSnapshot {
            project_key: "github:owner/repo@main".into(),
            display_name: "repo".into(),
            repository: "owner/repo".into(),
            branch: "main".into(),
            remote_head: Some("head".into()),
            project_blob_sha: None,
            tasks_blob_sha: Some("tasks".into()),
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
            progress_completed: Some(0),
            progress_total: Some(0),
            progress_percent: Some(0.0),
            last_completed_task_id: None,
            last_completed_task_title: None,
            updated_at: None,
            updated_by: None,
            total_tasks: Some(0),
            completed_tasks: Some(0),
            latest_commit_message: None,
            latest_commit_author: None,
            latest_commit_at: None,
            fetched_at: "2026-09-16T00:09:00Z".into(),
            content_fetched_at: Some("2026-09-15T00:00:00Z".into()),
            validated_at: Some("2026-09-16T00:09:00Z".into()),
            remote_health: "CURRENT".into(),
            error: None,
            recent_events: Vec::new(),
            task_rows: Vec::new(),
        };
        assert!(remote_validation_is_fresh(
            &remote,
            DateTime::parse_from_rfc3339("2026-09-16T01:09:00Z")
                .ok()
                .map(|value| value.with_timezone(&Utc))
        ));
        assert!(!remote_validation_is_fresh(
            &remote,
            DateTime::parse_from_rfc3339("2026-09-16T01:10:21Z")
                .ok()
                .map(|value| value.with_timezone(&Utc))
        ));
    }

    #[test]
    fn latest_pass_supersedes_an_older_failure_for_urgency() {
        let evidence = vec![
            FailureEvidence {
                evidence_id: "audit-old".into(),
                source: "audits".into(),
                result: "FAIL".into(),
                occurred_at: "2026-09-16T09:00:00Z".into(),
                age_seconds: 60,
                freshness: "CURRENT".into(),
            },
            FailureEvidence {
                evidence_id: "test-new".into(),
                source: "test_runs".into(),
                result: "PASS".into(),
                occurred_at: "2026-09-16T09:01:00Z".into(),
                age_seconds: 0,
                freshness: "CURRENT".into(),
            },
        ];
        assert_eq!(verified_failure_urgency(&evidence), 0);
    }

    #[test]
    fn more_than_128_candidates_are_scored_fairly_across_projects() {
        let mut values = (0..140)
            .map(|index| input("early-project", &format!("task-{index:03}")))
            .collect::<Vec<_>>();
        let mut late = input("zz-late-project", "highest");
        late.task_priority = Some(100);
        values.push(late);
        let set = build_candidate_set(values);
        let scored = score_candidates(set.eligible);
        assert_eq!(scored.len(), 141);
        assert_eq!(scored[0].candidate.input.project_id, "zz-late-project");
        assert!(scored
            .iter()
            .any(|candidate| candidate.candidate.input.project_id == "early-project"));
    }

    #[test]
    fn rank_explanations_use_actual_rank_and_score_difference() {
        let mut second = input("b", "task");
        second.project_priority = 0;
        let scored =
            score_candidates(build_candidate_set(vec![input("a", "task"), second]).eligible);
        let first = to_recommendation(&scored[0], 1, scored[0].score);
        let alternative = to_recommendation(&scored[1], 2, scored[0].score);
        assert!(first.explanation.contains("Ranked first"));
        assert!(alternative.explanation.contains("Ranked #2"));
        assert!(!alternative.explanation.starts_with("Ranked first"));
    }

    #[test]
    fn unknown_ci_audit_human_external_actors_fail_closed_to_attention() {
        let mut values = Vec::new();
        for actor in [
            Some("HUMAN"),
            Some("CI"),
            Some("GPT_AUDIT"),
            Some("EXTERNAL"),
            None,
        ] {
            let mut value = input("project", actor.unwrap_or("unknown"));
            value.required_actor = actor.map(str::to_string);
            values.push(value);
        }
        let set = build_candidate_set(values);
        assert!(set.eligible.is_empty());
        assert_eq!(set.attention.len(), 5);
    }

    #[test]
    fn snapshot_persists_truthful_first_and_unchanged_comparison() {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(
            project_dir.path().join("TASKS.md"),
            "# Work\n- [ ] TASK-1 — Build\n  Owner: Codex\n",
        )
        .unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Local".into()),
            },
        )
        .unwrap();
        let first = snapshot(&database).unwrap();
        record_history(&database, &first).unwrap();
        let second = snapshot(&database).unwrap();
        assert_eq!(first.comparison.state, "UNAVAILABLE_FIRST_SNAPSHOT");
        assert_eq!(second.comparison.state, "NO_COMPARABLE_CHANGE");
    }

    #[test]
    fn refresh_and_compare_uses_pre_refresh_fingerprint_when_evidence_changes() {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        let tasks = project_dir.path().join("TASKS.md");
        fs::write(&tasks, "- [ ] TASK-1 — First\n  Owner: Codex\n").unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Refresh change".into()),
            },
        )
        .unwrap();
        let post = refresh_and_compare(&database, || {
            fs::write(&tasks, "- [x] TASK-1 — First\n  Owner: Codex\n").map_err(|error| error.to_string())
        })
        .unwrap();
        assert_eq!(post.comparison.state, "CHANGED");
        assert!(post.comparison.previous_generated_at.is_some());
        assert!(post.recommended.is_none());
    }

    #[test]
    fn refresh_and_compare_has_truthful_no_change_and_failure_semantics() {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        let tasks = project_dir.path().join("TASKS.md");
        fs::write(&tasks, "- [ ] TASK-1 — Stable\n  Owner: Codex\n").unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Refresh stable".into()),
            },
        )
        .unwrap();
        let unchanged = refresh_and_compare(&database, || Ok(())).unwrap();
        assert_eq!(unchanged.comparison.state, "NO_COMPARABLE_CHANGE");

        let failure_db_dir = tempdir().unwrap();
        let failure_project_dir = tempdir().unwrap();
        fs::write(
            failure_project_dir.path().join("TASKS.md"),
            "- [ ] TASK-1 — Failure\n  Owner: Codex\n",
        )
        .unwrap();
        let failure_database = DatabaseState::initialize(failure_db_dir.path().to_path_buf()).unwrap();
        register_project(
            &failure_database,
            RegisterProjectRequest {
                path: failure_project_dir.path().to_string_lossy().into(),
                name: Some("Refresh failure".into()),
            },
        )
        .unwrap();
        assert!(refresh_and_compare(&failure_database, || Err("refresh failed".into())).is_err());
        assert_eq!(snapshot(&failure_database).unwrap().comparison.state, "UNAVAILABLE_FIRST_SNAPSHOT");
    }

    #[test]
    fn m19_snapshot_is_observational_until_history_is_explicitly_recorded() {
        let db_dir = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(
            project_dir.path().join("TASKS.md"),
            "- [ ] TASK-1 — Observe\n  Owner: Codex\n",
        )
        .unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Pure".into()),
            },
        )
        .unwrap();
        let before = database.open_connection().unwrap().query_row("SELECT (SELECT COUNT(*) FROM settings)+(SELECT COUNT(*) FROM task_events)+(SELECT COUNT(*) FROM task_sources)", [], |row| row.get::<_, i64>(0)).unwrap();
        let _ = snapshot(&database).unwrap();
        let _ = snapshot(&database).unwrap();
        let after = database.open_connection().unwrap().query_row("SELECT (SELECT COUNT(*) FROM settings)+(SELECT COUNT(*) FROM task_events)+(SELECT COUNT(*) FROM task_sources)", [], |row| row.get::<_, i64>(0)).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn missing_and_malformed_root_tasks_fail_closed() {
        let db_dir = tempdir().unwrap();
        let missing_dir = tempdir().unwrap();
        let malformed_dir = tempdir().unwrap();
        fs::write(malformed_dir.path().join("TASKS.md"), [0xff, 0xfe, 0xfd]).unwrap();
        let database = DatabaseState::initialize(db_dir.path().to_path_buf()).unwrap();
        register_project(
            &database,
            RegisterProjectRequest {
                path: missing_dir.path().to_string_lossy().into(),
                name: Some("Missing root".into()),
            },
        )
        .unwrap();
        register_project(
            &database,
            RegisterProjectRequest {
                path: malformed_dir.path().to_string_lossy().into(),
                name: Some("Malformed root".into()),
            },
        )
        .unwrap();
        let result = snapshot(&database).unwrap();
        assert!(result.recommended.is_none());
        assert!(result.unavailable_inputs.iter().any(
            |value| value.contains("Missing root") && value.contains("TASKS.md is unavailable")
        ));
        assert!(result
            .unavailable_inputs
            .iter()
            .any(|value| value.contains("Malformed root")
                && value.contains("contains invalid UTF-8")));
    }
}
