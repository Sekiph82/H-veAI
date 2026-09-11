use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Migration {
    pub version: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    pub schema_version: i64,
    pub migration_count: i64,
    pub last_migration_status: &'static str,
}

const INITIAL_SCHEMA: &str = r#"
CREATE TABLE projects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    local_path TEXT,
    default_branch TEXT,
    priority INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    metadata_json TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE repositories (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    remote_url TEXT,
    github_owner TEXT,
    github_repo TEXT,
    default_branch TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE project_sources (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_path TEXT NOT NULL,
    source_kind TEXT NOT NULL,
    content_hash TEXT,
    discovered_at TEXT NOT NULL,
    metadata_json TEXT
);

CREATE TABLE git_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    repository_id TEXT NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    branch TEXT,
    head_sha TEXT,
    status_json TEXT,
    captured_at TEXT NOT NULL
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_id TEXT REFERENCES task_sources(id) ON DELETE SET NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL,
    required_actor TEXT,
    milestone TEXT,
    metadata_json TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE task_dependencies (
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    depends_on_task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    dependency_kind TEXT NOT NULL DEFAULT 'BLOCKS',
    created_at TEXT NOT NULL,
    PRIMARY KEY (task_id, depends_on_task_id)
);

CREATE TABLE task_sources (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    source_path TEXT NOT NULL,
    source_kind TEXT NOT NULL,
    locator TEXT,
    content_hash TEXT,
    discovered_at TEXT NOT NULL
);

CREATE TABLE task_events (
    id TEXT PRIMARY KEY NOT NULL,
    task_id TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    from_state TEXT,
    to_state TEXT,
    actor_type TEXT,
    summary TEXT NOT NULL,
    evidence_json TEXT,
    occurred_at TEXT NOT NULL
);

CREATE TABLE prompts (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
    task_id TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    kind TEXT NOT NULL,
    current_version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE prompt_versions (
    id TEXT PRIMARY KEY NOT NULL,
    prompt_id TEXT NOT NULL REFERENCES prompts(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE (prompt_id, version)
);

CREATE TABLE agent_sessions (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
    task_id TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    provider TEXT NOT NULL,
    state TEXT NOT NULL,
    started_at TEXT,
    ended_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE agent_events (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL REFERENCES agent_sessions(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    payload_json TEXT,
    occurred_at TEXT NOT NULL
);

CREATE TABLE agent_tool_calls (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL REFERENCES agent_sessions(id) ON DELETE CASCADE,
    tool_name TEXT NOT NULL,
    status TEXT NOT NULL,
    input_metadata_json TEXT,
    output_metadata_json TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE permission_requests (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT REFERENCES agent_sessions(id) ON DELETE SET NULL,
    permission_kind TEXT NOT NULL,
    requested_resource TEXT,
    state TEXT NOT NULL,
    decided_by TEXT,
    created_at TEXT NOT NULL,
    decided_at TEXT
);

CREATE TABLE audits (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
    task_id TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    result TEXT NOT NULL,
    summary TEXT,
    confidence REAL,
    created_at TEXT NOT NULL
);

CREATE TABLE audit_findings (
    id TEXT PRIMARY KEY NOT NULL,
    audit_id TEXT NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    severity TEXT NOT NULL,
    title TEXT NOT NULL,
    detail TEXT,
    file_path TEXT,
    line_number INTEGER,
    created_at TEXT NOT NULL
);

CREATE TABLE test_runs (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE SET NULL,
    task_id TEXT REFERENCES tasks(id) ON DELETE SET NULL,
    command TEXT NOT NULL,
    result TEXT NOT NULL,
    output_metadata_json TEXT,
    started_at TEXT NOT NULL,
    finished_at TEXT
);

CREATE TABLE alerts (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE CASCADE,
    task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,
    state TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TEXT NOT NULL,
    resolved_at TEXT
);

CREATE TABLE decisions (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT REFERENCES projects(id) ON DELETE CASCADE,
    task_id TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    audit_id TEXT REFERENCES audits(id) ON DELETE SET NULL,
    decision_kind TEXT NOT NULL,
    decision TEXT NOT NULL,
    rationale TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE github_sync_state (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    resource_kind TEXT NOT NULL,
    resource_cursor TEXT,
    last_synced_at TEXT,
    metadata_json TEXT,
    UNIQUE (project_id, resource_kind)
);

CREATE TABLE settings (
    key TEXT PRIMARY KEY NOT NULL,
    value_json TEXT NOT NULL,
    scope TEXT NOT NULL DEFAULT 'WORKSPACE',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
"#;

const INITIAL_INDEXES: &str = r#"
CREATE INDEX idx_repositories_project ON repositories(project_id);
CREATE INDEX idx_project_sources_project ON project_sources(project_id, discovered_at);
CREATE INDEX idx_git_snapshots_repository ON git_snapshots(repository_id, captured_at);
CREATE INDEX idx_tasks_project_state ON tasks(project_id, state, updated_at);
CREATE INDEX idx_tasks_source ON tasks(source_id);
CREATE INDEX idx_task_dependencies_dependency ON task_dependencies(depends_on_task_id);
CREATE INDEX idx_task_sources_project ON task_sources(project_id, discovered_at);
CREATE INDEX idx_task_events_task_time ON task_events(task_id, occurred_at);
CREATE INDEX idx_prompt_versions_prompt ON prompt_versions(prompt_id, version);
CREATE INDEX idx_agent_sessions_project_state ON agent_sessions(project_id, state);
CREATE INDEX idx_agent_events_session_time ON agent_events(session_id, occurred_at);
CREATE INDEX idx_agent_tool_calls_session ON agent_tool_calls(session_id, created_at);
CREATE INDEX idx_permission_requests_state ON permission_requests(state, created_at);
CREATE INDEX idx_audits_project_time ON audits(project_id, created_at);
CREATE INDEX idx_audit_findings_audit ON audit_findings(audit_id, severity);
CREATE INDEX idx_test_runs_project_time ON test_runs(project_id, started_at);
CREATE INDEX idx_alerts_state ON alerts(state, created_at);
CREATE INDEX idx_decisions_project_time ON decisions(project_id, created_at);
CREATE INDEX idx_github_sync_project ON github_sync_state(project_id, last_synced_at);
CREATE INDEX idx_settings_scope ON settings(scope);
"#;

const PROJECT_REGISTRY_FIELDS: &str = r#"
ALTER TABLE projects ADD COLUMN original_path TEXT;
ALTER TABLE projects ADD COLUMN normalized_path TEXT;
ALTER TABLE projects ADD COLUMN registered_at TEXT;
ALTER TABLE projects ADD COLUMN last_validated_at TEXT;
ALTER TABLE projects ADD COLUMN preferred_builder TEXT;
ALTER TABLE projects ADD COLUMN preferred_auditor TEXT;
ALTER TABLE projects ADD COLUMN task_source_policy TEXT;
ALTER TABLE projects ADD COLUMN archived_at TEXT;
ALTER TABLE repositories ADD COLUMN repository_root TEXT;
ALTER TABLE repositories ADD COLUMN is_git_repository INTEGER NOT NULL DEFAULT 0;
ALTER TABLE repositories ADD COLUMN current_branch TEXT;
ALTER TABLE repositories ADD COLUMN head_sha TEXT;
ALTER TABLE repositories ADD COLUMN remote_urls_json TEXT;
"#;

const PROJECT_SNAPSHOT_FIELDS: &str = r#"
CREATE TABLE project_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    availability TEXT NOT NULL,
    git_snapshot_id TEXT REFERENCES git_snapshots(id) ON DELETE SET NULL,
    last_filesystem_event_at TEXT,
    last_watcher_refresh_at TEXT,
    evidence_generated_at TEXT NOT NULL,
    changed_path_count INTEGER NOT NULL DEFAULT 0,
    rescan_required INTEGER NOT NULL DEFAULT 0,
    watcher_health TEXT NOT NULL,
    created_at TEXT NOT NULL
);
CREATE INDEX idx_project_snapshots_project_time ON project_snapshots(project_id, evidence_generated_at);
"#;

const TIMESTAMP_STANDARDIZATION: &str = r#"
UPDATE projects SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE projects SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
UPDATE projects SET registered_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(registered_at AS INTEGER), 'unixepoch') WHERE registered_at GLOB '[0-9]*' AND registered_at NOT GLOB '*[^0-9]*';
UPDATE projects SET last_validated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(last_validated_at AS INTEGER), 'unixepoch') WHERE last_validated_at GLOB '[0-9]*' AND last_validated_at NOT GLOB '*[^0-9]*';
UPDATE projects SET archived_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(archived_at AS INTEGER), 'unixepoch') WHERE archived_at GLOB '[0-9]*' AND archived_at NOT GLOB '*[^0-9]*';
UPDATE repositories SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE repositories SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
UPDATE git_snapshots SET captured_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(captured_at AS INTEGER), 'unixepoch') WHERE captured_at GLOB '[0-9]*' AND captured_at NOT GLOB '*[^0-9]*';
UPDATE project_snapshots SET last_filesystem_event_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(last_filesystem_event_at AS INTEGER), 'unixepoch') WHERE last_filesystem_event_at GLOB '[0-9]*' AND last_filesystem_event_at NOT GLOB '*[^0-9]*';
UPDATE project_snapshots SET last_watcher_refresh_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(last_watcher_refresh_at AS INTEGER), 'unixepoch') WHERE last_watcher_refresh_at GLOB '[0-9]*' AND last_watcher_refresh_at NOT GLOB '*[^0-9]*';
UPDATE project_snapshots SET evidence_generated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(evidence_generated_at AS INTEGER), 'unixepoch') WHERE evidence_generated_at GLOB '[0-9]*' AND evidence_generated_at NOT GLOB '*[^0-9]*';
"#;

