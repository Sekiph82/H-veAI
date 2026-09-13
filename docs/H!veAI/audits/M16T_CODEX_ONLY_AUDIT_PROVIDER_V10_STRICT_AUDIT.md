# M16T Codex-Only Audit Provider V10 — Independent Strict Audit

## 1. VERDICT

**PASS / AWAITING_OWNER_NATIVE_REACCEPTANCE**

V10 closes both residual V09 truth findings. `SCHEMA_INCOMPATIBLE` now survives provider failure classification into degraded evaluation, persistence, current-result presentation, and immutable audit history without collapsing to `UNAVAILABLE`. Settings readiness now derives its compatibility probe from the production `audit_result_schema()` using a deterministic zero-requirement synthetic freeform input and validates the dedicated final result through the production parse and semantic contract. No source-level blocker remains in V10 scope. M16 remains OPEN only for the owner-native readiness and three-consecutive-run gate. M17 remains NOT ACTIVATED/BLOCKED.

## 2. CONTRACT RECOVERY

V10 was required to preserve the accepted Codex-only architecture while closing two narrow defects from the V09 strict audit:

1. preserve explicit `SCHEMA_INCOMPATIBLE` truth through provider -> degraded evaluation -> persistence -> UI/history;
2. replace the trivial readiness schema with a bounded representative feature probe of the actual V08 freeform production audit schema.

It also had to preserve V05 FormuLab/main and exact-eight-project tracking, V06 degraded-state/ACL behavior, V07 history truth, V08 fail-closed dynamic schema/semantic behavior, V09 explicit failure classification and bounded diagnostics, the no-API-key architecture, and the M17 block.

## 3. BRANCH / HEAD / DIFF SCOPE

- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Prompt/start SHA: `c52fd2c071d3a1debbc7047006c5cc48bd481573`
- V10 implementation SHA: `f5d48459551359a8af6341e41fdcb12cc4ddbeb9`
- V10 builder-log SHA / audited pre-audit main: `bb0a1f16c606b790f96755d92d65ef7a8e148f1a`
- Implementation is exactly one commit ahead of the prompt/start SHA.
- Changed implementation files:
  - `src-tauri/src/audit_engine.rs`
  - `tests/m16-audit-center-focused.test.tsx`
  - `TASKS.md`
  - `CODEX_ROADMAP.md`

## 4. ACCEPTANCE CRITERIA MATRIX

| Criterion | Result | Evidence |
|---|---|---|
| Provider schema rejection classifies as `SCHEMA_INCOMPATIBLE` | PASS | V09 classifier retained |
| `unavailable_evaluation` preserves `SCHEMA_INCOMPATIBLE` | PASS | Explicit model-status branch and category-specific summary |
| Schema-incompatible run is non-authoritative | PASS | Direct fixture asserts `FAILED + CONDITIONAL + SCHEMA_INCOMPATIBLE` |
| Persisted/reloaded audit preserves category | PASS | Direct fixture reload asserts `SCHEMA_INCOMPATIBLE + FAILED` |
| Audit Center history distinguishes schema incompatibility | PASS | Focused frontend fixture and selection/diagnostic test |
| Readiness schema is derived from production schema | PASS | `codex_readiness_schema() -> audit_result_schema(&synthetic_readiness_input())` |
| Readiness exercises freeform V08 material schema features | PASS | Direct schema assertions for nested objects, enums, `maxItems:0`, and one-row coverage bounds |
| Readiness uses production process boundary | PASS | Same `CodexProcessRunner`, `--output-schema`, read-only ephemeral path, dedicated final-message channel |
| Readiness validates representative final output | PASS | `parse_model_output` + `validate_semantic_evaluation` on synthetic freeform input |
| Trivial/plain `READY` is insufficient | PASS | Focused test expects `SCHEMA_INCOMPATIBLE` |
| Explicit quota remains `USAGE_LIMITED` | PASS | V09 classifier/readiness tests retained |
| Plain CLI `Usage:` remains non-quota | PASS | V09 classifier test retained |
| Readiness persists no audit row | PASS | Provider-only probe path preserved; no audit persistence call introduced |
| Diagnostics remain bounded and sanitized | PASS | V09 diagnostic helpers unchanged |
| V08 semantic fail-closed behavior remains intact | PASS | Production validator remains authoritative after parse |
| V05 FormuLab/main + 8-project architecture preserved | PASS | No related source touched in V10 implementation |
| OpenAI API-key/direct HTTP provider absent | PASS | Active-source searches return no `OPENAI_API_KEY` or `api.openai.com` path |
| M17/Claude remains blocked | PASS | Tracker and roadmap remain M16/V10 with M17 not activated |
| Owner-native closure gate | PENDING | Intentionally not fabricated by builder or source audit |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

