# M16T Codex-Only Audit Provider V01 Plan

## Decision

H!veAI will not use an OpenAI API-key based audit provider.

The production GPT Audit Engine will use the locally installed **Codex CLI authenticated through the owner's existing ChatGPT account**. H!veAI must not require, read, set, store, display, transmit, or document an `OPENAI_API_KEY` as an active product configuration path.

The existing OpenAI HTTP / Responses API provider introduced during M16S is superseded by this owner architecture decision and must be removed from active production code and current/prospective product documentation. Historical immutable prompts, logs, and audits remain unchanged as provenance.

This is a new bounded work item, `M16T`, because it changes the production provider architecture rather than merely remediating the M16S V02 implementation.

## Product intent

H!veAI is a local-first AI development command center. The owner already uses Codex locally under a ChatGPT subscription. Audit execution should reuse that authenticated local agent instead of introducing a second pay-per-token API credential path.

The desired provider topology is:

```text
Audit Engine
  -> AuditModel boundary
     -> CodexCliAuditModel        [only production model-backed provider]
        -> local codex executable
        -> existing ChatGPT login/auth owned by Codex CLI
     -> UnavailableAuditModel     [truthful fallback]
```

There is no active `OpenAiApiAuditModel` branch after M16T.

## Non-negotiable security boundary

H!veAI must never inspect, copy, parse, persist, or expose Codex authentication tokens or Codex auth files.

H!veAI may invoke supported Codex CLI commands such as version/auth-status/headless execution and consume bounded non-secret stdout/stderr/exit status. Authentication remains wholly owned by Codex CLI.

H!veAI must not:

- read `~/.codex/auth.json` or equivalent credential-store files;
- read or set `OPENAI_API_KEY`;
- retain the M16S native OpenAI HTTPS provider as a hidden fallback;
- call `api.openai.com` directly for audit execution or readiness;
- automate the ChatGPT desktop GUI;
- scrape a terminal/TUI screen;
- persist authentication material to SQLite, logs, prompts, diagnostics, frontend state, localStorage, environment files, or source control.

## Existing foundation to reuse

H!veAI already contains a mature native `CodexAdapter` foundation with:

- native `codex.exe` resolution;
- bounded version probing;
- native child-process execution;
- stdin prompt transport;
- stdout/stderr capture and redaction;
- session lifecycle/persistence;
- stop/reconcile behavior;
- no shell-mediated process launch;
- local project working-directory validation.

M16T must reuse or factor this machinery rather than create an unrelated second Codex executable resolver/process-policy implementation.

If needed, extract a small shared native Codex runtime module used by both the existing agent adapter and the new audit provider. Keep agent-session orchestration and audit execution as separate consumers with separate safety policies.

## Production audit execution model

### 1. Provider resolution

The production audit model resolver becomes:

- `CodexCliAuditModel` when the supported native Codex CLI is available and authenticated/usable;
- `UnavailableAuditModel` otherwise.

No API-key provider remains.

### 2. Authentication readiness

Cheap page-load readiness may check only local executable/version/auth status and must not consume a model turn.

A user-triggered **Check readiness** operation should be able to perform a bounded end-to-end Codex CLI probe so stale login state that merely reports `Logged in using ChatGPT` cannot be misrepresented as fully ready.

Readiness should distinguish at minimum:

- `READY`;
- `CODEX_NOT_FOUND`;
- `AUTH_REQUIRED` or equivalent;
- `AUTH_UNVERIFIED` for cheap local-only state if appropriate;
- `USAGE_LIMITED` / rate-or-quota limited where detectable;
- `NETWORK_ERROR`;
- `TIMEOUT`;
- `PROCESS_ERROR` / malformed execution.

The explicit readiness probe may consume a tiny amount of the owner's Codex plan allowance, but it must never incur a separate OpenAI API-key billing path.

### 3. Headless audit process

Use the installed Codex CLI's supported headless execution mode after verifying the exact installed CLI contract.

Current Codex supports headless `codex exec`, JSON event output, final-message output, and JSON-schema-constrained output. M16T must verify the installed command surface at runtime/build time rather than assuming historical flags blindly.

Preferred audit execution properties:

- non-interactive/headless;
- ephemeral audit thread/session where supported;
- strict JSON Schema for the existing H!veAI audit result contract;
- read-only sandbox;
- no approval prompts;
- no dangerous bypass flags;
- no user MCP/plugin/tool configuration if the installed CLI offers a supported isolation flag;
- dedicated temporary audit working directory rather than the owner's project working tree;
- no Git mutation and no source mutation;
- bounded prompt bytes, result bytes, stderr bytes, duration, and process lifetime;
- explicit kill/escalation on timeout;
- temporary schema/output files removed after bounded use.

The already-collected H!veAI `AuditInput` is the authoritative evidence supplied to the model. Codex must be instructed not to discover extra project evidence independently during the audit turn.

### 4. Final-output authority

Do not treat the first JSONL `agent_message` as the final audit result.

Use the CLI's final-message/output-file contract where supported, then independently parse and validate the final result against the existing H!veAI Rust schema and semantic validation gates.

JSONL may be retained only as bounded operational diagnostics/lifecycle evidence. stderr must never be merged into the JSONL protocol parser.

### 5. Runtime provenance

Persist only runtime-authoritative provenance.

At minimum:

- `auditor_provider = CODEX_CLI`;
- `auditor_version =` bounded native Codex CLI version actually invoked;
- `auditor_model =` an explicit model H!veAI passed to Codex, if one is intentionally configured, otherwise a truthful non-spoofing value such as `CLI_DEFAULT` / `CLI_DEFAULT_UNVERIFIED`.

Do not scrape the Codex TUI to infer a model name. Do not allow generated model text to provide execution identity.