const RESIDUAL_TIMESTAMP_STANDARDIZATION: &str = r#"
UPDATE tasks SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE tasks SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
UPDATE task_sources SET discovered_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(discovered_at AS INTEGER), 'unixepoch') WHERE discovered_at GLOB '[0-9]*' AND discovered_at NOT GLOB '*[^0-9]*';
UPDATE task_events SET occurred_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(occurred_at AS INTEGER), 'unixepoch') WHERE occurred_at GLOB '[0-9]*' AND occurred_at NOT GLOB '*[^0-9]*';
UPDATE prompts SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE prompts SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
UPDATE prompt_versions SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE agent_sessions SET started_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(started_at AS INTEGER), 'unixepoch') WHERE started_at GLOB '[0-9]*' AND started_at NOT GLOB '*[^0-9]*';
UPDATE agent_sessions SET ended_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(ended_at AS INTEGER), 'unixepoch') WHERE ended_at GLOB '[0-9]*' AND ended_at NOT GLOB '*[^0-9]*';
UPDATE agent_sessions SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE agent_events SET occurred_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(occurred_at AS INTEGER), 'unixepoch') WHERE occurred_at GLOB '[0-9]*' AND occurred_at NOT GLOB '*[^0-9]*';
UPDATE agent_tool_calls SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE permission_requests SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE audits SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE audit_findings SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE test_runs SET started_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(started_at AS INTEGER), 'unixepoch') WHERE started_at GLOB '[0-9]*' AND started_at NOT GLOB '*[^0-9]*';
UPDATE test_runs SET finished_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(finished_at AS INTEGER), 'unixepoch') WHERE finished_at GLOB '[0-9]*' AND finished_at NOT GLOB '*[^0-9]*';
UPDATE alerts SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE alerts SET resolved_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(resolved_at AS INTEGER), 'unixepoch') WHERE resolved_at GLOB '[0-9]*' AND resolved_at NOT GLOB '*[^0-9]*';
UPDATE decisions SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE github_sync_state SET last_synced_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(last_synced_at AS INTEGER), 'unixepoch') WHERE last_synced_at GLOB '[0-9]*' AND last_synced_at NOT GLOB '*[^0-9]*';
UPDATE settings SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';
UPDATE settings SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(updated_at AS INTEGER), 'unixepoch') WHERE updated_at GLOB '[0-9]*' AND updated_at NOT GLOB '*[^0-9]*';
"#;

