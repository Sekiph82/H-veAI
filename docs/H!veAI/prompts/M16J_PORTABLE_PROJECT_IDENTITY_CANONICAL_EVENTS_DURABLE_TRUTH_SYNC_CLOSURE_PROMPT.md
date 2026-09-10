# M16J Portable Project Identity + Canonical Events + Durable Truth Sync Final Closure Prompt

Date: 2026-09-09
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous whole-system remediation run.

Read and close every finding in:

`H!veAI/docs/H!veAI/audits/M16I_DURABLE_PROJECT_TRUTH_MATERIALIZATION_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`

Close together:

- UCP-R27 BLOCKER
- UCP-R28 BLOCKER
- UCP-R29 BLOCKER
- UCP-R30 MAJOR
- UCP-R31 MAJOR

Do not stop after any individual finding.

Preserve every previously closed M16 audit-engine and UCP finding.

After named fixes, perform a fresh whole-M16 adversarial sweep. If any adjacent BLOCKER or MAJOR is found, fix it in the same run and add direct regression coverage.

Do not activate M17.
Do not start M21.
M16 remains OPEN until independent whole-system strict re-audit + owner native/visual acceptance.

---

# 1. Required reading before edits

Read completely:

1. `H!veAI/GPT.md`
2. all M16A-I prompts/logs/audits
3. `M16I_DURABLE_PROJECT_TRUTH_MATERIALIZATION_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`
4. `control_plane.rs`
5. `workflow.rs`
6. `task_intelligence.rs`
7. `audit_engine.rs`
8. `agent_session_center.rs`
9. watcher
10. Registry/projects
11. Command Center
12. Project Cockpit
13. Git engine
14. DB migrations
15. native command/ACL boundaries
16. all existing direct tests
17. actual target-branch `.hiveai/PROJECT.json`, `STATE.json`, `RULES.md`, `HANDOFF.md`, and representative `EVENTS.jsonl` for all eight tracked repositories

Builder logs are claims only.

---

# 2. UCP-R27 — preserve portable repository projectKey

The repository-owned control-plane identity is not the local Registry UUID.

Implement an explicit identity split.

## 2.1 Stable repository identity

Read `ProjectDocument.project_key` from `.hiveai/PROJECT.json`.

This stable key must be used for:

- STATE.projectKey
- HANDOFF project identity if serialized/referenced
- EVENTS.projectKey
- portable control-plane provenance

Never replace it with H!veAI's local Registry UUID.

## 2.2 Local Registry identity

The local Registry UUID belongs only in:

- SQLite Registry/project row
- `PROJECT.json.localRegistryId` only if current machine policy permits this field
- in-memory/native references

A new machine or re-registration may assign a different local UUID without changing the portable projectKey.

## 2.3 Actual required stable keys

Verify and preserve:

- AI-Commerce-HQ → `ai-commerce-hq`
- Bulk-Edit → `bulk-edit`
- fmcg-erp-system → `fmcg-erp-system`
- FormuLab → `formulab`
- PackLab → `packlab`
- PackLab-3D → `packlab-3d`
- ScrubBots → `scrubbots`
- ScrubBots-Level-Factory → `scrubbots-level-factory`

## 2.4 Cross-machine regression

Create a fixture:

1. repository files contain stable projectKey `portable-project`;
2. register locally as UUID A;
3. materialize;
4. assert PROJECT/STATE/EVENTS still use `portable-project`;
5. reinitialize a fresh DB / re-register as UUID B;
6. materialize again;
7. assert portable project files did not change identity.

This is mandatory.

---

# 3. UCP-R28 — one canonical hiveai-event/v1 writer

Create one canonical event writer contract for all new rows.

Every new H!veAI-written event must serialize the common core:

```json
{
  "schema": "hiveai-event/v1",
  "eventId": "...",
  "projectKey": "...",
  "type": "...",
  "at": "...",
  "actor": "...",
  "taskId": null,
  "workflowState": null,
  "summary": "...",
  "commit": null,
  "auditId": null,
  "sessionId": null
}
```

