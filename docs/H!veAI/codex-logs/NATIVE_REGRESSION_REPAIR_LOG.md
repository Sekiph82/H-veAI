# Native Regression Repair Log

Date: 2026-09-10
Scope: H!veAI native project tracking, remote refresh, and startup responsiveness

## Root Causes

- The GitHub v3 portfolio reconciler matched only an exact canonical identity, so the eight older `DISCOVER_STANDARD_FILES` rows survived beside the eight GitHub rows. Local-path identity was therefore incorrectly treated as a second project.
- Remote refresh events were not consumed by the cockpit or registry views, and production watcher reconciliation still inspected local project roots. A reachable GitHub repository could therefore remain visually pending or be contaminated by stale local metadata.
- Tauri setup performed portfolio repair, steady-state database integrity work, local watcher initialization, and related reconciliation synchronously before the first usable frame. The one-time duplicate repair was especially expensive on the existing database.

## Changes

- Added a transactional eight-target portfolio reconciliation that promotes the canonical GitHub identity, reassigns project-owned child rows, preserves useful path/preferences, and deletes duplicate project/repository rows.
- Made production watcher reconciliation GitHub-v3 remote-only, while retaining local watcher behavior for isolated local/test managers.
- Deferred portfolio repair and watcher registry refresh to background work, removed the steady-state full integrity preflight for an already-current database, and kept the startup path responsive.
- Subscribed the registry and exact project cockpit to the existing refresh events so background GitHub updates repopulate Command Center, Projects, shortcuts, and cockpit state without restart.
- No visible UI redesign was made and the canonical opening-video bytes were preserved.

## Native Evidence

- Governed publication passed, including production build, stable executable swap/rollback checks, shortcut checks, frontend readiness, and no development ports.
- Published executable: `dev-bin/H!veAI.exe`
- Published executable SHA-256: `E08FB4FADC926E35A54DFF3BE9EB03123A2375E62C512EF6CA7DCFBE8E2560A`
- Canonical opening video SHA-256: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`
- Live database evidence: exactly 8 active projects, all `GITHUB_REMOTE_V3`, 8 sync rows, and no duplicate repository identities. ScrubBots - Pixel Art Generator resolves to remote HEAD `4379400527bfd81061512840958f3e939b880e87`, `CURRENT` health, milestone `PAG-M07`, and current task `Structural Metric Semantics & Acceptance Evidence Remediation`.

## Verification

- Rust full regression without PTY: `416 passed; 0 failed`.
- Rust full regression with PTY: `417 passed; 0 failed`.
- Frontend tests: 15 files, 125 tests passed.
- Typecheck, production frontend build, focused GitHub merge/cockpit/watcher tests, database corruption safety test, formatting, diff check, and high/critical npm audit gate passed.
- Existing audit output reports two moderate Vitest advisory findings; no high or critical findings.
- M17 and M21 were not activated.
