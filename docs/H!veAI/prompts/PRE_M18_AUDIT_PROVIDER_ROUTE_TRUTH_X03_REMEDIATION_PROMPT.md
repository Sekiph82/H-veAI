# Pre-M18 Native Audit Provider Route-Truth Hotfix X03 — Remediation Prompt

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

Then read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/audits/PRE_M18_AUDIT_PROVIDER_ROUTE_TRUTH_X03_AUDIT.md`
- accepted M16T V10/V11 strict audits and owner-native acceptance
- current `src-tauri/src/audit_engine.rs`
- current Tauri audit-provider command/state wiring
- `src/auditEngine.ts`
- `src/pages.tsx` Settings/AuditProviderSettings
- `src/AuditCenterPage.tsx`
- focused M16/audit frontend and Rust tests.

Every Codex-facing artifact and builder log must be entirely in English.

## CANONICAL TRACKER FILES ARE READ-ONLY FOR CODEX

This is mandatory.

Codex MUST NOT edit `TASKS.md` or `CODEX_ROADMAP.md`.

Do not tick tasks, close/open milestones, change Current Task, change Required Actor, change progress percentages, or activate M18. Canonical tracker transitions are owned by ChatGPT after independent audit/owner acceptance.

Codex only synchronizes its local checkout to the GitHub tracker state through the safe sync-first contract.

If source changes require a future tracker transition, record the proposed transition in the builder log only. Do not edit canonical tracker files.

## WORK ITEM

- Work item: Pre-M18 Native Audit Provider Route-Truth Hotfix X03
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed audit: `docs/H!veAI/audits/PRE_M18_AUDIT_PROVIDER_ROUTE_TRUTH_X03_AUDIT.md`
- Required builder log: `docs/H!veAI/codex-logs/PRE_M18_AUDIT_PROVIDER_ROUTE_TRUTH_X03_LOG.md`
- Scope: close F-X03-001 and F-X03-002 only
- M17 remains PASS/CLOSED
- M18 must remain NOT ACTIVATED

Do not broaden into M18 GitHub Integration.

---

# F-X03-001 — PERSIST THE LAST EXPLICITLY VERIFIED AUDIT-PROVIDER READINESS ACROSS ROUTES

Current Settings behavior is route-local:

- `AuditProviderSettings` owns `readiness` in component state;
- on every mount it calls `getAuditProviderReadiness()`;
- an explicit `checkAuditProviderReadiness()` can return `READY`;
- navigating away unmounts the component;
- returning remounts it and replaces the verified state with the weaker baseline result, commonly `AUTH_UNVERIFIED`.

Implement one governed current-readiness truth that survives React route remounts inside the same native H!veAI process.

## Required behavior

1. After an explicit end-to-end readiness check returns READY, leaving Settings and returning during the same native process must still show that verified READY result without requiring another expensive check.
2. Explicit verified failure states such as `USAGE_LIMITED`, `SCHEMA_INCOMPATIBLE`, `MALFORMED`, `AUTH_REQUIRED`, `NETWORK_ERROR`, `TIMEOUT`, and `PROCESS_ERROR` must likewise remain the current verified truth until superseded or invalidated.
3. Application restart may reset explicit verification and fall back to truthful baseline readiness such as AUTH_UNVERIFIED.
4. A materially changed Codex executable/version/path must invalidate a cached verified result or require fresh verification before presenting READY.
5. The explicit production-equivalent structured-output readiness probe remains unchanged and authoritative.
6. Do not cache credentials, tokens, auth files, prompt output containing secrets, or raw provider streams.
7. Do not introduce `OPENAI_API_KEY`, direct OpenAI HTTP/Responses transport, auth-file inspection, or GUI automation.

## Preferred design

Prefer a backend/process-scoped readiness state owned near the audit provider rather than page-local React state.

A small backend cache/state can retain only the sanitized `AuditProviderReadiness` result plus the executable/version identity and checked-at/invalidation metadata required for truthful reuse.

`hiveai_audit_provider_readiness` should return the latest still-valid explicitly verified result when one exists, otherwise the baseline probe. `hiveai_audit_provider_check_readiness` should run the full production-equivalent probe and update the process-scoped verified result.

If you choose another design, prove that Settings, Audit Center, and audit execution cannot disagree about current provider readiness merely because a route remounted.

---

# F-X03-002 — SEPARATE IMMUTABLE HISTORICAL AUDIT STATUS FROM LIVE PROVIDER READINESS

Current Audit Center selects persisted audit history and labels the right-hand panel `Current verdict`. An old `UNAVAILABLE` run from 2026-09-08 is rendered with present-tense text saying the Codex CLI audit provider is not configured, even when Settings has just verified READY.

The old run must remain immutable. Fix the presentation, not the history.

## Required behavior

1. Rename/contextualize the selected-run panel so it cannot be mistaken for current live provider readiness. Examples: `Selected audit verdict`, `Persisted audit verdict`, or equivalent clear wording.
2. Keep the audit timestamp visibly associated with the selected result.
3. For historical `UNAVAILABLE`, `MALFORMED`, `USAGE_LIMITED`, `SCHEMA_INCOMPATIBLE`, etc., phrase diagnostics as the state of **that audit run**.
4. Never rewrite a historical run because the provider is healthy now.
5. A historical `UNAVAILABLE` run must not display text that asserts the provider **is currently** unconfigured.
6. If Audit Center displays current live provider readiness, use the same governed current-readiness source as Settings and clearly label it as live/current provider state.
7. Fresh audit execution and the persisted model-status/state contract remain unchanged.

A minimal solution may simply fix the selected-run terminology and historical warning language. Adding a live readiness badge is optional unless needed to remove ambiguity.

---

# DETERMINISTIC TESTS

Add direct tests covering at minimum:

1. baseline readiness with no explicit verified cache -> truthful baseline status;
2. explicit check returns READY -> normal readiness getter returns READY afterward in same process;
3. explicit verified non-READY result -> getter retains that verified category;
4. process-state/new-state fixture proves restart semantics reset verification;
5. executable/version identity mismatch invalidates retained READY;
6. Settings remount/navigation fixture: READY -> unmount/remount -> READY remains visible;
7. Audit Center historical `UNAVAILABLE` fixture remains immutable but is labeled as historical selected-run truth;
8. with current live READY plus historical UNAVAILABLE audit, UI does not claim the provider is currently unconfigured;
9. M16T V08-V11 freeform/task schema, failure-classification, representative readiness, and audit-history validity regressions remain green.

Do not consume real Codex quota in deterministic tests.

---

# REGRESSION / SECURITY GATES

Run and record at minimum:

- focused Rust readiness-state tests;
- focused Settings route-remount frontend tests;
- focused Audit Center historical/live-truth frontend tests;
- existing M16T focused Rust/frontend tests;
- full serialized Rust library regression;
- full frontend Vitest regression;
- `npm run typecheck`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `npm run build`;
- `git diff --check`;
- active-source scans proving no `OPENAI_API_KEY`, direct OpenAI HTTP/Responses audit transport, Codex auth-file inspection, GUI automation, or M18 implementation work;
- exact eight-project portfolio regression;
- `Sekiph82/FormuLab@main` regression;
- governed native QA publication through the existing production `--no-bundle` path.

Builder command/test counts are claims for independent review, not acceptance evidence.

---

# REQUIRED BUILDER LOG

Create only:

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_PROVIDER_ROUTE_TRUTH_X03_LOG.md`

Record:

- starting synchronized SHA;
- implementation/test commit SHA(s);
- exact changed files;
- exact fixes for F-X03-001 and F-X03-002;
- deterministic test names/results;
- full regression/security/publication claims;
- published executable SHA-256;
- explicit statement that `TASKS.md` and `CODEX_ROADMAP.md` were not modified by Codex;
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

- GitHub URL/path for the X03 builder log;
- implementation/test commit SHA(s);
- log commit SHA;
- final GitHub main SHA;
- concise status.

Do not update canonical tracker files. Do not activate M18.
