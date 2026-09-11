# M21-R02 Cockpit Tasks and AI-Commerce Retirement V01

Status: `COMPLETE_AWAITING_OWNER_REACCEPTANCE`

## Scope and starting point

- Work item: `M21-R02`.
- Starting synchronized `main` SHA: `fade69d76e667cc2065d2c3ab8051833862fd87e`.
- The local standalone workspace was fast-forward synchronized with `origin/main` before implementation. No reset, rebase, force-push, stash, clean, or destructive reconciliation was used.
- The local AI-Commerce parent directory and the GitHub repository `Sekiph82/AI-Commerce-HQ` were preserved. No local or remote deletion was performed.

## F01: Cockpit Tasks canonical remote rows

Root cause: the remote GitHub tracker already parsed root `TASKS.md` summary fields, but `project_cockpit::snapshot_remote_primary` deliberately projected `task_intelligence` as `None` and an empty legacy workflow list. `CockpitLiveTasks` rendered only that legacy task-intelligence list, so a reachable GitHub project could truthfully have current remote summary data while its Tasks view displayed an unavailable/empty state.

Remediation:

- `github_tracking::parse_root_tasks` now materializes bounded canonical `TASKS.md` checklist rows with stable task id, title, status, source path, and source line metadata in `RemoteTrackingSnapshot.task_rows`.
- `project_cockpit::snapshot_remote_primary` projects those rows as `remote_tasks` without reviving `.hiveai`, `PROJECT.json`, or local task-intelligence dependencies.
- The remote Cockpit Tasks surface renders `remote_tasks` directly and keeps current/next/blocker/health values on the same remote snapshot. Empty and degraded remote states remain explicit and bounded.
- Command Center, Cockpit Overview, and Cockpit Tasks therefore use the same fetched remote snapshot and remote HEAD semantics.

Evidence:

- `github_tracking::tests::root_tasks_parser_materializes_current_state_and_exact_counts` passed.
- `project_cockpit::tests::m21r02_remote_tasks_materialize_when_legacy_intelligence_is_empty` passed.
- `project_cockpit::tests::m21r02_remote_empty_tasks_is_truthful_and_not_legacy_error` passed.
- `project_cockpit::tests::m16q_command_center_and_cockpit_share_remote_primary_semantics` passed, including row identity, status, source, and count parity.

## F02: eight-project portfolio retirement boundary

Before remediation, `github_tracking::ensure_portfolio` declared nine default targets, including `Sekiph82/AI-Commerce-HQ`, and retained special branch-selection behavior for that repository. The target set is now exactly eight repositories, with `Sekiph82/H-veAI` and the other seven current GitHub projects as the only default portfolio identities. The obsolete AI-Commerce-HQ target and its special branch path were removed from the default portfolio definition.

Existing persisted state remains safe: portfolio reconciliation still merges duplicate repository identities and archives non-target persisted rows through the existing registry behavior. No delete operation was added, and AI-Commerce-HQ was not deleted locally or on GitHub.

Evidence:

- `github_tracking::tests::ensure_portfolio_merges_legacy_repository_rows_into_one_remote_identity` passed with eight active portfolio projects and no AI-Commerce-HQ target.
- `github_tracking::tests::ensure_portfolio_archives_non_portfolio_persisted_rows` passed with the corrected eight-project active set.
- Fresh and existing-state portfolio expectations in the project cockpit tests passed.

## Tracker truth

Root `TASKS.md` now records M21-R02 as the active implementation-complete remediation awaiting independent audit and owner native re-acceptance. Historical M21 and M21-R01 closure truth remains intact, and the user-facing roadmap denominator was not changed. Final waiting actor is HUMAN because owner native acceptance and independent strict audit remain outstanding; CODEX was the implementation actor for this remediation.

## Regression and publication gates

- Focused Rust M21-R02 tests: passed.
- GitHub tracking tests: 6 passed.
- Bounded full Rust library regression: 415 passed, 0 failed, with only the three pre-existing long-running observation/recovery tests excluded from that bounded sweep.
- Frontend test suite: 15 files / 125 tests passed.
- `npm run typecheck`: passed.
- `npm run build`: passed.
- `git diff --check`: passed.
- Existing `scripts/publish-dev-qa.ps1`: passed production Tauri build, candidate smoke, stable smoke, no forbidden development ports, no visible console host, frontend readiness, stable PE validation, and shortcut target/icon validation.
- Published executable SHA-256: `7B73704DBB0C7B1310A7E58E3EEA2249C61C973D7C9916B642269659829CF4E4`.
- Implementation commit: `d8e7cb1`.

The required post-publication equality gate is rerun after this log is pushed; its exact local `HEAD`, `origin/main`, and remote GitHub `main` SHA values are returned with the completion response. This log intentionally does not contain its own creating commit SHA.

## Acceptance boundary

`OWNER_REACCEPTANCE_REQUIRED`: native visual acceptance of the corrected eight-project portfolio, live GitHub refresh behavior, Cockpit Tasks content, and the explicit retirement decision for AI-Commerce-HQ remain with the owner/independent auditor. The parent directory and `Sekiph82/AI-Commerce-HQ` remain preserved by design.
