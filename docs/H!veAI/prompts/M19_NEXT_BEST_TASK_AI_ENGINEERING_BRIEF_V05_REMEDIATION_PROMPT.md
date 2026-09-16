# M19 Next Best Task AI + Engineering Brief V05 - Authoritative Residual Remediation Prompt

## 0. Authority and execution boundary

You are performing a narrow residual remediation of **M19 - Next Best Task AI + Engineering Brief** after the independent V04 strict re-audit returned **CHANGES_REQUIRED**.

Authoritative V04 audit:

`docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V04_STRICT_REAUDIT.md`

Before changing source:

1. Safely synchronize the standalone H!veAI workspace with current GitHub `main` using fetch + fast-forward only when safe.
2. Do not reset, rebase, force-push, auto-stash, clean, discard, or overwrite unrelated owner work.
3. Read in full:
   - `AGENTS.md`
   - `CONSTITUTION.md`
   - `ARCHITECTURE.md`
   - root `TASKS.md`
   - `CODEX_ROADMAP.md`
   - `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V04_REMEDIATION_PROMPT.md`
   - `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V04_LOG.md`
   - `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V04_STRICT_REAUDIT.md`
4. Record exact synchronized `origin/main` SHA in the V05 log.
5. Run focused baseline tests for every source area touched below before remediation where practical, then record before/after behavior.

`TASKS.md` and `CODEX_ROADMAP.md` are **read-only for Codex**. ChatGPT owns tracker transitions. Do not mark M19 PASS/CLOSED. Do not start M20.

Close every finding `F-M19-V04-STRICT-001` through `F-M19-V04-STRICT-011` with direct production-path evidence.

---

# 1. Preserve V04 source-positive work

Do not regress:

- pure exact-root local `TASKS.md` M19 parsing;
- M19 observational snapshot purity;
- local and remote shared structured failure-evidence lookup;
- latest-PASS-over-older-FAIL urgency semantics;
- global scoring before final candidate truncation;
- actor fail-closed semantics;
- one integrated M19 Engineering Brief surface;
- source-positive repository-identity validation for same-path recovery;
- authoritative registration disposition DTO;
- archived ninth-project same-ID restoration path;
- visible `Local workspace` project-card copy;
- source-positive responsive card CSS;
- grouped user-facing rate-limit warning;
- secret redaction and GitHub mutation-denied boundary;
- M18 panel-local failure containment;
- V04 full Rust/frontend regression coverage that remains valid;
- M20 stop boundary.

V05 must repair residual semantics, not replace correct V04 work wholesale.

---

# 2. F-M19-V04-STRICT-001 - remote duplicate explicit task IDs must remain distinct canonical rows and fail closed

## Problem

Remote root-TASKS materialization currently uses the explicit task ID as `RemoteTaskRow.id`. Two distinct rows with the same explicit ID can therefore collapse in M19 maps before ambiguity is detected.

## Required remote task contract

Preserve two identities:

1. **canonical row identity** - unique and deterministic for the specific root-TASKS row, for example repository/branch/content-hash/source-line plus normalized explicit ID, or another equally stable bounded identity;
2. **explicit task ID** - the owner-authored ID used by dependency references and UI.

Do not use an explicit ID as the only canonical row key.

M19 graph construction must index:

`normalized explicit ID -> one or more unique canonical row identities`

Then:

- one match -> resolvable;
- zero matches -> missing dependency, fail closed;
- more than one match -> ambiguous duplicate explicit ID, fail closed;
- conflicting duplicate completion states must never be collapsed by a `HashMap` overwrite.

### Required direct tests

- remote root TASKS with two distinct `TASK-A` rows and B depends on A -> B ineligible/attention;
- duplicate A rows with conflicting complete/open states -> still ambiguous, never winner-by-overwrite;
- same test with case-equivalent duplicate IDs;
- unique A -> normal resolution;
- local duplicate explicit IDs remain fail-closed as well.

