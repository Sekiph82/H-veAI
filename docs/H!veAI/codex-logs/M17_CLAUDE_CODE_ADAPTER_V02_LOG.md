# M17 Claude Code Adapter V02 — Codex Builder Log

## Scope and status

- Work code: M17 V02.
- Repository: `Sekiph82/H-veAI`, branch `main`.
- Required prompt: `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REMEDIATION_PROMPT.md`.
- Authoritative failed audit: `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V01_STRICT_AUDIT.md`.
- Scope: F-M17-V01-001 through F-M17-V01-006 only.
- Builder status: implementation complete; M17 remains `[~]` awaiting independent V02 strict re-audit and owner-native Claude acceptance.
- M18 was not activated.

## Synchronization and tracker transitions

The standalone checkout was `C:\Users\sekip\Desktop\H!veAI`. The required safe preflight found a clean local checkout behind clean `origin/main`. Synchronization used only `git fetch origin main` followed by `git merge --ff-only origin/main`; no reset, rebase, force-push, stash, clean, destructive checkout, or divergent reconciliation was used.

- Synchronized starting SHA: `27d852df29bcb7d20c3709f4e948131e3a745978`.
- V02 tracker-transition SHA: `e0e80b1aa3a53ef3bf9e84d6176242d60aef547d`.
- Final builder tracker-transition SHA: `eaab418e68b8b3976cc5056f5aa6f89d6743db1e`.

The first tracker transition changed current truth to M17 V02 `CHANGES_REQUIRED / REMEDIATION_IN_PROGRESS`, required actor `CODEX`, M16 `PASS/CLOSED`, M16T validated complete, strict progress `17 / 20 = 85%`, M17 `[~]`, and M18-M20 planned/blocked. After implementation, regression, publication, and guardrail gates passed, the final tracker transition changed current truth to `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`, required actor `HUMAN`, while M17 remained `[~]` and M18 remained blocked.

## Implementation commits

- Implementation SHA: `21f6eb57c6b80dbb2c8200a7a53d077467f4042a`.
- Final tracker SHA: `eaab418e68b8b3976cc5056f5aa6f89d6743db1e`.

The implementation preserves the accepted M16 Codex-only audit provider, M00-M16 behavior, exact eight-project portfolio, `Sekiph82/FormuLab@main` tracking, and historical V01 prompt/log/audit artifacts.

## Exact remediation mapping

### F-M17-V01-001 — owned-process stop locking

The Claude monitor now polls `Child::try_wait()` and releases the shared child mutex between polls. It never holds the mutex across blocking `Child::wait()`. The stop path acquires the owned session entry, records `STOP_REQUESTED`, performs bounded owned-child termination, and uses the existing owned-PID Windows tree escalation only after the bounded initial path fails. The initial hard child kill is recorded as non-graceful; forced tree termination is recorded as `STOP_ESCALATED`. The run monitor performs one terminal finalization and removes only the session's ownership-map entry.

The deterministic disposable-process test proves stop reaches a still-running owned fixture promptly, the owned process terminates, the monitor observes terminal exit, and a separately spawned unrelated process remains alive until test cleanup.

### F-M17-V01-002 — resume setup failure truthfulness

Resume now uses one bounded setup ownership path. After durable `STARTING`, every setup stage is named and failures run the same cleanup: kill and boundedly reap an already spawned child, remove an inserted ownership-map entry, persist `FAILED`, append a stage-specific bounded redacted diagnostic, and materialize control-plane truth. The child is not handed to the monitor until all streams, ownership registration, and `RUNNING` persistence succeed. The deterministic cleanup test proves process termination, ownership removal, `FAILED` database truth, and the exact prompt-write diagnostic event.

The start path also closes the equivalent ownership-registration and `RUNNING` persistence cleanup gap without changing Codex lifecycle code.

### F-M17-V01-003 — fail-closed readiness/auth classification

`probe_claude_auth` now requires successful auth-status process completion before `{loggedIn:true}` can become authenticated evidence. A non-zero process with misleading authenticated JSON becomes `PROCESS_ERROR`, never `READY`. The bounded classifier distinguishes `AUTH_REQUIRED`, `AUTH_UNVERIFIED`, `USAGE_LIMITED`, `NETWORK_ERROR`, `TIMEOUT`, and `PROCESS_ERROR` from sanitized stdout/stderr and exit/timeout evidence. Quota classification requires explicit rate/quota evidence; generic `usage` text does not classify as limited. The stable `claude auth status --json` command remains the only auth source and no auth storage is inspected.

### F-M17-V01-004 — immutable provider session identity

The neutral `AdapterSession` DTO now explicitly carries provider session identity, provenance, and canonical provider cwd. Claude stream processing and finalization use one compare-and-set identity helper. The first valid provider session ID is canonical, repeated identical IDs are idempotent, and a different later ID produces `CLAUDE_PROVIDER_SESSION_ID_MISMATCH`, preserves the canonical database value, fails the affected run truthfully, and never makes it resumable against the conflicting ID. First capture also updates the durable provenance JSON with the same canonical identity. Exact resume builds `--resume` from that persisted canonical ID only.

### F-M17-V01-005 — durable live attention state

