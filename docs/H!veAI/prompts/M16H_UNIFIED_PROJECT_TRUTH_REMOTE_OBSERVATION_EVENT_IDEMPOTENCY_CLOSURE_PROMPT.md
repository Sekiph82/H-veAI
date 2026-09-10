# M16H Unified Project Truth + Remote Observation + Event Idempotency Final Closure Prompt

Date: 2026-09-09
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous whole-system remediation run.

Read and close every finding in:

`H!veAI/docs/H!veAI/audits/M16G_UNIFIED_PROJECT_CONTROL_PLANE_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`

Close together:

- UCP-R19 BLOCKER
- UCP-R20 BLOCKER
- UCP-R21 MAJOR

Do not stop after any individual finding.

Preserve all previously closed M16 audit-engine and UCP findings.

After named fixes, perform a fresh whole-M16 adversarial sweep. If any adjacent BLOCKER or MAJOR is found, fix it in this same run and add a direct regression.

Do not activate M17.
Do not start M21.
M16 remains OPEN until independent strict re-audit + owner native/visual acceptance.

---

# 1. Read before editing

Read completely:

1. `H!veAI/GPT.md`
2. all M16A-G prompts/logs/audits
3. `M16G_UNIFIED_PROJECT_CONTROL_PLANE_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`
4. `control_plane.rs`
5. `command_center.rs`
6. `project_cockpit.rs`
7. `project_dashboard.rs`
8. task intelligence
9. workflow
10. watcher
11. Git engine
12. Registry
13. current direct tests
14. current eight target repository PROJECT/STATE/HANDOFF/task files

Builder logs are claims only.

---

# 2. UCP-R19 — implement one shared ProjectTruthResolver

The current product has split truth logic.

Replace that with one shared resolver consumed by:

- control-plane state reconciliation;
- Command Center project summary;
- Project Cockpit.

## 2.1 Required output

Create a typed resolved truth structure carrying at minimum:

- projectId
- currentTaskId
- currentTaskTitle
- currentTaskStatus
- currentMilestone
- currentCycle
- workflowState
- requiredActor
- nextAction
- blockers
- progressPercent
- progressScope
- authoritySource
- provenance
- reconciliationState
- warnings

## 2.2 Authority precedence

Resolve in this order:

1. verified active native workflow task that belongs to canonical task authority;
2. valid explicit STATE currentTaskId;
3. valid explicit HANDOFF currentTaskId;
4. exact canonical task lookup for that ID;
5. explicit current milestone/cycle scoped canonical evidence;
6. otherwise NEEDS_RECONCILIATION.

Never infer current task from the first incomplete task in the whole file.

Never use a milestone heading as a task.

Never use transition prose as a task.

Never revive an old historical OPEN task merely because it appears before later tasks.

## 2.3 Remove old first-open fallback

In Command Center, remove the branch equivalent to:

`tasks.iter().find(!task_is_complete)`

for adopted control-plane projects.

If there is no authoritative current task:

- currentTask = null
- reconciliationState = NEEDS_RECONCILIATION
- nextAction explains what evidence is missing

Do not guess.

## 2.4 Shared use

Command Center and Project Cockpit must call the same resolver.

Control-plane reconciliation must call the same resolver with DatabaseState available.

Do not maintain duplicate precedence logic.

## 2.5 State write rules

The resolver may materialize normalized STATE only when evidence is authoritative.

Write:

- task ID/title
- milestone
- cycle
- workflow
- actor
- next action
- blockers
- progress

only from verified sources.

If sources conflict:

- set workflow/reconciliation to NEEDS_RECONCILIATION
- persist explicit conflict details
- keep disputed values out of authoritative fields

---

# 3. Current milestone progress semantics

Remove whole-project checkbox fallback for current milestone progress.

Current milestone/cycle progress is valid only if:

1. STATE has explicit scoped progress and scope identity matches current milestone/cycle; or
2. canonical task intelligence can deterministically group tasks under the resolved current milestone/cycle.

Otherwise:

`progressPercent = null`

Do not derive milestone progress from every historical task in the canonical ledger.

Portfolio total/completed task counts may still exist as separate portfolio metrics, but they must not be displayed as “Milestone progress”.

Add explicit `progressScope` provenance.

---

# 4. Exact portfolio truth regressions

Add direct fixtures for all eight projects.

## AI-Commerce-HQ
With bootstrap STATE/HANDOFF placeholders:
- no guessed first-open task
- truth = NEEDS_RECONCILIATION unless native workflow gives a verified current task
- nested H!veAI/TASKS.md remains canonical

