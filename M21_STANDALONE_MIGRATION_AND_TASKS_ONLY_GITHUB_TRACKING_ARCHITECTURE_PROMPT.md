# H!veAI Standalone Migration + GitHub TASKS-Only Tracking Architecture Prompt

You are responsible for migrating H!veAI out of the legacy `Sekiph82/AI-Commerce-HQ` repository and establishing the new standalone `Sekiph82/H-veAI` repository as the permanent product repository.

This is not a narrow patch. Treat it as a controlled product migration plus architectural simplification.

The owner has clarified the product requirement and it supersedes the previous `.hiveai` control-plane architecture for GitHub project tracking.

Read this entire prompt before changing anything.

## 1. Final product definition

H!veAI is a desktop application for tracking GitHub software projects.

The product must answer, from one place:

- Which GitHub projects am I tracking?
- Which ones are active?
- What is the latest repository activity?
- How many tasks exist in each project?
- How many tasks are completed?
- How many tasks remain?
- What percentage of the project is complete?
- What milestone is active?
- What sprint is active, if the project uses sprints?
- What task is currently being worked on?
- What task or ordered tasks come next?
- What is the current task status?
- Who/which agent is expected to act next, if the project uses that concept?

The owner must be able to open H!veAI from any computer and see the same project state because the state comes from GitHub, not from one machine's local folders.

Core rule:

> GitHub repository = project source of truth.
>
> Root `TASKS.md` = project-management source of truth.

Do not use local workspace state as project truth.

## 2. Permanent standalone repository

New permanent repository:

`https://github.com/Sekiph82/H-veAI`

Legacy source currently lives under:

Repository: `https://github.com/Sekiph82/AI-Commerce-HQ`

Branch: `H!veAI`

Subtree: `H!veAI/`

Promote the H!veAI subtree to the root of `Sekiph82/H-veAI`.

After migration, the new repository must look and behave like a normal standalone application repository. It must not retain runtime/build assumptions that the application lives inside `AI-Commerce-HQ/H!veAI/...`.

Paths such as `H!veAI/src/...`, `H!veAI/src-tauri/...`, and `H!veAI/package.json` should become root-relative equivalents where appropriate.

Do not keep an unnecessary extra `H!veAI/` wrapper folder in the standalone repository.

## 3. Migration safety and preservation

Before moving files, inspect both repositories and record:

- legacy H!veAI branch HEAD;
- new H-veAI repository HEAD;
- current source tree;
- native app assets;
- build scripts;
- Tauri configuration;
- package files;
- Rust workspace/configuration;
- frontend configuration;
- tests;
- publishing scripts;
- Windows icon files;
- accepted opening video;
- documentation that is still relevant;
- current startup behavior;
- current stable executable behavior if available.

Preserve all working product source code and assets.

Preserve the accepted opening video bytes unless a change is technically unavoidable.

Preserve useful Git history where practical. If subtree history can be preserved cleanly, do so. If history filtering introduces unnecessary risk, document the decision and preserve migration provenance instead.

Do not retire the legacy repository until the standalone repository is independently verified.

## 4. Remove the old project-tracking architecture

The owner no longer wants the previous H!veAI project control-plane system for repository tracking.

The following concepts are superseded as authoritative project-tracking inputs:

- `.hiveai/PROJECT.json`
- `.hiveai/STATE.json`
- `.hiveai/HANDOFF.md`
- `.hiveai/EVENTS.jsonl`
- `.hiveai/PROJECT_DASHBOARD.md`
- `.hiveai/ACTIVE_CYCLES.md`
- `.hiveai/ARTIFACT_MAP.md`
- `.hiveai/PROGRESS_SNAPSHOT.md`
- manifest-based project truth
- local control-plane reconciliation
- local filesystem truth resolution
- local Git dirty/clean state as project health
- historical prompt files as task sources
- audit logs as task sources
- builder logs as task sources
- provider handoff files as current project truth
- broad Markdown/file discovery as project truth
- first-unchecked-task heuristics across arbitrary repository files

H!veAI may still have audit, agent, prompt, log, and workflow features as separate product features if useful. Those features must not be required to answer the simple question: `What is this GitHub project's current task state?`

For GitHub project tracking, remove or bypass unnecessary control-plane layers rather than continuing to repair them.

## 5. New project tracking architecture

For every tracked project H!veAI needs only two categories of information.

### A. GitHub repository metadata