const PROMPT_ENGINE_FIELDS: &str = r#"
ALTER TABLE prompt_versions ADD COLUMN title TEXT;
ALTER TABLE prompt_versions ADD COLUMN summary TEXT;
ALTER TABLE prompt_versions ADD COLUMN origin TEXT NOT NULL DEFAULT 'SYSTEM_GENERATED';
ALTER TABLE prompt_versions ADD COLUMN context_manifest_json TEXT;
ALTER TABLE prompt_versions ADD COLUMN provenance_json TEXT;
ALTER TABLE prompt_versions ADD COLUMN approval_state TEXT NOT NULL DEFAULT 'DRAFT';
ALTER TABLE prompt_versions ADD COLUMN approved_at TEXT;
ALTER TABLE prompt_versions ADD COLUMN approved_body_sha256 TEXT;
ALTER TABLE prompt_versions ADD COLUMN used_at TEXT;
ALTER TABLE prompt_versions ADD COLUMN selected_provider TEXT;
ALTER TABLE prompt_versions ADD COLUMN dispatched_session_id TEXT;
ALTER TABLE prompt_versions ADD COLUMN superseded_at TEXT;
ALTER TABLE agent_sessions ADD COLUMN prompt_id TEXT;
ALTER TABLE agent_sessions ADD COLUMN prompt_version_id TEXT;
ALTER TABLE agent_sessions ADD COLUMN prompt_version INTEGER;
ALTER TABLE agent_sessions ADD COLUMN prompt_version_sha256 TEXT;
CREATE INDEX idx_prompt_versions_approval ON prompt_versions(approval_state, created_at);
CREATE INDEX idx_agent_sessions_prompt_version ON agent_sessions(prompt_version_id);
"#;

const PROMPT_DISPATCH_FIELDS: &str = r#"
ALTER TABLE prompt_versions ADD COLUMN dispatch_state TEXT NOT NULL DEFAULT 'AVAILABLE';
ALTER TABLE prompt_versions ADD COLUMN dispatch_reservation_id TEXT;
ALTER TABLE prompt_versions ADD COLUMN dispatch_reserved_at TEXT;
ALTER TABLE prompt_versions ADD COLUMN dispatch_provenance_json TEXT;
ALTER TABLE prompt_versions ADD COLUMN dispatch_error TEXT;
UPDATE prompt_versions SET dispatch_state = CASE WHEN dispatched_session_id IS NOT NULL OR used_at IS NOT NULL THEN 'DISPATCHED' ELSE 'AVAILABLE' END;
CREATE INDEX idx_prompt_versions_dispatch_state ON prompt_versions(dispatch_state, created_at);
CREATE UNIQUE INDEX idx_prompt_versions_dispatch_reservation ON prompt_versions(dispatch_reservation_id) WHERE dispatch_reservation_id IS NOT NULL;
"#;

const AUDIT_ENGINE_FIELDS: &str = r#"
ALTER TABLE audits ADD COLUMN audit_type TEXT NOT NULL DEFAULT 'IMPLEMENTATION';
ALTER TABLE audits ADD COLUMN audited_branch TEXT;
ALTER TABLE audits ADD COLUMN audited_head_sha TEXT;
ALTER TABLE audits ADD COLUMN baseline_ref TEXT;
ALTER TABLE audits ADD COLUMN input_manifest_sha256 TEXT;
ALTER TABLE audits ADD COLUMN schema_version INTEGER NOT NULL DEFAULT 1;
ALTER TABLE audits ADD COLUMN confidence_level TEXT;
ALTER TABLE audits ADD COLUMN regression_risk TEXT;
ALTER TABLE audits ADD COLUMN state TEXT NOT NULL DEFAULT 'COMPLETED';
ALTER TABLE audits ADD COLUMN started_at TEXT;
ALTER TABLE audits ADD COLUMN finished_at TEXT;
ALTER TABLE audits ADD COLUMN auditor_provider TEXT;
ALTER TABLE audits ADD COLUMN auditor_model TEXT;
ALTER TABLE audits ADD COLUMN auditor_version TEXT;
ALTER TABLE audits ADD COLUMN model_status TEXT NOT NULL DEFAULT 'AVAILABLE';
ALTER TABLE audits ADD COLUMN diagnostic TEXT;
ALTER TABLE audits ADD COLUMN prior_audit_id TEXT REFERENCES audits(id) ON DELETE SET NULL;
ALTER TABLE audits ADD COLUMN remediation_prompt_id TEXT REFERENCES prompts(id) ON DELETE SET NULL;
ALTER TABLE audits ADD COLUMN remediation_prompt_version_id TEXT REFERENCES prompt_versions(id) ON DELETE SET NULL;
ALTER TABLE audits ADD COLUMN remediation_session_id TEXT REFERENCES agent_sessions(id) ON DELETE SET NULL;
ALTER TABLE audit_findings ADD COLUMN finding_key TEXT;
ALTER TABLE audit_findings ADD COLUMN confidence_level TEXT;
ALTER TABLE audit_findings ADD COLUMN status TEXT NOT NULL DEFAULT 'OPEN';
ALTER TABLE audit_findings ADD COLUMN requirement_refs_json TEXT;
ALTER TABLE audit_findings ADD COLUMN evidence_refs_json TEXT;
ALTER TABLE audit_findings ADD COLUMN source_locator TEXT;
ALTER TABLE audit_findings ADD COLUMN test_locator TEXT;
ALTER TABLE audit_findings ADD COLUMN remediation_guidance TEXT;
ALTER TABLE audit_findings ADD COLUMN blocks_release INTEGER NOT NULL DEFAULT 0;
ALTER TABLE audit_findings ADD COLUMN closed_by_audit_id TEXT REFERENCES audits(id) ON DELETE SET NULL;
CREATE TABLE audit_evidence (
    id TEXT PRIMARY KEY NOT NULL,
    audit_id TEXT NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    evidence_kind TEXT NOT NULL,
    verification_status TEXT NOT NULL,
    locator TEXT,
    summary TEXT NOT NULL,
    content TEXT,
    content_sha256 TEXT,
    byte_count INTEGER NOT NULL DEFAULT 0,
    truncated INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);
CREATE TABLE audit_requirement_coverage (
    id TEXT PRIMARY KEY NOT NULL,
    audit_id TEXT NOT NULL REFERENCES audits(id) ON DELETE CASCADE,
    requirement_ref TEXT NOT NULL,
    requirement_text TEXT NOT NULL,
    status TEXT NOT NULL,
    evidence_refs_json TEXT NOT NULL,
    rationale TEXT NOT NULL,
    created_at TEXT NOT NULL
);
CREATE INDEX idx_audits_project_state_time ON audits(project_id, state, created_at);
CREATE INDEX idx_audits_prior ON audits(prior_audit_id);
CREATE INDEX idx_audit_findings_key ON audit_findings(audit_id, finding_key);
CREATE INDEX idx_audit_evidence_audit ON audit_evidence(audit_id, evidence_kind);
CREATE INDEX idx_audit_coverage_audit ON audit_requirement_coverage(audit_id, requirement_ref);
"#;

