# M16T Codex-Only Audit Provider V06 — Native Acceptance Remediation

## MANDATORY SYNC-FIRST / GITHUB-FIRST CONTRACT

Before reading or changing any repository file, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the working tree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never use reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or any operation that discards owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

After synchronization read, at minimum:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V06_NATIVE_ACCEPTANCE_AUDIT.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V05_AUDIT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V05_LOG.md`
- `src-tauri/permissions/foundation.toml`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/lib.rs`
- `src-tauri/src/audit_engine.rs`
- `src-tauri/src/codex_runtime.rs`
- `src/auditEngine.ts`
- `src/AuditCenterPage.tsx`
- the Settings/provider-readiness UI implementation and its tests

All Codex-facing work and the required builder log must be entirely in English.

All H!veAI repository changes made by this task must be committed and pushed before completion is claimed. At the end verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == live remote main` and a clean H!veAI worktree.

---

## WORK ITEM

- Work code: `M16T`
- Version: `V06`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V06_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

The owner-native M16T acceptance failed after V05. Preserve all accepted V05 FormuLab/eight-project behavior and fix only the provider-readiness, audit-state truth, structured semantic contract, and diagnostic/UI failures documented in the V06 strict audit.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

---

# ABSOLUTE ARCHITECTURE GUARDRAILS

The production model-backed audit provider is **local Codex CLI only**.

Do not introduce, restore, request, read, set, persist, or use:

- `OPENAI_API_KEY`;
- direct OpenAI HTTP audit transport;
- OpenAI Responses API audit calls;
- API-key authentication or API-key fallback;
- ChatGPT desktop GUI automation;
- direct reading of Codex auth/token files;
- Claude/M17 implementation;
- broad shell/filesystem permissions.

Keep Codex-managed ChatGPT login as the only accepted authentication boundary for the model-backed audit provider.

Preserve the accepted bounded/ephemeral/read-only Codex runtime and `CLI_DEFAULT` model selection unless a directly reproducible defect requires a narrow change.

---

# FINDING V06-F01 — BLOCKER — READINESS COMMANDS ARE DENIED BY TAURI ACL

## Current defect

The native Settings screen reproduces:

`Command hiveai_audit_provider_check_readiness not allowed by ACL`

Current repository truth:

- `src-tauri/src/lib.rs` registers both `hiveai_audit_provider_readiness` and `hiveai_audit_provider_check_readiness`;
- `src/auditEngine.ts` invokes those exact commands;
- `src-tauri/permissions/foundation.toml` permission `allow-audit-engine` omits both commands.

The same native build can run an audit that records `CODEX_CLI / CLI_DEFAULT`, proving the Settings `CODEX_NOT_FOUND` presentation is a false negative caused by readiness-path integration, not proof that Codex cannot execute.

## Required fix

Add exactly these commands to the existing bounded `allow-audit-engine` permission:

- `hiveai_audit_provider_readiness`
- `hiveai_audit_provider_check_readiness`

Do not create a broad catch-all permission and do not widen unrelated capabilities.

Also harden the frontend readiness presentation so an invoke/ACL failure is shown as an invocation/readiness error rather than silently translated to `CODEX_NOT_FOUND`.

## Required tests

Add direct tests/contract checks proving:

1. both commands remain registered in the invoke handler;
2. both commands are included in the intended main-window ACL permission;
3. no unrelated shell/filesystem permission is added;
4. a frontend invoke rejection is not rendered as `CODEX_NOT_FOUND` unless the native readiness result itself is actually `CODEX_NOT_FOUND`;
5. successful readiness metadata can round-trip to the Settings UI.

---

# FINDING V06-F02 — BLOCKER — `MALFORMED` RESULT IS FALSELY `COMPLETED`

## Current defect

A live owner audit persisted and rendered:

- verdict `CONDITIONAL`;
- state `COMPLETED`;
- model `CODEX_CLI CLI_DEFAULT`;
- semantic summary `Structured audit model output was semantically invalid; no authoritative verdict was invented.`

The frontend simultaneously showed the success notice:

`Audit completed with structured evidence.`

Repository truth explains the contradiction:

- semantic validation failure becomes a degraded evaluation with `model_status = MALFORMED`;
- run-state selection currently becomes `COMPLETED` whenever freshness has not changed, independent of model validity;
- `AuditCenterPage.tsx` treats every model status except exactly `UNAVAILABLE` as a successful structured completion notice.

## Required target semantics

Define and test one canonical state rule:

1. `STALE` remains authoritative when freshness changes before persistence.
2. `COMPLETED` is permitted only when the persisted model-backed result is semantically authoritative, i.e. `modelStatus == AVAILABLE` and semantic validation has passed.
3. `MALFORMED`, process failure, timeout, auth failure, policy-blocked auth, usage-limited/provider-unavailable conditions must not be labeled `COMPLETED`.
4. Persist degraded evidence/history safely using an appropriate non-completed state, normally `FAILED` unless an existing more-specific state is already contractually correct.
5. Preserve the safe fallback verdict/diagnostic as historical evidence, but the UI must explicitly state that no authoritative model verdict was accepted.

Do not delete or hide degraded audit history.

## Frontend requirements

For non-AVAILABLE/degraded runs:

- do not show `Audit completed with structured evidence.`;
- show model status explicitly (`MALFORMED`, `UNAVAILABLE`, `PROCESS_ERROR`, etc.);
- show the concrete persisted `diagnostic` in a bounded diagnostics panel;
- show state truthfully (`FAILED`, `STALE`, etc.);
- do not visually imply PASS/completed acceptance;
- audit-history rows should make a failed/degraded model run distinguishable from a valid completed run.

## Required tests

Add backend and frontend tests proving at minimum:

- semantic validation error => `modelStatus = MALFORMED` and state is not `COMPLETED`;
- unavailable/process/auth/timeout degraded evaluations cannot become `COMPLETED`;
- valid AVAILABLE evaluation can still become `COMPLETED`;
- stale freshness still dominates and remains `STALE`;
- MALFORMED frontend notice is failure/degraded wording, never success wording;
- diagnostic text is rendered for degraded runs;
- historical degraded run remains selectable/reloadable.

---

# FINDING V06-F03 — MAJOR — PROJECT/FREEFORM CODEX OUTPUT CONTRACT DOES NOT GUARANTEE SEMANTIC VALIDITY

## Current native evidence

The owner ran a ScrubBots **Freeform project operation** audit.

Native evidence showed:

- task-specific requirements unavailable by design;
- `TASK_REQUIREMENTS · project-audit` evidence unavailable;
- model runtime `CODEX_CLI / CLI_DEFAULT`;
- final result `MALFORMED` after semantic validation;
- zero persisted requirement-coverage rows because degraded evaluation clears model-derived rows.

The exact semantic diagnostic was not visible in the screenshots. Do not invent it. The remediation must expose it.

## Existing semantic contract to preserve

When `AuditInput.requirements` is empty for a project/freeform audit, the current semantic validator expects exactly one project-level coverage row:

- `requirementRef = "project-audit"`
- `status = "NOT_APPLICABLE"`

For that freeform shape, finding `requirementRefs` must not invent unknown task requirement IDs.

The structural JSON schema alone cannot express this input-dependent rule. Existing transport tests accepting a schema-valid `requirementCoverage: []` prove that schema/transport success alone is not sufficient acceptance evidence.

## Required fix

Make the host-to-Codex contract deterministic and input-aware before execution.

At minimum:

1. Build explicit model instructions from the exact canonical input.
2. Enumerate the exact allowed evidence refs.
3. Enumerate the exact allowed task requirement refs when task requirements exist.
4. For project/freeform audits with zero canonical requirements, explicitly require exactly one coverage object with:
   - `requirementRef: "project-audit"`
   - `status: "NOT_APPLICABLE"`
   - bounded requirement text/rationale
   - no invented task requirement refs.
5. Explicitly instruct findings in the zero-requirement project/freeform case not to invent task requirement IDs. Preserve whichever empty/project-level requirement-ref behavior matches the semantic validator; keep prompt and validator generated from one canonical rule.
6. Do not weaken semantic validation merely to accept arbitrary model output.
7. Do not auto-promote or silently repair a materially inconsistent model verdict. Deterministic host-owned metadata such as the project-audit N/A coverage sentinel may be normalized only if it does not invent model judgment; document and test any such normalization.
8. Surface the exact semantic diagnostic when the output is still rejected.

Prefer centralizing the contract so prompt generation and semantic validation cannot drift independently.

## Required focused tests

Add a direct project/freeform end-to-end fixture through:

`AuditInput -> Codex request/prompt -> structured final JSON -> parse_model_output -> semantic validation -> run persistence`

The fixture must prove a valid project audit produces:

- modelStatus `AVAILABLE`;
- state `COMPLETED`;
- exactly one persisted `project-audit / NOT_APPLICABLE` coverage row;
- no unknown requirement refs;
- no invented PASS from missing evidence.

Also prove malformed project/freeform outputs such as an empty coverage array or unknown requirement refs remain rejected/degraded and are not `COMPLETED`.

Task-scoped audit fixtures must remain valid and must continue requiring exact canonical task requirement refs.

---

# PRESERVE V05 / PORTFOLIO TRUTH

Do not regress any accepted V05 behavior.

Required regression assertions:

- exactly 8 active portfolio projects;
- exactly one FormuLab project;
- FormuLab tracks `Sekiph82/FormuLab@main`;
- FormuLab attached local workspace metadata survives;
- branch-aware cache reuse remains correct;
- AI-Commerce-HQ does not return to the active portfolio;
- all other seven tracked repos remain on their accepted branches.

---

# TRACKER GOVERNANCE

The owner-native acceptance attempt has failed. Current/prospective tracker truth must no longer say that owner acceptance is merely pending with no remediation.

At implementation start, transition current truth to V06 remediation and Required Actor `CODEX`.

After successful builder completion, transition to:

- Current Milestone: `M16`
- Current Sprint: `M16T-CODEX-ONLY`
- Current Task: `M16T V06 — Native Codex audit acceptance remediation`
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_REACCEPTANCE`
- Next Task/Action: independent M16T V06 strict audit, then owner native Settings readiness + real Codex audit re-acceptance
- Required Actor: `HUMAN`
- M16T summary marker: `[~]`
- M16: `OPEN`
- M17: `NOT ACTIVATED / BLOCKED`
- strict completed milestone progress remains `16 / 20 = 80%`