---

# 3. F-M19-V04-STRICT-002 - separate remote content freshness from successful HEAD validation freshness

## Problem

A same-HEAD scheduler observation currently returns the old cached snapshot unchanged. M19 uses the old `fetched_at` as a five-minute decision freshness proof, so a quiet repository ages out even though HEAD was successfully revalidated seconds ago.

## Required contract

Introduce explicit remote validation freshness. Recommended shape:

- `content_fetched_at`: when root TASKS/content was actually fetched/materialized;
- `validated_at`: last successful remote identity/branch/HEAD validation;
- retain backward compatibility for existing persisted snapshots with bounded migration/default semantics.

On successful same-HEAD validation:

- update `validated_at`;
- persist the refreshed validation observation;
- keep content hash/task rows unchanged;
- keep `content_fetched_at` truthful;
- clear obsolete transient error state if validation succeeded.

M19 executable freshness must use successful `validated_at` plus identity/branch/HEAD/root-hash invariants. It must not require content changes every five minutes.

On timeout/offline/rate-limit/failed HEAD validation, do not advance `validated_at`.

### Required tests

Use an injectable/fake clock where necessary:

- initial CURRENT content at T0;
- same HEAD successfully validated after >5 minutes -> still CURRENT/eligible because `validated_at` is fresh;
- no validation after >5 minutes -> stale/ineligible;
- failed validation -> `validated_at` not advanced;
- changed HEAD -> content refetched and both timestamps advance truthfully;
- restart from persisted legacy snapshot -> safe/fail-closed migration behavior.

---

# 4. F-M19-V04-STRICT-003 - rate-limit cause must trip the circuit even when presentation state is STALE

## Problem

A 403/429 request with last-known-good cache is currently returned as `state=STALE`. The acquisition loop only opens its circuit when `state==RATE_LIMITED`, so cached rate-limit failures can continue fan-out.

## Required result model

Separate:

- presentation/cache state: `CURRENT`, `STALE`, `UNAVAILABLE`;
- machine-readable failure cause/class: `NONE`, `RATE_LIMITED`, `AUTH_REQUIRED`, `TIMEOUT`, `OFFLINE`, `MALFORMED`, `BUDGET_EXHAUSTED`, etc.

Do not infer circuit behavior from presentation state.

403 GitHub rate-limit and 429 must set failure cause `RATE_LIMITED` whether or not stale cache is available.

On first rate-limit cause:

- open process-wide circuit immediately;
- stop remaining primary calls;
- stop all optional enrichment calls;
- use last-known-good data as STALE where available;
- use RATE_LIMITED/UNAVAILABLE where no cache exists;
- emit one grouped warning;
- recover only according to the bounded reset/backoff policy;
- successful recovery clears obsolete warnings.

### Required tests

Test all four key combinations:

- 403 + no cache;
- 403 + last-good cache;
- 429 + no cache;
- 429 + last-good cache.

In every case prove network fan-out stops after the first rate-limit observation.

---

# 5. F-M19-V04-STRICT-004 - replace the starvation cap with a quota-safe fair acquisition architecture

A simple `min(demand, 48)` proof is not acceptable.

## Required product behavior

The product must remain useful with 8, 9, 10, and 20 registered projects under the documented unauthenticated boundary.

Because eight detailed API resources per project cannot all be cold-refreshed hourly for 20 projects under a 60-request/hour public quota, **do not design idle behavior as a full detailed eight-resource portfolio refresh**.

Use a tiered acquisition model. A valid design can include:

### Tier A - portfolio/idle

- no automatic eight-resource API fan-out for every registered project;
- consume durable cache and the existing bounded remote tracking source where possible;
- portfolio count must not linearly multiply detailed GitHub API calls while idle.

### Tier B - selected/open GitHub project

