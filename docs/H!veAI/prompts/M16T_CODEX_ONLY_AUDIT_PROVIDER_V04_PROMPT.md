# M16T Codex-Only Audit Provider V04 — Tracker Truth + FormuLab Main-Branch Remediation

## MANDATORY SYNC-FIRST AND GITHUB-FIRST CONTRACT

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

After synchronization, read these current authorities before editing:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V02_AUDIT.md`
- `docs/H!veAI/audits/M16T_FORMULAB_BRANCH_MAPPING_V01_AUDIT.md`
- `docs/H!veAI/prompts/M16T_CODEX_ONLY_AUDIT_PROVIDER_V03_PROMPT.md`
- `src-tauri/src/github_tracking.rs`

All Codex-facing work and the required builder log must be entirely in English.

All H!veAI repository changes made by this task must be committed and pushed before completion is claimed. At the end verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == live remote main` and a clean H!veAI working tree. Otherwise return `SYNC_BLOCKED`.

Owner-facing final response must contain only relevant GitHub H!veAI file URLs/paths, implementation/log commit SHA(s), final GitHub `main` SHA, and concise status.

---

## WORK ITEM

- Work code: `M16T`
- Version: `V04`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V04_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

The previously published V03 prompt is **superseded before execution**. Do not execute V03 separately.

V04 closes exactly three bounded items:

1. reconcile the M16T current task marker so pending independent audit/native acceptance is not falsely counted as validated complete;
2. record the correct V02 implementation SHA in new immutable evidence without rewriting the historical V02 log;
3. change the active FormuLab portfolio tracking branch from historical `feature/laboratory-stability` to canonical `main` and prove reconciliation does not create a duplicate project.

Do not redesign the Codex audit provider runtime. The accepted Codex-only architecture remains binding.

---

# ABSOLUTE ARCHITECTURE GUARDRAIL

H!veAI has no OpenAI API-key/direct-HTTP audit provider.

Do not introduce or use:

- `OPENAI_API_KEY`;
- direct `api.openai.com` audit execution/readiness;
- Responses API transport;
- API-key UI/setup;
- API-key authentication as accepted audit mode;
- ChatGPT desktop GUI automation;
- Claude/M17 implementation;
- hard-coded Codex model overrides where the accepted architecture uses CLI default model selection.

Do not modify `Sekiph82/FormuLab`. This remediation changes H!veAI tracking configuration only.

---

# FINDING V04-F01 — TRACKER MARKER TRUTH

Root `TASKS.md` defines:

- `[x]` = validated complete;
- `[~]` = active/in progress.

The live GitHub tracking parser also counts `[x]` as `TASK_COMPLETE` and includes it in completed task totals.

M16T is not yet validated complete because independent strict audit and owner native Codex acceptance are still pending.

Therefore, while the current top-level status remains:

`IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`

The summary task row for M16T must **not** use `[x]` yet.

Use `[~]` for M16T until independent audit and owner native acceptance pass.

After V04 builder completion, current tracker truth must say:

- Current Milestone: M16
- Current Sprint: M16T-CODEX-ONLY
- Current Task: M16T V04 — tracker truth and FormuLab branch remediation
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Next Task/Action: independent M16T V04 strict audit and owner native Codex acceptance
- Required Actor: `HUMAN`
- M16T summary row: `[~]`, implementation complete, awaiting independent audit/native acceptance
- M16 remains OPEN
- M17 remains NOT ACTIVATED/BLOCKED
- milestone denominator remains 20

Update `CODEX_ROADMAP.md` current/prospective status consistently. Do not rewrite immutable historical artifacts.

---

# FINDING V04-F02 — FORMULAB MUST TRACK CANONICAL MAIN

Current production source in `src-tauri/src/github_tracking.rs::ensure_portfolio` still contains:

```rust
(
    "formulab",
    "FormuLab",
    "Sekiph82/FormuLab",
    "feature/laboratory-stability",
),
```

The canonical/default branch of `Sekiph82/FormuLab` is now `main`.

