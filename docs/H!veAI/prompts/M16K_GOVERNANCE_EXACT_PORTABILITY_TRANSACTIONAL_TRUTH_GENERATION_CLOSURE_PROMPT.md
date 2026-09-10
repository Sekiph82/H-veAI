# M16K Governance-Exact Portability + Transactional Truth Generation Final Closure Prompt

Date: 2026-09-09
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous whole-system remediation run.

Read and close every finding in:

`H!veAI/docs/H!veAI/audits/M16J_PORTABLE_IDENTITY_CANONICAL_EVENTS_DURABLE_TRUTH_SYNC_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`

Close together:

- UCP-R32 BLOCKER
- UCP-R33 BLOCKER
- UCP-R34 MAJOR
- UCP-R35 MAJOR

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
2. all M16A-J prompts/logs/audits
3. the M16J strict re-audit
4. `control_plane.rs`
5. `workflow.rs`
6. `task_intelligence.rs`
7. `audit_engine.rs`
8. `agent_session_center.rs`
9. watcher
10. Registry/projects
11. Command Center
12. Project Cockpit
13. DB migrations
14. current tests
15. literal target-branch PROJECT/RULES/STATE/HANDOFF shapes for all eight repositories

Builder logs are claims only.

---

# 2. UCP-R32 — enforce real machine-readable HANDOFF governance

Do not infer governance by substring searching serialized JSON.

Create a typed governance parser.

Support the actual installed field used by ScrubBots-Level-Factory:

`governance.builderMayMutateHandoff: false`

Also support an explicit future H!veAI handoff policy only if represented as a typed field.

Required precedence:

1. explicit machine-readable deny in PROJECT governance
2. stricter deny in RULES
3. otherwise allow only under standard H!veAI policy

If either source denies automation, H!veAI must not mutate HANDOFF.

## 2.1 Actual Level Factory regression

Use the literal current project governance fixture:

- projectKey: `scrubbots-level-factory`
- canonicalTaskSource: `tasks.md`
- governance.builderMayMutateHandoff = false

Use the literal RULES wording containing owner-approved authority.

Prove:

- handoff automation denied
- STATE still materializes
- EVENTS still append when permitted
- HANDOFF remains byte-identical

## 2.2 Externally governed malformed marker behavior

If HANDOFF is externally governed:

- do not attempt to render/replace managed blocks
- do not fail STATE materialization because managed markers are malformed
- surface a governance note, not a write error

H!veAI has no authority to repair that HANDOFF automatically.

---

# 3. UCP-R33 — remove local Registry identity from portable PROJECT.json

Portable PROJECT.json must contain only repository-portable identity.

Do not serialize local H!veAI Registry UUID into version-controlled PROJECT.json.

Remove or stop writing:

`localRegistryId`

from newly adopted portable manifests.

If backward-compatible parsing keeps the optional field, treat it as ignored legacy metadata and never rewrite it from current-machine Registry state.

Preferred behavior:

- Registry mapping lives only in SQLite
- portable files remain machine-neutral

## 3.1 Adoption regression

1. fresh repository with canonical task ledger
2. DB A registers as local UUID A
3. adopt
4. PROJECT.json contains stable projectKey
5. PROJECT.json contains no UUID A
6. DB B re-registers same repository as UUID B
7. materialize
8. PROJECT.json identity bytes remain portable

---

# 4. UCP-R34 — transactional truth generation / durable dirty intent

Post-commit best-effort invocation is not enough.

Implement a monotonic authoritative truth generation.

Suggested fields in projects table:

- truth_generation INTEGER NOT NULL DEFAULT 0
- truth_materialized_generation INTEGER NOT NULL DEFAULT 0
- truth_sync_status
- truth_sync_error
- truth_sync_trigger
- timestamps

## 4.1 Domain transaction rule

Every domain mutation that can change project truth must increment/mark truth generation in the **same SQLite transaction** as the authoritative domain mutation.

Examples:

- workflow transition
- workflow override
- audit persistence
- task-intelligence authoritative refresh persistence
- agent lifecycle transition when it changes project truth
- owner-approved project repair state
- other existing truth-changing DB transactions

The transaction commits both:

- domain truth
- durable dirty generation

There must be no crash window where domain truth commits while sync remains falsely CURRENT.

## 4.2 Materializer rule

Materializer:

1. reads current authoritative truth generation G
2. resolves truth
3. writes STATE/HANDOFF/EVENTS
4. on success sets materialized_generation = G only if G is still current
5. CURRENT iff materialized_generation == truth_generation
6. if G advanced during filesystem work, leave sync PENDING for newer generation

## 4.3 Startup/safety recovery

Startup and safety reconciliation must query:

`truth_materialized_generation < truth_generation`

regardless of previous textual status.

Retry latest generation.

This makes crash recovery deterministic.

## 4.4 Crash-window test

Add a failpoint after the domain SQLite transaction commits but before any materializer invocation.

Scenario:

1. project CURRENT at generation 10
2. workflow transition commits generation 11
3. fail/crash before materializer call
4. restart
5. DB shows generation 11 > materialized 10
6. project is PENDING/DEGRADED, never HEALTHY/CURRENT
7. safety retry materializes generation 11
8. materialized_generation becomes 11
9. CURRENT only then

Repeat representative audit/agent lifecycle coverage.

---

# 5. UCP-R35 — monotonic concurrency-safe truth-sync state

All truth-sync transitions must be compare-and-set/generation guarded.

Do not let stale materializer A overwrite newer B.

## Required rules

For generation G:

- mark PENDING only for current authoritative generation G
- mark DEGRADED only if current authoritative generation is still G
- mark CURRENT only if current authoritative generation is still G
- stale completion/failure for older generation records history only, never changes current status

## Deterministic concurrency test

1. start materializer A for generation 20
2. authoritative state advances to generation 21
3. materializer B completes 21 CURRENT
4. A fails afterward
5. current sync must remain generation 21 CURRENT
6. A's failure may be logged historically but cannot replace current status

Also test:

- A completes after B
- retry of stale generation
- simultaneous two retries
- no generation counter rollback

---

# 6. Literal eight-repository contract fixtures

Stop relying only on synthetic key/value approximations for portfolio policy.

Create checked-in test fixtures that preserve the relevant literal shapes of current:

- PROJECT.json governance
- RULES governance
- canonicalTaskSource
- stable projectKey

for all eight tracked repositories.

At minimum verify:

| Project | Stable key | Canonical task source |
|---|---|---|
| AI-Commerce-HQ | ai-commerce-hq | H!veAI/TASKS.md |
| Bulk-Edit | bulk-edit | TASKS.md |
| fmcg-erp-system | fmcg-erp-system | TASKS.md |
| FormuLab | formulab | docs/FORMULAB_V1_TASK_TRACKER.md |
| PackLab | packlab | TASKS.md |
| PackLab-3D | packlab-3d | tasks.md |
| ScrubBots | scrubbots | tasks.md |
| ScrubBots-Level-Factory | scrubbots-level-factory | tasks.md |

Do not mutate remote repositories for tests.

Use literal fixture copies and document their target source SHA.

---

# 7. Truth-sync health

Health remains:

1. MISSING / MALFORMED / UNADOPTED
2. NEEDS_RECONCILIATION
3. TRUTH_SYNC_PENDING / DEGRADED or generation mismatch
4. BLOCKED
5. Git/remote attention
6. HEALTHY

A project with:

`truth_generation > truth_materialized_generation`

must never be HEALTHY, even if textual status accidentally says CURRENT.

Generation relation is authoritative.

---

# 8. HANDOFF managed block

Preserve M16J managed-block behavior for automation-allowed projects.

Required:

- one managed block only
- unlimited repeat idempotency
- unmanaged content byte-preserved
- malformed markers fail closed only when H!veAI is authorized to manage that HANDOFF
- denied governance => zero HANDOFF writes

---

# 9. Canonical events

Preserve M16J event closure.

All new rows remain canonical `hiveai-event/v1`.

Materialization event identity must use:

- portable projectKey
- authoritative generation/revision identity

Do not use local Registry UUID in event ID or projectKey.

Preserve UCP-R21 crash-window/idempotency guarantees.

---

# 10. Lifecycle integration matrix

Ensure the durable generation dirty bit is set transactionally for every truth-changing domain operation.

Provide a table in the builder log:

| Domain operation | Transaction marks generation? | Post-commit materializer? | Restart recovery test? |

Include:

- workflow transition
- workflow override
- task refresh persistence
- audit lifecycle
- agent finish
- agent failure
- agent recovery
- explicit reconcile
- adoption
- remote FF where truth changes
- watcher-driven task truth refresh

For operations where truth cannot change, state explicitly why generation is not incremented.

---

# 11. Full direct tests

Required focused tests:

1. actual Level Factory PROJECT governance deny
2. actual Level Factory RULES deny
3. denied HANDOFF byte identity
4. denied malformed-marker STATE materialization
5. automation-allowed managed block
6. adoption without localRegistryId
7. cross-machine re-registration
8. workflow transactional generation increment
9. workflow crash before materializer recovery
10. audit transactional generation increment
11. audit crash recovery
12. agent transactional generation increment where applicable
13. agent crash recovery
14. task-refresh generation increment
15. startup generation mismatch retry
16. safety generation mismatch retry
17. stale generation DEGRADED cannot overwrite newer CURRENT
18. stale generation CURRENT cannot overwrite newer CURRENT
19. simultaneous retry race
20. health on generation mismatch
21. canonical event portable identity
22. HANDOFF repeat idempotency
23. all eight literal project fixtures
24. SB-M02-017 regression
25. fmcg prose regression
26. progress scope invariant
27. UCP-R21 event idempotency
28. M16 R82-R85
29. all UCP R13-R31 regressions

---

# 12. Full verification

Run:

- focused governance tests
- focused truth-generation tests
- focused concurrency tests
- focused restart tests
- control-plane suite
- workflow/audit/agent/task integration suites
- watcher/safety tests
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

# 13. Whole-M16 adversarial sweep

Before final log inspect:

- exact portfolio governance
- portable identities
- local-only Registry identity
- truth generations
- crash boundaries
- concurrency
- events
- resolver
- materializer
- HANDOFF
- task intelligence
- workflow
- audit
- agent sessions
- watcher
- Git/remote observation
- Command Center
- Cockpit
- DB migrations
- ACL/native commands
- degraded paths
- tests
- publication

If any adjacent BLOCKER or MAJOR exists, fix it in this same run.

Do not stop and request another prompt.

---

# 14. Explicit gates

1. Read GPT.md.
2. Sync H!veAI FF-only.
3. Record starting HEAD.
4. Read M16J strict re-audit.
5. Reproduce R32.
6. Reproduce R33.
7. Reproduce R34.
8. Reproduce R35.
9. Add typed governance parser.
10. Add actual Level Factory fixture.
11. Deny HANDOFF correctly.
12. Skip HANDOFF rendering when denied.
13. Remove new localRegistryId writes.
14. Add adoption portability test.
15. Add truth generation migration.
16. Wire workflow transaction generation.
17. Wire audit transaction generation.
18. Wire task refresh transaction generation.
19. Wire agent truth-changing transaction generation.
20. Wire remaining truth-changing domains.
21. Make materializer generation-aware.
22. Make CURRENT generation-aware.
23. Make DEGRADED generation-aware.
24. Make PENDING generation-aware.
25. Add startup mismatch retry.
26. Add safety mismatch retry.
27. Add crash-before-materializer test.
28. Add stale-run concurrency test.
29. Add simultaneous retry test.
30. Add health generation mismatch rule.
31. Preserve canonical events.
32. Preserve progress invariant.
33. Preserve UCP-R21.
34. Add eight literal portfolio fixtures.
35. Run focused suites.
36. Run M16 R82-R85.
37. Run all prior UCP regressions.
38. Run full Rust.
39. Run pty.
40. Run frontend once.
41. Run typecheck/build/audit/fmt/diff.
42. Whole-M16 adversarial sweep.
43. Fix adjacent BLOCKER/MAJOR same run.
44. Re-run affected suites.
45. Publisher rollback 9/9.
46. Governed publish.
47. Verify stable SHA.
48. Native smoke if feasible.
49. Create immutable builder log.
50. Commit scoped changes.
51. Push normally.
52. Verify local/origin equality.
53. Leave M16 OPEN pending independent strict re-audit + owner acceptance.
54. Do not activate M17.
55. Do not start M21.

---

# 15. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16K_GOVERNANCE_EXACT_PORTABILITY_TRANSACTIONAL_TRUTH_GENERATION_CLOSURE_LOG.md`

Required sections:

- starting HEAD
- implementation commits
- R32-R35 reproduction
- typed governance architecture
- actual Level Factory governance proof
- PROJECT portability proof
- truth-generation architecture
- lifecycle transaction matrix
- crash-window proof
- concurrency monotonicity proof
- restart recovery proof
- eight literal portfolio fixtures
- full tests
- whole-M16 adversarial sweep
- publication SHA
- final pushed HEAD
- native acceptance pending

End exactly with:

`M16K WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
