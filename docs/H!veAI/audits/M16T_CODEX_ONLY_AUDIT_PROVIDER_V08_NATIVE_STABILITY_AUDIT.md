# M16T Codex-Only Audit Provider V08 Native Stability Audit

## 1. VERDICT

**FAIL / CHANGES_REQUIRED**

Owner-native re-acceptance proves that the V06/V07 remediation substantially works: Codex readiness is now genuinely READY through the local Codex CLI and ChatGPT login, a real project/freeform audit can complete with `modelStatus=AVAILABLE`, and immutable audit-history rows now visibly distinguish execution state from model status. However, a second real freeform audit on the same project/HEAD immediately produced `FAILED / MALFORMED` with diagnostic `AUDIT_REQUIREMENT_REFERENCE_UNKNOWN`.

This is a production stability defect, not an owner-operation error. The current freeform output contract constrains the synthetic `requirementCoverage` row but does not constrain `findings[].requirementRefs`, while the semantic validator rejects every non-empty finding requirement reference when the canonical task-requirement set is empty. The static JSON output schema also permits arbitrary finding requirement-reference strings. The resulting contract/schema/validator mismatch makes otherwise valid freeform audits nondeterministically fail depending on model wording.

M16 remains OPEN. M17 remains NOT ACTIVATED/BLOCKED.

## 2. CONTRACT RECOVERY

Final owner-native M16T acceptance requires all of the following simultaneously:

1. local Codex CLI is the sole model-backed production audit provider;
2. ChatGPT-managed Codex login is READY without any API key;
3. freeform/project audits with no task-scoped requirements have one deterministic synthetic coverage row: `project-audit / NOT_APPLICABLE`;
4. findings remain evidence-backed and semantically valid without inventing task requirement references;
5. model schema, model prompt contract, and semantic validator agree on the exact legal output shape;
6. degraded/malformed output fails closed rather than being promoted to success;
7. immutable history visibly distinguishes completed/available from failed/malformed runs;
8. V05 FormuLab `main`, exact eight-project portfolio, Codex-only architecture, and M17 block remain preserved.

## 3. OWNER-NATIVE EVIDENCE

The owner supplied current stable-build screenshots showing:

- Settings -> Codex Audit Provider: CLI Available, `codex-cli 0.130.0-alpha.5`, Login `ChatGPT authenticated`, Status `READY`, and `Live provider readiness checked.`
- A real ScrubBots freeform audit at HEAD `fd84520742e2d7c3244bff25efe9e3e90b065cdb` completed with `CONDITIONAL / COMPLETED`, `modelStatus=AVAILABLE`, model `CODEX_CLI CLI_DEFAULT`, and one bounded requirement-coverage row.
- Audit history visibly distinguishes `COMPLETED / AVAILABLE`, `STALE / AVAILABLE`, historical `COMPLETED / MALFORMED`, and historical `COMPLETED / UNAVAILABLE` records, proving the V07 history-truth UI is working.
- A subsequent real ScrubBots freeform audit at the same HEAD failed as `CONDITIONAL / FAILED`, `modelStatus=MALFORMED`, with exact diagnostic `AUDIT_REQUIREMENT_REFERENCE_UNKNOWN`.

The successful and failed runs occurring against the same freeform shape and same HEAD demonstrate output-contract instability rather than repository freshness drift.

## 4. SOURCE-LEVEL ROOT CAUSE

### Semantic validator

`src-tauri/src/audit_engine.rs::validate_semantic_evaluation` builds `canonical_requirements` exclusively from real required task requirements. For freeform/project audits this set is empty.

For every finding it currently rejects any reference not present in that set:

```rust
if finding
    .requirement_refs
    .iter()
    .any(|reference| !canonical_requirements.contains(reference.as_str()))
{
    return Err("AUDIT_REQUIREMENT_REFERENCE_UNKNOWN".into());
}
```

Therefore **every non-empty `findings[].requirementRefs` array is illegal in a freeform/project audit**.

### Freeform prompt contract

`audit_output_contract(input)` currently tells Codex only:

- return exactly one `requirementCoverage` row;
- `requirementRef = project-audit`;
- `status = NOT_APPLICABLE`;
- empty coverage evidence refs;
- do not invent task requirement references.

It does **not** explicitly say that every finding must use `requirementRefs: []` when no task requirements exist.

### Structured-output schema

`audit_result_schema()` is static. `findings[].requirementRefs` is currently defined only as an arbitrary string array. The schema therefore permits the exact output that the semantic validator later rejects.

