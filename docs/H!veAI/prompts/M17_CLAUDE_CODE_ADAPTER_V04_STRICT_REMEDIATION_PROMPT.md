# M17 Claude Code Adapter V04 — Strict Action-Truth / Claim / Terminalization Remediation Prompt

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
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V01_STRICT_AUDIT.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REMEDIATION_PROMPT.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V02_STRICT_REAUDIT.md`
- `docs/H!veAI/prompts/M17_CLAUDE_CODE_ADAPTER_V03_STRICT_REMEDIATION_PROMPT.md`
- `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V03_LOG.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V03_STRICT_REAUDIT.md`
- `docs/H!veAI/audits/M16T_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`
- current `src-tauri/src/agent_session_center.rs`
- current `src-tauri/src/agent_adapter.rs`
- current `src-tauri/src/codex_adapter.rs`
- current Agents/session frontend and focused tests
- current Tauri startup/reconcile wiring.

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
- Version: `V04`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Authoritative failed audit: `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V03_STRICT_REAUDIT.md`
- Required log: `docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V04_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

V03 is `CHANGES_REQUIRED`. This is one bounded remediation package for F-M17-V03-001 through F-M17-V03-005 only. Do not activate M18 and do not broaden scope into unrelated roadmap work.

---

# FIRST ACTION — RECONCILE CANONICAL TRACKER TRUTH

Before substantive source changes, update `TASKS.md` and `CODEX_ROADMAP.md` so current prospective truth states:

- Current Milestone: `M17`
- Current Sprint: `M17-CLAUDE-ADAPTER`
- Current Task: `M17 V04 — Claude Code Adapter action-truth / claim / terminalization remediation`
- Current Task Status: `CHANGES_REQUIRED / REMEDIATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M16 remains `PASS/CLOSED`
- M16T remains validated complete
- strict completed milestone count remains `17 / 20 = 85%`
- M17 remains `[~]` OPEN
- M18-M20 remain planned/blocked.

Reference the V03 strict re-audit as the authoritative reason for V04. Historical V01/V02/V03 prompt, log, and audit artifacts are immutable.

Commit and push this tracker transition before substantive implementation.

At builder completion, if and only if every V04 gate passes, transition to:

- Current Task: `M17 V04 — Claude Code Adapter action-truth / claim / terminalization remediation`
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_REAUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Required Actor: `HUMAN`
- M17 remains `[~]`
- M18 remains blocked.

Do not mark M17 `[x]`.

---

# ABSOLUTE PRESERVATION / SECURITY GUARDRAILS

Preserve all accepted M17 improvements not implicated by the V03 strict re-audit:

- centralized backend Claude process-live / stop / restart-reconcile / resume state vocabulary;
- no blocking `Child::wait()` while holding the shared Claude child mutex;
- bounded owned-process termination and unrelated-process protection;
- fail-closed Claude auth readiness with explicit readiness classes;
- immutable first provider-native Claude session ID with mismatch rejection;
- provider session ID/provenance/canonical cwd in the neutral adapter DTO;
- restart reconciliation across STARTING, RUNNING, WAITING_PERMISSION, WAITING_USER, AUTH_REQUIRED, USAGE_LIMITED, NETWORK_ERROR, STOPPING;
- live-state no-op coalescing concept;
- retryable stop-failure concept;
- accepted Codex and M16 audit-provider behavior.

Required boundaries remain:

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
- no weakening of M16 Codex audit-provider readiness/structured-output contracts;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` remain unchanged;
- M18 stays NOT ACTIVATED.

Do not redesign the full Agents stack. Prefer small lifecycle/action-eligibility primitives and deterministic injectable tests.

---

# F-M17-V03-001 — MAKE SESSION-LEVEL `supportsResume` MATCH THE REAL RESUME CONTRACT

Current list/DTO projection can advertise `supportsResume=true` based only on provider, provider session ID, durable state, and no process ownership. Actual `resume()` additionally rejects invalid current project path/status, task relationship, and canonical provider cwd.

This must become one source of truth.

## Required behavior

