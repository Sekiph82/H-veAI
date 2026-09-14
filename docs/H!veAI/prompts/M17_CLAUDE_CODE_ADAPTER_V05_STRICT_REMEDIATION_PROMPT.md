# M17 Claude Code Adapter V05 — Strict Diagnostic / Action-Truth / Production-Matrix Remediation Prompt

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
- all M17 V01-V04 strict audits/remediation prompts relevant to current lifecycle truth
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V04_LOG.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V04_STRICT_REAUDIT.md`
- current `src-tauri/src/agent_session_center.rs`
- current migration files
- current AgentSession frontend DTO/pages and focused M14/M17 tests
- current Tauri startup/reconcile wiring.

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
- Version: `V05`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed audit: `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V04_STRICT_REAUDIT.md`
- Required log: `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V05_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

V04 is `CHANGES_REQUIRED`. This is one bounded remediation package for F-M17-V04-001 through F-M17-V04-003 only. Do not activate M18 and do not broaden scope into unrelated roadmap work.

---

# FIRST ACTION — RECONCILE CANONICAL TRACKER TRUTH

Before substantive source changes, update `TASKS.md` and `CODEX_ROADMAP.md` so prospective truth states:

- Current Milestone: `M17`
- Current Sprint: `M17-CLAUDE-ADAPTER`
- Current Task: `M17 V05 — Claude Code Adapter diagnostic / action-truth / production-matrix remediation`
- Current Task Status: `CHANGES_REQUIRED / REMEDIATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16 remains `PASS/CLOSED`
- M16T remains validated complete
- strict completed milestone count remains `17 / 20 = 85%`
- M17 remains `[~]` OPEN
- M18-M20 remain planned/blocked.

Reference the V04 strict re-audit as the authoritative reason for V05. Historical V01-V04 prompt/log/audit artifacts are immutable.

Commit and push this tracker transition before substantive implementation.

At builder completion, if and only if every V05 gate passes, transition to:

- Current Task: `M17 V05 — Claude Code Adapter diagnostic / action-truth / production-matrix remediation`
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Required Actor: `HUMAN`
- M17 remains `[~]`
- M18 remains blocked.

Do not mark M17 `[x]`.

---

# ABSOLUTE PRESERVATION / SECURITY GUARDRAILS

Preserve all accepted M17 V01-V04 improvements not implicated by the V04 strict re-audit:

- centralized backend Claude process-live / stop / restart-reconcile / resume state vocabulary;
- exact provider identity, current project, task relationship, and canonical-cwd resume validation;
- per-session resume claim with RAII cleanup;
- no blocking `Child::wait()` while holding the shared Claude child mutex;
- bounded owned-process termination and unrelated-process protection;
- fail-closed Claude auth readiness with explicit readiness classes;
- immutable first provider-native Claude session ID with mismatch rejection;
- provider session ID/provenance/canonical cwd in the neutral adapter DTO;
- attention-state process-exit terminalization to truthful terminal state with preserved cause;
- retryable stop-failure concept;
- dedicated bounded current diagnostic columns and bounded control-event budget;
- accepted Codex and M16 audit-provider behavior.

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
- no weakening of M16 Codex audit-provider readiness/structured-output contracts;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` remain unchanged;
- M18 stays NOT ACTIVATED.

Do not redesign the full Agents stack. Prefer narrow transition/projection primitives plus deterministic test-only seams.

---

# F-M17-V04-001 — MAKE CURRENT DIAGNOSTIC TRUTH FOLLOW EVERY LIFECYCLE TRANSITION

V04 made `current_diagnostic_code` and `current_diagnostic_message` authoritative in session loading, but not every lifecycle transition maintains them.

Current defects:

1. Successful exact resume moves a prior FAILED/STOPPED/COMPLETED/ORPHANED row through STARTING to RUNNING without clearing stale current diagnostic columns.
2. `persist_live_claude_state` only writes when `state != ?2`. Therefore the first ordinary RUNNING provider record after successful resume is a no-op and cannot clear a stale diagnostic from the prior terminal state.
3. Restart reconciliation writes ORPHANED and inserts `PROCESS_ORPHANED`, but does not update the authoritative current diagnostic columns. Prior attention diagnostics can remain falsely current after ownership is gone.

## Required design

Make durable state + current diagnostic one governed truth transition. A small helper is preferred if it reduces duplicated SQL, but do not broadly rewrite the state machine.

Required behavior:

- entering STARTING for exact resume must not retain a stale terminal/attention/stop-failure diagnostic;
- entering RUNNING must have no stale attention/terminal diagnostic unless there is an explicitly current RUNNING diagnostic contract;
- a RUNNING provider record must be able to clear stale diagnostic fields even when durable state is already RUNNING, without generating unbounded duplicate events/materialization;
- a real RUNNING -> attention transition sets the corresponding current diagnostic;
- attention -> RUNNING clears it;
- restart reconciliation to ORPHANED must set an explicit current recovery diagnostic such as `CLAUDE_PROCESS_NOT_OWNED_AFTER_RESTART`, while retaining prior attention/error provenance in immutable historical events;
- finalization and stop-failure diagnostics remain truthful;
- state and diagnostic updates must not create unbounded event/materialization churn.

Required deterministic tests:

1. FAILED with `CLAUDE_USAGE_LIMITED` -> exact resume -> STARTING/RUNNING -> current diagnostic cleared.
2. FAILED/ORPHANED with another persisted diagnostic -> successful resume -> same-state provider RUNNING record -> diagnostic remains correctly clear.
3. WAITING_PERMISSION -> RUNNING clears current diagnostic and records only governed transition evidence.
4. restart reconcile from RUNNING -> ORPHANED -> current diagnostic is orphan/recovery truth.
5. restart reconcile from WAITING_PERMISSION/AUTH_REQUIRED/USAGE_LIMITED/NETWORK_ERROR -> ORPHANED -> current diagnostic is orphan/recovery truth while prior cause remains in history.
6. repeated same-state RUNNING/attention records stay bounded/no-op when state+diagnostic already match.

---

# F-M17-V04-002 — MAKE CURRENT ACTION PROJECTION CORRECT FOR EVERY RETURNED CLAUDE SESSION

V04 list projection uses actual ownership/claim truth, but direct command results still call plain `load_session()`, which forces `process_owned=false`.

This makes current action truth endpoint-dependent.

Required behavior:

1. Introduce one current Claude session projection path that can evaluate actual `AgentSessionCenter` ownership/claim state.
2. Use that projection for every Claude command that returns an `AgentSession`, including at minimum:
   - new Start success;
   - exact Resume success;
   - Retry success when it creates a Claude session;
   - Stop immediate result;
   - failed/unconfirmed Stop result where ownership is deliberately preserved;
   - list/session refresh behavior.
3. Do not fake ownership from durable state. Read actual process/claim truth.
4. A RUNNING newly started/resumed owned Claude session must return `canStop=true`.
5. A live owned attention state must return `canStop=true` and `supportsResume=false`.
6. An unconfirmed Stop failure that restores a retryable live state while ownership remains must return `canStop=true`, `supportsResume=false`, and the stop-failed current diagnostic.
7. A terminal/orphaned non-owned eligible row must return `canStop=false` and exact `supportsResume` truth from the existing eligibility evaluator.
8. A confirmed exited/finalized session must not advertise Stop.
9. Keep Codex behavior unchanged; do not force Codex into Claude ownership semantics.

Required deterministic tests must assert the DTO returned directly by the production functions, not only a later list refresh:

- `start()` Claude success -> returned RUNNING session has truthful `canStop`.
- `resume()` success -> returned RUNNING session has truthful `canStop` and no concurrent Resume.
- `retry()` Claude success -> returned session action truth matches ownership.
- `stop()` unconfirmed injected failure -> returned session is retryable Stop truth immediately.
- later list of the same session matches the mutation-return DTO action fields.
- terminal/orphaned eligible resume projection remains unchanged.

The frontend may continue refreshing after mutations, but correctness must not depend on that refresh to repair a lying command result.

---

# F-M17-V04-003 — COMPLETE THE REQUIRED PRODUCTION STOP / BUDGET MATRIX

V04 added an ordinary production Stop + monitor/finalizer test, but the strict prompt also required deterministic production-boundary escalation outcomes and end-to-end drain/finalization after both event budgets are saturated.

## A. Production Stop outcome injection

Create a narrow deterministic seam around the owned termination outcome used by the real `stop()` path. Prefer a test-only injected strategy or a small injectable primitive. Do not replace the real production termination implementation and do not weaken PID ownership protection.

Required production-path tests:

1. owned RUNNING Stop, no escalation needed -> real Stop state machine + monitor -> exactly one STOPPED finalization.
2. injected initial termination/escalation success through the real `stop()` path -> `STOP_ESCALATED(exited=true)` and monitor/finalizer -> STOPPED exactly once.
3. injected escalation cannot confirm exit through the real `stop()` path ->
   - no false STOPPED;
   - no permanent STOPPING;
   - process ownership remains;
   - `CLAUDE_STOP_FAILED_PROCESS_STILL_OWNED` is current diagnostic;
   - direct returned DTO says Stop remains retryable;
   - `stop_requested` policy is truthful;
   - later Stop can retry.
