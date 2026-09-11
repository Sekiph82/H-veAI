# Native Product Polish + Portfolio Behavior Remediation

This remediation combines the remaining strict-audit finding with new owner-native acceptance findings observed in the current published H!veAI application.

Work in `Sekiph82/H-veAI` on `main`.

Do not reopen or redesign the already-remediated `.hiveai/PROJECT.json` path unless a real regression is discovered. The intended project-tracking model remains GitHub repository metadata + root `TASKS.md`.

The current baseline portfolio contains 9 projects, including both `Sekiph82/AI-Commerce-HQ` and `Sekiph82/H-veAI` as separate projects.

## 1. Close the remaining AI-Commerce-HQ TASKS contract gap

The strict audit at:

`docs/H!veAI/audits/URGENT_PROJECT_JSON_RUNTIME_REMOVAL_STRICT_AUDIT.md`

identified that `Sekiph82/AI-Commerce-HQ@H!veAI` still uses an older root `TASKS.md` format that does not produce meaningful counts in the current root-TASKS parser.

Correct behavior:

- preserve the existing AI-Commerce-HQ task history;
- make its root `TASKS.md` compatible with the same GitHub + root `TASKS.md` model used by the other projects;
- if repository truth remains 20 completed tasks out of 20, H!veAI must show 20 total, 20 completed, 0 remaining/open, and 100% completion;
- do not invent an active task or future work that does not exist;
- completed/closed projects must be representable truthfully without fabricating a current task.

## 2. Project Cockpit completion percentage formatting

Owner-native evidence shows milestone/project progress such as:

`31.74807197943445%`

This is visually noisy and not acceptable as a user-facing percentage.

Correct behavior:

- user-facing completion percentages should display exactly two digits after the decimal point when a percentage is shown;
- example: `31.74807197943445%` must display as `31.75%`;
- use the same formatting consistently anywhere project completion/progress percentages are presented in the native UI;
- the underlying calculation may retain higher precision internally, but the visible presentation should not expose long floating-point values.

## 3. Command Center Running count is inconsistent with Tasks

Owner-native evidence shows ScrubBots with one current task in `IN_PROGRESS` state.

The Tasks page correctly shows:

- `Running = 1`

while Command Center simultaneously shows:

- `Running = 0`

This is contradictory product state.

Correct behavior:

- Command Center `Running` must be the portfolio-wide count of genuinely running/in-progress current work according to the authoritative GitHub/root-TASKS state;
- if ScrubBots is the only project currently `IN_PROGRESS`, Command Center must show `Running = 1`;
- if multiple projects are in progress, show the correct aggregate;
- the same state semantics must be used by Tasks, Project Cockpit, Workflow, and Command Center so one screen cannot say a task is running while another says there are zero running tasks;
- blocked/waiting/completed states must not be counted as running unless the authoritative task/workflow state actually represents active execution.

## 4. Command Center layout cleanup

The owner does not want the `Recent activity` section on Command Center.

Correct behavior:

- remove the `Recent activity` panel from Command Center;
- do not leave an empty placeholder or dead space where it used to be;
- use the reclaimed space to improve the usability of the operational panels on the right side;
- panels such as `System Status`, `Active Work Queue`, and the other queue/status panels in that stack must not appear vertically squeezed, clipped, or reduced to tiny unreadable cards;
- important rows, labels, counts and controls in those panels must be visible and usable;
- if content exceeds available space, use the product's normal internal scrolling/expansion behavior rather than compressing the panel into an unusable sliver;
- preserve the overall visual language and desktop layout of H!veAI.

## 5. Project removal must actually remove the project from H!veAI views

On the Projects page, the trash/delete control currently opens a confirmation dialog such as:

`Remove AI-Commerce-HQ from H!veAI registry? The folder will not be deleted.`

The owner expects this action to remove the project from H!veAI tracking, not merely hide one card while it continues to appear elsewhere.

Correct behavior:

- the confirmation must describe H!veAI registry/tracking behavior, not imply that a local folder is the primary project object;
- make it clear that removing a project from H!veAI does not delete the GitHub repository;
- after confirmation, the project must disappear from all active H!veAI project-facing surfaces, including Projects, Command Center, project shortcuts, Tasks project selection/context, Project Cockpit navigation, attention/queue panels, and other active portfolio summaries;
- project counts and portfolio KPIs must update accordingly;
- the deleted project must not automatically reappear on the next refresh or app restart merely because it exists in a built-in portfolio list;
- explicit owner removal must be persisted until the owner explicitly adds/registers that repository again;
- deleting a project from H!veAI must not delete, archive, modify, or otherwise damage the GitHub repository itself.

The current 9-project set is the initial/current baseline, not a rule that should override an explicit owner deletion.

## 6. Builder and Auditor assignment must be editable from Project Cockpit Settings

Projects cards currently show fields such as:

- `Builder Unassigned`
- `Auditor Unassigned`

The owner wants these assignments manageable from the selected project's Project Cockpit `Settings` tab.

Correct behavior:

- Project Cockpit > Settings must expose the selected project's Builder assignment;
- Project Cockpit > Settings must expose the selected project's Auditor assignment;
- the owner must be able to change each value there;
- changes must persist for that project;
- Projects cards and any other surfaces that display Builder/Auditor must reflect the updated values;
- changing Builder or Auditor for one project must not alter another project;
- an explicit Unassigned state must remain possible.

## 7. Preserve the now-working GitHub/root-TASKS model

Do not regress the working native fixes already demonstrated after PROJECT.json remediation.

Preserve:

- no `.hiveai/PROJECT.json` runtime errors;
- GitHub + root `TASKS.md` as project-management truth;
- 9-project current baseline before any explicit owner removal;
- both AI-Commerce-HQ and H-veAI as separate projects;
- immediate startup video;
- no visible Git/cmd/PowerShell/Terminal flashing;
- responsive background GitHub refresh;
- one normal root `TASKS.md` task source per GitHub-tracked project;
- Command Center tabs and Project Cockpit navigation already functioning.

## Native validation required

Do not close this task from automated tests alone. Build and publish the actual native executable used by the owner and validate the real production path.

At minimum verify:

1. AI-Commerce-HQ reports meaningful task counts and correct completion from root `TASKS.md` without invented active work.
2. Project Cockpit percentages are displayed to exactly two decimal places, e.g. `31.75%`.
3. A project with one `IN_PROGRESS` current task contributes exactly one to Command Center `Running`.
4. Tasks and Command Center agree on running/in-progress semantics.
5. `Recent activity` is absent from Command Center.
6. System Status, Active Work Queue, and adjacent operational panels are visibly usable and not squeezed/clipped.
7. Removing a project from Projects removes it from all active H!veAI surfaces and updates portfolio counts.
8. The removed project stays removed after refresh/restart until explicitly re-added.
9. Removing a project from H!veAI does not alter the GitHub repository.
10. Project Cockpit > Settings allows Builder and Auditor changes for the selected project.
11. Builder/Auditor changes persist and are reflected on Projects cards.
12. No `.hiveai/PROJECT.json` error reappears.
13. Startup video remains immediate and no visible terminal windows return.

## Logging

Create a new immutable log at:

`docs/H!veAI/codex-logs/NATIVE_PRODUCT_POLISH_AND_PORTFOLIO_BEHAVIOR_REMEDIATION_LOG.md`

Include:

- root cause for each issue above;
- files changed;
- tests performed;
- native validation evidence;
- AI-Commerce-HQ task normalization result;
- before/after Running aggregation behavior;
- confirmation of two-decimal progress formatting;
- Command Center layout result;
- project-removal persistence result;
- Builder/Auditor Settings result;
- regression confirmation for GitHub + root TASKS tracking;
- implementation commit SHA;
- stable EXE SHA-256;
- log commit SHA;
- final `origin/main` HEAD.

Do not declare PASS until the actual published native application exhibits the behavior above.