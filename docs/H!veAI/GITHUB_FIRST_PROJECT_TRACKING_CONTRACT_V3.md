# H!veAI GitHub-First Project Tracking Contract v3

> Historical contract retained for audit provenance only. It is superseded by
> the standalone root `TASKS.md` contract and is not read by production
> project tracking.

This contract supersedes the local-first v2 tracking design.

## Product goal

H!veAI is a GitHub project command center.

The owner should be able to open H!veAI, see the true current state of all tracked GitHub repositories, click any project, and resume from the exact current task and next action.

The tracked GitHub branch is authoritative.

Local folders are not project-truth sources.

## Canonical repository files

Every tracked GitHub repository uses the same four files:

- `.hiveai/PROJECT.json`
- `.hiveai/TASKS.md`
- `.hiveai/RULES.md`
- `.hiveai/EVENTS.jsonl`

There is intentionally no separate canonical STATE.json or HANDOFF.md in v3.

Current milestone, current task, next action, required actor, workflow state, blockers, and exact progress all live in the machine header of `.hiveai/TASKS.md`.

This removes duplicate state projections that can drift.

## GitHub authority

H!veAI reads these files from:

`https://github.com/<owner>/<repo>@<trackedBranch>`

using GitHub remote/API data.

It does not read local working-tree copies for portfolio truth.

If the remote cannot be reached:
- show the last successful remote snapshot;
- show `STALE_REMOTE_SNAPSHOT` with timestamp;
- never substitute local files silently.

## PROJECT.json

Exact schema:

`hiveai-project/v3`

Required fields:

- schema
- projectKey
- displayName
- repository.owner
- repository.name
- trackedBranch
- tasksPath = ".hiveai/TASKS.md"
- rulesPath = ".hiveai/RULES.md"
- eventsPath = ".hiveai/EVENTS.jsonl"
- refresh.mode = "github-poll"
- refresh.intervalSeconds
- refresh.manualRefresh = true

No local path.
No local registry UUID.
No machine-specific value.
No local dirty state.

## TASKS.md

This is the single operational project truth.

The file begins with:

`<!-- HIVEAI_TRACKER_V3_START`

followed by one strict JSON object and:

`HIVEAI_TRACKER_V3_END -->`

Required JSON fields:

- schema = "hiveai-task-tracker/v3"
- projectKey
- currentMilestone
- currentSprint
- currentTaskId
- currentTaskTitle
- workflowState
- requiredActor
- nextAction
- blockers
- progress.scopeType
- progress.scopeId
- progress.completed
- progress.total
- progress.percent
- lastCompletedTaskId
- lastCompletedTaskTitle
- updatedAt
- updatedBy

Progress rules:
- percent must equal completed / total for the exact scope;
- if exact completed/total cannot be proven, all progress fields are null;
- H!veAI must never invent a percentage.

Human Markdown follows the machine header:

1. Current
2. Milestones
3. Active / Waiting
4. Planned
5. Completed
6. History / migration notes

Task markers:

- `[x]` completed
- `[~]` active
- `[ ]` planned
- `[!]` blocked

H!veAI does not discover current task from Markdown ordering. It uses the machine header.

## RULES.md

The H!veAI core rules section is identical in all repositories.

Every provider must:

1. read `.hiveai/RULES.md`;
2. read `.hiveai/TASKS.md`;
3. perform the requested project work;
4. update `.hiveai/TASKS.md` if project state changed;
5. append one `hiveai-event/v1` row when lifecycle state changed;
6. commit;
7. push to the configured tracked GitHub branch;
8. only then report final completion.

If push is blocked, the provider must report that the GitHub project state is not yet synchronized. It must not claim H!veAI has been updated.

## EVENTS.jsonl

Append-only lifecycle history.

Events do not override TASKS current-state truth.

Each event includes:
- schema = hiveai-event/v1
- id
- projectKey
- type
- taskId when applicable
- from
- to
- actor
- timestamp
- commitSha when available

## Provider adapters

Provider-specific instruction files are adapters, not state stores.

Each repository should have:
- one Codex adapter, usually `AGENTS.md`;
- one Claude instruction file, preserving the repository's actual chosen filename/casing.

Do not keep both `Claude.md` and `CLAUDE.md`.

The adapter must say:
- read `.hiveai/RULES.md`;
- H!veAI project truth is GitHub `.hiveai/TASKS.md`;
- do not maintain a competing current-task/progress/handoff section elsewhere.

## H!veAI refresh model

Primary refresh:
- GitHub API / remote branch polling.

Recommended:
- poll active project every 10 seconds;
- poll remaining portfolio every 30 seconds;
- manual Refresh triggers immediate remote fetch;
- use ETag/commit-SHA conditional requests where possible.

On new tracked-branch HEAD:
1. fetch PROJECT.json;
2. fetch TASKS.md;
3. validate schema;
4. parse tracker header;
5. fetch/validate optional events delta;
6. update Command Center/Cockpit atomically.

No local filesystem watcher is required for project truth.

## Native display

Command Center and Project Cockpit show:

- GitHub repository
- tracked branch
- remote HEAD
- current milestone
- current sprint
- current task
- workflow state
- required actor
- next action
- blockers
- exact progress
- last completed task
- last GitHub tracker update time
- remote health

Local workspace status may exist only in a collapsed Technical/Local Workspace area and cannot change project truth.

The owner should not need to understand Git dirty/clean terminology to use the main product.

## Migration

For each tracked repository:

1. inspect actual current project truth from its latest authoritative project-specific evidence;
2. create the four v3 files;
3. normalize current and future task state into `.hiveai/TASKS.md`;
4. keep useful historical project documents, but mark them non-authoritative for H!veAI;
5. remove obsolete H!veAI STATE/HANDOFF/dashboard control files that could be mistaken as live authority;
6. ensure the provider adapters point to the v3 rules;
7. commit and push to the tracked GitHub branch.

After migration, H!veAI must not parse legacy task files for live current state.

## Acceptance

The contract is accepted only when all eight actual target branches:

- contain valid v3 PROJECT.json;
- contain valid v3 TASKS machine headers;
- show the real current task/milestone/next action;
- have exact or null progress;
- have no stale schema/reconciliation warning caused by legacy H!veAI files;
- appear identically in native H!veAI after GitHub refresh.

Pixel Art Generator must specifically reflect its real PAG-M05/WFC state, not stale PAG-M02 state.