Change only the active FormuLab target branch to:

```rust
(
    "formulab",
    "FormuLab",
    "Sekiph82/FormuLab",
    "main",
),
```

Search active H!veAI source/tests/current prospective documentation for intentional FormuLab target-branch assumptions and update only those that represent current production truth.

Do not rewrite historical immutable prompts/logs/audits merely because they contain the historical feature branch.

## Required behavioral regression proof

Add or update focused tests proving:

1. the eight-target portfolio contains exactly one FormuLab target;
2. FormuLab target branch is `main`;
3. `ensure_portfolio`/equivalent reconciliation updates an existing FormuLab repository/project whose stored branch is `feature/laboratory-stability` to `main`;
4. the reconciliation preserves the same canonical project identity and does not create a duplicate FormuLab project;
5. total portfolio target count remains exactly 8;
6. no other project target branch changes as part of this fix.

If existing test helpers make direct constant inspection awkward, test the persisted post-reconciliation repository/project rows instead of weakening encapsulation.

---

# FINDING V04-F03 — IMMUTABLE V02 LOG SHA TYPO

Do not edit `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V02_LOG.md`.

The V02 log contains a one-character typo in the implementation SHA.

Incorrect historical log text:

`daa5aeec7f7c3bd15c59a64b378d88b3b683781`

Actual V02 implementation commit:

`daa5aeeec7f7c3bd15c59a64b378d88b3b683781`

Record the correct SHA explicitly in the V04 builder log and state that the V02 immutable artifact is preserved unchanged.

---

# REQUIRED VALIDATION

Run the narrowest focused tests first, then required regressions.

At minimum:

1. focused `github_tracking` portfolio/reconciliation tests covering FormuLab `main` and no duplicate project;
2. existing eight-project portfolio tests;
3. M16T Codex provider focused tests sufficient to prove V04 did not regress the accepted provider path;
4. full frontend suite;
5. relevant/full Rust library suite under repository policy;
6. `npm run typecheck`;
7. `cargo check --manifest-path src-tauri/Cargo.toml`;
8. `npm run build`;
9. `git diff --check`;
10. active-source search proving no accepted OpenAI API-key/direct HTTP audit path has returned;
11. search proving active `src-tauri/src/github_tracking.rs` no longer contains the historical FormuLab target branch;
12. governed native QA publication if required by repository policy because production Rust source changed;
13. stable Desktop shortcut/no-terminal-flash regression under existing publication policy.

Automated tests must not consume a real Codex model turn/quota. Use deterministic tests/mocks.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V04_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- V04 implementation commit SHA(s) known before log publication;
- exact V02 implementation SHA correction: `daa5aeeec7f7c3bd15c59a64b378d88b3b683781`;
- statement that V02 immutable log was not edited;
- exact FormuLab old branch and new branch;
- focused reconciliation/no-duplicate test evidence;
- final eight-project target matrix or concise branch matrix showing FormuLab on `main`;
- tracker final state with M16T `[~]`, HUMAN, awaiting independent audit/native acceptance;
- focused/full test commands and counts;
- typecheck/cargo check/build/diff-check result;
- forbidden-provider search result;
- native publication evidence if publication is run;
- statement that M16 remains OPEN and M17 remains blocked/inactive;
- statement that no Claude implementation and no OpenAI API-key/direct HTTP audit path was introduced.

Do not require the log to contain its own creating commit SHA. After publishing it, verify the log commit and final remote `main` and return those SHAs in the final response.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- M16T summary marker truth is corrected;
- FormuLab production tracking points to `main`;
- no duplicate FormuLab project is created by reconciliation;
- all required tests/gates pass;
- all H!veAI changes are committed and pushed;
- local HEAD, `origin/main`, and live GitHub main are identical;
- M16 remains OPEN pending independent audit + owner native acceptance;
- M17 remains blocked.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must show only:

- GitHub V04 log URL/path;
- implementation commit SHA(s);
- V04 log commit SHA;
- final GitHub `main` SHA;
- concise status.
