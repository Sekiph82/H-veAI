# M16M Physical Adoption Convergence + True Read Purity + Portfolio Provenance Final Closure Prompt

Date: 2026-09-09
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous whole-system remediation run.

Read and close every finding in:

`H!veAI/docs/H!veAI/audits/M16L_TRUTH_GENERATION_READ_PURITY_FIXTURE_FRESHNESS_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`

Close together:

- UCP-R39 BLOCKER
- UCP-R40 MAJOR
- UCP-R41 MAJOR
- UCP-R42 MINOR

Do not stop after any individual finding.

Preserve all previously closed M16 audit-engine and UCP findings.

After named fixes, perform one fresh whole-M16 adversarial sweep. If any adjacent BLOCKER or MAJOR is found, fix it in this same run and add direct regression coverage.

Do not activate M17.
Do not start M21.
M16 remains OPEN until independent whole-system strict re-audit + owner native/visual acceptance.

---

# 1. Required reading

Read completely before edits:

1. `H!veAI/GPT.md`
2. all M16A-L prompts/logs/audits
3. `M16L_TRUTH_GENERATION_READ_PURITY_FIXTURE_FRESHNESS_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`
4. `control_plane.rs`
5. DB migrations
6. project Registry / repository metadata
7. watcher startup/reconcile
8. Command Center
9. Project Cockpit
10. task intelligence/workflow/audit/agent integration
11. current portfolio fixtures
12. current target-branch PROJECT/RULES/STATE/HANDOFF contracts for all eight projects

Builder logs are claims only.

---

# 2. UCP-R39 — make physical adoption authoritative and DB metadata convergent

Current migration/bootstrap uses the DB field:

`control_plane_status='ADOPTED'`

but physical PROJECT.json is the real contract authority.

The current `adopt(...)` path creates valid control-plane files and marks truth dirty but does not persist control_plane_status/schema transactionally.

Fix this.

## 2.1 Physical adoption probe

Implement one bounded verifier:

`probe_physical_control_plane(project_root) -> PhysicalControlPlaneStatus`

At minimum:

- MISSING
- MALFORMED
- UNSUPPORTED_SCHEMA
- ADOPTED

For ADOPTED verify:

- PROJECT.json parse
- schema
- stable projectKey
- repository identity
- canonical task source path safety
- required pointer paths

Do not infer ADOPTED from DB metadata alone.

## 2.2 DB convergence

On startup/reconcile for each ACTIVE project:

- probe physical control plane
- update DB control_plane_status/schema/revision projection only from verified physical result
- never let stale DB ADOPTED override malformed/missing files
- never let stale DB UNADOPTED suppress a valid physical adopted project

This convergence may be a startup maintenance transaction and need not be a UI read side effect.

## 2.3 Generation bootstrap after convergence

For a physically ADOPTED project with no generation history:

- if generation=0/materialized=0 and no prior generation bootstrap exists:
  - set authoritative generation to 1
  - materialized generation 0
  - PENDING / GENERATION_BOOTSTRAP
- then retry/materialize normally

Do not rely on the pre-existing DB adoption flag.

## 2.4 New adoption transaction

When `adopt(...)` succeeds in creating/validating the portable files:

within the authoritative DB transaction:

- persist control_plane_status=ADOPTED
- persist control_plane_schema
- mark truth dirty/increment generation
- commit

Then materialize post-commit.

If the DB transaction fails, do not claim fully adopted/synchronized success.

## 2.5 Exact stale-metadata regressions

Add:

A. physical valid ADOPTED + DB UNADOPTED + generation 0  
→ startup/reconcile self-heals DB, bootstraps generation, materializes CURRENT.

B. physical malformed + DB ADOPTED  
→ DB converges to MALFORMED/attention state, no generation truth fabrication.

C. physical missing + DB ADOPTED  
→ DB converges to missing/unadopted/degraded state.

D. fresh adopt  
→ DB ADOPTED + generation dirty intent committed together.

E. restart after convergence  
→ no repeated bootstrap.

---

# 3. UCP-R40 — separate target branch identity from contract-content provenance

The fixture currently overloads `sourceSha`.

Replace it with explicit fields.

Required per project:

- `targetBranchHeadSha`
- `projectBlobSha` or equivalent PROJECT.json content/blob identity
- optional `rulesBlobSha`
- optional `stateBlobSha`
- fixtureCapturedAt / verification label if useful

