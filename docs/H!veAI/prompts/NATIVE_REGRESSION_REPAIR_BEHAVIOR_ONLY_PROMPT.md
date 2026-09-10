# Native Regression Repair — Behavior-Only Prompt

Inspect the current H!veAI implementation and fix the following owner-observed native regressions.

Do not assume a specific implementation strategy from this prompt. Diagnose the real root causes yourself from the current production code, database/registry state, startup flow, GitHub tracking flow, and UI behavior.

## Observed problems

### 1. Duplicate projects

H!veAI should track exactly 8 projects, but the app showed 16.

Each real project appeared twice. One copy was the normal active project, while the duplicate copy often showed `Path missing`, `Non-Git folder`, or `Not detected`. Opening those duplicate project cockpits resulted in an empty or unusable page.

### Expected behavior

- There must be exactly 8 logical projects in H!veAI.
- Each GitHub repository must appear only once everywhere: Command Center, Projects, Project shortcuts, and Project Cockpit.
- Local path information must not create a second logical project.
- Existing duplicate records must be repaired, not merely hidden in the frontend.
- Opening any of the 8 project cards must open its working cockpit.

### 2. GitHub project tracking was not producing current project state

Example owner evidence showed `ScrubBots - Pixel Art Generator` stuck with remote refresh pending, Remote HEAD unavailable, and milestone, sprint, task, workflow, next action, and progress all Unknown even though the tracked GitHub repository contained the project tracker files and active work.

### Expected behavior

- GitHub is the authoritative source for tracked project state.
- H!veAI should automatically read the tracked GitHub repository and display the latest milestone, sprint, current task, current task title, next action, workflow state, required actor, blockers/waits, progress, last completed work, remote branch, remote HEAD, and health/state.
- These values must update automatically when the GitHub project changes.
- The user should not need to restart H!veAI.
- Local tracker files must not override newer GitHub state.
- A project must not remain permanently stuck in `pending` or `Unknown` while GitHub is reachable.
- Command Center and Project Cockpit must show consistent data for the same project.

### 3. Startup had regressed

Previously, double-clicking `H!veAI.exe` caused the startup video to appear almost immediately and the app opened normally. The regression caused a long blank/dark delay and the startup video started roughly 15–20 seconds later.

### Expected behavior

- Startup video should begin promptly after launching H!veAI.
- GitHub refreshes, project synchronization, registry checks, Git operations, polling, or tracker parsing must not visibly delay startup.
- H!veAI must stay responsive during startup.
- No Git, cmd, PowerShell, or Terminal windows should appear.
- Preserve the existing accepted startup video and normal startup experience.

## Product behavior to preserve

Return H!veAI to its intended behavior:

- exactly 8 tracked GitHub projects;
- one identity per project;
- working cockpit for every project;
- current project state taken from GitHub;
- automatic background updates when GitHub changes;
- no dependency on local tracker files for project truth;
- responsive application;
- prompt startup video;
- no duplicate/path-missing ghost projects;
- no permanent Unknown/pending state when the remote repository is available.

## Working rules

- Investigate the current code before modifying it.
- Find the real root causes.
- Do not preserve a newer architecture merely because it was introduced recently if it is the cause of these regressions.
- Prefer restoring simple, reliable product behavior over adding more abstraction.
- Do not make unrelated changes.
- Do not activate M17 or start M21.
- Complete all of the above problems in one implementation cycle before returning.
- Provide a concise implementation log explaining the root cause of each problem, what changed, what native behavior proves the fix, test results, and the final published executable.