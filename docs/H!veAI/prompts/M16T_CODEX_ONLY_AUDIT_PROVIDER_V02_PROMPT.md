# M16T Codex-Only Audit Provider V02 Remediation

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
- `docs/H!veAI/prompts/M16T_CODEX_ONLY_AUDIT_PROVIDER_V01_PROMPT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V01_LOG.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V01_AUDIT.md`
- `src-tauri/src/codex_runtime.rs`
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
- Version: `V02`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V02_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

This is a **narrow remediation** of the accepted M16T Codex-only architecture. Do not redesign the provider architecture.

The V01 architecture remains binding:

> **H!veAI has no OpenAI API-key/direct-HTTP audit provider. The only production model-backed audit provider is the locally installed Codex CLI authenticated through Codex-managed ChatGPT login.**

Do not activate M17 or implement Claude work.

---

# ABSOLUTE ARCHITECTURE GUARDRAIL

V02 must not reintroduce any of the following into active production code:

- `OPENAI_API_KEY` as a product configuration path;
- direct H!veAI calls to `api.openai.com` for audit execution/readiness;
- OpenAI Responses API transport in the H!veAI audit engine;
- API-key UI/setup instructions;
- API-key authentication as an accepted Codex audit mode;
- Codex auth-file/token inspection;
- ChatGPT desktop GUI automation;
- terminal/TUI scraping;
- a hard-coded audit model override where the accepted architecture uses Codex CLI default model selection.

Historical immutable M16S/M16T V01 artifacts remain unchanged even if they contain historical API terminology.

---

# FINDING M16T-V02-F01 — MAJOR — SAFE BOUNDED RETENTION MUST CONTINUE DRAINING CHILD STREAMS

## Current defect

`src-tauri/src/codex_runtime.rs::read_bounded` currently stops reading the child pipe when retained output reaches the configured maximum.

That behavior bounds retained memory, but it does **not** safely bound process capture. After the reader thread exits, a verbose Codex process can continue writing JSON events or diagnostics until the OS pipe fills. The child can then block before normal exit/final-output completion, while the parent waits for exit until timeout and may report a false timeout.

This is particularly important because audit execution uses `codex exec --json`: generic operational output can legitimately be much larger than the small retained diagnostic prefix even when the dedicated final-message result is valid.

## Required implementation

Change the shared bounded reader semantics so that:

1. at most `max` bytes are retained in memory;
2. after the retained limit is reached, the reader **continues reading/draining to EOF** and discards additional bytes;
3. `truncated` becomes true as soon as data exists beyond the retained cap;
4. no unbounded accumulation occurs;
5. stdout and stderr continue to drain concurrently;
6. timeout/kill behavior remains bounded and deterministic;
7. zero-byte and exact-boundary behavior remains correct;
8. malformed I/O still degrades truthfully.

Do not solve this by increasing limits to huge values.

Do not remove output bounding.

## Dedicated final-message authority

The accepted M16T architecture says the dedicated `--output-last-message` file is the authoritative model result. Generic `--json` stdout/stderr are bounded operational evidence only.

Therefore, audit evaluation must **not fail solely because generic stdout or stderr was truncated** when all of the following are true:

- process completed successfully;
- no timeout occurred;
- dedicated final-message output exists;
- dedicated final-message output is within its own strict size bound;
- final JSON satisfies the existing audit schema and semantic/freshness validation.

Operational truncation should remain visible as diagnostics/metadata where appropriate, but it must not override a valid authoritative final result.

For explicit readiness, use the same principle carefully: the dedicated final output (`READY`) is authoritative if the process otherwise completed successfully. Do not silently ignore a non-zero exit, timeout, missing final output, or malformed final output.

## Required direct tests

Add deterministic tests that prove at minimum:

1. a child writes more than the stdout retention cap and then exits normally; parent returns before timeout, retained stdout length stays `<= cap`, and `stdout_truncated == true`;
2. the same for stderr;
3. stdout and stderr both exceed their caps concurrently and the child still exits normally without deadlock;
4. output exactly at the cap is not falsely marked truncated unless additional data exists;
5. timeout still kills/reaps the child and joins drain threads cleanly;
6. an audit mock/process result with truncated operational stdout/stderr plus a valid bounded dedicated final result still produces the model evaluation rather than `AUDIT_CODEX_TRANSPORT_TRUNCATED`;
7. truncated/missing/oversized **dedicated final output** remains a truthful failure;
8. non-zero exit remains a truthful failure regardless of operational output.

Use deterministic process/test fixtures. Do not consume owner Codex quota in automated tests.

---

# FINDING M16T-V02-F02 — MAJOR — RECONCILE CANONICAL CURRENT TRACKER TRUTH

## Current defect

After the V01 completion-tracker commit and builder log, current tracking contradicts itself.

