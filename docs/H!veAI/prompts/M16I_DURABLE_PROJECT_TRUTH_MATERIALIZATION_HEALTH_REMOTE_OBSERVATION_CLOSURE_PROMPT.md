# M16I Durable Project Truth Materialization + Health Precedence + Remote Observation Closure Prompt

Date: 2026-09-09
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous whole-system remediation run.

Read and close every finding in:

`H!veAI/docs/H!veAI/audits/M16H_UNIFIED_PROJECT_TRUTH_REMOTE_OBSERVATION_EVENT_IDEMPOTENCY_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`

Close together:

- UCP-R22 BLOCKER
- UCP-R23 BLOCKER
- UCP-R24 MAJOR
- UCP-R25 MAJOR
- UCP-R26 MAJOR

Do not stop after any individual finding.

Preserve all previously closed M16 audit-engine and UCP findings, including UCP-R21's bounded event-idempotency model.

After all named fixes, perform a fresh whole-M16 adversarial sweep. If any adjacent BLOCKER or MAJOR is discovered, fix it in the same run and add a direct regression.

Do not activate M17.
Do not start M21.
M16 remains OPEN until independent strict re-audit + owner native/visual acceptance.

---

# 1. Required reading

Before edits read:

1. `H!veAI/GPT.md`
2. all M16A-H prompts/logs/audits
3. the M16H strict re-audit above
4. `control_plane.rs`
5. `command_center.rs`
6. `project_cockpit.rs`
7. `project_dashboard.rs`
8. `workflow.rs`
9. audit engine / audit lifecycle source
10. agent-session lifecycle source
11. task intelligence
12. watcher
13. Git engine
14. Registry
15. DB migrations
16. native command/ACL registrations
17. current direct tests
18. current eight target repository control-plane files

Builder logs are claims only.

---

# 2. UCP-R22 — add one durable ProjectTruthMaterializer

The shared ProjectTruthResolver is read-only today.

Implement one DB-aware materializer that owns normalized project-state persistence.

Suggested public API:

`materialize_project_truth(database, project_id, trigger) -> MaterializationResult`

Do not duplicate resolution logic. The materializer must call the existing shared ProjectTruthResolver.

## 2.1 Materialized STATE contract

STATE must be the durable normalized materialization of the resolved project truth.

Persist at minimum:

- schema
- projectKey
- reconciliationStatus
- workflowState
- currentMilestone
- currentCycle/currentSprint according to supported compatibility shape
- currentTaskId
- currentTaskTitle
- requiredActor
- nextAction
- blockers
- progressPercent
- progressScope
- authoritySource
- provenance summary / evidence refs where the schema supports them
- updatedAt
- updatedBy / trigger

Rules:

- never invent a current task;
- disputed current-task fields must be null when unresolved;
- stale prior values must be cleared when authoritative evidence says “unknown/unresolved”;
- atomic compare-and-replace;
- bounded file size;
- no canonical task file mutation.

## 2.2 HANDOFF contract

HANDOFF remains Markdown.

Update it only when governance permits.

It must remain a resume pointer, not a second competing state database.

When materializing a resolved truth:

- current task ID/title
- milestone/cycle
- actor
- next action
- resume pointer

may be updated to agree with STATE.

When unresolved:

- show NEEDS_RECONCILIATION and explicit reason
- do not preserve a stale previous task as current.

Preserve project-specific additional prose/sections where required by stricter governance.

For repositories where RULES/governance forbids automated HANDOFF mutation, STATE must still materialize correctly and HANDOFF must be reported as externally governed rather than overwritten.

## 2.3 Event emission

A materialization that changes normalized truth should append one idempotent control-plane event.

Use deterministic transition identity so watcher re-entry does not create event storms.

No event when materialized truth is byte/evidence equivalent.

---

# 3. Wire materialization into all meaningful transitions

The same materializer must run after project truth can change.

Required triggers:

1. explicit Reconcile
2. startup reconciliation
3. 60-second safety reconciliation
4. canonical task intelligence refresh success
5. workflow transition completion
6. audit lifecycle transition that changes task authority/state
7. agent session completion/failure/cancellation where task truth changes
8. successful safe remote fetch when observed branch state changes
9. successful ff-only
10. project adoption
11. owner-approved path/Git repair
12. relevant STATE/HANDOFF/canonical-task watcher event

Do not wire bespoke file edits into each subsystem.

Every subsystem calls the same materializer.

---

# 4. Prevent watcher self-write loops

Materialization writes STATE/HANDOFF/EVENTS and watcher observes those files.

Implement deterministic loop suppression.

Accepted approaches include:

- content/revision hash;
- materialization generation token;
- idempotent event identity;
- compare-before-write plus watcher dedupe.

Required proof:

1. one external canonical task/workflow change;
2. one materialization;
3. watcher sees internal writes;
4. no infinite loop;
5. no unbounded event growth;
6. system converges to zero further writes.

Add a direct convergence test.

---

# 5. UCP-R23 — fix health precedence

Project truth health must outrank Git cleanliness.

Implement one health precedence function.

Required order:

1. MISSING / MALFORMED / UNADOPTED
2. NEEDS_RECONCILIATION
3. BLOCKED
4. Git CONFLICTED / DIVERGED / BEHIND / AHEAD / DIRTY sync attention
5. HEALTHY

A clean Git repository can never turn unresolved truth into HEALTHY.

Required regression:

- adopted
- clean
- IN_SYNC
- no authoritative current task
- resolver reconciliationState = NEEDS_RECONCILIATION
- snapshot health = NEEDS_RECONCILIATION
- Command Center = same
- Project Cockpit = same

Also prove resolved + clean + in-sync becomes HEALTHY.

---

# 6. UCP-R24 — multiple active workflow tasks must fail closed

Do not rank multiple active workflow tasks by latest event / attention / lexical task ID.

If more than one active canonical workflow candidate exists:

### Case A: no stronger explicit identity
Return:

- currentTaskId = null
- reconciliationState = NEEDS_RECONCILIATION
- warning lists conflicting task IDs

### Case B: explicit valid STATE/HANDOFF identity matches exactly one candidate
Resolve only if conflict policy says this explicit identity is authoritative and not contradicted by stronger evidence.

### Case C: explicit identity conflicts
Return NEEDS_RECONCILIATION.

Add tests:

1. two active workflows, no explicit ID
2. two active workflows + valid explicit current ID
3. workflow vs STATE conflict
4. workflow vs HANDOFF conflict

Never guess.

---

# 7. UCP-R25 — validate progress scope identity exactly

STATE progress is valid only when its declared scope matches resolved truth.

Implement exact validation.

Accepted normalized forms may include:

- `MILESTONE:<id>`
- `CYCLE:<id>`

Rules:

- if scope is milestone, it must equal currentMilestone
- if scope is cycle, it must equal currentCycle
- mismatch invalidates progressPercent
- mismatch adds reconciliation warning
- stale percent must not remain displayed or materialized as current

If both current cycle and milestone exist, prefer the more specific accepted cycle scope if task grouping supports it.

Canonical computed progress must also be grouped only inside the exact resolved scope.

Tests:

1. matching milestone scope
2. stale milestone scope
3. matching cycle scope
4. stale cycle scope
5. no scope => no current milestone progress
6. historical global completed ratio never leaks in

---

# 8. UCP-R26 — persist background remote observation status

Remote observation is operational truth and must survive/disclose failures.

Add durable fields in DB/project projection, migration if required:

- last_remote_observation_at
- last_remote_observation_status
- last_remote_observation_error
- optional last_remote_observed_upstream
- optional observed ahead/behind/diverged snapshot identity

Use bounded strings.

## 8.1 Successful fetch

Persist:

- SUCCESS
- timestamp
- current upstream
- no error
- observed Git state

Then materialize/refresh project truth as needed.