- primary resource acquisition is prioritized for the project the owner is actually viewing;
- use a realistic freshness horizon/cadence justified by quota math;
- a selected-project refresh must not trigger detailed fetches for every other project.

### Tier C - optional enrichment

- PR files/reviews/comments/checks/jobs/logs are lower priority and preferably lazy/on-demand;
- optional enrichment must have a separate small allowance or reservation policy;
- optional work must never consume capacity reserved for primary health;
- one PR-rich repository must not be able to consume the entire hourly process budget automatically.

## Fairness

If demand exceeds quota:

- primary health must be scheduled fairly;
- do not let first-alphabetical/first-opened projects permanently starve later projects;
- selected owner intent may receive bounded priority, but starvation must remain bounded and explicit;
- return truthful budget-wait/stale state rather than pretending it is CURRENT.

## Mandatory quota math

Publish exact worst-case calculations for 8, 9, 10, and 20 projects for:

- idle hour;
- one selected project held open for an hour;
- navigation across several projects;
- manual refresh;
- optional enrichment demand;
- recovery after exhausted external/shared public quota.

The proof must show actual scheduled calls per resource class, not `min(total, cap)`.

### Required tests

Use an injectable governor clock/state and prove:

- 8-project idle portfolio does not starve two projects by construction;
- 20-project idle portfolio does not execute 160 detailed primary requests;
- selected-project primary requests receive service while optional work is shed first;
- optional enrichment cannot consume reserved primary capacity;
- fairness/rotation works across multiple projects under constrained budget;
- cache reuse and freshness cadence match the published math;
- manual refresh obeys governor and cannot bypass quota;
- budget reset advances correctly with fake clock.

---

# 6. F-M19-V04-STRICT-005 - implement real in-flight result sharing and test the production governor

## Required architecture

Do not compile the real governor/coalescer out under `cfg(test)`.

Factor governor behavior behind injectable components:

- clock;
- transport;
- quota policy;
- per-key in-flight table.

For each request key, concurrent callers must share one bounded result object/future/condition variable/channel.

Required semantics:

- first caller becomes owner;
- later same-key callers wait for the same result, up to a timeout consistent with the network timeout;
- if owner succeeds, all waiters receive the same CURRENT payload/cache metadata;
- if owner fails, all waiters receive the same classified failure/stale result;
- no arbitrary 100-ms cache poll race;
- owner completion always removes/settles the in-flight entry;
- panic/error path cannot leave a permanent in-flight tombstone.

### Required concurrency tests

With a deliberately slow fake transport:

- two same-key callers -> exactly one transport request and two same CURRENT results;
- ten same-key callers -> exactly one transport request;
- owner failure -> all waiters get same failure class;
- old cache + slow owner -> waiter receives final owner result, not immediately stale old cache;
- different keys may proceed within governor limits.

These tests must execute the same governor code compiled for production.

---

# 7. F-M19-V04-STRICT-006 - make manual refresh completion and M19 history ordering factual

## Problem

`hiveai_github_tracking_refresh` currently enqueues a scheduler request and returns immediately. Frontend then records M19 history before actual remote refresh completion.

## Required lifecycle

Introduce an explicit refresh generation/completion contract.

One acceptable model:

1. capture the currently displayed/current M19 factual fingerprint as the **previous** candidate history in memory;
2. request a specific GitHub refresh generation;
3. await bounded completion for that generation, including success/degraded result;
4. only when the intended lifecycle condition is satisfied, persist the buffered previous fingerprint as M19-owned history;
5. compute/read the new Command Center snapshot;
6. comparison uses the persisted previous fingerprint against the new current fingerprint.

Other designs are acceptable if they preserve the same factual previous-vs-current semantics.

Requirements:

- "refresh requested" is never treated as "refresh completed";
- pure snapshot reads remain write-free;
- failed/timeout refresh has explicit bounded behavior and cannot silently record a misleading successful comparison baseline;
- first snapshot remains truthful;
- later successful refresh can produce NO_CHANGE or CHANGED based on actual before/after evidence.

