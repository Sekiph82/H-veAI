# M16T Codex-Only Audit Provider V06 Strict Audit

## 1. VERDICT

**FAIL / CHANGES_REQUIRED**

The V06 implementation closes the three owner-native production blockers that triggered the remediation: the readiness commands are now present in the bounded Tauri ACL, degraded/MALFORMED model output no longer persists as `COMPLETED`, and the project/freeform Codex output contract now explicitly requires the canonical `project-audit / NOT_APPLICABLE` coverage shape. However, one explicit V06 frontend acceptance criterion remains unimplemented: audit-history rows still do not distinguish failed/degraded model runs from valid completed runs. Because this was a required V06 behavior, V06 cannot receive unconditional PASS yet.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

## 2. CONTRACT RECOVERY

V06 was required to remediate the failed owner-native acceptance by:

1. allowing both provider-readiness commands through the existing bounded `allow-audit-engine` ACL;
2. preventing invoke/ACL failure from being presented as `CODEX_NOT_FOUND`;
3. allowing `COMPLETED` only for semantically authoritative `AVAILABLE` model results, with freshness `STALE` taking precedence;
4. persisting degraded history safely as non-completed state and surfacing exact diagnostics;
5. making the zero-task/freeform Codex contract deterministic with exactly one `project-audit / NOT_APPLICABLE` coverage row;
6. keeping invalid freeform output rejected rather than weakening semantic validation;
7. making degraded/failed audit-history rows visibly distinguishable from valid completed runs;
8. preserving Codex-only architecture, V05 FormuLab `main`, exact eight-project portfolio, and M17 block.

## 3. BRANCH / HEAD / DIFF SCOPE

Repository: `Sekiph82/H-veAI`

Branch: `main`

V06 prompt/start SHA: `f6d10d3710fb550683e99d4864bc74bdf588d636`

Implementation commit: `657dabc55c6c56efd1e0b37fd70282d3c07341f6`

Builder log commit / audited live main before this audit: `07eef1990139c2488d45fbd12470491a101f87b5`

Implementation scope includes tracker/roadmap, `src-tauri/permissions/foundation.toml`, audit engine/runtime-facing semantics, Audit Center/Settings UI, and focused tests.

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Evidence |
| --- | --- | --- |
| Readiness commands allowed by bounded ACL | PASS | `allow-audit-engine` contains both readiness commands. |
| No broad shell/filesystem permission expansion | PASS | Existing permission boundary retained. |
| Invoke failure not automatically shown as CODEX_NOT_FOUND | PASS | Settings fallback now uses `UNAVAILABLE` on message/error and `CHECKING` before result. |
| AVAILABLE semantic result may be COMPLETED | PASS | `state_for_evaluation` maps AVAILABLE to COMPLETED. |
| MALFORMED/degraded result cannot be COMPLETED | PASS | Non-AVAILABLE maps to FAILED unless freshness is STALE. |
| STALE freshness dominates | PASS | stale branch is evaluated before normal state selection. |
| Exact diagnostic visible for degraded runs | PASS | Audit Center renders non-AVAILABLE model status and persisted diagnostic. |
| Freeform project contract explicitly requires project-audit/N/A | PASS | `audit_output_contract` encodes exact zero-requirement rule. |
| Freeform semantic validator remains strict | PASS | exact one-row, project-audit, N/A, empty evidenceRefs enforced. |
| Degraded run history remains selectable | PASS | immutable history/list behavior retained. |
| Audit-history row visually distinguishes failed/degraded vs completed/valid | **FAIL** | History row still renders verdict + task + HEAD only; no state or model-status badge/label. |
| OpenAI API-key/direct HTTP path absent | PASS | No V06 reintroduction found in changed production scope. |
| M17/Claude not activated | PASS | Tracker remains M16 OPEN / M17 blocked. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

The builder log accurately describes the major backend/ACL/freeform fixes. Reported test counts remain builder claims because no independent CI status is attached to the implementation commit. Source and committed test bodies independently support the principal fixes.

The log states that the UI keeps failed history selectable, which is true, but it does not call out that the V06 prompt separately required audit-history rows themselves to make degraded/failed runs distinguishable from valid completed runs. The committed history row still omits `state` and `modelStatus`.

## 6. FILE / SYMBOL EVIDENCE

### ACL

`src-tauri/permissions/foundation.toml` now includes:

- `hiveai_audit_provider_readiness`
- `hiveai_audit_provider_check_readiness`

inside existing `allow-audit-engine`.

### Run-state truth

`src-tauri/src/audit_engine.rs::state_for_evaluation` maps:

- stale -> `STALE`;
- `model_status == AVAILABLE` -> `COMPLETED`;
- every other model status -> `FAILED`.

`run_with_model` preserves freshness dominance.

### Freeform semantic contract

`audit_output_contract` explicitly instructs zero-requirement audits to return exactly one `project-audit / NOT_APPLICABLE` coverage row with empty evidence refs. `validate_semantic_evaluation` independently enforces that same shape.

### UI degraded presentation

`src/AuditCenterPage.tsx` now renders model status and diagnostic for non-AVAILABLE runs and avoids the previous success notice for degraded output.

