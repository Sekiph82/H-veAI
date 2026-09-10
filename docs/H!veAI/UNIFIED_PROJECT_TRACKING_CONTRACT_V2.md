# H!veAI Unified Project Tracking Contract v2

This is the single tracking contract for every H!veAI-managed repository.

## Goal

H!veAI must show the actual current project state, not infer it from old prose, stale dashboards, provider-specific handoff files, or historical task ledgers.

Every tracked repository uses the same six files:

- .hiveai/PROJECT.json
- .hiveai/RULES.md
- .hiveai/TASKS.md
- .hiveai/STATE.json
- .hiveai/HANDOFF.md
- .hiveai/EVENTS.jsonl

H!veAI reads these files only for project tracking.

Legacy files such as root TASKS.md/tasks.md, PROJECT_STATUS.md, HANDOFF.md, PROJECT_DASHBOARD.md, coordination trackers, Claude-specific status docs, etc. may remain as project history/reference, but they are not H!veAI tracking authority after migration.

## Authority

1. .hiveai/TASKS.md = canonical task ledger used by H!veAI
2. .hiveai/STATE.json = canonical current operational state
3. .hiveai/HANDOFF.md = exact resume pointer
4. .hiveai/EVENTS.jsonl = append-only history
5. .hiveai/PROJECT.json = identity/config
6. .hiveai/RULES.md = provider-neutral operating rules

## Mandatory update rule

After every meaningful project action, before the AI responds, it MUST atomically synchronize:
- TASKS.md when task truth changed
- STATE.json
- HANDOFF.md
- append one event to EVENTS.jsonl

This applies to Codex, Claude, ChatGPT, and future agents.

No provider may maintain a competing current-state tracker.

## STATE.json invariant

STATE.json must always contain explicit values for:
- projectKey
- currentMilestone
- currentSprint
- currentTaskId
- currentTaskTitle
- workflowState
- requiredActor
- nextAction
- blockers
- progress.scopeId
- progress.completed
- progress.total
- progress.percent
- git.branch
- git.head
- git.dirty
- updatedAt
- updatedBy

If exact progress scope is unknown, percent MUST be null. Never emit a percent without scopeId/completed/total.

## Live refresh

H!veAI watches all six files plus Git HEAD/index/refs.
A local file write triggers immediate debounced refresh.
The 60 second scan is a safety net only.

## Provider adapters

Provider files do not own project state.

AGENTS.md and the one canonical Claude instruction file contain only a short adapter telling the provider to read .hiveai/RULES.md first and to synchronize the six files after work.

No duplicate Claude.md / CLAUDE.md case variants may coexist in one repository.

## Resume behavior

When the computer starts:
1. H!veAI reads STATE.json immediately.
2. H!veAI verifies TASKS.md and Git.
3. If STATE is internally consistent, display it immediately.
4. If files changed since STATE.updatedAt, reconcile from TASKS + Git and rewrite STATE/HANDOFF.
5. Never fall back to random historical OPEN task prose.
6. Never derive current task from the first unchecked historical checkbox.

## Migration rule

For each repository:
- preserve legacy planning files as history unless deletion is explicitly safe;
- create the six canonical files above;
- set PROJECT.canonicalTaskSource to .hiveai/TASKS.md;
- migrate the actual current task/milestone/next action into STATE/HANDOFF;
- move only active/future task truth into .hiveai/TASKS.md;
- archive historical completed detail under a clearly marked Legacy/History section or leave it in old files as non-authoritative reference;
- remove duplicate H!veAI tracker/dashboard files that could be mistaken for authority.

This contract is intentionally simple. H!veAI is a project command center, not a document archaeology engine.
