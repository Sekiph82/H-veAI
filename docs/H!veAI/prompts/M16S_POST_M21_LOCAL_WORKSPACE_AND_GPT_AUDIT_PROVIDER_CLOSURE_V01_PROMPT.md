# M16S Post-M21 Local Workspace and GPT Audit Provider Closure V01

## MANDATORY SYNC-FIRST AND GITHUB-FIRST CONTRACT

Before reading or changing any implementation file, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

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

After synchronization, read exactly these current authorities before implementation:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V01_AUDIT.md`
- `docs/H!veAI/audits/M21-R03-ACC_OWNER_FINAL_LAUNCH_CONFIRMATION_V01_AUDIT.md`
- `docs/H!veAI/audits/M16Q_OWNER_NATIVE_ACCEPTANCE_FAILURE_AND_M16R_SCOPE.md`
- `docs/H!veAI/prompts/M16_GPT_AUDIT_ENGINE_UNIFIED_WHOLE_MILESTONE_IMPLEMENTATION_PROMPT.md`
- `docs/H!veAI/UI_LAYOUT_GOVERNANCE.md`

All Codex-facing work and the required builder log must be entirely in English.

All H!veAI repository changes made by this task must be committed and pushed before completion is claimed. At the end verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == live remote main` and a clean H!veAI working tree. Otherwise return `SYNC_BLOCKED`.

Owner-facing final response must contain only relevant GitHub H!veAI file URLs/paths, implementation/log commit SHA(s), final GitHub `main` SHA, and concise status. Do not dump ordinary local changed-file paths.

---

## WORK ITEM

- Work code: `M16S`
- Version: `V01`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V01_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

This is a bounded M16 closure remediation after the completed standalone M21 migration. Do not activate M17 and do not implement Claude work in this task.

The owner-native evidence exposes exactly three current findings:

1. `M16S-F01` BLOCKER: production Audit Engine is permanently wired to `UnavailableAuditModel`, so a real configured GPT audit can never run.
2. `M16S-F02` MAJOR: an existing ACTIVE GitHub-first project cannot explicitly attach or change its local workspace through normal native UI even though safe backend rebinding already exists.
3. `M16S-F03` MAJOR: current tracker/roadmap truth is stale after M21-R03 independent audit, owner final launch acceptance, and retirement of the historical local parent.

Close all three in one implementation cycle. Do not expand into M17, M18, general Settings redesign, generic multi-provider orchestration, or unrelated registry architecture.

---

# FINDING M16S-F01 — REAL PRODUCTION OPENAI GPT AUDIT PROVIDER

## Current incorrect behavior

`src-tauri/src/audit_engine.rs` defines a provider-neutral `AuditModel` trait and a safe `UnavailableAuditModel`, but production:

```rust
pub fn run(database: &DatabaseState, request: AuditInputRequest) -> Result<AuditRun, String> {
    run_with_model(database, request, &UnavailableAuditModel)
}
```

always chooses the unavailable implementation.

The native UI therefore truthfully reports `GPT audit provider is not configured`, `UNAVAILABLE`, `CONDITIONAL`, and low confidence even when the owner wants a real GPT audit.

The unavailable fallback is good safety behavior and must remain. The defect is that there is no real configured production provider path.

## Required target architecture

Preserve the existing `AuditModel` abstraction and strict M16 evidence/result contracts. Add one production OpenAI audit provider behind that boundary.

The provider resolution model must be deterministic:

1. resolve provider readiness/configuration;
2. if a valid OpenAI audit credential and model are configured, instantiate the production OpenAI provider;
3. otherwise use the existing unavailable fallback and explain the exact non-secret readiness reason;
4. never silently use Codex or Claude as the GPT audit model;
5. never turn network/provider failure into PASS.

If the current `AuditModel` trait's `'static` provider/model/version return types prevent a configurable model name, change the trait cleanly rather than hard-coding a model in scattered call sites. Update all fixture/unavailable tests accordingly.

## OpenAI API contract

Use the current OpenAI Responses API, not a legacy completion contract.

Current official API contract to implement against:

- HTTPS `POST https://api.openai.com/v1/responses`
- bearer API-key authentication
- strict Structured Outputs using JSON Schema for the existing typed audit result contract
- bounded `max_output_tokens`
- set API response storage to disabled (`store: false`) for audit payload privacy unless a later explicit product decision changes it
- do not enable web search, file search, computer use, or unrelated OpenAI tools for this audit call

Official reference for implementation verification:

- `https://developers.openai.com/api/reference/resources/responses/methods/create`