const AUDIT_REAUDIT_FIELDS: &str = r#"
ALTER TABLE audits ADD COLUMN freshness_token TEXT;
ALTER TABLE audit_findings ADD COLUMN prior_finding_key TEXT;
ALTER TABLE audit_findings ADD COLUMN disposition TEXT;
ALTER TABLE audit_findings ADD COLUMN disposition_evidence_refs_json TEXT;
ALTER TABLE audit_findings ADD COLUMN disposition_rationale TEXT;
ALTER TABLE audit_findings ADD COLUMN superseded_by_finding_key TEXT;
CREATE INDEX idx_audit_findings_prior_disposition ON audit_findings(audit_id, prior_finding_key, disposition);
"#;

const AUDIT_EVIDENCE_IDENTITY_FIELDS: &str = r#"
ALTER TABLE audit_evidence ADD COLUMN logical_evidence_id TEXT;
UPDATE audit_evidence
SET logical_evidence_id = CASE
    WHEN substr(id, 1, length(audit_id) + 1) = audit_id || ':'
        THEN substr(id, length(audit_id) + 2)
    ELSE id
END
WHERE logical_evidence_id IS NULL;
CREATE UNIQUE INDEX idx_audit_evidence_audit_logical
    ON audit_evidence(audit_id, logical_evidence_id);
"#;

const AUDIT_COMPREHENSIVE_IDENTITY_FIELDS: &str = r#"
ALTER TABLE audit_findings ADD COLUMN logical_finding_id TEXT;
UPDATE audit_findings AS current
SET logical_finding_id = COALESCE(NULLIF(current.finding_key, ''), current.id) || CASE
    WHEN (SELECT COUNT(*) FROM audit_findings AS prior
          WHERE prior.audit_id = current.audit_id
            AND COALESCE(NULLIF(prior.finding_key, ''), prior.id) = COALESCE(NULLIF(current.finding_key, ''), current.id)
            AND prior.rowid < current.rowid) > 0
    THEN '#' || ((SELECT COUNT(*) FROM audit_findings AS prior
                  WHERE prior.audit_id = current.audit_id
                    AND COALESCE(NULLIF(prior.finding_key, ''), prior.id) = COALESCE(NULLIF(current.finding_key, ''), current.id)
                    AND prior.rowid < current.rowid) + 1)
    ELSE ''
END;
ALTER TABLE audit_requirement_coverage ADD COLUMN logical_coverage_id TEXT;
UPDATE audit_requirement_coverage AS current
SET logical_coverage_id = COALESCE(NULLIF(current.requirement_ref, ''), current.id) || CASE
    WHEN (SELECT COUNT(*) FROM audit_requirement_coverage AS prior
          WHERE prior.audit_id = current.audit_id
            AND COALESCE(NULLIF(prior.requirement_ref, ''), prior.id) = COALESCE(NULLIF(current.requirement_ref, ''), current.id)
            AND prior.rowid < current.rowid) > 0
    THEN '#' || ((SELECT COUNT(*) FROM audit_requirement_coverage AS prior
                  WHERE prior.audit_id = current.audit_id
                    AND COALESCE(NULLIF(prior.requirement_ref, ''), prior.id) = COALESCE(NULLIF(current.requirement_ref, ''), current.id)
                    AND prior.rowid < current.rowid) + 1)
    ELSE ''
END;
UPDATE audit_findings SET status = 'OPEN', disposition = 'STILL_OPEN' WHERE status = 'STILL_OPEN';
CREATE UNIQUE INDEX idx_audit_findings_audit_logical ON audit_findings(audit_id, logical_finding_id);
CREATE UNIQUE INDEX idx_audit_coverage_audit_logical ON audit_requirement_coverage(audit_id, logical_coverage_id);
ALTER TABLE audits ADD COLUMN git_scope TEXT NOT NULL DEFAULT 'WORKING_TREE';
ALTER TABLE audits ADD COLUMN audited_base_sha TEXT;
ALTER TABLE audits ADD COLUMN audited_change_set_sha256 TEXT;
ALTER TABLE audits ADD COLUMN audited_session_id TEXT;
ALTER TABLE audits ADD COLUMN audited_prompt_version_id TEXT;
"#;

const AUDIT_FINAL_CLOSURE_FIELDS: &str = r#"
ALTER TABLE audits ADD COLUMN target_origin TEXT NOT NULL DEFAULT 'AUTO';
ALTER TABLE audit_findings ADD COLUMN inherited_prior_audit_id TEXT;
ALTER TABLE audit_findings ADD COLUMN inherited_prior_evidence_refs_json TEXT NOT NULL DEFAULT '[]';
CREATE INDEX idx_audit_findings_inherited_prior ON audit_findings(inherited_prior_audit_id, prior_finding_key);
"#;

const CONTROL_PLANE_FIELDS: &str = r#"
ALTER TABLE projects ADD COLUMN control_plane_schema TEXT;
ALTER TABLE projects ADD COLUMN control_plane_status TEXT NOT NULL DEFAULT 'UNADOPTED';
ALTER TABLE projects ADD COLUMN control_plane_revision TEXT;
ALTER TABLE projects ADD COLUMN control_plane_last_event_at TEXT;
ALTER TABLE projects ADD COLUMN control_plane_sync_status TEXT NOT NULL DEFAULT 'UNKNOWN';
ALTER TABLE projects ADD COLUMN control_plane_auto_ff INTEGER NOT NULL DEFAULT 0;
CREATE INDEX idx_projects_control_plane_status ON projects(control_plane_status, control_plane_sync_status);
"#;

const REMOTE_OBSERVATION_FIELDS: &str = r#"
ALTER TABLE projects ADD COLUMN last_remote_observation_at TEXT;
ALTER TABLE projects ADD COLUMN last_remote_observation_status TEXT NOT NULL DEFAULT 'UNKNOWN';
ALTER TABLE projects ADD COLUMN last_remote_observation_error TEXT;
ALTER TABLE projects ADD COLUMN last_remote_observed_upstream TEXT;
ALTER TABLE projects ADD COLUMN last_remote_observed_ahead INTEGER;
ALTER TABLE projects ADD COLUMN last_remote_observed_behind INTEGER;
ALTER TABLE projects ADD COLUMN last_remote_observed_diverged INTEGER;
"#;

