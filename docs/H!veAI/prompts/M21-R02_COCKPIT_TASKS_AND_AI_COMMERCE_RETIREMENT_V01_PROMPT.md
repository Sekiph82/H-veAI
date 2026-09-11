# M21-R02 Project Cockpit Tasks and AI-Commerce Retirement Remediation V01

## MANDATORY SYNC-FIRST AND GITHUB-ONLY COMPLETION CONTRACT

Before reading or modifying any H!veAI work item, operate from the standalone `Sekiph82/H-veAI` repository and safely synchronize the local checkout with `origin/main`.

Run:

```powershell
git fetch origin main
git status --short
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the working tree is clean and local HEAD is only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never use reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or any operation that discards dirty/divergent owner work. If safe fast-forward synchronization is impossible, stop and report `SYNC_BLOCKED` with the exact reason.

After synchronization, read the following GitHub-authoritative files from the updated checkout before implementation:

- `AGENTS.md`
- `TASKS.md`
- `docs/H!veAI/audits/M21-R02_OWNER_NATIVE_ACCEPTANCE_AND_RETIREMENT_GATE_V01_AUDIT.md`
- `docs/H!veAI/audits/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V03_AUDIT.md`
- relevant current source/tests for Project Cockpit and GitHub tracking

Work only in `Sekiph82/H-veAI`. Do not inspect or modify another GitHub repository as part of this work item.

All H!veAI repository changes must be committed and pushed before completion is claimed. At the end run:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == remote refs/heads/main` and a clean H!veAI working tree. Otherwise return `SYNC_BLOCKED`, not `COMPLETE`.

The final owner-facing response must show only the relevant GitHub file URLs/paths, implementation/log commit SHA(s), final GitHub `main` SHA, and concise status. Do not dump ordinary local changed-file paths.

## WORK ITEM