Read directly from GitHub as appropriate:

- owner/repository name;
- repository URL;
- tracked branch;
- current remote HEAD SHA;
- latest commit SHA;
- latest commit message;
- latest commit timestamp;
- last push timestamp;
- repository language if useful;
- open issues if useful;
- other lightweight repository activity metadata that improves the dashboard.

### B. Root `TASKS.md`

This is the single project-management truth source.

H!veAI must not need any other repository file to calculate or display task state.

A project may contain CLAUDE.md, AGENTS.md, README files, architecture docs, audit docs, prompts, logs, or any number of other files. They are not task-tracking sources.

## 6. Canonical TASKS.md convention

Standardize all tracked repositories around one lightweight root-level `TASKS.md` convention.

The format must remain pleasant for humans to edit in GitHub while being reliably parseable by H!veAI.

Use a clear project status section near the top.

Recommended canonical shape:

```markdown
# Project Tasks

## Project Status

- Current Milestone: PAG-M07
- Current Sprint: PAG-M07
- Current Task: PAG-M07-C003
- Current Task Status: IN_PROGRESS
- Next Task: PAG-M07-C004
- Required Actor: CODEX

## Tasks

### PAG-M07

- [x] PAG-M07-C001 — ...
- [x] PAG-M07-C002 — ...
- [ ] PAG-M07-C003 — ... `IN_PROGRESS`
- [ ] PAG-M07-C004 — ...
- [ ] PAG-M07-C005 — ...
```

The exact formatting may be adjusted if existing project task structures require a richer representation, but all eight repositories must use the same parsing contract for fields H!veAI needs.

At minimum H!veAI must deterministically obtain:

- current milestone;
- current sprint or null;
- current task ID;
- current task title;
- current task status;
- next task or ordered next tasks;
- required actor or null;
- all actual task rows;
- completed task rows;
- open/incomplete task rows.

## 7. Task counting rules

Task metrics must be simple and transparent.

`Total Tasks = all recognized task rows`

`Completed Tasks = all recognized completed task rows`

`Remaining Tasks = Total Tasks - Completed Tasks`

`Completion Percentage = Completed Tasks / Total Tasks * 100`

Do not count headings, prose paragraphs, prompts, audit findings, logs, source files, milestone headings, explanatory bullets that are not tasks, or duplicate historical copies of the same task.

If some task rows should not contribute to project completion, define one simple explicit convention for excluding them.

Do not silently invent special cases per repository.

If exact completion cannot be calculated because a task file is structurally ambiguous, show `Unavailable` rather than a fabricated percentage and normalize that repository's TASKS.md during migration.

## 8. Current task rules

Do not infer current task by finding the first unchecked checkbox.

Current task is explicitly declared in the `Project Status` section.

That explicit declaration is authoritative.

The current task should also exist in the task list below. If the header points to a task absent from the list, surface a clear tracker inconsistency rather than selecting a historical task.

## 9. Next task rules

Next task must also be explicit or deterministically ordered.

Prefer `Next Task: <ID>`.

If a project legitimately has more than one ordered upcoming task, support a simple standardized repeated field or compact list.

Do not search arbitrary prose for words such as `next`, `after`, or `then` to invent next actions.

## 10. GitHub-first refresh behavior

H!veAI must behave like a direct GitHub dashboard.

At startup:

1. Show the accepted startup video immediately.
2. Create/show the usable application shell promptly.
3. Load last successful cached GitHub portfolio state if available.
4. Refresh repository state in the background.
5. Do not block startup on network requests.

For each tracked project:

1. resolve configured GitHub repository and branch;
2. check current remote HEAD;
3. if HEAD is unchanged, reuse cached parsed task state;
4. if HEAD changed, fetch new root `TASKS.md` and repository metadata;
5. parse it;
6. update the corresponding project view atomically.

Use caching and conditional requests where useful. Respect GitHub rate limits. Degrade gracefully when offline.

Do not use local `git fetch`, local filesystem watchers, or local worktree parsing as primary project-truth mechanisms when the GitHub API can provide the required data directly.

No visible command prompt, PowerShell, Git console, or terminal windows may appear during background refresh.

## 11. Offline/cache behavior

If GitHub is temporarily unavailable:

- keep last successful GitHub snapshot;
- display when it was last refreshed;
- mark it stale/offline clearly;
- do not replace it with local workspace state;
- do not erase known task information because one refresh failed.

