# M17 Claude Code Adapter V01 — Independent Strict Audit

## 1. VERDICT
CHANGES_REQUIRED.

M17 V01 is not eligible for owner-native acceptance yet. The implementation establishes substantial Claude Code adapter structure, but core lifecycle and truthfulness defects remain in stop, resume failure handling, readiness, provider-session identity stability, and required deterministic coverage.

M17 remains OPEN. M18 must remain blocked.

## 2. CONTRACT RECOVERY
Authoritative inputs reviewed:

- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V01_PROMPT.md`
- `docs/H!veAI/plans/M17_CLAUDE_CODE_ADAPTER_V01_PLAN.md`
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V01_LOG.md`
- root `TASKS.md`
- M16 final owner acceptance and accepted M16 Codex audit-provider architecture
- implementation commit `bf81d738ab6ecd3ee02f5c47e6ae678051455f30`
- current production source and focused tests on GitHub `main`.

Builder logs are claims, not acceptance evidence.

## 3. BRANCH / HEAD / DIFF SCOPE
Repository: `Sekiph82/H-veAI`

Branch: `main`

Tracker transition: `f0b8bda739b03aa1be4a10b912371311b9d6a1c7`

Implementation: `bf81d738ab6ecd3ee02f5c47e6ae678051455f30`

Builder completion/log commit before this audit: `fb226d41d7d4062365d07299ebaaf221eb6effda`

The implementation commit changes the provider-neutral adapter module, Claude session center, Codex adapter re-export boundary, migration 24, Tauri wiring, Agents UI/types, and focused frontend tests.

## 4. ACCEPTANCE CRITERIA MATRIX
- Safe sync-first / GitHub-first transition: PASS by repository history and tracker state.
- M16 remains PASS/CLOSED: PASS.
- M18 not activated: PASS.
- Local Claude CLI only / no Anthropic API-key transport: PASS in reviewed source.
- Native executable resolution/version/help/auth probes: PARTIAL.
- Provider-neutral adapter foundation: PARTIAL.
- Project/task/cwd validation: PASS in reviewed source.
- Structured Claude stream parsing/redaction: PASS with residual identity/state defects below.
- Exact-session resume identity: PARTIAL.
- Stop lifecycle: FAIL.
- Resume failure truthfulness: FAIL.
- Permission/wait attention state: PARTIAL.
- Crash/orphan reconciliation: PASS at source level.
- Required deterministic security/lifecycle coverage: FAIL/PARTIAL.
- Governed native publication: builder claim only; not independently promoted to proof.
- Owner-native acceptance: NOT REACHED because source audit fails.

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH
The builder reports 451 Rust tests, 140 frontend tests, typecheck/build/cargo check, Claude 2.1.270 probes, auth success, and native publication. GitHub exposes no independent CI status checks for the implementation commit, so those execution counts remain builder claims.

Repository source does independently confirm substantial implementation work: native `claude.exe` resolution, bounded probes, exact `--resume` usage, migration 24, structured stream parsing, provider-session persistence fields, UI exposure, and fail-closed malformed JSON handling.

## 6. FILE / SYMBOL EVIDENCE
Primary audited symbols:

- `src-tauri/src/agent_adapter.rs`
  - `AgentProvider`
  - `AdapterReadiness`
  - `AdapterSession`
  - `AgentAdapter`
- `src-tauri/src/agent_session_center.rs`
  - `claude_readiness`
  - `probe_claude_auth`
  - `start_claude`
  - `run_claude_session`
  - `parse_claude_record`
  - `stop`
  - `resume`
  - `reconcile`
  - `load_session`
- `src-tauri/src/db/migrations.rs`
  - migration 24 / `agent_provider_session_provenance`
- `src-tauri/src/final_response.rs`
- `src-tauri/src/stream_sanitizer.rs`
- `tests/m17-claude-adapter-focused.test.tsx`.

## 7. FOCUSED TEST EVIDENCE
Reviewed Rust tests cover provider parsing, native-candidate resolution, redaction, bounded capture, project/task cross-project rejection, fixed args, help capability parsing, representative stream records/error classification, and provenance round-trip.

Reviewed frontend coverage verifies READY presentation, ORPHANED presentation, and dispatch of `hiveai_agent_resume` for the exact H!veAI session.

The required adversarial coverage is incomplete. In particular, no reviewed deterministic test proves a running Claude process can be stopped without blocking behind the monitor wait lock; no reviewed test proves resume spawn/stdin/stdout failure cleans session/process truth; no reviewed test rejects a mid-stream provider-session-ID change; and no reviewed auth test proves a non-zero `claude auth status --json` process can never produce READY.

