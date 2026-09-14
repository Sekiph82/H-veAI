# Pre-M18 Audit Evidence Authority X04 V04 — Consumer / Presentation Authority Remediation Prompt

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

Do not tick tasks, close/open milestones, change Current Task, change Required Actor, change progress, or activate M18. Tracker transitions are owned by the independent ChatGPT audit flow after source audit and owner-native acceptance.

## WORK ITEM

- Work item: Pre-M18 Audit Evidence Authority X04 V04
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed audit: `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V03_STRICT_AUDIT.md`
- Required builder log: `docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V04_LOG.md`
- Findings to close: F-X04-V03-001 and F-X04-V03-002 only
- X03 remains PASS/CLOSED
- M17 remains PASS/CLOSED
- M18 must remain NOT ACTIVATED

Do not broaden into M18 GitHub Integration or unrelated architecture cleanup.

## OWNER DECISION — END-TO-END ROOT TASKS CURRENT-STATE AUTHORITY

For H!veAI current project tracking/state presentation, repository-root `TASKS.md` is the **only** source allowed to determine current project/task/workflow status.

This is an end-to-end contract. It applies not only to `ProjectTruthResolver`, but also to every backend payload, Project Dashboard resolution, Command Center summary/attention/work queue, Project Cockpit snapshot, and frontend current-state rendering path.

The following may remain historical/contextual evidence, but must never become current-state fallback, corroboration, disambiguation, health/attention truth, actor truth, progress truth, or current work truth:

- `.hiveai/PROJECT_DASHBOARD.md`
- `.hiveai/PROJECT.json`
- `.hiveai/STATE.json`
- `.hiveai/HANDOFF.md`
- `.hiveai/EVENT_INDEX.json`
- `.hiveai/TASKS.md`
- `.hiveai/RULES.md`
- `.hiveai/EVENTS.jsonl`
- persisted workflow/current-state rows
- watcher/materialized projections
- any equivalent historical/generated/local state ledger

Git metadata, implementation files, test/build evidence, audit history, immutable workflow history, and clearly historical activity may remain available in their own domains.

## ACCEPTED V03 BEHAVIOR — DO NOT REGRESS

The following V03 behavior is accepted and must be preserved:

1. ACTIVE non-GitHub local projects with readable root `TASKS.md` enter ROOT_TASKS authority unconditionally.
2. Missing/unreadable/non-file root `TASKS.md` fails closed as unavailable/needs-reconciliation without hidden-source fallback.
3. `control_plane::resolve_root_tasks_truth()` does not query persisted workflow rows to choose or override current truth.
4. Current task/status/workflow/required actor/next action/blockers/milestone/progress in canonical `ProjectTruth` are TASKS-derived only.
5. Multiple active root-TASKS candidates fail closed rather than being disambiguated by workflow DB.
6. X04 V02 hidden `.hiveai` source exclusion from task discovery/custom paths/audit source snippets remains intact.
7. The audit output contract remains ROOT_TASKS_ONLY.

Do not undo these accepted closures.

## F-X04-V03-001 — REMOVE CURRENT-STATE MATERIALIZATION FROM PROJECT DASHBOARD CONTEXT

### Defect

`project_dashboard::root_tasks_resolution()` still parses `.hiveai/PROJECT_DASHBOARD.md`, copies `parsed.materialized` into the public `ProjectDashboardResolution.materialized`, and preserves dashboard `trackingMode` such as `single-dashboard-watch`.

The materialized object contains current-state-bearing values including project status/health, current milestone, current task title/id, declared workflow state, progress, required actor, next action, waiting state, current work, and blockers/waiting.

V03 tests explicitly preserve those fields, which contradicts the requirement that contextual dashboard parsing must not feed current project/task/workflow state.

### Required behavior

1. For ROOT_TASKS local projects, `.hiveai/PROJECT_DASHBOARD.md` must not emit current-state-bearing materialized fields into the runtime current-state contract.
2. At minimum, the following dashboard-derived fields must be empty/default/non-authoritative under ROOT_TASKS:
   - `project_status`
   - `health`
   - `current_milestone`
   - `current_task_title`
   - `current_task_id`
   - `declared_workflow_state`
   - `progress_raw`
   - `progress_percent`
   - `required_actor`
   - `next_action`
   - `waiting_on`
   - `current_work`
   - `blockers_waiting`
