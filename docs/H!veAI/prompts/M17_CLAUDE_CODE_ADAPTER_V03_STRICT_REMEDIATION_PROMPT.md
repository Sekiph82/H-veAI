# M17 Claude Code Adapter V03 - Strict Ownership / Recovery / Boundedness Remediation Prompt

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
- `docs/H!veAI/plans/M17_CLAUDE_CODE_ADAPTER_V01_PLAN.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V01_PROMPT.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V01_STRICT_AUDIT.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REMEDIATION_PROMPT.md`
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V02_LOG.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REAUDIT.md`
- `docs/H!veAI/audits/M16T_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`
- `src-tauri/src/agent_adapter.rs`
- `src-tauri/src/agent_session_center.rs`
- `src-tauri/src/codex_adapter.rs`
- `src-tauri/src/final_response.rs`
- `src-tauri/src/stream_sanitizer.rs`
- current Agents/session frontend and focused tests.

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
- Version: `V03`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed audit: `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REAUDIT.md`
- Required log: `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V03_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

V02 is `CHANGES_REQUIRED`. This is a bounded remediation package for F-M17-V02-001 through F-M17-V02-005 only. Do not activate M18 and do not broaden scope into unrelated roadmap work.

---

# FIRST ACTION: RECONCILE CANONICAL TRACKER TRUTH

Before substantive source changes, update `TASKS.md` and `CODEX_ROADMAP.md` so current prospective truth states:

- Current Milestone: `M17`
- Current Sprint: `M17-CLAUDE-ADAPTER`
- Current Task: `M17 V03 - Claude Code Adapter ownership / recovery / boundedness remediation`
- Current Task Status: `CHANGES_REQUIRED / REMEDIATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16 remains `PASS/CLOSED`
- M16T remains validated complete
- strict completed milestone count remains `17 / 20 = 85%`
- M17 remains `[~]` OPEN
- M18-M20 remain planned/blocked.

Reference the V02 strict re-audit as the authoritative reason for V03. Do not rewrite historical V01/V02 prompt, log, or audit artifacts.

Commit and push this tracker transition before substantive implementation.

At builder completion, if and only if every V03 gate passes, transition to:

- Current Task: `M17 V03 - Claude Code Adapter ownership / recovery / boundedness remediation`
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Required Actor: `HUMAN`
- M17 remains `[~]`
- M18 remains blocked.

Do not mark M17 `[x]`.

---

# ABSOLUTE PRESERVATION / SECURITY GUARDRAILS

Preserve the accepted V02 improvements:

- no blocking `Child::wait()` while holding the shared Claude child mutex;
- bounded owned-process polling;
- fail-closed Claude auth readiness requiring successful process completion;
- explicit AUTH_REQUIRED / AUTH_UNVERIFIED / USAGE_LIMITED / NETWORK_ERROR / TIMEOUT / PROCESS_ERROR readiness classes;
- immutable first provider-native Claude session ID with mismatch rejection;
- provider session ID/provenance/canonical cwd fields in the neutral session DTO;
- shared resume setup failure cleanup architecture;
- live attention-state persistence concept;
- existing Codex behavior and M16 audit-provider behavior.

Required boundaries remain:

- local Claude Code CLI only;
- owner-managed Claude login only;
- no `ANTHROPIC_API_KEY` required, read, written, persisted, or used as fallback;
- no direct Anthropic HTTP/API transport;
- no Claude credential/auth/token-file inspection;
- no Claude desktop GUI automation;
- no browser automation to drive Claude;
- no automatic Claude installation/update;
- no shell command-string prompt transport;
- no `--dangerously-skip-permissions` or equivalent blanket permission bypass;
- no cross-project / cross-task / wrong-cwd exact-session resume;
- no unrelated Claude process termination;
- no weakening of M16 Codex audit-provider readiness or structured-output contracts;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` remain unchanged;
- M18 stays NOT ACTIVATED.

Do not redesign the full Agents stack. Prefer one explicit lifecycle vocabulary/predicate layer and small testable primitives.

---

# F-M17-V02-001: SEPARATE PROCESS OWNERSHIP FROM PROVIDER-CONVERSATION RESUMABILITY

V02 introduced durable live attention states while a Claude process may still be alive. `resume()` currently rejects only RUNNING / STARTING / STOPPING and the UI exposes Resume whenever a provider session ID exists.

This must be corrected so one H!veAI session can never spawn a second exact-session Claude process while the first process is still owned.

## Required design

