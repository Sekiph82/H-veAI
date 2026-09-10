# M16P — Remote-Only Primary Truth + Live GitHub Polling Closure

## Execution mode

Read and obey:

1. `H!veAI/GPT.md`
2. `H!veAI/docs/H!veAI/GITHUB_FIRST_PROJECT_TRACKING_CONTRACT_V3.md`
3. `H!veAI/docs/H!veAI/audits/M16O_GITHUB_FIRST_EIGHT_REPOSITORY_TRACKING_RESET_INDEPENDENT_STRICT_REAUDIT.md`

Close together in one continuous run:

- M16O-R43 BLOCKER
- M16O-R44 BLOCKER
- M16O-R45 BLOCKER
- M16O-R46 MAJOR
- M16P-R47 BLOCKER: native startup is blocked by synchronous/visible Git child processes after M16O

Do not stop after individual findings.
Do not activate M17.
Do not start M21.
M16 remains OPEN until independent re-audit + owner native acceptance.

---

# 1. Product invariant

For every migrated `hiveai-project/v3` repository:

> GitHub remote tracked branch is the only primary project truth.

Primary truth means:

- current milestone
- current sprint
- current task ID/title
- workflow state
- required actor
- next action
- blockers
- exact progress
- last completed task
- tracker update time
- project health
- Command Center attention/queue metrics

Local project files, local dashboard/control-plane state, local task intelligence, local workflow, local Git dirty/clean, and local filesystem watcher events must never override or contaminate these fields.

Local data may remain only under explicit secondary technical telemetry.

---

# 2. Implement a real GitHub polling scheduler

The current runtime is request-driven. Replace this gap with one dedicated GitHub tracking scheduler.

## Required schedule

- startup: launch H!veAI immediately and begin GitHub refresh asynchronously after the native shell/frontend is responsive;
- selected project: remote HEAD check every 10 seconds;
- other portfolio projects: remote HEAD check every 30 seconds;
- manual Refresh: immediate forced refresh;
- bounded exponential/backoff behavior on network failure;
- no dependency on local filesystem activity.

## Required architecture

Add a dedicated manager/service, e.g. `GitHubTrackingManager`, owned by Tauri application state.

It must:

1. maintain selected project identity;
2. maintain per-project next refresh deadline;
3. resolve remote branch HEAD;
4. compare HEAD with cached remote HEAD;
5. if unchanged, avoid reparsing all four blobs unnecessarily;
6. if changed, fetch all canonical blobs from the same resolved commit;
7. validate v3 contract;
8. atomically persist the new remote snapshot;
9. emit `github-tracking-updated` with project ID and remote HEAD;
10. emit portfolio refresh signal for Command Center/Cockpit subscribers.

Do not implement this by repurposing the local `WatcherManager`.

The local watcher may remain for local execution telemetry, but it is not the GitHub project-truth scheduler.

## Cache semantics

The current universal 30-second freshness cache must not defeat the selected-project 10-second refresh requirement.

Separate:

- remote HEAD observation cadence;
- blob cache reuse.

A 10-second HEAD observation may reuse cached blobs only when the remote HEAD is unchanged.

---

# 2A. BLOCKER: restore instant native startup and eliminate visible Git console storms

Owner-native reproduction after M16O:

- launching the desktop `H!veAI.exe` no longer immediately reaches the accepted startup video/application;
- the application title becomes `H!veAI (Not Responding)`;
- Windows Terminal / Git command windows repeatedly appear, with the shell showing a Git executable path such as `C:\Program Files\Git\cmd\git...`;
- dozens of Git child launches can occur before the UI becomes usable.

Treat this as a release-blocking regression introduced by GitHub-first remote observation.

The current implementation uses repeated `std::process::Command::new("git")` remote operations and performs remote refresh from synchronous snapshot paths. This must not block Tauri startup, the webview/UI thread, or native IPC handling.

Required corrections:

