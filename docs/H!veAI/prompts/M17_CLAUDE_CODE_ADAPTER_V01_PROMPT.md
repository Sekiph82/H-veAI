# M17 Claude Code Adapter V01 — Whole-Milestone Implementation Prompt

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
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16T_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V11_STRICT_AUDIT.md`
- `docs/H!veAI/plans/M17_CLAUDE_CODE_ADAPTER_V01_PLAN.md`
- `src-tauri/src/codex_adapter.rs`
- `src-tauri/src/codex_runtime.rs`
- `src-tauri/src/agent_session_center.rs`
- `src-tauri/src/final_response.rs`
- `src-tauri/src/stream_sanitizer.rs`
- `src-tauri/src/process_policy.rs`
- relevant DB migrations/schema for `agent_sessions` and `agent_events`
- Tauri command registration/capability ACL for Agents/session commands
- current Agents/session frontend components and focused tests.

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

- Work code: `M17`
- Version: `V01`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required plan: `docs/H!veAI/plans/M17_CLAUDE_CODE_ADAPTER_V01_PLAN.md`
- Required log: `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V01_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

M16 has passed independent strict source audit and final owner-native acceptance. The final M16T owner acceptance proves Codex CLI readiness and three consecutive native `COMPLETED / AVAILABLE` freeform audits on an unchanged audited HEAD. M16 is therefore eligible to close and M17 is eligible to activate.

---

# FIRST ACTION — CANONICAL TRACKER TRANSITION

Before production implementation, reconcile prospective tracker truth from the accepted M16 closure into M17 activation.

Update `TASKS.md` and `CODEX_ROADMAP.md` so current prospective truth is exactly:

- Current Milestone: `M17`
- Current Sprint: `M17-CLAUDE-ADAPTER`
- Current Task: `M17 V01 — Claude Code Adapter`
- Current Task Status: `IMPLEMENTATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16: `PASS/CLOSED`
- M16T: validated complete / `[x]`
- strict completed milestone count: `17 / 20 = 85%`
- M17: `[~]` active
- M18-M20: remain planned/blocked by dependency order.

Do not rewrite immutable historical M16 prompts/logs/audits. Historical package notes that explicitly describe old states may remain historical if clearly labeled as such, but current/prospective truth at the top of the canonical tracker and roadmap must no longer say M16 is open or M17 is blocked.

Commit and push this tracker transition before substantive M17 implementation so GitHub remains the source of truth throughout the work.

Do not mark M17 `[x]` during builder implementation. `[x]` means validated complete and requires later independent audit plus owner acceptance.

---

# ABSOLUTE ARCHITECTURE GUARDRAILS

Preserve all accepted M00-M16 behavior, especially:

- standalone `Sekiph82/H-veAI` root on `main`;
- exact eight-project GitHub portfolio;
- `Sekiph82/FormuLab@main` tracking;
- GitHub root `TASKS.md` as canonical task authority;
- existing Codex engineering adapter behavior;
- M16 Codex-only audit provider;
- local Codex ChatGPT-managed authentication;
- no `OPENAI_API_KEY` and no direct OpenAI HTTP/Responses audit provider;
- bounded process/output persistence;
- stream redaction before persistence/UI exposure;
- native Tauri ACL least privilege;
- stable development publication at `dev-bin/H!veAI.exe` and Desktop shortcut.

Claude integration is **local Claude Code CLI only**.

Forbidden:

- `ANTHROPIC_API_KEY` as a required or fallback provider path;
- direct Anthropic HTTP/API transport;
- reading Claude credential/token/auth files;
- Claude desktop GUI automation;
- browser automation to drive Claude;
- auto-installing or auto-updating Claude Code;
- shell command-string construction;
- `--dangerously-skip-permissions` or equivalent blanket permission bypass;
- cross-project or cross-task session resume;
- fake READY/completed/waiting states;
- activating M18 before M17 independent closure.

The installed Claude binary's bounded `--version` and `--help` output is the runtime authority for supported CLI flags. Do not assume that provisional flags already in the repository are valid. Current official Claude Code documentation describes non-interactive `--print`, `--output-format stream-json`, `--resume`, `--continue`, and `--permission-mode`, but the installed executable must be checked before using any flag.

---

# IMPORTANT EXISTING PROVISIONAL CLAUDE PATH

`src-tauri/src/agent_session_center.rs` already contains a provisional Claude implementation. Treat it as unaccepted source material and audit it adversarially before reusing it.

Known current contradictions/gaps include at least:

- `claude_readiness()` verifies version but leaves authentication unknown;
- the provisional process args include `--no-session-persistence` while M17 requires real resume/continue;
- `AgentSession.supports_resume` is always false;
- `resume()` always returns `RESUME_UNSUPPORTED`;
- provider-neutral `AgentAdapter` / `AgentProvider` ownership is still Codex-centric;
- Claude lifecycle/process ownership is implemented separately in `AgentSessionCenter` rather than through a fully neutral adapter contract;
- permission/wait/auth/quota/provider states are not yet a complete Claude state machine;
- Claude provider-native session identity is not durably modeled for deterministic resume;
- the provisional stop path escalates through `taskkill.exe` without the same mature ownership/grace semantics as the Codex path;
- provisional fixed args include flags that may be stale or unsupported in the installed Claude version.

Do not preserve any of these merely because tests currently cover them.

---

# M17.01 — CLAUDE AVAILABILITY / READINESS

Implement a bounded, truthful Claude Code readiness boundary.

## Executable resolution

- Discover only directly invokable safe candidates.
- On Windows, prefer a native Claude executable path when available.
- Reject unsafe wrappers or shell-only shims if invoking them would require `cmd.exe`, PowerShell, or shell-string construction.
- Record skipped invalid candidates diagnostically without leaking sensitive paths unnecessarily.
- Do not auto-install Claude.

## Version/capability probe

Use bounded direct process probes for:

- `--version`;
- `--help` or another bounded capability surface needed to prove supported flags.

Do not hard-code a minimum version unless a specific tested capability requires it. Prefer capability detection over arbitrary version gates.

## Authentication/readiness

Never inspect Claude auth files.

If the installed CLI exposes a stable bounded auth-status command, use it. Otherwise implement a harmless bounded end-to-end print-mode readiness turn in a temporary directory with mutation-capable tools disabled/restricted so the probe cannot modify a registered project.

Readiness must distinguish at least:

- `READY`
- `EXECUTABLE_NOT_FOUND`
- `VERSION_PROBE_FAILED` / `VERSION_UNSUPPORTED` when applicable
- `AUTH_REQUIRED`
- `AUTH_UNVERIFIED`
- `USAGE_LIMITED`
- `NETWORK_ERROR`
- `TIMEOUT`
- `PROCESS_ERROR`

Only explicit evidence may produce `USAGE_LIMITED`, `AUTH_REQUIRED`, or `READY`.

A readiness check must not create an engineering session or mutate project files.

---

# M17.02 — TRUE PROVIDER-NEUTRAL ADAPTER CONTRACT

The current `AgentAdapter` trait and `AgentProvider` enum live in `codex_adapter.rs`, while `AgentSessionCenter` contains parallel Claude behavior. Refactor only as much as required to create a genuinely provider-neutral contract.

A suitable end state may introduce a neutral module such as `agent_adapter.rs`, but the exact file layout is yours to determine from repository architecture.

The neutral contract must support at minimum:

- provider identity;
- readiness/capabilities;
- start;
- list/get;
- stop;
- resume/continue where provider-supported;
- reconcile after H!veAI restart;
- bounded stream/final-response persistence;
- provider-specific session identity/provenance.

Preserve public Tauri/API compatibility where practical. Do not rewrite the mature Codex adapter merely for symmetry.

Codex regressions are blockers.

---

# M17.03 — START / CONTINUE / RESUME / STOP

## Start

Before spawning Claude:

- validate provider;
- validate bounded prompt;
- validate project is ACTIVE;
- canonicalize registered project/worktree cwd;
- validate task belongs to the same project;
- persist deterministic H!veAI session provenance.

Start Claude only in the canonical registered cwd.

Use the installed CLI's verified non-interactive structured-output mode. Current official documentation describes `--print` / `-p` and `--output-format stream-json`; verify them against the installed CLI before use.

Pass prompts through bounded stdin or another argument-safe mechanism. Never interpolate prompts into shell command strings.

## Provider-native session identity

Capture Claude's authoritative provider session ID from structured output and persist it durably.

If the current DB does not model provider-native session identity, add a forward-only migration with compatibility for existing Codex and historical Claude rows.

The persisted identity must be enough to prove:

- provider;
- H!veAI session ID;
- provider-native Claude session ID;
- project ID;
- optional task ID;
- canonical cwd identity;
- provider version/capability context where useful.

Do not treat the Claude provider session ID as a credential.

## Resume / continue

Implement deterministic provider-backed continuation.

- Resume must use the exact persisted Claude provider session ID.
- A resume request must prove same provider, project, task relationship, and canonical cwd boundary.
- Never use a global "most recent Claude conversation" as a substitute for exact identity when H!veAI already has a persisted session ID.
- If a `continue` UX is provided, define it deterministically as continuing the latest eligible H!veAI-owned Claude session for the selected project/task, not the latest arbitrary Claude conversation on the machine.
- Remove or stop using `--no-session-persistence` if it makes resume impossible, unless the installed CLI provides another verified resumability mechanism.
- If the installed Claude version cannot support safe deterministic resume, expose that capability as false and do not fabricate it; however M17 cannot close until the roadmap's required resume/continue behavior is implemented on the supported owner environment.

## Stop

Stop only an H!veAI-owned process.

- attempt a safe/graceful owned-process termination path first where supported;
- use bounded escalation only if required;
- never kill unrelated Claude processes;
- persist STOP_REQUESTED / STOPPED / escalation truthfully.

Reuse mature Codex stop/process-policy primitives when that reduces duplicated risk without breaking Codex.

---

# M17.04 — STRUCTURED STREAM MAPPING

Do not treat Claude `stream-json` as opaque stdout.

Implement a bounded parser that maps provider records into H!veAI session events while retaining sanitized bounded raw evidence as needed.

At minimum recognize and persist truthful evidence for:

- provider/session metadata including Claude session ID;
- assistant text/final response;
- tool-use start/result where the installed stream format emits them;
- permission/wait/attention events where observable;
- auth/rate/quota/provider errors;
- stdout/stderr;
- process exit;
- truncation/degradation.

Malformed JSON/event records must not silently become successful completion. Use explicit diagnostics and fail/degrade truthfully.

Final assistant response must be captured through the shared `FinalResponseCapture` contract or an equivalently governed neutral path. Do not allow arbitrary progress events to masquerade as the final answer.

Streams must always continue draining even after persistence/display caps are reached so child processes cannot deadlock on full pipes.

---

# M17.05 — PERMISSION / WAIT / ATTENTION DETECTION

Never use `--dangerously-skip-permissions`.

Inspect installed Claude capabilities and implement the safest programmatic permission strategy compatible with H!veAI.

Required truthful session states/attention signals where observable:

- `RUNNING`
- `WAITING_PERMISSION`
- `WAITING_USER`
- `AUTH_REQUIRED`
- `USAGE_LIMITED`
- `NETWORK_ERROR`
- `STOPPING`
- `STOPPED`
- `COMPLETED`
- `FAILED`
- `CRASHED` / `ORPHANED`

If Claude cannot perform a requested action in non-interactive mode because permission is required, do not call the session COMPLETED. Surface actionable attention with a bounded diagnostic.

Do not broaden filesystem access outside the registered project/worktree merely to avoid permission friction.

---

# M17.06 — CRASH / ORPHAN / RESTART RECOVERY

On application startup/reconcile:

- no stale Claude session may remain falsely RUNNING;
- distinguish process orphan/crash from clean completion;
- preserve provider-native session identity even after the OS process is gone;
- allow a later explicit owner resume where eligibility checks pass;
- never auto-resume or spawn Claude during reconciliation;
- preserve control-plane truth-dirty/materialization contracts.

A previously crashed/orphaned H!veAI process row may still be resumable at the Claude conversation level if provider identity is valid. Model these as separate truths rather than conflating process ownership with provider-session resumability.

---

# M17.07 — SECURITY / PROCESS TESTS

Add deterministic tests that consume no live Claude quota for the core parser/lifecycle logic.

At minimum prove:

1. executable resolution rejects unsafe/non-native candidates according to policy;
2. version/capability probe is bounded;
3. readiness classifiers distinguish auth/quota/network/process/timeout states from generic help/progress text;
4. readiness creates no engineering session row;
5. project cwd containment and ACTIVE-project requirement;
6. task/project identity enforcement;
7. prompt/argument injection resistance;
8. structured stream parser maps representative Claude events correctly;
9. malformed stream records do not become successful completion;
10. stream persistence/output bounds do not stop pipe draining;
11. secret redaction occurs before persisted/UI stream exposure;
12. provider-native session ID is captured and persisted;
13. resume rejects cross-project, cross-task, wrong-provider, missing-ID, and cwd mismatch cases;
14. exact-session resume uses the persisted Claude session ID;
15. continue selection is deterministic and project-scoped if implemented;
16. permission/wait mapping is truthful;
17. explicit quota -> `USAGE_LIMITED` without false positives;
18. stop targets only owned PID/process tree and records escalation truthfully;
19. application restart reconciliation marks lost processes truthfully without auto-spawning;
20. eligible provider session remains explicitly resumable after process reconciliation;
21. Codex adapter/start/stop/list/final-response regressions remain green;
22. M16 audit-provider readiness and audit regressions remain green;
23. V05 FormuLab@main and exact-eight-project regressions remain green.

Use fixtures captured from documented/observed Claude event shapes, but keep fixtures small and credential-free.

---

# M17.08 — FRONTEND / NATIVE UX / CLOSURE EVIDENCE

Update the existing Agents/session UI rather than creating a disconnected demo page.

The user must be able to see, for Claude:

- installed/unavailable state;
- version;
- readiness/auth state;
- capabilities including resume support;
- running/attention/completed/failed session state;
- project/task identity;
- bounded stream/events;
- final assistant response;
- stop action when owned and running;
- resume/continue action only when valid;
- actionable diagnostic when auth/quota/permission/network/provider failure occurs.

Do not display secrets, auth tokens, credential paths, or raw sensitive environment data.

No fake placeholder data is acceptable.

---

# REQUIRED LIVE / OWNER-ENVIRONMENT BUILDER CHECKS

Builder may perform bounded live probes against the owner's installed Claude Code only when they are non-destructive and do not fabricate owner acceptance.

At minimum, if Claude is installed on the owner environment:

- record `claude --version` output;
- inspect bounded `claude --help` capability evidence;
- verify readiness classification without reading auth files;
- run one harmless non-mutating temporary-directory readiness turn if required by the implementation.

Do not run an uncontrolled code-changing live Claude session merely to prove the adapter during builder execution.

Owner native acceptance occurs only after independent strict audit.

If Claude Code is not installed/authenticated, implementation may still complete on deterministic fixtures, but native availability must remain truthful and final M17 closure will require later HUMAN environment acceptance.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused M17 adapter/readiness tests;
2. focused Claude structured-stream/parser tests;
3. resume/continue identity tests;
4. permission/wait/quota/error classification tests;
5. crash/orphan/reconcile tests;
6. Codex adapter regressions;
7. M16 audit-provider regressions including readiness and freeform schema behavior;
8. Agent Session Center regressions;
9. Tauri command/ACL tests for any new commands;
10. frontend Agents/session focused tests;
11. exact eight-project and FormuLab@main tests;
12. full frontend Vitest regression;
13. full Rust library regression under repository policy;
14. `npm run typecheck`;
15. `cargo check --manifest-path src-tauri/Cargo.toml`;
16. `npm run build`;
17. `git diff --check`;
18. active-source guardrail scan proving no `ANTHROPIC_API_KEY`, direct Anthropic HTTP provider, Claude auth-file inspection, GUI automation, or blanket permission bypass was introduced;
19. existing guardrail scan proving M16 still has no `OPENAI_API_KEY` / direct OpenAI HTTP audit provider;
20. governed native QA publication;
21. stable Desktop shortcut/no-terminal-flash publication checks.

Builder execution output is claim evidence until independently audited.

---

# TRACKER GOVERNANCE AT BUILDER COMPLETION

If and only if implementation/test/publication gates pass, update current/prospective truth to:

- Current Milestone: `M17`
- Current Sprint: `M17-CLAUDE-ADAPTER`
- Current Task: `M17 V01 — Claude Code Adapter`
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Required Actor: `HUMAN`
- Next action: independent M17 strict audit, then owner native Claude readiness/start/resume/stop acceptance if source audit passes
- strict completed milestone count remains `17 / 20 = 85%` because M17 itself is not yet independently closed
- M17 marker remains `[~]`
- M18 remains NOT ACTIVATED/BLOCKED.

Do not mark M17 `[x]` before independent audit plus HUMAN acceptance.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V01_LOG.md`

