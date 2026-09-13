# M16T Codex-Only Audit Provider V11 — Structured-Output Subset and Readiness Diagnostic Parity Remediation

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

Never reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or discard owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

Then read at minimum:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V10_STRICT_AUDIT.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V11_NATIVE_SCHEMA_COMPATIBILITY_AUDIT.md`
- `docs/H!veAI/prompts/M16T_CODEX_ONLY_AUDIT_PROVIDER_V10_SCHEMA_STATUS_AND_READINESS_PARITY_REMEDIATION_PROMPT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V10_LOG.md`
- `src-tauri/src/audit_engine.rs`
- nearest provider/readiness/schema/semantic tests
- `src/App.tsx`
- focused Settings/Audit Center frontend tests

Every Codex-facing artifact and builder log must be entirely in English.

All H!veAI repository changes must be committed and pushed before completion. Final completion requires:

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
- Version: `V11`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V11_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

V10 passed independent source audit, but the owner-native V10 readiness gate on `codex-cli 0.154.0` returns:

```text
SCHEMA_INCOMPATIBLE
AUDIT_CODEX_SCHEMA_INCOMPATIBLE: Reading prompt from stdin...
```

The old `0.130.0-alpha.5` model-cache parsing failure is gone. V11 must close the newly exposed structured-output subset and diagnostic-truth defects without weakening any accepted audit semantic guard.

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
- V08 input-aware freeform/task semantic fail-closed rules remain intact;
- V09 explicit failure classifier and bounded/redacted diagnostic behavior remain intact;
- V10 explicit `SCHEMA_INCOMPATIBLE` persistence/history truth remains intact;
- M17/Claude remains blocked.

Do not weaken the semantic validator to make the provider transport pass.

---

# FINDING M16T-V11-F01 — PROVIDER TRANSPORT SCHEMA EMITS NONESSENTIAL `uniqueItems`

## Current defect

`audit_result_schema(input)` currently materializes coverage bounds and always emits a `uniqueItems` field into `requirementCoverage`, including `uniqueItems: false` in the freeform readiness schema.

Current OpenAI Structured Outputs documentation describes a supported JSON Schema subset. For arrays it documents `minItems` and `maxItems` as supported constraints; `uniqueItems` is not part of the documented supported array-property list.

H!veAI does not need transport-level `uniqueItems` because `validate_semantic_evaluation` already enforces stronger application truth:

- freeform audits must contain exactly one `project-audit / NOT_APPLICABLE` coverage row;
- task-scoped audits require exactly one row for every canonical required requirement reference;
- duplicate canonical requirement coverage fails semantic validation;
- extra/noncanonical references fail semantic validation.

## Required remediation

1. Remove `uniqueItems` from the JSON schema sent via Codex `--output-schema`.
2. Do **not** remove or weaken the existing semantic uniqueness/exact-cardinality checks.
3. Keep provider-supported constraints that are useful and documented, including `minItems` / `maxItems`, unless a direct deterministic compatibility reason proves they must also be transported differently.
4. Preserve `additionalProperties: false` on every object and keep all declared Structured Output properties required.
5. Preserve nullable fields through the existing required `["string", "null"]` pattern.
6. Keep canonical enum constraints and input-aware freeform/task requirement references.
7. Prefer one production schema helper; do not create a divergent readiness-only contract.

### Direct schema tests

Inspect the generated schema object directly and prove at minimum:

- `uniqueItems` does not appear anywhere in the transport schema;
- freeform `findings[].requirementRefs` keeps the existing intended transport shape except for unsupported/nonessential keywords;
- freeform coverage still carries the supported one-row bounds where compatible;
- task-scoped canonical enums remain present;
- every object has `additionalProperties: false`;
- every object property remains listed in that object's `required` array;
- V08 semantic validator still rejects duplicate/extra/missing requirement coverage independently of transport constraints.

Do not silently normalize an invalid model result into an accepted one.

---

# FINDING M16T-V11-F02 — READINESS COLLAPSES FINAL-RESULT FAILURES INTO `SCHEMA_INCOMPATIBLE`

## Current defect

`check_codex_readiness_with_runner` correctly classifies nonzero child-process failures through `classify_codex_failure_category`.

But for `exit_code == 0`, it currently reduces all dedicated-final-result validation to a boolean:

```text
final_message -> parse_model_output -> validate_semantic_evaluation -> is_some()
```

Any false result is returned as `SCHEMA_INCOMPATIBLE`.

This conflates materially different cases:

- provider/process explicitly rejected the output schema;
- dedicated final message is missing;
- final message is not valid JSON / cannot be parsed into the audit shape;
- final message is syntactically valid but fails H!veAI semantic validation.

The user-facing diagnostic is then built from process streams. A benign Codex progress line such as:

```text
Reading prompt from stdin...
```

can therefore mask the actual final-result validation failure.

The owner-native V10 screenshot demonstrates this exact ambiguous state.

## Required remediation

Refactor explicit readiness validation to preserve the real failure stage.

For a successful child process (`exit_code == 0`, not timed out):

1. If dedicated `final_message` is missing, return a truthful non-ready category such as `PROCESS_ERROR` (or a narrowly named existing final-output category if already supported) with diagnostic `AUDIT_CODEX_FINAL_OUTPUT_MISSING`.
2. If `parse_model_output(final_message)` fails, return `MALFORMED` with the exact bounded parse error code/message. Do not call this provider schema incompatibility.
3. If parsing succeeds but `validate_semantic_evaluation` fails, return `MALFORMED` with the exact bounded semantic error code/message.
4. Only return `SCHEMA_INCOMPATIBLE` when the provider/process explicitly reports schema/structured-output incompatibility through the classified failure path, or another direct transport-level proof exists.
5. If parsing and semantic validation both succeed, return `READY`.

### Diagnostic precedence

- Prefer the specific final-result parse/semantic error when that is the failure source.
- Do not replace it with benign stderr progress such as `Reading prompt from stdin...`.
- For nonzero process failures, preserve the V09 bounded/redacted process diagnostic rules.
- Never expose credentials, auth files, bearer tokens, API keys, or private paths beyond existing sanitized policy.

### UI vocabulary

Settings should distinguish at least:

- `READY`
- `SCHEMA_INCOMPATIBLE`
- `MALFORMED`
- `USAGE_LIMITED`
- `AUTH_REQUIRED`
- `AUTH_POLICY_BLOCKED`
- `NETWORK_ERROR`
- `TIMEOUT`
- `PROCESS_ERROR`

Do not claim that a `MALFORMED` readiness result proves provider schema incompatibility.

---

# REQUIRED DIRECT TESTS

Add deterministic tests that consume no live Codex quota. At minimum prove:

1. generated freeform transport schema contains no `uniqueItems` at any depth;
2. generated task-scoped transport schema contains no `uniqueItems` at any depth;
3. semantic duplicate/extra/missing coverage rejection remains intact after removing transport `uniqueItems`;
4. current supported schema properties, required arrays, `additionalProperties:false`, canonical enums, nullable required fields, and freeform/task distinctions remain intact;
5. readiness nonzero explicit schema rejection -> `SCHEMA_INCOMPATIBLE` with bounded sanitized provider diagnostic;
6. readiness exit 0 + missing final message -> truthful final-output/process category, not `SCHEMA_INCOMPATIBLE`;
7. readiness exit 0 + malformed JSON -> `MALFORMED` with parse diagnostic;
8. readiness exit 0 + semantic contract error -> `MALFORMED` with semantic diagnostic;
9. benign stderr `Reading prompt from stdin...` does not mask the parse/semantic error;
10. readiness conformant representative output -> `READY`;
11. explicit quota -> `USAGE_LIMITED`;
12. plain CLI help `Usage: codex exec ...` alone remains `PROCESS_ERROR`;
13. readiness creates no audit row;
14. production audit `SCHEMA_INCOMPATIBLE` persistence/history truth from V10 remains green;
15. V08 freeform/task schema + semantic tests remain green;
16. V07 history truth remains green;
17. V06 ACL/degraded-state tests remain green;
18. V05 FormuLab/main and exact-eight-project tests remain green.

Add a small recursive schema-keyword assertion helper in tests if useful, but do not introduce a second schema implementation.

---

# FRONTEND / UX REQUIREMENTS

Keep Settings compact but truthful.

- `SCHEMA_INCOMPATIBLE` means provider/transport structured-output incompatibility is explicitly proven.
- `MALFORMED` means the provider returned a final result that failed H!veAI parsing or semantic validation.
- The bounded diagnostic must show the specific parse/semantic code instead of benign process progress when applicable.
- Existing Audit Center history truth remains unchanged.

No new settings for API keys or direct HTTP providers may appear.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused V11 transport-schema and readiness-stage tests;
2. V10 schema-status/readiness tests;
3. V09 classifier/diagnostic tests;
4. V08 audit schema/semantic tests;
5. V06 provider/readiness/degraded-state tests;
6. V07 Audit Center history focused frontend tests;
7. Settings/readiness focused frontend tests;
8. V05 portfolio/FormuLab tests;
9. full frontend regression;
10. full Rust library regression under repository policy;
11. `npm run typecheck`;
12. `cargo check --manifest-path src-tauri/Cargo.toml`;
13. `npm run build`;
14. `git diff --check`;
15. active-source guardrail search proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses provider, API-key fallback, GUI automation, or Codex auth-file inspection was introduced;
16. governed native QA publication;
17. stable Desktop shortcut/no-terminal-flash publication checks.

Builder execution output remains claim evidence until independently audited.

---

# TRACKER GOVERNANCE

At implementation start move current/prospective truth to:

- Current Milestone: `M16`
- Current Sprint: `M16T-CODEX-ONLY`
- Current Task: `M16T V11 — Structured-output subset and readiness diagnostic parity remediation`
- Current Task Status: `IMPLEMENTATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16T marker: `[~]`
- M16: `OPEN`
- M17: `NOT ACTIVATED / BLOCKED`
- strict milestone progress remains `16 / 20 = 80%`

