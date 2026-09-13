# M16T Codex-Only Audit Provider V11 — Owner-Native Schema Compatibility Audit

## 1. VERDICT

**FAIL / CHANGES_REQUIRED**

V10 source remediation passed independent review, but the owner-native V10 readiness gate on Codex CLI `0.154.0` still returns `SCHEMA_INCOMPATIBLE`. The old `0.130.0-alpha.5` model-cache parsing defect is gone, so this is a new and narrower compatibility/diagnostic failure. Two production findings remain: the transport schema emits a nonessential `uniqueItems` keyword outside the documented Structured Outputs array-property subset, and the readiness path conflates exit-zero final-result parse/semantic failure with transport schema incompatibility while masking the real reason behind benign stderr such as `Reading prompt from stdin...`.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

## 2. CONTRACT RECOVERY

The V10 contract required Settings readiness to feature-probe the production freeform audit schema through local Codex CLI, preserve explicit provider-category truth, remain Codex-only and API-key-free, and expose bounded actionable diagnostics. Owner-native evidence now proves that the V10 governed build reaches Codex CLI `0.154.0`, authenticates through ChatGPT login, executes the representative structured-output probe, and then reports `SCHEMA_INCOMPATIBLE` with the diagnostic `AUDIT_CODEX_SCHEMA_INCOMPATIBLE: Reading prompt from stdin...`.

That diagnostic is not sufficient to identify whether the provider rejected the schema, the dedicated final result was missing, JSON parsing failed, or semantic validation failed.

## 3. BRANCH / HEAD / DIFF SCOPE

- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- V10 implementation SHA: `f5d48459551359a8af6341e41fdcb12cc4ddbeb9`
- V10 builder-log SHA: `bb0a1f16c606b790f96755d92d65ef7a8e148f1a`
- Accepted V10 independent audit SHA: `cc79324bbee7dfa9258dd4572f2411e4338d050a`
- Native environment shown by owner: `codex-cli 0.154.0`, local Codex installed, ChatGPT login available
- Native readiness result: `SCHEMA_INCOMPATIBLE`

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Evidence |
|---|---|---|
| Codex executable detected | PASS | Native Settings shows `CLI Available` |
| Current CLI version detected | PASS | Native Settings shows `codex-cli 0.154.0` |
| Old 0.130 model-cache `max` parsing defect removed | PASS | Native diagnostic no longer contains the old model-cache decode failure |
| ChatGPT-login path preserved | PASS | Settings reports ChatGPT login; no API-key path introduced |
| V10 representative readiness probe executes | PASS | Native readiness produces a bounded provider result rather than ACL/not-found failure |
| Readiness returns READY | FAIL | Native result is `SCHEMA_INCOMPATIBLE` |
| Transport schema is limited to supported Structured Outputs subset | FAIL/PARTIAL | `audit_result_schema` always emits `uniqueItems`, while current OpenAI Structured Outputs documentation lists `minItems` and `maxItems` as supported array constraints but not `uniqueItems` |
| Readiness diagnostic identifies the real final-result failure | FAIL | Exit-zero nonconformant final result is collapsed to `SCHEMA_INCOMPATIBLE`; diagnostic can become only `Reading prompt from stdin...` |
| Three consecutive native audits | BLOCKED | Must not proceed while readiness is not READY |

## 5. BUILDER CLAIMS VS REPOSITORY / NATIVE TRUTH

The V10 builder log claims representative production-schema readiness and publication PASS. Repository source independently confirms that the representative production schema is used and that `SCHEMA_INCOMPATIBLE` persistence is fixed. The owner-native run, however, proves that the published V10 readiness gate still fails on a current Codex CLI installation.

This is not evidence that V10's source audit was fabricated. It is new runtime evidence exposing a remaining schema-subset/diagnostic mismatch that deterministic mocks did not exercise.

## 6. FILE / SYMBOL EVIDENCE

### F-V11-001 — Transport schema emits nonessential `uniqueItems`

`audit_result_schema(input)` builds coverage bounds and always emits:

```text
"uniqueItems": coverage_bounds["uniqueItems"].as_bool().unwrap_or(false)
```

For freeform readiness this becomes `uniqueItems: false`; for task-scoped audits it can become true. Exact freeform cardinality and task requirement uniqueness are already enforced again by `validate_semantic_evaluation` using the canonical requirement set and exact occurrence counts.

OpenAI's current Structured Outputs documentation explicitly describes the feature as a JSON Schema subset. Its supported array-property list contains `minItems` and `maxItems`; `uniqueItems` is not listed. Therefore H!veAI should not send `uniqueItems` merely to duplicate a constraint already enforced by the semantic validator.

### F-V11-002 — Readiness conflates final-result invalidity with schema incompatibility

`check_codex_readiness_with_runner` correctly classifies nonzero process failures through `classify_codex_failure_category`. But on exit code zero it reduces parsing + semantic validation to a boolean `schema_conformant` value. Any false result is returned as `SCHEMA_INCOMPATIBLE`.