The immutable V10 builder log claims 44 focused audit-engine Rust tests, 11 V05 tracking tests, 10 focused frontend tests, 138 full frontend tests, 445 full Rust tests, typecheck, cargo check, build, formatter/diff checks, guardrail scan, and governed native publication PASS.

Direct source, diff, and test-body inspection independently corroborate the two V10 remediation behaviors. GitHub exposes no independent combined CI status checks for the implementation commit, so the reported execution counts and publication SHA remain builder claims rather than independent CI proof. This does not create a source finding because the implementation and direct test contracts are independently inspectable and no contradictory repository evidence was found.

## 6. FILE / SYMBOL EVIDENCE

### F-V09-001 closure: schema-status parity

`unavailable_evaluation()` now recognizes errors containing `SCHEMA_INCOMPATIBLE` and maps them to:

- `model_status = SCHEMA_INCOMPATIBLE`;
- category-specific non-authoritative summary;
- existing degraded `CONDITIONAL / LOW / HIGH` evaluation semantics.

The V10 fixture `schema_incompatible_degraded_audit_persists_explicit_failed_truth` runs a real fixture project through the Codex-process failure path, asserts `SCHEMA_INCOMPATIBLE`, `FAILED`, and `CONDITIONAL`, verifies the diagnostic retains the bounded schema error, reloads the persisted row, and asserts the same model status/state.

Focused Audit Center frontend coverage adds a `FAILED / SCHEMA_INCOMPATIBLE` immutable history row and verifies the persisted diagnostic is selectable and visible.

### F-V09-002 closure: representative production readiness

`synthetic_readiness_input()` creates a deterministic in-memory zero-requirement freeform audit input with complete synthetic Git identity and no project evidence.

`codex_readiness_schema()` no longer has an independent trivial schema. It directly calls:

`audit_result_schema(&synthetic_readiness_input())`

Therefore readiness and actual freeform audit execution share the same schema generator for the material V08 path.

`check_codex_readiness_with_runner()` now requests a tiny freeform-shaped result and, after the bounded process succeeds, accepts `READY` only when the dedicated final message:

1. parses through `parse_model_output`, and
2. passes `validate_semantic_evaluation` against the same synthetic readiness input.

A nonconformant final result returns `SCHEMA_INCOMPATIBLE`; explicit process-level schema rejection remains separately classified by V09.

## 7. FOCUSED TEST EVIDENCE

Direct V10 test bodies were inspected and materially prove:

- schema-incompatible provider failures persist as `SCHEMA_INCOMPATIBLE` and `FAILED`;
- current/history UI can distinguish `SCHEMA_INCOMPATIBLE` from other degraded statuses;
- readiness requests contain the representative production-derived schema;
- finding items use nested `additionalProperties:false`;
- freeform finding `requirementRefs.maxItems == 0`;
- requirement coverage has `minItems == 1` and `maxItems == 1`;
- coverage evidence refs have `maxItems == 0`;
- top-level and nested enums remain present;
- representative conformant freeform output reaches `READY`;
- plain text `READY` does not reach READY;
- schema rejection and explicit quota remain distinct.

V09 explicit classifier tests for `Usage:`, quota, auth, network, unknown process failure, bounded diagnostics, and secret redaction remain present.

## 8. REGRESSION EVIDENCE

Implementation scope is narrow. The diff changes only audit-engine truth/readiness behavior, focused Audit Center coverage, and prospective tracker/roadmap wording. No V05 FormuLab mapping, project portfolio configuration, local-workspace code, agent/session runtime, prompt engine, or M17 source was changed.

Builder regression counts are claim evidence because no GitHub CI status is attached to the implementation commit. No direct source regression was found.

