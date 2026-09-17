# M19 Next Best Task AI + Engineering Brief V07 — Final Residual Source Remediation Prompt

## 0. Authority and execution boundary

You are performing a **strictly narrow residual remediation** of M19 after the independent V06 source re-audit returned `CHANGES_REQUIRED`.

Authoritative audit:

`docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V06_STRICT_REAUDIT.md`

Before editing source:

1. safely synchronize standalone H!veAI with current GitHub `main` using fetch + fast-forward only when safe;
2. do not reset, rebase, force-push, auto-stash, clean, discard, or overwrite unrelated owner work;
3. read in full:
   - `AGENTS.md`
   - `CONSTITUTION.md`
   - `ARCHITECTURE.md`
   - root `TASKS.md`
   - `CODEX_ROADMAP.md`
   - `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V06_REMEDIATION_PROMPT.md`
   - `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V06_LOG.md`
   - `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V06_STRICT_REAUDIT.md`
4. record exact synchronized `origin/main` SHA in the V07 log;
5. keep `TASKS.md` and `CODEX_ROADMAP.md` read-only. ChatGPT owns tracker transitions;
6. do not mark M19 PASS/CLOSED and do not start M20.

Close every finding `F-M19-V06-STRICT-001` through `F-M19-V06-STRICT-007` with direct production-path evidence.

V07 is not permission to redesign completed M19 work. Preserve source-positive V06 behavior.

---

# 1. Preserve accepted V06 work

Do not regress:

- unique canonical remote row identity and duplicate explicit-ID fail-closed behavior;
- exact-root TASKS authority;
- structured dependency/blocker/actor/failure evidence;
- same-HEAD `validated_at` update semantics;
- pre-refresh M19 fingerprint capture and native `refresh_and_compare` ordering;
- actor fail-closed behavior and global candidate scoring;
- detailed GitHub `GitHubAcquisitionContext` intent model;
- idle cache-only detailed GitHub behavior;
- process-wide detailed GitHub governor and in-flight coalescing;
- primary and optional 403/429 process-wide circuit behavior;
- Registry ninth-project recovery/identity/disposition behavior;
- visible `Local workspace` copy and source-positive project-card containment CSS;
- actual React `/projects` Chromium harness architecture;
- one integrated Engineering Brief surface;
- M18 failure containment;
- tracker ownership and M20 stop boundary.

---

# 2. F-M19-V06-STRICT-001 — reconcile M19 portfolio freshness with quota-safe remote validation cadence

## Problem

Background GitHub-tracked projects are validated once/hour while M19 currently rejects remote evidence older than five minutes. That makes healthy non-selected projects disappear from the portfolio recommendation set for most of every hour.

## Required contract

Define one explicit remote-decision freshness model with distinct concepts where needed:

- content materialization time;
- last successful identity/branch/HEAD validation time;
- selected-project validation target;
- background portfolio validation target;
- hard maximum age after which M19 must fail closed;
- failure/degraded state independent of age.

The hard M19 acceptance age must be mathematically compatible with the scheduler that can actually service the portfolio under its quota. Do not restore high-frequency polling just to satisfy the old five-minute constant.

A valid model may use different desired cadence and hard-stale horizon, provided the semantics are explicit and tested. A successfully validated unchanged repository must remain truthfully usable until the documented hard validation horizon. Failed validation must not advance freshness.

### Mandatory tests

Use an injectable/fake clock and at least 8, 10, and 20 projects:

- background project remains eligible beyond five minutes while still inside the documented hard validation horizon;
- selected project uses its selected cadence/horizon correctly;
- successful same-HEAD validation advances validation freshness only;
- failed validation does not advance validation freshness;
- after hard maximum age, task input becomes unavailable/fail-closed;
- no policy requires network demand above the published governor math.

Do not hide this by simply increasing a constant without testing scheduler compatibility.

---

# 3. F-M19-V06-STRICT-002 — replace inverted five-minute failure retry with monotonic quota-safe backoff

## Problem

Current degraded/error scheduling starts from the one-hour background interval and applies `min(..., 300s)`, collapsing every failure retry to five minutes.

