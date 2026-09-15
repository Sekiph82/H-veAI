# M18 GitHub Integration V04 — Strict Remediation + Owner-Directed UX Prompt

## AUTHORITY

Execute this prompt only after GitHub `main` contains:

- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V03_STRICT_AUDIT.md` with verdict `CHANGES_REQUIRED`;
- `docs/H!veAI/plans/M18_V04_OWNER_PROMPT_AGENT_UX_CONSOLIDATION_PLAN.md`;
- root `TASKS.md` showing M18 still open and V04 remediation as the current work;
- `CODEX_ROADMAP.md` showing M18 open and M19 blocked.

This prompt closes exactly these two bounded groups:

1. Independent strict-audit findings:
   - F-M18-V03-001
   - F-M18-V03-002
   - F-M18-V03-003
   - F-M18-V03-004
   - F-M18-V03-005
2. Owner-directed native UX/runtime tasks:
   - M18.10.01
   - M18.10.02
   - M18.10.03
   - M18.10.04
   - M18.10.05

Do not broaden scope. Do not reopen M14 or M15. Do not activate M19.

## MANDATORY SAFE SYNC

Before reading or modifying implementation files:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the worktree is clean and strictly behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never reset, rebase, force-push, destructively checkout, automatically stash, run `git clean`, or discard owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED` and do not modify implementation files.

## CANONICAL TRACKER OWNERSHIP

`TASKS.md` and `CODEX_ROADMAP.md` are strictly READ-ONLY for Codex.

Codex must not tick tasks, change Project Status metadata, close/open milestones, alter milestone progress, or activate M19. ChatGPT owns tracker transitions after independent audit and any required owner-native acceptance.

Before coding, prove both files are unchanged relative to synchronized `origin/main`. At completion, prove they remain unchanged.

## REQUIRED CONTEXT

Read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`
- `docs/H!veAI/plans/M18_V04_OWNER_PROMPT_AGENT_UX_CONSOLIDATION_PLAN.md`
- `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V03_PROMPT.md`
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V03_STRICT_AUDIT.md`
- `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V03_LOG.md`
- current M18 implementation and focused tests
- current Prompt Engine, Agent Session Center, Settings, navigation, routing, and prompt-context implementation/tests.

Preserve all accepted M00-M17, X03, and X04 behavior, including:

- root `TASKS.md` as the sole current project/task/workflow-status authority for local projects;
- tracked-branch root `TASKS.md` as the sole current authority for GitHub-tracked projects;
- hidden `.hiveai` control-plane files remain historical/domain evidence only;
- exact eight-project portfolio;
- `Sekiph82/FormuLab@main`;
- Registry project identity;
- Local Git Engine read-only-by-default safety;
- M14 Agent Session Center process/session/output semantics;
- M15 Prompt Engine immutable-version, approval, dispatch, and exact session provenance;
- M16 Codex-only audit provider;
- M17 Claude adapter;
- governed native QA publication;
- no new PAT/API-key setting merely for convenience;
- no remote or local destructive mutation.

---

# PART A — CLOSE THE FIVE M18 V03 STRICT-AUDIT FINDINGS

## REMEDIATION 1 — REAL PRODUCTION PR EVIDENCE

The V03 transport only fetches the pull-request list while the DTO/parser/UI expects richer fields that are not present in list responses.

Fix the production acquisition model so the bounded PR window is enriched from the actual required GitHub resources.

At minimum, for the bounded selected PR set, obtain and truthfully model:

- PR detail required for changed-file/addition/deletion summary where available;
- bounded changed-file evidence using the appropriate PR files resource;
- bounded reviews/review state using the appropriate review resource;
- bounded issue comments and/or review comments according to the UI contract;
- check/status evidence for the exact PR head SHA using the appropriate checks/status resource;
- explicit per-subresource state so unavailable/stale/rate-limited/auth/offline evidence is not rendered as a verified empty collection.

Define explicit per-snapshot limits for:

- number of PRs enriched;
- files per PR;
- comments/reviews per PR;
- checks/status records;
- total subresource requests;
- response bytes and retained text.

The product must never fabricate `reviewStatus`, `checkStatus`, changed-file counts, or comments from fields that the selected endpoint does not provide.