Include:

- synchronized starting SHA;
- tracker-transition SHA;
- implementation SHA(s);
- exact existing provisional Claude behavior audited/replaced/preserved;
- installed Claude version/help/capability evidence if available;
- auth/readiness strategy and proof no auth-file inspection/API-key fallback exists;
- provider-neutral adapter refactor summary;
- provider-native session-ID persistence design;
- start/resume/continue/stop semantics;
- structured stream event mapping;
- permission/wait/quota/error classifier behavior;
- crash/orphan/restart recovery behavior;
- DB migration details if any;
- Tauri ACL changes if any;
- focused and full test/build counts/commands;
- M00-M16 regression evidence, especially Codex adapter and M16 audit provider;
- forbidden-provider/auth/GUI/bypass guardrail scans;
- governed publication EXE SHA-256 and shortcut evidence;
- final tracker state;
- final local HEAD / origin-main / live-main equality.

Do not require the log to contain the SHA of the commit that first creates itself. Commit the immutable log separately after implementation/publication evidence is complete, then verify remote equality again.

---

# INDEPENDENT / OWNER CLOSURE GATE

Do not fabricate M17 acceptance.

After builder completion:

1. ChatGPT performs the independent 20-section strict M17 audit.
2. Only if that source audit passes does the owner perform native Claude acceptance.
3. Native acceptance should prove installed/authenticated readiness, real project-scoped start, structured stream/final response, stop, exact-session resume/continue, and truthful attention/failure behavior without cross-project leakage.
4. M17 remains OPEN until both independent audit and owner-native acceptance pass.
5. M18 remains blocked until M17 closes.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- canonical tracker transition to M17 is committed/pushed;
- M16 remains PASS/CLOSED and M16T remains validated complete;
- Claude local-CLI-only boundary is preserved;
- no Anthropic API-key/direct HTTP provider exists;
- provider-neutral adapter contract supports Codex and Claude without Codex regression;
- Claude readiness is bounded and truthful;
- start/list/stop/resume/continue contracts are implemented as required;
- exact provider session identity is durable and project/task scoped;
- structured stream mapping is bounded, redacted, and fail-closed;
- permission/wait/quota/network/provider states are truthful;
- crash/orphan recovery is truthful and does not auto-resume;
- required deterministic tests and full regressions pass;
- governed native QA publication succeeds;
- every repository change is committed and pushed;
- local HEAD == origin/main == live main;
- worktree is clean;
- M17 remains `[~]` awaiting independent strict audit and owner acceptance;
- M18 remains blocked.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must contain only:

- GitHub M17 log URL/path;
- tracker-transition commit SHA;
- implementation commit SHA(s);
- M17 log commit SHA;
- final GitHub main SHA;
- concise status.