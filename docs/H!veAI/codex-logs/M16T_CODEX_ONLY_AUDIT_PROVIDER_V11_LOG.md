# M16T Codex-Only Audit Provider V11 Remediation Log

- Work item: M16T V11, structured-output subset and readiness diagnostic parity remediation
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Execution date: 2026-09-14
- Starting synchronized HEAD: `4348422c67c342bdecadc8d6560a45184efda1ec`
- Implementation commit: `de627653ac5c3b1905ead4ea7d76152dfe82f82e`

## Scope and boundaries

This run closes the V11 findings only. The accepted Codex-only audit-provider architecture remains in place. No provider transport redesign, API-key audit path, direct HTTP audit provider, authentication-file inspection, GUI automation, Claude implementation, or M17 activation was introduced. V05 through V10 behavior remains covered by regression evidence.

M16 remains OPEN. M16T is `[~]` at 16/20 with implementation complete pending independent V11 strict audit and owner-native readiness re-acceptance. M17 remains blocked. The next actor is the independent auditor, followed by owner native acceptance; a three-run acceptance is required only after readiness is READY.

## Finding F-V11-001: transport schema subset compatibility

### Root cause

The generated audit transport schema emitted the JSON Schema `uniqueItems` keyword for `requirementCoverage` in task-scoped inputs. This keyword was rejected by the installed Codex structured-output subset. The earlier contract also relied on the keyword for uniqueness that is a semantic audit requirement, not a transport-schema requirement.

### Remediation

The transport schema no longer emits `uniqueItems`. The semantic evaluator remains authoritative for exact requirement coverage, duplicate references, missing references, extra references, and requirement-level validity. The schema still preserves the required properties, closed nested objects, enums, nullable required fields, and exact freeform/task cardinality and bounds. Task-scoped requirement references remain constrained by the canonical enum.

Direct Rust coverage includes recursive assertions that every object schema is closed, every declared property is required, and no schema node contains `uniqueItems`, plus freeform and multi-requirement task contract checks.

## Finding F-V11-002: readiness failure classification and diagnostic parity

### Root cause

The exit-zero readiness path previously collapsed missing final output, parse failure, and semantic contract failure into `SCHEMA_INCOMPATIBLE`. Its diagnostic could also be derived from generic process stderr, allowing a benign line such as `Reading prompt from stdin...` to mask the actual final-result failure.

### Remediation

Readiness now classifies the final-result stages independently:

- missing final assistant output: `PROCESS_ERROR` with `AUDIT_CODEX_FINAL_OUTPUT_MISSING`
- invalid final payload: `MALFORMED` with the exact model-parse diagnostic
- semantically invalid final payload: `MALFORMED` with the exact semantic-contract diagnostic
- explicit nonzero provider/schema rejection: `SCHEMA_INCOMPATIBLE`
- explicit quota/rate-limit signal: `USAGE_LIMITED`

Final-output parse and semantic diagnostics take precedence over benign process stderr. The frontend readiness vocabulary now renders `MALFORMED` distinctly and explains that the bounded probe completed but its final result failed parsing or semantic validation.

Direct tests cover missing output with benign stderr, parse failure, semantic failure, explicit schema rejection, and explicit quota classification. No readiness audit is persisted by the probe.

## Verification

- Audit-engine focused Rust tests: `46 passed, 0 failed`
- Full Rust library regression: `447 passed, 0 failed`
- GitHub tracking regression: `11 passed, 0 failed`
- Focused Audit Center and provider/workspace frontend tests: `11 passed, 0 failed`
- Full frontend Vitest regression: PASS
- TypeScript typecheck: PASS
- Rust `cargo check`: PASS
- Production frontend build: PASS
- Changed audit-engine formatter check: PASS
- Diff check: PASS
- Active-source architecture guardrail scan: PASS
- Repository-wide formatter check: existing unrelated `src-tauri/src/projects/registry.rs` formatting drift remains; that file was not touched.

## Governed native publication

The governed publication script completed successfully and rebuilt the native app from the implementation commit. The published artifact is:

`C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.exe`

- EXE SHA-256: `AFD1B38BEC5F7C85473AA2A528125900948E1731C1FEC2291562455DE7A83AB4`
- Desktop shortcut target: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.exe`
- Shortcut working directory: `C:\Users\sekip\Desktop\H!veAI\dev-bin`
- Shortcut icon: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.ico,0`
- No-terminal native smoke: PASS

## Publication record

The implementation was pushed normally to `origin/main` before publication. This log is immutable and is committed separately. Final local HEAD, `origin/main`, and live GitHub `main` are required to be verified equal after the log push, with a clean worktree.