Keep PR creation unavailable unless a safe authenticated mutation path already exists. Do not add a credential system for this remediation.

## REMEDIATION 2 — REAL ACTIONS JOB / STEP / LOG EVIDENCE AND EMPTY-VS-UNAVAILABLE TRUTH

The V03 transport only fetches `/actions/runs`; GitHub returns job/log relationship resources separately.

Implement bounded production enrichment for the selected recent run window.

At minimum:

- keep run identity/status/conclusion/event/branch/SHA/PR association;
- fetch bounded jobs for selected runs from the actual run-jobs resource;
- retain bounded step summaries from those jobs;
- for failed/cancelled/timed-out runs where useful and supported, fetch bounded failed-log evidence using the actual job/run log resource without retaining unbounded archives or arbitrary files;
- define explicit request/byte/run/job/step/log limits;
- model per-resource state and provenance.

The frontend must distinguish:

- verified CURRENT empty Actions list => `No CI runs`;
- Actions resource unavailable;
- stale cached Actions evidence;
- rate limited;
- auth required;
- offline/network error;
- malformed response;
- current runs with no jobs yet;
- jobs unavailable while run metadata remains current.

Never show `No CI runs` merely because the returned actions vector is empty when the Actions resource itself is not verified CURRENT.

Workflow retry remains unavailable unless a previously governed authenticated mutation boundary supports it safely. Do not implement a new mutation credential path in V04.

## REMEDIATION 3 — SANITIZE BEFORE PERSISTENCE AND BEFORE UI

V03 persists successful raw GitHub JSON before safe projection. Refactor caching so plaintext secret/token-like values from successful remote responses cannot be written to SQLite or returned to the frontend.

Requirements:

1. Persist minimal bounded projected evidence rather than whole raw remote responses whenever practical.
2. Apply one shared sanitizer/redactor before persistence and before frontend DTO exposure.
3. Redact representative forms including:
   - `Authorization:` header values;
   - arbitrary `Bearer <token>` values, not only known prefixes;
   - `ghp_...`;
   - `github_pat_...`;
   - `gho_`, `ghu_`, `ghs_`, `ghr_` token families where applicable;
   - query/body key-value forms such as `token=`, `access_token=`, and equivalent obvious credential keys.
4. Redaction must be deterministic and bounded.
5. Do not inspect Git credential files or auth stores.
6. Cached metadata must remain schema/version/branch/repository scoped and backward-safe. Malformed or old unsafe cache rows must fail closed or be safely replaced, never surfaced as current.

Add direct persistence tests that query `github_sync_state.metadata_json` after injecting representative secrets into successful remote fixture payloads. Assert the plaintext values do not exist in SQLite and do not exist in the returned frontend-facing snapshot.

## REMEDIATION 4 — STRICT EXPLICIT LINK GRAMMAR

Replace broad `M...digit` recognition.

Task/session/project linkage must be evidence-backed and deterministic.

At minimum:

- accept canonical milestone/task forms intentionally supported by H!veAI, such as strict `M<number>` / `M<number>.<number>[.<number>...]` forms where appropriate;
- preserve supported explicit `TASK-...` and `SESSION-...` forms with bounded character rules;
- reject ordinary prose tokens merely beginning with `M` and containing digits;
- do not infer a link from title similarity;
- where the product claims canonical task linkage rather than a raw reference, validate the ID against the selected project's canonical task identity when practical without weakening X04 authority.

Required adversarial negatives include at least:

- `Model5`
- `May2026`
- `Mfoo7bar`
- random prose containing digits
- another project's unrelated task/session identifier.

Required positives include canonical examples used by the existing eight-project portfolio and the `M18.10.xx` task family introduced by the owner-directed addendum.

## REMEDIATION 5 — PRODUCTION-PATH TESTABILITY AND FULL GREEN REGRESSION

Do not keep the architecture where `snapshot()` swaps the whole production fetch path for synthetic `fixture_resources()` under `cfg(test)` and therefore never tests endpoint sequencing.

Introduce a narrow injectable/read-transport seam used by the same production acquisition/enrichment logic in tests. The deterministic test transport must return controlled responses for exact GitHub API request paths without real network mutation.

Directly prove at minimum:

