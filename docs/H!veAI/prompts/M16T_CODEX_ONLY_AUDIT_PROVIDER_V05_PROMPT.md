# M16T Codex-Only Audit Provider V05 — FormuLab Canonical Main Tracking Remediation

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
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V03_AUDIT.md`
- `docs/H!veAI/audits/M16T_FORMULAB_BRANCH_MAPPING_V01_AUDIT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V03_LOG.md`
- `src-tauri/src/github_tracking.rs`

The previously published `M16T_CODEX_ONLY_AUDIT_PROVIDER_V04_PROMPT.md` is **superseded before execution**. Do not execute V04 separately and do not reproduce its now-redundant V03 tracker remediation.

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
- Version: `V05`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V05_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

M16T V03 has passed independent strict audit for its tracker/evidence scope. Preserve that accepted truth.

V05 is a narrow production tracking remediation for one still-live defect:

> H!veAI tracks `Sekiph82/FormuLab` on historical branch `feature/laboratory-stability`, while the canonical/default FormuLab branch is now `main`.

M16 remains OPEN until V05 passes independent audit and owner native acceptance. M17 remains blocked and inactive.

---

# ABSOLUTE ARCHITECTURE GUARDRAILS

Preserve the accepted Codex-only audit architecture exactly.

Do not introduce or use:

- `OPENAI_API_KEY`;
- direct OpenAI HTTP audit execution/readiness;
- OpenAI Responses API transport;
- API-key authentication as an accepted audit mode;
- ChatGPT desktop GUI automation;
- Claude/M17 implementation;
- hard-coded Codex model overrides where the accepted architecture uses CLI default model selection.

Do not modify the `Sekiph82/FormuLab` repository. This work item modifies only H!veAI's portfolio tracking configuration and the minimum tests/current tracker state required to prove it.

Do not modify accepted M16T Codex runtime behavior unless a new directly reproducible regression is required to keep this branch-tracking remediation correct.

---

# FINDING V05-F01 — FORMULAB MUST TRACK CANONICAL `main`

Current active H!veAI production source in `src-tauri/src/github_tracking.rs::ensure_portfolio` contains:

```rust
(
    "formulab",
    "FormuLab",
    "Sekiph82/FormuLab",
    "feature/laboratory-stability",
),
```

Independent GitHub repository metadata confirms `Sekiph82/FormuLab` default/canonical branch is `main`.

Change the active target to:

```rust
(
    "formulab",
    "FormuLab",
    "Sekiph82/FormuLab",
    "main",
),
```

Do not change any other portfolio target branch unless direct current repository evidence proves a separate defect. This task is not a general portfolio redesign.

---

# MIGRATION / CACHE CORRECTNESS

The one-line target change must also be safe for an existing H!veAI installation whose persisted FormuLab project/repository row was created while the target branch was `feature/laboratory-stability`.

Prove the following behavior rather than assuming a fresh database:

1. `ensure_portfolio` reconciles the existing FormuLab repository/default branch to `main`;
2. the active portfolio remains exactly 8 projects;
3. there is exactly one active FormuLab project/repository identity after reconciliation;
4. any already-attached FormuLab local workspace metadata is preserved;
5. reconciliation does not create a second FormuLab project merely because the configured branch changed;
6. H!veAI cannot reuse a cached `CURRENT` FormuLab remote snapshot from `feature/laboratory-stability` as if it were a valid `main` snapshot solely because both branches happen to resolve to the same commit SHA.

For item 6, use the narrowest safe design. Acceptable solutions include branch-aware cache reuse or explicit invalidation of the affected project's `GITHUB_TASKS_REMOTE` cache when its tracked branch changes. Do not add broad destructive cache clearing if a project-scoped solution is sufficient.

The resulting remote snapshot/current truth must identify FormuLab branch `main`.

---

# REQUIRED FOCUSED TESTS

Add or update deterministic Rust tests proving at minimum:

1. fresh portfolio bootstrap creates exactly eight active targets;
2. exactly one FormuLab target exists;
3. FormuLab persisted repository/default branch is `main`;
4. an existing FormuLab record with stored branch `feature/laboratory-stability` is reconciled to `main`;
5. the reconciliation does not create a duplicate FormuLab project;
6. an attached local workspace/path survives that branch migration;
7. a cached remote snapshot whose recorded branch is `feature/laboratory-stability` is not reused as a valid `main` snapshot merely because the fetched HEAD SHA is equal;
8. all seven other portfolio repository/branch targets remain unchanged;
9. active portfolio count remains 8 and AI-Commerce-HQ does not return.

Do not make automated tests perform live GitHub HTTP or consume Codex quota. Use deterministic database/snapshot fixtures.

---

# PRESERVE ACCEPTED M16T V03 TRACKER TRUTH

V03 independently passed. Do not regress its semantics.

During V05 implementation, current/prospective tracker truth should transition to:

- Current Milestone: `M16`
- Current Sprint: `M16T-CODEX-ONLY`
- Current Task: `M16T V05 — FormuLab canonical main tracking remediation`
- Current Task Status after builder completion: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Next Task/Action after builder completion: independent M16T V05 strict audit and owner native Codex acceptance
- Required Actor after builder completion: `HUMAN`
- M16T summary row: `[~]`, not `[x]`
- M16: `OPEN`
- M17: `NOT ACTIVATED / BLOCKED`
- strict milestone progress remains `16 / 20 = 80%` until M16 actually closes

Update `CODEX_ROADMAP.md` only as needed to keep current/prospective truth aligned. Do not rewrite historical immutable prompts/logs/audits.

---

# REQUIRED VALIDATION

Run the narrowest focused tests first, then the repository-required regressions.

At minimum:

1. focused `github_tracking` portfolio/branch-migration/cache-identity tests;
2. existing remote-task cache/self-heal tests;
3. existing eight-project portfolio identity/duplicate-merge tests;
4. focused M16T Codex-provider tests sufficient to prove the accepted provider architecture was not regressed;
5. full Rust library suite under repository policy;
6. full frontend suite;
7. `npm run typecheck`;
8. `cargo check --manifest-path src-tauri/Cargo.toml`;
9. `npm run build`;
10. `git diff --check`;
11. active-source search proving no accepted OpenAI API-key/direct HTTP audit path has returned;
12. active-source/current-source check proving the FormuLab production target is `main` and no current target still uses `feature/laboratory-stability`;
13. governed native QA publication because production Rust tracking source changes;
14. stable Desktop shortcut target/icon and no-terminal-flash regression under existing publication policy.

Builder test output remains builder evidence, not independent acceptance. Record exact commands and counts in the V05 log.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V05_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- V05 implementation commit SHA(s) known before log publication;
- statement that V03 independent tracker/evidence audit was accepted and preserved;
- statement that V04 was superseded before execution;
- exact FormuLab old target branch and new target branch;
- exact source symbol/path changed;
- focused fresh-bootstrap and existing-install branch-migration test evidence;
- proof that branch migration creates no duplicate FormuLab project and keeps total active portfolio count at 8;
- proof that attached local workspace metadata survives migration;
- proof that old-branch cached remote truth cannot be reused as valid `main` truth on equal HEAD alone;
- concise final eight-target repository/branch matrix showing FormuLab on `main`;
- tracker final state with M16T `[~]`, Required Actor HUMAN, M16 OPEN, M17 blocked;
- focused/full test commands and counts;
- typecheck/cargo-check/build/diff-check results;
- forbidden-provider search result;
- native publication evidence;
- statement that no Claude/M17 implementation and no OpenAI API-key/direct HTTP audit path was introduced.

Do not require the log to contain the SHA of the commit that first creates itself. After publishing the log, verify the log commit and final remote `main` and return those values in the final response.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- FormuLab active H!veAI tracking target is `main`;
- existing-install migration is proven safe and duplicate-free;
- old-branch cache cannot masquerade as main-branch truth;
- active portfolio remains exactly 8;
- accepted Codex-only provider architecture and V03 tracker truth remain intact;
- all required tests/publication gates pass;
- every H!veAI change is committed and pushed;
- local `HEAD`, `origin/main`, and live GitHub `main` are identical;
- working tree is clean;
- M16 remains OPEN pending independent V05 audit + owner native acceptance;
- M17 remains blocked.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must show only:

- GitHub V05 log URL/path;
- implementation commit SHA(s);
- V05 log commit SHA;
- final GitHub `main` SHA;
- concise status.
