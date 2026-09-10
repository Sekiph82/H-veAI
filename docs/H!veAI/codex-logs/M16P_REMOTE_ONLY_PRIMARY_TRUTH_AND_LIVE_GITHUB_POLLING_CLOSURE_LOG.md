# M16P Remote-Only Primary Truth and Live GitHub Polling Closure Log

## Scope

- Authoritative prompt: `M16P_REMOTE_ONLY_PRIMARY_TRUTH_AND_LIVE_GITHUB_POLLING_CLOSURE_PROMPT.md`.
- Mandatory addendum: `M16P_ADDENDUM_STARTUP_RESPONSIVENESS_AND_HIDDEN_GIT_PROCESS_HOTFIX_PROMPT.md`.
- Starting synchronized HEAD: `046df7acb9e966dde26f7e0be40ed7be02d92ef2`.
- Implementation commit after safe rebase: `5831a58f814e36e951ab8beaff05065a330b4199`.
- Scope closed together: M16O-R43, M16O-R44, M16O-R45, M16O-R46, and M16P-R47.
- M16 remains open for independent strict re-audit and owner acceptance.

## R43-R46 Implementation

`H!veAI/src-tauri/src/github_tracking.rs` now provides one production GitHub observer path:

- Every observation performs a lightweight `git ls-remote` HEAD check first.
- Cached blobs are reused when the observed HEAD and cached health are unchanged.
- PROJECT.json, RULES.md, EVENTS.jsonl, and the canonical task ledger are fetched and parsed only after a changed HEAD.
- Fetch and blob parsing failures persist truthful `UNAVAILABLE`, `STALE`, or `ERROR` states without deleting the last successful snapshot.
- Manual and scheduled refresh requests coalesce through one atomic gate.
- Selected-project cadence is 10 seconds; portfolio cadence is 30 seconds.
- Four bounded worker slots provide per-project in-flight suppression, failure isolation, exponential degraded backoff capped at 300 seconds, and graceful shutdown.
- The observer emits `github-tracking-updated` and `hiveai-command-center-refresh` only when the remote identity/health signature changes.

Command Center and Project Cockpit use the cached GitHub v3 remote snapshot as the sole primary project-truth source. Portfolio counts, attention, work queue, recent activity, progress, current task, and health are derived from remote tracker fields only. Local Git, local control-plane, local task intelligence, local audit/session evidence, and local workspace state remain explicitly secondary telemetry and cannot override remote truth. Remote health distinguishes healthy, blocked, waiting on owner, waiting on audit, stale, error, and unavailable states.

The v3 project detector requires the explicit `GITHUB_REMOTE_V3` task-source policy plus a GitHub repository identity. This preserves legacy local read-model behavior for non-v3 fixtures and prevents a remote-looking but non-migrated control-plane fixture from being silently reclassified.

## M16P Addendum - Startup responsiveness and hidden child-process remediation

The startup path no longer performs network refresh, Git blob fetch, or portfolio observation synchronously. `GitHubTrackingManager::start` performs only lightweight portfolio registration and starts a bounded background worker. Selection and manual refresh only signal that worker; UI command paths remain cache-only and never launch Git/network work.

All Git subprocesses used by the GitHub observer run through `process_policy::background_command`, with null standard input, piped output, bounded 30-second execution, timeout kill/wait cleanup, no shell wrapper, and Windows `CREATE_NO_WINDOW`. This removes visible repeated Git terminal windows. The process-policy unit test passed on Windows.

Startup remains cache-first, unchanged-HEAD polling avoids temporary repository reinitialization and repeated add/remove/fetch/show work, and the canonical opening-video bytes at `H!veAI/src/assets/H!veAI.mp4` were not modified.

The governed publisher smoke opened both candidate and stable builds, waited for the embedded `HIVEAI_FRONTEND_READY` marker, checked the `H!veAI` window title, checked forbidden development ports, and detected zero newly visible console hosts. An explicit post-publication launch reproduced the same result: `alive=True`, `title=H!veAI`, `frontendReadyMarker=True`, `newVisibleConhost=0`, elapsed approximately 3091 ms. Owner visual/native acceptance remains pending even though the automated native smoke passed.

