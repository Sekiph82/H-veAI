# M16O — GitHub-First Eight-Repository Tracking Reset

## Status

This prompt explicitly supersedes the unexecuted local-first M16N prompt:

`H!veAI/docs/H!veAI/prompts/M16N_EIGHT_REPOSITORY_TRACKING_HARD_RESET_AND_LIVE_RESUME_PROMPT.md`

Do NOT execute M16N.

## Purpose

Return H!veAI to the owner's original product requirement:

> H!veAI tracks the owner's GitHub projects. The GitHub tracked branch is authoritative. The owner opens one app, sees the exact current state of each project, clicks one, and continues from the correct current task/next action.

Local folders are implementation workspaces only. They are not H!veAI project-truth authorities.

Read and obey:

1. `H!veAI/GPT.md`
2. `H!veAI/docs/H!veAI/GITHUB_FIRST_PROJECT_TRACKING_CONTRACT_V3.md`

Execute this entire remediation in one continuous run across all eight repositories and H!veAI itself.

Do not stop after one repository.
Do not activate M17.
Do not start M21.
Do not close M16 until owner native acceptance.

---

# 1. Portfolio

Canonical GitHub targets:

1. `Sekiph82/AI-Commerce-HQ` branch `H!veAI`
2. `Sekiph82/Bulk-Edit` branch `main`
3. `Sekiph82/fmcg-erp-system` branch `main`
4. `Sekiph82/FormuLab` branch `feature/laboratory-stability`
5. `Sekiph82/PackLab` branch `main`
6. `Sekiph82/PackLab-3D` branch `main`
7. `Sekiph82/Scrubbots` branch `main`
8. `Sekiph82/ScrubBots-Level-Factory` branch `main`

The local clones may be used to edit/test/commit/push, but their working-tree copies MUST NOT become the state read by H!veAI.

---

# 2. Hard architectural rule

H!veAI current project truth MUST come from the configured GitHub target branch.

For current milestone/task/next action/progress/workflow/required actor/blockers:

Allowed:
- GitHub remote/API contents of `.hiveai/PROJECT.json`
- GitHub remote/API contents of `.hiveai/TASKS.md`
- GitHub remote/API contents of `.hiveai/RULES.md`
- GitHub remote/API contents of `.hiveai/EVENTS.jsonl`

Forbidden as truth:
- local working-tree `.hiveai` files
- local root TASKS/tasks files
- local STATE/HANDOFF
- local dirty/clean state
- old PROJECT_DASHBOARD
- old ACTIVE_CYCLES
- old PROGRESS_SNAPSHOT
- old coordination/handoff prose
- first unchecked historical checkbox
- inferred current task from arbitrary Markdown prose
- provider self-assessment

If GitHub is unreachable, show the last successful remote snapshot with an explicit stale timestamp. Never silently substitute local project state.

---

# 3. Simplify the repository control plane

For each of the eight GitHub repositories, canonical H!veAI tracking becomes exactly four files:

- `.hiveai/PROJECT.json`
- `.hiveai/TASKS.md`
- `.hiveai/RULES.md`
- `.hiveai/EVENTS.jsonl`

Remove/deprecate H!veAI's separate canonical STATE.json and HANDOFF.md concept.

Why:
- state duplication caused drift;
- current task, next action and progress must exist once;
- H!veAI should consume one authoritative operational tracker.

Legacy STATE/HANDOFF files may be deleted from the H!veAI control plane after migration if they are not required by the project itself. If retained for project history, H!veAI must ignore them.

---

# 4. Rewrite all eight PROJECT.json files to one exact schema

Exact schema:

`hiveai-project/v3`

Required shape:

```json
{
  "schema": "hiveai-project/v3",
  "projectKey": "...",
  "displayName": "...",
  "repository": {
    "owner": "Sekiph82",
    "name": "..."
  },
  "trackedBranch": "...",
  "tasksPath": ".hiveai/TASKS.md",
  "rulesPath": ".hiveai/RULES.md",
  "eventsPath": ".hiveai/EVENTS.jsonl",
  "refresh": {
    "mode": "github-poll",
    "intervalSeconds": 30,
    "manualRefresh": true
  }
}
```

No local path.
No local registry ID.
No local Git dirty field.
No old v1 schema.
No alternate project-specific schema.

---

# 5. Rewrite all eight TASKS trackers to one exact format

Each repository gets:

`.hiveai/TASKS.md`

The first bytes must contain:

`<!-- HIVEAI_TRACKER_V3_START`

then one strict JSON object, then:

`HIVEAI_TRACKER_V3_END -->`

Required machine fields:

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

Then human-readable Markdown sections:

1. Current
2. Milestones
3. Active / Waiting
4. Planned
5. Completed
6. History / migration notes

Task markers:
- [x] completed
- [~] active
- [ ] planned
- [!] blocked

H!veAI reads the machine header for current-state truth.
It MUST NOT infer current state from Markdown ordering.

---

# 6. Migrate REAL current state, not stale H!veAI state

