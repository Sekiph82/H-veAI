# M17 Claude Code Adapter V04 — Independent Strict Re-Audit

## 1. VERDICT
CHANGES_REQUIRED.

V04 closes most of the V03 residuals at source level: session-level exact-resume eligibility is materially stronger, resume claims are RAII guarded, attention-state process exit is terminalized, and authoritative control diagnostics now have dedicated bounded persistence. However, V04 is not yet eligible for owner-native Claude acceptance because action truth is still inconsistent in direct command return DTOs, the new current-diagnostic columns are not transitioned consistently through exact resume/restart recovery, and the V04 prompt's required production stop-escalation plus dual-budget drain/finalization matrix is still incomplete.

M17 remains OPEN. M18 must remain blocked.

## 2. CONTRACT RECOVERY
Authoritative inputs reviewed:

- `AGENTS.md` strict audit governance and standalone/GitHub-first rules.
- `TASKS.md` current M17 V04 status.
- `CODEX_ROADMAP.md` milestone dependency status.
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V01_PROMPT.md`.
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V01_STRICT_AUDIT.md`.
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REMEDIATION_PROMPT.md`.
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REAUDIT.md`.
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V03_STRICT_REMEDIATION_PROMPT.md`.
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V03_STRICT_REAUDIT.md`.
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V04_STRICT_REMEDIATION_PROMPT.md`.
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V04_LOG.md`.
- current production source/tests on GitHub `main`.

Builder logs remain claims, not acceptance evidence.

The V04 contract was bounded to F-M17-V03-001 through F-M17-V03-005 and additionally imposed a state/action consistency gate: any returned `AgentSession` must agree with durable state, `ended_at`, actual H!veAI ownership, active resume claim, current resume eligibility, Stop eligibility, current diagnostic, provider identity, and project/task/cwd provenance.

## 3. BRANCH / HEAD / DIFF SCOPE
Repository: `Sekiph82/H-veAI`.

Branch: `main`.

V04 base/prompt SHA: `37bce12f65a8b7c4811534b3894b23e825cdfc40`.

V04 tracker transition: `62fdedebb64bdf28d4fc00888f3750dd0f10e6e3`.

Implementation SHAs:

- `762e371e36d72150f76b081c8132946a11d993f1`
- `e121f5945d628ce33c14485fc7b7bcabc8d66157`
- `91c36278b8ffbdaa4b665dd5134a361f93be0f0d`

Builder log/current pre-audit `main`: `194686caa3c07e200fd638247bde7d07c2df6c49`.

The compare from the V04 prompt SHA to the builder-log SHA is linear, ahead by five commits, with no divergence. Changed production/test scope includes `agent_session_center.rs`, migration 25, the AgentSession frontend DTO/action rendering, focused M14/M17 frontend fixtures, TASKS/roadmap state, and the V04 log.

## 4. ACCEPTANCE CRITERIA MATRIX
- Safe sync-first / GitHub-first execution: PASS by linear pushed history; local builder-worktree cleanliness remains builder-reported.
- M16 remains PASS/CLOSED: PASS.
- M18 not activated: PASS.
- Local Claude CLI only / owner-managed login / no direct Anthropic API transport: PASS in reviewed source/search.
- V04 F-M17-V03-001 exact-resume eligibility uses project/task/cwd/current state/ownership truth: PASS for list projection and resume preflight.
- Resume claim RAII/fail-safe cleanup: PASS at source level.
- Pre-STARTING injected resume failure preserves prior terminal/orphaned state: PASS in reviewed test/source path.
- Attention state + observed provider exit terminalizes to FAILED while preserving cause: PASS at source level.
- Dedicated bounded current diagnostic persistence independent of ordinary output event budget: PASS/PARTIAL; storage/budget exists, but transition truth is incomplete across resume/reconcile.
- Current diagnostic clears/updates consistently across all lifecycle transitions: FAIL.
- `canStop` / current action truth is accurate for every returned `AgentSession`: FAIL.
- Restart reconciliation preserves action truth and current diagnostic truth: PARTIAL; state/ownership recovery remains correct, current diagnostic columns are not reconciled.
- Production Stop ordinary owned-process path + monitor finalizes STOPPED exactly once: PASS in reviewed source/test.
- Production Stop injected escalation-success path through `stop()`: FAIL/UNVERIFIED.
- Production Stop injected unconfirmed-escalation failure through `stop()` with retry: FAIL/UNVERIFIED.
- Generic + control event budget exhaustion followed by real stream drain/finalization: FAIL/UNVERIFIED.
- Full deterministic V04 adversarial matrix: PARTIAL.
- Migration 25 registration/version expectations: PASS at source level.
- M14 frontend action fixture updated for `canStop`: PASS.
- Governed native publication / executable hash / no-console smoke checks: UNVERIFIED independently; builder claim only.
- Owner-native Claude acceptance: NOT REACHED because strict source/test audit fails.

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH
The builder reports 471 Rust tests, 141 frontend tests, typecheck, cargo check, production build, publication, security scans, and a published executable SHA-256 of `F9FE9F5A06DF0D723C130DEAFE336F590ADAE2CC6683DF8BBC932B9FCCDC7D12`.

