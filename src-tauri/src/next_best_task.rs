use crate::agent_session_center::ProviderReadiness;
use crate::db::DatabaseState;
use crate::github_tracking;
use crate::projects::{list_projects, ProjectListQuery, ProjectRecord};
use crate::task_intelligence::{self, TaskIntelligenceSnapshot};
use crate::time::utc_timestamp;
use chrono::{DateTime, Duration, Utc};
use rusqlite::OptionalExtension;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

pub const MAX_CANDIDATES: usize = 128;
pub const MAX_ATTENTION: usize = 64;
pub const MAX_FACTS: usize = 32;
const RECENT_FAILURE_WINDOW: Duration = Duration::days(7);

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
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct M19Recommendation {
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
    pub task_title: String,
    pub factual_state: String,
    pub dependencies: Vec<String>,
    pub blockers: Vec<String>,
    pub required_actor: Option<String>,
    pub external_wait: Option<String>,
    pub evidence: Vec<String>,
    pub evidence_freshness: String,
    pub verified_failure_urgency: i64,
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

pub fn snapshot(database: &DatabaseState) -> Result<M19Snapshot, String> {
    let projects = list_projects(
        database,
        ProjectListQuery {
            include_archived: Some(false),
            ..Default::default()
        },
    )?;
    let readiness = crate::agent_session_center::readiness();
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
        left.project_name
            .to_ascii_lowercase()
            .cmp(&right.project_name.to_ascii_lowercase())
            .then(left.task_id.cmp(&right.task_id))
            .then(left.title.cmp(&right.title))
    });
    attention.truncate(MAX_ATTENTION);
    unavailable_inputs.extend(candidates.unavailable_inputs);
    unavailable_inputs.sort();
    unavailable_inputs.dedup();
    unavailable_inputs.truncate(MAX_FACTS);

    let recommendations = scored.iter().map(to_recommendation).collect::<Vec<_>>();
    let recommended = recommendations.first().cloned();
    let alternatives = recommendations.into_iter().skip(1).take(3).collect();
    let mut facts = vec![
        M19Fact {
            label: "Active projects".into(),
            value: projects
                .iter()
                .filter(|project| project.status == "ACTIVE")
                .count()
                .to_string(),
            source: "Registry projects.status".into(),
            freshness: "CURRENT".into(),
        },
        M19Fact {
            label: "Eligible task candidates".into(),
            value: scored.len().to_string(),
            source: "M19 deterministic candidate builder".into(),
            freshness: "CURRENT".into(),
        },
        M19Fact {
            label: "Attention items".into(),
            value: attention.len().to_string(),
            source: "M19 eligibility boundary".into(),
            freshness: "CURRENT".into(),
        },
    ];
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
        generated_at: utc_timestamp(),
        active_projects: projects
            .iter()
            .filter(|project| project.status == "ACTIVE")
            .count(),
        candidate_count: scored.len(),
        recommended,
        alternatives,
        attention,
        facts,
        unavailable_inputs,
    })
}

fn collect_local_inputs(
    database: &DatabaseState,
    project: &ProjectRecord,
    inputs: &mut Vec<CandidateInput>,
    attention: &mut Vec<M19Attention>,
    unavailable: &mut Vec<String>,
) {
    let intelligence = task_intelligence::list(database, &project.id)
        .or_else(|_| task_intelligence::parse(database, &project.id));
    match intelligence {
        Ok(snapshot) => collect_parsed_inputs(database, project, &snapshot, inputs, attention),
        Err(error) => {
            unavailable.push(format!("{} TASKS truth: {}", project.name, bounded(&error)))
        }
    }
}

fn collect_parsed_inputs(
    database: &DatabaseState,
    project: &ProjectRecord,
    snapshot: &TaskIntelligenceSnapshot,
    inputs: &mut Vec<CandidateInput>,
    attention: &mut Vec<M19Attention>,
) {
    let task_ids = snapshot
        .tasks
        .iter()
        .map(|task| task.id.clone())
        .collect::<HashSet<_>>();
    let completed = snapshot
        .tasks
        .iter()
        .filter(|task| is_completed(&task.parsed_status))
        .map(|task| task.id.clone())
        .collect::<HashSet<_>>();
    for task in &snapshot.tasks {
        let dependencies = task.dependency_references.clone();
        let mut blockers = task.blockers.clone();
        for dependency in &dependencies {
            if !task_ids.contains(dependency) || !completed.contains(dependency) {
                blockers.push(format!(
                    "dependency {dependency} is not complete or unavailable"
                ));
            }
        }
        let mut input = CandidateInput {
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            project_priority: project.priority,
            task_id: task.id.clone(),
            task_title: task.title.clone(),
            factual_state: task.parsed_status.clone(),
            dependencies,
            blockers,
            required_actor: task.required_actor.clone(),
            external_wait: task
                .external_wait
                .clone()
                .or_else(|| task.owner_gate.clone()),
            evidence: vec![format!(
                "{}:{}-{}",
                task.evidence.source_path, task.evidence.start_line, task.evidence.end_line
            )],
            evidence_freshness: "CURRENT".into(),
            verified_failure_urgency: recent_failure_urgency(database, &project.id, &task.id),
        };
        input.blockers.sort();
        input.blockers.dedup();
        add_or_defer(input, inputs, attention);
    }
}