1. **No network Git operation may execute synchronously on the Tauri/UI command thread.**
   - Move remote Git work to a dedicated worker/runtime using `spawn_blocking`, a worker thread, or an equivalent bounded asynchronous execution model.
   - Tauri snapshot commands must return promptly from cached state and may trigger/await bounded background refresh only when explicitly appropriate.

2. **Startup must never wait for all eight repositories.**
   - `setup()` may seed static registry identity and initialize the scheduler only.
   - Do not fetch eight repositories inside blocking app startup.
   - Accepted startup video/window must appear first, as it did before M16O.
   - Initial GitHub refresh runs in the background after the app shell is ready.

3. **Git child processes must be hidden on Windows.**
   - Use Windows process creation flags or the project's standard hidden-process helper so `git.exe`, `git.cmd`, `cmd.exe`, PowerShell, or Windows Terminal windows never flash/open for background tracking.
   - Do not solve this by globally suppressing the H!veAI application window or by disabling the startup video.

4. **Bound concurrency.**
   - Do not launch all Git subcommands for all repositories at once.
   - Use a small bounded worker pool / semaphore and deduplicate in-flight refreshes per repository+branch.
   - A scheduled poll must never overlap another refresh for the same project.

5. **Use a lightweight HEAD check.**
   - Polling should first use one lightweight remote HEAD observation, e.g. `git ls-remote` or a GitHub API equivalent.
   - Only when HEAD changes should the canonical v3 blobs be fetched/read.
   - Do not execute repeated remote-remove/remote-add/fetch/show/rev-parse chains every 10 seconds when HEAD is unchanged.

6. **No terminal dependency.**
   - H!veAI must not require Windows Terminal to provide GitHub tracking.
   - Background tracking processes must have no visible console and no interactive prompt.
   - Authentication/network failures must become bounded application state, not a child terminal waiting for input.

7. **Timeout and cancellation.**
   - Every remote observation has a bounded timeout.
   - Scheduler shutdown/restart cancels or safely drains work.
   - Hung Git/network operations must never cause Windows to mark H!veAI `Not Responding`.

8. **Cache-first native UX.**
   - On startup immediately display the last successful remote snapshot if available and mark it `REFRESHING`/`STALE` as appropriate.
   - Replace it atomically when the background GitHub observation succeeds.
   - With no cache, show `Loading GitHub state…` without freezing the app.

9. **Do not regress the accepted startup experience.**
   - Preserve `H!veAI/src/assets/H!veAI.mp4` startup behavior and existing no-console native publication guarantees.
   - Do not restore `opening-video.mp4`.

Required direct native regression test/harness:

- launch stable `H!veAI.exe` with all eight repositories registered;
- verify the native window becomes responsive promptly and startup video/application shell is visible without waiting for network completion;
- verify no visible Git/cmd/PowerShell/Windows Terminal child window appears;
- artificially delay one remote Git operation and prove UI remains responsive;
- artificially hang/fail one remote operation and prove timeout/degraded state rather than application hang;
- prove no duplicate in-flight refresh occurs for the same repository;
- prove an unchanged HEAD performs no blob refetch chain;
- prove the publisher `no visible console host` guarantee still passes after the scheduler is enabled.

This blocker must be closed in the same M16P run. Do not ask the owner to tolerate the startup regression as a side effect of live GitHub tracking.

---

# 3. Make Project Cockpit remote-primary

Create an explicit remote-primary cockpit projection for v3 projects.

The primary cockpit must be built directly from one `RemoteTrackingSnapshot`.

Primary fields:

- repository
- branch
- remote HEAD
- remote health
- fetchedAt
- tracker updatedAt/updatedBy
- current milestone
- current sprint
- current task
- workflow state
- required actor
- next action
- blockers
- progress scope/completed/total/percent
- last completed task

## Local separation

Move all local-only structures under one optional secondary structure, e.g.:

`localWorkspaceTelemetry`

This may contain:

- local path
- local branch/HEAD
- local Git status
- local test runs
- local sessions
- local audits
- execution telemetry

But local telemetry must not affect the primary current project truth or health.

## Warning isolation

If remote v3 is valid/current:

- do not surface old local PROJECT schema warnings in the primary warning banner;
- do not surface local STATE/HANDOFF/progress reconciliation warnings as remote project warnings;
- do not show local milestone/current task beside remote milestone/current task.

Local warnings belong only in the collapsed technical/local section.

---

# 4. Rebuild Command Center from remote snapshots

For migrated v3 projects, do not call the old local summary pipeline as the primary data source and then overlay a few remote fields.

Build one `RemoteProjectOperationSummary` or equivalent directly from remote tracker snapshots.

## Remote-only portfolio metrics

For v3 projects:

- project count = registered GitHub projects;
- current task = remote TASKS v3 header;
- progress = remote exact progress only;
- active work = remote workflow/current task only;
- completed count = only when exact remote tracker counts support it;
- needs attention = remote health/workflow/blockers/required actor;
- active work queue = remote current task + next action;
- recent tracking activity = canonical remote EVENTS tail;
- warnings = remote fetch/schema/tracker validation only.

Do not use local M08/M09/M10/project-dashboard/task-intelligence values for migrated v3 project KPIs.

If an exact KPI is not represented by the v3 tracker, show unavailable rather than deriving from legacy local data.

## Legacy projects

If future projects are not migrated to v3, they may enter an explicitly labeled LEGACY mode.

Legacy mode must never contaminate the v3 portfolio truth calculations silently.

---

# 5. Fix remote health semantics

Health must use remote v3 workflow semantics.

Required mapping principles:

- remote unavailable, no cache -> UNAVAILABLE
- stale cached remote -> STALE
- malformed/inconsistent v3 -> ERROR
- workflow BLOCKED -> BLOCKED / ATTENTION
- blockers non-empty -> BLOCKED / ATTENTION
- awaiting owner -> WAITING_OWNER / ATTENTION
- awaiting independent audit -> WAITING_AUDIT / ATTENTION or accepted waiting status
- implementation ready/runnable and no blockers -> HEALTHY

A remote project cannot be HEALTHY merely because `blockers=[]` when `workflowState=BLOCKED`.

Add exact Bulk-Edit fixture:

- workflowState = BLOCKED
- requiredActor = OWNER
- blockers = []

Expected primary health != HEALTHY.

---

# 6. Frontend live refresh

Update the frontend to subscribe to `github-tracking-updated` / portfolio refresh events.

Additionally ensure the manager itself owns polling, so refresh continues even if Command Center is not the active screen.

When a project update event arrives:

- refresh Command Center snapshot if mounted;
- refresh the matching Project Cockpit if mounted;
- preserve user selection/scroll state where practical.

Manual Refresh must force a remote observation immediately.

Do not require page reload.

---

# 7. Eight-repository remote fixtures

Re-verify the exact current tracked branches:

1. `Sekiph82/AI-Commerce-HQ` / `H!veAI`
2. `Sekiph82/Bulk-Edit` / `main`
3. `Sekiph82/fmcg-erp-system` / `main`
4. `Sekiph82/FormuLab` / `feature/laboratory-stability`
5. `Sekiph82/PackLab` / `main`
6. `Sekiph82/PackLab-3D` / `main`
7. `Sekiph82/Scrubbots` / `main`
8. `Sekiph82/ScrubBots-Level-Factory` / `main`

All must still contain:

- `hiveai-project/v3`
- `.hiveai/TASKS.md` v3 machine header
- GitHub-first v3 RULES
- canonical v1 EVENTS

Do not overwrite their task truth merely to satisfy tests.

Use current branch state as fixture truth.

Important current regression examples:

- ScrubBots is beyond historical SB-M02-017 and must use the current remote M19 chain.
- Level Factory is currently beyond the old PAG-M02/PAG-M05 stale states and must reflect the actual current remote tracker at verification time.
- Bulk-Edit BLOCKED/OWNER state must not be rendered HEALTHY.

---

# 8. Direct release-gate tests

Add direct tests proving:

1. H!veAI starts with remote project A at HEAD1.
2. A new GitHub/upstream commit changes `.hiveai/TASKS.md` to HEAD2.
3. No local project file changes occur.
4. No manual Refresh is pressed.
5. polling observes HEAD2 and emits update event.
6. Command Center updates to HEAD2 truth.
7. Project Cockpit updates to HEAD2 truth.