Do not assume a model from an obsolete historical prompt. The audit model is a non-secret configurable setting. Use a validated default only if it is explicitly supported by the current OpenAI Responses API; otherwise require the user to configure the model and report readiness as incomplete. Do not fetch or guess model names from untrusted network content.

## Secret handling

Raw OpenAI API credentials are security-sensitive.

Required rules:

- never commit an API key;
- never write the raw key to SQLite, JSON receipts, localStorage, sessionStorage, logs, audit rows, diagnostics, prompt bodies, crash output, or frontend-readable application state;
- prefer Windows OS-backed secure credential storage for an API key entered through H!veAI;
- if OS-backed secure storage cannot be implemented safely within this bounded task, support `OPENAI_API_KEY` as a process-environment fallback and expose a clear native setup/readiness message, but do not invent plaintext app storage;
- environment fallback may be read only by the native backend;
- the frontend may receive only booleans/status metadata such as `configured`, provider name, selected model and non-secret error category;
- redact Authorization headers and credential-looking values from all diagnostics;
- deleting/replacing a stored credential must not expose its old value.

Do not ask the owner to paste an API key into Codex, GitHub, the builder log, or source files.

## Settings/readiness UX

Add a compact `GPT Audit Provider` section to the existing Settings experience, consistent with current UI governance.

It must show at minimum:

- Provider: OpenAI
- Status: CONFIGURED / NOT_CONFIGURED / AUTH_ERROR / RATE_LIMITED / NETWORK_ERROR / READY as applicable
- selected audit model
- credential source in non-secret form, for example `Windows secure storage` or `OPENAI_API_KEY environment`
- Save/Replace credential action only if secure OS-backed credential storage is implemented
- Remove credential action only if secure OS-backed credential storage is implemented
- editable non-secret model setting
- a bounded `Check readiness` action that never prints the key

Do not render the existing key back into an input field.

Do not create a giant provider administration screen.

## Provider transport and execution safety

Use a maintained HTTPS client appropriate to the existing Rust/Tauri architecture. Do not shell out to curl, PowerShell, Python, Node, or browser JavaScript to call OpenAI.

Requirements:

- explicit connection and total request timeout;
- bounded request body using the existing M16 evidence limits;
- bounded response body before parsing;
- no UI-thread indefinite blocking;
- deterministic HTTP status classification;
- 401/403 => authentication/configuration failure;
- 429 => rate/quota limitation;
- timeout/network => unavailable/degraded result;
- 5xx => provider unavailable/error;
- malformed JSON or schema-invalid structured output => no fabricated result, deterministic CONDITIONAL/FAILED behavior consistent with M16 contracts;
- provider refusal/incomplete response => truthful non-PASS state;
- provider/model/version used must be persisted on successful real audit runs;
- preserve audit freshness checks before final persistence so a response for stale evidence cannot be treated as current.

## Structured result

Do not create a second audit-result schema.

Map the OpenAI Structured Output exactly into the existing M16 typed semantics:

- verdict: PASS / CONDITIONAL / FAIL
- confidence: HIGH / MEDIUM / LOW
- regression risk
- summary
- findings with bounded stable keys/severity/evidence refs/remediation
- requirement coverage
- prior finding dispositions when applicable

Validate all model evidence references against the input evidence set before persistence. Preserve existing semantic verdict validation and prior-finding rules.

## Required F01 tests

Add focused tests using injected/mock HTTP transport or an equivalent deterministic boundary. Tests must not spend real API credits.

At minimum prove:

- configured provider constructs the expected Responses API request;
- `store: false` is sent;
- no tools are enabled;
- Structured Output schema is requested;
- successful structured response maps to existing AuditEvaluation correctly;
- provider/model/version persistence is correct;
- malformed JSON is rejected;
- schema-invalid output is rejected;
- invalid evidence references cannot be persisted as valid findings/coverage;
- 401/403 becomes auth failure and never PASS;
- 429 becomes rate/quota limitation and never PASS;
- timeout/network/5xx never PASS;
- missing credential retains existing truthful unavailable fallback;
- secret value never appears in serialized frontend readiness state, audit diagnostic, log output, or persisted audit data;
- stale-head/freshness protection still rejects a response collected against obsolete repository truth.

A real owner API key is not required for automated tests.

---

# FINDING M16S-F02 — ATTACH / CHANGE LOCAL WORKSPACE FOR EXISTING GITHUB-FIRST PROJECT

## Current incorrect behavior

