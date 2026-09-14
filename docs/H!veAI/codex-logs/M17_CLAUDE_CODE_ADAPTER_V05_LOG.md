# M17 Claude Code Adapter V05 Builder Log

## Scope and synchronization

- Work code: M17.
- Version: V05 strict remediation.
- Repository: `Sekiph82/H-veAI`.
- Branch: `main`.
- Required scope: close F-M17-V04-001 through F-M17-V04-003 only.
- Starting synchronized SHA after safe fast-forward: `caa4c746f15b3a33b8c163e40f76c1ececdab22f`.
- V05 tracker-transition SHA: `13357efd119c2ee7d3e97ad92d0314443503b1c6`.
- Implementation/test SHA: `e28d8f4db643056526ea73954940253d77bda6e5`.
- Exact implementation file changed: `src-tauri/src/agent_session_center.rs`.
- Final tracker files: `TASKS.md`, `CODEX_ROADMAP.md`.

The checkout was inspected before synchronization. It was clean and strictly
behind `origin/main` with divergence `0 2`; only `git merge --ff-only origin/main`
was used. No reset, rebase, force-push, automatic stash, `git clean`, destructive
checkout, or owner-work discard/reconciliation was performed.

## F-M17-V04-001 — current diagnostic truth across lifecycle transitions

The governed live-state transition now updates state and current diagnostic
columns together when either the state or diagnostic differs. Same-state
RUNNING records can therefore clear stale terminal/attention diagnostics without
creating duplicate no-op transitions. Exact resume clears current diagnostics
when entering STARTING, and a successful RUNNING record keeps them clear.

Restart reconciliation now writes explicit current recovery truth:
`CLAUDE_PROCESS_NOT_OWNED_AFTER_RESTART` with a bounded recovery message while
immutable prior attention evidence remains in the event history. Finalization
and stop-failure diagnostics remain unchanged and authoritative.

Focused production-boundary evidence:

- `current_diagnostic_truth_clears_on_resume_and_reconcile`.
- Existing live attention, resume, restart, finalization, and bounded-control
  tests remain green.

## F-M17-V04-002 — direct mutation-return action projection

One current projection path now reads actual process ownership and active resume
claim truth separately. Claude start, exact resume, retry, stop, and list results
use that projection. `canStop` is true only for an actually owned live child;
`supportsResume` continues to use the centralized exact-resume eligibility
predicate including active claims. A process already observed exited does not
advertise Stop, while an unconfirmed owned stop failure immediately returns a
retryable Stop projection with `supportsResume=false` and the stop-failed
diagnostic. Codex return behavior remains on its existing path.

Focused production-boundary evidence:

- `direct_claude_mutations_return_current_action_truth` covers Start, Resume,
  Retry, direct returned DTO action fields, and later list parity.
- `production_stop_escalation_failure_is_retryable_and_later_stop_completes`
  covers immediate failed-stop action truth and later completion.

## F-M17-V04-003 — production Stop escalation and dual-budget matrix

A narrow test-only termination-outcome seam is used by the real `stop()` path;
the production termination implementation and PID ownership protections remain
unchanged. Deterministic production-path coverage now proves initial/forced
escalation failure preserves ownership and retryable truth, a later Stop can
complete, escalation evidence is bounded, and the monitor finalizes exactly once.
Existing Stop coverage for RUNNING and all five attention states remains green,
including unrelated-process protection.

`production_dual_budget_stream_drains_and_finalizes` drives the actual
`run_claude_session` stream/finalization boundary with enough ordinary records
to saturate `MAX_OUTPUT_EVENTS`, enough attention/RUNNING transitions to saturate
the control budget, an attention transition after generic exhaustion, recovery to
RUNNING, a successful final result, and process exit. It proves pipes are still
drained, final response and `ended_at` persist, ownership is removed, state is
truthful, and ordinary/control event counts remain bounded.

## Regression, security, and publication gates

- Focused Agent Session Center Rust suite: 37 passed, 0 failed.
- Full serialized Rust library regression:
  `cargo test --manifest-path src-tauri/Cargo.toml --lib -- --test-threads=1`:
  475 passed, 0 failed.
- Full frontend Vitest regression: 18 files, 141 tests passed.
- Focused M14/M17 frontend regression: 11 tests passed.
- `npm run typecheck`: passed.
- `cargo check --manifest-path src-tauri/Cargo.toml`: passed.
- `npm run build`: passed.
- `git diff --check`: passed.
- `ANTHROPIC_API_KEY`: 0 active-source matches.
- Direct Anthropic HTTP/API transport: 0 active-source matches.
- `dangerously-skip-permissions`: 0 active-source matches.
- Claude auth boundary: source review found no credential/auth/token-file
  inspection or file-reading path; readiness uses the bounded local Claude CLI
  auth-status probe only.
- GUI/browser automation and automatic Claude install/update: not introduced.
- M16 Codex-only audit-provider direct OpenAI API guardrail scan: 0 matches.
- Exact eight-project portfolio and `Sekiph82/FormuLab@main` regression:
  passed in the full Rust regression.
- Governed native QA publication through
  `scripts/publish-dev-qa.ps1`: passed with production `--no-bundle` build.
- Published executable SHA-256:
  `A1AEEF85AD37E8D338481E4D8C08892793BD4E150A603666C0BB34DB20C682A5`.
- Stable executable: `dev-bin/H!veAI.exe`.
- Stable Desktop shortcut target: `dev-bin/H!veAI.exe`.
- Stable shortcut icon: `dev-bin/H!veAI.ico,0`.
- Native smoke checks passed with no visible console and no dev-server ports
  5173 or 8765.

## Preserved boundaries and final tracker state

- Local Claude Code CLI and the owner's Claude-managed login remain the only
  Claude integration boundary.
- No `ANTHROPIC_API_KEY`, direct Anthropic transport, credential-file
  inspection, GUI/browser automation, blanket permission bypass, or M18 work
  was introduced.
- Accepted V01-V04 fixes, all accepted M00-M16 behavior, and the Codex-only M16
  audit provider are preserved.
- M16 remains PASS/CLOSED; M16T remains validated complete.
- Exact eight-project portfolio and `Sekiph82/FormuLab@main` tracking remain
  unchanged.
- M17 remains `[~]` with status
  `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`.
- M18 remains planned/blocked and was not activated.

After this log is committed and pushed, the final completion command verifies
that local `HEAD`, `origin/main`, and live GitHub `main` are identical and that
`git status --short` is empty. The log is builder evidence, not independent
strict acceptance.
