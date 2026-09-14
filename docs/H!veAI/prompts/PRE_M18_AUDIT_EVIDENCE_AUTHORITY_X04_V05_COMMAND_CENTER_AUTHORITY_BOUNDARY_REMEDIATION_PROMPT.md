# Pre-M18 Audit Evidence Authority X04 V05 — Command Center Authority-Boundary Remediation Prompt

## MANDATORY SYNC-FIRST / GITHUB-FIRST CONTRACT

Before reading or changing any repository file, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the worktree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never reset, rebase, force-push, destructively checkout, automatically stash, run `git clean`, discard owner work, or reconcile divergence automatically. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

Every Codex-facing artifact and builder log must be entirely in English.

## CANONICAL TRACKER FILES ARE READ-ONLY FOR CODEX

`TASKS.md` and `CODEX_ROADMAP.md` are strictly READ-ONLY for Codex.

Do not tick tasks, close/open milestones, change Current Task, change Required Actor, change progress, or activate M18. Tracker transitions remain owned by the independent ChatGPT audit/acceptance flow.

## WORK ITEM

- Work item: Pre-M18 Audit Evidence Authority X04 V05
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed audit: `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V04_STRICT_AUDIT.md`
- Required builder log: `docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V05_LOG.md`
- Findings to close: F-X04-V04-001 through F-X04-V04-004 only
- M18 must remain NOT ACTIVATED

Do not broaden into M18 GitHub Integration, provider redesign, tracker edits, or unrelated architecture cleanup.

## OWNER DECISION — END-TO-END ROOT TASKS CURRENT-STATE AUTHORITY

For current project/task/workflow-state presentation, repository-root `TASKS.md` is the only current-state authority for local ROOT_TASKS projects. GitHub-tracked projects continue to use tracked-branch root `TASKS.md` truth.

Historical/domain-specific evidence may remain visible in its own evidence surfaces, but it must not silently become current project/task/workflow status, health, actor, blockers, attention, work queue, progress, or next-action truth.

This V05 prompt is specifically about the **remaining outer Command Center boundary and regression-evidence defects**. Preserve all accepted V02/V03/V04 closures.

## ACCEPTED V04 BEHAVIOR — DO NOT REGRESS

Preserve all of the following:

1. Local ACTIVE projects with readable repository-root `TASKS.md` resolve as ROOT_TASKS_ONLY.
2. Missing/unreadable/non-file root TASKS fails closed as unavailable/needs-reconciliation.
3. `ProjectTruthResolver` does not use persisted workflow rows to select or override current truth.
4. ROOT_TASKS `ProjectDashboardResolution.materialized` is default/empty for current-state-bearing dashboard fields.
5. ROOT_TASKS `tracking_mode` is `ROOT_TASKS_ONLY`, never dashboard `single-dashboard-watch`.
6. Local ROOT_TASKS Project Cockpit omits the legacy `control_plane` current-state snapshot.
7. Project Cockpit current fields render from canonical `truth` / TASKS-derived data.
8. Persisted workflow rows and dashboard materialization do not create current attention/work-queue items inside ROOT_TASKS `summarize_project()`.
9. Hidden `.hiveai` source exclusion from task discovery/custom paths/audit source snippets remains intact.
10. GitHub remote root-TASKS behavior remains unchanged.

## F-X04-V04-001 — REMOVE MIXED DATABASE EVIDENCE FROM ROOT_TASKS CURRENT ATTENTION / WORK QUEUE / KPI TRUTH

### Defect

The outer `command_center::snapshot()` still merges `read_evidence_items()` into the same `attention` and `work_queue` arrays that the UI renders as **Needs Your Attention** and **Active Work Queue**.

`read_evidence_items()` currently emits entries from:

- failed `test_runs`;
- failed `audits`;
- pending `permission_requests`;
- live `agent_sessions`.