H!veAI should prefer the Codex CLI account/default model rather than hard-coding an obsolete model slug. Existing active hard-coded Codex model names such as `gpt-5.5` must be reviewed and removed if they would override the user's current supported Codex default without an explicit product requirement.

## Settings UX

Replace the current OpenAI API-provider Settings surface with a **Codex Audit Provider** surface.

It should show non-secret information only, such as:

- provider: Codex CLI;
- executable available/unavailable;
- CLI version;
- login/auth status;
- readiness state;
- last bounded diagnostic category;
- `Check readiness` action.

There must be:

- no API-key field;
- no `OPENAI_API_KEY` setup guidance;
- no OpenAI API model text field whose purpose is API billing/configuration;
- no direct HTTP provider readiness action.

If model selection is retained at all, it must be explicitly Codex-CLI-native and optional. Default behavior should use the installed Codex CLI's supported account default.

## Active-source removal requirements

Remove active production support for the superseded API provider, including as applicable:

- `OpenAiAuditModel` / equivalent;
- `ReqwestAuditTransport` audit-provider transport;
- Responses API and model-endpoint constants;
- `OPENAI_API_KEY` lookup/readiness logic;
- API-provider model setting keys and commands;
- API-provider frontend types/settings copy;
- API-provider focused tests that no longer represent production behavior;
- `reqwest` dependency if no other active H!veAI source requires it after the removal.

Historical immutable M16S prompts/logs/audits may still contain those names and must not be rewritten.

A final scoped repository search must prove that active source/current prospective docs contain no live `OPENAI_API_KEY` or direct OpenAI audit HTTP provider path.

## Local workspace acceptance retained

The M16S local-workspace restoration remains valid and must not regress.

The existing H!veAI project is now attached to:

`C:\Users\sekip\Desktop\H!veAI`

The Projects UI must retain visible Attach / Change / Repair Local Workspace controls, preserve one logical GitHub project identity, and keep the portfolio at eight projects.

M16T is not permission to redesign Project Registry.

## Relationship to M17 Claude Code Adapter

M17 remains blocked until M16T is independently audited and owner-native accepted.

M16T implements **Codex CLI as the audit provider only**. It does not implement Claude.

After M16T closes M16:

- M17 will implement the Claude Code Adapter for builder/agent execution;
- Codex CLI remains the independent audit provider by default;
- later architecture may allow explicit provider-role selection, but no API-key audit provider is required.

Preferred future role split:

```text
Builder / implementation agent: Claude Code (after M17)
Independent auditor:          Codex CLI
```

Codex may still be available as a builder where already supported, but M17 must not weaken the separation between builder claims and independent audit evidence.

## Tracker transition

At M16T implementation start, current/prospective tracker truth should move from `M16S V02 awaiting owner acceptance` to the owner-directed `M16T V01 Codex-only audit provider migration`.

M16S source work may be recorded as superseded by the owner provider decision rather than treated as a failed implementation.

During M16T:

- M16 = OPEN;
- M16T = active;
- M17-M20 = planned/blocked;
- M21 and M21-R01 through M21-R03 remain PASS/CLOSED;
- completed roadmap denominator remains 16/20 until M16 final acceptance.

After implementation, tracker status becomes implementation-complete awaiting independent strict audit and owner-native acceptance.

Only after both pass may M16 become PASS/CLOSED and M17 become the next active milestone.

## Required implementation validation

M16T must include deterministic tests for:

1. native Codex executable resolution shared/reused correctly;
2. no executable => truthful unavailable state;
3. `codex login status` / supported auth-state classification without reading credential files;
4. explicit end-to-end readiness probe success/failure classification using a mocked process runner;
5. timeout and process termination;
6. auth-required, usage-limit/rate-limit, network, malformed output and nonzero-exit classifications;
7. exact safe audit CLI argument policy;
8. read-only/no-dangerous-bypass execution policy;
9. strict output-schema handoff;
10. authoritative final-message capture rather than first/intermediate JSONL message;
11. stderr kept separate from JSONL protocol parsing;
12. strict Rust audit-result parse + semantic validation;
13. malformed/schema-invalid result can never become PASS;
14. trusted provider/version/model provenance;
15. audit history persistence/reload;
16. no API key/env lookup/direct OpenAI HTTP request in active production audit code;
17. Settings shows Codex provider and contains no API-key setup surface;
18. existing local-workspace attach/change/repair UI remains working;
19. eight-project portfolio remains invariant;
20. full frontend/Rust/typecheck/build/diff-check and governed native publication.

Automated tests must not consume the owner's real Codex quota; use a deterministic injected/mock process runner. The real Codex turn is owner-native acceptance after independent source audit PASS.

## Owner-native acceptance after implementation

The final human acceptance gate should prove:

1. Settings shows **Codex Audit Provider**, not an OpenAI API-key provider.
2. No API key is requested anywhere.
3. Codex CLI readiness identifies the locally installed CLI and ChatGPT login state.
4. Explicit readiness check succeeds with the owner's existing Codex/ChatGPT account.
5. A real Audit Center run on a selected project completes through Codex CLI.
6. The persisted audit has `auditor_provider = CODEX_CLI`, a truthful CLI/model provenance value, `modelStatus = AVAILABLE`, and a strict PASS/CONDITIONAL/FAIL result according to evidence.
7. No terminal window flashes visibly during native execution.
8. Projects still show eight logical projects and the H!veAI local workspace remains attached.

## Exit condition

M16T exits only when:

- the OpenAI API-key provider is absent from active production architecture;
- Codex CLI is the sole model-backed audit provider;
- deterministic source/tests/publication pass;
- independent ChatGPT strict source audit passes;
- owner-native Codex readiness and real audit execution pass.

Then M16 may be marked PASS/CLOSED and M17 Claude Code Adapter may start.
