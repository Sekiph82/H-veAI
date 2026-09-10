# Project Cockpit + Source Scope + Command Center + Tasks Native Fix — Behavior-Only Prompt

Inspect the current H!veAI implementation and fix only the owner-observed native problems below.

Do not follow a prescribed architecture from this prompt. Read the current production code, reproduce the behavior, identify the real root causes yourself, and choose the simplest reliable fix.

Do not redesign unrelated parts of H!veAI.
Do not activate M17.
Do not start M21.

## What is now working and must NOT regress

The following owner-native behaviors are currently correct and must remain correct:

- H!veAI startup video begins immediately after launch with no long delay.
- H!veAI no longer visibly opens repeated Git/cmd/PowerShell/Terminal windows.
- The project portfolio is back to exactly 8 projects.
- The Projects page shows exactly 8 logical projects, with no duplicate `Path missing` copies.

Preserve all of this.

---

# Problem 1 — Open cockpit leads to a blank Project Cockpit page

## Observed behavior

The Projects page correctly shows 8 projects.

However, pressing `Open cockpit` for a project can navigate to `Workspace / Project Cockpit` and show only the page background/header/sidebar, with no actual project cockpit content rendered.

This means the user has a valid project card and a valid navigation action, but the destination page is effectively blank.

This must be fixed for all 8 projects, not just one fixture or one repository.

## Expected behavior

For every one of the 8 registered projects:

- `Open cockpit` must always open a usable Project Cockpit page.
- The selected project identity must be preserved correctly during navigation.
- The cockpit must render its project header and the available GitHub-tracked project information.
- A transient loading state is acceptable while data is being fetched.
- A truthful error/unavailable state is acceptable if GitHub cannot be reached.
- A completely blank cockpit page is never acceptable.
- The user must never need to return to Projects and retry because the selected project context was lost.
- All 8 project cards must be tested through the same production routing/state path used by the native application.

Do not paper over this by adding a generic static page. Fix the real navigation/state/data-loading failure causing the blank cockpit.

---

# Problem 2 — H!veAI is treating too many repository files as project “sources”

## Observed behavior

For `ScrubBots - Pixel Art Generator`, the Tasks / Task Sources screen reports `56 available` sources.

The source inventory is clearly too broad. H!veAI is discovering files such as implementation prompts, audit prompts, historical documentation, logs, and other repository material as though they were normal task/project tracking sources.

The owner expects only the small set of files that actually define the live tracked project state to be treated as primary project sources.

## Expected behavior

The normal source inventory for a GitHub-tracked project should contain only the small bounded set of files that H!veAI actually uses to understand current project state, normally around 5–6 files depending on the project contract.

The exact filenames should be determined from the current H!veAI tracking contract and repository conventions, not guessed from this prompt.

Conceptually, primary/live tracking sources are things such as:

- project identity / project contract
- canonical task tracker
- current state / workflow state
- handoff / next-action state
- project rules or control-plane rules when they are part of the live contract
- event/history file when it is part of the live tracking contract

Files that are merely supporting artifacts must NOT appear as ordinary live project sources. Examples include:

- generated implementation prompts
- remediation prompts
- audit prompts
- Codex logs
- builder logs
- independent audit reports
- historical documentation
- roadmap/archive/reference documents
- arbitrary Markdown files discovered simply because they exist in the repository

These supporting files may remain accessible elsewhere in H!veAI if useful, but they must not inflate the normal project source count or be treated as canonical current-state inputs.

## Correct source behavior

- Source discovery must be contract-driven and bounded, not broad filesystem/repository crawling.
- GitHub-tracked projects should expose only the files that are authoritative for live project tracking as the normal source set.
- Supporting prompts/logs/audits/docs must be clearly secondary evidence or history, not project-state sources.
- `Rescan sources` must not re-add every prompt/log/document in the repository.
- The source count should remain stable and understandable after repeated rescans.
- The same source-selection rules must apply consistently across all 8 projects.
- Do not create project-specific hacks just to make one repository show a smaller number.

---

# Problem 3 — Command Center project tabs are visible but do not behave like working tabs

## Observed behavior

Inside the Command Center, the selected project panel shows these controls:

- `Cockpit`
- `Tasks`
- `Workflow`
- `Audit`
- `Logs`

