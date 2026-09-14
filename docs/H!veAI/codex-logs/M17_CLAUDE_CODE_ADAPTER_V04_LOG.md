# M17 Claude Code Adapter V04 Builder Log

## Scope and governance

- Work code: M17.
- Version: V04 strict remediation.
- Repository: `Sekiph82/H-veAI`.
- Branch: `main`.
- Required scope: close F-M17-V03-001 through F-M17-V03-005 only.
- Starting synchronized SHA after safe fast-forward: `37bce12f65a8b7c4811534b3894b23e825cdfc40`.
- V04 tracker-transition SHA: `62fdedebb64bdf28d4fc00888f3750dd0f10e6e3`.
- Implementation SHAs: `762e371e36d72150f76b081c8132946a11d993f1`, `e121f5945d628ce33c14485fc7b7bcabc8d66157`, `91c36278b8ffbdaa4b665dd5134a361f93be0f0d`.

Synchronization was GitHub-first and non-destructive. The checkout was inspected,
clean, and strictly behind `origin/main`; only `git merge --ff-only origin/main`
was used. No reset, rebase, force-push, stash, clean, destructive checkout, or
owner-work reconciliation was performed.

## F-M17-V03-001 — action truth and session-level resume eligibility

Resume eligibility is now evaluated by one backend predicate using provider
identity, claimed/owned state, terminal lifecycle state, registered project,
task relationship, and exact canonical project cwd. Project and session list/load
DTOs use that same predicate, so `supportsResume` cannot be broader than the
actual resume contract. Stop truth is separately exposed as `canStop` and is
derived from current backend ownership plus a stop-eligible state.

The frontend uses `canStop` for the Claude live action. This removes stale Stop
actions after ownership ends while preserving exact Resume only where the
backend proves the full contract.

Focused evidence: `exact_claude_resume_requires_provider_identity_and_canonical_cwd`,
`resume_claim_guard_releases_on_every_pre_starting_return`, and the M17 focused
frontend suite passed.

## F-M17-V03-002 — resume claim cleanup on database-open failure

Resume claims are guarded by `ClaudeResumeClaimGuard`. Every pre-starting return,
including the database-open failure boundary, releases the claim through the
guard. The guard is disarmed only when process ownership has been transferred to
the running-session monitor.

Focused evidence: `resume_claim_guard_releases_on_every_pre_starting_return` and
the production resume pipeline failure-injection matrix, including DATABASE_OPEN,
SPAWN, STDIN, PROMPT_WRITE, STDOUT, STDERR, OWNERSHIP, and RUNNING boundaries.

## F-M17-V03-003 — terminalization after provider process exit

Attention states are finalized as `FAILED` when the Claude process exits, while
the actionable attention diagnostic code/message remains durable. Ownership is
removed, stale Stop is unavailable, and `supportsResume` is recomputed from the
same exact provenance/project/task/cwd contract. Live state clears the prior
diagnostic fields; migration-backed current diagnostic fields are authoritative
for final session loading.

Focused evidence: `attention_process_exit_is_terminal_actionable_and_preserves_cause`.

## F-M17-V03-004 — bounded authoritative attention diagnostics

Control-state diagnostics now use an independent bounded control-event budget of
32 entries, separate from the generic stdout/stderr event budget of 128 entries.
Attention diagnostics therefore remain authoritative under output pressure while
stdout/stderr readers continue draining and finalization remains unblocked.
Migration 25 adds durable current diagnostic code/message columns.

Focused evidence: `control_diagnostic_survives_generic_output_budget_and_remains_bounded`
and the migration regression suite.

## F-M17-V03-005 — production-boundary adversarial coverage

The V04 tests exercise the production resume pipeline and lifecycle boundaries,
not only helper functions: eligibility/list/load parity, claim cleanup at every
injected setup stage, process-exit terminalization, bounded diagnostic retention,
frontend Stop/Resume action truth, and preservation of provider identity and
project/task/cwd provenance. The existing V01-V03 lifecycle, Codex adapter, M16
Codex-only audit-provider, exact eight-project portfolio, and
`Sekiph82/FormuLab@main` tracking regressions remain covered.

## Regression, security, and publication gates

- Full serialized Rust library regression: 471 passed, 0 failed, 0 ignored.
- Full frontend Vitest regression: 18 files, 141 tests passed.
- Focused M17 frontend suite: 2 passed.
- `npm run typecheck`: passed.
- `cargo check --manifest-path src-tauri/Cargo.toml`: passed.
- `npm run build`: passed.
- `git diff --check`: passed.
- Active-source `ANTHROPIC_API_KEY`: 0 matches.
- Active-source direct Anthropic HTTP/API transport: 0 matches.
- Claude authentication remains local-CLI operation only; no credential-file
  inspection, GUI/browser automation, blanket permission bypass, or automatic
  Claude installation/update was introduced.
- M16 Codex-only audit-provider and direct OpenAI API transport guardrails:
  passed with 0 prohibited active-source matches.
- Exact eight-project portfolio and `Sekiph82/FormuLab@main` tracking:
  regression coverage passed.
- Governed native QA publication: passed through
  `scripts/publish-dev-qa.ps1` using a production `--no-bundle` build.
- Final published executable SHA-256:
  `F9FE9F5A06DF0D723C130DEAFE336F590ADAE2CC6683DF8BBC932B9FCCDC7D12`.
- Stable executable: `dev-bin/H!veAI.exe`.
- Stable shortcut target: `dev-bin/H!veAI.exe`.
- Stable shortcut icon: `dev-bin/H!veAI.ico,0`.
- Native smoke checks passed with no visible console and no dev-server ports
  5173 or 8765.

## Preserved boundaries and final tracker state

- Claude integration uses only the locally installed Claude Code CLI and the
  owner's Claude-managed login.
- No `ANTHROPIC_API_KEY`, direct Anthropic transport, auth-file inspection,
  GUI/browser automation, blanket permission bypass, or M18 work was introduced.
- Accepted V01-V03 fixes, all accepted M00-M16 behavior, and the Codex-only M16
  audit provider are preserved.
- M16 remains PASS/CLOSED and M16T remains validated complete.
- M17 remains `[~]` and is
  `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`.
- M18 remains planned/blocked and was not activated.

The log is committed separately after implementation, regression, and native
publication evidence. The final completion check verifies local HEAD,
`origin/main`, and live GitHub `main` equality plus a clean worktree after this
log is published.