The GitHub-first portfolio now correctly maintains one logical project per repository. Remote-only seeded records can be `ACTIVE` with no meaningful local workspace path.

The backend already supports safe rebinding through `repair_project_path`, and frontend IPC exposes `repairProjectPath(projectId, path)`.

However `ProjectRegistryCard` only renders the path action when:

```tsx
project.status === 'MISSING'
```

An ACTIVE remote-only project therefore has no explicit way to attach its local checkout. This is why the owner cannot attach the new standalone H!veAI checkout to the existing H!veAI project through normal UI.

## Required target behavior

One GitHub repository remains one logical project.

Local workspace is optional capability metadata on that same project identity.

For a non-archived GitHub project expose an explicit native local-workspace action with truthful label/state:

- no bound local path => `Attach local workspace`
- bound valid local path => `Change local workspace`
- bound path missing/invalid => `Repair local workspace`

The action must edit the existing project record. Do not tell the owner to use `Add project` for an already tracked GitHub identity, and do not create a duplicate ninth project.

Provide this local-workspace control in a discoverable project surface, preferably both:

- Projects project-card actions or an explicit local-workspace row/action; and
- Project Cockpit Settings for the selected project.

Keep the UI compact and consistent with `UI_LAYOUT_GOVERNANCE.md`.

## Local path selection

Reuse the existing safe path-binding backend. Do not bypass it with direct SQLite mutation.

If an already-approved native directory picker exists, use it. If none exists, a compact explicit path-selection dialog reusing the current register/repair UX is acceptable. Do not add broad frontend filesystem permissions merely to implement a picker.

The owner must be able to attach the standalone H!veAI checkout after publication:

`<Desktop>\H!veAI`

Do not hard-code the username or this path into application source/database migrations. It is owner-selected runtime state.

## Identity/safety invariants

Preserve and test:

- canonical path validation;
- path existence/directory checks;
- Git metadata probe;
- repository remote identity match with the existing project;
- mismatch/ambiguous remote rejection;
- duplicate local-path prevention;
- no automatic reset/rebase/checkout/pull/merge/clean/stash;
- no local file move/delete;
- no new logical project created;
- project UUID and linked task/session/audit history preserved;
- portfolio count remains exactly 8;
- GitHub/root `TASKS.md` remains task/status truth even when a local workspace is attached;
- local workspace is used only for local capabilities such as local Git, Files, Agents/worktree execution and other existing local project functions.

After a successful attach/change, refresh affected Registry/Git/watcher/local-capability state without changing remote project truth or duplicating the card.

## Required F02 tests

At minimum prove:

- ACTIVE remote-only project exposes Attach local workspace;
- successful matching repository attach updates same project ID;
- portfolio remains exactly 8;
- no duplicate project is created;
- ACTIVE project with valid local path exposes Change local workspace;
- missing path exposes Repair local workspace;
- wrong GitHub remote is rejected;
- non-Git/path-type mismatch is rejected when project identity requires Git;
- duplicate local path is rejected;
- linked project identity/history remains intact;
- attaching a local workspace does not replace GitHub/root-TASKS current task truth;
- project watcher/local Git refresh remains bounded and no terminal window regression is introduced.

---

# FINDING M16S-F03 — POST-M21 TRACKER AND ROADMAP RECONCILIATION

Current root tracking is stale after completed M21-R03 independent audit, owner final launch confirmation, and retirement of the historical local parent.

Update current/prospective truth only.

Required final tracker state after successful M16S implementation:

- M21 standalone migration / M21-R01 / M21-R02 / M21-R03 are PASS/CLOSED based on existing accepted evidence;
- the historical local AI-Commerce parent retirement is recorded as owner-completed;
- do not claim deletion of the GitHub `AI-Commerce-HQ` repository unless repository reality proves the owner has actually deleted it;
- M16 remains OPEN during implementation;
- after builder completion, M16S is `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`;
- Required Actor becomes HUMAN after builder completion;
- M17 remains NOT ACTIVATED / BLOCKED until M16S independent audit and owner native acceptance both PASS;
- remove or correct stale current/prospective statements in `TASKS.md` and `CODEX_ROADMAP.md` that still describe M21 as not started;
- preserve historical immutable audits/prompts/logs and do not rewrite their then-current statements.

Do not change the 20-milestone user-facing denominator.

---

# REQUIRED VALIDATION

Run the narrowest relevant focused tests first, then full regressions required by repository governance.

At minimum:

1. Rust focused Audit Engine provider/config/transport/result tests.
2. Rust Project Registry/path repair/identity tests.
3. Frontend Project Registry/local-workspace UX tests.
4. Frontend Audit Center/Settings readiness-state tests.
5. Existing M16 audit-engine regression suites.
6. Existing GitHub tracking / Project Cockpit / Command Center regressions that protect the eight-project GitHub-first behavior.
7. Full frontend test suite.
8. Relevant full Rust test suite under the repository's bounded policy.
9. TypeScript typecheck.
10. Production frontend build.
11. `git diff --check`.
12. Governed native QA publication using the existing safe publisher after all tests pass.
13. Validate stable `Desktop\H!veAI.lnk` still launches the newly published `dev-bin\H!veAI.exe` with no terminal flash.

Do not use a real OpenAI API key in automated CI/test fixtures. Automated tests must use deterministic mocked transport.

If a real provider credential already exists securely on the owner machine, a minimal readiness smoke may be performed only if it cannot expose the key and the owner has already configured it. Do not ask Codex to obtain or print the key. Owner real-provider acceptance remains a post-builder manual gate.

---

# PROHIBITED SHORTCUTS

Do not:

- hard-code a fake successful GPT audit;
- treat `FixtureAuditModel` as production;
- automatically convert UNAVAILABLE into PASS;
- store an API key in SQLite, source, `.env` committed to Git, frontend storage, logs or audit diagnostics;
- expose the API key to React/frontend IPC;
- shell out to curl/PowerShell/Python/Node for OpenAI network requests;
- weaken the existing evidence/freshness/result schema merely to accept model output;
- create a ninth project to attach H!veAI locally;
- reintroduce `AI-Commerce-HQ` into the eight-project default portfolio;
- make local task files override GitHub/root `TASKS.md` current truth;
- modify another GitHub repository;
- implement M17 Claude Code Adapter;
- rewrite historical immutable prompts/logs/audits;
- delete preservation material;
- reset/rebase/force-push/clean/stash away owner work.

---

# OWNER MANUAL ACCEPTANCE REQUIRED AFTER BUILDER + INDEPENDENT AUDIT

Do not claim these owner actions yourself.

The owner must later verify in the published native app:

1. H!veAI project exposes `Attach local workspace` or equivalent explicit action.
2. Owner attaches the standalone H!veAI checkout through the UI.
3. The H!veAI project shows the new local path and local capabilities work.
4. Portfolio remains exactly 8 and no duplicate H!veAI card appears.
5. Settings shows GPT Audit Provider readiness without displaying the secret.
6. Owner securely configures an OpenAI API key if not already configured and selects/accepts the audit model.
7. A real Audit Center audit completes with a real OpenAI provider/model, model status not `UNAVAILABLE`, and structured verdict/coverage/findings.
8. Audit history persists provider/model provenance and no credential text is visible.
9. An unavailable/auth/quota/network condition still reports truthful non-PASS behavior.

M17 may only be activated after independent M16S audit and these owner acceptance gates pass.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V01_LOG.md`

The log must include:

- synchronized starting H!veAI SHA;
- implementation commit SHA(s) known before log publication;
- exact production provider resolution design;
- secret-storage mechanism and environment fallback behavior;
- non-secret readiness states;
- OpenAI Responses API request contract implemented;
- Structured Output/schema enforcement evidence;
- provider HTTP/error classification evidence;
- list of focused provider tests and results;
- local workspace attach/change UX and backend path used;
- identity/duplicate-prevention evidence;
- portfolio count regression evidence;
- tracker/roadmap reconciliation summary;
- typecheck/build/full test results;
- native publication result and executable SHA-256;
- explicit statement that no raw API credential was logged/committed/persisted to SQLite/frontend storage;
- explicit statement that M17 was not activated;
- final GitHub synchronization proof.

Do not attempt to place the log's own creating commit SHA inside the log. Return that commit SHA in the final response after publication.

---

# COMPLETION STATUS

Return exactly one:

- `READY_FOR_INDEPENDENT_M16S_AUDIT`
- `SYNC_BLOCKED`
- `IMPLEMENTATION_BLOCKED`

`READY_FOR_INDEPENDENT_M16S_AUDIT` is allowed only if all repository implementation/tests/publication gates pass, every H!veAI change is committed and pushed, and local HEAD/origin/main/live remote main are identical.

A missing owner OpenAI API key does not justify insecure credential handling and does not by itself make the code implementation blocked. In that case the production provider implementation must be complete and deterministic mocked transport tests must pass; real provider execution remains explicitly pending owner manual acceptance after independent audit.
