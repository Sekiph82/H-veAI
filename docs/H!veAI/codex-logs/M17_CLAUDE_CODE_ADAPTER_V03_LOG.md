# M17 Claude Code Adapter V03 Builder Log

## Scope and governance

- Work code: M17.
- Version: V03.
- Repository: `Sekiph82/H-veAI`.
- Branch: `main`.
- Required scope: close F-M17-V02-001 through F-M17-V02-005 only.
- Starting synchronized SHA after the safe fast-forward: `f00827912316af7eca81da4eecacd51ad677d47f`.
- V03 tracker-transition SHA: `3dc8fd749370c176b94db99d4553c788d619e17f`.
- Implementation SHA: `5eda604bf73024229bfae88e2704a01cd5235e47`.
- Additional implementation/test-evidence SHA: `6d10b53f2783d082fe9133da29529e11ad3303b8`.

Synchronization was GitHub-first and non-destructive. The checkout was clean and
strictly behind `origin/main`; only `git merge --ff-only origin/main` was used.
No reset, rebase, force-push, stash, clean, destructive checkout, or owner-work
reconciliation was performed.

## F-M17-V02-001 — ownership versus resumability

The Claude lifecycle policy is centralized in `agent_session_center.rs`:

- `CLAUDE_PROCESS_LIVE_STATES` governs STARTING, RUNNING, WAITING_PERMISSION,
  WAITING_USER, AUTH_REQUIRED, USAGE_LIMITED, NETWORK_ERROR, and STOPPING.
- `CLAUDE_RESUME_ELIGIBLE_STATES` governs only STOPPED, FAILED, COMPLETED, and
  ORPHANED rows after provider identity, project/task, and canonical-cwd checks.
- Shared predicates govern process-live, stop-eligible, restart-reconcile, and
  exact-resume eligibility decisions.
- A per-session resume claim closes the race between the ownership check and
  process registration. Resume fails closed with
  `CLAUDE_RESUME_SESSION_STILL_OWNED` and never kills the existing process.
- Session listing derives `supportsResume` from the backend lifecycle policy and
  current ownership/claim state. The frontend has one Claude live-state helper;
  live attention rows expose Stop and do not expose enabled exact Resume.

Focused deterministic evidence:

- `centralized_lifecycle_policy_separates_owned_processes_and_resumability`.
- `owned_live_attention_states_reject_exact_resume_before_spawn` covers
  WAITING_PERMISSION, WAITING_USER, AUTH_REQUIRED, USAGE_LIMITED, and
  NETWORK_ERROR with an owned child and proves no resume spawn.
- `production_stop_persists_and_handles_every_live_attention_state` covers
  production stop persistence for all five attention states.
- Frontend M17 focus: 2 tests passed, including hidden Resume and visible Stop
  for a persisted permission-waiting session.

## F-M17-V02-002 — complete restart reconciliation

Restart reconciliation reuses the centralized process-live predicate and scans
all Claude rows. Every governed live state becomes ORPHANED without spawning or
resuming Claude. Provider session ID, provenance, project/task identity, cwd,
final-response data, and prior diagnostic events remain durable. One bounded
PROCESS_ORPHANED event and one control-plane dirty/materialization path are
recorded per reconciled session.

Focused deterministic evidence:

- `restart_reconcile_covers_every_owned_live_state_and_preserves_identity`
  covers STARTING, RUNNING, WAITING_PERMISSION, WAITING_USER, AUTH_REQUIRED,
  USAGE_LIMITED, NETWORK_ERROR, and STOPPING.
- The same test proves terminal/non-live states remain unchanged, provider
  identity survives, and reconciled rows are explicitly resumable.
- No startup reconciliation path spawns a provider process.

## F-M17-V02-003 — bounded live-state event persistence

`persist_live_claude_state` uses a SQL state-change guard, so identical records
are no-ops. Real live-state transitions use the existing `MAX_OUTPUT_EVENTS =
128` event budget plus an explicit `MAX_LIVE_STATE_TRANSITIONS = 32` cap for
state-event and materialization work. Database state still updates after the
transition cap, while stdout/stderr readers continue draining and finalization
is not blocked. Redaction remains before parsing and persistence.

Focused deterministic evidence:

- `live_state_event_persistence_is_coalesced_and_bounded` sends 1,000 repeated
  RUNNING records and 1,000 alternating attention/RUNNING records; identical
  input creates no event and transitions remain at or below 32.
- `live_attention_state_is_durable_and_can_return_to_running` proves the real
  WAITING_PERMISSION -> WAITING_USER -> RUNNING transitions remain visible.

## F-M17-V02-004 — truthful and retryable stop failure