GitHub exposes no commit status checks for the current builder-log HEAD, so those execution counts/publication results remain claims rather than independent CI evidence.

Repository truth does confirm substantial V04 implementation:

- `evaluate_claude_resume_eligibility` now validates provider, ownership/claim, provider identity, resume state, current project validity, task relationship, and canonical cwd.
- `ClaudeResumeClaimGuard` structurally releases leaked claims on error paths.
- `run_claude_session` converts an attention-state process exit into terminal `FAILED` while preserving the attention cause.
- migration 25 adds durable `current_diagnostic_code` and `current_diagnostic_message`.
- control-state events have a separate bounded budget.
- the frontend consumes backend `canStop` instead of re-deriving Claude liveness from a duplicated state set.

The log overstates closure because the three findings below remain visible in current production source/tests.

## 6. FILE / SYMBOL EVIDENCE
Primary reviewed production symbols:

- `src-tauri/src/agent_session_center.rs`
  - `AgentSession.can_stop`
  - `ClaudeResumeClaimGuard`
  - `evaluate_claude_resume_eligibility`
  - `start_claude`
  - `run_claude_session`
  - `persist_live_claude_state`
  - `stop`
  - `resume`
  - `retry`
  - `reconcile`
  - `load_session`
  - `load_session_with_ownership`
  - `insert_bounded_control_event`
  - V01-V04 focused Rust lifecycle tests.
- `src-tauri/src/db/migrations.rs`
  - migration 25 `claude_control_diagnostic_fields`.
- `src/agentSessionCenter.ts`
  - `AgentSession.canStop`.
- `src/pages.tsx`
  - Claude Stop visibility now derives from `canStop`.
- `tests/m17-claude-adapter-focused.test.tsx`.
- `tests/m14-agent-session-center-focused.test.tsx`.

## 7. FOCUSED TEST EVIDENCE
V04 adds meaningful deterministic evidence:

- production resume setup failure injection across DATABASE_OPEN, SPAWN, STDIN, PROMPT_WRITE, STDOUT, STDERR, OWNERSHIP, and RUNNING;
- valid/invalid session-level resume eligibility cases;
- current diagnostic survival after the generic event budget is exhausted;
- all five attention states followed by observed fixture process exit;
- an ordinary production Stop + monitor/finalizer path reaching STOPPED once;
- existing stop-from-attention, restart reconciliation, provider identity, malformed-stream, auth classifier, and bounded-state tests remain present.

Residual test gaps are material. The required escalation-success and unconfirmed-escalation cases are still injected only into `persist_stop_termination_outcome`, not through the production `stop()` termination boundary. There is no test-only termination strategy in the production stop path. The control-budget test invokes persistence helpers directly; it does not exhaust both budgets through `run_claude_session` and prove pipe drain plus terminal finalization afterward.

## 8. REGRESSION EVIDENCE
Positive evidence:

- migration expectations were updated from schema 24 to 25.
- the historical M14 frontend fixture was updated for the new `canStop` field and its running-session action truth.
- no reviewed V04 source change modifies the accepted M16 Codex-only audit-provider transport.
- the exact-resume provider ID/cwd/task/project checks remain stricter than V03.

Regression introduced by V04's dedicated diagnostic truth is described in F-M17-V04-001: the new columns are authoritative on load, but resume/reconcile do not maintain them consistently.

## 9. SECURITY / SAFETY REVIEW
PASS on provider/security boundary; CHANGES_REQUIRED on lifecycle truth.

Independent default-branch searches found no `ANTHROPIC_API_KEY`, `api.anthropic.com`, or `dangerously-skip-permissions` implementation matches. Reviewed Claude command construction remains argument-safe, prompt transport remains stdin-based, and exact resume still uses the canonical provider session ID rather than an arbitrary user-supplied shell string.

