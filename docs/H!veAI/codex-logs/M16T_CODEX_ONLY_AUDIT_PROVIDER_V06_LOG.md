# M16T Codex-only Audit Provider V06 Native Acceptance Remediation Log

- Work code: `M16T`
- Version: `V06`
- Date: `2026-09-13`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Synchronized starting GitHub SHA: `f6d10d3710fb550683e99d4864bc74bdf588d636`
- Implementation commit: `657dabc55c6c56efd1e0b37fd70282d3c07341f6`

## Scope and preserved boundaries

This run addressed only the failed V06 native-acceptance findings: readiness ACL/invoke truthfulness, degraded audit-state truthfulness, and the input-aware project/freeform Codex output contract. The accepted Codex-only provider architecture remains in place: production model execution uses the locally installed Codex CLI with the owner's existing ChatGPT login. No `OPENAI_API_KEY`, direct OpenAI HTTP audit transport, API-key authentication or fallback, Codex auth-file access, Claude/M17 work, or broad shell/filesystem capability was introduced.

The accepted V05 FormuLab `main` mapping and exact eight-project portfolio were preserved. M16 remains open pending independent strict audit and owner native re-acceptance; M17 remains not activated and blocked.

## Finding V06-F01 — readiness ACL and false executable diagnosis

The native invoke handler already registered both readiness commands, but the bounded `allow-audit-engine` permission omitted them. Settings therefore received an ACL rejection and the frontend's generic failure path presented `CODEX_NOT_FOUND`, even though the audit path could record `CODEX_CLI / CLI_DEFAULT`.

The existing narrow permission now includes exactly `hiveai_audit_provider_readiness` and `hiveai_audit_provider_check_readiness`. The frontend maps invoke/ACL rejection to an explicit `UNAVAILABLE`/readiness error and reserves `CODEX_NOT_FOUND` for a native readiness result that actually reports that status. Direct capability and frontend tests cover registration, ACL membership, rejection mapping, and successful metadata rendering. No unrelated permission was added.

## Finding V06-F02 — degraded audit state truthfulness

Semantic-invalid output previously produced a `MALFORMED` evaluation while the run-state logic could still persist `COMPLETED`; the UI then displayed `Audit completed with structured evidence.` for every model status other than `UNAVAILABLE`. This contradicted the stored diagnostic and falsely implied authoritative acceptance.

Run-state selection is now canonical: freshness change remains authoritative as `STALE`; only a semantically valid `AVAILABLE` model result can be `COMPLETED`; degraded statuses persist safely as `FAILED` unless a more-specific contractual state applies. Persisted diagnostics and fallback verdict/history remain available. The UI explicitly renders the model status and bounded diagnostic for degraded runs, removes success wording, and keeps failed history selectable.

## Finding V06-F03 — project/freeform Codex contract

The previous host-to-Codex prompt did not deterministically constrain a freeform project audit to the input-dependent semantic contract. A schema-valid empty coverage array could therefore reach semantic validation and become `MALFORMED`, with no visible exact contract diagnostic.

The Codex prompt now branches on the input shape. With empty requirements it requires exactly one `project-audit` coverage row with `NOT_APPLICABLE`, empty evidence references, bounded text/rationale, and no invented task requirement IDs. Task-shaped prompts enumerate the canonical required references without allowing invented, omitted, or duplicate IDs. The semantic validator remains strict and now exposes the exact project-coverage diagnostic. Direct fixtures cover input -> prompt -> final JSON -> parse -> semantic validation -> persistence, plus invalid task/project shapes.

## Evidence and gate results

- Focused audit-engine Rust tests: `36 passed; 0 failed`.
- Capability/ACL Rust tests: `2 passed; 0 failed`.
- Focused Audit Center frontend tests: `4 passed; 0 failed`.
- Full frontend regression: `17` files, `136` tests passed.
- Deterministic full Rust library regression: `437 passed; 0 failed` with one test thread.
- `cargo check --manifest-path src-tauri/Cargo.toml`: PASS; warnings only.
- `npm run typecheck`: PASS.
- `npm run build`: PASS.
- `git diff --check`: PASS.
- Codex-only/OpenAI API-key guardrail search: PASS; no prohibited active-source match.
- The default concurrent Rust run reproduced one pre-existing watcher timing race; its isolated test passed, and the required deterministic one-thread full regression passed without failure.

The V06 prompt's 108 gates are covered by the focused ACL/readiness, degraded-persistence, semantic-contract, UI, Codex-runtime, M16-audit, V05 portfolio, full Rust/frontend, typecheck, build, diff, guardrail, and governed-publication checks above. No live owner Codex acceptance operation was performed; that native acceptance is intentionally pending for the owner/independent auditor.

## Governed native publication

The governed publisher built, staged, smoke-tested, and promoted the real stable native executable. Startup readiness completed without a visible console host or forbidden development-port listener, and the desktop shortcut was verified against the stable standalone target.

- Published executable: `dev-bin/H!veAI.exe`
- SHA-256: `3F89B173B7224A793AB1FA14355AB1A015DDBE2C1F5DA686819F7C8D25F7F44F`
- Shortcut target: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.exe`
- Shortcut working directory: `C:\Users\sekip\Desktop\H!veAI\dev-bin`
- Shortcut icon: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.ico,0`

## Tracker transition

- Current milestone: `M16`
- Current sprint: `M16T-CODEX-ONLY`
- Current task: `M16T V06 - Codex-only audit provider native acceptance remediation`
- Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_REACCEPTANCE`
- Next action: independent V06 strict audit, then owner native readiness and real Codex acceptance
- Required actor: `HUMAN`
- M16T: `[~]`; M16: `OPEN`; M17: `NOT ACTIVATED / BLOCKED`
- Strict milestone progress: `16 / 20 = 80%`

All H!veAI changes for V06 are committed locally and the implementation commit is pushed to `origin/main`. The immutable log commit and final local/origin/live-main equality proof are recorded by the completion response after the log commit.
