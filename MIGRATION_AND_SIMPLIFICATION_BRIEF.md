# H!veAI Standalone Migration and GitHub-Only Tracking Simplification

## Owner requirement

H!veAI is a GitHub project tracking application.

It does not use local project folders as project-truth sources.

For project tracking:

- GitHub repository is the only external source of truth.
- Each tracked project uses its repository task tracker as the project-management truth.
- H!veAI should read repository metadata plus the canonical task tracker and derive the dashboard from those values.
- The previous `.hiveai` control-plane architecture is superseded for project tracking.

## Standalone repository

Target repository:

`Sekiph82/H-veAI`

The current H!veAI application lives under the `H!veAI/` subtree of `Sekiph82/AI-Commerce-HQ` on branch `H!veAI`.

The application must be promoted to the root of this standalone repository while preserving working source code, assets, tests, build configuration, native packaging, and useful Git history where practical.

The standalone repository must build and run without depending on the old AI-Commerce-HQ repository layout.

## Simplified tracking model

H!veAI should track the configured GitHub repositories directly.

Primary project data:

1. GitHub repository metadata, including repository identity, tracked branch, HEAD/latest commit and recent activity as useful.
2. One canonical repository task file, standardized as `TASKS.md` at repository root.

Do not require `.hiveai/PROJECT.json`, `.hiveai/STATE.json`, `.hiveai/HANDOFF.md`, `.hiveai/EVENTS.jsonl`, dashboards, manifests, reconciliation documents, provider handoff files, audit prompts or logs to determine current project state.

Provider-specific files such as `CLAUDE.md` or `AGENTS.md` may remain when they contain useful development instructions, but they are not H!veAI project-state sources.

## TASKS.md information H!veAI must provide

From each repository's canonical `TASKS.md`, H!veAI must be able to show at minimum:

- total tasks;
- completed tasks;
- remaining/open tasks;
- completion percentage;
- current milestone;
- current sprint if applicable;
- current task;
- current task status;
- next planned task or ordered next planned tasks;
- required actor if the project uses that concept.

Task completion percentage is derived from the canonical task list and must not be invented from unrelated files.

A consistent lightweight header/section convention may be introduced into each `TASKS.md` where necessary, but `TASKS.md` remains the single project-management truth source.

## GitHub behavior

H!veAI should behave more like a direct GitHub dashboard:

- fetch the configured repositories from GitHub;
- fetch the tracked branch HEAD;
- fetch/parse `TASKS.md`;
- display latest commit information;
- refresh automatically in the background;
- when HEAD changes, refresh the affected repository/task data;
- allow manual refresh;
- cache the last successful remote state for temporary GitHub/network failures;
- never silently replace remote truth with local workspace files.

The product must remain responsive. GitHub refresh must not delay the accepted startup video or open visible terminal windows.

## Portfolio

The current tracked portfolio is:

1. `Sekiph82/H-veAI`
2. `Sekiph82/Bulk-Edit`
3. `Sekiph82/fmcg-erp-system`
4. `Sekiph82/FormuLab`
5. `Sekiph82/PackLab`
6. `Sekiph82/PackLab-3D`
7. `Sekiph82/Scrubbots`
8. `Sekiph82/ScrubBots-Level-Factory`

`Sekiph82/AI-Commerce-HQ` is temporary only during migration and is not intended to remain after standalone migration is fully verified.

## Migration safety

Do not delete `Sekiph82/AI-Commerce-HQ` until all of the following have been independently verified:

- the entire H!veAI application exists in `Sekiph82/H-veAI`;
- the new repository root builds successfully;
- native Windows packaging works;
- the accepted startup video still starts promptly;
- the application launches correctly;
- the 8-project portfolio works from GitHub;
- task tracking reads the canonical repository `TASKS.md` files;
- no required H!veAI source/assets/history were lost in the move;
- the old parent repository is no longer needed by H!veAI.

Repository deletion is the final irreversible operation and must happen only after this validation.
