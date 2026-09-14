# Pre-M18 Audit Evidence Authority X04 V03 — TASKS-Only Residual Remediation Prompt

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

Never reset, rebase, force-push, destructively checkout, automatically stash, run `git clean`, or discard/reconcile owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

Every Codex-facing artifact and builder log must be entirely in English.

## CANONICAL TRACKER FILES ARE READ-ONLY FOR CODEX

`TASKS.md` and `CODEX_ROADMAP.md` are strictly READ-ONLY for Codex.

Do not tick tasks, close/open milestones, change Current Task, change Required Actor, change progress percentages, or activate M18. Tracker transitions are owned by ChatGPT after independent audit and owner-native acceptance.

## WORK ITEM

- Work item: Pre-M18 Audit Evidence Authority Hotfix X04 V03
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed strict audit: `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V02_STRICT_AUDIT.md`
- Required builder log: `docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V03_LOG.md`
- Findings to close: F-X04-V02-001 and F-X04-V02-002 only
- Preserve all accepted X04 V02 behavior
- X03 remains PASS/CLOSED
- M17 remains PASS/CLOSED
- M18 must remain NOT ACTIVATED

Do not broaden into M18 GitHub Integration.

## OWNER DECISION — NON-NEGOTIABLE TASKS-ONLY CURRENT-STATE AUTHORITY

For H!veAI current project tracking/state resolution, repository-root `TASKS.md` is the **only** source allowed to determine current project/task/workflow status.

Nothing else may select, corroborate, override, reconcile, infer, or repair current project/task/workflow-status truth, including:

- `.hiveai/STATE.json`
- `.hiveai/HANDOFF.md`
- `.hiveai/EVENT_INDEX.json`
- `.hiveai/PROJECT.json`
- `.hiveai/TASKS.md`
- `.hiveai/RULES.md`
- `.hiveai/EVENTS.jsonl`
- `.hiveai/PROJECT_DASHBOARD.md`
- persisted workflow/current-state rows
- watcher/materialized projections
- any equivalent historical/generated/local state ledger

Git identity, branch, HEAD, diff, implementation source, test results, audit history, and architecture/governance evidence remain legitimate evidence for their own domains. They are not current project/task/workflow-status authority.

## F-X04-V02-001 — REMOVE PROJECT DASHBOARD / LEGACY CONTROL-PLANE CURRENT-STATE FALLBACK

### Defect

`project_dashboard::resolve()` currently enters `root_tasks_resolution()` only when root `TASKS.md` exists **and** one of the listed hidden control-plane files exists. A local project with root TASKS but without those hidden projections may still resolve `.hiveai/PROJECT_DASHBOARD.md` or legacy `.hiveai/PROJECT.json` as a current-looking source and materialize milestone/task/workflow/actor/next-action/progress values from it.

### Required behavior

1. For every ACTIVE non-GitHub-tracked local project, if repository-root `TASKS.md` is a valid file, current-state dashboard authority must be ROOT_TASKS_ONLY regardless of whether any `.hiveai` files exist.
2. `.hiveai/PROJECT_DASHBOARD.md`, PROJECT/STATE/HANDOFF/EVENT_INDEX/TASKS/RULES/EVENTS, and any equivalent projection must not populate or override current project/task/workflow-status fields.
3. If dashboard/source-map parsing remains for non-current contextual presentation, it must be explicitly non-authoritative and must not feed current milestone, current task, workflow state, required actor, next action, blockers, waiting state, or progress.
4. If repository-root `TASKS.md` is absent, unreadable, not a file, or otherwise unavailable, current-state resolution must fail closed as unavailable/needs-reconciliation. Do not promote a hidden/legacy source to current authority.
5. Preserve GitHub-tracked project behavior where the tracked branch root `TASKS.md` remains the remote current-state authority.
6. Do not create, revive, rewrite, migrate, or require hidden control-plane files.
7. Preserve accepted X04 V02 hidden-source exclusion from task discovery and audit source evidence.

## F-X04-V02-002 — REMOVE PERSISTED WORKFLOW ROWS AS A SECOND CURRENT-STATE AUTHORITY

### Defect

`control_plane::resolve_root_tasks_truth()` filters task identity to root `TASKS.md`, but then uses persisted workflow rows to select/disambiguate current task and to override current status, workflow state, required actor, and next action.

### Required behavior

1. Current task selection must be derived from root `TASKS.md` only.
2. Current task status and workflow-status truth must be derived from root `TASKS.md` only.
3. Required actor, next action, blockers, milestone/cycle, and progress used as current project-state truth must be derived from root `TASKS.md` only.
4. Persisted workflow rows may remain historical/operational evidence, but they may not select, disambiguate, corroborate, or override current TASKS truth.
5. If root TASKS contains ambiguous multiple current/active candidates and does not itself identify one canonical current task, fail closed as `NEEDS_RECONCILIATION`/unknown. Do not resolve ambiguity from workflow DB state.
6. If TASKS has no usable current-state truth, fail closed. Do not fall back to STATE/HANDOFF/dashboard/workflow materialization.
7. Preserve immutable workflow history and unrelated M10 behavior; this remediation changes authority consumption for project-current-state resolution, not historical event retention.