fn collect_remote_inputs(
    database: &DatabaseState,
    project: &ProjectRecord,
    inputs: &mut Vec<CandidateInput>,
    attention: &mut Vec<M19Attention>,
    unavailable: &mut Vec<String>,
) {
    let snapshot = github_tracking::cached_snapshot(database, project)
        .ok()
        .flatten();
    let Some(remote) = snapshot else {
        unavailable.push(format!(
            "{} GitHub TASKS snapshot is unavailable",
            project.name
        ));
        return;
    };
    if remote.remote_health != "CURRENT" {
        unavailable.push(format!(
            "{} GitHub evidence is {}",
            project.name, remote.remote_health
        ));
        return;
    }
    if remote.task_rows.is_empty() {
        if remote.total_tasks != Some(0) {
            unavailable.push(format!(
                "{} GitHub TASKS rows are unavailable",
                project.name
            ));
        }
        return;
    }
    let completed = remote
        .task_rows
        .iter()
        .filter(|task| is_completed(&task.status))
        .map(|task| task.id.clone())
        .collect::<HashSet<_>>();
    for task in remote.task_rows {
        let blockers = if remote.current_task_id.as_deref() == Some(task.id.as_str()) {
            remote.blockers.clone()
        } else {
            Vec::new()
        };
        let input = CandidateInput {
            project_id: project.id.clone(),
            project_name: project.name.clone(),
            project_priority: project.priority,
            task_id: task.id.clone(),
            task_title: task.title,
            factual_state: task.status.clone(),
            dependencies: Vec::new(),
            blockers,
            required_actor: remote.required_actor.clone(),
            external_wait: None,
            evidence: vec![format!(
                "GitHub {}/TASKS.md@{}",
                remote.repository,
                remote.remote_head.as_deref().unwrap_or("HEAD unavailable")
            )],
            evidence_freshness: "CURRENT".into(),
            verified_failure_urgency: 0,
        };
        if completed.contains(&input.task_id) {
            continue;
        }
        add_or_defer(input, inputs, attention);
    }
}

fn add_or_defer(
    input: CandidateInput,
    inputs: &mut Vec<CandidateInput>,
    attention: &mut Vec<M19Attention>,
) {
    if is_completed(&input.factual_state) {
        return;
    }
    if !input.blockers.is_empty() {
        attention.push(M19Attention {
            project_id: input.project_id,
            project_name: input.project_name,
            task_id: Some(input.task_id),
            title: input.task_title,
            category: "BLOCKED_OR_DEPENDENCY".into(),
            detail: input.blockers.join("; "),
            evidence: input.evidence,
        });
        return;
    }
    if let Some(wait) = input
        .external_wait
        .as_deref()
        .filter(|value| !value.trim().is_empty())
    {
        attention.push(M19Attention {
            project_id: input.project_id,
            project_name: input.project_name,
            task_id: Some(input.task_id),
            title: input.task_title,
            category: "HUMAN_OR_EXTERNAL_WAIT".into(),
            detail: bounded(wait),
            evidence: input.evidence,
        });
        return;
    }
    if matches!(
        input
            .required_actor
            .as_deref()
            .map(|value| value.to_ascii_uppercase())
            .as_deref(),
        Some("HUMAN") | Some("EXTERNAL")
    ) {
        attention.push(M19Attention {
            project_id: input.project_id,
            project_name: input.project_name,
            task_id: Some(input.task_id),
            title: input.task_title,
            category: "HUMAN_OR_EXTERNAL_WAIT".into(),
            detail: format!(
                "required actor: {}",
                input.required_actor.as_deref().unwrap_or("unknown")
            ),
            evidence: input.evidence,
        });
        return;
    }
    inputs.push(input);
}

