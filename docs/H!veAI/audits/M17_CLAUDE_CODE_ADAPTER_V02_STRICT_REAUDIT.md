# M17 Claude Code Adapter V02 — Independent Strict Re-Audit

## 1. VERDICT
CHANGES_REQUIRED.

M17 V02 materially improves the V01 implementation and closes several original source defects, but it is not eligible for owner-native Claude acceptance yet. The new durable live-attention implementation creates new lifecycle contradictions around exact resume, restart reconciliation, bounded event persistence, and failed stop escalation. Required V02 adversarial coverage is also still incomplete.

M17 remains OPEN. M18 must remain blocked.

## 2. CONTRACT RECOVERY
Authoritative inputs reviewed:

- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V01_PROMPT.md`
- `docs/H!veAI/plans/M17_CLAUDE_CODE_ADAPTER_V01_PLAN.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V01_STRICT_AUDIT.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REMEDIATION_PROMPT.md`
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V02_LOG.md`
- root `TASKS.md`
- implementation commit `21f6eb57c6b80dbb2c8200a7a53d077467f4042a`
- final builder tracker commit `eaab418e68b8b3976cc5056f5aa6f89d6743db1e`
- builder log commit `d92190edbcd6f00b73c651a3f1418a29e7155213`
- current production source and focused tests on GitHub `main`.

Builder logs are claims, not acceptance evidence.

## 3. BRANCH / HEAD / DIFF SCOPE
Repository: `Sekiph82/H-veAI`

Branch: `main`

V02 tracker transition: `e0e80b1aa3a53ef3bf9e84d6176242d60aef547d`

Implementation: `21f6eb57c6b80dbb2c8200a7a53d077467f4042a`

Final builder tracker transition: `eaab418e68b8b3976cc5056f5aa6f89d6743db1e`

Builder log commit before this audit: `d92190edbcd6f00b73c651a3f1418a29e7155213`

The implementation changes the provider-neutral session DTO, Claude process lifecycle, auth classifier, provider-session identity handling, live attention persistence, resume setup cleanup, Codex DTO compatibility, and focused frontend/Rust tests.

## 4. ACCEPTANCE CRITERIA MATRIX
- Safe sync-first / GitHub-first transition: PASS by repository history and tracker state.
- M16 remains PASS/CLOSED: PASS.
- M18 not activated: PASS.
- Local Claude CLI only / no Anthropic API-key transport: PASS in reviewed source.
- V01 stop mutex deadlock remediation: PASS at source level.
- V01 resume setup cleanup architecture: PASS/PARTIAL; fail-closed helper exists, but required injection coverage remains incomplete.
- V01 readiness/auth fail-closed classifier: PASS at source level.
- V01 provider-session ID immutability: PASS at source level.
- Provider-neutral identity/provenance DTO: PASS.
- Durable live attention-state persistence: PARTIAL; persistence exists but introduces resume/reconcile/boundedness defects below.
- Exact-session resume safety: FAIL for still-owned waiting/attention sessions.
- Restart/orphan recovery: FAIL for several newly live attention states.
- Bounded event persistence: FAIL for live state events.
- Stop lifecycle terminal/degradation truth: FAIL/PARTIAL on unconfirmed termination.
- Required deterministic adversarial matrix: FAIL/PARTIAL.
- Governed native publication: builder claim only; not independently promoted to proof.
- Owner-native acceptance: NOT REACHED because source re-audit fails.

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH
The builder reports 19 focused Rust tests, 457 full Rust tests, 2 focused frontend tests, 141 full frontend tests, typecheck/build/cargo check, guardrail scans, and native publication. GitHub exposes no independent CI status checks for the implementation commit, so those execution counts remain builder claims.

Repository source does independently confirm real improvement: the blocking wait-under-mutex pattern is gone; auth READY now requires a successful auth process; provider-native identity is compare-and-set; live attention rows are persisted; and resume setup has a shared failure-cleanup helper.

However, source-level contradictions remain and are sufficient to block native acceptance regardless of builder pass counts.

## 6. FILE / SYMBOL EVIDENCE
Primary audited symbols:

- `src-tauri/src/agent_adapter.rs`
  - `AdapterSession`
- `src-tauri/src/agent_session_center.rs`
  - `monitor_owned_child`
  - `terminate_owned_child_bounded`
  - `claude_readiness`
  - `classify_claude_auth_probe`
  - `run_claude_session`
  - `accept_claude_provider_session_id`
  - `persist_live_claude_state`
  - `parse_claude_record`
  - `stop`
  - `resume`
  - `reconcile`
  - `load_session`
  - focused Rust tests
- `src/pages.tsx`
  - Claude session detail Resume/Stop controls
- `tests/m17-claude-adapter-focused.test.tsx`.

## 7. FOCUSED TEST EVIDENCE
V02 adds useful tests for:

- fail-closed auth classification;
- native owned-child termination without the previous wait-lock deadlock;
- one resume cleanup helper path;
- provider-session compare-and-set identity;
- live WAITING_PERMISSION / WAITING_USER persistence and return to RUNNING;
- missing provider identity / cwd mismatch;
- frontend rendering of a persisted WAITING_PERMISSION diagnostic.

