use crate::codex_runtime::{
    probe_login_status, probe_version, resolve_codex_executable, run_bounded_process, LoginState,
    ProbeError, READINESS_TIMEOUT,
};
use crate::db::DatabaseState;
use crate::git_engine::{self, GitDiffRequest, GitDiffScope, GitSnapshot, GitSnapshotRequest};
use crate::project_dashboard;
use crate::projects::fetch_project;
use crate::prompt_engine::{self, PromptGenerateRequest, PromptKind, PromptVersion};
use crate::task_intelligence;
use crate::time::utc_timestamp;
use crate::workflow::{self, WorkflowProjectListQuery, WorkflowTask};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

pub const AUDIT_SCHEMA_VERSION: i64 = 1;
pub const MAX_AUDIT_INPUT_BYTES: usize = 128 * 1024;
pub const MAX_EVIDENCE_ITEMS: usize = 128;
pub const MAX_GIT_DIFF_BYTES: usize = 96 * 1024;
pub const MAX_SOURCE_SNIPPETS: usize = 32;
pub const MAX_SOURCE_SNIPPET_BYTES: usize = 8 * 1024;
pub const MAX_TEST_SNIPPETS: usize = 32;
pub const MAX_BUILDER_LOG_CLAIM_BYTES: usize = 8 * 1024;
pub const MAX_BUILDER_LOG_TOTAL_BYTES: usize = 32 * 1024;
pub const MAX_TEST_SCAN_BYTES: usize = 64 * 1024;
pub const MAX_TEST_SCAN_LINES: usize = 4096;
pub const MAX_REQUIREMENTS: usize = 64;
pub const MAX_FINDINGS: usize = 64;
pub const MAX_MODEL_OUTPUT_BYTES: usize = 128 * 1024;
pub const MAX_MODEL_SUMMARY_BYTES: usize = 4096;
pub const MAX_LOCATOR_BYTES: usize = 1024;
pub const MAX_HISTORY: usize = 100;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationStatus {
    Verified,
    Corroborated,
    ClaimOnly,
    Unverified,
    Unavailable,
    Stale,
    Truncated,
    Excluded,
    Partial,
}

impl VerificationStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "VERIFIED",
            Self::Corroborated => "CORROBORATED",
            Self::ClaimOnly => "CLAIM_ONLY",
            Self::Unverified => "UNVERIFIED",
            Self::Unavailable => "UNAVAILABLE",
            Self::Stale => "STALE",
            Self::Truncated => "TRUNCATED",
            Self::Excluded => "EXCLUDED",
            Self::Partial => "PARTIAL",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditVerdict {
    Pass,
    Conditional,
    Fail,
}

