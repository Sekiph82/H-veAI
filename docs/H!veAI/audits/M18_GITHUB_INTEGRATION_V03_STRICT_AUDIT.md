# M18 GitHub Integration V03 — Independent Strict Audit

Date: 2026-09-15
Repository: `Sekiph82/H-veAI`
Branch: `main`
Implementation commit: `b02f575200d5f37660af932f72706272bf2f0052`
Builder-log commit: `210684e10e1dbe8bb2c4627fd980c72b30ccb6a9`
Base / M18 activation SHA: `2c18d00be7df94a1d03d503fa0bb595886f63028`

## 1. VERDICT

**CHANGES_REQUIRED**

Severity summary:

- BLOCKER: 0
- MAJOR: 5
- MINOR: 0
- NOTE: 0

M18 remains OPEN. M19 must remain blocked. Owner-native M18 acceptance must not begin yet.

The implementation establishes a useful bounded read-only GitHub surface, Registry-scoped identity, local/remote reconciliation, cache metadata, mutation denial, and a Project Cockpit GitHub tab. However, several acceptance-critical evidence fields shown by the product are not actually fetched by the production transport, raw remote payloads are persisted before sanitization, explicit task-link extraction can fabricate links, and the required full regression gate did not pass.

## 2. CONTRACT RECOVERY

Authoritative scope recovered from:

- `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`
- `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V01_PROMPT.md` for detailed M18.01-M18.09 requirements
- `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V03_PROMPT.md` for post-X04 execution authority and tracker-read-only rules
- accepted M17, X03, and X04 closure evidence
- root `TASKS.md` and `CODEX_ROADMAP.md` as read-only canonical tracker truth for the builder

Critical recovered rules include bounded project-scoped repository/branch/commit truth; PR diff/review/check/comment evidence; Actions jobs/steps and failed-log evidence; truthful no-CI versus unavailable-CI classification; bounded cache/offline/rate-limit behavior; no destructive local reconciliation; secret redaction before persistence/UI; evidence-backed task/session linkage; deterministic production-path tests; full Rust/frontend regression; governed publication only after gates pass; and no Codex-authored tracker transition.

## 3. BRANCH / HEAD / DIFF SCOPE

`2c18d00...210684e` is a linear two-commit range: implementation then builder log.

Implementation changes are bounded to:

- `src-tauri/src/github_integration.rs`
- `src-tauri/src/github_tracking.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/capabilities/default.json`
- `src-tauri/permissions/foundation.toml`
- `src/githubIntegration.ts`
- `src/pages.tsx`
- `tests/m12-project-cockpit-focused.test.tsx`
- the M18 V03 builder log

`TASKS.md` and `CODEX_ROADMAP.md` were not modified by the builder. M19 was not activated.

## 4. ACCEPTANCE CRITERIA MATRIX

- M18.01 repository / branch / commit reads: **PASS with bounded observations**.
- M18.02 pull requests: **FAIL**. Core list metadata exists, but production does not retrieve the required changed-file/diff, review/check, or bounded comment/review evidence it exposes in its DTO/UI.
- M18.03 issues: **PARTIAL**. Repository-scoped issue metadata/body excerpts work, but raw cache/security and linkage defects prevent acceptance.
- M18.04 Actions / CI: **FAIL**. Run metadata is fetched, but jobs/steps/log evidence is not fetched by production; no-CI versus unavailable-CI presentation is not trustworthy.
- M18.05 releases / tags: **PASS at source level**.
- M18.06 cache / rate limit / offline: **FAIL security boundary** because raw remote payloads are persisted before redaction/projection.
- M18.07 local / remote reconciliation: **PASS at source level** for observational reconciliation and dirty-state separation.
- M18.08 security / permission boundary: **FAIL** due raw-payload persistence and false-positive linkage extraction; remote mutation remains safely unavailable/default-denied.
- M18.09 UI / tests / publication: **FAIL** because the product renders fields unsupported by the production fetch topology and the mandatory full Rust suite did not pass.

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

Accepted as source truth:

- Registry-scoped owner/repository validation exists.
- fixed GitHub API host construction exists.
- mutation commands were not added; PR creation/workflow retry are reported unavailable.
- Tauri exposes one read-only GitHub integration command.
- Project Cockpit contains a GitHub tab.
- exact eight-project tracking remains present and `Sekiph82/FormuLab@main` remains intact.
- canonical tracker files were not edited by Codex.

Not accepted as independent truth:

- builder-reported test totals and publication success remain builder claims because the implementation commit has no GitHub status checks;
- the builder itself records that the full Rust library suite failed one watcher test twice;
- builder descriptions claiming PR diff/review/check/comment evidence and Actions job/step/log evidence exceed what the production fetch topology actually retrieves.

## 6. FILE / SYMBOL EVIDENCE

Primary implementation evidence is in `src-tauri/src/github_integration.rs`:

- `fetch_resources()` requests only the repository, branches, commits, pull-list, issue-list, workflow-run-list, releases, and tags endpoints.
- `parse_pull_requests()` expects fields including `changed_files`, `additions`, `deletions`, `review_status`, `check_status`, and an array-like `comments` value directly on pull-list items.
- `parse_actions()` expects embedded `jobs`, embedded job `steps`, and `failed_log_summary` / `log_summary` directly on workflow-run-list items.
- `load_or_fetch()` constructs a `CacheEnvelope` from the raw JSON `Value` and calls `persist_cache()` before field-level projection/redaction.
- `persist_cache()` serializes the complete cache envelope into `github_sync_state.metadata_json`.
- `sanitize_error()` is applied to HTTP error text, not successful remote payloads, and does not safely consume arbitrary bearer-token values.
- `explicit_links()` accepts any token starting with `M` that contains a digit, not only a strict milestone/task-ID grammar.

Frontend evidence in `src/pages.tsx` renders PR diff/review/check/comment fields and Actions jobs/steps/log summary from the DTO; when `actions` is empty it renders a no-CI message without consulting the Actions resource state.

## 7. FOCUSED TEST EVIDENCE

The focused M18 Rust tests validate parsers and DTO helpers against synthetic enriched JSON. In particular:

- the Actions test manually inserts `jobs`, `steps`, and `failed_log_summary` inside a workflow-run object even though the production workflow-run-list endpoint does not supply those nested values;
- the PR parser test does not prove production changed-file/review/check/comment acquisition;
- production `snapshot()` is compiled to `fixture_resources()` under `cfg(test)`, so the real `fetch_resources()` / `run_http()` topology is not exercised by the normal unit test path;
- the link test covers positive IDs but not adversarial non-task tokens such as `Model5`, `May2026`, or similar `M...digit` prose.

The single added mounted frontend test proves the GitHub tab renders a synthetic DTO and that mutation controls are absent. It does not prove production API-shape truth, per-resource unavailable/empty distinction, or redaction persistence.

## 8. REGRESSION EVIDENCE

Builder log claims:

- focused frontend: 10 passed;
- full frontend: 144 passed;
- typecheck/build/cargo check: passed;
- focused M18 Rust: 10 passed;
- governed publication: passed.

However, the mandatory full Rust command ran 505 tests and failed `watcher::tests::live_dashboard_contract_changes_reconcile_watcher_scope_without_restart` twice in the parallel suite. The test later passed in isolation. A flaky/non-deterministic full regression is still a failed required release gate until it is diagnosed and made stable or a production defect is fixed.

GitHub combined status for `b02f575...` contains no status checks, so no independent CI corroboration exists.

## 9. SECURITY / SAFETY REVIEW

Positive:

- no arbitrary frontend-provided host/endpoint exists;
- owner/repository/branch identity validation is bounded;
- no arbitrary shell string is built;
- remote mutations are not exposed;
- no automatic push/merge/rebase/reset/checkout/tag/release mutation was added;
- Local Git reconciliation is observational.

Failure:

Successful remote JSON is cached raw before sanitization. PR and issue bodies can therefore be persisted verbatim in SQLite. The same body-derived text is bounded for UI length but is not secret-redacted. This violates the explicit M18 rule that secrets/tokens must not be persisted or surfaced and that remote evidence must be sanitized/redacted before persistence/UI exposure.