Create explicit, centralized lifecycle predicates or equivalent strongly governed helpers for at least:

1. states that may represent a currently owned live process;
2. states eligible for stop;
3. states eligible for restart reconciliation;
4. states eligible for exact resume when no process is owned.

Do not duplicate drifting string lists across `stop`, `resume`, `reconcile`, `load_session`, and UI logic.

## Resume contract

Before any resume state transition or spawn:

- prove provider is CLAUDE;
- prove same project/task/canonical cwd as already required;
- prove provider session ID exists and remains canonical;
- prove no H!veAI-owned process entry exists for this H!veAI session;
- prove the durable session state is eligible for resume according to the centralized lifecycle policy.

A currently owned process must make resume unavailable regardless of whether its semantic state is:

- RUNNING
- STARTING
- STOPPING
- WAITING_PERMISSION
- WAITING_USER
- AUTH_REQUIRED
- USAGE_LIMITED
- NETWORK_ERROR
- or any future live-attention state governed by the same predicate.

Return one explicit bounded diagnostic such as `CLAUDE_RESUME_SESSION_STILL_OWNED` for the ownership conflict. Do not kill the existing process merely because a resume was attempted.

Provider conversation resumability after the process is gone is separate truth. A terminal/orphaned row may remain resume-capable when exact provider identity and cwd/task/project provenance are valid.

## UI contract

The frontend must not render an enabled `Resume exact session` action merely because `supportsResume` is true at provider capability level.

Expose or derive an explicit session-level resume eligibility signal. The UI must show resume only when the backend lifecycle says the session is currently resumable. A live waiting/attention session must show Stop / attention handling, not concurrent Resume.

## Required tests

Add deterministic tests proving:

- WAITING_PERMISSION + owned process cannot resume;
- WAITING_USER + owned process cannot resume;
- AUTH_REQUIRED + owned process cannot resume;
- USAGE_LIMITED + owned process cannot resume;
- NETWORK_ERROR + owned process cannot resume;
- ORPHANED + valid identity + no owned process remains resume eligible;
- STOPPED / FAILED / COMPLETED behavior matches the explicit policy;
- frontend hides/disables Resume for live owned attention states and exposes it for an eligible orphaned/terminal session.

Do not use live Claude quota for these tests.

---

# F-M17-V02-002: MAKE RESTART RECONCILIATION COVER THE COMPLETE LIVE STATE SET

V02 reconciliation currently misses newly durable AUTH_REQUIRED / USAGE_LIMITED / NETWORK_ERROR live states.

On application restart, in-memory process ownership is gone. Any persisted Claude row whose state may have represented an owned live process at shutdown must be reconciled truthfully.

Requirements:

1. Reuse the centralized process-live/reconcile predicate from F-M17-V02-001.
2. Include every live state, including the three omitted V02 attention states.
3. Transition stale live rows to `ORPHANED` without spawning/resuming Claude.
4. Preserve provider session ID, provenance, task/project identity, canonical cwd, final-response data, and prior attention diagnostic evidence.
5. Append one bounded orphan/recovery event.
6. Mark/materialize control-plane truth once per reconciled session/project as governed by existing contracts.
7. After reconcile, exact resume may be offered only if the provider-session/cwd/task/project prerequisites pass.
8. No startup path may auto-resume or auto-spawn Claude.

Required deterministic tests:

- STARTING -> ORPHANED;
- RUNNING -> ORPHANED;
- WAITING_PERMISSION -> ORPHANED;
- WAITING_USER -> ORPHANED;
- AUTH_REQUIRED -> ORPHANED;
- USAGE_LIMITED -> ORPHANED;
- NETWORK_ERROR -> ORPHANED;
- STOPPING -> ORPHANED;
- already terminal/non-live states remain unchanged;
- provider session identity survives;
- eligible reconciled row is explicitly resumable afterward;
- reconciliation never creates a provider process.

---

# F-M17-V02-003: RESTORE BOUNDED LIVE-STATE EVENT PERSISTENCE

V02 `persist_live_claude_state()` currently runs for every parsed RUNNING/attention record and calls unbounded `insert_event()` plus control-plane materialization whenever the UPDATE matches. The UPDATE does not require an actual state change.

This defeats the M17 bounded-event contract.

## Required behavior

