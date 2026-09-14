# M17 Claude Code Adapter V05 — Independent Strict Re-Audit

## 1. VERDICT

**PASS — source/remediation acceptance complete; owner-native Claude workflow acceptance remains required before M17 may close.**

No BLOCKER, MAJOR, or MINOR source defect was found in the bounded V05 remediation scope. M18 must remain blocked until owner-native acceptance closes M17.

## 2. CONTRACT RECOVERY

V05 was required to close only F-M17-V04-001 through F-M17-V04-003 while preserving accepted M17 V01-V04 work and all accepted M00-M16 behavior.

The recovered V05 contract required:

1. current Claude diagnostic truth to follow resume, RUNNING recovery, attention transitions, stop failure, terminalization, and restart reconciliation;
2. every direct Claude mutation result to project current process ownership / resume-claim truth, not stale durable-state assumptions;
3. production-boundary Stop escalation success/failure/retry coverage and end-to-end dual-budget stream drain/finalization coverage;
4. no Anthropic API-key/direct-HTTP/auth-file/GUI/browser/blanket-permission-bypass fallback;
5. no M18 activation;
6. governed native QA publication only after regression gates.

M17 overall still requires owner-native readiness/start/stop/exact-resume acceptance after an independent source PASS.

## 3. BRANCH / HEAD / DIFF SCOPE

- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- V05 prompt baseline: `caa4c746f15b3a33b8c163e40f76c1ececdab22f`
- Tracker-transition commit: `13357efd119c2ee7d3e97ad92d0314443503b1c6`
- Implementation/test commit: `e28d8f4db643056526ea73954940253d77bda6e5`
- Builder-log commit / audited pre-audit HEAD: `d1f20db781c0f39915c587b7b55fe64ba99ae7c7`

The V05 range is three commits ahead of the prompt baseline. Substantive production/test work is concentrated in `src-tauri/src/agent_session_center.rs`; tracker/roadmap changes and the immutable V05 builder log complete the range.

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Independent evidence |
|---|---|---|
| Resume clears stale current diagnostic | PASS | Resume STARTING transition clears both current diagnostic columns; focused production-boundary test exercises prior `CLAUDE_USAGE_LIMITED` truth. |
| Same-state RUNNING can clear stale diagnostic | PASS | Live-state SQL updates when state **or diagnostic fields** differ, while identical state+diagnostic remains a no-op. |
| Restart reconciliation publishes current orphan/recovery diagnostic | PASS | Reconcile writes `CLAUDE_PROCESS_NOT_OWNED_AFTER_RESTART` while prior immutable events remain historical evidence. |
| Direct Start action projection truthful | PASS | Claude Start returns through current ownership/claim projection. |
| Direct Resume action projection truthful | PASS | Resume returns through current ownership/claim projection before monitor handoff completes. |
| Direct Retry action projection truthful | PASS | Claude Retry reloads through current projection; Codex retains its existing path. |
| Stop failure returned DTO truthful | PASS | Failed/unconfirmed Stop preserves ownership, resets stop-request policy, persists retryable diagnostic, and returns current action truth. |
| Confirmed-exit session does not advertise Stop | PASS | Current projection checks actual child liveness rather than durable-state inference. |
| Production Stop escalation failure + retry exercised | PASS | Real `stop()` path uses a narrow test-only termination-outcome seam; failure preserves owned process/retry truth and later injected success completes through monitor/finalizer. |
| Stop finalizes once | PASS | Production monitor tests assert one `SESSION_FINISHED`. |
| Dual output/control budgets remain bounded | PASS | Production stream/finalization test saturates both budgets and asserts bounded ordinary/control event counts. |
| Budget saturation does not block drain/finalization | PASS | Same production test reaches `COMPLETED`, persists final response/`ended_at`, removes ownership, and clears current diagnostic. |
| Prior M17 architecture preserved | PASS | No incompatible provider redesign observed in V05 diff. |
| M16 Codex-only architecture preserved | PASS | No V05 source change in M16 provider path. |
| M18 remains blocked | PASS | Builder log/tracker state retain M17 `[~]` and M18 blocked. |
| Owner-native workflow acceptance | UNVERIFIED | Must be performed by the owner after this independent PASS. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

The central V05 builder claims match repository source truth:

- `e28d8f4...` introduces actual-process liveness projection and a shared current-session projection;
- direct Claude Start, Resume, Retry, Stop, and List use current projection where relevant;
- exact Resume clears current diagnostic fields before re-entering RUNNING;
- restart reconcile writes an explicit current recovery diagnostic;
- production-path deterministic tests for mutation action truth, Stop escalation failure/retry, and dual-budget drain/finalization are present.

The builder claims `37` focused Rust tests, `475` full serialized Rust tests, `141` frontend tests, typecheck/check/build success, publication success, and executable SHA-256 `A1AEEF85AD37E8D338481E4D8C08892793BD4E150A603666C0BB34DB20C682A5`. Those execution counts/publication results remain builder evidence because the implementation commit has no independent GitHub status-check evidence.

## 6. FILE / SYMBOL EVIDENCE

Primary audited implementation: `src-tauri/src/agent_session_center.rs`.

Key verified symbols/paths include:

- `claude_process_is_owned` — checks the owned child itself with bounded non-blocking liveness observation rather than inferring ownership from durable state;
- `load_current_session` — central current action projection using actual child truth plus owned-or-claimed truth;
- `start_claude` — returns `load_current_session`;
- `resume` — clears stale current diagnostics entering STARTING and returns a current projection;
- `retry` — uses current projection for Claude while preserving Codex behavior;
- `stop` — uses the real owned process, persists retryable failure truth, then returns current projection;
- `persist_live_claude_state` — updates when state or current diagnostic differs, allowing same-state RUNNING to clear stale diagnostics without repeated exact no-op transitions;
- `reconcile` — writes explicit ORPHANED recovery diagnostic;
- `terminate_owned_child_for_stop` — production termination by default, with only a `cfg(test)` deterministic outcome seam.