They visually look like tabs, but clicking them does not reliably replace the content inside the Command Center project panel with the corresponding project information.

The controls therefore look interactive without providing the expected navigation/content-switching behavior.

## Expected behavior

These five controls must behave as real tabs for the currently selected project.

When the user clicks a tab, the main project-detail area inside the Command Center must update in place to show that tab's relevant content for the same selected project.

Expected meaning:

- `Cockpit` shows the project's current high-level live status, current work, workflow summary, next action, health/progress and other cockpit-level information.
- `Tasks` shows the relevant task information for that project.
- `Workflow` shows the project's current workflow/state information.
- `Audit` shows the relevant audit status/history/findings available for that project.
- `Logs` shows the relevant project/session/activity/log information intended for that tab.

The exact components, data sources and implementation are for you to determine from the current codebase.

The important product behavior is:

- clicking a tab must visibly change the content in the Command Center project panel;
- the selected tab must have a clear active state;
- switching tabs must not change the selected project;
- switching projects must keep the tab system functional for the newly selected project;
- unavailable data should show a truthful empty/unavailable state, not a dead control or blank panel;
- these controls must not require opening a separate page merely to appear functional unless that behavior is already explicitly intended by the current product design;
- all five tabs must work consistently for all 8 registered projects.

Do not merely make the labels clickable. Restore the actual user-facing behavior implied by the tab interface.

---

# Problem 4 — Command Center task summary cards are not showing their real values

## Observed behavior

At the top of Command Center, the portfolio summary cards include:

- `Active tasks`
- `Running`
- `Completed tasks`

These cards currently show no meaningful task counts even though the tracked projects have task data and H!veAI is supposed to provide a live portfolio overview.

The cards therefore exist visually but are not delivering the portfolio information they are meant to summarize.

## Expected behavior

The Command Center summary cards must display truthful, current portfolio-wide values derived from the tracked GitHub project state.

Expected meaning:

- `Active tasks` shows the current number of active/open tasks across the 8 tracked projects according to the authoritative tracker state.
- `Completed tasks` shows the current number of completed tasks across the 8 tracked projects according to the authoritative tracker state.
- `Running` shows the current number of tasks/workflows that are actually in a running/in-progress execution state according to the live project/workflow state.

The exact calculation and data path are for you to determine from the current production code and tracker contract.

The important product behavior is:

- these cards must no longer stay blank or show placeholder values when authoritative data is available;
- values must be calculated consistently across all 8 projects;
- the same underlying project truth used by Project Cockpit / Tasks / Workflow should drive the Command Center summary, so the numbers do not contradict project-level screens;
- values must update when the tracked GitHub project state changes;
- if a value genuinely cannot be determined, the UI must show a truthful unavailable state rather than silently presenting an incorrect number;
- do not count prompts, logs, audits, documentation files or other non-task artifacts as tasks;
- do not inflate counts by counting the same logical task more than once;
- zero is valid only when the real authoritative state is actually zero.

The result should make the top of Command Center a real live portfolio summary rather than decorative cards with missing values.

---

# Problem 5 — Tasks page is exposing legacy/internal tracking concepts and is missing the actual task-state dashboard the user needs

## Observed behavior

On the Tasks page for `ScrubBots - Pixel Art Generator`, the `Project Intelligence / Dashboard Contract` section exposes a field labeled `M09 REFRESH` with value `CURRENT`.

This is confusing and looks like an old internal milestone-era implementation detail rather than useful current project information. The product is now intended to be GitHub-first and to show the actual current state of the selected project, not historical internal labels that the user must understand.

At the same time, the Tasks page is mostly a source-inventory screen. It does not prominently show the task information the user actually expects from a page called `Tasks`, such as:

- active/open tasks
- current task
- next task / next action
- running/in-progress task when applicable
- completed tasks
- task counts/progress for the selected project

This makes the Tasks page technically descriptive of source files but weak as an operational project-task view.

## Expected behavior

The Tasks page should primarily answer: **What is this project's task status right now?**

For the currently selected project, it should clearly expose the authoritative GitHub-tracked task state, including at minimum the meaningful equivalents of:

- current task
- next task / next action
- active/open tasks
- running/in-progress work when applicable
- completed tasks
- useful task totals/progress when those values are available from authoritative task data

