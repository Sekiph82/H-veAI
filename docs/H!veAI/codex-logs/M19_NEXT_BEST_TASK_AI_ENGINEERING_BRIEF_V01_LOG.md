# M19 Next Best Task AI + Engineering Brief V01 — Immutable Execution Log

- Execution date: 2026-09-16
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative prompt: `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_PROMPT.md`
- Starting SHA after safe sync: `061834239257e800f4e3afa722187f93a3152756`
- Ending implementation SHA (before this log artifact): `93b1dfbf016a8d47b65a109cfa11cd9da1a613c5`
- Final SHA containing this immutable log: recorded by the final handoff and verified against `origin/main`.

## Execution boundary

The local workspace was safely synchronized with GitHub `main` using `fetch` followed by `merge --ff-only`. The starting state was clean and five commits behind `origin/main`; no reset, rebase, force push, stash, or unrelated overwrite was used.

`TASKS.md` and `CODEX_ROADMAP.md` were read as required and remained read-only. They were not modified or staged. M20 was not started. No M19 owner-native acceptance verdict is claimed by Codex.

## M19.00 production/native root causes and fixes

### Exact-eight portfolio and ninth-project persistence

Root cause: the native `github_tracking::ensure_portfolio` path treated the eight GitHub seed projects as the complete portfolio. Each refresh rewrote seed identity fields and archived every non-seed active Registry row, deleting its sync state. This made an explicitly registered ninth project disappear on refresh/restart and made a tenth project impossible to retain.

Fix: the eight-project list is now bootstrap-only. Existing Registry identity and lifecycle state are preserved; non-seed projects are not auto-archived or deleted. Explicit archive/remove operations remain authoritative. The next-best-task engine enumerates active Registry projects rather than the seed list.

Evidence: native Rust coverage verifies the eight-project regression, duplicate-safe ninth/tenth registration, refresh preservation, SQLite close/reinitialize preservation, and explicit archive behavior. The focused tracking suite passed `13/13`.

Before: active portfolio was forced back to the eight seed projects after `ensure_portfolio`.

After: independently registered ninth and tenth projects remain active through refresh and restart; explicit archive remains effective.

### Add Project / persistence

Root cause: the Projects dialog had no pending-submit guard, no immediate Registry refresh/visible success path, and native registration errors could be swallowed by the generic page action flow. Enter and button submission therefore lacked one canonical, observable path.

Fix: Enter and the Add Project button use one form submit path with a pending duplicate guard; UI whitespace is trimmed before native registration; local non-Git and Git paths remain valid; native errors stay visible in a bounded dialog error; successful registration immediately refreshes and inserts the canonical Registry record, selects it, and survives refresh/restart through the native Registry.

Evidence: focused frontend tests cover exactly-once Enter submission, trimmed values, visible success/list update, pending button state, bounded failure, and dialog retention. `tests/m07.06-focused.test.tsx` passed `30/30`.

### Bulk-Edit and Pixel Art Generator GitHub acquisition

Root cause in the real native path: repository metadata acquisition was valid, but GitHub resource fan-out appended primary resources plus up to 40 optional PR/action/issue/branch enrichment results to the snapshot. The frontend contract enforced `MAX_GITHUB_RESOURCES = 8` over the complete resource array. Repositories with enough PRs/issues/branches therefore returned a valid identity but were rejected as `github.repo_failed` by the snapshot validator. This was a cross-layer shape mismatch, not a repository identity or authentication failure.

Live native-path reproduction evidence, using unauthenticated GitHub API requests with no PAT or credential scraping:

| Repository | Default branch | Live identity | PRs | Issues | Branches | Failure risk |
|---|---:|---|---:|---:|---:|---|
| `Sekiph82/H-veAI` | `main` | HTTP 200 | 0 | 0 | 1 | baseline succeeds |
| `Sekiph82/Bulk-Edit` | `main` | HTTP 200 | 10 | 20 | 25 | primary + enrichment exceeded frontend eight-resource contract |
| `Sekiph82/ScrubBots-Level-Factory` | `main` | HTTP 200 | 3 | 3 | 4 | primary + enrichment exceeded frontend eight-resource contract |
| `Sekiph82/FormuLab` (additional control) | `main` | HTTP 200 | 4 | 15 | 1 | same fan-out boundary exercised |

