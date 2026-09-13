# M16T Codex-Only Audit Provider V05 Strict Audit

## 1. VERDICT

**PASS**

M16T V05 closes the live FormuLab branch-mapping defect without regressing the accepted Codex-only audit-provider architecture or V03 tracker truth. H!veAI now tracks `Sekiph82/FormuLab` on canonical/default branch `main`, reconciles an existing persisted FormuLab record in place, preserves attached local-workspace metadata, invalidates only the affected remote-task cache when the tracked branch changes, and makes same-HEAD cache reuse branch-aware.

This PASS is the independent source/implementation audit for V05. **M16 remains OPEN pending owner native acceptance.** M17 remains NOT ACTIVATED/BLOCKED.

## 2. CONTRACT RECOVERY

V05 was required to remediate one production tracking defect while preserving prior accepted decisions:

1. replace FormuLab target branch `feature/laboratory-stability` with `main` in active H!veAI portfolio tracking;
2. keep the active portfolio at exactly eight projects;
3. avoid creating a duplicate FormuLab project when an existing install already has the old branch persisted;
4. preserve attached FormuLab local-workspace/path metadata across the branch migration;
5. prevent a cached `CURRENT` old-branch snapshot from being accepted as valid `main` truth merely because both branches resolve to the same HEAD SHA;
6. preserve the accepted Codex-only audit provider, with no `OPENAI_API_KEY`, direct OpenAI HTTP audit transport, API-key fallback, or Claude/M17 activation;
7. preserve V03 current tracker semantics: M16T `[~]`, Required Actor HUMAN, M16 OPEN, M17 blocked, progress 16/20;
8. publish deterministic focused/regression evidence and a governed native build for subsequent owner acceptance.

## 3. BRANCH / HEAD / DIFF SCOPE

Repository: `Sekiph82/H-veAI`

Branch: `main`

V05 prompt/base SHA: `3eaa2b345975ce7e38775a4d1d36bfed9a628e90`

V05 implementation SHA: `bcbc8675ef95ea25da73b3fac1902c0dfc272fb6`

V05 log SHA/final audited HEAD before this audit artifact: `7945c7e883122d37fb89c44b26c9fc1ef3418c4b`

Implementation diff is one commit ahead of the prompt base and changes only:

- `src-tauri/src/github_tracking.rs`;
- `TASKS.md`;
- `CODEX_ROADMAP.md`.

The V05 log commit is directly parented by the implementation commit.

## 4. ACCEPTANCE CRITERIA MATRIX

| Requirement | Result | Audit evidence |
| --- | --- | --- |
| Active FormuLab target is `main` | PASS | `ensure_portfolio::TARGETS` now contains `Sekiph82/FormuLab @ main`. |
| FormuLab GitHub canonical/default branch is `main` | PASS | Independent live repository metadata reports `default_branch = main`; root `TASKS.md` also declares `Tracking Branch: main`. |
| Active portfolio remains exactly 8 | PASS by source/test evidence | Fresh-bootstrap test asserts the exact eight-repository matrix. |
| Exactly one FormuLab project remains | PASS by source/test evidence | Migration test asserts one FormuLab record after reconciliation. |
| Existing old-branch FormuLab record migrates in place | PASS | Reconciliation resolves existing project by GitHub identity and updates repository/default branch rather than creating a second identity. |
| Local workspace/path metadata is preserved | PASS by implementation/test evidence | Branch update does not rewrite path fields; migration test asserts original/normalized path preservation. |
| Old branch cache invalidated on branch change | PASS | `GITHUB_TASKS_REMOTE` is deleted only for the affected project when persisted branch differs. |
| Equal-HEAD old branch cannot masquerade as `main` | PASS | `same_head_cache_is_reusable` now requires `snapshot.branch == expected_branch`; direct fixture covers same HEAD/different branch. |
| Other seven portfolio targets unchanged | PASS | Source matrix remains the prior seven repositories on `main`; focused test asserts one each. |
| AI-Commerce-HQ does not return to active portfolio | PASS | Exact eight-target matrix excludes it. |
| V03 tracker semantics preserved | PASS | M16T remains `[~]`, actor HUMAN, M16 OPEN, M17 blocked, 16/20. |
| Codex-only provider architecture preserved | PASS | V05 has no `audit_engine.rs` or `codex_runtime.rs` production changes; active-source search returns no `OPENAI_API_KEY`. |
| Full builder regression claims independently CI-verified | UNVERIFIED | No GitHub commit status/check is published for the implementation commit; counts remain builder claims. |
| Owner native acceptance | PENDING MANUAL ACCEPTANCE | Required after this independent source audit before M16 can close. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