Authoritative parsed Claude states `WAITING_PERMISSION`, `WAITING_USER`, `AUTH_REQUIRED`, `USAGE_LIMITED`, and `NETWORK_ERROR` now update the canonical `agent_sessions.state` while the process is alive, append bounded `ATTENTION_STATE` diagnostics, mark truth dirty, and materialize project truth. Later authoritative `RUNNING` evidence can return the same session to `RUNNING`; stop accepts all live attention states and transitions through `STOPPING` to truthful terminal `STOPPED`. Finalization preserves a stronger live attention state when the provider exits successfully without a later authoritative completion record. Session reload exposes the persisted diagnostic to the frontend.

The frontend focused test verifies that a persisted live `WAITING_PERMISSION` state and actionable diagnostic are visible in the Agents session detail.

### F-M17-V01-006 — adversarial lifecycle/security matrix

Added deterministic focused coverage for stop ownership, unrelated-process protection, resume cleanup, auth evidence classes and false-positive avoidance, provider identity CAS and provenance, exact resume argument identity, missing-ID and canonical-cwd rejection, live permission/user-wait transitions, later return to running, malformed result failure mapping, final-response preservation, and redaction. Existing cross-project/task/cwd, orphan reconciliation, Codex, M16 audit-provider, portfolio, and FormuLab regression tests remain in the full gates.

## Provider-neutral identity/provenance decision

V02 selects the prompt's option A. `AdapterSession` now exposes `provider_session_id`, `provider_session_provenance`, and `provider_cwd_identity` as neutral durable lifecycle fields. Claude owns the provider-native identity semantics through the shared compare-and-set helper; Codex remains behaviorally unchanged and returns neutral empty provider-session identity with its canonical cwd.

## Verification evidence

- Focused Rust M17 session tests: `cargo test --manifest-path src-tauri/Cargo.toml --lib agent_session_center -- --test-threads=1` — 19 passed, 0 failed.
- Focused frontend Agents/session tests: `npm test -- --run tests/m17-claude-adapter-focused.test.tsx` — 2 passed, 0 failed.
- Full serialized Rust library regression: `cargo test --manifest-path src-tauri/Cargo.toml --lib --quiet -- --test-threads=1` — 457 passed, 0 failed, 0 ignored.
- Full frontend Vitest regression: `npm test -- --run` — 18 files, 141 passed, 0 failed.
- TypeScript: `npm run typecheck` — passed.
- Frontend production build: `npm run build` — passed.
- Rust compile gate: `cargo check --manifest-path src-tauri/Cargo.toml` — passed.
- Diff hygiene: `git diff --check` — passed at tracker and implementation commits.
- Exact eight-project portfolio and `Sekiph82/FormuLab@main` tests were included in the full Rust regression and remained green.
- Codex adapter/start/stop/list/final-response regressions and M16 audit-provider readiness/freeform structured-output regressions remained green in the full frontend/Rust gates.

## Security and boundary scans

Active-source scans found zero `ANTHROPIC_API_KEY`, direct Anthropic HTTP/API transport, `--dangerously-skip-permissions`, browser/GUI automation, or credential-file inspection matches. The only `auth.*file` source match is the pre-existing Codex regression-test name `login_status_classification_never_reads_auth_files`; it is not an implementation path. The M16 OpenAI API-key/direct-provider guardrail scan found zero matches in `src-tauri/src`.

Claude uses only the directly resolved local native CLI, bounded `--version`/`--help`/`auth status --json` probes, the owner's Claude-managed login, bounded stdin prompt transport, `--permission-mode plan`, `--permission-prompts none`, and `--restricted`. No Claude auth/token file is read or persisted, no API key fallback exists, and no GUI/browser automation or blanket permission bypass was introduced.

## Governed native publication

`powershell -NoProfile -ExecutionPolicy Bypass -File scripts/publish-dev-qa.ps1` passed. The helper performed the no-bundle release build, PE validation, candidate smoke launch, stable swap, stable smoke launch, embedded frontend readiness, forbidden-port check, no-visible-console check, and shortcut validation.

- Stable executable: `dev-bin/H!veAI.exe`.
- Stable executable SHA-256: `7DE3F84358FEF9F1F96DFD15949874C2F90852FDB1BDE4035230414F28401E50`.
- Desktop shortcut target: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.exe`.
- Desktop shortcut icon: `C:\Users\sekip\Desktop\H!veAI\dev-bin\H!veAI.ico,0`.

## Final builder state

- M16: `PASS/CLOSED`.
- M16T: validated complete; Codex-only audit provider preserved.
- M17: `[~]`, implementation complete, awaiting independent V02 strict re-audit and HUMAN owner-native Claude acceptance.
- M18: not activated and remains blocked.
- Exact eight-project portfolio and `Sekiph82/FormuLab@main` tracking: unchanged.
- Pre-log local HEAD, `origin/main`, and live GitHub `main` equality: `eaab418e68b8b3976cc5056f5aa6f89d6743db1e`.

This builder log is committed separately after implementation and publication evidence. Its creating commit SHA is intentionally not required to appear inside the self-created artifact; the post-log equality is verified by the completion gate and final response.
