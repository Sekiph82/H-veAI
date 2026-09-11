# M16T Codex-only Audit Provider V01 Builder Log

- Work item: M16T V01 — Codex-only audit provider migration
- Execution date: 2026-09-12
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Synchronized starting GitHub SHA: `98606ce44199367a3974d9b5824962c0beb4b6d4`
- Tracker-transition commit: `060f2ebd9b110e81a2332ab50208d10ef4ca1a6e`
- Implementation commit: `b61ed662a8f893c6127138430c33f066e3ebb891`
- Completion-tracker commit: `0b3c1ca90ac999993e725fe91db34e9dfd6dc016`

## Scope and root causes

The active M16S audit provider was an OpenAI HTTP/API-key path with configurable model state, while the existing Codex adapter already contained the native executable and process-safety primitives needed by the owner-directed architecture. M16T removes that active provider boundary and makes the local Codex CLI the only production audit model, with an explicit unavailable model when the CLI cannot be verified.

## Implemented boundary

- Added one shared native Codex runtime for deterministic executable resolution, PE validation, bounded version/login probes, and bounded stdout/stderr process capture.
- Reused the resolver/process foundation for the existing Codex agent adapter and removed the hard-coded `gpt-5.5` override; the runtime model identity is `CLI_DEFAULT`.
- Production audit execution is `codex exec --ephemeral --json --sandbox read-only --skip-git-repo-check --ignore-user-config --ignore-rules --color never --cd <dedicated-temp-dir> --output-last-message <bounded-final-file>` with `--output-schema <schema-file>` for audit results. No shell wrapper or project working directory is used.
- The dedicated final-message file is authoritative. Generic stdout/stderr are bounded operational evidence only; timeout, nonzero exit, truncation, missing final output, malformed output, authentication, usage, network, and process failures remain truthful failures/unavailable states.
- Cheap readiness uses only `codex --version` and `codex login status`. `Logged in using ChatGPT` is recognized; API-key authentication is policy-blocked; unknown login state stays unverified. The explicit Check readiness action uses a tiny deterministic `READY` turn and never persists an audit.
- Removed the active OpenAI HTTP audit model, API-key/model settings command and UI, direct audit transport dependency, and stale GPT-provider labels from the active product surface. Historical prompts, audits, and logs remain immutable provenance.
- Preserved strict audit schema/semantic/freshness/persistence behavior, M09/M16 history integration, local workspace controls, native startup/process boundaries, and the existing project portfolio contract.

## Security and credential proof

- `OPENAI_API_KEY` was not requested, read, set, persisted, or exposed.
- No OpenAI HTTP audit URL, API-key lookup, API-key settings command, or OpenAI audit transport remains in operative source, tests, or current tracker content. The authoritative M16T plan retains negative requirements and historical terminology as immutable contract/provenance text.
- No Codex auth files, tokens, or credential stores were read.
- Installed CLI evidence: `codex-cli 0.153.4`; `codex login status` reported `Logged in using ChatGPT`.
- Automated provider tests use deterministic mock process runners and do not consume a real Codex turn or quota.

## Verification gates

- `cargo check --manifest-path src-tauri/Cargo.toml`: PASS.
- Native audit/provider Rust tests: PASS, 32/32 focused tests.
- Full Rust library regression: PASS, 426/426 tests.
- `npm run typecheck`: PASS.
- Full frontend regression: PASS, 17/17 test files and 135/135 tests.
- `git diff --check`: PASS.
- Active forbidden-provider search over source, tests, and current tracker files: PASS; only the M16T plan's explicit prohibition language and the Codex install directory name remain outside the operative provider path.
- Governed native publication `scripts/publish-dev-qa.ps1`: PASS. The real Tauri production build completed, candidate and stable PE validation passed, startup readiness marker passed, forbidden dev-port check passed, no visible console host was detected, and the desktop shortcut was rewritten and verified against the stable standalone executable/icon.

## Published native artifact

- Stable executable: `dev-bin/H!veAI.exe`
- EXE SHA-256: `5496E9E30647B1E36229D2CAA18AEB31AA6897E8293F4C63723151815E536C1B`
- Shortcut target verified: standalone `dev-bin/H!veAI.exe`
- Shortcut icon verified: standalone `dev-bin/H!veAI.ico,0`

## Governance status

M16T V01 is implementation-complete and remains pending independent strict audit and owner native Codex acceptance. M16 remains OPEN. M17-M20 remain NOT ACTIVATED/BLOCKED. No Claude/M17 work, M21 work, installer, or M16 closure was performed.
