# M18 GitHub Integration V04 — Independent Strict Re-Audit

Date: 2026-09-15
Repository: `Sekiph82/H-veAI`
Branch: `main`
V04 implementation commit: `3469c7399a1a5f98593f7f731070d83b2a67a692`
V04 builder-log commit: `3f05257e5c1d1b495120ae741834ac0623bde630`
V04 synchronized base / owner-scope SHA: `a6520fb11a02fd14d06cb21f0148dfeec7d99ef1`

## 1. VERDICT

**FAIL — CHANGES_REQUIRED**

Severity summary:

- BLOCKER: 0
- MAJOR: 4
- MINOR: 1
- NOTE: 0

M18 remains OPEN. M19 remains blocked. Owner-native M18 acceptance must not begin yet.

V04 substantially improves the V03 implementation. Production PR subresource acquisition is now real, Actions jobs are fetched, verified-empty CI is separated from unavailable CI, successful JSON is passed through a shared sanitizer before cache/UI projection, the broad `M...digit` grammar is replaced, the real acquisition path has an injectable transport seam, the nullable Prompt Engine `required_actor` failure is fixed, Prompt Engine now contains an embedded Sessions surface, primary Agents navigation is removed, and builder readiness is moved into Settings.

However, source inspection finds four acceptance-critical residual defects: successful-payload redaction can still miss secrets embedded in URLs or no-space Authorization forms and legacy cache schema 1 is still trusted; failed Actions log selection is based on the run conclusion rather than the selected job conclusion; syntactically valid cross-project `TASK-...` / `SESSION-...` references are still presented as links without project ownership validation; and bare `/agents` still renders the standalone Agent Session Center, including the old Claude readiness wall. The nominal subresource request budget is also consumed after the request has already happened.

## 2. CONTRACT RECOVERY

The audited V04 contract is recovered from:

- `AGENTS.md` strict-audit governance;
- `CONSTITUTION.md` and `ARCHITECTURE.md`;
- root `TASKS.md` and `CODEX_ROADMAP.md`;
- `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`;
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V03_STRICT_AUDIT.md`;
- `docs/H!veAI/plans/M18_V04_OWNER_PROMPT_AGENT_UX_CONSOLIDATION_PLAN.md`;
- `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V04_STRICT_REMEDIATION_PROMPT.md`;
- `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V04_LOG.md` as builder claims only;
- the V04 implementation diff and focused tests.

The V04 contract required closure of F-M18-V03-001 through F-M18-V03-005 plus owner-directed M18.10.01 through M18.10.05. Critical rules include real bounded PR and Actions enrichment; verified empty versus unavailable truth; sanitization before persistence and frontend projection; backward-safe cache handling; strict evidence-backed task/session linkage including adversarial cross-project negatives; deterministic production-path tests; genuinely green regression before publication; one top-level Prompt Engine workflow with Sessions inside it; all legacy `/agents` entry forms resolving into that integrated workflow; provider readiness in Settings; nullable `required_actor`; and preservation of accepted M00-M17, X03, X04, exact-eight-project, `Sekiph82/FormuLab@main`, Codex-audit, Claude-adapter, and TASKS-only authority contracts.

## 3. BRANCH / HEAD / DIFF SCOPE

The V04 implementation is one linear commit from `a6520fb...` to `3469c739...`, followed by the immutable builder log at `3f05257...`.

Implementation/test files changed:

- `src-tauri/src/github_integration.rs`
- `src-tauri/src/prompt_engine.rs`
- `src-tauri/src/watcher.rs`
- `src/App.tsx`
- `src/PromptEnginePage.tsx`
- `src/agentNavigation.ts`
- `src/components/Shell.tsx`
- `src/githubIntegration.ts`
- `src/pages.tsx`
- `src/styles.css`
- `tests/m15c-post-dispatch-handoff-focused.test.tsx`

The builder did not modify `TASKS.md` or `CODEX_ROADMAP.md`. GitHub `main` at audit time is the builder-log commit `3f05257e5c1d1b495120ae741834ac0623bde630`.

## 4. ACCEPTANCE CRITERIA MATRIX

- F-M18-V03-001, real production PR evidence: **PASS**. Selected PRs are enriched from detail/files/reviews/issue-comments/review-comments/check-runs/status endpoints with per-resource state.
- F-M18-V03-002, Actions jobs/steps/logs and empty-vs-unavailable truth: **PARTIAL / FAIL**. Jobs/steps and resource-state UI are real, but failed-log selection/attribution can select successful jobs and miss the actual failed job.
- F-M18-V03-003, sanitize before persistence/UI: **PARTIAL / FAIL**. A shared sanitizer now runs before persistence/projection, but it misses embedded query credentials and no-space Authorization patterns, cache schema remains version 1, and legacy raw rows are not categorically invalidated.
- F-M18-V03-004, strict explicit linkage: **PARTIAL / FAIL**. Broad `Model5`-style false positives are fixed, but valid-looking `TASK-...` and `SESSION-...` references are not validated against the selected project and the required cross-project adversarial case is not tested.
- F-M18-V03-005, production-path tests/full regression: **PARTIAL**. The real acquisition logic now uses an injectable transport seam and V04 adds useful endpoint-path tests; hosted checks are absent, and residual cases above remain outside the test matrix. Builder-reported 509 Rust / 144 frontend passing results remain claims rather than independent hosted evidence.
- M18.10.01, Prompt Engine/Agents consolidation: **PARTIAL / FAIL**. Primary navigation is consolidated and Prompt Engine embeds Sessions, but bare `/agents` still renders the standalone Agents page.
- M18.10.02, backward-compatible `/agents` deep links: **PARTIAL / FAIL**. Exact query deep links redirect safely, but the route itself remains a competing standalone workspace when no target parameters are supplied.
- M18.10.03, readiness in Settings and Start gating: **PASS at source level**. Builder Providers shows Codex and Claude separately from Codex Audit Provider; session Start remains gated by selected provider availability.
- M18.10.04, nullable `required_actor`: **PASS at source level**. SQLite decoding now uses `Option<String>` and preserves JSON null; a direct DB-backed regression exists.
- M18.10.05, focused regression/native gate: **PARTIAL**. Focused handoff coverage exists, but it does not test bare `/agents`; owner-native acceptance correctly has not occurred yet.

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

Builder claims that all F-M18-V03-001 through F-M18-V03-005 and M18.10.01 through M18.10.05 are closed.

Repository truth supports several of those claims:

- real PR endpoint enrichment exists;
- no-CI rendering now consults `GITHUB_ACTIONS` resource state;
- an injectable read transport is used by production acquisition logic and deterministic tests;
- strict dotted milestone grammar rejects the V03 `Model5` / `May2026` / `Mfoo7bar` examples;
- `required_actor` is nullable end to end;
- sidebar/command-palette Agents entry is removed;
- Prompt Engine has Prompt Builder and Sessions surfaces;
- Settings has a distinct Builder Providers section;
- new handoffs target `/prompts?surface=sessions...`.

Repository truth does not support complete closure for sanitizer/cache safety, failed-job log attribution, cross-project link semantics, or the bare `/agents` route. The builder log also describes logs as fetched only for failed/cancelled/timed-out/action-required jobs, but the implementation filters on the run conclusion and then takes the first two job IDs without checking each job conclusion.

## 6. FILE / SYMBOL EVIDENCE

### GitHub integration

`src-tauri/src/github_integration.rs` now defines explicit PR/Actions enrichment limits and the `GitHubReadTransport` seam. `fetch_resources_with_transport()` follows the expected PR detail/files/reviews/comments/check/status resources and Actions jobs/log resources.

The residual security problem is in `sanitize_error()` / `sanitize_value()` / `load_cache()`:

- secret detection is whitespace-token based;
- `token=`, `access_token=`, `api_key=`, and `apikey=` are redacted only when the whitespace token starts with that key;
- therefore strings such as `https://example.invalid/callback?access_token=TOPSECRET` can retain the secret;
- `Authorization:Bearer TOPSECRET` does not set the pending-bearer state for the following token, so `TOPSECRET` can remain;
- `CACHE_SCHEMA_VERSION` is still `1`, the same version used by the unsafe V03 cache representation;
- cache rows are accepted if re-sanitizing the payload produces no difference, so there is no hard schema/version invalidation of pre-V04 raw rows;
- the cache still stores a sanitized bounded response-shaped `Value`, not a deliberately minimal projected resource DTO.

The residual Actions defect is in `fetch_resources_with_transport()` and `enrich_actions()`:

- a failed/cancelled/timed-out/action-required **run** causes `parse_job_ids(...).take(2)` to fetch logs for the first two jobs;
- the individual job conclusion is not examined before log fetch;
- if job 1 succeeds and job 3 fails, job 1 may be fetched while the actual failed job is missed;
- `enrich_actions()` can then assign the first available fetched job log to `failed_log_summary` regardless of that job's conclusion.

The residual linkage problem is in `explicit_links()` plus PR/issue projection:

- grammar validation is now strict enough to reject arbitrary `M...digit` prose;
- any syntactically valid `TASK-...` or `SESSION-...` is nevertheless emitted without checking canonical selected-project task identity or persisted session ownership;
- therefore a valid identifier belonging to another project can still be presented by the UI as an explicit task/session link.

The nominal request budget also does not guard acquisition itself. Calls are written as `push_enrichment(resources, load_or_fetch(...), budget)`. Rust evaluates `load_or_fetch(...)` before `push_enrichment()` calls `budget.take()`, so the budget controls retention/state replacement after the request rather than preventing the request. Current static loop caps still bound the snapshot, which keeps this at MINOR severity.

### Prompt/session UX

`src/components/Shell.tsx` correctly removes Agents from primary navigation and the command palette. `src/PromptEnginePage.tsx` correctly embeds `Agents` in a Sessions surface and redirects targeted legacy `/agents?...` URLs to `/prompts?surface=sessions...`. `src/agentNavigation.ts` correctly generates the integrated Prompt Engine target.

However, `src/App.tsx::LegacyAgentsRoute()` returns `<Agents />` whenever neither `projectId` nor `sessionId` is present. This preserves a standalone Agent Session Center at bare `/agents` instead of making `/agents` uniformly legacy-only. Since `Agents` renders the old large Claude readiness card when `embedded == false`, direct navigation to bare `/agents` also resurrects the exact capability wall the owner directed to move to Settings.

`src-tauri/src/prompt_engine.rs::collect_context()` correctly models `required_actor` as `Option<String>` and preserves null instead of inventing `HUMAN`.

## 7. FOCUSED TEST EVIDENCE

Useful V04 evidence exists:

- `production_transport_seam_enriches_exact_pr_and_actions_paths` proves concrete PR and Actions request paths through the shared acquisition logic;
- `successful_remote_payloads_are_redacted_before_cache_and_frontend_projection` directly queries `github_sync_state.metadata_json` and checks representative whitespace-separated secrets;
- `strict_link_grammar_rejects_prose_and_accepts_canonical_refs` covers `Model5`, `May2026`, `Mfoo7bar`, dotted M IDs, `TASK-42`, and `SESSION-abc`;
- `nullable_required_actor_is_preserved_through_context_and_generation` covers a real NULL task row;
- `tests/m15c-post-dispatch-handoff-focused.test.tsx` covers targeted legacy `/agents?...`, exact session selection, no redispatch, wrong-project target rejection, and polling.

Missing/adversarial evidence remains:

- sanitizer tests for embedded URL/query/body values and no-space Authorization forms;
- hard invalidation/migration tests for old unsafe cache schema rows;
- Actions mixed-job ordering where a successful job precedes the failing job or failure lies after the first two jobs;
- a syntactically valid `TASK-...` / `SESSION-...` owned by another project;
- canonical positive identifiers from governed portfolio task truth;
- bare `/agents`, partial `/agents?projectId=...`, malformed legacy routes, and proof that no standalone readiness wall can be reached;
- a test proving the request budget blocks transport invocation before the N+1 request rather than after it.

## 8. REGRESSION EVIDENCE

The builder log reports:

- Rust library suite: 509 passed, 0 failed, 0 ignored;
- frontend: 18 files / 144 tests passed;
- typecheck passed;
- frontend build passed;
- governed QA publication passed;
- published EXE SHA-256 `8888A2724BFB2138BB8AC0CC09C9AC68246E2D012D931EE1AF1C09595326F4C4`.

Source inspection shows the former watcher timing assertion was changed toward polling durable state rather than weakening production watcher logic. That is directionally appropriate.

GitHub combined status for implementation commit `3469c739...` contains no hosted statuses/checks. The test totals and executable publication therefore remain builder claims, not independently reproduced audit evidence. A passing suite would not override the direct source defects above in any case.

## 9. SECURITY / SAFETY REVIEW

Positive:

- no GitHub PAT/API-key setting was introduced;
- fixed GitHub host construction remains;
- no arbitrary frontend-controlled API host is exposed;
- remote mutations remain denied/unavailable;
- no automatic push/merge/rebase/reset/checkout/release/issue/workflow mutation is added;
- provider credential files are not read;
- successful payloads now pass through a sanitizer before persistence/UI, which is a major improvement over V03.

Failure:

The sanitizer is not context-complete enough for the contract. Embedded credential key/value forms and no-space Authorization variants can evade it, and legacy schema-1 cache rows are not hard-invalidated. This remains a persistence/security boundary defect.

## 10. ARCHITECTURE CONSISTENCY

The V04 architecture is mostly consistent with H!veAI:

- Registry remains project identity authority;
- X04 TASKS-only current truth remains intact;
- GitHub integration remains observational;
- Local Git Engine remains read-only by default;
- M14 session backend is reused rather than rewritten;
- M15 immutable prompt/session provenance remains intact;
- Codex audit readiness stays separate from builder/session readiness;
- Prompt Engine is moving toward the intended single workflow.

The bare `/agents` exception is the primary information-architecture inconsistency. The cross-project reference model also conflates lexical explicit references with validated project-owned links.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

`TASKS.md` and `CODEX_ROADMAP.md` were not modified by Codex, consistent with tracker-ownership governance. M19 was not activated.

The V04 builder log is useful and records endpoint topology, bounds, test claims, publication, and sync evidence. Its closure statements are nevertheless too strong for F-M18-V03-002, F-M18-V03-003, F-M18-V03-004, and M18.10.01/02 because the residual source defects above remain.

Canonical tracker transition after this re-audit belongs to ChatGPT.

## 12. FINAL REPOSITORY STATE

At audit time GitHub `main` is `3f05257e5c1d1b495120ae741834ac0623bde630`, whose parent is V04 implementation `3469c7399a1a5f98593f7f731070d83b2a67a692`.

The implementation range is linear. The builder log is committed and visible. No builder-authored canonical tracker edits are present. No registered external project repository is changed by the V04 implementation commit.

## 13. OPEN CROSS-MILESTONE FINDINGS

No evidence was found that reopens accepted M13-M17, X03, or X04 backend contracts. The M14/M15 session/provenance backend appears preserved.

The user-directed M18.10 consolidation remains incomplete only at the presentation/routing boundary because bare `/agents` still exposes a standalone view. M19 remains blocked.

## 14. DEFECTS BY SEVERITY

### F-M18-V04-001 — MAJOR — Sanitization/cache compatibility still permits secret persistence

`sanitize_error()` is whitespace-token based and does not redact credential key/value material embedded later in a URL/string, such as `...?access_token=TOPSECRET` or `...?token=TOPSECRET`. A no-space form such as `Authorization:Bearer TOPSECRET` can also leave the following secret token intact. `CACHE_SCHEMA_VERSION` remains 1, so V03-era raw cache rows are not categorically rejected by a version boundary. V04's persistence test does not cover these cases.

Required result: one deterministic shared sanitizer must redact supported credential patterns wherever they occur in retained text, before persistence and before DTO exposure; the cache format/version must make all pre-safe rows fail closed or migrate them through a verified safe projection; direct DB tests must prove plaintext absence for whitespace, URL/query, punctuation-adjacent, no-space Authorization/Bearer, and GitHub token-family cases.

### F-M18-V04-002 — MAJOR — Actions failed-log acquisition is run-filtered, not failed-job-filtered

A failed run causes logs for the first two job IDs to be fetched without checking those jobs' conclusions. This can fetch a successful job log, miss the actual failed job, and present the wrong text as `failedLogSummary`.

Required result: derive deterministic bounded log candidates from the fetched job records themselves; only failed/cancelled/timed-out/action-required jobs may contribute failure-log evidence; preserve job identity/provenance in the summary model; test success-before-failure, failure-after-two-successes, cancelled/timed-out/action-required cases, partial log failures, and the no-failed-job case.

### F-M18-V04-003 — MAJOR — Valid-looking cross-project task/session references are not ownership-validated

The lexical grammar is now strict, but any valid `TASK-...` or `SESSION-...` is emitted as a product link without selected-project validation. The required adversarial negative for another project's syntactically valid task/session identifier is absent.

Required result: separate raw explicit references from validated project-owned links, or validate before labeling them links. TASK validation must respect X04 TASKS-only authority; SESSION validation must use persisted selected-project session ownership. Add valid foreign TASK/SESSION negatives plus canonical portfolio positives. Never revive hidden `.hiveai` current truth.