## AUDIT CONTRACT PARITY

The accepted X04 V02 audit contract says ROOT_TASKS_ONLY. Keep it.

Do not weaken `audit_output_contract()` or re-admit hidden `.hiveai` source snippets. Instead make actual runtime resolution agree with that contract.

If `authority_evidence()` continues to emit a fixed ROOT_TASKS_ONLY policy object, add direct tests proving the actual current-state resolver and Project Dashboard resolver obey the same policy in both ordinary and adversarial local-project cases.

## REQUIRED DETERMINISTIC TESTS

Add direct production-path tests covering at minimum:

1. Local ACTIVE project with root `TASKS.md` and **no hidden control-plane files** -> Project Dashboard current authority is ROOT_TASKS_ONLY.
2. Same fixture plus a conflicting `.hiveai/PROJECT_DASHBOARD.md` that claims a different milestone/task/workflow/actor/progress -> current state still follows root `TASKS.md`; conflicting materialized dashboard values do not become current truth.
3. Root `TASKS.md` plus conflicting `.hiveai/PROJECT.json`, STATE, HANDOFF, EVENT_INDEX, hidden TASKS, RULES, EVENTS -> root TASKS remains sole current authority.
4. Root `TASKS.md` absent plus otherwise-valid hidden PROJECT/STATE/HANDOFF/PROJECT_DASHBOARD -> current-state resolution is unavailable/needs-reconciliation, never hidden-source current truth.
5. Root `TASKS.md` says one current status/actor/next action while a persisted workflow row says conflicting values -> resolver returns TASKS-derived truth only.
6. Ambiguous multiple active TASKS candidates plus one workflow row attempting to disambiguate -> resolver remains NEEDS_RECONCILIATION unless TASKS itself resolves the ambiguity.
7. Existing GitHub-tracked root-TASKS behavior remains green.
8. X04 V02 hidden source exclusion remains green for task-source discovery, custom paths, and audit source evidence.
9. Ordinary implementation source outside `.hiveai` remains auditable.
10. Working-tree, staged, and commit-range audit scope remains exact.
11. X03 readiness persistence/historical wording regressions remain green.
12. M16T structured-output/freeform/task semantic-contract regressions remain green.
13. Exact eight-project portfolio and `Sekiph82/FormuLab@main` regressions remain green.
14. No registered project repository is mutated by resolution or audit collection.

Do not consume real Codex quota in deterministic tests.

## REQUIRED CONTEXT

Read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V02_STRICT_AUDIT.md`
- X03 strict audit / owner-native acceptance
- X04 V02 prompt/log
- `src-tauri/src/control_plane.rs`
- `src-tauri/src/project_dashboard.rs`
- `src-tauri/src/audit_engine.rs`
- `src-tauri/src/task_sources.rs`
- relevant task-intelligence parser and current-state tests.

Use registered external projects only as read-only regression examples. Do not modify Bulk-Edit or any other project repository.

## REGRESSION / SECURITY GATES

Run and record at minimum:

- focused ProjectTruthResolver tests;
- focused Project Dashboard authority tests;
- focused X04 audit/task-source tests;
- focused X03/M16 frontend tests;
- existing M16T focused Rust tests;
- full serialized Rust library regression;
- full frontend Vitest regression;
- `npm run typecheck`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `npm run build`;
- `git diff --check`;
- active-source scan proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses audit transport, Codex auth-file inspection, GUI automation, or M18 implementation;
- governed native QA publication through the existing production `--no-bundle` path.

Builder aggregate test counts are claims pending independent review.

## REQUIRED BUILDER LOG

Create only:

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V03_LOG.md`

Record:

- starting synchronized SHA;
- implementation/test commit SHA(s);
- exact changed files;
- exact closure evidence for F-X04-V02-001 and F-X04-V02-002;
- direct test names for ordinary local TASKS-only, conflicting dashboard, missing TASKS, conflicting workflow row, and ambiguous-TASKS cases;
- regression/security/publication claims;
- published executable SHA-256;
- explicit statement that `TASKS.md` and `CODEX_ROADMAP.md` were not modified;
- explicit statement that no registered external project repository was modified;
- **actual final values** for local HEAD, `origin/main`, live GitHub `main`, and clean-worktree status after the log commit is pushed.

Do not merely say final equality will be checked later. Record the actual final equality evidence.

## FINAL COMPLETION CONTRACT

Commit and push every H!veAI implementation/test/log change before completion.

After the log commit is pushed, run:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

All three main SHAs must be identical and `git status --short` must be empty. Record those actual values in the builder log.

Final owner-facing response must contain only:

- GitHub URL/path for the X04 V03 builder log;
- implementation/test commit SHA(s);
- log commit SHA;
- final GitHub main SHA;
- concise status.

Do not update canonical tracker files. Do not activate M18.