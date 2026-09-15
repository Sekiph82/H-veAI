# M18 GitHub Integration V03 — Post-X04 Activated Whole-Milestone Builder Prompt

## AUTHORITY

This prompt supersedes `M18_GITHUB_INTEGRATION_V01_PROMPT.md` and `M18_GITHUB_INTEGRATION_V02_PROMPT.md` for execution.

Execute M18 only after GitHub `main` shows all of the following:

1. M17 PASS/CLOSED;
2. Pre-M18 X03 PASS/CLOSED;
3. Pre-M18 X04 PASS/CLOSED with owner-native acceptance recorded in `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_OWNER_NATIVE_ACCEPTANCE.md`;
4. root `TASKS.md` declares M18 ACTIVE;
5. `CODEX_ROADMAP.md` declares M18 ACTIVE.

If any prerequisite is absent, stop with `M18_NOT_ACTIVATED` and do not modify implementation files.

## MANDATORY SAFE SYNC

Before reading or changing implementation files:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the worktree is clean and strictly behind, update only with:

```powershell
git merge --ff-only origin/main
```

Never reset, rebase, force-push, destructively checkout, automatically stash, run `git clean`, or discard owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

## CANONICAL TRACKER OWNERSHIP

`TASKS.md` and `CODEX_ROADMAP.md` are strictly READ-ONLY for Codex.

Codex must never tick tasks, change Project Status metadata, close/open milestones, activate M19, or author any canonical tracker transition. ChatGPT owns tracker transitions after independent audit and required owner-native acceptance under `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`.

## REQUIRED CONTEXT

Read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`
- `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V01_PROMPT.md` for the detailed M18.01-M18.09 implementation/acceptance scope, excluding all tracker-edit instructions
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`
- `docs/H!veAI/audits/PRE_M18_AUDIT_PROVIDER_ROUTE_TRUTH_X03_OWNER_NATIVE_ACCEPTANCE.md`
- `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V05_STRICT_AUDIT.md`
- `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_OWNER_NATIVE_ACCEPTANCE.md`.

## NON-NEGOTIABLE X04 AUTHORITY BOUNDARY

Preserve the accepted owner decision end to end:

- repository-root `TASKS.md` is the sole current project/task/workflow-status authority for local projects;
- tracked-branch root `TASKS.md` is the sole current project/task/workflow-status authority for GitHub-tracked remote projects;
- `.hiveai/PROJECT.json`, `.hiveai/STATE.json`, `.hiveai/HANDOFF.md`, `.hiveai/EVENT_INDEX.json`, `.hiveai/EVENTS.jsonl`, `.hiveai/RULES.md`, `.hiveai/TASKS.md`, Project Dashboard materialization, persisted workflow rows, watcher projections, audits, tests, permissions, and agent sessions may remain historical/domain evidence but must never select, corroborate, disambiguate, override, or backfill current project/task/workflow truth;
- unavailable/ambiguous TASKS truth must fail closed as unavailable / `NEEDS_RECONCILIATION`, never fall back to a legacy control plane;
- Audit Center must not request creation/restoration/reconciliation of hidden `.hiveai` files as current project authority.

M18 must not weaken this boundary while adding GitHub integration.

## M18 WHOLE-MILESTONE SCOPE

Execute the entire M18.01-M18.09 scope defined by the M18 plan and V01 detailed prompt:

- repository/branch/commit remote reads;
- pull-request metadata/diff/status/comments inspection and permission-gated creation only if safely supported;
- issue reads with explicit relationships only;
- Actions/CI run/job/step/log-summary inspection and human-confirmed retry only where safely supported;
- release/tag reads;
- bounded cache, rate-limit, offline, stale-data behavior;
- local Git Engine versus remote GitHub reconciliation without automatic reset/rebase/checkout/merge/push or local-work destruction;
- least-privilege mutations, explicit human approval, credential/token secrecy and redaction;
- direct deterministic remote/offline/rate-limit/reconciliation/security tests;
- full regression and governed native publication.

Preserve all accepted M00-M17, X03, and X04 behavior, including:

- Codex-only Audit Provider architecture;
- Claude Code CLI adapter architecture;
- exact eight-project portfolio;
- `Sekiph82/FormuLab@main`;
- Registry-scoped repository identity;
- provider credential/auth-file non-inspection;
- no new PAT/API-key setting merely for convenience;
- no M19 implementation or activation.

Use the existing accepted GitHub transport/auth boundary where possible. If a mutation cannot be supported under the available least-privilege boundary, expose it truthfully as unavailable rather than introducing a secret-setting shortcut.

## REQUIRED TEST / SECURITY GATES

Run and record at minimum:

- focused M18 repository/branch/commit tests;
- PR/issue/Actions/release read tests;
- mutation permission/confirmation tests;
- rate-limit/offline/stale-cache tests;
- local/remote divergence/reconciliation tests;
- dirty/ahead/behind/diverged local-work preservation tests;
- X04 TASKS-only authority regressions across Command Center, Project Cockpit, Audit Center, and current attention/queue/KPI truth;
- X03 readiness persistence/historical-audit wording regressions;
- M16 Codex audit-provider regressions;
- M17 Claude adapter regressions;
- exact eight-project portfolio and FormuLab@main regressions;
- full serialized Rust regression;
- full frontend Vitest regression;
- TypeScript typecheck;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- production frontend build;
- `git diff --check`;
- active-source security scans for secret leakage, direct prohibited provider transports, auth-file inspection, blanket permission bypass, destructive Git behavior, and accidental M19 implementation;
- governed native QA publication through the existing production `--no-bundle` path.

Builder test/publication counts remain claims until independently reviewed.

## REQUIRED BUILDER LOG

Create only:

`docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V03_LOG.md`

Record:

- synchronized starting SHA;
- implementation/test commit SHA(s);
- exact changed files;
- evidence for every M18.01-M18.09 package;
- X04 authority-regression evidence;
- exact read/mutation permission boundaries;
- offline/rate-limit/stale-cache behavior;
- local/remote reconciliation safety evidence;
- full regression/security/publication claims;
- published executable SHA-256;
- explicit statement that `TASKS.md` and `CODEX_ROADMAP.md` were not modified;
- explicit statement that no registered external project repository was modified;
- final local HEAD / `origin/main` / live GitHub `main` equality and clean-worktree evidence.

## COMPLETION CONTRACT

Commit and push every H!veAI implementation, test, and builder-log change.

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

- GitHub URL/path for `M18_GITHUB_INTEGRATION_V03_LOG.md`;
- implementation/test commit SHA(s);
- log commit SHA;
- final GitHub main SHA;
- concise status.

Do not edit canonical tracker files. Do not mark M18 PASS/CLOSED. Do not activate M19.
