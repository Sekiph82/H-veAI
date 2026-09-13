# M17 Claude Code Adapter V01 — Codex Builder Log

## Scope and status

- Work code: M17 V01.
- Repository: `Sekiph82/H-veAI`, branch `main`.
- Required prompt: `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V01_PROMPT.md`.
- Authoritative plan: `docs/H!veAI/plans/M17_CLAUDE_CODE_ADAPTER_V01_PLAN.md`.
- M16 final owner acceptance used: `docs/H!veAI/audits/M16T_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`.
- Builder status: implementation complete; M17 remains `[~]` awaiting independent strict audit and owner-native Claude workflow acceptance.
- M18 was not activated.

## Synchronization and tracker transition

The standalone checkout was found at `C:\Users\sekip\Desktop\H!veAI`. The required pre-change checks showed a clean checkout at `c9227faa1a0cd2c88de6ee180e222f28f26dd78d`, four commits behind clean `origin/main` at `5bfe337a6423b4ebc82077a65e9d7cc593e10ffe`. Synchronization used only `git merge --ff-only origin/main`. No reset, rebase, force-push, stash, clean, destructive checkout, or divergent reconciliation was used.

The prospective tracker transition was committed and pushed before substantive implementation:

- Tracker-transition SHA: `f0b8bda739b03aa1be4a10b912371311b9d6a1c7`.
- Current truth became M17 / M17-CLAUDE-ADAPTER / M17 V01, M16 PASS/CLOSED, M16T validated complete, 17/20 = 85%, M17 `[~]`, and M18-M20 blocked/planned.

## Implementation

Implementation SHA: `bf81d73` (`feat: implement M17 Claude Code adapter`). It is pushed to `origin/main`.

The implementation:

- Moves `AgentProvider`, `AgentAdapter`, and shared adapter DTOs into `src-tauri/src/agent_adapter.rs`; Codex continues to implement the same contract and keeps its existing process, final-response, retry, stop, and audit behavior.
- Adds forward-only migration 24, `agent_provider_session_provenance`, for the provider session ID, provenance JSON, canonical cwd identity, and provider identity index. Existing Codex and historical Claude rows remain compatible.
- Uses only a directly invokable native Claude Code executable resolved from bounded candidates. The installed CLI evidence was `2.1.270 (Claude Code)`.
- Derives the required invocation surface from bounded `--help` evidence: print mode, stream JSON, exact resume, continue capability, permission mode, permission-prompt policy, and restricted mode. Start and resume pass prompts through bounded stdin and never use shell command strings or prompt arguments.
- Verifies managed login only through bounded `claude auth status --json` parsing of the boolean login result. Raw auth output is not persisted, auth storage is not inspected, and no API-key or direct provider transport was added.
- Persists the provider-native Claude session ID from structured output and resumes only by that exact ID after project, task, canonical cwd, provider, and active-session checks. H!veAI `Resume exact session` is the deterministic continue action; the CLI's global most-recent `--continue` behavior is not exposed because it cannot prove project/session identity.
- Parses Claude structured records into bounded H!veAI events for metadata/session identity, assistant text/final response, tool-use/result, permission and user attention, rate/usage limits, provider errors, malformed records, stderr, exit status, and truncation/degradation. Malformed structured output cannot become successful completion.
- Keeps permission handling fail-closed with `--permission-prompts none`, `--permission-mode plan`, and `--restricted`; it does not use a blanket permission bypass. Observable permission, auth, usage, network, user-attention, orphan, failed, stopped, and completed states remain distinct.
- Uses owned-process stop with a bounded direct termination attempt followed only by bounded escalation against the owned PID tree. Restart reconciliation marks no-longer-owned active Claude sessions `ORPHANED` and never auto-resumes them.
- Exposes Claude readiness, version, capability flags, attention state, final response, provider session identity, canonical cwd, and exact-session resume in the Agents UI.

No accepted M16 Codex audit provider behavior was changed. No M18 work was introduced.

## Verification evidence

- `cargo test --manifest-path src-tauri/Cargo.toml --lib agent_session_center`: 13 passed.
- Serialized full Rust gate: 451 passed, 0 failed, 0 ignored (`-- --test-threads=1`).
- `npm test -- --run`: 18 files, 140 tests passed.
- `npm run typecheck`: passed.
- `npm run build`: passed; Vite production bundle generated.
- `cargo check --manifest-path src-tauri/Cargo.toml`: passed.
- Claude probes: native `claude --version` returned `2.1.270 (Claude Code)`; bounded help output proved the required M17 flags; bounded auth status reported logged in. No credential or token file was read.
- Guardrail scan of active source found no API-key transport, direct Anthropic HTTP/API transport, GUI/browser automation, or blanket permission bypass. The only auth-file wording found was in an existing Codex regression-test name, not an implementation path.
- Focused UI verification covered `READY`, capabilities, `ORPHANED`, and exact provider-session resume dispatch.

## Governed native publication

`scripts/publish-dev-qa.ps1` completed successfully from the implementation commit. It performed the no-bundle release build, candidate PE validation, candidate smoke launch, stable swap, stable smoke launch, no-forbidden-port check, no-visible-console check, and shortcut verification.

- Stable executable: `dev-bin/H!veAI.exe`.
- Stable executable SHA-256: `3A67D61F7E1F08B015DB4FAB42EBD7265652937EBDD41D4C78A7B95AD93BBA9C`.
- Desktop shortcut target: `dev-bin/H!veAI.exe`.
- Desktop shortcut icon: `dev-bin/H!veAI.ico,0`.

## Final builder transition

After implementation and publication evidence, the canonical tracker was changed to `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`, while M17 remains `[~]`. The log is builder evidence, not independent acceptance. The subsequent log/tracker commit SHA and final live-main equality are recorded by the completion response after push and verification.