## Eight-Repository Remote Matrix

Each row below was fetched from the real GitHub remote branch into an isolated temporary bare repository. The v3 contract probe verified that the current HEAD contains `.hiveai/PROJECT.json`, `.hiveai/RULES.md`, `.hiveai/EVENTS.jsonl`, and the repository-specific canonical task ledger.

| Repository | Branch | Observed HEAD | v3 contract blobs |
|---|---|---|---|
| `Sekiph82/AI-Commerce-HQ` | `H!veAI` | `046df7acb9e966dde26f7e0be40ed7be02d92ef2` | PASS |
| `Sekiph82/Bulk-Edit` | `main` | `9d43d382d3a3339bc40091d90710faea405443c8` | PASS |
| `Sekiph82/fmcg-erp-system` | `main` | `392671e511ea9038669d105bf9635c921cc254f0` | PASS |
| `Sekiph82/FormuLab` | `feature/laboratory-stability` | `a4b0f41e6398a68f32aec1bc14e5eb1eec694443` | PASS |
| `Sekiph82/PackLab` | `main` | `d17fb6db51a55911741707f14eec3f72bed94788` | PASS |
| `Sekiph82/PackLab-3D` | `main` | `df73d218fa1546ba88f65283736ee77700fcb8fd` | PASS |
| `Sekiph82/Scrubbots` | `main` | `5a2ed2765a1d2d77578223f88e2b456d2709d985` | PASS |
| `Sekiph82/ScrubBots-Level-Factory` | `main` | `1cb466b4efa09ea7feaa63c3473c700ecb0aab5d` | PASS |

The current-HEAD matrix is remote evidence only. It does not rewrite any target repository and does not promote local fixture metadata over GitHub truth.

## Regression and Publication Gates

- `cargo fmt --all -- --check`: PASS.
- `cargo test --all-targets -- --test-threads=1`: PASS, 411 passed, 0 failed.
- `cargo test --all-targets --features pty-support -- --test-threads=1`: PASS, 412 passed, 0 failed.
- `npm test -- --run`: PASS, 15 files and 125 tests passed.
- `npm run typecheck`: PASS.
- `npm run build`: PASS.
- `npm audit --audit-level=high`: PASS at the requested threshold. The existing report contains two moderate Vitest advisories and no high/critical advisory.
- `git diff --check`: PASS.
- `scripts/tests/publish-dev-qa-failure-harness.ps1`: PASS, 9/9 rollback, cleanup, lock, and no-bypass cases.
- Governed `scripts/publish-dev-qa.ps1`: PASS. Candidate and stable native smoke passed, rollback boundaries remained available, and the stable executable was published to `H!veAI/dev-bin/H!veAI.exe`.
- Published stable executable SHA-256: `532EBE7C7269A95B519216CF02EFE346116383A8F13F4313C36291FB461AD548`.
- Preserved canonical opening-video SHA-256: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`.

## Publication and Boundary Proof

The implementation was committed after the synchronized prompt commits and rebased without discarding either side. The implementation commit is `5831a58f814e36e951ab8beaff05065a330b4199`. M16P does not activate M17, does not start M21, does not redesign the visible UI, and does not replace the canonical opening video. Existing M09 through M16O behavior remains covered by the full native and frontend suites.

The final log publication commit and pushed branch equality proof are recorded in the final response after this immutable log is committed and pushed. The only untracked paths intentionally left outside the scoped commit are the pre-existing user files `.hiveai/EVENT_INDEX.json`, `.hiveai/HANDOFF.md`, `.hiveai/STATE.json`, `start-demo.bat`, and `task.md`.

M16P REMOTE-ONLY GITHUB TRACKING CLOSURE COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE ACCEPTANCE.
M16 remains OPEN.
M17 NOT ACTIVATED.
M21 NOT STARTED.