Stop remains owned-process scoped and never removes ownership while the child is
unconfirmed alive. Confirmed termination remains monitor-finalized to STOPPED.
An unconfirmed bounded termination records `STOP_ESCALATED` when escalation was
attempted, clears stop intent, restores a truthful retryable live state, and
records `CLAUDE_STOP_FAILED_PROCESS_STILL_OWNED` in `STOP_FAILED`. STOPPING is
therefore transient rather than an indefinite completed-stop result. The monitor
remains active and later Stop can retry.

Focused deterministic evidence:

- `injected_stop_outcomes_record_escalation_and_failure_truthfully` injects
  confirmed escalation and unconfirmed escalation outcomes through the same
  production persistence path.
- `failed_stop_is_truthful_retryable_and_not_permanent_stopping` proves the
  stop-failed diagnostic and restoration to a retryable attention state.
- `production_stop_persists_and_handles_every_live_attention_state` proves
  stop requests for all five attention states.
- `owned_stop_reaches_process_without_wait_lock_and_preserves_unrelated_process`
  proves bounded owned termination and unrelated-process protection.

## F-M17-V02-005 — adversarial lifecycle matrix

The V03 matrix exercises production boundaries and shared cleanup primitives:

- ownership/resume rejection for all live attention states;
- restart reconciliation for all live states and terminal preservation;
- repeated-state coalescing, transition cap, and waiting-to-running visibility;
- production stop persistence, injected escalation success, injected escalation
  failure, stop-from-attention, and unrelated-process protection;
- resume setup stages SPAWN, STDIN, PROMPT_WRITE, STDOUT, STDERR, OWNERSHIP,
  and RUNNING;
- cleanup after each injected failure, no lingering owned process, no false
  STARTING/RUNNING row, and immutable provider session identity;
- malformed-record failure mapping, auth/quota/network fail-closed classifiers,
  exact canonical resume args, and redaction-before-persistence regressions;
- Codex adapter/start/stop/list/final-response and M16 audit-provider
  readiness/freeform structured-output regressions;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` tracking tests.

Focused Rust M17 V03 lifecycle suite: 28 passed, 0 failed.
Original V01/V02 focused Rust coverage remains green within that suite and the
full serialized library regression.

## Regression, security, and publication gates

- Full serialized Rust library regression: 466 passed, 0 failed, 0 ignored.
- Full frontend Vitest regression: 18 files, 141 tests passed.
- Focused frontend Agents/session regression: 2 passed.
- `npm run typecheck`: passed.
- `cargo check --manifest-path src-tauri/Cargo.toml`: passed.
- `npm run build`: passed.
- `git diff --check`: passed.
- Active-source `ANTHROPIC_API_KEY`: 0 matches.
- Active-source direct Anthropic HTTP/API transport: 0 matches.
- Active-source Claude auth/token-file inspection: 0 implementation matches;
  the sole auth-file text match is the pre-existing Codex regression-test name.
- Active-source GUI/browser automation and blanket permission bypass scans: 0.
- M16 OpenAI API-key/direct-HTTP audit-provider guardrail scan: 0 matches in
  `src-tauri/src`.
- Governed native QA publication: passed through
  `scripts/publish-dev-qa.ps1` with smoke-tested production `--no-bundle`.
- Final published executable SHA-256:
  `EA33E891CFDC26180DF1FDB6299A1C113197836C4EEB19BC5088F39D9016F23C`.
- Stable shortcut target: `dev-bin/H!veAI.exe`.
- Stable shortcut icon: `dev-bin/H!veAI.ico,0`.

## Preserved boundaries and final tracker state

- Local Claude Code CLI only and owner-managed Claude login only.
- No `ANTHROPIC_API_KEY`, direct Anthropic transport, credential-file
  inspection, GUI/browser automation, automatic Claude installation/update,
  shell-command prompt transport, or blanket permission bypass.
- Accepted V02 stop-lock, auth fail-closed, provider identity, resume cleanup,
  live attention, and neutral DTO behavior preserved.
- Accepted Codex-only M16 audit provider and M00-M16 behavior preserved.
- Exact eight-project portfolio and `Sekiph82/FormuLab@main` tracking preserved.
- M16 remains PASS/CLOSED and M16T remains validated complete.
- M17 remains `[~]` and is
  `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`.
- M18 remains planned/blocked and was not activated.
- Final tracker-state commit before this log: `a98d41d82b3c739ae1bad789086df5736daf5493`.

The log is committed separately after implementation and publication evidence.
The final completion command verifies local HEAD, `origin/main`, and live
GitHub `main` equality and a clean worktree after this log is published.
