# M16T Codex-Only Audit Provider V10 — Schema Status and Readiness Parity Remediation

## MANDATORY SYNC-FIRST / GITHUB-FIRST CONTRACT

Before reading or changing any repository file, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the worktree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never reset, rebase, force-push, destructive checkout, auto-stash, `git clean`, or discard owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

Then read at minimum:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V09_STRICT_AUDIT.md`
- `docs/H!veAI/prompts/M16T_CODEX_ONLY_AUDIT_PROVIDER_V09_FAILURE_CLASSIFICATION_AND_READINESS_REMEDIATION_PROMPT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V09_LOG.md`
- `src-tauri/src/audit_engine.rs`
- nearest Codex provider/readiness/degraded-state Rust tests
- `src/App.tsx`
- focused Settings/Audit Center frontend tests

Every Codex-facing artifact and required builder log must be entirely in English.

All H!veAI changes must be committed and pushed before completion. Final completion requires:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Local `HEAD`, `origin/main`, and live GitHub `main` must match and the worktree must be clean.

---

## WORK ITEM

- Work code: `M16T`
- Version: `V10`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V10_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

V09 improved explicit failure classification, bounded diagnostics, and structured-output readiness. Independent audit found two residual truth gaps:

1. `SCHEMA_INCOMPATIBLE` is produced by the classifier/readiness path but is not preserved by `unavailable_evaluation` into persisted production audit `model_status`.
2. Settings readiness uses only a trivial `{ready:true}` schema, so it does not prove compatibility with the material structured-output features used by the actual V08 freeform audit schema.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

---

# ABSOLUTE ARCHITECTURE GUARDRAILS

Preserve all accepted behavior:

- Codex CLI is the only production model-backed audit provider;
- authentication is the owner's Codex-managed ChatGPT login;
- no `OPENAI_API_KEY`;
- no direct OpenAI HTTP/Responses audit transport;
- no API-key auth/fallback;
- no Codex auth/token-file inspection;
- no GUI automation;
- bounded native child-process execution;
- read-only ephemeral audit turns;
- supplied `AuditInput` is the only audit evidence authority;
- V05 `Sekiph82/FormuLab@main` and exact eight-project portfolio remain unchanged;
- V06 ACL/degraded semantics remain intact;
- V07 history state/model-status truth remains intact;
- V08 input-aware schema and fail-closed semantic validation remain intact;
- V09 explicit classifier hierarchy and bounded sanitized diagnostic behavior remain intact;
- M17/Claude remains blocked.

Do not weaken V08 or V09 to make tests pass.

---

# FINDING M16T-V10-F01 — `SCHEMA_INCOMPATIBLE` IS LOST AT THE PERSISTED AUDIT BOUNDARY

## Current defect

The provider process can return an error beginning with:

```text
AUDIT_CODEX_SCHEMA_INCOMPATIBLE: ...
```

However `unavailable_evaluation` currently maps only:

- `AUTH_POLICY_BLOCKED`
- `AUTH_REQUIRED`
- `USAGE_LIMITED`
- `TIMEOUT`
- `NETWORK_ERROR`
- `PROCESS_ERROR` / `FINAL_OUTPUT`
- fallback `UNAVAILABLE`

As a result, real production schema incompatibility can be persisted as:

```text
modelStatus = UNAVAILABLE
```

while the diagnostic itself says `AUDIT_CODEX_SCHEMA_INCOMPATIBLE`.

This is contradictory audit truth.

## Required remediation

Preserve an explicit `SCHEMA_INCOMPATIBLE` category through every relevant layer:

- provider failure classification;
- `unavailable_evaluation` model-status mapping;
- summary/rationale generation;
- persisted audit `model_status`;
- Audit Center current-result presentation;
- Audit history model-status badge;
- any Settings/readiness presentation already using this category.

A schema-incompatible audit must remain non-authoritative and non-completed under existing degraded-state rules. Do not convert it to AVAILABLE or COMPLETED.

Prefer one deterministic category mapping source if a small safe refactor reduces drift. Do not perform a broad unrelated refactor.

---

# FINDING M16T-V10-F02 — READINESS DOES NOT FEATURE-PROBE THE MATERIAL V08 AUDIT SCHEMA

## Current defect

The V09 readiness schema is effectively:

```json
{
  "type": "object",
  "additionalProperties": false,
  "properties": {
    "ready": { "type": "boolean", "enum": [true] }
  },
  "required": ["ready"]
}
```

This proves only that the CLI/provider can accept a basic `--output-schema` request.

It does **not** prove the structured-output capabilities the real freeform audit path relies on, such as:

- nested object arrays;
- nested `additionalProperties: false`;
- enum-constrained strings;
- `findings[].requirementRefs` array bounds including `maxItems: 0`;
- exactly one coverage row (`minItems/maxItems`);
- nested coverage evidence arrays with `maxItems: 0`;
- the same dedicated final-message path used by audits.

Settings can therefore say `READY` while a real freeform audit schema is rejected.

## Required remediation

Make explicit readiness a bounded **representative production-schema feature probe**.

Preferred implementation:

1. Construct a tiny deterministic synthetic freeform `AuditInput` in memory with zero canonical task requirements.
2. Build the readiness schema from the same production `audit_result_schema(&synthetic_input)` helper used by real audits, or from a dedicated compatibility helper proven by direct structural equality/subset assertions to exercise all material V08 freeform schema features above.
3. Use the same `CodexProcessRunner`, `--output-schema`, read-only ephemeral process policy, and dedicated final-message path.
4. Prompt Codex for one tiny deterministic schema-conformant freeform result, for example:
   - verdict `CONDITIONAL`;
   - bounded summary;
   - no findings;
   - exactly one `project-audit / NOT_APPLICABLE` coverage row;
   - empty coverage evidence refs;
   - empty prior-finding dispositions.
5. Parse the dedicated final result through the relevant production parse/semantic contract where practical, or through an explicit readiness validator that proves the same schema feature set without persisting an audit.
6. Readiness must create **no audit row** and must not inspect project files/Git/web/plugins.
7. If the representative schema is rejected by the installed CLI/provider, return `SCHEMA_INCOMPATIBLE` with the bounded sanitized V09 diagnostic.
8. If explicit quota/rate allowance blocks the probe, return `USAGE_LIMITED`.
9. If successful, return `READY`.

Do not hard-code a CLI version floor as the authority. Feature-probe the installed runtime.

### Important

The readiness probe must remain small. Do not send actual project evidence and do not consume a full project audit turn merely to test readiness.

---

# REQUIRED DIRECT TESTS

Add deterministic tests that do not consume live Codex quota. At minimum prove:

1. `AUDIT_CODEX_SCHEMA_INCOMPATIBLE` becomes `model_status = SCHEMA_INCOMPATIBLE` in degraded audit evaluation;
2. the resulting audit state remains FAILED/non-authoritative under current state rules;
3. summary/diagnostic truth remains category-consistent;
4. history/current result can display `SCHEMA_INCOMPATIBLE` distinctly from `UNAVAILABLE` and `USAGE_LIMITED`;
5. representative readiness schema exercises the material V08 freeform constructs, including nested arrays, canonical enums, `maxItems:0`, and one-row coverage bounds;
6. readiness success requires a conformant representative freeform result;
7. readiness basic/trivial success that would not satisfy the representative contract is not enough for READY;
8. representative schema rejection -> `SCHEMA_INCOMPATIBLE`;
9. explicit quota/rate failure -> `USAGE_LIMITED`;
10. `Usage: codex exec ...` alone remains `PROCESS_ERROR`;
11. readiness creates no audit row;
12. diagnostics remain bounded/redacted;
13. V08 valid/invalid freeform requirement-reference tests remain green;
14. V07 history truth tests remain green;
15. V06 ACL/degraded-state tests remain green;
16. V05 FormuLab/main and exact-eight-project tests remain green.

Where practical, inspect the generated readiness schema object directly in tests rather than testing only mocked process status strings.

---

# FRONTEND / UX REQUIREMENTS

Settings and Audit Center must use the same category truth vocabulary.

When schema incompatibility is proven:

- Settings: show Codex installed/login state truth plus `SCHEMA_INCOMPATIBLE` and bounded actionable diagnostic;
- Audit Center current result: show FAILED/non-authoritative + `SCHEMA_INCOMPATIBLE`;
- Audit history: show `SCHEMA_INCOMPATIBLE` rather than collapsing to `UNAVAILABLE`.

Do not invent an upgrade requirement unless the runtime probe actually proves incompatibility. You may suggest updating Codex as an action after incompatibility is proven, but runtime version alone must not determine readiness.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused V10 schema-status/readiness Rust tests;
2. V09 classifier/diagnostic tests;
3. V08 audit schema/semantic tests;
4. V06 provider/readiness/degraded-state tests;
5. V07 Audit Center history focused frontend tests;
6. Settings/readiness focused frontend tests;
7. V05 portfolio/FormuLab tests;
8. full frontend regression;
9. full Rust library regression under repository policy;
10. `npm run typecheck`;
11. `cargo check --manifest-path src-tauri/Cargo.toml`;
12. `npm run build`;
13. `git diff --check`;
14. active-source guardrail search proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses provider, API-key fallback, GUI automation, or Codex auth-file inspection was introduced;
15. governed native QA publication;
16. stable Desktop shortcut/no-terminal-flash publication checks.

Builder execution output remains claim evidence until independently audited.

---

# TRACKER GOVERNANCE

At implementation start move current/prospective truth to:

- Current Milestone: `M16`
- Current Sprint: `M16T-CODEX-ONLY`
- Current Task: `M16T V10 — Schema status and representative readiness parity remediation`
- Current Task Status: `IMPLEMENTATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16T marker: `[~]`
- M16: `OPEN`
- M17: `NOT ACTIVATED / BLOCKED`
- strict milestone progress remains `16 / 20 = 80%`