## Required policy

Implement explicit retry constants/policy, for example a bounded exponential schedule with:

- documented base retry;
- monotonic increase on repeated failures;
- documented maximum;
- success reset;
- selected-owner intent and background intent handled deliberately;
- portfolio fairness under repeated failures;
- governor budget wait treated as budget state, not a reason for aggressive retry loops.

Do not derive retry by accidentally clamping a one-hour interval downward.

### Mandatory tests

With fake time/state:

- retry delays monotonically increase through repeated failures;
- success resets failure backoff;
- 20 failing projects cannot monopolize all subsequent healthy-project service;
- budget-wait does not create a five-minute thundering herd;
- selected manual refresh remains explicit and bounded.

---

# 4. F-M19-V06-STRICT-003 — bind refresh project scope to refresh generation

## Problem

A single `RefreshScopeGate<Option<String>>` can be overwritten while multiple `RefreshCompletion` generations exist. A later request can replace the earlier project's scope and then complete both generations.

## Required lifecycle

Represent each requested refresh generation with its own immutable declared scope, or an equivalent queue/state machine.

Required semantics:

- generation N knows its exact project id;
- generation N can complete only after that project's observation for N settles;
- generation N+1 cannot overwrite N's project scope;
- same-project requests may be deliberately coalesced only if every waiter receives a truthful completion for that same scope;
- different-project generations may not be falsely coalesced;
- failure/timeout belongs to the correct generation;
- queue is bounded and cannot grow without limit.

### Mandatory concurrency tests

- two rapid requests, project A then project B: A waits for A, B waits for B;
- B completion cannot release A;
- A completion cannot release B;
- two same-project requests follow the documented coalescing/ordering policy;
- three mixed scopes preserve deterministic order or documented safe parallelism;
- `refresh_and_compare` never records a previous fingerprint for a generation whose declared scope did not complete.

Exercise manager/scheduler scope logic, not `RefreshCompletion` in isolation.

---

# 5. F-M19-V06-STRICT-004 — make refresh timeout compatible with maximum declared tracking work

## Problem

The manual generation wait is 30 seconds while a changed-HEAD observation can execute three sequential bounded HTTP stages, each with a 20-second transport timeout plus bounded process overhead.

## Required contract

Choose one consistent deadline architecture:

- either a generation deadline derived from the maximum number of scoped stages;
- or one shared end-to-end deadline propagated through all stages;
- or another bounded cancellation model that guarantees command timeout and worker completion semantics agree.

The UI/native caller must never be told "timed out" while the same declared refresh is still legitimately running inside a larger documented per-stage timeout budget unless cancellation is actually performed and observed.

### Mandatory tests

Use deliberately slow transport/observation seams:

- same-HEAD slow validation inside bound succeeds;
- changed-HEAD three-stage work inside bound succeeds;
- genuinely over-deadline work fails once and is cancelled/settled according to contract;
- no late generation completion can mutate history after caller was told the lifecycle failed.

---

# 6. F-M19-V06-STRICT-005 — persist/advance navigation rotation across acquisition windows

## Problem

The public portfolio navigation path currently starts every invocation with `rotation_cursor = 0`. Per-project rotation differs inside one call but repeated calls service the same primary resource subset for the same project.

## Required architecture

Maintain a process-safe acquisition-window rotation cursor or equivalent fair scheduler state that advances across Navigation windows.

Requirements:

- one invocation cannot permanently define the same resource subset forever;
- every project receives first-round service before unfair repeats within a window;
- repeated constrained windows eventually service all eight primary resource kinds for every active project;
- selected owner intent can receive bounded priority without resetting/poisoning background rotation;
- cache hits do not consume network admission;
- budget reset does not silently reset fairness unless explicitly documented and tested;
- product call path that claims Navigation semantics must actually use this advancing state.

### Mandatory tests

- 8-project repeated windows;
- 20-project repeated windows;
- prove eventual coverage of all eight primary resource kinds per project;
- prove no first-project or first-resource monopoly;
- prove selected/manual operations do not permanently starve navigation rotation.

