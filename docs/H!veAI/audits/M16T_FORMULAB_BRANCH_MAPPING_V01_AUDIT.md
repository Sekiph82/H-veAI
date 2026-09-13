# M16T FormuLab Canonical Branch Mapping V01 Audit

## VERDICT

**CHANGES_REQUIRED**

A new independently reproducible production tracking defect exists in the current H!veAI source.

`src-tauri/src/github_tracking.rs::ensure_portfolio` still configures the FormuLab portfolio target as:

`Sekiph82/FormuLab @ feature/laboratory-stability`

The live `Sekiph82/FormuLab` repository has canonical/default branch `main`, and the current `main` branch is active. H!veAI must therefore track FormuLab on `main`.

This finding is independent from the Codex audit-provider runtime. The accepted Codex-only provider architecture remains unchanged.

## EVIDENCE

### H!veAI source truth

Current `src-tauri/src/github_tracking.rs` contains:

```rust
(
    "formulab",
    "FormuLab",
    "Sekiph82/FormuLab",
    "feature/laboratory-stability",
),
```

### FormuLab repository truth

GitHub repository metadata for `Sekiph82/FormuLab` reports default branch:

`main`

The live `main` branch currently resolves successfully and is therefore a valid canonical tracking target.

## IMPACT

The H!veAI eight-project portfolio can observe the wrong FormuLab branch, producing stale or branch-specific task/current-state truth in Command Center, Project Cockpit, task counts, next action, and audit targeting.

Because `ensure_portfolio` also persists/updates repository branch metadata, leaving this mapping stale can keep normal runtime tracking pointed at the historical feature branch even though the repository's canonical branch has moved to `main`.

## REQUIRED FIX

In active production source change exactly the FormuLab target branch:

```rust
"feature/laboratory-stability",
```

To:

```rust
"main",
```

Then update focused tests/fixtures/current prospective tracker text that intentionally encode the active FormuLab target branch. Do not rewrite immutable historical prompts/logs/audits merely because they mention the historical branch.

Add/adjust a direct regression test proving the eight-target portfolio maps FormuLab to `main` and that reconciliation updates an existing FormuLab project/repository record from the historical branch to `main` without creating a duplicate project.

## SCOPE BOUNDARY

- Repository to modify: `Sekiph82/H-veAI` only.
- Do not modify `Sekiph82/FormuLab` as part of this remediation.
- Do not activate M17 or implement Claude.
- Do not alter the accepted Codex-only audit provider runtime except if required to keep tests compiling; no provider behavior change is requested.
- Do not introduce OpenAI API-key or direct HTTP audit paths.

## RELATION TO M16T V03

The previously published `M16T_CODEX_ONLY_AUDIT_PROVIDER_V03_PROMPT.md` is superseded before execution. The next builder prompt must close both:

1. the V02 tracker marker/evidence findings; and
2. this FormuLab canonical-branch production mapping defect.