const TRUTH_SYNC_FIELDS: &str = r#"
ALTER TABLE projects ADD COLUMN truth_sync_status TEXT NOT NULL DEFAULT 'CURRENT';
ALTER TABLE projects ADD COLUMN truth_sync_revision TEXT;
ALTER TABLE projects ADD COLUMN truth_sync_trigger TEXT;
ALTER TABLE projects ADD COLUMN truth_sync_error TEXT;
ALTER TABLE projects ADD COLUMN truth_sync_attempted_at TEXT;
ALTER TABLE projects ADD COLUMN truth_sync_completed_at TEXT;
ALTER TABLE projects ADD COLUMN truth_sync_retry_count INTEGER NOT NULL DEFAULT 0;
CREATE INDEX idx_projects_truth_sync ON projects(truth_sync_status, truth_sync_attempted_at);
"#;

const TRUTH_GENERATION_FIELDS: &str = r#"
ALTER TABLE projects ADD COLUMN truth_generation INTEGER NOT NULL DEFAULT 0;
ALTER TABLE projects ADD COLUMN truth_materialized_generation INTEGER NOT NULL DEFAULT 0;
CREATE INDEX idx_projects_truth_generation ON projects(truth_generation, truth_materialized_generation);
"#;

const TRUTH_GENERATION_BOOTSTRAP: &str = r#"
-- Physical adoption is authoritative. Startup/reconcile probes the bounded
-- on-disk contract before deciding whether generation zero may bootstrap.
SELECT 1 WHERE 0;
"#;

const PROJECT_REMOVAL_EXCLUSIONS: &str = r#"
CREATE TABLE github_project_exclusions (
    repository TEXT NOT NULL,
    branch TEXT NOT NULL,
    removed_at TEXT NOT NULL,
    PRIMARY KEY (repository, branch)
);
"#;

pub fn migrations() -> &'static [Migration] {
    &[
        Migration {
            version: 1,
            name: "initial_hiveai_schema",
            sql: INITIAL_SCHEMA,
        },
        Migration {
            version: 2,
            name: "initial_lookup_indexes",
            sql: INITIAL_INDEXES,
        },
        Migration {
            version: 3,
            name: "project_registry_fields",
            sql: PROJECT_REGISTRY_FIELDS,
        },
        Migration {
            version: 4,
            name: "project_snapshot_fields",
            sql: PROJECT_SNAPSHOT_FIELDS,
        },
        Migration {
            version: 5,
            name: "timestamp_standardization",
            sql: TIMESTAMP_STANDARDIZATION,
        },
        Migration {
            version: 6,
            name: "residual_timestamp_standardization",
            sql: RESIDUAL_TIMESTAMP_STANDARDIZATION,
        },
        Migration {
            version: 7,
            name: "project_snapshot_created_timestamp",
            sql: "UPDATE project_snapshots SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', CAST(created_at AS INTEGER), 'unixepoch') WHERE created_at GLOB '[0-9]*' AND created_at NOT GLOB '*[^0-9]*';",
        },
        Migration {
            version: 8,
            name: "project_preferred_agent_provider",
            sql: "ALTER TABLE projects ADD COLUMN preferred_agent_provider TEXT;",
        },
        Migration {
            version: 9,
            name: "agent_session_prompt_body",
            sql: "ALTER TABLE agent_sessions ADD COLUMN prompt_body TEXT;",
        },
        Migration {
            version: 10,
            name: "agent_session_final_response",
            sql: "ALTER TABLE agent_sessions ADD COLUMN final_response TEXT; ALTER TABLE agent_sessions ADD COLUMN final_response_truncated INTEGER NOT NULL DEFAULT 0; ALTER TABLE agent_sessions ADD COLUMN final_response_state TEXT NOT NULL DEFAULT 'UNAVAILABLE'; ALTER TABLE agent_sessions ADD COLUMN final_response_role TEXT;",
        },
        Migration {
            version: 11,
            name: "prompt_engine_fields",
            sql: PROMPT_ENGINE_FIELDS,
        },
        Migration {
            version: 12,
            name: "prompt_dispatch_reservations",
            sql: PROMPT_DISPATCH_FIELDS,
        },
        Migration {
            version: 13,
            name: "gpt_audit_engine_fields",
            sql: AUDIT_ENGINE_FIELDS,
        },
        Migration {
            version: 14,
            name: "audit_reaudit_provenance_and_freshness",
            sql: AUDIT_REAUDIT_FIELDS,
        },
        Migration {
            version: 15,
            name: "audit_evidence_logical_identity",
            sql: AUDIT_EVIDENCE_IDENTITY_FIELDS,
        },
        Migration {
            version: 16,
            name: "audit_comprehensive_identity_and_git_scope",
            sql: AUDIT_COMPREHENSIVE_IDENTITY_FIELDS,
        },
        Migration {
            version: 17,
            name: "audit_final_closure_target_and_inherited_provenance",
            sql: AUDIT_FINAL_CLOSURE_FIELDS,
        },
        Migration {
            version: 18,
            name: "unified_project_control_plane_metadata",
            sql: CONTROL_PLANE_FIELDS,
        },
        Migration {
            version: 19,
            name: "remote_observation_projection",
            sql: REMOTE_OBSERVATION_FIELDS,
        },
        Migration {
            version: 20,
            name: "durable_truth_sync_projection",
            sql: TRUTH_SYNC_FIELDS,
        },
        Migration {
            version: 21,
            name: "transactional_truth_generation",
            sql: TRUTH_GENERATION_FIELDS,
        },
        Migration {
            version: 22,
            name: "truth_generation_zero_bootstrap",
            sql: TRUTH_GENERATION_BOOTSTRAP,
        },
        Migration {
            version: 23,
            name: "github_project_removal_exclusions",
            sql: PROJECT_REMOVAL_EXCLUSIONS,
        },
    ]
}