Do not satisfy this only with a detached helper schedule.

---

# 7. F-M19-V06-STRICT-006 — regenerate complete production-governed acquisition evidence

Regenerate:

`docs/H!veAI/evidence/M19_V07_GITHUB_ACQUISITION_MATRIX.md`

from the final production-governed seam.

For each measured stage include, when applicable:

- project id;
- repository;
- branch;
- acquisition intent;
- acquisition/window rotation cursor or position;
- resource class;
- exact resource kind;
- cache before;
- admission result;
- owner/coalesced state;
- explicit network yes/no;
- HTTP status/class when network occurred;
- primary budget before/after;
- optional budget before/after;
- tracking budget before/after for tracking stages;
- failure class;
- fetched/content time;
- validated time for tracking evidence;
- last-known-good age;
- final presentation state;
- bounded sanitized diagnostic;
- rate-limit limit/remaining/reset or explicit unavailable-from-transport statement.

Publish separate exact tables for:

1. detailed GitHub integration API demand, Idle/Selected/Navigation/Manual;
2. root-TASKS remote tracking demand, background and one-selected-project scenarios;
3. 8, 9, 10, and 20 projects;
4. unchanged HEAD and changed HEAD worst-case;
5. repeated failure/backoff scenario.

Do not report only `min(demand, cap)`. Show scheduled demand and admitted demand separately.

Minimum repository identities remain H-veAI, Bulk-Edit, ScrubBots-Level-Factory, and one control project.

---

# 8. F-M19-V06-STRICT-007 — complete real browser geometry evidence at required widths

Keep the real Vite + production React `/projects` + Chromium architecture.

Run representative viewport widths approximately:

- 1536 px;
- 900 px;
- 640 px.

Do not substitute 1280 for the required middle-width case.

For each width verify actual computed DOM geometry:

- card footer is inside card;
- archive/remove buttons are inside and non-overlapping;
- Open cockpit and Local workspace have non-zero clickable boxes;
- no document horizontal overflow;
- every visible workspace action says `Local workspace` and not `Change local workspace`;
- keyboard focus is on the intended control;
- focus visual is not clipped by any ancestor overflow boundary. Walk relevant ancestors or use another real-browser clipping proof rather than checking only that an outline property exists.

Record measured viewport inner widths in the V07 log.

---

# 9. Regression and stop gates

Run and record at minimum:

1. `npm run typecheck`;
2. `npm run build`;
3. full frontend tests single-worker and normal invocation;
4. actual production `/projects` Chromium geometry at ~1536/~900/~640;
5. `cargo check --manifest-path src-tauri\Cargo.toml`;
6. focused github_tracking tests for freshness/backoff/generation scope/deadline;
7. focused github_integration repeated-window fairness tests;
8. focused next_best_task remote freshness and refresh/history tests;
9. two/three concurrent refresh-generation tests on different project scopes;
10. slow changed-HEAD deadline test;
11. 8/9/10/20 scheduler and quota tests with fake time;
12. optional and primary 403/429 regression tests;
13. coalescing, duplicate-ID, dependency, Registry recovery regressions;
14. exact M16 observational-read regression;
15. full `cargo test --manifest-path src-tauri\Cargo.toml --lib --no-fail-fast` with sufficient timeout;
16. changed Rust rustfmt check;
17. full cargo fmt check with unrelated baseline drift distinguished;
18. `git diff --check`;
19. security/redaction tests;
20. V07 production-governed evidence generation/validation;
21. accepted publication helper;
22. stable EXE SHA-256.

Create the immutable builder log only after implementation and publication:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_LOG.md`

The log must include explicit disposition of all seven V06 strict findings, exact test results, changed files, implementation SHA, V07 evidence artifact link, publication SHA-256, unchanged tracker statement, M20-not-started statement, and owner-native acceptance pending statement.

After implementation:

- commit/push non-force;
- verify clean worktree;
- verify local HEAD == origin/main == live GitHub main;
- stop for independent strict re-audit.

Do not edit `TASKS.md` or `CODEX_ROADMAP.md`.
Do not start M20.
Do not self-certify owner-native acceptance.