The V05 builder log claims implementation commit `bcbc8675ef95ea25da73b3fac1902c0dfc272fb6`. That commit exists and its diff matches the claimed scope.

Confirmed independently:

- FormuLab active target changed from the historical feature branch to `main`;
- branch migration reads the previously persisted repository branch before update;
- project-scoped `GITHUB_TASKS_REMOTE` cache invalidation occurs when the configured branch changes;
- same-head cache reuse now also checks the recorded snapshot branch;
- fresh-bootstrap and existing-install branch migration tests are present in source;
- current TASKS/roadmap state matches the builder log;
- V05 log commit `7945c7e883122d37fb89c44b26c9fc1ef3418c4b` is directly parented by the implementation commit.

Builder-reported counts such as `435 passed`, `135/135`, typecheck/build/publication PASS, and the published executable SHA are treated as builder evidence because no independent GitHub CI status is attached to the implementation commit.

## 6. FILE / SYMBOL EVIDENCE

Primary production evidence is `src-tauri/src/github_tracking.rs`:

- `ensure_portfolio::TARGETS` now configures `("formulab", "FormuLab", "Sekiph82/FormuLab", "main")`;
- existing project lookup first permits exact target identity and then falls back to the stable GitHub repository identity, allowing an old persisted FormuLab project to be reused instead of duplicated;
- `previous_branch` is read before repository update;
- when `previous_branch != configured branch`, only that project's `GITHUB_TASKS_REMOTE` cache row is deleted;
- `same_head_cache_is_reusable(snapshot, fetched_head, expected_branch)` rejects branch mismatch before considering HEAD and task-row structural completeness;
- `observe_project` passes the currently tracked branch into the cache-reuse predicate.

Independent FormuLab evidence:

- repository metadata reports default branch `main`;
- live root `TASKS.md` reports `Tracking Repository: Sekiph82/FormuLab` and `Tracking Branch: main`.

## 7. FOCUSED TEST EVIDENCE

Source contains deterministic tests directly covering the changed behavior:

- `ensure_portfolio_bootstraps_exactly_eight_canonical_targets` asserts the eight-repository matrix and FormuLab `main`;
- `ensure_portfolio_migrates_formulab_branch_without_duplicate_or_cache_reuse` creates an old persisted `feature/laboratory-stability` FormuLab project, attaches a local workspace, persists an old-branch snapshot, verifies same-HEAD old-branch reuse is rejected for `main`, runs reconciliation, and asserts:
  - portfolio count remains 8;
  - exactly one FormuLab project exists;
  - original project ID is preserved;
  - repository default branch becomes `main`;
  - original/normalized local paths are preserved;
  - affected cached snapshot is removed;
  - each of the other seven repositories remains exactly once.

These test bodies directly exercise the previously missed production failure mode. Their presence and assertions are independently verified; the builder's execution counts are not independently rerun by this GitHub-only audit.

## 8. REGRESSION EVIDENCE

The implementation diff does not modify the Codex audit runtime or audit engine. Existing remote-tracking logic remains intact aside from the branch-aware reuse parameter and project-scoped branch-change cache invalidation.

The existing exact eight-target design is preserved. The FormuLab change is narrow and the new migration test explicitly guards duplicate creation and path loss.

The builder reports:

- focused `github_tracking`: 11/11;
- Codex runtime: 6/6;
- audit engine: 34/34;
- Rust library: 435/435;
- frontend: 135/135;
- typecheck, cargo check, build, diff-check and governed native publication PASS.

No GitHub status checks are available, so those execution results remain builder evidence rather than independent CI proof.

## 9. SECURITY / SAFETY REVIEW

No secret-handling surface was added. No `OPENAI_API_KEY` path is present in active source search, and V05 does not modify the accepted Codex CLI audit runtime.

The migration uses an existing SQLite transaction and performs a narrow cache deletion scoped by both `project_id` and `GITHUB_TASKS_REMOTE` resource kind. It does not broadly clear caches, delete project records, reset repositories, or mutate FormuLab itself.

No destructive Git operation, shell-mediated provider path, Claude integration, or API-key fallback is introduced by the audited diff.

## 10. ARCHITECTURE CONSISTENCY

V05 remains consistent with the GitHub-first eight-project architecture:

- remote project identity is GitHub repository-based;
- root `TASKS.md` remains remote task authority;
- local workspace is metadata attached to the same project rather than a duplicate project;
- remote branch truth is explicit and now canonical for FormuLab;
- branch-specific remote cache cannot silently cross branch boundaries;
- the accepted local Codex CLI audit-provider architecture remains unchanged.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Current root `TASKS.md` truth is internally consistent:

- Current Milestone M16;
- Current Sprint M16T-CODEX-ONLY;
- Current Task M16T V05;
- status implementation complete / awaiting independent audit and owner native acceptance;
- Required Actor HUMAN;
- M16T marker `[~]`;
- M16 remains OPEN;
- M17-M20 remain planned/blocked;
- strict milestone progress remains 16/20.

`CODEX_ROADMAP.md` mirrors the same prospective state.

The V05 builder log accurately identifies the implementation SHA and the final eight-target matrix. It correctly leaves its own log SHA out of the file and does not rewrite historical V02/V03 artifacts.

Following publication of this independent audit, the only remaining M16T gate is owner native acceptance; canonical current tracker text should be updated accordingly before final M16 closure.

## 12. FINAL REPOSITORY STATE

Before publication of this audit artifact, live GitHub `main` resolves to V05 log commit:

`7945c7e883122d37fb89c44b26c9fc1ef3418c4b`

Its parent is the V05 implementation commit:

`bcbc8675ef95ea25da73b3fac1902c0dfc272fb6`

Historical V02/V03 logs and audits remain preserved. V04 remains a superseded-before-execution artifact. No other repository is modified by V05.

The builder claims local HEAD, origin/main and live main equality and a clean worktree after V05 publication. The live GitHub side is independently visible; the local workstation equality/cleanliness claim cannot be independently observed through GitHub and remains builder evidence.

## 13. OPEN CROSS-MILESTONE FINDINGS

- **M16 owner native acceptance remains pending.** This is the next required quality gate and is not a source-code defect.
- M17 Claude Code Adapter remains intentionally blocked until M16 closes.
- No still-open V01/V02/V03 source defect remains in the audited V05 scope.

## 14. DEFECTS BY SEVERITY

**BLOCKER:** none.

**MAJOR:** none.

**MINOR:** none found in V05 production/source scope.

**NOTE:** builder execution counts and Windows publication/smoke details lack an independent GitHub CI status and therefore remain builder evidence until owner/native acceptance provides the required host-level confirmation.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Non-blocking future hardening opportunity: move the fixed eight-target portfolio matrix toward a governed typed configuration or central canonical target registry if the portfolio becomes dynamic. This is not required for V05 and must not be used to expand current scope.

The current branch-aware cache key/predicate is appropriate and materially safer than relying on HEAD alone.

## 16. UNVERIFIED ITEMS

The following are explicitly unverified by this GitHub-only independent audit:

- actual execution of the builder-reported 435 Rust tests and 135 frontend tests;
- actual Windows native smoke/publication run;
- published executable SHA on the owner's filesystem;
- Desktop shortcut target/icon after V05 publication;
- live H!veAI UI observation showing FormuLab on `main`;
- real Codex CLI readiness and a real Audit Center model turn in the newly published native executable;
- local checkout cleanliness and local/origin/live equality beyond the live GitHub commit chain.

These are not converted into source-audit PASS evidence. The host-native items belong to the pending owner acceptance gate.

## 17. REGRESSION RISK

**MEDIUM** until owner native acceptance.

The source change is narrow and directly tested, but it touches portfolio reconciliation and remote-cache reuse, both central to Command Center and Project Cockpit truth. Deterministic migration tests materially reduce risk, and no provider/runtime code changed. Native verification should confirm the real persisted owner database migrates correctly.

## 18. AUDIT CONFIDENCE

**HIGH** for source/commit/tracker correctness.

Confidence is based on direct GitHub inspection of the implementation diff, current production source, deterministic test bodies, current tracker/roadmap, commit ancestry, live FormuLab repository metadata, and FormuLab root task authority.

Confidence is intentionally lower for workstation-only runtime/publication claims because the independent auditor cannot execute the owner's Windows binary through GitHub.

## 19. FINAL VERDICT

**PASS.**

M16T V05 is source-audit accepted. The FormuLab branch-mapping defect is closed in production source and has appropriate existing-install/cache regression coverage.

**M16 remains OPEN only for owner native acceptance. M17 remains blocked.**

## 20. REQUIRED REMEDIATION

No additional Codex/code remediation is required for V05.

Required next gate is **owner native acceptance**, not another builder prompt. At minimum confirm in the published H!veAI executable that:

1. FormuLab appears exactly once and reports/tracks branch `main`;
2. the active portfolio remains 8 projects;
3. any attached local workspace remains attached;
4. Codex Audit Provider readiness succeeds through the owner's existing ChatGPT-authenticated local Codex CLI, with no API-key setup;
5. a real Audit Center run completes through Codex CLI without reintroducing an OpenAI API-key path.

Only after that owner acceptance is recorded should M16 be marked PASS/CLOSED and M17 be activated.