The same static-schema looseness also means project/freeform `requirementCoverage` is not constrained at the schema layer to exactly one `project-audit / NOT_APPLICABLE` row even though the later semantic validator requires that exact shape.

## 5. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Evidence |
| --- | --- | --- |
| Codex CLI executable found | PASS | Owner-native Settings screenshot |
| ChatGPT-managed login accepted | PASS | Owner-native Settings screenshot |
| Explicit readiness reaches READY | PASS | Owner-native Settings screenshot |
| No OpenAI API-key path required | PASS | Accepted M16T architecture + native readiness |
| Real freeform Codex turn can produce AVAILABLE/COMPLETED | PASS | Owner-native successful audit |
| V07 history validity display works | PASS | Owner-native history screenshot |
| Degraded result fails closed | PASS | Owner-native FAILED/MALFORMED run |
| Freeform model contract is deterministic across equivalent runs | **FAIL** | Same freeform/HEAD subsequently produced AUDIT_REQUIREMENT_REFERENCE_UNKNOWN |
| Prompt/schema/semantic validator agree on finding requirement refs | **FAIL** | Prompt/schema allow ambiguity; validator requires empty finding refs |
| M16 can close | **FAIL** | Stability defect remains |

## 6. BUILDER CLAIMS VS REPOSITORY TRUTH

V07 builder claims are consistent with the accepted history-row source change and owner-native visual evidence. The new defect was not in V07 scope and was exposed only by the required real owner-native Codex turns. This is exactly why final native acceptance remained a separate gate.

## 7. SCHEMA / PROMPT / VALIDATOR CONSISTENCY

The correct contract for a project/freeform audit with no task-scoped requirements is:

- `requirementCoverage` contains exactly one synthetic row with `requirementRef=project-audit`, `status=NOT_APPLICABLE`, and `evidenceRefs=[]`;
- `findings` may still contain evidence-backed project-level defects;
- **every project/freeform finding must have `requirementRefs=[]`**, because there are no canonical task requirements to reference;
- finding `evidenceRefs` remain subject to normal evidence-reference integrity and quality rules;
- the semantic validator must remain fail-closed and must not silently rewrite unknown requirement references into a valid state.

For task-scoped audits, finding requirement references must remain a subset of the actual canonical required requirement refs.

## 8. REQUIRED SCHEMA HARDENING

The output schema should become input-aware rather than leaving requirement references unconstrained.

At minimum:

- project/freeform schema: `findings[].requirementRefs` has `maxItems: 0`;
- project/freeform schema: `requirementCoverage` has exactly one item (`minItems: 1`, `maxItems: 1`) whose `requirementRef` is `const: "project-audit"`, status is `const: "NOT_APPLICABLE"`, and `evidenceRefs` has `maxItems: 0`;
- task-scoped schema: `requirementCoverage[].requirementRef` is constrained to the canonical requirement-ref set and the coverage count is bounded to the required count;
- task-scoped findings may use only canonical requirement refs when refs are present.

The semantic validator remains the independent second line of defense.

## 9. REQUIRED PROMPT-CONTRACT HARDENING

For project/freeform audits the model instruction must explicitly state:

- there are zero canonical task requirements;
- **every finding must return `requirementRefs: []`;**
- `project-audit` is a synthetic coverage identifier only and must not be used as a finding requirement reference;
- one and only one `project-audit / NOT_APPLICABLE` coverage row is required;
- project-level findings may still be emitted when directly supported by supplied evidence.

For task-scoped audits the prompt must explicitly say finding requirement refs, when present, must be selected only from the canonical list.

## 10. DO NOT WEAKEN VALIDATION

Do not fix this by:

- accepting arbitrary unknown requirement refs;
- treating `project-audit` as a real task requirement globally;
- deleting finding requirement refs after parsing;
- silently normalizing malformed output into authoritative output;
- converting a malformed run into AVAILABLE/COMPLETED;
- retrying indefinitely until a passing shape appears.

Fail-closed semantic validation remains required.

## 11. DETERMINISTIC TEST REQUIREMENTS

Required direct tests include at minimum:

1. project/freeform schema explicitly forbids non-empty finding requirement refs;
2. project/freeform schema requires exactly one `project-audit / NOT_APPLICABLE` coverage row with empty evidence refs;
3. project/freeform prompt contract explicitly says every finding `requirementRefs` must be empty and `project-audit` must not be used there;
4. a valid project-level finding with `requirementRefs=[]` plus valid evidence is accepted;
5. a project-level finding with `requirementRefs=["project-audit"]` remains semantically rejected if it bypasses/violates the schema fixture;
6. another invented finding requirement ref remains rejected;
7. task-scoped schema/semantic behavior continues to accept only canonical task refs;
8. same-verdict history truth, V06 degraded-state behavior, V05 FormuLab branch behavior, and readiness ACL behavior remain green.

