# M19 Next Best Task AI + Engineering Brief V05 — Independent Strict Re-Audit

## Verdict

**CHANGES_REQUIRED**

M19 is not ready for owner-native acceptance. M20 must remain unopened.

This audit treats `M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V05_LOG.md` as builder claims and reviews the implementation commit directly.

## Audited state

- V05 authoritative prompt baseline: `f45f5ad047a2cb8cacab271f2ba20eef8a3568e7`
- V05 implementation commit: `496d906e9ace7870e0e24704c56dfe6dce07e349`
- V05 immutable log commit: `3410e53a1d301db059cebe732512418c6d268b3f`
- Implementation is exactly one commit ahead of the V05 prompt baseline.
- `TASKS.md` and `CODEX_ROADMAP.md` are absent from the implementation diff.
- No M20 implementation is present in the V05 implementation diff.

## Source-positive V05 work to preserve

The following work is materially source-positive and should not be regressed:

- remote task rows now carry a unique `canonical_row_id` separate from the owner-authored explicit task ID;
- M19 dependency resolution indexes aliases to canonical rows and duplicate explicit IDs can fail closed;
- same-HEAD GitHub tracking now refreshes a dedicated validation timestamp while retaining a separate content-fetch timestamp;
- primary 403/429 failures are recognized independently from the stale-cache presentation state;
- real in-flight result sharing uses a condition variable/result slot and is compiled into the production path;
- dependency references are normalized/deduplicated before graph resolution;
- the browser geometry harness uses Chromium rather than mocked `getBoundingClientRect`;
- full frontend and Rust suites are reported passing in the immutable V05 log.

Those positives do not close the findings below.

---

## F-M19-V05-STRICT-001 — CRITICAL — The fair/tiered GitHub acquisition plan is not wired into the real integration acquisition path

### Evidence

`github_integration.rs` defines `AcquisitionIntent`, `portfolio_acquisition_plan()` and `fair_primary_schedule()`, and tests exercise those helpers.

However the real product entry point still does:

`github_integration::snapshot()` → `snapshot_with_transport()` → `fetch_resources_with_transport()`

and `fetch_resources_with_transport()` iterates the same eight primary resource specifications for the current snapshot, followed by optional enrichment. The function receives no acquisition intent, project-count schedule, round-robin cursor, or selected/navigation plan.

The process governor itself remains a first-come global counter: 40 primary + 8 optional per hour. `fair_primary_schedule()` does not drive which actual project/resource request is admitted.

Therefore the V05 fairness proof is a proof of helper arithmetic, not of production scheduling. Under rapid navigation, earlier snapshots can still consume the 40 primary admissions before later projects are serviced.

### Required remediation

Wire acquisition intent into the actual product path. The production scheduler/admission layer, not a detached helper, must decide which project/resource is serviced.

Tests must invoke the real snapshot/acquisition orchestration across multiple projects and prove round-robin service under constrained budget. A constant-plan unit test is insufficient.

---

## F-M19-V05-STRICT-002 — CRITICAL — Remote-tracking network cadence and manual refresh contradict the published request math

### Evidence

`github_tracking.rs` still defines:

- `SELECTED_PROJECT_REFRESH_SECONDS = 10`
- `PORTFOLIO_REFRESH_SECONDS = 30`

The polling loop schedules every active GitHub-tracked project repeatedly. A non-selected project can therefore perform roughly 120 HEAD observations per hour, while a selected project can perform roughly 360 per hour.

For 20 idle projects, the unchanged-HEAD path alone can therefore approach roughly 2,400 HEAD observations per hour, not the `20` remote-tracking calls shown in the V05 request table.

These observations use `github_tracking::run_http()` directly against GitHub/Raw endpoints and are outside `ProcessRequestGovernor` in `github_integration.rs`.

The manual refresh contract also does not match the V05 table. `refresh_now()` has no project argument. When `refresh_requested` is consumed, the polling loop sets all known `next_due` entries to `now`, then proceeds to observe the portfolio in waves. Thus Command Center manual refresh is portfolio-wide remote tracking, not one selected-project tracking call.

