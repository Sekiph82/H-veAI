# Pre-M18 Audit Evidence Authority X04 V03 — Independent Strict Audit

## 1. Verdict

**FAIL / CHANGES_REQUIRED**

X04 V03 successfully removes persisted workflow rows from the canonical `ProjectTruth` current-state resolver and makes local Project Dashboard authority enter ROOT_TASKS_ONLY whenever repository-root `TASKS.md` is readable. However, the stronger owner contract is still not true end-to-end: `.hiveai/PROJECT_DASHBOARD.md` and legacy control-plane materialization remain present in public runtime payloads and are still consumed as current-looking fallback/operational state by Project Cockpit and Command Center. Two MAJOR consumer-boundary findings remain. M18 must remain blocked.

## 2. Scope

Repository: `Sekiph82/H-veAI`

Branch: `main`

Builder implementation/test commit:

- `f0b06a215fa2ea64ec413342c0a5978cf8b1bb3b`

Builder log/current live main before this audit:

- `c3512614ef070c428af13c81a1951bb7a8f14eb0`

Authoritative prompt:

- `docs/H!veAI/prompts/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V03_TASKS_ONLY_RESIDUAL_REMEDIATION_PROMPT.md`

## 3. Builder-log treatment

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V03_LOG.md` was treated as builder claim evidence only. Acceptance was determined from live GitHub source, direct test bodies, implementation diff, repository state, and the owner TASKS-only authority decision.

The builder reports 489 serialized Rust tests, 141 frontend tests, typecheck, cargo check, production build, security scan, and governed publication as passing. GitHub exposes no independent status checks for the implementation commit, so aggregate test/publication claims remain unverified builder claims.

## 4. Accepted closure — unconditional local ROOT_TASKS authority entry

**ACCEPTED.** `project_dashboard::resolve()` now checks readable repository-root `TASKS.md` directly for every ACTIVE non-GitHub local project and returns `root_tasks_resolution()` without requiring hidden `.hiveai` files to exist.

If root `TASKS.md` is absent, unreadable, or not a file, the resolver now returns `ROOT_TASKS_UNAVAILABLE` rather than promoting PROJECT/STATE/HANDOFF/PROJECT_DASHBOARD to current authority.

This closes the outer resolver-branch defect from F-X04-V02-001.

## 5. Accepted closure — canonical ProjectTruth no longer reads workflow DB

**ACCEPTED.** `control_plane::resolve_root_tasks_truth()` now derives current task selection, status, workflow state, required actor, next action, blockers, milestone, and progress from root-TASKS task intelligence only.

Persisted workflow rows are no longer queried in this canonical resolver. Multiple active TASKS candidates fail closed as `NEEDS_RECONCILIATION`, and missing/unreadable root TASKS returns `ROOT_TASKS_UNAVAILABLE`.

This closes the resolver-level defect from F-X04-V02-002.

## 6. Accepted closure — root TASKS wins conflicting workflow row in resolver tests

**ACCEPTED.** V03 adds direct adversarial tests where persisted workflow state conflicts with root `TASKS.md`, and where workflow history attempts to disambiguate multiple active TASKS candidates. The canonical resolver remains TASKS-derived or fails closed.

The implementation diff also removes workflow-derived current status, actor, and next-action precedence from `resolve_root_tasks_truth()`.

## 7. Accepted closure — missing root TASKS fails closed

**ACCEPTED.** V03 adds an explicit unreadable/absent-root-TASKS path that returns no current task, no current status, no actor, no progress, `NEEDS_RECONCILIATION`, and a bounded restore-TASKS next action.

The legacy hidden control plane is therefore no longer a fallback for the canonical `ProjectTruth` resolver.

## 8. Finding F-X04-V03-001 — Project Dashboard still serializes forbidden current-state projection fields

**MAJOR / OPEN.** `root_tasks_resolution()` still parses `.hiveai/PROJECT_DASHBOARD.md` when present and copies `parsed.materialized` wholesale into `ProjectDashboardResolution.materialized`.

That materialized structure contains current-state-bearing fields including project status, health, current milestone, current task title/id, declared workflow state, progress, required actor, next action, waiting state, current work, and blockers/waiting.

The V03 requirement was stricter: contextual dashboard parsing may remain only if it **does not feed current milestone, current task, workflow state, required actor, next action, blockers, waiting state, or progress**.

Returning those values in the runtime dashboard object leaves a second current-looking state channel alive even though the authority flags say ROOT_TASKS_ONLY.

## 9. Direct test evidence for F-X04-V03-001

The V03 test `resolver_keeps_dashboard_materialization_contextual_to_root_tasks_authority` explicitly asserts that a ROOT_TASKS resolution still exposes dashboard-derived `materialized.project_status == ACTIVE` and `materialized.progress_percent == 55`.

The test `hiveai_dogfood_dashboard_cannot_override_root_tasks_authority` likewise asserts that ROOT_TASKS resolution still exposes `materialized.current_milestone == M14` from the dashboard fixture.

Those tests prove authority labels changed while forbidden current-state materialization remains available to consumers. They do not prove the V03 non-feed requirement; they preserve the opposite behavior.

## 10. Required closure for F-X04-V03-001

For ROOT_TASKS_ONLY local projects, Project Dashboard may retain only clearly non-current contextual evidence that cannot masquerade as project/task/workflow status.

At minimum, dashboard-derived project status/health, current milestone/task id/task title, declared workflow state, progress, required actor, next action, waiting state, current work, and blockers/waiting must not be emitted as current-state materialization.

If quality verification, historical activity, build/test facts, or provenance are retained, they must be explicitly contextual and must not alter current project/task/workflow truth, health, attention, queue, actor, or next action.

`tracking_mode` for a ROOT_TASKS resolution must not inherit a dashboard mode that re-enables legacy operational materialization; it should remain an unambiguous ROOT_TASKS-only contract.

## 11. Finding F-X04-V03-002 — Cockpit and Command Center consume projection data as current state

**MAJOR / OPEN.** The residual materialization is not merely serialized dead context.

`ProjectCockpitSnapshot` still includes the full legacy `control_plane` snapshot alongside `truth` and `dashboard` for a local project. That legacy snapshot carries current-task/workflow/milestone/actor/next-action/progress fields from the old control-plane model.

The frontend Project Cockpit then uses projection fields as current fallbacks. Examples in `src/pages.tsx` include:

- current task title falling back to `dashboard.materialized.currentTaskTitle`;
- workflow/current state falling back to `dashboard.materialized.declaredWorkflowState`;
- current milestone using `controlPlane.currentMilestone ?? materialized.currentMilestone`;
- required actor using `controlPlane.requiredActor` and then materialized actor fallback.

Therefore an old PROJECT_DASHBOARD/control-plane projection can still become visible current state when canonical TASKS truth is absent/ambiguous at a particular field.

## 12. Command Center operational leak

The Command Center also still admits projection state through current operational surfaces.

`root_tasks_resolution()` preserves the dashboard-provided `trackingMode`, including `single-dashboard-watch`. `command_center::is_single_dashboard_resolution()` treats that mode as eligible for `materialized_operational_evidence()`, which converts dashboard `current_task_id`, blockers/waiting, project status, health, and other materialized facts into current attention/work-queue items.

In addition, persisted workflow rows remain used to create current attention/work-queue entries for local projects even when `root_tasks_only` is true. Historical/operational retention is allowed by V03, but it must not be presented as a second current project/task/workflow-status channel. Current queue/attention derived from contradictory persisted workflow state is not safely separated from canonical current truth.

## 13. Required closure for F-X04-V03-002

For ROOT_TASKS_ONLY projects, every current-facing consumer must use canonical `ProjectTruth`/TASKS-derived summary fields for current milestone, task, state, actor, next action, blockers, progress, health, attention, and current work queue.

Project Cockpit must not use legacy `controlPlane` or dashboard materialized current-state fields as fallbacks. Those projections may be omitted from the ROOT_TASKS payload or retained only in a clearly historical/contextual structure that current-facing rendering cannot consume.

Command Center must not create current attention/work-queue truth from PROJECT_DASHBOARD materialization under ROOT_TASKS. Persisted workflow rows may remain immutable history/diagnostics, but contradictory workflow state must not become current attention/queue/status truth.

## 14. Frontend parity requirement

This is an end-to-end authority contract, not only a Rust resolver contract.

A backend object marked ROOT_TASKS_ONLY while the frontend still falls back to `controlPlane.currentMilestone` or `dashboard.materialized.currentTaskTitle` is not compliant. V04 must add direct frontend tests proving conflicting legacy projection values are never rendered in current-task/current-state/current-milestone/current-actor/current-progress surfaces for a ROOT_TASKS project.

## 15. Audit-source boundary remains accepted

X04 V02 hidden-source exclusion remains accepted. `.hiveai` control-plane files are excluded from task-source discovery/custom paths and ordinary audit source snippets, while normal implementation/Git/test evidence remains auditable.

V04 must preserve that boundary and must not weaken the accepted audit output contract stating that root `TASKS.md` is the sole current project/task/workflow-status authority.

## 16. Security / mutation / scope boundary

No evidence in the bounded V03 implementation diff indicates direct OpenAI/Anthropic API-key transport, auth-file inspection, GUI/browser automation, blanket permission bypass, or M18 implementation. No registered external project repository modification is present in the GitHub diff.

V04 must remain narrowly limited to the consumer/presentation authority leak and preserve the existing Codex-only audit provider and all accepted M00-M17/X03/X04 behavior.

## 17. Tracker governance and repository diff

Comparison from V03 starting SHA `931d5512c307660abad10494bfb2705aaad0aae1` through live builder-log main `c3512614ef070c428af13c81a1951bb7a8f14eb0` changes only:

- `src-tauri/src/command_center.rs`
- `src-tauri/src/control_plane.rs`
- `src-tauri/src/project_cockpit.rs`
- `src-tauri/src/project_dashboard.rs`
- `docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V03_LOG.md`

`TASKS.md` and `CODEX_ROADMAP.md` were not changed by the builder diff. Tracker-transition ownership remains with the independent ChatGPT audit flow. M18 remains NOT ACTIVATED.

## 18. CI / completion-evidence note

GitHub exposes no independent status checks on `f0b06a215fa2ea64ec413342c0a5978cf8b1bb3b`; builder regression counts remain claims.

The V03 log's recorded equality triplet is `bbcf8046767675f6070c97b5974a18d5c599d68b`, while the subsequently revised log is on live main `c3512614ef070c428af13c81a1951bb7a8f14eb0`. The earlier equality statement therefore does not itself prove final local/origin/live equality after the log revision. This is a process-evidence wrinkle, not the source-level reason for FAIL.

## 19. Native gate

Do **not** run owner-native X04 acceptance yet.

A native test now could show correct TASKS-derived summary fields while still silently rendering dashboard/control-plane fallback values in fields where TASKS is ambiguous or absent. V04 must first close the public-payload and consumer fallbacks, then pass independent strict audit.

## 20. Closure decision

X04 V03 is **CHANGES_REQUIRED**.

F-X04-V02-002 is closed at canonical resolver level, and the unconditional ROOT_TASKS resolver entry from F-X04-V02-001 is accepted. However, ROOT_TASKS_ONLY is still not end-to-end because current-state projection values survive in `ProjectDashboardResolution`, `ProjectCockpitSnapshot`, frontend current-field fallbacks, and Command Center operational materialization.

Required next action: execute a narrowly bounded X04 V04 consumer/presentation authority remediation. Preserve all accepted X04 V02/V03 resolver and audit-source behavior, X03, M00-M17, exact eight-project portfolio, and `Sekiph82/FormuLab@main`. `TASKS.md` and `CODEX_ROADMAP.md` remain READ-ONLY for Codex. M18 remains blocked.