## 12. Portfolio definition

After standalone migration, tracked portfolio is exactly:

1. `Sekiph82/H-veAI`
2. `Sekiph82/Bulk-Edit`
3. `Sekiph82/fmcg-erp-system`
4. `Sekiph82/FormuLab`
5. `Sekiph82/PackLab`
6. `Sekiph82/PackLab-3D`
7. `Sekiph82/Scrubbots`
8. `Sekiph82/ScrubBots-Level-Factory`

`Sekiph82/AI-Commerce-HQ` is temporary migration history only and must not remain in the final active portfolio.

One GitHub repository corresponds to one logical H!veAI project.

## 13. Normalize all eight TASKS.md files

As part of this migration, inspect the real current tracked branches of all eight repositories.

For each repository:

1. find its current detailed task tracker;
2. determine latest real project state;
3. preserve historical task information;
4. create or normalize root `TASKS.md` using the shared format;
5. move/merge real current task truth into the canonical status section;
6. make task rows parseable and countable;
7. preserve completed task history;
8. preserve future planned tasks;
9. remove H!veAI-specific duplicate trackers when no longer needed;
10. commit and push canonical `TASKS.md` to the actual tracked GitHub branch.

If an existing task tracker is lowercase `tasks.md` or elsewhere, migrate it carefully to canonical root `TASKS.md`, preserving contents and history where practical.

## 14. Provider behavior after migration

Claude, Codex, ChatGPT, or another agent working on a repository should follow one simple tracking rule.

Before work:

- read root `TASKS.md`;
- identify current task.

After work that changes task state:

- update root `TASKS.md`;
- mark completed tasks correctly;
- update current task;
- update next task;
- update milestone/sprint when applicable;
- commit and push those updates with implementation work.

No provider needs to update `.hiveai` state files for H!veAI project tracking.

CLAUDE.md and AGENTS.md may remain as development instruction files, but must not become competing project-status databases.

## 15. Remove old `.hiveai` project-tracking files from the eight repositories

Inspect every tracked repository for the old H!veAI control-plane system.

Remove obsolete H!veAI tracking files when they exist only for the superseded architecture, including as applicable:

- `.hiveai/PROJECT.json`
- `.hiveai/RULES.md`
- `.hiveai/TASKS.md`
- `.hiveai/STATE.json`
- `.hiveai/HANDOFF.md`
- `.hiveai/EVENTS.jsonl`
- `.hiveai/PROJECT_DASHBOARD.md`
- `.hiveai/ACTIVE_CYCLES.md`
- `.hiveai/ARTIFACT_MAP.md`
- `.hiveai/PROGRESS_SNAPSHOT.md`
- H!veAI-specific tracker manifests or generated control-plane files.

Do not blindly remove an entire `.hiveai` directory if it contains project-specific evidence valuable outside H!veAI. Inspect first.

Preserve historical audits/logs if useful, but H!veAI must not parse them for task state.

Remove old provider rules that force agents to maintain obsolete H!veAI tracking files.

## 16. H!veAI UI expectations

### Command Center

Must show portfolio-wide values derived from the eight GitHub repositories and root TASKS.md files:

- Projects = 8
- Active tasks
- Running/currently in-progress tasks
- Completed tasks
- overall/per-project progress where meaningful
- selected project current milestone/task
- latest GitHub activity

Do not derive counts from historical prompts/logs or local sources.

### Projects page

Exactly eight logical project cards.

Each card should show useful direct GitHub/project information such as repository, current milestone, current task, completion percentage, latest commit/push, and status.

Opening Cockpit must always produce a usable page.

### Project Cockpit

For selected repository show:

- repository name/link;
- branch;
- remote HEAD;
- latest commit message;
- last push;
- current milestone;
- sprint;
- current task;
- current task status;
- next task(s);
- total tasks;
- completed tasks;
- remaining tasks;
- completion percentage;
- required actor if used.

A valid GitHub repository and TASKS.md should not produce internal reconciliation jargon as the main user state.

### Tasks page

Tasks page should primarily be a useful task-status view, not a source-discovery/debug page.

Show at minimum:

- current task;
- current task status;
- next task(s);
- active/open tasks;
- completed tasks;
- total tasks;
- progress;
- milestone/sprint context.

If a technical source view remains, it should show the single canonical TASKS.md source rather than dozens of prompts/logs/documents.

## 17. Latest commit as supporting evidence