## 10. ARCHITECTURE CONSISTENCY

The implementation correctly extends the existing Registry / `github_sync_state` / Local Git Engine architecture instead of creating a parallel project identity system. X04 root-TASKS authority is not replaced by the GitHub integration.

The main inconsistency is transport-to-model mismatch: the DTO models enriched PR and Actions evidence, but the production transport only fetches list endpoints. The architecture therefore presents a richer semantic contract than the transport can populate.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Builder correctly left `TASKS.md` and `CODEX_ROADMAP.md` untouched and did not activate M19.

The builder log is incomplete relative to the V01/V03 completion contract because it does not explicitly record GitHub status-check absence and it describes M18.02/M18.04 as implemented without acknowledging that production subresource acquisition is missing.

Canonical tracker transition after this audit belongs to ChatGPT, not Codex.

## 12. FINAL REPOSITORY STATE

At audit time GitHub `main` is `210684e10e1dbe8bb2c4627fd980c72b30ccb6a9`, the builder-log commit. The implementation and log commits are linear descendants of the M18 activation SHA.

No builder-authored canonical tracker edit is present.

## 13. OPEN CROSS-MILESTONE FINDINGS

No accepted M00-M17, X03, or X04 regression was found in the reviewed M18 diff.

The pre-existing watcher test instability observed by the builder must be stabilized before M18 can pass because M18's release gate explicitly requires the full Rust regression suite.

M19 remains blocked.

## 14. DEFECTS BY SEVERITY

### F-M18-V03-001 — MAJOR — Production PR evidence is not actually acquired

`fetch_resources()` requests only `/pulls?state=all&per_page=...`. Production does not follow each selected PR to detail/files/reviews/comments/check/status endpoints.

A live repository check against `Sekiph82/Bulk-Edit` confirms the list response for PR #145 contains relationship URLs such as `comments_url` and `statuses_url` but does not contain `changed_files`, `additions`, or `deletions`. The single-PR detail endpoint is a separate resource, and files/reviews/comments/checks are separate resources. The parser nevertheless reads those non-list fields as though they were embedded.

Result: the UI's Diff, Review/checks, and bounded comments/reviews surfaces are structurally `Unavailable`/empty in production even though the milestone requires that evidence where the GitHub REST transport supports it.

### F-M18-V03-002 — MAJOR — Production Actions jobs/steps/log evidence is not fetched, and unavailable CI can render as no CI

`fetch_resources()` requests only `/actions/runs?per_page=...`. `parse_actions()` expects embedded `jobs`, `steps`, and `failed_log_summary` values that are not part of the workflow-run-list payload.

A live `Sekiph82/Scrubbots` run proves the list payload exposes `jobs_url` and `logs_url`; the separate `/actions/runs/{run_id}/jobs` endpoint contains the actual job and step records. Production never follows those resources.

Additionally, the frontend renders `No CI runs in bounded window` whenever `data.actions.length == 0` and does not gate that message on the Actions resource state. Therefore an unavailable/offline/rate-limited Actions request with no cached value is visually indistinguishable from a verified current empty CI list.

### F-M18-V03-003 — MAJOR — Raw successful remote payloads are persisted and surfaced before secret redaction

`load_or_fetch()` stores the successful raw JSON `Value` inside `CacheEnvelope`; `persist_cache()` serializes the entire envelope into SQLite before any safe field projection. `sanitize_error()` only sanitizes error text. `bound_text()` truncates UI strings but does not redact them.

This permits accidentally committed/published token-like strings in issue/PR bodies or other returned fields to be retained in `github_sync_state.metadata_json` and potentially surfaced. Generic `Authorization: Bearer <value>` handling is also incomplete because the token following `Bearer` is not generically consumed/redacted unless it happens to match one of the few recognized prefixes.

### F-M18-V03-004 — MAJOR — Explicit task-link extraction can fabricate links from ordinary prose

