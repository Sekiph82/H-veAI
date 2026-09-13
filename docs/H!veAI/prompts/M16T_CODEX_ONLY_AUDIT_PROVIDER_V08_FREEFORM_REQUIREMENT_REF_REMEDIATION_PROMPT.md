# M16T Codex-Only Audit Provider V08 — Freeform Requirement-Reference Stability Remediation

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

If the working tree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never use reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or any operation that discards owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

After synchronization read at minimum:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V07_STRICT_AUDIT.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V08_NATIVE_STABILITY_AUDIT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V07_LOG.md`
- `src-tauri/src/audit_engine.rs`
- `tests/m16-audit-center-focused.test.tsx`
- the nearest focused Rust audit-engine test modules

All Codex-facing work and the required builder log must be entirely in English.

All H!veAI repository changes made by this task must be committed and pushed before completion is claimed. At the end verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == live remote main` and a clean H!veAI worktree.

---

## WORK ITEM

- Work code: `M16T`
- Version: `V08`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V08_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

V07 history-truth remediation is independently accepted. Owner-native re-acceptance then proved:

- local Codex CLI readiness reaches READY through the owner's ChatGPT-managed Codex login;
- a real freeform/project audit can persist `AVAILABLE + COMPLETED`;
- V07 history rows correctly distinguish run state and model status;
- but a subsequent equivalent freeform run on the same HEAD persisted `FAILED + MALFORMED` with diagnostic `AUDIT_REQUIREMENT_REFERENCE_UNKNOWN`.

The source-level root cause is a contract mismatch among `audit_output_contract`, the static `audit_result_schema`, and `validate_semantic_evaluation` for audits with zero canonical task requirements.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

---

# ABSOLUTE ARCHITECTURE GUARDRAILS

Preserve all accepted M16T architecture and earlier remediation behavior:

- Codex CLI is the only production model-backed audit provider;
- authentication is Codex-managed ChatGPT login;
- no `OPENAI_API_KEY` anywhere in the production audit path;
- no direct OpenAI HTTP or Responses audit transport;
- no API-key authentication or fallback;
- no reading Codex auth/token files;
- bounded ephemeral Codex audit process;
- read-only sandbox and supplied AuditInput-only authority;
- V06 readiness ACL and truthful degraded-state semantics;
- V07 audit-history state/model-status presentation;
- V05 `Sekiph82/FormuLab@main` and branch-aware cache behavior;
- exact eight-project active portfolio;
- M17/Claude blocked and inactive.

Do not activate Claude/M17.

Do not weaken semantic validation to make the symptom disappear.

---

# FINDING M16T-V08-F01 — FREEFORM FINDING REQUIREMENT REFERENCES ARE UNDERSPECIFIED

## Current mismatch

For project/freeform audits `input.requirements` is empty. Therefore `validate_semantic_evaluation` builds an empty canonical requirement set.

The validator correctly rejects every non-empty `finding.requirement_refs` value because no task requirement exists:

```rust
if finding
    .requirement_refs
    .iter()
    .any(|reference| !canonical_requirements.contains(reference.as_str()))
{
    return Err("AUDIT_REQUIREMENT_REFERENCE_UNKNOWN".into());
}
```

However the current project/freeform model contract tells Codex only how to shape `requirementCoverage`. It does not explicitly state that every finding must use `requirementRefs: []`.

The static structured-output schema likewise permits any string array for `findings[].requirementRefs`.

This lets equivalent real Codex turns alternate between semantically accepted and semantically rejected outputs.

## Required project/freeform contract

When there are zero canonical task requirements:

1. `requirementCoverage` must contain exactly one row;
2. that row must use `requirementRef = "project-audit"`;
3. that row must use `status = "NOT_APPLICABLE"`;
4. that row must use `evidenceRefs = []`;
5. every finding must use `requirementRefs = []`;
6. `project-audit` is a synthetic coverage identifier only and must **not** be used as a finding requirement ref;
7. project-level findings are still allowed when directly supported by supplied evidence;
8. finding evidence refs remain subject to normal evidence identity and quality validation.

Do not invent a fake canonical task requirement merely to make finding refs legal.

---

# FINDING M16T-V08-F02 — STATIC SCHEMA PERMITS KNOWN-ILLEGAL REQUIREMENT IDENTITIES

