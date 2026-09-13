# M16T Codex-Only Audit Provider V09 — Independent Strict Audit

## 1. VERDICT

**FAIL / CHANGES_REQUIRED**

V09 materially improves Codex CLI failure classification, bounded diagnostics, and the readiness execution path, but two MAJOR truth gaps remain. Production audit execution can still lose the new `SCHEMA_INCOMPATIBLE` category when converting provider failure into persisted audit state, and the readiness probe does not exercise the V08 production schema capabilities strongly enough to justify `READY` for the actual audit contract. M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

## 2. CONTRACT RECOVERY

V09 was required to preserve the accepted Codex-only architecture while making provider truth deterministic: generic CLI `Usage:` help must not become quota exhaustion; explicit quota signals must become `USAGE_LIMITED`; schema/structured-output incompatibility must be distinct and actionable; Settings readiness must feature-probe the production-equivalent structured-output boundary; failed provider turns must retain bounded sanitized diagnostics; V05-V08 behavior must remain intact; no OpenAI API-key/direct HTTP path or Claude/M17 work may be introduced.

## 3. BRANCH / HEAD / DIFF SCOPE

- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Prompt/start SHA: `d3e5d55dba2328e4aa00385c265983502362c086`
- Implementation SHA: `50a4a5a40760c4aa56ec209f6aaf463a2283d5f6`
- Builder-log SHA / audited pre-audit main: `d531c885de68b5a4055af39dea060fe6ee776dc8`
- Implementation scope: `src-tauri/src/audit_engine.rs`, `TASKS.md`, `CODEX_ROADMAP.md`

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Evidence |
|---|---|---|
| Generic `Usage:` does not classify as quota | PASS | Explicit classifier test maps `Usage: codex exec ...` to `PROCESS_ERROR` |
| Explicit quota/rate-limit signals classify as `USAGE_LIMITED` | PASS | Explicit phrase set and direct tests |
| Schema rejection has distinct category | PARTIAL | Classifier/readiness can produce `SCHEMA_INCOMPATIBLE`, but production audit persistence loses that category |
| Auth/network/timeout categories remain distinct | PASS | Classifier hierarchy and existing timeout paths |
| Diagnostics are bounded | PASS | 12-line / 2048-byte process diagnostic bound |
| Diagnostics are sanitized | PASS | `sanitize_text` redacts authorization/token/password/API-key/auth-path style lines |
| Readiness uses `--output-schema` path | PASS | Readiness request now carries a schema through the same process runner |
| Readiness proves production-relevant schema capabilities | FAIL | Probe schema is only `{ready:boolean}` and does not exercise the V08 freeform schema constructs used by real audits |
| Readiness never persists an audit row | PASS | Existing design preserved; readiness is a provider probe only |
| V08 dynamic audit schema remains intact | PASS | `audit_result_schema(input)` remains production audit schema authority |
| V07 history truth remains intact | PASS | No V07 UI/runtime redesign in V09 implementation scope |
| V05 FormuLab/main + 8-project architecture preserved | PASS | No related production changes in V09 scope |
| Codex-only / no API-key architecture preserved | PASS | No prohibited provider transport introduced |
| Owner-native final acceptance | UNVERIFIED | Cannot proceed until residual source findings are closed |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

The builder log claims focused audit-engine tests, V05 tracking tests, 138 frontend tests, 443 Rust tests, typecheck/build/cargo-check/publication PASS, and a governed executable hash. Direct source and test bodies independently corroborate the classifier and basic structured-readiness changes. GitHub exposes no independent status checks for the implementation commit, so execution counts remain builder claims rather than independent CI proof.

The log states schema incompatibility is distinct from quota/usage in backend status. That is true for readiness, but not fully true for persisted production audit results because `unavailable_evaluation` does not map `SCHEMA_INCOMPATIBLE`.

## 6. FILE / SYMBOL EVIDENCE

### Accepted V09 changes

`classify_codex_failure_category` now requires explicit signals. Plain `Usage: codex exec ...` falls through to `PROCESS_ERROR`; quota/rate/429 phrases map to `USAGE_LIMITED`; structured-output rejection phrases map to `SCHEMA_INCOMPATIBLE`.

`diagnostic_source` prefers relevant stderr/error lines, and `sanitize_process_diagnostic` bounds the retained excerpt. `sanitize_text` redacts credential-bearing lines.

`check_codex_readiness_with_runner` now passes a schema to the same `CodexProcessRunner` boundary and requires a schema-conformant dedicated final result.

### Residual defect F-V09-001

`unavailable_evaluation` maps `AUTH_POLICY_BLOCKED`, `AUTH_REQUIRED`, `USAGE_LIMITED`, `TIMEOUT`, `NETWORK_ERROR`, `PROCESS_ERROR`, and `FINAL_OUTPUT`, but has no `SCHEMA_INCOMPATIBLE` branch. A production audit process error such as `AUDIT_CODEX_SCHEMA_INCOMPATIBLE: ...` therefore falls through to persisted `model_status = UNAVAILABLE` even though the diagnostic carries the more specific category.

This violates V09's requirement that schema incompatibility remain distinct and truthful in the audit provider result path.

### Residual defect F-V09-002

`codex_readiness_schema()` is only a one-property boolean object. It proves that some `--output-schema` request can work, but it does not feature-probe the production V08 freeform audit schema features that motivated V09, including nested finding/coverage arrays, nested `additionalProperties:false`, enums, fixed one-row coverage shape, and `maxItems:0` constraints.

