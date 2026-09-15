# M18 GitHub Integration V06 — Final Residual Strict Remediation Prompt

## AUTHORITY

Execute this prompt only after GitHub `main` contains:

- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V05_STRICT_REAUDIT.md` with verdict `FAIL — CHANGES_REQUIRED`;
- the historical V05 builder log at `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V05_LOG.md`;
- M18 still OPEN and M19 still blocked.

This V06 prompt closes only:

- F-M18-V05-001
- F-M18-V05-002
- F-M18-V05-003

Do not broaden scope. Do not reopen accepted M14/M15/M17 architecture. Do not activate M19.

`TASKS.md` and `CODEX_ROADMAP.md` are strictly READ-ONLY for Codex. ChatGPT owns all canonical tracker transitions.

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

If and only if the worktree is clean and local HEAD is strictly behind `origin/main`, update with:

```powershell
git merge --ff-only origin/main
```

Never reset, rebase, force-push, destructively checkout, automatically stash, run `git clean`, or discard owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED` and do not modify implementation files.

At completion, every H!veAI change must be committed and pushed. Do not report complete unless local HEAD, `origin/main`, and live GitHub `refs/heads/main` are identical and the worktree is clean.

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
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V04_STRICT_REAUDIT.md`
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V05_STRICT_REAUDIT.md`
- `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V05_LOG.md`
- current `github_integration`, Prompt Engine/session routing, GitHub tracking, task-intelligence test failpoints, and focused tests.

## PRESERVE ALL ACCEPTED V05 WORK

Do not rewrite or regress the V05 architecture that already passed source inspection:

- GitHub cache schema version 2 and schema-1 rejection;
- shared bounded pre-persistence sanitizer architecture;
- selected-PR detail/files/reviews/comments/check/status enrichment;
- verified-empty versus unavailable CI truth;
- Actions jobs/steps acquisition;
- job-level failure candidate selection;
- job ID/name provenance for retained failed-log excerpts;
- raw explicit task/session references separated from validated links;
- selected-project task validation through accepted X04 tracked-branch TASKS-only authority;
- selected-project persisted session ownership validation;
- `/agents` as legacy-only routing into Prompt Engine Sessions;
- no standalone top-level Agents navigation/command-palette surface;
- Prompt Builder + Sessions integrated Prompt Engine workspace;
- Builder Providers in Settings, separate from Codex Audit Provider;
- selected-provider readiness gating;
- pre-acquisition enrichment request-budget reservation;
- exact eight-project portfolio;
- `Sekiph82/FormuLab@main`;
- Local Git read-only/default-denied safety;
- Codex-only audit provider;
- Claude adapter and exact-resume contracts;
- M14/M15 prompt/session provenance;
- X03/X04 behavior;
- all accepted M00-M17 behavior.

## F-M18-V05-001 — MAJOR — CLOSE QUOTED AUTHORIZATION REDACTION HOLE

### Current incorrect behavior

`sanitize_error()` now handles many credential forms, but `authorization_span()` does not tolerate a quoted key boundary between `Authorization` and the colon.

A retained GitHub string such as:

```text
{"Authorization":"Basic QUOTED_AUTH_SECRET"}
```

can bypass the Authorization matcher because the byte following `Authorization` is the closing quote, not `:`. The bearer matcher cannot rescue a non-Bearer scheme such as `Basic`.

This is a production security boundary because an issue/PR body or Actions log may contain JSON/code snippets as ordinary string content. Such inner text is sanitized as a string; it is not recursively parsed as a JSON object.

### Required target behavior

1. Make Authorization-key recognition safe for reasonable retained-text forms including:
   - `Authorization:Basic SECRET`
   - `Authorization: Basic SECRET`
   - `Authorization:Bearer SECRET`
   - `Authorization: Bearer SECRET`
   - `"Authorization":"Basic SECRET"`
   - `"authorization": "Bearer SECRET"`
   - punctuation-adjacent equivalents with reasonable credential boundaries.
2. Preserve harmless surrounding text where practical while removing the credential value.
3. Keep the scanner bounded and deterministic. Do not add unbounded regex/backtracking behavior.
4. Keep cache schema version 2 unless an actual representation incompatibility requires another bump.
5. Preserve rejection of schema-1 and malformed/unsanitized cache rows.
6. Do not read GitHub credentials, environment secrets, auth stores, or provider credential files.

