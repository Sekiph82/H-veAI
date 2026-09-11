# M16T Codex-Only Audit Provider V01

## MANDATORY SYNC-FIRST AND GITHUB-FIRST CONTRACT

Before reading or changing implementation files, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

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

After synchronization, read these current authorities before implementation:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/plans/M16T_CODEX_ONLY_AUDIT_PROVIDER_V01_PLAN.md`
- `docs/H!veAI/audits/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V03_AUDIT.md`
- `docs/H!veAI/codex-logs/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V02_LOG.md`
- `src-tauri/src/codex_adapter.rs`
- `src-tauri/src/audit_engine.rs`

All Codex-facing work and the required builder log must be entirely in English.

All H!veAI repository changes made by this task must be committed and pushed before completion is claimed. At the end verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == live remote main` and a clean H!veAI working tree. Otherwise return `SYNC_BLOCKED`.

Owner-facing final response must contain only relevant GitHub H!veAI file URLs/paths, implementation/log commit SHA(s), final GitHub `main` SHA, and concise status. Do not dump ordinary local changed-file paths.

---

## WORK ITEM

- Work code: `M16T`
- Version: `V01`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Plan: `docs/H!veAI/plans/M16T_CODEX_ONLY_AUDIT_PROVIDER_V01_PLAN.md`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V01_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

This work item implements an explicit owner architecture decision:

> **H!veAI must not have an OpenAI API-key audit path. The only production model-backed audit provider is the locally installed Codex CLI authenticated through the owner's existing ChatGPT account.**

This is not permission to activate M17 or implement Claude.

---

# OWNER DECISION — ABSOLUTE API-KEY PROHIBITION

The following are prohibited in active production architecture after M16T:

- `OPENAI_API_KEY` as an H!veAI configuration path;
- direct H!veAI audit calls to `api.openai.com`;
- OpenAI Responses API HTTP audit execution from H!veAI;
- an `OpenAiAuditModel` / OpenAI HTTP provider hidden as fallback;
- API-key input fields or API-key setup guidance in H!veAI UI;
- model readiness implemented through OpenAI HTTP endpoints;
- reading, writing, copying, parsing, or displaying Codex authentication files/tokens;
- automating ChatGPT Desktop GUI windows;
- terminal/TUI screen scraping.

Historical immutable M16S prompts, logs, and audits may contain the old API-provider design and MUST NOT be rewritten merely to erase history.

The active product may use the normal Codex CLI authentication state owned by Codex itself. H!veAI must not extract the underlying token.

If `codex login status` indicates API-key authentication rather than ChatGPT authentication, H!veAI must refuse to treat that state as the accepted M16T production auth mode and report a truthful policy/readiness state. Do not read auth files to determine this.

---

# PHASE 0 — TRACKER TRANSITION BEFORE PRODUCT CHANGES

The current tracker still reflects M16S V02 awaiting owner acceptance. The owner has superseded the API-provider direction before final acceptance.

Before production implementation changes:

1. update current/prospective `TASKS.md` and `CODEX_ROADMAP.md` truth to activate `M16T V01 — Codex-only audit provider`;
2. record M16S V02 as source-audit PASS but superseded before final owner audit-provider acceptance by the owner architecture decision;
3. keep the local-workspace M16S acceptance intact;
4. keep M16 OPEN;
5. keep M17-M20 NOT ACTIVATED/BLOCKED;
6. keep M21 and M21-R01 through M21-R03 PASS/CLOSED;
7. keep completed roadmap progress at 16/20 until final M16 acceptance;
8. set Required Actor to CODEX while M16T implementation is active.

Commit and push this tracker transition before the product implementation commit(s). Do not rewrite historical immutable artifacts.

---

# PHASE 1 — RECOVER AND SHARE THE EXISTING CODEX RUNTIME FOUNDATION

H!veAI already contains a substantial `CodexAdapter`. Reuse it.

Inspect the current native Codex foundation, including at minimum:

- executable resolution;
- native executable validation;
- version probe;
- background/no-console process policy;
- stdin/stdout/stderr capture;
- bounded output;
- redaction;
- process timeout/stop/reconcile behavior;
- fixed argument construction;
- current hard-coded model behavior, if any;
- current `codex login status` support or absence.

Do not implement a second unrelated `codex.exe` resolver.

Preferred architecture: extract the smallest shared native Codex runtime primitives into a dedicated internal module used by both:

- the existing agent/session `CodexAdapter`; and
- the new audit `CodexCliAuditModel`.

Do not merge agent-session persistence semantics into audit persistence. Shared process/runtime primitives are appropriate; product workflows remain separate.

## Remove obsolete hard-coded Codex model assumptions

Inspect active Codex adapter argument policy for historical hard-coded model slugs such as `gpt-5.5`.

Unless a current explicit product contract requires a fixed model, do not force an obsolete model slug. Prefer the installed Codex CLI's current ChatGPT-account default.

If H!veAI does not explicitly pass `--model`, persisted audit provenance must say a truthful runtime value such as `CLI_DEFAULT`, not invent an exact model name.

Do not scrape TUI text to infer the effective model.

---

# PHASE 2 — REMOVE THE OPENAI HTTP / API-KEY AUDIT PROVIDER

Remove the superseded active production provider and its product-facing configuration.

At minimum inspect and remove/refactor as applicable:

- `OpenAiAuditModel` or equivalent HTTP-backed model;
- `ReqwestAuditTransport` used only for the audit provider;
- OpenAI Responses API URL constants;
- OpenAI model endpoint/readiness URL constants;
- `OPENAI_API_KEY` environment reads;
- API-provider model-setting keys and setter/readiness commands;
- frontend OpenAI API-provider settings types and copy;
- API-provider `Check readiness` HTTP path;
- direct HTTP provider tests that no longer represent production architecture.

Remove `reqwest` from `src-tauri/Cargo.toml` / lockfile if and only if no other active H!veAI production source uses it after the provider removal. Prove usage before removing the dependency.

Do not weaken the existing evidence collector, audit schema, semantic validator, freshness logic, history chain, finding persistence, remediation provenance, or truthful non-PASS fallback behavior.

At task completion run a scoped search proving active source/current prospective docs contain no live API-key audit path. Historical immutable artifacts are excluded from that absence check.

At minimum the active production tree must not retain operative references to:

- `OPENAI_API_KEY`;
- `OpenAiAuditModel`;
- OpenAI audit `ReqwestAuditTransport`;
- `https://api.openai.com/v1/responses` for H!veAI audit execution.

---

# PHASE 3 — IMPLEMENT `CodexCliAuditModel`

Keep the existing provider-neutral `AuditModel` boundary.

Production model resolution becomes exactly:

```text
CodexCliAuditModel
or
UnavailableAuditModel
```

There is no production API-key model branch.

## Codex auth policy

Use supported Codex CLI commands only.

Cheap local readiness may use bounded:

- native executable resolution;
- `codex --version`;
- `codex login status`.

Do not read Codex credential files.

Recognize ChatGPT-authenticated Codex state as the accepted auth path. A CLI state explicitly reporting API-key authentication must be treated as unsupported by this H!veAI policy, even if Codex itself could technically use it.

Because `codex login status` can be stale relative to a real request, configuration/auth presence alone MUST NOT become full `READY`.

## Explicit end-to-end readiness

Implement an explicit user-triggered bounded readiness operation that performs the smallest safe real Codex headless turn necessary to prove:

- executable works;
- ChatGPT authentication works end-to-end;
- Codex backend is reachable;
- the account can execute a turn.

This may consume a tiny amount of the owner's Codex plan allowance. It must not use an H!veAI API key and must not create an H!veAI audit record.

Classify truthfully at minimum:

- `READY`;
- `CODEX_NOT_FOUND`;
- `AUTH_REQUIRED`;
- `AUTH_POLICY_BLOCKED` when Codex reports API-key auth instead of ChatGPT auth;
- `AUTH_UNVERIFIED` / local configuration only where useful;
- `USAGE_LIMITED` or equivalent when detectable from bounded process output;
- `NETWORK_ERROR`;
- `TIMEOUT`;
- `PROCESS_ERROR`;
- malformed/unexpected protocol output.

Never convert a failed readiness probe into READY.

---

# PHASE 4 — SAFE HEADLESS CODEX AUDIT EXECUTION

Use the installed Codex CLI's supported headless execution contract. Before fixing the argument policy, inspect the installed/current supported CLI help/schema rather than relying on historical assumptions.

The current Codex project supports headless `codex exec`, JSON/JSONL event output, final-message output, JSON-schema-constrained output, sandbox selection, and ephemeral execution in current versions. Use only flags actually supported by the target installed CLI and verified by focused tests/feature checks.

## Mandatory audit safety properties

The audit process must be:

- native child process, not shell-mediated;
- headless/non-interactive;
- bounded by explicit total timeout;
- killed/escalated safely on timeout;
- isolated from the owner project working tree as much as practical;
- run from a dedicated temporary audit directory rather than the project checkout;
- read-only sandboxed;
- non-interactive approval policy;
- never run with `--dangerously-bypass-approvals-and-sandbox` or equivalent bypass;
- ephemeral where the installed supported CLI provides that mode;
- isolated from user MCP/plugin/tool configuration where a supported Codex isolation flag exists;
- supplied only the bounded H!veAI `AuditInput` as authoritative evidence;
- instructed not to discover extra repository/filesystem/web evidence;
- bounded in prompt bytes, stdout bytes, stderr bytes, JSONL events, final-result bytes, and temp-file sizes.

A dedicated temporary audit directory may contain only the minimum files needed for the CLI contract, such as the generated output schema and final-output target.

Audit execution must never mutate the selected project or any Git repository.

## Structured result

Use the existing H!veAI audit result schema.

Preferred current CLI pattern, if supported by the installed CLI, is equivalent to:

```text
codex exec
  --ephemeral
  --json
  --output-schema <schema-file>
  --output-last-message <final-output-file>
  --sandbox read-only
  --skip-git-repo-check
  ...safe fixed config...
```

This example is an intent, not permission to blindly hard-code flags without checking the installed CLI contract.

Do not parse the first/intermediate JSONL `agent_message` as the audit result.

Treat the dedicated final-message/output file as the candidate final result when supported, then independently parse and validate it with the existing Rust schema and semantic validator.

Keep stderr separate from JSONL parsing. Do not merge stderr into the JSON protocol stream.

If the CLI completes with a nonzero exit, timeout, missing final result, invalid JSON, schema-invalid result, or semantically invalid result, produce a truthful degraded/non-PASS audit state.

## Evidence prompt

The prompt passed to Codex must explicitly state:

- the supplied JSON evidence is authoritative;
- builder logs are claims, not proof;
- do not inspect the filesystem, Git repository, web, MCP, plugins, or unrelated context;
- do not modify anything;
- return only the required structured audit result;
- never fabricate PASS when evidence is unavailable or contradictory.

---

# PHASE 5 — TRUSTED AUDIT PROVENANCE

Execution provenance comes from H!veAI runtime authority, never generated text.

Persist:

- `auditor_provider = CODEX_CLI`;
- `auditor_version =` exact bounded `codex --version` value for the executable used;
- `auditor_model =` explicit model passed by H!veAI if one is intentionally configured, otherwise `CLI_DEFAULT` or another truthful constant making no unsupported exact-model claim.

Do not ask the model to report its own execution identity for persistence.

Do not parse/scrape interactive Codex TUI output to infer model identity.

Historical audit rows remain readable and are not rewritten.

---

# PHASE 6 — SETTINGS UX BECOMES CODEX-ONLY

Replace the active Settings `GPT Audit Provider` / OpenAI API-provider configuration with a clear **Codex Audit Provider** panel.

Show only non-secret information such as:

- Provider: `Codex CLI`;
- CLI availability;
- CLI version;
- Login state (`ChatGPT authenticated`, `not logged in`, `unsupported API-key auth`, etc.);
- Readiness state;
- concise safe diagnostic category;
- `Check readiness` button.

There must be no:

- API-key field;
- `OPENAI_API_KEY` setup instruction;
- OpenAI HTTP model readiness control;
- API billing language;
- API-provider model text field.

If model selection remains, it must be explicitly optional Codex-CLI-native behavior. The default product behavior should use the CLI/account default and should not pin an obsolete model slug.

The UI must make it clear that H!veAI uses the local Codex installation and the login managed by Codex itself.

---

# PHASE 7 — PRESERVE ACCEPTED LOCAL WORKSPACE BEHAVIOR

Do not regress M16S local-workspace work.

The Projects screen must retain visible:

- Attach local workspace;
- Change local workspace;
- Repair local workspace.

The existing H!veAI logical project must remain bound to its standalone local checkout without duplication.

Portfolio count must remain exactly eight.

Do not redesign Project Registry in M16T.

---

# PHASE 8 — TESTING

Use injected/mocked process execution for automated tests. Automated tests MUST NOT consume the owner's real Codex quota and MUST NOT depend on real ChatGPT authentication.

At minimum add direct deterministic tests for:

1. shared native Codex executable resolution;
2. missing executable => truthful unavailable state;
3. bounded version probe;
4. `codex login status` ChatGPT-auth success classification;
5. not-logged-in classification;
6. API-key-auth => `AUTH_POLICY_BLOCKED`;
7. explicit readiness probe READY;
8. readiness auth failure;
9. readiness usage/rate-limit failure;
10. readiness network/process failure;
11. readiness timeout and child termination;
12. readiness does not create an audit row;
13. exact safe fixed audit argument policy;
14. no dangerous sandbox bypass flag;
15. read-only audit sandbox policy;
16. dedicated temp audit working directory;
17. strict output-schema handoff;
18. final-output file is authoritative over intermediate JSONL messages;
19. stderr cannot be consumed as JSONL protocol events;
20. process nonzero exit => non-PASS;
21. timeout => non-PASS;
22. missing final output => non-PASS;
23. malformed JSON => non-PASS;
24. schema-invalid result => non-PASS;
25. semantic-invalid result => non-PASS;
26. trusted `CODEX_CLI` / CLI version / model provenance;
27. persisted audit round-trip;
28. historical audit rows remain readable;
29. Settings Codex-only provider states and no API-key UX;
30. active production source contains no `OPENAI_API_KEY` audit path;
31. local workspace controls remain visible;
32. eight-project portfolio invariant.

Then run the required regressions:

- focused Rust audit/Codex runtime tests;
- focused frontend Settings/Audit/Projects tests;
- existing Codex adapter tests;
- existing audit engine tests;
- registry/local-workspace tests;
- full frontend test suite;
- full Rust test suite under repository policy;
- TypeScript typecheck;
- production frontend build;
- `git diff --check`;
- governed native QA publication via the existing safe publisher;
- stable Desktop shortcut/icon validation;
- no terminal-flash regression.

After all source tests pass, do NOT run a real owner Codex audit automatically unless the prompt explicitly has safe access to the owner's interactive acceptance context. The real Codex turn is an OWNER native acceptance gate after independent strict source audit PASS.

---

# PHASE 9 — ACTIVE-SOURCE ABSENCE PROOF

Before completion, perform and log a scoped search over active production/current prospective files.

Exclude historical immutable prompt/log/audit provenance from this absence check.

Prove there is no operative production audit path using:

- `OPENAI_API_KEY`;
- `OpenAiAuditModel`;
- direct OpenAI Responses HTTP transport;
- OpenAI model-endpoint readiness;
- API-key setup UI/copy.

Also inspect whether `reqwest` remains used anywhere else in active production. Remove the dependency only if it is truly unused.

The builder log must list exactly what was removed and what intentionally remains historical.

---

# PHASE 10 — TRACKER COMPLETION STATE

