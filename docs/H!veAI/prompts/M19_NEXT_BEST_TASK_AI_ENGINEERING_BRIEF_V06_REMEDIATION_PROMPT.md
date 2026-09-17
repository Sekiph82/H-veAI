# M19 Next Best Task AI + Engineering Brief V06 — Authoritative Residual Remediation Prompt

## 0. Authority and execution boundary

You are performing a **narrow residual remediation** of M19 after the independent V05 strict re-audit returned `CHANGES_REQUIRED`.

Authoritative audit:

`docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V05_STRICT_REAUDIT.md`

Before editing source:

1. safely synchronize the standalone H!veAI workspace with GitHub `main` using fetch + fast-forward only when safe;
2. do not reset, rebase, force-push, auto-stash, clean, discard, or overwrite unrelated owner work;
3. read in full:
   - `AGENTS.md`
   - `CONSTITUTION.md`
   - `ARCHITECTURE.md`
   - root `TASKS.md`
   - `CODEX_ROADMAP.md`
   - `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V05_REMEDIATION_PROMPT.md`
   - `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V05_LOG.md`
   - `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V05_STRICT_REAUDIT.md`
4. record exact synchronized `origin/main` SHA in the V06 log;
5. keep `TASKS.md` and `CODEX_ROADMAP.md` read-only. ChatGPT owns tracker transitions;
6. do not mark M19 PASS/CLOSED and do not start M20.

Close every finding `F-M19-V05-STRICT-001` through `F-M19-V05-STRICT-007` with direct production-path evidence.

---

# 1. Preserve V05 source-positive work

Do not regress:

- separate canonical row identity vs explicit task ID;
- duplicate explicit task-ID ambiguity fail-closed behavior;
- same-HEAD `validated_at` freshness semantics;
- exact-root M19 authority;
- structured failure evidence;
- global scoring before candidate truncation;
- actor fail-closed behavior;
- primary cached 403/429 stop-after-first behavior;
- real in-flight result sharing;
- dependency-reference deduplication;
- Registry same-path recovery, repository identity validation, and registration disposition;
- visible `Local workspace` copy;
- one integrated Engineering Brief surface;
- grouped warning presentation;
- secret redaction and mutation-denied boundary;
- M18 failure containment;
- M20 stop boundary.

V06 is not a rewrite.

---

# 2. F-M19-V05-STRICT-001 — wire fair/tiered acquisition into the actual production path

The current helper-only `AcquisitionIntent`/`portfolio_acquisition_plan`/`fair_primary_schedule` arithmetic is insufficient.

## Required production architecture

The real detailed GitHub integration path must be intent-aware.

A valid design may introduce an explicit acquisition request/context carrying:

- project identity;
- acquisition intent: idle / selected / navigation / manual;
- portfolio size;
- selected project identity where relevant;
- fair-rotation cursor/window state;
- primary/optional admission policy.

The actual function that issues resource requests must consume that policy. Do not leave `fetch_resources_with_transport()` as an unconditional eight-primary loop while only tests exercise a separate fair schedule.

### Product rules

- **Idle portfolio:** no detailed eight-resource fan-out per project.
- **Selected/open project:** bounded primary service for the viewed project.
- **Navigation:** actual per-project/resource admissions follow a deterministic fair rotation, not first-come exhaustion of a global counter.
- **Optional enrichment:** separate small allowance; never consumes reserved primary capacity.
- **Budget exhausted:** truthful STALE/BUDGET_WAIT/UNAVAILABLE semantics, never CURRENT.
- **Manual refresh:** cannot bypass governor.

### Mandatory real orchestration tests

Use multiple actual project records and the real acquisition function:

- constrained primary budget with 8 projects -> every project receives first-round service before any project repeats;
- 20 projects -> no first-five-project monopoly;
- repeated navigation rotates service across windows;
- selected project gets bounded priority without permanent starvation;
- optional work is shed before primary capacity;
- cache hits consume zero network admission;
- budget reset/rotation cursor advance with injectable clock/state.

A test that only calls `portfolio_acquisition_plan()` is not acceptance evidence.

---

# 3. F-M19-V05-STRICT-002 — fix remote-tracking cadence, scope, and request math

The 10-second selected / 30-second portfolio remote-tracking loop is too aggressive and contradicts the V05 published hourly math.

## Required contract

Design one explicit bounded policy for root-TASKS remote tracking.

Requirements:

- no 30-second polling of every registered repository indefinitely;
- exact request/hour math must be derived from the **real scheduler intervals and actual HTTP stages**;
- same-HEAD validation cadence must remain compatible with the M19 freshness horizon;
- if the M19 freshness horizon changes, document/test the new bound and keep stale inputs fail-closed;
- selected-owner intent may refresh more often than background projects, but with a documented upper bound;
- remote tracking must have a shared/network admission policy or another equally explicit bounded coordinator;
- failures use bounded backoff;
- no hidden multiplicative second Atom/raw request without it appearing in request math.