## 8.2 Failed fetch

Persist:

- DEGRADED or FAILED
- timestamp
- SAFE_FETCH_FAILED reason
- preserve last known good Git counts separately
- surface sync attention in Command Center and Cockpit

Never silently replace last known good counts with invented zero/IN_SYNC.

## 8.3 Scheduler

The 60-second scheduler must not discard the result.

No:

`let _ = sync_remote(...)`

without recording the plan.

Create one remote observation function returning/persisting structured status.

Manual Sync remote must use the same path.

---

# 9. Fresh restart durability test

Create an end-to-end test proving the owner's primary contract.

Scenario:

1. register project
2. canonical task source contains task A and task B
3. native workflow establishes task B as current
4. run materializer
5. inspect STATE on disk
6. destroy/reinitialize runtime/DB handle as realistically supported
7. read project again
8. Command Center current task = B
9. Project Cockpit current task = B
10. STATE current task = B
11. no dependency on an in-memory resolver cache

This is mandatory.

---

# 10. Eight-project control-plane matrix

Re-run all eight fixtures through:

- resolver
- materializer
- fresh read
- Command Center
- Cockpit
- health
- progress
- Git projection

Required canonical sources remain:

- AI-Commerce-HQ → `H!veAI/TASKS.md`
- Bulk-Edit → `TASKS.md`
- fmcg-erp-system → `TASKS.md`
- FormuLab → `docs/FORMULAB_V1_TASK_TRACKER.md`
- PackLab → `TASKS.md`
- PackLab-3D → `tasks.md`
- ScrubBots → `tasks.md`
- ScrubBots-Level-Factory → `tasks.md`

Specific regressions:

- ScrubBots: `SB-M02-017` must not revive
- fmcg: transition prose must not become synthetic task
- Level Factory: retain authoritative PAG-M02-C002 semantics if still current in fetched repo truth
- unresolved projects remain visibly NEEDS_RECONCILIATION, not HEALTHY

---

# 11. Remote observation end-to-end matrix

Using a bare upstream + local + second clone:

1. local in sync
2. remote advances
3. scheduler/manual observation fetches with auto-FF OFF
4. projection becomes BEHIND
5. local HEAD unchanged
6. observation status SUCCESS
7. enable auto-FF
8. strict ff-only succeeds
9. truth materializer runs
10. local HEAD matches upstream

Then separately prove:

- dirty worktree fetch-only
- local ahead fetch-only
- diverged fetch-only
- fetch failure persists DEGRADED status
- recovery after later successful fetch clears current error without deleting historical evidence

---

# 12. Preserve event-idempotency closure

Do not regress UCP-R21.

Keep:

- recent tail reconciliation
- crash-window retry rejection
- sidecar repair
- explicit 4096 bounded horizon
- >4096 tests

If materializer appends events, use the same idempotency guarantees.

---

# 13. Direct test matrix

Add/update direct regressions for:

1. resolver -> materializer STATE persistence
2. HANDOFF safe update
3. externally governed HANDOFF preservation
4. workflow transition triggers materialization
5. task intelligence refresh triggers materialization
6. audit lifecycle trigger
7. agent completion trigger
8. Git fetch/ff trigger
9. startup/safety trigger
10. no self-write loop
11. no duplicate materialization event
12. fresh restart durability
13. unresolved clean Git health
14. resolved clean Git health
15. two active workflow ambiguity
16. explicit task disambiguation
17. workflow/STATE conflict
18. exact milestone progress scope
19. stale milestone scope
20. exact cycle scope
21. stale cycle scope
22. remote observation success persistence
23. remote observation failure persistence
24. auto-FF OFF remote advance
25. auto-FF ON strict FF
26. dirty refusal
27. ahead refusal
28. divergence refusal
29. all eight portfolio fixtures
30. SB-M02-017
31. fmcg transition prose
32. M16 R82-R85
33. all UCP R13-R21 regressions

---

# 14. Full verification

Run:

- focused ProjectTruth tests
- focused materializer tests
- focused watcher convergence tests
- focused workflow/audit/agent transition tests
- focused remote-observation tests
- focused Command Center/Cockpit tests
- all M16 R82-R85 regressions
- all prior UCP regressions
- full serialized Rust
- all-targets default
- pty-support
- frontend Vitest once with zero retry-dependent failure
- TypeScript typecheck
- production frontend build
- npm audit high
- cargo fmt
- git diff --check
- publisher rollback 9/9
- governed publication
- stable candidate SHA equality
- PE/startup/shortcut/icon/no-console checks

---

# 15. Whole-M16 adversarial sweep

Before final log inspect the entire active milestone:

- schema symmetry
- DB migrations
- project registry
- task source/intelligence
- workflow
- audit engine
- agent session lifecycle
- ProjectTruthResolver
- ProjectTruthMaterializer
- STATE/HANDOFF durability
- watcher loop behavior
- event idempotency
- Git remote observation
- auto-FF safety
- Command Center
- Cockpit
- native commands/ACL
- degraded paths
- test quality
- publication

If any adjacent BLOCKER or MAJOR exists, fix it in this same run.

Do not stop for another prompt.

---

# 16. Explicit gates

1. Read GPT.md.
2. Sync H!veAI FF-only.
3. Record starting HEAD.
4. Read M16H strict re-audit.
5. Reproduce R22.
6. Reproduce R23.
7. Reproduce R24.
8. Reproduce R25.
9. Reproduce R26.
10. Design ProjectTruthMaterializer.
11. Add required DB migration if needed.
12. Materialize STATE atomically.
13. Preserve HANDOFF governance.
14. Add idempotent materialization event.
15. Wire explicit Reconcile.
16. Wire startup/safety.
17. Wire task refresh.
18. Wire workflow transition.
19. Wire audit transition.
20. Wire agent transition.
21. Wire Git fetch/FF.
22. Prevent watcher self-loop.
23. Fix health precedence.
24. Add unresolved-clean health test.
25. Fail closed on multiple workflows.
26. Add workflow ambiguity tests.
27. Validate exact progress scope.
28. Add stale-scope tests.
29. Persist remote observation status.
30. Surface remote failure in Command Center.
31. Surface remote failure in Cockpit.
32. Add fresh-restart durability test.
33. Re-run eight-project matrix.
34. Re-run SB-M02-017 regression.
35. Re-run fmcg prose regression.
36. Preserve UCP-R21.
37. Run focused Rust.
38. Run focused frontend.
39. Run R82-R85.
40. Run prior UCP regressions.
41. Run full Rust.
42. Run pty.
43. Run full frontend once.
44. Run typecheck/build/audit/fmt/diff.
45. Perform whole-M16 adversarial sweep.
46. Fix adjacent BLOCKER/MAJOR same run.
47. Re-run affected suites.
48. Publisher rollback 9/9.
49. Governed publish.
50. Verify stable SHA.
51. Native smoke if feasible.
52. Create immutable builder log.
53. Commit scoped changes.
54. Push normally.
55. Verify local/origin equality.
56. Leave M16 OPEN pending independent strict re-audit + owner acceptance.
57. Do not activate M17.
58. Do not start M21.

---

# 17. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16I_DURABLE_PROJECT_TRUTH_MATERIALIZATION_HEALTH_REMOTE_OBSERVATION_CLOSURE_LOG.md`

Required sections:

- starting HEAD
- implementation commits
- R22-R26 reproduction
- ProjectTruthMaterializer architecture
- STATE materialization proof
- HANDOFF governance proof
- transition wiring matrix
- self-loop prevention proof
- health precedence proof
- workflow ambiguity proof
- progress scope proof
- remote observation persistence proof
- fetch failure proof
- fresh restart proof
- eight-project matrix
- full tests
- whole-M16 adversarial sweep
- publication SHA
- final pushed HEAD
- native acceptance pending

End exactly with:

`M16I WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
