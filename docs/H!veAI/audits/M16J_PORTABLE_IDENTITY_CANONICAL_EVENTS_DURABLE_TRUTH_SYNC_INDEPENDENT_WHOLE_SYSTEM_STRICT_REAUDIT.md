# M16J Portable Project Identity / Canonical Events / Durable Truth Sync — Independent Whole-System Strict Re-Audit

Date: 2026-09-09  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16J_PORTABLE_PROJECT_IDENTITY_CANONICAL_EVENTS_DURABLE_TRUTH_SYNC_CLOSURE_LOG.md`  
Implementation commit reviewed: `f546033135c454d4da29ee4802142ee15db24915`  
Published evidence HEAD reviewed: `254bca74ec91ad6e8e420e468d442dd48aef9810`

## Verdict

**FAIL / CHANGES REQUIRED**

### Previously named M16J findings
- UCP-R27: PARTIALLY CLOSED
- UCP-R28: CLOSED
- UCP-R29: PARTIALLY CLOSED
- UCP-R30: PARTIALLY CLOSED
- UCP-R31: CLOSED

### New whole-system findings
- BLOCKER: 2
- MAJOR: 2
- MINOR: 0

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap progress remains `16 / 20 = 80%`.

M16J is materially closer to the requested architecture. The stable PROJECT.projectKey is now used by the materializer, canonical `hiveai-event/v1` rows are emitted, progress percent/scope coherence is enforced, and truth-sync failure state is surfaced. The remaining defects are concentrated in portability/governance and crash/concurrency boundaries.

---

# Confirmed closures

## UCP-R28 — CLOSED

New H!veAI event writes now use a canonical `hiveai-event/v1` core containing:

- schema
- eventId
- projectKey
- type
- at
- actor
- taskId
- workflowState
- summary
- commit
- auditId
- sessionId

Legacy rows remain readable.

## UCP-R31 — CLOSED

The resolver now guarantees that a returned progress percentage has an exact scope. Deterministic computed milestone progress sets `MILESTONE:<id>` together with the percentage, and explicit stale scope does not silently survive.

---

# UCP-R32 — BLOCKER
## Actual ScrubBots-Level-Factory HANDOFF governance is not recognized by the production policy parser

The M16J log claims:

> PROJECT machine-readable governance and RULES governance are both consulted; owner-governed repositories remain byte-identical.

The actual target repository contains:

`PROJECT.json.governance.builderMayMutateHandoff = false`

and RULES text stating that Codex may not rewrite HANDOFF unless an owner-approved prompt changes governance.

Production `handoff_automation_allowed(...)` does **not** parse that actual field.

It only searches serialized PROJECT text for:

- `handoffAutomationAllowed`
- `handoffPolicy`

and RULES text for:

- `manual`
- `owner only`
- `owner-only`
- `owner approval`

The real Level Factory rule uses `owner-approved`, and the real machine field is `builderMayMutateHandoff`.

Therefore the real repository can evaluate as automation allowed.

The direct M16J governance test is synthetic:

`{"handoffAutomationAllowed": false}`

It does not reproduce the actual Level Factory PROJECT contract.

### Impact

H!veAI can automatically rewrite `.hiveai/HANDOFF.md` in the one project whose explicit governance says builder automation may not mutate HANDOFF.

That is a release-blocking authority violation.

### Required remediation

Parse typed machine governance, not substring-search arbitrary serialized JSON.

At minimum support:

- `governance.builderMayMutateHandoff: false`
- explicit H!veAI handoff automation policy if introduced later

Then apply RULES as an additional stricter layer.

Use actual repository governance fixtures from ScrubBots-Level-Factory.

Required invariant:

> if either PROJECT governance or RULES forbids automated HANDOFF mutation, H!veAI must never write HANDOFF.

Also, when HANDOFF is externally governed, malformed H!veAI managed markers must not block STATE materialization because H!veAI has no authority to edit that file.

---

# UCP-R33 — BLOCKER
## New adoption still writes a machine-local Registry UUID into the portable repository contract

M16J correctly separates portable `projectKey` from local Registry identity during materialization.

But `adopt(...)` still creates PROJECT.json with:

`local_registry_id: Some(project.id.clone())`

The Registry project ID is machine-local.

This means a newly adopted repository contains a machine-specific UUID in a file intended to be version-controlled and shared across:

- GitHub
- another laptop
- Codex
- Claude
- ChatGPT
- H!veAI re-registration

The eight already-migrated repositories do not require this field for portable identity.

### Impact

Machine A can commit its local UUID into PROJECT.json. Machine B then sees Machine A's registry identity, defeating the portability contract and making repository bytes depend on which computer adopted the project first.

This is exactly what UCP-R27 was meant to eliminate.

### Required remediation

Do not persist local Registry UUID in the portable version-controlled PROJECT.json.

Preferred:

- remove `localRegistryId` from newly written portable PROJECT.json;
- keep local mapping only in SQLite Registry.

If a local sidecar is ever needed, it must be explicitly untracked/local-only and not part of the shared project contract.

Add adoption/re-registration regression:

1. adopt on DB A;
2. inspect PROJECT.json;
3. assert no local Registry UUID is serialized;
4. initialize DB B with a different Registry ID;
5. same repository files remain portable and unchanged.

---

# UCP-R34 — MAJOR
## Durable truth-sync intent is not armed atomically with the domain transition

M16J improves post-commit failure handling, but the durable sync status begins inside `materialize_project_truth(...)`.

Lifecycle code still follows this pattern:

1. commit domain transaction;
2. call `materialize_best_effort(...)`;
3. `materialize_project_truth` then marks truth sync PENDING.

There is a crash window between steps 1 and 2.

If the process exits after the authoritative workflow/audit/agent/task transition commits but before materialization starts:

- the domain transition is durable;
- truth_sync may still say CURRENT for the previous revision;
- no new PENDING revision has been recorded.

The M16J restart test does not exercise this window. It manually removes STATE and calls `materialize_project_truth`, which means PENDING/DEGRADED tracking has already begun.

### Required remediation

Use a durable post-commit outbox/intention row tied to the domain transaction or an equivalent revision source that startup can deterministically detect.

Acceptable architecture:

- within the same SQLite transaction as the domain change, mark project truth dirty / increment a monotonic truth generation;
- after commit, materializer consumes that generation;
- CURRENT is only valid when materialized_generation == authoritative_generation.

Then a crash before filesystem work is naturally recoverable.

Required failpoint:

1. commit workflow transition;
2. crash/fail before materializer invocation;
3. restart;
4. project must be visibly PENDING/DEGRADED or generation-mismatched;
5. safety retry materializes latest truth;
6. CURRENT only after exact generation is durable in files.

Repeat representative audit/agent path coverage.

---

# UCP-R35 — MAJOR
## Truth-sync revision writes are not fully monotonic under concurrent materialization

M16J correctly guards `mark_truth_sync_current(...)` with the expected revision.

But:

- `mark_truth_sync_pending(...)` unconditionally replaces `truth_sync_revision`;
- `mark_truth_sync_degraded(...)` also unconditionally replaces it.

Therefore two overlapping materializations can race:

1. run A starts revision A;
2. run B starts newer revision B;
3. B completes and marks B CURRENT;
4. A fails later;
5. A can overwrite status/revision back to A DEGRADED.

The system then reports an old failure as the current synchronization state.

This contradicts the M16J claim that retry/status handling is revision-aware and that an older revision cannot overwrite newer truth.

### Required remediation

Make all state transitions monotonic.

Use a generation/revision compare-and-set policy:

- PENDING may only advance to the current authoritative generation;
- DEGRADED may only update the same still-current generation;
- CURRENT may only complete the same still-current generation;
- stale runs may record history/diagnostics but cannot replace current project sync status.

Add deterministic concurrency/failpoint tests proving stale A cannot overwrite newer B CURRENT.

---

# Whole-M16 audit notes

The builder log's automated test counts are useful regression claims, but the strict audit found a recurring evidence gap: synthetic fixtures do not always reproduce the exact contracts already installed in the eight real repositories.

The clearest example is HANDOFF governance:

- actual repository field: `builderMayMutateHandoff`
- test field: `handoffAutomationAllowed`

The next run must use literal checked-in fixtures derived from all eight target PROJECT/RULES shapes for cross-project policy tests.

---

# Required closure strategy

Do not fix R32-R35 in separate loops.

The next remediation must close all four in one continuous run and perform a fresh whole-M16 adversarial sweep.

Required final invariants:

1. actual Level Factory governance is enforced exactly;
2. externally governed HANDOFF is never rewritten by H!veAI;
3. portable PROJECT.json never contains machine-local Registry identity;
4. authoritative domain transitions durably mark truth dirty in the same DB transaction/generation boundary;
5. crash before filesystem materialization is restart-safe;
6. truth-sync generations are monotonic under concurrency;
7. stale materializer failures cannot overwrite newer CURRENT truth;
8. all eight literal repository contract fixtures are exercised;
9. M16 R82-R85 and UCP R13-R31 regressions remain green.

Only after independent whole-system PASS plus owner native/visual acceptance may M16 close and M17 activate.
