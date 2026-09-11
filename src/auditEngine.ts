import { invoke } from "@tauri-apps/api/core";

export type AuditProviderReadiness = {
  provider: string;
  status: "READY" | "NOT_CONFIGURED" | "AUTH_ERROR" | "RATE_LIMITED" | "NETWORK_ERROR";
  configured: boolean;
  model: string | null;
  credentialSource: string | null;
  errorCategory: string | null;
};

export type VerificationStatus = "VERIFIED" | "CORROBORATED" | "CLAIM_ONLY" | "UNVERIFIED" | "UNAVAILABLE" | "STALE" | "TRUNCATED" | "EXCLUDED" | "PARTIAL";
export type AuditVerdict = "PASS" | "CONDITIONAL" | "FAIL";
export type FindingSeverity = "BLOCKER" | "MAJOR" | "MINOR" | "NOTE";
export type CoverageStatus = "VERIFIED" | "PARTIAL" | "UNVERIFIED" | "FAILED" | "NOT_APPLICABLE";
export type ConfidenceLevel = "HIGH" | "MEDIUM" | "LOW";
export type RegressionRisk = "LOW" | "MEDIUM" | "HIGH" | "CRITICAL";
export type AuditState = "PREPARING" | "READY" | "RUNNING" | "COMPLETED" | "FAILED" | "STALE" | "CANCELLED";

export type AuditGitScope = "STAGED" | "WORKING_TREE" | "COMMIT_RANGE";
export type AuditTargetOrigin = "AUTO" | "AGENT_SESSION" | "PRIOR_AUDIT" | "REGISTERED_POLICY" | "MANUAL";
export type AuditGitTarget = { scope: AuditGitScope; baseRef?: string | null; headSha?: string | null; targetOrigin?: AuditTargetOrigin; auditedSessionId?: string | null; auditedPromptVersionId?: string | null };
export type AuditInput = {
  projectId: string;
  taskId: string | null;
  task: { taskId: string; title: string; workflowState: string; requirements: string[]; dependencies: string[]; blockers: string[]; requiredActor: string | null; milestone: string | null; sourcePath: string | null; sourceHash: string | null; identityStatus?: VerificationStatus; requirementsStatus?: VerificationStatus; requirementsProvenance?: string | null } | null;
  requirements: { requirementRef: string; requirementText: string; required: boolean }[];
  git: { scope?: AuditGitScope; targetOrigin?: AuditTargetOrigin; branch: string | null; headSha: string | null; baselineRef: string | null; baseSha?: string | null; stagedFiles: string[]; unstagedFiles: string[]; untrackedFiles: string[]; conflictedFiles: string[]; diff: string | null; diffTruncated: boolean; repositoryIdentity: string | null; changedFiles?: string[]; fullChangeSetSha256?: string | null; stagedContentSha256?: string | null; workingTrackedContentSha256?: string | null; untrackedContentSha256?: string | null; conflictIdentity?: string | null; committedRangeIdentity?: string | null; identityComplete?: boolean; identityDiagnostic?: string | null };
  evidence: AuditEvidence[];
  auditedBranch: string | null;
  auditedHeadSha: string | null;
  baselineRef: string | null;
  collectedAt: string;
  inputManifestSha256: string;
  freshnessToken: string;
};

export type AuditEvidence = { id: string; logicalEvidenceId?: string; kind: string; verificationStatus: VerificationStatus; locator: string | null; summary: string; content: string | null; contentSha256: string | null; byteCount: number; truncated: boolean };
export type RequirementCoverage = { id: string; logicalCoverageId?: string; requirementRef: string; requirementText: string; status: CoverageStatus; evidenceRefs: string[]; rationale: string };
export type AuditFinding = { id: string; logicalFindingId?: string; findingKey: string; severity: FindingSeverity; title: string; detail: string; requirementRefs: string[]; evidenceRefs: string[]; sourceLocator: string | null; testLocator: string | null; confidence: ConfidenceLevel; status: string; remediationGuidance: string; blocksRelease: boolean; closedByAuditId: string | null; priorFindingKey: string | null; disposition: string | null; dispositionEvidenceRefs: string[]; dispositionRationale: string | null; supersededByFindingKey: string | null };
export type AuditRun = { id: string; projectId: string; taskId: string | null; task?: AuditInput["task"]; auditType: string; auditedBranch: string | null; auditedHeadSha: string | null; baselineRef: string | null; gitScope?: AuditGitScope; targetOrigin?: AuditTargetOrigin; auditedBaseSha?: string | null; auditedChangeSetSha256?: string | null; auditedSessionId?: string | null; auditedPromptVersionId?: string | null; inputManifestSha256: string; freshnessToken: string; schemaVersion: number; verdict: AuditVerdict; confidence: ConfidenceLevel; regressionRisk: RegressionRisk; state: AuditState; summary: string; startedAt: string; finishedAt: string | null; auditorProvider: string | null; auditorModel: string | null; auditorVersion: string | null; modelStatus: string; diagnostic: string | null; priorAuditId: string | null; remediationPromptId: string | null; remediationPromptVersionId: string | null; remediationSessionId: string | null; findings: AuditFinding[]; coverage: RequirementCoverage[]; evidence: AuditEvidence[] };

export function collectAuditInput(projectId: string, taskId: string | null, gitTarget?: AuditGitTarget) {
  const request = gitTarget ? { projectId, taskId, priorAuditId: null, gitTarget } : { projectId, taskId, priorAuditId: null };
  return invoke<AuditInput>("hiveai_audit_input_collect", { request });
}
export function runAudit(projectId: string, taskId: string | null, priorAuditId: string | null = null, gitTarget?: AuditGitTarget) {
  const request = gitTarget ? { projectId, taskId, priorAuditId, gitTarget } : { projectId, taskId, priorAuditId };
  return invoke<AuditRun>("hiveai_audit_run", { request });
}
export function getAuditProviderReadiness() {
  return invoke<AuditProviderReadiness>("hiveai_audit_provider_readiness");
}
export function setAuditProviderModel(model: string) {
  return invoke<AuditProviderReadiness>("hiveai_audit_provider_set_model", { request: { model } });
}
export function listAudits(projectId: string) {
  return invoke<AuditRun[]>("hiveai_audits_list", { projectId });
}
export function getAudit(projectId: string, auditId: string) {
  return invoke<AuditRun>("hiveai_audit_get", { projectId, auditId });
}
export function createRemediationPrompt(projectId: string, auditId: string, findingIds: string[], title: string, summary: string) {
  return invoke<{ promptId: string; id: string; version: number; title: string | null }>("hiveai_audit_create_remediation_prompt", { projectId, auditId, findingIds, title, summary });
}
export function linkRemediationSession(projectId: string, auditId: string, sessionId: string) {
  return invoke<void>("hiveai_audit_link_remediation_session", { projectId, auditId, sessionId });
}