Display latest commit information because it is useful context.

A message such as `tracker: index PAG-M07-C002 fail and C003 active cycle` is useful supporting evidence.

However:

- latest commit message is supporting evidence;
- root TASKS.md remains authoritative for project-management state.

If latest commit and TASKS.md visibly disagree, show a small consistency warning rather than inventing a new task state.

## 18. Remove legacy H!veAI self-tracking assumptions

Standalone `Sekiph82/H-veAI` must itself follow the same simplified model.

Its project tracking source is root `TASKS.md`.

Do not recreate nested `H!veAI/TASKS.md` or a special `.hiveai` control plane for H!veAI itself.

It is just another GitHub repository in the eight-project portfolio.

## 19. Repository-root migration work

Promote application subtree to standalone root and repair every path assumption.

Inspect and adjust as necessary:

- package.json;
- lockfiles;
- frontend src paths;
- Vite config;
- TypeScript config;
- Tauri config;
- Rust Cargo manifests;
- native build scripts;
- icon paths;
- startup-video paths;
- dev-bin paths;
- tests;
- scripts;
- CI workflows;
- publishing/rollback scripts;
- documentation;
- developer instructions;
- hard-coded `H!veAI/` prefixes;
- hard-coded AI-Commerce-HQ dependencies;
- assumptions about parent-directory traversal.

Search the complete codebase for references to `AI-Commerce-HQ`, `H!veAI/`, old parent paths, and legacy nested locations.

Classify each as historical documentation, still-valid external reference, or migration-required runtime/build path. Change production/runtime assumptions, not immutable historical text merely for cosmetic cleanup.

## 20. Migration provenance

Create a permanent migration record in the new repository.

Record:

- legacy source repository;
- legacy branch;
- legacy source HEAD;
- method used to promote subtree;
- new standalone initial migration commit;
- files intentionally excluded;
- files transformed;
- old tracking architecture removed;
- eight TASKS.md normalization commits;
- build/test/publication evidence.

Create a durable pre-retirement reference/tag or equivalent provenance before the old parent repository is retired by the owner.

## 21. Testing strategy

Do not consider migration complete because files copied successfully.

### Standalone repository tests

Verify from new H-veAI root:

- dependency installation;
- frontend tests;
- frontend production build;
- Rust tests;
- Tauri build;
- Windows executable publication;
- icon resources;
- opening video;
- startup behavior;
- no dependency on old parent repository.

### GitHub tracking tests

For all eight actual repositories:

- repository can be fetched from GitHub;
- tracked branch resolves;
- root TASKS.md exists;
- TASKS.md parses;
- task counts are correct;
- completed counts are correct;
- percentage calculation is correct;
- current task is explicit;
- next task is explicit/ordered;
- latest commit metadata is fetched;
- project UI matches TASKS.md.

### Conflict tests

Prove:

- local old TASKS.md cannot override newer GitHub TASKS.md;
- old `.hiveai` files cannot affect current project display;
- prompt/log/audit files are not counted as sources/tasks;
- same repository cannot be registered twice as local+remote identities;
- no 16-project duplication regression.

### Startup tests

Prove:

- startup video begins promptly;
- GitHub refresh cannot delay first visible startup;
- no visible terminal windows appear;
- slow/offline GitHub does not make H!veAI Not Responding.

## 22. Native acceptance target

Before migration is ready for owner acceptance, published native application should satisfy:

1. startup video begins immediately;
2. application remains responsive;
3. no visible Git/cmd/PowerShell/Terminal windows;
4. exactly eight projects;
5. every project cockpit opens;
6. each project gets state from GitHub root TASKS.md;
7. Tasks page shows current/next/active/completed/progress information;
8. Command Center summary cards contain real values;
9. latest repository commit is visible as supporting information;
10. GitHub changes become visible after automatic refresh without local file changes;
11. temporary GitHub failure preserves cached remote state;
12. no obsolete `.hiveai` warning/reconciliation state appears for valid repositories.

## 23. AI-Commerce-HQ retirement readiness

Do not retire the legacy parent repository until complete standalone migration has been verified.

Before declaring it safe to retire, verify at minimum:

- all required H!veAI application files exist in H-veAI;
- all required assets exist;
- standalone build works;
- Windows executable works;
- startup video works;
- GitHub tracking works;
- eight-repository TASKS.md model works;
- H-veAI has no runtime/build dependency on AI-Commerce-HQ;
- migration provenance is recorded;
- no H!veAI-only code remains stranded in AI-Commerce-HQ that is absent from H-veAI.

