# M16P Addendum — Startup Responsiveness + Hidden Git Process Hotfix

## Purpose

This is an ADDENDUM to the already-running M16P remediation.

Do NOT restart or abandon the current M16P work.

Finish the currently executing M16P work first. Then immediately execute this addendum before finalizing the M16P builder log and before declaring M16P complete.

The owner has reproduced a native Windows regression:

- launching the desktop H!veAI executable no longer opens immediately;
- the expected startup video/app experience is interrupted;
- Windows shows `H!veAI (Not Responding)`;
- visible terminal windows repeatedly appear, showing Git command execution such as `C:\Program Files\Git\cmd\git...`;
- many child Git/terminal processes may appear during startup/refresh.

This behavior is unacceptable.

H!veAI must launch immediately and remain responsive while GitHub project tracking happens silently in the background.

---

# A. Hard product invariant

Launching H!veAI must never wait for network, Git, GitHub, repository scanning, portfolio refresh, or tracking reconciliation.

Required user-visible startup sequence:

1. user double-clicks H!veAI.exe;
2. process starts immediately;
3. accepted startup video experience remains intact;
4. main UI becomes responsive promptly;
5. cached/last-known GitHub project state may render first;
6. GitHub refresh begins asynchronously in background;
7. project cards update when fresh remote data arrives.

The app must never display Windows `Not Responding` during normal startup or background project polling.

---

# B. No visible child console/process windows

All Git/GitHub helper processes launched by H!veAI on Windows must be fully hidden.

No user-visible windows may appear for:

- git.exe
- cmd.exe
- powershell.exe
- pwsh.exe
- Windows Terminal
- conhost.exe windows attributable to spawned CLI tracking commands

If the current implementation uses `std::process::Command`, use the appropriate Windows creation flags / startup configuration so child processes run without creating visible console windows.

If shell wrapping is unnecessary, invoke the executable directly rather than through cmd/PowerShell.

Add a Windows-specific helper/abstraction for hidden background child processes so this rule is not reimplemented inconsistently.

Directly test/inspect every Git command path used by:

- app startup
- GitHub tracking refresh
- selected project polling
- portfolio polling
- manual Refresh
- Project Cockpit refresh
- Command Center refresh

No path may spawn a visible console window.

---

# C. Remove synchronous Git/network work from startup/UI paths

Audit the actual native startup call chain.

Identify every synchronous operation reachable before or during first render, including:

- remote fetch
- git fetch
- git show
- git rev-parse
- GitHub API calls
- repository enumeration
- v3 tracker parsing across all projects
- cache validation
- project reconciliation

None may block the UI/event thread.

Move all network/Git tracking work to bounded asynchronous/background execution.

The startup path may only:

- initialize lightweight local app state;
- load already-persisted cache/settings;
- initialize the startup video/UI;
- schedule background tracking work;
- return control to the event loop.

Do not call blocking process/network work while holding global application-state locks needed by frontend commands/events.

---

# D. Dedicated bounded background tracking worker

The GitHub tracking scheduler introduced by M16P must own remote refresh work.

Requirements:

- one bounded background manager/service;
- no refresh work on frontend/UI thread;
- no unbounded task spawning;
- no duplicate concurrent refresh for the same project;
- coalesce repeated refresh requests;
- selected-project and portfolio schedules share the same in-flight guard;
- each remote operation has a timeout;
- failure of one repository cannot stall the remaining portfolio;
- cancellation/shutdown is graceful;
- no global lock held while waiting for child process/network completion.

Per project maintain at minimum:

- refresh in-flight flag;
- last successful remote HEAD;
- last attempted refresh;
- next allowed refresh;
- timeout/error status.

If a second refresh request arrives while one is running, coalesce it instead of spawning another full command chain.

---

# E. Stop expensive full Git command chains on every poll

Do not run a heavy sequence such as multiple combinations of:

- git init
- remote remove/add
- git fetch
- git show
- rev-parse

for every 10-second / 30-second polling interval.

The normal polling fast path must be lightweight.

Preferred architecture:

1. observe remote branch HEAD using GitHub API or a lightweight remote-ref request;
2. compare with cached remote HEAD;
3. if unchanged, stop immediately;
4. only if changed, fetch the canonical GitHub v3 blobs for that resolved commit;
5. parse/persist snapshot;
6. emit update event.

