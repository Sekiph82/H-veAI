# M18 GitHub Integration V06 Builder Log

## Scope and authority

- Prompt executed: `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V06_STRICT_REMEDIATION_PROMPT.md`
- Authoritative failed re-audit: `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V05_STRICT_REAUDIT.md`
- Findings closed: F-M18-V05-001, F-M18-V05-002, F-M18-V05-003 only.
- The root `TASKS.md` and `CODEX_ROADMAP.md` were not modified. M19 was not activated.
- Accepted M00-M17, X03/X04, M14/M15 provenance/session behavior, exact eight-project portfolio, `Sekiph82/FormuLab@main`, TASKS-only authority, Codex-only audit provider, Claude adapter, and Prompt Engine Sessions integration were preserved.

## Safe synchronization

- Initial standalone checkout: clean `main`, local/origin `aa081fa9775d5000928f25488df96740aeb97935`.
- After fetch, origin/main was `46eba0b492c14b2a8a925cd0e0ea7525efedf7be`; local was clean and strictly behind by two commits.
- Ancestor check passed and only `git merge --ff-only origin/main` was used.
- Remediation started from `46eba0b492c14b2a8a925cd0e0ea7525efedf7be`.
- No reset, rebase, force-push, automatic stash, `git clean`, destructive checkout, or owner-work discard was used.

## Remediation evidence

### F-M18-V05-001

- The production sanitizer now recognizes quoted Authorization key boundaries, URL query credentials, no-space and spaced Bearer/Basic forms, quoted JSON Authorization, quoted token/api-key fields, and all GitHub token families.
- Direct end-to-end fixture coverage runs the real `snapshot_with_transport` path, then verifies both the projected issue DTO and persisted cache metadata contain none of the synthetic secret values.
- Cache schema remains version 2 and schema-1/malformed cache rows remain rejected.
- The cache reload assertion also verifies the sanitized envelope is idempotent and reusable.

### F-M18-V05-002

- Added production-path `snapshot_with_transport` Actions matrix coverage for success/skipped exclusion, cancelled/timed-out/action-required eligibility, unavailable jobs, no eligible jobs, partial log failure, exact two-log acquisition, excerpt truncation, and no third-log request.
- Existing request-budget coverage remains green and proves no N+1 enrichment.
- Added cross-project ownership coverage for selected `Sekiph82/H-veAI@main` versus foreign `Sekiph82/FormuLab@main` task/session references; only current project-owned canonical references are promoted.
- M15C route matrix coverage now includes bare, exact, project-only, session-only, duplicate project/session, malformed, overlong, wrong-project, missing, no-dispatch, and readiness-polling behavior, plus Settings/providers and M14/M15 surfaces.

### F-M18-V05-003

- Exact initial normal parallel reproduction from `src-tauri`: `510 passed; 4 failed`.
- Reproduced failures:
  - `task_intelligence::tests::p01_single_stable_edit_is_parsed_after_one_refresh`: `ParserWarning { code: "SOURCE_READ_FAILED", message: "refreshed source is outside registered root", source_path: Some("TASKS.md") }`.
  - `task_intelligence::tests::p01_retry_rechecks_physical_containment`: `called Result::unwrap_err() on an Ok value`.
  - `task_intelligence::tests::p01_second_change_after_refresh_is_skipped_after_exactly_one_retry`: `called Result::unwrap() on an Err value: PoisonError { .. }`.
  - `watcher::tests::migrated_project_attaches_single_dashboard_scope_and_refreshes_only_at_dashboard_signal`: expected `changed before dashboard signal`, observed `first task`.
- The two task-intelligence failpoints are now thread-local and test-only, eliminating process-global cross-test interference without serializing production behavior.
- The watcher test uses bounded polling for the expected asynchronous refresh, preserving the production polling-based stabilization behavior.
- One subsequent isolated parallel-load run exposed the unrelated process-fixture assertion `codex_adapter::tests::owned_stop_escalates_after_bounded_unsupported_graceful_attempt`; it passed in isolation and on the next full run. No production stop semantics were changed.

## Verification gates

- Focused Rust GitHub integration suite: 19 passed, 0 failed.
- Normal parallel `cargo test --lib`: 515 passed, 0 failed; two consecutive green runs completed after the final Rust changes.
- Full frontend regression: `npx vitest run --maxWorkers=1`, 18 files / 151 tests passed, 0 failed.
- Focused M13/M14/M15C/M17 regression: 4 files / 34 tests passed, 0 failed.
- `npm run typecheck`: passed.
- `npm run build`: passed.
- `cargo check`: passed.
- `git diff --check`: passed.
- Tracker immutability check: `git diff --exit-code -- TASKS.md CODEX_ROADMAP.md` passed.
- Security search found only intentional test fixtures and historical audit examples; no live credential was added.
- Repository-wide `cargo fmt --all -- --check` reports pre-existing formatting differences in unrelated owner files; no unrelated formatting was rewritten.

## Commits, publication, and hosted observation

- Implementation/test commit: `94bbc45705511ae503befe8ea1f5c2905ee6bf1f`.
- Hosted observation for that commit: GitHub combined status `pending` with zero statuses, zero check runs, and zero workflow runs; no required hosted check was available to wait on.
- Governed publication: `scripts/publish-dev-qa.ps1` passed release build, PE validation, readiness smoke, no ports 5173/8765, no visible console, stable swap, and shortcut/icon verification.
- Published executable SHA-256: `DC0E01D746C89742A5570C999D46528A343D5279EAC226F9766906C116759ACD`.
- This log is the final builder-log change for V06. Work stops here for independent strict re-audit; no M18 PASS/CLOSED claim is made by this log.