Make the audit structured-output schema input-aware.

A preferred bounded direction is to evolve:

```rust
fn audit_result_schema() -> Value
```

into an input-aware builder such as:

```rust
fn audit_result_schema(input: &AuditInput) -> Value
```

or an equivalently explicit helper.

## Project/freeform schema requirements

When the canonical required-requirement set is empty, schema must enforce at least:

- `findings[].requirementRefs` has `maxItems: 0`;
- `requirementCoverage` has exactly one row (`minItems: 1`, `maxItems: 1`);
- coverage `requirementRef` is fixed to `project-audit`;
- coverage `status` is fixed to `NOT_APPLICABLE`;
- coverage `evidenceRefs` has `maxItems: 0`.

Keep all existing object `additionalProperties: false` behavior and output bounds.

## Task-scoped schema requirements

When required canonical task refs exist:

- coverage requirement refs must be selected only from the canonical required-ref set;
- finding requirement refs, when present, must be selected only from that same canonical set;
- coverage count must remain consistent with the canonical required set;
- do not weaken the independent semantic validator;
- do not make unrelated findings impossible solely because they choose an empty requirement-ref list if current semantic policy permits that.

If JSON Schema features supported by the current Codex CLI structured-output implementation require a simpler equivalent representation, use the narrowest compatible schema that still prevents the known illegal freeform outputs. Document any compatibility choice in the V08 log.

---

# REQUIRED PROMPT-CONTRACT HARDENING

Update `audit_output_contract(input)` consistently with the schema.

For project/freeform audits explicitly instruct:

- there are zero canonical task-scoped requirements;
- every finding must return `requirementRefs: []`;
- `project-audit` is only the synthetic coverage identifier and must never appear in a finding requirementRefs array;
- return exactly one `project-audit / NOT_APPLICABLE` coverage row with empty evidence refs;
- evidence-backed project-level findings remain permitted.

For task-scoped audits explicitly instruct:

- use exactly the supplied canonical refs for coverage;
- finding requirement refs, if present, must be selected only from the supplied canonical list;
- do not invent requirement identities.

---

# SEMANTIC VALIDATOR MUST REMAIN FAIL-CLOSED

Do not solve V08 by making malformed model output authoritative.

Specifically forbidden:

- accepting arbitrary unknown requirement refs;
- silently deleting invalid finding requirement refs after parsing;
- silently translating `project-audit` finding refs to empty arrays;
- globally adding `project-audit` to the real canonical requirement set;
- changing MALFORMED output to AVAILABLE;
- changing FAILED output to COMPLETED;
- retrying until a syntactically convenient result appears without a strict bounded policy;
- weakening evidence-reference validation.

The independent semantic validator must continue to reject output that violates the canonical contract even if a fixture bypasses the structured-output schema.

---

# REQUIRED DIRECT TESTS

Add deterministic tests that do not consume live Codex quota.

At minimum prove:

1. a project/freeform schema forbids non-empty `findings[].requirementRefs`;
2. a project/freeform schema requires exactly one coverage row;
3. that row is constrained to `project-audit / NOT_APPLICABLE` with empty evidence refs;
4. the project/freeform prompt contract explicitly requires finding `requirementRefs: []` and says `project-audit` must not be used there;
5. a valid project-level finding with `requirementRefs=[]` plus valid direct evidence is semantically accepted;
6. a fixture that bypasses schema and supplies `requirementRefs=["project-audit"]` is still rejected with `AUDIT_REQUIREMENT_REFERENCE_UNKNOWN`;
7. a fixture that supplies another invented freeform finding requirement ref remains rejected;
8. a valid task-scoped finding referencing a canonical task requirement remains accepted;
9. an invented task-scoped finding requirement ref remains rejected;
10. task-scoped requirement coverage remains complete, unique, and canonical;
11. V06 degraded-state semantics remain green;
12. V07 same-verdict history validity tests remain green;
13. readiness ACL/settings behavior remains green;
14. V05 FormuLab/main and exact eight-project portfolio tests remain green.

