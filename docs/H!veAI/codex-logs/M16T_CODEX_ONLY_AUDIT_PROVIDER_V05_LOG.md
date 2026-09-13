# M16T Codex-only Audit Provider V05 FormuLab Tracking Remediation Log

- Work code: `M16T`
- Version: `V05`
- Date: `2026-09-13`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Synchronized starting GitHub SHA: `3eaa2b345975ce7e38775a4d1d36bfed9a628e90`
- Implementation commit: `bcbc8675ef95ea25da73b3fac1902c0dfc272fb6`

## Scope and preserved decisions

V03 independently passed its tracker/evidence audit and its accepted truth is preserved. The previously published V04 prompt was superseded before execution and was not executed separately. V05 addresses only the independently identified FormuLab branch-mapping defect plus the minimum deterministic regression coverage, current tracker transition, and publication evidence required to prove it.

No changes were made to `Sekiph82/FormuLab`. No accepted Codex audit-provider runtime behavior was redesigned. M16 remains open pending independent V05 audit and owner native acceptance; M17 remains not activated and blocked.

## Finding V05-F01

The production target in `src-tauri/src/github_tracking.rs::ensure_portfolio` previously tracked `Sekiph82/FormuLab` on `feature/laboratory-stability`, while the canonical/default FormuLab branch is `main`. The active target now tracks exactly `Sekiph82/FormuLab @ main`.

When an existing persisted project is found by GitHub identity, `ensure_portfolio` updates its project and repository branch in place. If that repository’s prior default branch differs from the configured branch, it deletes only that project’s `GITHUB_TASKS_REMOTE` cache row. This preserves the project identity, attached local workspace metadata, and unrelated project caches while preventing an old-branch snapshot from surviving as current main-branch truth.

The remote cache reuse predicate is also branch-aware: a snapshot is reusable only when its recorded branch, remote HEAD, health, and materialized task-row shape all match the requested observation. Equal HEAD alone cannot make a `feature/laboratory-stability` snapshot valid for `main`.

## Focused deterministic evidence

- Fresh bootstrap creates exactly eight active portfolio projects.
- Exactly one FormuLab identity exists and its persisted repository default branch is `main`.
- Existing FormuLab data created with `feature/laboratory-stability` is reconciled in place to `main`.
- The migrated project keeps its original project ID and attached local workspace/original/normalized path metadata.
- Reconciliation creates no duplicate FormuLab project and leaves the active portfolio at exactly eight.
- A cached snapshot recorded for `feature/laboratory-stability` is rejected for `main` even when its fetched HEAD is equal; the migration also invalidates the affected project-scoped remote cache.
- The other seven targets remain exactly one each on `main`.
- Focused `github_tracking` tests: `11 passed; 0 failed`.
- Existing Codex runtime tests: `6 passed; 0 failed`.
- Existing audit-engine tests: `34 passed; 0 failed`.

## Final eight-target matrix

| Repository | Tracked branch |
| --- | --- |
| `Sekiph82/H-veAI` | `main` |
| `Sekiph82/Bulk-Edit` | `main` |
| `Sekiph82/fmcg-erp-system` | `main` |
| `Sekiph82/FormuLab` | `main` |
| `Sekiph82/PackLab` | `main` |
| `Sekiph82/PackLab-3D` | `main` |
| `Sekiph82/Scrubbots` | `main` |
| `Sekiph82/ScrubBots-Level-Factory` | `main` |

## Tracker truth

Current/prospective tracker state is V05 and remains pending acceptance:

- Current Milestone: `M16`
- Current Sprint: `M16T-CODEX-ONLY`
- Current Task: `M16T V05 - FormuLab canonical main tracking remediation`
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Next Task/Action: independent M16T V05 strict audit and owner native Codex acceptance
- Required Actor: `HUMAN`
- M16T summary marker: `[~]`, not validated-complete `[x]`
- M16: `OPEN`
- M17: `NOT ACTIVATED / BLOCKED`
- Strict milestone progress: `16 / 20 = 80%`

The roadmap current/prospective status matches these fields. Accepted V03 history and immutable V02/V03 artifacts were not rewritten.

## Regression and boundary gates

- Full Rust library regression: `435 passed; 0 failed; 0 ignored`.
- Frontend regression: `17/17` files and `135/135` tests passed using one worker to avoid existing cross-file timing noise. An initial parallel run had an existing flaky M12/M07 test; the isolated M12 test passed and the serialized full suite passed.
- `npm run typecheck`: PASS.
- `cargo check --manifest-path src-tauri/Cargo.toml`: PASS with existing warnings only.
- `npm run build`: PASS.
- `git diff --check`: PASS.
- Active-source provider guardrail search: PASS. No `OPENAI_API_KEY`, direct OpenAI HTTP audit transport, OpenAI Responses audit transport, API-key authentication, or fallback provider was introduced.
- Active production target inspection: PASS; `ensure_portfolio` configures FormuLab on `main`. The historical branch string exists only in the deterministic migration fixture.
- Accepted Codex runtime diff: none; no `audit_engine.rs` or `codex_runtime.rs` production changes.

No live GitHub HTTP or Codex quota was consumed by tests. No Claude/M17 work was introduced.

## Native governed publication

The existing governed publication helper passed production candidate/stable build and smoke checks, including startup readiness, no visible console host, no forbidden development-port listener, stable shortcut target, and stable shortcut icon. The final published executable was captured as:

- Stable executable: `dev-bin/H!veAI.exe`
- SHA-256: `273EC0EE8AD5BC59A82AC84F7AA100736EE3B4CB19E166E281AE7180AFB6DA5E`
- Desktop shortcut target: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.exe`
- Desktop shortcut icon: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.ico,0`

## Security and final boundary

The only production model-backed audit provider remains the locally installed Codex CLI authenticated through the owner’s existing ChatGPT login. This run did not request, read, set, persist, or use `OPENAI_API_KEY`; did not restore direct OpenAI HTTP audit transport; did not add API-key authentication or fallback; did not inspect Codex auth files/tokens; and did not activate M17 or implement Claude.

All H!veAI changes for V05 were committed and pushed. The V05 log commit itself is intentionally not recorded in this file; final equality proof records it after publication. M16 remains open pending independent V05 audit and owner native Codex acceptance.
