# M16R — Native Registry, Remote Tracking, and Startup Recovery

## Authority

Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`

Read and obey before changing code:

1. `H!veAI/GPT.md`
2. `H!veAI/CONSTITUTION.md`
3. `H!veAI/TASKS.md`
4. `H!veAI/CODEX_ROADMAP.md`
5. `H!veAI/docs/H!veAI/GITHUB_FIRST_PROJECT_TRACKING_CONTRACT_V3.md`
6. `H!veAI/docs/H!veAI/audits/M16Q_OWNER_NATIVE_ACCEPTANCE_FAILURE_AND_M16R_SCOPE.md`
7. `H!veAI/docs/H!veAI/codex-logs/M16O_GITHUB_FIRST_EIGHT_REPOSITORY_TRACKING_RESET_LOG.md`
8. `H!veAI/docs/H!veAI/codex-logs/M16P_REMOTE_ONLY_PRIMARY_TRUTH_AND_LIVE_GITHUB_POLLING_CLOSURE_LOG.md`
9. `H!veAI/docs/H!veAI/codex-logs/M16Q_PROJECT_COCKPIT_REMOTE_ISOLATION_AND_LOCAL_TELEMETRY_FIREWALL_REMEDIATION_LOG.md`

This is ONE comprehensive closure remediation for the complete owner-native defect set discovered after M16Q.

Do not stop after R50, R51, or R52 individually.
Do not return for audit until the full M16R acceptance matrix is implemented and tested together.
Do not activate M17.
Do not start M21.
Do not close M16 yourself.

---

# Product intent

H!veAI is a GitHub-first project control application for exactly eight tracked projects.

The owner must be able to:

- launch H!veAI immediately;
- see the opening video without waiting on GitHub/Git/database/project enumeration;
- see exactly eight logical projects, never duplicate local/remote registrations;
- open any project once;
- see its current GitHub milestone, current task, next action, progress, workflow and health;
- leave H!veAI running and have those values update automatically when GitHub changes.

For the migrated v3 portfolio, GitHub remote tracker files are primary truth. Local workspaces are optional secondary telemetry only.

---

# Fixed eight-project portfolio

There must be exactly one logical registered project for each of these repository identities:

1. `Sekiph82/AI-Commerce-HQ` branch `H!veAI`
2. `Sekiph82/Bulk-Edit` branch `main`
3. `Sekiph82/fmcg-erp-system` branch `main`
4. `Sekiph82/FormuLab` branch `feature/laboratory-stability`
5. `Sekiph82/PackLab` branch `main`
6. `Sekiph82/PackLab-3D` branch `main`
7. `Sekiph82/Scrubbots` branch `main`
8. `Sekiph82/ScrubBots-Level-Factory` branch `main`

The existing local paths may remain associated with these logical projects as metadata, but local path identity must never create a second project row.

---

# R50 — BLOCKER — collapse 16 registry rows back to exactly 8 logical projects

The native owner build currently shows 16 projects. Each GitHub repository has effectively been registered twice: one original/local row plus one path-missing remote row.

Fix the architecture, not only the displayed list.

## Required registry identity model

For every migrated GitHub-v3 project, the canonical uniqueness identity is:

`normalized GitHub owner + normalized repository + tracked branch policy`

Local path is metadata attached to the same project. It is not a separate project identity.

Inspect the real registry schema, migrations, seeding code, startup registration code and any M16O/P additions that inserted GitHub-first records.

Implement a durable migration/reconciliation path that:

1. Detects duplicate pairs by GitHub repository identity.
2. Selects one canonical project record deterministically.
3. Preserves the useful local path from the existing local registration where available.
4. Preserves GitHub-v3 tracking policy and remote identity on the canonical record.
5. Reparents or deterministically migrates foreign references from the duplicate record before deletion, including any task sources, workflow state, audits, tests, sessions, permissions, cached GitHub sync state, activity/history and other project-id keyed records.
6. Deletes only the duplicate registry identity after references are safe.
7. Does not delete the user's actual local folders.
8. Is idempotent. Re-running startup/migration must never recreate duplicates.
9. Prevents future duplicate insertion with a database/schema/application invariant, not only frontend filtering.

If the database can support a safe unique index for normalized remote identity, add one with a migration strategy compatible with existing user data. If not, implement an equally strong transactional uniqueness guard and prove it with concurrency/idempotence tests.

## UI requirements

After repair:

- Projects page shows exactly 8 cards.
- Command Center says exactly 8 projects.
- shortcuts show one row per project, not duplicate pairs.
- there are no extra `Path missing` duplicate cards for these eight repositories.
- every project card opens the one canonical cockpit.

Do not hide duplicate rows in React while leaving bad database rows behind.

---

# R51 — BLOCKER — make the production GitHub observer actually populate remote truth

Owner-native evidence shows `ScrubBots - Pixel Art Generator` stuck at:

- `GitHub refresh is pending; no cached remote snapshot is available`
- remote HEAD unavailable
- milestone/task/workflow/next action/progress unknown

This is unacceptable even if the UI is technically truthful.

The earlier remote matrix proved that the required v3 contract files exist remotely. Diagnose why the production native observer is not delivering them to the actual project record used by Project Cockpit.

## Trace the entire real lifecycle

Instrument/test and inspect these boundaries in production code:

1. canonical project selected from registry;
2. project qualifies as GitHub v3;
3. owner/repository/branch identity reaches GitHubTrackingManager;
4. manager startup registers exactly those eight canonical project IDs;
5. cache lookup uses the same canonical project ID/remote identity as the cockpit;
6. observer worker is actually scheduled;
7. `git ls-remote` or equivalent remote observation launches through hidden process policy;
8. timeout/error/output handling is correct;
9. changed or first HEAD triggers blob fetch;
10. PROJECT/TASKS/RULES/EVENTS parse against v3 contract;
11. successful snapshot is transactionally persisted;
12. cached snapshot is retrievable on the very next cockpit/command-center read;
13. `github-tracking-updated` / refresh events invalidate or refresh the relevant frontend state;
14. selected-project 10-second polling and portfolio 30-second polling continue after first run;
15. a remote HEAD change replaces the cached primary snapshot without app restart.

Do not solve this by falling back to local files.

## First-refresh behavior

A new/fixed canonical project with no cache must not remain indefinitely `pending`.

Define a bounded first-observation state machine such as:

`UNINITIALIZED -> FETCHING -> CURRENT | STALE | ERROR | UNAVAILABLE`

The exact implementation is up to you, but it must be observable and testable. A refresh request cannot silently disappear due to duplicate IDs, coalescing, manager registration, backoff, or cache-key mismatch.

## Remote truth fields

Once CURRENT, both Command Center and Project Cockpit must use the same cached snapshot for:

- repository
- branch
- remote HEAD
- tracker-updated timestamp
- milestone
- sprint
- current task ID/title
- workflow
- required actor
- next action
- blockers
- progress scope/count/percent
- last completed task
- health

## Required real-repository verification

During validation, verify all eight actual GitHub repositories/branches listed above, not only fixtures.

For each one, record:

- remote HEAD
- PROJECT blob result
- TASKS blob result
- RULES blob result
- EVENTS blob result
- parsed milestone/sprint/current task
- parsed next action
- parsed progress or explicit null
- resulting remote health

For `Sekiph82/ScrubBots-Level-Factory@main`, explicitly prove the current values from the remote tracker at validation time and show those exact values through the same production cache/read model used by the native UI.

Do not hardcode PAG-M02/PAG-M05/PAG-M06 or any milestone into H!veAI. The remote repository remains authoritative.

---

# R52 — MAJOR — restore immediate startup-video playback

Owner reports the accepted startup video now begins only after approximately 15–20 seconds.

The original accepted product behavior is that launching `H!veAI.exe` immediately presents and starts the canonical opening video, then enters the application.

## Required startup architecture

Audit the true cold-start critical path from process entry to first visible video frame.

Before first-frame playback, there must be no blocking dependency on:

- GitHub remote observation
- Git subprocesses
- portfolio refresh
- eight-project remote polling
- duplicate-registry repair beyond the minimum DB-open/migration work that cannot safely be deferred
- Project Cockpit snapshot generation
- Command Center snapshot generation
- local Git probing
- task intelligence
- local control-plane parsing
- large synchronous project enumeration

Move nonessential work after the video/frontend is visible and ready.

If migrations must run before UI creation, measure them. If the new dedupe migration could be large, keep it bounded/transactional and ensure it cannot create a 15-second blank window. Prefer showing the startup surface before background portfolio warming whenever technically safe.

Do not replace or re-encode the opening video.

Canonical video:

`H!veAI/src/assets/H!veAI.mp4`

Required SHA-256:

`C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`

## Startup timing evidence

Add instrumentation/test evidence for at least:

- process start -> native window created
- process start -> frontend/startup surface ready
- process start -> video play requested
- process start -> first-frame/playback-ready signal if available
- time at which GitHub background tracking starts

The GitHub observer must start after it can no longer delay first visible startup playback.

The previous approximately 3-second process-alive smoke is insufficient by itself. Specifically validate video-start latency.

---

# Frontend and routing recovery

Inspect Projects, Command Center and Project Cockpit together.

Required:

- exactly eight logical rows/cards everywhere;
- no stale duplicate shortcut rows;
- no clickable ghost/path-missing duplicate;
- no blank cockpit caused by registry duplicate identity;
- loading state is explicit while first remote fetch is actually in flight;
- when remote snapshot becomes CURRENT, the UI updates without requiring app restart;
- local workspace telemetry remains collapsed/secondary and never replaces GitHub primary truth.

Do not perform unrelated visual redesign.

---

# Mandatory direct regression tests

Do not rely on test names. Add direct bodies that would fail on the current owner-reported build.

## Registry tests

1. Existing DB with 16 rows representing eight duplicate GitHub pairs -> migration -> exactly 8 canonical rows.
2. Foreign project-id references on both halves of a duplicate pair -> reconciliation preserves/reparents data.
3. Fresh database -> exactly 8 seeds.
4. Restart x5 -> still exactly 8.
5. Concurrent/duplicate seed attempts -> no 9th row.
6. Canonical row preserves local path when one duplicate has it.
7. No local path -> project still exists once and remote cockpit works.

## Remote observer tests

8. Canonical registry ID receives first remote snapshot and cockpit reads same cache key.
9. No-cache startup moves out of pending into CURRENT/ERROR within bounded time.
10. selected-project refresh cannot be swallowed by in-flight/coalescing state.
11. unchanged HEAD reuses cache.
12. changed HEAD refreshes blobs and UI/read model.
13. remote error preserves prior stale remote snapshot, never local truth.
14. all eight real remote identities pass v3 validation at final verification.
15. Command Center and Cockpit return identical remote HEAD/milestone/task/progress semantics.

## Startup tests

16. Cold native start does not synchronously run portfolio GitHub observation before startup surface/video request.
17. background Git processes remain `CREATE_NO_WINDOW` / hidden.
18. video hash unchanged.
19. startup timing harness records prompt video start and fails a regression that delays playback into the owner-observed 15–20 second range.

## UI tests

20. Projects page receives eight records and renders eight cards.
21. shortcuts render eight unique logical projects.
22. canonical project cockpit is navigable for every portfolio entry.
23. remote update event replaces pending/unknown fields with CURRENT data.

---

# Whole-scope validation

Run the full relevant suite after all three findings are solved together:

- migration/database tests
- project registry tests
- GitHub tracking tests
- Project Cockpit tests
- Command Center tests
- startup/readiness/video tests
- all Rust tests
- all-targets
- pty-support
- frontend Vitest
- TypeScript typecheck
- production build
- npm audit high threshold
- cargo fmt check
- git diff --check
- publisher rollback harness 9/9
- governed publication
- actual native cold-start smoke
- actual eight-repository remote tracking matrix

Do not publish if any of the 23 direct acceptance cases fail.

---

# Native QA required before returning

After publishing the stable EXE, perform the strongest native automation possible and leave these final owner checks pending explicitly:

1. double-click stable EXE;
2. startup video starts promptly;
3. no visible terminal windows;
4. Command Center says 8 projects;
5. Projects page shows 8 unique cards;
6. no duplicate `Path missing` project cards;
7. open all eight cockpits successfully;
8. Pixel Art Generator shows current GitHub remote tracker state rather than `Unknown`;
9. wait through at least one selected-project polling interval and prove refresh continues;
10. compare UI/read-model values with exact remote `.hiveai/TASKS.md` values at the same HEAD.

Owner visual/native acceptance is still required after your builder run.

---

# Evidence discipline

Builder log is a claim, not independent acceptance.

For every fixed finding include:

- pre-fix reproduction or a regression test demonstrably representing the owner failure;
- production symbols/files changed;
- exact test body names and what each proves;
- final database row count;
- exact eight canonical identities;
- actual remote matrix;
- native startup timing;
- published stable EXE SHA-256;
- canonical video SHA-256;
- implementation commit;
- final branch HEAD.

Do not claim PASS because a helper test passed while the production path differs.

---

# Required immutable log

Create:

`H!veAI/docs/H!veAI/codex-logs/M16R_NATIVE_REGISTRY_REMOTE_TRACKING_AND_STARTUP_RECOVERY_LOG.md`

The log must contain separate evidence sections for R50, R51 and R52, followed by one whole-scope validation table.

End exactly:

`M16R NATIVE REGISTRY + REMOTE TRACKING + STARTUP RECOVERY IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`