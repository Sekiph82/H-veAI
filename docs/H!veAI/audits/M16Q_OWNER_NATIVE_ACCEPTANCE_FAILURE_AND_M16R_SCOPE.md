# M16Q Owner Native Acceptance Failure and M16R Scope

## Verdict

**M16Q NATIVE ACCEPTANCE: FAIL**

Technical re-audit remains informative, but owner native evidence has exposed product regressions that block M16 closure.

- BLOCKER: 2
- MAJOR: 1
- MINOR: 0

M16 remains **OPEN**.
M17 is **NOT ACTIVATED**.
M21 is **NOT STARTED**.

## Native evidence supplied by owner

The owner tested the newly published native Windows build on 2026-09-10 and supplied screenshots showing the following live behavior.

### M16R-R50 — BLOCKER — Registry identity duplication: 8 intended projects became 16

The Command Center reports `Projects 16` and `16 registered workspaces` even though the product contract is the fixed eight-project portfolio.

The Projects page visibly contains duplicate records for the same GitHub identities. One record is the original active/local registration and a second record for the same repository appears as `Path missing`, `Non-Git folder`, `Not detected`.

Examples visible in owner evidence include duplicate pairs for:

- AI-Commerce-HQ
- Bulk-Edit
- fmcg-erp-system
- FormuLab
- and the remaining portfolio entries via the expanded list

The duplicate remote registration is not an acceptable representation of one logical project. Registry identity must be one row per tracked GitHub repository, with local workspace metadata attached to that same logical project when available.

Opening a duplicate `Path missing` cockpit produces an effectively blank page, confirming that the duplicate entries are not merely cosmetic.

**Required closure invariant:** exactly 8 logical projects, exactly one registry identity per GitHub repository, no duplicate shortcut, card, Command Center row, or cockpit route.

### M16R-R51 — BLOCKER — GitHub remote current-state tracking is not functioning in the owner-native cockpit

For `ScrubBots - Pixel Art Generator`, the Project Cockpit displays:

- `GitHub refresh is pending; no cached remote snapshot is available`
- `NEEDS_RECONCILIATION`
- Remote HEAD `Unavailable`
- milestone `Unknown`
- sprint `Unknown`
- current task `Unknown`
- workflow `Unknown`
- next action `Unknown`
- progress `Unknown`
- last completed `Unknown`

This directly violates the product purpose of the GitHub-first tracker. The target repository is known and the earlier M16O remote matrix proved that the required v3 blobs existed on the tracked branch. A production build that never materializes the remote snapshot is not accepted merely because the UI truthfully says `Unknown`.

The issue must be diagnosed across the actual production observer lifecycle: registration identity, manager start, polling worker, repository/branch mapping, Git command/network execution, cache persistence, cache lookup key, frontend event invalidation, and selected-project refresh.

**Required closure invariant:** on a normal connected machine, all eight registered GitHub projects automatically obtain and continuously refresh their remote v3 snapshot. Project Cockpit and Command Center must show the exact remote milestone/current task/next action/progress/health without requiring local tracker files.

### M16R-R52 — MAJOR — Startup video regressed from immediate playback to approximately 15–20 seconds delayed

Owner reports that launching the desktop executable no longer starts the accepted opening video immediately. The application window appears and the startup video begins only after approximately 15–20 seconds.

The accepted native behavior is that double-clicking the stable EXE immediately enters the startup-video experience and then the application becomes usable. Network observation, registry migration, database work, cache probing, Git execution, or project enumeration must not delay first visible video playback.

The opening video bytes themselves must remain unchanged.

**Required closure invariant:** startup-video first frame/playback begins promptly on launch and is structurally isolated from GitHub polling/registry/cache initialization.

## Audit interpretation

The previous M16Q technical PASS does not override this owner-native failure. The M16Q code successfully isolated remote-primary fields from local telemetry, but the product still fails because:

1. logical registry identity is duplicated;
2. the production remote snapshot pipeline is not yielding usable remote truth in the native application;
3. the startup critical path has regressed visibly.

These three findings must be remediated together in one bounded M16R closure cycle. Do not fix only one symptom and return.

## Required M16R acceptance matrix

M16R must prove all of the following in one implementation cycle:

1. Existing upgraded user database with duplicate eight+eight registrations is migrated/deduplicated safely to exactly eight logical projects.
2. Fresh database also seeds exactly eight, never sixteen.
3. Repeated startup is idempotent and remains exactly eight.
4. GitHub identity is the uniqueness key for migrated GitHub-v3 projects; local path is metadata, not a second identity.
5. Existing project IDs/references are preserved or deterministically remapped without orphaning task/session/audit/history references.
6. No `Path missing` duplicate cards remain for the eight tracked repositories.
7. Opening every one of the eight project cards routes to a working cockpit.
8. On connected native startup, remote snapshots for all eight become CURRENT within a bounded testable interval.
9. Selected-project refresh works and is not permanently stuck at `pending`.
10. `ScrubBots-Level-Factory@main` displays the remote milestone/task/next-action/progress from the actual GitHub v3 tracker rather than `Unknown`.
11. Command Center and Cockpit read the same cached snapshot identity/HEAD.
12. An actual remote HEAD change is detected and reflected without app restart.
13. Offline/degraded mode remains truthful and never substitutes local tracker truth.
14. Startup video starts promptly before GitHub/network/portfolio background work can block it.
15. No visible Git/cmd/PowerShell/Terminal windows appear.
16. Opening video SHA-256 remains the accepted canonical value.

## Final state

**M16Q OWNER NATIVE ACCEPTANCE FAILED.**

**M16R-R50 BLOCKER OPEN.**  
**M16R-R51 BLOCKER OPEN.**  
**M16R-R52 MAJOR OPEN.**

**M16 remains OPEN.**  
**M17 NOT ACTIVATED.**  
**M21 NOT STARTED.**