At reproduction time the unauthenticated response reported a remaining rate budget of `54/60`; no rate-limit failure was used to explain the bug. Request evidence was kept redacted and bounded.

Fix: the cached `resources` array now contains exactly the eight primary resource kinds expected by the frontend. Optional enrichment remains independently represented at parent-record health level, so partial enrichment is `PARTIAL` rather than a false repository failure; cached primary data is reported as `STALE` when appropriate. HTTP 429 is explicitly classified as rate-limited, reconciliation fails closed when remote data is unavailable, and resource/request limits and redaction remain enforced.

Evidence: native GitHub integration tests passed `24/24`, including exact repository identities, optional-enrichment partial health, bounded fan-out for Bulk-Edit-sized data, cache behavior, explicit 429 classification, and secret/error redaction. M18 control-plane boundaries remain read-only and were not reopened.

## M19 next-best-task engine

Added the native `next_best_task` engine and Tauri command. It uses active Registry projects, local task intelligence and cached remote GitHub task data, deterministic eligibility/filtering, bounded candidate and attention outputs, actor readiness, completion/dependency/blocker/human-wait handling, explainable score components, and separate factual evidence from recommendation text. Fresh linked failures increase attention only within the bounded freshness window; unavailable or stale inputs are surfaced rather than converted into false positives. Native engine tests passed `3/3`.

## Changed files

- `src-tauri/src/github_integration.rs`
- `src-tauri/src/github_tracking.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/next_best_task.rs`
- `src-tauri/src/projects/paths.rs`
- `src/commandCenter.ts`
- `src/command_center_view.tsx`
- `src/pages.tsx`
- `tests/m07.06-focused.test.tsx`

No tracker file was changed.

## Verification gates

| Gate | Result |
|---|---|
| `npm run typecheck` | PASS |
| `npm run build` | PASS; only existing Vite dynamic-import/chunk-size warnings |
| `npx vitest run tests/m07.06-focused.test.tsx --reporter=dot` | PASS, `30/30` |
| `npm test -- --reporter=dot` | `156/158` passed; two pre-existing 5-second timeout failures in M14 and M17, no M19 failure |
| `cargo check --manifest-path src-tauri\\Cargo.toml` | PASS; existing warnings only |
| `cargo test --manifest-path src-tauri\\Cargo.toml --lib github_integration::tests` | PASS, `24/24` |
| `cargo test --manifest-path src-tauri\\Cargo.toml --lib github_tracking::tests` | PASS, `13/13` |
| `cargo test --manifest-path src-tauri\\Cargo.toml --lib next_best_task::tests` | PASS, `3/3` |
| `rustfmt --check src-tauri\\src\\next_best_task.rs` | PASS |
| full `cargo test --manifest-path src-tauri\\Cargo.toml --lib` | Incomplete: 524 tests ran with no observed test failure before an existing `control_plane::tests::m16l_current_command_center_cockpit_and_control_reads_are_observational` test exceeded 60 seconds; the bounded run was terminated |
| full `cargo fmt --manifest-path src-tauri\\Cargo.toml -- --check` | FAIL due pre-existing formatting drift in unrelated Rust files; no formatter-only unrelated changes were retained |
| `git diff --check` | PASS |
| security/redaction/request-budget coverage | PASS through the `github_integration` focused suite |

## Publication gate

Ran the stable publication helper `scripts/publish-dev-qa.ps1`. It built the release Tauri executable, performed the hidden smoke launch/readiness check, verified no visible console host, swapped the stable binary with rollback handling, and verified the desktop shortcut target/icon.

- Published binary: `dev-bin/H!veAI.exe`
- Published binary SHA-256: `B979F96C20B20D0269C3D54A1E2E92FDB8361A5CB88D3EC291F5BA46CCACA1E4`
- Publication result: PASS

The owner-native UI acceptance gate and independent strict audit are not claimed complete by Codex; they remain required for final M19 acceptance.

## Publication and stop condition

Implementation commit `93b1dfbf016a8d47b65a109cfa11cd9da1a613c5` was pushed non-force to `origin/main` before this log was created. This file is immutable after its log commit. The final handoff reports the log commit SHA and verifies local `HEAD`, `origin/main`, and the remote `main` ref are identical. No further M19 or M20 work is started after that verification.
