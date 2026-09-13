# M16T Codex-Only Audit Provider V07 — Audit History Truth Remediation

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

If the working tree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never use reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or any operation that discards owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

After synchronization read at minimum:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V06_STRICT_AUDIT.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V06_NATIVE_ACCEPTANCE_AUDIT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V06_LOG.md`
- `src/AuditCenterPage.tsx`
- `tests/m16-audit-center-focused.test.tsx`

All Codex-facing work and the required builder log must be entirely in English.

All H!veAI repository changes made by this task must be committed and pushed before completion is claimed. At the end verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == live remote main` and a clean H!veAI worktree.

---

## WORK ITEM

- Work code: `M16T`
- Version: `V07`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V07_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

V06 fixed the native readiness ACL, degraded audit-state truth, and project/freeform Codex semantic contract. Preserve those accepted source fixes exactly. V07 closes only the remaining explicit V06 acceptance gap: immutable audit-history rows must visibly distinguish authoritative completed runs from failed/degraded runs.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

---

# ABSOLUTE SCOPE GUARDRAILS

This is a narrow frontend truthfulness remediation.

Do not redesign or modify accepted production backend/runtime behavior unless a new directly reproducible defect makes that unavoidable.

In particular, preserve:

- Codex CLI as the only production model-backed audit provider;
- Codex-managed ChatGPT login;
- no `OPENAI_API_KEY`;
- no direct OpenAI HTTP/Responses audit transport;
- V06 readiness ACL commands and readiness behavior;
- V06 `AVAILABLE -> COMPLETED`, degraded -> `FAILED`, freshness -> `STALE` state semantics;
- V06 project/freeform `project-audit / NOT_APPLICABLE` prompt + semantic contract;
- V05 FormuLab `Sekiph82/FormuLab@main` mapping and branch-aware cache behavior;
- exact eight-project active portfolio;
- M17/Claude blocked and inactive.

Do not activate Claude/M17.

---

# FINDING M16T-V06-F04 — MAJOR — AUDIT HISTORY HIDES RUN VALIDITY STATE

## Current production behavior

`src/AuditCenterPage.tsx` audit-history rows currently show:

- verdict badge;
- task/project label;
- abbreviated HEAD.

They do not show `audit.state` or `audit.modelStatus`.

Therefore these two materially different immutable records can look identical in the history list:

- `CONDITIONAL + COMPLETED + AVAILABLE`
- `CONDITIONAL + FAILED + MALFORMED`

The owner must open each run to discover whether the model result was authoritative. This violates the V06 requirement that failed/degraded history be visibly distinguishable from valid completed history.

## Required target behavior

Each audit-history row must communicate execution/model validity without opening the row.

At minimum render:

1. verdict;
2. audit state;
3. model status;
4. target/task label and/or HEAD as currently useful.

Examples that must be immediately distinguishable:

- `COMPLETED / AVAILABLE`
- `FAILED / MALFORMED`
- `FAILED / UNAVAILABLE`
- `FAILED / AUTH_REQUIRED`
- `STALE / AVAILABLE` or other stale historical state

Do not remove the verdict. Verdict and execution validity answer different questions and both matter.

Use existing badge primitives/styles where practical. Keep the row readable at the accepted desktop layout. Do not expand this into an Audit Center redesign.

---

# REQUIRED FOCUSED TESTS

Update `tests/m16-audit-center-focused.test.tsx` or the nearest existing focused Audit Center test suite.

At minimum prove:

1. a `CONDITIONAL / COMPLETED / AVAILABLE` history row visibly renders `COMPLETED` and `AVAILABLE`;
2. a `CONDITIONAL / FAILED / MALFORMED` history row visibly renders `FAILED` and `MALFORMED`;
3. the two runs remain distinguishable even though their verdict is identical;
4. a degraded historical row remains selectable and its diagnostic detail is shown after selection;
5. a valid completed row remains selectable and does not show degraded diagnostics;
6. existing Audit Center start/re-audit/remediation behavior remains green.

Use deterministic frontend fixtures. Do not consume live Codex quota in automated tests.

---

# REQUIRED REGRESSION GUARDS

Run and record at minimum:

1. focused M16 Audit Center frontend tests;
2. Settings/readiness focused frontend tests sufficient to prove V06 readiness presentation was not regressed;
3. existing V06 audit-engine focused Rust tests sufficient to prove no backend state/semantic regression;
4. V05 portfolio/FormuLab focused tests sufficient to prove no tracking regression;
5. full frontend suite;
6. full Rust library suite under repository policy;
7. `npm run typecheck`;
8. `cargo check --manifest-path src-tauri/Cargo.toml`;
9. `npm run build`;
10. `git diff --check`;
11. active-source guardrail search proving no `OPENAI_API_KEY`, direct OpenAI HTTP audit transport, Responses API audit transport, or API-key fallback has returned;
12. governed native QA publication because a user-visible native Audit Center screen changed;
13. stable Desktop shortcut target/icon and no-terminal-flash regression under existing publication policy.

Builder test output remains builder evidence, not independent acceptance.

---

# TRACKER GOVERNANCE

At implementation start, move current/prospective truth to V07 remediation with Required Actor `CODEX`.

After successful builder completion, set:

- Current Milestone: `M16`
- Current Sprint: `M16T-CODEX-ONLY`
- Current Task: `M16T V07 — Audit history truth remediation`
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_REACCEPTANCE`
- Next Task/Action: independent M16T V07 strict audit, then owner native Settings readiness + real Codex audit re-acceptance
- Required Actor: `HUMAN`
- M16T summary marker: `[~]`
- M16: `OPEN`
- M17: `NOT ACTIVATED / BLOCKED`
- strict milestone progress remains `16 / 20 = 80%`

Historical prompts/logs/audits remain immutable.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V07_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- implementation commit SHA(s) known before log publication;
- exact Audit Center history-row UI change;
- focused test evidence demonstrating same-verdict valid/degraded runs are visually distinguishable;
- proof degraded history remains selectable and diagnostic remains visible;
- proof valid completed history remains selectable without degraded warning;
- regression commands/counts;
- statement that V06 backend/runtime/ACL/freeform contract was not redesigned;
- V05 FormuLab/eight-project preservation evidence;
- provider guardrail search result;
- governed native publication executable SHA-256 and shortcut target/icon evidence;
- tracker final state;
- explicit statement that no OpenAI API-key/direct HTTP provider and no Claude/M17 implementation was introduced.

Do not require the log to contain the SHA of the commit that first creates itself. After publishing the log, verify final remote `main` and return the log commit SHA in the final response.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- audit-history rows visibly distinguish execution state and model status;
- same-verdict completed/available and failed/malformed fixtures are directly proven distinguishable;
- degraded row diagnostics and valid row behavior remain correct;
- V06 backend/ACL/freeform semantic behavior remains green;
- V05 FormuLab/eight-project behavior remains green;
- Codex-only/no-API-key architecture remains intact;
- all required tests/build/publication gates pass;
- every H!veAI change is committed and pushed;
- local HEAD == origin/main == live GitHub main;
- worktree is clean;
- M16 remains OPEN pending independent V07 audit + final owner native re-acceptance;
- M17 remains blocked.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must contain only:

- GitHub V07 log URL/path;
- implementation commit SHA(s);
- V07 log commit SHA;
- final GitHub main SHA;
- concise status.