Historical prompts/logs/audits remain immutable.

---

# REQUIRED VALIDATION

Run focused tests first, then repository-required regressions.

At minimum run and record:

1. Tauri permission/ACL contract tests for both readiness commands;
2. audit-provider readiness backend tests;
3. Settings readiness frontend tests, including invoke rejection vs true `CODEX_NOT_FOUND`;
4. project/freeform structured semantic integration tests;
5. task-scoped semantic integration tests;
6. MALFORMED/non-AVAILABLE audit-state persistence tests;
7. Audit Center notice/badge/diagnostic rendering tests;
8. existing Codex runtime bounded-process tests;
9. existing M16 audit-engine suite;
10. V05 portfolio/FormuLab regression tests;
11. full Rust library suite;
12. full frontend suite;
13. `npm run typecheck`;
14. `cargo check --manifest-path src-tauri/Cargo.toml`;
15. `npm run build`;
16. `git diff --check`;
17. active-source search proving no `OPENAI_API_KEY`, direct OpenAI HTTP audit transport, Responses API audit transport, or API-key fallback has returned;
18. governed native QA publication because Tauri permissions/native Rust and audit UI behavior changed;
19. stable Desktop shortcut target/icon and no-terminal-flash regression.

Automated tests must not consume live Codex quota. Mock deterministic process/final-output fixtures for automated coverage.