4. After that injected failure, change the injected outcome to success and prove the later Stop request can complete without duplicate finalization.
5. Stop from WAITING_PERMISSION, WAITING_USER, AUTH_REQUIRED, USAGE_LIMITED, and NETWORK_ERROR remains covered.
6. Unrelated process remains alive.

Do not rely on a persistence-helper-only test for these acceptance cases.

## B. End-to-end dual-budget saturation and finalization

Exercise `run_claude_session` or the same production stream/finalization boundary with deterministic in-memory streams/process fixtures.

Required test:

1. generate enough ordinary structured/raw records to saturate `MAX_OUTPUT_EVENTS`;
2. generate enough real control-state transitions to saturate `MAX_CONTROL_STATE_EVENTS` / the governed transition cap;
3. include an authoritative attention transition after generic budget exhaustion and prove current diagnostic remains durable;
4. recover to RUNNING truthfully;
5. continue supplying data after both caps to prove pipes are still drained;
6. terminate with a valid successful terminal provider result/process exit;
7. prove terminal finalization completes, `ended_at` is set, ownership is removed, state is truthful, and final response/current diagnostic truth is not blocked by either budget;
8. prove persisted ordinary/control event counts remain bounded.

Do not consume live Claude quota for these tests.

---

# STATE / ACTION CONSISTENCY GATE

Before V05 is complete, inspect every Claude production transition/projection path and prove that these concepts agree:

- durable state;
- current diagnostic code/message;
- `ended_at`;
- actual H!veAI process ownership;
- active resume claim;
- session-level `supportsResume`;
- `canStop`;
- provider session identity/provenance;
- canonical cwd/task/project validity.

No mutation command may return an `AgentSession` whose current action flags are knowingly stale relative to the ownership map.

No successful resume may continue showing a diagnostic from the prior terminal/attention state.

No ORPHANED restart-recovery row may present a stale live-attention diagnostic as the current cause.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused current-diagnostic transition tests;
2. focused direct mutation-return action-truth tests;
3. production Stop injected escalation-success/failure/retry tests;
4. end-to-end dual-budget stream drain/finalization test;
5. all prior M17 V01-V04 focused Rust tests;
6. focused M17 frontend tests;
7. M14 Agent Session Center frontend regression;
8. full serialized Rust library regression;
9. full frontend Vitest regression;
10. `npm run typecheck`;
11. `cargo check --manifest-path src-tauri/Cargo.toml`;
12. `npm run build`;
13. `git diff --check`;
14. active-source guardrail searches for:
    - `ANTHROPIC_API_KEY`;
    - direct Anthropic HTTP/API transport;
    - Claude credential/auth/token-file inspection;
    - GUI/browser automation for Claude;
    - blanket permission bypass;
    - direct OpenAI API transport in the M16 Codex-only audit-provider path;
15. exact-eight-project portfolio regression;
16. `Sekiph82/FormuLab@main` tracking regression.

Do not convert builder-only command output into independent audit proof. Record exact commands/results in the V05 builder log as claims for later independent review.

---

# NATIVE QA PUBLICATION

If and only if all V05 implementation/regression gates pass:

- publish through the governed `scripts/publish-dev-qa.ps1` production `--no-bundle` path;
- preserve the stable `dev-bin/H!veAI.exe` target and `dev-bin/H!veAI.ico` icon contract;
- record the final executable SHA-256;
- verify the stable Desktop shortcut still targets the standalone H!veAI executable;
- do not activate M18;
- do not ask the owner to perform native Claude acceptance until ChatGPT independently re-audits V05 and says PASS.

---

# REQUIRED BUILDER LOG

Create:

`docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V05_LOG.md`

Record at minimum:

- starting synchronized SHA;
- V05 tracker-transition SHA;
- implementation/test SHAs;
- exact files changed;
- exact remediation for F-M17-V04-001 through F-M17-V04-003;
- focused test names and results;
- full regression/build/security results;
- native QA publication result and executable SHA-256;
- final tracker state;
- final local HEAD / `origin/main` / live GitHub main equality and clean-worktree check.

The log is evidence/claim material, not independent acceptance.

---

# FINAL COMPLETION CONTRACT

Before saying COMPLETE:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

All three main SHAs must match and the worktree must be clean.

Final owner-facing response must contain only:

- GitHub URL/path for the V05 builder log;
- V05 tracker-transition SHA;
- implementation/test SHA(s);
- log SHA;
- final GitHub `main` SHA;
- concise status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`.

Do not claim M17 PASS/CLOSED. Do not activate M18.