1. Persist live semantic state only when the durable state actually changes.
2. Repeated identical RUNNING records must not create repeated `SESSION_STATE` events or repeated truth materializations.
3. Repeated identical WAITING_PERMISSION / WAITING_USER / AUTH_REQUIRED / USAGE_LIMITED / NETWORK_ERROR records must be coalesced or bounded.
4. State-transition diagnostics must be subject to a bounded per-session event policy. Integrate with the existing session event budget or introduce a separate explicit bounded transition budget with a deterministic cap.
5. Hitting an event/persistence cap must not stop stdout/stderr pipe draining.
6. Raw stream redaction must still happen before any persistence/UI exposure.
7. One real transition from waiting back to running must remain visible and durable.
8. Do not repeatedly call `materialize_best_effort` for no-op same-state records.

Prefer a state transition helper that reads/updates atomically and returns whether a real transition happened. Do not depend on SQLite affected-row semantics for a no-op update unless the SQL explicitly guards against identical state.

Required deterministic tests:

- hundreds/thousands of repeated RUNNING records remain within the event cap;
- repeated same attention state remains bounded;
- WAITING_PERMISSION -> RUNNING produces exactly the governed transitions;
- event cap does not prevent child stream draining/finalization;
- no-op state input does not trigger repeated control-plane materialization.

---

# F-M17-V02-004: MAKE FAILED STOP ESCALATION TRUTHFUL AND RETRYABLE

V02 correctly removed the wait-lock deadlock and correctly labels hard `Child::kill()` as non-graceful. However, when bounded initial termination plus escalation still cannot confirm exit, `stop()` can return with durable state `STOPPING` forever.

Model this outcome explicitly.

Requirements:

1. `STOPPING` is transient and must not be the indefinite final result of a completed stop command.
2. If bounded termination confirms process exit, allow the monitor to finalize `STOPPED` exactly once.
3. If bounded termination does not confirm exit:
   - do not falsely claim STOPPED;
   - preserve ownership of the still-running child;
   - persist an explicit bounded diagnostic such as `CLAUDE_STOP_FAILED_PROCESS_STILL_OWNED`;
   - transition from transient STOPPING to a truthful retryable live/degraded state according to the centralized lifecycle model;
   - ensure a later Stop attempt can retry;
   - keep the monitor active;
   - define whether `stop_requested` remains set or is cleared so a later natural exit is classified truthfully.
4. Do not remove process ownership until the process has actually exited or lifecycle ownership is otherwise truthfully lost.
5. `STOP_ESCALATED` must record escalation attempt and confirmed outcome truthfully.
6. No unrelated process may be targeted.
7. Avoid double-finalization races between stop and monitor.

Use injectable process-termination behavior for deterministic tests rather than relying only on real OS escalation.

Required deterministic tests:

- initial owned termination succeeds -> terminal STOPPED through monitor;
- initial termination requires escalation and escalation succeeds -> truthful STOP_ESCALATED + STOPPED;
- escalation cannot confirm exit -> no false STOPPED, no permanent STOPPING, owned process remains retryable, explicit stop-failed diagnostic;
- stop from WAITING_PERMISSION works;
- stop from WAITING_USER works;
- stop from AUTH_REQUIRED / USAGE_LIMITED / NETWORK_ERROR works when still owned;
- unrelated process remains alive.

---

# F-M17-V02-005: COMPLETE THE ADVERSARIAL LIFECYCLE TEST MATRIX

The V02 source tests are not yet sufficient to claim closure. Complete the matrix around actual production lifecycle boundaries, not only helper methods.

At minimum prove:

1. all F-M17-V02-001 ownership/resume cases;
2. all F-M17-V02-002 restart/reconcile cases;
3. all F-M17-V02-003 bounded state-event cases;
4. all F-M17-V02-004 stop outcome cases;
5. production `stop()` persistence, not only `terminate_owned_child_bounded()`;
6. production resume rejection before spawn while an owned process exists;
7. resume setup failure injection for the important stages:
   - spawn;
   - stdin;
   - prompt write;
   - stdout;
   - stderr;
   - ownership registration;
   - RUNNING/control-plane persistence;
8. every post-STARTING failure leaves no false STARTING/RUNNING row and no unmonitored child;
9. provider session ID remains immutable across all failure paths;
10. exact resume uses only the canonical ID;
11. malformed provider records cannot become success;
12. auth/quota/network classifiers remain fail-closed and avoid false positives;
13. stream redaction still precedes persistence/UI exposure;
14. Codex adapter/start/stop/list/final-response regressions remain green;
15. M16 audit-provider readiness/freeform structured-output regressions remain green;
16. exact-eight-project and FormuLab@main regressions remain green.

