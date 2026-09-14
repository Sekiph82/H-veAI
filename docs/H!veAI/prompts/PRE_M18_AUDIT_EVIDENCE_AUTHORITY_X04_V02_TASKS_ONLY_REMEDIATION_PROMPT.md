# Pre-M18 Audit Evidence Authority Hotfix X04 V02 — TASKS-Only Authority Remediation Prompt

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

- Work item: Pre-M18 Audit Evidence Authority Hotfix X04 V02
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Supersedes before execution: `docs/H!veAI/prompts/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_REMEDIATION_PROMPT.md`
- Authoritative failed audit: `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_AUDIT.md`
- Required builder log: `docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V02_LOG.md`
- Findings to close: F-X04-001 and F-X04-002 only
- X03 remains PASS/CLOSED
- M17 remains PASS/CLOSED
- M18 must remain NOT ACTIVATED

Do not broaden into M18 GitHub Integration.

## OWNER DECISION — ROOT TASKS.md IS THE ONLY CURRENT PROJECT-STATE AUTHORITY

This is the primary requirement of X04 V02.

For H!veAI current project tracking/state resolution, the repository-root `TASKS.md` is the **only** current project/task/workflow-status authority.

The following files must **never** be used to determine, corroborate, override, reconcile, infer, or repair current project/task/workflow status:

- `.hiveai/STATE.json`
- `.hiveai/HANDOFF.md`
- `.hiveai/EVENT_INDEX.json`
- `.hiveai/PROJECT.json`
- `.hiveai/TASKS.md`
- `.hiveai/RULES.md`
- `.hiveai/EVENTS.jsonl`
- any equivalent hidden/local/generated control-plane ledger or projection

This rule applies even if such files exist, are changed, are untracked, are internally consistent, are newer than `TASKS.md`, or were produced by an older H!veAI architecture.

Do not revive, generate, rewrite, migrate, synchronize, or require any of these files for current-state truth.

Git repository metadata such as repository identity, branch, HEAD, diff, changed files, remote state, and commit evidence may still be used for Git/repository evidence. Real implementation source files may still be audited. The TASKS-only rule is specifically about **current project/task/workflow tracking authority**, not about suppressing legitimate source-code or Git evidence.

## F-X04-001 — REMOVE HIDDEN CONTROL-PLANE FILES FROM CURRENT-STATE AND AUDIT-AUTHORITY INPUTS

### Required behavior

1. Make root `TASKS.md` the sole current project/task/workflow-status authority in the relevant H!veAI resolution path.
2. Do not consult `.hiveai/STATE.json`, `.hiveai/HANDOFF.md`, `.hiveai/EVENT_INDEX.json`, `.hiveai/PROJECT.json`, `.hiveai/TASKS.md`, `.hiveai/RULES.md`, `.hiveai/EVENTS.jsonl`, or equivalent generated ledgers as current-state authority.
3. For freeform/task audits, those hidden control-plane files must not be emitted as ordinary VERIFIED implementation/current-authority source snippets merely because they are changed or untracked.
4. The safest expected behavior is to exclude this hidden control-plane family from audit implementation-source evidence entirely. If a bounded historical marker is retained for diagnostics, it must be explicitly non-authoritative and must not expose its contents as evidence of current project state.
5. Root `TASKS.md` may be supplied as the project-state/tracker authority evidence when relevant to the audit contract.
6. Real changed/untracked implementation files outside the hidden control-plane family remain auditable.
7. Preserve exact working-tree/staged/commit-range Git scope semantics.
8. Preserve secret-path exclusion and containment safety.
9. Do not mutate any registered project while collecting audit evidence.
10. Do not implement repository-name-specific behavior. This is a general H!veAI authority rule.

## F-X04-002 — AUDIT PROMPT AND REMEDIATION GUIDANCE MUST RESPECT TASKS-ONLY AUTHORITY

### Required behavior

1. Update the audit evidence/prompt contract so the model is explicitly told that root `TASKS.md` is the sole current project/task/workflow-status authority.
2. The model must not be asked or encouraged to resolve project-state conflicts against hidden `.hiveai` control-plane projections.
3. The model must never recommend creating/reviving `.hiveai/PROJECT.json`, `STATE.json`, `HANDOFF.md`, `EVENT_INDEX.json`, `.hiveai/TASKS.md`, `RULES.md`, or `EVENTS.jsonl` to establish current project truth.
4. Findings must not use excluded hidden control-plane content as proof of a current project-state defect.
5. A historical file's existence must not create a missing-authority requirement or reconciliation finding.
6. Do not hard-code a PASS verdict. Fix the authority/evidence contract and let the model evaluate legitimate implementation/Git/test evidence normally.