After successful builder completion set:

- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_REACCEPTANCE`
- Required Actor: `HUMAN`
- Next action: independent V11 strict audit, then owner Settings readiness; only if READY, run the native three-run freeform acceptance gate
- M16T remains `[~]`
- M16 remains OPEN
- M17 remains blocked

Historical V05-V10 prompts/logs/audits and persisted native audit history remain immutable.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V11_LOG.md`

Include:

- synchronized starting SHA;
- implementation SHA(s);
- exact transport-schema change and why semantic validation remains authoritative for uniqueness/cardinality;
- proof that generated schemas no longer emit `uniqueItems`;
- exact readiness stage/error-category refactor;
- tests for missing/malformed/semantic-invalid/conformant dedicated final results;
- proof benign `Reading prompt from stdin...` does not mask a more specific final-result diagnostic;
- V05-V10 regression evidence;
- full test/build counts and commands;
- forbidden-provider/auth-file/GUI guardrail scan;
- governed publication EXE SHA-256 and shortcut evidence;
- tracker final state;
- explicit statement that no OpenAI API-key/direct HTTP provider and no Claude/M17 implementation was introduced.

Do not require the log to contain the SHA of the commit that first creates itself. After log publication verify final remote-main equality and return the log commit SHA in the final response.

---

# OWNER NATIVE GATE AFTER INDEPENDENT V11 PASS

