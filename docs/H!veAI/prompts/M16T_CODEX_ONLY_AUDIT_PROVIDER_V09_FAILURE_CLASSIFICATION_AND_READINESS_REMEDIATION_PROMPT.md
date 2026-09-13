# M16T Codex-Only Audit Provider V09 — Failure Classification and Production-Equivalent Readiness Remediation

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

Never reset, rebase, force-push, destructive-checkout, auto-stash, `git clean`, or discard owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

Then read at minimum:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V08_STRICT_AUDIT.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V09_NATIVE_FAILURE_CLASSIFICATION_AUDIT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V08_LOG.md`
- `src-tauri/src/audit_engine.rs`
- nearest provider/readiness/failure-classification Rust tests
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

Local HEAD, `origin/main`, and live GitHub main must match and the worktree must be clean.

---

## WORK ITEM

- Work code: `M16T`
- Version: `V09`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V09_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

The accepted V08 schema/semantic remediation must remain intact.

Owner-native V08 acceptance produced three consecutive `FAILED / USAGE_LIMITED` freeform audit rows after Settings had reached `READY`. Repository inspection proves that the current result label is not trustworthy enough to establish a real quota exhaustion because `classify_codex_failure` treats the generic substring `usage` as `USAGE_LIMITED`, and `Check readiness` does not exercise the production structured-output/schema path.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

---

# ABSOLUTE ARCHITECTURE GUARDRAILS

Preserve all accepted H!veAI architecture:

- Codex CLI is the only production model-backed audit provider;
- authentication is the owner's Codex-managed ChatGPT login;
- no `OPENAI_API_KEY`;
- no direct OpenAI HTTP/Responses audit transport;
- no API-key auth or fallback;
- no reading Codex auth/token files;
- no ChatGPT or Codex GUI automation;
- bounded native child process execution;
- read-only ephemeral audit turns;
- supplied AuditInput is the only audit evidence authority;
- V05 `Sekiph82/FormuLab@main` and exact eight-project portfolio remain unchanged;
- V06 ACL/degraded state semantics remain intact;
- V07 history state/model-status truth remains intact;
- V08 dynamic schema, prompt contract, and fail-closed semantic validation remain intact;
- M17/Claude remains blocked.

Do not weaken V08 schema or semantic validation to make process errors disappear.

---

# FINDING M16T-V09-F01 — GENERIC `usage` SUBSTRING CAUSES FALSE `USAGE_LIMITED`

The current classifier effectively performs:

```rust
text.contains("usage") || text.contains("quota") || text.contains("rate limit")
```

This is not an acceptable quota detector because ordinary command-line help/errors commonly contain a `Usage:` line.

## Required classification behavior

Implement explicit and deterministic failure categories. At minimum support:

- `AUTH_POLICY_BLOCKED`
- `AUTH_REQUIRED`
- `USAGE_LIMITED`
- `SCHEMA_INCOMPATIBLE` (or an equivalently explicit schema/structured-output category)
- `NETWORK_ERROR`
- `TIMEOUT`
- `PROCESS_ERROR`

`USAGE_LIMITED` must require an explicit quota/rate/allowance signal. Examples that may qualify include exact/normalized phrases such as:

- `usage limit`
- `you've hit your usage limit`
- `rate limit`
- `rate_limit`
- `quota exceeded`
- `insufficient quota`
- an explicit 429/rate-limit provider error if observable in the bounded result

The generic word `usage` by itself must **not** qualify.

A line beginning with or containing ordinary CLI help such as:

```text
Usage: codex exec ...
```

must not be classified as `USAGE_LIMITED` unless the same bounded output also contains an explicit quota/rate-limit signal.

Tighten auth and network classification similarly. Do not classify success-ish text merely because it contains generic words such as `authenticated`.

Fallback remains `PROCESS_ERROR` when no stronger category is proven.

---

# FINDING M16T-V09-F02 — READINESS DOES NOT EXERCISE PRODUCTION STRUCTURED OUTPUT

Current `Check readiness` runs a plain text turn with `schema: None` and expects `READY`. This proves executable/login/basic-turn availability only.

Explicit readiness must also prove the structured-output boundary H!veAI actually uses for audits.

## Required readiness behavior

`Check readiness` must perform a bounded, non-persisted, read-only, ephemeral structured-output probe that uses the same Codex process path and the relevant schema capabilities required by production audits.

Preferred direction:

- create a tiny deterministic readiness schema/prompt, or use a minimal freeform-audit-compatible schema fixture;
- invoke through the same `CodexProcessRunner` and `--output-schema` mechanism used by audits;
- require a schema-conformant dedicated final message;
- do **not** create an audit history row;
- do **not** inspect the project filesystem or Git repository;
- keep the probe small enough to minimize usage;
- preserve the existing detected Codex version and ChatGPT-login state.