Also test:

- selected project refresh deadline is 10 sec;
- non-selected project refresh deadline is 30 sec;
- unchanged HEAD avoids unnecessary blob reload;
- manual refresh bypasses freshness delay;
- offline cache becomes STALE with last-success timestamp;
- local old PAG-M02 + remote current Level Factory tracker -> remote wins;
- local malformed PROJECT + remote valid v3 -> no primary local-schema warning;
- local workflow/task intelligence cannot alter remote current task;
- Command Center KPI/attention/queue are remote-derived;
- Bulk-Edit BLOCKED fixture not HEALTHY;
- all eight actual remote v3 tracker parser fixtures pass;
- native startup does not block on remote refresh;
- no visible Git/terminal child process is created during startup or polling;
- a deliberately slow GitHub observation cannot make the H!veAI native window unresponsive.

---

# 9. Remove misleading primary UX

In primary Command Center/Cockpit presentation, remove/demote wording that implies local workspace state is project truth.

Primary UI should clearly identify:

`Source: GitHub <owner>/<repo>@<branch>`

and show:

- remote HEAD short SHA
- last GitHub refresh
- tracker updated time
- remote status

Local workspace data may exist only under a clearly separate collapsed technical section.

The owner must not need to interpret `dirty`, local watcher modes, legacy control-plane reconciliation, or local dashboard authority to understand project status.

---

# 10. Existing old control-plane code

Do not perform an unnecessary destructive rewrite of old M08-M16 modules if they are still needed for agent execution, Prompt Engine, Audit Engine, tests, or local telemetry.

But establish a hard boundary:

- GitHub v3 portfolio tracking -> remote-only primary truth
- agent/audit/local execution systems -> secondary operational machinery

No accidental cross-feed into primary project state.

Document this boundary in architecture docs.

---

# 11. Full verification

Run:

- focused GitHub scheduler tests;
- remote HEAD-change tests;
- cache tests;
- startup non-blocking/background-worker tests;
- hidden-child-process tests on Windows;
- hung/slow Git timeout tests;
- in-flight deduplication tests;
- Command Center remote-only tests;
- Project Cockpit remote-only tests;
- all-eight v3 contract tests;
- local-vs-remote conflict tests;
- frontend event-refresh tests;
- M16 audit-engine regressions;
- full Rust all-targets serialized;
- pty-support suite;
- full frontend Vitest once;
- TypeScript typecheck;
- production build;
- npm audit high;
- cargo fmt;
- git diff --check;
- publisher rollback 9/9;
- governed publication;
- stable candidate SHA equality;
- native smoke including responsive startup and zero visible Git/terminal windows.

Then perform a fresh whole-M16 adversarial sweep focused on the owner's actual product questions:

> If GitHub changes and local files do not, will H!veAI automatically and exclusively show the new GitHub truth?

> Can H!veAI launch immediately and remain responsive while that GitHub tracking happens entirely in the background with zero visible child consoles?

If any BLOCKER or MAJOR still violates either statement, fix it in this same run.

---

# 12. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16P_REMOTE_ONLY_PRIMARY_TRUTH_AND_LIVE_GITHUB_POLLING_CLOSURE_LOG.md`

Include:

- starting HEAD
- R43-R47 reproduction
- scheduler architecture
- proof startup no longer blocks on remote work
- proof no Git/cmd/PowerShell/Terminal windows are visible
- proof slow/hung Git cannot freeze UI
- selected 10s / portfolio 30s proof
- remote HEAD-change E2E proof
- remote-only Command Center proof
- remote-only Project Cockpit proof
- local-warning isolation proof
- Bulk-Edit health proof
- eight remote tracker verification matrix
- frontend event-refresh proof
- full test counts
- publication SHA
- final pushed HEAD
- owner native acceptance pending

End exactly:

`M16P REMOTE-ONLY GITHUB TRACKING CLOSURE COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
