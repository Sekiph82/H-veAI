# M18 GitHub Integration V02 — Tracker-Read-Only Whole-Milestone Builder Prompt

## DO NOT EXECUTE BEFORE X03 ACCEPTANCE

This prompt supersedes `M18_GITHUB_INTEGRATION_V01_PROMPT.md` for execution.

M18 V01 remains immutable historical planning material, but its instruction allowing Codex to edit `TASKS.md` / `CODEX_ROADMAP.md` is superseded and MUST NOT be followed.

M18 V02 may be executed only after:

1. `PRE_M18_AUDIT_PROVIDER_ROUTE_TRUTH_X03` is independently audited PASS;
2. required owner-native X03 acceptance is recorded;
3. ChatGPT has updated canonical GitHub tracker truth to M18 ACTIVE.

If those prerequisites are not present in GitHub `main`, stop with `M18_NOT_ACTIVATED`.

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

Never reset, rebase, force-push, destructively checkout, automatically stash, run `git clean`, or discard owner work.

Read:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`
- `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V01_PROMPT.md` as the detailed M18 implementation/acceptance scope, EXCLUDING its tracker-transition section
- accepted X03 audit/closure evidence
- accepted M17 final audit/owner acceptance.

## TRACKER OWNERSHIP

Codex MUST NOT modify `TASKS.md` or `CODEX_ROADMAP.md`.

Codex reads canonical tracker truth from GitHub after safe synchronization. ChatGPT owns all milestone/task status transitions after independent audit and owner acceptance.

Do not mark M18 complete and do not activate M19.

## M18 IMPLEMENTATION SCOPE

Execute the entire M18 implementation scope defined by:

- `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`
- M18.01 through M18.09 implementation/security/test requirements in `M18_GITHUB_INTEGRATION_V01_PROMPT.md`.

Preserve all accepted M00-M17 and X03 behavior.

Required core boundaries remain:

- repository/branch/commit remote truth;
- PR inspection and only permission-gated creation when safely supported;
- issue reads and explicit linkage only;
- Actions/CI inspection and only human-confirmed retry when safely supported;
- release/tag reads;
- bounded cache/rate-limit/offline/stale truth;
- local/remote reconciliation without automatic merge/rebase/reset/checkout/push;
- Registry-scoped repository identity;
- secret/token redaction;
- exact eight-project portfolio;
- `Sekiph82/FormuLab@main`;
- M16 Codex audit-provider and M17 Claude adapter regressions;
- M19 NOT ACTIVATED.

Do not introduce a new PAT/API-key setting merely for convenience. Recover and use the existing accepted GitHub transport/auth boundary where possible; otherwise keep unsupported mutations truthfully unavailable.

## REQUIRED BUILDER LOG

Create:

`docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V02_LOG.md`

Record implementation/test SHAs, exact files changed, M18.01-M18.09 evidence, regression/security/publication claims, and explicit proof that Codex did not modify `TASKS.md` or `CODEX_ROADMAP.md`.

## COMPLETION CONTRACT

Commit and push all implementation/log changes.

Before reporting completion:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Local HEAD, `origin/main`, and live GitHub `main` must match and the worktree must be clean.

Final owner-facing response must contain only:

- GitHub M18 V02 builder-log URL/path;
- implementation/test commit SHA(s);
- log commit SHA;
- final GitHub main SHA;
- concise status.

Do not edit canonical tracker files. Do not activate M19.
