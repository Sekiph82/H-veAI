# M17 Claude Code Adapter V02 — Strict Lifecycle / Identity Remediation Prompt

## MANDATORY SYNC-FIRST / GITHUB-FIRST CONTRACT

Before reading or changing any repository file, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the worktree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or discard owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

Then read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/plans/M17_CLAUDE_CODE_ADAPTER_V01_PLAN.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V01_PROMPT.md`
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V01_LOG.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V01_STRICT_AUDIT.md`
- `docs/H!veAI/audits/M16T_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`
- `src-tauri/src/agent_adapter.rs`
- `src-tauri/src/agent_session_center.rs`
- `src-tauri/src/codex_adapter.rs`
- `src-tauri/src/codex_runtime.rs`
- `src-tauri/src/final_response.rs`
- `src-tauri/src/stream_sanitizer.rs`
- migration 24 and current `agent_sessions` / `agent_events` schema
- current Agents/session frontend and focused tests.

Every Codex-facing artifact and builder log must be entirely in English.

All H!veAI repository changes must be committed and pushed before completion. Final completion requires:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Local `HEAD`, `origin/main`, and live GitHub `main` must match and the worktree must be clean.

---

## WORK ITEM

- Work code: `M17`
- Version: `V02`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed audit: `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V01_STRICT_AUDIT.md`
- Required log: `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V02_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

V01 is `CHANGES_REQUIRED`. This is a bounded strict remediation package for F-M17-V01-001 through F-M17-V01-006 only. Do not activate M18 and do not broaden scope into unrelated roadmap work.

---

# FIRST ACTION — RECONCILE CANONICAL TRACKER TRUTH

Before substantive source changes, update `TASKS.md` and `CODEX_ROADMAP.md` so current prospective truth states:

- Current Milestone: `M17`
- Current Sprint: `M17-CLAUDE-ADAPTER`
- Current Task: `M17 V02 — Claude Code Adapter strict lifecycle / identity remediation`
- Current Task Status: `CHANGES_REQUIRED / REMEDIATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16 remains `PASS/CLOSED`
- M16T remains validated complete
- strict completed milestone count remains `17 / 20 = 85%`
- M17 remains `[~]` OPEN
- M18-M20 remain planned/blocked.

Reference the V01 strict audit as the authoritative reason for remediation. Do not rewrite historical V01 prompt/log/audit artifacts.

Commit and push this tracker transition before substantive implementation.

At builder completion, if all V02 gates pass, transition to:

- Current Task: `M17 V02 — Claude Code Adapter strict lifecycle / identity remediation`
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Required Actor: `HUMAN`
- M17 remains `[~]`
- M18 remains blocked.

Do not mark M17 `[x]`.

---

# ABSOLUTE PRESERVATION / SECURITY GUARDRAILS

Preserve all accepted M00-M16 behavior and all V01 behavior not implicated by the failed audit.

Required boundaries:

- local Claude Code CLI only;
- owner-managed Claude login only;
- no `ANTHROPIC_API_KEY` required, read, written, persisted, or used as fallback;
- no direct Anthropic HTTP/API transport;
- no Claude credential/auth/token-file inspection;
- no Claude desktop GUI automation;
- no browser automation to drive Claude;
- no automatic Claude installation/update;
- no shell command-string prompt transport;
- no `--dangerously-skip-permissions` or equivalent blanket permission bypass;
- no cross-project / cross-task / wrong-cwd exact-session resume;
- no unrelated Claude process termination;
- no weakening of M16 Codex audit-provider readiness, structured-output, or Codex-managed ChatGPT authentication;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` remain unchanged;
- M18 stays NOT ACTIVATED.

Do not redesign the full Agents stack. Prefer small, testable lifecycle primitives and focused changes.

---

# F-M17-V01-001 — FIX OWNED-PROCESS STOP LOCKING

The V01 monitor currently spawns a thread that obtains `Arc<Mutex<Child>>` and calls blocking `Child::wait()` while holding the mutex. `stop()` needs the same mutex before it can call kill/termination, so Stop can block behind the monitor until natural process exit.

Remediate this completely.

Requirements:

1. No thread may hold the shared Claude child mutex across an unbounded/blocking `wait()` while the process is running.
2. Use bounded `try_wait()` polling or another ownership-safe monitor primitive that releases the mutex between polls, analogous in safety to the mature Codex process lifecycle.
3. Streams must continue draining concurrently and safely.
4. `stop()` must be able to reach an owned running process promptly.
5. Stop only the PID/process tree owned by the H!veAI session.
6. Persist `STOP_REQUESTED`, `STOPPING`, terminal `STOPPED`, and escalation truthfully.
7. Do not label a hard `Child::kill()` as graceful if it is not a graceful provider operation. Record the actual termination method.
8. Bounded escalation may use the existing owned-PID Windows mechanism only after the initial owned-process termination path fails to produce an exit within the bounded grace window.
9. The monitor and stop path must not race into double-finalization or leave the session permanently STOPPING.

Add deterministic tests with a disposable long-running owned process fixture proving:

- Stop returns without waiting for natural process completion;
- the owned process terminates;
- terminal state is truthful;
- an unrelated process is not targeted;
- escalation evidence is correct when forced.

Do not consume live Claude quota for these tests.

---

# F-M17-V01-002 — MAKE RESUME SETUP FAIL-CLOSED AND DURABLE

V01 persists `STARTING` before spawn, but multiple later resume setup errors can escape through `?` without cleanup.

Implement one bounded resume-setup ownership/rollback discipline.

For every failure after the resume request has moved durable state to STARTING, including at least:

- process spawn;
- stdin acquisition;
- prompt write;
- stdout/stderr acquisition;
- process ownership registration;
- transition to RUNNING / control-plane persistence;

ensure all applicable cleanup occurs:

1. kill/terminate any child already spawned;
2. wait/reap it boundedly where required;
3. remove any ownership-map entry already inserted;
4. persist a truthful terminal state (`FAILED` unless a more specific accepted terminal state is warranted);
5. persist a bounded redacted diagnostic specific to the failure stage;
6. mark/materialize control-plane truth;
7. never leave a false STARTING/RUNNING row merely because setup returned an error;
8. never leave an unmonitored child behind.

Prefer a small RAII/setup guard or one explicit helper over many inconsistent early-return branches.

Add deterministic failure-injection tests for the important setup stages. The tests must prove both database truth and process ownership cleanup.

---

# F-M17-V01-003 — MAKE CLAUDE READINESS STRICTLY FAIL-CLOSED

`probe_claude_auth` must never contribute to READY unless the bounded auth-status process itself succeeds.

Requirements:

1. Require `output.status.success()` before accepting `loggedIn=true` as successful auth evidence.
2. Treat non-zero exit as failure even if stdout contains parseable JSON.
3. Build a bounded classifier over timeout, exit status, sanitized stdout, and sanitized stderr that can preserve explicit evidence for:
   - `AUTH_REQUIRED`
   - `AUTH_UNVERIFIED`
   - `USAGE_LIMITED` / `RATE_LIMITED`
   - `NETWORK_ERROR`
   - `TIMEOUT`
   - `PROCESS_ERROR`
4. Only explicit provider evidence may produce auth/usage/network-specific classifications.
5. Generic help text, progress text, or the mere presence of words such as `usage` must not become a quota classification.
6. Keep the stable auth-status command as the primary auth source if supported by installed Claude; do not inspect auth files.
7. Do not add an uncontrolled model turn merely to manufacture quota/network evidence. If a harmless end-to-end operational probe is introduced, it must run only in an isolated temporary directory with mutation-capable behavior constrained, be bounded, and have explicit justification/tests.
8. Readiness must not create an engineering session row or modify a registered project.

Add focused tests for:

- exit 0 + loggedIn true -> READY-eligible auth evidence;
- exit 0 + loggedIn false -> AUTH_REQUIRED;
- non-zero exit + misleading `{loggedIn:true}` -> never READY;
- explicit network evidence -> NETWORK_ERROR;
- explicit rate/quota evidence -> USAGE_LIMITED/RATE_LIMITED;
- timeout -> TIMEOUT;
- generic process failure -> PROCESS_ERROR;
- malformed/ambiguous output -> AUTH_UNVERIFIED.

---

# F-M17-V01-004 — MAKE PROVIDER SESSION ID IMMUTABLE

The first accepted Claude provider session ID for one H!veAI session is canonical and must not drift.

Implement a single identity-acceptance helper used by stream processing/finalization.

Requirements:

1. First valid provider session ID may populate the H!veAI session.
2. Repeated identical IDs are accepted idempotently.
3. Any later different provider session ID for the same H!veAI session is a hard identity conflict.
4. On conflict:
   - do not overwrite the canonical ID;
   - persist `CLAUDE_PROVIDER_SESSION_ID_MISMATCH` (or an equally explicit bounded code);
   - fail/degrade the session truthfully;
   - never make that run resumable against the conflicting ID.
5. Never ignore affected-row / compare-and-set failure and then overwrite the value in finalization.
6. Finalization may persist only the validated canonical ID.
7. Resume must use only that canonical ID.
8. Provider session IDs remain identifiers, not credentials.

Add deterministic tests for:

- first ID capture;
- same ID repeated across multiple records;
- mismatching later ID rejected;
- DB canonical ID unchanged after conflict;
- exact resume uses the unchanged canonical ID.

---

# F-M17-V01-005 — PERSIST LIVE ATTENTION STATE

When authoritative structured Claude output reports a provider state requiring owner attention while the process is still alive, H!veAI must not keep the canonical session row falsely RUNNING.

Required live state handling where provider evidence is explicit:

- `WAITING_PERMISSION`
- `WAITING_USER`
- `AUTH_REQUIRED`
- `USAGE_LIMITED`
- `NETWORK_ERROR`

Requirements:

1. Persist the state transition promptly when the parsed provider record is accepted.
2. Persist a bounded diagnostic/event explaining the attention state.
3. Mark/materialize project truth consistently.
4. Allow a later authoritative provider record to transition the same session forward where valid.
5. Stop must work from waiting states.
6. Do not invent waiting state from ambiguous prose.
7. Terminal finalization must not accidentally overwrite a stronger truthful failure/attention state with COMPLETED solely because process exit code is zero.
8. Frontend session detail/list must show the persisted current state and actionable diagnostic without fake placeholders.

Add deterministic tests for live WAITING_PERMISSION and WAITING_USER transition, stop-from-waiting, and later completion/failure behavior.

---

# F-M17-V01-006 — COMPLETE THE REQUIRED ADVERSARIAL TEST MATRIX

The V01 prompt required deterministic lifecycle/security coverage beyond the tests currently present.

At minimum V02 must add focused tests proving all remediations above plus the original invariants that are still under-covered:

- exact resume rejects wrong project;
- exact resume cannot cross task relationship;
- exact resume rejects canonical cwd mismatch;
- missing provider session ID is not resumable;
- provider session ID mismatch cannot replace canonical identity;
- resume setup failure leaves no false STARTING/RUNNING truth;
- stop reaches a running owned process and does not target unrelated processes;
- restart reconciliation marks lost process ORPHANED without auto-spawn;
- an eligible ORPHANED Claude provider session remains explicitly resumable;
- permission/wait states are live and truthful;
- quota/network/auth classifiers avoid false positives;
- stream redaction still precedes persistence/UI exposure;
- malformed provider records cannot become successful completion;
- Codex adapter/start/stop/list/final-response regressions remain green;
- M16 audit-provider readiness/freeform structured-output regressions remain green;
- exact-eight-project and FormuLab@main regressions remain green.

Tests must be deterministic and should not consume live Claude quota except for the bounded builder probes explicitly permitted by the original M17 contract.

---

# PROVIDER-NEUTRAL CONTRACT CLEANUP

During V02, address the V01 audit architecture note without unnecessary redesign.

`AgentAdapter` may keep provider-native identity opaque internally, but the contract must have one explicit, testable way to preserve/access the identity/provenance needed by lifecycle operations. Choose one:

A. extend the neutral `AdapterSession` DTO with provider session ID/provenance/canonical provider cwd; or
B. explicitly document provider-native identity as adapter-private durable state and add neutral lifecycle tests proving start/list/resume/reconcile preserve it correctly without a parallel contradictory identity source.

Do not create two competing canonical session models.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused M17 V02 stop monitor tests;
2. resume failure-injection tests;
3. readiness/auth classifier tests;
4. provider-session identity stability tests;
5. live attention-state tests;
6. original M17 focused Rust tests;
7. frontend Agents/session focused tests;
8. Codex adapter regression tests;
9. M16 audit-provider regression tests including V11 readiness/freeform behavior;
10. exact-eight-project and FormuLab@main tests;
11. full serialized Rust library regression under repository policy;
12. full frontend Vitest regression;
13. `npm run typecheck`;
14. `cargo check --manifest-path src-tauri/Cargo.toml`;
15. `npm run build`;
16. `git diff --check`;
17. active-source scan proving no `ANTHROPIC_API_KEY`, direct Anthropic HTTP/API provider, Claude auth-file inspection, GUI/browser automation, or blanket permission bypass;
18. existing M16 guardrail scan proving no OpenAI API-key/direct HTTP audit-provider regression;
19. governed native QA publication;
20. stable shortcut / no-visible-console checks.

Builder execution output remains claim evidence until independent audit.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V02_LOG.md`

