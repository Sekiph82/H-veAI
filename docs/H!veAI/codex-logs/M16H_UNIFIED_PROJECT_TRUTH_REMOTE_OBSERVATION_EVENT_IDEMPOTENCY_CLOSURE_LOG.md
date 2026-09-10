# M16H Unified Project Truth, Remote Observation, and Event Idempotency Closure Log

Date: 2026-09-09
Branch: H!veAI
Role: Codex builder
Authority: M16H_UNIFIED_PROJECT_TRUTH_REMOTE_OBSERVATION_EVENT_IDEMPOTENCY_CLOSURE_PROMPT.md

## Boundary and synchronization

- Read and obeyed `H!veAI/GPT.md` before implementation.
- Safely synchronized the branch with `origin/H!veAI` by `git fetch origin H!veAI` followed by fast-forward-only merge. Starting synchronized HEAD was `1bab9326fb462ae1abad82dc2e4e2f1375991253`.
- No M17 activation, M21 work, installer work, visible UI redesign, canonical opening-video byte change, or unrelated root-file change was performed.
- M16 remains OPEN pending independent whole-M16 strict re-audit and owner native/visual acceptance.

## UCP-R19: shared ProjectTruthResolver

- Added one typed `ProjectTruth` contract in the native control-plane module and made control-plane snapshots, Command Center summaries, and Project Cockpit snapshots consume the same resolver.
- Implemented the required precedence: verified active native workflow task, valid explicit STATE current task, valid explicit HANDOFF current task, exact canonical task lookup, and only deterministic explicit milestone/cycle scope; otherwise the result is `NEEDS_RECONCILIATION` with a null current task.
- Removed Command Center first-open task selection and whole-file completion-ratio progress fallback. Progress now carries `progressScope` and comes only from explicit scoped state or deterministic current milestone grouping.
- Current task title/status, workflow state, actor, next action, blockers, authority source, provenance, and reconciliation state remain evidence-backed and project-scoped. Materialized dashboard prose cannot revive an old task or become a global fallback.

## UCP-R20: fetch-first remote observation

- The 60-second safety scheduler now runs the same remote synchronization path for every eligible ACTIVE local Git project, regardless of auto-fast-forward preference.
- Manual and scheduled synchronization fetch bounded remote metadata first, recompute ahead/behind/diverged state, and refresh the persisted projection. Auto-fast-forward gates only the subsequent `merge --ff-only @{upstream}` step.
- Dirty, ahead, diverged, detached, or otherwise unsafe local worktrees remain fetch-only and are never reset, rebased, stashed, or overwritten. Fetch failure is returned as explicit `SAFE_FETCH_FAILED` attention evidence.
- Added an end-to-end bare-remote fixture proving auto-FF OFF observes a remote-only commit without moving the local head, and auto-FF ON then performs the verified strict fast-forward.

## Eight-project truth matrix

The inherited M16G migration matrix remains the canonical cross-project fixture set. M16H routes each registered project through the same typed resolver while preserving each project-specific task authority:

| Repository | Branch | Verified remote SHA | Canonical task source |
| --- | --- | --- | --- |
| AI-Commerce-HQ | H!veAI | `c46617648daf4e2c2f48679f1da107ae3b1cb4a1` | `H!veAI/TASKS.md` |
| Bulk-Edit | main | `05a059ab5aff211be8a9cd8feccd3d5cba7845fd` | `TASKS.md` |
| FormuLab | feature/laboratory-stability | `db2520d648a7b379af15e2516c6c220f91aabc04` | `docs/FORMULAB_V1_TASK_TRACKER.md` |
| PackLab | main | `46cdf07c3c5138594301214ffb351792c378e125` | `TASKS.md` |
| PackLab-3D | main | `af5d83f089d753b82101369c303880d45b2ffc9e` | `tasks.md` |
| ScrubBots | main | `f44f1d50c6ea8f3427a4a62410887cc2b554945b` | `tasks.md` |
| ScrubBots-Level-Factory | main | `7ccbc1ad3209d0e2d3f4e03d4c19127a69b33ce6` | `tasks.md` |
| fmcg-erp-system | main | `77aa33b2d18811e019d17609a4298929946e603a` | `TASKS.md` |

All eight remain governed by their own canonical ledgers and remote identities; unresolved current-task truth is explicit rather than synthesized from another project or a global fallback. External dirty worktrees remain read-only evidence.

## UCP-R21: crash-consistent event idempotency

- Event append now loads the durable index and reconciles missing IDs from a bounded 2 MiB recent `EVENTS.jsonl` tail before deciding whether an event is a duplicate.
- The append order remains durable `EVENTS.jsonl` first, index second. If index persistence fails after the event append, retry sees the durable tail and rejects the duplicate; duplicate retries also repair the sidecar index.
- The bounded horizon is explicit: the newest 4096 event IDs, reconciled from the bounded tail. A direct >4096-event test verifies deterministic pruning and documents the intentional horizon.
- A failpoint test proves the required failure sequence: valid index, durable append, failed index persistence, retry rejection from the tail, sidecar repair, and later retry rejection.

## Whole-M16 adversarial sweep

- Rechecked prior M16 audit, re-audit, freshness, degraded-persistence, evidence identity, project-control-plane, watcher, and publication boundaries.
- Rechecked that unresolved truth does not select the first open task, use a historical task, use milestone prose as task identity, or report portfolio-wide progress.
- Rechecked that remote fetch and auto-FF are independent, that unsafe local histories remain unmodified, and that the Windows repository-root identity check is canonicalized without broadening authority.
- Rechecked bounded event tail parsing, malformed-line tolerance, sidecar repair, duplicate rejection after a simulated crash, and documented >4096 behavior.
- Existing Command Center and watcher tests that asserted the removed first-open/materialized-only fallbacks were updated to assert the M16H fail-closed contract. No production parser logic was changed.

## Verification and publication gates

- Focused event-idempotency tests: passed, including crash-window and >4096 horizon cases.
- Focused remote-observation fixture: passed.
- Full Rust regression without PTY: `394 passed, 0 failed`.
- Full Rust regression with `pty-support`: `395 passed, 0 failed`.
- Frontend Vitest: `15 files, 125 passed, 0 failed`.
- TypeScript typecheck: passed.
- Production frontend build: passed.
- `npm audit --audit-level=high`: passed with two pre-existing moderate Vitest advisories; no high or critical vulnerability was reported and no breaking dependency upgrade was introduced.
- `cargo fmt --all`: passed.
- `git diff --check`: passed.
- Publisher rollback and failure harness: `9 of 9` passed.
- Governed production publication: passed. Stable executable SHA-256 is `3F0809933CDCBF93E276F7C7CC67A00478F9050ECDA55373819E6F42D4B629DF`; size is `22882304` bytes; PE signature is `MZ`.
- Stable artifact: `H!veAI/dev-bin/H!veAI.exe`; companion icon: `H!veAI/dev-bin/H!veAI.ico`.
- Canonical opening-video SHA-256 remains `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`.
- No development-port listener was introduced by the published executable; the only source `devUrl` reference remains the existing Tauri development configuration.

The complete M16H gate set was executed through direct resolver, remote-observation, event-failure, source, adversarial, regression, security, and governed-publication checks. This builder log records implementation claims and test evidence only; it does not self-close M16.

M16H WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.
M16 remains OPEN.
M17 NOT ACTIVATED.
M21 NOT STARTED.