Automated tests must not consume live Codex quota.

## 12. OWNER-NATIVE STABILITY GATE AFTER REMEDIATION

After independent source audit PASS, final owner-native re-acceptance must prove:

- Settings readiness = READY with ChatGPT-authenticated local Codex;
- the current stable H!veAI build performs **three consecutive freeform/project audits on the same unchanged HEAD**;
- all three persist as `modelStatus=AVAILABLE` and `state=COMPLETED` (verdict itself may legitimately be PASS/CONDITIONAL/FAIL based on evidence);
- each has exactly one `project-audit / NOT_APPLICABLE` coverage row;
- no run produces `AUDIT_REQUIREMENT_REFERENCE_UNKNOWN`, MALFORMED, or schema/semantic degradation;
- audit history visibly shows the three authoritative runs and keeps historical degraded runs immutable.

## 13. SECURITY / PRIVACY

The accepted Codex-only boundary remains mandatory:

- no `OPENAI_API_KEY`;
- no direct OpenAI HTTP/Responses audit transport;
- no API-key fallback;
- no Codex auth-file/token inspection;
- Codex-managed ChatGPT login only;
- bounded ephemeral read-only audit process;
- supplied AuditInput remains the only authoritative evidence given to the model.

## 14. TRACKER GOVERNANCE

M16 remains OPEN. M17 remains blocked.

Prospective truth should move to `M16T V08 — Freeform requirement-reference contract stability remediation`, Required Actor CODEX during implementation, then HUMAN after builder completion pending independent strict audit and owner native re-acceptance.

The M16T summary marker remains `[~]` until final owner-native stability acceptance.

## 15. REGRESSION RISK

**MEDIUM.** The required source change is bounded, but it touches the machine-readable model output contract shared by live audit execution. Schema and semantic behavior must be tested together to avoid making task-scoped audits overly restrictive.

## 16. UNVERIFIED ITEMS

- The exact invalid finding reference emitted by the failed live Codex turn is not exposed by the current bounded diagnostic; the validator category is sufficient to identify the contract class.
- Builder full-suite claims for the eventual V08 remediation will remain claims unless independent CI evidence exists.
- Final native stability cannot be accepted until the owner repeats real Codex turns on the remediated stable build.

## 17. FINDINGS BY SEVERITY

### M16T-V08-F01 — MAJOR — Freeform finding requirement-reference contract is underspecified

**Location:** `audit_output_contract`, `audit_result_schema`, `validate_semantic_evaluation`.

**Impact:** equivalent live freeform audits can nondeterministically alternate between authoritative AVAILABLE/COMPLETED output and FAILED/MALFORMED output solely because the model may populate a finding requirement reference that schema/prompt do not clearly forbid but semantic validation rejects.

### M16T-V08-F02 — MAJOR — Static output schema permits values known to be semantically illegal

**Location:** `audit_result_schema()`.

**Impact:** structured output constrains JSON shape but does not constrain freeform requirement identities, allowing avoidable semantic rejection after an otherwise successful model turn.

## 18. AUDIT CONFIDENCE

**HIGH.** The owner-native failure is directly reproduced by a persisted diagnostic, and the source-level contract mismatch is explicit: freeform canonical requirements are empty, findings are checked against that empty set, while prompt/schema do not force finding refs empty.

## 19. FINAL VERDICT

**FAIL / CHANGES_REQUIRED.**

V07 is accepted for history presentation, readiness is accepted, and one live authoritative audit succeeded. M16 nevertheless cannot close because the next equivalent freeform turn exposed a deterministic contract gap that causes stochastic production failure.

## 20. REQUIRED REMEDIATION

Create one narrow V08 remediation that:

1. makes audit structured-output schema input-aware;
2. explicitly forbids finding requirement refs in zero-task/freeform audits;
3. locks freeform coverage to exactly one `project-audit / NOT_APPLICABLE` row with empty evidence refs;
4. constrains task-scoped refs to canonical requirement identities;
5. strengthens the model prompt contract consistently;
6. keeps semantic validation fail-closed and independent;
7. adds direct deterministic tests for valid/invalid freeform findings and task-scoped regressions;
8. preserves V05-V07 accepted behavior, Codex-only architecture, eight-project portfolio, and M17 block;
9. publishes the governed native build;
10. leaves final closure to independent strict audit plus the three-consecutive-run owner-native stability gate.