## 8. REGRESSION EVIDENCE
No direct source regression was found in the accepted M16 Codex audit-provider path. The Codex adapter still implements the shared adapter contract and its mature monitor/stop logic remains separate.

However, the M17 Claude lifecycle duplicates critical process management instead of reusing the Codex monitor pattern, and the duplicated implementation contains the stop-lock defect described below.

## 9. SECURITY / SAFETY REVIEW
PASS with lifecycle remediation required.

Positive findings:

- prompts travel through bounded stdin rather than shell strings;
- no `ANTHROPIC_API_KEY` or direct Anthropic HTTP provider was introduced in the reviewed implementation diff;
- no auth-file inspection was found;
- Claude is invoked directly as a native executable;
- provider output is redacted before raw stream persistence;
- `--dangerously-skip-permissions` is not used;
- resume uses a persisted provider session ID rather than global `--continue` selection.

Residual identity and process-lifecycle defects are correctness/safety concerns and must be closed before native acceptance.

## 10. ARCHITECTURE CONSISTENCY
PARTIAL.

The new `agent_adapter.rs` correctly moves provider identity and lifecycle methods out of Codex-specific ownership. However, `AdapterSession` does not carry provider-native session identity/provenance/canonical provider cwd fields even though M17 requires the neutral contract to support provider-specific session identity/provenance. Claude's richer identity currently remains available only through the parallel `AgentSession` model.

This does not by itself prove a runtime failure, but the V02 remediation should either extend the neutral DTO contract or make the opaque-provider-identity ownership explicit and test that all neutral lifecycle calls preserve exact identity.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS
Current root tracker truth correctly leaves M17 `[~]` and M18-M20 blocked, but it still says the next action is independent audit followed by owner acceptance. This audit changes the prospective truth: V01 is `CHANGES_REQUIRED` and the next actor is CODEX for bounded M17 V02 remediation. The next builder prompt must reconcile the root tracker and roadmap before substantive V02 changes.

## 12. FINAL REPOSITORY STATE
Before this audit artifact, live `main` was `fb226d41d7d4062365d07299ebaaf221eb6effda`.

No production source is modified by this audit.

## 13. OPEN CROSS-MILESTONE FINDINGS
M16 remains accepted and is not reopened.

M18 remains blocked until M17 passes independent strict audit and owner-native acceptance.

## 14. DEFECTS BY SEVERITY

### F-M17-V01-001 — MAJOR — Stop can block behind a mutex held by the Claude wait thread
`run_claude_session` creates a wait thread that locks `Arc<Mutex<Child>>` and then calls blocking `Child::wait()` while holding that mutex for the lifetime of the running process. The `stop` command later tries to lock the same mutex before calling `kill()`.

Result: while Claude is still running, Stop can block waiting for the monitor's mutex; it cannot reliably reach the kill/escalation path until the process has already exited. This defeats M17's owned-process stop contract.

Required remediation: replace the blocking wait-under-lock design with a monitor that never holds the child mutex across a blocking wait. Use bounded `try_wait()` polling or another ownership-safe primitive analogous to the mature Codex lifecycle. Add a deterministic test that proves Stop can terminate a still-running owned fixture without waiting for natural exit and cannot target an unrelated process.

### F-M17-V01-002 — MAJOR — Resume setup failures can leave a false STARTING session and an unmonitored child
`resume()` persists `state='STARTING'` before process spawn. The subsequent spawn/stdin-write/stdout/stderr/process-registration/running-state steps use direct `?` error exits and do not consistently call `finish_failed`, kill an already-created child, remove ownership, or restore terminal truth.

Result: a resume invocation failure can leave a H!veAI row falsely STARTING until application restart reconciliation; some failure points can also leave a child process unmonitored.

Required remediation: implement one fail-closed resume setup path that records a bounded diagnostic, kills/joins any child already created, removes ownership if inserted, transitions the H!veAI session to FAILED (or another explicit truthful terminal state), marks/materializes control-plane truth, and returns the error. Add deterministic failure-injection tests for spawn, stdin, stdout/stderr setup, ownership registration, and running-state persistence where practical.

### F-M17-V01-003 — MAJOR — Claude readiness can claim READY without proving successful auth-probe process completion and does not preserve required failure classes
`probe_claude_auth` parses stdout for `loggedIn` but does not require `output.status.success()`. Therefore a non-zero auth-status process that happens to emit parseable `{loggedIn:true}` can still contribute to READY.

The readiness path also collapses most non-timeout auth-probe failures into `AUTH_UNVERIFIED`; it does not truthfully preserve explicit `PROCESS_ERROR`, `NETWORK_ERROR`, or `USAGE_LIMITED/RATE_LIMITED` evidence when such evidence is present.