### Required direct tests

For each representative class below, exercise the real successful GitHub resource path, then directly inspect both:

- persisted `github_sync_state.metadata_json`;
- the frontend-facing `GitHubIntegrationSnapshot`/DTO projection.

The plaintext secret must be absent from both.

Required representatives:

- URL `access_token=`;
- URL `token=`;
- no-space Authorization Bearer;
- spaced Authorization Bearer;
- no-space Authorization Basic;
- quoted JSON-looking Authorization Basic inside an issue/PR/log string;
- quoted token/api-key assignment;
- each supported GitHub token prefix family.

Also prove:

- schema-1 unsafe row rejected;
- malformed row rejected/fails closed;
- sanitized schema-2 row reloads successfully;
- repository/branch/resource scope remains exact.

## F-M18-V05-002 — MAJOR — COMPLETE THE REQUIRED PRODUCTION-PATH EVIDENCE MATRIX

V05 source logic is substantially improved, but the exact required adversarial matrix was not implemented in full. Add deterministic tests that exercise the same production acquisition/selection/routing functions rather than helper-only projections.

### A. Actions acquisition/log matrix

Using `snapshot_with_transport` / `fetch_resources_with_transport` or the exact production-equivalent seam, prove with exact requested URL assertions:

1. success job before failing job -> only failed job log requested;
2. two success/skipped jobs before a third failing job -> third failed job log requested;
3. cancelled job eligible;
4. timed-out job eligible;
5. action-required job eligible when represented by the supported job contract;
6. success/skipped jobs excluded;
7. first eligible failed log unavailable and second current -> truthful `PARTIAL`, second evidence retained with job provenance;
8. jobs resource unavailable -> no fabricated failed log and truthful jobs/log state;
9. failed run with current jobs but no eligible failed-job record -> no successful job log attached and truthful `NOT_APPLICABLE`/chosen state;
10. deterministic two-log bound;
11. retained excerpt truncation bound;
12. no N+1 transport acquisition after the global subresource budget is exhausted.

Do not loosen or increase limits merely to make tests pass. Do not remove truthful bounded/unavailable resource state.

### B. Project-owned reference matrix

Build selected-project and foreign-project canonical fixtures using syntactically valid IDs. Prove:

- selected-project canonical task validates;
- selected-project persisted session validates;
- syntactically valid `TASK-...` canonical only in another project remains raw and is not a validated link;
- syntactically valid `SESSION-...` owned by another project remains raw and is not a validated link;
- unknown syntactically valid task/session remains raw only;
- `Model5`, `May2026`, `Mfoo7bar`, and prose-with-digits are not references;
- no title similarity or hidden `.hiveai` fallback is used;
- exact eight-project portfolio remains eight and includes `Sekiph82/FormuLab@main`;
- accepted X04 TASKS-only authority remains intact.

### C. Legacy `/agents` mounted route matrix

Mounted frontend tests must directly and separately prove:

- bare `/agents` -> `/prompts?surface=sessions` integrated surface;
- exact `projectId + sessionId` -> exact persisted session;
- project-only partial target fails closed;
- session-only partial target fails closed;
- duplicate `projectId` fails closed;
- duplicate `sessionId` fails closed;
- malformed ID fails closed;
- overlong ID fails closed;
- wrong-project persisted session fails closed;
- missing session fails closed;
- none of these paths dispatches or relaunches a provider;
- no top-level/legacy Claude readiness wall is rendered;
- Settings still exposes Builder Providers separately from Codex Audit Provider;
- accepted M14 polling/selection/output and M15 exact dispatch provenance remain green.

## F-M18-V05-003 — MAJOR — RESTORE A GENUINELY GREEN NORMAL FULL RUST GATE

### Current incorrect behavior

The V05 builder log records that the normal parallel:

```powershell
cargo test --lib
```

exposed three shared `task_intelligence` failpoint races. A later:

```powershell
cargo test --lib -- --test-threads=1
```

passed 514/514, but V05 explicitly required the normal non-ignored full Rust invocation to be green. Publication nevertheless proceeded.

### Required target behavior

