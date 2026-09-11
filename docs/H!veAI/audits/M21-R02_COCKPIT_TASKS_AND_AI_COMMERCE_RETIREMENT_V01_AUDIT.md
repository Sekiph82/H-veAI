# M21-R02 Cockpit Tasks and AI-Commerce Retirement V01 Strict Audit

## 1. VERDICT

**FAIL**

The V01 implementation correctly removes `Sekiph82/AI-Commerce-HQ` from the default portfolio and adds a real GitHub-root-`TASKS.md` task-row production path, but it does not safely migrate existing persisted GitHub snapshots created before `task_rows` existed. The current owner environment is precisely an existing-state upgrade, so this defect can preserve the original empty Tasks symptom even after the new executable is published.

A second truthfulness defect remains in the remote Tasks UI: an unavailable/error remote snapshot with no rows is rendered with the same message as a valid current `TASKS.md` containing zero parseable tasks.

Owner re-acceptance is therefore premature. Do not delete the local AI-Commerce parent or the GitHub `Sekiph82/AI-Commerce-HQ` repository.

## 2. CONTRACT RECOVERY

V01 was required to:

1. make Project Cockpit Tasks render canonical remote rows from GitHub root `TASKS.md` without requiring legacy persisted task intelligence;
2. keep Command Center, Cockpit Overview and Cockpit Tasks semantically consistent for the same remote observation;
3. preserve truthful current/empty/stale/unavailable behavior;
4. remove AI-Commerce-HQ from the permanent default portfolio while preserving existing user state;
5. keep local parent and GitHub AI-Commerce deletion prohibited;
6. update tracker truth, run focused regressions, publish the native QA executable and leave owner acceptance pending.

## 3. BRANCH / HEAD / DIFF SCOPE

Repository: `Sekiph82/H-veAI`

Branch: `main`

Prompt/base commit: `fade69d76e667cc2065d2c3ab8051833862fd87e`

Implementation commit: `d8e7cb14016810b420662f1ae3d2428c8ae2eb30`

Builder-log commit: `572db3652e1d1884ea2d5729e077f042fbc0fb24`

GitHub `main` resolved to the builder-log commit before this audit was published.

The implementation changes the canonical tracker, GitHub remote tracking model/parser, Project Cockpit backend contract, frontend Project Cockpit Tasks rendering, and related tests.

## 4. ACCEPTANCE CRITERIA MATRIX

| Requirement | Result | Audit note |
| --- | --- | --- |
| Real remote task-row production path | PASS | `parse_root_tasks` now materializes `RemoteTaskRow` values and Project Cockpit exposes them. |
| No dependency on legacy task intelligence for remote Tasks | PASS | Remote projects continue to set legacy intelligence to `None`, while the UI consumes `remoteTasks`. |
| Populated fresh/current remote snapshot renders task rows | PASS | Source path and focused fixture tests support this path. |
| Existing pre-V01 persisted remote snapshot upgrades safely | **FAIL** | Old JSON deserializes `task_rows` as empty, and same-HEAD/current fast path returns that cache without reparsing. |
| Valid canonical empty state | PASS for current snapshots | A current snapshot with `total_tasks == 0` and empty rows has an explicit empty state. |
| Unavailable/error remote state is labeled truthfully | **FAIL** | UI says the remote `TASKS.md` contains no parseable rows even when the remote observation is unavailable/error and no document was successfully observed. |
| Stale cached remote truth does not switch to local authority | PASS/PARTIAL | Stale snapshots remain remote-derived, but old-schema row hydration is not guaranteed. |
| Cross-surface same-snapshot semantics | PASS for newly materialized snapshots | Backend uses the same `RemoteTrackingSnapshot`; parity test covers current task/progress and row identity. |
| Default portfolio is exactly eight repositories | PASS | Default target list is now eight and excludes AI-Commerce-HQ. |
| Fresh DB does not require an exclusion row to hide AI-Commerce-HQ | PASS | The target was removed from bootstrap source. |
| Existing unrelated project state preserved | PASS | Existing reconciliation/archival behavior remains bounded; focused tests cover duplicates/non-target rows. |
| AI-Commerce local/GitHub deletion prohibited | PASS | No destructive deletion is in the implementation. |
| Tracker reflects implementation complete / owner acceptance pending | PASS before this audit | Root `TASKS.md` truth matched the builder state. V02 must now record this failed audit/remediation state. |
| Native QA publication | UNVERIFIED | Builder log reports success and an executable hash, but host-local publication cannot be independently reconstructed from GitHub. |
| Final local HEAD/remote equality | UNVERIFIED | GitHub publication is visible; exact host-local final equality is not independently observable. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