## Bulk-Edit
- do not reproduce old arbitrary 69% unless scoped evidence proves it
- no prose-only task identity

## fmcg-erp-system
- sentence containing closed TASK-014.3 and “next TASK-014.4” is not one synthetic task
- resolve exact canonical task only if authoritative

## FormuLab
- nested tracker remains authoritative
- unresolved current pointer returns NEEDS_RECONCILIATION, not first open tracker row

## PackLab
- unresolved state remains explicit
- known remote remains visible

## PackLab-3D
- lowercase tasks.md
- no first-open fallback

## ScrubBots
Add the exact stale regression:
`SB-M02-017`

If current workflow/state/handoff does not authoritatively point to it, it must never become current task merely because it is the first incomplete historical task.

## ScrubBots-Level-Factory
- keep authoritative PAG-M02-C002 state
- cycle/task distinctions remain correct
- actor/next action remain correct

---

# 5. UCP-R20 — remote observation must not depend on auto-fast-forward

Remote observation and local mutation are separate concepts.

## 5.1 Safety scheduler

Every 60 seconds, for each eligible ACTIVE local Git repository with a configured remote/upstream:

1. safely fetch remote metadata using bounded Git execution;
2. recompute upstream/ahead/behind/diverged;
3. persist/update H!veAI Git projection;
4. refresh Command Center / Cockpit.

This remote observation happens regardless of auto-fast-forward setting.

## 5.2 Auto-fast-forward semantics

The setting controls only whether H!veAI may perform:

`merge --ff-only @{upstream}`

after fetch.

### auto-FF OFF

If remote advanced:

- fetch
- show BEHIND
- do not mutate worktree

### auto-FF ON

If:
- clean
- ahead=0
- behind>0
- not diverged
- branch/upstream valid

then:
- fetch
- ff-only
- reconcile
- refresh

### dirty/ahead/diverged

Fetch may update remote observation.

Never:

- reset
- rebase
- stash
- force checkout
- merge divergence
- delete local changes

## 5.3 Fix the circular blind spot

Do not decide whether to fetch from pre-fetch behind counts.

A remote commit is invisible until fetch.

Therefore the scheduler must attempt bounded fetch first when remote observation is due, then compute the plan.

Manual `Sync remote` must follow the same observation-first rule.

---

# 6. Remote-only end-to-end regression

Create a real Git fixture:

1. create bare upstream;
2. clone local;
3. local remote-tracking ref is in-sync;
4. create/push a new commit from a second clone;
5. do not manually fetch local;
6. run H!veAI remote observation;
7. prove local projection changes from IN_SYNC to BEHIND;
8. with auto-FF OFF prove local HEAD does not move;
9. with auto-FF ON + clean prove ff-only updates local HEAD;
10. with dirty worktree prove no merge occurs;
11. with divergence prove no merge occurs.

This is mandatory.

---

# 7. UCP-R21 — crash-consistent event idempotency

Current append order can leave EVENTS updated while EVENT_INDEX remains stale.

Fix replay protection.

## 7.1 Required minimum

Before append:

- load durable index;
- read bounded recent EVENTS tail;
- reconcile missing recent IDs into the effective idempotency set;
- reject duplicate if present in either.

After append:

- persist index.

If index persistence fails after event append, a retry must still be rejected because the just-appended event exists in the bounded tail.

## 7.2 Long-horizon behavior

Do not silently imply infinite identity durability if the index is capped.

Choose one:

### Preferred
DB table with unique:

`(project_id, event_id)`

and bounded metadata.

Or:

### Accepted bounded model
Explicit documented idempotency horizon with:
- maximum retained IDs
- deterministic pruning
- warning when horizon is exceeded

But even under bounded mode, interrupted recent append must be replay-safe.

## 7.3 Direct failpoint

Add a test:

1. valid index exists;
2. append event succeeds to EVENTS;
3. fail index persistence;
4. retry same event;
5. retry must not append a second line;
6. repair index;
7. subsequent retry still rejected.

Also test history >4096 records.

---

# 8. Watcher + reconciliation convergence

Ensure a meaningful file/workflow/Git change results in one converged project truth.

Requirements:

- canonical task watcher triggers task intelligence refresh;
- state/handoff watcher triggers truth re-resolution;
- Git fetch/FF triggers truth re-resolution;
- no self-write infinite loop;
- no duplicate lifecycle event churn;
- Command Center and Cockpit update from the same resolver result.

---

# 9. Actual eight-repository verification

Do not rewrite remote control-plane files unless a real fix requires it.

Before final log verify all eight target branches still have:

`hiveai-project-control-plane/v1`

and correct canonical task paths.

Fetch and record target HEAD + PROJECT.json schema for:

- AI-Commerce-HQ
- Bulk-Edit
- fmcg-erp-system
- FormuLab
- PackLab
- PackLab-3D
- ScrubBots
- ScrubBots-Level-Factory

Preserve task ledgers and stricter governance.

---

# 10. Full tests

Required focused tests:

1. shared resolver workflow precedence
2. STATE precedence
3. HANDOFF precedence
4. unresolved state returns NEEDS_RECONCILIATION
5. exact ScrubBots SB-M02-017 stale regression
6. fmcg transition-prose regression
7. no first-open fallback
8. no whole-file milestone progress fallback
9. scoped progress
10. eight project fixtures
11. Command Center/Cockpit same truth
12. remote-only commit discovery auto-FF OFF
13. remote-only commit FF auto-FF ON
14. dirty refusal
15. divergence refusal
16. fetch failure degraded state
17. event index interrupted-write replay
18. >4096 event behavior
19. M16 R82-R85
20. all prior UCP regressions

Then run:

- full serialized Rust
- all-targets default
- pty-support
- full frontend suite once
- typecheck
- production build
- npm audit high
- cargo fmt
- git diff --check
- publisher rollback 9/9
- governed publication
- candidate/stable SHA equality
- PE/startup/shortcut/icon/no-console checks

---

# 11. Final adversarial sweep

Inspect the entire active M16 system again:

- audit engine
- control plane
- state resolver
- task intelligence
- workflow
- Command Center
- Project Cockpit
- watcher
- Registry
- Git sync
- event history/idempotency
- migrations
- ACL/native commands
- tests
- publication

If a new BLOCKER or MAJOR is found, fix it in the same run.

Do not stop and request another prompt.

---

# 12. Explicit gates

1. Read GPT.md.
2. Sync H!veAI FF-only.
3. Record starting HEAD.
4. Read M16G strict audit.
5. Reproduce R19.
6. Reproduce R20.
7. Reproduce R21.
8. Design shared ProjectTruthResolver.
9. Wire resolver to control plane.
10. Wire resolver to Command Center.
11. Wire resolver to Project Cockpit.
12. Remove first-open task fallback.
13. Remove whole-file milestone progress fallback.
14. Add scoped progress provenance.
15. Add unresolved truth state.
16. Add SB-M02-017 regression.
17. Add fmcg prose regression.
18. Add all-eight truth fixtures.
19. Implement observation-first remote fetch.
20. Decouple fetch from auto-FF.
21. Update 60-second scheduler.
22. Update manual Sync remote.
23. Add remote-only commit fixture.
24. Prove auto-FF OFF no mutation.
25. Prove auto-FF ON ff-only.
26. Prove dirty refusal.
27. Prove divergence refusal.
28. Make event replay crash-consistent.
29. Add index-persist failpoint.
30. Add >4096 behavior test.
31. Verify watcher convergence.
32. Verify Command Center/Cockpit convergence.
33. Verify all eight remote schemas.
34. Verify canonical task paths.
35. Preserve governance.
36. Run focused Rust.
37. Run focused frontend.
38. Run R82-R85 regressions.
39. Run full Rust.
40. Run pty.
41. Run full frontend once.
42. Run typecheck/build/audit/fmt/diff.
43. Perform whole-M16 adversarial sweep.
44. Fix adjacent BLOCKER/MAJOR same run.
45. Re-run affected suites.
46. Run publisher rollback.
47. Governed publish.
48. Verify stable SHA.
49. Native smoke if feasible.
50. Create immutable builder log.
51. Commit scoped changes.
52. Push normally.
53. Verify local/origin equality.
54. Leave M16 OPEN pending independent strict re-audit + owner acceptance.
55. Do not activate M17.
56. Do not start M21.

---

# 13. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16H_UNIFIED_PROJECT_TRUTH_REMOTE_OBSERVATION_EVENT_IDEMPOTENCY_CLOSURE_LOG.md`

Record:

- starting HEAD
- implementation commits
- R19/R20/R21 reproduction
- shared resolver architecture
- exact removal of first-open/global-progress heuristics
- eight-project truth matrix
- remote-only Git fixture evidence
- auto-FF OFF/ON evidence
- dirty/diverged safety
- event crash-consistency evidence
- >4096 behavior
- Command Center/Cockpit convergence
- full test counts
- adversarial sweep
- publication SHA
- final pushed HEAD
- native acceptance pending

End with:

`M16H WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
