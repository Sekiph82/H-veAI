# M16T Codex-Only Audit Provider V09 — Native Failure Classification Strict Audit

## 1. VERDICT

**FAIL / CHANGES_REQUIRED**

V08 source remediation remains accepted, but the owner-native three-run gate cannot be interpreted reliably because all three post-V08 audit turns were persisted as `FAILED / USAGE_LIMITED` while Settings had just reached `READY`. Repository inspection proves that the current failure classifier can misclassify ordinary Codex CLI help/error output containing the generic word `usage` as a quota exhaustion. The readiness probe also does not exercise the production structured-output schema path. M16 remains OPEN and M17 remains BLOCKED.

## 2. CONTRACT RECOVERY

M16T requires a truthful Codex-only audit provider. Provider readiness, runtime failures, model status, diagnostics, and audit history must distinguish authentication, usage/quota, schema compatibility, network, process, malformed-model, and unavailable states without inventing success or inventing a quota failure. The explicit owner-native gate requires three consecutive authoritative freeform audits, but environmental/provider failures must first be classified truthfully.

## 3. BRANCH / HEAD / DIFF SCOPE

- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Accepted V08 audit SHA: `fc31c512a2bc8aa5846acec54e2b2e0b50c28786`
- Owner native audit HEAD shown in screenshots: `a88b5074b5ea74e13ff94f72aa82aa37bdc1ee9a`
- Observed native result: three consecutive project/freeform runs persisted `FAILED / USAGE_LIMITED`.
- Settings readiness immediately before the runs reached `READY` with local Codex CLI and ChatGPT authentication.

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Evidence |
|---|---|---|
| V08 freeform contract source fix remains present | PASS | Accepted V08 source audit |
| Settings can discover Codex + ChatGPT login | PASS | Owner screenshot |
| Settings readiness truthfully proves production structured-output compatibility | FAIL | readiness probe uses `schema: None` |
| USAGE_LIMITED classification is specific to real usage/rate/quota failures | FAIL | classifier matches generic `text.contains("usage")` |
| CLI usage/help text cannot become a false quota result | FAIL | no exclusion for `Usage:` help output |
| Runtime diagnostics preserve enough sanitized provider detail to disambiguate failures | FAIL | classifier returns only generic category sentence |
| Three consecutive authoritative freeform audits | FAIL/UNVERIFIED | all three failed before authoritative verdict; underlying reason is not yet trustworthy |
| No API-key/direct OpenAI provider | PASS | accepted architecture preserved |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

V08 builder evidence and strict source audit support the schema/prompt remediation. The new native failure is not evidence that V08 semantic rules regressed. It exposes an older process-classification/readiness observability weakness that V08 did not change.

## 6. FILE / SYMBOL EVIDENCE

`src-tauri/src/audit_engine.rs::classify_codex_failure` lowercases combined stdout/stderr and classifies `USAGE_LIMITED` when the text contains any of `usage`, `quota`, or `rate limit`. Generic CLI help commonly contains the word `Usage`, so the condition is not specific enough to prove quota exhaustion.

`check_codex_readiness_with_runner` runs a headless Codex turn with `schema: None` and asks for the literal text `READY`. Therefore Settings `READY` verifies executable/auth/basic-turn readiness, but not the production `--output-schema` path used by actual audits.

## 7. FOCUSED TEST EVIDENCE

Existing V08 tests cover the generated audit schema and semantic validator, but the owner-native evidence proves the failure-classification/readiness boundary needs direct tests for CLI help text, explicit quota text, schema-invalid output, and structured-output readiness probing.

## 8. REGRESSION EVIDENCE

V05 FormuLab/main behavior, V06 ACL/degraded-state semantics, V07 history truth, and V08 schema/semantic contract remain accepted. V09 must be narrowly additive around process classification, diagnostic preservation, and readiness equivalence.

## 9. SECURITY / SAFETY REVIEW