The final HUMAN acceptance after independent audit will perform the real Codex turn.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V06_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- implementation commit SHA(s) known before log publication;
- exact ACL commands added and permission identifier used;
- exact final audit-state semantics for AVAILABLE, MALFORMED, UNAVAILABLE/process/auth/timeout, and STALE;
- project/freeform model-contract rule and exact `project-audit / NOT_APPLICABLE` behavior;
- exact frontend degraded-state wording/diagnostic behavior;
- focused test commands and counts;
- full regression counts;
- FormuLab/eight-project preservation evidence;
- provider guardrail search result;
- native publication executable SHA-256 and shortcut target/icon evidence;
- tracker final state;
- explicit statement that no OpenAI API-key/direct HTTP provider and no Claude/M17 implementation was introduced.

Do not require the log to contain the SHA of the commit that first creates itself. After log publication, verify final remote `main` and report the log commit SHA in the final response.

---

# COMPLETION GATE

Return `COMPLETE` only when all of the following are true:

- both provider-readiness commands are allowed through the intended Tauri ACL;
- Settings readiness no longer false-negatives because of ACL denial;
- MALFORMED/degraded model results cannot be state `COMPLETED`;
- Audit Center never shows success wording for malformed/degraded model output;
- concrete non-AVAILABLE diagnostic is visible;
- deterministic project/freeform semantic integration tests pass;
- valid project/freeform model output persists the required `project-audit / NOT_APPLICABLE` coverage row and becomes `AVAILABLE + COMPLETED`;
- invalid freeform output remains rejected and non-completed;
- V05 FormuLab `main`, exact eight-project portfolio, local workspace preservation, and Codex-only architecture remain green;
- all required tests/build/publication gates pass;
- all repository changes are committed/pushed;
- local HEAD == origin/main == live GitHub main;
- worktree is clean;
- M16 remains OPEN pending independent V06 audit + owner native re-acceptance;
- M17 remains blocked.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must contain only:

- GitHub V06 log URL/path;
- implementation commit SHA(s);
- V06 log commit SHA;
- final GitHub main SHA;
- concise status.