After implementation/tests/publication succeed:

Update current/prospective `TASKS.md` and `CODEX_ROADMAP.md` to:

- M16 remains OPEN;
- M16T V01 = `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`;
- Required Actor = HUMAN;
- M16S V02 = source-audit PASS but superseded before final owner provider acceptance by the Codex-only owner decision;
- M17 remains NOT ACTIVATED/BLOCKED;
- M21 chain remains PASS/CLOSED;
- progress remains 16/20 until final M16 acceptance.

Do not mark M16 PASS/CLOSED. That occurs only after independent ChatGPT audit and owner-native real Codex acceptance.

---

# PROHIBITED SHORTCUTS

Do not:

- retain the API-key provider disabled behind a hidden switch;
- use `OPENAI_API_KEY` for Codex or audit readiness;
- inspect Codex auth files/tokens;
- implement GUI automation against ChatGPT.exe;
- use browser JavaScript to launch model requests;
- shell through `cmd`, PowerShell, Python, Node, or curl for audit execution;
- use dangerous Codex bypass flags;
- allow Codex audit execution to mutate project files;
- parse the first JSONL assistant message as the final audit result;
- merge stderr into JSONL parsing;
- invent an exact model name that runtime cannot prove;
- hard-code a stale model slug merely because historical code did;
- create a second audit-result schema;
- weaken existing audit evidence/freshness/provenance checks;
- create a ninth project;
- modify another GitHub repository;
- activate M17 or implement Claude;
- rewrite historical immutable M16S artifacts.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V01_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- tracker-transition commit SHA;
- implementation commit SHA(s) known before log publication;
- exact active OpenAI API-provider code/settings/dependencies removed;
- exact shared Codex runtime primitives reused/extracted;
- installed/current Codex CLI command contract inspected;
- fixed audit process argument/sandbox/output policy;
- auth-policy behavior for ChatGPT auth vs API-key auth;
- exact readiness state model;
- final-output authority design;
- timeout/termination design;
- provider/model/version provenance policy;
- active-source absence-search results;
- focused test commands/counts;
- full frontend/Rust/typecheck/build/diff-check results;
- governed native publication evidence;
- explicit confirmation that no real owner API key was requested/read/stored and no real owner Codex quota was consumed by automated tests;
- explicit statement that M17 remains blocked and no Claude implementation was done.

Do not require the log file to contain the SHA of the commit that first creates itself. After publishing the log, verify its commit and final remote `main` and return those SHA values in the final response.

---

# OWNER NATIVE ACCEPTANCE AFTER INDEPENDENT AUDIT

Do not claim M16 closed from builder evidence.

After independent source audit PASS, the owner will manually verify:

1. Settings shows `Codex Audit Provider` and no OpenAI API-key configuration exists.
2. No API key is requested anywhere.
3. H!veAI identifies the local Codex CLI/version.
4. Codex auth is the owner's existing ChatGPT login, not API-key auth.
5. Explicit `Check readiness` succeeds end-to-end.
6. A real Audit Center run completes through local Codex CLI.
7. Persisted result uses `CODEX_CLI` runtime provenance and a truthful model value.
8. Strict verdict/findings/coverage are populated without provider fallback.
9. No terminal window flashes visibly.
10. Projects remain eight and H!veAI local workspace remains attached.

Only then may M16 become PASS/CLOSED and M17 become the next active milestone.

---

# COMPLETION GATE

Return `COMPLETE` only when all M16T source/test/publication requirements pass and GitHub synchronization is proven.

Return `SYNC_BLOCKED` if safe Git synchronization cannot be completed.

Return `CODEX_RUNTIME_BLOCKED` if the installed supported Codex CLI cannot provide a safe bounded headless audit path without violating this contract. Explain the exact blocker and do not reintroduce an API-key HTTP provider as fallback.

Final owner-facing response must show only:

- GitHub M16T log URL/path;
- tracker-transition commit SHA;
- implementation commit SHA(s);
- M16T log commit SHA;
- final GitHub `main` SHA;
- concise status.