If the representative structured-output path is unsupported/rejected, readiness must **not** return `READY`. Return a truthful schema/process category such as `SCHEMA_INCOMPATIBLE` with actionable diagnostic text.

Do not hard-code a version floor as the sole authority. Feature-probe the actual installed CLI. It is acceptable to surface the detected version and suggest updating Codex when the feature probe proves incompatibility.

---

# FINDING M16T-V09-F03 — DIAGNOSTIC DISCARDS THE PROVIDER FAILURE DETAIL

Current process classification returns only a generic sentence, preventing the owner/auditor from distinguishing a genuine quota response from command usage/help, invalid schema, unsupported flag, or another process failure.

## Required diagnostic behavior

Persist/display a bounded sanitized diagnostic excerpt for failed provider turns.

Requirements:

- include the H!veAI category plus a short excerpt of relevant bounded stderr/stdout;
- strip/redact secrets, authorization headers, bearer tokens, passwords, API keys, auth paths/tokens, and other credential material;
- reuse `sanitize_text` where appropriate or create a narrowly equivalent process-diagnostic sanitizer;
- cap bytes/lines aggressively;
- prefer stderr and error events over noisy normal JSON event output;
- do not persist arbitrary unbounded CLI output;
- do not expose Codex auth files or environment secrets;
- keep history/result truth immutable.

Example desired behavior:

```text
AUDIT_CODEX_SCHEMA_INCOMPATIBLE: invalid_json_schema: unsupported keyword ...
```

or

```text
AUDIT_CODEX_USAGE_LIMITED: You've hit your usage limit ... try again at ...
```

not merely:

```text
AUDIT_CODEX_USAGE_LIMITED: bounded Codex process did not produce an accepted result
```

---

# STRUCTURED EVENT AWARENESS

Because audit execution already uses `codex exec --json`, prefer structured/explicit failure evidence when practical instead of broad substring guessing across arbitrary stdout.

You may parse bounded JSONL error/event records or implement conservative text extraction from known error lines. Do not make the implementation fragile by depending on undocumented exact full payload shapes when a bounded fallback is necessary.

The classification hierarchy must be deterministic and covered by tests.

---

# V08 STRUCTURED-OUTPUT COMPATIBILITY

Do not assume the owner's three `USAGE_LIMITED` rows were a real quota exhaustion.

The installed owner version observed in native evidence is `codex-cli 0.130.0-alpha.5`. Upstream Codex has continued to evolve `--output-schema` behavior. V09 must make H!veAI tell the truth about the installed CLI by feature probing the production-equivalent structured-output boundary.

If V08's generated schema contains a feature rejected by the installed CLI/backend structured-output subset, classify and surface that as `SCHEMA_INCOMPATIBLE` rather than `USAGE_LIMITED`.

If the installed CLI genuinely reports a usage/rate/quota limit, preserve that as `USAGE_LIMITED` and surface the bounded provider reset/limit detail when available.

Do not automatically upgrade Codex as part of H!veAI runtime. H!veAI may provide an actionable recommendation if compatibility probing fails.

---

# REQUIRED DIRECT TESTS

Add deterministic tests without consuming live Codex quota. At minimum prove:

1. `Usage: codex exec ...` alone -> **not** `USAGE_LIMITED`;
2. `error: unknown argument ...\nUsage: codex exec ...` -> `PROCESS_ERROR` or the appropriate explicit non-quota category;
3. `You've hit your usage limit` -> `USAGE_LIMITED`;
4. `rate limit` / explicit quota exceeded -> `USAGE_LIMITED`;
5. invalid/unsupported JSON/output schema text -> `SCHEMA_INCOMPATIBLE`;
6. explicit login/auth-required failure -> `AUTH_REQUIRED`;
7. API-key authentication policy failure remains `AUTH_POLICY_BLOCKED`;
8. explicit network/connectivity failure -> `NETWORK_ERROR`;
9. unknown non-zero failure -> `PROCESS_ERROR`;
10. diagnostics are bounded and sanitized;
11. credential-like strings are redacted;
12. readiness success requires a schema-conformant structured-output probe;
13. readiness schema rejection returns `SCHEMA_INCOMPATIBLE`, not READY and not USAGE_LIMITED;
14. readiness explicit quota failure returns `USAGE_LIMITED`;
15. readiness does not persist an audit row;
16. V08 dynamic freeform/task schema tests remain green;
17. V07 history truth tests remain green;
18. V06 ACL/degraded-state tests remain green;
19. V05 FormuLab/main and exact-eight-project tests remain green.

---

# FRONTEND / UX REQUIREMENTS

Settings must truthfully display at least:

- provider
- executable availability
- detected CLI version
- login state
- readiness status
- actionable error category/diagnostic when not READY

