# M19 Next Best Task AI + Engineering Brief V04 Builder Log

Status: implementation and builder verification complete; independent V04 strict re-audit pending. This log does not mark M19 PASS or CLOSED.

## Authority and synchronization

- Authoritative prompt: `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V04_REMEDIATION_PROMPT.md`.
- Authoritative audit read in full: `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_STRICT_REAUDIT.md`.
- Prior V03 and V03 R02 prompt/log material was read in full before implementation.
- Safe synchronization: `git fetch origin main` followed by `git merge --ff-only origin/main`.
- Synchronized start `origin/main`: `9bd8f4aaa88c8d66b38b761beecb825bae4f29f4`.
- Final implementation commit: `89ad88c43d586dd233b72469e8c0d6856dee17c5`.
- The final log commit SHA is supplied in the publication handoff. It is intentionally not embedded as a self-referential value in this immutable file.

No reset, rebase, force-push, auto-stash, clean, or unrelated-worktree discard was used.

## V04 remediation disposition

| Finding | Production-path remediation and direct evidence | Disposition |
|---|---|---|
| F-M19-V03R02-STRICT-001 | `next_best_task::build_candidate_set` now constructs one canonical normalized-ID graph, resolves/deduplicates edges, and derives unfinished, missing, ambiguous, and case-equivalent dependency state from factual task state. Local exact-root and remote `TASKS.md` candidates use the same graph. Unlock credit is granted only when the dependent has no unmet dependency or independent gate. Native graph tests and the full 537-test Rust suite passed. | Implemented; audit pending |
| F-M19-V03R02-STRICT-002 | `recent_failure_evidence` now returns a structured `Result` path with source, stable evidence ID, result, timestamp, age/freshness, and explicit uncertainty. Remote/database errors fail closed as unavailable evidence. Latest PASS supersedes an older FAIL; stale FAIL has no urgency bonus. The evidence ID is included in recommendation summaries. Targeted latest-pass test and full Rust suite passed. | Implemented; audit pending |
| F-M19-V03R02-STRICT-003 | `recordNextBestTaskHistory` is called by the mounted Command Center manual GitHub refresh lifecycle only after the native refresh succeeds; `snapshot()` remains pure. The mounted M11 lifecycle test verifies refresh-before-record ordering. | Implemented; audit pending |
| F-M19-V03R02-STRICT-004 | `github_integration` now has a process-wide one-hour governor, bounded 48-request allowance, keyed in-flight coalescing, bounded shared-cache reuse, primary-before-optional admission, and fail-closed stale/unavailable behavior on 403/429. Recovery clears obsolete warnings and returns current data. FixtureTransport/native tests cover coalescing, cache reuse, circuit/recovery, redaction, and budget. The current curl seam exposes no response-validator headers; ETag/Last-Modified is recorded as unavailable rather than fabricated. | Implemented; audit pending |
| F-M19-V03R02-STRICT-005 | `validate_repository_identity` is shared by registration and path repair. ACTIVE, ARCHIVED, and MISSING same-path recovery require compatible Git/remote/HEAD/ancestry identity; replacement, ambiguity, Git/non-Git contradiction, and unrelated no-remote history fail closed. Native archived, missing, incompatible-remote, and identity matrix tests passed. | Implemented; audit pending |
| F-M19-V03R02-STRICT-006 | Native registration returns `CREATED`, `ALREADY_ACTIVE`, `RESTORED_ARCHIVED`, or `RESTORED_MISSING` through `RegisterProjectResult`; Add Project copy uses this authoritative disposition, not the visible active-record list. Same-ID persistence is covered by native recovery tests. | Implemented; audit pending |
| F-M19-V03R02-STRICT-007 | `semanticAttentionKey` uses project/task plus structured evidence/blocker/wait/failure identity, so wording/category changes dedupe only the same factual issue while distinct evidence remains visible. Mounted semantic dedupe tests passed. | Implemented; audit pending |
| F-M19-V03R02-STRICT-008 | Project cards now expose deterministic card/footer/action test IDs, visible `Local workspace` copy, contained delete action semantics, and the mounted geometry test checks 1536px, 900px, and 640px layouts, containment, action presence, and the forbidden `Change local workspace` copy. | Implemented; audit pending |
| F-M19-V03R02-STRICT-009 | The production-path matrix is represented by the native full suite plus mounted M11/M19/M07 coverage: graph/freshness, actors/scoring, history lifecycle, registry recovery/disposition, geometry, GitHub circuit/coalescing/budget, and M18 error-boundary behavior. | Implemented; audit pending |
| F-M19-V03R02-STRICT-010 | This checked-in log includes the redacted four-repository/eight-resource acquisition matrix below, deterministic request-budget math, live public HTTP evidence, and explicit transport-header limitations. | Implemented; audit pending |
| F-M19-V03R02-STRICT-011 | The accepted publication helper produced and smoke-tested the release executable, including stable swap and shortcut verification. Native smoke evidence and builder-level behavior limits are recorded below. | Implemented; owner/independent QA pending |