The required matrix remains incomplete. No reviewed deterministic test proves the production `stop()` state machine reaches truthful terminal/degraded state, forced escalation evidence, stop-from-waiting, rejection of resume while a waiting/attention process is still owned, restart reconciliation for all live attention states, event-budget preservation under many RUNNING records, or failure injection across the important resume setup stages.

## 8. REGRESSION EVIDENCE
No direct regression was found in the accepted M16 Codex-only audit-provider source path. The neutral DTO extension gives Codex empty provider-native identity fields while preserving its existing adapter behavior.

The V02 regressions are localized to the Claude lifecycle/state model introduced or modified by M17.

## 9. SECURITY / SAFETY REVIEW
PASS on provider boundary, CHANGES_REQUIRED on lifecycle safety.

Positive source findings:

- no `ANTHROPIC_API_KEY` provider path was introduced;
- no direct Anthropic HTTP/API transport was introduced;
- no credential/auth-file inspection was introduced;
- prompts remain stdin transported rather than shell-string interpolated;
- `--dangerously-skip-permissions` is not used;
- provider-native session ID is treated as durable identity and no longer silently drifts.

Residual lifecycle defects can create duplicate concurrent exact-session processes, stale post-restart state, and unbounded persistence churn. Those are safety/correctness blockers for native acceptance.

## 10. ARCHITECTURE CONSISTENCY
PARTIAL.

V02 correctly chose the neutral DTO option and added provider session ID, provenance, and canonical provider cwd to `AdapterSession`.

The remaining architecture problem is that process ownership and semantic session state are not cleanly separated. Newly persisted waiting/attention states can still represent a live H!veAI-owned process, but resume eligibility is inferred mainly from provider session identity and a narrow state guard. Recovery likewise uses a hard-coded subset of live states. V03 should centralize explicit process-live and resume-eligible predicates rather than allow state lists to drift between stop/resume/reconcile/UI.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS
The root tracker correctly says M17 V02 is awaiting independent re-audit and owner-native acceptance. This audit changes prospective truth: V02 is `CHANGES_REQUIRED`; the next actor is CODEX for one bounded M17 V03 remediation package.

The V02 builder log overstates F-M17-V01-006 closure because required adversarial tests remain absent from the reviewed source.

## 12. FINAL REPOSITORY STATE
Before this audit artifact, live `main` was `d92190edbcd6f00b73c651a3f1418a29e7155213`.

No production source is modified by this audit.

## 13. OPEN CROSS-MILESTONE FINDINGS
M16 remains accepted and is not reopened.

M18 remains blocked until M17 passes independent strict source audit and owner-native Claude acceptance.

## 14. DEFECTS BY SEVERITY

### F-M17-V02-001 — MAJOR — Exact resume is allowed for sessions whose Claude process can still be owned and alive
V02 now durably persists live states such as `WAITING_PERMISSION`, `WAITING_USER`, `AUTH_REQUIRED`, `USAGE_LIMITED`, and `NETWORK_ERROR` while the provider process can remain alive.

`resume()` rejects only `RUNNING`, `STARTING`, and `STOPPING`. It does not reject those live waiting/attention states and it does not first reject an existing `claude_processes` ownership-map entry. `load_session()` sets `supports_resume` true for any Claude row with a provider session ID, independent of process ownership. The frontend renders `Resume exact session` whenever `supportsResume` is true.

Result: a still-owned Claude process that is waiting for permission/user attention, auth, quota recovery, or network recovery can be resumed into a second process against the same provider-native session identity.

Required remediation: separate provider-conversation resumability from current OS-process ownership. Resume must fail closed whenever the H!veAI session still owns a process, regardless of semantic state. Define one explicit resume-eligible predicate for non-owned/terminal-or-orphaned sessions, and use the same truth in backend/UI. Add deterministic tests for every live waiting/attention state.

### F-M17-V02-002 — MAJOR — Restart reconciliation omits newly live AUTH_REQUIRED / USAGE_LIMITED / NETWORK_ERROR states
`reconcile()` marks `STARTING`, `RUNNING`, `WAITING_PERMISSION`, `WAITING_USER`, and `STOPPING` Claude rows ORPHANED after restart. It does not include `AUTH_REQUIRED`, `USAGE_LIMITED`, or `NETWORK_ERROR`, even though V02 explicitly made those durable live states while a process can still be alive.

Result: after H!veAI restart, a lost process can remain indefinitely represented as AUTH_REQUIRED / USAGE_LIMITED / NETWORK_ERROR with no owned process, instead of truthful `ORPHANED` recovery. This also makes process truth depend on which attention label happened to be last.

Required remediation: centralize the set of states that may correspond to an owned live process and use it in reconciliation. On restart, stale rows in every such state must become ORPHANED without auto-spawn while preserving provider-session identity and prior diagnostic provenance. Add deterministic restart/reconcile tests for all live states and explicit post-reconcile resumability.