Therefore Settings can still return `READY` while the actual V08 `audit_result_schema(input)` is rejected by the installed CLI/provider structured-output subset. That is precisely the ambiguity V09 was intended to remove.

## 7. FOCUSED TEST EVIDENCE

Direct test bodies were inspected. V09 adds good deterministic tests proving:

- `Usage: codex exec ...` -> `PROCESS_ERROR`;
- unsupported output-schema help -> `SCHEMA_INCOMPATIBLE`;
- explicit usage limit/rate/quota -> `USAGE_LIMITED`;
- auth/network categories remain explicit;
- credential-like diagnostics are redacted and bounded;
- readiness request contains a schema;
- plain `READY` is rejected as `SCHEMA_INCOMPATIBLE`;
- explicit schema rejection and explicit quota are separated.

Missing focused proof:

- a production audit provider failure categorized as `SCHEMA_INCOMPATIBLE` round-trips to persisted/evaluated `model_status = SCHEMA_INCOMPATIBLE`;
- a readiness feature probe exercises the same relevant schema constructs required by V08 real audits.

## 8. REGRESSION EVIDENCE

Implementation scope is narrow and V08 schema code remains present. Builder claims V05-V08 focused and full regression gates green. No independent GitHub CI status exists for the implementation commit. No source regression was found outside the two V09 residual truth gaps.

## 9. SECURITY / SAFETY REVIEW

**PASS.** Diagnostics are bounded and sanitized, no auth/token files are read, no API-key fallback or direct OpenAI HTTP transport is introduced, and the Codex process remains bounded, ephemeral, and read-only. The residual findings are truth/compatibility defects, not secret-exposure defects.

## 10. ARCHITECTURE CONSISTENCY

**PARTIAL.** Codex CLI remains the only production model-backed provider and the no-API-key decision is preserved. However the architecture promises evidence/provider truth at the persisted audit boundary; losing `SCHEMA_INCOMPATIBLE` to `UNAVAILABLE` violates that invariant. Readiness also does not yet feature-probe enough of the real audit schema to be called production-equivalent.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

`TASKS.md` and `CODEX_ROADMAP.md` correctly identify V09 as implementation-complete, M16 OPEN, M17 blocked, and HUMAN as the next actor before this independent audit. After this audit the next action is a bounded V10 remediation, not owner final acceptance.

The immutable V09 builder log is mostly accurate but overstates complete schema-category/backend parity because of F-V09-001 and production-equivalent readiness because of F-V09-002.

## 12. FINAL REPOSITORY STATE

At audit start, live `main` is `d531c885de68b5a4055af39dea060fe6ee776dc8`, whose parent is the implementation commit `50a4a5a40760c4aa56ec209f6aaf463a2283d5f6`. The V09 log is present on `main`. Historical artifacts are preserved. GitHub has no combined status checks attached to the implementation commit.

## 13. OPEN CROSS-MILESTONE FINDINGS

- M16 remains OPEN.
- M17/Claude remains blocked.
- V08 owner-native three-success audit gate cannot be resumed until V09 residual truth/compatibility findings are closed.

## 14. DEFECTS BY SEVERITY

### MAJOR — F-V09-001: `SCHEMA_INCOMPATIBLE` is lost in production audit model status

A real audit schema rejection can be diagnosed as `AUDIT_CODEX_SCHEMA_INCOMPATIBLE` but persisted as `modelStatus=UNAVAILABLE` because `unavailable_evaluation` has no matching branch.

### MAJOR — F-V09-002: Readiness schema is not representative of V08 production schema capabilities

The readiness feature probe proves only a trivial boolean object schema. It can return READY even if the real freeform audit schema is rejected for nested/array/bounds features.

### NOTE

GitHub exposes no independent CI status for the implementation commit; builder run counts remain claim evidence.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

A single shared provider-failure category enum/normalizer would reduce future drift between classifier, readiness status, and persisted audit model status. This can be introduced only if it remains a bounded refactor; it is not required if explicit exhaustive mapping is clearer.

## 16. UNVERIFIED ITEMS

- Real owner-native behavior on the V09 governed executable is intentionally not accepted because source defects remain.
- Builder-reported full test counts and executable publication hash were not independently reproduced through GitHub CI.

## 17. REGRESSION RISK

**MEDIUM.** Classification itself is improved, but a schema failure can still be misrepresented and readiness can still provide a false-positive READY for the exact production schema compatibility question.

## 18. AUDIT CONFIDENCE

**HIGH.** The V09 prompt, immutable log, implementation diff, production functions, direct tests, tracker, live branch head, and status-check absence were independently inspected.

## 19. FINAL VERDICT

**FAIL / CHANGES_REQUIRED.**

V09 cannot progress directly to owner acceptance. A narrow V10 must close model-status category parity and make readiness genuinely representative of the real V08 audit schema boundary.

## 20. REQUIRED REMEDIATION

1. Preserve `SCHEMA_INCOMPATIBLE` through `unavailable_evaluation` into persisted audit `model_status`, summary, state, and UI/history truth.
2. Replace the trivial readiness schema probe with a bounded representative schema probe that exercises the material structured-output constructs used by the real V08 freeform audit contract, preferably by deriving a synthetic minimal freeform `AuditInput` and using the production `audit_result_schema` or an explicitly equivalent compatibility schema.
3. Add direct tests proving both behaviors.
4. Preserve V05-V09 accepted behavior, Codex-only/no-API-key architecture, and M17 block.
5. Re-run governed publication and then perform independent strict re-audit before any new owner-native gate.
