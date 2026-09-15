# M18 GitHub Integration V07 Builder Log

## Scope and authority

- Prompt executed: `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V07_NATIVE_GITHUB_BLACK_SCREEN_REMEDIATION_PROMPT.md`
- Authoritative source PASS: `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V06_STRICT_REAUDIT.md`
- Authoritative owner-native partial acceptance: `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V06_OWNER_NATIVE_ACCEPTANCE.md`
- Finding closed: F-M18-V06-NATIVE-001 only.
- `TASKS.md` and `CODEX_ROADMAP.md` were not modified. M19 was not activated. M18 remains open for independent audit.
- Accepted V06, M00-M17, X03/X04, exact eight-project portfolio, `Sekiph82/FormuLab@main`, TASKS-only authority, Prompt Engine Builder + Sessions, Builder Providers/Settings separation, Codex-only audit provider, Claude adapter, and M14/M15/M17 provenance/session behavior were preserved.

## Safe synchronization

- Synchronized starting SHA: `c4841ba299eaef41b51f78986b6209d314762b46`.
- The standalone `main` worktree was clean and local `HEAD` was strictly behind `origin/main` after fetch.
- Ancestry was verified and only `git merge --ff-only origin/main` was used.
- No reset, rebase, force-push, automatic stash, `git clean`, destructive checkout, or owner-work discard was used.

## Root cause and remediation

- The production Rust `GitHubReconciliation` DTO previously serialized field keys as `STATE`, `LOCAL_BRANCH`, `LOCAL_HEAD`, `REMOTE_BRANCH`, `REMOTE_HEAD`, `LOCAL_DIRTY`, and `EVIDENCE` because of `SCREAMING_SNAKE_CASE`.
- The frontend contract consumed `state`, `localBranch`, `localHead`, `remoteBranch`, `remoteHead`, `localDirty`, and `evidence`; the panel passed `evidence` into `CockpitList`, where the missing array could throw during render.
- Rust now serializes the production reconciliation struct with exact camelCase field keys while preserving uppercase domain values such as `DIVERGED`.
- Added a production-struct `serde_json` test asserting all seven exact keys, absence of `STATE`, `LOCAL_BRANCH`, and `EVIDENCE`, and unchanged domain-state/evidence values.
- Added a runtime guard at the GitHub snapshot boundary. It rejects malformed or incomplete repository/reconciliation/local/cache/mutation-policy data, null or over-limit render arrays, malformed nested records, and project-ID mismatches with the bounded diagnostic `GitHub integration evidence is malformed/unavailable`.
- Added a narrow `GitHubIntegrationErrorBoundary` around only the GitHub cockpit surface. Unexpected panel render exceptions become the same panel-local diagnostic; the Project Cockpit shell remains mounted, tab navigation remains available, and no reload or external navigation is performed.
- Existing request cleanup and loading finalization remain active for success, rejection, malformed payloads, tab changes, and project changes.

## Exact changed files

- `src-tauri/src/github_integration.rs`
- `src/githubIntegration.ts`
- `src/pages.tsx`
- `tests/m12-project-cockpit-focused.test.tsx`
- `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V07_LOG.md`

## Focused and regression evidence

- Focused Rust GitHub integration tests: `20 passed; 0 failed`, including the serialization contract, sanitizer/cache, Actions, and project-owned linkage coverage.
- Mounted Project Cockpit/GitHub tests: `15 passed; 0 failed`. Coverage includes valid repository/reconciliation/PR/Issue/Actions rendering, camelCase evidence, missing evidence, malformed reconciliation/arrays, native rejection, contained render exception, stale tab completion, stale project completion, shell navigation, and route preservation.
- Focused M12/M13/M14/M15C/M17 frontend regression: `49 passed; 0 failed`.
- Full frontend regression: `18 files; 156 tests passed; 0 failed` using `npx vitest run --maxWorkers=1`.
- Normal parallel Rust library gate: `516 passed; 0 failed` using `cargo test --manifest-path src-tauri/Cargo.toml --lib`.
- `npm run typecheck`: passed.
- `npm run build`: passed.
- `cargo check --manifest-path src-tauri/Cargo.toml`: passed.
- `git diff --check`: passed.
- Tracker immutability check `git diff --exit-code -- TASKS.md CODEX_ROADMAP.md`: passed.
- Security/negative searches found no added credentials, PAT/API-key settings, direct audit HTTP transport, arbitrary GitHub host, shell command-string construction, mutation fallback, hidden current-truth fallback, reload, external navigation, Agents workspace, or M19 activation.
- Existing React test warnings and Rust compiler warnings are non-failing pre-existing test/build diagnostics; no unrelated source was changed to suppress them.

## Implementation/test commit and hosted observation

- Implementation/test commit: `5039ed53ba5f7b03ac6ee93f1cc0034e0432d33e`.
- GitHub hosted observation for that commit: combined status `pending`, zero statuses, zero check runs, and zero workflow runs. No hosted required gate was available to claim or await.

## Governed publication

- Only `scripts/publish-dev-qa.ps1` was used after all required gates passed.
- Release build, embedded frontend readiness smoke, no-port check, no-visible-console check, stable executable swap, direct shortcut target, stable icon contract, and stable-path smoke all passed.
- Published executable: `dev-bin/H!veAI.exe`.
- Published executable SHA-256: `98238D769C0A88D802D877694DAB71CAAA20213814E36CC6710EE5F1FF8C47F9`.

## Final repository state

- The builder-log commit was pushed to `origin/main` after implementation/test publication.
- Final verification confirms local `HEAD`, `origin/main`, and live GitHub `refs/heads/main` are identical and the worktree is clean; the final SHA is reported in the owner-facing completion response.
- Work stops for independent V07 strict source re-audit. No owner-native acceptance was performed and no M18 PASS/CLOSED claim is made.