## IMPORTANT ARCHITECTURE BOUNDARY

Do **not** interpret “TASKS.md only” as “the audit may only inspect TASKS.md.”

The audit engine must continue to inspect legitimate evidence such as:

- Git diff and changed-file identity;
- relevant implementation source snippets;
- test evidence/results where available;
- architecture/governance constraints where required to interpret implementation behavior;
- persisted audit provenance.

However, none of those sources may replace root `TASKS.md` as the source of **current project/task/workflow tracking status**.

## DETERMINISTIC TESTS

Add direct tests covering at minimum:

1. root `TASKS.md` plus conflicting `.hiveai/STATE.json` -> current state follows root `TASKS.md` only;
2. root `TASKS.md` plus conflicting `.hiveai/HANDOFF.md` -> current state follows root `TASKS.md` only;
3. root `TASKS.md` plus conflicting `.hiveai/EVENT_INDEX.json` -> current state follows root `TASKS.md` only;
4. root `TASKS.md` plus conflicting `.hiveai/PROJECT.json` -> current state follows root `TASKS.md` only and PROJECT.json is not required;
5. forbidden legacy `.hiveai/TASKS.md`, `.hiveai/RULES.md`, and `.hiveai/EVENTS.jsonl` do not become current authority;
6. changed/untracked hidden control-plane files are not ordinary VERIFIED implementation/current-authority source snippets in a freeform audit;
7. changed/untracked real implementation `.rs`, `.ts`, `.tsx`, ordinary `.json`, and `.md` files outside the hidden control-plane family remain auditable;
8. staged and commit-range scope remain exact;
9. project files are not mutated by audit collection;
10. X03 readiness persistence and historical selected-run wording remain green;
11. M16T structured-output/freeform/task semantic-contract regressions remain green;
12. exact eight-project portfolio and `Sekiph82/FormuLab@main` regressions remain green.

Include at least one adversarial fixture where every hidden control-plane file disagrees with root `TASKS.md`; the resolved current project/task/workflow status must still be derived only from root `TASKS.md`.

Do not consume real Codex quota in deterministic tests.

## REQUIRED CONTEXT

Read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- X03 strict audit/addendum/owner-native acceptance
- `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_AUDIT.md`
- `src-tauri/src/audit_engine.rs`
- current project-state/task authority resolver paths
- audit-focused Rust/frontend tests.

Use Bulk-Edit only as a read-only regression example. Do not modify `Sekiph82/Bulk-Edit` or any other registered project.

## REGRESSION / SECURITY GATES

Run and record at minimum:

- focused project-state authority tests;
- focused Rust audit-source authority tests;
- focused M16/X03 frontend tests;
- existing M16T focused Rust tests;
- full serialized Rust library regression;
- full frontend Vitest regression;
- `npm run typecheck`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `npm run build`;
- `git diff --check`;
- active-source scan proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses transport, Codex auth-file inspection, GUI automation, or M18 implementation;
- governed native QA publication through the existing production `--no-bundle` path.

Builder test counts are claims pending independent review.

## REQUIRED BUILDER LOG

Create only:

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V02_LOG.md`

Record:

- starting synchronized SHA;
- implementation/test commit SHA(s);
- exact changed files;
- exact closure evidence for F-X04-001 and F-X04-002;
- exact evidence that root `TASKS.md` is the sole current project/task/workflow-status authority;
- adversarial conflicting-hidden-files test names/results;
- full regression/security/publication claims;
- published executable SHA-256;
- explicit statement that `TASKS.md` and `CODEX_ROADMAP.md` were not modified;
- explicit statement that no registered project repository was modified;
- final local HEAD / `origin/main` / live GitHub `main` equality and clean-worktree evidence.

## FINAL COMPLETION CONTRACT

Commit and push every H!veAI change before completion.

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

- GitHub URL/path for the X04 V02 builder log;
- implementation/test commit SHA(s);
- log commit SHA;
- final GitHub main SHA;
- concise status.

Do not update canonical tracker files. Do not activate M18.
