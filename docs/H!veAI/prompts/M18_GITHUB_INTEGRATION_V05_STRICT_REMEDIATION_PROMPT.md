# M18 GitHub Integration V05 — Strict Residual Remediation Prompt

## AUTHORITY

Execute this prompt only after GitHub `main` contains:

- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V04_STRICT_REAUDIT.md` with verdict `FAIL — CHANGES_REQUIRED`;
- root `TASKS.md` showing M18 still open and V05 remediation as the current work;
- `CODEX_ROADMAP.md` showing M18 still open and M19 blocked.

This prompt closes only:

- F-M18-V04-001
- F-M18-V04-002
- F-M18-V04-003
- F-M18-V04-004
- F-M18-V04-005

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
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V03_STRICT_AUDIT.md`
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V04_STRICT_REAUDIT.md`
- `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V04_LOG.md`
- current M18 GitHub integration, Prompt Engine, session routing, Settings, and focused tests.

Preserve every accepted V04 improvement, including:

- real selected-PR enrichment from detail/files/reviews/comments/check/status resources;
- explicit per-resource PR truth state;
- Actions run -> jobs -> steps acquisition;
- verified-empty Actions versus unavailable/no-CI truth;
- the injectable production read-transport seam;
- strict dotted `M<number>[.<number>...]` grammar and rejection of `Model5`, `May2026`, and `Mfoo7bar`;
- nullable Prompt Engine `required_actor` handling with no invented actor;
- Prompt Engine Prompt Builder + Sessions surfaces;
- exact post-dispatch handoff with no redispatch;
- primary-navigation and command-palette removal of Agents;
- Builder Providers in Settings, separate from Codex Audit Provider;
- selected-provider readiness gating for Start;
- all accepted M00-M17, X03, and X04 behavior;
- exact eight-project portfolio;
- `Sekiph82/FormuLab@main`;
- root TASKS-only current-state authority;
- Local Git Engine read-only-by-default safety;
- Codex-only audit provider;
- Claude adapter;
- M14/M15 process/session/prompt provenance;
- governed native QA publication.

## F-M18-V04-001 — MAJOR — HARDEN SUCCESSFUL-PAYLOAD SANITIZATION AND CACHE VERSION SAFETY

### Current incorrect behavior

V04 introduced shared pre-persistence sanitization, but `sanitize_error()` is whitespace-token based. It can miss secrets when obvious credential key/value patterns occur inside a larger retained string, for example:

- `https://example.invalid/callback?access_token=TOPSECRET`
- `https://example.invalid/?token=TOPSECRET&x=1`
- `Authorization:Bearer TOPSECRET`
- punctuation-adjacent or JSON-derived retained text where the sensitive key/value is not the start of a whitespace token.

`CACHE_SCHEMA_VERSION` also remains the same version used by the earlier unsafe cache representation, so pre-V04 raw cache rows are not categorically invalidated by a schema/version boundary.

### Required target behavior

1. Use one deterministic bounded sanitizer/redactor for all retained GitHub text before persistence and before frontend DTO exposure.
2. Redact at minimum, wherever they appear with reasonable credential boundaries:
   - `Authorization:` values, with or without whitespace after the colon;
   - arbitrary `Bearer <value>` forms, including punctuation-adjacent/no-space header forms;
   - `ghp_...`, `github_pat_...`, `gho_...`, `ghu_...`, `ghs_...`, `ghr_...` token families;
   - `token=`, `access_token=`, `api_key=`, `apikey=`, and equivalent obvious credential key/value forms inside URL query strings, bodies, or retained excerpts;
   - representative quoted JSON/text forms.
3. Preserve harmless surrounding text where practical without retaining secret plaintext.
4. Redaction must stay bounded and deterministic. Do not add an unbounded regex/backtracking surface.
5. Bump the GitHub cache schema/representation version or otherwise provide an equally strong explicit incompatibility marker so all pre-safe V03/V04 cache rows fail closed and cannot be surfaced as current merely because a legacy payload happens to compare equal under the new sanitizer.
6. Prefer persisting minimal projected resource evidence instead of response-shaped raw JSON. If a bounded response-shaped projection remains, document exactly why it is safe and prove every retained string passes the sanitizer.
7. Do not read Git credentials, auth stores, provider credential files, or environment secrets.

### Required tests

Directly query `github_sync_state.metadata_json` and inspect the frontend-facing snapshot after injecting each representative secret form above. Assert the plaintext value is absent from both.

Add direct legacy-cache tests proving:

- a V03/V04 schema-1 unsafe row is rejected/fails closed;
- malformed legacy payloads are never surfaced as current;
- branch/repository/resource scoping remains intact;
- a newly sanitized current row still reloads successfully.

## F-M18-V04-002 — MAJOR — SELECT ACTIONS FAILURE LOGS BY JOB TRUTH

### Current incorrect behavior

V04 checks the workflow-run conclusion and then fetches logs for the first two job IDs. It does not filter those jobs by their own conclusion/status. Therefore a failed run may fetch a successful job's log, miss the actual failed job, and expose the wrong text as `failedLogSummary`.

