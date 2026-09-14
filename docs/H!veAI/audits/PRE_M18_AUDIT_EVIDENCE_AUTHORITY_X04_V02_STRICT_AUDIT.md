# Pre-M18 Audit Evidence Authority X04 V02 — Independent Strict Audit

## 1. Verdict

**FAIL / CHANGES_REQUIRED**

X04 V02 closes the hidden-control-plane source-admission defect in the audit engine and task-source discovery, but the owner decision that repository-root `TASKS.md` is the **only** current project/task/workflow-status authority is not yet enforced across all current-state resolution paths. Two MAJOR findings remain. M18 must remain blocked.

## 2. Scope

Repository: `Sekiph82/H-veAI`

Branch: `main`

Builder implementation commit:

- `f5d85718b81b47fe6867b84e0b2bb38d50ac9996`

Builder log commit / audited live `main` before this audit:

- `4d02f0304c1dc21c9409b14df6b38b65ce7ab70f`

Authoritative prompt:

- `docs/H!veAI/prompts/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V02_TASKS_ONLY_REMEDIATION_PROMPT.md`

## 3. Builder-log treatment

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V02_LOG.md` was treated as builder claim evidence only. Acceptance was determined from live GitHub source, direct test bodies, commit diff, repository state, and the owner TASKS-only authority decision.

## 4. Accepted closure — hidden source admission

**ACCEPTED.** `task_sources::is_hidden_control_plane_path()` centralizes a case-insensitive `.hiveai` path boundary. Standard discovery no longer traverses `.hiveai`; custom source normalization rejects paths entering it; collection also rejects hidden paths.

This closes the original path by which hidden STATE/HANDOFF/EVENT_INDEX/PROJECT/TASKS/RULES/EVENTS projections could re-enter task-source discovery merely because they existed locally.

## 5. Accepted closure — audit source evidence

**ACCEPTED.** `audit_engine::is_auditable_path()` and `read_source_evidence()` exclude hidden `.hiveai` paths from ordinary source snippets while preserving normal implementation `.rs`, `.ts`, `.tsx`, ordinary `.json`, and `.md` evidence outside that family.

The exclusion applies before working-tree/staged/commit-range source material is admitted, so hidden control-plane contents no longer become VERIFIED implementation/current-authority evidence.

## 6. Accepted closure — audit model contract

**ACCEPTED.** `audit_output_contract()` explicitly states that repository-root `TASKS.md` is the sole current project/task/workflow-status authority, that hidden/local/generated control-plane files may not determine or repair current state, and that remediation must not recommend creating or reviving them.

Legitimate Git metadata, implementation source, tests, and governance/architecture evidence remain available for their own purposes.

## 7. Accepted closure — adversarial hidden-projection test

**ACCEPTED AS FAR AS IT GOES.** `control_plane::tests::x04_root_tasks_remains_truth_when_every_hidden_projection_conflicts` creates conflicting PROJECT/STATE/HANDOFF/EVENT_INDEX/TASKS/RULES/EVENTS files and proves the tested resolver selects the root TASKS task instead.

The test is valuable, but it does not exercise the residual paths described below.

## 8. Finding F-X04-V02-001 — Project Dashboard still has alternate current-state authorities

**MAJOR / OPEN.** `project_dashboard::resolve()` returns `root_tasks_resolution()` only when both conditions are true:

1. root `TASKS.md` exists; and
2. `hidden_control_plane_present(&root)` is true.

If root `TASKS.md` exists but the listed hidden projection files do **not** exist, the resolver continues through legacy `.hiveai/PROJECT.json` / `.hiveai/PROJECT_DASHBOARD.md` paths. Those paths can materialize current milestone, current task, declared workflow state, required actor, next action, progress, waiting state, and other current-looking values.

The owner contract is unconditional: root `TASKS.md` is the only current project/task/workflow-status authority whether hidden files exist or not.

## 9. Direct evidence for F-X04-V02-001

The current test fixture itself demonstrates the gap. `project_dashboard` tests retain behavior where a valid `PROJECT_DASHBOARD.md` can populate `materialized.current_milestone`, `declared_workflow_state`, `required_actor`, progress, and related fields while root `TASKS.md` exists and no listed hidden control-plane projection triggers `root_tasks_resolution()`.

Likewise, when root `TASKS.md` is absent, current resolver code can still enter legacy control-plane or dashboard-manifest resolution. Under the owner decision, absence of root TASKS must degrade to explicit unavailable/reconciliation truth, not promote a hidden/legacy file to current authority.

## 10. Required closure for F-X04-V02-001

For local ACTIVE projects, current-state Project Dashboard resolution must be TASKS-only independent of hidden-file presence. A dashboard/manifest may remain bounded non-authoritative context if still needed for presentation, but it must not populate or override current project/task/workflow-status truth.

If root `TASKS.md` is absent/unavailable, current-state resolution must fail closed as unavailable/needs-reconciliation rather than falling back to PROJECT/STATE/HANDOFF/PROJECT_DASHBOARD current-state materialization.

## 11. Finding F-X04-V02-002 — workflow DB rows still override TASKS current truth

**MAJOR / OPEN.** `control_plane::resolve_root_tasks_truth()` filters task identity to root `TASKS.md`, but it then queries persisted workflow rows and allows those rows to act as a second current-state authority.

Specifically, a matching workflow row can:

- select the current task;
- override `current_task_status` with `workflow.current_state`;
- override `workflow_state` with `workflow.current_state`;
- override `required_actor`;
- replace TASKS `next_step` with the first workflow `allowed_next_states` value.

That violates the owner decision that root `TASKS.md` is the **only** current project/task/workflow-status authority.

## 12. Required closure for F-X04-V02-002

Persisted workflow rows may remain historical/operational evidence, but they may not select, disambiguate, corroborate, or override current TASKS-derived project/task/workflow-status fields.

Current task/status/workflow/required actor/next action/blockers/progress must be derived from root `TASKS.md` only. Ambiguity in TASKS must fail closed as reconciliation/unknown rather than being silently resolved from another state ledger.

## 13. Audit authority evidence consistency

`audit_engine::authority_evidence()` now emits a fixed ROOT_TASKS_ONLY contract and intentionally ignores the supplied dashboard resolution. That contract is correct as policy, but until F-X04-V02-001 and F-X04-V02-002 close, the runtime has paths that do not actually obey the contract.

V03 must make runtime truth agree with the audit contract rather than weakening the contract or hiding inconsistent runtime state.

## 14. Task-source behavior

The broad task-source inventory may continue to expose legitimate non-hidden documentation/source material for contextual purposes, but no non-TASKS source may become current project/task/workflow-status authority. Hidden `.hiveai` content must remain excluded from current-state and ordinary audit source admission.

## 15. Security and mutation boundary

No evidence was found that the X04 V02 implementation mutates Bulk-Edit or another registered project as part of audit collection. The hidden-path exclusions are read-policy changes. No `OPENAI_API_KEY`, direct OpenAI HTTP/Responses audit transport, auth-file inspection, GUI automation, or M18 implementation was introduced in the inspected bounded diff.

## 16. Tracker governance

Comparison from `b8879fca599e444822e1ce4fe0f15eb784118bef` through builder log commit `4d02f0304c1dc21c9409b14df6b38b65ce7ab70f` changes only:

- `src-tauri/src/audit_engine.rs`
- `src-tauri/src/control_plane.rs`
- `src-tauri/src/project_dashboard.rs`
- `src-tauri/src/task_sources.rs`
- the X04 V02 builder log

`TASKS.md` and `CODEX_ROADMAP.md` were not modified by Codex, satisfying `TRACKER_TRANSITION_OWNERSHIP_V01.md`.

## 17. CI / test claim status

The builder reports 486 serialized Rust tests, 141 frontend tests, typecheck, cargo check, frontend build, `git diff --check`, security scan, and governed publication as passing. GitHub exposes no independent status checks on the implementation commit, so aggregate pass counts remain builder claims.

Direct inspected source/test bodies substantiate the accepted portions but also expose the two MAJOR residual authority defects.

## 18. Completion-contract evidence note

The builder log records the implementation and publication claims but does not print the final local HEAD / `origin/main` / live-main SHA triplet and clean-worktree output requested by the prompt; it states that final publication *will* verify them. Live GitHub `main` was independently observed at `4d02f0304c1dc21c9409b14df6b38b65ce7ab70f`, but local-worktree cleanliness cannot be independently reconstructed from GitHub.

This is an evidence/process gap, not the reason for the FAIL; the two source findings above are independently blocking.

## 19. Native gate

Do **not** run owner-native X04 acceptance yet. A native test against the current build could pass for GitHub-tracked portfolio projects while the residual local-project/dashboard/workflow authority paths remain defective.

Owner-native acceptance resumes only after V03 independently closes both MAJOR findings.

## 20. Closure decision

X04 V02 is **CHANGES_REQUIRED**. F-X04-001/F-X04-002 are partially closed, but the stronger owner TASKS-only decision is not yet globally true in current-state resolution.

Required next action: execute a bounded X04 V03 remediation for F-X04-V02-001 and F-X04-V02-002 only. Preserve the accepted hidden-source exclusion, audit contract, X03 behavior, M00-M17 behavior, exact eight-project portfolio, and `Sekiph82/FormuLab@main`. `TASKS.md` and `CODEX_ROADMAP.md` remain READ-ONLY for Codex. M18 remains blocked.