## 9. SECURITY / SAFETY REVIEW

**PASS.** The readiness probe is in-memory, bounded, read-only, ephemeral, and uses the same managed Codex CLI boundary as audits. It does not inspect project files, Git repositories, web, MCP, plugins, or auth files. V09 bounded/sanitized diagnostic behavior remains intact. No OpenAI API-key path or direct HTTP provider was introduced.

## 10. ARCHITECTURE CONSISTENCY

**PASS.** Codex CLI remains the sole production model-backed audit provider. Authentication remains Codex-managed ChatGPT login. The audit and readiness paths now share the material production schema authority rather than maintaining divergent compatibility contracts. Provider failure categories remain evidence-truth categories rather than being silently collapsed.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

`TASKS.md` and `CODEX_ROADMAP.md` correctly describe V10 as implementation-complete, M16 OPEN, required actor HUMAN, and M17 blocked at the audited pre-audit head. The immutable builder log correctly states that owner-native readiness and the three-run gate remain pending.

After this independent PASS, the repository's conservative pre-audit wording still says independent V10 audit is pending; it must not be interpreted as a failed gate. The authoritative next substantive gate is owner-native V10 re-acceptance. Final M16 closure/tracker transition must occur only after that HUMAN evidence.

## 12. FINAL REPOSITORY STATE

At strict-audit start, live `main` is `bb0a1f16c606b790f96755d92d65ef7a8e148f1a`, whose parent is V10 implementation SHA `f5d48459551359a8af6341e41fdcb12cc4ddbeb9`. The required immutable V10 log exists on `main`. No independent GitHub combined status checks are attached to the implementation commit.

## 13. OPEN CROSS-MILESTONE FINDINGS

- M16 remains OPEN only because the required owner-native V10 gate has not yet been performed on this accepted source.
- M17/Claude remains blocked until M16 owner-native acceptance and closure.
- Historical degraded audit rows remain immutable evidence and must not be rewritten.

## 14. DEFECTS BY SEVERITY

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- NOTE: GitHub exposes no independent CI status for the V10 implementation commit; builder test/publication counts remain claim evidence.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

A typed internal provider-failure category enum could reduce future string-mapping drift, but V10's explicit mapping now satisfies the current contract. This is not a closure requirement and should not trigger another remediation cycle without a reproduced defect.

## 16. UNVERIFIED ITEMS

- Real owner-native Settings readiness result on the governed V10 executable.
- Three consecutive owner-native freeform audits on unchanged HEAD.
- Builder-reported full test counts and governed EXE hash were not independently reproduced through GitHub CI.

## 17. REGRESSION RISK

**LOW-MEDIUM.** V10 is narrowly scoped and reuses production schema generation rather than introducing a second schema implementation. The remaining uncertainty is external/native Codex behavior on the owner's installed CLI and account allowance, which is exactly what the required HUMAN gate is designed to test.

## 18. AUDIT CONFIDENCE

**HIGH.** The V10 prompt, V09 strict findings, immutable V10 builder log, implementation commit/diff, production audit/readiness functions, direct tests, tracker/roadmap, live branch head, and absence of GitHub status checks were independently inspected.

## 19. FINAL VERDICT

**PASS / AWAITING_OWNER_NATIVE_REACCEPTANCE.**

No further Codex remediation prompt is justified from repository evidence. The next action is the owner-native gate on the governed V10 build.

## 20. REQUIRED NEXT ACTION

1. Launch the governed published H!veAI build from the Desktop shortcut.
2. Run Settings -> Codex Audit Provider -> Check readiness.
3. Record the exact native category and bounded diagnostic. If the result is not READY, do not run three audits; the category itself is the acceptance evidence to evaluate.
4. If readiness is READY, select the same project/freeform target on an unchanged HEAD and run three consecutive audits.
5. All three must become `COMPLETED + AVAILABLE` and each must preserve the V08 one-row `project-audit / NOT_APPLICABLE` coverage contract.
6. No false `USAGE_LIMITED`, `SCHEMA_INCOMPATIBLE`, `MALFORMED`, or unknown requirement-reference result may occur.
7. Only after the HUMAN gate passes may M16 be closed and M17/Claude activated.