The failure message then uses `diagnostic_source(&result)`. When Codex writes the ordinary progress line `Reading prompt from stdin...` to stderr, that benign line can become the sole user-facing diagnostic even though the actual failure occurred in final-message presence/parsing/semantic validation.

The owner screenshot exhibits exactly this ambiguity.

## 7. FOCUSED TEST EVIDENCE

Existing V10 tests prove the representative schema contains V08 constructs and that mocked conformant output becomes READY. They do not prove that the emitted schema contains only provider-supported keywords, and they do not separately test:

- exit 0 + missing final message;
- exit 0 + malformed JSON;
- exit 0 + syntactically valid but semantically invalid readiness result;
- benign stderr plus a more specific final-result validation failure.

## 8. REGRESSION EVIDENCE

V05-V10 source behavior remains accepted outside these findings. No evidence suggests regression in FormuLab tracking, eight-project portfolio, ACL, history badges, Codex-only authentication policy, or V08 semantic fail-closed behavior.

## 9. SECURITY / SAFETY REVIEW

**PASS.** The failures are schema compatibility and truth diagnostics. No API key, direct OpenAI HTTP provider, auth-file inspection, GUI automation, secret leakage, or write-enabled Codex turn is required to remediate them.

## 10. ARCHITECTURE CONSISTENCY

**PARTIAL.** The architecture correctly feature-probes the local Codex runtime, but the transport schema should conform to the provider-supported Structured Outputs subset while semantic validation remains H!veAI's stronger application-level guard. Provider transport validity and model-result semantic validity must remain distinct categories.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

V10 correctly remains `[~]` and M16 remains OPEN pending native acceptance. After this audit, owner-native acceptance is not the next action; a bounded V11 remediation is required first. M17 remains blocked.

## 12. FINAL REPOSITORY STATE

Accepted V10 independent audit is on live `main` at `cc79324bbee7dfa9258dd4572f2411e4338d050a` before this V11 audit artifact. GitHub exposes no independent CI status for the V10 implementation commit; builder test counts remain claims.

## 13. OPEN CROSS-MILESTONE FINDINGS

- M16 remains OPEN.
- M17 remains NOT ACTIVATED/BLOCKED.
- Native three-run acceptance remains blocked until readiness returns truthful READY.

## 14. DEFECTS BY SEVERITY

### MAJOR — F-V11-001: Production/readiness schema emits `uniqueItems`

The structured-output transport schema includes a nonessential keyword not listed in the current supported array-property subset. Semantic validation already enforces the corresponding uniqueness/cardinality truth.

### MAJOR — F-V11-002: Exit-zero final-result failures are mislabeled/masked

Missing, unparsable, or semantically invalid dedicated final output is collapsed into `SCHEMA_INCOMPATIBLE`, and benign stderr can mask the actual validation error.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

A dedicated transport-schema compatibility builder could separate provider-supported JSON Schema constraints from stronger application semantic constraints. This should remain a narrow helper, not a second independent audit schema authority.

## 16. UNVERIFIED ITEMS

- The exact provider-side reason for the current readiness failure is not visible because V10 masks exit-zero final-result validation errors.
- Native behavior after removal of unsupported/nonessential schema keywords is pending.

## 17. REGRESSION RISK

**MEDIUM.** Removing transport-only `uniqueItems` is low risk because semantic uniqueness remains enforced, but readiness/error categorization must not weaken fail-closed audit behavior.

## 18. AUDIT CONFIDENCE

**HIGH** for the two source-level truth defects. Owner screenshots, current source, V10 prompt/log/audit, and current Structured Outputs documentation were inspected. Confidence is intentionally lower about the exact provider-internal rejection because V10 does not expose that root cause correctly.

## 19. FINAL VERDICT

**FAIL / CHANGES_REQUIRED.**

Do not run the three-audit native closure test yet. A narrow V11 must first make the Codex transport schema provider-compatible without weakening semantic validation and make readiness diagnostics preserve the actual exit-zero final-result failure category.

## 20. REQUIRED REMEDIATION

1. Remove `uniqueItems` from the schema sent through Codex `--output-schema`; keep exact cardinality/uniqueness enforced by the existing semantic validator.
2. Add deterministic transport-schema tests proving H!veAI does not emit unsupported/nonessential keywords such as `uniqueItems` while preserving required `additionalProperties:false`, required fields, enums, nullable types, and supported bounds.
3. Refactor readiness result validation so exit-zero outcomes distinguish:
   - missing dedicated final message;
   - JSON/shape parse failure;
   - semantic contract failure;
   - successful READY.
4. Do not label parse/semantic failure as provider schema incompatibility unless the process/provider explicitly reports schema rejection.
5. Prefer the specific final-result validation error over benign stderr such as `Reading prompt from stdin...`.
6. Preserve bounded/redacted diagnostics and all V05-V10 accepted behavior.
7. Preserve Codex-only ChatGPT-login architecture; no OpenAI API key/direct HTTP path.
8. Publish a governed build, then independently re-audit before owner-native readiness is retried.