Confirmed:

- the implementation commit exists and directly precedes the log commit;
- `RemoteTaskRow` is a real backend model rather than fabricated frontend-only content;
- root `TASKS.md` checklist rows are parsed into bounded remote rows;
- Project Cockpit projects the row list to the frontend;
- the Tasks UI renders `remoteTasks` for GitHub-tracked projects;
- AI-Commerce-HQ is absent from the eight-entry default portfolio;
- the historical special branch handling for AI-Commerce-HQ was removed;
- focused current-state and portfolio tests were added/updated.

Not supported by repository truth:

- the builder log says empty and degraded remote states remain explicit and bounded. The current frontend does not distinguish a genuine canonical empty document from unavailable/error remote data when `remoteTasks` is empty.
- the builder log implies the fix is ready for owner re-acceptance, but the old-cache migration path can leave populated projects with zero rows indefinitely while the remote HEAD remains unchanged.

## 6. FILE / SYMBOL EVIDENCE

### `src-tauri/src/github_tracking.rs`

`RemoteTrackingSnapshot.task_rows` uses `#[serde(default)]`. This is correct for backward-compatible deserialization but means every old persisted snapshot silently becomes `task_rows = []`.

`observe_project(...)` loads the previous persisted snapshot, fetches the remote HEAD and immediately returns the cached snapshot when:

- cached `remote_head` equals fetched HEAD; and
- cached `remote_health == "CURRENT"`.

There is no compatibility check that a previously populated tracker (`total_tasks > 0`) also contains the newly required row materialization. Therefore an old cache can remain structurally incomplete forever until the remote HEAD changes.

This is especially material to the owner's ScrubBots case: the original defect was observed before `task_rows` existed, so the local database can contain exactly this legacy snapshot shape.

### `src/pages.tsx::CockpitLiveTasks`

The remote branch is selected with `Boolean(snapshot.githubTracking)`. When `remoteTasks` is empty, the rendered empty state always says:

`The remote root TASKS.md contains no parseable task rows.`

That statement is only valid for a successfully observed canonical document. It is not valid for `UNAVAILABLE` or `ERROR` remote health.

### `src-tauri/src/project_cockpit.rs`

The backend correctly keeps legacy `task_intelligence` absent for remote-primary projects and projects `remote.task_rows` as `remote_tasks`. This portion of the architecture is correct and should be preserved in V02.

### Default portfolio

`ensure_portfolio(...)` now declares exactly eight targets and no longer contains `Sekiph82/AI-Commerce-HQ`. This finding is closed.

## 7. FOCUSED TEST EVIDENCE

The added tests prove:

- fresh/newly materialized remote rows work when legacy task intelligence is empty;
- a deliberately current zero-task snapshot is represented without a legacy-intelligence error;
- current cross-surface row identity/count parity;
- eight-project portfolio behavior and non-target archival.

The required upgrade case is not covered: there is no focused test that starts from a persisted pre-`task_rows` JSON snapshot with `total_tasks > 0`, the same remote HEAD and `CURRENT` health, then proves the production refresh path forces a reparse/materialization instead of returning the structurally incomplete cache.

The UI also lacks focused evidence that `CURRENT + zero tasks`, `STALE + cached rows`, and `UNAVAILABLE/ERROR + no rows` render distinct truthful states.

## 8. REGRESSION EVIDENCE

The builder reports 415 bounded Rust tests, 125 frontend tests, typecheck/build and publication smoke passing. Those results are useful regression evidence but do not override the direct compatibility defect above.

F02 portfolio retirement appears regression-safe from the inspected source and focused tests.

## 9. SECURITY / SAFETY REVIEW

PASS for destructive-safety boundaries.

No deletion of the local parent, preservation trees, or GitHub AI-Commerce repository was introduced. No reset/rebase/force-push/clean behavior is part of the implementation diff.

The remaining defects are truth/correctness defects, not destructive-security defects.

## 10. ARCHITECTURE CONSISTENCY

The core V01 architectural direction is sound: GitHub root `TASKS.md` remains canonical, remote rows are transported through the backend snapshot, and legacy `.hiveai` task intelligence is not revived.

