# M18 GitHub Integration V03 Builder Log

Date: 2026-09-15
Repository: `Sekiph82/H-veAI`
Branch: `main`

## Scope and governance

Executed only the M18 V03 prompt. `TASKS.md` and `CODEX_ROADMAP.md` were read-only and were not modified. M19 was not activated, and M18 was not marked closed. The accepted X04 rule remains intact: root `TASKS.md` is the sole current authority for local projects, and tracked-branch root `TASKS.md` is the sole current authority for GitHub-tracked projects.

The standalone checkout was inspected before synchronization. Initial local `main` was `89110209345cdd57a2b61b64c62bc12340716453`; after fetch, `origin/main` was `2c18d00be7df94a1d03d503fa0bb595886f63028`, with the local commit an ancestor, no local commits ahead, five remote commits behind, and a clean worktree. Synchronization used only `git merge --ff-only origin/main`.

## M18.01-M18.09 implementation

- M18.01: project-registry-bound repository identity, default/tracked branch, remote head, bounded branches and commits, and explicit local evidence.
- M18.02: bounded pull-request listing with branch/SHA/diff/review/check/comment evidence and explicit task/session/M links only; no title guessing.
- M18.03: bounded issue listing with state, labels, body, and explicit links only; no title guessing or mutation.
- M18.04: bounded Actions/CI runs with status/conclusion, event, branch/SHA, PR association, failure summary, jobs, steps, and bounded log evidence.
- M18.05: bounded releases and tags with publication metadata.
- M18.06: resource/branch-scoped cache envelopes, age and provenance, stale/offline/rate-limit/auth/timeout/malformed/unavailable classes, bounded payloads, and cache invalidation separation.
- M18.07: observational local/remote reconciliation with `EQUAL`, `LOCAL_AHEAD`, `LOCAL_BEHIND`, `DIVERGED`, detached/no-upstream, missing branch, stale, unavailable, and unknown states; dirty state remains separate.
- M18.08: fixed GitHub REST read transport, project-scoped identity validation, bounded curl process/output, sanitized diagnostics, no auth-file inspection, and denied/unavailable remote mutations.
- M18.09: native GitHub cockpit tab, capability/permission registration, project scoping tests, focused tests, build, and governed native QA publication.

Implementation/test files were limited to:

`src-tauri/src/github_integration.rs`, `src-tauri/src/github_tracking.rs`, `src-tauri/src/lib.rs`, `src-tauri/capabilities/default.json`, `src-tauri/permissions/foundation.toml`, `src/githubIntegration.ts`, `src/pages.tsx`, and `tests/m12-project-cockpit-focused.test.tsx`.

No tracker file was staged or committed.

## Verification

Passed:

- `npm test -- --run tests/m12-project-cockpit-focused.test.tsx` — 10 passed.
- `npm test -- --run --reporter=dot` — 18 files, 144 passed.
- `npm run typecheck` — passed.
- `npm run build` — passed; Vite emitted only the existing chunk-size warning.
- `cargo check --manifest-path src-tauri/Cargo.toml` — passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib github_integration` — 10 passed.
- `git diff --check` — passed.
- `scripts/publish-dev-qa.ps1` — passed; production build, executable smoke checks, process/port checks, stable swap, and shortcut publication completed.

The full Rust command `cargo test --manifest-path src-tauri/Cargo.toml --lib` ran 505 tests. One existing timing-sensitive watcher test failed twice in the full parallel suite at `watcher::tests::live_dashboard_contract_changes_reconcile_watcher_scope_without_restart` (`legacy task` versus `ignored while migrated`); the exact test passed when rerun in isolation. No watcher implementation was changed by M18. All M18-specific Rust tests passed.

Native publication output:

`C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.exe`

SHA-256: `4B72F6B91562D901B6BD5B7C65F1CC63A11C2293AA64FED099958026A6734134`

## Commits and stop condition

Implementation/test commit: `b02f575200d5f37660af932f72706272bf2f0052`

The required log is committed separately after this implementation commit. The agent stops here for independent audit and does not claim M18 closure.