### F-M18-V04-004 — MAJOR — Bare `/agents` still exposes the standalone Agent Session Center

`LegacyAgentsRoute()` redirects only when `projectId` or `sessionId` exists; otherwise it returns `<Agents />`. This contradicts the owner-directed one-workspace consolidation and makes the old standalone Claude readiness card directly reachable.

Required result: every `/agents` form is legacy-only and routes into `/prompts?surface=sessions`, preserving valid bounded parameters when present and failing malformed/partial targets safely inside the integrated surface. The standalone `<Agents />` must not be a top-level route. The component may remain as an internal embedded implementation detail.

### F-M18-V04-005 — MINOR — Request budget is consumed after acquisition

`push_enrichment(resources, load_or_fetch(...), budget)` evaluates `load_or_fetch` before `budget.take()`. The current static loop caps still keep snapshots bounded, so this is not presently an unbounded-request defect, but the advertised total-subresource budget does not enforce what its name/contract claims.

Required result: reserve budget before any network/cache acquisition and prove the N+1 transport call does not occur after the cap is exhausted.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Prefer a versioned projected cache schema that persists only fields H!veAI actually needs rather than a sanitized response-shaped JSON tree. This reduces secret-retention surface, makes backward compatibility explicit, and makes per-resource schema validation easier.

For links, model `rawReferences` and `validatedLinks` separately if canonical task formats vary by repository. This keeps lexical evidence useful without promoting it to ownership truth.

For Actions, attach bounded log excerpts to explicit job IDs/names instead of a single run-level string where practical.

## 16. UNVERIFIED ITEMS

- Builder-reported 509 Rust tests, 144 frontend tests, typecheck/build success, native publication, and executable SHA-256 were not independently executed by this GitHub source audit.
- No GitHub hosted status/check evidence exists on implementation commit `3469c739...`.
- Native visual behavior of the new Prompt Builder/Sessions switcher and Settings provider cards remains pending because source acceptance failed.
- Private/authenticated GitHub repository behavior remains unverified; remote mutation remains unavailable by design.

## 17. REGRESSION RISK

**MEDIUM**.

The V04 implementation is read-only on GitHub and preserves the established provider/session backend, so destructive risk is low. Remaining risk is evidence trust and UX truth: secret-like text can still evade persistence redaction, failure logs can be misattributed, foreign IDs can be presented as links, and a hidden duplicate Agents workspace remains reachable.

## 18. AUDIT CONFIDENCE

**HIGH**.

Confidence comes from the exact V04 diff, direct production-symbol inspection, direct focused-test inspection, independent GitHub-main/commit-status inspection, and comparison against the V03 audit plus the owner-directed M18.10 plan. The findings are source-structural and do not depend on reproducing builder-reported test runs.

## 19. FINAL VERDICT

**FAIL — CHANGES_REQUIRED**

M18 V04 is not eligible for owner-native acceptance or closure. M19 remains blocked. V05 must be a bounded remediation of F-M18-V04-001 through F-M18-V04-005 only, preserving all V04 improvements that are already correct.

## 20. REQUIRED REMEDIATION

Create M18 V05 with no scope expansion and no M19 activation. It must:

1. harden successful-payload sanitization and cache-version safety so representative secrets cannot persist or surface, including embedded query/value and no-space Authorization/Bearer forms;
2. select Actions failure-log candidates from actual failed/cancelled/timed-out/action-required job records and preserve truthful job provenance;
3. validate syntactically valid TASK/SESSION references against selected-project authority/ownership before presenting them as links, with valid cross-project negatives and governed portfolio positives;
4. make all `/agents` forms legacy-only aliases into Prompt Engine Sessions and make the standalone Agent Session Center unreachable as a top-level page;
5. enforce the subresource request budget before transport/cache acquisition;
6. add direct adversarial tests for every residual finding, then rerun focused/full Rust/frontend/typecheck/build/security/diff gates and governed publication;
7. keep `TASKS.md` and `CODEX_ROADMAP.md` read-only for Codex, preserve exact eight-project / `Sekiph82/FormuLab@main`, X04 TASKS-only authority, Codex-only audit provider, Claude adapter, M14/M15 provenance, and all accepted M00-M17 behavior;
8. stop after commit/push/log and independent source re-audit. Do not run owner-native acceptance and do not activate M19.