Optional bounded extensions may include:

- `evidenceRefs`
- provider/session metadata

but they must not replace the common core.

## 3.1 Backward reader compatibility

Readers may accept:

- existing canonical rows;
- older H!veAI internal rows if they exist.

Writers must emit canonical v1 only.

## 3.2 Materialization event

`PROJECT_TRUTH_MATERIALIZED` must include:

- stable projectKey
- current workflowState
- current task ID
- canonical schema
- any known audit/session/commit references where available

Do not invent unavailable values. Serialize null.

## 3.3 Direct fixture round-trip

Use representative actual event rows from:

- AI-Commerce-HQ
- ScrubBots-Level-Factory

Prove:

- parse succeeds;
- reserialization retains canonical core;
- newly emitted materialization event has the same core schema.

---

# 4. UCP-R29 — durable truth-sync convergence, never silent failure

The current post-commit `let _ = materialize_project_truth(...)` calls are unacceptable because file sync failure disappears.

Do not attempt a distributed transaction rollback across SQLite and filesystem.

Implement a durable synchronization projection/outbox.

## 4.1 Required DB state

Add versioned migration if needed.

Persist per project at minimum:

- truth_sync_status: CURRENT | PENDING | DEGRADED
- truth_sync_revision/fingerprint
- truth_sync_trigger
- truth_sync_error
- truth_sync_attempted_at
- truth_sync_completed_at
- truth_sync_retry_count

Bound all strings.

## 4.2 Domain transition behavior

For every meaningful native domain transition:

1. commit authoritative domain state;
2. compute expected truth revision/fingerprint;
3. mark truth sync PENDING before or as part of the durable post-commit scheduling path;
4. attempt materialization;
5. on success mark CURRENT for that exact revision;
6. on failure mark DEGRADED/PENDING with bounded error;
7. return domain result plus durable sync status or make surfaces immediately expose pending state.

Never return a UI state that implies repository files are synchronized when they are not.

## 4.3 Required lifecycle integrations

Cover:

- workflow transition
- workflow override
- task intelligence refresh
- audit lifecycle
- agent finish
- agent failure
- agent recovery/orphan repair
- adoption
- explicit reconcile
- remote observation/FF
- path repair
- watcher-triggered reconciliation

## 4.4 Retry

Watcher/startup/60-second safety pass must retry pending sync.

Retry must be:

- idempotent
- revision-aware
- bounded
- restart-safe
- non-destructive

If a newer revision exists, never mark an older revision CURRENT over it.

## 4.5 Surface

Command Center and Cockpit must show:

- truth sync CURRENT
- truth sync PENDING/DEGRADED
- bounded last error
- last attempt/completion

A project with stale repository files cannot be HEALTHY.

## 4.6 Failpoint tests

Add filesystem/materialization failpoints proving:

1. workflow commits, STATE write fails;
2. domain operation persists;
3. truth sync becomes DEGRADED/PENDING;
4. restart occurs;
5. safety retry later succeeds;
6. exact latest revision becomes CURRENT;
7. UI surfaces recover.

Repeat representative coverage for audit, agent, and task refresh.

---

# 5. UCP-R30 — lossless HANDOFF managed block

Do not rebuild the whole HANDOFF body.

Introduce explicit managed markers, for example:

```md
<!-- HIVEAI:BEGIN MANAGED CURRENT -->
...
<!-- HIVEAI:END MANAGED CURRENT -->
```

Rules:

- replace only the managed block;
- preserve all bytes outside the block as much as line-ending normalization permits;
- preserve project-specific notes across unlimited reruns;
- preserve project-specific headings and governance text;
- never duplicate the managed block;
- if no managed block exists and automation is allowed, insert one safely without destroying the existing document.

For governance-disallowed repositories, do not mutate HANDOFF at all.

## 5.1 ScrubBots-Level-Factory

Its PROJECT/RULES governance explicitly says builder automation may not rewrite HANDOFF without owner-approved authority.

Honor both:

- machine-readable PROJECT governance; and
- RULES.md governance.

Do not rely only on substring scanning of RULES.md.

## 5.2 Required tests

1. first materialization preserves notes
2. second materialization preserves notes
3. ten repeated runs byte-preserve unmanaged sections
4. existing managed block is replaced in place
5. malformed/multiple marker case fails closed
6. owner-governed HANDOFF remains byte-identical
7. ScrubBots-Level-Factory governance fixture

---

# 6. UCP-R31 — progress percent/scope invariant

Enforce globally:

`progressPercent != null => progressScope != null`

and the scope must exactly match the resolved current milestone/cycle.

## 6.1 Explicit STATE progress

If STATE has:

- matching `MILESTONE:<id>` or `CYCLE:<id>` → may accept percent
- stale/mismatched scope → reject stale percent and warn

## 6.2 Computed progress

If H!veAI deterministically computes progress from canonical tasks:

- compute only in the exact resolved milestone/cycle;
- set the corresponding generated `progressScope` at the same time.

Never compute a percent while returning `progressScope = null`.

## 6.3 Required tests

- exact milestone scope
- stale milestone scope
- exact cycle scope
- stale cycle scope
- computed milestone scope
- computed cycle scope where supported
- no scope/no percent invariant
- global historical ratio never leaks into current milestone progress

Add a property/invariant test over all resolver outputs.

---

# 7. Actual eight-repository contract matrix

This run must test the actual shared contract shapes, not only freshly adopted fixtures.

For all eight:

- parse PROJECT
- preserve stable projectKey
- parse STATE
- materialize with a synthetic local Registry UUID different from projectKey
- verify canonical task source
- verify event schema
- verify HANDOFF governance
- verify percent/scope invariant
- verify fresh restart
- verify Command Center/Cockpit convergence

Repositories:

1. AI-Commerce-HQ / H!veAI
2. Bulk-Edit / main
3. fmcg-erp-system / main
4. FormuLab / feature/laboratory-stability
5. PackLab / main
6. PackLab-3D / main
7. ScrubBots / main
8. ScrubBots-Level-Factory / main

Do not mutate these remote repositories merely to run tests.

Use checked-in/test fixture copies or bounded fetched evidence as appropriate.

---

# 8. Health precedence extension

Preserve M16I health closure and add truth-sync health.

Required precedence:

1. MISSING / MALFORMED / UNADOPTED
2. NEEDS_RECONCILIATION
3. TRUTH_SYNC_DEGRADED / PENDING
4. BLOCKED
5. Git/remote sync attention
6. HEALTHY

A project cannot be HEALTHY while its repository STATE/HANDOFF/EVENTS materialization is pending or failed.

---

# 9. Event/idempotency compatibility

Preserve UCP-R21.

Canonical event upgrade must not regress:

- crash-window duplicate rejection
- recent-tail reconciliation
- sidecar repair
- 4096-ID bounded horizon
- >4096 behavior

Event identity fingerprint must use stable projectKey, not local Registry UUID.

---

# 10. Full direct test matrix

Required focused tests:

1. stable projectKey materialization
2. cross-machine re-registration identity
3. canonical event v1 serialization
4. canonical event v1 parsing
5. legacy event compatibility
6. materialization event canonical core
7. workflow materialization failure pending status
8. audit materialization failure pending status
9. agent materialization failure pending status
10. task-refresh materialization failure pending status
11. restart retry
12. stale-revision retry cannot overwrite newer CURRENT
13. Command Center pending sync
14. Cockpit pending sync
15. pending sync health precedence
16. first HANDOFF materialization preservation
17. second HANDOFF preservation
18. ten-run HANDOFF idempotency
19. governed HANDOFF byte identity
20. ScrubBots-Level-Factory governance
21. exact progress milestone
22. stale progress milestone
23. exact progress cycle
24. stale progress cycle
25. computed percent carries generated scope
26. percent-never-without-scope property
27. all eight actual contract fixtures
28. SB-M02-017 regression
29. fmcg prose regression
30. M16 R82-R85
31. all UCP R13-R26 regressions