Do not fabricate owner acceptance.

After independent V11 source audit PASS:

1. owner launches the governed H!veAI build;
2. Settings -> Check readiness runs the representative production-schema probe;
3. if the result is not READY, the category and diagnostic must identify the actual failure stage truthfully;
4. if READY, owner runs the same project/freeform audit three consecutive times on unchanged HEAD;
5. all three must be `AVAILABLE + COMPLETED` with the V08 one-row `project-audit / NOT_APPLICABLE` contract;
6. no false quota, false schema incompatibility, MALFORMED drift, or unknown requirement reference may occur;
7. a genuine external limit/provider failure keeps M16 open until the three-run gate can complete.

Only after this HUMAN gate may M16 close and M17 activate.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- Codex transport schemas no longer emit `uniqueItems`;
- semantic uniqueness/cardinality remains fail-closed;
- readiness distinguishes provider schema rejection from final-output missing/parse/semantic failure;
- specific final-result diagnostics outrank benign stderr progress;
- V10 `SCHEMA_INCOMPATIBLE` persistence/history truth remains intact;
- V09 explicit classifier and sanitized diagnostics remain intact;
- V08 semantic fail-closed behavior remains intact;
- V05-V07 behavior remains intact;
- no prohibited provider/auth mechanism is introduced;
- all required tests/build/publication checks pass;
- every repository change is committed and pushed;
- local HEAD == origin/main == live main;
- worktree is clean;
- M16 remains OPEN pending independent V11 audit + owner native re-acceptance;
- M17 remains blocked.

Return `SYNC_BLOCKED` if synchronization cannot be safely completed.

Final owner-facing response must contain only:

- GitHub V11 log URL/path;
- implementation commit SHA(s);
- V11 log commit SHA;
- final GitHub main SHA;
- concise status.
