# M16B Degraded Re-audit Persistence Independent Strict Re-Audit

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
M16B implementation commit: `404dca9101f2d646322bdaa176d1435b7fc266f3`
Evidence publication HEAD: `6f38c6d88796918ca19e693a4e98b32e53595aee`

## Verdict

**FAIL / CHANGES REQUIRED**

- BLOCKER: 0
- MAJOR: 1
- MINOR: 0
- NOTE: 0

### Prior findings

- M16-R59: CLOSED
- M16-R60: CLOSED
- M16-R61: CLOSED
- M16-R62: CLOSED technically

### New finding

- **M16-R63 MAJOR — persisted evidence IDs no longer match finding/coverage evidence references after re-audit-safe evidence scoping**

M16 remains OPEN.
M17 MUST NOT activate.
M21 MUST NOT start.
Roadmap completed progress remains `16 / 20 = 80%`.

---

# M16-R62 closure — PASS

M16B correctly gates prior-finding disposition processing on:

`state == COMPLETED && evaluation.model_status == "AVAILABLE"`

Therefore:

- fresh UNAVAILABLE re-audits persist;
- fresh MALFORMED re-audits persist;
- prior audit linkage is preserved;
- prior OPEN findings remain untouched;
- no degraded synthetic disposition rows are created;
- STALE remains higher priority;
- AVAILABLE explicit disposition behavior remains intact.

The new production/domain tests directly exercise these paths.

R62 is CLOSED.

---

# M16-R63 — MAJOR
## Persisted evidence IDs are rewritten, but finding/coverage evidence references are not

### Required contract

M16 requires persisted findings and requirement coverage to link to their supporting persisted evidence.

The audit result contracts use `evidenceRefs` specifically so an historical audit can resolve a finding/coverage row back to exact persisted evidence.

### M16B production change

To avoid primary-key collisions when the same logical evidence IDs recur across immutable re-audits, M16B changed evidence persistence from:

`item.id`

to:

`format!("{audit_id}:{}", item.id)`

This solves the database PK collision problem.

However, persisted findings and requirement coverage continue storing the **original logical evidence IDs** from model/input contracts.

Example:

Logical evidence ID:

`TEST_RUN:proof`

Persisted `audit_evidence.id`:

`<audit-uuid>:TEST_RUN:proof`

Persisted finding `evidence_refs_json`:

`["TEST_RUN:proof"]`

After `get(...)`, the returned `AuditRun` therefore contains:

- evidence rows whose IDs are audit-prefixed;
- findings/coverage whose refs are not audit-prefixed.

The references cannot resolve against `audit.evidence[].id`.

### Why this is MAJOR

This breaks the durable evidence-link contract that M16 is built around.

Historical consumers cannot reliably answer:

- which persisted source/test/Git evidence supports this finding?
- which evidence supports this requirement coverage row?
- which exact evidence was used for an explicit prior-finding closure?

It also makes technical evidence navigation and future Audit Center provenance features fragile because the durable IDs and durable refs live in two different namespaces.

The builder log itself notes that logical input evidence IDs are stable across runs, but the implementation does not persist that logical ID separately.

### Required remediation

Use one explicit durable two-ID contract.

Recommended:

1. Keep `audit_evidence.id` globally unique, audit-scoped PK.
2. Add `logical_evidence_id TEXT NOT NULL` (or equivalent) via additive migration.
3. Persist:
   - global row ID = `<audit_id>:<logical_id>`
   - logical ID = original `item.id`
4. Choose and document one external/reference contract:
   - either findings/coverage/disposition refs use logical IDs and resolver matches within the same audit by `logical_evidence_id`;
   - or rewrite refs to global persisted row IDs before persistence.
5. Returned `AuditRun` must expose enough information to resolve every valid evidence ref deterministically.
6. Add invariant validation before commit:
   - every persisted finding evidence ref resolves;
   - every coverage evidence ref resolves;
   - every disposition evidence ref resolves.
7. Historical re-audits may reuse the same logical ID without PK collisions.
8. Do not allow cross-audit evidence resolution accidentally.

### Required tests

Add direct tests for:

- two immutable audits with the same logical evidence ID both persist;
- each audit's finding refs resolve only to its own evidence;
- requirement coverage refs resolve;
- prior-finding disposition refs resolve;
- no cross-audit evidence leakage;
- historical `get(...)` returns resolvable evidence links;
- degraded UNAVAILABLE/MALFORMED re-audits still persist;
- R59-R62 regressions remain green.

---

# Verification record

Builder evidence reports:

- focused M16B/M16 audit engine: 15 PASS;
- full Rust: 358 PASS;
- full frontend: 125 PASS;
- migration suite: 13 PASS;
- Prompt Engine: 10 PASS;
- Agent Session Center: 9 PASS;
- Codex/process: 25 PASS;
- Audit Center frontend: 3 PASS;
- publisher rollback harness: 9/9 PASS;
- governed stable publication: PASS;
- stable SHA-256:
  `E608D5DE9B2976835AA08231E500DA8101ABA46AE8F6C8A6610920C8F06E6372`.

These establish broad regression health but do not assert the persisted evidence-reference invariant introduced by M16B's evidence-PK scoping change.

---

# Provenance

M16B implementation commit:

`404dca9101f2d646322bdaa176d1435b7fc266f3`

Evidence log HEAD:

`6f38c6d88796918ca19e693a4e98b32e53595aee`

The implementation commit directly parents the evidence publication commit.

---

# Required next state

Create a narrowly bounded M16C remediation for R63 only.

Do not redesign Audit Center.
Do not add a GPT provider.
Do not reopen R59-R62.
Do not activate M17.
Do not start M21.

After M16C:

1. independent strict re-audit;
2. native Audit Center acceptance;
3. if both pass, M16 may close and roadmap progress becomes `17 / 20 = 85%`.
