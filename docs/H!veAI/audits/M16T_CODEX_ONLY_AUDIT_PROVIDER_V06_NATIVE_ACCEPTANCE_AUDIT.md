# M16T Codex-Only Audit Provider V06 Native Acceptance Strict Audit

## 1. VERDICT

**FAIL / CHANGES_REQUIRED**

The V05 FormuLab canonical-main tracking remediation remains accepted. The owner-native M16T Codex audit acceptance, however, exposed two direct production defects and one model-contract integration gap that block M16 closure.

Native evidence shows all of the following in the stable desktop application:

1. Command Center has exactly eight GitHub-tracked projects and FormuLab resolves healthy canonical truth from `Sekiph82/FormuLab@main`.
2. FormuLab Project Cockpit shows the correct `main` remote authority, current task, next action, and preserved local workspace path.
3. Settings renders the Codex Audit Provider as `CLI Unavailable / Version Unavailable / Login Unknown / CODEX_NOT_FOUND` and the explicit readiness check fails with `Command hiveai_audit_provider_check_readiness not allowed by ACL`.
4. Despite the Settings false-negative, Audit Center successfully launches the local Codex CLI and persists a run whose runtime identity is `CODEX_CLI / CLI_DEFAULT`.
5. That run has `modelStatus = MALFORMED` because the structured result fails semantic validation, but the persisted audit state is `COMPLETED` and the UI displays the success notice `Audit completed with structured evidence.`

Therefore the application currently contradicts itself about provider readiness and misrepresents a semantically rejected model result as a completed audit. M16 remains OPEN and M17 remains blocked.

## 2. CONTRACT RECOVERY

The accepted M16T architecture requires:

- the locally installed Codex CLI as the only production model-backed audit provider;
- Codex-managed ChatGPT authentication rather than API-key authentication;
- truthful readiness reporting;
- evidence-first structured audit output;
- no invented PASS when provider/model evidence is unavailable or invalid;
- bounded, schema-constrained, semantic validation before authoritative persistence;
- truthful native UI states;
- owner-native acceptance before M16 closes;
- no `OPENAI_API_KEY`, direct OpenAI HTTP audit transport, or API-key fallback;
- M17/Claude remains inactive until M16 is accepted.

V05 additionally established that the eight-project portfolio and FormuLab `main` authority must remain intact.

## 3. BRANCH / HEAD / DIFF SCOPE

Repository: `Sekiph82/H-veAI`

Branch: `main`

Accepted V05 implementation commit: `bcbc8675ef95ea25da73b3fac1902c0dfc272fb6`

V05 builder log commit: `7945c7e883122d37fb89c44b26c9fc1ef3418c4b`

Independent V05 strict-audit commit: `0825205c99e6595def0a8a21acfcc7fdfcfdf640`

Native acceptance was performed against the governed stable H!veAI publication produced by V05.

Relevant current production paths inspected:

- `src-tauri/permissions/foundation.toml`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/lib.rs`
- `src-tauri/src/audit_engine.rs`
- `src/auditEngine.ts`
- `src/AuditCenterPage.tsx`
- Settings page/provider-readiness presentation path

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Evidence |
| --- | --- | --- |
| Eight-project portfolio remains exact | PASS | Native Command Center shows 8 projects. |
| FormuLab canonical branch is `main` | PASS | Native Command Center/Cockpit show `Sekiph82/FormuLab@main`. |
| FormuLab local workspace remains attached | PASS | Native Cockpit shows `C:\Users\sekip\Desktop\FormuLab`. |
| Settings can read provider readiness | FAIL | Readiness command is absent from Tauri ACL permission set. |
| Explicit Check readiness can execute | FAIL | Native error: `hiveai_audit_provider_check_readiness not allowed by ACL`. |
| Settings readiness truth matches audit runtime truth | FAIL | Settings says `CODEX_NOT_FOUND`; Audit Center actually launches `CODEX_CLI`. |
| Codex CLI is the model-backed audit provider | PASS | Live audit records `CODEX_CLI / CLI_DEFAULT`. |
| Structured model output passes semantic validation | FAIL | Live audit records `MALFORMED`. |
| Invalid model output is never presented as a completed authoritative audit | FAIL | State/UI show `COMPLETED` and a success notice despite `MALFORMED`. |
| Diagnostic is sufficiently visible to owner | PARTIAL | Generic semantic-invalid summary is visible, but the concrete persisted diagnostic is not surfaced in the main verdict UI. |
| No OpenAI API-key/direct HTTP audit path | PASS | Accepted Codex-only architecture remains current. |
| M16 may close | FAIL | Native acceptance did not pass. |
| M17 may activate | FAIL | M16 closure gate remains unsatisfied. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

V05 builder claims about FormuLab tracking are corroborated by source and native runtime evidence.

The M16T provider runtime itself can launch Codex, which proves the Settings `CODEX_NOT_FOUND` presentation is not a trustworthy reflection of actual runtime availability in this build.

Current Tauri command registration in `src-tauri/src/lib.rs` contains both:

- `hiveai_audit_provider_readiness`
- `hiveai_audit_provider_check_readiness`

but `src-tauri/permissions/foundation.toml` `allow-audit-engine` does not allow either command. It allows audit input/run/list/get/remediation commands only. The native ACL rejection is therefore directly explained by repository truth.

## 6. FILE / SYMBOL EVIDENCE

### F-V06-001 — BLOCKER — Provider readiness commands are registered but not permitted

`src-tauri/src/lib.rs` registers `hiveai_audit_provider_readiness` and `hiveai_audit_provider_check_readiness` in the Tauri invoke handler.

`src/auditEngine.ts` invokes those exact commands from `getAuditProviderReadiness()` and `checkAuditProviderReadiness()`.

`src-tauri/permissions/foundation.toml`, permission `allow-audit-engine`, omits both commands.

The native Settings error reproduces the ACL denial directly.

### F-V06-002 — BLOCKER — `MALFORMED` model result is persisted/presented as `COMPLETED`

`src-tauri/src/audit_engine.rs::evaluate_with` converts semantic-validation failure into a degraded evaluation with:

- verdict `CONDITIONAL`;
- confidence `LOW`;
- regression risk `HIGH`;
- `model_status = MALFORMED`;
- diagnostic containing the semantic validation error.

The audit run state selection, however, uses `COMPLETED` whenever freshness has not changed. It does not require `model_status == AVAILABLE`.

`src/AuditCenterPage.tsx::startAudit` shows `Audit completed with structured evidence.` for every model status except exactly `UNAVAILABLE`. Thus `MALFORMED` is presented with a success notice.

The native screenshot reproduces this exact state: `CONDITIONAL + COMPLETED`, summary `Structured audit model output was semantically invalid`, and model `CODEX_CLI CLI_DEFAULT`.

### F-V06-003 — MAJOR — Structural JSON schema and semantic project-audit contract are not aligned strongly enough

For a project/freeform audit, `AuditInput.requirements` is empty and `TASK_REQUIREMENTS: project-audit` is recorded as unavailable by design.

The semantic validator nevertheless requires the model evaluation to contain exactly one project-level coverage row when no canonical requirements exist:

- `requirementRef = project-audit`;
- `status = NOT_APPLICABLE`.

It also rejects finding `requirementRefs` that are not members of the canonical requirements set, which is empty for a freeform project audit.

The output JSON schema only constrains `requirementCoverage` as an array of structurally valid coverage objects; it cannot by itself enforce the special project/freeform semantic rule. Existing Codex transport tests include schema-valid fixture output with `requirementCoverage: []`, demonstrating that transport/schema success alone does not prove semantic acceptance for the freeform input shape.

The owner-native ScrubBots freeform audit produced exactly this class of semantic rejection. The concrete diagnostic must be surfaced and the Codex prompt/host contract must deterministically guide the model into the accepted project-audit shape.

## 7. FOCUSED TEST EVIDENCE

Existing tests cover substantial audit parsing, persistence, Codex transport, bounded final output, and semantic validation behavior.

Missing direct acceptance coverage exposed by native testing includes:

- Tauri ACL coverage for the two new readiness commands;
- an end-to-end freeform/project audit fixture through Codex-model parsing plus semantic validation;
- explicit state semantics asserting `MALFORMED` cannot produce `AuditState::Completed`;
- frontend notice/verdict rendering asserting `MALFORMED` is not shown as successful completion;
- Settings readiness invocation failure must not be translated into `CODEX_NOT_FOUND` when the command itself was denied.

## 8. REGRESSION EVIDENCE

V05 regression evidence remains accepted for:

- eight-project portfolio identity;
- FormuLab `main` migration;
- duplicate prevention;
- local workspace preservation;
- branch-aware cache safety;
- bounded Codex runtime process behavior.

V06 must preserve all of them.

## 9. SECURITY / SAFETY REVIEW

The ACL defect must be fixed narrowly by allowing only the two intended read/readiness commands under the existing `allow-audit-engine` capability. Do not broaden shell/filesystem/network permissions.

The Codex-only provider remains mandatory. Do not introduce:

- `OPENAI_API_KEY`;
- direct OpenAI HTTP audit calls;
- API-key authentication fallback;
- ChatGPT desktop GUI automation;
- unrestricted shell permissions;
- reading Codex auth/token files directly.

## 10. ARCHITECTURE CONSISTENCY

The successful live `CODEX_CLI / CLI_DEFAULT` invocation validates the high-level Codex-only architectural decision.

The defects are integration/contract defects around that architecture, not a reason to revert to an API-key provider.

Readiness must use the same bounded Codex resolver/runtime truth as the audit path, and UI state must represent semantic validity truthfully.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Current tracker truth saying owner native acceptance is pending is now stale because the owner attempted acceptance and it failed.

The next implementation package must transition current/prospective truth to M16T V06 native-acceptance remediation with Required Actor `CODEX` while implementation is active, then back to `HUMAN` only after the builder completes and a new independent strict audit passes.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

Historical V01-V05 prompts/logs/audits remain immutable.

## 12. FINAL REPOSITORY STATE

At audit time GitHub `main` contains the accepted V05 strict audit at `0825205c99e6595def0a8a21acfcc7fdfcfdf640` before this V06 audit artifact is published.

No production source is modified by this independent audit.

## 13. OPEN CROSS-MILESTONE FINDINGS

Open:

- F-V06-001 readiness ACL omission;
- F-V06-002 malformed result falsely represented as completed;
- F-V06-003 freeform/project structured-output semantic contract gap and insufficient diagnostic presentation.

Closed/preserved:

- FormuLab branch mapping defect is closed by V05 and native evidence.
- OpenAI API-key architecture is retired and must remain absent.

## 14. DEFECTS BY SEVERITY

- **BLOCKER F-V06-001:** Settings provider readiness commands are denied by ACL.
- **BLOCKER F-V06-002:** Semantically invalid model output is stored/displayed as completed audit execution.
- **MAJOR F-V06-003:** Freeform project audit structured-output contract is insufficiently aligned with semantic validation, and concrete diagnostic visibility is inadequate.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Non-blocking after the required fixes:

- expose provider executable/version/login model metadata in one compact diagnostics panel;
- include model status in audit history rows so `MALFORMED`, `FAILED`, and `AVAILABLE` are visually distinguishable;
- keep semantic contract generation centralized rather than duplicating allowed refs between prompt construction and validators.

## 16. UNVERIFIED ITEMS

The exact live semantic diagnostic string for the ScrubBots audit is not visible in the provided native screenshots. Repository semantics narrow the failure class, but the remediation must expose the concrete diagnostic rather than guessing it.

Builder-reported automated test counts remain claims until independently corroborated. A final real native Codex turn is still required after remediation.

## 17. REGRESSION RISK

**MEDIUM**

The fixes touch Tauri permissions, audit run-state semantics, structured model-contract guidance, and frontend status rendering. Scope is bounded, but audit persistence/history and native provider readiness are central M16 behavior.

## 18. AUDIT CONFIDENCE

**HIGH**

The ACL defect and false-completion behavior are both directly reproduced in native evidence and directly explained by current source. The project/freeform semantic gap is supported by the validator contract plus native `MALFORMED` evidence; only the exact diagnostic text remains unavailable.

## 19. FINAL VERDICT

**FAIL / CHANGES_REQUIRED**

V05 FormuLab tracking remains PASS. M16T overall owner-native acceptance fails until V06 closes the readiness ACL defect, truthful malformed/failure state semantics, and project/freeform semantic-output contract.

M16 remains OPEN. M17 remains blocked.

## 20. REQUIRED REMEDIATION

Create one bounded M16T V06 implementation package that:

1. adds `hiveai_audit_provider_readiness` and `hiveai_audit_provider_check_readiness` to the existing narrow `allow-audit-engine` Tauri permission;
2. proves both readiness commands are callable from the main window without broadening unrelated capabilities;
3. ensures readiness invocation/ACL errors are not mislabeled as `CODEX_NOT_FOUND`;
4. makes only semantically valid `modelStatus = AVAILABLE` runs eligible for `AuditState::Completed`;
5. persists degraded evidence safely but marks malformed/provider/process/auth failures as non-completed failure states while preserving truthful diagnostics;
6. changes Audit Center notices/badges so `MALFORMED` can never display `Audit completed with structured evidence.`;
7. visibly surfaces non-AVAILABLE model status and concrete diagnostic text in the verdict/diagnostics UI;
8. aligns Codex prompt/semantic contract for project/freeform audits so an empty canonical requirement set has the deterministic `project-audit / NOT_APPLICABLE` coverage shape and does not invent unknown requirement references;
9. adds focused backend/frontend tests for all three findings, including a full freeform/project model-output path through semantic validation;
10. preserves Codex-only provider architecture, V05 FormuLab `main`, exact eight-project portfolio, attached local workspaces, bounded process behavior, and all prior accepted M16 behavior;
11. republishes the governed native executable;
12. leaves M16 OPEN and M17 blocked until a fresh independent audit and owner-native test prove Settings readiness = READY and a real Codex audit = `AVAILABLE + COMPLETED` with semantically valid structured output.