fn build_candidate_set(inputs: Vec<CandidateInput>) -> CandidateSet {
    let mut set = CandidateSet::default();
    let mut task_ids = HashSet::new();
    for input in &inputs {
        task_ids.insert((input.project_id.clone(), input.task_id.clone()));
    }
    for input in inputs {
        if is_completed(&input.factual_state) {
            continue;
        }
        let mut uncertainty = Vec::new();
        if input.evidence_freshness != "CURRENT" {
            uncertainty.push(format!("evidence is {}", input.evidence_freshness));
        }
        let actor_readiness = match input
            .required_actor
            .as_deref()
            .map(|value| value.to_ascii_uppercase())
        {
            Some(actor) => actor,
            None => {
                uncertainty.push("required actor is unavailable".into());
                "UNKNOWN".into()
            }
        };
        set.eligible.push(Candidate {
            input,
            uncertainty,
            actor_readiness,
            unblocks: 0,
        });
    }
    let clone = set
        .eligible
        .iter()
        .map(|candidate| candidate.input.clone())
        .collect::<Vec<_>>();
    for candidate in &mut set.eligible {
        candidate.unblocks = clone
            .iter()
            .filter(|other| {
                other.project_id == candidate.input.project_id
                    && other.dependencies.contains(&candidate.input.task_id)
                    && task_ids.contains(&(
                        candidate.input.project_id.clone(),
                        candidate.input.task_id.clone(),
                    ))
            })
            .count() as i64;
    }
    set.eligible.truncate(MAX_CANDIDATES);
    set
}

fn apply_readiness(set: &mut CandidateSet, readiness: &[ProviderReadiness]) {
    let states = readiness
        .iter()
        .map(|value| (value.provider.to_ascii_uppercase(), value))
        .collect::<HashMap<_, _>>();
    let mut keep = Vec::new();
    for mut candidate in set.eligible.drain(..) {
        let actor = candidate
            .input
            .required_actor
            .as_deref()
            .unwrap_or("UNKNOWN")
            .to_ascii_uppercase();
        if matches!(actor.as_str(), "CODEX" | "CLAUDE") {
            match states.get(&actor) {
                Some(provider) if provider.available => {
                    candidate.actor_readiness = format!("{} / AVAILABLE", actor)
                }
                Some(provider) => {
                    set.attention.push(M19Attention {
                        project_id: candidate.input.project_id.clone(),
                        project_name: candidate.input.project_name.clone(),
                        task_id: Some(candidate.input.task_id.clone()),
                        title: candidate.input.task_title.clone(),
                        category: "ACTOR_UNAVAILABLE".into(),
                        detail: provider
                            .diagnostic_message
                            .clone()
                            .unwrap_or_else(|| format!("{actor} is {}", provider.readiness_state)),
                        evidence: candidate.input.evidence.clone(),
                    });
                    continue;
                }
                None => candidate
                    .uncertainty
                    .push(format!("{actor} readiness is unavailable")),
            }
        }
        keep.push(candidate);
    }
    set.eligible = keep;
}

