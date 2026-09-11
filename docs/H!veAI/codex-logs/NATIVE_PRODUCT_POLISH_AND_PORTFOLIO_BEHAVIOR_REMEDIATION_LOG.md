# Native Product Polish and Portfolio Behavior Remediation Log

Date: 2026-09-11
Repository: `Sekiph82/H-veAI`
Branch: `main`

## Result

Implemented and published the native portfolio behavior remediation. The production model remains GitHub repository metadata plus each repository's root `TASKS.md`; `.hiveai/PROJECT.json` was not restored to the runtime path.

## Root Causes and Fixes

- AI-Commerce-HQ used a legacy checklist format and its declared 20-task ledger was not parseable by the root-task parser. Its existing history is preserved, the canonical ledger is now 20 checked tasks, the three remaining historical feature notes remain visible outside the canonical ledger, and the tracker declares completed/closed with no current or next task. External normalization commit: `4ba9220de12caba0a85b7083398d4a61705b787c` on `Sekiph82/AI-Commerce-HQ@H!veAI`.
- Progress values were rendered with raw floating-point interpolation. Shared `formatPercent` and `ProgressIndicator` rendering now use exactly two decimal places throughout project progress surfaces.
- Command Center counted only a narrow set of workflow labels while Tasks used a broader current-state regex. Backend summary, queue, and Tasks now prefer current-task status and share bounded running-state semantics. `IN_PROGRESS` is counted as running; ready, blocked, waiting, and complete states are not.
- Command Center rendered a global Recent Activity section and used a squeezed right-rail grid. The global section is removed and the operational rail remains the focused brief, attention, queue, and system status surface.
- Project removal deleted only the row, so portfolio bootstrap could recreate built-in GitHub projects. Migration 23 adds durable repository/branch exclusions. Explicit register/repair clears the exclusion; removal never touches the remote repository or local folder.
- Cockpit Settings exposed Builder/Auditor as read-only facts. Settings now provide explicit Builder and Auditor selectors, including Unassigned, persist the values, and refresh project cards from the saved registry record.

## Validation

- Frontend typecheck: PASS.
- Frontend regression: `125 passed` across `15` files.
- Focused UI regression: `23 passed` for Command Center, cockpit settings, and startup behavior.
- Rust focused migrations/registry/Command Center tests: PASS.
- Rust regression excluding one pre-existing long-running observational test: `415 passed, 0 failed, 1 skipped`.
- The full Rust run reached all 416 tests; the existing observational test `m16l_current_command_center_cockpit_and_control_reads_are_observational` did not complete in the bounded run. The independent database-path assertion exposed by the new migration was corrected from schema `22` to schema `23` and its targeted test passes.
- Production frontend build: PASS.
- Tauri release build and governed standalone publication: PASS.
- Canonical opening-video SHA-256 preserved: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`.

## Native Evidence

- The Desktop shortcut target remains `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI\dev-bin\H!veAI.exe` and was synchronized to the final published binary.
- Final target process: `H!veAI`, `Responding=True`, with only `msedgewebview2.exe` as a child; no Git, cmd, PowerShell, or Terminal child was observed.
- Live SQLite validation after launching the final target: `9` active projects, schema/migration `23/23`, `0` removal exclusions, and `9` GitHub remote snapshots.
- AI-Commerce-HQ live remote state: `20/20`, `100.00%`, `COMPLETE`, no current task, no next task, remote health `CURRENT`, remote HEAD `4ba9220de12caba0a85b7083398d4a61705b787c`.
- The current remote portfolio snapshot has no project presently declaring `IN_PROGRESS`, so its live Running aggregate is `0`. The direct status test proves an `IN_PROGRESS` current task contributes exactly one and the same matcher is used by Command Center and Tasks.
- Project removal persistence and explicit re-add clearing are covered by the isolated SQLite test `github_removal_is_durable_until_explicit_reregister`; the removal SQL records only H!veAI exclusion state and does not mutate GitHub content.
- Builder/Auditor edit-and-persist behavior is covered by the live cockpit Settings test and the refreshed card contract.
- No `.hiveai/PROJECT.json` runtime error or legacy runtime dependency was reintroduced.

## Publication

- Implementation SHA: `62776206395832ba7eaceb28cca3343f12a92797`.
- Stable published executable: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI\dev-bin\H!veAI.exe`.
- Owner-facing synchronized executable: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI\dev-bin\H!veAI.exe`.
- Stable EXE SHA-256: `200D316F5A2D87CEFEE3FBCF9C9BCBA12CF43426138F5D1FFAAA6EEB7BFC45BE`.
- This log is immutable after publication. Its commit SHA and final `origin/main` HEAD are recorded in the final publication update immediately after this file is committed.
