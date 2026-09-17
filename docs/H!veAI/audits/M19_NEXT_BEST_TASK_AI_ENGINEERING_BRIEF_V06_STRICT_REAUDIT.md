# M19 Next Best Task AI + Engineering Brief V06 — Independent Strict Re-Audit

Date: 2026-09-17

Verdict: **CHANGES_REQUIRED**

Scope reviewed:

- authoritative V06 prompt at `48501f5293b3ae2c8749a25dea9eeb4a66ee1525`;
- implementation commit `b8ea73e45000bcbb5c981d601bf72be0f17a5221`;
- immutable builder log published at `da08cf6411246be692cd607bb27205a12dc39529`;
- production Rust/React paths changed by V06;
- generated `docs/H!veAI/evidence/M19_V06_GITHUB_ACQUISITION_MATRIX.md`;
- browser geometry harness.

The V06 implementation closes several V05 findings at source level: native pre-refresh M19 history capture is now correctly ordered, optional enrichment rate limits open the shared process circuit, the Projects geometry harness renders the real React `/projects` surface, and detailed GitHub acquisition now consumes an intent-aware context. Full regression claims in the builder log are source-compatible. However, the following residual defects prevent M19 source acceptance.

## F-M19-V06-STRICT-001 — CRITICAL — Remote tracking cadence is incompatible with M19's five-minute portfolio freshness contract

`src-tauri/src/github_tracking.rs` now validates the selected project every 300 seconds and background projects every 3600 seconds.

`src-tauri/src/next_best_task.rs` still declares:

`REMOTE_M19_MAX_AGE = Duration::minutes(5)`

M19 is a portfolio-wide recommendation engine. Therefore, after a background project's successful validation ages past five minutes, that project's GitHub task evidence becomes ineligible for M19 for roughly the next 55 minutes, even though the new scheduler intentionally will not validate it again until the hourly background turn.

This means the new quota-safe scheduler and the decision-engine freshness policy contradict each other. A project can be healthy and unchanged yet disappear from the recommendation candidate set solely because it is not the selected project.

### Required remediation

Define one explicit portfolio freshness contract that is mathematically compatible with the bounded tracking cadence. Do not solve this by restoring high-frequency portfolio polling.

The design must preserve:

- truthful `validated_at` semantics;
- fail-closed behavior when actual validation is overdue or failed;
- quota-safe 8/9/10/20-project behavior;
- fair eligibility of non-selected active projects in Next Best Task.

Add fake-clock tests that cross the old five-minute boundary and the new background validation boundary for selected and non-selected projects.

## F-M19-V06-STRICT-002 — CRITICAL — Tracking failure backoff is inverted and collapses to a fixed five-minute retry

In `polling_loop`, both degraded snapshots and hard observation errors derive retry from `PORTFOLIO_REFRESH_SECONDS` and then apply:

`min(retry, Duration::from_secs(300))`

Because `PORTFOLIO_REFRESH_SECONDS` is already 3600 seconds, every computed retry is reduced to 300 seconds. The multiplier never creates increasing backoff.

A failed background portfolio can therefore retry every five minutes, consume the shared 40-stage tracking governor, and starve otherwise healthy validation work. This contradicts the V06 requirement that failures use bounded backoff and the log's quota narrative.

### Required remediation

Use a monotonic bounded retry policy with explicit minimum/base and maximum. Failure retry must not become more aggressive than normal background cadence by accident. Prove with fake-clock tests that repeated failures back off monotonically and do not monopolize the tracking governor across 8/9/10/20 projects.

## F-M19-V06-STRICT-003 — CRITICAL — Concurrent scoped refresh generations can overwrite each other's project scope

`refresh_selected(project_id)` performs three independent actions:

1. increments a generation in `RefreshCompletion`;
2. stores the project in `RefreshScopeGate(Mutex<Option<String>>)`;
3. sets one boolean `RefreshRequestGate`.

`RefreshScopeGate::request()` overwrites the single stored `Option<String>`. Multiple rapid refresh requests can therefore create multiple generations while retaining only the latest project scope. `polling_loop` then reads the latest requested generation and one scope and can complete all waiters when that one scope drains.

For two near-simultaneous refresh requests targeting different projects, the first caller can be released even though its own project was never refreshed. This is especially serious because `refresh_and_compare` uses refresh completion as the factual boundary before persisting M19 history.

The existing V06 generation test exercises `RefreshCompletion` alone. It does not test the manager's generation + scope coalescing behavior.

### Required remediation

Associate scope with generation. Use a bounded queue/state machine or equivalent per-generation contract so a generation can only complete after its declared project scope has settled. Add concurrency tests with at least two different projects and rapid overlapping requests. No generation may inherit another generation's scope.

## F-M19-V06-STRICT-004 — CRITICAL — Manual refresh timeout is shorter than the declared bounded changed-HEAD work

`refresh_selected()` waits only 30 seconds for its generation.