For each repo, determine actual current truth from the latest authoritative project-specific GitHub evidence BEFORE rewriting the v3 tracker.

Use:
- latest task tracker
- latest accepted audit/owner acceptance
- latest current handoff if project-specific
- latest relevant builder/audit chain
- target branch HEAD

Do not use stale H!veAI STATE.json as authority.

Known required correction:

## Scrubbots - Pixel Art Generator

The owner states current work is PAG-M05 / WFC.

Therefore:
- do not show PAG-M02;
- do not show stale `Execute PAG-M02-C001...`;
- inspect actual current PAG-M05 task(s) and write the real current milestone/task/next action.

If several PAG-M05 items are simultaneously active, encode the exact currently resumable task in the header and list the others under Active / Waiting.

## AI-Commerce-HQ

Do not leave generic NEEDS_RECONCILIATION merely because old H!veAI state said so.
Resolve actual current M16 state from the live branch/document chain.

## Bulk-Edit

Remove the unsupported-schema warning by rewriting the remote PROJECT contract to v3 and making H!veAI read the remote v3 file.

## fmcg-erp-system

Do not convert prose like “X closed, Y next” into a synthetic task.
Use actual task IDs and statuses.

## FormuLab / PackLab / PackLab-3D / ScrubBots

Resolve their actual current task pointers from their current GitHub branch truth and migrate them into v3.

---

# 7. Exact progress semantics

Progress shown by H!veAI must be exact.

For each project:
- choose an explicit project or milestone scope;
- set scopeId;
- completed;
- total;
- percent = completed / total.

If exact count cannot be established without ambiguity:
- set scopeType/scopeId/completed/total/percent to null;
- H!veAI displays "Progress unavailable";
- never fabricate a percentage.

No more "progress percent is present without an exact progress scope".

---

# 8. RULES.md identical core across all eight

Write one identical H!veAI core rules block into all eight repos.

Core rules:

1. GitHub tracked branch is H!veAI truth.
2. Before starting, read `.hiveai/TASKS.md`.
3. After any state-changing action, update `.hiveai/TASKS.md`.
4. Append one canonical `hiveai-event/v1` lifecycle event if state changed.
5. Commit all intended project + tracker changes together when appropriate.
6. Push to the configured tracked branch before reporting final completion.
7. If push fails or is not permitted, explicitly report `GITHUB_TRACKING_NOT_SYNCED`.
8. Do not claim H!veAI state is updated until the remote push succeeded.
9. Do not maintain competing current-task/progress/handoff state in provider-specific files.
10. Never rewrite task state based only on builder self-assessment when independent acceptance is required.

Project-specific rules may follow the shared core, but cannot override these rules.

---

# 9. Provider instruction cleanup

For every repo inspect actual existing provider files.

Claude:
- keep ONE real canonical case variant;
- do not leave both Claude.md and CLAUDE.md;
- preserve useful project-specific Claude instructions;
- add a short adapter pointing to `.hiveai/RULES.md`.

Codex:
- use `AGENTS.md` when present/appropriate;
- preserve project-specific implementation rules;
- add the same adapter.

Neither provider file may contain a separate live current task/progress/handoff tracker.

---

# 10. H!veAI application rewrite: GitHub remote read model

Replace local-control-plane truth resolution.

## 10.1 Project registration

Registry stores:
- owner
- repo
- tracked branch
- display metadata
- optional local workspace path only as secondary convenience

## 10.2 Remote fetch service

Implement one GitHub tracking client/service.

Preferred primary mechanism:
- GitHub REST API contents/git data endpoints with authenticated GitHub access.

Acceptable implementation must support:
- fetch current branch HEAD SHA;
- fetch `.hiveai/PROJECT.json`;
- fetch `.hiveai/TASKS.md`;
- fetch `.hiveai/RULES.md`;
- fetch `.hiveai/EVENTS.jsonl` or bounded event tail;
- conditional requests / ETag or SHA cache;
- clear authentication/offline errors.

Do not require the local clone to obtain project truth.

## 10.3 Polling

- selected/active project: every 10 seconds;
- other registered projects: every 30 seconds;
- manual Refresh: immediate remote refresh;
- app startup: refresh all eight promptly;
- backoff on API/network failure.

A desktop app does not need incoming webhooks for this milestone.

## 10.4 Cache

Persist the last successful GitHub snapshot.

Each cached snapshot records:
- repo/branch
- remote HEAD SHA
- tracker blob SHA
- fetchedAt
- parsed current state

If offline:
- show cached state;
- clearly show last successful GitHub sync time;
- mark stale;
- do not read local files as replacement truth.

## 10.5 Atomic UI projection

Only update a project's displayed state after:
- remote HEAD determined;
- PROJECT v3 validated;
- TASKS v3 tracker header parsed.

Do not mix PROJECT from one commit with TASKS from another if avoidable.
Prefer reading blobs at one resolved commit SHA.

---

# 11. Remove local concepts from primary UX