No V04 change introduces direct Anthropic HTTP transport, credential-file inspection, GUI/browser automation, automatic Claude installation/update, blanket permission bypass, or unrelated-process termination logic.

The residual defects are correctness/action-truth failures, not credential or network-boundary regressions.

## 10. ARCHITECTURE CONSISTENCY
PARTIAL.

The backend now has a much better explicit ownership/resume model and the frontend consumes backend Stop truth. That is aligned with the provider-neutral session architecture.

However, V04 still has two sources of current-action projection: `list()` calls `load_session_with_ownership(...actual ownership...)`, while several command-return paths call plain `load_session()`, which hardcodes `process_owned=false`. This makes current action truth depend on which endpoint produced the DTO.

The dedicated diagnostic columns similarly became authoritative without making all lifecycle state transitions update those columns atomically. Current state and current diagnostic can therefore drift.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS
Before this audit, root `TASKS.md` truthfully states V04 implementation complete and awaiting independent strict re-audit plus owner-native acceptance. This audit changes prospective truth: V04 is `CHANGES_REQUIRED`; owner-native acceptance is not yet authorized; the next actor is CODEX for one bounded V05 remediation.

The V04 builder log is accurate about many implemented mechanisms but overstates complete closure of F-M17-V03-005 and the final state/action consistency gate.

M18 remains blocked.

## 12. FINAL REPOSITORY STATE
Pre-audit live GitHub `main`: `194686caa3c07e200fd638247bde7d07c2df6c49`.

The V04 commit range is linear and visible on GitHub. No production source is modified by this audit artifact itself.

No GitHub Actions/status checks are attached to the current HEAD. Local HEAD/origin/live-main equality and clean-worktree claims in the builder log cannot be independently re-executed from this GitHub-only audit environment.

## 13. OPEN CROSS-MILESTONE FINDINGS
M16 remains PASS/CLOSED and is not reopened.

M18-M20 remain dependency-blocked until M17 passes an independent source audit and owner-native Claude acceptance.

The historical M21 migration/retirement status is not changed by this audit.

## 14. DEFECTS BY SEVERITY

### F-M17-V04-001 — MAJOR — Current diagnostic truth is stale across successful exact resume and restart reconciliation
V04 makes `current_diagnostic_code` / `current_diagnostic_message` authoritative in `load_session_with_ownership`. `persist_live_claude_state` clears them for RUNNING only when the durable state actually changes (`state != ?2`).

But successful exact resume transitions an existing terminal/orphaned row to STARTING and then RUNNING without clearing the prior current diagnostic columns. The first ordinary provider RUNNING record is then a same-state RUNNING no-op, so the stale diagnostic may survive indefinitely during a healthy resumed session. Example: a FAILED row carrying `CLAUDE_USAGE_LIMITED` can resume successfully and remain RUNNING while the UI still presents the old usage-limit diagnostic.

Restart reconciliation has the same consistency problem in the opposite direction: it changes a live row to ORPHANED and inserts `PROCESS_ORPHANED`, but does not write the new authoritative current diagnostic columns. A prior WAITING_PERMISSION diagnostic can therefore remain the current diagnostic after the process is no longer owned, or a previously RUNNING row can become ORPHANED with no current orphan diagnostic at all.

Required remediation: make state + current diagnostic an atomic governed transition. Clear stale diagnostics when entering STARTING/RUNNING through resume; allow RUNNING to clear a stale diagnostic even if state is already RUNNING; write an explicit current orphan diagnostic during restart reconciliation while retaining historical attention evidence in events. Add exact resume and reconcile regression tests.

### F-M17-V04-002 — MAJOR — Direct Claude command results still lie about current Stop eligibility
`load_session()` is a wrapper over `load_session_with_ownership(..., false)`. `list()` correctly supplies actual owned/claimed truth, but production command-return paths still use plain `load_session()`.

Concrete cases:

- `start_claude()` inserts the child into `claude_processes`, transitions to RUNNING, spawns the monitor, then returns `load_session()`. The returned RUNNING session therefore has `canStop=false` despite an owned process.
- successful `resume()` inserts process ownership, transitions to RUNNING, then captures its return session through plain `load_session()`, again producing `canStop=false` while owned.
- `retry()` reloads the newly started session through plain `load_session()`.
- `stop()` returns plain `load_session()`. On unconfirmed termination, ownership is deliberately preserved and the durable state is restored to a retryable live state, yet the immediate returned DTO reports `canStop=false` even though a later Stop is supposed to remain retryable.