## Manual refresh scope

The Command Center Refresh button must not silently trigger a full-portfolio remote refresh if the product contract/log claims one selected tracking refresh.

Prefer a selected-project generation contract:

`refresh_selected(project_id)`

or an equivalent explicit scope.

If a separate portfolio refresh exists, it must be an explicit distinct operation.

The refresh generation must wait only for the work included in its declared scope, and its timeout must be compatible with that maximum work.

### Required tests

- 8/9/10/20-project scheduler math from actual intervals;
- one hour simulated with fake clock -> exact bounded network call count;
- selected project cadence vs background cadence;
- manual selected refresh touches only selected project tracking;
- explicit portfolio refresh, if retained, is separately named and bounded;
- timeout/backoff behavior under slow transport;
- unchanged HEAD validation remains within M19 freshness contract without 10/30-second storms.

---

# 4. F-M19-V05-STRICT-003 — make refresh history truly previous-vs-current

Do not keep the current frontend sequence:

`await refresh -> recompute current -> persist current as history -> read current again`

That records the post-refresh state as `previous`.

## Required lifecycle

Implement a factual lifecycle, preferably in one native command/service boundary:

1. compute/capture the pre-refresh M19 fingerprint;
2. request the intended scoped GitHub tracking generation;
3. await bounded generation completion;
4. if the defined lifecycle condition is satisfied, persist the captured **pre-refresh** fingerprint exactly once;
5. compute the post-refresh M19 snapshot;
6. return comparison against the persisted pre-refresh fingerprint.

Pure snapshot reads remain write-free.

Failure/timeout must not persist a misleading successful baseline.

Two rapid refresh actions must not reorder history generations.

### Direct tests

- T0 recommendation A; mutate remote evidence during refresh; post-refresh recommendation B -> `comparison.state == CHANGED` and previous identity is A;
- no evidence mutation -> `NO_COMPARABLE_CHANGE`;
- first-ever refresh -> truthful first-snapshot semantics;
- refresh failure -> no false successful baseline;
- timeout -> bounded error/no misleading history;
- two refresh generations -> deterministic ordering.

The frontend should consume this factual lifecycle rather than separately invoking a post-refresh history command.

---

# 5. F-M19-V05-STRICT-004 — any optional 403/429 must open the same process-wide circuit

`acquire_enrichment()` and `acquire_enrichment_text()` must not only stop the local optional budget.

On any structured `RATE_LIMITED` result from primary **or optional** acquisition:

- open the same process-wide rate-limit circuit immediately;
- stop remaining optional calls in the current snapshot;
- prevent subsequent snapshots from starting new primary/optional requests during backoff;
- retain stale last-good where available;
- present one grouped warning;
- successful post-backoff recovery clears obsolete warning/circuit state.

### Required tests

- all primary resources CURRENT, first PR enrichment returns 429 -> remaining enrichment zero, global circuit open;
- same for 403 rate limit;
- immediate second project/snapshot during backoff -> zero transport calls;
- stale cache remains STALE, never CURRENT;
- fake-clock advance beyond backoff + success -> CURRENT and warning clears.

---

# 6. F-M19-V05-STRICT-005 — emit a real production-governed acquisition evidence artifact

Create an immutable redacted artifact, for example:

`docs/H!veAI/evidence/M19_V06_GITHUB_ACQUISITION_MATRIX.md`

or CSV/JSON plus a short Markdown index.

It must be generated from the production-governed test seam, not manually summarized prose.

For each repository/resource/fetch stage include when available:

- repository;
- branch;
- resource class;
- resource kind;
- acquisition intent;
- cache state before;
- admission decision;
- governor primary/optional budget before and after;
- fair-rotation position/cursor when relevant;
- in-flight owner/coalesced state;
- network yes/no;
- HTTP code/status class;
- failure cause;
- content fetched-at;
- validated-at where relevant;
- last-good age;
- final presentation state;
- sanitized diagnostic;
- rate-limit metadata or explicit `UNAVAILABLE FROM CURRENT TRANSPORT`.

Minimum repositories:

- `Sekiph82/H-veAI@main`
- `Sekiph82/Bulk-Edit@main`
- `Sekiph82/ScrubBots-Level-Factory@main`
- one additional control.

Also publish exact real scheduler/request calculations for 8/9/10/20 projects.

---

# 7. F-M19-V05-STRICT-006 — geometry proof must render the actual production Projects surface