pub fn apply_migrations(conn: &mut Connection, requested: &[Migration]) -> Result<MigrationReport> {
    validate_migration_list(requested)?;
    conn.execute_batch("PRAGMA foreign_keys = ON; CREATE TABLE IF NOT EXISTS migrations (version INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, applied_at TEXT NOT NULL);")?;

    let applied_rows: Vec<(i64, String)> = {
        let mut applied = conn.prepare("SELECT version, name FROM migrations ORDER BY version")?;
        let rows = applied
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<_>>>()?;
        rows
    };
    for (index, (version, name)) in applied_rows.iter().enumerate() {
        let expected = requested.get(index).ok_or_else(|| {
            validation_error(format!("database has unknown migration version {version}"))
        })?;
        if *version != expected.version || name != expected.name {
            return Err(validation_error(format!(
                "migration history mismatch at version {version}"
            )));
        }
    }

    let mut applied_any = false;
    for migration in requested.iter().skip(applied_rows.len()) {
        let tx = conn.transaction()?;
        tx.execute_batch(migration.sql)?;
        tx.execute("INSERT INTO migrations (version, name, applied_at) VALUES (?1, ?2, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))", params![migration.version, migration.name])?;
        tx.commit()?;
        applied_any = true;
    }

    let schema_version = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM migrations",
        [],
        |row| row.get(0),
    )?;
    let migration_count =
        conn.query_row("SELECT COUNT(*) FROM migrations", [], |row| row.get(0))?;
    Ok(MigrationReport {
        schema_version,
        migration_count,
        last_migration_status: if applied_any {
            "APPLIED"
        } else {
            "ALREADY_CURRENT"
        },
    })
}

fn validate_migration_list(requested: &[Migration]) -> Result<()> {
    for (index, migration) in requested.iter().enumerate() {
        let expected = index as i64 + 1;
        if migration.version != expected {
            return Err(validation_error(format!(
                "migration versions must be contiguous; expected {expected}, got {}",
                migration.version
            )));
        }
    }
    Ok(())
}

fn validation_error(message: String) -> rusqlite::Error {
    rusqlite::Error::ToSqlConversionFailure(Box::new(MigrationValidationError(message)))
}

#[derive(Debug)]
struct MigrationValidationError(String);