impl AuditVerdict {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Conditional => "CONDITIONAL",
            Self::Fail => "FAIL",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FindingSeverity {
    Blocker,
    Major,
    Minor,
    Note,
}

impl FindingSeverity {
    fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "BLOCKER",
            Self::Major => "MAJOR",
            Self::Minor => "MINOR",
            Self::Note => "NOTE",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CoverageStatus {
    Verified,
    Partial,
    Unverified,
    Failed,
    NotApplicable,
}

impl CoverageStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "VERIFIED",
            Self::Partial => "PARTIAL",
            Self::Unverified => "UNVERIFIED",
            Self::Failed => "FAILED",
            Self::NotApplicable => "NOT_APPLICABLE",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
}

impl ConfidenceLevel {
    fn as_str(self) -> &'static str {
        match self {
            Self::High => "HIGH",
            Self::Medium => "MEDIUM",
            Self::Low => "LOW",
        }
    }
    fn score(self) -> f64 {
        match self {
            Self::High => 1.0,
            Self::Medium => 0.5,
            Self::Low => 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegressionRisk {
    Low,
    Medium,
    High,
    Critical,
}

impl RegressionRisk {
    fn as_str(self) -> &'static str {
        match self {
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
            Self::Critical => "CRITICAL",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditState {
    Preparing,
    Ready,
    Running,
    Completed,
    Failed,
    Stale,
    Cancelled,
}

impl AuditState {
    fn as_str(self) -> &'static str {
        match self {
            Self::Preparing => "PREPARING",
            Self::Ready => "READY",
            Self::Running => "RUNNING",
            Self::Completed => "COMPLETED",
            Self::Failed => "FAILED",
            Self::Stale => "STALE",
            Self::Cancelled => "CANCELLED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditInputRequest {
    pub project_id: String,
    pub task_id: Option<String>,
    pub prior_audit_id: Option<String>,
    #[serde(default)]
    pub git_target: AuditGitTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditGitTarget {
    pub scope: GitDiffScope,
    pub base_ref: Option<String>,
    pub head_sha: Option<String>,
    #[serde(default)]
    pub target_origin: AuditTargetOrigin,
    pub audited_session_id: Option<String>,
    pub audited_prompt_version_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditTargetOrigin {
    #[default]
    Auto,
    AgentSession,
    PriorAudit,
    RegisteredPolicy,
    Manual,
}

impl Default for AuditGitTarget {
    fn default() -> Self {
        Self {
            scope: GitDiffScope::WorkingTree,
            base_ref: None,
            head_sha: None,
            target_origin: AuditTargetOrigin::Auto,
            audited_session_id: None,
            audited_prompt_version_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRequirement {
    pub requirement_ref: String,
    pub requirement_text: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditTaskEvidence {
    pub task_id: String,
    pub title: String,
    pub workflow_state: String,
    pub requirements: Vec<String>,
    pub dependencies: Vec<String>,
    pub blockers: Vec<String>,
    pub required_actor: Option<String>,
    pub milestone: Option<String>,
    pub source_path: Option<String>,
    pub source_hash: Option<String>,
    pub identity_status: VerificationStatus,
    pub requirements_status: VerificationStatus,
    pub requirements_provenance: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditGitEvidence {
    pub scope: GitDiffScope,
    pub target_origin: AuditTargetOrigin,
    pub branch: Option<String>,
    pub head_sha: Option<String>,
    pub baseline_ref: Option<String>,
    pub base_sha: Option<String>,
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    pub untracked_files: Vec<String>,
    pub conflicted_files: Vec<String>,
    pub diff: Option<String>,
    pub diff_truncated: bool,
    pub repository_identity: Option<String>,
    pub changed_files: Vec<String>,
    pub full_change_set_sha256: Option<String>,
    pub staged_content_sha256: Option<String>,
    pub working_tracked_content_sha256: Option<String>,
    pub untracked_content_sha256: Option<String>,
    pub conflict_identity: Option<String>,
    pub committed_range_identity: Option<String>,
    pub identity_complete: bool,
    pub identity_diagnostic: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditEvidence {
    pub id: String,
    pub logical_evidence_id: String,
    pub kind: String,
    pub verification_status: VerificationStatus,
    pub locator: Option<String>,
    pub summary: String,
    pub content: Option<String>,
    pub content_sha256: Option<String>,
    pub byte_count: usize,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditInput {
    pub project_id: String,
    pub task_id: Option<String>,
    pub task: Option<AuditTaskEvidence>,
    pub requirements: Vec<AuditRequirement>,
    pub git: AuditGitEvidence,
    pub evidence: Vec<AuditEvidence>,
    pub audited_branch: Option<String>,
    pub audited_head_sha: Option<String>,
    pub baseline_ref: Option<String>,
    pub collected_at: String,
    pub input_manifest_sha256: String,
    pub freshness_token: String,
    pub audited_session_id: Option<String>,
    pub audited_prompt_version_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriorFindingDispositionKind {
    StillOpen,
    Closed,
    Superseded,
}

impl PriorFindingDispositionKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::StillOpen => "STILL_OPEN",
            Self::Closed => "CLOSED",
            Self::Superseded => "SUPERSEDED",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorFindingDisposition {
    pub prior_finding_key: String,
    pub disposition: PriorFindingDispositionKind,
    pub evidence_refs: Vec<String>,
    pub rationale: String,
    pub replacement_finding_key: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequirementCoverage {
    pub id: String,
    pub logical_coverage_id: String,
    pub requirement_ref: String,
    pub requirement_text: String,
    pub status: CoverageStatus,
    pub evidence_refs: Vec<String>,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditFinding {
    pub id: String,
    pub logical_finding_id: String,
    pub finding_key: String,
    pub severity: FindingSeverity,
    pub title: String,
    pub detail: String,
    pub requirement_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub source_locator: Option<String>,
    pub test_locator: Option<String>,
    pub confidence: ConfidenceLevel,
    pub status: String,
    pub remediation_guidance: String,
    pub blocks_release: bool,
    pub closed_by_audit_id: Option<String>,
    pub prior_finding_key: Option<String>,
    pub disposition: Option<String>,
    pub disposition_evidence_refs: Vec<String>,
    pub disposition_rationale: Option<String>,
    pub superseded_by_finding_key: Option<String>,
    pub inherited_prior_audit_id: Option<String>,
    pub inherited_prior_evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRun {
    pub id: String,
    pub project_id: String,
    pub task_id: Option<String>,
    pub audit_type: String,
    pub audited_branch: Option<String>,
    pub audited_head_sha: Option<String>,
    pub baseline_ref: Option<String>,
    pub git_scope: GitDiffScope,
    pub target_origin: AuditTargetOrigin,
    pub audited_base_sha: Option<String>,
    pub audited_change_set_sha256: Option<String>,
    pub audited_session_id: Option<String>,
    pub audited_prompt_version_id: Option<String>,
    pub input_manifest_sha256: String,
    pub freshness_token: String,
    pub schema_version: i64,
    pub verdict: AuditVerdict,
    pub confidence: ConfidenceLevel,
    pub regression_risk: RegressionRisk,
    pub state: AuditState,
    pub summary: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub auditor_provider: Option<String>,
    pub auditor_model: Option<String>,
    pub auditor_version: Option<String>,
    pub model_status: String,
    pub diagnostic: Option<String>,
    pub prior_audit_id: Option<String>,
    pub remediation_prompt_id: Option<String>,
    pub remediation_prompt_version_id: Option<String>,
    pub remediation_session_id: Option<String>,
    pub findings: Vec<AuditFinding>,
    pub coverage: Vec<RequirementCoverage>,
    pub evidence: Vec<AuditEvidence>,
}

#[derive(Debug, Clone)]
pub struct AuditEvaluation {
    pub verdict: AuditVerdict,
    pub confidence: ConfidenceLevel,
    pub regression_risk: RegressionRisk,
    pub summary: String,
    pub findings: Vec<AuditFinding>,
    pub prior_finding_dispositions: Vec<PriorFindingDisposition>,
    pub coverage: Vec<RequirementCoverage>,
    pub model_status: String,
    pub diagnostic: Option<String>,
    pub auditor_provider: Option<String>,
    pub auditor_model: Option<String>,
    pub auditor_version: Option<String>,
}

pub trait AuditModel {
    fn provider(&self) -> String;
    fn model(&self) -> String;
    fn version(&self) -> String;
    fn evaluate(&self, input: &AuditInput) -> Result<String, String>;
}

pub struct UnavailableAuditModel;

impl AuditModel for UnavailableAuditModel {
    fn provider(&self) -> String {
        "CODEX_CLI".into()
    }
    fn model(&self) -> String {
        CODEX_DEFAULT_MODEL.into()
    }
    fn version(&self) -> String {
        "UNAVAILABLE".into()
    }
    fn evaluate(&self, _input: &AuditInput) -> Result<String, String> {
        Err("AUDIT_MODEL_UNAVAILABLE: no supported Codex CLI audit provider is available".into())
    }
}

#[cfg(test)]
pub struct FixtureAuditModel {
    pub response: String,
}

#[cfg(test)]
impl AuditModel for FixtureAuditModel {
    fn provider(&self) -> String {
        "FIXTURE".into()
    }
    fn model(&self) -> String {
        "deterministic-fixture".into()
    }
    fn version(&self) -> String {
        "1".into()
    }
    fn evaluate(&self, _input: &AuditInput) -> Result<String, String> {
        Ok(self.response.clone())
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditProviderReadiness {
    pub provider: String,
    pub status: String,
    pub configured: bool,
    pub executable_available: bool,
    pub version: Option<String>,
    pub login_state: Option<String>,
    pub model: Option<String>,
    pub credential_source: Option<String>,
    pub error_category: Option<String>,
}

fn audit_result_schema() -> Value {
    json!({
        "type": "object",
        "additionalProperties": false,
        "properties": {
            "verdict": { "type": "string", "enum": ["PASS", "CONDITIONAL", "FAIL"] },
            "confidence": { "type": "string", "enum": ["HIGH", "MEDIUM", "LOW"] },
            "regressionRisk": { "type": "string", "enum": ["LOW", "MEDIUM", "HIGH", "CRITICAL"] },
            "summary": { "type": "string" },
            "findings": { "type": "array", "items": { "type": "object", "additionalProperties": false, "properties": {
                "findingKey": { "type": "string" }, "severity": { "type": "string", "enum": ["BLOCKER", "MAJOR", "MINOR", "NOTE"] }, "title": { "type": "string" }, "detail": { "type": "string" }, "requirementRefs": { "type": "array", "items": { "type": "string" } }, "evidenceRefs": { "type": "array", "items": { "type": "string" } }, "sourceLocator": { "type": ["string", "null"] }, "testLocator": { "type": ["string", "null"] }, "remediationGuidance": { "type": "string" }, "blocksRelease": { "type": "boolean" }
            }, "required": ["findingKey", "severity", "title", "detail", "requirementRefs", "evidenceRefs", "sourceLocator", "testLocator", "remediationGuidance", "blocksRelease"] } },
            "requirementCoverage": { "type": "array", "items": { "type": "object", "additionalProperties": false, "properties": {
                "requirementRef": { "type": "string" }, "requirementText": { "type": "string" }, "status": { "type": "string", "enum": ["VERIFIED", "PARTIAL", "UNVERIFIED", "FAILED", "NOT_APPLICABLE"] }, "evidenceRefs": { "type": "array", "items": { "type": "string" } }, "rationale": { "type": "string" }
            }, "required": ["requirementRef", "requirementText", "status", "evidenceRefs", "rationale"] } },
            "priorFindingDispositions": { "type": "array", "items": { "type": "object", "additionalProperties": false, "properties": {
                "priorFindingKey": { "type": "string" }, "disposition": { "type": "string", "enum": ["STILL_OPEN", "CLOSED", "SUPERSEDED"] }, "evidenceRefs": { "type": "array", "items": { "type": "string" } }, "rationale": { "type": "string" }, "replacementFindingKey": { "type": ["string", "null"] }
            }, "required": ["priorFindingKey", "disposition", "evidenceRefs", "rationale", "replacementFindingKey"] } }
        },
        "required": ["verdict", "confidence", "regressionRisk", "summary", "findings", "requirementCoverage", "priorFindingDispositions"]
    })
}

const CODEX_DEFAULT_MODEL: &str = "CLI_DEFAULT";
const CODEX_AUDIT_TIMEOUT: Duration = Duration::from_secs(120);
const CODEX_READINESS_TIMEOUT: Duration = Duration::from_secs(20);
const CODEX_MAX_STDOUT_BYTES: usize = 64 * 1024;
const CODEX_MAX_STDERR_BYTES: usize = 16 * 1024;
const CODEX_MAX_FINAL_BYTES: usize = MAX_MODEL_OUTPUT_BYTES;

#[derive(Debug, Clone)]
struct CodexProcessRequest {
    prompt: String,
    schema: Option<Value>,
    model: Option<String>,
    timeout: Duration,
}

#[derive(Debug, Clone)]
struct CodexProcessResult {
    stdout: String,
    stderr: String,
    final_message: Option<String>,
    exit_code: Option<i32>,
    timed_out: bool,
    stdout_truncated: bool,
    stderr_truncated: bool,
}

trait CodexProcessRunner: Send + Sync {
    fn run(&self, request: &CodexProcessRequest) -> Result<CodexProcessResult, String>;
}

struct NativeCodexProcessRunner {
    executable: PathBuf,
}

impl CodexProcessRunner for NativeCodexProcessRunner {
    fn run(&self, request: &CodexProcessRequest) -> Result<CodexProcessResult, String> {
        let directory = std::env::temp_dir().join(format!("hiveai-codex-audit-{}", Uuid::new_v4()));
        fs::create_dir(&directory).map_err(|_| {
            "AUDIT_CODEX_PROCESS_ERROR: dedicated temporary audit directory unavailable".to_string()
        })?;
        let schema_path = directory.join("audit-result-schema.json");
        let final_path = directory.join("audit-final-message.json");
        let result = (|| {
            if let Some(schema) = &request.schema {
                let mut file = File::create(&schema_path).map_err(|_| {
                    "AUDIT_CODEX_PROCESS_ERROR: output schema file unavailable".to_string()
                })?;
                let bytes = serde_json::to_vec(schema).map_err(|_| {
                    "AUDIT_CODEX_PROCESS_ERROR: output schema serialization failed".to_string()
                })?;
                file.write_all(&bytes).map_err(|_| {
                    "AUDIT_CODEX_PROCESS_ERROR: output schema write failed".to_string()
                })?;
            }
            let args = build_codex_audit_args(
                &directory,
                &schema_path,
                &final_path,
                request.model.as_deref(),
                request.schema.is_some(),
            );
            let mut command = crate::process_policy::background_command(&self.executable);
            command.args(args).current_dir(&directory);
            let process = run_bounded_process(
                command,
                Some(request.prompt.as_bytes()),
                request.timeout,
                CODEX_MAX_STDOUT_BYTES,
                CODEX_MAX_STDERR_BYTES,
            )
            .map_err(|_| {
                "AUDIT_CODEX_PROCESS_ERROR: Codex process could not be started or observed"
                    .to_string()
            })?;
            let final_message = read_bounded_final_message(&final_path)?;
            Ok(CodexProcessResult {
                stdout: String::from_utf8_lossy(&process.output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&process.output.stderr).into_owned(),
                final_message,
                exit_code: process.output.status.code(),
                timed_out: process.timed_out,
                stdout_truncated: process.stdout_truncated,
                stderr_truncated: process.stderr_truncated,
            })
        })();
        let _ = fs::remove_dir_all(&directory);
        result
    }
}

fn read_bounded_final_message(path: &Path) -> Result<Option<String>, String> {
    let Ok(bytes) = fs::read(path) else {
        return Ok(None);
    };
    if bytes.len() > CODEX_MAX_FINAL_BYTES {
        return Err(
            "AUDIT_CODEX_FINAL_OUTPUT_TRUNCATED: dedicated final result exceeded its bound".into(),
        );
    }
    String::from_utf8(bytes).map(Some).map_err(|_| {
        "AUDIT_CODEX_FINAL_OUTPUT_MALFORMED: dedicated final result was not UTF-8".into()
    })
}

fn build_codex_audit_args(
    directory: &Path,
    schema_path: &Path,
    final_path: &Path,
    model: Option<&str>,
    with_schema: bool,
) -> Vec<String> {
    let mut args = vec![
        "exec".into(),
        "--ephemeral".into(),
        "--json".into(),
        "--sandbox".into(),
        "read-only".into(),
        "--skip-git-repo-check".into(),
        "--ignore-user-config".into(),
        "--ignore-rules".into(),
        "--color".into(),
        "never".into(),
        "--cd".into(),
        directory.to_string_lossy().into_owned(),
        "--output-last-message".into(),
        final_path.to_string_lossy().into_owned(),
    ];
    if with_schema {
        args.extend([
            "--output-schema".into(),
            schema_path.to_string_lossy().into_owned(),
        ]);
    }
    if let Some(model) = model {
        args.extend(["--model".into(), model.to_string()]);
    }
    args
}

struct CodexCliAuditModel {
    version: String,
    runner: Arc<dyn CodexProcessRunner>,
}

impl AuditModel for CodexCliAuditModel {
    fn provider(&self) -> String {
        "CODEX_CLI".into()
    }

    fn model(&self) -> String {
        CODEX_DEFAULT_MODEL.into()
    }

    fn version(&self) -> String {
        self.version.clone()
    }

    fn evaluate(&self, input: &AuditInput) -> Result<String, String> {
        let input_json = serde_json::to_string(input)
            .map_err(|_| "AUDIT_CODEX_INPUT_SERIALIZATION_FAILED".to_string())?;
        let prompt = format!(
            "You are the independent H!veAI audit provider. The supplied JSON AuditInput is the complete and authoritative evidence for this audit. Builder logs are claims, not proof. Do not inspect the filesystem, Git repository, web, MCP, plugins, or unrelated context. Do not modify anything. Return only one JSON object matching the supplied output schema. Never fabricate PASS when evidence is unavailable or contradictory.\n\nAuditInput:\n{}",
            input_json
        );
        if prompt.len() > MAX_AUDIT_INPUT_BYTES + 8192 {
            return Err("AUDIT_CODEX_INPUT_BOUNDS: bounded audit prompt exceeded its limit".into());
        }
        let result = self.runner.run(&CodexProcessRequest {
            prompt,
            schema: Some(audit_result_schema()),
            model: None,
            timeout: CODEX_AUDIT_TIMEOUT,
        })?;
        if result.timed_out {
            return Err("AUDIT_CODEX_TIMEOUT: bounded Codex audit process timed out".into());
        }
        if result.exit_code != Some(0) {
            return Err(classify_codex_failure(&result, "PROCESS_ERROR"));
        }
        result.final_message.ok_or_else(|| {
            "AUDIT_CODEX_FINAL_OUTPUT_MISSING: Codex did not produce a dedicated final result"
                .into()
        })
    }
}

fn classify_codex_failure(result: &CodexProcessResult, fallback: &str) -> String {
    let text = format!("{}\n{}", result.stdout, result.stderr).to_ascii_lowercase();
    let category = if text.contains("api key") || text.contains("api-key") {
        "AUTH_POLICY_BLOCKED"
    } else if text.contains("login")
        || text.contains("authenticated")
        || text.contains("unauthorized")
    {
        "AUTH_REQUIRED"
    } else if text.contains("usage") || text.contains("quota") || text.contains("rate limit") {
        "USAGE_LIMITED"
    } else if text.contains("network") || text.contains("connect") || text.contains("timeout") {
        "NETWORK_ERROR"
    } else {
        fallback
    };
    format!("AUDIT_CODEX_{category}: bounded Codex process did not produce an accepted result")
}

fn provider_readiness(
    status: &str,
    available: bool,
    version: Option<String>,
    diagnostic_message: &str,
) -> AuditProviderReadiness {
    let login_state = match status {
        "AUTH_POLICY_BLOCKED" => "API-key authentication unsupported",
        "AUTH_REQUIRED" => "Not logged in",
        "AUTH_UNVERIFIED" => "ChatGPT login reported; turn unverified",
        "READY" => "ChatGPT authenticated",
        _ => "Unavailable",
    };
    AuditProviderReadiness {
        provider: "Codex CLI".into(),
        status: status.into(),
        configured: available,
        executable_available: version.is_some(),
        version,
        login_state: Some(login_state.into()),
        model: Some(CODEX_DEFAULT_MODEL.into()),
        credential_source: Some("Codex-managed login state".into()),
        error_category: (!diagnostic_message.is_empty()).then(|| diagnostic_message.into()),
    }
}

pub fn audit_provider_readiness(
    _database: &DatabaseState,
) -> Result<AuditProviderReadiness, String> {
    let resolution = resolve_codex_executable();
    let Some(executable) = resolution.selected else {
        return Ok(provider_readiness(
            "CODEX_NOT_FOUND",
            false,
            None,
            "No native Codex executable was found.",
        ));
    };
    let version = match probe_version(&executable, READINESS_TIMEOUT) {
        Ok(value) => value,
        Err(ProbeError::Timeout) => {
            return Ok(provider_readiness(
                "TIMEOUT",
                false,
                None,
                "Codex version probe timed out.",
            ))
        }
        Err(ProbeError::Malformed) => {
            return Ok(provider_readiness(
                "PROCESS_ERROR",
                false,
                None,
                "Codex version output was malformed.",
            ))
        }
        Err(ProbeError::Failed) => {
            return Ok(provider_readiness(
                "PROCESS_ERROR",
                false,
                None,
                "Codex version probe failed.",
            ))
        }
    };
    match probe_login_status(&executable, READINESS_TIMEOUT) {
        Ok(LoginState::ChatGpt) => Ok(provider_readiness(
            "AUTH_UNVERIFIED",
            true,
            Some(version),
            "Codex reports a ChatGPT login; end-to-end readiness requires Check readiness.",
        )),
        Ok(LoginState::ApiKey) => Ok(provider_readiness(
            "AUTH_POLICY_BLOCKED",
            false,
            Some(version),
            "Codex reports API-key authentication, which H!veAI does not accept for audits.",
        )),
        Ok(LoginState::NotLoggedIn) => Ok(provider_readiness(
            "AUTH_REQUIRED",
            false,
            Some(version),
            "Codex is not logged in through a supported ChatGPT session.",
        )),
        Ok(LoginState::Unknown) => Ok(provider_readiness(
            "AUTH_UNVERIFIED",
            false,
            Some(version),
            "Codex executable is available but its bounded login status was not recognized.",
        )),
        Err(ProbeError::Timeout) => Ok(provider_readiness(
            "TIMEOUT",
            false,
            Some(version),
            "Codex login status timed out.",
        )),
        Err(_) => Ok(provider_readiness(
            "PROCESS_ERROR",
            false,
            Some(version),
            "Codex login status could not be verified.",
        )),
    }
}

fn check_codex_readiness_with_runner(
    version: String,
    runner: &dyn CodexProcessRunner,
) -> AuditProviderReadiness {
    let result = runner.run(&CodexProcessRequest {
        prompt: "Reply with exactly READY. Do not inspect files, repositories, web, MCP, plugins, or unrelated context.".into(),
        schema: None,
        model: None,
        timeout: CODEX_READINESS_TIMEOUT,
    });
    let failure = |status: &str, message: String| AuditProviderReadiness {
        provider: "Codex CLI".into(),
        status: status.into(),
        configured: status == "READY",
        executable_available: true,
        version: Some(version.clone()),
        login_state: Some(
            match status {
                "AUTH_POLICY_BLOCKED" => "API-key authentication unsupported",
                "AUTH_REQUIRED" => "Not logged in",
                _ => "ChatGPT login end-to-end check failed",
            }
            .into(),
        ),
        model: Some(CODEX_DEFAULT_MODEL.into()),
        credential_source: Some("Codex-managed login state".into()),
        error_category: Some(format!("{} ({})", message, version)),
    };
    match result {
        Err(error) => failure("PROCESS_ERROR", error),
        Ok(result) if result.timed_out => {
            failure("TIMEOUT", "Codex readiness probe timed out".into())
        }
        Ok(result) if result.exit_code != Some(0) => {
            let error = classify_codex_failure(&result, "PROCESS_ERROR");
            let status = if error.contains("AUTH_POLICY_BLOCKED") {
                "AUTH_POLICY_BLOCKED"
            } else if error.contains("AUTH_REQUIRED") {
                "AUTH_REQUIRED"
            } else if error.contains("USAGE_LIMITED") {
                "USAGE_LIMITED"
            } else if error.contains("NETWORK_ERROR") {
                "NETWORK_ERROR"
            } else {
                "PROCESS_ERROR"
            };
            failure(status, error)
        }
        Ok(result) if result.final_message.as_deref().map(str::trim) == Some("READY") => {
            AuditProviderReadiness {
                provider: "Codex CLI".into(),
                status: "READY".into(),
                configured: true,
                executable_available: true,
                version: Some(version),
                login_state: Some("ChatGPT authenticated".into()),
                model: Some(CODEX_DEFAULT_MODEL.into()),
                credential_source: Some("Codex-managed login state".into()),
                error_category: None,
            }
        }
        Ok(_) => failure(
            "PROCESS_ERROR",
            "Codex readiness final output was not exactly READY".into(),
        ),
    }
}

pub fn check_audit_provider_readiness(
    database: &DatabaseState,
) -> Result<AuditProviderReadiness, String> {
    let local = audit_provider_readiness(database)?;
    if local.status != "AUTH_UNVERIFIED" || !local.configured {
        return Ok(local);
    }
    let resolution = resolve_codex_executable();
    let Some(executable) = resolution.selected else {
        return Ok(provider_readiness(
            "CODEX_NOT_FOUND",
            false,
            None,
            "No native Codex executable was found.",
        ));
    };
    let version =
        probe_version(&executable, READINESS_TIMEOUT).unwrap_or_else(|_| "UNAVAILABLE".into());
    Ok(check_codex_readiness_with_runner(
        version,
        &NativeCodexProcessRunner { executable },
    ))
}

fn resolve_production_model(database: &DatabaseState) -> ProductionAuditModel {
    let readiness = match audit_provider_readiness(database) {
        Ok(value) => value,
        Err(_) => return ProductionAuditModel::Unavailable(UnavailableAuditModel),
    };
    if readiness.status != "AUTH_UNVERIFIED" || !readiness.configured {
        return ProductionAuditModel::Unavailable(UnavailableAuditModel);
    }
    let resolution = resolve_codex_executable();
    let Some(executable) = resolution.selected else {
        return ProductionAuditModel::Unavailable(UnavailableAuditModel);
    };
    let Ok(version) = probe_version(&executable, READINESS_TIMEOUT) else {
        return ProductionAuditModel::Unavailable(UnavailableAuditModel);
    };
    ProductionAuditModel::Codex(CodexCliAuditModel {
        version,
        runner: Arc::new(NativeCodexProcessRunner { executable }),
    })
}

enum ProductionAuditModel {
    Codex(CodexCliAuditModel),
    Unavailable(UnavailableAuditModel),
}

impl AuditModel for ProductionAuditModel {
    fn provider(&self) -> String {
        match self {
            Self::Codex(model) => model.provider(),
            Self::Unavailable(model) => model.provider(),
        }
    }

    fn model(&self) -> String {
        match self {
            Self::Codex(model) => model.model(),
            Self::Unavailable(model) => model.model(),
        }
    }

    fn version(&self) -> String {
        match self {
            Self::Codex(model) => model.version(),
            Self::Unavailable(model) => model.version(),
        }
    }

    fn evaluate(&self, input: &AuditInput) -> Result<String, String> {
        match self {
            Self::Codex(model) => model.evaluate(input),
            Self::Unavailable(model) => model.evaluate(input),
        }
    }
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn truncate_utf8(value: &str, max: usize) -> String {
    if value.len() <= max {
        return value.to_string();
    }
    let mut end = max.min(value.len());
    while end > 0 && !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_string()
}

fn bound_text(value: &str, max: usize) -> (String, bool) {
    if value.len() <= max {
        return (value.to_string(), false);
    }
    let marker = "\n[TRUNCATED: bounded audit evidence]";
    (
        format!(
            "{}{}",
            truncate_utf8(value, max.saturating_sub(marker.len())),
            marker
        ),
        true,
    )
}

fn sanitize_text(value: &str) -> String {
    value
        .lines()
        .map(|line| {
            let lower = line.to_ascii_lowercase();
            if lower.contains("authorization:")
                || lower.contains("bearer ")
                || lower.contains("api_key")
                || lower.contains("apikey")
                || lower.contains("password=")
                || lower.contains("token=")
                || lower.contains("secret=")
            {
                "[REDACTED_SENSITIVE_EVIDENCE]".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn valid_project(
    database: &DatabaseState,
    project_id: &str,
) -> Result<crate::projects::ProjectRecord, String> {
    let project = fetch_project(database, project_id)?;
    if project.status != "ACTIVE" {
        return Err("AUDIT_PROJECT_NOT_ACTIVE: audit requires an ACTIVE registered project".into());
    }
    Ok(project)
}

pub fn collect_input(
    database: &DatabaseState,
    request: AuditInputRequest,
) -> Result<AuditInput, String> {
    let project = valid_project(database, &request.project_id)?;
    validate_audit_target(database, &request.project_id, &request.git_target)?;
    let workflow = workflow::project_list(
        database,
        WorkflowProjectListQuery {
            project_id: request.project_id.clone(),
            limit: Some(MAX_REQUIREMENTS),
        },
    )?;
    let parsed = if request.task_id.is_some() {
        Some(task_intelligence::list(database, &request.project_id))
    } else {
        None
    };
    let task = match request.task_id.as_ref() {
        Some(task_id) => {
            let workflow_task = workflow
                .tasks
                .iter()
                .find(|task| &task.task_id == task_id)
                .ok_or_else(|| "AUDIT_TASK_NOT_FOUND_OR_PROJECT_MISMATCH".to_string())?;
            let parsed_task = parsed
                .as_ref()
                .and_then(|result| result.as_ref().ok())
                .and_then(|snapshot| snapshot.tasks.iter().find(|task| &task.id == task_id));
            Some(task_evidence(workflow_task, parsed_task))
        }
        None => None,
    };
    let git_snapshot = git_engine::snapshot(
        database,
        GitSnapshotRequest {
            project_id: request.project_id.clone(),
            persist: Some(false),
        },
    )
    .ok();
    let diff_result = git_engine::diff(
        database,
        GitDiffRequest {
            project_id: request.project_id.clone(),
            scope: request.git_target.scope,
            base_ref: request.git_target.base_ref.clone(),
            head_sha: request.git_target.head_sha.clone(),
        },
    );
    let diff = match diff_result {
        Ok(value) => Some(value),
        Err(error) if request.git_target.scope == GitDiffScope::CommitRange => return Err(error),
        Err(_) => None,
    };
    let dashboard = project_dashboard::resolve(database, &request.project_id)?;
    let git = git_evidence(
        git_snapshot.as_ref(),
        diff.as_ref(),
        &project,
        &request.git_target,
    );
    let requirements = task
        .as_ref()
        .map(|task| {
            task.requirements
                .iter()
                .take(MAX_REQUIREMENTS)
                .enumerate()
                .map(|(index, text)| AuditRequirement {
                    requirement_ref: format!("task-requirement-{index}"),
                    requirement_text: truncate_utf8(text, 4096),
                    required: true,
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut evidence = Vec::new();
    push_evidence(&mut evidence, authority_evidence(&dashboard));
    push_evidence(&mut evidence, task_evidence_item(task.as_ref()));
    push_evidence(&mut evidence, git_snapshot_evidence(git_snapshot.as_ref()));
    push_evidence(&mut evidence, git_diff_evidence(diff.as_ref()));
    let changed = changed_paths(
        git_snapshot.as_ref(),
        diff.as_ref(),
        request.git_target.scope,
    );
    let selected = select_source_paths(&changed, request.git_target.scope);
    let diff_text = diff.as_ref().map(|value| value.text.as_str());
    for path in selected {
        if evidence
            .iter()
            .filter(|item| item.kind == "SOURCE_SNIPPET")
            .count()
            >= MAX_SOURCE_SNIPPETS
        {
            break;
        }
        let line_hint = diff_text.and_then(|text| diff_line_hint(text, &path));
        if let Some(item) = read_source_evidence(
            &project.normalized_path,
            &path,
            line_hint,
            request.git_target.scope,
            diff.as_ref().and_then(|value| value.head_sha.as_deref()),
        ) {
            push_evidence(&mut evidence, item);
        }
    }
    let connection = database.open_connection()?;
    for item in read_test_evidence(&connection, &request.project_id, &project.normalized_path)? {
        if evidence
            .iter()
            .filter(|item| item.kind == "TEST_RUN" || item.kind == "TEST_BODY")
            .count()
            >= MAX_TEST_SNIPPETS
        {
            break;
        }
        push_evidence(&mut evidence, item);
    }
    for item in read_builder_claims(&project.normalized_path, &request) {
        push_evidence(&mut evidence, item);
    }
    evidence.sort_by(|left, right| left.id.cmp(&right.id));
    evidence.truncate(MAX_EVIDENCE_ITEMS);
    let collected_at = utc_timestamp();
    let mut input = AuditInput {
        project_id: request.project_id,
        task_id: request.task_id,
        task,
        requirements,
        audited_branch: git.branch.clone(),
        audited_head_sha: git.head_sha.clone(),
        baseline_ref: git.baseline_ref.clone(),
        git,
        evidence,
        collected_at,
        input_manifest_sha256: String::new(),
        freshness_token: String::new(),
        audited_session_id: request.git_target.audited_session_id,
        audited_prompt_version_id: request.git_target.audited_prompt_version_id,
    };
    bound_input(&mut input);
    input.freshness_token = freshness_token(&input.git);
    input.input_manifest_sha256 =
        hash_bytes(&serde_json::to_vec(&input).map_err(|e| e.to_string())?);
    Ok(input)
}

fn validate_audit_target(
    database: &DatabaseState,
    project_id: &str,
    target: &AuditGitTarget,
) -> Result<(), String> {
    for reference in [target.base_ref.as_deref(), target.head_sha.as_deref()]
        .into_iter()
        .flatten()
    {
        if reference.trim().is_empty()
            || reference.len() > 256
            || reference.starts_with('-')
            || reference
                .chars()
                .any(|value| value.is_whitespace() || value.is_control() || value == '\\')
        {
            return Err("AUDIT_GIT_TARGET_REFERENCE_INVALID".into());
        }
    }
    if target.scope == GitDiffScope::CommitRange && target.base_ref.is_none() {
        return Err("AUDIT_GIT_TARGET_BASE_REQUIRED".into());
    }
    if target.scope != GitDiffScope::CommitRange
        && (target.base_ref.is_some() || target.head_sha.is_some())
    {
        return Err("AUDIT_GIT_TARGET_REFS_NOT_ALLOWED_FOR_SCOPE".into());
    }
    match (
        &target.audited_session_id,
        &target.audited_prompt_version_id,
    ) {
        (None, None) => Ok(()),
        (Some(_), None) | (None, Some(_)) => {
            Err("AUDIT_PROVENANCE_SESSION_AND_PROMPT_REQUIRED".into())
        }
        (Some(session_id), Some(prompt_version_id)) => {
            let connection = database.open_connection()?;
            let valid = connection
                .query_row(
                    "SELECT 1 FROM agent_sessions s JOIN prompt_versions v ON v.id=?1 JOIN prompts p ON p.id=v.prompt_id WHERE s.id=?2 AND s.project_id=?3 AND s.prompt_version_id=v.id AND p.project_id=?3",
                    params![prompt_version_id, session_id, project_id],
                    |_| Ok(()),
                )
                .optional()
                .map_err(|error| error.to_string())?
                .is_some();
            if valid {
                Ok(())
            } else {
                Err("AUDIT_PROVENANCE_SESSION_PROMPT_MISMATCH".into())
            }
        }
    }
}

fn task_evidence(
    workflow_task: &WorkflowTask,
    parsed: Option<&task_intelligence::ParsedTask>,
) -> AuditTaskEvidence {
    AuditTaskEvidence {
        task_id: workflow_task.task_id.clone(),
        title: workflow_task.title.clone(),
        workflow_state: workflow_task.current_state.to_string(),
        requirements: parsed
            .map(|task| task.acceptance_criteria.clone())
            .unwrap_or_default(),
        dependencies: parsed
            .map(|task| task.dependency_references.clone())
            .unwrap_or_default(),
        blockers: parsed.map(|task| task.blockers.clone()).unwrap_or_default(),
        required_actor: workflow_task
            .required_actor
            .clone()
            .or_else(|| parsed.and_then(|task| task.required_actor.clone())),
        milestone: workflow_task
            .milestone
            .clone()
            .or_else(|| parsed.and_then(|task| task.milestone.clone())),
        source_path: parsed.map(|task| task.evidence.source_path.clone()),
        source_hash: parsed.map(|task| task.evidence.content_hash.clone()),
        identity_status: VerificationStatus::Verified,
        requirements_status: if parsed.is_some() {
            VerificationStatus::Verified
        } else {
            VerificationStatus::Unavailable
        },
        requirements_provenance: parsed.map(|task| task.evidence.content_hash.clone()),
    }
}

fn git_evidence(
    snapshot: Option<&GitSnapshot>,
    diff: Option<&git_engine::GitDiff>,
    project: &crate::projects::ProjectRecord,
    target: &AuditGitTarget,
) -> AuditGitEvidence {
    let (diff_value, diff_bounded) = diff
        .map(|diff| bound_text(&diff.text, MAX_GIT_DIFF_BYTES))
        .map(|(text, bounded)| (Some(text), bounded))
        .unwrap_or((None, false));
    AuditGitEvidence {
        scope: target.scope,
        target_origin: target.target_origin,
        branch: snapshot.and_then(|snapshot| snapshot.current_branch.clone()),
        head_sha: diff
            .filter(|diff| diff.scope == GitDiffScope::CommitRange)
            .and_then(|diff| diff.head_sha.clone())
            .or_else(|| snapshot.and_then(|snapshot| snapshot.head_sha.clone())),
        baseline_ref: diff
            .filter(|diff| diff.scope == GitDiffScope::CommitRange)
            .and_then(|diff| diff.base_ref.clone())
            .or_else(|| {
                project
                    .repository
                    .as_ref()
                    .and_then(|repository| repository.default_branch.clone())
            }),
        base_sha: diff.and_then(|diff| diff.base_sha.clone()),
        staged_files: snapshot
            .map(|snapshot| {
                snapshot
                    .staged_files
                    .iter()
                    .map(|file| file.path.clone())
                    .collect()
            })
            .unwrap_or_default(),
        unstaged_files: snapshot
            .map(|snapshot| {
                snapshot
                    .unstaged_files
                    .iter()
                    .map(|file| file.path.clone())
                    .collect()
            })
            .unwrap_or_default(),
        untracked_files: snapshot
            .map(|snapshot| snapshot.untracked_files.clone())
            .unwrap_or_default(),
        conflicted_files: snapshot
            .map(|snapshot| snapshot.conflicted_files.clone())
            .unwrap_or_default(),
        diff: diff_value,
        diff_truncated: diff.map(|diff| diff.truncated).unwrap_or(false) || diff_bounded,
        repository_identity: project.repository.as_ref().map(|repository| {
            format!(
                "{}/{}",
                repository.github_owner.clone().unwrap_or_default(),
                repository.github_repo.clone().unwrap_or_default()
            )
        }),
        changed_files: diff
            .map(|diff| diff.changed_files.clone())
            .unwrap_or_default(),
        full_change_set_sha256: diff.map(|diff| diff.full_change_set_sha256.clone()),
        staged_content_sha256: diff
            .filter(|diff| diff.scope == GitDiffScope::Staged)
            .map(|diff| diff.full_change_set_sha256.clone()),
        working_tracked_content_sha256: diff
            .filter(|diff| diff.scope == GitDiffScope::WorkingTree)
            .map(|diff| diff.full_change_set_sha256.clone()),
        untracked_content_sha256: diff.and_then(|diff| diff.untracked_content_sha256.clone()),
        conflict_identity: snapshot
            .map(|snapshot| hash_bytes(snapshot.conflicted_files.join("\n").as_bytes())),
        committed_range_identity: diff
            .filter(|diff| diff.scope == GitDiffScope::CommitRange)
            .map(|diff| diff.full_change_set_sha256.clone()),
        identity_complete: diff.map(|diff| diff.identity_complete).unwrap_or(false),
        identity_diagnostic: diff.and_then(|diff| diff.identity_diagnostic.clone()),
    }
}

fn freshness_token(git: &AuditGitEvidence) -> String {
    let identity = json!({
        "scope": git.scope,
        "targetOrigin": git.target_origin,
        "branch": git.branch,
        "headSha": git.head_sha,
        "baseSha": git.base_sha,
        "stagedFiles": git.staged_files,
        "unstagedFiles": git.unstaged_files,
        "untrackedFiles": git.untracked_files,
        "conflictedFiles": git.conflicted_files,
        "changedFiles": git.changed_files,
        "fullChangeSetSha256": git.full_change_set_sha256,
        "stagedContentSha256": git.staged_content_sha256,
        "workingTrackedContentSha256": git.working_tracked_content_sha256,
        "untrackedContentSha256": git.untracked_content_sha256,
        "conflictIdentity": git.conflict_identity,
        "committedRangeIdentity": git.committed_range_identity,
        "diffTruncated": git.diff_truncated,
        "repositoryIdentity": git.repository_identity,
        "identityComplete": git.identity_complete,
        "identityDiagnostic": git.identity_diagnostic,
    });
    hash_bytes(identity.to_string().as_bytes())
}

fn current_freshness_token(
    database: &DatabaseState,
    project_id: &str,
    target: &AuditGitTarget,
) -> Result<String, String> {
    let project = valid_project(database, project_id)?;
    let snapshot = git_engine::snapshot(
        database,
        GitSnapshotRequest {
            project_id: project_id.to_string(),
            persist: Some(false),
        },
    )?;
    let diff = git_engine::diff(
        database,
        GitDiffRequest {
            project_id: project_id.to_string(),
            scope: target.scope,
            base_ref: target.base_ref.clone(),
            head_sha: target.head_sha.clone(),
        },
    )?;
    Ok(freshness_token(&git_evidence(
        Some(&snapshot),
        Some(&diff),
        &project,
        target,
    )))
}

fn changed_paths(
    snapshot: Option<&GitSnapshot>,
    diff: Option<&git_engine::GitDiff>,
    scope: GitDiffScope,
) -> Vec<String> {
    let mut paths = match scope {
        GitDiffScope::WorkingTree => snapshot
            .map(|snapshot| {
                snapshot
                    .unstaged_files
                    .iter()
                    .map(|file| file.path.clone())
                    .chain(snapshot.untracked_files.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        GitDiffScope::Staged => snapshot
            .map(|snapshot| {
                snapshot
                    .staged_files
                    .iter()
                    .map(|file| file.path.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        GitDiffScope::CommitRange => Vec::new(),
    };
    if let Some(diff) = diff {
        paths.extend(diff.changed_files.iter().cloned());
    }
    paths.sort();
    paths.dedup();
    paths
}

fn authority_evidence(dashboard: &project_dashboard::ProjectDashboardResolution) -> AuditEvidence {
    let content = json!({
        "manifestStatus": dashboard.manifest_status,
        "taskAuthority": dashboard.task_authority,
        "canonicalTaskSource": dashboard.canonical_task_source,
        "provenanceMode": dashboard.provenance_mode,
        "branchPolicy": dashboard.branch_policy,
        "warnings": dashboard.warnings,
    })
    .to_string();
    evidence(
        "GOVERNANCE",
        "project-dashboard",
        VerificationStatus::Verified,
        "Project Dashboard authority resolution",
        None,
        Some(content),
        false,
    )
}

fn task_evidence_item(task: Option<&AuditTaskEvidence>) -> AuditEvidence {
    match task {
        Some(task) => evidence(
            "TASK_REQUIREMENTS",
            &task.task_id,
            if task.requirements_status == VerificationStatus::Verified {
                VerificationStatus::Verified
            } else {
                VerificationStatus::Partial
            },
            if task.requirements_status == VerificationStatus::Verified {
                "Task requirements and workflow authority collected"
            } else {
                "Task identity verified; task-intelligence requirements authority unavailable"
            },
            None,
            serde_json::to_string(task).ok(),
            false,
        ),
        None => evidence(
            "TASK_REQUIREMENTS",
            "project-audit",
            VerificationStatus::Unavailable,
            "Task-specific evidence unavailable for project/freeform audit",
            None,
            None,
            false,
        ),
    }
}

fn git_snapshot_evidence(snapshot: Option<&GitSnapshot>) -> AuditEvidence {
    match snapshot {
        Some(snapshot) => evidence(
            "GIT_SNAPSHOT",
            &snapshot.project_id,
            VerificationStatus::Verified,
            "Git Engine snapshot authority",
            None,
            serde_json::to_string(snapshot).ok(),
            false,
        ),
        None => evidence(
            "GIT_SNAPSHOT",
            "repository",
            VerificationStatus::Unavailable,
            "Git snapshot unavailable for this registered project",
            None,
            None,
            false,
        ),
    }
}

fn git_diff_evidence(diff: Option<&git_engine::GitDiff>) -> AuditEvidence {
    match diff {
        Some(diff) => evidence(
            "GIT_DIFF",
            &diff.project_id,
            if diff.truncated {
                VerificationStatus::Truncated
            } else {
                VerificationStatus::Verified
            },
            &format!("Bounded Git Engine {} diff", git_scope_str(diff.scope)),
            None,
            Some(diff.text.clone()),
            diff.truncated,
        ),
        None => evidence(
            "GIT_DIFF",
            "working-tree",
            VerificationStatus::Unavailable,
            "Git diff unavailable",
            None,
            None,
            false,
        ),
    }
}

fn evidence(
    kind: &str,
    locator: &str,
    status: VerificationStatus,
    summary: &str,
    content_hash: Option<String>,
    content: Option<String>,
    truncated: bool,
) -> AuditEvidence {
    let (content, was_truncated) = content
        .map(|value| bound_text(&sanitize_text(&value), MAX_SOURCE_SNIPPET_BYTES))
        .map(|(value, truncated)| (Some(value), truncated))
        .unwrap_or((None, false));
    let content_sha256 = content
        .as_ref()
        .map(|value| hash_bytes(value.as_bytes()))
        .or(content_hash);
    AuditEvidence {
        id: format!("{}:{}", kind, locator),
        logical_evidence_id: format!("{}:{}", kind, locator),
        kind: kind.to_string(),
        verification_status: if was_truncated {
            VerificationStatus::Truncated
        } else {
            status
        },
        locator: Some(truncate_utf8(locator, MAX_LOCATOR_BYTES)),
        summary: truncate_utf8(summary, MAX_MODEL_SUMMARY_BYTES),
        byte_count: content.as_ref().map(|value| value.len()).unwrap_or(0),
        content,
        content_sha256,
        truncated: truncated || was_truncated,
    }
}

fn push_evidence(evidence: &mut Vec<AuditEvidence>, item: AuditEvidence) {
    if evidence.len() < MAX_EVIDENCE_ITEMS
        && !evidence.iter().any(|existing| existing.id == item.id)
    {
        evidence.push(item);
    }
}

fn select_source_paths(changed: &[String], scope: GitDiffScope) -> Vec<String> {
    let mut selected = changed
        .iter()
        .filter(|path| is_auditable_path(path))
        .cloned()
        .collect::<Vec<_>>();
    // Only the selected Git target can provide implementation proof. The
    // source planner deliberately omits fallback/context files so scope cannot
    // be silently widened by a staged or committed-range audit.
    let _ = scope;
    selected.sort();
    selected.dedup();
    selected.truncate(MAX_SOURCE_SNIPPETS);
    selected
}

fn is_auditable_path(path: &str) -> bool {
    let lower = path.replace('\\', "/").to_ascii_lowercase();
    if lower.contains("..")
        || lower.starts_with('/')
        || lower.contains(":")
        || is_secret_path(&lower)
    {
        return false;
    }
    [".rs", ".ts", ".tsx", ".js", ".jsx", ".json", ".toml", ".md"]
        .iter()
        .any(|suffix| lower.ends_with(suffix))
}

fn is_secret_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower == ".env"
        || lower.starts_with(".env.")
        || lower.contains("secret")
        || lower.contains("credential")
        || lower.ends_with(".pem")
        || lower.ends_with(".key")
}

fn diff_line_hint(diff: &str, relative: &str) -> Option<usize> {
    let normalized = relative.replace('\\', "/");
    let mut in_file = false;
    for line in diff.lines() {
        if line.starts_with("diff --git ") {
            in_file = line
                .split_whitespace()
                .nth(3)
                .map(|value| value.trim_start_matches("b/") == normalized)
                .unwrap_or(false);
        } else if in_file && line.starts_with("@@ ") {
            let marker = line.split_whitespace().nth(2)?;
            let start = marker.trim_start_matches('+').split(',').next()?;
            return start.parse::<usize>().ok();
        }
    }
    None
}

fn bounded_source_window(lines: &[&str], line_hint: Option<usize>) -> (String, usize) {
    if lines.is_empty() {
        return (String::new(), 1);
    }
    let center = line_hint
        .unwrap_or(1)
        .saturating_sub(1)
        .min(lines.len() - 1);
    let start = center.saturating_sub(24);
    let end = (start + 96).min(lines.len());
    (lines[start..end].join("\n"), start + 1)
}

fn read_source_evidence(
    root: &str,
    relative: &str,
    line_hint: Option<usize>,
    scope: GitDiffScope,
    commit_head: Option<&str>,
) -> Option<AuditEvidence> {
    let root = fs::canonicalize(root).ok()?;
    let candidate = root.join(relative);
    let canonical = fs::canonicalize(&candidate).ok();
    if is_secret_path(relative)
        || (scope == GitDiffScope::WorkingTree
            && canonical
                .as_ref()
                .is_none_or(|path| !path.starts_with(&root) || !path.is_file()))
    {
        return Some(evidence(
            "SOURCE_SNIPPET",
            relative,
            VerificationStatus::Excluded,
            "Source path excluded by audit policy",
            None,
            None,
            false,
        ));
    }
    let bytes = match scope {
        GitDiffScope::WorkingTree => {
            let canonical = canonical?;
            let mut file = File::open(canonical).ok()?;
            let mut bytes = Vec::new();
            std::io::Read::by_ref(&mut file)
                .take((MAX_SOURCE_SNIPPET_BYTES * 16 + 1) as u64)
                .read_to_end(&mut bytes)
                .ok()?;
            bytes
        }
        GitDiffScope::Staged => {
            let spec = format!(":{relative}");
            let result = match git_engine::run_git_bounded(
                &root,
                &["show", "--no-ext-diff", &spec],
                MAX_SOURCE_SNIPPET_BYTES * 16 + 1,
                4096,
            ) {
                Ok(value) => value,
                Err(error) => {
                    return Some(evidence(
                        "SOURCE_SNIPPET",
                        relative,
                        VerificationStatus::Unavailable,
                        &format!("Staged Git blob was unavailable: {error}"),
                        None,
                        None,
                        false,
                    ))
                }
            };
            if result.stdout_truncated {
                return Some(evidence(
                    "SOURCE_SNIPPET",
                    relative,
                    VerificationStatus::Truncated,
                    "Staged source exceeded the bounded Git blob reader budget",
                    None,
                    Some(
                        String::from_utf8_lossy(
                            &result.stdout[..MAX_SOURCE_SNIPPET_BYTES.min(result.stdout.len())],
                        )
                        .into_owned(),
                    ),
                    true,
                ));
            }
            result.stdout
        }
        GitDiffScope::CommitRange => {
            let head = commit_head?;
            let spec = format!("{head}:{relative}");
            let result = match git_engine::run_git_bounded(
                &root,
                &["show", "--no-ext-diff", &spec],
                MAX_SOURCE_SNIPPET_BYTES * 16 + 1,
                4096,
            ) {
                Ok(value) => value,
                Err(error) => {
                    return Some(evidence(
                        "SOURCE_SNIPPET",
                        relative,
                        VerificationStatus::Unavailable,
                        &format!("Committed Git blob was unavailable: {error}"),
                        None,
                        None,
                        false,
                    ))
                }
            };
            if result.stdout_truncated {
                return Some(evidence(
                    "SOURCE_SNIPPET",
                    relative,
                    VerificationStatus::Truncated,
                    "Committed source exceeded the bounded Git blob reader budget",
                    None,
                    Some(
                        String::from_utf8_lossy(
                            &result.stdout[..MAX_SOURCE_SNIPPET_BYTES.min(result.stdout.len())],
                        )
                        .into_owned(),
                    ),
                    true,
                ));
            }
            result.stdout
        }
    };
    let raw = String::from_utf8_lossy(&bytes).into_owned();
    let lines = raw.lines().collect::<Vec<_>>();
    let (window, _) = bounded_source_window(&lines, line_hint);
    let (text, truncated) = bound_text(&sanitize_text(&window), MAX_SOURCE_SNIPPET_BYTES);
    let lower = relative.to_ascii_lowercase();
    let is_test = lower.contains("test") || lower.starts_with("tests/");
    let status = if truncated {
        VerificationStatus::Truncated
    } else if is_test
        && (window.contains("#[ignore")
            || window.contains(".skip(")
            || window.contains("describe.skip"))
    {
        VerificationStatus::Stale
    } else if is_test
        && (window.contains("vi.mock") || window.contains("jest.mock") || window.contains("mock("))
    {
        VerificationStatus::Partial
    } else if is_test && (window.contains("assert") || window.contains("expect(")) {
        VerificationStatus::Corroborated
    } else if is_test {
        VerificationStatus::ClaimOnly
    } else {
        VerificationStatus::Verified
    };
    Some(evidence(
        if is_test {
            "TEST_BODY"
        } else {
            "SOURCE_SNIPPET"
        },
        relative,
        status,
        if is_test {
            "Direct bounded test-body inspection"
        } else {
            "Direct bounded production source inspection"
        },
        None,
        Some(text),
        truncated,
    ))
}

fn read_test_evidence(
    connection: &Connection,
    project_id: &str,
    root: &str,
) -> Result<Vec<AuditEvidence>, String> {
    let mut statement = connection.prepare("SELECT id,command,result,output_metadata_json,started_at,finished_at FROM test_runs WHERE project_id=?1 ORDER BY started_at DESC,id DESC LIMIT ?2").map_err(|e| e.to_string())?;
    let rows = statement
        .query_map(params![project_id, MAX_TEST_SNIPPETS as i64], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
            ))
        })
        .map_err(|e| e.to_string())?;
    rows.map(|row| {
        row.map(|(id, command, result, metadata, started, finished)| {
            let claim = metadata
                .as_deref()
                .and_then(parse_test_claim_metadata);
            if let Some((file, name)) = claim {
                let located = locate_test_body(root, &file, &name);
                if let Some((body, start_line, end_line)) = located.body {
                    let status = classify_test_body(&body);
                    return evidence(
                        "TEST_BODY",
                        &format!("{id}:{name}"),
                        status,
                        "Claim-directed bounded test-body inspection",
                        None,
                        Some(json!({"command": command, "result": result, "output": metadata, "startedAt": started, "finishedAt": finished, "testFile": file, "testName": name, "startLine": start_line, "endLine": end_line, "directAssertion": has_relevant_assertion(&body), "mockedProductionBoundary": has_production_mock(&body)}).to_string()),
                        false,
                    );
                }
                if located.truncated {
                    return evidence(
                        "TEST_BODY",
                        &format!("{id}:{name}"),
                        VerificationStatus::Truncated,
                        "Test definition was beyond the bounded scan window",
                        None,
                        None,
                        true,
                    );
                }
            }
            evidence(
                "TEST_RUN",
                &id,
                VerificationStatus::Unverified,
                &format!("Persisted test run {result}; exact test-body mapping is unavailable"),
                None,
                Some(json!({"command": command, "result": result, "output": metadata, "startedAt": started, "finishedAt": finished}).to_string()),
                false,
            )
        })
        .map_err(|e| e.to_string())
    })
    .collect()
}

fn parse_test_claim_metadata(raw: &str) -> Option<(String, String)> {
    let value: serde_json::Value = serde_json::from_str(raw).ok()?;
    let file = value
        .get("testFile")
        .or_else(|| value.get("testPath"))
        .and_then(serde_json::Value::as_str)?;
    let name = value
        .get("testName")
        .or_else(|| value.get("test"))
        .or_else(|| value.get("name"))
        .and_then(serde_json::Value::as_str)?;
    Some((file.to_string(), name.to_string()))
}

struct TestBodyLookup {
    body: Option<(String, usize, usize)>,
    truncated: bool,
}

fn locate_test_body(root: &str, relative: &str, name: &str) -> TestBodyLookup {
    if !is_auditable_path(relative) || name.trim().is_empty() {
        return TestBodyLookup {
            body: None,
            truncated: false,
        };
    }
    let Some(root_path) = fs::canonicalize(root).ok() else {
        return TestBodyLookup {
            body: None,
            truncated: false,
        };
    };
    let candidate = root_path.join(relative);
    let Some(canonical) = fs::canonicalize(candidate).ok() else {
        return TestBodyLookup {
            body: None,
            truncated: false,
        };
    };
    if !canonical.starts_with(&root_path) || !canonical.is_file() {
        return TestBodyLookup {
            body: None,
            truncated: false,
        };
    }
    let Ok(file) = File::open(canonical) else {
        return TestBodyLookup {
            body: None,
            truncated: false,
        };
    };
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut bytes_read = 0usize;
    let mut truncated = false;
    while lines.len() < MAX_TEST_SCAN_LINES && bytes_read < MAX_TEST_SCAN_BYTES {
        let (next_line, read, line_truncated) =
            match read_bounded_line(&mut reader, MAX_TEST_SCAN_BYTES - bytes_read) {
                Ok(Some(value)) => value,
                Ok(None) => break,
                Err(_) => {
                    return TestBodyLookup {
                        body: None,
                        truncated: false,
                    }
                }
            };
        bytes_read = bytes_read.saturating_add(read);
        lines.push(next_line);
        truncated |= line_truncated;
    }
    if lines.len() >= MAX_TEST_SCAN_LINES {
        truncated = true;
    } else if bytes_read >= MAX_TEST_SCAN_BYTES {
        truncated |= reader
            .fill_buf()
            .map(|buffer| !buffer.is_empty())
            .unwrap_or(true);
    }
    let start = lines.iter().position(|line| {
        line.contains(&format!("fn {name}("))
            || line.contains(&format!("test(\"{name}\""))
            || line.contains(&format!("it(\"{name}\""))
            || line.contains(&format!("describe(\"{name}\""))
    });
    let Some(start) = start else {
        return TestBodyLookup {
            body: None,
            truncated,
        };
    };
    let end = (start + 80).min(lines.len());
    TestBodyLookup {
        body: Some((lines[start..end].join("\n"), start + 1, end)),
        truncated: false,
    }
}

fn read_bounded_line<R: BufRead>(
    reader: &mut R,
    max_bytes: usize,
) -> io::Result<Option<(String, usize, bool)>> {
    if max_bytes == 0 {
        return Ok(Some((String::new(), 0, true)));
    }
    let mut bytes = Vec::with_capacity(max_bytes.min(4096));
    let mut consumed = 0usize;
    let mut ended = false;
    while consumed < max_bytes {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            if consumed == 0 {
                return Ok(None);
            }
            break;
        }
        let available = (max_bytes - consumed).min(buffer.len());
        let newline = buffer[..available].iter().position(|byte| *byte == b'\n');
        let take = newline.map(|index| index + 1).unwrap_or(available);
        bytes.extend_from_slice(&buffer[..take]);
        reader.consume(take);
        consumed += take;
        if newline.is_some() {
            ended = true;
            break;
        }
    }
    let truncated = !ended && consumed >= max_bytes;
    Ok(Some((
        String::from_utf8_lossy(&bytes)
            .trim_end_matches(['\r', '\n'])
            .to_string(),
        consumed,
        truncated,
    )))
}

fn has_relevant_assertion(body: &str) -> bool {
    body.contains("assert!(")
        || body.contains("assert_eq!(")
        || body.contains("assert_ne!(")
        || body.contains("expect(")
        || body.contains("toHave")
        || body.contains("toEqual")
}

fn has_production_mock(body: &str) -> bool {
    body.contains("vi.mock")
        || body.contains("jest.mock")
        || body.contains("mockall")
        || body.contains("Mock")
}

fn classify_test_body(body: &str) -> VerificationStatus {
    if body.contains("#[ignore]")
        || body.contains(".skip(")
        || body.contains("describe.skip")
        || body.contains("test.skip")
    {
        VerificationStatus::Stale
    } else if !has_relevant_assertion(body) {
        VerificationStatus::Unverified
    } else if has_production_mock(body) {
        VerificationStatus::Partial
    } else {
        VerificationStatus::Corroborated
    }
}

fn read_builder_claims(root: &str, request: &AuditInputRequest) -> Vec<AuditEvidence> {
    let root_path = match fs::canonicalize(root) {
        Ok(path) => path,
        Err(_) => return Vec::new(),
    };
    let log_relative = Path::new("docs/H!veAI/codex-logs");
    let log_root = Path::new(root).join(log_relative);
    let canonical_log_root = match fs::canonicalize(&log_root) {
        Ok(path) if path.starts_with(&root_path) && path.is_dir() => path,
        Ok(_) => {
            return vec![evidence(
                "BUILDER_LOG_CLAIM",
                &log_relative.to_string_lossy(),
                VerificationStatus::Excluded,
                "Builder log directory resolves outside the canonical project root",
                None,
                None,
                false,
            )]
        }
        Err(_) => return Vec::new(),
    };
    let mut excluded = Vec::new();
    let mut candidates = fs::read_dir(&canonical_log_root)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("md") {
                return None;
            }
            let relative = path
                .strip_prefix(&root_path)
                .ok()
                .map(|value| value.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|| path.to_string_lossy().into_owned());
            let file_type = entry.file_type().ok();
            let canonical = fs::canonicalize(&path).ok();
            if file_type.as_ref().is_some_and(|kind| kind.is_symlink())
                || canonical
                    .as_ref()
                    .is_none_or(|value| value != &path || !value.starts_with(&root_path))
                || canonical.as_ref().is_none_or(|value| !value.is_file())
            {
                excluded.push(evidence(
                    "BUILDER_LOG_CLAIM",
                    &relative,
                    VerificationStatus::Excluded,
                    "Builder log link or out-of-root path was excluded before reading",
                    None,
                    None,
                    false,
                ));
                return None;
            }
            let modified = entry
                .metadata()
                .ok()
                .and_then(|metadata| metadata.modified().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH);
            let filename = path.file_name()?.to_string_lossy().to_ascii_uppercase();
            let relevance = builder_log_relevance(&filename, request);
            Some((relevance, modified, path, relative))
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| right.1.cmp(&left.1))
            .then_with(|| left.3.cmp(&right.3))
    });
    let mut result = excluded;
    let mut total_claim_bytes = 0usize;
    for (_, _, path, relative) in candidates.into_iter().take(4) {
        if total_claim_bytes >= MAX_BUILDER_LOG_TOTAL_BYTES {
            result.push(evidence(
                "BUILDER_LOG_CLAIM",
                &relative,
                VerificationStatus::Truncated,
                "Builder log claim omitted after the bounded total claim budget",
                None,
                None,
                true,
            ));
            continue;
        }
        let remaining = MAX_BUILDER_LOG_TOTAL_BYTES - total_claim_bytes;
        let per_file_limit = MAX_BUILDER_LOG_CLAIM_BYTES.min(remaining);
        let (raw, reader_truncated) = match read_bounded_text_file(&path, per_file_limit) {
            Ok(value) => value,
            Err(_) => continue,
        };
        total_claim_bytes = total_claim_bytes.saturating_add(raw.len());
        let (text, bounded_truncated) = bound_text(&sanitize_text(&raw), per_file_limit);
        let truncated = reader_truncated || bounded_truncated;
        result.push(evidence(
            "BUILDER_LOG_CLAIM",
            &relative,
            if truncated {
                VerificationStatus::Truncated
            } else {
                VerificationStatus::ClaimOnly
            },
            "Builder log is secondary CLAIM_ONLY evidence; source and direct tests remain authoritative",
            None,
            Some(text),
            truncated,
        ));
    }
    result
}

fn read_bounded_text_file(path: &Path, max_bytes: usize) -> Result<(String, bool), String> {
    let file = File::open(path).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    BufReader::new(file)
        .take(max_bytes.saturating_add(1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    let truncated = bytes.len() > max_bytes;
    bytes.truncate(max_bytes);
    Ok((String::from_utf8_lossy(&bytes).into_owned(), truncated))
}

fn builder_log_relevance(filename: &str, request: &AuditInputRequest) -> u8 {
    let mut score = 0;
    if request
        .prior_audit_id
        .as_ref()
        .is_some_and(|id| filename.contains(&id.to_ascii_uppercase()))
    {
        score += 8;
    }
    if request
        .git_target
        .audited_session_id
        .as_ref()
        .is_some_and(|id| filename.contains(&id.to_ascii_uppercase()))
    {
        score += 8;
    }
    if request
        .git_target
        .audited_prompt_version_id
        .as_ref()
        .is_some_and(|id| filename.contains(&id.to_ascii_uppercase()))
    {
        score += 8;
    }
    if filename.contains("M16") {
        score += 4;
    }
    if filename.contains("REMEDIATION") || filename.contains("AUDIT") {
        score += 2;
    }
    if let Some(task_id) = &request.task_id {
        if filename.contains(&task_id.to_ascii_uppercase()) {
            score += 3;
        }
    }
    if let Some(prompt_id) = &request.git_target.audited_prompt_version_id {
        if filename.contains(&prompt_id.to_ascii_uppercase()) {
            score += 2;
        }
    }
    score
}

fn bound_input(input: &mut AuditInput) {
    loop {
        let size = serde_json::to_vec(input)
            .map(|bytes| bytes.len())
            .unwrap_or(MAX_AUDIT_INPUT_BYTES + 1);
        if size <= MAX_AUDIT_INPUT_BYTES || input.evidence.is_empty() {
            break;
        }
        if let Some(item) = input
            .evidence
            .iter_mut()
            .rev()
            .find(|item| item.content.is_some())
        {
            item.content = None;
            item.byte_count = 0;
            item.truncated = true;
            item.verification_status = VerificationStatus::Truncated;
        } else {
            input.evidence.pop();
        }
    }
}

pub fn parse_model_output(raw: &str) -> Result<AuditEvaluation, String> {
    if raw.len() > MAX_MODEL_OUTPUT_BYTES {
        return Err(
            "AUDIT_MODEL_OUTPUT_TRUNCATED: model response exceeded the bounded response limit"
                .into(),
        );
    }
    let parsed: RawModelOutput = serde_json::from_str(raw).map_err(|_| {
        "AUDIT_MODEL_SCHEMA_INVALID: structured model response could not be parsed".to_string()
    })?;
    let verdict = parse_verdict(&parsed.verdict)?;
    let confidence = parse_confidence(&parsed.confidence)?;
    let regression_risk = parse_risk(&parsed.regression_risk)?;
    if parsed.findings.len() > MAX_FINDINGS
        || parsed.requirement_coverage.len() > MAX_REQUIREMENTS
        || parsed.prior_finding_dispositions.len() > MAX_FINDINGS
    {
        return Err(
            "AUDIT_MODEL_SCHEMA_BOUNDS: model findings or coverage exceed the bounded limit".into(),
        );
    }
    let findings = parsed
        .findings
        .into_iter()
        .enumerate()
        .map(|(index, finding)| {
            let severity = parse_severity(&finding.severity)?;
            let key = bounded_required(&finding.finding_key, 256, "finding key")?;
            Ok(AuditFinding {
                id: format!(
                    "finding-{index}-{}",
                    hash_bytes(key.as_bytes())[..12].to_string()
                ),
                logical_finding_id: key.clone(),
                finding_key: key,
                severity,
                title: bounded_required(&finding.title, 512, "finding title")?,
                detail: bounded_required(&finding.detail, 4096, "finding detail")?,
                requirement_refs: bound_list(finding.requirement_refs, MAX_REQUIREMENTS),
                evidence_refs: bound_list(finding.evidence_refs, MAX_EVIDENCE_ITEMS),
                source_locator: bound_optional(finding.source_locator, MAX_LOCATOR_BYTES),
                test_locator: bound_optional(finding.test_locator, MAX_LOCATOR_BYTES),
                confidence,
                status: "OPEN".into(),
                remediation_guidance: bounded_required(
                    &finding.remediation_guidance,
                    4096,
                    "remediation guidance",
                )?,
                blocks_release: finding.blocks_release,
                closed_by_audit_id: None,
                prior_finding_key: None,
                disposition: None,
                disposition_evidence_refs: Vec::new(),
                disposition_rationale: None,
                superseded_by_finding_key: None,
                inherited_prior_audit_id: None,
                inherited_prior_evidence_refs: Vec::new(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let coverage = parsed
        .requirement_coverage
        .into_iter()
        .enumerate()
        .map(|(index, coverage)| {
            Ok(RequirementCoverage {
                id: format!("coverage-{index}"),
                logical_coverage_id: bounded_required(
                    &coverage.requirement_ref,
                    256,
                    "requirement ref",
                )?,
                requirement_ref: bounded_required(
                    &coverage.requirement_ref,
                    256,
                    "requirement ref",
                )?,
                requirement_text: bounded_required(
                    &coverage.requirement_text,
                    4096,
                    "requirement text",
                )?,
                status: parse_coverage(&coverage.status)?,
                evidence_refs: bound_list(coverage.evidence_refs, MAX_EVIDENCE_ITEMS),
                rationale: bounded_required(&coverage.rationale, 4096, "coverage rationale")?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(AuditEvaluation {
        verdict,
        confidence,
        regression_risk,
        summary: bounded_required(&parsed.summary, MAX_MODEL_SUMMARY_BYTES, "audit summary")?,
        findings,
        prior_finding_dispositions: parsed
            .prior_finding_dispositions
            .into_iter()
            .enumerate()
            .map(|(_, disposition)| {
                Ok(PriorFindingDisposition {
                    prior_finding_key: bounded_required(
                        &disposition.prior_finding_key,
                        256,
                        "prior finding key",
                    )?,
                    disposition: parse_disposition(&disposition.disposition)?,
                    evidence_refs: bound_list(disposition.evidence_refs, MAX_EVIDENCE_ITEMS),
                    rationale: bounded_required(
                        &disposition.rationale,
                        4096,
                        "disposition rationale",
                    )?,
                    replacement_finding_key: bound_optional(
                        disposition.replacement_finding_key,
                        256,
                    ),
                })
            })
            .collect::<Result<Vec<_>, String>>()?,
        coverage,
        model_status: "AVAILABLE".into(),
        diagnostic: None,
        // Model identity fields are accepted only for backward-compatible input parsing.
        // evaluate_with replaces them with the executing AuditModel's trusted identity.
        auditor_provider: None,
        auditor_model: None,
        auditor_version: None,
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RawModelOutput {
    verdict: String,
    confidence: String,
    regression_risk: String,
    summary: String,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    model_version: Option<String>,
    #[serde(default)]
    findings: Vec<RawFinding>,
    #[serde(default)]
    requirement_coverage: Vec<RawCoverage>,
    #[serde(default)]
    prior_finding_dispositions: Vec<RawPriorFindingDisposition>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RawFinding {
    finding_key: String,
    severity: String,
    title: String,
    detail: String,
    #[serde(default)]
    requirement_refs: Vec<String>,
    #[serde(default)]
    evidence_refs: Vec<String>,
    #[serde(default)]
    source_locator: Option<String>,
    #[serde(default)]
    test_locator: Option<String>,
    remediation_guidance: String,
    #[serde(default)]
    blocks_release: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RawCoverage {
    requirement_ref: String,
    requirement_text: String,
    status: String,
    #[serde(default)]
    evidence_refs: Vec<String>,
    rationale: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RawPriorFindingDisposition {
    prior_finding_key: String,
    disposition: String,
    #[serde(default)]
    evidence_refs: Vec<String>,
    #[serde(default)]
    rationale: String,
    #[serde(default)]
    replacement_finding_key: Option<String>,
}

fn bounded_required(value: &str, max: usize, name: &str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(format!("AUDIT_MODEL_SCHEMA_INVALID: {name} is required"));
    }
    Ok(truncate_utf8(value, max))
}
fn bound_optional(value: Option<String>, max: usize) -> Option<String> {
    value.map(|value| truncate_utf8(&value, max))
}
fn bound_list(values: Vec<String>, max: usize) -> Vec<String> {
    values
        .into_iter()
        .take(max)
        .map(|value| truncate_utf8(&value, MAX_LOCATOR_BYTES))
        .collect()
}
fn parse_verdict(value: &str) -> Result<AuditVerdict, String> {
    match value.to_ascii_uppercase().as_str() {
        "PASS" => Ok(AuditVerdict::Pass),
        "CONDITIONAL" => Ok(AuditVerdict::Conditional),
        "FAIL" => Ok(AuditVerdict::Fail),
        _ => Err("AUDIT_MODEL_SCHEMA_INVALID: unsupported verdict".into()),
    }
}
fn parse_severity(value: &str) -> Result<FindingSeverity, String> {
    match value.to_ascii_uppercase().as_str() {
        "BLOCKER" => Ok(FindingSeverity::Blocker),
        "MAJOR" => Ok(FindingSeverity::Major),
        "MINOR" => Ok(FindingSeverity::Minor),
        "NOTE" => Ok(FindingSeverity::Note),
        _ => Err("AUDIT_MODEL_SCHEMA_INVALID: unsupported finding severity".into()),
    }
}
fn parse_coverage(value: &str) -> Result<CoverageStatus, String> {
    match value.to_ascii_uppercase().as_str() {
        "VERIFIED" => Ok(CoverageStatus::Verified),
        "PARTIAL" => Ok(CoverageStatus::Partial),
        "UNVERIFIED" => Ok(CoverageStatus::Unverified),
        "FAILED" => Ok(CoverageStatus::Failed),
        "NOT_APPLICABLE" => Ok(CoverageStatus::NotApplicable),
        _ => Err("AUDIT_MODEL_SCHEMA_INVALID: unsupported coverage status".into()),
    }
}
fn parse_confidence(value: &str) -> Result<ConfidenceLevel, String> {
    match value.to_ascii_uppercase().as_str() {
        "HIGH" => Ok(ConfidenceLevel::High),
        "MEDIUM" => Ok(ConfidenceLevel::Medium),
        "LOW" => Ok(ConfidenceLevel::Low),
        _ => Err("AUDIT_MODEL_SCHEMA_INVALID: unsupported confidence".into()),
    }
}
fn parse_risk(value: &str) -> Result<RegressionRisk, String> {
    match value.to_ascii_uppercase().as_str() {
        "LOW" => Ok(RegressionRisk::Low),
        "MEDIUM" => Ok(RegressionRisk::Medium),
        "HIGH" => Ok(RegressionRisk::High),
        "CRITICAL" => Ok(RegressionRisk::Critical),
        _ => Err("AUDIT_MODEL_SCHEMA_INVALID: unsupported regression risk".into()),
    }
}

fn parse_disposition(value: &str) -> Result<PriorFindingDispositionKind, String> {
    match value.to_ascii_uppercase().as_str() {
        "STILL_OPEN" => Ok(PriorFindingDispositionKind::StillOpen),
        "CLOSED" => Ok(PriorFindingDispositionKind::Closed),
        "SUPERSEDED" => Ok(PriorFindingDispositionKind::Superseded),
        _ => Err("AUDIT_MODEL_SCHEMA_INVALID: unsupported prior finding disposition".into()),
    }
}

fn unavailable_evaluation<M: AuditModel>(
    error: String,
    input: &AuditInput,
    model: &M,
) -> AuditEvaluation {
    let mut coverage = Vec::new();
    if let Some(task) = &input.task {
        for (index, requirement) in task.requirements.iter().take(MAX_REQUIREMENTS).enumerate() {
            let requirement_ref = format!("task-requirement-{index}");
            coverage.push(RequirementCoverage { id: format!("coverage-{index}"), logical_coverage_id: requirement_ref.clone(), requirement_ref, requirement_text: truncate_utf8(requirement, 4096), status: CoverageStatus::Unverified, evidence_refs: vec![format!("TASK_REQUIREMENTS:{}", task.task_id)], rationale: "No configured GPT audit model was available to verify this requirement.".into() });
        }
    }
    let model_status = if error.contains("AUTH_POLICY_BLOCKED") {
        "AUTH_POLICY_BLOCKED"
    } else if error.contains("AUTH_REQUIRED") {
        "AUTH_REQUIRED"
    } else if error.contains("USAGE_LIMITED") {
        "USAGE_LIMITED"
    } else if error.contains("TIMEOUT") {
        "TIMEOUT"
    } else if error.contains("NETWORK_ERROR") {
        "NETWORK_ERROR"
    } else if error.contains("PROCESS_ERROR") || error.contains("FINAL_OUTPUT") {
        "PROCESS_ERROR"
    } else {
        "UNAVAILABLE"
    };
    let summary = match model_status {
        "AUTH_POLICY_BLOCKED" => {
            "Codex reports API-key authentication, which H!veAI does not accept for audits."
        }
        "AUTH_REQUIRED" => {
            "Codex is not authenticated through the owner's supported ChatGPT login."
        }
        "USAGE_LIMITED" => "Codex usage is limited; no model verdict is treated as PASS.",
        "TIMEOUT" => "Codex audit execution timed out; no model verdict is treated as PASS.",
        "NETWORK_ERROR" => {
            "The Codex audit provider was unavailable; no model verdict is treated as PASS."
        }
        "PROCESS_ERROR" => "Codex audit execution failed; no model verdict is treated as PASS.",
        _ => {
            "Audit evidence was collected, but no supported Codex CLI audit provider is available."
        }
    };
    AuditEvaluation {
        verdict: AuditVerdict::Conditional,
        confidence: ConfidenceLevel::Low,
        regression_risk: RegressionRisk::High,
        summary: summary.into(),
        findings: Vec::new(),
        prior_finding_dispositions: Vec::new(),
        coverage,
        model_status: model_status.into(),
        diagnostic: Some(truncate_utf8(&error, 2048)),
        auditor_provider: Some(model.provider()),
        auditor_model: Some(model.model()),
        auditor_version: Some(model.version()),
    }
}

fn validate_semantic_evaluation(
    input: &AuditInput,
    evaluation: &mut AuditEvaluation,
) -> Result<(), String> {
    validate_evidence_references(input, evaluation)?;
    let evidence_status = input
        .evidence
        .iter()
        .map(|item| (item.logical_evidence_id.as_str(), item.verification_status))
        .collect::<HashMap<_, _>>();
    let canonical_requirements = input
        .requirements
        .iter()
        .filter(|requirement| requirement.required)
        .map(|requirement| requirement.requirement_ref.as_str())
        .collect::<HashSet<_>>();
    let mut finding_keys = HashSet::new();
    for finding in &mut evaluation.findings {
        if !finding_keys.insert(finding.finding_key.as_str()) {
            return Err("AUDIT_FINDING_KEY_DUPLICATE".into());
        }
        finding.logical_finding_id = finding.finding_key.clone();
        if finding
            .requirement_refs
            .iter()
            .any(|reference| !canonical_requirements.contains(reference.as_str()))
        {
            return Err("AUDIT_REQUIREMENT_REFERENCE_UNKNOWN".into());
        }
        if finding.evidence_refs.iter().any(|reference| {
            matches!(
                evidence_status.get(reference.as_str()),
                Some(
                    VerificationStatus::ClaimOnly
                        | VerificationStatus::Unverified
                        | VerificationStatus::Partial
                        | VerificationStatus::Unavailable
                        | VerificationStatus::Stale
                        | VerificationStatus::Truncated
                )
            )
        }) {
            finding.confidence = ConfidenceLevel::Low;
        }
    }
    let mut coverage_keys = HashSet::new();
    let mut weak_coverage = false;
    let mut failed_coverage = false;
    for coverage in &mut evaluation.coverage {
        if !coverage_keys.insert(coverage.requirement_ref.as_str()) {
            return Err("AUDIT_REQUIREMENT_REFERENCE_DUPLICATE".into());
        }
        coverage.logical_coverage_id = coverage.requirement_ref.clone();
        if canonical_requirements.is_empty() {
            if coverage.requirement_ref != "project-audit"
                || coverage.status != CoverageStatus::NotApplicable
            {
                return Err("AUDIT_REQUIREMENT_REFERENCE_UNKNOWN".into());
            }
        } else if !canonical_requirements.contains(coverage.requirement_ref.as_str()) {
            return Err("AUDIT_REQUIREMENT_REFERENCE_UNKNOWN".into());
        }
        let weak = coverage.evidence_refs.is_empty()
            || coverage.evidence_refs.iter().any(|reference| {
                matches!(
                    evidence_status.get(reference.as_str()),
                    Some(
                        VerificationStatus::ClaimOnly
                            | VerificationStatus::Unverified
                            | VerificationStatus::Partial
                            | VerificationStatus::Unavailable
                            | VerificationStatus::Stale
                            | VerificationStatus::Truncated
                    )
                )
            });
        if coverage.status == CoverageStatus::Verified && weak {
            coverage.status = CoverageStatus::Unverified;
            coverage.rationale = format!(
                "{} Evidence quality is insufficient for VERIFIED coverage.",
                coverage.rationale
            );
        }
        weak_coverage |= matches!(
            coverage.status,
            CoverageStatus::Partial | CoverageStatus::Unverified
        );
        failed_coverage |= coverage.status == CoverageStatus::Failed;
    }
    let complete_required_coverage = if canonical_requirements.is_empty() {
        evaluation.coverage.len() == 1
            && evaluation.coverage[0].requirement_ref == "project-audit"
            && evaluation.coverage[0].status == CoverageStatus::NotApplicable
    } else {
        canonical_requirements.iter().all(|reference| {
            evaluation
                .coverage
                .iter()
                .filter(|coverage| coverage.requirement_ref == *reference)
                .count()
                == 1
        }) && evaluation
            .coverage
            .iter()
            .all(|coverage| canonical_requirements.contains(coverage.requirement_ref.as_str()))
    };
    let required_evidence = evaluation
        .coverage
        .iter()
        .filter(|coverage| coverage.status != CoverageStatus::NotApplicable)
        .flat_map(|coverage| coverage.evidence_refs.iter())
        .chain(
            evaluation
                .findings
                .iter()
                .flat_map(|finding| finding.evidence_refs.iter()),
        )
        .collect::<HashSet<_>>();
    let weak_required_evidence = required_evidence.iter().any(|reference| {
        matches!(
            evidence_status.get(reference.as_str()),
            Some(
                VerificationStatus::ClaimOnly
                    | VerificationStatus::Unverified
                    | VerificationStatus::Partial
                    | VerificationStatus::Unavailable
                    | VerificationStatus::Stale
                    | VerificationStatus::Truncated
            )
        )
    });
    let finding_keys = evaluation
        .findings
        .iter()
        .map(|finding| finding.finding_key.as_str())
        .collect::<HashSet<_>>();
    for disposition in &evaluation.prior_finding_dispositions {
        if disposition.disposition == PriorFindingDispositionKind::Superseded
            && disposition
                .replacement_finding_key
                .as_deref()
                .map(|key| !finding_keys.contains(key))
                .unwrap_or(true)
        {
            return Err("AUDIT_SUPERSEDED_REPLACEMENT_NOT_FOUND".into());
        }
    }
    let unresolved_release_blocker = evaluation.findings.iter().any(|finding| {
        finding.blocks_release && !matches!(finding.status.as_str(), "CLOSED" | "SUPERSEDED")
    });
    let unresolved_major_or_blocker = evaluation.findings.iter().any(|finding| {
        matches!(
            finding.severity,
            FindingSeverity::Major | FindingSeverity::Blocker
        ) && !matches!(finding.status.as_str(), "CLOSED" | "SUPERSEDED")
    });
    let weak_evidence = weak_coverage
        || !complete_required_coverage
        || weak_required_evidence
        || (!evaluation.findings.is_empty()
            && evaluation
                .findings
                .iter()
                .any(|finding| finding.evidence_refs.is_empty()))
        || !input.git.identity_complete
        || input
            .task
            .as_ref()
            .is_some_and(|task| task.requirements_status != VerificationStatus::Verified);
    if evaluation.verdict == AuditVerdict::Pass {
        if unresolved_major_or_blocker || failed_coverage || unresolved_release_blocker {
            evaluation.verdict = AuditVerdict::Fail;
            evaluation.regression_risk = if evaluation
                .findings
                .iter()
                .any(|finding| finding.severity == FindingSeverity::Blocker)
            {
                RegressionRisk::Critical
            } else {
                RegressionRisk::High
            };
            evaluation.summary = format!(
                "Deterministic audit guard rejected PASS: {}",
                evaluation.summary
            );
        } else if weak_evidence {
            evaluation.verdict = AuditVerdict::Conditional;
            evaluation.confidence = ConfidenceLevel::Low;
            evaluation.regression_risk = RegressionRisk::High;
            evaluation.summary = format!(
                "Deterministic audit guard downgraded PASS for insufficient evidence: {}",
                evaluation.summary
            );
        }
    }
    if weak_evidence && evaluation.confidence == ConfidenceLevel::High {
        evaluation.confidence = ConfidenceLevel::Low;
    }
    Ok(())
}

fn evaluate_with<M: AuditModel>(model: &M, input: &AuditInput) -> AuditEvaluation {
    match model.evaluate(input) {
        Ok(raw) => match parse_model_output(&raw) {
            Ok(mut evaluation) => {
                if let Err(error) = validate_semantic_evaluation(input, &mut evaluation) {
                    semantic_degraded_evaluation(error, model)
                } else {
                    evaluation.auditor_provider = Some(model.provider());
                    evaluation.auditor_model = Some(model.model());
                    evaluation.auditor_version = Some(model.version());
                    evaluation
                }
            }
            Err(error) => semantic_degraded_evaluation(error, model),
        },
        Err(error) => unavailable_evaluation(error, input, model),
    }
}

fn resolve_reaudit_request(
    database: &DatabaseState,
    request: AuditInputRequest,
) -> Result<AuditInputRequest, String> {
    let Some(prior_id) = request.prior_audit_id.as_deref() else {
        return Ok(request);
    };
    let connection = database.open_connection()?;
    let prior_project_matches: i64 = connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM audits WHERE id=?1 AND project_id=?2)",
            params![prior_id, request.project_id],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    if prior_project_matches != 1 {
        return Err("AUDIT_PRIOR_PROJECT_MISMATCH_OR_NOT_FOUND".into());
    }
    let prior = get(database, &request.project_id, prior_id)?;
    if request.task_id != prior.task_id {
        return Err("AUDIT_REAUDIT_TASK_TARGET_MISMATCH".into());
    }
    let prior_target = AuditGitTarget {
        scope: prior.git_scope,
        base_ref: (prior.git_scope == GitDiffScope::CommitRange)
            .then(|| prior.baseline_ref.clone())
            .flatten(),
        head_sha: None,
        target_origin: AuditTargetOrigin::PriorAudit,
        audited_session_id: prior.audited_session_id.clone(),
        audited_prompt_version_id: prior.audited_prompt_version_id.clone(),
    };
    if request.git_target.target_origin == AuditTargetOrigin::Auto {
        return Ok(AuditInputRequest {
            git_target: prior_target,
            ..request
        });
    }
    let target = &request.git_target;
    if target.scope != prior.git_scope {
        return Err("AUDIT_REAUDIT_GIT_SCOPE_MISMATCH".into());
    }
    if target.scope == GitDiffScope::CommitRange
        && target.base_ref.as_deref() != prior.baseline_ref.as_deref()
    {
        return Err("AUDIT_REAUDIT_GIT_BASE_MISMATCH".into());
    }
    let prior_provenance = (
        prior.audited_session_id.as_deref(),
        prior.audited_prompt_version_id.as_deref(),
    );
    let requested_provenance = (
        target.audited_session_id.as_deref(),
        target.audited_prompt_version_id.as_deref(),
    );
    if requested_provenance != prior_provenance
        && !(prior.remediation_session_id.as_deref() == requested_provenance.0
            && prior.remediation_prompt_version_id.as_deref() == requested_provenance.1)
    {
        return Err("AUDIT_REAUDIT_PROVENANCE_MISMATCH".into());
    }
    Ok(request)
}

fn semantic_degraded_evaluation<M: AuditModel>(error: String, model: &M) -> AuditEvaluation {
    AuditEvaluation {
        verdict: AuditVerdict::Conditional,
        confidence: ConfidenceLevel::Low,
        regression_risk: RegressionRisk::High,
        summary: "Structured audit model output was semantically invalid; no authoritative verdict was invented.".into(),
        findings: Vec::new(),
        prior_finding_dispositions: Vec::new(),
        coverage: Vec::new(),
        model_status: "MALFORMED".into(),
        diagnostic: Some(error),
        auditor_provider: Some(model.provider().into()),
        auditor_model: Some(model.model().into()),
        auditor_version: Some(model.version().into()),
    }
}

pub fn run(database: &DatabaseState, request: AuditInputRequest) -> Result<AuditRun, String> {
    let model = resolve_production_model(database);
    run_with_model(database, request, &model)
}

fn run_with_model<M: AuditModel>(
    database: &DatabaseState,
    request: AuditInputRequest,
    model: &M,
) -> Result<AuditRun, String> {
    let request = resolve_reaudit_request(database, request)?;
    let input = collect_input(database, request.clone())?;
    let started_at = utc_timestamp();
    let mut evaluation = evaluate_with(model, &input);
    let state = if current_freshness_token(database, &input.project_id, &request.git_target)?
        != input.freshness_token
    {
        evaluation.verdict = AuditVerdict::Conditional;
        evaluation.confidence = ConfidenceLevel::Low;
        evaluation.regression_risk = RegressionRisk::High;
        evaluation.summary =
            "Audit evidence became stale before persistence; re-run the audit.".into();
        evaluation.diagnostic = Some("AUDIT_STALE_REPOSITORY_CHANGED: Git authority changed after evidence collection; re-run audit.".into());
        evaluation.findings.clear();
        evaluation.prior_finding_dispositions.clear();
        evaluation.coverage.clear();
        AuditState::Stale
    } else {
        AuditState::Completed
    };
    let result = persist_run(
        database,
        &input,
        request.prior_audit_id,
        &started_at,
        evaluation,
        state,
    )?;
    crate::control_plane::materialize_best_effort(database, &input.project_id, "AUDIT_LIFECYCLE");
    Ok(result)
}

#[cfg(test)]
pub fn evaluate_fixture(input: &AuditInput, model: &FixtureAuditModel) -> AuditEvaluation {
    evaluate_with(model, input)
}

#[cfg(test)]
pub fn run_fixture(
    database: &DatabaseState,
    request: AuditInputRequest,
    model: &FixtureAuditModel,
) -> Result<AuditRun, String> {
    run_with_model(database, request, model)
}

fn persist_run(
    database: &DatabaseState,
    input: &AuditInput,
    prior_audit_id: Option<String>,
    started_at: &str,
    mut evaluation: AuditEvaluation,
    state: AuditState,
) -> Result<AuditRun, String> {
    let audit_id = Uuid::new_v4().to_string();
    let finished_at = utc_timestamp();
    let connection = database.open_connection()?;
    let tx = connection
        .unchecked_transaction()
        .map_err(|e| e.to_string())?;
    if let Some(prior_id) = &prior_audit_id {
        let project_matches: i64 = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM audits WHERE id=?1 AND project_id=?2)",
                params![prior_id, input.project_id],
                |row| row.get(0),
            )
            .map_err(|e| e.to_string())?;
        if project_matches != 1 {
            return Err("AUDIT_PRIOR_PROJECT_MISMATCH_OR_NOT_FOUND".into());
        }
        // Only an eligible completed model may interpret prior findings. Degraded
        // and stale runs remain truthful history without synthetic dispositions.
        if state == AuditState::Completed && evaluation.model_status == "AVAILABLE" {
            apply_prior_finding_dispositions(&tx, prior_id, &audit_id, input, &mut evaluation)?;
        }
    }
    validate_semantic_evaluation(input, &mut evaluation)?;
    tx.execute("INSERT INTO audits (id,project_id,task_id,result,summary,confidence,created_at,audit_type,audited_branch,audited_head_sha,baseline_ref,input_manifest_sha256,schema_version,confidence_level,regression_risk,state,started_at,finished_at,auditor_provider,auditor_model,auditor_version,model_status,diagnostic,prior_audit_id,freshness_token,git_scope,audited_base_sha,audited_change_set_sha256,audited_session_id,audited_prompt_version_id,target_origin) VALUES (?1,?2,?3,?4,?5,?6,?7,'IMPLEMENTATION',?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30)", params![audit_id, input.project_id, input.task_id, evaluation.verdict.as_str(), evaluation.summary, evaluation.confidence.score(), finished_at, input.audited_branch, input.audited_head_sha, input.baseline_ref, input.input_manifest_sha256, AUDIT_SCHEMA_VERSION, evaluation.confidence.as_str(), evaluation.regression_risk.as_str(), state.as_str(), started_at, finished_at, evaluation.auditor_provider, evaluation.auditor_model, evaluation.auditor_version, evaluation.model_status, evaluation.diagnostic, prior_audit_id, input.freshness_token, git_scope_str(input.git.scope), input.git.base_sha, input.git.full_change_set_sha256, input.audited_session_id, input.audited_prompt_version_id, target_origin_str(input.git.target_origin)]).map_err(|e| e.to_string())?;
    for finding in &evaluation.findings {
        let persisted_id = format!(
            "{audit_id}:finding:{}",
            hash_bytes(finding.logical_finding_id.as_bytes())[..16].to_string()
        );
        tx.execute("INSERT INTO audit_findings (id,audit_id,logical_finding_id,severity,title,detail,file_path,line_number,created_at,finding_key,confidence_level,status,requirement_refs_json,evidence_refs_json,source_locator,test_locator,remediation_guidance,blocks_release,prior_finding_key,disposition,disposition_evidence_refs_json,disposition_rationale,superseded_by_finding_key,inherited_prior_audit_id,inherited_prior_evidence_refs_json) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25)", params![persisted_id, audit_id, finding.logical_finding_id, finding.severity.as_str(), finding.title, finding.detail, finding.source_locator.as_deref().and_then(|value| value.split(':').next()), finding.source_locator.as_deref().and_then(|value| value.rsplit(':').next()).and_then(|value| value.parse::<i64>().ok()), finished_at, finding.finding_key, finding.confidence.as_str(), finding.status, serde_json::to_string(&finding.requirement_refs).unwrap_or_else(|_| "[]".into()), serde_json::to_string(&finding.evidence_refs).unwrap_or_else(|_| "[]".into()), finding.source_locator, finding.test_locator, finding.remediation_guidance, finding.blocks_release as i64, finding.prior_finding_key, finding.disposition, serde_json::to_string(&finding.disposition_evidence_refs).unwrap_or_else(|_| "[]".into()), finding.disposition_rationale, finding.superseded_by_finding_key, finding.inherited_prior_audit_id, serde_json::to_string(&finding.inherited_prior_evidence_refs).unwrap_or_else(|_| "[]".into())]).map_err(|e| e.to_string())?;
    }
    for coverage in &evaluation.coverage {
        let persisted_id = format!(
            "{audit_id}:coverage:{}",
            hash_bytes(coverage.logical_coverage_id.as_bytes())[..16].to_string()
        );
        tx.execute("INSERT INTO audit_requirement_coverage (id,audit_id,logical_coverage_id,requirement_ref,requirement_text,status,evidence_refs_json,rationale,created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)", params![persisted_id, audit_id, coverage.logical_coverage_id, coverage.requirement_ref, coverage.requirement_text, coverage.status.as_str(), serde_json::to_string(&coverage.evidence_refs).unwrap_or_else(|_| "[]".into()), coverage.rationale, finished_at]).map_err(|e| e.to_string())?;
    }
    for item in &input.evidence {
        // Evidence identities are stable within an input, so scope the persisted
        // primary key to its immutable audit row while retaining logical refs.
        let persisted_evidence_id = format!("{audit_id}:{}", item.id);
        tx.execute("INSERT INTO audit_evidence (id,audit_id,logical_evidence_id,evidence_kind,verification_status,locator,summary,content,content_sha256,byte_count,truncated,created_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)", params![persisted_evidence_id, audit_id, item.logical_evidence_id, item.kind, item.verification_status.as_str(), item.locator, item.summary, item.content, item.content_sha256, item.byte_count as i64, item.truncated as i64, finished_at]).map_err(|e| e.to_string())?;
    }
    crate::control_plane::mark_truth_dirty_tx(&tx, &input.project_id, "AUDIT_LIFECYCLE")?;
    tx.commit().map_err(|e| e.to_string())?;
    get(database, &input.project_id, &audit_id)
}

pub fn list(database: &DatabaseState, project_id: &str) -> Result<Vec<AuditRun>, String> {
    valid_project(database, project_id)?;
    let connection = database.open_connection()?;
    let mut statement = connection
        .prepare(
            "SELECT id FROM audits WHERE project_id=?1 ORDER BY created_at DESC,id DESC LIMIT ?2",
        )
        .map_err(|e| e.to_string())?;
    let ids = statement
        .query_map(params![project_id, MAX_HISTORY as i64], |row| {
            row.get::<_, String>(0)
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    ids.into_iter()
        .map(|id| get(database, project_id, &id))
        .collect()
}

pub fn get(database: &DatabaseState, project_id: &str, audit_id: &str) -> Result<AuditRun, String> {
    valid_project(database, project_id)?;
    let connection = database.open_connection()?;
    let row = connection.query_row("SELECT id,project_id,task_id,audit_type,audited_branch,audited_head_sha,baseline_ref,input_manifest_sha256,schema_version,result,confidence_level,regression_risk,state,summary,started_at,finished_at,auditor_provider,auditor_model,auditor_version,model_status,diagnostic,prior_audit_id,remediation_prompt_id,remediation_prompt_version_id,remediation_session_id,freshness_token FROM audits WHERE id=?1 AND project_id=?2", params![audit_id, project_id], |row| Ok((row.get::<_, String>(0)?,row.get::<_, String>(1)?,row.get::<_, Option<String>>(2)?,row.get::<_, String>(3)?,row.get::<_, Option<String>>(4)?,row.get::<_, Option<String>>(5)?,row.get::<_, Option<String>>(6)?,row.get::<_, String>(7)?,row.get::<_, i64>(8)?,row.get::<_, String>(9)?,row.get::<_, Option<String>>(10)?,row.get::<_, Option<String>>(11)?,row.get::<_, String>(12)?,row.get::<_, Option<String>>(13)?,row.get::<_, Option<String>>(14)?,row.get::<_, Option<String>>(15)?,row.get::<_, Option<String>>(16)?,row.get::<_, Option<String>>(17)?,row.get::<_, Option<String>>(18)?,row.get::<_, String>(19)?,row.get::<_, Option<String>>(20)?,row.get::<_, Option<String>>(21)?,row.get::<_, Option<String>>(22)?,row.get::<_, Option<String>>(23)?,row.get::<_, Option<String>>(24)?,row.get::<_, Option<String>>(25)?))).optional().map_err(|e| e.to_string())?.ok_or_else(|| "AUDIT_NOT_FOUND_OR_PROJECT_MISMATCH".to_string())?;
    let findings = read_findings(&connection, audit_id)?;
    let coverage = read_coverage(&connection, audit_id)?;
    let evidence = read_evidence(&connection, audit_id)?;
    let identity = connection
        .query_row(
            "SELECT git_scope,audited_base_sha,audited_change_set_sha256,audited_session_id,audited_prompt_version_id,target_origin FROM audits WHERE id=?1 AND project_id=?2",
            params![audit_id, project_id],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, Option<String>>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, Option<String>>(3)?, row.get::<_, Option<String>>(4)?, row.get::<_, Option<String>>(5)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?
        .unwrap_or_else(|| ("WORKING_TREE".into(), None, None, None, None, Some("AUTO".into())));
    let confidence = match row.10.as_deref().unwrap_or("LOW") {
        "HIGH" => ConfidenceLevel::High,
        "MEDIUM" => ConfidenceLevel::Medium,
        _ => ConfidenceLevel::Low,
    };
    let verdict = parse_verdict(&row.9)?;
    let regression_risk = parse_risk(row.11.as_deref().unwrap_or("HIGH"))?;
    let state = match row.12.as_str() {
        "PREPARING" => AuditState::Preparing,
        "READY" => AuditState::Ready,
        "RUNNING" => AuditState::Running,
        "FAILED" => AuditState::Failed,
        "STALE" => AuditState::Stale,
        "CANCELLED" => AuditState::Cancelled,
        _ => AuditState::Completed,
    };
    Ok(AuditRun {
        id: row.0,
        project_id: row.1,
        task_id: row.2,
        audit_type: row.3,
        audited_branch: row.4,
        audited_head_sha: row.5,
        baseline_ref: row.6,
        git_scope: match identity.0.as_str() {
            "STAGED" => GitDiffScope::Staged,
            "COMMIT_RANGE" => GitDiffScope::CommitRange,
            _ => GitDiffScope::WorkingTree,
        },
        target_origin: parse_target_origin(identity.5.as_deref().unwrap_or("AUTO")),
        audited_base_sha: identity.1,
        audited_change_set_sha256: identity.2,
        audited_session_id: identity.3,
        audited_prompt_version_id: identity.4,
        input_manifest_sha256: row.7,
        freshness_token: row.25.unwrap_or_default(),
        schema_version: row.8,
        verdict,
        confidence,
        regression_risk,
        state,
        summary: row.13.unwrap_or_default(),
        started_at: row.14.unwrap_or_default(),
        finished_at: row.15,
        auditor_provider: row.16,
        auditor_model: row.17,
        auditor_version: row.18,
        model_status: row.19,
        diagnostic: row.20,
        prior_audit_id: row.21,
        remediation_prompt_id: row.22,
        remediation_prompt_version_id: row.23,
        remediation_session_id: row.24,
        findings,
        coverage,
        evidence,
    })
}

fn apply_prior_finding_dispositions(
    tx: &rusqlite::Transaction<'_>,
    prior_audit_id: &str,
    current_audit_id: &str,
    input: &AuditInput,
    evaluation: &mut AuditEvaluation,
) -> Result<(), String> {
    if evaluation.model_status != "AVAILABLE" {
        return Err("AUDIT_PRIOR_DISPOSITION_PROVIDER_NOT_ELIGIBLE".into());
    }
    let mut statement = tx.prepare("SELECT id,finding_key,severity,title,detail,requirement_refs_json,evidence_refs_json,source_locator,test_locator,confidence_level,remediation_guidance,blocks_release FROM audit_findings WHERE audit_id=?1 AND status IN ('OPEN','STILL_OPEN')").map_err(|e| e.to_string())?;
    let prior = statement
        .query_map([prior_audit_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, Option<String>>(10)?,
                row.get::<_, i64>(11)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;
    let evidence_ids = input
        .evidence
        .iter()
        .map(|item| item.logical_evidence_id.as_str())
        .collect::<std::collections::HashSet<_>>();
    let mut seen = std::collections::HashSet::new();
    for disposition in &evaluation.prior_finding_dispositions {
        if !seen.insert(disposition.prior_finding_key.as_str()) {
            return Err("AUDIT_PRIOR_DISPOSITION_DUPLICATE_KEY".into());
        }
        let prior_row = prior
            .iter()
            .find(|row| row.1.as_deref().unwrap_or(row.0.as_str()) == disposition.prior_finding_key)
            .ok_or_else(|| "AUDIT_PRIOR_DISPOSITION_UNKNOWN_FINDING_KEY".to_string())?;
        if disposition
            .evidence_refs
            .iter()
            .any(|reference| !evidence_ids.contains(reference.as_str()))
        {
            return Err("AUDIT_PRIOR_DISPOSITION_EVIDENCE_NOT_IN_CURRENT_INPUT".into());
        }
        if matches!(
            disposition.disposition,
            PriorFindingDispositionKind::Closed | PriorFindingDispositionKind::Superseded
        ) && (disposition.evidence_refs.is_empty() || disposition.rationale.trim().is_empty())
        {
            return Err("AUDIT_PRIOR_DISPOSITION_CLOSURE_PROOF_REQUIRED".into());
        }
        if matches!(
            disposition.disposition,
            PriorFindingDispositionKind::Superseded
        ) && disposition.replacement_finding_key.is_none()
        {
            return Err("AUDIT_PRIOR_DISPOSITION_REPLACEMENT_REQUIRED".into());
        }
        let (
            old_id,
            finding_key,
            severity,
            title,
            detail,
            requirement_refs_json,
            _evidence_refs_json,
            source_locator,
            test_locator,
            confidence_level,
            remediation_guidance,
            blocks_release,
        ) = prior_row.clone();
        let key = finding_key.unwrap_or_else(|| old_id.clone());
        if evaluation
            .findings
            .iter()
            .any(|finding| finding.finding_key == key)
        {
            continue;
        }
        let status = match disposition.disposition {
            PriorFindingDispositionKind::StillOpen => "OPEN",
            PriorFindingDispositionKind::Closed => "CLOSED",
            PriorFindingDispositionKind::Superseded => "SUPERSEDED",
        }
        .to_string();
        evaluation.findings.push(AuditFinding {
            id: format!("disposition-{}-{}", current_audit_id, old_id),
            logical_finding_id: key.clone(),
            finding_key: key.clone(),
            severity: parse_severity(&severity).unwrap_or(FindingSeverity::Note),
            title,
            detail: detail.unwrap_or_default(),
            requirement_refs: parse_list(requirement_refs_json),
            // Prior evidence is never copied into the current namespace. A
            // STILL_OPEN row without current proof remains open and carries
            // its old refs only in inherited_prior_evidence_refs below.
            evidence_refs: disposition.evidence_refs.clone(),
            source_locator,
            test_locator,
            confidence: parse_confidence(confidence_level.as_deref().unwrap_or("LOW"))
                .unwrap_or(ConfidenceLevel::Low),
            status,
            remediation_guidance: remediation_guidance.unwrap_or_default(),
            blocks_release: blocks_release != 0,
            closed_by_audit_id: if disposition.disposition == PriorFindingDispositionKind::StillOpen
            {
                None
            } else {
                Some(current_audit_id.into())
            },
            prior_finding_key: Some(disposition.prior_finding_key.clone()),
            disposition: Some(disposition.disposition.as_str().into()),
            disposition_evidence_refs: disposition.evidence_refs.clone(),
            disposition_rationale: Some(disposition.rationale.clone()),
            superseded_by_finding_key: disposition.replacement_finding_key.clone(),
            inherited_prior_audit_id: None,
            inherited_prior_evidence_refs: Vec::new(),
        });
    }
    let explicit_keys = evaluation
        .prior_finding_dispositions
        .iter()
        .map(|item| item.prior_finding_key.as_str())
        .collect::<HashSet<_>>();
    for row in prior {
        let key = row.1.clone().unwrap_or_else(|| row.0.clone());
        if explicit_keys.contains(key.as_str())
            || evaluation
                .findings
                .iter()
                .any(|finding| finding.finding_key == key)
        {
            continue;
        }
        evaluation.findings.push(AuditFinding {
            id: format!("inherited-{}-{}", current_audit_id, row.0),
            logical_finding_id: key.clone(),
            finding_key: key,
            severity: parse_severity(&row.2).unwrap_or(FindingSeverity::Note),
            title: row.3,
            detail: row.4.unwrap_or_default(),
            requirement_refs: parse_list(row.5),
            evidence_refs: Vec::new(),
            source_locator: row.7,
            test_locator: row.8,
            confidence: parse_confidence(row.9.as_deref().unwrap_or("LOW"))
                .unwrap_or(ConfidenceLevel::Low),
            status: "OPEN".into(),
            remediation_guidance: row.10.unwrap_or_default(),
            blocks_release: row.11 != 0,
            closed_by_audit_id: None,
            prior_finding_key: Some(row.1.unwrap_or(row.0)),
            disposition: Some("STILL_OPEN".into()),
            disposition_evidence_refs: Vec::new(),
            disposition_rationale: Some(
                "Inherited unresolved prior finding; omission cannot close it.".into(),
            ),
            superseded_by_finding_key: None,
            inherited_prior_audit_id: Some(prior_audit_id.to_string()),
            inherited_prior_evidence_refs: parse_list(row.6),
        });
    }
    Ok(())
}

fn read_findings(connection: &Connection, audit_id: &str) -> Result<Vec<AuditFinding>, String> {
    let mut statement = connection.prepare("SELECT id,logical_finding_id,finding_key,severity,title,detail,requirement_refs_json,evidence_refs_json,source_locator,test_locator,confidence_level,status,remediation_guidance,blocks_release,closed_by_audit_id,prior_finding_key,disposition,disposition_evidence_refs_json,disposition_rationale,superseded_by_finding_key,inherited_prior_audit_id,inherited_prior_evidence_refs_json FROM audit_findings WHERE audit_id=?1 ORDER BY severity,id").map_err(|e| e.to_string())?;
    let rows = statement
        .query_map([audit_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, Option<String>>(10)?,
                row.get::<_, String>(11)?,
                row.get::<_, Option<String>>(12)?,
                row.get::<_, i64>(13)?,
                row.get::<_, Option<String>>(14)?,
                row.get::<_, Option<String>>(15)?,
                row.get::<_, Option<String>>(16)?,
                row.get::<_, Option<String>>(17)?,
                row.get::<_, Option<String>>(18)?,
                row.get::<_, Option<String>>(19)?,
                row.get::<_, Option<String>>(20)?,
                row.get::<_, Option<String>>(21)?,
            ))
        })
        .map_err(|e| e.to_string())?;
    rows.map(|row| {
        row.map(|row| {
            let id = row.0;
            AuditFinding {
                id: id.clone(),
                logical_finding_id: row
                    .1
                    .clone()
                    .unwrap_or_else(|| row.2.clone().unwrap_or_else(|| id.clone())),
                finding_key: row.2.unwrap_or_default(),
                severity: parse_severity(&row.3).unwrap_or(FindingSeverity::Note),
                title: row.4,
                detail: row.5.unwrap_or_default(),
                requirement_refs: parse_list(row.6),
                evidence_refs: parse_list(row.7),
                source_locator: row.8,
                test_locator: row.9,
                confidence: parse_confidence(row.10.as_deref().unwrap_or("LOW"))
                    .unwrap_or(ConfidenceLevel::Low),
                status: row.11,
                remediation_guidance: row.12.unwrap_or_default(),
                blocks_release: row.13 != 0,
                closed_by_audit_id: row.14,
                prior_finding_key: row.15,
                disposition: row.16,
                disposition_evidence_refs: parse_list(row.17),
                disposition_rationale: row.18,
                superseded_by_finding_key: row.19,
                inherited_prior_audit_id: row.20,
                inherited_prior_evidence_refs: parse_list(row.21),
            }
        })
    })
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())
}

fn read_coverage(
    connection: &Connection,
    audit_id: &str,
) -> Result<Vec<RequirementCoverage>, String> {
    let mut statement = connection.prepare("SELECT id,logical_coverage_id,requirement_ref,requirement_text,status,evidence_refs_json,rationale FROM audit_requirement_coverage WHERE audit_id=?1 ORDER BY requirement_ref,id").map_err(|e| e.to_string())?;
    let rows = statement
        .query_map([audit_id], |row| {
            Ok(RequirementCoverage {
                id: row.get(0)?,
                logical_coverage_id: row
                    .get::<_, Option<String>>(1)?
                    .unwrap_or_else(|| row.get::<_, String>(0).unwrap_or_default()),
                requirement_ref: row.get(2)?,
                requirement_text: row.get(3)?,
                status: parse_coverage(&row.get::<_, String>(4)?)
                    .unwrap_or(CoverageStatus::Unverified),
                evidence_refs: parse_list(row.get(5)?),
                rationale: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn validate_evidence_references(
    input: &AuditInput,
    evaluation: &AuditEvaluation,
) -> Result<(), String> {
    let mut logical_ids = std::collections::HashSet::new();
    for item in &input.evidence {
        if item.logical_evidence_id.trim().is_empty() {
            return Err("AUDIT_EVIDENCE_REFERENCE_NOT_FOUND".into());
        }
        if !logical_ids.insert(item.logical_evidence_id.as_str()) {
            return Err("AUDIT_EVIDENCE_REFERENCE_DUPLICATE".into());
        }
    }
    let validate = |references: &[String]| {
        references
            .iter()
            .all(|reference| logical_ids.contains(reference.as_str()))
    };
    if evaluation
        .findings
        .iter()
        .any(|finding| !validate(&finding.evidence_refs))
        || evaluation
            .coverage
            .iter()
            .any(|coverage| !validate(&coverage.evidence_refs))
        || evaluation
            .prior_finding_dispositions
            .iter()
            .any(|disposition| !validate(&disposition.evidence_refs))
    {
        return Err("AUDIT_EVIDENCE_REFERENCE_NOT_FOUND".into());
    }
    Ok(())
}

fn resolve_evidence_row_id(
    connection: &Connection,
    audit_id: &str,
    logical_evidence_id: &str,
) -> Result<String, String> {
    connection
        .query_row(
            "SELECT id FROM audit_evidence WHERE audit_id=?1 AND logical_evidence_id=?2",
            params![audit_id, logical_evidence_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "AUDIT_EVIDENCE_REFERENCE_NOT_FOUND".into())
}

fn read_evidence(connection: &Connection, audit_id: &str) -> Result<Vec<AuditEvidence>, String> {
    let mut statement = connection.prepare("SELECT id,logical_evidence_id,evidence_kind,verification_status,locator,summary,content,content_sha256,byte_count,truncated FROM audit_evidence WHERE audit_id=?1 ORDER BY id LIMIT ?2").map_err(|e| e.to_string())?;
    let rows = statement
        .query_map(params![audit_id, MAX_EVIDENCE_ITEMS as i64], |row| {
            Ok(AuditEvidence {
                id: row.get(0)?,
                logical_evidence_id: row
                    .get::<_, Option<String>>(1)?
                    .unwrap_or_else(|| row.get::<_, String>(0).unwrap_or_default()),
                kind: row.get(2)?,
                verification_status: parse_verification(&row.get::<_, String>(3)?),
                locator: row.get(4)?,
                summary: row.get(5)?,
                content: row.get(6)?,
                content_sha256: row.get(7)?,
                byte_count: row.get::<_, i64>(8)? as usize,
                truncated: row.get::<_, i64>(9)? != 0,
            })
        })
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

fn parse_list(raw: Option<String>) -> Vec<String> {
    raw.and_then(|value| serde_json::from_str(&value).ok())
        .unwrap_or_default()
}
fn parse_verification(value: &str) -> VerificationStatus {
    match value {
        "VERIFIED" => VerificationStatus::Verified,
        "CORROBORATED" => VerificationStatus::Corroborated,
        "CLAIM_ONLY" => VerificationStatus::ClaimOnly,
        "UNAVAILABLE" => VerificationStatus::Unavailable,
        "STALE" => VerificationStatus::Stale,
        "TRUNCATED" => VerificationStatus::Truncated,
        "EXCLUDED" => VerificationStatus::Excluded,
        "PARTIAL" => VerificationStatus::Partial,
        _ => VerificationStatus::Unverified,
    }
}

fn git_scope_str(scope: GitDiffScope) -> &'static str {
    match scope {
        GitDiffScope::WorkingTree => "WORKING_TREE",
        GitDiffScope::Staged => "STAGED",
        GitDiffScope::CommitRange => "COMMIT_RANGE",
    }
}

fn target_origin_str(origin: AuditTargetOrigin) -> &'static str {
    match origin {
        AuditTargetOrigin::Auto => "AUTO",
        AuditTargetOrigin::AgentSession => "AGENT_SESSION",
        AuditTargetOrigin::PriorAudit => "PRIOR_AUDIT",
        AuditTargetOrigin::RegisteredPolicy => "REGISTERED_POLICY",
        AuditTargetOrigin::Manual => "MANUAL",
    }
}

fn parse_target_origin(value: &str) -> AuditTargetOrigin {
    match value {
        "AGENT_SESSION" => AuditTargetOrigin::AgentSession,
        "PRIOR_AUDIT" => AuditTargetOrigin::PriorAudit,
        "REGISTERED_POLICY" => AuditTargetOrigin::RegisteredPolicy,
        "MANUAL" => AuditTargetOrigin::Manual,
        _ => AuditTargetOrigin::Auto,
    }
}

pub fn create_remediation_prompt(
    database: &DatabaseState,
    project_id: &str,
    audit_id: &str,
    finding_ids: Vec<String>,
    title: String,
    summary: String,
) -> Result<PromptVersion, String> {
    let audit = get(database, project_id, audit_id)?;
    let selected = audit
        .findings
        .iter()
        .filter(|finding| {
            finding_ids.iter().any(|id| id == &finding.id) && finding.status == "OPEN"
        })
        .map(|finding| finding.id.clone())
        .collect::<Vec<_>>();
    if selected.is_empty() || selected.len() != finding_ids.len() {
        return Err("AUDIT_FINDING_SELECTION_INVALID_OR_CLOSED".into());
    }
    let prompt = prompt_engine::generate(
        database,
        PromptGenerateRequest {
            project_id: project_id.into(),
            task_id: audit.task_id.clone(),
            kind: PromptKind::Remediation,
            title,
            summary,
            finding_ids: Some(selected),
        },
    )?;
    let connection = database.open_connection()?;
    connection.execute("UPDATE audits SET remediation_prompt_id=?1,remediation_prompt_version_id=?2 WHERE id=?3 AND project_id=?4", params![prompt.prompt_id, prompt.id, audit_id, project_id]).map_err(|e| e.to_string())?;
    Ok(prompt)
}

pub fn link_remediation_session(
    database: &DatabaseState,
    project_id: &str,
    audit_id: &str,
    session_id: &str,
) -> Result<(), String> {
    let _ = get(database, project_id, audit_id)?;
    let connection = database.open_connection()?;
    let provenance: Option<(Option<String>, Option<String>, String, Option<String>, String, Option<String>, Option<i64>, Option<String>, Option<String>, Option<String>, Option<i64>, Option<String>, Option<String>)> = connection
        .query_row(
            "SELECT a.remediation_prompt_id,a.remediation_prompt_version_id,v.content,v.approved_body_sha256,v.dispatch_state,v.dispatched_session_id,v.version,v.dispatch_provenance_json,s.prompt_id,s.prompt_version_id,s.prompt_version,s.prompt_version_sha256,s.prompt_body FROM audits a JOIN prompt_versions v ON v.id=a.remediation_prompt_version_id AND v.prompt_id=a.remediation_prompt_id JOIN agent_sessions s ON s.id=?1 AND s.project_id=?2 WHERE a.id=?3 AND a.project_id=?2",
            params![session_id, project_id, audit_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?, row.get(9)?, row.get(10)?, row.get(11)?, row.get(12)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let (
        prompt_id,
        version_id,
        content,
        approved_hash,
        dispatch_state,
        dispatched_session_id,
        prompt_version,
        dispatch_provenance,
        session_prompt_id,
        session_version_id,
        session_version,
        session_hash,
        session_body,
    ) = provenance.ok_or_else(|| "AUDIT_REMEDIATION_PROVENANCE_MISMATCH".to_string())?;
    let prompt_id =
        prompt_id.ok_or_else(|| "AUDIT_REMEDIATION_PROMPT_NOT_ASSOCIATED".to_string())?;
    let version_id =
        version_id.ok_or_else(|| "AUDIT_REMEDIATION_PROMPT_NOT_ASSOCIATED".to_string())?;
    let approved_hash =
        approved_hash.ok_or_else(|| "AUDIT_REMEDIATION_PROMPT_NOT_APPROVED".to_string())?;
    if prompt_id.trim().is_empty() || version_id.trim().is_empty() {
        return Err("AUDIT_REMEDIATION_PROMPT_NOT_ASSOCIATED".into());
    }
    if dispatch_state != "DISPATCHED" || dispatched_session_id.as_deref() != Some(session_id) {
        return Err("AUDIT_REMEDIATION_PROMPT_NOT_DISPATCHED_TO_SESSION".into());
    }
    if session_prompt_id.as_deref() != Some(prompt_id.as_str())
        || session_version_id.as_deref() != Some(version_id.as_str())
        || session_version != prompt_version
    {
        return Err("AUDIT_SESSION_PROMPT_VERSION_MISMATCH".into());
    }
    if approved_hash != hash_bytes(content.as_bytes())
        || session_hash.as_deref() != Some(approved_hash.as_str())
        || session_body
            .as_deref()
            .map(|body| hash_bytes(body.as_bytes()))
            != Some(approved_hash.clone())
    {
        return Err("AUDIT_SESSION_PROMPT_PROVENANCE_HASH_MISMATCH".into());
    }
    let dispatch_provenance: serde_json::Value =
        serde_json::from_str(dispatch_provenance.as_deref().unwrap_or("{}"))
            .map_err(|_| "AUDIT_SESSION_DISPATCH_PROVENANCE_INVALID".to_string())?;
    if dispatch_provenance
        .get("promptId")
        .and_then(|value| value.as_str())
        != Some(prompt_id.as_str())
        || dispatch_provenance
            .get("promptVersionId")
            .and_then(|value| value.as_str())
            != Some(version_id.as_str())
        || dispatch_provenance
            .get("promptVersionSha256")
            .and_then(|value| value.as_str())
            != Some(approved_hash.as_str())
        || dispatch_provenance
            .get("sessionId")
            .and_then(|value| value.as_str())
            != Some(session_id)
    {
        return Err("AUDIT_SESSION_DISPATCH_PROVENANCE_MISMATCH".into());
    }
    connection
        .execute(
            "UPDATE audits SET remediation_session_id=?1 WHERE id=?2 AND project_id=?3",
            params![session_id, audit_id, project_id],
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn current_repository_is_fresh(
    database: &DatabaseState,
    audit: &AuditRun,
) -> Result<bool, String> {
    let target = AuditGitTarget {
        scope: audit.git_scope,
        base_ref: audit.baseline_ref.clone(),
        head_sha: audit.audited_head_sha.clone(),
        target_origin: audit.target_origin,
        audited_session_id: audit.audited_session_id.clone(),
        audited_prompt_version_id: audit.audited_prompt_version_id.clone(),
    };
    Ok(current_freshness_token(database, &audit.project_id, &target)? == audit.freshness_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projects::{register_project, RegisterProjectRequest};
    use std::fs;
    use std::process::Command;
    use tempfile::tempdir;

    fn input() -> AuditInput {
        AuditInput {
            project_id: "p".into(),
            task_id: None,
            task: None,
            requirements: Vec::new(),
            git: AuditGitEvidence {
                scope: GitDiffScope::WorkingTree,
                target_origin: AuditTargetOrigin::Auto,
                branch: Some("main".into()),
                head_sha: Some("abc".into()),
                baseline_ref: None,
                base_sha: None,
                staged_files: vec![],
                unstaged_files: vec![],
                untracked_files: vec![],
                conflicted_files: vec![],
                diff: None,
                diff_truncated: false,
                repository_identity: None,
                changed_files: vec![],
                full_change_set_sha256: None,
                staged_content_sha256: None,
                working_tracked_content_sha256: None,
                untracked_content_sha256: None,
                conflict_identity: None,
                committed_range_identity: None,
                identity_complete: true,
                identity_diagnostic: None,
            },
            evidence: vec![],
            audited_branch: Some("main".into()),
            audited_head_sha: Some("abc".into()),
            baseline_ref: None,
            collected_at: "now".into(),
            input_manifest_sha256: "hash".into(),
            audited_session_id: None,
            audited_prompt_version_id: None,
            freshness_token: freshness_token(&AuditGitEvidence {
                scope: GitDiffScope::WorkingTree,
                target_origin: AuditTargetOrigin::Auto,
                branch: Some("main".into()),
                head_sha: Some("abc".into()),
                baseline_ref: None,
                base_sha: None,
                staged_files: vec![],
                unstaged_files: vec![],
                untracked_files: vec![],
                conflicted_files: vec![],
                diff: None,
                diff_truncated: false,
                repository_identity: None,
                changed_files: vec![],
                full_change_set_sha256: None,
                staged_content_sha256: None,
                working_tracked_content_sha256: None,
                untracked_content_sha256: None,
                conflict_identity: None,
                committed_range_identity: None,
                identity_complete: true,
                identity_diagnostic: None,
            }),
        }
    }

    #[test]
    fn strict_model_parser_accepts_only_canonical_verdicts_and_severities() {
        let model = FixtureAuditModel { response: r#"{"verdict":"FAIL","confidence":"HIGH","regressionRisk":"CRITICAL","summary":"Known defect","findings":[{"findingKey":"missing-symbol","severity":"MAJOR","title":"Missing symbol","detail":"The required symbol is absent.","requirementRefs":["req-1"],"evidenceRefs":["SOURCE_SNIPPET:src/lib.rs"],"sourceLocator":"src/lib.rs:10","testLocator":null,"remediationGuidance":"Add the symbol and a direct test.","blocksRelease":true}],"requirementCoverage":[{"requirementRef":"req-1","requirementText":"Required symbol exists","status":"FAILED","evidenceRefs":["SOURCE_SNIPPET:src/lib.rs"],"rationale":"Source inspection disproves it."}]}"#.into() };
        let mut fixture_input = input();
        fixture_input.requirements = vec![AuditRequirement {
            requirement_ref: "req-1".into(),
            requirement_text: "Fixture requirement".into(),
            required: true,
        }];
        fixture_input.evidence.push(evidence(
            "SOURCE_SNIPPET",
            "src/lib.rs",
            VerificationStatus::Verified,
            "source",
            None,
            Some("symbol source".into()),
            false,
        ));
        let result = evaluate_fixture(&fixture_input, &model);
        assert_eq!(result.verdict, AuditVerdict::Fail);
        assert_eq!(result.findings[0].severity, FindingSeverity::Major);
        assert!(parse_model_output(
            r#"{"verdict":"MAYBE","confidence":"HIGH","regressionRisk":"LOW","summary":"x"}"#
        )
        .is_err());
    }

    #[test]
    fn malformed_model_output_is_conditional_and_never_invents_findings() {
        let model = FixtureAuditModel {
            response: "not-json".into(),
        };
        let result = evaluate_fixture(&input(), &model);
        assert_eq!(result.verdict, AuditVerdict::Conditional);
        assert_eq!(result.model_status, "MALFORMED");
        assert!(result.findings.is_empty());
    }

    #[test]
    fn unavailable_model_is_truthful_and_does_not_pass() {
        let result = evaluate_with(&UnavailableAuditModel, &input());
        assert_eq!(result.verdict, AuditVerdict::Conditional);
        assert_eq!(result.model_status, "UNAVAILABLE");
        assert_eq!(result.confidence, ConfidenceLevel::Low);
    }

    #[test]
    fn evidence_status_and_utf8_bounds_are_explicit() {
        let value = "ş".repeat(5000);
        let (bounded, truncated) = bound_text(&value, 1024);
        assert!(truncated);
        assert!(bounded.is_char_boundary(bounded.len()));
        assert_eq!(
            parse_verification("CLAIM_ONLY"),
            VerificationStatus::ClaimOnly
        );
        assert_eq!(
            parse_verification("unknown"),
            VerificationStatus::Unverified
        );
    }

    #[test]
    fn source_policy_rejects_escape_and_secret_paths() {
        assert!(!is_auditable_path("../secret.ts"));
        assert!(!is_auditable_path(".env"));
        assert!(!is_auditable_path("C:/outside.ts"));
        assert!(is_auditable_path("src-tauri/src/lib.rs"));
    }

    #[test]
    fn builder_claims_are_not_verified() {
        let item = evidence(
            "BUILDER_LOG_CLAIM",
            "log.md",
            VerificationStatus::ClaimOnly,
            "claim",
            None,
            Some("all tests pass".into()),
            false,
        );
        assert_eq!(item.verification_status, VerificationStatus::ClaimOnly);
    }

    #[test]
    fn claim_directed_test_planner_reads_exact_body_and_ignores_unrelated_mock() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("proof.test.ts");
        fs::write(
            &path,
            "const unrelated = vi.mock('unrelated');\ntest(\"proof\", () => {\n  expect(realBoundary()).toEqual(true);\n});\n",
        )
        .unwrap();
        let (body, start, end) =
            locate_test_body(directory.path().to_str().unwrap(), "proof.test.ts", "proof")
                .body
                .unwrap();
        assert_eq!(start, 2);
        assert!(end >= start);
        assert_eq!(classify_test_body(&body), VerificationStatus::Corroborated);
        assert!(body.contains("realBoundary"));
    }

    #[test]
    fn claim_directed_test_planner_downgrades_mocked_empty_and_skipped_targets() {
        assert_eq!(
            classify_test_body("test(\"mocked\", () => { vi.mock('real'); expect(true); });"),
            VerificationStatus::Partial
        );
        assert_eq!(
            classify_test_body("test(\"empty\", () => { helper(); });"),
            VerificationStatus::Unverified
        );
        assert_eq!(
            classify_test_body("test.skip(\"skipped\", () => { expect(true); });"),
            VerificationStatus::Stale
        );
    }

    #[test]
    fn test_and_builder_readers_stop_at_explicit_budgets() {
        let directory = tempdir().unwrap();
        let test_body = format!(
            "{}\ntest(\"late\", () => {{ expect(true); }});\n",
            "x".repeat(MAX_TEST_SCAN_BYTES + 1024)
        );
        fs::write(directory.path().join("large.test.ts"), test_body).unwrap();
        let located = locate_test_body(directory.path().to_str().unwrap(), "large.test.ts", "late");
        assert!(located.body.is_none());
        assert!(located.truncated);

        let log_root = directory.path().join("docs/H!veAI/codex-logs");
        fs::create_dir_all(&log_root).unwrap();
        fs::write(
            log_root.join("M16D_large.md"),
            "claim ".repeat(MAX_BUILDER_LOG_CLAIM_BYTES + 1024),
        )
        .unwrap();
        let claims = read_builder_claims(
            directory.path().to_str().unwrap(),
            &AuditInputRequest {
                project_id: "project".into(),
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget::default(),
            },
        );
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].verification_status, VerificationStatus::Truncated);
        assert!(claims[0]
            .content
            .as_deref()
            .is_some_and(|content| content.len() <= MAX_BUILDER_LOG_CLAIM_BYTES));
    }

    #[test]
    fn builder_log_selection_is_relevant_and_canonical() {
        let directory = tempdir().unwrap();
        let log_root = directory.path().join("docs/H!veAI/codex-logs");
        fs::create_dir_all(&log_root).unwrap();
        fs::write(log_root.join("A_old_project.md"), "old project claim").unwrap();
        fs::write(
            log_root.join("M16E_UNIFIED_CONTROL_PLANE_IMPLEMENTATION_LOG.md"),
            "relevant current claim",
        )
        .unwrap();
        let request = AuditInputRequest {
            project_id: "project".into(),
            task_id: None,
            prior_audit_id: None,
            git_target: AuditGitTarget::default(),
        };
        let claims = read_builder_claims(directory.path().to_str().unwrap(), &request);
        assert_eq!(claims[0].verification_status, VerificationStatus::ClaimOnly);
        assert!(claims[0]
            .locator
            .as_deref()
            .unwrap()
            .contains("M16E_UNIFIED"));
    }

    #[test]
    fn builder_log_root_and_child_link_escapes_are_excluded_without_reading() {
        use std::os::windows::fs::{symlink_dir, symlink_file};
        let directory = tempdir().unwrap();
        let outside = tempdir().unwrap();
        let root_log = directory.path().join("docs/H!veAI/codex-logs");
        fs::create_dir_all(&root_log).unwrap();
        fs::write(outside.path().join("outside.md"), "external secret claim").unwrap();
        let child_link = root_log.join("child-link.md");
        if symlink_file(outside.path().join("outside.md"), &child_link).is_ok() {
            let request = AuditInputRequest {
                project_id: "project".into(),
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget::default(),
            };
            let claims = read_builder_claims(directory.path().to_str().unwrap(), &request);
            assert!(claims.iter().any(|item| {
                item.verification_status == VerificationStatus::Excluded && item.content.is_none()
            }));
            fs::remove_file(&child_link).unwrap();
        }

        let root_link_parent = tempdir().unwrap();
        let linked_log = root_link_parent.path().join("linked-logs");
        fs::create_dir_all(&linked_log).unwrap();
        fs::remove_dir(&root_log).unwrap();
        if symlink_dir(&linked_log, &root_log).is_ok() {
            let request = AuditInputRequest {
                project_id: "project".into(),
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget::default(),
            };
            let claims = read_builder_claims(directory.path().to_str().unwrap(), &request);
            assert!(claims.iter().any(|item| {
                item.verification_status == VerificationStatus::Excluded && item.content.is_none()
            }));
        }
    }

    #[test]
    fn audit_target_rejects_invalid_scope_refs_and_unknown_tasks() {
        let (_app, _project_dir, database, project_id) =
            git_project_fixture("Target validation fixture");
        let invalid_scope = AuditInputRequest {
            project_id: project_id.clone(),
            task_id: None,
            prior_audit_id: None,
            git_target: AuditGitTarget {
                scope: GitDiffScope::WorkingTree,
                base_ref: Some("main".into()),
                ..AuditGitTarget::default()
            },
        };
        assert_eq!(
            collect_input(&database, invalid_scope).unwrap_err(),
            "AUDIT_GIT_TARGET_REFS_NOT_ALLOWED_FOR_SCOPE"
        );
        let invalid_range = AuditInputRequest {
            project_id: project_id.clone(),
            task_id: None,
            prior_audit_id: None,
            git_target: AuditGitTarget {
                scope: GitDiffScope::CommitRange,
                ..AuditGitTarget::default()
            },
        };
        assert_eq!(
            collect_input(&database, invalid_range).unwrap_err(),
            "AUDIT_GIT_TARGET_BASE_REQUIRED"
        );
        let unknown_task = AuditInputRequest {
            project_id,
            task_id: Some("does-not-exist".into()),
            prior_audit_id: None,
            git_target: AuditGitTarget::default(),
        };
        assert_eq!(
            collect_input(&database, unknown_task).unwrap_err(),
            "AUDIT_TASK_NOT_FOUND_OR_PROJECT_MISMATCH"
        );
    }

    #[test]
    fn source_planner_uses_only_the_selected_git_scope() {
        let (_app, project_dir, database, project_id) =
            git_project_fixture("Scope planner fixture");
        fs::write(project_dir.path().join("staged.txt"), "staged").unwrap();
        assert!(Command::new("git")
            .args(["add", "staged.txt"])
            .current_dir(project_dir.path())
            .output()
            .unwrap()
            .status
            .success());
        fs::write(project_dir.path().join("tracked.txt"), "working").unwrap();
        fs::write(project_dir.path().join("untracked.txt"), "untracked").unwrap();
        let snapshot = git_engine::snapshot(
            &database,
            git_engine::GitSnapshotRequest {
                project_id: project_id.clone(),
                persist: Some(false),
            },
        )
        .unwrap();
        let working_diff = git_engine::diff(
            &database,
            git_engine::GitDiffRequest {
                project_id: project_id.clone(),
                scope: GitDiffScope::WorkingTree,
                base_ref: None,
                head_sha: None,
            },
        )
        .unwrap();
        let working = changed_paths(
            Some(&snapshot),
            Some(&working_diff),
            GitDiffScope::WorkingTree,
        );
        assert!(working.contains(&"tracked.txt".into()));
        assert!(working.contains(&"untracked.txt".into()));
        assert!(!working.contains(&"staged.txt".into()));

        let staged_diff = git_engine::diff(
            &database,
            git_engine::GitDiffRequest {
                project_id,
                scope: GitDiffScope::Staged,
                base_ref: None,
                head_sha: None,
            },
        )
        .unwrap();
        let staged = changed_paths(Some(&snapshot), Some(&staged_diff), GitDiffScope::Staged);
        assert_eq!(staged, vec!["staged.txt"]);
    }

    #[test]
    fn strict_schema_rejects_unknown_fields() {
        let result = parse_model_output(
            r#"{"verdict":"PASS","confidence":"HIGH","regressionRisk":"LOW","summary":"ok","unexpected":true}"#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn semantic_validator_rejects_false_passes_and_requires_superseded_replacement() {
        let mut fixture_input = input();
        fixture_input.requirements = vec![
            AuditRequirement {
                requirement_ref: "req-failed".into(),
                requirement_text: "The requirement must hold".into(),
                required: true,
            },
            AuditRequirement {
                requirement_ref: "req-claim".into(),
                requirement_text: "The claim requirement must hold".into(),
                required: true,
            },
        ];
        fixture_input.evidence.push(evidence(
            "TEST_RUN",
            "proof",
            VerificationStatus::Verified,
            "proof",
            None,
            Some("direct proof".into()),
            false,
        ));
        let mut major = base_evaluation();
        major.findings = vec![finding_for_evidence("TEST_RUN:proof")];
        validate_semantic_evaluation(&fixture_input, &mut major).unwrap();
        assert_eq!(major.verdict, AuditVerdict::Fail);

        let mut failed_coverage = base_evaluation();
        failed_coverage.coverage = vec![RequirementCoverage {
            id: "coverage-failed".into(),
            logical_coverage_id: "req-failed".into(),
            requirement_ref: "req-failed".into(),
            requirement_text: "The requirement must hold".into(),
            status: CoverageStatus::Failed,
            evidence_refs: vec!["TEST_RUN:proof".into()],
            rationale: "Fixture failure".into(),
        }];
        validate_semantic_evaluation(&fixture_input, &mut failed_coverage).unwrap();
        assert_eq!(failed_coverage.verdict, AuditVerdict::Fail);

        let mut claimed = base_evaluation();
        claimed.coverage = vec![RequirementCoverage {
            id: "coverage-claim".into(),
            logical_coverage_id: "req-claim".into(),
            requirement_ref: "req-claim".into(),
            requirement_text: "The requirement must hold".into(),
            status: CoverageStatus::Verified,
            evidence_refs: vec!["TEST_RUN:proof".into()],
            rationale: "Claim-only fixture".into(),
        }];
        fixture_input.evidence[0].verification_status = VerificationStatus::ClaimOnly;
        validate_semantic_evaluation(&fixture_input, &mut claimed).unwrap();
        assert_eq!(claimed.verdict, AuditVerdict::Conditional);
        assert_eq!(claimed.coverage[0].status, CoverageStatus::Unverified);

        let mut superseded = base_evaluation();
        superseded.prior_finding_dispositions = vec![PriorFindingDisposition {
            prior_finding_key: "old".into(),
            disposition: PriorFindingDispositionKind::Superseded,
            evidence_refs: vec!["TEST_RUN:proof".into()],
            rationale: "replacement missing".into(),
            replacement_finding_key: Some("missing-replacement".into()),
        }];
        assert_eq!(
            validate_semantic_evaluation(&fixture_input, &mut superseded).unwrap_err(),
            "AUDIT_SUPERSEDED_REPLACEMENT_NOT_FOUND"
        );
    }

    #[test]
    fn semantic_validator_ignores_unrelated_claims_and_supports_project_not_applicable() {
        let mut fixture_input = input();
        fixture_input.requirements = vec![AuditRequirement {
            requirement_ref: "req-verified".into(),
            requirement_text: "The implementation is verified".into(),
            required: true,
        }];
        fixture_input.evidence = vec![
            evidence(
                "SOURCE_SNIPPET",
                "required",
                VerificationStatus::Verified,
                "required source",
                None,
                Some("verified".into()),
                false,
            ),
            evidence(
                "BUILDER_LOG_CLAIM",
                "unrelated",
                VerificationStatus::ClaimOnly,
                "unrelated builder claim",
                None,
                Some("all tests pass".into()),
                false,
            ),
        ];
        let mut verified = base_evaluation();
        verified.coverage = vec![RequirementCoverage {
            id: "coverage-verified".into(),
            logical_coverage_id: "req-verified".into(),
            requirement_ref: "req-verified".into(),
            requirement_text: "The implementation is verified".into(),
            status: CoverageStatus::Verified,
            evidence_refs: vec!["SOURCE_SNIPPET:required".into()],
            rationale: "Direct source evidence".into(),
        }];
        validate_semantic_evaluation(&fixture_input, &mut verified).unwrap();
        assert_eq!(verified.verdict, AuditVerdict::Pass);

        let mut project_input = input();
        project_input.evidence = vec![evidence(
            "BUILDER_LOG_CLAIM",
            "project",
            VerificationStatus::ClaimOnly,
            "project claim",
            None,
            Some("project-level claim".into()),
            false,
        )];
        let mut project = base_evaluation();
        project.coverage = vec![RequirementCoverage {
            id: "coverage-project".into(),
            logical_coverage_id: "project-audit".into(),
            requirement_ref: "project-audit".into(),
            requirement_text: "No task requirements apply".into(),
            status: CoverageStatus::NotApplicable,
            evidence_refs: Vec::new(),
            rationale: "Project audit has no task-scoped criteria".into(),
        }];
        validate_semantic_evaluation(&project_input, &mut project).unwrap();
        assert_eq!(project.verdict, AuditVerdict::Pass);
    }

    #[test]
    fn semantic_validator_requires_exactly_one_canonical_coverage_row() {
        let mut fixture_input = input();
        fixture_input.requirements = vec![
            AuditRequirement {
                requirement_ref: "req-a".into(),
                requirement_text: "A".into(),
                required: true,
            },
            AuditRequirement {
                requirement_ref: "req-b".into(),
                requirement_text: "B".into(),
                required: true,
            },
        ];
        fixture_input.evidence.push(evidence(
            "SOURCE_SNIPPET",
            "proof",
            VerificationStatus::Verified,
            "proof",
            None,
            Some("verified".into()),
            false,
        ));
        let coverage = |reference: &str| RequirementCoverage {
            id: format!("coverage-{reference}"),
            logical_coverage_id: reference.into(),
            requirement_ref: reference.into(),
            requirement_text: reference.into(),
            status: CoverageStatus::Verified,
            evidence_refs: vec!["SOURCE_SNIPPET:proof".into()],
            rationale: "verified".into(),
        };
        let mut missing = base_evaluation();
        missing.coverage = vec![coverage("req-a")];
        validate_semantic_evaluation(&fixture_input, &mut missing).unwrap();
        assert_eq!(missing.verdict, AuditVerdict::Conditional);

        let mut unknown = base_evaluation();
        unknown.coverage = vec![coverage("req-a"), coverage("req-unknown")];
        assert_eq!(
            validate_semantic_evaluation(&fixture_input, &mut unknown).unwrap_err(),
            "AUDIT_REQUIREMENT_REFERENCE_UNKNOWN"
        );

        let mut duplicate = base_evaluation();
        duplicate.coverage = vec![coverage("req-a"), coverage("req-a")];
        assert_eq!(
            validate_semantic_evaluation(&fixture_input, &mut duplicate).unwrap_err(),
            "AUDIT_REQUIREMENT_REFERENCE_DUPLICATE"
        );
    }

    #[test]
    fn explicit_reaudit_dispositions_are_required_and_validated() {
        let connection = Connection::open_in_memory().expect("in-memory database");
        connection.execute_batch("CREATE TABLE audit_findings (id TEXT, audit_id TEXT, finding_key TEXT, severity TEXT, title TEXT, detail TEXT, requirement_refs_json TEXT, evidence_refs_json TEXT, source_locator TEXT, test_locator TEXT, confidence_level TEXT, remediation_guidance TEXT, blocks_release INTEGER, status TEXT);").expect("fixture schema");
        connection.execute("INSERT INTO audit_findings VALUES ('old','prior','old-key','MAJOR','Old finding','details','[]',?1,'src/lib.rs:1',NULL,'HIGH','fix',1,'OPEN')", [r#"["OLD:EVIDENCE"]"#]).expect("prior finding");
        let tx = connection.unchecked_transaction().expect("transaction");
        let mut omitted = AuditEvaluation {
            verdict: AuditVerdict::Pass,
            confidence: ConfidenceLevel::High,
            regression_risk: RegressionRisk::Low,
            summary: "fixed".into(),
            findings: Vec::new(),
            prior_finding_dispositions: Vec::new(),
            coverage: Vec::new(),
            model_status: "AVAILABLE".into(),
            diagnostic: None,
            auditor_provider: Some("FIXTURE".into()),
            auditor_model: Some("fixture".into()),
            auditor_version: Some("1".into()),
        };
        apply_prior_finding_dispositions(&tx, "prior", "current", &input(), &mut omitted)
            .expect("omitted prior findings remain actionable");
        assert_eq!(omitted.findings.len(), 1);
        assert_eq!(omitted.findings[0].status, "OPEN");
        assert_eq!(
            omitted.findings[0].disposition.as_deref(),
            Some("STILL_OPEN")
        );
        assert!(omitted.findings[0].evidence_refs.is_empty());
        assert_eq!(
            omitted.findings[0].inherited_prior_audit_id.as_deref(),
            Some("prior")
        );
        assert_eq!(
            omitted.findings[0].inherited_prior_evidence_refs,
            vec!["OLD:EVIDENCE"]
        );
        let mut available = AuditEvaluation {
            verdict: AuditVerdict::Pass,
            confidence: ConfidenceLevel::High,
            regression_risk: RegressionRisk::Low,
            summary: "fixed".into(),
            findings: Vec::new(),
            prior_finding_dispositions: vec![PriorFindingDisposition {
                prior_finding_key: "old-key".into(),
                disposition: PriorFindingDispositionKind::Closed,
                evidence_refs: vec!["TEST_RUN:proof".into()],
                rationale: "Current evidence proves the prior defect is fixed.".into(),
                replacement_finding_key: None,
            }],
            coverage: Vec::new(),
            model_status: "AVAILABLE".into(),
            diagnostic: None,
            auditor_provider: Some("FIXTURE".into()),
            auditor_model: Some("fixture".into()),
            auditor_version: Some("1".into()),
        };
        let mut current_input = input();
        current_input.evidence.push(evidence(
            "TEST_RUN",
            "proof",
            VerificationStatus::Verified,
            "proof",
            None,
            None,
            false,
        ));
        apply_prior_finding_dispositions(&tx, "prior", "current", &current_input, &mut available)
            .expect("explicit closure");
        assert_eq!(available.findings[0].status, "CLOSED");
        assert_eq!(
            available.findings[0].closed_by_audit_id.as_deref(),
            Some("current")
        );
        assert_eq!(
            available.findings[0].closed_by_audit_id.as_deref(),
            Some("current")
        );
    }

    fn evidence_input(project_id: &str) -> AuditInput {
        let mut current = input();
        current.project_id = project_id.into();
        current.evidence = vec![evidence(
            "TEST_RUN",
            "proof",
            VerificationStatus::Verified,
            "proof",
            None,
            Some("verified proof".into()),
            false,
        )];
        current.requirements = vec![AuditRequirement {
            requirement_ref: "proof-required".into(),
            requirement_text: "Proof is persisted".into(),
            required: true,
        }];
        current
    }

    fn base_evaluation() -> AuditEvaluation {
        AuditEvaluation {
            verdict: AuditVerdict::Pass,
            confidence: ConfidenceLevel::High,
            regression_risk: RegressionRisk::Low,
            summary: "fixture".into(),
            findings: Vec::new(),
            prior_finding_dispositions: Vec::new(),
            coverage: Vec::new(),
            model_status: "AVAILABLE".into(),
            diagnostic: None,
            auditor_provider: Some("FIXTURE".into()),
            auditor_model: Some("fixture".into()),
            auditor_version: Some("1".into()),
        }
    }

    fn finding_for_evidence(reference: &str) -> AuditFinding {
        AuditFinding {
            id: "fixture-finding".into(),
            logical_finding_id: "fixture-finding".into(),
            finding_key: "fixture-finding".into(),
            severity: FindingSeverity::Major,
            title: "Fixture finding".into(),
            detail: "Fixture finding detail".into(),
            requirement_refs: Vec::new(),
            evidence_refs: vec![reference.into()],
            source_locator: Some("fixture.rs:1".into()),
            test_locator: Some("fixture.rs:1".into()),
            confidence: ConfidenceLevel::High,
            status: "OPEN".into(),
            remediation_guidance: "Fixture remediation".into(),
            blocks_release: true,
            closed_by_audit_id: None,
            prior_finding_key: None,
            disposition: None,
            disposition_evidence_refs: Vec::new(),
            disposition_rationale: None,
            superseded_by_finding_key: None,
            inherited_prior_audit_id: None,
            inherited_prior_evidence_refs: Vec::new(),
        }
    }

    #[test]
    fn persisted_evidence_has_global_and_audit_scoped_logical_identity() {
        let (_app, _project_dir, database, project_id) =
            git_project_fixture("Evidence identity fixture");
        let first_input = evidence_input(&project_id);
        let mut first_evaluation = base_evaluation();
        first_evaluation.verdict = AuditVerdict::Fail;
        first_evaluation.findings = vec![finding_for_evidence("TEST_RUN:proof")];
        first_evaluation.coverage = vec![RequirementCoverage {
            id: "coverage-fixture".into(),
            logical_coverage_id: "proof-required".into(),
            requirement_ref: "proof-required".into(),
            requirement_text: "Proof is persisted".into(),
            status: CoverageStatus::Verified,
            evidence_refs: vec!["TEST_RUN:proof".into()],
            rationale: "Fixture evidence resolves".into(),
        }];
        let first = persist_run(
            &database,
            &first_input,
            None,
            "first",
            first_evaluation,
            AuditState::Completed,
        )
        .unwrap();

        let mut second_evaluation = base_evaluation();
        second_evaluation.prior_finding_dispositions = vec![PriorFindingDisposition {
            prior_finding_key: "fixture-finding".into(),
            disposition: PriorFindingDispositionKind::Closed,
            evidence_refs: vec!["TEST_RUN:proof".into()],
            rationale: "The current proof closes the fixture finding.".into(),
            replacement_finding_key: None,
        }];
        let second = persist_run(
            &database,
            &first_input,
            Some(first.id.clone()),
            "second",
            second_evaluation,
            AuditState::Completed,
        )
        .unwrap();

        assert_ne!(first.evidence[0].id, second.evidence[0].id);
        assert_eq!(first.evidence[0].logical_evidence_id, "TEST_RUN:proof");
        assert_eq!(second.evidence[0].logical_evidence_id, "TEST_RUN:proof");
        assert_eq!(first.findings[0].evidence_refs, vec!["TEST_RUN:proof"]);
        assert_eq!(first.coverage[0].evidence_refs, vec!["TEST_RUN:proof"]);
        assert_eq!(
            second.findings[0].disposition_evidence_refs,
            vec!["TEST_RUN:proof"]
        );

        let connection = database.open_connection().unwrap();
        assert_eq!(
            resolve_evidence_row_id(&connection, &first.id, "TEST_RUN:proof").unwrap(),
            first.evidence[0].id
        );
        assert_eq!(
            resolve_evidence_row_id(&connection, &second.id, "TEST_RUN:proof").unwrap(),
            second.evidence[0].id
        );
        assert!(resolve_evidence_row_id(&connection, "missing-audit", "TEST_RUN:proof").is_err());

        let mut third_evaluation = base_evaluation();
        third_evaluation.findings = vec![finding_for_evidence("TEST_RUN:proof")];
        third_evaluation.coverage = vec![RequirementCoverage {
            id: "coverage-third".into(),
            logical_coverage_id: "proof-required".into(),
            requirement_ref: "proof-required".into(),
            requirement_text: "Proof is reused in another audit".into(),
            status: CoverageStatus::Verified,
            evidence_refs: vec!["TEST_RUN:proof".into()],
            rationale: "Same logical reference is audit-scoped".into(),
        }];
        let third = persist_run(
            &database,
            &first_input,
            None,
            "third",
            third_evaluation,
            AuditState::Completed,
        )
        .unwrap();
        assert_ne!(first.findings[0].id, third.findings[0].id);
        assert_ne!(first.coverage[0].id, third.coverage[0].id);
    }

    #[test]
    fn persisted_evidence_rejects_dangling_and_duplicate_logical_references() {
        let (_app, _project_dir, database, project_id) =
            git_project_fixture("Evidence validation fixture");
        let current = evidence_input(&project_id);

        let mut finding_evaluation = base_evaluation();
        finding_evaluation.findings = vec![finding_for_evidence("TEST_RUN:missing")];
        assert_eq!(
            persist_run(
                &database,
                &current,
                None,
                "finding",
                finding_evaluation,
                AuditState::Completed
            )
            .unwrap_err(),
            "AUDIT_EVIDENCE_REFERENCE_NOT_FOUND"
        );

        let mut coverage_evaluation = base_evaluation();
        coverage_evaluation.coverage = vec![RequirementCoverage {
            id: "coverage-invalid".into(),
            logical_coverage_id: "invalid".into(),
            requirement_ref: "invalid".into(),
            requirement_text: "Invalid".into(),
            status: CoverageStatus::Failed,
            evidence_refs: vec!["TEST_RUN:missing".into()],
            rationale: "Invalid".into(),
        }];
        assert_eq!(
            persist_run(
                &database,
                &current,
                None,
                "coverage",
                coverage_evaluation,
                AuditState::Completed
            )
            .unwrap_err(),
            "AUDIT_EVIDENCE_REFERENCE_NOT_FOUND"
        );

        let mut disposition_evaluation = base_evaluation();
        disposition_evaluation.prior_finding_dispositions = vec![PriorFindingDisposition {
            prior_finding_key: "missing-prior".into(),
            disposition: PriorFindingDispositionKind::StillOpen,
            evidence_refs: vec!["TEST_RUN:missing".into()],
            rationale: "Invalid".into(),
            replacement_finding_key: None,
        }];
        assert_eq!(
            persist_run(
                &database,
                &current,
                None,
                "disposition",
                disposition_evaluation,
                AuditState::Completed
            )
            .unwrap_err(),
            "AUDIT_EVIDENCE_REFERENCE_NOT_FOUND"
        );

        let mut duplicate_input = current.clone();
        duplicate_input.evidence.push(evidence(
            "TEST_RUN",
            "proof",
            VerificationStatus::Verified,
            "duplicate",
            None,
            None,
            false,
        ));
        assert_eq!(
            persist_run(
                &database,
                &duplicate_input,
                None,
                "duplicate",
                base_evaluation(),
                AuditState::Completed,
            )
            .unwrap_err(),
            "AUDIT_EVIDENCE_REFERENCE_DUPLICATE"
        );

        let connection = database.open_connection().unwrap();
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM audits", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0, "invalid persistence must not commit an audit row");
    }

    fn git_project_fixture(
        name: &str,
    ) -> (tempfile::TempDir, tempfile::TempDir, DatabaseState, String) {
        let app_data = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(project_dir.path().join("tracked.txt"), "before").unwrap();
        assert!(Command::new("git")
            .args(["init"])
            .current_dir(project_dir.path())
            .output()
            .unwrap()
            .status
            .success());
        assert!(Command::new("git")
            .args(["add", "."])
            .current_dir(project_dir.path())
            .output()
            .unwrap()
            .status
            .success());
        assert!(Command::new("git")
            .args([
                "-c",
                "user.email=test@example.com",
                "-c",
                "user.name=Test",
                "commit",
                "-m",
                "fixture",
            ])
            .current_dir(project_dir.path())
            .output()
            .unwrap()
            .status
            .success());
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some(name.into()),
            },
        )
        .unwrap();
        (app_data, project_dir, database, project.id)
    }

    fn prior_open_finding_fixture() -> (
        tempfile::TempDir,
        tempfile::TempDir,
        DatabaseState,
        String,
        String,
    ) {
        let (app_data, project_dir, database, project_id) =
            git_project_fixture("Degraded audit fixture");
        let prior = run_with_model(
            &database,
            AuditInputRequest {
                project_id: project_id.clone(),
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget::default(),
            },
            &FixtureAuditModel {
                response: r#"{"verdict":"FAIL","confidence":"HIGH","regressionRisk":"HIGH","summary":"prior audit","findings":[{"findingKey":"prior-open","severity":"MAJOR","title":"Prior open finding","detail":"Prior evidence requires remediation.","requirementRefs":[],"evidenceRefs":[],"sourceLocator":"tracked.txt:1","testLocator":null,"remediationGuidance":"Address the defect.","blocksRelease":true}]}"#.into(),
            },
        )
        .unwrap();
        (app_data, project_dir, database, project_id, prior.id)
    }

    #[test]
    fn unavailable_fresh_reaudit_persists_and_preserves_prior_open_finding() {
        let (_app, _project_dir, database, project_id, prior_id) = prior_open_finding_fixture();
        let result = run_with_model(
            &database,
            AuditInputRequest {
                project_id: project_id.clone(),
                task_id: None,
                prior_audit_id: Some(prior_id.clone()),
                git_target: AuditGitTarget::default(),
            },
            &UnavailableAuditModel,
        )
        .expect("unavailable re-audit persists as degraded history");
        assert_eq!(result.state, AuditState::Completed);
        assert_eq!(result.model_status, "UNAVAILABLE");
        assert_eq!(result.verdict, AuditVerdict::Conditional);
        assert_eq!(result.prior_audit_id.as_deref(), Some(prior_id.as_str()));
        assert!(
            result.findings.is_empty(),
            "degraded runs do not synthesize disposition rows"
        );

        let prior = get(&database, &project_id, &prior_id).unwrap();
        assert_eq!(prior.findings.len(), 1);
        assert_eq!(prior.findings[0].finding_key, "prior-open");
        assert_eq!(prior.findings[0].status, "OPEN");
        assert_eq!(
            get(&database, &project_id, &result.id).unwrap().state,
            AuditState::Completed
        );
    }

    #[test]
    fn malformed_fresh_reaudit_persists_and_does_not_close_prior_finding() {
        let (_app, _project_dir, database, project_id, prior_id) = prior_open_finding_fixture();
        let result = run_with_model(
            &database,
            AuditInputRequest {
                project_id: project_id.clone(),
                task_id: None,
                prior_audit_id: Some(prior_id.clone()),
                git_target: AuditGitTarget::default(),
            },
            &FixtureAuditModel {
                response: "not-json".into(),
            },
        )
        .expect("malformed re-audit persists as degraded history");
        assert_eq!(result.state, AuditState::Completed);
        assert_eq!(result.model_status, "MALFORMED");
        assert_eq!(result.verdict, AuditVerdict::Conditional);
        assert_eq!(result.prior_audit_id.as_deref(), Some(prior_id.as_str()));
        assert!(
            result.findings.is_empty(),
            "malformed runs do not synthesize disposition rows"
        );
        assert_eq!(
            get(&database, &project_id, &prior_id).unwrap().findings[0].status,
            "OPEN"
        );
    }

    #[derive(Clone, Copy)]
    enum DegradedMode {
        Unavailable,
        Malformed,
    }

    struct MutatingDegradedModel {
        path: std::path::PathBuf,
        mode: DegradedMode,
    }

    impl AuditModel for MutatingDegradedModel {
        fn provider(&self) -> String {
            "FIXTURE".into()
        }
        fn model(&self) -> String {
            "mutating-degraded".into()
        }
        fn version(&self) -> String {
            "1".into()
        }
        fn evaluate(&self, _input: &AuditInput) -> Result<String, String> {
            match self.mode {
                DegradedMode::Unavailable => {
                    fs::write(&self.path, "after-unavailable").map_err(|e| e.to_string())?;
                    Err("provider unavailable".into())
                }
                DegradedMode::Malformed => {
                    fs::write(&self.path, "after-malformed").map_err(|e| e.to_string())?;
                    Ok("not-json".into())
                }
            }
        }
    }

    #[test]
    fn degraded_reaudits_keep_stale_precedence_after_repository_change() {
        let (_app, project_dir, database, project_id) =
            git_project_fixture("Stale degraded audit fixture");
        let unavailable = run_with_model(
            &database,
            AuditInputRequest {
                project_id: project_id.clone(),
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget::default(),
            },
            &MutatingDegradedModel {
                path: project_dir.path().join("tracked.txt"),
                mode: DegradedMode::Unavailable,
            },
        )
        .unwrap();
        assert_eq!(unavailable.state, AuditState::Stale);
        assert_eq!(unavailable.model_status, "UNAVAILABLE");

        let malformed = run_with_model(
            &database,
            AuditInputRequest {
                project_id,
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget::default(),
            },
            &MutatingDegradedModel {
                path: project_dir.path().join("tracked.txt"),
                mode: DegradedMode::Malformed,
            },
        )
        .unwrap();
        assert_eq!(malformed.state, AuditState::Stale);
        assert_eq!(malformed.model_status, "MALFORMED");
        assert!(malformed
            .diagnostic
            .unwrap()
            .starts_with("AUDIT_STALE_REPOSITORY_CHANGED:"));
    }

    #[test]
    fn degraded_reaudit_keeps_cross_project_prior_validation() {
        let (_app, _project_dir, database, _project_id, prior_id) = prior_open_finding_fixture();
        let (_other_app, other_dir, other_project_id) = {
            let app_data = tempdir().unwrap();
            let project_dir = tempdir().unwrap();
            fs::write(project_dir.path().join("other.txt"), "other").unwrap();
            Command::new("git")
                .args(["init"])
                .current_dir(project_dir.path())
                .output()
                .unwrap();
            Command::new("git")
                .args(["add", "."])
                .current_dir(project_dir.path())
                .output()
                .unwrap();
            Command::new("git")
                .args([
                    "-c",
                    "user.email=test@example.com",
                    "-c",
                    "user.name=Test",
                    "commit",
                    "-m",
                    "fixture",
                ])
                .current_dir(project_dir.path())
                .output()
                .unwrap();
            let project = register_project(
                &database,
                RegisterProjectRequest {
                    path: project_dir.path().to_string_lossy().into(),
                    name: Some("Other audit fixture".into()),
                },
            )
            .unwrap();
            (app_data, project_dir, project.id)
        };
        let error = run_with_model(
            &database,
            AuditInputRequest {
                project_id: other_project_id,
                task_id: None,
                prior_audit_id: Some(prior_id),
                git_target: AuditGitTarget::default(),
            },
            &UnavailableAuditModel,
        )
        .unwrap_err();
        assert_eq!(error, "AUDIT_PRIOR_PROJECT_MISMATCH_OR_NOT_FOUND");
        drop(other_dir);
    }

    #[test]
    fn reaudit_target_must_match_prior_scope_and_preserves_target_origin() {
        let (_app, _project_dir, database, project_id, prior_id) = prior_open_finding_fixture();
        let error = run_with_model(
            &database,
            AuditInputRequest {
                project_id: project_id.clone(),
                task_id: None,
                prior_audit_id: Some(prior_id.clone()),
                git_target: AuditGitTarget {
                    scope: GitDiffScope::Staged,
                    target_origin: AuditTargetOrigin::Manual,
                    ..AuditGitTarget::default()
                },
            },
            &UnavailableAuditModel,
        )
        .unwrap_err();
        assert_eq!(error, "AUDIT_REAUDIT_GIT_SCOPE_MISMATCH");

        let result = run_with_model(
            &database,
            AuditInputRequest {
                project_id,
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget {
                    target_origin: AuditTargetOrigin::Manual,
                    ..AuditGitTarget::default()
                },
            },
            &UnavailableAuditModel,
        )
        .unwrap();
        assert_eq!(result.target_origin, AuditTargetOrigin::Manual);
    }

    fn link_fixture() -> (tempfile::TempDir, tempfile::TempDir, DatabaseState, String) {
        let app_data = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Audit link fixture".into()),
            },
        )
        .unwrap();
        let content = "exact approved remediation";
        let hash = hash_bytes(content.as_bytes());
        let connection = database.open_connection().unwrap();
        connection.execute("INSERT INTO prompts (id,project_id,kind,current_version,created_at,updated_at) VALUES ('audit-prompt',?1,'REMEDIATION',1,'now','now')", [&project.id]).unwrap();
        connection.execute("INSERT INTO prompt_versions (id,prompt_id,version,content,created_by,created_at,approval_state,approved_body_sha256,dispatch_state,dispatched_session_id,dispatch_provenance_json) VALUES ('audit-version','audit-prompt',1,?1,'test','now','APPROVED',?2,'DISPATCHED','audit-session',?3)", params![content, hash, json!({"promptId":"audit-prompt","promptVersionId":"audit-version","promptVersionSha256":hash,"sessionId":"audit-session"}).to_string()]).unwrap();
        connection.execute("INSERT INTO audits (id,project_id,result,summary,confidence,created_at,audit_type,audited_branch,audited_head_sha,input_manifest_sha256,schema_version,confidence_level,regression_risk,state,started_at,finished_at,model_status,remediation_prompt_id,remediation_prompt_version_id,freshness_token) VALUES ('audit-link',?1,'FAIL','fixture',0.0,'now','IMPLEMENTATION','main','head','manifest',1,'LOW','HIGH','COMPLETED','now','now','AVAILABLE','audit-prompt','audit-version','freshness')", [&project.id]).unwrap();
        connection.execute("INSERT INTO agent_sessions (id,project_id,provider,state,created_at,prompt_body,prompt_id,prompt_version_id,prompt_version,prompt_version_sha256) VALUES ('audit-session',?1,'CODEX','COMPLETED','now',?2,'audit-prompt','audit-version',1,?3)", params![project.id, content, hash]).unwrap();
        (app_data, project_dir, database, project.id)
    }

    #[test]
    fn remediation_link_requires_exact_prompt_version_provenance_and_project() {
        let (_app, _project, database, project_id) = link_fixture();
        link_remediation_session(&database, &project_id, "audit-link", "audit-session").unwrap();
        assert_eq!(
            get(&database, &project_id, "audit-link")
                .unwrap()
                .remediation_session_id
                .as_deref(),
            Some("audit-session")
        );
        let connection = database.open_connection().unwrap();
        connection.execute("UPDATE agent_sessions SET prompt_version_id='wrong-version' WHERE id='audit-session'", []).unwrap();
        assert_eq!(
            link_remediation_session(&database, &project_id, "audit-link", "audit-session")
                .unwrap_err(),
            "AUDIT_SESSION_PROMPT_VERSION_MISMATCH"
        );
        connection.execute("UPDATE agent_sessions SET prompt_version_id='audit-version' WHERE id='audit-session'", []).unwrap();
        connection.execute("UPDATE audits SET remediation_prompt_id=NULL,remediation_prompt_version_id=NULL WHERE id='audit-link'", []).unwrap();
        assert_eq!(
            link_remediation_session(&database, &project_id, "audit-link", "audit-session")
                .unwrap_err(),
            "AUDIT_REMEDIATION_PROVENANCE_MISMATCH"
        );
    }

    #[test]
    fn freshness_identity_covers_all_git_authority_dimensions() {
        let base = AuditGitEvidence {
            scope: GitDiffScope::WorkingTree,
            target_origin: AuditTargetOrigin::Auto,
            branch: Some("main".into()),
            head_sha: Some("head".into()),
            baseline_ref: Some("main".into()),
            base_sha: Some("base".into()),
            staged_files: vec!["a".into()],
            unstaged_files: vec!["b".into()],
            untracked_files: vec!["c".into()],
            conflicted_files: vec!["d".into()],
            diff: Some("diff".into()),
            diff_truncated: false,
            repository_identity: Some("owner/repo".into()),
            changed_files: vec!["changed.rs".into()],
            full_change_set_sha256: Some("diff-hash".into()),
            staged_content_sha256: Some("staged-hash".into()),
            working_tracked_content_sha256: Some("tracked-hash".into()),
            untracked_content_sha256: Some("untracked-hash".into()),
            conflict_identity: Some("conflict-hash".into()),
            committed_range_identity: Some("range-hash".into()),
            identity_complete: true,
            identity_diagnostic: None,
        };
        let fields = [
            AuditGitEvidence {
                branch: Some("other".into()),
                ..base.clone()
            },
            AuditGitEvidence {
                head_sha: Some("other".into()),
                ..base.clone()
            },
            AuditGitEvidence {
                staged_files: vec!["changed".into()],
                ..base.clone()
            },
            AuditGitEvidence {
                unstaged_files: vec!["changed".into()],
                ..base.clone()
            },
            AuditGitEvidence {
                untracked_files: vec!["changed".into()],
                ..base.clone()
            },
            AuditGitEvidence {
                conflicted_files: vec!["changed".into()],
                ..base.clone()
            },
            AuditGitEvidence {
                working_tracked_content_sha256: Some("changed".into()),
                ..base.clone()
            },
            AuditGitEvidence {
                repository_identity: Some("other/repo".into()),
                ..base.clone()
            },
        ];
        for changed in fields {
            assert_ne!(freshness_token(&base), freshness_token(&changed));
        }
        assert_eq!(freshness_token(&base), freshness_token(&base));
    }

    #[test]
    fn production_run_marks_same_head_worktree_change_stale() {
        let app_data = tempdir().unwrap();
        let project_dir = tempdir().unwrap();
        fs::write(project_dir.path().join("tracked.txt"), "before").unwrap();
        Command::new("git")
            .args(["init"])
            .current_dir(project_dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(project_dir.path())
            .output()
            .unwrap();
        Command::new("git")
            .args([
                "-c",
                "user.email=test@example.com",
                "-c",
                "user.name=Test",
                "commit",
                "-m",
                "fixture",
            ])
            .current_dir(project_dir.path())
            .output()
            .unwrap();
        let database = DatabaseState::initialize(app_data.path().to_path_buf()).unwrap();
        let project = register_project(
            &database,
            RegisterProjectRequest {
                path: project_dir.path().to_string_lossy().into(),
                name: Some("Freshness fixture".into()),
            },
        )
        .unwrap();
        struct MutatingModel {
            path: std::path::PathBuf,
        }
        impl AuditModel for MutatingModel {
            fn provider(&self) -> String {
                "FIXTURE".into()
            }
            fn model(&self) -> String {
                "mutating".into()
            }
            fn version(&self) -> String {
                "1".into()
            }
            fn evaluate(&self, _input: &AuditInput) -> Result<String, String> {
                fs::write(&self.path, "after").map_err(|e| e.to_string())?;
                Ok(r#"{"verdict":"PASS","confidence":"HIGH","regressionRisk":"LOW","summary":"ok"}"#.into())
            }
        }
        let result = run_with_model(
            &database,
            AuditInputRequest {
                project_id: project.id.clone(),
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget::default(),
            },
            &MutatingModel {
                path: project_dir.path().join("tracked.txt"),
            },
        )
        .unwrap();
        assert_eq!(result.state, AuditState::Stale);
        assert_eq!(result.model_status, "AVAILABLE");
        assert!(result
            .diagnostic
            .unwrap()
            .starts_with("AUDIT_STALE_REPOSITORY_CHANGED:"));
        assert_eq!(
            get(&database, &project.id, &result.id).unwrap().state,
            AuditState::Stale
        );
    }

    #[derive(Clone)]
    struct MockCodexProcessRunner {
        result: CodexProcessResult,
        requests: std::sync::Arc<std::sync::Mutex<Vec<CodexProcessRequest>>>,
    }

    impl CodexProcessRunner for MockCodexProcessRunner {
        fn run(&self, request: &CodexProcessRequest) -> Result<CodexProcessResult, String> {
            self.requests.lock().unwrap().push(request.clone());
            Ok(self.result.clone())
        }
    }

    fn mock_codex_result(final_message: Option<&str>) -> CodexProcessResult {
        CodexProcessResult {
            stdout: "progress".into(),
            stderr: String::new(),
            final_message: final_message.map(str::to_string),
            exit_code: Some(0),
            timed_out: false,
            stdout_truncated: false,
            stderr_truncated: false,
        }
    }

    #[test]
    fn codex_audit_process_policy_is_read_only_ephemeral_and_final_file_first() {
        let args = build_codex_audit_args(
            Path::new("C:\\Tools\\codex.exe"),
            Path::new("C:\\Temp\\schema.json"),
            Path::new("C:\\Temp\\final.json"),
            None,
            true,
        );
        assert!(args
            .windows(2)
            .any(|pair| pair == ["--sandbox", "read-only"]));
        assert!(args.contains(&"--ephemeral".into()));
        assert!(args.contains(&"--ignore-user-config".into()));
        assert!(args.contains(&"--ignore-rules".into()));
        assert!(!args.iter().any(|value| value.contains("dangerously")));
        assert!(args.contains(&"--output-schema".into()));
        assert!(args.contains(&"--output-last-message".into()));
        assert!(!args.contains(&"--model".into()));
    }

    #[test]
    fn codex_audit_uses_dedicated_final_message_and_runtime_provenance() {
        let requests = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let model = CodexCliAuditModel {
            version: "codex-cli 0.153.4".into(),
            runner: std::sync::Arc::new(MockCodexProcessRunner {
                result: mock_codex_result(Some(
                    r#"{"verdict":"CONDITIONAL","confidence":"LOW","regressionRisk":"HIGH","summary":"final","findings":[],"requirementCoverage":[],"priorFindingDispositions":[]}"#,
                )),
                requests: requests.clone(),
            }),
        };
        let output = model.evaluate(&input()).unwrap();
        assert!(output.contains("\"verdict\":\"CONDITIONAL\""));
        let request = requests.lock().unwrap().pop().unwrap();
        assert!(request.prompt.contains("authoritative evidence"));
        assert!(request.prompt.contains("Do not inspect"));
        assert!(request.schema.is_some());
        assert_eq!(model.provider(), "CODEX_CLI");
        assert_eq!(model.model(), "CLI_DEFAULT");
        assert_eq!(model.version(), "codex-cli 0.153.4");
    }

    #[test]
    fn valid_dedicated_final_remains_authoritative_when_operational_streams_are_truncated() {
        let final_message = r#"{"verdict":"CONDITIONAL","confidence":"LOW","regressionRisk":"HIGH","summary":"final survives transport bounds","findings":[],"requirementCoverage":[],"priorFindingDispositions":[]}"#;
        let model = CodexCliAuditModel {
            version: "codex-cli 0.153.4".into(),
            runner: std::sync::Arc::new(MockCodexProcessRunner {
                result: CodexProcessResult {
                    stdout: "retained prefix".into(),
                    stderr: "retained diagnostics".into(),
                    final_message: Some(final_message.into()),
                    exit_code: Some(0),
                    timed_out: false,
                    stdout_truncated: true,
                    stderr_truncated: true,
                },
                requests: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            }),
        };
        let output = model.evaluate(&input()).unwrap();
        assert!(output.contains("final survives transport bounds"));
    }

    #[test]
    fn missing_oversized_and_nonzero_final_results_fail_truthfully() {
        let missing = CodexCliAuditModel {
            version: "codex-cli 0.153.4".into(),
            runner: std::sync::Arc::new(MockCodexProcessRunner {
                result: mock_codex_result(None),
                requests: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            }),
        };
        assert!(missing
            .evaluate(&input())
            .unwrap_err()
            .contains("FINAL_OUTPUT_MISSING"));

        let nonzero = CodexCliAuditModel {
            version: "codex-cli 0.153.4".into(),
            runner: std::sync::Arc::new(MockCodexProcessRunner {
                result: CodexProcessResult {
                    exit_code: Some(7),
                    final_message: Some("valid-looking result".into()),
                    ..mock_codex_result(None)
                },
                requests: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            }),
        };
        assert!(nonzero
            .evaluate(&input())
            .unwrap_err()
            .contains("AUDIT_CODEX_PROCESS_ERROR"));

        let directory = tempdir().unwrap();
        let oversized = directory.path().join("oversized-final.json");
        fs::write(&oversized, vec![b'x'; MAX_MODEL_OUTPUT_BYTES + 1]).unwrap();
        assert!(read_bounded_final_message(&oversized)
            .unwrap_err()
            .contains("FINAL_OUTPUT_TRUNCATED"));
        let malformed = directory.path().join("malformed-final.json");
        fs::write(&malformed, [0xff, 0xfe]).unwrap();
        assert!(read_bounded_final_message(&malformed)
            .unwrap_err()
            .contains("FINAL_OUTPUT_MALFORMED"));
    }

    #[test]
    fn codex_readiness_classifies_success_failure_timeout_and_never_persists_audit() {
        for (result, expected) in [
            (mock_codex_result(Some("READY")), "READY"),
            (mock_codex_result(Some("not-ready")), "PROCESS_ERROR"),
            (
                CodexProcessResult {
                    timed_out: true,
                    ..mock_codex_result(None)
                },
                "TIMEOUT",
            ),
        ] {
            let runner = MockCodexProcessRunner {
                result,
                requests: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            };
            let readiness = check_codex_readiness_with_runner("codex-cli 0.153.4".into(), &runner);
            assert_eq!(readiness.status, expected);
        }

        let runner = MockCodexProcessRunner {
            result: CodexProcessResult {
                stdout_truncated: true,
                stderr_truncated: true,
                ..mock_codex_result(Some("READY"))
            },
            requests: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        };
        assert_eq!(
            check_codex_readiness_with_runner("codex-cli 0.153.4".into(), &runner).status,
            "READY"
        );
    }

    #[test]
    fn successful_audit_identity_is_runtime_authoritative_and_round_trips() {
        let response = r#"{"verdict":"CONDITIONAL","confidence":"LOW","regressionRisk":"HIGH","summary":"runtime identity","model":"spoofed-model","modelVersion":"spoofed-version","findings":[],"requirementCoverage":[],"priorFindingDispositions":[]}"#;
        let model = FixtureAuditModel {
            response: response.into(),
        };
        let evaluation = evaluate_fixture(&input(), &model);
        assert_eq!(evaluation.auditor_provider.as_deref(), Some("FIXTURE"));
        assert_eq!(
            evaluation.auditor_model.as_deref(),
            Some("deterministic-fixture")
        );
        assert_eq!(evaluation.auditor_version.as_deref(), Some("1"));

        let (_app, _project_dir, database, project_id) =
            git_project_fixture("Runtime identity fixture");
        let persisted = run_with_model(
            &database,
            AuditInputRequest {
                project_id: project_id.clone(),
                task_id: None,
                prior_audit_id: None,
                git_target: AuditGitTarget::default(),
            },
            &model,
        )
        .unwrap();
        let reloaded = get(&database, &project_id, &persisted.id).unwrap();
        assert_eq!(reloaded.auditor_provider.as_deref(), Some("FIXTURE"));
        assert_eq!(
            reloaded.auditor_model.as_deref(),
            Some("deterministic-fixture")
        );
        assert_eq!(reloaded.auditor_version.as_deref(), Some("1"));

        let malformed = evaluate_fixture(
            &input(),
            &FixtureAuditModel {
                response: "not-json".into(),
            },
        );
        assert_eq!(malformed.model_status, "MALFORMED");
        assert_eq!(malformed.auditor_provider.as_deref(), Some("FIXTURE"));
        assert_eq!(
            malformed.auditor_model.as_deref(),
            Some("deterministic-fixture")
        );
        assert_eq!(malformed.auditor_version.as_deref(), Some("1"));
    }
}