### Required tests

Use an async/delayed fake scheduler completion:

- history write does not occur while refresh is only queued/in-flight;
- successful completion -> prior fingerprint persisted once, then new snapshot compares;
- failed refresh -> specified fail-closed history behavior;
- timeout -> bounded failure;
- two refresh clicks cannot reorder generations/history.

---

# 8. F-M19-V04-STRICT-007 - use one stable issue identity across legacy and M19 attention

The frontend must not choose one identity source for legacy items and a different one for M19 items.

Prefer a native/shared factual issue key carried on both DTO projections, built from stable evidence such as:

- project ID;
- canonical task ID;
- normalized dependency/blocker/wait identity;
- structured failure evidence ID;
- provider identity;
- source evidence identity when it is the actual issue discriminator.

If backend changes are required, add a bounded optional `issueKey`/`issueIdentity` field to both attention contracts.

### Required mounted tests

Use the real DTO shapes:

1. legacy attention with detail but no evidence + M19 attention with evidence for the same factual blocker -> rendered once;
2. same task with two genuinely distinct blockers -> both rendered;
3. same task blocker + provider unavailable -> both rendered;
4. same wording but different stable evidence IDs -> both rendered when factually distinct.

---

# 9. F-M19-V04-STRICT-008 - replace mocked geometry with real layout evidence

The current test that mocks `getBoundingClientRect` must not be used as acceptance proof.

Use an actual browser layout engine. Prefer an existing browser harness. If none exists, add a dev-only Playwright/Chromium test or another bounded real-layout harness that does not alter production runtime dependencies.

At representative widths including approximately 1536, 900, and 640 px, assert actual computed geometry for a populated Projects grid:

- card footer inside card;
- archive/remove buttons fully inside card;
- archive/remove do not overlap;
- Open cockpit and Local workspace have non-zero visible clickable boxes;
- no horizontal document overflow;
- focus outlines are not clipped by overflow containers;
- `Change local workspace` is not visible card copy;
- `Local workspace` is visible on every non-archived project card.

Do not mock `getBoundingClientRect`, `clientWidth`, `scrollWidth`, or computed layout values in this acceptance test.

Keep source-level/unit tests if useful, but they are supplementary only.

---

# 10. F-M19-V04-STRICT-009 - dedupe dependency edges before graph resolution

Normalize and deduplicate dependency references before resolving them.

For one dependent/prerequisite pair, retain exactly one canonical edge regardless of duplicate textual references or case-equivalent duplicates.

Required tests:

- `Depends on: A, A` -> one edge, one unfinished blocker, one attention fact;
- `A, a` under case-equivalent semantics -> one edge;
- unlock score counts once;
- evidence/explanation does not duplicate the same prerequisite.

---

# 11. F-M19-V04-STRICT-010 - complete the targeted production-path matrix and acquisition evidence

In addition to regression suites, add direct tests for every V05 finding.

The immutable V05 evidence must include a **production-governed** acquisition matrix separate from any shell/public probe.

For each tested repository/resource stage, record when available:

- repository;
- branch;
- resource class: primary/optional/remote-tracking;
- cache state before request;
- admission decision;
- governor window/budget before and after;
- in-flight owner/coalesced status;
- network request yes/no;
- HTTP code/status class where a network call occurred;
- failure cause;
- content fetched-at;
- validation time where applicable;
- last-known-good age;
- final presentation state;
- bounded sanitized diagnostic;
- rate-limit limit/remaining/reset, or explicit unavailability.

Keep any live shell probe clearly labeled as external corroboration only.

At minimum exercise:

- `Sekiph82/H-veAI@main`;
- `Sekiph82/Bulk-Edit@main`;
- `Sekiph82/ScrubBots-Level-Factory@main`;
- one additional control.

---

