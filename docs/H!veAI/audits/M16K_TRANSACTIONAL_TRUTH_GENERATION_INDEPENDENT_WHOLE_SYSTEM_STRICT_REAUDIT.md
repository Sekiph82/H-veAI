# M16K Governance-Exact Portability / Transactional Truth Generation — Independent Whole-System Strict Re-Audit

Date: 2026-09-09  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16K_GOVERNANCE_EXACT_PORTABILITY_TRANSACTIONAL_TRUTH_GENERATION_CLOSURE_LOG.md`  
Implementation commit reviewed: `754715d640d40b1db9eaf8c38376c541f49b992f`  
Published evidence HEAD reviewed: `2572fea88799871943fc0cd3f621172e281c1609`

## Verdict

**FAIL / CHANGES REQUIRED**

### M16K named findings
- UCP-R32: CLOSED
- UCP-R33: CLOSED
- UCP-R34: PARTIALLY CLOSED
- UCP-R35: PARTIALLY CLOSED

### New whole-system findings
- BLOCKER: 1
- MAJOR: 2
- MINOR: 0

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap progress remains `16 / 20 = 80%`.

M16K closes the exact Level Factory governance defect and removes new machine-local Registry identity from portable PROJECT serialization. It also introduces generation-guarded truth synchronization. However, strict inspection found one release-blocking generation bootstrap defect plus two materialization/evidence defects that prevent the new generation model from being considered closed.

---

# Confirmed closures

## UCP-R32 — CLOSED

Production now parses typed `governance.builderMayMutateHandoff` and recognizes the actual Level Factory RULES wording. Governance denial is checked before HANDOFF managed-block rendering, so externally governed HANDOFF can remain byte-identical while STATE still materializes.

## UCP-R33 — CLOSED for new writes

`ProjectDocument.local_registry_id` is backward-readable but `skip_serializing`. New adoption no longer emits the local Registry UUID into portable PROJECT.json.

---

# UCP-R36 — BLOCKER
## Existing projects migrated to generation 0 can enter a permanent PENDING loop and never become CURRENT

Migration v21 adds:

- `truth_generation INTEGER NOT NULL DEFAULT 0`
- `truth_materialized_generation INTEGER NOT NULL DEFAULT 0`

to every existing project row.

For those already-existing projects, the initial state is therefore:

`generation = 0`
`materialized_generation = 0`

Production `snapshot(...)` immediately calls the materializer.

The materializer calls `mark_truth_sync_pending(... generation=0 ...)`.

That SQL sets status to PENDING but cannot make `truth_materialized_generation` less than zero because it uses:

`MAX(truth_generation - 1, 0)`

So the row remains:

`generation = 0`
`materialized_generation = 0`
`status = PENDING`

At completion, `mark_truth_sync_current(...)` requires:

`truth_materialized_generation < truth_generation`

which is:

`0 < 0`

and therefore false.

Worse, `mark_truth_sync_current` ignores the affected-row count and returns `Ok(())` even when the compare-and-set updated zero rows.

Therefore:

1. a migrated pre-v21 project can be put into PENDING merely by opening its snapshot;
2. it cannot transition back to CURRENT at generation 0;
3. `retry_pending_truth_sync` can count the materializer call as successful because it returns `Ok`;
4. the project remains PENDING and is retried again indefinitely.

This directly affects the real portfolio because the eight projects existed before migration v21.

### Required remediation

Define a valid bootstrap generation model.

Acceptable examples:

- migration v22 backfills existing ACTIVE adopted projects to `truth_generation = 1`, `truth_materialized_generation = 0`; or
- define generation 0 as already materialized and make no-op reads avoid PENDING; then only a real dirty transition increments to 1.

Required invariants:

- CURRENT iff authoritative generation equals materialized generation and no current sync error exists;
- materialization of an already-current generation is a no-op, not a forced demotion;
- a successful compare-and-set must verify exactly one row changed when a state transition is claimed;
- retry must count recovery only when the resulting row is actually CURRENT for the authoritative generation.

Add a migration-from-v20 fixture containing all eight pre-existing project rows.

---

# UCP-R37 — MAJOR
## A read-only Project Cockpit / Command Center snapshot still mutates durable truth-sync state

`snapshot(...)` unconditionally executes:

`ProjectTruthMaterializer::materialize(..., "SNAPSHOT_READ")`

even when:

- truth generation is already fully materialized;
- no canonical task/workflow/audit/agent/Git truth changed;
- the user is only opening or refreshing the UI.

`mark_truth_sync_pending(...)` then deliberately demotes:

`truth_materialized_generation = truth_generation - 1`

whenever materialized generation is already current.

The later CURRENT call usually restores it for generation > 0, but the read path creates an unnecessary durable PENDING window and DB writes on every snapshot.

If the app/process exits during this read-triggered window, a previously fully synchronized project becomes dirty solely because it was viewed.

This also creates unnecessary retry-count churn and concurrency pressure.

### Required remediation

Separate read projection from dirty materialization.

Required behavior:

- `snapshot()` is observational;
- if `truth_generation == truth_materialized_generation`, do not mark PENDING and do not rewrite STATE/HANDOFF/EVENTS merely to render the UI;
- materializer should run only for:
  - generation mismatch;
  - explicit reconcile/adopt/repair;
  - real file/domain change requiring convergence;
  - pending/degraded retry.

A snapshot may trigger a bounded retry only if the project is already known dirty/pending/degraded.

Add a test proving 100 repeated snapshots of a CURRENT project produce:

- no generation change;
- no materialized-generation change;
- no retry-count change;
- no new event;
- no STATE/HANDOFF byte change.

---

# UCP-R38 — MAJOR
## The “literal eight-repository fixtures” are stale approximations, not the current target-branch contracts

The M16K log claims:

> `src-tauri/fixtures/m16k/portfolio.json` records the requested literal shapes of the eight repositories.

Independent inspection shows the fixture is stale relative to current target branches.

Examples:

### Bulk-Edit
Fixture still contains:

`"schema":"hiveai-project/v1"`
and string repository identity.

Actual target branch contains:

`"schema":"hiveai-project-control-plane/v1"`
and typed repository object.

### FormuLab / PackLab / ScrubBots / Level Factory
The same stale old-schema pattern exists in the fixture.

### fmcg-erp-system
Fixture repository is recorded as:

`fmcg-erp-system/fmcg-erp-system`

Actual target branch repository identity is:

`Sekiph82/fmcg-erp-system`.

### ScrubBots-Level-Factory
Fixture metadata includes a stale pre-migration PROJECT shape while the actual target branch is already on the final schema.

Therefore the test suite can pass against historical approximations while missing incompatibilities in the current portfolio.

This repeats the evidence-quality problem that caused earlier control-plane iterations.

### Required remediation

Create a reproducible portfolio-contract snapshot process.

At minimum:

1. refresh literal fixtures from the current target-branch files;
2. record the exact target commit SHA each fixture came from;
3. include the exact PROJECT governance/pointers used by production;
4. fail the fixture-verification test if the recorded normalized fields do not match expected current schema/owner/repo/canonical task source.

For M16 closure, independently verify the live target branches again after the implementation run.

Fixture freshness itself must be an auditable gate, not a builder assertion.

---

# M16K concurrency note

Generation guards are materially improved, but the generation-zero defect means UCP-R34/R35 cannot yet be declared completely closed for the installed database population.

Also, `mark_truth_sync_current(...)` currently maps a zero-row compare-and-set to `Ok(())`. The next remediation must distinguish:

- completed current generation;
- superseded stale run;
- impossible/no-op generation;
- true persistence failure.

No caller should count a zero-row stale/no-op update as successful recovery.

---

# Required final remediation

The next run must close UCP-R36 through UCP-R38 together.

Required final invariants:

1. existing v20 databases migrate into a valid generation state;
2. generation zero cannot become permanent PENDING;
3. snapshot/read paths do not dirty synchronized projects;
4. retry success means the authoritative generation is actually CURRENT/materialized;
5. CAS zero-row outcomes are explicit, not silently successful;
6. portfolio fixtures match current target-branch contracts and source SHAs;
7. all eight actual target branches are re-verified;
8. all M16 R82-R85 and UCP R13-R35 regressions remain green.

Only after independent whole-system PASS plus owner native/visual acceptance may M16 close.