### Residual defect

The audit-history row remains effectively:

`verdict badge + task/project label + HEAD`

with no `audit.state` or `audit.modelStatus` rendering. A `CONDITIONAL / FAILED / MALFORMED` run and a `CONDITIONAL / COMPLETED / AVAILABLE` run therefore remain visually ambiguous in the history list until selected.

## 7. FOCUSED TEST EVIDENCE

Committed V06 test changes cover:

- ACL/readiness rejection mapping;
- degraded state persistence;
- freeform semantic contract;
- degraded detail-panel diagnostics.

No direct history-row test proves that state/model status is visible in each row. The missing test aligns with the missing implementation.

## 8. REGRESSION EVIDENCE

The V06 diff does not alter the accepted FormuLab branch-migration implementation or Codex process transport. Builder reports full Rust/frontend regressions and governed publication PASS; those counts are treated as builder evidence rather than independent CI proof.

## 9. SECURITY / SAFETY REVIEW

PASS for the audited scope. The ACL expansion is narrowly limited to two already-registered audit-readiness commands. No broad shell/filesystem permission, API-key provider, direct OpenAI HTTP audit transport, auth-file inspection, or Claude implementation was introduced.

## 10. ARCHITECTURE CONSISTENCY

The implementation remains consistent with the accepted Codex-only architecture and CLI-managed ChatGPT authentication boundary. The residual history-row issue is presentation truthfulness, not an architecture failure.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

`TASKS.md` and `CODEX_ROADMAP.md` correctly keep M16 OPEN, M16T active/pending independent audit + owner re-acceptance, Required Actor HUMAN after builder completion, and M17 blocked. Because this audit found required remediation, current prospective truth must next move to V07 remediation rather than native re-acceptance.

## 12. FINAL REPOSITORY STATE

Before this audit artifact, live `main` was `07eef1990139c2488d45fbd12470491a101f87b5`, whose parent is implementation commit `657dabc55c6c56efd1e0b37fd70282d3c07341f6`. The V06 builder log exists at the required canonical path.

## 13. OPEN CROSS-MILESTONE FINDINGS

No new cross-milestone production defect was found. V05 FormuLab `main` tracking remains accepted. M17 remains blocked solely by M16 closure governance.

## 14. DEFECTS BY SEVERITY

### M16T-V06-F04 — MAJOR — Audit history hides run validity state

**Location:** `src/AuditCenterPage.tsx`, audit-history row rendering.

**Current behavior:** history rows show verdict, target label, and HEAD only.

**Required behavior:** each history row must make run validity immediately distinguishable, at minimum by showing `state` and `modelStatus` (or an equally explicit combined degraded/valid indicator). A failed/malformed audit must not visually resemble a completed/available audit with the same verdict.

**Impact:** the owner cannot reliably distinguish authoritative and degraded audit runs from the immutable-history list without opening each run, contradicting the V06 truthfulness requirement.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Non-blocking: Settings currently retains previous readiness metadata if a later explicit readiness invoke fails. A future refinement could mark prior metadata stale while showing the invoke error, but this is not required for V06-F04 closure because the ACL defect is now fixed and the UI no longer fabricates CODEX_NOT_FOUND.

## 16. UNVERIFIED ITEMS

- Real owner-native Codex readiness after V06 publication remains unverified until the next native acceptance attempt.
- Real owner-native freeform audit producing `AVAILABLE + COMPLETED` remains unverified until owner re-acceptance.
- Builder-reported full test counts/native smoke are not independently re-executed by this GitHub-only audit.

## 17. REGRESSION RISK

**LOW** for the required V07 fix. The residual change is bounded to history-row presentation and focused frontend tests. Backend/runtime/ACL/freeform semantic logic should remain untouched.

## 18. AUDIT CONFIDENCE

**HIGH.** The residual requirement is explicit in the V06 prompt, and the production JSX directly shows that state/model status are absent from the history row.

## 19. FINAL VERDICT

**FAIL / CHANGES_REQUIRED.**

V06 closes the original native blockers but misses one explicit truthfulness criterion. Do not perform final owner native M16 acceptance yet. Apply one narrow V07 UI remediation, independently audit it, then repeat Settings readiness + real Codex freeform audit acceptance.

## 20. REQUIRED REMEDIATION

Create a narrow V07 remediation that:

1. changes only the audit-history presentation and directly related frontend tests/tracker/log unless a directly reproducible regression requires more;
2. shows each audit history row's `state` and `modelStatus` clearly enough to distinguish `FAILED/MALFORMED`, `FAILED/UNAVAILABLE`, `STALE/*`, and `COMPLETED/AVAILABLE` cases;
3. preserves verdict display without treating verdict alone as execution validity;
4. adds direct tests with at least one valid completed run and one degraded failed run sharing the same verdict, proving the rows are distinguishable;
5. preserves V06 backend, ACL, freeform semantic contract, V05 FormuLab behavior, Codex-only architecture, and M17 block;
6. publishes the normal governed native build because the user-facing Audit Center changed;
7. leaves M16 OPEN pending independent V07 audit and final owner native re-acceptance.