## 3.1 Semantics

`targetBranchHeadSha`:
- exact fetched target branch HEAD at verification time.

`projectBlobSha`:
- exact Git blob SHA of the PROJECT.json content copied into the fixture.

Do not call a historical starting commit the “current target SHA” merely because the contract file content did not change.

## 3.2 AI-Commerce-HQ self-reference

Because this repository changes during the implementation run:

1. implement code;
2. create implementation commit;
3. fetch/verify current H!veAI target HEAD after that commit is pushed;
4. record that target HEAD in evidence;
5. if the later immutable log-only commit changes HEAD again but not contract files, state this explicitly and keep separate:
   - implementation verification HEAD
   - final log HEAD
   - unchanged PROJECT blob SHA

Do not make circular fixture edits that require rewriting the fixture after every log-only commit.

## 3.3 Eight target verification

For all eight targets record:

- repo
- branch
- verification HEAD
- PROJECT blob SHA
- schema
- owner/name
- canonicalTaskSource
- governance summary

A direct test must validate the checked-in contract fields. Live GitHub freshness itself remains a builder/audit evidence gate, not a unit-test fiction.

---

# 4. UCP-R41 — prove real Command Center and Cockpit read purity

Use the same real adopted CURRENT project fixture for all read surfaces.

## 4.1 Setup

- create real Git temp project
- canonical task source
- register
- adopt
- materialize CURRENT
- ensure Git snapshot/remote state is valid enough for Command Center/Cockpit

Record before state:

- truth_generation
- truth_materialized_generation
- truth_sync_status
- retry_count
- project metadata fields
- latest git snapshot count/IDs
- task event counts if relevant
- STATE bytes
- HANDOFF bytes
- EVENTS bytes
- EVENT_INDEX bytes

## 4.2 100 Command Center reads

Call the actual production Command Center snapshot path 100 times against the database containing that adopted project.

Assert:

- no truth-sync mutation
- no metadata persistence
- no Git snapshot persistence
- no task refresh persistence
- no event append
- no portable-file changes

## 4.3 100 Project Cockpit reads

Call the actual production Project Cockpit snapshot 100 times for the exact adopted project.

Assert the same purity invariants.

Do not use:

- empty database
- missing project
- error-only path

as evidence for adopted-project read purity.

## 4.4 Real mutation after reads

After purity proof:

- perform one genuine truth-changing transaction
- assert generation increments once
- recovery/materialization occurs
- new CURRENT state persists
- subsequent 100 reads are pure again

---

# 5. UCP-R42 — fix canonical tracking current-truth summary

Update current/prospective status documentation without self-closing M16.

Required:

- TASKS Current truth points to M16L/M16M as latest implementation/remediation chain
- preserve M16A-L history
- CODEX_ROADMAP latest M16 status is consistent
- README/doc README status mirrors latest prospective state
- no PASS/CLOSED wording for M16
- progress remains 16/20 = 80%
- M17 blocked
- M21 not started

Do not rewrite immutable audits/logs.

---

# 6. Generation state machine re-check

Re-verify:

CURRENT:
- materialized generation == authoritative generation
- no current error

PENDING:
- authoritative generation ahead
- or explicit recovery attempt for current dirty generation

DEGRADED:
- current authoritative generation failed materialization

SUPERSEDED:
- stale attempt cannot alter current state

Physical-adoption convergence must happen before generation-bootstrap eligibility is decided.

---

# 7. Lifecycle integration re-check

Re-run truth-generation and durable convergence regressions for:

- workflow transition
- workflow override
- task refresh
- audit lifecycle
- agent finish
- agent fail/recovery
- adopt
- explicit reconcile
- remote ff
- watcher-driven task refresh

Confirm each truth-changing transaction increments exactly once.

---

# 8. Portfolio live verification

Before final log, fetch the actual target branches again:

- AI-Commerce-HQ / H!veAI
- Bulk-Edit / main
- fmcg-erp-system / main
- FormuLab / feature/laboratory-stability
- PackLab / main
- PackLab-3D / main
- ScrubBots / main
- ScrubBots-Level-Factory / main

Record current verification HEAD + PROJECT blob SHA.

Verify all remain:

`hiveai-project-control-plane/v1`

and preserve expected task-source paths/governance.

---

# 9. Full direct test matrix

