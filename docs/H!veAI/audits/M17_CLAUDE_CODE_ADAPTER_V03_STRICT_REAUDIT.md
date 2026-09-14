# M17 Claude Code Adapter V03 — Independent Strict Re-Audit

## 1. VERDICT
CHANGES_REQUIRED.

M17 V03 materially improves the V02 lifecycle model and closes the five V02 findings at the primary mechanism level: live-state vocabulary is centralized, concurrent exact-resume against an owned session is blocked, restart reconciliation covers the complete governed live-state set, live-state events are coalesced/capped, and failed stop escalation is represented as retryable rather than permanently STOPPING.

However, M17 is still not eligible for owner-native Claude acceptance. The re-audit found four production lifecycle defects and one remaining deterministic-evidence gap. The residual defects concern session-level resume eligibility truth, resume-claim cleanup, process-exit handling for attention states, and preservation of authoritative attention diagnostics after the generic output-event budget is exhausted.

M17 remains OPEN. M18 must remain blocked.

## 2. CONTRACT RECOVERY
Authoritative inputs reviewed:

- `AGENTS.md`
- root `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/plans/M17_CLAUDE_CODE_ADAPTER_V01_PLAN.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V01_PROMPT.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V01_STRICT_AUDIT.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REMEDIATION_PROMPT.md`
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V02_LOG.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REAUDIT.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V03_STRICT_REMEDIATION_PROMPT.md`
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V03_LOG.md`
- implementation commit `5eda604bf73024229bfae88e2704a01cd5235e47`
- additional lifecycle/test-evidence commit `6d10b53f2783d082fe9133da29529e11ad3303b8`
- current production `src-tauri/src/agent_session_center.rs`
- current Agents frontend and `tests/m17-claude-adapter-focused.test.tsx`
- current Tauri startup/reconcile wiring.

Builder logs are claims, not acceptance evidence.

## 3. BRANCH / HEAD / DIFF SCOPE
Repository: `Sekiph82/H-veAI`

Branch: `main`

V03 tracker transition: `3dc8fd749370c176b94db99d4553c788d619e17f`

Primary implementation: `5eda604bf73024229bfae88e2704a01cd5235e47`

Additional implementation/test evidence: `6d10b53f2783d082fe9133da29529e11ad3303b8`

Final builder tracker state before log: `a98d41d82b3c739ae1bad789086df5736daf5493`

Builder log / live-main SHA before this audit: `36ff7e7aabce3409a4fc941f051cc6ff713a8d7e`

V03 production changes are bounded to the Claude lifecycle/session center plus the Agents live-state presentation and focused tests. No accepted M16 Codex audit-provider production source was intentionally redesigned.

## 4. ACCEPTANCE CRITERIA MATRIX
- Safe sync-first / GitHub-first builder transition: PASS by repository history and V03 log claim consistency.
- M16 remains PASS/CLOSED: PASS.
- M18 remains blocked: PASS.
- Local Claude CLI only / no Anthropic API-key transport: PASS in reviewed source boundary.
- V02-001 owned-process versus exact-resume separation: PASS/PARTIAL; duplicate live-process resume is blocked, but session-level `supportsResume` can still overstate actual resumability.
- V02-002 complete restart reconciliation: PASS at source level.
- V02-003 coalesced/bounded live-state persistence: PASS/PARTIAL; state transitions are coalesced/capped, but critical attention diagnostics can be silently dropped after the generic event budget is exhausted.
- V02-004 retryable stop-failure truth: PASS/PARTIAL; retryable failure state exists, but required production terminal-monitor evidence remains incomplete.
- V02-005 adversarial lifecycle matrix: FAIL/PARTIAL; several tests still exercise helpers rather than injectable production boundaries.
- Canonical provider session ID immutability: PASS.
- Fail-closed Claude auth readiness: PASS.
- Resume cleanup helper architecture: PASS/PARTIAL; a new resume-claim leak exists before the STARTING transaction.
- Process-exit/live-state consistency: FAIL for attention-state process exit.
- Frontend action truth: FAIL/PARTIAL because action availability is state-derived without full resume prerequisites and can show Stop after the provider process is already gone.
- Governed native publication: builder claim only; not independently promoted to proof.
- Owner-native acceptance: NOT REACHED because source re-audit fails.

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH
The builder reports 28 focused M17 Rust tests, 466 full Rust tests, 141 frontend tests, typecheck/build/cargo-check success, security scans, and governed native publication with executable SHA-256 `EA33E891CFDC26180DF1FDB6299A1C113197836C4EEB19BC5088F39D9016F23C`.