Diagnostics must remain bounded and sanitized. Do not expose Codex auth material, tokens, headers, environment secrets, or arbitrary unbounded stderr/stdout. Reuse the existing evidence sanitization strategy or a dedicated equivalent.

## 10. ARCHITECTURE CONSISTENCY

Codex CLI remains the only model-backed provider. No API key, direct OpenAI HTTP transport, auth-file inspection, GUI automation, or Claude/M17 implementation is permitted.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

The canonical tracker currently says V08 is awaiting owner-native stability acceptance. The native gate did not pass. V09 must become the active remediation and keep M16 OPEN / M17 BLOCKED.

## 12. FINAL REPOSITORY STATE

At audit start, live `main` contained accepted V08 source remediation and audit. This V09 audit records a newly reproduced owner-native defect; historical V08 artifacts remain immutable.

## 13. OPEN CROSS-MILESTONE FINDINGS

M16 remains the only active closure gate. M17 must not be activated until V09 source audit and owner native acceptance are complete.

## 14. DEFECTS BY SEVERITY

- **MAJOR M16T-V09-F01:** `USAGE_LIMITED` classifier uses the generic substring `usage`; CLI help/error output can be falsely labeled quota exhaustion.
- **MAJOR M16T-V09-F02:** Settings readiness does not exercise the production structured-output/schema path, so `READY` can coexist with a production schema/process incompatibility.
- **MAJOR M16T-V09-F03:** failed Codex turns discard the bounded sanitized provider excerpt needed to distinguish quota, schema, CLI invocation, network, and other process failures.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

The owner is running `codex-cli 0.130.0-alpha.5`. Current upstream Codex releases have continued to fix structured-output behavior. H!veAI should prefer feature probing over a brittle hard-coded version floor, while surfacing the detected CLI version and an actionable incompatibility category when the representative structured-output probe fails.

## 16. UNVERIFIED ITEMS

- Whether the owner's ChatGPT/Codex allowance was actually exhausted at the time of the three runs.
- Whether the immediate native failure was caused by a schema compatibility rejection, another CLI invocation error, or a genuine service-side usage limit.
- The exact provider stderr/stdout was not surfaced by the current generic diagnostic, which is itself part of the defect.

## 17. REGRESSION RISK

**MEDIUM.** Classification changes affect degraded-state truth and Settings readiness. Scope is small but user-facing audit authority depends on it.

## 18. AUDIT CONFIDENCE

**HIGH** that the current code cannot distinguish these failure classes reliably. **LOW** on the exact underlying native failure category until V09 preserves and classifies bounded provider diagnostics correctly.

## 19. FINAL VERDICT

**FAIL / CHANGES_REQUIRED.**

Do not tell the owner that the Codex quota is definitely exhausted based solely on the current H!veAI `USAGE_LIMITED` label. The label is not sufficiently specific under the present classifier.

## 20. REQUIRED REMEDIATION

1. Replace generic substring-based failure classification with explicit, tested categories. Literal CLI help `Usage:` must never by itself imply usage/quota exhaustion.
2. Recognize genuine usage-limit signals only from explicit phrases/statuses such as `usage limit`, `rate limit`, `quota exceeded`, `You've hit your usage limit`, or equivalent structured/error evidence.
3. Add a distinct schema/structured-output incompatibility category when the CLI reports invalid/unsupported output schema behavior.
4. Preserve a bounded, sanitized diagnostic excerpt so owner/auditor can see the actual provider failure without leaking secrets.
5. Make explicit `Check readiness` exercise a representative structured-output turn compatible with the production audit boundary, without persisting audit history.
6. Keep V05/V06/V07/V08 behavior unchanged and keep the Codex-only/no-API-key architecture.
7. Add deterministic classifier/readiness tests, including generic `Usage:` help output, genuine quota text, schema errors, auth failures, network failures, and successful schema readiness.
8. Governed native publication is required. Owner acceptance must then retry readiness and the three-run freeform stability gate.
