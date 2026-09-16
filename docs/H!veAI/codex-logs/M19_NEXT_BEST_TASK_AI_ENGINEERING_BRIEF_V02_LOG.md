# M19 Next Best Task / AI Engineering Brief — V02 Remediation Log

- Date: 2026-09-16
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative remediation prompt: [`M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_REMEDIATION_PROMPT.md`](../prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_REMEDIATION_PROMPT.md)
- Starting synchronized SHA: `44fe673a30b58dea508df214d385ba95956ec1c6`
- Ending implementation SHA: `91dee49724830c715ae17881c1d4db98e3e170f3`

## Scope and governance

This V02 remediation stayed within M19.00. `TASKS.md` and `CODEX_ROADMAP.md` were not modified, staged, or used as writable planning state. No M20 task was started, claimed, or advanced. No reset, rebase, force-push, clean, auto-stash, or unrelated-worktree overwrite was used.

The source implementation commit is the immutable predecessor of this log commit. The final log commit records the verified implementation SHA and is pushed non-force to `main`; after equality verification the workflow stops for independent source re-audit. Owner-native acceptance is not self-claimed by Codex and remains deferred until that independent V02 re-audit returns PASS.

## Closure evidence — V01 strict findings

### F-M19-V01-STRICT-001 — seed bootstrap-only behavior

`github_tracking::ensure_portfolio()` now creates missing historical seed rows but preserves non-null Registry task policy, repository URL/owner/repo, branch/default branch, and Git capability state. Archive state and explicit removal exclusions are preserved; non-seed projects are not reconciled by the seed list. Direct test `ensure_portfolio_preserves_seed_settings_identity_archive_and_remove` covers custom settings, repository identity, branch, capability, archive, removal/exclusion, and repeated reconciliation. Existing FormuLab identity/cache preservation and ninth/tenth persistence tests also pass.

### F-M19-V01-STRICT-002 and 008 — exact root TASKS authority and freshness

M19 reads the canonical repository-root `TASKS.md` directly at decision time, computes SHA-256 before and after native `task_intelligence::parse()`, and admits only `source_kind=TASKS`, exact relative `TASKS.md`, and matching evidence hash. ROADMAP/HANDOFF/CUSTOM rows are excluded. Missing, invalid UTF-8, oversized, empty, parser-warning, unreadable, and during-decision changed roots fail closed into unavailable/stale/malformed evidence. Tests cover mixed-source filtering, valid fresh root, missing/malformed root, and persisted-snapshot comparison.

### F-M19-V01-STRICT-003 — complete per-task remote truth

`RemoteTaskRow` now carries per-task actor, dependencies, blockers, owner gate, external wait, priority, root content hash, and metadata completeness in addition to ID/title/status/path/line. The GitHub root parser extracts metadata per task rather than applying snapshot-global actor/blocker values. Incomplete or stale per-task metadata is routed to unavailable/attention and cannot become a recommendation. `remote_task_metadata_is_per_task_and_malformed_fails_closed` plus the GitHub integration production identity/cache/partial-failure matrix pass.

### F-M19-V01-STRICT-004 and 005 — full graph, dependency unlock, fairness

The full canonical graph is built before eligibility deferral. Completed, blocked, waiting, and unknown-actor rows remain represented while unique unfinished dependent edges are counted once. Output is bounded only after global scoring, so a later project cannot be starved by an earlier project. Direct test evidence: 141 candidates are scored before the 128 output bound; a later project with task priority 100 ranks first; the prerequisite receives exactly one unlock edge worth 15 points; the blocked dependent remains attention.

### F-M19-V01-STRICT-006 and 007 — real Engineering Brief and M19 attention

The native M19 snapshot is now embedded in `CommandCenterSnapshot.engineering_brief` and the Command Center M19 Engineering Brief panel. It exposes portfolio state, rank-1 recommendation, bounded alternatives, rank/difference, evidence, score components, attention, provider/actor readiness, and unavailable/partial/stale inputs. M19 attention is deduplicated against legacy attention by project/task/category. The existing M18 GitHub error boundary remains intact and the M18 focused suite passes.

### F-M19-V01-STRICT-009 — complete scoring and fail-closed actors/evidence

The deterministic score includes Registry project priority, authoritative task priority, exact dependency unlocks, fresh linked audit/test failure urgency, actor readiness, context-switch cost, owner-focus (zero with explicit “no owner-focus setting configured” evidence), and uncertainty. Unknown/unavailable actor, database query failure, malformed/missing failure timestamp, stale/missing evidence, HUMAN/EXTERNAL wait, CI, and GPT Audit all fail closed into attention/unavailable states. Historical/closed failures do not receive fresh urgency.

