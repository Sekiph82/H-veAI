# M16L Truth-Generation Bootstrap + Read-Purity + Portfolio Fixture Freshness Final Closure Prompt

Date: 2026-09-09
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous whole-system remediation run.

Read and close every finding in:

`H!veAI/docs/H!veAI/audits/M16K_TRANSACTIONAL_TRUTH_GENERATION_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`

Close together:

- UCP-R36 BLOCKER
- UCP-R37 MAJOR
- UCP-R38 MAJOR

Do not stop after any individual finding.

Preserve all previously closed M16 audit-engine and UCP findings.

After named fixes, perform a fresh whole-M16 adversarial sweep. If any adjacent BLOCKER or MAJOR is found, fix it in the same run and add direct regression coverage.

Do not activate M17.
Do not start M21.
M16 remains OPEN until independent whole-system strict re-audit + owner native/visual acceptance.

---

# 1. Required reading

Read completely before edits:

1. `H!veAI/GPT.md`
2. all M16A-K prompts/logs/audits
3. `M16K_TRANSACTIONAL_TRUTH_GENERATION_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`
4. `control_plane.rs`
5. DB migrations
6. watcher
7. Command Center
8. Project Cockpit
9. workflow/audit/agent/task lifecycle integration
10. all generation/concurrency tests
11. current target-branch PROJECT/RULES/STATE/HANDOFF contracts for all eight repositories

Builder logs are claims only.

---

# 2. UCP-R36 — fix generation bootstrap for existing databases

Migration v21 currently gives existing project rows:

- truth_generation = 0
- truth_materialized_generation = 0

while the materializer can mark generation 0 PENDING but cannot complete CURRENT because completion requires materialized_generation < truth_generation.

Fix this explicitly.

## 2.1 Migration strategy

Add migration v22 or an equivalent safe idempotent repair.

Preferred model:

For existing ACTIVE adopted projects that predate generation tracking:

- truth_generation = 1
- truth_materialized_generation = 0
- truth_sync_status = PENDING
- trigger = GENERATION_BOOTSTRAP

Then safety reconciliation materializes generation 1 exactly once.

For projects that are not adopted/unavailable, preserve truthful status and do not fabricate CURRENT.

Alternative model is allowed only if generation 0 is treated as already-materialized and read paths never mark it PENDING.

Whichever model is chosen, document it and test v20 → latest migration directly.

## 2.2 CURRENT invariant

Define:

`CURRENT <=> truth_materialized_generation == truth_generation AND no active materialization error/pending newer generation`

Do not infer CURRENT only from the text column.

## 2.3 CAS result semantics

`mark_truth_sync_current(...)` must inspect the affected-row count.

Return a typed result such as:

- APPLIED
- ALREADY_CURRENT
- SUPERSEDED
- INVALID_GENERATION

Do not return success merely because the SQL statement executed.

Likewise DEGRADED/PENDING transitions should make zero-row stale/superseded results explicit.

## 2.4 Retry accounting

`retry_pending_truth_sync` counts a project as recovered only if, after the attempt:

- authoritative generation == materialized generation
- status is CURRENT

A superseded/no-op materializer is not counted as recovery.

## 2.5 Required tests

1. v20 database with eight existing projects → migrate to latest
2. no project is trapped generation 0 PENDING
3. safety retry completes bootstrap generation
4. retry count stops after CURRENT
5. zero-row CURRENT CAS is not reported as applied
6. stale generation is classified as superseded
7. restart during bootstrap recovers

---

# 3. UCP-R37 — make snapshot/read paths observational

Do not dirty a synchronized project merely because the user opens Command Center or Project Cockpit.

## 3.1 Snapshot behavior

Before materialization:

- read truth sync generation state
- if generation == materialized_generation and status CURRENT:
  - do not mark PENDING
  - do not write STATE
  - do not write HANDOFF
  - do not append EVENTS
  - do not increment retry count

