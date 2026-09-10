# Urgent Project JSON Runtime Removal

Date: 2026-09-11
Repository: `Sekiph82/H-veAI`
Branch: `main`
Scope: Updated urgent remediation, nine-project portfolio

## Verdict

PASS. The production native path no longer reads, requests, parses, validates, discovers, or displays legacy `.hiveai/PROJECT.json` state for GitHub-tracked projects. Project truth is GitHub plus each repository's root `TASKS.md`.

The active portfolio is exactly nine logical projects. `Sekiph82/AI-Commerce-HQ` remains active on its configured `H!veAI` branch, alongside the separate `Sekiph82/H-veAI` project. Stale persisted local/path records are archived during portfolio upgrade; they are not hidden in the frontend.

## Root Causes

1. The GitHub-first migration still routed remote cockpit reads through the local dashboard, control-plane, truth resolver, and task-intelligence paths. Those paths could request `.hiveai/PROJECT.json` and surface the resulting error.
2. Startup reconciliation treated remote projects like local projects and ran legacy physical/control-plane repair work.
3. Existing persisted state still contained the old eight-project target set and legacy `GITHUB_TRACKING_V3` cache rows.
4. The hidden curl runner held stdout/stderr pipes without draining them. Large real `TASKS.md` responses filled the Windows pipe buffer, causing false 35-second timeouts.
5. Several real repositories declare the current task in the status block without repeating it in the historical checklist. The parser incorrectly treated that valid shape as an observation error.

## Changes

- Reconciled the persisted portfolio to the exact nine GitHub identities and preserved AI-Commerce-HQ's configured branch.
- Archived non-portfolio persisted projects and removed all legacy sync-resource rows.
- Short-circuited remote Command Center, dashboard, cockpit, control-plane, watcher, and native command paths to the remote snapshot model before any local legacy read.
- Kept local repository data as secondary telemetry only.
- Persisted first-observation failures truthfully.
- Drained hidden HTTP process output concurrently, preserving `CREATE_NO_WINDOW` and bounded timeout behavior.
- Accepted valid status-block task declarations without requiring duplicate checklist rows.
- Updated the user-facing authority wording and the affected frontend test expectation.

## Verification

- `cargo fmt --manifest-path src-tauri/Cargo.toml --all`: PASS
- `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`: PASS
- `cargo test --manifest-path src-tauri/Cargo.toml --lib --quiet`: **415 passed, 0 failed**
- `npm test -- --run`: **125 passed, 15 files passed**
- `npm run typecheck`: PASS
- `npm run build`: PASS
- `npm run tauri:build`: PASS
- `git diff --check`: PASS
- Direct remote gate: all nine tracked branches resolved and all nine root `TASKS.md` files returned non-empty content.

## Native Production Evidence

Published executable: `dev-bin/H!veAI.exe`

- Native launch smoke: process responsive, title `H!veAI`.
- Existing persisted database upgraded in the real app path: `ACTIVE_COUNT=9`.
- Remote cache: `REMOTE_CACHE_ROWS=9`; every active project has a `CURRENT` snapshot after background polling.
- Legacy cache: `LEGACY_CACHE_ROWS=0`.
- ScrubBots - Pixel Art Generator displayed remote `PAG-M09`, current task `CLI & Local Batch Generation`, its current next action, and computed progress from GitHub root `TASKS.md`.
- AI-Commerce-HQ and H-veAI remained separate active identities.
- No `cmd`, PowerShell, Terminal, or visible Git child remained under the native app after polling; the only observed child was WebView2.
- Canonical opening-video SHA-256 unchanged: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`.
- Published EXE SHA-256: `DA063EE2A1883C7F49C7E123F4D8CA9E88F6C75C2551137B2324C02A73709F09`.

## Git Publication

Implementation commits: `0b5e155`, `1d3e475`, `359d6bc`, `0c53241`.

This log is immutable after publication. The final log commit and final `origin/main` SHA are recorded in the completion response after push.

No M17/M18 work was started.