- repository/default-branch/tracked-branch/remote-HEAD reads;
- bounded branches/commits;
- PR list -> detail/files/reviews/comments/check/status enrichment with exact request-path assertions;
- PR open/closed/merged/draft behavior;
- exact changed-file/diff summary bounds;
- explicit link positives and adversarial negatives;
- issue reads and no implicit title-match ownership;
- Actions run -> jobs -> steps enrichment with exact request-path assertions;
- failed-log bounded acquisition and truncation;
- no-CI versus unavailable/stale/rate-limit/auth/offline/malformed states;
- releases/draft/prerelease/tags;
- last-known-good cache retained only as stale/non-current;
- tracked-branch cache invalidation;
- response/request/cache bounds;
- successful-response secret redaction before persistence and UI;
- cross-project/arbitrary-host rejection;
- remote mutation unavailable/default-denied;
- Local Git reconciliation EQUAL/AHEAD/BEHIND/DIVERGED/detached/missing/stale/unavailable and dirty separation;
- exact eight-project portfolio;
- `Sekiph82/FormuLab@main`;
- X04 TASKS-only authority regression;
- M13-M17 provider/session/audit regressions relevant to identity;
- mounted GitHub frontend current/empty/unavailable/stale/error states.

### Full Rust regression gate

The V03 full Rust run failed `watcher::tests::live_dashboard_contract_changes_reconcile_watcher_scope_without_restart` twice in the parallel suite but passed in isolation.

Diagnose this deterministically.

- If it is a test-harness race, fix the test/harness without weakening the production watcher contract.
- If it exposes a real watcher defect, fix the smallest production defect and add direct regression coverage.
- Do not skip, ignore, early-return, mark flaky, lower assertions, serialize the entire suite merely to hide the race, or otherwise suppress the signal without root-cause evidence.

M18 V04 cannot report implementation complete unless the full Rust library suite passes in the required normal invocation.

---

# PART B — OWNER-DIRECTED PROMPT ENGINE / SESSION UX CONSOLIDATION

The native screenshots supplied by the owner are authoritative acceptance input for this bounded V04 addendum. Implement M18.10.01 through M18.10.05 exactly as described in `docs/H!veAI/plans/M18_V04_OWNER_PROMPT_AGENT_UX_CONSOLIDATION_PLAN.md`.

## M18.10.01 — ONE TOP-LEVEL PROMPT ENGINE WORKSPACE

Current problem:

- `Agents` and `Prompt Engine` are separate primary navigation destinations even though session execution is the continuation of the prompt workflow.

Required result:

- remove the standalone `Agents` item from primary sidebar navigation and the command palette;
- keep `Prompt Engine` as the single top-level entry for prompt creation and provider-session work;
- inside Prompt Engine expose two clear internal surfaces, tabs, or equivalent segmented navigation:
  - `Prompt Builder`
  - `Sessions`
- move/reuse the accepted Agent Session Center UI under `Sessions` rather than cloning provider/session logic;
- preserve session project selector, task identity, prompt/session operations where still appropriate, current conversation, persisted sessions, lifecycle actions, PTY/output/final-response reader, permission/wait diagnostics, Git/diff evidence, polling, and exact persisted-session selection;
- do not rewrite the M14 provider/session backend merely to change information architecture.

The integrated page must remain readable at the accepted desktop viewport. Follow `docs/H!veAI/UI_LAYOUT_GOVERNANCE.md` and preserve the current H!veAI dark-glass/background/topbar visual language.

## M18.10.02 — BACKWARD-COMPATIBLE SESSION LINKS AND EXACT POST-DISPATCH HANDOFF

Preserve accepted M15C/M15D provenance semantics.

Required behavior:

- existing `/agents?...` links remain safe and backward compatible;
- a legacy `/agents?projectId=<registered>&sessionId=<persisted>` target must redirect/resolve into the integrated Prompt Engine Sessions surface while preserving and validating the exact target;
- new Prompt Engine post-dispatch handoff must target the integrated Sessions surface directly;
- replace user-facing copy such as `View result in Agents` with integrated-workspace wording such as `View session` or equivalent;
- opening the dispatched result selects the exact persisted session and registered project;
- navigation must never redispatch the prompt, relaunch the provider, or create a second session;
- missing, malformed, stale, unregistered-project, wrong-project, or cross-project targets fail closed with bounded truthful UI;
- preserve ordinary manual Sessions usage when no deep-link target exists.

