# M16 GPT Audit Engine Independent Strict Audit

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Authoritative implementation prompt: `docs/H!veAI/prompts/M16_GPT_AUDIT_ENGINE_UNIFIED_WHOLE_MILESTONE_IMPLEMENTATION_PROMPT.md`
Builder log: `docs/H!veAI/codex-logs/M16_GPT_AUDIT_ENGINE_IMPLEMENTATION_LOG.md`
Audited implementation commit: `3fc850b9e314980a5837660eacae26347427cf8d`
Audited evidence HEAD: `88089cfc154bc302daba11d73d495c972cfa9a3c`

## Verdict

**FAIL / CHANGES REQUIRED**

- BLOCKER: 0
- MAJOR: 3
- MINOR: 0
- NOTE: 1

M15 remains PASS/CLOSED.
M16 remains OPEN.
M17 MUST NOT activate.
M21 MUST NOT start.
Roadmap completed progress remains `16 / 20 = 80%` because M15 is closed, but M16 is not accepted/closed.

The builder completed a substantial M16 foundation and its automated regression record is strong, but three source-level contract defects prevent independent closure.

---

# Finding M16-R59 — MAJOR
## Re-audit automatically closes any prior finding omitted by the model

### Required contract

The M16 prompt requires re-audit to compare prior/open/closed/new findings truthfully and explicitly says:

- prior audits remain immutable;
- unavailable/malformed evaluation must not close prior findings;
- a finding must not be declared fixed merely because the builder claims it;
- re-audit must collect fresh evidence and determine which findings are actually closed.

### Production behavior

`reconcile_prior_findings(...)` runs when `evaluation.model_status == "AVAILABLE"`.

For every prior OPEN finding, it checks only whether the current model returned the same `finding_key`.

If the current model did **not** return that key, production code appends a new current-audit finding with:

- `status = "CLOSED"`
- `closed_by_audit_id = current_audit_id`

There is no requirement that the current audit has:

- VERIFIED requirement coverage proving the defect is fixed;
- direct source/test evidence tied to the prior finding;
- an explicit model closure decision;
- a specific closure rationale for that finding.

Therefore simple omission from a valid model response is treated as proof of remediation.

### Why this is unsafe

A model can miss a previously detected defect, truncate attention, or produce a narrower set of findings. Omission is not evidence of closure.

The existing test `available_reaudit_closes_absent_prior_finding_but_unavailable_does_not` actually locks this unsafe behavior in as expected behavior.

### Required remediation

Introduce an explicit per-prior-finding re-audit disposition contract.

A prior OPEN finding may close only when the new structured result explicitly references the prior finding/finding_key and declares a closure status supported by fresh evidence/coverage.

At minimum require:

- prior finding key/reference;
- disposition such as `STILL_OPEN | CLOSED | SUPERSEDED`;
- evidence refs;
- rationale;
- validated refs must exist in current bounded audit input;
- CLOSED must not be accepted from absence alone.

Update fixtures so omission preserves the finding as unresolved rather than closing it.

---

# Finding M16-R60 — MAJOR
## Remediation session provenance is not actually wired into the Audit → Prompt Engine → Agent flow

### Required contract

M16.05 requires:

`Audit FAIL → selected findings → remediation Prompt Engine draft → explicit approval → remediation session provenance → re-audit → preserved chain`

The audit record must link the remediation prompt **and the actual dispatched remediation session**.

### Production behavior

`create_remediation_prompt(...)` correctly persists:

- `remediation_prompt_id`
- `remediation_prompt_version_id`

The Audit Center then navigates to:

`/prompts?projectId=...&promptId=...&auditId=...`

However, `PromptEnginePage.tsx` only consumes `projectId` and `promptId`. It does not consume `auditId`.

After the remediation prompt is approved and dispatched, Prompt Engine performs the normal M15 dispatch and records the Agent session, but it never calls:

`hiveai_audit_link_remediation_session`

The frontend helper `linkRemediationSession(...)` exists in `src/auditEngine.ts`, but the production remediation dispatch flow does not use it.

Therefore `audits.remediation_session_id` remains unlinked in the intended user flow.

### Additional provenance weakness

The native `link_remediation_session(...)` command itself validates only:

- audit belongs to project;
- session belongs to project.

It does **not** prove that the session was dispatched from that audit's exact:

- `remediation_prompt_id`
- `remediation_prompt_version_id`

So even if the command were called, an unrelated same-project session could be attached to the audit.

### Required remediation

Wire the end-to-end provenance path.

Recommended boundary:

1. Audit Center creates remediation draft and passes bounded audit identity to Prompt Engine.
2. Prompt Engine recognizes a validated remediation-audit handoff.
3. After successful dispatch, native linking must atomically/strictly verify:
   - audit project matches session project;
   - audit remediation prompt ID matches session prompt ID;
   - audit remediation prompt version ID matches session prompt version ID;
   - session operation/provenance represents that exact Prompt Engine dispatch.
4. Only then persist `remediation_session_id`.
5. UI/history should display the linked session.
6. Add direct tests proving unrelated same-project sessions are rejected.