Audit Center must continue to show failed model status/state and diagnostic truthfully.

If `SCHEMA_INCOMPATIBLE`, the UI should make clear that Codex is installed/authenticated but the installed CLI/structured-output path is incompatible with the audit contract, instead of saying quota is exhausted.

If genuine `USAGE_LIMITED`, state that the external Codex allowance/rate limit prevented an authoritative audit and preserve any bounded reset guidance supplied by Codex.

Do not invent an exact reset time if Codex did not provide one.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused classifier/readiness Rust tests;
2. V08 audit-engine schema/semantic focused tests;
3. V06 provider/readiness/degraded-state focused tests;
4. V07 Audit Center focused frontend tests;
5. Settings/readiness focused frontend tests;
6. V05 portfolio/FormuLab tests;
7. full frontend regression;
8. full Rust library regression under repository policy;
9. `npm run typecheck`;
10. `cargo check --manifest-path src-tauri/Cargo.toml`;
11. `npm run build`;
12. `git diff --check`;
13. active-source guardrail search proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses provider, API-key fallback, GUI automation, or Codex auth-file inspection was introduced;
14. governed native QA publication;
15. stable Desktop shortcut/no-terminal-flash publication checks.

Builder execution output remains claim evidence until independently audited.

---

# TRACKER GOVERNANCE

At implementation start:

- Current Milestone: `M16`
- Current Sprint: `M16T-CODEX-ONLY`
- Current Task: `M16T V09 — Codex failure classification and production-equivalent readiness remediation`
- Status: `IMPLEMENTATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16T `[~]`
- M16 `OPEN`
- M17 `NOT ACTIVATED / BLOCKED`
- strict progress remains `16 / 20 = 80%`

After successful builder completion:

- Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_REACCEPTANCE`
- Required Actor: `HUMAN`
- Next action: independent V09 strict audit, then owner Settings readiness and native freeform acceptance
- M16T remains `[~]`
- M16 remains OPEN
- M17 remains blocked

Historical V05-V08 prompts/logs/audits and persisted native audit history remain immutable.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V09_LOG.md`

Include:

- synchronized starting SHA;
- implementation SHA(s);
- exact failure-classification root cause;
- exact classifier hierarchy/pattern policy;
- production-equivalent structured-output readiness design;
- exact sanitized diagnostic behavior and bounds;
- tests proving `Usage:` help is not quota;
- tests proving explicit quota is quota;
- tests proving schema incompatibility is separately classified;
- V05-V08 regression evidence;
- full test/build counts and commands;
- forbidden-provider/auth-file/GUI guardrail scan;
- governed publication EXE SHA-256 and shortcut evidence;
- tracker final state;
- explicit statement that no OpenAI API-key/direct HTTP provider and no Claude/M17 implementation was introduced.

Do not require the log to contain the SHA of the commit that first creates itself. After log publication verify final remote-main equality and return the log commit SHA in the final response.

---

# OWNER NATIVE GATE AFTER INDEPENDENT V09 PASS

Do not fabricate owner acceptance.

After independent V09 source audit PASS:

1. owner launches governed H!veAI build;
2. Settings -> Check readiness must either:
   - reach `READY` after a representative structured-output probe, or
   - truthfully show a specific external/provider incompatibility/usage category with bounded diagnostic;
3. if READY, owner runs the same project/freeform audit three consecutive times on an unchanged HEAD;
4. all three must become `AVAILABLE + COMPLETED` with the V08 one-row `project-audit / NOT_APPLICABLE` coverage contract;
5. no false `USAGE_LIMITED`, schema drift, MALFORMED, or unknown requirement reference may occur;
6. if a genuine Codex usage limit is explicitly proven by Codex, the code-path truth is accepted but M16's three-success-run gate remains pending until allowance resets or becomes available.

Only after the human gate may M16 close and M17 activate.

---

# COMPLETION GATE

Return COMPLETE only when:

- generic `usage` no longer means quota;
- explicit quota signals classify as `USAGE_LIMITED`;
- schema incompatibility is distinct and actionable;
- readiness exercises production-equivalent structured output;
- diagnostics are bounded, sanitized, and useful;
- V05-V08 behavior remains intact;
- no forbidden provider/auth mechanism is introduced;
- all required tests/build/publication checks pass;
- every change is committed and pushed;
- local HEAD == origin/main == live main;
- clean worktree;
- M16 remains OPEN pending independent V09 audit + owner native re-acceptance;
- M17 remains blocked.

Return `SYNC_BLOCKED` if synchronization cannot be safely completed.

Final owner-facing response must contain only:

- GitHub V09 log URL/path;
- implementation commit SHA(s);
- V09 log commit SHA;
- final GitHub main SHA;
- concise status.