The Chromium harness must stop hand-building a clone of the card DOM.

Use one of:

- Playwright/Chromium against the actual built React app route;
- an existing browser harness that mounts the real Projects page/component with production providers/fixtures;
- another deterministic browser route that executes the same production React tree.

At approximately 1536, 900, and 640 px prove actual computed geometry:

- footer/actions inside each real card;
- archive/remove fully inside and non-overlapping;
- `Open cockpit` and `Local workspace` non-zero clickable boxes;
- no horizontal document overflow;
- focused action outline is actually visible and not clipped by any ancestor;
- visible copy contains `Local workspace` and not `Change local workspace`.

Do not use a copied HTML string as the acceptance geometry surface.

---

# 8. F-M19-V05-STRICT-007 — publication boundary and owner-native handoff

Codex must not fabricate interactive owner-window evidence if its environment lacks a real UI-control channel.

For V06, builder acceptance requires:

- accepted publication pipeline PASS;
- stable published EXE SHA-256;
- process/startup/ready-marker smoke;
- actual production Projects-route browser geometry;
- production source/regression/evidence gates below.

Then stop for independent source audit.

If independent V06 source audit returns PASS, the **owner** will perform the final published-app interactive acceptance for:

1. archived ninth project explicit Add -> same ID ACTIVE and visible;
2. restart -> ninth remains;
3. tenth unrelated project -> ninth+tenth visible;
4. real Projects delete/archive containment;
5. all workspace buttons show `Local workspace`;
6. Bulk-Edit GitHub page no warning wall;
7. ScrubBots-Level-Factory GitHub page no warning wall;
8. real stale/rate-limit presentation and later recovery as observable;
9. H-veAI/control behavior;
10. no M18 black surface;
11. M19 Engineering Brief previous-vs-current behavior after a real refresh.

Builder log must explicitly label this owner acceptance as pending, not as a builder failure.

---

# 9. Mandatory regression gates

Run and record at minimum:

1. `npm run typecheck`;
2. `npm run build`;
3. focused mounted Command Center/M19 refresh-history tests;
4. actual production Projects-route browser geometry test;
5. full frontend tests single-worker;
6. normal/default frontend tests;
7. `cargo check --manifest-path src-tauri\Cargo.toml`;
8. focused Rust tests for github_tracking, github_integration, next_best_task, command_center, Registry paths touched;
9. production multi-project fair-acquisition orchestration tests;
10. fake-clock 8/9/10/20 scheduler/request-math tests;
11. selected/manual refresh-scope tests;
12. pre-refresh fingerprint -> post-refresh comparison tests;
13. optional 403/429 global-circuit tests;
14. coalescing regression tests;
15. same-HEAD freshness regression tests;
16. duplicate-ID/dependency regression tests;
17. exact M16 observational-read test;
18. full `cargo test --manifest-path src-tauri\Cargo.toml --lib --no-fail-fast` with sufficient timeout;
19. changed Rust `rustfmt --check`;
20. full cargo fmt check with unrelated baseline drift distinguished;
21. `git diff --check`;
22. security/redaction tests;
23. accepted publication helper;
24. stable EXE SHA-256;
25. V06 production-governed acquisition evidence artifact validation.

Do not claim PASS from detached planning helpers when the real product orchestration is untested.

---

# 10. Required immutable V06 builder log

Create only after implementation/tests/publication are complete:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V06_LOG.md`

It must include:

- synchronized start SHA;
- implementation SHA;
- changed files;
- explicit disposition of `F-M19-V05-STRICT-001` through `007`;
- real multi-project production acquisition proof;
- exact remote-tracking scheduler math and actual intervals;
- manual refresh scope proof;
- previous-vs-current refresh/history proof with an intentional evidence change;
- optional 403/429 process-wide circuit proof;
- link to immutable V06 acquisition evidence artifact;
- actual production Projects-route geometry results;
- full frontend/Rust results;
- M16 exact result;
- publication result and stable EXE SHA-256;
- explicit statement `TASKS.md` and `CODEX_ROADMAP.md` unchanged;
- explicit M20 not started statement;
- explicit statement that final owner-native interactive acceptance remains pending and was not self-claimed.

The log remains a claim until independent source re-audit.

---

# 11. Stop gate

After all work:

1. verify trackers unchanged;
2. verify M20 not started;
3. commit implementation;
4. create/commit immutable V06 evidence artifact(s) and log after implementation SHA is final;
5. push non-force to `origin/main`;
6. verify clean worktree;
7. verify local `HEAD == origin/main == live GitHub main`;
8. verify V06 log and evidence artifact reachable on GitHub;
9. stop for independent strict re-audit.

Do not ask Codex to perform or self-certify the owner's final interactive acceptance.