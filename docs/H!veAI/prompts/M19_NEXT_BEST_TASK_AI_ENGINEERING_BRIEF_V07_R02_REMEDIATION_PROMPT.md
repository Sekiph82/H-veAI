# M19 Next Best Task AI + Engineering Brief V07 R02 — Narrow Lifecycle Remediation Prompt

## 0. Authority and scope

You are performing a **narrow V07 R02 lifecycle remediation** after the independent V07 strict re-audit returned `CHANGES_REQUIRED`.

Authoritative audit:

`docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_STRICT_REAUDIT.md`

Before source changes:

1. safely synchronize standalone H!veAI with GitHub `main` using fetch + fast-forward only when safe;
2. do not reset, rebase, force-push, clean, auto-stash, discard, or overwrite owner work;
3. read in full:
   - `AGENTS.md`
   - `CONSTITUTION.md`
   - `ARCHITECTURE.md`
   - root `TASKS.md`
   - `CODEX_ROADMAP.md`
   - `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_REMEDIATION_PROMPT.md`
   - `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_LOG.md`
   - `docs/H!veAI/evidence/M19_V07_GITHUB_ACQUISITION_MATRIX.md`
   - `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_STRICT_REAUDIT.md`
4. record exact synchronized `origin/main` SHA;
5. keep `TASKS.md` and `CODEX_ROADMAP.md` read-only;
6. do not mark M19 PASS/CLOSED;
7. do not start M20.

Close only `F-M19-V07-STRICT-001` through `F-M19-V07-STRICT-005`. This is not permission for another broad M19 rewrite.

---

## 1. Preserve accepted V07 behavior

Do not regress:

- exact-root TASKS authority;
- canonical remote row identity and duplicate explicit-ID fail-closed behavior;
- dependency/blocker/actor/failure evidence semantics;
- pre-refresh fingerprint capture and history ordering;
- monotonic failure backoff;
- process-wide tracking governor;
- process-wide detailed GitHub governor;
- primary/optional 403/429 circuit behavior;
- Navigation public-path advancing rotation cursor;
- idle/selected/navigation/manual detailed GitHub intent behavior;
- V07 acquisition evidence schema quality;
- Registry ninth-project recovery and repository-identity behavior;
- `Local workspace` copy and card containment fixes;
- real React `/projects` Chromium geometry architecture;
- one integrated Engineering Brief;
- M18 failure containment;
- tracker ownership and M20 stop boundary.

---

# 2. F-M19-V07-STRICT-001 — use one coherent validation timestamp / scheduler horizon boundary

## Current defect

A changed-HEAD observation captures `fetched_at` before network work starts and later uses that start timestamp as `validated_at`. The scheduler schedules the next observation from result/completion time. The published background hard horizon is only:

`3600s cadence + 80s completion budget = 3680s`.

That is not sufficient when the previous successful changed-HEAD observation itself consumed part of the 80-second budget before next-due was calculated. At the next due boundary, the old validation can already be at/over the hard horizon while the next legitimate validation is still executing.

## Required semantics

Use the same factual lifecycle boundary for freshness and scheduling.

Preferred repair:

- `content_fetched_at` may continue to represent content acquisition time;
- `validated_at` must represent the time a successful identity/branch/HEAD validation is actually complete enough to be accepted as current;
- same-HEAD and changed-HEAD success must update `validated_at` from that successful completion boundary;
- failed validation must never advance it;
- next-due and M19 hard horizon arithmetic must be provably compatible with this same boundary.

If a different design is chosen, prove equivalent semantics mathematically and with production-path tests.

### Mandatory tests

Use injectable/fake clock or deterministic timing seams:

1. slow changed-HEAD success -> `validated_at` reflects successful validation completion rather than pre-network start;
2. next background due after that success -> old evidence remains usable while the next legitimate validation runs inside the documented bound;
3. two consecutive slow changed-HEAD cycles -> no unintended M19-unavailable gap;
4. same-HEAD validation advances only validation freshness, not content materialization time;
5. failed validation leaves prior `validated_at` unchanged and health fails closed;
6. 8/10/20-project quota math still fits the selected/background freshness policy.

Do not fix this only by inflating a constant while keeping incompatible timestamp origins.