# 12. F-M19-V04-STRICT-011 - perform the required builder-native published-build smoke matrix

After source/tests pass and the accepted publication helper produces the stable executable, perform builder-native QA against that published build.

Record bounded evidence for all of these:

1. archived ninth project + explicit Add -> same ID ACTIVE and visible;
2. restart published app -> ninth remains visible;
3. add tenth unrelated project -> ninth and tenth both visible;
4. Projects card actions are visually contained at owner-like width;
5. every visible workspace button says `Local workspace`;
6. Bulk-Edit GitHub page loads without repeated warning wall;
7. ScrubBots-Level-Factory GitHub page same;
8. forced cached-last-good 403/429 fixture -> one grouped warning, circuit stops fan-out, stale last-good retained;
9. recovery -> obsolete warning clears and CURRENT returns;
10. H-veAI + one control project follow the same behavior;
11. no M18 full-app black-surface regression.

This is builder QA, **not owner acceptance**. Do not self-certify owner acceptance.

---

# 13. Mandatory regression gates

Run and record at minimum:

1. `npm run typecheck`;
2. `npm run build`;
3. focused mounted M19/Engineering Brief/Projects/Add Project tests;
4. real-layout browser/Playwright Projects geometry test;
5. full frontend tests single-worker;
6. normal/default frontend test invocation;
7. `cargo check --manifest-path src-tauri\Cargo.toml`;
8. focused Rust tests for next_best_task, task_intelligence, registry, github_tracking, github_integration, command_center;
9. exact M16 observational-read test;
10. full `cargo test --manifest-path src-tauri\Cargo.toml --lib --no-fail-fast` with sufficient timeout;
11. production governor concurrency/fake-clock tests;
12. same-HEAD remote validation freshness tests;
13. remote duplicate-ID ambiguity tests;
14. cached 403/429 circuit tests;
15. 8/9/10/20 quota/fairness tests;
16. formatting checks for changed Rust files;
17. full cargo fmt check with unrelated baseline drift distinguished;
18. `git diff --check`;
19. security/redaction tests;
20. accepted publication pipeline;
21. stable EXE SHA-256;
22. builder-native V05 smoke matrix.

Do not claim a full gate when a test merely asserts a constant/cap without exercising the production algorithm.

---

# 14. Required immutable V05 builder log

Create only after implementation, tests, publication, and builder-native smoke are complete:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V05_LOG.md`

The log must include:

- synchronized start SHA;
- final implementation SHA;
- changed files;
- explicit disposition of `F-M19-V04-STRICT-001` through `011`;
- remote duplicate-ID pre-fix/post-fix evidence;
- same-HEAD validation freshness evidence beyond five minutes;
- cached 403 and cached 429 stop-after-first evidence;
- real governor/coalescing concurrency evidence;
- fair 8/9/10/20 portfolio request math and tests;
- manual refresh generation/history ordering evidence;
- actual legacy-vs-M19 attention dedupe evidence;
- actual browser geometry evidence;
- deduped dependency-edge evidence;
- production-governed GitHub acquisition matrix;
- full frontend/Rust results;
- M16 exact result;
- publication result and stable EXE SHA-256;
- complete builder-native smoke matrix;
- explicit `TASKS.md` and `CODEX_ROADMAP.md` unchanged statement;
- explicit M20 not started statement;
- explicit owner-native acceptance not self-claimed statement.

The builder log remains a claim until independent source re-audit.

---

# 15. Stop gate

After all work:

1. verify trackers unchanged;
2. verify M20 not started;
3. commit implementation;
4. create/commit immutable V05 log only after implementation SHA is final;
5. push non-force to `origin/main`;
6. verify clean worktree;
7. verify local HEAD == origin/main == live GitHub main;
8. verify V05 log reachable on GitHub;
9. stop for independent strict re-audit.

Do **not** request owner-native acceptance until the independent V05 source audit returns PASS.