Where practical, inspect the generated schema object directly in tests rather than relying only on a mocked model response.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused audit-engine Rust tests covering the dynamic schema/prompt/semantic contract;
2. existing V06 audit provider/readiness/degraded-state focused Rust tests;
3. existing V07 Audit Center focused frontend tests;
4. Settings/readiness focused frontend tests;
5. V05 portfolio/FormuLab focused tests;
6. full frontend regression;
7. full Rust library regression under repository policy;
8. `npm run typecheck`;
9. `cargo check --manifest-path src-tauri/Cargo.toml`;
10. `npm run build`;
11. `git diff --check`;
12. active-source guardrail search proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses audit transport, API-key fallback, or auth-file inspection was introduced;
13. governed native QA publication because live audit behavior changed;
14. stable Desktop shortcut target/icon and no-terminal-flash regression under existing publication policy.

Builder test output remains builder evidence, not independent acceptance.

---

# TRACKER GOVERNANCE

At implementation start move current/prospective truth to:

- Current Milestone: `M16`
- Current Sprint: `M16T-CODEX-ONLY`
- Current Task: `M16T V08 — Freeform requirement-reference contract stability remediation`
- Current Task Status: `IMPLEMENTATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16T summary marker: `[~]`
- M16: `OPEN`
- M17: `NOT ACTIVATED / BLOCKED`
- strict milestone progress remains `16 / 20 = 80%`

After successful builder completion set:

- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_STABILITY_REACCEPTANCE`
- Next Task/Action: independent M16T V08 strict audit, then final owner native readiness plus three-consecutive-freeform-run stability acceptance
- Required Actor: `HUMAN`
- M16T summary marker remains `[~]`
- M16 remains OPEN
- M17 remains NOT ACTIVATED/BLOCKED

Historical V05-V07 prompts/logs/audits and previous persisted native audit history remain immutable.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V08_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- implementation commit SHA(s) known before log publication;
- exact source root cause and exact contract/schema fix;
- generated-schema behavior for freeform and task-scoped inputs;
- prompt-contract behavior;
- semantic-validator preservation proof;
- direct valid/invalid freeform finding tests;
- direct task-scoped canonical/noncanonical reference tests;
- V06/V07/V05/readiness regressions;
- full test/build counts and commands;
- provider guardrail search result;
- governed native publication executable SHA-256 and shortcut target/icon evidence;
- tracker final state;
- explicit statement that no OpenAI API-key/direct HTTP provider and no Claude/M17 implementation was introduced.

Do not require the log to contain the SHA of the commit that first creates itself. After publishing the log, verify final remote `main` and return the log commit SHA in the final response.

---

# FINAL OWNER-NATIVE STABILITY GATE AFTER INDEPENDENT V08 PASS

Do not fabricate or automate owner acceptance.

After independent V08 source audit PASS, the owner will use the governed stable H!veAI build and prove:

1. Settings -> Codex Audit Provider reports READY with ChatGPT-authenticated local Codex;
2. the same registered project and unchanged HEAD are used for three consecutive project/freeform audits;
3. all three persist with `modelStatus=AVAILABLE` and `state=COMPLETED`;
4. verdict may legitimately vary based on evidence, but none may fail because of schema/semantic requirement-reference contract drift;
5. each run contains exactly one `project-audit / NOT_APPLICABLE` coverage row;
6. none produces `AUDIT_REQUIREMENT_REFERENCE_UNKNOWN`, MALFORMED, or schema degradation;
7. history visibly distinguishes those authoritative runs while preserving older degraded runs immutably.

Only after that human gate may M16 be closed and M17 activated.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- project/freeform finding requirement refs are explicitly empty at prompt and schema layers;
- freeform coverage is schema-constrained to the synthetic one-row contract;
- task-scoped references are constrained to canonical identities;
- semantic validation remains independent and fail-closed;
- all required deterministic tests pass;
- V05/V06/V07/readiness behavior remains intact;
- Codex-only/no-API-key architecture remains intact;
- governed native publication passes;
- every H!veAI change is committed and pushed;
- local HEAD == origin/main == live GitHub main;
- worktree is clean;
- M16 remains OPEN pending independent V08 audit + owner native stability re-acceptance;
- M17 remains blocked.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must contain only:

- GitHub V08 log URL/path;
- implementation commit SHA(s);
- V08 log commit SHA;
- final GitHub main SHA;
- concise status.