1. Reproduce the parallel failures and record the exact failing test names and failure messages in the V06 log.
2. Isolate the actual shared test/failpoint interference.
3. Fix test isolation at the narrowest safe boundary.
4. Do not change accepted task-intelligence production semantics merely to make concurrency tests pass.
5. Do not globally serialize the full suite.
6. Do not mark tests ignored, weaken assertions, delete tests, add arbitrary sleeps, or hide failures behind retries.
7. If multiple tests share mutable test-only failpoints, use deterministic test-only synchronization/scoping so unrelated parallel tests cannot consume each other's failpoint state.
8. Preserve all accepted M09 parser behavior and existing direct retry/containment evidence.

### Required verification

The final acceptance evidence must include:

- focused affected task-intelligence tests under normal parallel execution;
- at least **two consecutive successful normal** `cargo test --lib` runs with no `--test-threads=1` override;
- no ignored-test increase;
- no production behavior weakening.

A serialized full-suite run may be recorded as supplemental diagnostic evidence but is not an acceptance substitute.

## FULL REQUIRED VERIFICATION

After all three findings are closed, run and record exact results for:

- focused M18 GitHub integration Rust tests;
- complete sanitizer/cache persistence/DTO matrix;
- complete Actions acquisition/log matrix;
- project-owned task/session linkage matrix;
- exact-eight-project / `Sekiph82/FormuLab@main` / X04 TASKS-only regressions;
- complete legacy `/agents` mounted route matrix;
- relevant M14/M15/M17 provider/session/provenance regressions;
- Local Git reconciliation regressions;
- focused task-intelligence failpoint isolation tests;
- **normal `cargo test --lib` twice consecutively**;
- full frontend suite;
- `npm run typecheck`;
- `npm run build`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `git diff --check`;
- final security/negative searches.

Check GitHub combined status/check/workflow availability for the final implementation commit and record the result without claiming hosted CI when none exists.

## SECURITY / NEGATIVE SEARCH GATES

Before completion, prove no accidental addition of:

- GitHub PAT/API-key settings;
- real plaintext credentials in source/tests/logs/fixtures;
- credential/auth-store reads;
- arbitrary frontend-controlled GitHub/API hosts;
- shell command-string construction;
- automatic push/merge/rebase/reset/checkout/tag/release/issue mutation/workflow retry;
- hidden `.hiveai` current-truth fallback;
- Codex edits to `TASKS.md` or `CODEX_ROADMAP.md`;
- M19 activation;
- standalone top-level Agents navigation/workspace.

Synthetic test secrets must be obviously fake and bounded.

## GOVERNED PUBLICATION

Do not publish until every required implementation, focused-test, full frontend, and **normal full Rust** gate is green.

Then use only:

`scripts/publish-dev-qa.ps1`

Preserve:

- `dev-bin/H!veAI.exe` stable path;
- Desktop `H!veAI.lnk` direct target;
- stable icon contract;
- no dev server;
- no visible terminal/console regression.

Record the final executable SHA-256.

Do not perform owner-native acceptance. Stop for independent V06 source re-audit first.

## REQUIRED BUILDER LOG

Create:

`docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V06_LOG.md`

Record at minimum:

- synchronized starting SHA;
- implementation/test SHA(s);
- exact changed files;
- disposition of F-M18-V05-001 through F-M18-V05-003;
- exact quoted-Authorization sanitizer behavior and end-to-end cache/DTO evidence;
- exact Actions production-path matrix results;
- exact project-owned linkage matrix results;
- exact legacy `/agents` route matrix results;
- exact initially reproduced parallel Rust failure names/messages/root cause;
- exact test-isolation remediation;
- two consecutive normal full Rust results;
- frontend/typecheck/build/cargo-check/diff/security results;
- GitHub hosted-status/check observation;
- governed publication result and executable SHA-256;
- final local HEAD / origin/main / live GitHub main equality and clean worktree.

Historical V04/V05 audit/log artifacts are immutable. Do not edit them.

## STOP CONDITION

At completion:

- commit and push all implementation/test/log changes;
- keep `TASKS.md` and `CODEX_ROADMAP.md` unchanged;
- keep M18 OPEN;
- keep M19 blocked;
- stop for independent strict V06 re-audit.

Do not claim M18 PASS/CLOSED and do not perform owner-native acceptance.

## OWNER-FACING FINAL RESPONSE

Return only:

- GitHub V06 log URL/path;
- implementation/test SHA(s);
- builder-log SHA;
- final GitHub main SHA;
- concise status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_V06_STRICT_REAUDIT`.