Create one backend session-level exact-resume eligibility evaluator, or equivalent tightly governed primitive, used by both:

- session list/DTO projection that produces `supportsResume` or an equivalent current-action field; and
- the `resume()` preflight.

Current exact-resume eligibility must require all of the following:

1. provider is CLAUDE;
2. canonical provider session ID exists;
3. durable session state is resume-eligible;
4. no H!veAI-owned process exists for the session;
5. no resume claim is active;
6. registered project exists and is currently valid/active according to existing project-operation policy;
7. current canonical project path is available;
8. persisted provider cwd identity exactly matches the current canonical cwd;
9. persisted task relationship, when present, still belongs to that same project;
10. no other accepted resume provenance invariant is violated.

The UI must not render an enabled `Resume exact session` when any of those current prerequisites fail. Do not make the frontend independently reimplement filesystem/task validation. Consume backend current eligibility truth.

Add deterministic tests for:

- valid ORPHANED row -> resume eligible;
- valid STOPPED / FAILED / COMPLETED rows according to policy;
- wrong canonical cwd -> list/DTO `supportsResume=false` and `resume()` rejects consistently;
- missing provider session ID -> false;
- task/project mismatch -> false;
- unavailable/inactive project -> false;
- owned live process -> false;
- active resume claim -> false;
- frontend hides Resume whenever backend session eligibility is false.

---

# F-M17-V03-002 — MAKE RESUME CLAIM OWNERSHIP RAII / FAIL-SAFE

Current `resume()` acquires a per-session resume claim and then has at least one direct fallible `?` path before the existing cleanup helper is guaranteed to run. A database-open failure can therefore leak the claim in memory without spawning a child or transitioning durable state.

Remediate the claim as a real ownership resource.

Requirements:

1. Once a resume claim is acquired, every return path must release it unless ownership has been intentionally and atomically transferred to the process map.
2. Prefer a small RAII guard whose `Drop` releases the claim unless explicitly disarmed after process ownership is registered.
3. Do not manually scatter release calls across future error branches if a guard can make correctness structural.
4. A pre-STARTING failure must not mutate the durable session into FAILED merely because the temporary claim failed; preserve the previously truthful terminal/orphaned state unless the accepted contract requires another explicit result.
5. A post-STARTING setup failure must retain the existing fail-closed cleanup behavior: child termination/reaping, ownership cleanup, FAILED truth, bounded diagnostic, control-plane marking/materialization.
6. A claim must never be interpreted as an OS process.

Add deterministic injection for failure immediately after claim acquisition and before STARTING persistence, including database-open/pre-transaction failure. Prove:

- claim released;
- no process ownership inserted;
- no child spawned;
- durable row remains the prior truthful state;
- next valid resume attempt is not blocked by a stale claim.

---

# F-M17-V03-003 — TERMINALIZE ATTENTION STATES TRUTHFULLY WHEN THE PROVIDER PROCESS EXITS

Attention states are live semantic states while a provider process is owned. They must not remain forever "live" after the process has actually exited and ownership has been removed.

Current terminal selection can preserve WAITING_PERMISSION / WAITING_USER / AUTH_REQUIRED / USAGE_LIMITED / NETWORK_ERROR after child exit, set `ended_at`, remove ownership, and leave the frontend offering Stop against a non-owned session while Resume remains unavailable.

Define an explicit post-process-exit policy.

Requirements:

1. Separate "current attention while process is alive" from "process ended with attention/error as its last authoritative cause."
2. After child exit and finalization, the durable state/action model must not claim an owned live process when none exists.
3. Preserve the last attention/error diagnostic as cause/provenance.
4. Choose one clear bounded model, for example:
   - terminal `FAILED`/accepted degraded terminal state with preserved cause and exact-resume eligibility when provenance is valid; or
   - another explicit terminal state already compatible with the application contract.
5. Do not misuse `ORPHANED` for a normally observed process exit; ORPHANED remains restart/lost-ownership recovery truth.
6. `ended_at`, ownership map, current state, `canStop`, and `canResume`/`supportsResume` must agree.
7. The frontend must never show Stop solely because the stale semantic label is in a live-state list after ownership is gone.
8. A later exact resume must use only the canonical provider session ID and existing project/task/cwd provenance.