---

# 3. F-M19-V07-STRICT-002 — bind each refresh generation to a qualifying observation, not merely a project id

## Current defect

The queue now stores `(generation, project_id)`, but worker jobs/results still identify only the project. `settle_project(project_id)` completes all pending generations for that project from any result.

Therefore an observation that started **before** a manual generation was requested can release that new generation when it returns.

## Required architecture

Introduce an observation identity/epoch/admission token or equivalent lifecycle so the scheduler can prove which refresh generations a job is allowed to settle.

Required semantics:

- a manual generation is completed only by an observation that qualifies for that generation;
- an already-running pre-request observation must not satisfy a new manual generation unless the design can prove its factual observation boundary occurred after the request;
- different-project generations remain isolated;
- same-project generations may deliberately coalesce only when they are attached to the same qualifying observation;
- job/result messages must carry sufficient identity to enforce this, not just `project_id`;
- success/failure belongs only to the generation(s) attached to that observation;
- timeout/cancellation detaches the timed-out generation safely;
- late unrelated results cannot release it.

### Mandatory production-scheduler tests

Exercise the actual scheduler/worker/result state machine, not only helper structs:

1. scheduled A job already in flight -> manual A request -> old A result cannot complete manual generation;
2. after a new qualifying A observation settles, manual A completes;
3. A and B manual requests while unrelated workers run -> B result cannot release A and vice versa;
4. same-project A1/A2 requests before a qualifying job may coalesce if documented and both settle truthfully;
5. A2 arriving after A1's observation has crossed the factual observation boundary must not be falsely released by A1;
6. qualifying observation failure fails only attached generations;
7. `refresh_and_compare` does not record history until its own qualifying generation succeeds.

---

# 4. F-M19-V07-STRICT-003 — make manual refresh deadline compatible with worker admission/queue state

## Current defect

The 80-second deadline covers the declared three-stage observation work but starts before the requested project is guaranteed a worker slot. Four unrelated jobs can already be in flight and legitimately consume most of their own bounds before the manual job starts.

## Required contract

Separate **admission/queue wait** from **observation execution** or otherwise prove one bounded end-to-end deadline.

Preferred options:

### Option A — explicit admission + execution deadline

- manual request enters a bounded admission queue;
- admission itself has a bounded deadline;
- once a qualifying job gets a worker slot, the 80-second observation deadline begins;
- if admission cannot happen in time, return a truthful `BUSY`/`BACKPRESSURE`/equivalent state without pretending the observation timed out.

### Option B — reserved manual capacity

- reserve bounded capacity for explicit manual generations so they cannot sit behind four arbitrary background jobs;
- prove this does not starve background work.

### Option C — proven bounded end-to-end deadline

- derive the deadline from maximum queue delay + maximum observation work;
- prove the queue bound makes that maximum real and acceptable.

Do not leave the current 80-second wait starting at request acceptance while worker admission remains unbounded relative to that 80 seconds.

### Mandatory tests

1. saturate four workers with slow legitimate observations, then issue manual refresh;
2. prove manual request receives either bounded truthful backpressure or its full declared execution budget after admission;
3. no false `timed out` while its own qualifying observation is still legitimately inside execution budget;
4. genuinely over-deadline execution fails exactly once;
5. timed-out/cancelled generation cannot later mutate M19 history;
6. worker/backpressure policy remains bounded under 32 queued requests.

---

# 5. F-M19-V07-STRICT-004 — replace helper-only refresh concurrency proof with production scheduler proof

The existing helper tests for `RefreshScopeGate`, `RefreshGenerationCoordinator`, and `RefreshCompletion` may remain, but they are insufficient acceptance evidence.

Add a deterministic native scheduler harness/seam that executes:

`request -> admission -> in-flight job -> observation result -> generation completion`

with controlled workers and controlled observations.

The test seam must exercise the same production state-transition functions used by the real polling manager. Avoid duplicating production logic in a test-only simulation.

At minimum cover all concurrency/deadline cases in sections 3 and 4, including a job already in flight before a generation request.

---

# 6. F-M19-V07-STRICT-005 — make the selected freshness contract truthful

Current evidence exposes:

- selected cadence = 300s;
- selected horizon = 380s;
- background horizon = 3680s.

But `next_best_task::remote_validation_is_fresh()` currently uses the background horizon unconditionally.

Choose and document exactly one model.

## Model A — one M19 hard horizon, selected cadence is only a target

If 380 seconds is not an M19 eligibility cutoff:

- rename it to avoid calling it a selected M19 hard horizon;
- document that selected=300s is a desired scheduler validation target while M19 hard acceptance remains the portfolio hard horizon;
- update V07/R02 evidence language accordingly;
- test selected scheduling separately from M19 hard acceptance;
- remove/deprecate misleading unused decision-horizon APIs.

## Model B — selected M19 horizon is real

If selected project evidence is intended to fail the age gate after 380 seconds:

- pass selected project identity/context into remote M19 collection;
- call the selected horizon for that project and background horizon for others;
- production-path tests must prove both.

Do not publish a selected M19 horizon that the decision engine cannot use.

---

# 7. Evidence and regression requirements

Regenerate a narrow R02 evidence artifact:

`docs/H!veAI/evidence/M19_V07_R02_TRACKING_LIFECYCLE_MATRIX.md`

It must be generated from the production scheduler/observation test seam and include at minimum:

- generation id;
- project id;
- request accepted time/epoch;
- qualifying observation id/epoch;
- observation start and completion boundary;
- worker admission state/time;
- pre-existing in-flight yes/no;
- queue/admission outcome;
- result health;
- validated-at before/after;
- next due boundary;
- freshness horizon used;
- generation completion/failure/timeout result;
- whether history recording is permitted;
- sanitized diagnostic;
- rows for A-preexisting-inflight/manual-A, A+B concurrency, same-project coalescing, saturated four-worker admission, slow changed-HEAD success, over-deadline failure.

Run and record at minimum:

1. `npm run typecheck`;
2. `npm run build`;
3. full frontend tests single-worker and default invocation;
4. existing real React geometry gate, no regression;
5. `cargo check --manifest-path src-tauri/Cargo.toml`;
6. focused `github_tracking` lifecycle/freshness/backoff tests;
7. focused `next_best_task` freshness/history tests;
8. actual production scheduler generation-binding tests;
9. saturated-worker admission/deadline tests;
10. two consecutive slow changed-HEAD freshness-cycle test;
11. selected-vs-background freshness contract test under the chosen model;
12. 8/9/10/20 request-budget regression tests;
13. Navigation rotation regression tests;
14. primary/optional 403/429 circuit regression tests;
15. coalescing and same-HEAD regressions;
16. duplicate-ID/dependency and Registry recovery regressions;
17. exact M16 observational-read regression;
18. full Rust lib suite with sufficient timeout;
19. changed Rust `rustfmt --check`;
20. full cargo fmt result with unrelated baseline drift distinguished;
21. `git diff --check`;
22. accepted publication helper;
23. stable published EXE SHA-256;
24. R02 lifecycle evidence generation validation.

---

# 8. Immutable R02 log and stop gate

After implementation/tests/publication, create:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_R02_LOG.md`

The log must include:

- synchronized start SHA;
- implementation SHA;
- changed files;
- explicit disposition of `F-M19-V07-STRICT-001` through `005`;
- exact timestamp/freshness lifecycle chosen;
- qualifying-observation generation-binding design;
- admission/deadline design and saturated-worker result;
- selected freshness model chosen and why;
- production scheduler concurrency test results;
- link to `M19_V07_R02_TRACKING_LIFECYCLE_MATRIX.md`;
- full frontend/Rust results;
- M16 exact result;
- publication result and EXE SHA-256;
- explicit `TASKS.md` and `CODEX_ROADMAP.md` unchanged statement;
- explicit M20 not started statement;
- explicit owner-native acceptance pending statement.

Then:

1. verify trackers unchanged;
2. verify M20 not started;
3. commit implementation;
4. create/commit immutable R02 evidence and log;
5. push non-force to `origin/main`;
6. verify clean worktree;
7. verify local `HEAD == origin/main == live GitHub main`;
8. verify R02 log/evidence are reachable on GitHub;
9. stop for independent source review.

Do not self-certify owner-native acceptance.