After successful builder completion set:

- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_REACCEPTANCE`
- Required Actor: `HUMAN`
- Next action: independent V10 strict audit, then owner Settings readiness and native three-run acceptance if source audit passes
- M16T remains `[~]`
- M16 remains OPEN
- M17 remains blocked

Historical V05-V09 prompts/logs/audits and persisted native audit history remain immutable.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V10_LOG.md`

Include:

- synchronized starting SHA;
- implementation SHA(s);
- exact F-V09-001 model-status parity fix;
- exact F-V09-002 representative readiness design;
- proof that readiness uses the material V08 freeform schema capabilities;
- direct tests for `SCHEMA_INCOMPATIBLE` persistence/state/history truth;
- direct tests for representative readiness success/rejection/quota/help-text behavior;
- V05-V09 regression evidence;
- full test/build counts and commands;
- forbidden-provider/auth-file/GUI guardrail scan;
- governed publication EXE SHA-256 and shortcut evidence;
- tracker final state;
- explicit statement that no OpenAI API-key/direct HTTP provider and no Claude/M17 implementation was introduced.

Do not require the log to contain the SHA of the commit that first creates itself. After log publication verify final remote-main equality and return the log commit SHA in the final response.

---

# OWNER NATIVE GATE AFTER INDEPENDENT V10 PASS