### F-M17-V02-003 — MAJOR — Live state persistence bypasses the bounded session-event budget
`run_claude_session()` already uses `persisted_event_count` plus `insert_bounded_event()` to cap structured stream/event persistence. V02 calls `persist_live_claude_state()` for each parsed state. That helper executes an UPDATE for every matching record and, whenever a row matches, calls unbounded `insert_event()` plus `materialize_best_effort()`.

The UPDATE does not require the state to actually change. Therefore repeated assistant/tool/stream records classified as `RUNNING` can generate an unbounded sequence of `SESSION_STATE` rows and repeated control-plane materializations outside `MAX_OUTPUT_EVENTS`.

Result: the M17 bounded-output/event contract can be defeated by an ordinary long structured Claude stream even though raw stream events themselves are capped.

Required remediation: persist/materialize only real state transitions, and keep live-state diagnostic events inside a bounded session-event policy or an explicitly bounded/coalesced state-transition budget. Continue draining provider pipes after caps. Add a deterministic high-record-count test proving event rows and materialization-triggering transitions remain bounded.

### F-M17-V02-004 — MAJOR — Failed bounded stop escalation can leave durable STOPPING indefinitely
`stop()` first persists `STOPPING`, then calls `terminate_owned_child_bounded()`. If escalation is required it records `STOP_ESCALATED`, including `exited:false` when the process still has not exited after the escalation timeout. The function then simply returns the reloaded session. There is no source path that restores a truthful live state, emits a stop-failed diagnostic, resets stop intent, or otherwise prevents an unconfirmed termination from remaining `STOPPING` indefinitely if the owned process never exits.

This directly contradicts the V02 requirement that monitor/stop races must not leave a session permanently STOPPING.

Required remediation: explicitly model bounded stop failure. If termination is not confirmed, preserve process ownership, emit a bounded stop-failed diagnostic, transition out of transient STOPPING into a truthful retryable live/degraded state, and keep monitor behavior consistent. Do not claim STOPPED while the child is alive. Add deterministic injected termination outcomes including forced escalation success and escalation failure.

### F-M17-V02-005 — MAJOR — Required V02 adversarial lifecycle matrix is still incomplete
The V02 prompt explicitly required production-state tests for stop terminal truth, forced escalation evidence, stop-from-waiting, important resume setup failure stages, restart/orphan explicit resume, and the original lifecycle invariants.

Reviewed V02 tests cover the low-level termination helper and one `fail_resume_setup(..., "PROMPT_WRITE", ...)` helper invocation, but do not exercise the actual production `stop()` terminal state machine, forced escalation branch, unconfirmed-stop behavior, live-state resume exclusion, restart reconciliation of newly live attention states, or high-volume live-state event bounding. The single cleanup helper test does not demonstrate spawn/stdin/stdout/stderr/ownership/RUNNING failure handling through the real resume setup path.

Required remediation: complete the deterministic matrix around the actual production lifecycle boundaries, preferably through injectable process/setup primitives rather than live Claude quota.

Severity count:

- BLOCKER: 0
- MAJOR: 5
- MINOR: 0

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES
After closure, consider extracting one small shared lifecycle vocabulary/predicate layer for:

- process-live states;
- resumable states;
- reconcile-on-restart states;
- stoppable states.

This would reduce repeated string-state lists drifting apart. Do not rewrite the mature Codex adapter merely for symmetry.

## 16. UNVERIFIED ITEMS
The builder's Claude 2.1.270 environment probes, full local regression execution, EXE hash, shortcut verification, and native publication cannot be independently executed from this GitHub-only audit environment. They remain claims pending owner-native acceptance after source remediation passes.

No owner-native Claude start/stop/resume acceptance should be requested while this source audit remains CHANGES_REQUIRED.

## 17. REGRESSION RISK
HIGH until V03 closes the ownership/resume and recovery contradictions. Duplicate exact-session processes or stale recovery state can corrupt session truth even when the provider itself behaves correctly. Unbounded state-event persistence can also create avoidable database/control-plane churn during normal long sessions.

## 18. AUDIT CONFIDENCE
HIGH for F-M17-V02-001 through F-M17-V02-004 because each follows directly from production control flow. HIGH for the test-gap finding because the required cases are explicit in the V02 prompt and absent from the reviewed focused source.

## 19. FINAL VERDICT
CHANGES_REQUIRED.

M17 remains OPEN. Do not perform owner-native Claude acceptance yet. Do not activate M18.

## 20. REQUIRED REMEDIATION
Execute one bounded M17 V03 remediation package covering F-M17-V02-001 through F-M17-V02-005 only.

Preserve:

- the accepted V02 fixes for stop mutex deadlock, auth fail-closed behavior, provider-session ID immutability, resume cleanup architecture, and neutral identity DTO;
- accepted M16 Codex audit-provider behavior;
- exact eight-project portfolio and FormuLab@main tracking;
- local Claude Code CLI only;
- Claude-managed login only;
- no `ANTHROPIC_API_KEY` or direct Anthropic HTTP transport;
- no auth-file inspection or GUI/browser automation;
- safe sync-first GitHub governance;
- M18 blocked state.

After V03 builder completion, ChatGPT must perform a new independent strict audit before any owner-native Claude workflow acceptance.