## Changed files

- `src-tauri/src/github_integration.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/next_best_task.rs`
- `src-tauri/src/projects/mod.rs`
- `src-tauri/src/projects/registry.rs`
- `src/commandCenter.ts`
- `src/command_center_view.tsx`
- `src/components/ProjectRegistryCard.tsx`
- `src/pages.tsx`
- `src/projectRegistry.ts`
- `tests/m07.06-focused.test.tsx`
- `tests/m11-command-center-focused.test.tsx`
- `tests/m19-v04-project-card-geometry.test.tsx`
- `vitest.config.ts` (default frontend verification is single-worker because existing jsdom adapter suites race under parallel file execution)

## Direct evidence highlights

### Canonical dependency and failure graph

The candidate builder now normalizes explicit IDs, creates the canonical ID-to-task map, rejects missing and duplicate matches, deduplicates dependency edges, and derives unfinished prerequisite attention from `factual_state`. No fixture injects a synthetic `dependency unfinished` blocker to make the result ineligible. Local exact-root and remote rows enter the same candidate graph.

Failure evidence is fail-closed: query errors, missing timestamps, malformed timestamps, and uncertain remote state become explicit uncertainty/unavailable inputs. Absence is scored as zero only when the query proves there is no linked evidence. Stable evidence IDs are preserved in the structured object and summary. A newer PASS/closure controls urgency over an older FAIL, and stale FAIL evidence earns no urgency bonus.

### History lifecycle

The pure snapshot path remains observational. A mounted Command Center manual refresh calls native GitHub refresh, then records the already-computed M19 history fingerprint, then refreshes the displayed snapshot. The test asserts `hiveai_next_best_task_record_history` occurs after `hiveai_github_tracking_refresh`; direct reads alone do not call the mutation.

### Registration and identity

Registration now returns an explicit disposition from native code. Same normalized path is not sufficient: remote identity, Git capability, and no-remote HEAD/ancestry evidence are checked before ACTIVE/ARCHIVED/MISSING restoration. Native tests cover archived same-ID restoration, MISSING same-ID restoration after the metadata refresh records MISSING, incompatible remote rejection, same remote/head, advanced ancestor, unrelated no-remote rejection, and credential-bearing remote sanitization.

### Card geometry and semantics

`tests/m19-v04-project-card-geometry.test.tsx` mounts three cards at representative 1536px, 900px, and 640px widths with deterministic bounding boxes. It asserts each footer and delete icon are inside the card, actions exist, `Local workspace` is visible, and no `Change local workspace` copy is rendered. The mounted M11 tests assert same factual attention emitted with different category/title wording appears once, while distinct evidence remains separate.

## GitHub governor and request-budget evidence

Production integration uses a process-wide 48-request/hour governor. Request keys include repository, branch, and resource kind. One owner performs an in-flight fetch; concurrent same-key readers coalesce against the shared cache with bounded polling. Optional enrichment is admitted only after primary data and is shed when the budget is exhausted. 403/429 opens one process-wide circuit, halts fan-out, retains last-known-good data as STALE, and allows reset/backoff recovery to CURRENT.

The required deterministic primary-resource budget is eight resource requests per project per full portfolio refresh, with the 48-request governor cap. The following is the worst-case proof used by the native test:

| Portfolio | Idle hourly refreshes | Selected-project refresh | Manual full refresh | Governed primary requests | Quota result |
|---:|---:|---:|---:|---:|---|
| 8 projects | 8 × 8 = 64 theoretical; coalesced/cache reuse applies | 8 | min(64, 48) = 48 | 48 | below unauthenticated 60/hour |
| 9 projects | 9 × 8 = 72 theoretical; coalesced/cache reuse applies | 8 | min(72, 48) = 48 | 48 | below unauthenticated 60/hour |
| 10 projects | 10 × 8 = 80 theoretical; coalesced/cache reuse applies | 8 | min(80, 48) = 48 | 48 | below unauthenticated 60/hour |
| 20 projects | 20 × 8 = 160 theoretical; coalesced/cache reuse applies | 8 | min(160, 48) = 48 | 48 | below unauthenticated 60/hour |

The optional enrichment cap is independently bounded at 40 requests in its native test and is lower priority than primary repository state. Selected-project refresh does not fan out to the whole portfolio. The native `process_budget_is_below_unauthenticated_quota_for_required_portfolios` test covers 8/9/10/20 deterministically; coalescing, cache reuse, rate-limit stop/recovery, stale retention, generic 403 distinction, malformed/offline/timeout distinction, and redaction tests are included in the full Rust suite.

Conditional validator headers are `UNAVAILABLE FROM CURRENT TRANSPORT`: the current curl response seam does not expose ETag/Last-Modified headers. No secret or plaintext credential was added.

### Public GitHub acquisition matrix

