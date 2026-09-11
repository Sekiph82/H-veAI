# M21-R02 Project Cockpit Tasks and AI-Commerce Retirement Remediation V02

## MANDATORY SYNC-FIRST AND GITHUB-ONLY COMPLETION CONTRACT

Before reading or modifying the work item, operate only from the standalone `Sekiph82/H-veAI` repository and safely synchronize the local checkout with `origin/main`.

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

After synchronization, read before implementation:

- `AGENTS.md`
- `TASKS.md`
- `docs/H!veAI/audits/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V01_AUDIT.md`
- `docs/H!veAI/codex-logs/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V01_LOG.md`
- `docs/H!veAI/prompts/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V01_PROMPT.md`
- the current GitHub tracking, Project Cockpit, frontend Tasks rendering and focused tests

Work only in `Sekiph82/H-veAI`. Do not inspect or modify another GitHub repository.

All repository changes must be committed and pushed before completion is claimed. At the end run:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == remote refs/heads/main` and a clean H!veAI working tree. Otherwise return `SYNC_BLOCKED`, not completion.

The final owner-facing response must contain only the relevant GitHub file URLs/paths, implementation/log commit SHA(s), final GitHub `main` SHA, and concise status. Do not dump ordinary local changed-file paths.

## WORK ITEM

- Work code: `M21-R02`
- Version: `V02`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V02_LOG.md`
- Authoritative failed audit: `docs/H!veAI/audits/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V01_AUDIT.md`

V01 independent audit verdict is `FAIL`. Do not ask for owner native re-acceptance until V02 is implemented, published, independently audited, and accepted for retest.

The V01 eight-project default-portfolio retirement is already correct. Preserve it. Do not reintroduce `Sekiph82/AI-Commerce-HQ` as a target.

## FINDING M21-R02-V01-F01 — MAJOR — LEGACY PERSISTED REMOTE CACHE CAN SUPPRESS NEW TASK ROWS

### Proven current behavior

V01 added `RemoteTrackingSnapshot.task_rows` with `#[serde(default)]`.

A persisted remote snapshot written by a pre-V01 build therefore deserializes successfully with:

- its previous `remote_head`;
- `remote_health == CURRENT`;
- prior summary counts such as `total_tasks > 0`;
- `task_rows == []` because that field did not exist in the historical JSON.

Current `observe_project(...)` fetches the remote HEAD and returns the cached snapshot immediately when cached HEAD equals fetched HEAD and cached health is `CURRENT`. It does not validate that the cached snapshot contains the new row materialization.

Therefore an existing user's populated project can keep zero Cockpit Tasks indefinitely until the remote repository receives a new commit.

This is a production upgrade defect, not a test-only concern.

### Required target behavior

A structurally incomplete historical remote snapshot must self-heal without requiring a new remote commit.

A same-HEAD/current cache may be reused only when it is materially complete for the current snapshot contract.

At minimum:

- `total_tasks > 0` with zero/missing `task_rows` must force a root-`TASKS.md` fetch/parse/materialization and persistence;
- a valid current canonical snapshot with `total_tasks == 0` and zero rows must remain a legitimate empty state and must not be forced into an infinite refresh loop;
- the solution must remain bounded and deterministic;
- do not delete the SQLite database, clear all app data, or require the owner to manually remove cache files;
- do not make local filesystem `TASKS.md` the repair source for GitHub-tracked projects.

### Preferred implementation shape

Use a narrow compatibility/materialization predicate or an explicit snapshot schema/materialization version. Do not create a broad cache rewrite unless strictly necessary.

If adding a schema/materialization version, preserve backward-compatible deserialization and make old versions trigger remote re-materialization rather than destructive database migration.

The compatibility decision should be unit-testable without requiring real GitHub network access.

### Required focused tests

Add production-path tests that prove at least:

1. a pre-V01 serialized snapshot with populated task counts, `CURRENT` health, same remote HEAD, and no `taskRows` field is considered structurally incomplete;
2. that incomplete cache is not eligible for the same-HEAD early return;
3. after canonical remote content is reparsed, rows are materialized and persisted;
4. a genuine current zero-task snapshot remains complete/valid;
5. a new-format populated snapshot remains eligible for normal same-HEAD reuse.

Do not fake this only by directly constructing a new snapshot that already contains rows. Include historical JSON shape or equivalent durable compatibility evidence.

## FINDING M21-R02-V01-F02 — MAJOR — REMOTE UNAVAILABLE/ERROR STATE IS MISLABELED AS CANONICAL EMPTY

### Proven current behavior

`CockpitLiveTasks` currently sets:

```ts
const isRemote = Boolean(snapshot.githubTracking);
```

and when `isRemote && remoteTasks.length === 0`, it renders an empty state whose detail says:

`The remote root TASKS.md contains no parseable task rows.`

That statement is only true when a canonical remote document was successfully observed and its parsed total is genuinely zero. It is false when remote health is `UNAVAILABLE` or `ERROR` and no usable task document was obtained.

### Required target behavior

The remote Tasks panel must distinguish at least these states:

1. **CURRENT + populated rows**: render canonical rows normally.
2. **CURRENT + confirmed zero tasks**: render a truthful canonical empty state.
3. **STALE + cached rows**: render the last-known remote rows with a clear stale/degraded indication already consistent with current H!veAI warning semantics.
4. **UNAVAILABLE/ERROR + no usable rows**: render a remote-unavailable/error state, including the bounded reason when available. Never claim the canonical document contains zero parseable tasks when it was not successfully observed.
5. **structural inconsistency** such as `totalTasks > 0` but zero rows after compatibility handling: treat it as degraded/inconsistent, not a legitimate empty document.

Do not revive legacy task intelligence or local filesystem task truth as fallback.

### Required focused tests

Add/update frontend or contract-level tests that cover:

- CURRENT populated;
- CURRENT confirmed empty;
- STALE cached rows;
- UNAVAILABLE with no rows;
- ERROR with no rows;
- structurally inconsistent populated-count/zero-row state if that state can reach the UI.

The tests must prove the displayed state/message is semantically truthful, not merely that the component renders.

## PRESERVE V01 F02 — EIGHT-PROJECT DEFAULT PORTFOLIO

The following V01 work is accepted and must remain unchanged in effect:

- default portfolio contains exactly the intended eight repositories;
- `Sekiph82/AI-Commerce-HQ` is not a bootstrap/default target;
- no special AI-Commerce branch bootstrap remains;
- fresh databases do not require an exclusion row to remain at eight projects;
- existing unrelated project state is not deleted or duplicated.

Keep the existing portfolio tests green. Add no new AI-Commerce compatibility branch that can recreate it as an active default target.

## TRACKER TRUTH

V01 failed independent audit. Update the root `TASKS.md` current operational state so it no longer implies V01 is merely awaiting owner re-acceptance.

During V02 implementation, `M21-R02` remains the active remediation. After implementation is complete, the correct state is implementation-complete V02 awaiting independent audit. Owner re-acceptance must remain pending until the independent V02 audit passes.

Preserve historical M21/M21-R01 truth and the user-facing 20-milestone denominator.

Do not rewrite historical V01 prompt/log/audit files.

## DELETION / RETIREMENT SAFETY BOUNDARY

This work item does not authorize destructive retirement.

Do not:

- delete or relocate the historical local AI-Commerce parent tree;
- delete preservation trees;
- delete the GitHub `Sekiph82/AI-Commerce-HQ` repository;
- modify another GitHub repository;
- claim `SAFE_TO_DELETE_PARENT_DIRECTORY`;
- move unrelated repositories.

The separate final relocation/retirement gate remains after M21-R02 passes independent audit and owner native re-acceptance.

## REQUIRED REGRESSION VALIDATION

Run the minimum relevant suite needed for confidence after the touched source set, including:

- focused legacy-cache compatibility tests;
- focused remote-state truthfulness tests;
- existing root `TASKS.md` parser tests;
- existing Project Cockpit remote-primary tests;
- existing Command Center/Cockpit parity tests;
- existing eight-project portfolio tests;
- relevant frontend tests;
- `npm run typecheck`;
- `npm run build`;
- relevant Rust library tests;
- `git diff --check`.

The source diff must not add secrets, local DB files, caches, build output, preservation copies, machine-specific private data, or unrelated repository source.

## NATIVE QA PUBLICATION

Only after the implementation and required automated validation pass, publish the validated native QA executable using the existing safe H!veAI development publication process.

Preserve the stable Desktop shortcut model and its icon/target rules.

Automated smoke checks may be performed. Do not fabricate owner visual acceptance.

Do not ask the owner to re-test until the V02 log is published and an independent audit is requested.

## REQUIRED CODEX LOG

Create exactly:

`docs/H!veAI/codex-logs/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V02_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- exact implementation commit SHA(s) known before log publication;
- the V01 F01 root cause and exact compatibility rule implemented;
- historical-cache fixture/evidence and test names/results;
- proof that genuine zero-task snapshots remain valid;
- V01 F02 UI truthfulness remediation and exact state matrix;
- focused test names/results for CURRENT populated/empty, STALE, UNAVAILABLE, ERROR and structural inconsistency where applicable;
- proof the eight-project default portfolio remains intact and AI-Commerce-HQ remains absent;
- tracker update summary;
- relevant regression/typecheck/build results;
- native QA publication/smoke result and executable hash if available;
- explicit statement that no local parent/preservation tree/GitHub AI-Commerce repository was deleted;
- explicit statement that owner re-acceptance remains pending independent V02 audit;
- final GitHub publication verification semantics.

The log must not try to contain its own creating commit SHA. Return the log commit SHA and final remote `main` SHA in the final Codex response.

## COMPLETION STATUS

Return exactly one high-level status:

- `COMPLETE_AWAITING_INDEPENDENT_AUDIT`
- `SYNC_BLOCKED`
- `IMPLEMENTATION_BLOCKED`

`COMPLETE_AWAITING_INDEPENDENT_AUDIT` is allowed only when all required implementation/tests/publication gates pass, all H!veAI changes are committed/pushed, and local `HEAD`, `origin/main`, and remote GitHub `main` are identical.

Do not return an owner acceptance PASS. The next gate is independent strict audit.