impl fmt::Display for MigrationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for MigrationValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::path::Path;
    use tempfile::tempdir;

    fn temp_connection() -> (tempfile::TempDir, Connection) {
        let directory = tempdir().expect("temp directory");
        let connection =
            Connection::open(directory.path().join("test-hiveai.db")).expect("temp database");
        (directory, connection)
    }

    #[test]
    fn fresh_database_reaches_latest_version() {
        let (_directory, mut connection) = temp_connection();
        let report = apply_migrations(&mut connection, migrations()).expect("migrations apply");
        assert_eq!(report.schema_version, 23);
        assert_eq!(report.migration_count, 23);
        assert_eq!(report.last_migration_status, "APPLIED");
    }

    #[test]
    fn rerunning_migrations_is_idempotent() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("first apply");
        let report = apply_migrations(&mut connection, migrations()).expect("second apply");
        assert_eq!(report.last_migration_status, "ALREADY_CURRENT");
        assert_eq!(report.migration_count, 23);
    }

    #[test]
    fn migration_v22_defers_generation_bootstrap_to_physical_convergence() {
        let (directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, &migrations()[..21]).expect("v1-v21 apply");
        for index in 0..8 {
            connection
                .execute(
                    "INSERT INTO projects (id, name, local_path, normalized_path, status, control_plane_status, created_at, updated_at) VALUES (?1, ?2, ?3, ?3, 'ACTIVE', 'ADOPTED', 'now', 'now')",
                    params![
                        format!("portfolio-{index}"),
                        format!("Portfolio {index}"),
                        directory.path().to_string_lossy()
                    ],
                )
                .unwrap();
        }
        connection
            .execute(
                "INSERT INTO projects (id, name, status, control_plane_status, created_at, updated_at) VALUES ('unadopted', 'Unadopted', 'ACTIVE', 'UNADOPTED', 'now', 'now')",
                [],
            )
            .unwrap();
        apply_migrations(&mut connection, migrations()).expect("v22 apply");
        let adopted: Vec<(i64, i64, String, Option<String>)> = connection
            .prepare("SELECT truth_generation, truth_materialized_generation, truth_sync_status, truth_sync_trigger FROM projects WHERE id LIKE 'portfolio-%' ORDER BY id")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)))
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(adopted.len(), 8);
        assert!(adopted
            .iter()
            .all(|row| row == &(0, 0, "CURRENT".into(), None)));
        let unadopted: (i64, i64, String) = connection
            .query_row(
                "SELECT truth_generation, truth_materialized_generation, truth_sync_status FROM projects WHERE id='unadopted'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(unadopted, (0, 0, "CURRENT".into()));
    }

    #[test]
    fn migration_v7_converts_numeric_snapshot_timestamp_from_v6_fixture() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, &migrations()[..6]).expect("v1-v6 apply");
        connection
            .execute(
                "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p7', 'P7', 'now', 'now')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO project_snapshots (id, project_id, availability, evidence_generated_at, watcher_health, created_at) VALUES ('s7', 'p7', 'AVAILABLE', 'now', 'HEALTHY', '1700000000')",
                [],
            )
            .unwrap();
        apply_migrations(&mut connection, migrations()).expect("v7 apply");
        let value: String = connection
            .query_row(
                "SELECT created_at FROM project_snapshots WHERE id = 's7'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(value.starts_with("2023-11-14T22:13:20."));
    }

    #[test]
    fn migration_v7_preserves_valid_and_malformed_snapshot_timestamps() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, &migrations()[..6]).expect("v1-v6 apply");
        connection
            .execute(
                "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p8', 'P8', 'now', 'now')",
                [],
            )
            .unwrap();
        for (id, value) in [
            ("valid", "2024-01-01T00:00:00.123Z"),
            ("bad", "not-a-timestamp"),
        ] {
            connection
                .execute(
                    "INSERT INTO project_snapshots (id, project_id, availability, evidence_generated_at, watcher_health, created_at) VALUES (?1, 'p8', 'AVAILABLE', 'now', 'HEALTHY', ?2)",
                    params![id, value],
                )
                .unwrap();
        }
        apply_migrations(&mut connection, migrations()).expect("v7 apply");
        let values: Vec<String> = connection
            .prepare("SELECT created_at FROM project_snapshots ORDER BY id")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(values, vec!["not-a-timestamp", "2024-01-01T00:00:00.123Z"]);
    }

    #[test]
    fn migration_v7_real_fixture_is_idempotent_and_history_safe() {
        let (_directory, mut connection) = temp_connection();
        let first = apply_migrations(&mut connection, migrations()).expect("first apply");
        let second = apply_migrations(&mut connection, migrations()).expect("rerun");
        assert_eq!(first.schema_version, 23);
        assert_eq!(second.last_migration_status, "ALREADY_CURRENT");
        let mismatch = [Migration {
            version: 1,
            name: "wrong",
            sql: "",
        }];
        assert!(apply_migrations(&mut connection, &mismatch).is_err());
    }

    #[test]
    fn migration_history_is_inspectable() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        let rows: Vec<(i64, String)> = connection
            .prepare("SELECT version, name FROM migrations ORDER BY version")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(
            rows,
            vec![
                (1, "initial_hiveai_schema".to_string()),
                (2, "initial_lookup_indexes".to_string()),
                (3, "project_registry_fields".to_string()),
                (4, "project_snapshot_fields".to_string()),
                (5, "timestamp_standardization".to_string()),
                (6, "residual_timestamp_standardization".to_string()),
                (7, "project_snapshot_created_timestamp".to_string()),
                (8, "project_preferred_agent_provider".to_string()),
                (9, "agent_session_prompt_body".to_string()),
                (10, "agent_session_final_response".to_string()),
                (11, "prompt_engine_fields".to_string()),
                (12, "prompt_dispatch_reservations".to_string()),
                (13, "gpt_audit_engine_fields".to_string()),
                (14, "audit_reaudit_provenance_and_freshness".to_string()),
                (15, "audit_evidence_logical_identity".to_string()),
                (16, "audit_comprehensive_identity_and_git_scope".to_string()),
                (
                    17,
                    "audit_final_closure_target_and_inherited_provenance".to_string(),
                ),
                (18, "unified_project_control_plane_metadata".to_string()),
                (19, "remote_observation_projection".to_string()),
                (20, "durable_truth_sync_projection".to_string()),
                (21, "transactional_truth_generation".to_string()),
                (22, "truth_generation_zero_bootstrap".to_string()),
                (23, "github_project_removal_exclusions".to_string()),
            ]
        );
    }

    #[test]
    fn migration_v14_backfills_prefixed_and_legacy_evidence_ids() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, &migrations()[..14]).expect("v1-v14 apply");
        connection
            .execute(
                "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p15', 'P15', 'now', 'now')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audits (id, project_id, result, summary, confidence, created_at) VALUES ('audit-prefixed', 'p15', 'PASS', 'fixture', 1.0, 'now'), ('audit-legacy', 'p15', 'PASS', 'fixture', 1.0, 'now')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audit_evidence (id, audit_id, evidence_kind, verification_status, summary, created_at) VALUES ('audit-prefixed:TEST_RUN:proof', 'audit-prefixed', 'TEST_RUN', 'VERIFIED', 'proof', 'now'), ('legacy-evidence', 'audit-legacy', 'TEST_RUN', 'VERIFIED', 'legacy', 'now')",
                [],
            )
            .unwrap();
        apply_migrations(&mut connection, migrations()).expect("v15 apply");
        let rows: Vec<(String, String)> = connection
            .prepare("SELECT id, logical_evidence_id FROM audit_evidence ORDER BY id")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(
            rows,
            vec![
                (
                    "audit-prefixed:TEST_RUN:proof".into(),
                    "TEST_RUN:proof".into()
                ),
                ("legacy-evidence".into(), "legacy-evidence".into()),
            ]
        );
    }

    #[test]
    fn migration_v16_backfills_scoped_logical_ids_and_preserves_global_rows() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, &migrations()[..15]).expect("v1-v15 apply");
        connection
            .execute(
                "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p16', 'P16', 'now', 'now')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audits (id, project_id, result, summary, confidence, created_at) VALUES ('audit-16-a', 'p16', 'FAIL', 'fixture', 0.0, 'now'), ('audit-16-b', 'p16', 'FAIL', 'fixture', 0.0, 'now')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audit_findings (id, audit_id, severity, title, created_at, finding_key, status) VALUES ('finding-a-1', 'audit-16-a', 'MAJOR', 'same key', 'now', 'same-key', 'STILL_OPEN'), ('finding-a-2', 'audit-16-a', 'MAJOR', 'same key again', 'now', 'same-key', 'OPEN'), ('finding-b-1', 'audit-16-b', 'MAJOR', 'same key other audit', 'now', 'same-key', 'OPEN')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audit_requirement_coverage (id, audit_id, requirement_ref, requirement_text, status, evidence_refs_json, rationale, created_at) VALUES ('coverage-a-1', 'audit-16-a', 'req', 'req', 'VERIFIED', '[]', 'fixture', 'now'), ('coverage-a-2', 'audit-16-a', 'req', 'req again', 'UNVERIFIED', '[]', 'fixture', 'now')",
                [],
            )
            .unwrap();
        apply_migrations(&mut connection, migrations()).expect("v16 apply");
        let findings: Vec<(String, String, String)> = connection
            .prepare("SELECT id, logical_finding_id, status FROM audit_findings WHERE audit_id='audit-16-a' ORDER BY id")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(findings[0].1, "same-key");
        assert_eq!(findings[1].1, "same-key#2");
        assert_eq!(findings[0].2, "OPEN");
        let other_audit: String = connection
            .query_row(
                "SELECT logical_finding_id FROM audit_findings WHERE id='finding-b-1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(other_audit, "same-key");
        let coverage: Vec<String> = connection
            .prepare("SELECT logical_coverage_id FROM audit_requirement_coverage WHERE audit_id='audit-16-a' ORDER BY id")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(coverage, vec!["req", "req#2"]);
        let scope: String = connection
            .query_row(
                "SELECT git_scope FROM audits WHERE id='audit-16-a'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(scope, "WORKING_TREE");
    }

    #[test]
    fn migration_v13_backfills_historical_evidence_without_rewriting_rows() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, &migrations()[..13]).expect("v1-v13 apply");
        connection
            .execute(
                "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p13', 'P13', 'now', 'now')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audits (id, project_id, result, summary, confidence, created_at) VALUES ('audit-v13', 'p13', 'PASS', 'fixture', 1.0, 'now')",
                [],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO audit_evidence (id, audit_id, evidence_kind, verification_status, summary, created_at) VALUES ('TEST_RUN:proof', 'audit-v13', 'TEST_RUN', 'VERIFIED', 'proof', 'now')",
                [],
            )
            .unwrap();
        apply_migrations(&mut connection, migrations()).expect("v14-v15 apply");
        let logical: String = connection
            .query_row(
                "SELECT logical_evidence_id FROM audit_evidence WHERE id='TEST_RUN:proof'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(logical, "TEST_RUN:proof");
    }

    #[test]
    fn prompt_body_migration_is_nullable_and_backward_compatible() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        connection
            .execute(
                "INSERT INTO agent_sessions (id, project_id, provider, state, created_at) VALUES ('legacy', NULL, 'CODEX', 'COMPLETED', 'now')",
                [],
            )
            .expect("legacy session insert");
        let value: Option<String> = connection
            .query_row(
                "SELECT prompt_body FROM agent_sessions WHERE id='legacy'",
                [],
                |row| row.get(0),
            )
            .expect("prompt body column is readable");
        assert_eq!(value, None);
    }

    #[test]
    fn foreign_keys_are_enabled() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        assert_eq!(
            connection
                .query_row("PRAGMA foreign_keys", [], |row| row.get::<_, i64>(0))
                .unwrap(),
            1
        );
    }

    #[test]
    fn required_tables_and_indexes_exist() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        for table in [
            "projects",
            "repositories",
            "project_sources",
            "git_snapshots",
            "tasks",
            "task_dependencies",
            "task_sources",
            "task_events",
            "prompts",
            "prompt_versions",
            "agent_sessions",
            "agent_events",
            "agent_tool_calls",
            "permission_requests",
            "audits",
            "audit_findings",
            "test_runs",
            "alerts",
            "decisions",
            "github_sync_state",
            "settings",
            "audit_evidence",
            "audit_requirement_coverage",
            "github_project_exclusions",
            "migrations",
        ] {
            assert_eq!(
                connection
                    .query_row(
                        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                        [table],
                        |row| row.get::<_, i64>(0)
                    )
                    .unwrap(),
                1,
                "missing table {table}"
            );
        }
        for index in [
            "idx_tasks_project_state",
            "idx_task_dependencies_dependency",
            "idx_prompt_versions_prompt",
            "idx_prompt_versions_dispatch_state",
            "idx_prompt_versions_dispatch_reservation",
            "idx_agent_sessions_project_state",
            "idx_audit_findings_audit",
            "idx_audit_evidence_audit_logical",
            "idx_audit_findings_audit_logical",
            "idx_audit_coverage_audit_logical",
            "idx_github_sync_project",
        ] {
            assert_eq!(
                connection
                    .query_row(
                        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'index' AND name = ?1",
                        [index],
                        |row| row.get::<_, i64>(0)
                    )
                    .unwrap(),
                1,
                "missing index {index}"
            );
        }
    }

    #[test]
    fn migration_failure_rolls_back_transaction() {
        let (_directory, mut connection) = temp_connection();
        let failing = [
            Migration {
                version: 1,
                name: "ok",
                sql: "CREATE TABLE safe_table (id TEXT PRIMARY KEY);",
            },
            Migration {
                version: 2,
                name: "fails",
                sql:
                    "CREATE TABLE broken_table (id TEXT PRIMARY KEY); SELECT * FROM missing_table;",
            },
        ];
        assert!(apply_migrations(&mut connection, &failing).is_err());
        assert_eq!(connection.query_row("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'safe_table'", [], |row| row.get::<_, i64>(0)).unwrap(), 1);
        assert_eq!(connection.query_row("SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'broken_table'", [], |row| row.get::<_, i64>(0)).unwrap(), 0);
        assert_eq!(
            connection
                .query_row("SELECT COUNT(*) FROM migrations", [], |row| row
                    .get::<_, i64>(0))
                .unwrap(),
            1
        );
    }

    #[test]
    fn incorrect_versioned_database_fails_safely() {
        let (_directory, mut connection) = temp_connection();
        connection.execute_batch("CREATE TABLE migrations (version INTEGER PRIMARY KEY NOT NULL, name TEXT NOT NULL, applied_at TEXT NOT NULL); INSERT INTO migrations VALUES (2, 'wrong', 'now');").unwrap();
        assert!(apply_migrations(&mut connection, migrations()).is_err());
    }

    #[test]
    fn tests_use_isolated_temp_path_not_repository_path() {
        let (directory, _connection) = temp_connection();
        assert!(directory.path().is_dir());
        assert!(!directory
            .path()
            .starts_with(Path::new(env!("CARGO_MANIFEST_DIR"))));
    }

    #[test]
    fn representative_foreign_key_relationships_are_enforced() {
        let (_directory, mut connection) = temp_connection();
        apply_migrations(&mut connection, migrations()).expect("migrations apply");
        connection.execute("INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p1', 'Project', 'now', 'now')", []).unwrap();
        connection.execute("INSERT INTO repositories (id, project_id, created_at, updated_at) VALUES ('r1', 'p1', 'now', 'now')", []).unwrap();
        connection.execute("INSERT INTO tasks (id, project_id, title, state, created_at, updated_at) VALUES ('t1', 'p1', 'Task', 'BACKLOG', 'now', 'now')", []).unwrap();
        connection.execute("INSERT INTO prompts (id, project_id, task_id, kind, created_at, updated_at) VALUES ('pr1', 'p1', 't1', 'BUILD', 'now', 'now')", []).unwrap();
        connection.execute("INSERT INTO prompt_versions (id, prompt_id, version, content, created_by, created_at) VALUES ('pv1', 'pr1', 1, 'content', 'test', 'now')", []).unwrap();
        connection.execute("INSERT INTO audits (id, project_id, task_id, result, created_at) VALUES ('a1', 'p1', 't1', 'PASS', 'now')", []).unwrap();
        connection.execute("INSERT INTO audit_findings (id, audit_id, severity, title, created_at) VALUES ('f1', 'a1', 'LOW', 'Finding', 'now')", []).unwrap();
        assert!(connection.execute("INSERT INTO repositories (id, project_id, created_at, updated_at) VALUES ('bad', 'missing', 'now', 'now')", []).is_err());
        assert!(connection.execute("INSERT INTO prompt_versions (id, prompt_id, version, content, created_by, created_at) VALUES ('badpv', 'missing', 1, 'content', 'test', 'now')", []).is_err());
        assert!(connection.execute("INSERT INTO audit_findings (id, audit_id, severity, title, created_at) VALUES ('badf', 'missing', 'LOW', 'Finding', 'now')", []).is_err());
    }
}