Prefer one canonical route-target builder/parser shared by old compatibility and new links rather than two divergent query formats.

## M18.10.03 — MOVE BUILDER PROVIDER READINESS TO SETTINGS

Current problem:

- the Agent Session Center begins with a large `Claude Code readiness` capability panel, while provider configuration/diagnostics belong with Settings.

Required result:

- remove the large Claude readiness/capability card from the Sessions operation surface;
- add a dedicated `Builder Providers` or equivalent Settings panel using the existing provider-neutral readiness API;
- show both `CODEX` and `CLAUDE` truthfully, including readiness state, version, authentication/readiness diagnostics, resume support, and bounded capabilities where available;
- keep builder-provider diagnostics visibly distinct from the existing `Codex Audit Provider` Settings panel because builder/session readiness and audit-provider readiness are separate contracts;
- do not add API keys, PATs, token fields, credential-file reads, or auth-file inspection;
- Sessions must still use current readiness internally to enable/disable Start truthfully;
- when the selected provider is unavailable, Sessions may show a concise bounded diagnostic and/or Settings navigation hint, but must not recreate the large readiness chip wall.

Moving the presentation must not weaken M17 readiness, auth, quota, permission, or exact-resume semantics.

## M18.10.04 — FIX THE PROMPT ENGINE NULLABLE `required_actor` CRASH

Owner-native evidence shows this exact Prompt Engine error:

`Invalid column type Null at index: 2, name: required_actor`

Current production source reads `tasks.required_actor` as a non-null Rust `String` inside Prompt Engine context collection even though real task rows may contain SQL NULL.

Required fix:

- read `required_actor` as `Option<String>` in the production query/model path;
- preserve SQL NULL as null/unavailable truth in the context manifest;
- do not invent `HUMAN`, `CODEX`, `CLAUDE`, or any other actor when source truth is null;
- verify other selected nullable task fields remain decoded with optional types;
- context refresh and draft generation must work for a valid task whose `required_actor` is NULL;
- preserve exact project/task confinement and X04 TASKS-only authority.

Add a direct Rust regression using a real temporary database/task row with `required_actor = NULL`. The test must exercise the production context collector and prove the observed SQLite error cannot recur. Also cover prompt generation from that context where practical.

## M18.10.05 — FOCUSED UX/RUNTIME REGRESSION + NATIVE ACCEPTANCE GATE

Add deterministic evidence for the owner-directed changes.

Frontend tests must directly prove:

1. primary sidebar/command-palette navigation contains `Prompt Engine` but no separate `Agents` entry;
2. Prompt Engine exposes `Prompt Builder` and `Sessions` internal surfaces;
3. the integrated Sessions surface still renders existing session operations and persisted session evidence;
4. legacy `/agents?projectId=...&sessionId=...` resolves to the exact integrated session;
5. new post-dispatch handoff resolves to the exact integrated session and does not invoke dispatch again;
6. wrong-project, missing, malformed, and stale session targets fail closed;
7. Settings renders builder-provider readiness for CODEX and CLAUDE separately from the Codex Audit Provider panel;
8. the Sessions surface no longer renders the large Claude readiness capability card;
9. selected-provider readiness still gates session Start truthfully;
10. existing M14 and M15 focused routing/session/provenance tests remain green.

Rust tests must directly prove:

- `required_actor = NULL` context collection succeeds;
- null actor is represented as unavailable/null, not fabricated;
- prompt generation remains bounded and project/task scoped;
- existing provider/session/audit identity boundaries remain intact.

Codex must not perform owner-native acceptance. After source re-audit passes, the owner will visually verify the integrated navigation, Sessions placement, Settings diagnostics placement, and absence of the nullable-actor error.

---

# UI REQUIREMENTS

Preserve the current visual system and Project Cockpit structure.

The M18 GitHub tab must remain dense and readable and display truthfully:

- repository/tracked branch/remote HEAD;
- local/remote reconciliation;
- PR evidence;
- issues;
- Actions/CI with explicit resource health;
- releases/tags;
- cache/freshness/rate-limit/auth/offline diagnostics;
- mutation unavailability where applicable.