The frontend currently refreshes the list after actions, which usually repairs the UI shortly afterward, but the command result itself violates the V04 requirement that every returned `AgentSession` agree with current ownership/action truth.

Required remediation: centralize current session projection with the real `AgentSessionCenter` ownership/claim state and use it for every Claude start/resume/retry/stop/list result. Add direct command-return tests, including failed-stop retry truth.

### F-M17-V04-003 — MAJOR — Required production stop-escalation and dual-budget drain/finalization matrix is still incomplete
V04 adds a real ordinary Stop + monitor/finalizer test, but the prompt explicitly required production-boundary injected outcomes for:

- initial termination requiring escalation and escalation succeeding;
- escalation not confirming exit, preserving ownership, leaving no permanent STOPPING, recording `CLAUDE_STOP_FAILED_PROCESS_STILL_OWNED`, and allowing a later Stop retry.

Current injected escalation tests still call `persist_stop_termination_outcome` directly. `stop()` continues to call `terminate_owned_child_bounded` directly and has no injectable termination seam, so those required production branches are not deterministically exercised.

The prompt also required exhausting generic and control budgets and then proving stream drain and terminal finalization complete. The V04 budget test fills generic events and exercises live-state persistence helpers, but does not route a saturated stream through `run_claude_session` to terminal completion.

Required remediation: add a narrow test-only/injectable owned-termination outcome seam used by the real `stop()` path, plus a real `run_claude_session` budget-saturation/finalization fixture. Do not consume live Claude quota.

Severity count:

- BLOCKER: 0
- MAJOR: 3
- MINOR: 0
- NOTE: 0

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES
After M17 closure, consider consolidating state + current-diagnostic writes into a small typed transition helper rather than allowing SQL statements in resume, stop, reconciliation, live-state persistence, and finalization to update overlapping truth fields independently.

Also consider one single `current_session(center, database, id)` projection function for all Claude-returning IPC paths so future action fields cannot silently diverge between list and mutation commands.

These are appropriate implementation shapes for V05, but V05 should remain bounded to the three findings above rather than redesigning the Agents subsystem.

## 16. UNVERIFIED ITEMS
The following remain builder claims or otherwise unverified independently:

- 471/0 Rust regression execution.
- 141/0 frontend regression execution.
- focused M17 2/0 frontend execution.
- local typecheck/cargo check/build/diff-check command results.
- the published executable SHA-256.
- stable Desktop shortcut target/icon verification.
- no-console native smoke test and dev-port checks.
- local final worktree cleanliness and local HEAD/origin/live-main equality.

Those items may be used as builder evidence but are not converted into independent PASS without a trusted execution/CI source or owner-native acceptance.

## 17. REGRESSION RISK
MEDIUM-HIGH.

The major remaining production defects do not reintroduce credential/network hazards, but they can present stale or contradictory session truth exactly at the lifecycle boundaries the owner will use during native Claude acceptance. The missing injected stop matrix also leaves a high-value failure branch unproven.

## 18. AUDIT CONFIDENCE
HIGH.

F-M17-V04-001 and F-M17-V04-002 follow directly from current production SQL/control flow and DTO projection. F-M17-V04-003 follows directly from the explicit V04 prompt requirements and the reviewed test/source inventory.

## 19. FINAL VERDICT
CHANGES_REQUIRED.

Do not perform owner-native Claude start/stop/exact-resume acceptance yet. M17 remains OPEN and M18 remains blocked.

## 20. REQUIRED REMEDIATION
Execute one bounded M17 V05 remediation package covering only F-M17-V04-001 through F-M17-V04-003.

Preserve all accepted V01-V04 improvements, especially:

- centralized process-live/stop/reconcile/resume state vocabulary;
- exact provider identity/project/task/cwd resume validation;
- resume claim RAII;
- attention-process-exit terminalization;
- bounded dedicated current diagnostic storage/control-event budget;
- retryable stop-failure behavior;
- accepted M16 Codex-only audit-provider behavior;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` tracking;
- local Claude CLI + Claude-managed login only;
- no direct Anthropic API transport or credential inspection;
- M18 blocked state.

After V05 builder completion, ChatGPT must perform a new independent strict re-audit before owner-native Claude acceptance.