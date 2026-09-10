# M16O GitHub-First Eight-Repository Tracking Reset — Independent Strict Re-Audit

Date: 2026-09-10  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16O_GITHUB_FIRST_EIGHT_REPOSITORY_TRACKING_RESET_LOG.md`

## Verdict

**FAIL / CHANGES REQUIRED**

- BLOCKER: 3
- MAJOR: 1
- MINOR: 0

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap progress remains `16 / 20 = 80%`.

The eight repositories were materially migrated to the new GitHub-first v3 file contract. The remaining failure is now in H!veAI itself: the native runtime still mixes the new remote truth with the old local-control-plane architecture, and there is no real continuous GitHub polling loop.

---

# Confirmed good work

The M16O run did complete substantial migration work:

- all eight target repositories have a v3 GitHub tracking contract according to the builder log and sampled independent target-branch verification;
- `.hiveai/TASKS.md` machine headers are now explicit rather than inferred from arbitrary Markdown ordering;
- ScrubBots current tracker is now on the active M19 audit cycle, not the old stale SB-M02-017 fallback;
- ScrubBots-Level-Factory current tracker is now on PAG-M06-C002, not the old PAG-M02 state;
- H!veAI contains a dedicated `github_tracking` remote reader and remote snapshot cache;
- Command Center can overlay remote current-task fields from GitHub.

These are real improvements. They are not enough for product acceptance because the runtime does not yet obey the new authority boundary end-to-end.

---

# M16O-R43 — BLOCKER
## There is no actual continuous GitHub polling scheduler

The v3 contract and M16O prompt require:

- active/selected project refresh about every 10 seconds;
- remaining portfolio refresh about every 30 seconds;
- GitHub remote state to update automatically while H!veAI is open.

Production source does not implement that behavior.

### Source evidence

`github_tracking.rs` defines `refresh_all(...)` and `refresh_project(...)`, but no production scheduler calls `refresh_all(...)` periodically.

`CommandCenterLive`:

- calls `refresh()` on mount;
- refreshes on a Tauri event;
- refreshes when the user presses the Refresh button;
- has no `setInterval`, timer, or 10/30-second GitHub polling loop.

The existing `WatcherManager` worker runs a 60-second safety loop, but that loop is the old local filesystem / local Git reconciliation path. It does not call the GitHub v3 tracking refresh service.

`github_tracking::refresh_project(...)` also treats a cached snapshot younger than 30 seconds as fresh for every project. Therefore even a future 10-second active-project caller would not obtain a new remote HEAD during the first 30 seconds.

### Impact

A provider can push a new `.hiveai/TASKS.md` to GitHub while H!veAI sits open and idle, and H!veAI has no guaranteed mechanism to notice it automatically.

That fails the owner's primary product requirement: H!veAI must continuously track GitHub projects.

### Required remediation

Implement one real GitHub refresh scheduler owned by the GitHub tracking subsystem, not by the local filesystem watcher.

Required behavior:

- startup: refresh all registered GitHub projects;
- selected project: refresh remote HEAD every 10 seconds;
- non-selected projects: refresh every 30 seconds;
- manual refresh: immediate, bypass normal freshness delay;
- successful changed snapshot emits one native `github-tracking-updated` event;
- unchanged remote HEAD performs no unnecessary blob parsing/UI churn;
- network failure retains last successful remote snapshot and marks it stale;
- backoff is bounded and visible;
- local filesystem activity is irrelevant to the remote poll decision.

The active-project 10-second requirement must not be defeated by a universal 30-second cache TTL.

---

# M16O-R44 — BLOCKER
## Project Cockpit still treats the old local control plane as first-class project truth

`project_cockpit::snapshot(...)` currently resolves all of the following from the local machine before returning the snapshot:

- `project_dashboard`
- `ProjectTruthResolver`
- `control_plane::snapshot`
- local task intelligence
- local workflow
- local Git snapshot/diff
- local task sources
- local warnings

It separately fetches `github_tracking`, but the old structures remain present in the same primary `ProjectCockpitSnapshot`.

The cockpit warning list is explicitly built from local dashboard/control-plane warnings before the remote snapshot is considered.

### Impact

This is precisely the architecture that produced the owner's earlier native screenshots:

- remote v3 project is valid;
- local legacy PROJECT/STATE/HANDOFF is stale;
- cockpit can still display local schema/reconciliation/progress warnings or local project metadata beside remote task truth.

The new authority contract says local state may exist only as secondary technical telemetry. Current production still exposes it as first-class cockpit state.

### Required remediation

Create a remote-primary cockpit model.

Primary Project Cockpit fields must come exclusively from one `RemoteTrackingSnapshot` resolved at one GitHub commit:

- repository / branch / remote HEAD
- current milestone
- current sprint
- current task
- workflow state
- required actor
- next action
- blockers
- exact progress
- last completed task
- tracker updatedAt/updatedBy
- remote health / refresh timestamp

Move all local-only material into a distinct optional structure such as `localWorkspaceTelemetry` and a collapsed Technical section.

Local dashboard/control-plane/task-intelligence/workflow warnings must never appear as project-truth warnings when the remote v3 contract is valid.

If the remote v3 snapshot is CURRENT, old local schema warnings are irrelevant to the primary cockpit.

---

# M16O-R45 — BLOCKER
## Command Center portfolio metrics, attention, queue and activity are still computed from local legacy state before remote overlay

`command_center::snapshot(...)` currently:

1. builds each project through the old `summarize_project(...)` local pipeline;
2. builds attention/work queue/activity from local workflow/dashboard/task evidence;
3. only afterward calls `github_tracking::refresh_project(...)` and overlays selected project-summary fields.

`apply_remote_tracking(...)` replaces current task/progress/status fields but does not rebuild the already-created portfolio attention/work queue/activity from remote tracker state.

It also leaves old fields such as local `materialized` state and pre-existing local warnings in the summary.

### Impact

The owner can still see a Command Center whose project cards use remote current tasks while:

- Active Tasks counts come from legacy/local data;
- Completed counts or work queues come from local workflow state;
- Needs Attention is based on local M10 evidence;
- recent activity can be unrelated to the canonical GitHub tracker;
- old local warnings remain attached.

That is not a GitHub project tracker. It is a hybrid dashboard with two competing truth systems.

### Required remediation

For v3 GitHub-tracked projects, build Command Center primary portfolio data directly from remote snapshots.

At minimum:

- project current state = remote tracker only;
- active/complete/progress metrics = remote tracker only, or unavailable when the tracker does not provide exact counts;
- attention = remote workflow/blockers/remote health only;
- work queue = remote current task/next action only;
- recent H!veAI tracking activity = canonical remote EVENTS tail only;
- warnings = remote schema/refresh/tracker validation only;
- no local M08/M09/M10/project-dashboard fallback for migrated v3 projects.

Legacy local pipelines may remain only for explicitly non-v3/unmigrated projects, and those must be visually isolated as legacy mode.

---

# M16O-R46 — MAJOR
## Remote health can report HEALTHY while the canonical workflow itself is BLOCKED

`apply_remote_tracking(...)` currently maps:

- remote CURRENT + blockers empty -> `HEALTHY`
- remote CURRENT + blockers non-empty -> `ATTENTION`

It does not incorporate `workflowState`.

The current Bulk-Edit remote tracker independently shows:

- `workflowState = BLOCKED`
- `requiredActor = OWNER`
- `progress = null`
- `blockers = []`

Under current H!veAI mapping that remote snapshot can still become `HEALTHY` simply because the blockers array is empty.

### Required remediation

Define GitHub v3 health from remote truth only and include workflow semantics.

Examples:

- remote unavailable -> UNKNOWN/OFFLINE
- stale cache -> STALE
- malformed contract -> ERROR
- workflow BLOCKED -> ATTENTION/BLOCKED
- blockers non-empty -> ATTENTION/BLOCKED
- awaiting owner/audit -> WAITING or ATTENTION according to accepted UX
- runnable/current with no blockers -> HEALTHY

Add a direct fixture using the actual Bulk-Edit shape.

---

# Release-gate test requirements

The next remediation must include direct tests for:

1. remote commit pushed after H!veAI startup is detected automatically without pressing Refresh;
2. selected project change detected within the selected-project polling window;
3. non-selected project change detected within the portfolio polling window;
4. remote PAG-M06 while local PAG-M02 -> primary cockpit shows only PAG-M06 truth;
5. remote v3 valid while local PROJECT malformed -> no primary cockpit local-schema warning;
6. Command Center KPI/attention/queue use remote v3 state only;
7. Bulk-Edit `workflowState=BLOCKED` cannot be HEALTHY;
8. cached stale remote is shown when network fails, with timestamp;
9. local watcher/local dirty changes do not change GitHub current task/milestone/progress;
10. all eight current remote v3 trackers parse and produce expected primary summaries.

---

# Final acceptance state

After this remediation, H!veAI must have one unmistakable rule:

> For migrated v3 repositories, GitHub remote snapshot is the only primary project truth. Local project systems are optional telemetry only.

M16 remains OPEN pending remediation, independent strict re-audit, and owner native acceptance.