Then resolve/project the current truth for UI.

If the project is PENDING/DEGRADED or generation mismatch exists, a bounded recovery attempt is allowed.

## 3.2 Materializer behavior

Do not forcibly decrement:

`truth_materialized_generation = truth_generation - 1`

for an already-current generation just to begin a read-triggered materialization.

PENDING should represent a real dirty generation or an explicit recovery attempt, not a UI read.

## 3.3 Read purity tests

For a CURRENT synchronized project:

- call snapshot 100 times
- Command Center summary 100 times
- Project Cockpit snapshot 100 times

Assert:

- truth_generation unchanged
- truth_materialized_generation unchanged
- truth_sync_retry_count unchanged
- truth_sync_status remains CURRENT
- EVENTS byte-identical
- STATE byte-identical
- HANDOFF byte-identical

Then make a real domain mutation and prove the next recovery/materialization is triggered correctly.

---

# 4. UCP-R38 — replace stale portfolio fixtures with reproducible current target-branch fixtures

The current M16K portfolio fixture contains stale pre-migration shapes and incorrect repository owners.

Replace it.

## 4.1 Current target sources

Refresh the fixture from the actual current target branches:

- AI-Commerce-HQ / H!veAI
- Bulk-Edit / main
- fmcg-erp-system / main
- FormuLab / feature/laboratory-stability
- PackLab / main
- PackLab-3D / main
- ScrubBots / main
- ScrubBots-Level-Factory / main

Record exact target SHA for each.

## 4.2 Fixture contents

For each project store the relevant literal current fields:

- schema
- projectKey
- displayName
- repository owner/name/branch
- canonicalTaskSource
- rules
- state
- handoff
- events
- eventSources
- governance when present
- exact relevant RULES authority lines
- source SHA

Do not preserve historical old-schema fields in a fixture labeled “current”.

## 4.3 Verification

Create a test that parses every fixture with the production `ProjectDocument` parser and verifies:

- schema = hiveai-project-control-plane/v1
- owner = Sekiph82 for the current portfolio
- expected repository name
- expected canonical task path
- expected governance for Level Factory

Also add a fixture freshness manifest/tool or documented refresh command so later audits can distinguish stale fixture evidence from current target-branch evidence.

## 4.4 Final live target verification

Before the builder log, independently fetch/inspect the actual eight target PROJECT.json files again and record:

| repository | branch | target SHA | schema | owner/name | canonical task source |

No row may contradict the checked-in fixture.

---

# 5. Preserve M16K governance and portability fixes

Do not regress:

- typed builderMayMutateHandoff
- Level Factory owner-approved RULES deny
- no local Registry UUID in new PROJECT writes
- canonical hiveai-event/v1
- progress percent/scope invariant
- generation monotonicity for generation > 0
- stale-run protection
- HANDOFF managed-block preservation
- remote observation behavior

---

# 6. Generation-state machine specification

Document one compact state machine.

At minimum:

### CURRENT
- generation == materialized_generation
- no current sync error
- no dirty authoritative revision

### PENDING
- generation > materialized_generation
- or explicit retry in progress for the authoritative generation

### DEGRADED
- generation > materialized_generation
- latest attempt for the authoritative generation failed

### SUPERSEDED RUN
- materializer generation < current authoritative generation
- cannot alter current sync status

Read-only UI access must never create PENDING from CURRENT.

---

# 7. Lifecycle integration re-check

Re-run transactional generation tests for:

- workflow transition
- workflow override
- task refresh
- audit lifecycle
- agent finish
- agent failure/recovery
- adoption
- explicit reconcile
- remote fast-forward
- watcher-driven refresh

Confirm each real truth-changing transaction increments exactly once.

Confirm non-truth-changing remote observation does not increment.

---

# 8. Full direct test matrix

Required focused tests:

1. v20 → v22 eight-project generation bootstrap
2. generation-zero no permanent PENDING
3. CURRENT CAS affected-row classification
4. superseded CAS classification
5. retry recovery count correctness
6. 100 snapshot read-purity
7. 100 Command Center read-purity
8. 100 Cockpit read-purity
9. real mutation after pure reads triggers generation
10. fixture parser on all eight current PROJECT shapes
11. fixture owner/name validation
12. fixture source-SHA table
13. Level Factory governance fixture current shape
14. stable projectKey
15. canonical events
16. HANDOFF governance
17. progress invariant
18. generation concurrency
19. crash recovery
20. remote observation
21. SB-M02-017 regression
22. fmcg prose regression
23. M16 R82-R85
24. all UCP R13-R35 regressions

---

# 9. Full verification

Run:

- focused generation bootstrap tests
- focused read-purity tests
- focused portfolio-fixture tests
- control-plane suite
- workflow/audit/agent/task integration
- watcher/safety
- Command Center/Cockpit
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

# 10. Whole-M16 adversarial sweep

Before final log inspect:

- migration bootstrap
- generation state machine
- CAS affected-row semantics
- retries
- read purity
- concurrency
- crash boundaries
- portfolio fixture freshness
- target branch verification
- governance
- identities
- events
- resolver/materializer
- task intelligence
- workflow
- audit
- agents
- watcher
- remote observation
- Command Center
- Cockpit
- migrations
- ACL/native commands
- degraded paths
- tests
- publication

If any adjacent BLOCKER or MAJOR exists, fix it in this same run.

Do not stop and request another prompt.

---

# 11. Explicit gates

1. Read GPT.md.
2. Sync H!veAI FF-only.
3. Record starting HEAD.
4. Read M16K strict re-audit.
5. Reproduce R36.
6. Reproduce R37.
7. Reproduce R38.
8. Add migration v22/bootstrap repair.
9. Define generation state machine.
10. Make CURRENT invariant generation-based.
11. Add typed CAS outcome.
12. Fix retry recovery counting.
13. Remove read-triggered demotion.
14. Make snapshot observational.
15. Make Command Center read observational.
16. Make Cockpit read observational.
17. Add 100-read purity tests.
18. Refresh all eight literal fixtures.
19. Record current target SHAs.
20. Verify owner/name/schema/task-source.
21. Verify Level Factory governance.
22. Re-run transactional generation matrix.
23. Re-run concurrency.
24. Re-run crash recovery.
25. Re-run canonical event tests.
26. Re-run HANDOFF governance.
27. Re-run progress invariant.
28. Re-run SB-M02-017.
29. Re-run fmcg prose.
30. Run focused suites.
31. Run M16 R82-R85.
32. Run all UCP regressions.
33. Run full Rust.
34. Run pty.
35. Run frontend once.
36. Run typecheck/build/audit/fmt/diff.
37. Whole-M16 adversarial sweep.
38. Fix adjacent BLOCKER/MAJOR same run.
39. Re-run affected suites.
40. Publisher rollback 9/9.
41. Governed publish.
42. Verify stable SHA.
43. Native smoke if feasible.
44. Create immutable builder log.
45. Commit scoped changes.
46. Push normally.
47. Verify local/origin equality.
48. Leave M16 OPEN pending independent strict re-audit + owner acceptance.
49. Do not activate M17.
50. Do not start M21.

---

# 12. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16L_TRUTH_GENERATION_BOOTSTRAP_READ_PURITY_PORTFOLIO_FIXTURE_FRESHNESS_CLOSURE_LOG.md`

Required sections:

- starting HEAD
- implementation commits
- R36-R38 reproduction
- v22/bootstrap migration
- generation state machine
- CAS semantics
- read-purity proof
- 100-read tests
- fixture refresh method
- eight current target SHAs
- live target verification matrix
- transactional generation regression matrix
- full tests
- whole-M16 adversarial sweep
- publication SHA
- final pushed HEAD
- native acceptance pending

End exactly with:

`M16L WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