## 7. FOCUSED TEST EVIDENCE

Repository test bodies directly exercise the required V05 boundaries:

- `current_diagnostic_truth_clears_on_resume_and_reconcile`;
- `direct_claude_mutations_return_current_action_truth`;
- `production_stop_escalation_failure_is_retryable_and_later_stop_completes`;
- `production_dual_budget_stream_drains_and_finalizes`;
- previously added `production_stop_monitor_finalizes_once_and_preserves_unrelated_process` and attention-state production tests remain in the same source suite.

The direct-mutation test invokes production `start`, `resume`, `retry`, `list`, and `stop` paths rather than testing only DTO helpers.

## 8. REGRESSION EVIDENCE

The V05 diff is narrow and does not alter Codex audit-provider code, GitHub portfolio mapping, or FormuLab tracking. Existing M14/M17 frontend contracts remain structurally compatible because V05 does not change the frontend DTO shape introduced in V04.

Full command execution counts in the builder log are not independently replayed by this GitHub source audit and are therefore treated as claims, not independent CI proof.

## 9. SECURITY / SAFETY REVIEW

PASS.

V05 does not introduce an Anthropic HTTP transport, `ANTHROPIC_API_KEY`, credential/token-file inspection, GUI/browser automation, automatic Claude installation/update, shell-string prompt transport, or blanket permission bypass.

The new stop seam is compiled only for tests; normal production uses the pre-existing owned-child bounded termination function. PID ownership protection is not broadened.

## 10. ARCHITECTURE CONSISTENCY

PASS.

The implementation keeps local Claude Code CLI + owner-managed login as the provider boundary. Current action truth is backend-derived rather than duplicated in the frontend. Exact provider-session identity/project/task/cwd resume invariants remain centralized. Codex behavior remains separate where Claude ownership semantics do not apply.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

PASS for builder completion state.

`TASKS.md` correctly keeps M17 `[~]`, describes V05 implementation as complete, requires independent re-audit plus owner-native acceptance, and keeps M18 blocked. The V05 builder log explicitly identifies itself as builder evidence rather than independent acceptance.

After this independent PASS, canonical prospective truth should be interpreted as **M17 V05 source audit PASS / owner-native acceptance pending**. M17 must not be marked `[x]` until that human gate passes.

## 12. FINAL REPOSITORY STATE

The live `main` before this audit commit was `d1f20db781c0f39915c587b7b55fe64ba99ae7c7`, containing the tracker transition, implementation/test commit, and immutable builder log in linear order.

No independent GitHub status checks are attached to the implementation commit. This is an evidence limitation, not a contradictory failure signal.

## 13. OPEN CROSS-MILESTONE FINDINGS

No blocking cross-milestone defect is reopened by V05.

M18-M20 remain blocked by roadmap order until M17 owner-native acceptance closes.

## 14. DEFECTS BY SEVERITY

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- NOTE: 1 — builder-reported command/test/publication results lack independent GitHub CI status-check evidence.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Non-blocking future hardening opportunities:

- add CI/status-check coverage for the full Rust/frontend/typecheck/build gate so command-count claims can be independently verified from GitHub;
- retain the narrow current-session projection as the sole source for future Claude action fields to avoid endpoint-specific truth drift.

Neither is required for M17 native acceptance.

## 16. UNVERIFIED ITEMS

- Owner-native Claude readiness on the owner's actual workstation.
- Real owner-managed Claude authentication readiness.
- Real Claude Start -> Stop -> exact Resume behavior through the published executable.
- Builder command counts and published executable SHA as independently replayed CI evidence.

These are not converted into source PASS claims.

## 17. REGRESSION RISK

**MEDIUM-LOW.**

V05 touches concurrency/lifecycle projection code, but changes are narrow, fail-closed, and accompanied by direct production-boundary deterministic tests. Native provider behavior remains the remaining uncertainty.

## 18. AUDIT CONFIDENCE

**HIGH for source/remediation acceptance; MEDIUM for runtime acceptance.**

Repository source and focused test bodies directly demonstrate all three V05 remediation contracts. Runtime confidence remains intentionally lower until owner-native Claude execution is observed.

## 19. FINAL VERDICT

**PASS / AWAITING OWNER NATIVE ACCEPTANCE.**

F-M17-V04-001, F-M17-V04-002, and F-M17-V04-003 are closed at source/test-contract level. No additional Codex remediation prompt is warranted from this audit.

## 20. REQUIRED REMEDIATION

No source remediation required.

Next gate is HUMAN owner-native acceptance using the governed published H!veAI executable. At minimum verify:

1. Claude readiness is truthfully READY with the installed local Claude CLI and owner-managed login;
2. a registered-project Claude session starts in the correct canonical cwd;
3. the live session exposes truthful Stop action and bounded output/final-response evidence;
4. Stop reaches truthful STOPPED state without affecting unrelated processes;
5. the same persisted Claude provider session becomes eligible for **Resume exact session**;
6. exact Resume continues the same provider session under the same project/task/cwd provenance and again exposes truthful live action state;
7. no API key, direct Anthropic transport, browser/GUI automation, or permission-bypass fallback appears during the native workflow.

Only after this HUMAN gate passes may M17 be marked PASS/CLOSED and M18 activate.