Do not fabricate owner acceptance.

After independent V10 source audit PASS:

1. owner launches the governed H!veAI build;
2. Settings -> Check readiness runs the representative structured-output feature probe;
3. readiness must truthfully return one of:
   - `READY`,
   - `SCHEMA_INCOMPATIBLE`,
   - genuine `USAGE_LIMITED`,
   - or another explicit bounded provider category;
4. if `READY`, owner runs the same project/freeform audit three consecutive times on unchanged HEAD;
5. all three must be `AVAILABLE + COMPLETED` with the V08 one-row `project-audit / NOT_APPLICABLE` contract;
6. no false quota, schema drift, MALFORMED, or unknown requirement reference may occur;
7. if a genuine external usage limit is explicitly proven, the truth path is accepted but the three-success-run closure gate remains pending until allowance is available.

Only after this human gate may M16 close and M17 activate.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- `SCHEMA_INCOMPATIBLE` survives provider -> evaluation -> persistence -> UI/history truth;
- readiness feature-probes the material V08 production freeform schema capabilities;
- readiness remains bounded/nonpersisted/read-only/ephemeral;
- V09 explicit classifier and sanitized diagnostics remain intact;
- V08 semantic fail-closed behavior remains intact;
- V05-V07 behavior remains intact;
- no prohibited provider/auth mechanism is introduced;
- all required tests/build/publication checks pass;
- every repository change is committed and pushed;
- local HEAD == origin/main == live main;
- worktree is clean;
- M16 remains OPEN pending independent V10 audit + owner native re-acceptance;
- M17 remains blocked.

Return `SYNC_BLOCKED` if synchronization cannot be safely completed.

Final owner-facing response must contain only:

- GitHub V10 log URL/path;
- implementation commit SHA(s);
- V10 log commit SHA;
- final GitHub main SHA;
- concise status.