GitHub exposes no independent CI status checks for the current main SHA, so execution counts and native publication remain builder claims until owner-native closure.

Repository source independently confirms substantial V03 progress:

- one centralized backend lifecycle vocabulary now covers STARTING, RUNNING, all five attention states, and STOPPING;
- exact resume rejects an existing ownership-map entry and uses a per-session resume claim;
- restart reconcile now uses the centralized live-state predicate;
- repeated identical live-state records use `state != ?2` and a bounded transition counter;
- failed stop escalation restores a retryable live state and records `STOP_FAILED`;
- frontend hides Resume for a persisted live WAITING_PERMISSION session.

The findings below arise from current production control flow, not from disagreement with builder prose.

## 6. FILE / SYMBOL EVIDENCE
Primary audited symbols:

- `src-tauri/src/agent_session_center.rs`
  - `CLAUDE_PROCESS_LIVE_STATES`
  - `CLAUDE_RESUME_ELIGIBLE_STATES`
  - `claude_session_is_owned_or_claimed`
  - `claim_claude_resume`
  - `release_claude_resume_claim`
  - `run_claude_session`
  - `persist_live_claude_state`
  - `stop`
  - `resume`
  - `persist_stop_failure`
  - `persist_stop_termination_outcome`
  - `reconcile`
  - `load_session_with_ownership`
  - V03 focused Rust tests
- `src/pages.tsx`
  - `CLAUDE_LIVE_SESSION_STATES`
  - `isClaudeLiveSessionState`
  - `isLiveAgentSession`
  - Agent Stop/Resume action rendering
- `src/agentSessionCenter.ts`
  - `AgentSession.supportsResume`
- `tests/m17-claude-adapter-focused.test.tsx`
- `src-tauri/src/lib.rs`
  - startup-only Claude reconciliation wiring.

## 7. FOCUSED TEST EVIDENCE
V03 adds useful deterministic coverage for:

- centralized state predicates;
- owned WAITING_PERMISSION / WAITING_USER / AUTH_REQUIRED / USAGE_LIMITED / NETWORK_ERROR resume rejection;
- 1,000 no-op RUNNING records and 1,000 alternating attention/RUNNING helper transitions;
- restart reconciliation across all eight live states;
- helper-level stop-failure restoration;
- helper-level injected escalation success/failure recording;
- production `stop()` entry from each live attention state;
- stage-name cleanup helper calls for SPAWN / STDIN / PROMPT_WRITE / STDOUT / STDERR / OWNERSHIP / RUNNING;
- frontend hide-Resume/show-Stop behavior for WAITING_PERMISSION;
- eligible ORPHANED frontend Resume dispatch.

The matrix still does not satisfy the V03 prompt's production-boundary requirement. The stop tests do not create a complete `run_claude_session` monitor/finalizer and prove STOPPED after production Stop. The resume stage matrix directly invokes `fail_resume_setup` rather than injecting failures through the actual `resume()` setup pipeline. There is also no test for event-budget exhaustion followed by an authoritative attention transition, no test for the post-claim database-open failure, and no test for an exited attention-state process becoming actionably terminal/resumable.

## 8. REGRESSION EVIDENCE
No direct source regression was found in the accepted M16 Codex-only audit-provider path. V03 changes remain localized to M17 Claude lifecycle/frontend state behavior.

The exact-eight-project portfolio and FormuLab@main preservation are reported green by the builder, but those execution results remain builder evidence because no GitHub CI status is attached to the current main SHA.

