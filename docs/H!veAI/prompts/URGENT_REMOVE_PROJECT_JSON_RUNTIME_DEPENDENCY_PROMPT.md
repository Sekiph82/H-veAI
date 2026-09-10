# URGENT — Remove PROJECT.json Runtime Dependency and Restore TASKS.md-Only Tracking

The native H!veAI application is still failing in production because it continues to request legacy `.hiveai/PROJECT.json` files from tracked GitHub repositories.

This is a direct violation of the current owner-approved tracking architecture.

## Owner-approved architecture

For project tracking, GitHub is the only source of truth.

Each tracked repository has exactly one project-management truth source:

`TASKS.md` at the repository root.

H!veAI may also read normal GitHub repository metadata such as repository identity, tracked branch, HEAD SHA, latest commit metadata, push/activity metadata and similar native GitHub information.

H!veAI must NOT require, fetch, parse, validate, discover, fall back to, or display errors about any legacy H!veAI control-plane file, including:

- `.hiveai/PROJECT.json`
- `.hiveai/TASKS.md`
- `.hiveai/RULES.md`
- `.hiveai/EVENTS.jsonl`
- `.hiveai/STATE.json`
- HANDOFF files
- PROJECT_DASHBOARD files
- legacy manifest/control-plane files

Historical files may remain in repository history or documentation, but they are not production runtime inputs.

## Native failure evidence

Owner-native screenshots show the current published application still rendering errors such as:

`fatal: path '.hiveai/PROJECT.json' does not exist in '<commit sha>'`

This occurs in Project Cockpit and Command Center for repositories that were intentionally converted to root `TASKS.md` tracking.

The visible consequences include:

- Current task unavailable
- milestone Unknown
- sprint Unknown
- workflow Unknown
- progress Unknown
- remote health ERROR
- Command Center portfolio health incorrect
- Active tasks and Completed tasks unavailable
- legacy `GITHUB_REMOTE_V3` wording still visible

The application is therefore still executing a legacy runtime path despite the migration work and passing automated tests.

## Required result

Diagnose the production runtime path that is still reading `.hiveai/PROJECT.json` and remove that dependency completely.

Do not merely suppress the visible error message. The legacy fetch/parse/fallback path must not be used by the production GitHub tracking flow.

After this repair, every tracked project must resolve its current project state from:

1. GitHub repository/branch metadata; and
2. root `TASKS.md`.

Nothing else is required for project status.

The root `TASKS.md` parser must provide, where present or derivable:

- current milestone
- current sprint
- current task
- current task status
- next task / next action
- required actor when used by the project
- blockers when represented in TASKS.md
- total task count
- completed task count
- remaining/open task count
- completion percentage derived from completed / total
- last completed task when derivable

Latest GitHub commit metadata may be shown as supporting information but must not override TASKS.md task truth.

## Portfolio requirement

The active portfolio must contain exactly these 9 repositories:

1. `Sekiph82/H-veAI` — `main`
2. `Sekiph82/AI-Commerce-HQ` — keep its currently configured tracked branch; do not remove it from the portfolio
3. `Sekiph82/Bulk-Edit` — `main`
4. `Sekiph82/fmcg-erp-system` — `main`
5. `Sekiph82/FormuLab` — `feature/laboratory-stability`
6. `Sekiph82/PackLab` — `main`
7. `Sekiph82/PackLab-3D` — `main`
8. `Sekiph82/Scrubbots` — `main`
9. `Sekiph82/ScrubBots-Level-Factory` — `main`

`Sekiph82/AI-Commerce-HQ` is still an active tracked project. The standalone `H-veAI` repository is an additional project, not a replacement for AI-Commerce-HQ in the H!veAI portfolio.

Existing persisted/local application database state must converge to these exact 9 logical GitHub projects without duplicate local/remote identities or obsolete source-policy behavior after the upgrade.

## User-facing terminology

Remove production UI terminology that implies the obsolete control-plane architecture, including `GITHUB_REMOTE_V3`, `PROJECT.json`, v3 manifest authority, or similar internal legacy labels.

User-facing project tracking should simply communicate that the source is GitHub and that task authority is root `TASKS.md`.

## Command Center expected behavior

With all 9 projects successfully read from GitHub/root TASKS.md:

- Projects = 9
- Active tasks must show a real calculated value when task data exists
- Completed tasks must show a real calculated value when task data exists
- Running must reflect actual current/in-progress task/workflow state according to TASKS.md semantics
- Portfolio health must be based on the new GitHub + TASKS.md tracking path, not missing legacy files
- selected project Cockpit/Tasks/Workflow/Audit/Logs tabs must remain usable
- no `.hiveai/PROJECT.json` errors may appear anywhere

## Project Cockpit expected behavior

For all 9 projects:

- opening the cockpit must render current GitHub/root TASKS.md state
- current milestone/task/progress must not become Unknown merely because `.hiveai` files do not exist
- no legacy source file is required
- truthful unavailable/network state is acceptable if GitHub itself is unavailable
- a legacy-file-not-found error is never acceptable

## Production-path requirement

Do not accept a fix that exists only in unit tests, fixtures, a new unused function, or an alternate code path.

Trace the exact native production path from application startup/background refresh through project registry/snapshot generation to Command Center and Project Cockpit.

The currently published executable is proof that some production path still invokes the old control-plane implementation. Find it and eliminate or bypass it correctly.

Inspect the current source for all remaining legacy symbols and paths related to the old tracker, including production-reachable uses of concepts such as:

- `PROJECT.json`
- `ProjectV3`
- `TasksV3`
- `RemoteRaw`
- `GITHUB_TRACKING_V3`
- `GITHUB_REMOTE_V3`
- `.hiveai/`
- old source-policy branches
- old persisted registry/source rows
- old migration/runtime fallback logic

Remove, replace or isolate them so none can affect production GitHub project tracking.

Historical test fixtures/documentation may retain old strings only when clearly non-production and necessary for history, but production behavior must have one unambiguous tracking architecture.

## Upgrade / persisted-state requirement

This repair must work for the owner's existing installed/dev application state, not only for a brand-new empty database.

On launch of the new build, previously persisted stale project/source/snapshot records from the old architecture must not keep the application stuck on legacy `PROJECT.json` errors, duplicate projects, or incorrect project identities.

The repaired app must converge existing user state to the exact 9-project GitHub + root TASKS.md portfolio automatically and safely.

Do not require the owner to manually delete databases, clear application data, edit SQLite, or recreate every project.

## Do not regress

Preserve:

- immediate startup video
- no visible Git/cmd/PowerShell/Terminal window flashing
- native Tauri desktop launch
- existing UI visual design
- standalone `Sekiph82/H-veAI` repository root
- hidden/bounded background network refresh
- manual Refresh behavior
- cache/stale remote behavior for temporary network failure

## Required native validation

Automated tests are not sufficient for closure.

Build and publish the actual native executable that the owner launches, then validate the production application path using an existing-state scenario representative of the owner's current installation.

Verify at minimum:

1. exactly 9 active projects appear;
2. both `AI-Commerce-HQ` and `H-veAI` are present as separate logical projects;
3. no duplicate project identities appear;
4. no screen contains `.hiveai/PROJECT.json` errors;
5. all 9 root `TASKS.md` files are read successfully when GitHub is reachable;
6. Project Cockpit shows real current task/milestone/progress values for repositories whose TASKS.md provides them;
7. Command Center task totals are populated from TASKS.md data;
8. no project becomes ERROR simply because legacy `.hiveai` files are absent;
9. startup video remains immediate;
10. no visible terminal windows appear;
11. the stable `dev-bin/H!veAI.exe` is replaced with the newly validated executable;
12. record the final executable SHA-256.

## Logging

Create a new immutable log at:

`docs/H!veAI/codex-logs/URGENT_PROJECT_JSON_RUNTIME_REMOVAL_LOG.md`

Include:

- exact root cause explaining why the native executable still requested `.hiveai/PROJECT.json` after the M21 migration;
- exact production call path responsible;
- legacy production code removed/replaced/isolated;
- persisted-state/upgrade repair performed;
- files changed;
- tests run;
- native validation results for all 9 repositories;
- final portfolio count;
- implementation commit SHA;
- stable EXE SHA-256;
- log commit SHA and final `origin/main` HEAD.

Do not declare PASS until the actual published native application no longer requests `.hiveai/PROJECT.json` and the owner-facing project tracking works from GitHub + root `TASKS.md` only.