Required focused tests:

1. physical adopted + stale DB UNADOPTED self-heal
2. physical malformed + stale DB ADOPTED fail-closed
3. physical missing + stale DB ADOPTED fail-closed
4. fresh adopt persists DB adoption + generation transactionally
5. no repeated bootstrap after restart
6. generation-zero repair remains closed
7. CAS typed outcomes
8. 100 real Command Center reads pure
9. 100 real Cockpit reads pure
10. 100 control-plane reads pure
11. no git-snapshot persistence on reads
12. no task/event persistence on reads
13. real mutation after pure reads increments exactly once
14. second pure-read phase remains stable
15. fixture targetBranchHeadSha semantics
16. fixture PROJECT blob SHA semantics
17. all eight current contract shapes
18. Level Factory governance
19. stable projectKey
20. canonical events
21. progress invariant
22. remote observation
23. generation concurrency/restart
24. SB-M02-017
25. fmcg prose
26. M16 R82-R85
27. all UCP R13-R38 regressions

---

# 10. Full verification

Run:

- focused adoption-convergence tests
- focused generation/bootstrap tests
- focused real read-purity tests
- focused portfolio-provenance tests
- control-plane suite
- Registry
- watcher/safety
- workflow/audit/agent/task integration
- Command Center/Cockpit
- Git engine regression
- M16 R82-R85
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

# 11. Whole-M16 adversarial sweep

Inspect:

- physical-vs-DB adoption authority
- bootstrap generation
- read purity
- CAS/retry semantics
- portfolio evidence provenance
- current tracking truth
- governance
- project identities
- events
- progress
- resolver/materializer
- workflow/audit/agents/tasks
- watcher
- Git/remote observation
- Command Center
- Cockpit
- migrations
- ACL/native commands
- degraded paths
- tests
- publication

If any additional BLOCKER or MAJOR exists, fix it in this same run.

Do not stop for another prompt.

---

# 12. Explicit gates

1. Read GPT.md.
2. Sync H!veAI FF-only.
3. Record starting HEAD.
4. Read M16L strict re-audit.
5. Reproduce R39.
6. Reproduce R40.
7. Reproduce R41.
8. Reproduce R42.
9. Add physical control-plane probe.
10. Add DB adoption convergence.
11. Bootstrap generation after physical convergence.
12. Fix fresh adoption DB transaction.
13. Add stale DB adoption fixtures.
14. Separate targetBranchHeadSha from blob identity.
15. Refresh eight portfolio evidence entries.
16. Build real adopted Command Center purity fixture.
17. Build real adopted Cockpit purity fixture.
18. Assert no DB/file/event/Git persistence on reads.
19. Prove real mutation after pure reads.
20. Update canonical current-truth docs prospectively.
21. Re-run generation state machine.
22. Re-run lifecycle generation matrix.
23. Re-run all eight live contract verification.
24. Run focused suites.
25. Run M16 R82-R85.
26. Run all UCP regressions.
27. Run full Rust.
28. Run pty.
29. Run frontend once.
30. Run typecheck/build/audit/fmt/diff.
31. Whole-M16 adversarial sweep.
32. Fix adjacent BLOCKER/MAJOR same run.
33. Re-run affected suites.
34. Publisher rollback 9/9.
35. Governed publish.
36. Verify stable SHA.
37. Native smoke if feasible.
38. Create immutable builder log.
39. Commit scoped changes.
40. Push normally.
41. Verify local/origin equality.
42. Leave M16 OPEN pending independent strict re-audit + owner acceptance.
43. Do not activate M17.
44. Do not start M21.

---

# 13. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16M_PHYSICAL_ADOPTION_CONVERGENCE_TRUE_READ_PURITY_PORTFOLIO_PROVENANCE_CLOSURE_LOG.md`

Required sections:

- starting HEAD
- implementation commits
- R39-R42 reproduction
- physical adoption authority
- DB convergence
- bootstrap-after-convergence proof
- fresh adoption transaction proof
- 100-read Command Center purity
- 100-read Cockpit purity
- no-persistence proof
- portfolio branch/blob provenance matrix
- latest tracking truth
- lifecycle generation regression matrix
- full tests
- whole-M16 adversarial sweep
- publication SHA
- implementation verification HEAD
- final log HEAD
- native acceptance pending

End exactly with:

`M16M WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