Add deterministic monitor/finalization tests for each of:

- WAITING_PERMISSION then process exit;
- WAITING_USER then process exit;
- AUTH_REQUIRED then process exit;
- USAGE_LIMITED then process exit;
- NETWORK_ERROR then process exit.

For each prove final durable state, preserved diagnostic, `ended_at`, ownership removal, Stop visibility, Resume eligibility truth, and no application restart requirement.

---

# F-M17-V03-004 — RESERVE BOUNDED AUTHORITATIVE CONTROL-STATE EVIDENCE

The generic output-event budget must not starve critical lifecycle/control diagnostics.

Current live-state events share `persisted_event_count` with ordinary structured/raw output. If `MAX_OUTPUT_EVENTS` is already exhausted, a later real attention transition updates the session row but can silently lose its `ATTENTION_STATE` event and current diagnostic.

Required design:

1. Keep raw/ordinary output bounded exactly as required.
2. Keep lifecycle/control evidence independently bounded and very small.
3. A real current attention transition must retain a durable bounded diagnostic even when ordinary output events already reached their cap.
4. Repeated identical attention records must remain coalesced.
5. Repeated alternating transitions must remain bounded by an explicit lifecycle/control transition cap.
6. WAITING_PERMISSION -> RUNNING and similar real recovery must update current diagnostic truth correctly.
7. Hitting either budget must never stop stdout/stderr draining, child monitoring, or terminal finalization.
8. Do not allow unbounded control-plane materialization churn.

Acceptable approaches include a separate small lifecycle-event budget, a bounded replaceable latest-state record, or dedicated current diagnostic columns if done with a narrow migration and backward-compatible loading. Do not create unbounded logs.

Required deterministic tests:

- first exhaust the generic `MAX_OUTPUT_EVENTS` budget;
- then transition RUNNING -> WAITING_PERMISSION and prove state + current diagnostic remain durable;
- transition WAITING_PERMISSION -> RUNNING and prove diagnostic clears/updates truthfully;
- repeat attention/no-op records and prove bounded control-event count;
- alternate states beyond the control cap and prove bounded persistence;
- prove stream drain and finalization still complete after both budgets are exhausted.

---

# F-M17-V03-005 — COMPLETE THE PRODUCTION-BOUNDARY ADVERSARIAL MATRIX

V04 must close the remaining evidence gap by exercising the real lifecycle state machines, not only cleanup/persistence helpers.

Introduce narrow test-only injection seams or small production primitives where necessary. Do not consume live Claude quota for these tests.

At minimum prove through production-equivalent boundaries:

## Stop / monitor

- production Stop against an owned RUNNING fixture -> process exits -> monitor/finalizer persists STOPPED exactly once;
- initial termination requiring injected escalation success -> `STOP_ESCALATED` + monitor/finalizer STOPPED;
- injected escalation cannot confirm exit -> no false STOPPED, no permanent STOPPING, ownership preserved, explicit `CLAUDE_STOP_FAILED_PROCESS_STILL_OWNED`, later Stop remains retryable;
- Stop from every live attention state;
- unrelated process remains alive;
- no double-finalization.

## Resume setup

Inject failures through the actual resume setup pipeline at:

- SPAWN;
- STDIN acquisition;
- PROMPT_WRITE;
- STDOUT acquisition;
- STDERR acquisition;
- OWNERSHIP registration;
- RUNNING/control-plane persistence;
- post-claim/pre-STARTING database-open or equivalent pre-transition boundary.

For every applicable post-STARTING failure prove:

- spawned child terminated/reaped;
- ownership entry removed;
- resume claim released;
- no false STARTING/RUNNING row;
- explicit bounded stage diagnostic;
- canonical provider session ID unchanged.

For pre-STARTING claim failure prove the prior durable state remains unchanged and claim is released.

## Attention terminalization / budgets

