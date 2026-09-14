# Pre-M18 Native Audit Provider Route-Truth Hotfix X03

## Scope

- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Work item: Pre-M18 Native Audit Provider Route-Truth Hotfix X03
- Authoritative audit: `docs/H!veAI/audits/PRE_M18_AUDIT_PROVIDER_ROUTE_TRUTH_X03_AUDIT.md`
- Prompt executed: `docs/H!veAI/prompts/PRE_M18_AUDIT_PROVIDER_ROUTE_TRUTH_X03_REMEDIATION_PROMPT.md`
- Findings closed: F-X03-001 and F-X03-002 only
- M17 remains PASS/CLOSED; M18 was not activated

## Synchronization evidence

The standalone checkout was inspected before repository-file work. The initial checkout was clean and strictly behind `origin/main` by eight commits with no local-only commits. The only synchronization was:

```text
git merge --ff-only origin/main
```

Starting synchronized SHA: `a2c6849c7793886cbe5a1b8059c6439ad4308241`

No reset, rebase, force-push, automatic stash, `git clean`, destructive checkout, or owner-work discard was used.

## Implementation and tests

Implementation/test commits:

- `e9171d8c98f48226caf131c6e339785f802e7fb1` — process-scoped X03 readiness state and historical Audit Center truth
- `a3cb74c567ab16e066d85e7471bb8f4a060dee3a` — deterministic baseline, explicit READY, and live/history fixture coverage

Exact changed implementation/test files:

- `src-tauri/src/audit_engine.rs`
- `src-tauri/src/lib.rs`
- `src/AuditCenterPage.tsx`
- `tests/m16-audit-center-focused.test.tsx`
- `tests/m16s-provider-and-workspace-focused.test.tsx`

The required builder log is the only additional file created by this work.

### F-X03-001

Added a native process-scoped, non-sensitive readiness state managed by Tauri. It retains only the sanitized `AuditProviderReadiness` result and the selected Codex executable path/version identity in memory. The readiness getter reuses the last explicit check result across Settings route remounts, including explicit READY and explicit failure categories. A new process starts with no cached verification. A changed executable path or version does not reuse the cached result. The production-equivalent structured-output probe remains the explicit check authority, and audit execution accepts the governed READY result without changing the Codex-only provider contract.

No credentials, tokens, auth files, raw streams, API keys, direct OpenAI HTTP/API transport, or GUI automation were added.

### F-X03-002

The Audit Center selected-run panel is now titled `Selected persisted verdict`. Historical non-AVAILABLE diagnostics identify the state of that immutable run. Historical `UNAVAILABLE` text states that the selected audit run recorded provider unavailability at that time; it does not claim that the provider is currently unconfigured. The stored audit timestamp remains visible. Historical rows are not rewritten. The UI fixture includes current live READY data alongside historical UNAVAILABLE data and verifies the historical wording.

## Deterministic verification

Focused Rust readiness tests passed, including:

- `readiness_getter_uses_truthful_baseline_without_explicit_cache`
- `readiness_getter_returns_explicit_ready_result_after_check`
- `readiness_cache_preserves_explicit_ready_result_for_same_identity`
- `readiness_cache_preserves_explicit_nonready_result_for_same_identity`
- `readiness_cache_is_process_scoped_and_resets_with_new_state`
- `readiness_cache_invalidates_on_executable_or_version_identity_change`
- existing readiness classification, schema, and Codex-only regression tests

Focused frontend tests passed: 11 tests in `m16s-provider-and-workspace-focused.test.tsx` and `m16-audit-center-focused.test.tsx`, including Settings unmount/remount retention and historical UNAVAILABLE wording.

## Regression and security gates

- Full serialized Rust library suite: **481 passed, 0 failed** (`--test-threads=1`)
- Full frontend Vitest suite: **18 files, 141 passed, 0 failed**
- `npm run typecheck`: passed
- `cargo check --manifest-path src-tauri/Cargo.toml`: passed
- `npm run build`: passed
- `git diff --check`: passed
- Active-source scan: no `OPENAI_API_KEY`, `api.openai.com`, `reqwest`, `curl`, `Invoke-RestMethod`, or `WebRequest` in the Codex audit provider/runtime sources
- Auth-file boundary scan: no auth-file inspection added; the existing negative test and bounded diagnostic classifier remain only as defensive checks
- Exact eight-project portfolio and `Sekiph82/FormuLab@main` regressions remained green in the full Rust suite
- Existing M16T freeform/task schema, failure classification, representative readiness, immutable audit-history, Codex-only provider, and accepted M00-M16 behavior remained green

## Native QA publication

The existing governed production `--no-bundle` publication path passed twice, including the final source state. Final checks confirmed the expected `H!veAI` window title, frontend-ready marker, no forbidden development ports, no visible console host, and matching desktop shortcut target/icon.

Final published executable: `dev-bin/H!veAI.exe`

Final executable SHA-256: `95BF9FCF34F8CEF358A248E783EADA1D387EDF72FE3771C1B0B3F309C6F6724B`

## Governance and completion evidence

`TASKS.md` and `CODEX_ROADMAP.md` were read-only and were not modified by Codex. No tracker transition was performed. M18 was not activated or implemented.

The final local/origin/live-main equality and clean-worktree check was performed after the log commit and push using `git fetch origin main`, `git rev-parse HEAD`, `git rev-parse origin/main`, `git ls-remote origin refs/heads/main`, and `git status --short`.