The exact layout, controls, naming, data model and implementation are for you to determine from the existing product and tracker contract.

The important product behavior is:

- the Tasks page must be a real task-status view, not merely a repository source-discovery page;
- task information must come from the same authoritative GitHub project/task truth used elsewhere in H!veAI;
- current task, next task, active/open task counts and completed task counts must agree with Command Center and Project Cockpit;
- when a task changes in the tracked GitHub repository, the Tasks page must reflect that updated state through the normal H!veAI refresh/polling behavior;
- when data is unavailable, show a truthful unavailable state rather than invented task information;
- source inventory can remain available as supporting/advanced information, but it must not dominate the user-facing purpose of the Tasks page;
- internal legacy implementation labels such as `M09 REFRESH` should not be exposed as primary user-facing status unless they still represent a real current product concept that the owner needs to act on;
- if such a legacy/internal label is no longer operationally meaningful, the user-facing Tasks page should instead present the actual current GitHub tracking/refresh status in understandable product language;
- do not replace one internal milestone label with another opaque engineering label.

The user should be able to open `Tasks`, select a project, and immediately understand what is active, what is being worked on, what comes next, and what has already been completed.

---

# Required validation

Before returning, verify all five problems together in the native/product path:

1. Projects page still shows exactly 8 projects.
2. Open cockpit works for all 8 project cards.
3. No project opens to a blank page.
4. Startup video remains immediate.
5. No visible Git/cmd/PowerShell/Terminal windows return.
6. Source discovery for each GitHub-tracked project is bounded to the actual live tracking contract.
7. Prompt files, audit files, implementation logs, and general documentation are no longer counted as ordinary live project sources.
8. Repeated source rescans remain stable.
9. Existing GitHub-first project tracking behavior is preserved.
10. Command Center `Cockpit`, `Tasks`, `Workflow`, `Audit`, and `Logs` controls all work as real tabs.
11. Clicking each tab replaces the project-detail content inside Command Center with the expected information for the currently selected project.
12. Tab switching works consistently across all 8 projects.
13. Missing/unavailable information is represented truthfully instead of leaving a dead or blank tab.
14. `Active tasks` shows the real authoritative portfolio-wide active/open task count.
15. `Completed tasks` shows the real authoritative portfolio-wide completed task count.
16. `Running` shows the real current running/in-progress work count.
17. The Command Center summary values remain consistent with the project-level task/workflow data.
18. The summary values refresh when the tracked GitHub project state changes.
19. The Tasks page clearly exposes the selected project's current task state instead of functioning mainly as a source inventory screen.
20. Current task, next task/action, active/open tasks, running/in-progress state where applicable, completed tasks and useful counts/progress are visible or truthfully unavailable.
21. Tasks-page task values agree with Command Center and Project Cockpit for the same project.
22. Legacy/internal labels such as `M09 REFRESH` are not presented as primary user-facing project status unless they remain genuinely operationally meaningful.
23. Source inventory remains accessible as supporting information without overwhelming the actual task view.
24. The Tasks page updates when authoritative GitHub task state changes.

---

# Logging requirement

Before finishing, ensure the previous native regression repair work also has its implementation log committed and pushed to the `H!veAI` branch. If a previous repair log already exists, preserve it and report its exact GitHub path and commit. Do not overwrite historical logs.

Then create and commit a new immutable log for this work at:

`H!veAI/docs/H!veAI/codex-logs/PROJECT_COCKPIT_SOURCE_SCOPE_COMMAND_CENTER_AND_TASKS_NATIVE_FIX_LOG.md`

The new log should be concise and factual. Include:

- root cause of the blank Project Cockpit problem;
- root cause of the over-broad source discovery problem;
- root cause of the non-functional Command Center project tabs;
- root cause of the missing Command Center Active/Completed/Running summary values;
- root cause of the Tasks page exposing legacy/internal tracking state while omitting the useful operational task dashboard;
- files changed;
- tests performed;
- native/product validation performed;
- confirmation that the 8-project portfolio, immediate startup video, and hidden background Git behavior did not regress;
- previous repair log path/commit;
- current implementation commit;
- current log commit/final branch HEAD.

Do not claim success from test names alone. Return after the real product behavior above is working.