Do not create temporary repositories or repeatedly reconfigure Git remotes during ordinary polling.

If GitHub API already provides the required remote contents, prefer it over shelling out to Git for primary project tracking.

Git CLI should not be the normal GitHub tracking transport when the authenticated GitHub API path is available.

---

# F. Startup cache behavior

Persist the last successful GitHub remote snapshots.

At application launch:

- load cache only from local app-owned H!veAI cache/database;
- render it immediately;
- label last GitHub sync time;
- schedule remote refresh asynchronously.

Do NOT walk all project working directories merely to construct the first Command Center screen.

Do NOT require all eight GitHub projects to finish remote refresh before the main window becomes usable.

Project refreshes may complete independently and update the UI incrementally.

---

# G. Preserve accepted startup experience

Do not remove or bypass the accepted H!veAI startup media flow.

Verify the current accepted startup asset remains:

`H!veAI/src/assets/H!veAI.mp4`

Do not restore `opening-video.mp4`.

Do not degrade startup by delaying video playback behind repository/network initialization.

The video/startup UI must not be coupled to remote tracking completion.

---

# H. Failure/offline behavior

Simulate:

- no internet;
- GitHub timeout;
- one repository hanging;
- authentication failure;
- DNS delay;
- Git process timeout where still used.

Expected behavior:

- H!veAI still opens normally;
- no visible terminal window;
- no Not Responding state caused by tracking;
- cached remote snapshot remains visible where available;
- affected project shows stale/offline status;
- other projects remain usable;
- retries use bounded backoff.

---

# I. Direct regression tests

Add direct tests proving all of the following:

1. startup does not await remote portfolio refresh;
2. startup returns/render state before remote request completion;
3. simulated 30-second GitHub delay does not block the main UI command path;
4. one hanging repository does not block other repositories;
5. duplicate refresh requests for one project are coalesced;
6. at most one remote refresh per project is in flight;
7. unchanged remote HEAD performs no full blob reload;
8. polling does not reinitialize temporary Git repositories;
9. Windows child-process helper uses no-window creation semantics;
10. no shell wrapper is used where direct Git invocation is sufficient;
11. startup video is initialized independently from GitHub refresh;
12. offline startup uses cache without local-project-state fallback;
13. app shutdown does not leave runaway Git/background processes;
14. manual Refresh remains asynchronous and responsive.

Where automated GUI verification cannot prove absence of visible child windows, provide source-level proof and perform an actual native Windows smoke check before final completion.

---

# J. Native Windows acceptance run

After implementation/publication, perform a real native smoke sequence if the environment allows it:

1. ensure no H!veAI process is running;
2. launch the published desktop H!veAI.exe normally;
3. verify startup video appears promptly;
4. verify main app becomes responsive without waiting for eight-project refresh;
5. observe Task Manager/process behavior;
6. verify no visible Git/cmd/PowerShell/Terminal windows appear;
7. leave app open through at least two polling cycles;
8. navigate Command Center -> Projects -> Project Cockpit;
9. trigger manual Refresh;
10. verify UI remains responsive throughout.

If native environment is unavailable, clearly mark this as PENDING OWNER NATIVE ACCEPTANCE. Do not claim it passed.

---

# K. Scope discipline

This addendum is part of M16P.

Do not:

- create a separate roadmap milestone;
- activate M17;
- start M21;
- rewrite unrelated project logic;
- change the GitHub-first authority decision;
- revert to local-first project truth.

The goal is to make GitHub-first tracking silent, asynchronous, bounded, and invisible to startup responsiveness.

---

# L. Builder log integration

Because the main M16P prompt is already executing, do not create a competing primary M16P log.

Add an explicit section to:

`H!veAI/docs/H!veAI/codex-logs/M16P_REMOTE_ONLY_PRIMARY_TRUTH_AND_LIVE_GITHUB_POLLING_CLOSURE_LOG.md`

with heading:

`## M16P Addendum — Startup responsiveness and hidden child-process remediation`

Record:

- root cause;
- synchronous startup/UI paths found;
- process-spawn paths found;
- Windows hidden-process implementation;
- async/background architecture;
- in-flight/coalescing behavior;
- timeout/backoff behavior;
- startup cache behavior;
- direct regression tests;
- native smoke result or explicit owner acceptance pending.

Do not finalize the M16P log until this addendum is complete.

Final M16P completion remains pending independent strict re-audit and owner native acceptance.