3. Clearly non-current contextual facts such as bounded quality/build/test verification, historical activity, and provenance may be retained only if they cannot alter current-state fields, health, attention, queue, actor, next action, blockers, or progress.
4. ROOT_TASKS `tracking_mode` must not inherit `single-dashboard-watch` or another dashboard mode that re-enables legacy operational materialization. It must remain an unambiguous ROOT_TASKS-only tracking contract.
5. Do not delete historical files from registered projects and do not rewrite `.hiveai/PROJECT_DASHBOARD.md`. Fix H!veAI consumption only.
6. Do not revive legacy control-plane authority.

## F-X04-V03-002 — REMOVE CONSUMER / PRESENTATION FALLBACKS TO LEGACY PROJECTIONS

### Backend Project Cockpit

1. A ROOT_TASKS Project Cockpit snapshot must not expose legacy control-plane current-state values as a parallel current truth channel.
2. The current-facing payload must use canonical `ProjectTruth` / TASKS-derived `projectSummary` for current milestone, task, status/workflow, required actor, next action, blockers, progress, and reconciliation.
3. If the legacy `control_plane` snapshot remains available for diagnostics/history, it must be structurally or semantically separated as historical/contextual and must not be consumed by current-facing presentation. Prefer omitting/nulling the legacy current-state snapshot for ROOT_TASKS if that is the narrowest safe implementation.
4. Do not silently substitute PROJECT/STATE/HANDOFF/PROJECT_DASHBOARD values when TASKS truth is absent or ambiguous. Show unavailable/needs-reconciliation instead.

### Backend Command Center

5. For ROOT_TASKS projects, `materialized_operational_evidence()` must not create current attention/work-queue items from PROJECT_DASHBOARD materialization.
6. Dashboard-provided `trackingMode` must not activate `single-dashboard-watch` current operational behavior under ROOT_TASKS.
7. Persisted workflow rows may remain immutable history/diagnostic evidence, but conflicting workflow DB state must not create or override current project/task/workflow status, current health, required actor, next action, blocker truth, progress truth, or current attention/work queue for a ROOT_TASKS project.
8. If Command Center needs current attention/queue entries for a ROOT_TASKS project, derive them only from canonical TASKS/`ProjectTruth` fields. It is acceptable to show no current queue item when TASKS does not provide enough unambiguous current truth.
9. Historical workflow activity can remain in a clearly historical activity/history surface, provided it is not labeled or merged as current truth.

### Frontend Project Cockpit / current-state UI

10. Remove current-facing fallbacks such as:
   - current task title -> `dashboard.materialized.currentTaskTitle`
   - current workflow/state -> `dashboard.materialized.declaredWorkflowState`
   - current milestone -> `controlPlane.currentMilestone` or `materialized.currentMilestone`
   - required actor -> `controlPlane.requiredActor` or `materialized.requiredActor`
   - progress/current blockers/current work -> materialized/control-plane projection values
11. Current-facing fields must use remote GitHub root-TASKS truth for GitHub-tracked projects and canonical local `ProjectTruth`/TASKS-derived summary for local ROOT_TASKS projects.
12. If canonical TASKS truth is unavailable or ambiguous, render explicit unavailable/needs-reconciliation state. Never fill the gap with legacy projection data.
13. Workflow-history UI may still display persisted workflow records, but label them as historical/recorded operational evidence when necessary so they cannot be mistaken for canonical current truth.

## REQUIRED ADVERSARIAL TESTS

Add direct production-path tests covering at minimum:

1. ROOT_TASKS project plus `.hiveai/PROJECT_DASHBOARD.md` claiming conflicting current milestone/task/workflow/actor/progress -> `ProjectDashboardResolution` does not expose those claims as current materialized state.
2. ROOT_TASKS project plus dashboard `trackingMode: single-dashboard-watch` -> resolved tracking mode remains ROOT_TASKS-only and does not activate materialized current operations.
3. ROOT_TASKS project plus dashboard blockers/waiting/current-work rows -> Command Center current attention/work queue is not created from those dashboard projections.
4. ROOT_TASKS project plus conflicting persisted workflow row -> Command Center current state/health/actor/next action/attention/work queue remains TASKS-derived; workflow contradiction stays history/context only.
5. ROOT_TASKS Project Cockpit snapshot with poisoned legacy PROJECT/STATE/HANDOFF/dashboard values -> current-facing backend summary/truth remains TASKS-only and no parallel legacy current-state payload can be consumed as current truth.
6. Frontend render test with canonical TASKS truth plus deliberately poisoned `controlPlane` and `dashboard.materialized` current values -> current task/milestone/state/actor/progress rendered from canonical truth only; poisoned strings never appear in current surfaces.
7. Frontend render test with canonical TASKS truth unavailable/ambiguous plus poisoned legacy projection values -> UI shows unavailable/needs-reconciliation and never renders poisoned current values.
8. Historical workflow records remain visible only in their intended historical/operational surface and do not become the current-state header/summary.
9. Missing root TASKS still fails closed with no hidden/dashboard/workflow fallback.
10. Existing GitHub-tracked remote root-TASKS behavior remains unchanged.
11. X04 V02 hidden source exclusions remain green.
12. V03 canonical resolver conflict tests remain green.
13. X03 readiness persistence/historical wording regressions remain green.
14. M16T structured-output/freeform/task semantic-contract regressions remain green.
15. Exact eight-project portfolio and `Sekiph82/FormuLab@main` regressions remain green.
16. No registered external project repository is mutated.

Use deterministic fixtures only. Do not consume real Codex quota in tests.

## REQUIRED CONTEXT

Read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V03_STRICT_AUDIT.md`
- X04 V02/V03 prompts and logs
- X03 strict audit / owner-native acceptance
- `src-tauri/src/project_dashboard.rs`
- `src-tauri/src/control_plane.rs`
- `src-tauri/src/command_center.rs`
- `src-tauri/src/project_cockpit.rs`
- `src/pages.tsx`
- `src/projectCockpit.ts`
- relevant Rust and frontend tests.

Use registered external projects only as read-only regression examples. Do not modify Bulk-Edit, FormuLab, ScrubBots, or any other registered project repository.

## REGRESSION / SECURITY GATES

Run and record at minimum:

- focused Project Dashboard authority tests;
- focused ProjectTruth/root-TASKS tests;
- focused Command Center authority tests;
- focused Project Cockpit backend tests;
- focused frontend Project Cockpit/current-state rendering tests;
- X04 V02/V03 adversarial authority tests;
- X03 readiness persistence/historical wording tests;
- M16T focused structured-output/freeform/task semantic-contract tests;
- full serialized Rust library regression;
- full frontend Vitest regression;
- `npm run typecheck`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `npm run build`;
- `git diff --check`;
- active-source scan proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses transport, Codex auth-file inspection, GUI/browser automation, `ANTHROPIC_API_KEY`, direct Anthropic HTTP/API transport, blanket permission bypass, or M18 implementation;
- governed native QA publication through the existing production `--no-bundle` path.

Builder test counts are claims pending independent review.

## REQUIRED BUILDER LOG

Create only:

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V04_LOG.md`

Record:

- starting synchronized SHA;
- implementation/test commit SHA(s);
- exact changed files;
- exact closure evidence for F-X04-V03-001 and F-X04-V03-002;
- exact evidence that dashboard/control-plane/workflow projections cannot become current-facing fallback under ROOT_TASKS;
- exact adversarial Rust/frontend test names and results;
- full regression/security/publication claims;
- published executable SHA-256;
- explicit statement that `TASKS.md` and `CODEX_ROADMAP.md` were not modified;
- explicit statement that no registered project repository was modified;
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

- GitHub URL/path for the X04 V04 builder log;
- implementation/test commit SHA(s);
- log commit SHA;
- final GitHub main SHA;
- concise status.

Do not update canonical tracker files. Do not activate M18.