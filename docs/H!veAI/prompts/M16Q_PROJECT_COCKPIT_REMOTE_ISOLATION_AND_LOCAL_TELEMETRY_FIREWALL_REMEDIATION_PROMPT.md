# M16Q — Project Cockpit Remote Isolation and Local Telemetry Firewall Remediation

## Authority

Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`

Read and obey:

1. `H!veAI/GPT.md`
2. `H!veAI/docs/H!veAI/GITHUB_FIRST_PROJECT_TRACKING_CONTRACT_V3.md`
3. `H!veAI/docs/H!veAI/audits/M16P_REMOTE_ONLY_PRIMARY_TRUTH_AND_LIVE_GITHUB_POLLING_INDEPENDENT_STRICT_REAUDIT.md`

This is a bounded closure remediation for M16P-R48 and M16P-R49 only.

Do not reopen already closed GitHub polling/startup/process fixes.
Do not activate M17.
Do not start M21.
Do not close M16 yourself.

## Product invariant

For every migrated GitHub-v3 project:

> The GitHub remote snapshot is sufficient by itself to render the primary Project Cockpit.

A missing, stale, dirty, malformed, moved, non-Git, or otherwise broken local workspace must never prevent the owner from seeing the exact GitHub current milestone/task/next action/progress.

Local data is optional telemetry only.

---

# R48 — Remove structural local dependency from GitHub-v3 Project Cockpit

Current production `project_cockpit::snapshot()` still unconditionally calls legacy/local subsystems before returning the remote projection.

Refactor the production path so GitHub-v3 projects use an explicit remote-primary branch.

## Required behavior

When `github_tracking::is_github_v3_project(&project)` is true:

1. Load the cached GitHub v3 snapshot.
2. Construct all primary Project Cockpit state directly from that remote snapshot.
3. Return a valid cockpit even if every local/legacy subsystem fails.
4. Local telemetry may be gathered best-effort, but no local error may cause the command to return `Err`.
5. Do not call any local resolver with `?` on the remote-primary path if that resolver can fail independently of GitHub truth.

The following must NOT gate a GitHub-v3 cockpit:

- `project_dashboard::resolve`
- `ProjectTruthResolver::resolve`
- `control_plane::snapshot`
- task intelligence
- workflow project list/history
- local Git snapshot/diff
- task-source discovery
- local DB audit/test/session/history data

If the existing `ProjectCockpitSnapshot` type requires these legacy fields, choose one safe design:

A. make the legacy/secondary fields optional for remote-v3 projects; or
B. populate neutral secondary placeholders that cannot be mistaken for primary truth.

Prefer the cleaner explicit optional/secondary model if frontend changes are manageable.

Do not synthesize fake local values.

## Remote-primary fields

For GitHub-v3 projects, these must come only from `RemoteTrackingSnapshot`:

- repository
- tracked branch
- remote HEAD
- remote health
- fetched/update timestamp
- current milestone
- current sprint
- current task ID/title
- workflow state
- required actor
- next action
- blockers
- progress scope/count/percent
- last completed task

No local subsystem may override them.

---

# R49 — Build a hard warning firewall

For a GitHub-v3 project, the primary Project Cockpit warning list may contain ONLY:

- GitHub remote unavailable/stale/error state
- remote PROJECT v3 validation failure
- remote TASKS v3 validation failure
- remote RULES/EVENTS contract validation failure
- other remote canonical-v3 errors

The following must never appear in the primary warning list:

- local PROJECT.json schema warnings
- local progress-scope warnings
- local Project Dashboard warnings
- local control-plane warnings
- local task-intelligence errors
- local Git dirty/non-Git/diff errors
- local task-source errors
- local workflow reconciliation warnings

Put local warnings exclusively in a clearly secondary structure such as `localWorkspaceTelemetry.warnings`.

Local warnings must not affect:

- primary health
- attention status
- workflow
- required actor
- current task
- milestone
- next action
- progress
- remote sync state

---

# Frontend

Inspect the actual Project Cockpit UI.

For GitHub-v3 projects the main Overview must render from `remotePrimary` / GitHub snapshot fields.

Do not render legacy local dashboard/control-plane warnings in the main banner.

If local telemetry is shown at all, put it under a collapsed secondary section titled something like:

`Local workspace (technical)`

The owner should not need to understand dirty/clean/local reconciliation to use H!veAI.

Do not redesign unrelated UI.

---

# Direct regression tests

Add tests that use the real production branch, not only helper-function tests.

Required cases:

1. **Remote healthy + missing local directory**
   - valid GitHub-v3 cached snapshot
   - local path missing
   - Project Cockpit succeeds
   - exact remote current task is returned

2. **Remote healthy + malformed local old control-plane files**
   - primary cockpit still succeeds
   - primary warnings contain no local schema/reconciliation warning

3. **Remote PAG-M05 + local stale PAG-M02**
   - primary cockpit returns PAG-M05
   - PAG-M02 does not appear in current milestone/task/next action

4. **Remote healthy + local Git failure**
   - primary health remains remote-derived
   - local Git error exists only in secondary telemetry

5. **Remote unavailable + cached stale snapshot**
   - cockpit still returns cached remote truth with explicit STALE status
   - local state is not substituted

6. **Remote invalid and local valid**
   - show remote canonical failure
   - never promote local truth

7. **Command Center/Cockpit consistency**
   - same cached remote snapshot produces same milestone/task/workflow/progress/health semantics

8. **Primary warnings firewall**
   - inject local dashboard, task intelligence, source and Git errors
   - assert none enter primary warnings for GitHub-v3 project

---

# Preserve M16P startup/polling fixes

Do not regress:

- cache-first startup
- no synchronous network/Git observation on startup/UI command path
- selected project 10-second cadence
- portfolio 30-second cadence
- per-project in-flight suppression
- bounded four-worker concurrency
- refresh coalescing
- unchanged-HEAD cache reuse
- backoff
- hidden Git child processes
- 30-second Git process timeout
- opening video bytes

Add regression coverage if refactoring touches these boundaries.

---

# Validation and publication

Run at minimum:

- focused Project Cockpit remote-isolation tests
- focused warning-firewall tests
- GitHub tracking tests
- Command Center remote-primary tests
- all Rust tests
- all-targets
- pty-support
- frontend Vitest
- TypeScript typecheck
- production build
- npm audit high threshold
- cargo fmt check
- git diff --check
- publisher rollback harness 9/9
- governed publication
- native startup smoke

Native smoke must again prove:

- H!veAI launches responsively
- frontend ready marker appears
- zero visible conhost/cmd/PowerShell/Terminal windows created by background GitHub tracking
- canonical opening video remains unchanged

Record candidate/stable SHA-256.

---

# Required immutable log

Create:

`H!veAI/docs/H!veAI/codex-logs/M16Q_PROJECT_COCKPIT_REMOTE_ISOLATION_AND_LOCAL_TELEMETRY_FIREWALL_REMEDIATION_LOG.md`

Log must include:

- starting HEAD
- files changed
- exact R48 design
- exact R49 warning firewall
- direct test bodies/results
- startup/polling regression evidence
- publication evidence
- stable EXE SHA
- implementation commit
- final branch HEAD

End exactly:

`M16Q REMOTE PROJECT COCKPIT ISOLATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
