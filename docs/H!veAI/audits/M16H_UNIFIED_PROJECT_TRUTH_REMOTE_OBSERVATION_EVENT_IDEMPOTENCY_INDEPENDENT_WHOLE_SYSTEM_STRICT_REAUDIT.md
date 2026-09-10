# M16H Unified Project Truth / Remote Observation / Event Idempotency — Independent Whole-System Strict Re-Audit

Date: 2026-09-09  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16H_UNIFIED_PROJECT_TRUTH_REMOTE_OBSERVATION_EVENT_IDEMPOTENCY_CLOSURE_LOG.md`  
Implementation commit reviewed: `6acf4a36917f261fa8b23619a5d87043d3a09418`

## Verdict

**FAIL / CHANGES REQUIRED**

### M16H named findings
- UCP-R19: PARTIALLY CLOSED
- UCP-R20: PARTIALLY CLOSED
- UCP-R21: CLOSED within the explicitly accepted bounded horizon

### New whole-system findings
- BLOCKER: 2
- MAJOR: 3
- MINOR: 0

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap progress remains `16 / 20 = 80%`.

The M16H implementation is materially better. It removes the old first-open-task/global-progress fallback from Command Center, adds a shared typed `ProjectTruth` read path, makes remote observation fetch-first, and closes the recent crash window in EVENT_INDEX replay protection. However the repository/file-level truth contract requested by the owner is still not complete, and two source paths can still present false “healthy/current” state.

---

# Confirmed closures

## UCP-R21 — CLOSED under the accepted bounded model

Production now:

- reconciles recent durable EVENTS tail IDs into the sidecar before append;
- rejects a retry after an append succeeded but index persistence failed;
- repairs the sidecar on duplicate retry;
- explicitly documents the 4096-ID / bounded-tail horizon;
- has a direct failpoint test for the crash window;
- has >4096 horizon coverage.

This satisfies the accepted bounded-horizon option from M16H.

## Old first-open/global-progress heuristic — CLOSED in Command Center

The previous fallback selecting the first incomplete canonical task is gone from `summarize_project(...)`.

Command Center now uses `ProjectTruthResolver` for current task and progress.

---

# UCP-R22 — BLOCKER
## ProjectTruthResolver is only a read projection; normalized project files are still not materialized from the shared truth resolver

The owner's core requirement is not merely “compute the right task in memory.” It is:

> every project uses the same control files, those files update after each meaningful action, and H!veAI can immediately read the updated truth.

M16H does not yet implement that contract.

### Production evidence

`ProjectTruthResolver::resolve(database, project_id)` computes a typed `ProjectTruth`.

But `reconcile_control_plane(project)` is still the old file-only reconciler. It:

- has no `DatabaseState`;
- reads STATE;
- reads HANDOFF;
- copies missing HANDOFF values into STATE;
- records STATE/HANDOFF conflicts;
- optionally fills a title from the canonical task file.

It does **not** call `ProjectTruthResolver`.

`snapshot(...)` currently does:

1. `reconcile_control_plane(&project)`
2. `resolve_project(&project)`
3. `ProjectTruthResolver::resolve(...)`
4. overlays the calculated truth into the returned in-memory snapshot

That means the UI can temporarily show a calculated truth while `.hiveai/STATE.json` and `.hiveai/HANDOFF.md` remain stale/bootstrap.

### Cross-module evidence

The native workflow module contains no control-plane reconciliation hook. A workflow transition can change M10 truth without materializing the shared normalized STATE/HANDOFF files.

Therefore restart, another process, another machine, GitHub, or a later parser can still see stale control-plane files.

This directly violates M16H gates:

- “Wire resolver to control plane”
- “State write rules”
- “watcher + reconciliation convergence”
- “files update after meaningful workflow/Git/task changes”

### Required remediation

Create one DB-aware materializer, for example:

`materialize_project_truth(database, project_id, trigger)`

It must:

1. call the shared ProjectTruthResolver;
2. compare against current STATE atomically;
3. write only authoritative resolved fields;
4. write explicit reconciliation status/conflicts when unresolved;
5. update HANDOFF only under its governance policy and only as a human-readable resume pointer;
6. append a bounded idempotent control-plane event;
7. never rewrite canonical task truth;
8. avoid watcher self-write loops.

Invoke it from:

- explicit Reconcile;
- startup/safety pass;
- canonical task intelligence refresh;
- workflow transition completion;
- audit lifecycle transition;
- agent-session completion where it changes task truth;
- successful Git fetch/fast-forward reconciliation;
- owner-approved repair/adoption.

Add a direct test proving a native workflow transition changes the shared truth **and** materializes the same current task into STATE, survives a fresh process/read, and is shown identically by Command Center + Project Cockpit.

---

# UCP-R23 — BLOCKER
## An unresolved adopted project can be rewritten from NEEDS_RECONCILIATION to HEALTHY by Git health calculation

In `snapshot(...)`, M16H correctly does:

- if truth.reconciliation_state == NEEDS_RECONCILIATION → `result.health = "NEEDS_RECONCILIATION"`

But immediately afterward, if Git snapshot succeeds:

`result.health = health_for(&result, &result.git)`

`health_for(...)` does not preserve `NEEDS_RECONCILIATION`.

For an adopted project with:

- no blockers;
- clean local Git;
- IN_SYNC Git;

it returns:

`HEALTHY`

even when the shared truth resolver has no authoritative current task and explicitly returned NEEDS_RECONCILIATION.

### Impact

A project can display:

- no authoritative current task;
- reconciliation required;

while health simultaneously reports:

- HEALTHY.

This is exactly the kind of contradictory project status the owner asked H!veAI to eliminate.

### Required remediation

Health precedence must be explicit and monotonic.

At minimum:

1. malformed/unadopted
2. NEEDS_RECONCILIATION
3. BLOCKED
4. Git sync attention
5. HEALTHY

Git cleanliness must never erase truth-health degradation.

Add direct regression:

- adopted + clean + in-sync + no current truth
- expected health = NEEDS_RECONCILIATION
- never HEALTHY

Also assert Command Center and Cockpit present the same health.

---

# UCP-R24 — MAJOR
## Multiple active native workflow tasks are silently reduced to one arbitrary “current” task

`ProjectTruthResolver` builds all active workflow candidates, then chooses one by sorting:

- attention flag;
- latest event time;
- task ID.

That is a ranking heuristic, not authoritative current-task evidence.

If two tasks are both active/workflow-managed, the resolver silently selects one.

The owner's contract requires exact resumability, not a best guess.

### Required remediation

If multiple active workflow candidates exist and no stronger explicit current-task identity disambiguates them:

- do not rank one into “current”;
- return NEEDS_RECONCILIATION;
- include the conflicting task IDs in provenance/warnings.

If one explicit STATE/HANDOFF task matches one of the active workflow tasks and is valid, that can disambiguate only under the documented precedence/conflict policy.

Add direct two-active-workflow ambiguity test.

---

# UCP-R25 — MAJOR
## Explicit progressPercent is accepted without proving progressScope matches the resolved milestone/cycle

M16H requires:

> STATE scoped progress is valid only when its scope identity matches the current milestone/cycle.

Current production takes STATE `progressScope` if present and then accepts STATE `progressPercent` whenever:

- a selected task exists; and
- any non-empty progressScope exists.

It does not verify that, for example:

- `progressScope = MILESTONE:M01`

matches:

- `currentMilestone = M02`.

A stale progress value can therefore survive a milestone change and appear as current progress.

### Required remediation

Validate exact scope identity:

- if current cycle exists and progress is cycle-scoped, exact cycle match;
- if milestone-scoped, exact milestone match;
- mismatch → reject progress and set reconciliation warning;
- never silently reuse stale percent.

Add direct stale-scope test.

---

# UCP-R26 — MAJOR
## Background fetch failure is discarded instead of becoming persistent sync/degraded evidence

M16H correctly makes `sync_remote(...)` fetch first.

On fetch failure it returns:

- action = SYNC_ATTENTION
- reason = SAFE_FETCH_FAILED...

But the 60-second scheduler currently calls:

`let _ = control_plane::sync_remote(...)`

and discards the returned plan.

It then performs another local snapshot refresh.

Therefore a failed remote observation can leave the visible projection looking like the previous successful/in-sync state, with no persistent fetch-failure warning.

This violates the M16H requirement:

> fetch failure → explicit sync warning, no state invention.

### Required remediation

Persist remote-observation result independently of local Git snapshot:

- lastRemoteObservationAt
- lastRemoteObservationStatus
- lastRemoteObservationError
- observed ahead/behind/diverged when successful

Command Center/Cockpit must surface a current fetch failure as sync attention without destroying the last known good Git counts.

Add a scheduler-level fetch-failure regression, not only a direct `sync_remote` return-value test.

---

# Whole-system acceptance gap

The builder log says the complete M16H gate set was executed, but source inspection shows several prompt gates were only partially met:

- shared resolver exists, but control-plane file materialization does not use it;
- Command Center/Cockpit read convergence exists, but durable STATE convergence does not;
- remote fetch-first exists, but scheduler error projection is discarded;
- progress carries a scope string, but exact scope matching is not enforced.

These are production-contract issues, not documentation-only gaps.

---

# Required final remediation

The next remediation must close R22-R26 together in one continuous run.

Do not create separate cycles.

Required final invariants:

1. one ProjectTruthResolver;
2. one DB-aware ProjectTruthMaterializer;
3. STATE is the normalized durable materialization of resolved truth, not a stale bootstrap cache;
4. HANDOFF remains human-readable and governance-safe;
5. workflow/task/audit/agent/Git transitions trigger convergence;
6. no watcher self-loop;
7. unresolved truth can never report HEALTHY;
8. multiple active workflow ambiguity fails closed;
9. progress scope must exactly match current milestone/cycle;
10. background remote-fetch errors are persisted and visible;
11. Command Center, Cockpit, STATE, and fresh-restart read all agree;
12. all R82-R85 and UCP R13-R21 regressions remain green.

After remediation, run a new whole-M16 adversarial sweep.

Only after:

1. independent whole-system PASS; and
2. owner native/visual acceptance

may M16 close and M17 activate.
