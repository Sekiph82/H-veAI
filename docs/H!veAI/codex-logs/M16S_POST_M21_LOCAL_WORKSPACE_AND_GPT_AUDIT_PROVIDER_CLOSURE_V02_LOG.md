# M16S Post-M21 Local Workspace and GPT Audit Provider Closure V02

## Scope

- Work code: `M16S V02`
- Prompt: `docs/H!veAI/prompts/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V02_PROMPT.md`
- Strict audit: `docs/H!veAI/audits/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V02_AUDIT.md`
- Synchronized starting GitHub SHA: `20babddbab0b5ccf0b151af2c06f2a3f66adf6bd`
- Implementation commit SHA: `8a9e2d29636728704d8cdd2ff7f2db5967f3f817`

## Finding M16S-V02-F01

Implemented a separate cheap local configuration inspection and an explicit native live readiness check. The Settings page does not spend a provider request on load. The user-triggered native command reads the configured model and `OPENAI_API_KEY` only inside the Rust process, performs a bounded HTTPS request, and returns only non-secret readiness metadata.

The official contract is `GET https://api.openai.com/v1/models/{model}`. It is appropriate because it validates the credential and the configured model entitlement without creating an audit run or generating model output. The request uses native `reqwest` HTTPS, a 5 second connect timeout, a 10 second total timeout, a bounded UTF-8 response, and truthful classifications for ready, authentication failure, rate limiting, model unavailability, and transport/provider failure.

Changed source symbols include `AuditHttpTransport::get_model`, `ReqwestAuditTransport::readiness`, `ReqwestAuditTransport::get_model`, `read_bounded_response`, `audit_provider_readiness`, `check_readiness_with_transport`, `check_audit_provider_readiness`, `hiveai_audit_provider_check_readiness`, `AuditProviderSettings`, and the TypeScript `checkAuditProviderReadiness` bridge. The readiness path does not call audit creation, does not persist an audit or provider response, does not persist the API key or Authorization header, and does not return the key or raw provider body.

Deterministic mocked transport tests cover READY, AUTH_ERROR, RATE_LIMITED, NETWORK_ERROR, MODEL_UNAVAILABLE, invalid model input, bounded response failure, and the no-audit-persistence boundary.

## Finding M16S-V02-F02

Provider identity persistence now comes from the executing `AuditModel` boundary after semantic validation. `parse_model_output` ignores generated `model` and `modelVersion` for persisted identity, `evaluate_with` writes `model.provider()`, `model.model_name()`, and `model.version()`, and `unavailable_evaluation` uses the same trusted runtime metadata for degraded/provider-failure results. The structured-output schema no longer asks the model to author runtime identity fields, while the parser remains backward-compatible with historical optional fields.

Changed source symbols include `parse_model_output`, `evaluate_with`, `unavailable_evaluation`, `audit_result_schema`, and the `RawModelOutput` compatibility fields. Tests prove that spoofed and null generated identity cannot replace runtime identity, and that the trusted provider/model/version survives persistence and reload, including malformed-result handling.

## Finding M16S-V02-F03

The Projects card now exposes a visible compact `Attach local workspace`, `Change local workspace`, or `Repair local workspace` action with the existing safe repair callback. The registry dialog derives matching operation wording and helper text from the current project state, preserving the existing project identity and repair flow without creating duplicates.

Changed source symbols include `ProjectRegistryCard` path-action rendering and `ProjectRegistryDialog` operation copy. Frontend coverage proves attach/change behavior and the missing-path repair label.

## Verification

- Focused Rust: `cargo test --manifest-path src-tauri/Cargo.toml audit_engine::tests` — 34 passed, 0 failed.
- Focused frontend: `npm test -- --run tests/m16s-provider-and-workspace-focused.test.tsx tests/m16-audit-center-focused.test.tsx` — 7 passed, 0 failed.
- Full frontend: `npm test -- --run` — 17 files, 135 tests passed, 0 failed.
- Full Rust: `cargo test --manifest-path src-tauri/Cargo.toml` — 427 passed, 0 failed.
- Typecheck: `npm run typecheck` — passed.
- Production frontend build: `npm run build` — passed.
- Diff check: `git diff --check` — passed.

## Native publication

The governed publisher `scripts/publish-dev-qa.ps1` rebuilt and published the stable native executable, updated the desktop shortcut, ran candidate/stable smoke checks, verified the frontend readiness marker, and verified no visible terminal process was left by the publication flow.

- Published executable SHA-256: `5496E9E30647B1E36229D2CAA18AEB31AA6897E8293F4C63723151815E536C1B`
- Native shortcut target and icon validation passed for the published executable.
- The native readiness boundary is explicit user action only; no owner API key was requested, printed, persisted, or exposed.

## Governance

M16 remains `OPEN` pending independent strict audit and owner native acceptance. M17 remains `BLOCKED` and not activated. No Claude implementation was done. Historical M16S V01 prompt, audit, and log artifacts were not rewritten.

Final status: `READY_FOR_INDEPENDENT_M16S_AUDIT`