- Work code: `M21-R02`
- Version: `V01`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V01_LOG.md`
- Originating strict audit: `docs/H!veAI/audits/M21-R02_OWNER_NATIVE_ACCEPTANCE_AND_RETIREMENT_GATE_V01_AUDIT.md`

## OWNER NATIVE ACCEPTANCE EVIDENCE

The owner completed the M21 native acceptance and reported the following results:

- native H!veAI launch: PASS;
- startup video immediate: PASS;
- no visible terminal windows: PASS;
- exactly eight visible projects: PASS;
- Command Center task data: PASS;
- Project Cockpit Overview task data: PASS;
- general visual integrity: PASS;
- Project Cockpit `Tasks` tab: FAIL.

For the same ScrubBots project, Command Center and Cockpit Overview correctly display GitHub-root-`TASKS.md` truth, including current task, milestone/sprint, workflow/actor, next action, counts/progress and remote HEAD. Cockpit `Tasks` instead displays `Task intelligence unavailable`, `No parsed tasks`, and `Unknown` current/next values.

Treat that runtime evidence as a real acceptance failure. Do not dismiss it because automated tests are green.

## FINDING M21-R02-F01 — MAJOR — PROJECT COCKPIT TASKS DOES NOT RENDER CANONICAL REMOTE TASK TRUTH

### Confirmed current source behavior

Inspect at minimum:

- `src-tauri/src/project_cockpit.rs`
- `src/projectCockpit.ts`
- the Project Cockpit UI rendering in `src/pages.tsx` and/or the actual current component that owns the Tasks tab
- `src-tauri/src/github_tracking.rs`
- `src/commandCenter.ts`
- existing focused Project Cockpit / GitHub tracking tests

Current `src-tauri/src/project_cockpit.rs` routes GitHub-tasks-only projects into `snapshot_remote_primary(...)`. That function currently initializes legacy/persisted task-intelligence as absent and the workflow task list as empty while separately providing populated remote-primary GitHub truth. This explains why Overview can be correct while Tasks is empty.

### Required target behavior

For every project whose canonical authority is GitHub metadata plus root `TASKS.md`:

1. Project Cockpit Overview and Project Cockpit Tasks must use one consistent remote observation for the same refresh/remote HEAD.
2. The Tasks tab must render the canonical task rows from GitHub root `TASKS.md`; it must not require legacy local/persisted task-intelligence rows in order to display canonical remote tasks.
3. Current task, next action/task, milestone/sprint, required actor, workflow state, completed/total counts and progress shown across Command Center, Cockpit Overview and Cockpit Tasks must be semantically consistent for the same remote HEAD.
4. A valid populated remote root `TASKS.md` must never produce `Task intelligence unavailable`, `No parsed tasks`, or `Unknown` merely because the legacy persisted task-intelligence path is empty.
5. If remote root `TASKS.md` genuinely contains no task rows, show a truthful canonical empty state.
6. If the remote observation is stale/unavailable and a durable last-known remote snapshot exists under current architecture, use the existing bounded stale behavior and label it truthfully. Do not silently substitute local filesystem task truth.
7. Do not revive `.hiveai/TASKS.md`, `PROJECT.json`, `RULES.md`, `EVENTS.jsonl`, or another legacy control-plane source as primary authority.

### Implementation constraints

Do not solve this by hard-coding ScrubBots values, fabricating UI-only task rows, or copying summary strings into a fake task list.

Trace the actual root-`TASKS.md` parser and remote observation model. Extend or normalize the remote snapshot contract so the canonical task rows needed by Cockpit Tasks are available through a real production data path. Prefer one normalized remote task-row model shared/reused across Command Center and Cockpit surfaces where practical.

Preserve bounded payload sizes and deterministic ordering.

Do not make the Tasks tab depend on the selected project's local filesystem path for canonical project truth.

## FINDING M21-R02-F02 — MAJOR — AI-COMMERCE-HQ REMAINS A DEFAULT PORTFOLIO TARGET

### Confirmed current source behavior

`src-tauri/src/github_tracking.rs::ensure_portfolio(...)` currently defines a nine-entry target array that still includes:

- repository `Sekiph82/AI-Commerce-HQ`
- historical branch `H!veAI`

The owner's current UI correctly shows eight projects because an explicit exclusion record prevents that target from appearing in the current database. That is not sufficient for permanent repository retirement: a fresh database without the exclusion can recreate/attempt to observe the obsolete target.

### Required target behavior

The permanent default H!veAI portfolio must consist only of the intended current eight repositories and must not include `Sekiph82/AI-Commerce-HQ` as a bootstrap/default target.

Remove the obsolete AI-Commerce target and any special-case branch/bootstrap logic that exists only to support that target, provided it is no longer required for historical database compatibility.

Preserve migration safety for existing user state:

- do not resurrect AI-Commerce;
- do not delete or corrupt unrelated project records;
- do not remove the current eight projects;
- do not make a fresh database depend on a pre-existing exclusion row to remain at eight projects;
- historical exclusion rows may remain harmlessly if schema/data retention requires them, but they must not be required to suppress a default AI-Commerce target that no longer exists.

Do not delete the actual GitHub `Sekiph82/AI-Commerce-HQ` repository during this work item.

## FINDING M21-R02-F03 — BLOCKER FOR LOCAL PARENT DELETION — ACTIVE CHECKOUT RELOCATION IS NOT PROVEN

This implementation work item must not delete or relocate the historical local parent tree.

The last independently audited active H!veAI checkout path was physically inside the historical AI-Commerce parent tree. The owner wants that old local tree deleted, but deletion is not safe until the active standalone H!veAI checkout is proven at a final path outside the parent and the Desktop shortcut/runtime are validated from that new path.

For M21-R02:

- do not delete the local AI-Commerce parent;
- do not delete any preservation tree;
- do not move unrelated repositories;
- do not delete the GitHub AI-Commerce repository;
- do not claim `SAFE_TO_DELETE_PARENT_DIRECTORY`.

After M21-R02 passes independent audit and owner re-acceptance, a separate bounded retirement/relocation gate may authorize the destructive cleanup.

## TRACKER TRUTH

The owner acceptance failure means current `TASKS.md` project status must no longer say the only next action is human acceptance/retirement while this production defect is open.

Update current operational truth to make `M21-R02` the active remediation with `Required Actor: CODEX` during implementation. Preserve historical M21/M21-R01 closure truth, but record that owner acceptance exposed M21-R02 as a newly reopened production defect.

Do not change the user-facing 20-milestone denominator.

When implementation is complete, do not mark final owner acceptance as PASS. The correct post-builder state is implementation complete / awaiting independent audit and owner native re-acceptance.

## REQUIRED FOCUSED TESTS

Add or update tests that exercise production paths, not mocks that bypass the defect.

At minimum prove:

1. **Remote populated, legacy empty:** a GitHub-tasks-only project with populated root `TASKS.md` remote truth and empty/null legacy task intelligence still exposes canonical task rows to Cockpit Tasks.
2. **Cross-surface parity:** for one remote HEAD, Command Center, Cockpit Overview and Cockpit Tasks agree on current task identity, completed/total counts, progress, required actor and next action where those fields are represented.
3. **Canonical empty state:** a valid remote `TASKS.md` with no parseable task rows produces a truthful empty canonical state rather than a legacy-intelligence error.
4. **Remote failure/stale behavior:** existing bounded stale/unavailable semantics remain truthful and do not silently switch to local filesystem authority.
5. **Fresh portfolio bootstrap:** initializing portfolio state without any existing exclusion row yields exactly the intended eight default repositories and never creates `Sekiph82/AI-Commerce-HQ`.
6. **Existing state safety:** existing exclusion/history data does not remove or duplicate unrelated current projects.
7. **Regression:** current eight-project Command Center behavior remains intact.

Also run relevant existing frontend/Rust Project Cockpit, Command Center, GitHub tracking and root-TASKS focused suites. Run `git diff --check`.

Do not manufacture unrelated large regression work. Run the minimum existing broader suite needed to establish confidence after the touched source set.

## NATIVE PUBLICATION AND OWNER RE-TEST READINESS

After tests pass, publish the latest validated native QA executable using the repository's existing safe development publication process. Keep the stable Desktop H!veAI shortcut model intact. Do not point the shortcut at scripts, npm, cargo, PowerShell, cmd, or a browser.

The builder may perform automated native smoke checks, but it must not fabricate owner visual acceptance.

The required owner re-test after this work is:

- open Command Center and confirm eight projects;
- select ScrubBots and verify current task/count/progress/next action;
- open ScrubBots Cockpit Overview and verify the same remote truth;
- open ScrubBots Cockpit Tasks and verify real canonical tasks now appear instead of `No parsed tasks` / `Unknown`;
- verify no terminal flashes and no obvious UI regression.

## PROHIBITED SHORTCUTS

Do not:

- hard-code ScrubBots task data;
- display Overview summary fields as fake task rows;
- use local project files as the primary truth for GitHub-tracked projects;
- revive legacy `.hiveai` tracking architecture;
- delete the AI-Commerce GitHub repository;
- delete the historical local parent tree;
- delete preservation copies;
- modify another GitHub repository;
- rewrite historical prompts/logs/audits;
- reset, rebase, force-push, clean, auto-stash, or discard owner work;
- mark owner acceptance PASS without a new owner observation.

## REQUIRED VALIDATION

Before publishing the log, verify at minimum:

- focused F01 tests PASS;
- focused F02 fresh/eight-portfolio tests PASS;
- relevant Project Cockpit / Command Center / GitHub tracking regressions PASS;
- frontend typecheck/build required by touched code PASS;
- relevant Rust tests/build required by touched code PASS;
- `git diff --check` PASS;
- no unrelated repository source was added;
- no local database, secret, cache, build output, preserved tree, or machine-specific private artifact was committed;
- historical audit/log files remain immutable;
- root `TASKS.md` truth matches the actual M21-R02 state.

## REQUIRED CODEX LOG

Create exactly:

`docs/H!veAI/codex-logs/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V01_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- exact implementation commit SHA(s) known before log publication;
- root cause for F01 with file/symbol evidence;
- production data-flow change used to expose canonical remote task rows;
- focused test names/results for F01;
- exact default portfolio before/after behavior for F02;
- fresh-database/eight-project test evidence;
- tracker updates;
- relevant regression/build results;
- native QA publication/smoke result;
- explicit statement that no local parent or GitHub AI-Commerce repository was deleted;
- explicit `OWNER_REACCEPTANCE_REQUIRED` state;
- exact final publication verification performed after the log commit.

The log must not attempt to contain its own creating commit SHA. Return that SHA in the final completion response after publication.

## COMPLETION STATUS

Return exactly one high-level status:

- `COMPLETE_AWAITING_OWNER_REACCEPTANCE`
- `SYNC_BLOCKED`
- `IMPLEMENTATION_BLOCKED`

`COMPLETE_AWAITING_OWNER_REACCEPTANCE` is allowed only when all implementation/tests/publication requirements pass, all H!veAI changes are committed/pushed, and local `HEAD`, `origin/main`, and remote GitHub `main` are identical.

The final response must be GitHub-first and concise.