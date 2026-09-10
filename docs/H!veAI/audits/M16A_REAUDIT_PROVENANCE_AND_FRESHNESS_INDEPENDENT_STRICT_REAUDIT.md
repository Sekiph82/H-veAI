# M16A Re-audit Provenance and Freshness Independent Strict Re-Audit

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
M16A implementation commit: `3d95dc12dce7f0de804e19654cee4f7fa97458b8`
Evidence/tracker HEAD reviewed: `209461f1b00ca16ceb99b085463c417080633fdb`

## Verdict

**FAIL / CHANGES REQUIRED**

- BLOCKER: 0
- MAJOR: 1
- MINOR: 0
- NOTE: 0

### Prior findings

- M16-R59: CLOSED technically
- M16-R60: CLOSED technically
- M16-R61: CLOSED technically

### New finding

- M16-R62: MAJOR

M16 remains OPEN.
M17 MUST NOT activate.
M21 MUST NOT start.
Roadmap completed progress remains `16 / 20 = 80%`.

---

# M16-R59 closure — PASS

The prior omission-based closure behavior is removed.

Production now introduces typed `priorFindingDispositions` with:

- `priorFindingKey`
- `STILL_OPEN | CLOSED | SUPERSEDED`
- current `evidenceRefs`
- rationale
- optional replacement finding key

`apply_prior_finding_dispositions(...)` verifies:

- the disposition references a real OPEN prior finding;
- duplicate keys are rejected;
- evidence references exist in the current bounded audit input;
- CLOSED/SUPERSEDED require evidence and rationale;
- SUPERSEDED requires a replacement key.

If an AVAILABLE evaluation omits a prior finding, no disposition row is synthesized and the historical prior finding remains OPEN. Omission no longer proves closure.

R59 is CLOSED.

---

# M16-R60 closure — PASS

The Audit → Prompt Engine → Agent session provenance chain is now materially wired.

Prompt Engine:

1. reads bounded `auditId` from the handoff;
2. loads the audit through native authority;
3. validates the audit/project relationship;
4. verifies the exact persisted remediation prompt ID and version ID;
5. dispatches only through the existing Prompt Engine path;
6. calls `linkRemediationSession(...)` after the successful exact dispatch.

Native `link_remediation_session(...)` now verifies:

- audit/project ownership;
- remediation prompt association;
- exact prompt/version IDs;
- exact prompt version number;
- DISPATCHED state;
- exact dispatched session ID;
- approved body hash;
- session prompt/version/hash/body;
- dispatch provenance JSON.

An unrelated same-project session can no longer be linked merely because it belongs to the project.

Audit Center also exposes the persisted linked remediation session through bounded Agents navigation.

R60 is CLOSED.

---

# M16-R61 closure — PASS

Production now computes a deterministic freshness token from:

- branch;
- HEAD;
- staged files;
- unstaged files;
- untracked files;
- conflicted files;
- bounded working-tree diff hash;
- diff truncation state;
- repository identity.

`run_with_model(...)` collects evidence, evaluates the model, re-reads Git authority, recomputes the freshness token, and compares it before persistence.

A mismatch:

- forces `state = STALE`;
- forces a non-PASS CONDITIONAL result;
- clears current findings/dispositions/coverage;
- persists a bounded `AUDIT_STALE_REPOSITORY_CHANGED` diagnostic;
- prevents prior-finding disposition application.

This closes the original stale-evidence gap, including same-HEAD working-tree changes.

R61 is CLOSED.

---

# M16-R62 — MAJOR
## Production re-audit fails when the GPT provider is UNAVAILABLE or model output is MALFORMED

### Required behavior

M16 explicitly established truthful degraded states:

- no production GPT provider is currently configured;
- native audit runs therefore persist `CONDITIONAL / UNAVAILABLE`;
- unavailable or malformed re-audits must **not close prior findings**;
- historical/re-audit chain must remain truthful and usable.

The M16A remediation prompt likewise required:

`UNAVAILABLE, MALFORMED, STALE, FAILED evaluation never closes prior findings.`

That requirement does not mean the re-audit itself should fail to persist. It means it should persist truthfully without closing anything.

### Production defect

`run_with_model(...)` does this for the current production model:

1. collect input;
2. `evaluate_with(UnavailableAuditModel)` → `model_status = "UNAVAILABLE"`, verdict CONDITIONAL;
3. if repository is fresh, set `state = COMPLETED`;
4. call `persist_run(...)`.

Inside `persist_run(...)`, if `prior_audit_id` exists and state is COMPLETED, it unconditionally calls:

`apply_prior_finding_dispositions(...)`

But that function begins with:

`if evaluation.model_status != "AVAILABLE" { return Err("AUDIT_PRIOR_DISPOSITION_PROVIDER_NOT_ELIGIBLE") }`

Therefore a normal **Re-audit** in the current production environment, where GPT is intentionally UNAVAILABLE, returns an error instead of persisting the expected immutable CONDITIONAL/UNAVAILABLE re-audit row.

The same problem affects MALFORMED evaluation while repository evidence remains fresh.

This is particularly important because the production M16 provider is currently always `UnavailableAuditModel`. So the Audit Center's Re-audit button is expected to hit this defect in normal native use.

### Why this is MAJOR

The remediation fixed “unavailable must not close findings” by turning the whole fresh re-audit into an error.

That breaks:

- immutable re-audit history in the current production provider state;
- truthful degraded operation;
- Audit Center Re-audit UX;
- the contract that unavailable evidence remains visible rather than fabricated.

### Required remediation

Do not invoke disposition application unless the evaluation is eligible for disposition processing.

Recommended deterministic rule:

- `AVAILABLE + COMPLETED`: validate/apply explicit dispositions;
- `UNAVAILABLE + COMPLETED`: persist re-audit as CONDITIONAL/UNAVAILABLE, apply **zero** dispositions;
- `MALFORMED + COMPLETED`: persist re-audit as CONDITIONAL/MALFORMED, apply **zero** dispositions;
- `STALE`: persist STALE, apply zero dispositions;
- failure states: no closure.

In other words, `apply_prior_finding_dispositions` may remain AVAILABLE-only, but `persist_run` must not call it for ineligible model states.

Add direct production/domain tests for:

1. prior audit + UNAVAILABLE re-audit persists successfully;
2. prior OPEN finding remains unresolved;
3. prior audit + MALFORMED re-audit persists successfully;
4. prior OPEN finding remains unresolved;
5. AVAILABLE re-audit still applies explicit dispositions;
6. STALE re-audit still applies none.

---

# Verification record

Builder log reports:

- focused M16A Rust: 11 PASS;
- M16 frontend focused: 3 PASS;
- full Rust: 354 PASS;
- full frontend: 125 PASS;
- migration schema version 14: PASS;
- npm/typecheck/build/audit: PASS;
- Rust fmt/all-targets/pty-support: PASS;
- publisher rollback harness: 9/9 PASS;
- governed stable publication: PASS;
- candidate/stable SHA-256:
  `4305AC97557D02DDF7D5390270C6DB084E29F5299E6CDD6A100415E054F1CD78`.

These tests establish broad regression health but do not cover the production UNAVAILABLE re-audit path described in R62.

---

# Provenance

M16A implementation commit:

`3d95dc12dce7f0de804e19654cee4f7fa97458b8`

Current tracker/evidence HEAD:

`209461f1b00ca16ceb99b085463c417080633fdb`

The remediation source reviewed is present on the current branch.

---

# Required next state

Create a narrowly bounded **M16B** remediation for R62 only.

Do not redesign the Audit Engine.
Do not add a GPT provider.
Do not reopen R59-R61.
Do not activate M17.
Do not start M21.

After M16B:

1. independent strict re-audit;
2. native Audit Center acceptance;
3. if both pass, M16 may close and roadmap progress becomes `17 / 20 = 85%`.