`explicit_links()` treats any token beginning with `M` and containing at least one digit as an explicit task/milestone link. Ordinary tokens such as `Model5`, `May2026`, or similar prose can therefore become `taskLinks` merely because they occur in a PR or issue title/body.

This violates M18's evidence-backed/no-title-guess linkage rule. Link recognition must use a strict accepted ID grammar and, where practical, validate candidate task IDs against canonical project task identity rather than broad lexical coincidence.

### F-M18-V03-005 — MAJOR — Required production-path test matrix and full regression gate are incomplete

The test architecture swaps production fetching out under `cfg(test)` and uses synthetic enriched payloads, which means the exact production endpoint topology that causes F-001/F-002 is not exercised. Required deterministic cases for real list-vs-detail acquisition, no-CI-vs-unavailable, successful-payload redaction persistence, strict negative task-link parsing, and production timeout/response behavior are missing.

Separately, the mandatory full Rust library suite did not pass. The builder published anyway, despite the prompt's `if and only if implementation/regression gates pass` publication rule.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Non-blocking architecture recommendation for the remediation: introduce one injectable GitHub read transport used by both production and deterministic tests instead of replacing all production fetch behavior with `fixture_resources()` under `cfg(test)`. This would make endpoint sequencing, response-size handling, failures, and subresource enrichment directly testable without real network mutation.

Prefer cache entries containing minimal sanitized projected evidence rather than complete raw GitHub responses.

## 16. UNVERIFIED ITEMS

- Builder-reported executable SHA-256 and native smoke/publication were not independently reproduced by this source audit.
- There are no GitHub status checks on the implementation commit.
- Private-repository authenticated behavior remains unverified; public unauthenticated reads are allowed by the plan, while mutations remain safely unavailable.
- No owner-native M18 visual acceptance was run because source acceptance failed first.

## 17. REGRESSION RISK

**MEDIUM-HIGH** until remediation.

The new surface is read-only, which limits destructive risk. However, it can present incomplete PR/CI evidence as though the integration contract exists, can misclassify unavailable CI as empty, and persists raw remote payloads. These are trust/security defects in an evidence-first application.

## 18. AUDIT CONFIDENCE

**HIGH**.

Confidence is based on the exact implementation diff, direct source inspection, focused test bodies, live GitHub API response shapes from repositories in the governed eight-project portfolio, current branch state, and independent combined-status inspection.

## 19. FINAL VERDICT

**CHANGES_REQUIRED**

M18 V03 is not eligible for owner-native acceptance or milestone closure. M19 remains blocked.

## 20. REQUIRED REMEDIATION

Create one bounded M18 V04 remediation that closes only F-M18-V03-001 through F-M18-V03-005 while preserving all accepted M00-M17, X03, X04, exact-eight-project, FormuLab-main, Codex-audit-provider, Claude-adapter, and tracker-read-only behavior.

Required V04 outcomes:

1. Implement bounded production PR enrichment for selected PRs using the necessary detail/files/reviews/comments/check/status resources, with explicit per-subresource unavailable/stale truth rather than fabricated empty evidence.
2. Implement bounded production Actions enrichment for selected runs/jobs/steps and failed-log evidence where supported, and make UI distinguish verified no-CI from CI unavailable/stale/rate-limited/offline.
3. Redact/sanitize successful remote payloads before persistence and UI. Prefer persisted minimal projected evidence; prove SQLite cache and returned DTO never retain representative GitHub token/bearer/authorization secrets.
4. Replace broad `M...digit` linkage with strict deterministic ID recognition plus adversarial negative tests.
5. Replace or refactor the test-only transport seam so deterministic tests execute the same endpoint-selection/enrichment logic as production. Complete the required M18 matrix and obtain a genuinely green full Rust regression. Diagnose and stabilize the watcher-suite failure without weakening prior accepted watcher behavior.
6. Re-run frontend/Rust/typecheck/build/security/diff gates and governed publication only after all required gates pass.
7. Codex must not edit `TASKS.md` or `CODEX_ROADMAP.md`; ChatGPT owns the audit-driven tracker transition. Do not activate M19.