- each attention state followed by actual fixture process exit reaches the V04 terminal/action contract;
- generic event budget exhaustion cannot suppress authoritative current attention diagnostics;
- event caps do not block drain/finalization.

## Existing invariants

Keep deterministic coverage green for:

- wrong-project/task/cwd resume rejection;
- missing provider session ID;
- provider-session ID mismatch immutability;
- auth/quota/network fail-closed classifiers;
- malformed provider records cannot become success;
- redaction-before-persistence;
- Codex adapter/start/stop/list/final-response regressions;
- M16 audit-provider readiness/freeform structured-output regressions;
- exact-eight-project portfolio;
- `Sekiph82/FormuLab@main` tracking.

---

# STATE / ACTION CONSISTENCY GATE

Before V04 is considered complete, audit every production path that derives or consumes Claude action truth.

For any returned `AgentSession`, these concepts must agree:

- durable state;
- `ended_at`;
- current H!veAI process ownership;
- active resume claim;
- `supportsResume` / current resume eligibility;
- Stop visibility/eligibility;
- current diagnostic;
- provider session identity/provenance;
- canonical cwd/task/project validation.

A session must never simultaneously appear terminal and live, appear live without an owned process unless explicitly represented as recovery truth, or advertise Resume when the backend will deterministically reject it for known current provenance.

Do not add broad new state names unless necessary. Prefer derived current action eligibility.

---

# REQUIRED REGRESSION / SECURITY GATES

Run and record at minimum:

1. focused session-level resume eligibility tests;
2. resume-claim RAII/failure-injection tests;
3. attention-process-exit terminalization tests;
4. generic-output-budget then attention-control-event tests;
5. production stop/monitor terminal-state tests;
6. production resume setup failure-injection suite;
7. all prior M17 V01-V03 focused Rust tests;
8. focused frontend Agents/session tests;
9. full serialized Rust library regression under repository policy;
10. full frontend Vitest regression;
11. `npm run typecheck`;
12. `cargo check --manifest-path src-tauri/Cargo.toml`;
13. `npm run build`;
14. `git diff --check`;
15. active-source scan proving no `ANTHROPIC_API_KEY`, direct Anthropic HTTP/API transport, Claude auth-file inspection, GUI/browser automation, or blanket permission bypass;
16. M16 OpenAI API-key/direct-HTTP guardrail scan;
17. Codex lifecycle regressions;
18. M16 audit-provider readiness/freeform structured-output regressions;
19. exact-eight-project and FormuLab@main regressions;
20. governed native QA publication including stable shortcut and no-visible-console checks.

Builder execution output remains claim evidence until independent audit.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M17_CLAUDE_CODE_ADAPTER_V04_LOG.md`

Include:

- synchronized starting SHA;
- V04 tracker-transition SHA;
- implementation SHA(s);
- exact remediation mapping F-M17-V03-001 through F-M17-V03-005;
- final session-level resume-eligibility design;
- resume-claim ownership/RAII design;
- attention-after-process-exit terminalization design;
- separate bounded control-state evidence design;
- production stop/monitor injection design;
- production resume setup injection design;
- focused test names/results for every V03 finding;
- full Rust/frontend/typecheck/build/cargo-check results;
- M16/Codex/portfolio/FormuLab regression results;
- forbidden provider/auth/GUI/bypass scan results;
- governed native publication executable SHA-256 and shortcut evidence;
- final tracker state;
- final local HEAD / `origin/main` / live-main equality.

Commit the immutable log separately after implementation/publication evidence is complete. Do not require the log to contain the SHA of the commit that first creates itself.

---

# INDEPENDENT / OWNER CLOSURE GATE

After V04 builder completion:

1. Codex must stop. Do not self-audit or mark M17 closed.
2. ChatGPT performs a new independent strict source re-audit.
3. Only if that audit is PASS may the owner be asked to perform native Claude readiness/start/stop/exact-resume acceptance.
4. Only after both strict source PASS and owner-native acceptance may M17 become `[x]` PASS/CLOSED and M18 be considered.

Do not consume owner time on native Claude acceptance while any V04 source finding remains open.
