# PRE-M18 Audit Evidence Authority X04 V05 Remediation Log

## Scope

- Executed only `docs/H!veAI/prompts/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V05_COMMAND_CENTER_AUTHORITY_BOUNDARY_REMEDIATION_PROMPT.md`.
- Authoritative input: `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V04_STRICT_AUDIT.md`.
- Closed only F-X04-V04-001 through F-X04-V04-004.
- M18 was not activated.
- Root `TASKS.md` and `CODEX_ROADMAP.md` were read-only and unchanged.

## Safe synchronization

- Starting checkout was clean on `main` at `033be3ce2a1ceae20540dcc0885144080c729ffc`.
- `origin/main` was two commits ahead after inspection; the checkout was fast-forwarded only to `64278cd21bc8c601a916607e3955de49c9f228a5`.
- No reset, rebase, force-push, automatic stash, `git clean`, destructive checkout, or owner-work discard was used.

## Findings closed

### F-X04-V04-001 — current versus historical evidence boundary

- Removed the outer Command Center snapshot merge that promoted audit/test/permission/agent evidence into current `attention` and `work_queue`.
- Retained activity/history and domain evidence surfaces without treating them as current task truth.
- Current attention, queue, KPI, and Engineering Brief values now derive from repository-root `TASKS.md` for local projects and tracked-branch root `TASKS.md` for remote projects.
- Preserved accepted Codex-only audit-provider behavior and remote GitHub tracking behavior.

### F-X04-V04-002 — normalized project contract

- Added normalized `currentMilestone`, `requiredActor`, `blockers`, `progressScope`, `authoritySource`, `provenance`, and `reconciliationState` fields to the Rust and TypeScript Command Center project contract.
- Local normalized fields are populated from `ProjectTruth`; remote normalized fields are populated from GitHub root `TASKS.md` tracking.
- Command Center and Tasks presentation now consume normalized current fields instead of `githubTracking` current-state fallbacks. Historical completion metadata remains historical.

### F-X04-V04-003 — degraded fail-closed behavior

- Degraded project summaries no longer call or serialize the control plane.
- Unavailable root TASKS truth is represented as `ROOT_TASKS_UNAVAILABLE` with `NEEDS_RECONCILIATION`, no current task/actor/blocker values, and the bounded next action `Restore a readable repository-root TASKS.md`.
- No `CONTROL_PLANE_*` degraded current-state value is emitted.

### F-X04-V04-004 — adversarial coverage and dead assertions

- Removed the unconditional test early returns and unreachable assertion blocks in the Command Center authority tests.
- Added/retained adversarial coverage for poisoned dashboard evidence, workflow conflicts, mixed historical database evidence, running and blocked root TASKS truth, normalized local fields, poisoned legacy frontend fields, degraded summaries, and historical visibility.
- Regression coverage preserves accepted X04 V02/V03/V04 ROOT_TASKS behavior, Project Cockpit behavior, hidden-source exclusions, provider architecture, the exact eight-project portfolio, `Sekiph82/FormuLab@main`, M00-M17 behavior, X03 behavior, and M16 behavior.

## Verification

- Rust library: 494 passed, 0 failed, serialized.
- Focused Rust Command Center: 29 passed, 0 failed.
- Frontend focused authority/Cockpit/Registry tests: 43 passed, 0 failed.
- Frontend full suite: 18 files, 143 passed, 0 failed.
- TypeScript typecheck: passed.
- Rust `cargo check`: passed.
- Production frontend build: passed.
- `git diff --check`: passed.
- Governed `scripts/publish-dev-qa.ps1`: passed.
- Published executable SHA-256: `CA06024E2AC561ED1A6BCA469DA3F98F6850A1019149382E2BC49FD26D761761`.
- Active-source scans found no prohibited Anthropic API/key/HTTP transport, blanket permission bypass, or M18 activation.
- No external repository was modified.

## Commits

- Implementation/test commit: `d370a06` (`Close X04 V05 command center authority boundary`).
- This log is committed separately after implementation verification.

## Final synchronization evidence

- Final verification is performed after this log commit and push; local `HEAD`, `origin/main`, live GitHub `main`, and the clean worktree must be identical before completion is reported.
