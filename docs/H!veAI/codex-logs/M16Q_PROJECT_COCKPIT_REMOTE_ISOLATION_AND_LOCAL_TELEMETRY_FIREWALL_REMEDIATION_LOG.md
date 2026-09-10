# M16Q Project Cockpit Remote Isolation and Local Telemetry Firewall Remediation Log

## Scope

- Authoritative prompt: `M16Q_PROJECT_COCKPIT_REMOTE_ISOLATION_AND_LOCAL_TELEMETRY_FIREWALL_REMEDIATION_PROMPT.md`.
- Repository: `Sekiph82/AI-Commerce-HQ`.
- Branch: `H!veAI`.
- Local starting HEAD before synchronization: `baab1550d3cdf1dd7f77e86c4a86e391fa21b92a`.
- Synchronized authoritative origin HEAD before implementation: `080daf97c44d74f3a16cc7bfbf4ee781916f1b21`.
- Scoped findings closed together: M16P-R48 and M16P-R49 only.
- M16 remains open for independent strict re-audit and owner native acceptance.

## R48 - Remote-primary Project Cockpit

`H!veAI/src-tauri/src/project_cockpit.rs` now has an explicit production branch for every GitHub v3 project. It loads the cached GitHub remote snapshot first and constructs the primary cockpit fields directly from that snapshot. Remote refresh failure becomes a truthful remote unavailable/error result with cached truth retained where available; it does not cause the cockpit command to fail.

The remote-primary branch never uses `?` on local dashboard, control-plane, truth-resolution, task-intelligence, workflow, Git, task-source, audit, test, session, or permission readers. Those readers are best effort only. Their values and failures are kept in neutral secondary fields or `localWorkspaceTelemetry`; they cannot prevent a valid remote cockpit or replace remote repository, branch, HEAD, health, milestone, current task, workflow, actor, next action, blockers, progress, or last-completed-task values.

The remote projection supplies the dashboard, control-plane, truth, activity, files, task summary, and primary project summary from the GitHub v3 snapshot. No synthetic local truth is created. Non-v3 projects retain the prior legacy path.

## R49 - Primary Warning Firewall

For GitHub v3 projects, `warnings` is populated only from the remote projection. Local dashboard schema warnings, malformed local control-plane warnings, truth resolver errors, task-intelligence failures, workflow warnings, Git dirty/non-Git/diff failures, and task-source errors are collected only in `localWorkspaceTelemetry.warnings` or other secondary telemetry fields.

Local telemetry cannot affect primary health, attention, workflow, actor, current task, milestone, next action, progress, or remote synchronization state. Remote stale and invalid cases remain remote-derived and are never replaced by valid-looking local data.

## Direct Production-Branch Tests

The following tests call the real `project_cockpit::snapshot` production branch for a registered GitHub v3 project:

- `m16q_remote_primary_survives_broken_local_telemetry_and_firewalls_warnings`: healthy remote plus missing local root and local Git/telemetry failures still returns the exact remote task and leaves primary warnings empty.
- `m16q_stale_and_invalid_remote_truth_never_promotes_local_values`: stale cached remote truth remains visible with explicit STALE state, and invalid remote truth remains an ERROR without promoting local values.
- `m16q_malformed_local_control_plane_and_stale_dashboard_cannot_contaminate_remote_view`: malformed local control-plane data and stale local PAG-M02 cannot replace remote PAG-M05 or enter primary warnings.
- `m16q_command_center_and_cockpit_share_remote_primary_semantics`: Command Center and Project Cockpit expose matching milestone, task, workflow, progress, and health semantics from one cached remote snapshot.

These four production tests cover the eight required direct cases: missing local directory, malformed local files, remote PAG-M05 versus local PAG-M02, local Git failure, stale remote cache, invalid remote data, Command Center/Cockpit consistency, and the complete primary-warning firewall.

## Preserved M16P Boundaries

The existing M16P cache-first startup, asynchronous GitHub polling, selected-project 10-second cadence, portfolio 30-second cadence, per-project in-flight suppression, bounded four-worker concurrency, refresh coalescing, unchanged-HEAD cache reuse, degraded backoff, hidden Git child processes, bounded 30-second Git execution, and canonical opening-video bytes were not redesigned or weakened.

## Validation

- Focused M16Q tests: PASS, 4 passed, 0 failed.
- `cargo check --lib`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo test --all-targets -- --test-threads=1`: PASS, 415 passed, 0 failed.
- `cargo test --all-targets --features pty-support -- --test-threads=1`: PASS, 416 passed, 0 failed.
- `npm test -- --run`: PASS, 15 files and 125 tests passed.
- `npm run typecheck`: PASS.
- `npm run build`: PASS.
- `npm audit --audit-level=high`: PASS at the requested threshold; the existing report contains two moderate Vitest advisories and no high or critical advisory.
- `git diff --check`: PASS; only the normal Git LF/CRLF warning was emitted for the edited Rust file.
- `scripts/tests/publish-dev-qa-failure-harness.ps1`: PASS, 9/9 rollback, cleanup, lock, smoke-failure, and no-bypass cases.

## Native Smoke and Publication

Governed `scripts/publish-dev-qa.ps1` rebuilt, smoke-tested, and published the stable executable at `H!veAI/dev-bin/H!veAI.exe`.

The explicit post-publication native smoke reported:

`alive=True|title=H!veAI|elapsedMs=3075|newVisibleConhost=0|frontendReadyMarker=True`

The stable executable SHA-256 is:

`873E17968C8876D58CA8655A880DE803CBE604393F34C81EA342A362ED7E46E5`

The canonical opening video remained unchanged with SHA-256:

`C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`

## Publication Identity

- Implementation commit after safe rebase: `bf2580f0f4bbc203f251843a292d414c83c44958`.
- Files changed for this remediation: `H!veAI/src-tauri/src/project_cockpit.rs` and this immutable log.
- Final branch HEAD is the immutable log publication commit created after the implementation commit; its exact SHA and equality with `origin/H!veAI` are proven after push in the final response. The pre-existing user-owned untracked paths `.hiveai/EVENT_INDEX.json`, `.hiveai/HANDOFF.md`, `.hiveai/STATE.json`, `start-demo.bat`, and `task.md` were not modified or committed.
- M17 was not activated.
- M21 was not started.

M16Q REMOTE PROJECT COCKPIT ISOLATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE ACCEPTANCE.
M16 remains OPEN.
M17 NOT ACTIVATED.
M21 NOT STARTED.