The Prompt Engine consolidation must additionally satisfy:

- one primary `Prompt Engine` navigation entry for prompt/session work;
- obvious `Prompt Builder` / `Sessions` switching;
- no standalone Agents primary-nav item;
- no large Claude readiness panel at the top of Sessions;
- builder provider readiness diagnostics in Settings;
- no raw provider/API JSON dumped into ordinary UI;
- no new unnecessary nested scrolling or layout regression.

Do not expose raw GitHub API JSON.

# SECURITY / NEGATIVE SEARCH GATES

Before completion, independently search the final diff/source for accidental additions of:

- GitHub PAT/API-key settings;
- plaintext tokens/secrets;
- credential-file reads;
- arbitrary frontend-controlled GitHub API hosts;
- shell command-string construction;
- automatic push/merge/rebase/reset/checkout/tag/release/issue mutation/workflow retry;
- hidden `.hiveai` current-truth fallback;
- duplicate provider/session launch paths introduced by the UI consolidation;
- implicit provider redispatch during route/session selection;
- Codex edits to `TASKS.md` / `CODEX_ROADMAP.md`;
- M19 activation.

All must remain absent unless explicitly required above, which they are not.

# REQUIRED VERIFICATION

Run and record exact results for:

- focused M18 Rust tests through the same production GitHub acquisition/enrichment logic;
- relevant cache/redaction/security tests;
- relevant GitHub tracking / exact-eight-project / FormuLab tests;
- relevant Local Git Engine reconciliation tests;
- direct Prompt Engine null-`required_actor` Rust regression;
- relevant M13-M17 identity/provider/audit regressions;
- focused mounted GitHub frontend tests;
- focused Prompt Engine + Sessions + Settings + legacy-route frontend tests;
- existing M14/M15 focused frontend regressions;
- full frontend test suite;
- full Rust library suite, genuinely green;
- `npm run typecheck`;
- `npm run build`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `git diff --check`.

Check GitHub status-check availability on the final implementation commit and state the result explicitly in the log. Builder output remains claim evidence, not independent acceptance.

# GOVERNED NATIVE PUBLICATION

Publish only after all required implementation and regression gates pass.

Use the existing governed `scripts/publish-dev-qa.ps1` path. Preserve:

- stable `dev-bin/H!veAI.exe`;
- stable Desktop shortcut target;
- stable icon contract;
- no dev-server dependency;
- no visible terminal/console regression.

Record the published executable SHA-256.

Do not perform owner-native acceptance yourself. Stop for independent source audit first.

# REQUIRED BUILDER LOG

Create:

`docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V04_LOG.md`

Record at minimum:

- synchronized starting SHA;
- implementation/test commit SHA(s);
- exact files changed;
- disposition of F-M18-V03-001 through F-M18-V03-005;
- disposition of M18.10.01 through M18.10.05;
- production GitHub endpoint/enrichment topology and explicit request bounds;
- safe cache projection/redaction architecture;
- exact link grammar and adversarial negatives;
- watcher full-suite failure root cause and fix/evidence;
- Prompt Engine/Agent Sessions information-architecture changes;
- legacy `/agents` compatibility and new exact session-handoff behavior;
- Settings builder-provider readiness placement and separation from Codex Audit Provider;
- exact nullable-`required_actor` root cause, fix, and direct test evidence;
- focused and full test commands/results;
- GitHub status-check availability;
- security/negative-search results;
- governed publication result and EXE SHA-256;
- explicit proof `TASKS.md` and `CODEX_ROADMAP.md` were not modified by Codex;
- final local HEAD / `origin/main` / live GitHub main equality;
- clean worktree proof.

# COMPLETION CONTRACT

Commit and push all implementation/test/log changes.

Before reporting completion:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Local HEAD, `origin/main`, and live GitHub `main` must be identical and the worktree must be clean.

Do not mark M18 PASS/CLOSED. Do not activate M19. Do not edit canonical tracker files.

Final owner-facing response must contain only:

- GitHub URL/path for `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V04_LOG.md`;
- implementation/test commit SHA(s);
- builder-log commit SHA;
- final GitHub `main` SHA;
- concise status.