With four workers and a 30-second `refresh_now()` wait bound, a larger portfolio can also time out before all slow observations settle because each network observation itself can approach the HTTP timeout.

### Required remediation

Unify the actual remote-tracking cadence and manual-refresh scope with the documented product contract.

At minimum:

- publish exact requests/hour from the real scheduler intervals;
- remove the 10/30-second all-portfolio polling storm;
- make manual refresh scope explicit and bounded, preferably selected-project scoped for the owner-facing Command Center refresh unless an explicit portfolio refresh is requested;
- ensure the refresh-generation timeout is compatible with the maximum work actually included in that generation;
- either govern remote-tracking network use with a shared coordinator or justify a separate bounded policy with measured math.

Do not claim `N` tracking requests/hour while production schedules approximately `120 × N` unchanged-HEAD observations/hour.

---

## F-M19-V05-STRICT-003 — CRITICAL — M19 manual-refresh history still records the post-refresh state as the baseline

### Evidence

Frontend manual refresh currently executes:

1. `refreshGitHubTracking()` and waits for its promise;
2. `recordNextBestTaskHistory()`;
3. `refresh()` / read Command Center snapshot.

The native `hiveai_next_best_task_record_history` command computes a fresh `next_best_task::snapshot()` at the time it is called, then persists that snapshot's fingerprint.

Because the history command is called **after** GitHub refresh completion, the persisted fingerprint is the post-refresh/current state. The immediately following Command Center read compares the current state to the just-persisted current state. This can collapse a real refresh change into `NO_COMPARABLE_CHANGE`.

The mounted V05 test proves only that the history command is not called while the refresh promise is unresolved. It does not prove that the persisted fingerprint is the pre-refresh factual state.

### Required remediation

Make previous-vs-current atomic and factual.

One valid design is a native refresh-and-compare lifecycle that:

1. captures the pre-refresh M19 fingerprint;
2. requests/awaits the intended remote refresh generation;
3. on the defined successful/degraded completion boundary, persists the captured **pre-refresh** fingerprint exactly once;
4. computes/returns the post-refresh snapshot and comparison.

Do not recompute the history baseline after refresh and call it `previous`.

Direct tests must mutate remote evidence between the pre/post phases and prove `CHANGED`, plus prove `NO_COMPARABLE_CHANGE` when evidence truly does not change.

---

## F-M19-V05-STRICT-004 — CRITICAL — Optional-enrichment 403/429 does not open the process-wide rate-limit circuit

### Evidence

Primary acquisition checks `resource_is_rate_limited()` and calls `open_portfolio_rate_limit_circuit()`.

But `acquire_enrichment()` and `acquire_enrichment_text()` only set the local `RequestBudget.rate_limit_open = true` when an optional request is rate-limited. They do not open `PORTFOLIO_RATE_LIMIT_UNTIL`.

Consequences:

- remaining optional work in the same snapshot is stopped, which is good;
- but the process-wide circuit is not opened;
- a later snapshot can immediately resume primary API calls even though GitHub just returned a 403/429 during optional enrichment.

This violates the V05 contract that the first observed rate-limit cause opens the process-wide circuit independent of which resource class observed it.

### Required remediation

Any structured `RATE_LIMITED` cause from primary or optional acquisition must open the same process-wide circuit immediately.

Add direct tests where the first rate limit occurs in PR/action enrichment with current primary resources, then prove a subsequent snapshot makes zero new network calls until the backoff expires and recovery later clears the circuit.

---

## F-M19-V05-STRICT-005 — MAJOR — The published acquisition evidence matrix is descriptive, not the mandatory measured per-stage matrix

### Evidence

The V05 log includes one summary row per repository/class, but the V05 prompt required production-governed rows carrying, where available:

- cache state before request;
- admission decision;
- governor budget before and after;
- in-flight owner/coalesced state;
- network yes/no;
- HTTP code/status class;
- failure cause;
- fetched/validated timestamps;
- last-good age;
- final presentation state;
- sanitized diagnostic;
- rate-limit metadata or explicit unavailability.

The log's matrix does not provide those fields per resource/fetch stage and therefore cannot independently substantiate the request-budget claims.

### Required remediation

Generate an immutable redacted evidence artifact from the production-governed test seam with one row per acquisition stage for H-veAI, Bulk-Edit, ScrubBots-Level-Factory, and one control repository. Link it from the next builder log.

---

## F-M19-V05-STRICT-006 — MAJOR — Chromium geometry evidence still tests a hand-built clone rather than the actual Projects component

### Evidence

`scripts/m19-v05-browser-geometry.mjs` is a real Chromium layout test and is better than the V04 mocked rectangles. However it builds a synthetic HTML string containing manually copied `registry-card` markup rather than mounting/routing the actual React Projects page/component.

A future DOM/class mismatch in production can therefore leave this test green while the real Projects page regresses.

Its focus-clipping assertion is also only `getComputedStyle(card).overflow !== 'hidden'`, which does not actually measure whether the focused control's outline is clipped by an ancestor.

### Required remediation

Use the actual production Projects component/route in the browser harness, or a built-app browser surface that renders the same React tree. Keep actual computed bounding-box assertions and add a real focused-control visibility/outline containment check.

---

## F-M19-V05-STRICT-007 — MAJOR — Required published-build interactive builder smoke remains explicitly incomplete

### Evidence

The V05 prompt explicitly required builder-native QA against the published executable for ninth-project restoration/restart persistence, tenth-project visibility, project-card containment, Bulk-Edit/ScrubBots warning behavior, rate-limit stale/recovery, control repositories, and the M18 black-surface regression.

The V05 immutable log explicitly marks several of those scenarios as `not self-claimed` because no owner-native UI acceptance channel was available.

That honesty is correct, but it means the required V05 gate was not completed.

### Required remediation / workflow adjustment

Do not fabricate interactive evidence.

For V06, builder source acceptance should require:

- production source + direct tests;
- actual production Projects-route browser geometry;
- accepted publication helper + executable smoke.

The remaining human-visible interactions should then be performed as the **owner-native acceptance immediately after independent source PASS**, rather than repeatedly requiring Codex to claim a UI channel it does not possess.

---

# Gate assessment

| Gate | Result |
|---|---|
| V05 implementation/log chain | PASS |
| Tracker/M20 boundary | PASS |
| Duplicate explicit task-ID handling | SOURCE PASS |
| Same-HEAD validation freshness | SOURCE PASS |
| Dependency edge deduplication | SOURCE PASS |
| Primary cached 403/429 stop-after-first | SOURCE PASS |
| In-flight same-key result sharing | SOURCE PASS |
| Fair/tiered detailed GitHub acquisition | FAIL — helper plan is not wired into production orchestration |
| Remote tracking cadence/request math | FAIL |
| Manual refresh previous-vs-current history | FAIL |
| Optional-enrichment process-wide rate-limit circuit | FAIL |
| Production-governed evidence matrix | INCOMPLETE |
| Actual Projects-route geometry proof | PARTIAL |
| Published builder interactive smoke | INCOMPLETE / intentionally deferred |
| Owner-native acceptance readiness | NOT READY |

# Final disposition

**CHANGES_REQUIRED**

M19 remains ACTIVE. M20 remains blocked.

The next remediation must be narrow. Preserve V05 source-positive canonical-row, same-HEAD freshness, primary rate-limit, coalescing, dependency-dedupe, Registry, and visible-copy work. Fix the four production semantics above, publish real acquisition evidence, replace the synthetic Projects geometry fixture with the actual product route/component, then return for one more independent source re-audit.

If that source audit passes, proceed directly to owner-native acceptance instead of requiring Codex to self-claim unavailable interactive owner evidence.