Then produce a clear `AI-Commerce-HQ READY FOR OWNER RETIREMENT` report. Do not perform the final repository-removal operation as part of this implementation run.

## 24. Work order

Perform migration in this order:

1. Inspect current AI-Commerce-HQ/H!veAI subtree.
2. Inspect new H-veAI repository.
3. Record migration baseline.
4. Promote H!veAI subtree to H-veAI root.
5. Repair root-relative application/build paths.
6. Get standalone app building before architectural simplification.
7. Preserve startup/native behavior.
8. Replace old project tracking pipeline with direct GitHub + TASKS.md model.
9. Normalize H-veAI root TASKS.md.
10. Normalize other seven repository root TASKS.md files.
11. Remove/deprecate obsolete `.hiveai` tracking files/rules across all eight repositories.
12. Update provider instructions so future state-changing work updates root TASKS.md.
13. Test GitHub tracking against all eight actual repos.
14. Repair Command Center/Projects/Cockpit/Tasks UI to consume new simple model.
15. Run full tests/build/publication.
16. Perform native verification.
17. Record migration provenance and final SHAs.
18. Confirm H-veAI has no dependency on AI-Commerce-HQ.
19. Produce owner retirement-readiness report for AI-Commerce-HQ.

## 25. Avoid these failure patterns

Do not rebuild another complex control plane under different names.

Avoid:

- multiple copies of current task state;
- database state competing with GitHub task state;
- local files competing with GitHub task state;
- automatic broad repository Markdown discovery;
- inferred task state from arbitrary prose;
- one tracker file per subsystem;
- project-specific parser hacks;
- hidden compatibility fallbacks that resurrect old tasks;
- reconciliation as a normal user-facing state for valid repos;
- spawning Git CLI processes when a straightforward GitHub API request is sufficient;
- blocking application startup on network work;
- adding architecture merely to preserve obsolete architecture.

Prefer the smallest implementation that satisfies product behavior.

## 26. Required documentation

At H-veAI repository root create/update:

- `README.md` with standalone setup/build/run instructions;
- `TASKS.md` using standardized project tracker format;
- `ARCHITECTURE.md` describing simplified GitHub + TASKS.md model;
- `MIGRATION_FROM_AI_COMMERCE_HQ.md` recording migration provenance and retirement readiness;
- provider instruction files only if useful for development, with no competing task-state database.

Historical old H!veAI audit/prompt/log material may be archived under a clearly historical folder if worth preserving, but it must not become runtime project-state input.

## 27. Required final report

When implementation is complete, provide an evidence-based report containing:

### Standalone migration

- old repository/branch/HEAD;
- new repository/branch/HEAD;
- migration method;
- standalone build result;
- native EXE result;
- startup-video result.

### Tracking architecture

- old control-plane components removed/bypassed;
- new direct GitHub data flow;
- TASKS.md parser behavior;
- cache/refresh behavior.

### Eight-project matrix

For each project:

- repository;
- branch;
- TASKS.md path;
- current milestone;
- current task;
- next task;
- completed/total;
- percentage;
- latest commit SHA/message;
- normalization commit SHA.

### Old tracking cleanup

List which `.hiveai` and competing tracking files/rules were removed, deprecated, or retained only as history.

### Verification

- frontend tests;
- Rust tests;
- production build;
- native packaging;
- startup smoke;
- eight-repo GitHub read test;
- offline/cache test;
- duplicate-project regression test.

### Parent repository retirement readiness

End this section with either:

`AI-Commerce-HQ READY FOR OWNER RETIREMENT`

or

`AI-Commerce-HQ NOT YET SAFE TO RETIRE`

with precise reasons.

## Final acceptance principle

The migration succeeds when the product becomes simpler, not merely when old code has been moved.

Desired steady-state architecture:

```text
GitHub repositories
        │
        ├── repository metadata
        └── root TASKS.md
                │
                ▼
          H!veAI GitHub client
                │
                ├── cache
                ├── TASKS parser
                └── portfolio aggregation
                        │
                        ▼
       Command Center / Projects / Cockpit / Tasks
```

For project tracking there should be no required local truth layer and no required `.hiveai` control-plane layer between GitHub and the UI.

Keep the system direct, observable, and reliable.