The current test `m11a_r06_mixed_evidence_attention_queue_and_activity_are_real_and_bounded` explicitly proves those database rows enter the ROOT_TASKS fixture's current `attention` / `work_queue` arrays.

Those records are legitimate evidence in their own domains, but V04 requirement 8 was explicit: current attention/work queue for a ROOT_TASKS project must be canonical TASKS / `ProjectTruth` truth, not a second database current-state ledger.

The mixed arrays also feed:

- `kpis.needs_attention`;
- `kpis.running`;
- Engineering Brief `Needs attention`;
- Engineering Brief `Running workflow tasks`.

The Engineering Brief currently labels those mixed counts as root-TASKS / `GITHUB_TASKS_ONLY` evidence even when the actual item is TEST_RUN, AUDIT, PERMISSION, or AGENT_SESSION. That provenance is false.

### Required behavior

For local ROOT_TASKS projects:

1. Current `attention` entries must be derived only from canonical root TASKS / `ProjectTruth` current truth.
2. Current `work_queue` entries must be derived only from canonical root TASKS / `ProjectTruth` current truth.
3. Failed test/audit records, permission requests, agent-session state, immutable workflow history, watcher snapshots, and dashboard projections must not enter ROOT_TASKS current attention/work queue.
4. Preserve those records in clearly historical/domain-specific evidence surfaces such as Recent Activity, Project Cockpit Audit, Tests, Agents, Permissions, or equivalent non-current evidence lists.
5. `kpis.needs_attention` and `kpis.running` must count authoritative current entries only.
6. Engineering Brief facts must use truthful provenance. Never label a non-TASKS record as `GITHUB_TASKS_ONLY` or root TASKS workflow evidence.
7. GitHub-tracked remote current attention/queue behavior remains unchanged.

### Preferred narrow current-item model

If root TASKS provides unambiguous current truth, it is acceptable and preferred to materialize bounded current Command Center items directly from `ProjectTruth`:

- `BLOCKED`, `WAITING_HUMAN`, `WAITING_EXTERNAL`, or TASKS-derived reconciliation -> current attention item with explicit ROOT_TASKS provenance;
- `IN_PROGRESS` / `RUNNING` -> current work-queue item using canonical task title/id, milestone, required actor, and next action when available.

If TASKS does not provide enough unambiguous current truth, show no fabricated current queue item and expose explicit reconciliation/unavailable state instead.

Do not infer current state from test/audit/permission/agent/workflow/watcher/dashboard evidence.

## F-X04-V04-002 — NORMALIZE LOCAL AND REMOTE CURRENT FIELDS IN COMMAND CENTER

### Defect

The Command Center backend/frontend current-project contract remains remote-shaped.

Local ROOT_TASKS `ProjectTruth` has canonical current milestone, required actor, blockers, progress scope, authority source, and reconciliation state, but the normalized `ProjectOperationSummary` / TypeScript `CommandCenterProject` contract does not carry all of those fields.

`CommandCenterProjectPanel` still renders milestone / required actor / blockers through `project.githubTracking` (`remote`) only, causing local canonical TASKS values to appear as unavailable or absent.

### Required behavior

Create one authority-neutral current-project contract used by both local ROOT_TASKS and GitHub-tracked root-TASKS projects.

At minimum expose normalized current fields:

- `currentMilestone`;
- `requiredActor`;
- `blockers`;
- `progressScope` if useful to current progress presentation;
- `authoritySource`;
- `reconciliationState`.

Use existing normalized fields for:

- current task;
- current state/workflow;
- next action;
- progress percent.

Population rules:

1. local ROOT_TASKS -> values come from canonical `ProjectTruth` only;
2. GitHub-tracked project -> the same normalized fields come from tracked-branch root TASKS remote truth;
3. ambiguous/unavailable local TASKS -> explicit `Unavailable` / `NEEDS_RECONCILIATION`, with no legacy projection fallback.

Frontend rules:

1. `CommandCenterProjectPanel` must render normalized canonical fields rather than remote-only `githubTracking` branches for current milestone/actor/blockers;
2. the panel may still show remote repository/branch/HEAD metadata in a separate remote-source section;
3. do not label local canonical blockers as `remote blockers`;
4. do not fabricate zeros/defaults when truth is unavailable.

Update Rust and TypeScript contracts together.

## F-X04-V04-003 — REMOVE LEGACY CONTROL-PLANE DEGRADED FALLBACK

### Defect

`degraded_project_summary()` currently calls `control_plane::snapshot()` and emits:

- `task_authority = CONTROL_PLANE_RECONCILIATION_REQUIRED`;
- `provenance_mode = CONTROL_PLANE_DEGRADED`;
- `next_action = Reconcile project control plane`;
- a serialized `control_plane` summary / `truth_sync`.

This helper is used when local `summarize_project()` fails, so the error path can still revive the legacy control plane as a current-facing reconciliation channel.

### Required behavior

For non-GitHub local projects, degraded/failure summary must fail closed around repository-root TASKS authority.

It must:

1. not call or serialize legacy control-plane current state;
2. not use `CONTROL_PLANE_*` as current authority/provenance;
3. not instruct the user to reconcile the legacy control plane;
4. expose `NEEDS_RECONCILIATION` / ROOT_TASKS unavailable semantics;
5. use a bounded TASKS-oriented next action appropriate to the failure, for example retry/restore/reconcile readable repository-root `TASKS.md` evidence;
6. keep current milestone/task/state/actor/blockers/progress empty unless canonical TASKS truth was actually resolved.

Add a deterministic failure-path test. Prefer an injectable or direct production-helper path over relying on environmental failure.

## F-X04-V04-004 — RESTORE EXECUTABLE TEST INTEGRITY AND COMPLETE THE ADVERSARIAL MATRIX

### Defect

V04 inserted unconditional `return;` statements into multiple existing `command_center.rs` tests. Assertions below those returns are unreachable dead code.

The V04 frontend poison test covers the canonical-present case only. The prompt-required case where canonical TASKS truth is unavailable/ambiguous while legacy projection values are poisoned is missing.

### Required behavior

1. Remove every unconditional early `return;` introduced by V04 from the affected test bodies.
2. Do not preserve obsolete assertions as unreachable code. Rewrite or delete assertions that no longer describe supported behavior.
3. Keep meaningful helper-level coverage for any historical/contextual materialized behavior that still exists.
4. Do not reduce regression quality merely to preserve aggregate test counts.

### Required deterministic tests

Add or rewrite direct tests covering at minimum:

1. ROOT_TASKS + dashboard `single-dashboard-watch` + poisoned materialized current values -> dashboard current materialized state stays default and tracking remains ROOT_TASKS_ONLY.
2. ROOT_TASKS + conflicting persisted workflow rows -> current summary/health/actor/next action/attention/work queue remain TASKS-derived.
3. ROOT_TASKS + failed test + failed audit + pending permission + running/waiting agent session -> those records remain available in historical/domain-specific evidence but do **not** enter current `attention`, current `work_queue`, `needs_attention`, `running`, or root-TASKS Engineering Brief counts/provenance.
4. ROOT_TASKS with unambiguous RUNNING/IN_PROGRESS current TASKS truth -> any current queue item is generated from TASKS truth only.
5. ROOT_TASKS with BLOCKED/WAITING truth -> any current attention item is generated from TASKS truth only.
6. local ROOT_TASKS current milestone, required actor, blockers, next action, progress, authority, and reconciliation serialize through the normalized Command Center project contract.
7. Command Center frontend renders those local canonical milestone/actor/blocker values without requiring `githubTracking`.
8. local TASKS unavailable/ambiguous + poisoned dashboard/control-plane/workflow values -> Command Center and Project Cockpit current UI show unavailable/needs-reconciliation and never render poison.
9. Project Cockpit canonical-present poison test remains green.
10. degraded local Command Center failure summary contains no legacy control-plane current channel/action/provenance.
11. historical workflow/test/audit/agent activity remains visible only in intended historical/domain-specific surfaces.
12. GitHub-tracked remote current fields/attention/queue remain unchanged.
13. X04 V02 hidden-source exclusions remain green.
14. X04 V03 canonical resolver conflict tests remain green.
15. X04 V04 Project Dashboard empty-materialization / Project Cockpit no-control-plane tests remain green.
16. X03 readiness persistence/historical wording regressions remain green.
17. M16T structured-output/freeform/task semantic-contract regressions remain green.
18. Exact eight-project portfolio and `Sekiph82/FormuLab@main` regressions remain green.
19. No registered external project repository is mutated.