A changed remote HEAD can perform three sequential tracking HTTP stages:

- HEAD Atom observation;
- root `TASKS.md` fetch;
- Atom commit-feed fetch for commit metadata.

Each `run_http()` has a 20-second curl timeout plus bounded process-wait overhead. Therefore valid bounded changed-HEAD work can exceed 30 seconds without being hung.

The command can return timeout while the worker continues and later succeeds. In that case the M19 refresh/history lifecycle reports failure even though the declared bounded tracking operation was still legitimately in progress.

### Required remediation

Derive the refresh lifecycle timeout from its maximum scoped work, or redesign the observation so the declared scope has one compatible bounded deadline/cancellation policy. Add a deliberately slow changed-HEAD production-path test proving that work inside the documented network bounds does not falsely timeout.

## F-M19-V06-STRICT-005 — CRITICAL — Navigation fair rotation resets at every public portfolio call and can permanently starve resource kinds

V06 added `rotation_cursor` to `GitHubAcquisitionContext`, and `fetch_resources_with_context()` rotates the eight primary resource specifications using it.

However, public `portfolio_snapshots()` always calls `portfolio_snapshots_with_transport(..., rotation_cursor = 0, ...)`.

Inside that call each project uses `rotation_cursor + index`, so fairness exists within one invocation. There is no process/durable window cursor that advances between separate Navigation invocations.

For a 20-project portfolio, Navigation admits two primary kinds per project per invocation. Repeating the same public call produces the same starting rotation and can repeatedly service the same two kinds for a given project while its other six resource kinds remain perpetually stale unless another Selected/Manual path happens to refresh them.

This does not satisfy the V06 requirement that repeated navigation rotate service across windows.

### Required remediation

Advance fair rotation across acquisition windows, not only across projects within one call. The rotation state must be process-safe and testable, and reset semantics must be explicit. Add repeated-window tests demonstrating eventual service of all eight primary resource classes for every project under constrained budgets.

## F-M19-V06-STRICT-006 — MAJOR — Generated acquisition evidence does not fully match the mandatory V06 evidence contract

The generated artifact is useful and materially better than the earlier prose-only evidence. However its declared row schema omits several fields explicitly required by V06 when relevant, including fair-rotation cursor/position, explicit network yes/no, validation timestamp, bounded diagnostic, and rate-limit metadata/unavailability. `resourceKind` is also emitted generically as `PRIMARY_GITHUB_RESOURCE` while the exact endpoint is pushed into `stage`.

The top scheduler table reports background-only hourly counts. It does not present the selected-project 300-second cadence in the same exact 8/9/10/20 product math, even though source tests show that selected mode adds twelve observations/hour.

### Required remediation

Regenerate evidence from the final production seam with the complete contract. Separate detailed integration API quota from remote root-TASKS tracking quota, and publish both idle/background and selected-owner scenarios explicitly.

## F-M19-V06-STRICT-007 — MAJOR — Browser geometry gate omits the required ~900px viewport and does not prove ancestor focus clipping

The V06 prompt required representative widths approximately 1536, 900, and 640 pixels.

`scripts/m19-v06-browser-geometry.mjs` runs 1536, 1280, and 640. The builder log incorrectly describes those as the requested viewport classes.

The harness now correctly renders the real React Projects route and uses real browser geometry, which closes the previous cloned-DOM defect. However the focus assertion checks active element plus outline/box-shadow presence; it does not verify that the focus visual is not clipped by any ancestor overflow boundary as explicitly required.

### Required remediation

Run the production Projects route at approximately 1536, 900, and 640 widths. Add a real clipping assertion for the focused footer actions, or another direct browser proof that the visible focus indicator remains within unclipped ancestor geometry.

# Source-positive V06 work accepted for preservation

The following V06 work should not be rewritten unless necessary to close the findings above:

- native `refresh_and_compare` captures the pre-refresh M19 snapshot, awaits successful scoped refresh, then records that exact prior fingerprint and computes the current comparison;
- frontend Command Center uses the combined native refresh-and-compare lifecycle rather than the old post-refresh history command chain;
- optional enrichment 403/429 now opens the same process-wide detailed-GitHub circuit;
- detailed GitHub snapshot acquisition consumes `GitHubAcquisitionContext` and supports Idle/Selected/Navigation/Manual intent;
- idle detailed integration uses cache-only semantics rather than eight-resource fan-out;
- the V06 browser harness loads the actual React `/projects` route;
- remote duplicate-ID, dependency, same-HEAD freshness, in-flight coalescing, Registry recovery, and V05 source-positive semantics remain preserved;
- full regression/publication evidence in the immutable V06 log is accepted as builder evidence, not owner acceptance.

# Gate result

M19 remains **ACTIVE / CHANGES_REQUIRED**.

Do not start M20.

Do not request owner-native acceptance yet. Owner-native acceptance is appropriate only after the residual source findings above are closed and independently re-audited PASS.