The owner does not need to understand "dirty".

From Command Center/Cockpit primary panels remove or demote:
- Local Git DIRTY/CLEAN
- local sync as health authority
- local path as main identity
- local NEEDS_RECONCILIATION
- local watcher state

Primary panels should show:

- GitHub repo
- tracked branch
- GitHub sync state
- remote HEAD short SHA
- current milestone
- current task
- current sprint
- workflow
- required actor
- next action
- blockers
- progress
- last completed task
- last tracker update
- last GitHub refresh

Optional local workspace information may live under a collapsed Technical/Local Workspace section only.

---

# 12. No local fallback

Add direct tests proving:

- remote says PAG-M05 while local old file says PAG-M02 → UI shows PAG-M05;
- remote v3 valid while local PROJECT old/malformed → UI shows remote valid state;
- remote unavailable + local valid → UI shows cached remote stale, NOT local;
- local dirty/clean changes do not alter task/milestone/progress;
- local file watcher does not drive project truth.

This is a hard invariant.

---

# 13. Cross-repository migration execution

For each of the eight repos:

1. fetch/sync target branch safely;
2. inspect current project truth;
3. create/rewrite four canonical v3 files;
4. clean provider adapters;
5. remove obsolete H!veAI live-authority files if safe;
6. validate v3 parser locally;
7. commit;
8. push to the exact tracked branch;
9. verify via GitHub that remote branch now contains the expected v3 files;
10. record remote HEAD SHA.

The run is incomplete until all eight GitHub target branches are verified.

Do not merely edit local files.

---

# 14. Tests

Required direct tests:

1. GitHub-first remote-vs-local conflict chooses remote.
2. Remote PROJECT v3 parsing.
3. Remote TASKS v3 parsing.
4. Eight actual PROJECT contracts.
5. Eight actual tracker headers.
6. Pixel Generator PAG-M05 regression.
7. PAG-M02 stale-next-action regression.
8. Bulk-Edit schema-warning regression.
9. fmcg prose regression.
10. FormuLab current pointer regression.
11. PackLab current pointer regression.
12. PackLab-3D pointer regression.
13. ScrubBots pointer regression.
14. exact progress or null progress.
15. provider case-duplicate detection.
16. remote HEAD change refresh.
17. selected project 10-second refresh scheduling.
18. portfolio 30-second refresh scheduling.
19. offline cached-state behavior.
20. no local fallback.
21. local dirty state cannot change current project truth.
22. Command Center/Cockpit same remote snapshot.
23. full M16 audit-engine regressions remain green.
24. full frontend/Rust/publication regressions.

---

# 15. Native acceptance targets

After rebuild/publication, owner must be able to open H!veAI and see:

## Command Center
- 8 GitHub projects
- no bogus 1323/509 counts unless those counts exactly match normalized v3 trackers
- no local dirty health affecting project truth
- no stale local legacy warning

## Pixel Art Generator
- current milestone PAG-M05
- actual WFC current task
- actual next action
- no PAG-M02 stale text
- exact/null progress

## All other seven
- current task matches their remote `.hiveai/TASKS.md`
- schema valid
- no self-created NEEDS_RECONCILIATION
- exact/null progress
- GitHub last-sync timestamp visible

Change one v3 tracker on GitHub through a controlled commit/push and verify H!veAI updates automatically without local file editing.

---

# 16. Documentation cleanup

Update:
- GPT.md already records GitHub-first owner priority;
- README/docs explaining project tracking;
- remove/deprecate local-first v2 references from current architecture docs;
- keep historical prompts/logs/audits immutable.

Explicitly mark M16N superseded by M16O in prospective/current documentation.

---

# 17. Full verification/publication

Run:
- focused GitHub client tests;
- remote/local conflict tests;
- v3 parser tests;
- all eight contract tests;
- Command Center/Cockpit tests;
- audit engine regressions;
- all Rust;
- all-targets;
- pty support;
- frontend Vitest;
- typecheck;
- production build;
- npm audit high;
- cargo fmt;
- git diff --check;
- publisher rollback 9/9;
- governed publication;
- candidate/stable SHA equality;
- native smoke if feasible.

---

# 18. Required immutable log

Create:

`H!veAI/docs/H!veAI/codex-logs/M16O_GITHUB_FIRST_EIGHT_REPOSITORY_TRACKING_RESET_LOG.md`

Include:

- starting H!veAI HEAD
- all eight repositories
- old remote tracker format
- resolved real current milestone/task
- v3 migration commit
- pushed target branch HEAD
- provider adapter filename retained
- removed/deprecated H!veAI state files
- remote PROJECT blob SHA
- remote TASKS blob SHA
- progress scope/count
- GitHub refresh tests
- remote-vs-local conflict proof
- offline cache proof
- full regressions
- publication SHA
- native acceptance pending

End exactly:

`M16O GITHUB-FIRST EIGHT-REPOSITORY TRACKING RESET COMPLETE / PENDING OWNER NATIVE ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
