# M17 Claude Code Adapter V01 Plan

## Purpose

M17 makes locally installed Claude Code a first-class H!veAI engineering-agent provider while preserving the accepted Codex provider, M16 Codex-only audit architecture, project/task provenance, bounded process execution, native Windows UX, and GitHub-first governance.

M16 is accepted for closure by `docs/H!veAI/audits/M16T_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`. M17 is therefore eligible to activate. M18-M20 remain blocked by roadmap order.

## Non-negotiable provider boundary

Claude integration is local CLI only.

- Use the locally installed Claude Code executable and the owner's Claude-managed login/session.
- Do not require, read, set, persist, or use `ANTHROPIC_API_KEY`.
- Do not add a direct Anthropic HTTP/API transport as a fallback.
- Do not inspect Claude credential/token/auth files.
- Do not automate the Claude desktop GUI.
- Do not auto-install or auto-update Claude Code.
- Detect executable/version/capabilities truthfully and degrade safely when unavailable.

The installed Claude CLI's bounded `--help`/`--version` output is the runtime authority for supported flags. Official documentation may guide implementation, but H!veAI must not blindly hard-code stale flags.

## Existing provisional Claude code

`src-tauri/src/agent_session_center.rs` already contains a provisional Claude path. It is source material, not accepted M17 completion.

Known gaps that M17 must explicitly resolve rather than preserve accidentally include:

- Claude readiness currently verifies version but leaves authentication unknown;
- the provisional fixed args include `--no-session-persistence` while M17 requires resume/continue capability;
- `supports_resume` is always false;
- `resume()` always returns unsupported;
- Claude process state is owned separately from the Codex adapter abstraction;
- provider-neutral `AgentAdapter` currently lives in `codex_adapter.rs` and `AgentProvider` only contains Codex;
- permission/wait/quota states are not yet a complete first-class Claude state machine;
- stop currently escalates directly through Windows process termination rather than sharing the mature bounded stop semantics where practical;
- provider-native Claude session identity is not durably modeled for deterministic resume.

M17 must independently inspect all provisional behavior before deciding what to keep.

## M17.01 Availability and readiness

Implement bounded readiness that can distinguish at least:

- READY;
- EXECUTABLE_NOT_FOUND;
- VERSION_UNSUPPORTED or VERSION_PROBE_FAILED where applicable;
- AUTH_REQUIRED;
- AUTH_UNVERIFIED;
- USAGE_LIMITED / RATE_LIMITED where directly observable;
- NETWORK_ERROR;
- PROCESS_ERROR.

Readiness must not create an engineering session row. If Claude exposes a stable auth-status command in the installed version, use it through a bounded direct process. Otherwise use a harmless bounded non-mutating print-mode probe in a temporary directory with tools/permissions constrained so it cannot edit project files. Never read auth storage directly.

## M17.02 Provider-neutral adapter compliance

Move provider-neutral interfaces/types out of Codex-specific ownership when necessary. The end state must allow Codex and Claude to implement the same lifecycle contract without weakening the existing Codex behavior.

At minimum the common contract must cover:

- provider identity and capability flags;
- readiness;
- start;
- list/get;
- stop;
- continue/resume where supported;
- reconcile after application restart;
- bounded stdout/stderr/final-response capture;
- durable event/provenance persistence.

Do not duplicate an entire second lifecycle stack when a neutral primitive can safely be shared.

## M17.03 Start, continue/resume, and stop

Start Claude in the registered canonical project/worktree cwd only after existing project/task validation passes.

Programmatic mode should use Claude Code's current supported non-interactive structured-output interface. Current official CLI documentation describes `--print`, `--output-format stream-json`, `--resume`, and `--continue`; the installed binary must be checked before those flags are used.

For resumability:

- capture the provider-native Claude session ID from authoritative Claude output;
- persist it with the H!veAI session/provenance model;
- resume by exact persisted provider session ID, not by guessing the latest global Claude conversation;
- never allow a resume to cross project identity, task identity, or cwd boundaries;
- define deterministic semantics for H!veAI `continue` versus `resume` and test them;
- remove any `--no-session-persistence` behavior that makes the promised resume contract impossible, unless the installed Claude version offers another verified resumability mechanism.