The following is a redacted builder-side public API probe for `@main` repositories. It sent no Authorization header or credential. Every listed resource returned HTTP 200 during the 32-resource probe; each row consumed one builder HTTP request. The governor budget column is `N/A (shell probe; production governor is exercised by native FixtureTransport tests)` so shell traffic is not misrepresented as app-governed traffic. The subsequent rate header probe observed `27` remaining out of `60`; reset was returned by GitHub and not copied as a secret.

| Repository | Branch | Resource/fetch stage | HTTP | Failure/cache classification | Builder req | Coalesced | Final health |
|---|---|---|---:|---|---:|---|---|
| Sekiph82/H-veAI | main | repository | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/H-veAI | main | branches | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/H-veAI | main | commits | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/H-veAI | main | pull requests | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/H-veAI | main | issues | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/H-veAI | main | actions | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/H-veAI | main | releases | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/H-veAI | main | tags | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/Bulk-Edit | main | repository | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/Bulk-Edit | main | branches | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/Bulk-Edit | main | commits | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/Bulk-Edit | main | pull requests | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/Bulk-Edit | main | issues | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/Bulk-Edit | main | actions | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/Bulk-Edit | main | releases | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/Bulk-Edit | main | tags | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/ScrubBots-Level-Factory | main | repository | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/ScrubBots-Level-Factory | main | branches | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/ScrubBots-Level-Factory | main | commits | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/ScrubBots-Level-Factory | main | pull requests | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/ScrubBots-Level-Factory | main | issues | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/ScrubBots-Level-Factory | main | actions | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/ScrubBots-Level-Factory | main | releases | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/ScrubBots-Level-Factory | main | tags | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/FormuLab | main | repository | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/FormuLab | main | branches | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/FormuLab | main | commits | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/FormuLab | main | pull requests | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/FormuLab | main | issues | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/FormuLab | main | actions | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/FormuLab | main | releases | 200 | current / miss | 1 | false | CURRENT |
| Sekiph82/FormuLab | main | tags | 200 | current / miss | 1 | false | CURRENT |

Repository HEAD identity probes also succeeded:

- H-veAI: `9bd8f4aaa88c8d66b38b761beecb825bae4f29f4`
- Bulk-Edit: `7363bcc55d9c64359b66a02356e8f5a972094215`
- ScrubBots-Level-Factory: `1588ff8a2db20355e0b88f01092b58a3d88c00c2`
- FormuLab: `8e838933555f89330153641120d4f7255e1f05d2`

The live probe is acquisition evidence, not independent acceptance. 403/429 transitions are proved by deterministic native transport tests, where one grouped warning and stale last-good state are retained until recovery.

## Regression and publication gates

| Gate | Result |
|---|---|
| `npm run typecheck` | PASS |
| `npm run build` | PASS; existing dynamic-import and chunk-size warnings only |
| Focused mounted M07/M11/M19 suites, single worker | PASS, 42 tests |
| `npm test` normal/default invocation | PASS, 19 files / 164 tests; default configured single-worker to remove pre-existing jsdom cross-file races |
| `cargo check --manifest-path src-tauri/Cargo.toml` | PASS |
| Focused native graph, registry recovery, latest-PASS, governor tests | PASS |
| `cargo test --manifest-path src-tauri/Cargo.toml --lib --no-fail-fast` | PASS, 537 passed / 0 failed / 0 ignored |
| Exact M16 observational-read test | PASS in the full native suite; M16 observational-read coverage remained green |
| Changed Rust formatting with `rustfmt --check --config skip_children=true` where applicable | PASS for changed standalone files; `lib.rs` checked with children skipped |
| Full `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check` | FAIL only on pre-existing unrelated baseline formatting drift in files outside the changed M19 set; no tracker or unrelated source was reformatted or committed |
| `git diff --check` | PASS; Git reported only normal LF-to-CRLF working-copy warnings |
| `scripts/publish-dev-qa.ps1` | PASS; no-bundle release build, staged/native smoke, stable swap, shortcut checks |

The normal/default frontend run initially exposed existing parallel jsdom timing races in M13/M14/M17; the test runner was made single-worker and the required default command then passed 164/164. This is recorded rather than hidden.

## Published native evidence

- Publication command: `powershell -NoProfile -ExecutionPolicy Bypass -File scripts/publish-dev-qa.ps1`.
- Published stable executable: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.exe`.
- Stable EXE SHA-256: `1032A5C4C87592F1055C686C92B689F5F948A99E67DD820956AB64FABACFCC45`.
- Staged and stable native smoke passed with H!veAI ready marker/window title.
- Forbidden development ports `5173` and `8765` were absent.
- No visible console/conhost was detected.
- Shortcut target: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.exe`.
- Shortcut icon: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.ico,0`.

This is builder-native smoke evidence, not owner-native acceptance. No owner acceptance is self-claimed. The independent strict re-audit remains the next governance step.

## Scope boundary and final verification

- `TASKS.md`: unchanged and read-only.
- `CODEX_ROADMAP.md`: unchanged and read-only.
- M20: not started.
- Prior V03 R02 immutable log: not modified.
- Publication is non-force to `origin/main`; final local/GitHub equality is verified after the log commit and push in the handoff.