The missing piece is schema evolution of durable remote cache. A new required materialized field cannot rely only on serde defaulting if a same-HEAD optimization can permanently accept an old incomplete snapshot as current.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Before this audit, root `TASKS.md` truthfully described the builder state as implementation-complete and awaiting independent audit/owner re-acceptance.

This audit changes the gate: M21-R02 V01 failed independent audit. The next builder execution must update current tracker truth to V02 remediation in progress/implementation complete as appropriate and must not mark owner re-acceptance ready until the cache and remote-state findings are closed.

The V01 log should remain immutable.

## 12. FINAL REPOSITORY STATE

GitHub `main` before this audit: `572db3652e1d1884ea2d5729e077f042fbc0fb24`.

That commit contains the immutable V01 builder log and directly follows implementation commit `d8e7cb14016810b420662f1ae3d2428c8ae2eb30`.

No evidence indicates unrelated repository source was mixed into H!veAI.

## 13. OPEN CROSS-MILESTONE FINDINGS

- Local historical AI-Commerce parent deletion remains prohibited until a separate relocation/retirement gate proves the active H!veAI checkout and shortcut/runtime live outside the parent.
- GitHub `Sekiph82/AI-Commerce-HQ` deletion remains owner-controlled and must not occur until the H!veAI runtime no longer depends on it and the local relocation gate is closed.

## 14. DEFECTS BY SEVERITY

### M21-R02-V01-F01 — MAJOR — legacy remote cache can permanently suppress new task rows

Old persisted snapshots deserialize with empty `task_rows`. Same-HEAD/current optimization returns them without reparsing, so a populated canonical project can still display zero Tasks after upgrade.

### M21-R02-V01-F02 — MAJOR — unavailable/error remote state is mislabeled as canonical empty

`CockpitLiveTasks` treats every remote snapshot with zero rows as if a valid remote `TASKS.md` had been observed and contained no parseable rows.

### NOTE — host-local publication/equality remains independently unverified

This is an evidence limitation, not a repository defect.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Introduce an explicit remote snapshot schema/materialization version or a narrowly scoped compatibility predicate so future durable-snapshot shape changes can invalidate/rebuild stale structural representations without requiring a remote commit.

Keep this mechanism generic enough for remote snapshot evolution, but do not turn V02 into a broad cache framework rewrite.

## 16. UNVERIFIED ITEMS

- exact owner-machine SQLite contents after V01 publication;
- exact native executable installed/published bytes on the owner machine;
- exact final local working-tree cleanliness/equality at builder return.

The F01 finding does not depend on knowing the exact local database. The production code path is sufficient to prove that any pre-V01 populated cache with unchanged HEAD can remain incomplete.

## 17. REGRESSION RISK

**MEDIUM**

The architectural change is narrow and F02 is correct, but the affected state is durable user cache and the owner is upgrading an existing installation, which makes compatibility behavior release-significant.

## 18. AUDIT CONFIDENCE

**HIGH**

The implementation commit, current source, parser/cache behavior, frontend state rendering, focused tests, tracker and publication commit are directly inspectable in GitHub. The blocking findings follow from deterministic source paths rather than missing host-local evidence.

## 19. FINAL VERDICT

**FAIL**

Do not ask the owner to perform final native re-acceptance yet. V02 must first make old persisted remote snapshots self-heal without a remote HEAD change and make canonical-empty versus unavailable/error states truthful.

The eight-project AI-Commerce default-target retirement portion is **PASS** and must not be reverted.

## 20. REQUIRED REMEDIATION

Create `M21-R02 ... V02` with exactly these bounded goals:

1. Detect structurally incomplete legacy remote snapshots and force a real root-`TASKS.md` re-observation/materialization even when remote HEAD is unchanged. A populated snapshot (`total_tasks > 0`) with missing/empty rows must not be accepted as fully current merely because HEAD matches.
2. Preserve genuine `total_tasks == 0` canonical snapshots as valid empty state.
3. Add focused persisted-old-schema compatibility tests proving the production decision path self-heals without requiring a new remote commit.
4. Make Cockpit Tasks empty/error rendering health-aware: distinguish `CURRENT empty`, `STALE cached`, and `UNAVAILABLE/ERROR` truthfully.
5. Add focused UI/contract tests for those states.
6. Preserve the V01 remote-row architecture and the corrected eight-project default portfolio.
7. Do not delete or relocate the local AI-Commerce parent and do not delete the GitHub AI-Commerce repository.
8. Rebuild/publish the native QA executable only after the focused and relevant regression suites pass.
9. Keep owner re-acceptance pending until V02 passes independent audit.
