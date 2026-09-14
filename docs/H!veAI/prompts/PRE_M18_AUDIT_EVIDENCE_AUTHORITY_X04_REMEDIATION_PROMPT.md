# Pre-M18 Audit Evidence Authority Hotfix X04 — Remediation Prompt

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

- Work item: Pre-M18 Audit Evidence Authority Hotfix X04
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed audit: `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_AUDIT.md`
- Required builder log: `docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_LOG.md`
- Findings to close: F-X04-001 and F-X04-002 only
- X03 remains PASS/CLOSED
- M17 remains PASS/CLOSED
- M18 must remain NOT ACTIVATED

Do not broaden into M18 GitHub Integration.

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
- project-dashboard authority resolver/source classification code
- audit-focused Rust/frontend tests.

Use the current Bulk-Edit governance only as a regression fixture/authority example. Do not modify `Sekiph82/Bulk-Edit`.

## F-X04-001 — PREVENT NON-AUTHORITATIVE LOCAL CONTROL-PLANE PROJECTIONS FROM BECOMING IMPLEMENTATION SOURCE TRUTH

The working-tree source planner currently treats any changed/untracked auditable `.json` or `.md` path as eligible source evidence unless it is secret/path-unsafe.

This allowed local legacy/generated files such as:

- `.hiveai/STATE.json`
- `.hiveai/HANDOFF.md`
- `.hiveai/EVENT_INDEX.json`

into a fresh Bulk-Edit freeform audit as ordinary verified source snippets, even though current Bulk-Edit governance says root `TASKS.md` is the only authoritative project-status tracker and local STATE/HANDOFF/watcher projections never override remote truth.

### Required behavior

1. Add an explicit audit-evidence authority classification step before ordinary source-snippet admission.
2. When current project authority identifies root `TASKS.md` / GitHub-first current truth, known local/generated/superseded `.hiveai` control-plane projection files must not be supplied as equal verified implementation authority merely because they are changed or untracked.
3. At minimum classify the current local projection family correctly, including `STATE.json`, `HANDOFF.md`, and `EVENT_INDEX.json` under `.hiveai/`.
4. Legacy ledger files that current governance explicitly forbids reviving, such as `.hiveai/PROJECT.json`, `.hiveai/TASKS.md`, `.hiveai/RULES.md`, and `.hiveai/EVENTS.jsonl`, must not silently regain authority through the audit source planner.
5. Prefer one of these safe outcomes:
   - exclude them from implementation source snippets with an explicit bounded evidence status/rationale; or
   - include only a bounded provenance marker that clearly states they are historical/non-authoritative context and contains no current-authority claim.
6. Do not delete, rewrite, clean, migrate, or otherwise mutate any registered project file as part of audit collection.
7. Real changed/untracked implementation files outside the excluded historical/control-plane family must remain auditable.
8. Preserve staged/commit-range target-scope semantics and do not silently widen scope.
9. Preserve secret-path exclusions and containment safety.

Do not implement a brittle Bulk-Edit-name special case. The rule must derive from existing project authority/provenance classification and/or an explicit generic generated-control-plane path policy.

## F-X04-002 — GOVERNANCE MUST DOMINATE REMEDIATION GUIDANCE

The audit model must not be encouraged to request creation/revival of a project source that current repository governance explicitly marks superseded/forbidden.

### Required behavior

1. Preserve Project Dashboard governance evidence in the audit input.
2. Make the evidence contract/prompt explicit that current verified governance/authority evidence outranks historical or non-authoritative local projection context.
3. Findings/remediation must not treat excluded/historical control-plane projection content as proof of a current implementation defect.
4. In a root-TASKS/GitHub-first fixture, the audit input must not provide `.hiveai/PROJECT.json` as a required missing current-authority source when current governance says not to revive it.
5. Do not hard-code model conclusions. Fix the evidence contract so the model sees truthful authority.

## DETERMINISTIC TESTS

Add direct tests covering at minimum:

1. root-TASKS/GitHub-first project with untracked `.hiveai/STATE.json`, `.hiveai/HANDOFF.md`, and `.hiveai/EVENT_INDEX.json` -> these are not ordinary VERIFIED implementation source snippets;
2. the same fixture retains Project Dashboard/governance evidence identifying canonical current authority;
3. forbidden legacy `.hiveai/PROJECT.json` does not regain current authority merely by appearing in the working tree;
4. changed/untracked real implementation `.rs`, `.ts`, `.tsx`, `.json`, or `.md` files outside the historical-control-plane family remain eligible;
5. staged and commit-range scope behavior remains exact;
6. secret/path containment exclusions remain intact;
7. no project files are mutated by audit collection;
8. X03 readiness persistence and historical selected-run wording regressions remain green;
9. M16T structured-output/freeform/task semantic-contract regressions remain green;
10. exact eight-project portfolio and `Sekiph82/FormuLab@main` regressions remain green.

Do not consume real Codex quota in deterministic tests.

## REGRESSION / SECURITY GATES

Run and record at minimum:

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

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_LOG.md`

Record:

- starting synchronized SHA;
- implementation/test commit SHA(s);
- exact changed files;
- exact closure evidence for F-X04-001 and F-X04-002;
- deterministic test names/results;
- full regression/security/publication claims;
- published executable SHA-256;
- explicit statement that `TASKS.md` and `CODEX_ROADMAP.md` were not modified;
- explicit statement that no Bulk-Edit file was modified;
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

- GitHub URL/path for the X04 builder log;
- implementation/test commit SHA(s);
- log commit SHA;
- final GitHub main SHA;
- concise status.

Do not update canonical tracker files. Do not activate M18.