### F-M19-V01-STRICT-010 — rank-aware explanations

Recommendation conversion receives actual rank and top score. Only rank 1 uses “Ranked first”; alternatives state their actual rank and bounded score difference, including ties. Direct rank-1/rank-2 test passes.

### F-M19-V01-STRICT-012 — registered local-only success

Add Project success inspects the returned `Repository` record. GitHub-linked registration reports the bounded owner/repository identity; successful local/non-GitHub registration reports that it was registered locally and that GitHub identity is unavailable/not applicable. Both frontend states are directly tested.

## Production/native path evidence

- Exact root authority: local `TASKS.md` canonicalization + bounded read + before/after SHA-256 + native parser evidence hash match.
- Freshness: decision-time mutation is rejected; missing/malformed roots are unavailable and never recommended.
- Remote: GitHub-tracked branch root `TASKS.md` is parsed into per-task rows with task-local metadata and root hash/head evidence; stale/incomplete rows are attention-only.
- Portfolio: historical eight seeds remain bootstrap-only; explicit ninth/tenth Registry projects survive refresh/restart; archive/remove exclusions persist.
- Actor matrix: CODEX/CLAUDE require native provider availability; HUMAN/EXTERNAL are waits; CI/GPT_AUDIT are non-executable; UNKNOWN/unavailable is fail-closed attention.
- Engineering Brief: M19 is delivered through the native Command Center snapshot and one UI surface, not a detached second polling authority.

## GitHub acquisition matrix (bounded/redacted)

The production-native GitHub tests exercise these repository identities on `main`: `Sekiph82/H-veAI` (control), `Sekiph82/Bulk-Edit`, `Sekiph82/ScrubBots-Level-Factory` (Pixel Art Generator), and `Sekiph82/fmcg-erp-system` (additional working control). The matrix covers exact resource paths, primary-cache boundedness, optional enrichment isolation/partial state, 403/429/rate-limit classification, timeout/error/stale-last-good/malformed cache behavior, request-budget reservation, and secret redaction. No token, Authorization value, cookie, credential, or browser/session secret is present in this log. GitHub integration focused result: 24 passed; GitHub tracking focused result: 15 passed.

## Test and publication gates

- `npm run typecheck`: PASS.
- `npm run build`: PASS. Existing non-blocking Vite warnings: dynamic-import chunk note and large bundle advisory.
- Focused frontend `tests/m07.06-focused.test.tsx`: 31 passed.
- Full frontend `npm test -- --reporter=dot --maxWorkers=1`: 18 files / 159 tests passed. The default parallel invocation was also run; its three intermittent 5-second timeout failures were not reproducible in serial or focused runs and are the established resource-sensitive frontend timeout class.
- `cargo check --manifest-path src-tauri\\Cargo.toml`: PASS (existing warnings only).
- Focused Rust: M19 `7 passed`; GitHub tracking `15 passed`; GitHub integration `24 passed`; Command Center `29 passed`.
- Full Rust `cargo test --manifest-path src-tauri\\Cargo.toml --lib`: 529 passed, 1 known pre-existing M16 observational failure at `control_plane::tests::m16l_current_command_center_cockpit_and_control_reads_are_observational` after 259.59s. The test output continued through the established M16 observational path; no M19 test failed. This is recorded rather than relabeled as a V02 pass.
- `rustfmt --check --edition 2021 src-tauri/src/next_best_task.rs`: PASS. Full `cargo fmt -- --check` and checks of older touched files report unrelated pre-existing formatting drift in existing files; no `git diff --check` whitespace error exists.
- `git diff --check`: PASS.
- Publication helper: `scripts/publish-dev-qa.ps1` PASS twice after final source; release `--no-bundle` build, PE validation, hidden readiness smoke, forbidden-port check, no-visible-console check, stable swap, shortcut target/icon verification, stable re-smoke, and rollback-safe publication all passed.
- Published binary: `dev-bin/H!veAI.exe`, 24,502,784 bytes, SHA-256 `54230F277BE97B243FF288F8D9A933070DB19A2E1CBEA89D3044A6D6420D5229`.

## Final equality gate

Before this log is committed, verify the implementation commit SHA above against `git rev-parse HEAD`, then commit this file without further source changes. Push both commits non-force to `origin/main`. Verify clean worktree, `HEAD == origin/main`, live GitHub `main` resolves to the same SHA, and this log path is reachable from GitHub. This file is immutable after publication; any later correction requires a new explicitly versioned log, not an edit.

Final statements: `TASKS.md` and `CODEX_ROADMAP.md` were unchanged; M20 was not started; owner-native final acceptance is not self-claimed; stop for independent V02 strict re-audit.