The top of root `TASKS.md` still says:

- `Current Task Status: IMPLEMENTATION_IN_PROGRESS`;
- next action is to perform implementation;
- `Required Actor: CODEX`;
- M16T task row says implementation in progress.

But the deeper current-truth text and V01 builder log correctly say implementation is complete and awaiting independent audit plus owner native acceptance.

`CODEX_ROADMAP.md` top current status also still labels M16T active with Required Actor CODEX.

Because root `TASKS.md` is the canonical source consumed by H!veAI, this is not merely documentation polish. It can surface wrong next-action/actor truth in the product.

## Required target state after V02 builder completion

After the V02 implementation and tests are complete, current/prospective tracker truth must say:

- Current Milestone: M16;
- Current Sprint: M16T-CODEX-ONLY;
- Current Task: M16T V02 — Codex-only audit provider remediation;
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`;
- Next Task/Action: independent M16T V02 strict audit and owner native Codex acceptance;
- Required Actor: `HUMAN`;
- M16T task row: implementation complete, awaiting independent audit/native acceptance;
- M16 remains OPEN;
- M17 remains NOT ACTIVATED/BLOCKED;
- milestone denominator remains 20;
- M21 accepted history remains unchanged.

Update `CODEX_ROADMAP.md` current status to match the same prospective truth.

Do not rewrite immutable historical prompts/logs/audits just to change old statuses.

---

# REQUIRED VALIDATION

Run the narrowest focused tests first, then required regressions.

At minimum:

1. new shared runtime over-cap drain stress tests;
2. audit final-message authority tests covering generic stream truncation;
3. existing M16T provider argument/auth/readiness/privacy tests;
4. existing audit schema/semantic/freshness/persistence tests;
5. existing Codex adapter tests because `codex_runtime.rs` is shared foundation;
6. existing M16/M16S frontend provider tests;
7. full frontend suite;
8. relevant/full Rust library suite under repository policy;
9. `npm run typecheck`;
10. production frontend/native build as required by repository policy;
11. `git diff --check`;
12. active-source/current-tracker forbidden-provider search proving no OpenAI API-key/direct HTTP audit path has returned;
13. governed native QA publication via the existing safe publisher;
14. stable Desktop shortcut target/icon and no terminal-flash regression.

Builder test output is evidence/claim, not independent acceptance. Record exact commands and counts in the V02 log.

Do not use a real owner API key. There is no accepted API-key path.

Automated tests must not consume a real Codex model turn/quota. Use deterministic mock/process fixtures. Host-native real Codex readiness/audit remains an OWNER acceptance step after independent V02 source audit PASS.

---

# PROHIBITED SHORTCUTS

Do not:

- merely raise stdout/stderr size caps;
- stop reading a pipe when retained memory reaches the cap;
- create unbounded buffers;
- mark a valid authoritative final result failed solely because generic operational output was truncated;
- ignore non-zero exit, timeout, missing final output, oversized final output, malformed schema, semantic failure or freshness failure;
- remove `--sandbox read-only`;
- remove `--ephemeral`;
- remove `--ignore-user-config` or `--ignore-rules` from the audit path without a new independent architecture decision;
- add a shell-mediated production launch path;
- restore `OPENAI_API_KEY`, `reqwest` audit transport, OpenAI HTTP readiness, or API-key UI;
- read Codex auth files;
- modify another GitHub repository;
- activate M17 or implement Claude;
- rewrite V01 historical artifacts.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V02_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- V02 implementation commit SHA(s) known before log publication;
- exact source symbols changed;
- explanation of bounded retention vs continuous drain semantics;
- direct over-cap stdout/stderr test evidence and retained sizes/truncation flags;
- proof that valid dedicated final-message output remains authoritative despite generic stream truncation;
- proof that final-message truncation/absence/non-zero exit still fails truthfully;
- canonical tracker/roadmap final state;
- forbidden-provider search result;
- focused/full test commands and counts;
- typecheck/build/diff-check result;
- native publication evidence;
- statement that no API-key/direct HTTP audit provider was reintroduced;
- statement that M17 remains blocked and no Claude implementation was performed.

Do not require the log file to contain the SHA of the commit that first creates itself. After publishing the log, verify the log commit and final remote `main` and return those SHA values in the final response.

---

# COMPLETION GATE

Return `COMPLETE` only when both V01 findings are closed, all required regressions/publication gates pass, current tracker truth is reconciled, and GitHub synchronization is proven.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Return `RUNTIME_REMEDIATION_BLOCKED` if safe bounded continuous draining cannot be implemented without violating the existing process/security architecture. Do not weaken the security boundary to force completion.

Final owner-facing response must show only:

- GitHub V02 log URL/path;
- implementation commit SHA(s);
- V02 log commit SHA;
- final GitHub `main` SHA;
- concise status.
