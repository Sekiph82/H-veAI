# M16N — Eight-Repository H!veAI Tracking Hard Reset and Live Resume Contract

## Purpose

Stop treating heterogeneous historical trackers as live state.

The owner's actual product requirement is simple:

> Open H!veAI, see the true current state of all eight projects, click any project, and continue from the exact current task/next action.

The current native screenshots prove this is not true. This run is therefore a PRODUCT TRACKING RESET, not an audit-engine enhancement.

Read and obey `H!veAI/GPT.md`, especially the 2026-09-09 owner-locked unified-live-tracking priority.

Also read:

`H!veAI/docs/H!veAI/UNIFIED_PROJECT_TRACKING_CONTRACT_V2.md`

Do all work in one continuous run. Do not stop after fixing one repository.

## Registered portfolio

Migrate these eight actual local projects and their GitHub repositories:

1. AI-Commerce-HQ
   - local: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`
   - repo: `Sekiph82/AI-Commerce-HQ`
   - branch: `H!veAI`

2. Bulk-Edit
   - local: `C:\Users\sekip\Desktop\Bulk-Edit`
   - repo: `Sekiph82/Bulk-Edit`
   - branch: `main`

3. fmcg-erp-system
   - local: `C:\Users\sekip\Desktop\fmcg-erp-system`
   - repo: `Sekiph82/fmcg-erp-system`
   - branch: `main`

4. FormuLab
   - local: `C:\Users\sekip\Desktop\FormuLab`
   - repo: `Sekiph82/FormuLab`
   - branch: `feature/laboratory-stability`

5. PackLab
   - local: `C:\Users\sekip\Desktop\PackLab`
   - repo: `Sekiph82/PackLab`
   - branch: `main`

6. PackLab 3D
   - local: `C:\Users\sekip\Desktop\PackLab 3D`
   - repo: `Sekiph82/PackLab-3D`
   - branch: `main`

7. ScrubBots
   - local: `C:\Users\sekip\Desktop\ScrubBots`
   - repo: `Sekiph82/Scrubbots`
   - branch: `main`

8. Scrubbots - Pixel Art Generator
   - local: `C:\Users\sekip\Desktop\Scrubbots - Pixel Art Generator`
   - repo: `Sekiph82/ScrubBots-Level-Factory`
   - branch: `main`

The owner explicitly states that the Pixel Art Generator is currently in PAG-M05 / WFC work. Do not regress it to PAG-M02. Inspect the actual current local/GitHub task/handoff evidence and migrate that truth.

## Hard reset of H!veAI tracking authority

For EACH repository, H!veAI tracking authority after this run is EXACTLY:

- `.hiveai/PROJECT.json`
- `.hiveai/RULES.md`
- `.hiveai/TASKS.md`
- `.hiveai/STATE.json`
- `.hiveai/HANDOFF.md`
- `.hiveai/EVENTS.jsonl`

No other file is allowed to be a live H!veAI state authority.

Legacy files such as:

- root `TASKS.md` / `tasks.md`
- `PROJECT_STATUS.md`
- root `HANDOFF.md` / `handoff.md`
- `.hiveai/PROJECT_DASHBOARD.md`
- `.hiveai/ACTIVE_CYCLES.md`
- `.hiveai/ARTIFACT_MAP.md`
- `.hiveai/PROGRESS_SNAPSHOT.md`
- coordination session indexes
- provider-specific status files

may remain only as historical/project-specific reference where still needed by the project, but H!veAI MUST NOT parse them for current task, next action, milestone, progress, health, or workflow after migration.

Delete obsolete H!veAI live-tracker files that would compete with the six canonical files. Git history is the backup.

## Exact PROJECT.json contract

All eight use the SAME schema and field shape:

`schema = "hiveai-project-control-plane/v1"`

Required fields:

- projectKey
- displayName
- repository.owner
- repository.name
- repository.branch
- trackedBranch
- canonicalTaskSource = ".hiveai/TASKS.md"
- rules = ".hiveai/RULES.md"
- state = ".hiveai/STATE.json"
- handoff = ".hiveai/HANDOFF.md"
- events = ".hiveai/EVENTS.jsonl"
- refresh.mode = "watcher-first"
- refresh.debounceMs = 250
- refresh.safetyReconcileSeconds = 30
- git.remoteExpected = true
- git.localGitPreferred = true
- git.nonGitLocalFolderMustBeReported = true

No local Registry UUID.
No machine-specific path inside portable PROJECT.json.
No project-specific alternate schema.

## Exact .hiveai/TASKS.md format

Every repository must have the same parser-safe format.

The first bytes of the file must contain:

`<!-- HIVEAI_TRACKER_START`

then one strict JSON object, then:

`HIVEAI_TRACKER_END -->`

JSON fields:

- schema: "hiveai-task-tracker/v2"
- projectKey
- currentMilestone
- currentSprint
- currentTaskId
- currentTaskTitle
- workflowState
- requiredActor
- nextAction
- blockers: []
- progress:
  - scopeType: "PROJECT"
  - scopeId: projectKey
  - completed
  - total
  - percent
- updatedAt
- updatedBy

After the machine block, keep human-readable Markdown sections:

1. Current
2. Milestones
3. Active / Waiting
4. Planned
5. Completed summary
6. History / migration notes

Use the same task markers everywhere:

- `[x]` completed/accepted
- `[~]` active/in progress
- `[ ]` planned
- `[!]` blocked

Migrate actual project task truth from the existing tracker into this file.

Do not blindly copy stale historical prose as current truth.
Do not select the first unchecked historical item.
Do not invent progress.

Project progress = count of all normalized tracked task rows in this canonical tracker.
If task inventory cannot be normalized exactly during migration, fail the project migration visibly rather than display a false percent.

## STATE.json

STATE is the exact current operational projection.

Required:

- schema: "hiveai-state/v2"
- projectKey
- projectStatus
- currentMilestone
- currentSprint
- currentTaskId
- currentTaskTitle
- workflowState
- requiredActor
- nextAction
- blockers
- progress with exact scopeId/completed/total/percent
- git.branch
- git.head
- git.dirty
- remote.status
- lastAudit
- lastAgentSession
- updatedAt
- updatedBy

STATE and TASKS machine block must agree.

If they disagree, H!veAI displays `TRACKER_CONFLICT`, never an old random task.

## HANDOFF.md

Same format in all eight:

- Project
- Current milestone
- Current sprint
- Current task ID/title
- Workflow state
- Required actor
- Exact next action
- Blockers
- Git branch/HEAD
- Updated at/by

This is resume UX, not a competing ledger.

## EVENTS.jsonl

Append-only canonical lifecycle.

Every meaningful task/workflow/audit/session transition adds exactly one `hiveai-event/v1` row.

## RULES.md

Make the core H!veAI rules text IDENTICAL in all eight repositories.

Project-specific rules must live outside the shared core section and may not override state-sync semantics.

The rule must require EVERY provider, before responding after a state-changing action, to update:

1. .hiveai/TASKS.md
2. .hiveai/STATE.json
3. .hiveai/HANDOFF.md
4. append .hiveai/EVENTS.jsonl

No exceptions for Codex, Claude, ChatGPT, audits, remediation, owner acceptance, blocker changes, or task completion.

## Provider instruction files

Fix the case/duplication problem.

For each repo:

1. Detect actual case-sensitive Claude instruction files.
2. Never leave both `Claude.md` and `CLAUDE.md`.
3. Preserve existing project-specific instructions.
4. Add ONE short H!veAI adapter at the top of the surviving file.
5. The adapter says `.hiveai/RULES.md` is the provider-neutral tracking authority.
6. Do the same for `AGENTS.md` / Codex.
7. Provider-specific files must not maintain separate current task/handoff/progress state.

Do not discard useful project rules. Remove only duplicated/conflicting H!veAI tracker authority.

## One-time migration must use REAL current state

For each of the eight projects, inspect:

- current local branch/HEAD/status
- existing canonical tracker
- latest project-specific handoff/current pointer
- latest relevant audit/log
- current Git state

Then populate the new six-file contract with the actual current project state.

Known native regression examples that MUST be corrected:

- Pixel Art Generator must not display PAG-M02 when actual current milestone is PAG-M05.
- Its old next action `Execute PAG-M02-C001...` must disappear.
- AI-Commerce-HQ must not sit in generic NEEDS_RECONCILIATION when its own current M16 state is known.
- Bulk-Edit must not show unsupported PROJECT schema when its migrated contract is valid.
- fmcg must not synthesize prose such as completed/next sentences into a fake task.
- PackLab must resume from its actual PL task pointer.
- FormuLab must use its actual current FVL pointer.
- no project may show a percent without exact task scope.

## H!veAI application changes

Change H!veAI itself so it no longer tries to understand arbitrary project prose.

### Read model

Project Cockpit / Command Center current state comes from:

1. PROJECT.json
2. STATE.json
3. TASKS.md machine block
4. HANDOFF.md consistency check
5. Git state

Legacy trackers are not parsed for current truth.

### Watcher

Watch exact six canonical files plus:

- .git/HEAD
- .git/index
- .git/refs
- packed-refs where relevant

Local canonical-file changes trigger UI refresh within debounce window.

No manual Refresh button should be required for normal local agent work.

Safety reconcile every 30 seconds.

### Continuous update semantics

Projects do NOT need a network push into H!veAI.

The provider writes the local canonical files after every meaningful action.
The local watcher sees the filesystem event and H!veAI updates immediately.

GitHub push is durable collaboration/history, not the mechanism for updating the local UI.

### Remote sync

Background fetch can observe remote changes.

If worktree is clean and configured for safe FF-only sync, H!veAI may fast-forward according to existing permission policy.

If worktree is dirty:
- do not overwrite local work;
- show REMOTE_AHEAD / LOCAL_DIRTY truthfully;
- keep displaying local canonical state.

Never reset/clean/force checkout.

## Migration tool

Add a reusable command/tool in H!veAI that can migrate/repair one registered project or all registered projects.

Suggested native command:

- Repair tracking contract
- Repair all project tracking contracts

It must:
- validate the six files
- report legacy conflicts
- preserve project-specific source files
- rebuild STATE/HANDOFF from canonical tracker
- never overwrite unrelated product code

## Tests

Use the actual eight contract shapes and direct parser paths.

Required:
- all eight PROJECT files accepted
- all eight TASKS machine blocks parsed
- current state matches STATE
- tracker conflict detection
- exact progress scope
- provider filename duplicate detection
- local file event updates UI state
- 100 repeated reads do not mutate files
- Pixel Art Generator PAG-M05 regression
- no PAG-M02 stale next action
- no first-unchecked-history fallback
- fmcg prose regression
- FormuLab pointer regression
- PackLab pointer regression
- unsupported-schema warning absent after migration
- dirty worktree remote-safe behavior

## Local repository execution

This run must operate on the user's actual local repositories listed above.

Before each repo:
- inspect status
- preserve owner changes
- safely fetch
- FF-only pull only if safe
- never reset/clean/force
- if dirty, migrate tracking files without overwriting unrelated work

After each repository:
- validate files
- commit only the intended tracking migration and provider adapter edits
- push safely to its configured branch if possible
- record commit SHA

Do all eight, not only AI-Commerce-HQ.

## Final native acceptance

After rebuilding/publishing H!veAI:

1. launch native app;
2. Command Center must show eight parseable projects;
3. open all eight project cockpits;
4. compare displayed current milestone/task/next action/progress to canonical local .hiveai files;
5. edit one test project's STATE/HANDOFF/TASKS locally through a controlled test;
6. verify H!veAI changes automatically without pressing Refresh;
7. revert the controlled test cleanly;
8. verify no legacy schema or progress-scope warnings remain.

## Required immutable log

Create:

`H!veAI/docs/H!veAI/codex-logs/M16N_EIGHT_REPOSITORY_TRACKING_HARD_RESET_AND_LIVE_RESUME_LOG.md`

Include a table for all eight:

- local path
- remote
- branch
- starting HEAD
- migrated current milestone
- migrated current task
- next action
- completed/total/percent
- provider instruction filename kept
- deleted legacy H!veAI trackers
- migration commit
- pushed HEAD
- native displayed values

Do not declare M16 closed.
Do not activate M17.
Do not start M21.

End with:

`M16N EIGHT-REPOSITORY TRACKING RESET COMPLETE / PENDING OWNER NATIVE ACCEPTANCE.`