Required remediation: require successful auth-probe exit before READY. Add one bounded classifier over sanitized probe stdout/stderr/exit/timeout that preserves explicit AUTH_REQUIRED, NETWORK_ERROR, USAGE_LIMITED/RATE_LIMITED, PROCESS_ERROR, TIMEOUT, and AUTH_UNVERIFIED without false positives. Do not add API keys, auth-file inspection, or an uncontrolled project-mutating model turn.

### F-M17-V01-004 — MAJOR — Provider-native Claude session identity is not immutable once captured
During stream parsing, every record carrying a `session_id` overwrites the in-memory `provider_session_id`. The intermediate DB update attempts to reject a different ID with `WHERE provider_session_id IS NULL OR provider_session_id=?2`, but the affected-row result is ignored. Finalization then unconditionally writes `provider_session_id=COALESCE(?7, provider_session_id)`, so a later mismatching provider ID can replace the originally captured canonical Claude session ID.

Result: deterministic exact-session resume identity can drift silently.

Required remediation: the first valid provider-native session ID becomes immutable for that H!veAI session. Any later different ID must produce an explicit provider-identity mismatch diagnostic and must not overwrite canonical identity. Add adversarial parser/finalization tests for stable repeated ID and mismatching ID.

### F-M17-V01-005 — MAJOR — Waiting/attention records do not update live canonical session state
`run_claude_session` records parsed `WAITING_PERMISSION`, `WAITING_USER`, `AUTH_REQUIRED`, `USAGE_LIMITED`, and `NETWORK_ERROR` only in an in-memory `terminal_state` and event rows while the child is running. The `agent_sessions.state` row is not transitioned until the stream closes and finalization runs.

Result: a provider that remains alive while waiting can remain canonically `RUNNING` even though H!veAI already observed a waiting/attention event. This weakens the M17 requirement to expose actionable live permission/wait state.

Required remediation: persist bounded live state transitions when authoritative provider records expose a waiting/attention state, while allowing later authoritative records to move the session forward. Keep the transition vocabulary explicit and add deterministic tests for live WAITING_PERMISSION / WAITING_USER and subsequent completion/failure/stop.

### F-M17-V01-006 — MAJOR — Required lifecycle/security tests are incomplete
The V01 prompt explicitly required deterministic tests for resume identity mismatch cases, owned-process stop, restart/orphan explicit resume, permission/wait mapping, quota false positives, and related lifecycle invariants. The reviewed test source does not cover several of these critical paths, including the defects above.

Required remediation: add focused deterministic tests that reproduce every V01 strict finding and the original M17 required invariants. Builder-wide pass counts are not a substitute for the missing adversarial cases.

Severity count:

- BLOCKER: 0
- MAJOR: 6
- MINOR: 0

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES
After the closure defects are fixed, consider reducing lifecycle duplication between Codex and Claude by extracting a small owned-process monitor primitive. Do not rewrite the mature Codex adapter merely for aesthetic symmetry.

The neutral adapter DTO may also be extended to expose provider-native identity/provenance in a provider-neutral form if that improves future M19/M20 consumers.

## 16. UNVERIFIED ITEMS
The builder's live Claude 2.1.270 help/auth probes and native publication are not independently executable from this GitHub-only audit environment. They remain claims pending owner-native acceptance after source remediation passes.

No owner-native Claude start/resume/stop acceptance should be requested while the source audit remains CHANGES_REQUIRED.

## 17. REGRESSION RISK
MEDIUM-HIGH until V02 closes the stop and resume lifecycle defects. The affected code owns external processes and durable session state, so incorrect behavior can strand processes or present false lifecycle truth even when compilation/tests are green.

## 18. AUDIT CONFIDENCE
HIGH for the identified source defects. They are directly observable in the production Rust control flow and do not depend on builder logs or external provider behavior.

## 19. FINAL VERDICT
CHANGES_REQUIRED.

M17 remains OPEN. Do not perform owner-native acceptance yet. Do not activate M18.

## 20. REQUIRED REMEDIATION
Execute one bounded M17 V02 remediation package covering F-M17-V01-001 through F-M17-V01-006.

The remediation must preserve:

- accepted M16 Codex audit-provider behavior;
- exact eight-project portfolio and FormuLab@main tracking;
- local Claude Code CLI only;
- Claude-managed login only;
- no `ANTHROPIC_API_KEY` or direct Anthropic HTTP transport;
- no auth-file inspection or GUI/browser automation;
- safe sync-first GitHub governance;
- M18 blocked state.

After V02 builder completion, ChatGPT must perform a new independent strict audit before any owner-native Claude workflow acceptance.