---

# 11. Full verification

Run:

- focused control-plane tests
- focused truth-sync/outbox tests
- focused event-schema tests
- focused HANDOFF tests
- focused progress tests
- workflow/audit/agent/task integration tests
- watcher convergence tests
- remote observation tests
- Command Center/Cockpit tests
- all M16 R82-R85
- all prior UCP regressions
- full serialized Rust
- all-targets default
- pty-support
- full frontend Vitest once
- TypeScript typecheck
- production build
- npm audit high
- cargo fmt
- git diff --check
- publisher rollback 9/9
- governed publication
- candidate/stable SHA equality
- PE/startup/shortcut/icon/no-console checks

---

# 12. Whole-M16 adversarial sweep

Before final log re-audit the entire active milestone:

- portable identity
- schemas
- events
- task authority
- resolver
- materializer
- durable sync/outbox
- migrations
- restart recovery
- HANDOFF governance
- progress
- workflow
- audit
- agent sessions
- watcher
- Git/remote observation
- Command Center
- Cockpit
- ACL/native commands
- degraded paths
- tests
- publication

If any new BLOCKER or MAJOR is found, fix it in this same run.

Do not stop and ask for another prompt.

---

# 13. Explicit gates

1. Read GPT.md.
2. Sync H!veAI FF-only.
3. Record starting HEAD.
4. Read M16I strict re-audit.
5. Reproduce R27.
6. Reproduce R28.
7. Reproduce R29.
8. Reproduce R30.
9. Reproduce R31.
10. Split stable projectKey from local Registry UUID.
11. Update materializer identity writes.
12. Update event identity.
13. Introduce canonical hiveai-event/v1 writer.
14. Preserve legacy event reader compatibility.
15. Add canonical event tests.
16. Design truth-sync durable projection/outbox.
17. Add migration.
18. Wire workflow.
19. Wire task refresh.
20. Wire audit.
21. Wire agent success/failure/recovery.
22. Wire adoption/reconcile.
23. Wire remote fetch/FF.
24. Wire watcher/safety retry.
25. Surface pending/degraded sync.
26. Extend health precedence.
27. Replace HANDOFF managed-block algorithm.
28. Honor PROJECT governance.
29. Honor RULES governance.
30. Add repeat preservation tests.
31. Fix progress percent/scope invariant.
32. Add property test.
33. Run actual eight-project contract fixtures.
34. Re-run SB-M02-017.
35. Re-run fmcg transition prose.
36. Preserve UCP-R21.
37. Run focused suites.
38. Run M16 R82-R85.
39. Run all prior UCP regressions.
40. Run full Rust.
41. Run pty.
42. Run frontend once.
43. Run typecheck/build/audit/fmt/diff.
44. Perform whole-M16 adversarial sweep.
45. Fix adjacent BLOCKER/MAJOR same run.
46. Re-run affected suites.
47. Publisher rollback 9/9.
48. Governed publish.
49. Verify stable SHA.
50. Native smoke if feasible.
51. Create immutable builder log.
52. Commit scoped changes.
53. Push normally.
54. Verify local/origin equality.
55. Leave M16 OPEN pending independent strict re-audit + owner acceptance.
56. Do not activate M17.
57. Do not start M21.

---

# 14. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16J_PORTABLE_PROJECT_IDENTITY_CANONICAL_EVENTS_DURABLE_TRUTH_SYNC_CLOSURE_LOG.md`

Record:

- starting HEAD
- implementation commits
- R27-R31 reproduction
- projectKey/localRegistryId split
- eight stable projectKey matrix
- canonical event schema
- legacy event compatibility
- truth-sync outbox architecture
- lifecycle integration matrix
- failure/restart recovery
- HANDOFF managed-block preservation
- governance proof
- progress invariant proof
- eight-project contract fixture results
- full tests
- whole-M16 adversarial sweep
- publication SHA
- final pushed HEAD
- native acceptance pending

End exactly with:

`M16J WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