Prefer deterministic dependency injection or test-only process abstractions over brittle timing-only tests. Do not consume live Claude quota for lifecycle unit tests.

---

# STATE MODEL CONSISTENCY GATE

Before implementation is considered complete, audit every use of Claude session state strings in production source and ensure the same centralized semantics govern:

- start;
- stream/live transition persistence;
- stop eligibility;
- resume eligibility;
- restart reconciliation;
- UI Stop visibility;
- UI Resume visibility;
- finalization;
- diagnostics.

A new attention state must not require manually editing five unrelated hard-coded state lists in the future.

Do not change accepted public state names unless necessary. If a new internal predicate or resume-eligibility field is introduced, keep backward compatibility for existing persisted rows and frontend contracts.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused M17 V03 ownership/resume tests;
2. focused restart/reconcile tests;
3. focused bounded event/state-transition tests;
4. focused stop success/escalation/failure tests;
5. full resume failure-injection suite;
6. original M17 V01/V02 focused Rust tests;
7. frontend Agents/session focused tests;
8. Codex adapter regression tests;
9. M16 audit-provider regression tests including V11 readiness/freeform behavior;
10. exact-eight-project and FormuLab@main tests;
11. full serialized Rust library regression under repository policy;
12. full frontend Vitest regression;
13. `npm run typecheck`;
14. `cargo check --manifest-path src-tauri/Cargo.toml`;
15. `npm run build`;
16. `git diff --check`;
17. active-source scan proving no `ANTHROPIC_API_KEY`, direct Anthropic HTTP/API provider, Claude auth-file inspection, GUI/browser automation, or blanket permission bypass;
18. existing M16 guardrail scan proving no OpenAI API-key/direct HTTP audit-provider regression;
19. governed native QA publication;
20. stable shortcut / no-visible-console checks.

Builder execution output remains claim evidence until independent audit.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V03_LOG.md`

Include:

- synchronized starting SHA;
- V03 tracker-transition SHA;
- implementation SHA(s);
- exact remediation mapping F-M17-V02-001 through F-M17-V02-005;
- centralized lifecycle predicate/state-model design;
- process ownership versus provider-conversation resumability design;
- restart reconciliation coverage;
- bounded live-state event/materialization design and cap;
- stop success/escalation/failure truth model;
- resume failure-injection mechanism and stage matrix;
- focused test names/counts for every finding;
- full Rust/frontend/typecheck/build/cargo-check results;
- M16/Codex regression evidence;
- forbidden-provider/auth/GUI/bypass scans;
- governed native publication EXE SHA-256 and shortcut evidence;
- final tracker state;
- final local HEAD / `origin/main` / live-main equality.

Commit the immutable log separately after implementation/publication evidence is complete. Do not require the log to contain the SHA of the commit that first creates itself.

---

# INDEPENDENT / OWNER CLOSURE GATE

Do not fabricate M17 acceptance.

After builder completion:

1. ChatGPT performs the independent strict M17 V03 re-audit.
2. Only if source audit PASS does the owner perform native Claude acceptance.
3. Owner-native acceptance must then prove READY, real project-scoped Start, structured stream/final response, Stop, exact-session Resume, and truthful attention/failure behavior without cross-project leakage.
4. M17 remains OPEN until both independent source audit and HUMAN owner-native acceptance pass.
5. M18 remains blocked until M17 closes.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- canonical tracker transition to V03 is committed/pushed;
- F-M17-V02-001 through F-M17-V02-005 are closed in production source and deterministic tests;
- no live owned Claude process can be concurrently exact-resumed;
- restart reconciliation covers the complete governed live-state set;
- live state/event persistence is bounded and coalesced without stopping pipe draining;
- failed stop escalation cannot leave indefinite STOPPING and cannot falsely claim STOPPED;
- full adversarial lifecycle matrix passes;
- accepted V02 fixes remain intact;
- M16/Codex regressions remain green;
- local Claude CLI-only boundary remains intact;
- required full regressions and governed native publication pass;
- every repository change is committed and pushed;
- local HEAD == origin/main == live GitHub main;
- worktree is clean;
- M17 remains `[~]` awaiting independent strict re-audit and owner acceptance;
- M18 remains blocked.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must contain only:

- GitHub M17 V03 log URL/path;
- V03 tracker-transition commit SHA;
- implementation commit SHA(s);
- M17 V03 log commit SHA;
- final GitHub main SHA;
- concise status.