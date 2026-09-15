# M18 GitHub Integration V05 Builder Log

## Scope and synchronization

- Executed only `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V05_STRICT_REMEDIATION_PROMPT.md` against the authoritative V04 failed re-audit.
- Scope was limited to F-M18-V04-001 through F-M18-V04-005. M19 was not activated.
- Standalone checkout: `C:\Users\sekip\Desktop\H!veAI`, branch `main`.
- Initial inspected local/origin SHA: `3f05257e5c1d1b495120ae741834ac0623bde630`; worktree was clean and local was 0/0 divergent.
- After `git fetch origin main`, origin advanced to `868e9215351c842af3d447c41cda95284de44713`; local was a strict ancestor, three commits behind, and clean. Applied only `git merge --ff-only origin/main`.
- No reset, rebase, force-push, automatic stash, `git clean`, destructive checkout, or owner-work discard was used.

## Authority preservation

`TASKS.md` and `CODEX_ROADMAP.md` were not modified. `git diff --exit-code origin/main -- TASKS.md CODEX_ROADMAP.md` passed before implementation and again before commit. The X04 rule remains intact: root `TASKS.md` is the sole current authority for local projects and tracked-branch root `TASKS.md` is the sole current authority for GitHub-tracked projects; hidden `.hiveai` truth was not revived.

## Implementation/test commit

Implementation and test commit: `6291d7a815cc53e62018920ae2dc2cc7aac4e842`.

Files changed:

- `src-tauri/src/github_integration.rs`
- `src/App.tsx`
- `src/githubIntegration.ts`
- `src/pages.tsx`
- `tests/m14-agent-session-center-focused.test.tsx`
- `tests/m15c-post-dispatch-handoff-focused.test.tsx`
- `tests/m17-claude-adapter-focused.test.tsx`

## Finding disposition

- **F-M18-V04-001 — closed for remediation:** cache schema advanced from 1 to 2; legacy and malformed rows are not trusted; a shared bounded sanitizer covers whitespace, punctuation-adjacent, URL/query, JSON-quoted, no-space Authorization/Bearer, generic credential assignments, and GitHub token families before persistence and DTO projection. Direct tests prove redacted output and cache safety.
- **F-M18-V04-002 — closed for remediation:** Actions jobs are filtered by job-level failure truth (`FAILURE`, `CANCELLED`, `TIMED_OUT`, `ACTION_REQUIRED`) in deterministic fetched-job order; only the first two eligible jobs can contribute logs. Evidence preserves job ID, job name, and bounded excerpt, with partial/unavailable states retained. Success/skipped jobs are excluded by direct tests.
- **F-M18-V04-003 — closed for remediation:** raw explicit task/session references are stored separately from validated links. TASK links require current canonical tracked-branch `TASKS.md` materialization; SESSION links require persisted `agent_sessions.project_id` ownership. Foreign and hidden-control-plane references remain raw evidence only.
- **F-M18-V04-004 — closed for remediation:** every `/agents` entry form resolves into the integrated Prompt Engine Sessions surface at `/prompts?surface=sessions`; valid bounded parameters are preserved and duplicate, partial, empty, or malformed targets fail closed without dispatch. Standalone Agents is no longer a top-level route; internal embedding remains allowed.
- **F-M18-V04-005 — closed for remediation:** every enrichment acquisition reserves request budget before cache/load/transport work. The cap is 40 and a direct instrumented test proves the N+1 transport call is not made after exhaustion.

All accepted V04 improvements, M00-M17/X03/X04 behavior, M14/M15 session/provenance behavior, Codex-only audit provider, Claude adapter, exact eight-project portfolio, `Sekiph82/FormuLab@main`, and TASKS-only authority were preserved.

## Verification

- Focused Rust GitHub integration: `cargo test --lib github_integration -- --nocapture` — **18 passed, 0 failed**.
- Focused frontend M13-M17/M14/M15 routing and provider tests — **4 files, 29 passed, 0 failed**.
- Full frontend: `npm test -- --maxWorkers=1` — **18 files, 146 passed, 0 failed**.
- `npm run typecheck` — **passed**.
- `npm run build` — **passed**; existing Vite chunk/dynamic-import warnings only.
- Full Rust library, all non-ignored tests serialized to avoid the existing shared failpoint-test race: `cargo test --lib -- --test-threads=1` — **514 passed, 0 failed**.
- `cargo check --manifest-path src-tauri/Cargo.toml` — **passed**; existing warnings only.
- `git diff --check` — **passed**; line-ending conversion warnings only.
- The initial parallel `cargo test --lib` invocation exposed three pre-existing shared `task_intelligence` failpoint races; each passed individually and the complete serialized suite passed 514/514. No unrelated test or production behavior was changed to mask that race.

## GitHub status and security gates

- Final implementation commit GitHub API observation: combined status `pending`, total statuses `0`; check-runs `0`; no workflow runs were returned for `main` by the queried Actions endpoint. No hosted green check was claimed.
- Final diff negative searches found no added PAT/API-key settings, credential-file/auth-store reads, shell mutation command construction, automatic Git mutation, or `.hiveai` fallback. Matches for `api.github.com`, `/agents`, and `M19` are fixed production URL/route coverage and deliberately synthetic adversarial test data only.

## Governed publication

Ran only `scripts/publish-dev-qa.ps1`. The native Tauri production executable built, passed readiness/no-console/no-development-port smoke checks, and was installed at stable `dev-bin/H!veAI.exe` with the governed Desktop shortcut and icon contract.

Published executable SHA-256: `41CB0FD00653B576DD1B7DD1B6018EFAE17A9F0A128603CC2AB27CF63023E281`.

## Push and stop condition

The implementation/test commit was pushed normally to `origin/main`. The builder-log commit is this commit. Final verification was performed with `git fetch origin main`, `git rev-parse HEAD`, `git rev-parse origin/main`, `git ls-remote origin refs/heads/main`, and `git status --short`; local HEAD, `origin/main`, and live GitHub `main` were equal, and the worktree was clean at verification time.

This log stops for independent V05 source re-audit. It does not mark M18 PASS/CLOSED and does not activate M19.