Stop must first attempt the safest supported owned-process termination path, then use bounded escalation if needed. Do not kill unrelated processes.

## M17.04 Structured stream mapping

Parse Claude `stream-json` as structured records rather than treating all stdout as opaque text.

Map observable Claude events into H!veAI's common event vocabulary while preserving bounded sanitized raw evidence. At minimum retain truthful evidence for:

- assistant text/final response;
- tool-use start/result where emitted;
- provider-native session ID;
- permission/wait state where emitted;
- usage/rate-limit/provider error signals;
- stderr;
- exit status;
- stream truncation/degradation.

Malformed stream records must fail or degrade truthfully; never silently reinterpret malformed provider output as successful completion.

## M17.05 Permission and attention states

Do not use `--dangerously-skip-permissions` or an equivalent bypass.

Inspect the installed Claude CLI capability surface and choose a programmatic permission strategy that keeps H!veAI in control. File mutation must remain scoped to the registered project/worktree and must not rely on shell-string construction.

Where observable, map provider states into explicit H!veAI attention states such as:

- WAITING_PERMISSION;
- WAITING_USER;
- AUTH_REQUIRED;
- USAGE_LIMITED;
- NETWORK_ERROR;
- PROVIDER_ERROR.

If non-interactive Claude cannot safely complete a requested action because approval is required, surface that state instead of falsely reporting COMPLETED.

## M17.06 Crash and orphan recovery

On H!veAI restart, reconcile Claude sessions truthfully.

- A process that is no longer owned must not remain RUNNING.
- Persist enough provider session identity to allow an eligible Claude conversation to be resumed later even if the original OS process is gone.
- Distinguish CRASHED/ORPHANED from clean COMPLETED/STOPPED.
- Recovery must not spawn or resume Claude automatically without a user action.

## M17.07 Security and process guarantees

Required invariants:

- canonical cwd containment;
- no shell command-string construction;
- executable resolution is bounded and rejects unsafe wrappers when direct native invocation is required;
- prompt size is bounded;
- stdout/stderr/event persistence is bounded and drains pipes safely;
- secret/token redaction applies before persistence/UI exposure;
- provider session IDs are treated as identifiers, not credentials;
- no cross-project resume;
- no argument injection through user prompt, task ID, project path, or provider session ID;
- stop targets only the process owned by the H!veAI session;
- Codex provider regression remains green.

## M17.08 UI, testing, audit, and publication

The Agents/session UI must truthfully expose Claude availability, version/readiness, capability flags, current session state, attention state, final response, and resume/continue availability without fake placeholders.

Required deterministic tests must cover at least:

- executable resolution/version/readiness classification;
- auth-required and usage-limited classification fixtures;
- project/task/cwd containment;
- command argument injection resistance;
- structured stream parsing and malformed records;
- provider session-ID capture and persistence;
- resume/continue project/task identity enforcement;
- permission/wait state mapping;
- bounded stdout/stderr/final-response behavior;
- graceful stop/escalation ownership;
- crash/orphan reconciliation;
- restart followed by explicit resume;
- Codex adapter regression;
- M16 audit provider regression;
- exact eight-project portfolio and FormuLab@main regression;
- frontend Agents/session states;
- full Rust/frontend/typecheck/build regression;
- governed native publication.

Builder completion is not M17 closure. M17 remains active until an independent strict audit and owner-native acceptance pass.

## Tracker transition

At M17 implementation start, canonical prospective truth must become:

- Current Milestone: M17;
- Current Sprint: M17-CLAUDE-ADAPTER;
- Current Task: M17 V01 — Claude Code Adapter;
- Current Task Status: IMPLEMENTATION_IN_PROGRESS;
- Required Actor: CODEX;
- M16: PASS/CLOSED;
- M16T: validated complete;
- strict completed milestone count: 17/20 = 85%;
- M17: `[~]` active;
- M18-M20 remain blocked/planned.

At builder completion, M17 must still remain `[~]` and the status must become `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`. Do not mark M17 `[x]` before independent audit plus owner acceptance.

## Closure

M17 closes only when source, deterministic tests, governed native publication, independent 20-section strict audit, and owner-native Claude workflow acceptance all pass. Only then may M18 activate.