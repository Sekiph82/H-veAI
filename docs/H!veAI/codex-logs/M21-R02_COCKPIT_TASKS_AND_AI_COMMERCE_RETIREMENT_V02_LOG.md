# M21-R02 Cockpit Tasks and AI-Commerce Retirement V02

Status: `COMPLETE_AWAITING_INDEPENDENT_AUDIT`

## Synchronization and scope

- Work code/version: `M21-R02 V02`.
- Repository/branch: `Sekiph82/H-veAI` / `main`.
- Synchronized starting GitHub SHA: `4265b263549a478eefd614368135f600e13f0d5d`.
- Implementation commit: `5dc006d3dd65b5f136308cfb2aea01725936bf04`.
- The checkout was clean and only behind `origin/main` before the safe fast-forward. No reset, rebase, force-push, stash, clean, destructive checkout, or divergent-work overwrite was used.
- The V01 log and failed audit remain immutable. This V02 log is the sole new work-item evidence artifact.

## V01 F01: legacy remote-cache compatibility

Root cause: V01 added `RemoteTrackingSnapshot.task_rows` with `serde(default)`, so a pre-V01 persisted JSON snapshot with a populated `total_tasks`, `CURRENT` health, and unchanged remote HEAD silently deserialized with `task_rows = []`. `observe_project` then accepted that incomplete cache through the same-HEAD/current early return, allowing a populated project to remain empty in Cockpit Tasks until a new GitHub commit.

The bounded production fix is `same_head_cache_is_reusable`. The early return now requires the fetched HEAD and `CURRENT` health plus a structurally complete row shape:

- `total_tasks == 0` is reusable only with zero rows, preserving a genuine canonical empty document;
- `total_tasks > 0` is reusable only with at least one materialized row;
- missing/unknown task counts and populated-count/zero-row snapshots are not reusable and force the existing GitHub root `TASKS.md` fetch, parse, and persistence path.

No local `TASKS.md` fallback, database deletion, cache clearing, or broad migration was introduced. The accepted GitHub root tracker and remote-row architecture remain authoritative.

Focused evidence:

- `github_tracking::tests::legacy_remote_snapshot_without_task_rows_is_not_reusable_at_same_head` passed using serialized historical JSON with the `taskRows` field removed.
- `github_tracking::tests::same_head_cache_reuse_accepts_materialized_populated_and_empty_snapshots` passed for new populated and genuine zero-task snapshots.
- `github_tracking::tests::reparsing_and_persisting_legacy_snapshot_materializes_rows` passed through the durable SQLite cache path: historical JSON was loaded as incomplete, reparsed from canonical remote content, persisted, reloaded with two rows, and accepted for same-HEAD reuse.
- `github_tracking::tests::root_tasks_parser_materializes_current_state_and_exact_counts` passed.

## V01 F02: remote Tasks truthfulness

Root cause: the V01 UI treated every GitHub-backed snapshot with zero `remoteTasks` as a confirmed empty remote `TASKS.md`, including `UNAVAILABLE` and `ERROR` snapshots where no canonical document had been observed.

The frontend now derives `getRemoteTasksView` from the same remote snapshot and row projection. Its explicit state matrix is:

| Remote evidence | Tasks surface |
| --- | --- |
| `CURRENT` with rows | `Canonical remote tasks`; rows render normally. |
| `CURRENT` with `totalTasks == 0` and no rows | `No canonical remote tasks`; states that GitHub `TASKS.md` was observed successfully and was empty. |
| `STALE` with cached rows | `Showing stale remote tasks`; cached rows remain visible with the bounded reason. |
| `STALE` without rows | `Remote tasks unavailable`; stale snapshot has no usable rows. |
| `UNAVAILABLE` or `ERROR` without rows | `Remote tasks unavailable`; includes the bounded remote reason and never claims canonical emptiness. |
| `CURRENT` with a positive count and no rows | `Remote tasks need refresh`; marks structural inconsistency instead of empty. |
| degraded/error health with cached rows | `Showing cached remote tasks`; cached evidence is labeled degraded. |

Focused frontend contract evidence:

- `tests/m21-r02-cockpit-tasks-focused.test.ts` passed all 6 tests covering current populated, current confirmed empty, stale cached, unavailable, error, and structural inconsistency states.
- `tests/m12-project-cockpit-focused.test.tsx` passed all 8 existing Project Cockpit tests.
- The remote Tasks UI continues to consume `remoteTasks`; legacy task intelligence and local filesystem truth were not revived.

## Preserved V01 F02 portfolio boundary

The accepted V01 eight-project default portfolio remains unchanged in effect. `Sekiph82/AI-Commerce-HQ` remains absent from the default target list, no special AI-Commerce bootstrap branch was added, and existing unrelated persisted state is preserved by the existing bounded reconciliation behavior. Neither the local AI-Commerce parent/preservation trees nor the GitHub `Sekiph82/AI-Commerce-HQ` repository was deleted, relocated, or modified.

## Tracker truth

Root `TASKS.md` now records M21-R02 V02 as implementation-complete and awaiting an independent V02 strict audit. Historical M21 and M21-R01 truth remains preserved, the user-facing 20-milestone denominator is unchanged, and owner native re-acceptance remains pending until the independent V02 audit passes.

## Validation and publication

- New V02 cache compatibility tests: 3 passed.
- Focused GitHub tracking Rust suite: 9 passed.
- Focused Project Cockpit Rust suite: 14 passed.
- Broader Rust library regression: 418 passed, 0 failed; the same three pre-existing long-running observation/recovery tests were excluded from the bounded run.
- Full frontend regression: 16 test files / 131 tests passed. Existing React `act(...)` warnings remain non-failing test stderr only.
- `npm run typecheck`: passed.
- `npm run build`: passed.
- `git diff --check`: passed.
- Existing `scripts/publish-dev-qa.ps1`: passed production Tauri build, candidate smoke, stable smoke, PE/readiness validation, forbidden-port check, visible-console check, frontend readiness, and shortcut target/icon validation.
- Published executable SHA-256: `47E4E5443A9017CD81D6DCEAC3D0BA1CE3107B218847CB2323800C62B1C52528`.

After this log is pushed, the final publication gate reruns `git fetch origin main`, compares local `HEAD`, `origin/main`, and live `git ls-remote origin refs/heads/main`, and confirms a clean H!veAI worktree. The exact final SHA values are returned with the completion response. This immutable log intentionally does not contain its own creating commit SHA.

## Acceptance boundary

`OWNER_REACCEPTANCE_REQUIRED`: V02 must receive an independent strict audit before owner native re-acceptance. The local AI-Commerce parent, preservation trees, and GitHub `Sekiph82/AI-Commerce-HQ` remain preserved by explicit scope.