Do not weaken M15A dispatch semantics.

---

# Finding M16-R61 — MAJOR
## Audit freshness is not enforced between evidence collection and persistence

### Required contract

The M16 prompt explicitly requires:

- final branch/HEAD/diff-scope verification;
- if branch HEAD changes after input collection, mark the audit STALE/invalid rather than silently auditing the wrong code;
- stale evidence handling must be truthful.

### Production behavior

`run(...)` currently performs:

1. `collect_input(...)`
2. `evaluate_with(...)`
3. `persist_run(...)`

There is no freshness validation between steps 2 and 3.

A helper exists:

`current_head_is_fresh(database, audit)`

but it operates on an already persisted `AuditRun` and is not used by `run(...)` to gate persistence.

The implementation therefore does not detect even a HEAD change occurring after evidence collection and before final persistence.

It also does not detect working-tree changes that leave HEAD unchanged, despite the audit input containing a working-tree diff.

### Why this matters

Once a real network GPT provider is configured, evaluation may take seconds or longer. The repository can change during the model call. The persisted verdict can then claim to audit evidence from an earlier state while the user sees a newer repository state.

### Required remediation

Create an evidence-freshness token at collection time that covers at least:

- branch;
- HEAD;
- bounded changed-file/diff identity or deterministic Git snapshot identity.

Immediately before persisting an AVAILABLE/MALFORMED/UNAVAILABLE audit result, re-read the current Git authority and compare the token.

If changed:

- persist/return `STALE` according to a deterministic state contract, or reject and require recollection;
- do not persist a normal COMPLETED PASS/CONDITIONAL/FAIL against stale evidence;
- never close prior findings from a stale re-audit.

Add tests for:
- HEAD changes during evaluation;
- working-tree diff changes with same HEAD;
- branch changes;
- unchanged evidence remains eligible.

---

# Finding M16-N01 — NOTE
## Production GPT provider is intentionally unavailable

The implementation uses `UnavailableAuditModel` in production and therefore native audits currently produce a truthful CONDITIONAL / UNAVAILABLE result.

This is consistent with the authoritative M16 prompt, which explicitly allowed a provider-neutral interface plus truthful UNAVAILABLE behavior when no configured GPT provider exists.

This is **not** a defect for M16 implementation acceptance by itself.

Native/user acceptance should verify that this unavailable state is understandable and non-misleading.

---

# Verified strengths

The independent review confirms the following implementation areas are materially present:

- typed audit evidence/result enums;
- explicit PASS/CONDITIONAL/FAIL schema;
- BLOCKER/MAJOR/MINOR/NOTE severities;
- verification classifications including CLAIM_ONLY and UNVERIFIED;
- bounded evidence constants and UTF-8-safe truncation;
- builder logs treated as CLAIM_ONLY;
- persisted test runs treated as UNVERIFIED;
- source-path secret/traversal policy;
- strict structured model-output parsing with unknown-field rejection;
- malformed/unavailable model output cannot become PASS;
- migration 13 adds audit evidence/coverage/provenance structures;
- Audit Center replaces the prior placeholder;
- remediation draft creation goes through Prompt Engine rather than launching a provider directly;
- M15 human review/approval and dispatch boundaries remain present;
- Audit Center technical evidence is secondary/collapsed;
- production capability is narrow rather than broad shell/filesystem access.

---

# Builder verification record

The immutable M16 builder log reports:

- frontend: 124/124 PASS across 15 files;
- full native library: 351 PASS;
- focused audit native tests: 8 PASS;
- focused Audit Center frontend: 3 PASS;
- migration regression slice: 22 PASS;
- TypeScript typecheck/build/npm audit: PASS;
- Rust fmt/lib/all-targets/pty-support: PASS;
- publisher rollback harness: 9/9 PASS;
- governed stable publication: PASS;
- candidate/stable SHA-256:
  `2A87243DC31FCA0044490DA2DB888EEB9E0B5D5DB33D45C5C1081AC7FDF2E07F`;
- stable size: `22177792` bytes;
- native interactive Audit Center acceptance: pending.

These automated results are useful regression evidence, but they do not cover R59-R61 correctly.

---

# Provenance

Builder log abbreviates implementation commit as `3fc850b`.

The actual resolved implementation commit is:

`3fc850b9e314980a5837660eacae26347427cf8d`

Current evidence publication HEAD is:

`88089cfc154bc302daba11d73d495c972cfa9a3c`

The current HEAD directly parents the implementation commit through the documentation/evidence commit, so provenance is coherent.

---

# Required next state

M16 must remain OPEN.

A bounded M16A remediation should close only:

- M16-R59 re-audit closure proof;
- M16-R60 exact remediation-session provenance wiring;
- M16-R61 pre-persistence audit freshness enforcement.

Do not redesign unrelated M16 surfaces.
Do not implement a GPT provider as part of this remediation unless independently required.
Do not activate M17.
Do not start M21.

After M16A implementation:

1. independent strict re-audit;
2. native Audit Center acceptance;
3. if both pass, M16 may close and roadmap progress becomes `17 / 20 = 85%`.
