# M16P Remote-Only Primary Truth and Live GitHub Polling — Independent Strict Re-Audit

Date: 2026-09-10
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16P_REMOTE_ONLY_PRIMARY_TRUTH_AND_LIVE_GITHUB_POLLING_CLOSURE_LOG.md`
Implementation commit: `5831a58f814e36e951ab8beaff05065a330b4199`
Log commit / reviewed branch head: `baab1550d3cdf1dd7f77e86c4a86e391fa21b92a`

## Verdict

**FAIL / CHANGES REQUIRED**

- BLOCKER: 1
- MAJOR: 1
- MINOR: 0

M16 remains OPEN. M17 must not activate. M21 must not start.

## What is genuinely fixed

The M16P implementation materially improves the GitHub-first architecture:

- a background `GitHubTrackingManager` exists;
- selected-project cadence is 10 seconds and portfolio cadence is 30 seconds;
- requests coalesce and per-project observations are in-flight bounded;
- unchanged remote HEAD reuses the cached snapshot;
- Git subprocesses use the hidden/background process policy and a 30-second timeout;
- startup no longer launches synchronous network observation;
- Command Center has a GitHub-v3 remote-primary path;
- remote health now treats `workflowState=BLOCKED` as BLOCKED even when `blockers=[]`;
- builder regression and publication evidence is internally consistent, including zero visible conhost in its Windows smoke.

These closures are accepted for M16O-R43, R45, R46 and the visible-process/startup portion of R47.

---

# M16P-R48 — BLOCKER
## Project Cockpit is still structurally dependent on local/legacy subsystems before remote truth can be returned

`project_cockpit::snapshot()` fetches the GitHub cached snapshot, but then unconditionally executes legacy/local resolvers before constructing the remote-primary projection:

- `project_dashboard::resolve(...)` with `?`
- `ProjectTruthResolver::resolve(...)` with `?`
- `control_plane::snapshot(...)` with `?`
- task intelligence
- workflow project list/history with `?`
- local Git snapshot/diff
- task source discovery
- local DB tests/audits/sessions/permissions/activity/files

For a GitHub-v3 project this means a failure in local Project Dashboard, local control-plane truth, workflow DB state, or other legacy dependencies can still fail the entire Project Cockpit command before the valid GitHub remote projection reaches the UI.

This violates the owner-locked contract that GitHub is authoritative and local state is optional secondary telemetry only.

A GitHub-v3 project with a valid cached remote snapshot must remain fully usable even if the local project folder is missing, stale, malformed, moved, non-Git, dirty, or otherwise unreadable.

### Required closure

Introduce a genuine remote-primary snapshot path for GitHub-v3 projects that cannot be failed by local telemetry. Remote state must be constructed first and returned independently. Any local/legacy telemetry must be optional, isolated, lazily/best-effort collected, and must never gate the primary Project Cockpit response.

Direct regression test required:

> valid remote v3 cache + intentionally broken/missing local control-plane/dashboard/workflow/git sources => Project Cockpit still succeeds and displays the exact remote task/milestone/next action/progress.

---

# M16P-R49 — MAJOR
## Local telemetry errors still leak back into the primary `warnings` collection

The implementation correctly captures `local_warnings`, clears primary warnings when a GitHub snapshot exists, and creates `local_workspace_telemetry`.

However, after that separation, it loops over:

- `task_intelligence_error`
- `git_error`
- `git_diff_error`
- `sources_error`

and calls `push_warning(&mut warnings, error)`.

Therefore local task/Git/source errors are reintroduced into the primary Project Cockpit warning stream even for GitHub-v3 projects.

This can recreate exactly the class of native UX the owner rejected: valid remote GitHub state accompanied by local `malformed`, `dirty`, missing task source, local Git, or reconciliation warnings in the primary project surface.

### Required closure

For GitHub-v3 projects:

- primary `warnings` may contain only GitHub remote/canonical-v3 validation/availability warnings;
- every local-only warning/error must live exclusively under `localWorkspaceTelemetry` or another explicitly secondary technical structure;
- local warning content must not affect primary health, needs-attention, current task, milestone, next action, progress, or remote status.

Add direct tests proving local telemetry failures cannot appear in the primary warnings array for a healthy remote v3 project.

---

## Startup regression assessment

The specific owner-reported startup regression is technically addressed in source:

- `GitHubTrackingManager::start()` performs portfolio registration and starts the background scheduler rather than running remote observations synchronously;
- Git operations are worker-thread operations;
- `run_git()` uses `process_policy::background_command("git")`;
- each Git process has null stdin, captured output, bounded timeout, kill/wait cleanup;
- the builder reports a Windows native launch with `newVisibleConhost=0` and frontend-ready in approximately 3.1 seconds.

This still requires owner native confirmation because the reported failure was native/visual.

## Final status

M16P is not acceptable as final M16 closure while R48/R49 remain.

Required next action: one bounded remediation must close R48 and R49 together, preserve the completed GitHub polling/startup/process fixes, republish, and then return for owner native verification.

**M16P FAIL / CHANGES REQUIRED.**