fn score_candidates(candidates: Vec<Candidate>) -> Vec<ScoredCandidate> {
    let mut scored = candidates
        .into_iter()
        .map(|candidate| {
            let input = &candidate.input;
            let components = vec![
                M19ScoreComponent {
                    key: "project_priority".into(),
                    label: "Explicit project priority".into(),
                    points: input.project_priority.clamp(-100, 100) * 10,
                    evidence: "Registry priority".into(),
                },
                M19ScoreComponent {
                    key: "dependency_unlock".into(),
                    label: "Dependency unlock".into(),
                    points: candidate.unblocks.clamp(0, 20) * 15,
                    evidence: "Canonical task dependency references".into(),
                },
                M19ScoreComponent {
                    key: "verified_failure".into(),
                    label: "Fresh verified failure".into(),
                    points: input.verified_failure_urgency.clamp(0, 100),
                    evidence: "Latest linked audit/test evidence".into(),
                },
                M19ScoreComponent {
                    key: "actor_readiness".into(),
                    label: "Required actor readiness".into(),
                    points: if candidate.actor_readiness.ends_with("AVAILABLE") {
                        20
                    } else {
                        0
                    },
                    evidence: candidate.actor_readiness.clone(),
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
            .then(
                left.candidate
                    .input
                    .project_name
                    .to_ascii_lowercase()
                    .cmp(&right.candidate.input.project_name.to_ascii_lowercase()),
            )
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

fn to_recommendation(scored: &ScoredCandidate) -> M19Recommendation {
    let input = &scored.candidate.input;
    let positive = scored
        .components
        .iter()
        .filter(|component| component.points > 0)
        .map(|component| format!("{} +{}", component.label, component.points))
        .collect::<Vec<_>>();
    M19Recommendation {
        project_id: input.project_id.clone(),
        project_name: input.project_name.clone(),
        task_id: input.task_id.clone(),
        task_title: input.task_title.clone(),
        factual_state: input.factual_state.clone(),
        eligibility_reason:
            "ACTIVE task with no verified blocker, dependency, or human/external wait".into(),
        score: scored.score,
        score_components: scored.components.clone(),
        dependencies: input.dependencies.clone(),
        blockers: input.blockers.clone(),
        required_actor: input.required_actor.clone(),
        actor_readiness: scored.candidate.actor_readiness.clone(),
        evidence: input.evidence.clone(),
        uncertainty: scored.candidate.uncertainty.clone(),
        explanation: if positive.is_empty() {
            "Ranked first among the currently eligible candidates; no favorable evidence inputs were available.".into()
        } else {
            format!(
                "Ranked first from the same factual score object: {}.",
                positive.join(", ")
            )
        },
    }
}

fn recent_failure_urgency(database: &DatabaseState, project_id: &str, task_id: &str) -> i64 {
    let Ok(connection) = database.open_connection() else {
        return 0;
    };
    let now = DateTime::parse_from_rfc3339(&utc_timestamp())
        .ok()
        .map(|value| value.with_timezone(&Utc));
    let latest = [
        ("audits", "created_at", "created_at"),
        ("test_runs", "COALESCE(finished_at, started_at)", "COALESCE(finished_at, started_at)"),
    ].iter().filter_map(|(table, timestamp, order)| {
        let sql = format!("SELECT result, {timestamp} FROM {table} WHERE project_id=?1 AND task_id=?2 ORDER BY {order} DESC LIMIT 1");
        connection.query_row(&sql, rusqlite::params![project_id, task_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?))).optional().ok().flatten()
    }).collect::<Vec<_>>();
    latest
        .into_iter()
        .filter_map(|(result, at)| {
            if !matches!(
                result.to_ascii_uppercase().as_str(),
                "FAIL" | "FAILED" | "ERROR"
            ) {
                return None;
            }
            let current = now?;
            let parsed = DateTime::parse_from_rfc3339(at.as_deref()?)
                .ok()?
                .with_timezone(&Utc);
            (current.signed_duration_since(parsed) <= RECENT_FAILURE_WINDOW).then_some(100)
        })
        .max()
        .unwrap_or(0)
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
            | "[X]"
            | "X"
    )
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

    fn input(project: &str, task: &str) -> CandidateInput {
        CandidateInput {
            project_id: project.into(),
            project_name: project.into(),
            project_priority: 1,
            task_id: task.into(),
            task_title: format!("{task} title"),
            factual_state: "PLANNED".into(),
            dependencies: Vec::new(),
            blockers: Vec::new(),
            required_actor: Some("CODEX".into()),
            external_wait: None,
            evidence: vec!["TASKS.md:1".into()],
            evidence_freshness: "CURRENT".into(),
            verified_failure_urgency: 0,
        }
    }

    #[test]
    fn non_seed_projects_are_eligible_and_completed_tasks_are_excluded() {
        let mut completed = input("seed", "done");
        completed.factual_state = "COMPLETED".into();
        let set = build_candidate_set(vec![input("project-9", "M19.01"), completed]);
        assert_eq!(set.eligible.len(), 1);
        assert_eq!(set.eligible[0].input.project_id, "project-9");
    }

    #[test]
    fn blockers_and_human_waits_are_separate_attention_items() {
        let mut blocked = input("project-9", "blocked");
        blocked.blockers.push("dependency unfinished".into());
        let mut waiting = input("project-10", "wait");
        waiting.required_actor = Some("HUMAN".into());
        let mut set = CandidateSet::default();
        let mut inputs = Vec::new();
        let mut attention = Vec::new();
        add_or_defer(blocked, &mut inputs, &mut attention);
        add_or_defer(waiting, &mut inputs, &mut attention);
        set.eligible = build_candidate_set(inputs).eligible;
        set.attention = attention;
        assert!(set.eligible.is_empty());
        assert_eq!(set.attention.len(), 2);
        assert!(set
            .attention
            .iter()
            .any(|item| item.category == "HUMAN_OR_EXTERNAL_WAIT"));
    }

    #[test]
    fn scoring_and_tie_breaking_are_deterministic() {
        let mut high = input("z-project", "task");
        high.verified_failure_urgency = 100;
        let candidates = build_candidate_set(vec![input("a-project", "task"), high]).eligible;
        let scored = score_candidates(candidates);
        assert_eq!(scored[0].candidate.input.project_id, "z-project");
        assert_eq!(
            scored[0].score,
            scored[0]
                .components
                .iter()
                .map(|component| component.points)
                .sum::<i64>()
        );
        assert!(to_recommendation(&scored[0])
            .explanation
            .contains("Fresh verified failure"));
    }
}
