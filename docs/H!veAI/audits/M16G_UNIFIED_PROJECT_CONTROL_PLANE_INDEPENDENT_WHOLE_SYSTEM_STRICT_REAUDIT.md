# M16G Unified Project Control Plane — Independent Whole-System Strict Re-Audit

Date: 2026-09-09  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16G_UNIFIED_PROJECT_CONTROL_PLANE_FINAL_WHOLE_SYSTEM_CLOSURE_REMEDIATION_LOG.md`  
Implementation/log head reviewed: `02d83e20a4e29d1291335d091d18bc23e617129d`

## Verdict

**FAIL / CHANGES REQUIRED**

### Previously named M16G findings
- UCP-R13: CLOSED
- UCP-R14: CLOSED
- UCP-R15: CLOSED
- UCP-R16: NOT CLOSED
- UCP-R17: CLOSED
- UCP-R18: PARTIALLY CLOSED

### New whole-system findings
- BLOCKER: 2
- MAJOR: 1

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap progress remains `16 / 20 = 80%`.

The builder log's regression/publication claims are internally plausible and many source-level fixes are real. All eight target GitHub manifests are now on `hiveai-project-control-plane/v1`. However, the central product contract is still not satisfied: H!veAI can still invent stale current-task/progress truth when normalized state is unresolved, and remote-only GitHub changes are not actually discovered by the safety scheduler.

---

# Confirmed closures

## UCP-R13 — CLOSED
Production now models:

- `events: String`
- `eventSources: Vec<String>`

and the current eight target manifests use scalar `events` + array `eventSources`.

## UCP-R14 — CLOSED
Independent target-branch inspection confirms all eight repositories now use:

`hiveai-project-control-plane/v1`

with the required heterogeneous canonical task sources preserved.

## UCP-R15 — CLOSED
Generic adoption no longer hard-codes TASKS.md. It performs bounded source selection and creates human-readable Markdown HANDOFF content.

## UCP-R17 — CLOSED
HANDOFF parsing now separates task, milestone, cycle, actor, audit, session, and next-action fields.

---

# UCP-R19 — BLOCKER
## The promised unified current-state reconciler still does not exist as an authoritative resolver

The user-facing goal is:

> open H!veAI and see the real current project state, then continue exactly where the project stopped.

The M16G implementation still has two competing paths:

### A. `reconcile_control_plane(...)` is not a true project-state resolver

Production `reconcile_control_plane(project)` only:

- reads STATE.json;
- reads HANDOFF.md;
- copies missing HANDOFF fields into STATE;
- detects STATE/HANDOFF conflicts;
- looks up a task title only if a task ID already exists.

It does **not** inspect:

- native M10 workflow rows;
- task-intelligence lifecycle state;
- latest audit/session provenance;
- canonical current milestone/cycle scope;
- canonical next actionable task when STATE/HANDOFF are unresolved.

Its signature does not even receive `DatabaseState`, so it cannot implement the required “verified active native workflow first” precedence.

### B. Command Center still contains the old heuristic fallback

In `command_center::summarize_project(...)`, when no workflow/materialized current task exists, production still executes:

`tasks.iter().find(|task| !task_is_complete(...))`

That is the old “first incomplete/open task” heuristic.

This is exactly the failure class that previously showed stale `SB-M02-017`.

The same function also falls back to:

`progress_percent(completed_tasks, total_tasks)`

over the entire canonical task set when no materialized progress exists.

That reproduces the old whole-file percentages such as 69%, 23%, or 4% instead of current milestone/cycle-scoped progress.

### C. Actual repository state proves this path remains live

Independent inspection of current target branches shows seven of eight STATE/HANDOFF pairs still contain bootstrap placeholders:

- currentTaskId = null / NEEDS_RECONCILIATION
- current milestone = null / NEEDS_RECONCILIATION
- workflow = IDLE
- progress = null
- next action = generic reconciliation text

Only ScrubBots-Level-Factory currently carries meaningful normalized current state.

Therefore, after local repositories pull the new control-plane files, the old Command Center heuristic remains reachable for most projects.

### Required remediation

Implement **one shared authoritative ProjectTruthResolver** used by:

- control-plane reconciliation;
- Command Center;
- Project Cockpit.

Required precedence:

1. verified active native workflow task;
2. explicit valid STATE currentTaskId;
3. explicit valid HANDOFF currentTaskId;
4. exact canonical task lookup for that ID;
5. explicit current milestone/cycle scoped task evidence;
6. otherwise `NEEDS_RECONCILIATION`.

Never fall back to the first incomplete task globally.

Never calculate current milestone progress from all historical canonical tasks.

If authoritative current task/scope is unavailable, show Unknown/Needs reconciliation, not a guessed task or percentage.

Add exact direct regressions for:

- ScrubBots stale `SB-M02-017`;
- fmcg transition prose;
- seven bootstrap-placeholder portfolio states;
- milestone-scoped progress;
- no-current-task state returns NEEDS_RECONCILIATION rather than first open task.

---

# UCP-R20 — BLOCKER
## Remote-only GitHub changes are still not discoverable by the background live-sync path

The product requirement is that a GitHub-side project change eventually appears in H!veAI without requiring an external manual fetch.

Current safety reconciliation in `watcher.rs` only calls:

`control_plane::sync_remote(...)`

when `control_plane_auto_ff` is enabled.

Therefore, with auto-fast-forward disabled, the background scheduler performs **no remote fetch at all**.

Even when auto-fast-forward is enabled, `sync_remote(...)` first computes a Git snapshot and calls `git_sync_plan(...)`.

If local remote-tracking refs still say:

- ahead = 0
- behind = 0

then `fetch_required = false` and `sync_remote` returns **before running git fetch**.

But a new GitHub commit is invisible precisely until fetch updates the remote-tracking ref.

This creates a circular blind spot:

> no fetch because behind=0 → behind remains 0 because no fetch.

So a GitHub-only change can remain invisible indefinitely.

### Required remediation

Separate:

1. **remote observation** from
2. **local mutation**.

The 60-second safety scheduler must safely fetch configured upstream metadata for eligible Git repositories regardless of the auto-fast-forward setting.

After fetch:

- recompute ahead/behind/diverged;
- update H!veAI projection immediately.

Only the **merge --ff-only** step is gated by owner auto-fast-forward.

Required behavior:

- auto-FF OFF + remote advanced → H!veAI shows BEHIND / sync attention, no local mutation;
- auto-FF ON + clean strict-behind → fetch, then ff-only;
- dirty/ahead/diverged → fetch may update remote observation, but no merge/reset/rebase/stash;
- fetch failure → explicit sync warning, no state invention.

Add an end-to-end regression using a bare/upstream repo where a remote commit is created after the local tracking ref is in-sync. Prove the scheduler discovers it without any external manual fetch.

---

# UCP-R21 — MAJOR
## EVENT_INDEX idempotency is not crash-consistent and is only bounded to the newest 4096 IDs

M16G added `.hiveai/EVENT_INDEX.json`, which improves replay detection beyond the 128-event display tail.

However production append order is:

1. append event line to EVENTS.jsonl;
2. then persist EVENT_INDEX.json.

If step 1 succeeds and step 2 fails or the process exits, the event exists in history but not in the index.

On retry:

- `load_event_index` trusts an existing index file;
- it does not reconcile missing recent event IDs from EVENTS.jsonl;
- the same logical event can be appended again.

There is also a fixed 4096-ID truncation. Once an ID falls out of the index, a later replay can again pass the index check.

This is weaker than the builder-log claim that event IDs are durable idempotency identities.

### Required remediation

Make event append replay-safe across interrupted index persistence.

At minimum:

- validate candidate ID against durable index **and** the bounded recent EVENTS tail;
- repair/reconcile the index from the recent tail before append;
- add a failpoint test where event append succeeds but index persistence fails, then retry;
- ensure the retry does not append a duplicate.

For long-term identity, either:

- use a DB-backed project+eventId uniqueness table; or
- document and enforce a deterministic bounded idempotency horizon with explicit behavior instead of implying unlimited durability.

Add >4096-history behavior coverage.

---

# Additional source/test audit notes

1. The direct M16G tests cover schema round-trip and >256 event replay, but no exact production regression named `SB-M02-017` exists in Command Center tests.
2. The shared resolver promised by the prompt is not present; Project Cockpit obtains its `project_summary` from Command Center, while control-plane state uses a separate reconciliation implementation.
3. Seven current remote STATE/HANDOFF pairs remain explicit placeholders. That is acceptable only if H!veAI truthfully shows NEEDS_RECONCILIATION. It is not acceptable to replace those unknowns with first-open-task/global-progress heuristics.

---

# Required closure strategy

Do not patch these separately.

The next remediation must close UCP-R19, UCP-R20, and UCP-R21 in one run, then perform a fresh whole-M16 adversarial sweep.

Final required state:

1. one shared ProjectTruthResolver;
2. no first-open-task fallback for adopted projects;
3. no whole-file progress fallback for current milestone progress;
4. unresolved project truth remains explicitly unresolved;
5. remote-only GitHub commits are discovered by the 60-second safety fetch even when auto-FF is disabled;
6. auto-FF controls mutation only, not remote observation;
7. event idempotency survives an interrupted index update;
8. all eight project fixtures pass;
9. Command Center and Project Cockpit show the same current task/progress/next action;
10. M16 R82-R85 and all prior UCP regression gates remain green.

Only after an independent whole-system PASS and owner native/visual acceptance may M16 close.