## 9. SECURITY / SAFETY REVIEW
PASS on provider and credential boundary; CHANGES_REQUIRED on lifecycle truth.

Positive source findings:

- no reviewed `ANTHROPIC_API_KEY` provider path;
- no direct Anthropic HTTP/API provider transport;
- no Claude credential/auth/token-file inspection;
- prompt transport remains direct argv + bounded stdin, not shell-string interpolation;
- no `--dangerously-skip-permissions` blanket bypass;
- exact provider session identity remains compare-and-set and canonical;
- owned-process termination remains scoped to the H!veAI-owned PID/process tree.

The remaining defects are lifecycle/truthfulness defects rather than provider-secret defects.

## 10. ARCHITECTURE CONSISTENCY
PARTIAL.

V03 improves architecture by centralizing backend live/resume/reconcile/stop state semantics. That is the correct direction.

Two consistency gaps remain:

1. `supportsResume` is still computed from a subset of prerequisites rather than from the same full resume contract used by `resume()`.
2. semantic attention state and OS-process ownership are still conflated in the frontend. `isLiveAgentSession()` treats every governed attention state as live even when the monitor has finalized and removed ownership.

M17 closure requires one session-level eligibility truth source that remains correct after normal process exit, restart recovery, ownership loss, cwd/task/project drift, and bounded event exhaustion.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS
The root tracker and roadmap correctly leave M17 `[~]` and M18-M20 blocked while awaiting independent V03 re-audit.

This audit changes prospective truth: V03 is `CHANGES_REQUIRED`. The next actor is CODEX for one bounded V04 remediation package. M16 remains PASS/CLOSED and is not reopened.

The V03 builder log overstates closure of the adversarial matrix because helper-level failure injection is presented as though it exercised the production resume setup pipeline, and production stop tests do not prove monitor-finalized STOPPED truth.

## 12. FINAL REPOSITORY STATE
Live `main` before this audit artifact: `36ff7e7aabce3409a4fc941f051cc6ff713a8d7e`.

No production source is modified by this audit.

## 13. OPEN CROSS-MILESTONE FINDINGS
M16 remains accepted and PASS/CLOSED.

M18 remains blocked until M17 passes independent strict source re-audit and owner-native Claude acceptance.

No M18 implementation should begin from the current V03 state.

## 14. DEFECTS BY SEVERITY

### F-M17-V03-001 — MAJOR — `supportsResume` can advertise an exact resume that the backend will deterministically reject
`load_session_with_ownership()` currently sets `supports_resume` when all of the following are true:

- provider is CLAUDE;
- provider session ID exists;
- durable state is in STOPPED / FAILED / COMPLETED / ORPHANED;
- `process_owned` is false.

It does not validate the other prerequisites that `resume()` itself requires: active/available registered project root, task-to-project relationship, and equality of `provider_cwd_identity` with the current canonical project cwd.

The current focused test explicitly constructs an ORPHANED row with a wrong provider cwd and proves `resume()` returns `CLAUDE_RESUME_CWD_MISMATCH`, but the same row still satisfies the current `load_session_with_ownership()` `supports_resume` formula because provider identity/state/no-ownership are sufficient there.

Result: the Agents UI can render an enabled `Resume exact session` button for a session that the backend already knows is not currently resumable. The V03 contract explicitly required session-level resume eligibility to reflect provider identity plus project/task/cwd provenance, not merely provider capability/state.

Required remediation: create one backend session-level resume-eligibility evaluator used both by list/DTO projection and by `resume()` preflight. It must fail closed on project status/path availability, task relationship, canonical cwd mismatch, missing provider ID, ineligible state, existing process ownership, or active resume claim. The UI should consume this single current eligibility signal. Add deterministic list/UI tests for wrong cwd, missing/mismatched task, unavailable project, owned session, valid ORPHANED, and valid terminal rows.

### F-M17-V03-002 — MAJOR — A resume claim can leak permanently on a post-claim database-open failure
`resume()` calls `claim_claude_resume(center, session_id)` and then performs a second ownership check. After the claim is acquired, it constructs provenance and executes:

`let mut connection = database.open_connection()?;`

That fallible operation uses a direct `?`. If it fails, control returns before `transition_result` and before `fail_resume_setup()`, so `release_claude_resume_claim()` is never called.

Result: the in-memory session remains claimed even though no child was spawned and no STARTING transition occurred. `claude_session_is_owned_or_claimed()` then reports the session as unavailable for resume for the remainder of the process lifetime, and list/UI truth is falsely degraded until application restart.

Required remediation: make the resume claim RAII/guard-owned or otherwise guarantee release on every post-claim exit path. Add deterministic failure injection immediately after claim acquisition, including database-open / pre-STARTING failure, and prove the claim is released and the durable row remains unchanged/truthful.

### F-M17-V03-003 — MAJOR — A provider process can exit while an attention state remains durable, leaving a non-owned session with no valid action
`run_claude_session()` gives live attention states priority during terminal-state selection. If the last accepted provider state is WAITING_PERMISSION, WAITING_USER, AUTH_REQUIRED, USAGE_LIMITED, or NETWORK_ERROR, the final state remains that attention state even after the child process has exited. `finalize_claude()` sets `ended_at`, and `run_claude_session()` then removes the ownership-map entry.

V03 simultaneously defines those attention states as process-live / stop-eligible / non-resumable. The frontend's `isLiveAgentSession()` therefore renders Stop for them based on state alone. With the process already exited and the map entry removed, Stop returns `AGENT_SESSION_NOT_OWNED`, while Resume is hidden because the durable attention state is not resume eligible.

Result: a normal provider exit after an attention/error record can produce an ended, unowned session that visually behaves as live but can neither be stopped nor exactly resumed. Restart happens to convert it to ORPHANED, but requiring an application restart to restore actionability is not acceptable lifecycle truth.

Required remediation: explicitly separate "attention while process is alive" from "process exited with attention/error as terminal cause." On process exit, persist a truthful actionable terminal/degraded state or make session-level resume eligibility ownership-aware in a way that does not label an unowned ended row as live. Preserve the original attention diagnostic as cause/provenance. Add deterministic monitor/finalization tests for each attention class followed by provider exit, and verify UI action truth without restarting the application.

### F-M17-V03-004 — MAJOR — Exhausting the generic output-event budget can silently drop the authoritative attention diagnostic
V03 correctly routes `persist_live_claude_state()` through `insert_bounded_event()` and adds `MAX_LIVE_STATE_TRANSITIONS = 32`. However, state-transition events share the same `persisted_event_count` used by raw/structured output events.

When `persisted_event_count` has already reached `MAX_OUTPUT_EVENTS`, a later real WAITING_PERMISSION / WAITING_USER / AUTH_REQUIRED / USAGE_LIMITED / NETWORK_ERROR transition still updates the durable `agent_sessions.state` and increments the live-transition counter, but `insert_bounded_event()` returns without persisting `ATTENTION_STATE`.

`load_session()` derives `diagnostic_code` and `diagnostic_message` for live attention primarily from those events. Therefore the UI can receive an authoritative attention state with its actionable diagnostic missing or stale precisely in a long session where the generic event budget was consumed first.

Result: boundedness is preserved at the cost of losing critical control-state evidence. The V03 contract required bounded state-transition diagnostics, not silent loss behind raw-output events.

Required remediation: reserve an independent bounded control/state-event budget, coalesce/replace a bounded latest-state record, or persist current diagnostic truth in a dedicated bounded session field. Raw output must never starve the small authoritative lifecycle/control channel. Add deterministic tests that first exhaust the generic output-event budget and then transition into and out of an attention state, proving state plus current diagnostic remain durable/actionable and pipe draining/finalization remain unaffected.

### F-M17-V03-005 — MAJOR — The required production-boundary adversarial matrix is still incomplete
The V03 prompt explicitly required tests around actual production lifecycle boundaries rather than only helper methods.

Current gaps:

- `production_stop_persists_and_handles_every_live_attention_state` manually inserts an owned child without launching the production monitor and asserts the immediate return is STOPPING; it does not prove monitor-finalized STOPPED truth.
- `injected_stop_outcomes_record_escalation_and_failure_truthfully` injects `TerminationResult` only into the persistence helper. Its escalation-success case deliberately remains STOPPING because no monitor/finalizer participates.
- `resume_setup_failure_injection_cleans_every_boundary_without_identity_drift` enumerates stage names but directly calls `fail_resume_setup()` with a synthetic child. It does not inject failure through SPAWN / STDIN / PROMPT_WRITE / STDOUT / STDERR / OWNERSHIP / RUNNING in the actual `resume()` setup state machine.
- no deterministic test exercises the post-claim failure from F-M17-V03-002;
- no deterministic test exercises attention-state process exit from F-M17-V03-003;
- no deterministic test exhausts generic output events before the critical state transition from F-M17-V03-004.

Result: the builder's passing counts do not independently prove the lifecycle invariants that V03 was specifically created to close.

Required remediation: introduce narrow test-only/injectable process/setup/termination hooks or small production primitives that allow the actual resume/monitor/stop state machines to be exercised without live Claude quota. Prove initial stop success -> monitor STOPPED, escalation success -> STOP_ESCALATED + monitor STOPPED, escalation failure -> retryable non-STOPPING owned state, every resume setup boundary cleanup through the production setup path, claim release, attention exit truth, and control-event survival after generic event-budget exhaustion.

Severity count:

- BLOCKER: 0
- MAJOR: 5
- MINOR: 0

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES
After closure, consider representing session-level action eligibility as a small derived structure rather than a single provider capability boolean, for example:

- `canStop`
- `canResume`
- bounded reason/diagnostic when false.

This would make Agents UI action truth less dependent on duplicating semantic state lists. Do not broaden V04 into a general UI redesign.

Also consider a small RAII claim guard for resume ownership; this is safer than remembering manual release calls on each future error branch.

## 16. UNVERIFIED ITEMS
The builder's full local test execution, Claude environment probes, native publication smoke test, executable hash, shortcut target, and console-suppression result cannot be independently executed from this GitHub-only audit environment.

They remain builder claims pending successful source closure and owner-native acceptance.

No owner-native Claude Start/Stop/Resume acceptance should be requested while this audit is CHANGES_REQUIRED.

## 17. REGRESSION RISK
MEDIUM-HIGH until V04 closes the residual lifecycle gaps.

The primary duplicate-process race from V02 is substantially improved, but current residual defects can still expose impossible UI actions, strand a session behind a leaked claim, strand an ended attention session until restart, or lose the diagnostic needed for owner action in long sessions.

## 18. AUDIT CONFIDENCE
HIGH for F-M17-V03-001 through F-M17-V03-004 because each follows directly from current production source control flow.

HIGH for F-M17-V03-005 because the V03 prompt explicitly requires production-boundary deterministic tests and the reviewed test source still invokes helpers/synthetic ownership rather than those actual boundaries.

## 19. FINAL VERDICT
CHANGES_REQUIRED.

M17 remains OPEN. Do not perform owner-native Claude acceptance yet. Do not activate M18.

## 20. REQUIRED REMEDIATION
Execute one bounded M17 V04 remediation package covering F-M17-V03-001 through F-M17-V03-005 only.

Preserve:

- all accepted V01/V02/V03 improvements not implicated by this audit;
- centralized Claude live/stop/reconcile/resume lifecycle semantics;
- stop wait-lock fix and bounded owned-process termination;
- fail-closed Claude auth readiness;
- immutable provider-native session identity;
- existing provider-neutral identity/provenance DTO;
- accepted M16 Codex-only audit provider behavior;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` tracking;
- local Claude Code CLI only;
- Claude-managed login only;
- no `ANTHROPIC_API_KEY` or direct Anthropic HTTP transport;
- no Claude credential-file inspection;
- no GUI/browser automation or blanket permission bypass;
- safe sync-first GitHub governance;
- M18 blocked state.

After V04 builder completion, ChatGPT must perform a new independent strict re-audit before any owner-native Claude workflow acceptance.