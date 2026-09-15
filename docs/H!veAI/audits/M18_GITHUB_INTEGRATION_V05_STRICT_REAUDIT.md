# M18 GitHub Integration V05 — Independent Strict Re-Audit

Date: 2026-09-15
Repository: `Sekiph82/H-veAI`
Branch: `main`
V05 authority/base SHA: `868e9215351c842af3d447c41cda95284de44713`
Implementation/test SHA: `6291d7a815cc53e62018920ae2dc2cc7aac4e842`
Builder-log SHA: `aa081fa9775d5000928f25488df96740aeb97935`

## 1. VERDICT

**FAIL — CHANGES_REQUIRED**

Severity summary:

- BLOCKER: 0
- MAJOR: 3
- MINOR: 0
- NOTE: 0

M18 remains OPEN. M19 must remain blocked. Owner-native M18 acceptance must not begin yet.

V05 materially improves the V04 implementation: cache schema invalidation, job-level Actions failure selection, project-owned task/session link validation, legacy `/agents` consolidation, and pre-acquisition request-budget enforcement are all present in production source. However, one successful-payload redaction hole remains, the exact V05 adversarial/production-path test matrix is incomplete, and the mandatory normal full Rust regression gate did not pass before governed publication.

## 2. CONTRACT RECOVERY

Authoritative V05 scope was recovered from:

- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V04_STRICT_REAUDIT.md`
- `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V05_STRICT_REMEDIATION_PROMPT.md`
- `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`
- `docs/H!veAI/plans/M18_V04_OWNER_PROMPT_AGENT_UX_CONSOLIDATION_PLAN.md`
- `AGENTS.md`
- accepted M00-M17 and X03/X04 contracts
- root `TASKS.md` and `CODEX_ROADMAP.md` as canonical tracker truth, read-only for Codex

V05 was limited to F-M18-V04-001 through F-M18-V04-005 and explicitly required: robust successful-payload redaction plus cache-version invalidation; job-truth Actions log acquisition and provenance; selected-project task/session ownership validation; every `/agents` entry becoming legacy-only; request-budget reservation before acquisition; the enumerated direct production-path/adversarial tests; a genuinely green normal non-ignored full Rust suite; and publication only after every required gate was green.

## 3. BRANCH / HEAD / DIFF SCOPE

The audited range `868e921...aa081fa` is linear and two commits ahead of the V05 authority SHA:

1. `6291d7a815cc53e62018920ae2dc2cc7aac4e842` — implementation/tests
2. `aa081fa9775d5000928f25488df96740aeb97935` — builder log

Changed implementation/test files are bounded to:

- `src-tauri/src/github_integration.rs`
- `src/App.tsx`
- `src/githubIntegration.ts`
- `src/pages.tsx`
- `tests/m14-agent-session-center-focused.test.tsx`
- `tests/m15c-post-dispatch-handoff-focused.test.tsx`
- `tests/m17-claude-adapter-focused.test.tsx`
- V05 builder log

`TASKS.md` and `CODEX_ROADMAP.md` were not edited by Codex. No M19 activation is present.

## 4. ACCEPTANCE CRITERIA MATRIX

- F-M18-V04-001 sanitizer/cache safety: **FAIL**. Cache schema 2 and pre-persistence sanitization are real, but quoted `Authorization` text can still retain a non-Bearer secret in string-valued GitHub content.
- F-M18-V04-002 job-truth Actions failure logs: **PARTIAL / source implementation substantially PASS, required direct matrix FAIL**. Production now filters job records before log acquisition and preserves job provenance, but the exact required production-path matrix is not present.
- F-M18-V04-003 project-owned task/session links: **PASS at source level**. Raw references are separated from validated links; current GitHub TASKS cache and persisted project-owned sessions are used as ownership evidence.
- F-M18-V04-004 every `/agents` entry legacy-only: **PASS at source level**. `/agents` always mounts Prompt Engine legacy routing and redirects to integrated Sessions; embedded sessions suppress the old readiness wall.
- F-M18-V04-005 pre-acquisition request budget: **PASS at source level**. `acquire_enrichment*` consumes the request budget before cache/transport acquisition and emits a bounded resource without an N+1 call when exhausted.
- Mandatory full normal Rust regression: **FAIL**. Builder log records that the normal parallel `cargo test --lib` invocation exposed three task-intelligence failpoint races. A later serialized `--test-threads=1` pass is not the required normal gate.
- Governed publication gate: **FAIL** because publication occurred after the required normal full Rust gate had failed.

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

Confirmed from source:

- `CACHE_SCHEMA_VERSION` advanced from 1 to 2.
- legacy schema-1 cache rows are rejected.
- successful JSON/text resources are sanitized before persistence in the production load/fetch path.
- Actions failed-log acquisition is now based on parsed job failure truth rather than overall-run failure alone.
- retained Action log excerpts include job ID/name provenance.
- task/session raw references are distinct from validated selected-project links.
- canonical GitHub TASKS cache must be `CURRENT` before task links validate.
- session links must exist in `agent_sessions` for the selected project.
- bare and targeted `/agents` traffic routes through Prompt Engine Sessions.
- request budget is consumed before enrichment acquisition.

Not accepted as independent proof:

- builder-reported test totals and native publication remain builder claims; GitHub combined status for the implementation commit contains zero statuses and no workflow runs were found for that commit.
- the builder's statement that F-M18-V04-001 is fully closed is contradicted by the retained-text sanitizer logic described below.
- the builder's serialized Rust pass does not replace the explicitly required normal full-suite invocation, which the builder records as failed.

## 6. FILE / SYMBOL EVIDENCE

### Sanitization

`src-tauri/src/github_integration.rs::sanitize_error()` scans retained strings with `authorization_span`, `credential_assignment_span`, `bearer_span`, and GitHub token-family recognition.

`credential_assignment_span()` explicitly tolerates a closing quote after a key before `:`/`=`, which is necessary for JSON-looking retained text. `authorization_span()` does not. It requires the byte immediately after optional whitespace following `authorization` to be `:`.

Therefore a string value such as:

`{"Authorization":"Basic QUOTED_AUTH_SECRET"}`

is not matched by `authorization_span()` because the byte after `Authorization` is the closing quote. It is also not matched by `bearer_span()` because the scheme is `Basic`. This matters because issue/PR bodies and Actions logs are themselves retained `Value::String` values; their inner JSON-looking text is not recursively parsed as JSON. The plaintext secret can therefore survive `sanitize_error()`, be persisted in cache, and be surfaced in a DTO excerpt.

The V05 sanitizer test covers quoted `token`, `api_key`, and `access-token` examples but not quoted `Authorization` with a non-Bearer value, so the defect is not caught.

### Actions

`fetch_resources_with_transport()` now parses the fetched jobs resource, filters with `is_action_failure_job`, and requests at most two eligible job logs. `enrich_actions()` repeats the same eligibility rule and attaches `GitHubActionLogEvidence { job_id, job_name, excerpt }`. Successful and skipped jobs are not eligible merely because a run failed.

### Link validation

`validate_project_owned_links()` validates task references against `canonical_project_task_ids()` and session references against `persisted_project_session_ids()`. `canonical_project_task_ids()` uses only a `CURRENT` `github_tracking::cached_snapshot`, which is the accepted tracked-branch TASKS-only remote authority. Session lookup is explicitly scoped by `project_id`.

### Legacy route

`src/App.tsx::LegacyAgentsRoute()` always renders `PromptEnginePage legacyRoute`. `PromptEnginePage` normalizes `/agents` to `/prompts?surface=sessions...`. The internal `Agents` component is mounted with `embedded`, which suppresses the standalone Claude readiness card.

### Budget

`acquire_enrichment()` and `acquire_enrichment_text()` call `budget.take()` before invoking `load_or_fetch*`. Exhaustion returns `GITHUB_SUBRESOURCE_REQUEST_BOUND` without calling transport/cache acquisition.

## 7. FOCUSED TEST EVIDENCE

Positive direct coverage exists for:

- cache schema-1 rejection and malformed cache rejection;
- representative URL-query/token/Bearer/GitHub-token sanitizer cases at helper level;
- a production transport PR/Actions happy path;
- selected-project task/session ownership filtering;
- job-level success/skipped exclusion and failed/cancelled provenance at enrichment level;
- pre-acquisition budget enforcement;
- bare `/agents` integration;
- exact targeted-session legacy handoff;
- wrong-project/missing-session handling.

However, the exact V05 required matrix is incomplete:

- sanitizer representatives are not each driven through `github_sync_state.metadata_json` plus frontend snapshot projection; the quoted `Authorization`/non-Bearer case is missing and exposes the source defect;
- Actions does not directly exercise the required acquisition-path matrix for successful jobs before a third failing job, cancelled, timed-out, action-required, first failed log unavailable/second available, jobs unavailable, failed run with no eligible jobs, and deterministic truncation with exact transport assertions;
- legacy-route mounted coverage combines duplicate/partial/malformed behavior into one case and does not directly prove all required project-only, session-only, duplicate, malformed, and overlong variants;
- project-link coverage uses selected-project valid plus unknown/foreign-looking tokens, but does not build the requested cross-project canonical task ownership fixture across representative portfolio identities.

Passing helper-level assertions do not substitute for the production-path cases explicitly required by the prompt.

## 8. REGRESSION EVIDENCE

Builder claims:

- focused M18 Rust: 18 passed;
- focused frontend provider/session/routing: 29 passed;
- full frontend: 146 passed;
- typecheck/build/cargo check/diff check: passed;
- serialized Rust library suite: 514 passed.

The same builder log also records that the initial normal parallel `cargo test --lib` invocation exposed three pre-existing shared `task_intelligence` failpoint races. The V05 prompt explicitly required the normal non-ignored full Rust invocation to be genuinely green. Running serially is useful diagnostic evidence but does not satisfy that gate.

No hosted GitHub checks independently corroborate the builder's local results.

## 9. SECURITY / SAFETY REVIEW

Positive:

- no GitHub PAT/API-key setting was added;
- no GitHub credential/auth-store read was introduced;
- no remote mutation path was added;
- no arbitrary frontend-controlled API host was introduced;
- cache schema versioning now invalidates unsafe schema-1 rows;
- project/session ownership validation is fail-closed when canonical evidence is unavailable.

Failure:

Quoted `Authorization` text with a non-Bearer value inside retained string content can survive the current sanitizer. Because GitHub issue/PR bodies or job logs can legitimately contain JSON/code snippets, this remains a successful-payload persistence/UI secret-redaction defect.

## 10. ARCHITECTURE CONSISTENCY

The implementation remains consistent with the accepted Registry identity, X04 TASKS-only authority, read-only GitHub integration, Local Git Engine, Prompt Engine/session provenance, Codex-only audit provider, and Claude adapter boundaries.

The V05 route consolidation is architecture-consistent: Agents remains an internal component while Prompt Engine is the single top-level workflow.

The release-gate handling is not governance-consistent because a required normal full-suite failure was followed by publication.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Codex correctly kept `TASKS.md` and `CODEX_ROADMAP.md` read-only and did not activate M19.

The canonical tracker prose is stale relative to the actual V05 execution, still naming V04 as the current remediation. This is a ChatGPT-owned tracker transition issue, not a Codex scope violation, and must be corrected when V06 remediation is staged.

The V05 builder log is otherwise useful because it explicitly reports the failed normal Rust run instead of disguising it as a green gate.

## 12. FINAL REPOSITORY STATE

At audit time live GitHub `main` is:

`aa081fa9775d5000928f25488df96740aeb97935`

The V05 implementation and builder-log commits are linear descendants of the V05 authority SHA. No builder-authored tracker edit or M19 activation is present.

## 13. OPEN CROSS-MILESTONE FINDINGS

The normal Rust failure exposes a pre-existing shared failpoint/test-isolation race in `task_intelligence` under parallel library execution. Even if production semantics are unaffected, it is now an explicit M18 release-gate blocker because the current milestone requires a genuinely green normal full regression before publication/acceptance.

No accepted M00-M17 or X03/X04 production regression was identified in the reviewed V05 diff.

## 14. DEFECTS BY SEVERITY

### F-M18-V05-001 — MAJOR — Quoted Authorization text can bypass successful-payload redaction

`authorization_span()` does not accept a quoted key boundary between `Authorization` and `:`. A retained string containing `{"Authorization":"Basic QUOTED_AUTH_SECRET"}` therefore bypasses that matcher; because it is non-Bearer, the bearer matcher cannot rescue it. Issue/PR body strings and text logs can contain exactly this kind of JSON/code snippet.

Required closure:

- make quoted/punctuation-adjacent Authorization key/value forms safe without unbounded regex behavior;
- add direct persistence + snapshot tests for every V05 representative, including quoted non-Bearer Authorization;
- prove plaintext is absent from both `github_sync_state.metadata_json` and frontend-facing DTOs;
- preserve schema-2 legacy invalidation and bounded deterministic behavior.

### F-M18-V05-002 — MAJOR — Required V05 production-path evidence matrix is incomplete

Source logic for job filtering, ownership validation, legacy routing, and request-budget reservation is materially improved, but the explicit acceptance matrix was not implemented in full.

Required closure must add the missing direct cases, especially:

- Actions acquisition-path ordering/failure variants, unavailable/partial states, no-eligible-job case, and deterministic truncation;
- each required legacy `/agents` partial/duplicate/malformed/overlong mounted case;
- project-owned task/session cross-project fixtures using syntactically valid foreign identities;
- end-to-end sanitizer persistence/projection cases, not helper-only checks.

Tests must exercise the same production selection/enrichment/routing code, not only helper projections.

### F-M18-V05-003 — MAJOR — Mandatory normal full Rust gate failed, yet publication proceeded

The builder log records that normal parallel `cargo test --lib` exposed three shared `task_intelligence` failpoint races. The prompt explicitly required the normal non-ignored full Rust suite to be genuinely green and allowed publication only after every required gate passed.

A serialized `cargo test --lib -- --test-threads=1` pass does not satisfy that requirement.

Required closure:

- reproduce and isolate the parallel-test interference;
- repair test/failpoint isolation without weakening accepted M09/task-intelligence production behavior or hiding tests;
- run the normal full Rust library command successfully;
- only then run governed publication and record the new artifact hash.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

The global enrichment budget is now a real pre-acquisition boundary, which is correct. At high PR volume it can consume most of the budget before later Actions enrichment. This is not a V05 blocker because bounded/unavailable remainder states were explicitly permitted, but a future hardening pass may want a deterministic fairness allocation between PR and Actions evidence so one family cannot starve the other.

## 16. UNVERIFIED ITEMS

- Builder-reported native executable hash/publication smoke was not independently reproduced.
- There are no hosted GitHub status/check results for the implementation commit.
- The exact three parallel Rust test failure names were not persisted in the V05 builder log; the log records their subsystem/cause class but not the full failure transcript.
- Owner-native M18 UI acceptance has not been run because source/release acceptance failed first.

## 17. REGRESSION RISK

**MEDIUM-HIGH** until V06.

Most V05 functional fixes are structurally sound, but successful-payload redaction is a security/trust boundary, and an unrepaired parallel full-suite failure means the release gate is not deterministic.

## 18. AUDIT CONFIDENCE

**HIGH**.

Confidence is based on the exact V05 diff, production sanitizer/acquisition/link/route symbols, direct test bodies, builder-log gate evidence, current GitHub branch state, and independent GitHub combined-status/workflow inspection.

## 19. FINAL VERDICT

**FAIL — CHANGES_REQUIRED**

M18 V05 is not eligible for owner-native acceptance or closure. M19 remains blocked.

## 20. REQUIRED REMEDIATION

Create one bounded M18 V06 remediation limited to F-M18-V05-001 through F-M18-V05-003.

V06 must:

1. close the quoted/punctuation-adjacent Authorization redaction hole and complete end-to-end cache/DTO secret tests;
2. complete the exact missing V05 production-path/adversarial evidence matrix without rewriting already-correct V05 architecture;
3. stabilize the normal parallel Rust full-suite failpoint isolation, obtain a genuinely green normal `cargo test --lib`, and publish only after every required gate is green;
4. preserve cache schema 2, job-truth Actions selection, raw-vs-validated ownership separation, integrated Prompt Engine Sessions routing, Builder Providers in Settings, request-budget pre-acquisition, X04 TASKS-only authority, exact eight-project portfolio, `Sekiph82/FormuLab@main`, Codex-only audit provider, Claude adapter, and all accepted M00-M17 behavior;
5. keep `TASKS.md` and `CODEX_ROADMAP.md` read-only for Codex and leave M19 blocked;
6. stop for independent V06 source re-audit before owner-native acceptance.