Include:

- synchronized starting SHA;
- V02 tracker-transition SHA;
- implementation SHA(s);
- exact remediation mapping F-M17-V01-001 through F-M17-V01-006;
- stop-monitor ownership design;
- resume failure-cleanup design;
- readiness/auth classifier behavior;
- provider-session-ID immutability design;
- live attention-state transition behavior;
- provider-neutral identity/provenance contract decision;
- focused tests for each V01 finding;
- full test/typecheck/build/cargo-check results;
- M16/Codex regression evidence;
- forbidden-provider/auth/GUI/bypass scans;
- governed native publication EXE SHA-256 and shortcut evidence;
- final tracker state;
- final local HEAD / `origin/main` / live-main equality.

Commit the immutable log separately after implementation/publication evidence is complete. Do not require the log to contain the SHA of the commit that first creates itself.

---

# INDEPENDENT / OWNER CLOSURE GATE

Do not fabricate acceptance.

After V02 builder completion:

1. ChatGPT performs a new independent 20-section strict M17 V02 audit.
2. Only if V02 source audit passes does the owner perform native Claude acceptance.
3. Owner-native acceptance must then prove real Claude readiness, project-scoped start, structured stream/final response, Stop on a running session, exact-session resume, restart/orphan/resume if practical, and truthful attention/error behavior.
4. M17 remains OPEN until both strict audit and owner-native acceptance pass.
5. M18 remains blocked until M17 closes.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- V02 tracker truth is committed/pushed;
- F-M17-V01-001 through F-M17-V01-006 are closed in source and focused tests;
- Stop no longer blocks behind the monitor mutex;
- resume setup failures cannot leave false STARTING/RUNNING state or unmonitored children;
- readiness requires successful auth-probe process completion and preserves explicit failure classes;
- provider session ID is immutable once accepted;
- live waiting/attention state is durable and visible;
- provider-neutral lifecycle has one coherent identity/provenance source;
- accepted M16 behavior remains green;
- no forbidden Anthropic/OpenAI API-key/direct-provider path is introduced;
- required focused/full regressions and governed native publication pass;
- every repository change is committed and pushed;
- local HEAD == `origin/main` == live GitHub main;
- worktree is clean;
- M17 remains `[~]` awaiting independent re-audit and HUMAN acceptance;
- M18 remains blocked.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must contain only:

- GitHub M17 V02 log URL/path;
- V02 tracker-transition commit SHA;
- implementation commit SHA(s);
- V02 log commit SHA;
- final GitHub main SHA;
- concise status.