Do not consume real Codex quota in deterministic tests.

## REQUIRED CONTEXT

Read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V04_STRICT_AUDIT.md`
- X04 V02/V03/V04 prompts, audits, and logs
- X03 strict audit / owner-native acceptance
- `src-tauri/src/command_center.rs`
- `src-tauri/src/project_dashboard.rs`
- `src-tauri/src/control_plane.rs`
- `src-tauri/src/project_cockpit.rs`
- `src-tauri/src/watcher.rs`
- `src/commandCenter.ts`
- `src/command_center_view.tsx`
- `src/projectCockpit.ts`
- `src/pages.tsx`
- relevant Rust/frontend tests.

Use registered external projects only as read-only regression examples. Do not modify Bulk-Edit, FormuLab, ScrubBots, or any other registered project repository.

## REGRESSION / SECURITY GATES

Run and record at minimum:

- focused Command Center ROOT_TASKS authority tests;
- focused Command Center evidence-partition tests;
- focused Command Center degraded/failure-path tests;
- focused Command Center local normalized-field frontend tests;
- focused Project Dashboard / Project Cockpit authority tests;
- X04 V02/V03/V04 adversarial authority tests;
- X03 readiness persistence/historical wording tests;
- M16T focused structured-output/freeform/task semantic-contract tests;
- exact eight-project portfolio / FormuLab@main tests;
- full serialized Rust library regression;
- full frontend Vitest regression;
- `npm run typecheck`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `npm run build`;
- `git diff --check`;
- active-source scan proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses transport, Codex auth-file inspection, GUI/browser automation, `ANTHROPIC_API_KEY`, direct Anthropic HTTP/API transport, blanket permission bypass, or M18 implementation;
- governed native QA publication through the existing production `--no-bundle` path.

Builder test counts remain claims pending independent review.

## REQUIRED BUILDER LOG

Create only:

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V05_LOG.md`

Record:

- starting synchronized SHA;
- implementation/test commit SHA(s);
- exact changed files;
- exact closure evidence for F-X04-V04-001 through F-X04-V04-004;
- exact current-vs-historical evidence partition behavior;
- exact normalized local/remote Command Center current-field behavior;
- exact degraded ROOT_TASKS fail-closed behavior;
- exact removal of unconditional test early returns / dead assertion blocks;
- exact adversarial Rust/frontend test names and results;
- full regression/security/publication claims;
- published executable SHA-256;
- explicit statement that `TASKS.md` and `CODEX_ROADMAP.md` were not modified;
- explicit statement that no registered external project repository was modified;
- final local HEAD / `origin/main` / live GitHub `main` equality and clean-worktree evidence.

## FINAL COMPLETION CONTRACT

Commit and push every H!veAI implementation, test, and log change before completion.

Before reporting completion:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

All three main SHAs must be identical and the worktree must be clean.

Final owner-facing response must contain only:

- GitHub URL/path for the X04 V05 builder log;
- implementation/test commit SHA(s);
- log commit SHA;
- final GitHub main SHA;
- concise status.

Do not update canonical tracker files. Do not activate M18.