### Required target behavior

1. Parse the bounded jobs resource first.
2. Select log candidates from individual jobs whose conclusion/state truthfully represents failure/attention, at minimum:
   - FAILURE
   - CANCELLED
   - TIMED_OUT
   - ACTION_REQUIRED
   - equivalent GitHub job-level failure state if the production response uses another documented spelling already supported by the implementation.
3. Do not fetch a successful/skipped job log merely because the overall run failed.
4. Use a deterministic bounded candidate rule. If only N failed-job logs may be fetched, select the first N failed/attention jobs in the already bounded GitHub job ordering, not the first N jobs overall.
5. Preserve job provenance for retained log evidence. At minimum the product must be able to identify which job generated the retained excerpt. A run-level aggregate is acceptable only if every excerpt is explicitly tied to an eligible job.
6. If an eligible job log is unavailable while jobs are current, preserve truthful partial/unavailable log state without fabricating a successful empty log.
7. A current failed run with no eligible failed-job records must not attach a successful job log as failure evidence.

### Required tests

Use the same production acquisition/enrichment path and exact request assertions for:

- successful job before failing job;
- two successful jobs before a third failing job;
- cancelled job;
- timed-out job;
- action-required job;
- skipped/success jobs excluded;
- first failed log unavailable but second available;
- current run metadata with jobs unavailable;
- failed run with no eligible failed-job records;
- deterministic log bounds and truncation.

## F-M18-V04-003 — MAJOR — SEPARATE RAW EXPLICIT REFERENCES FROM VALIDATED PROJECT-OWNED LINKS

### Current incorrect behavior

The lexical grammar is now strict, but every syntactically valid `TASK-...` or `SESSION-...` token is emitted as a task/session link even if it belongs to another project. The V04 adversarial test uses `other-project-7`, which is not a syntactically valid `TASK-...` or `SESSION-...` foreign identifier and therefore does not prove project isolation.

### Required target behavior

1. Keep strict lexical recognition for raw explicit references.
2. Do not present a raw reference as a validated project-owned link until ownership is proved.
3. TASK references:
   - validate against the selected project's canonical current task identity derived from root `TASKS.md` / tracked-branch root `TASKS.md` under accepted X04 authority;
   - do not use hidden `.hiveai`, Project Dashboard materialization, stale workflow rows, or title similarity as fallback current truth;
   - if a repository's canonical task syntax cannot be safely mapped to a raw token, retain it as a raw explicit reference rather than fabricating a link.
4. SESSION references:
   - validate against persisted agent sessions owned by the selected project;
   - reject a syntactically valid session ID that belongs to another project.
5. Milestone references may remain lexical references if they match the strict supported grammar, but do not claim task/session ownership from milestone syntax alone.
6. Update the DTO/UI naming if necessary so `rawExplicitReferences` and `validatedTaskLinks` / `validatedSessionLinks` cannot be confused.

### Required tests

Add direct positives and negatives covering:

- canonical task examples actually used by the governed eight-project portfolio;
- valid selected-project task reference;
- valid selected-project session reference;
- syntactically valid `TASK-...` belonging to another project;
- syntactically valid `SESSION-...` belonging to another project;
- unknown but syntactically valid task/session reference;
- `Model5`, `May2026`, `Mfoo7bar`, random prose with digits;
- no title-similarity inference;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` regression;
- X04 TASKS-only authority regression.

## F-M18-V04-004 — MAJOR — MAKE EVERY `/agents` ENTRY LEGACY-ONLY

### Current incorrect behavior

`src/App.tsx::LegacyAgentsRoute()` redirects targeted `/agents?projectId=...&sessionId=...` traffic through Prompt Engine, but bare `/agents` still renders `<Agents />` as a standalone top-level page. The non-embedded component also renders the old large Claude readiness card that the owner explicitly moved to Settings.

### Required target behavior

1. `/agents` must no longer render a standalone Agent Session Center for any query shape.
2. Every legacy `/agents` entry must resolve into `/prompts?surface=sessions` or render the integrated Prompt Engine Sessions surface directly with one canonical URL outcome.
3. Preserve valid bounded `projectId` + `sessionId` pairs exactly.
4. Bare `/agents` must land on Prompt Engine Sessions without a target.
5. Partial, duplicated, malformed, overlong, wrong-project, missing-session, or stale target parameters must fail closed inside the integrated Prompt Engine flow. Never launch or redispatch a provider as a side effect of route normalization.
6. The old non-embedded Claude readiness wall must not be reachable through a top-level route. The `Agents` implementation may remain as an internal embedded component if useful.
7. Primary navigation and command palette must continue to contain only Prompt Engine for this workflow.
8. New post-dispatch handoffs must continue to use the integrated route and preserve exact persisted session selection.

### Required tests

Mounted frontend tests must directly prove:

- bare `/agents` -> integrated Prompt Engine Sessions;
- `/agents?projectId=...&sessionId=...` -> exact integrated target;
- project-only partial legacy URL;
- session-only partial legacy URL;
- duplicated query values;
- malformed/overlong IDs;
- wrong-project persisted session;
- missing session;
- no prompt redispatch/provider relaunch during any redirect;
- no standalone `Claude Code readiness` panel on any `/agents` route;
- Settings still shows Builder Providers and separate Codex Audit Provider;
- M14 session polling/selection/output and M15 exact provenance remain green.

## F-M18-V04-005 — MINOR — ENFORCE REQUEST BUDGET BEFORE ACQUISITION

### Current incorrect behavior

Calls such as `push_enrichment(resources, load_or_fetch(...), budget)` execute `load_or_fetch()` before `budget.take()` because function arguments are evaluated before entering `push_enrichment()`. Current loop caps still bound total work, but `MAX_SUBRESOURCE_REQUESTS` is not the actual pre-request enforcement boundary it claims to be.

### Required target behavior

1. Reserve/consume the budget before any enrichment transport/cache acquisition.
2. When exhausted, emit a deterministic bounded resource state/error without calling the transport for that N+1 request.
3. Keep existing per-PR/per-run/per-files/per-comments/per-jobs/per-steps limits.
4. Do not increase current limits merely to make a test pass.

### Required tests

Use an instrumented transport with enough eligible PR/run subresources to reach the configured cap and prove:

- exactly the allowed number of subresource transport acquisitions occur;
- the N+1 transport path is never called;
- remaining resources are represented truthfully as bounded/unavailable/not-requested according to the chosen contract;
- base repository reads remain separately bounded and unaffected.

## WATCHER REGRESSION PRESERVATION

Do not weaken or revert the V04 watcher stabilization merely because it is outside the five residual findings. Preserve production watcher behavior and the bounded polling-based durable-state assertion. The full normal Rust library suite must remain genuinely green.

## SECURITY / NEGATIVE SEARCH GATES

Before completion, search the final source/diff for accidental additions of:

- GitHub PAT/API-key settings;
- plaintext credentials/tokens in source, tests, logs, or cache fixtures outside deliberately synthetic test secrets;
- credential-file/auth-store reads;
- arbitrary frontend-controlled GitHub/API hosts;
- shell command-string construction;
- automatic push/merge/rebase/reset/checkout/tag/release/issue mutation/workflow retry;
- hidden `.hiveai` current-truth fallback;
- Codex edits to `TASKS.md` or `CODEX_ROADMAP.md`;
- M19 activation;
- a standalone top-level Agents navigation/route surface.

## REQUIRED VERIFICATION

Run and record exact results for:

- focused M18 GitHub integration Rust tests through the shared production transport path;
- cache-version/sanitizer/persistence tests;
- Actions failed-job selection/log provenance tests;
- task/session reference ownership tests;
- exact eight-project / `Sekiph82/FormuLab@main` / X04 TASKS-only regressions;
- focused Prompt Engine / legacy Agents routing / Settings provider frontend tests;
- relevant M14/M15 session/provenance regressions;
- relevant M13-M17 provider/audit identity regressions;
- Local Git reconciliation tests;
- watcher regression including the previously unstable live-dashboard test;
- full frontend test suite;
- full Rust library suite under the normal non-ignored invocation;
- `npm run typecheck`;
- `npm run build`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `git diff --check`.

Check GitHub status/check availability on the final implementation commit and record it explicitly. Builder results remain claims until independently audited.

## GOVERNED NATIVE PUBLICATION

Publish only if all required implementation and regression gates above are green.

Use only the existing `scripts/publish-dev-qa.ps1` governed path. Preserve:

- stable `dev-bin/H!veAI.exe`;
- stable Desktop `H!veAI.lnk` target;
- stable icon contract;
- no dev-server dependency;
- no visible console/terminal regression.

Record the published executable SHA-256.

Do not perform owner-native acceptance yourself. Stop for independent V05 source re-audit first.

## REQUIRED BUILDER LOG

Create:

`docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V05_LOG.md`

Record at minimum:

- synchronized starting SHA;
- implementation/test commit SHA(s);
- exact files changed;
- disposition of F-M18-V04-001 through F-M18-V04-005;
- cache schema/version transition and legacy-row behavior;
- sanitizer pattern matrix and direct SQLite evidence;
- Actions failed-job candidate algorithm and log provenance model;
- raw-reference versus validated-link architecture and X04 authority source;
- complete `/agents` legacy-route normalization behavior;
- pre-acquisition request-budget proof;
- focused/full test commands and exact results;
- GitHub status/check availability;
- security/negative-search results;
- governed publication result and EXE SHA-256;
- explicit proof `TASKS.md` and `CODEX_ROADMAP.md` were not modified by Codex;
- final local HEAD / `origin/main` / live GitHub main equality;
- clean worktree proof.

## COMPLETION CONTRACT

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

- GitHub URL/path for `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V05_LOG.md`;
- implementation/test commit SHA(s);
- builder-log commit SHA;
- final GitHub `main` SHA;
- concise status.
