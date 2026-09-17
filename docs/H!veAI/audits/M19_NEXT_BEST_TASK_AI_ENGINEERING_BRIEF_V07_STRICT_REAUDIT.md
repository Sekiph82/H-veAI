# M19 Next Best Task AI + Engineering Brief V07 — Independent Strict Re-Audit

Date: 2026-09-17 (Europe/Istanbul)

## Verdict

**CHANGES_REQUIRED**

M19 remains **OPEN**. M20 remains **BLOCKED**. Owner-native interactive acceptance is **NOT READY**.

This review treats `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_LOG.md` and `docs/H!veAI/evidence/M19_V07_GITHUB_ACQUISITION_MATRIX.md` as builder claims/evidence, then checks the V07 production source path independently.

Reviewed implementation boundary:

- V07 prompt/start SHA: `28b93c1e1fc97514b1830da2f3bc12a6201fb8fc`
- implementation SHA: `4288e232ac946a18432605a32d49d2255abcd82b`
- publication SHA: `0e82b482b951017c5467a220f0cfe8a497641ff4`
- implementation is exactly one commit ahead of the V07 prompt boundary;
- publication adds only the V07 log and V07 acquisition evidence artifact;
- `TASKS.md` and `CODEX_ROADMAP.md` were not changed by V07;
- no M20 source work is present in the V07 implementation diff.

## Source-positive V07 work accepted

The following V06 findings are materially improved/closed at source level and must not regress:

- monotonic failure backoff now starts from the selected/background cadence and increases rather than collapsing to five minutes;
- process-wide tracking governor remains explicit and bounded;
- refresh requests now carry queued generation/project scope records rather than one overwriteable `Option<String>`;
- changed-HEAD declared transport work is explicitly modeled as three bounded stages and the command wait was raised from 30s to 80s;
- Navigation now has a process-wide advancing rotation cursor on the public production path;
- V07 acquisition evidence contains substantially better per-stage fields, explicit network/admission/budget information, and separate demand tables;
- the real React `/projects` Chromium harness now runs approximately 1536/900/640 widths and walks clipping ancestors for focus evidence;
- pre-refresh M19 fingerprint ordering, optional 403/429 global circuit behavior, exact-root authority, duplicate-ID/dependency protections, Registry ninth-project recovery work, integrated Engineering Brief, and source-positive V06 behavior remain intact.

These positives do not close M19 because three production lifecycle defects and two verification/contract gaps remain.

---

## F-M19-V07-STRICT-001 — CRITICAL — Changed-HEAD validation timestamp and scheduler horizon can still create a stale gap

### Evidence

`github_tracking.rs` defines:

- background cadence = 3600s;
- changed-HEAD completion budget = 80s;
- background hard horizon = `3600 + 80 = 3680s`.

However, `observe_project()` captures `fetched_at = utc_timestamp()` **before** the HEAD/TASKS/commit-feed work begins. `parse_root_tasks()` then writes both `content_fetched_at` and `validated_at` from that original `fetched_at` value.

After the observation finishes, the polling scheduler sets the next due time from **result/completion time** plus the 3600-second interval.

Therefore, for a slow successful changed-HEAD observation:

1. validation timestamp is near observation start `T0`;
2. the observation may legitimately finish near `T0 + 80s`;
3. next due is then near `T0 + 3680s`;
4. the old M19 hard horizon also expires at `T0 + 3680s`;
5. the next validation still needs non-zero time to finish and publish a new `validated_at`.

M19 can therefore fail closed for a real gap during the next legitimate validation cycle. In the worst bounded changed-HEAD-to-changed-HEAD case, the current 3680-second proof does not include both the previous observation duration and the next validation duration.

### Why this matters

V07 was specifically required to make the hard M19 acceptance age mathematically compatible with the scheduler that can actually service the portfolio. The current timestamp origin and due-time origin are different, so the arithmetic is not yet closed.

### Required remediation

Use one coherent clock boundary. Acceptable designs include:

- set `validated_at` at successful validation completion, not at pre-network observation start; or
- schedule the next due relative to the same factual validation timestamp; or
- derive a hard horizon that rigorously covers the actual timestamp/due lifecycle without hiding stale periods.

Add a fake-clock two-cycle production test with a slow changed-HEAD observation followed by the next slow validation. Prove there is no unintended M19-unavailable gap before the documented hard horizon and prove failure still does not advance validation freshness.

---

## F-M19-V07-STRICT-002 — CRITICAL — A manual refresh generation can still be completed by an observation that started before that generation existed

### Evidence

V07 improved scope storage by queuing `(generation, project_id)`, but the actual worker job channel still carries only:

`(DatabaseState, ProjectRecord)`

and the result channel still carries only:

`(project_id, observation_result)`.

No generation/observation token is attached to the job/result.

When a result arrives, the scheduler calls `pending_refreshes.settle_project(project_id)` and completes **all currently pending generations for that project** from that result.

If project A is already in flight because of normal selected/background scheduling, and the owner then requests a manual refresh for A:

1. the new generation is queued for A;
2. `in_flight.contains(A)` prevents scheduling a second A observation;
3. the older observation, which may have begun and even fetched HEAD before the manual request existed, returns;
4. `settle_project(A)` releases the new manual generation anyway.

That is not a factual guarantee that generation N completed after an observation performed for generation N.

### Why this matters

`next_best_task::refresh_and_compare()` records the pre-refresh fingerprint only after `refresh_selected()` reports success. A pre-request observation can therefore falsely satisfy the lifecycle and allow history comparison to proceed without the requested post-click validation boundary.

### Required remediation

Bind accepted refresh generations to an observation epoch/token/job that is guaranteed to start after the generation was accepted, or explicitly join only an in-flight observation whose start boundary is known to satisfy the generation contract.

The worker/result path must carry enough identity to prove which generation(s) an observation can settle. Do not settle generations merely because the project IDs match.

Mandatory production scheduler tests:

- scheduled A observation already in flight -> manual A request -> pre-request observation must **not** satisfy the new generation;
- manual A then manual B while unrelated observations run -> each generation settles only from its own qualifying observation;
- same-project coalescing is allowed only when every waiter is attached to the same qualifying post-request observation;
- a failed qualifying observation fails only its attached waiter(s);
- `refresh_and_compare` never records history from a generation released by an older observation.

---

## F-M19-V07-STRICT-003 — CRITICAL — The 80-second manual refresh deadline still ignores scheduler queue delay

### Evidence

V07 correctly derives 80 seconds from three sequential tracking stages:

`3 * (20s HTTP + 5s process grace) + 5s completion overhead = 80s`.

But the polling scheduler has four shared workers and refuses to enqueue another job while `in_flight.len() >= 4`.

A manual refresh can therefore be accepted while all four workers are already executing legitimate background/selected changed-HEAD observations. Each existing job can consume most of the declared multi-stage bound before a worker becomes available. Only then can the newly requested project begin its own up-to-75-second observation.

`refresh_selected()` nevertheless starts its 80-second wait immediately when the request is accepted.

### Consequence

A valid manual refresh can be reported as timed out before its own observation has received enough execution time, even though all workers are behaving within their documented limits.

Removing the timed-out generation from `RefreshCompletion` prevents late history mutation, which is good, but it does not make the caller deadline semantically compatible with the scheduler queue.

### Required remediation

Choose one explicit admission/deadline model, for example:

- reserve/admit a manual slot before starting the lifecycle deadline;
- fail fast with a truthful bounded busy/backpressure state if the manual observation cannot be admitted;
- start the observation deadline only after a qualifying worker/job has been admitted, while keeping a separate bounded queue-admission deadline; or
- derive one end-to-end deadline including the maximum declared queue delay and prove it remains acceptable.

Mandatory test: saturate all four workers with slow valid observations, issue a manual refresh, and prove the command either receives truthful bounded backpressure or receives its full declared observation budget after admission. It must not time out merely because it waited behind unrelated work.

---

## F-M19-V07-STRICT-004 — MAJOR — Required refresh concurrency evidence still tests helper state more than the production scheduler lifecycle

The V07 source contains useful tests for `RefreshScopeGate`, `RefreshGenerationCoordinator`, and `RefreshCompletion`, including A/B completion isolation. Those tests are valuable but they do not execute the real worker channel, `in_flight` state, pending generation queue, result handling, and `settle_project()` lifecycle together.

The authoritative V07 prompt explicitly required manager/scheduler scope tests rather than `RefreshCompletion` in isolation. That missing integration coverage is why F-M19-V07-STRICT-002 was able to survive the green suite.

Required remediation: add a deterministic scheduler/worker seam and test the actual admission -> in-flight -> result -> generation completion path, including pre-existing in-flight work and queue saturation.

---

## F-M19-V07-STRICT-005 — MAJOR — Selected freshness contract is published but not actually exercised by M19 decision logic

`github_tracking::validation_horizon_seconds(true)` exposes a 380-second selected horizon and the V07 evidence artifact publishes a selected target horizon. However, `next_best_task::remote_validation_is_fresh()` calls:

`github_tracking::validation_horizon_seconds(false)`

unconditionally.

M19 therefore has no selected-project freshness input and cannot exercise the 380-second path. The selected cadence still improves data freshness operationally, and a failed validation marks health degraded, but the V07 mandatory condition "selected project uses its selected cadence/horizon correctly" is not demonstrated in the actual M19 eligibility path.

Required remediation: either:

1. formally define 380 seconds as a scheduler target only, rename/document it so it is not represented as an M19 hard horizon, and test that distinction; or
2. pass selected-project context into the M19 remote freshness decision and exercise the selected horizon there.

Do not keep a public/tested selected horizon that the decision engine cannot actually use while the evidence artifact implies otherwise.

---

# V07 gate assessment

| Gate | Result |
|---|---|
| V07 implementation boundary / one implementation commit | PASS |
| Trackers unchanged / M20 not started | PASS |
| Monotonic backoff | PASS at source/unit level |
| Navigation cursor advances on production public path | PASS at source level |
| V07 acquisition matrix generation | PASS / materially improved |
| Real React 1536/900/640 geometry | PASS at source/log level |
| Changed-HEAD freshness horizon compatibility | **FAIL** |
| Refresh generation tied to qualifying observation | **FAIL** |
| Manual deadline compatible with worker admission/queue | **FAIL** |
| Production scheduler concurrency verification | **INCOMPLETE** |
| Selected freshness contract in actual M19 path | **PARTIAL / INCOMPLETE** |
| Full Rust/frontend regressions | Builder reports PASS; no contradiction found in reviewed source |
| Owner-native acceptance | **NOT READY** |

## Next action

Perform one **V07 R02 narrow lifecycle remediation**, not a broad M19 redesign. Close only the five findings above, preserve all accepted V07 work, publish the corresponding immutable R02 log/evidence, then stop for another independent source check.

If that source check passes, proceed directly to owner-native published-app acceptance. Do not start M20 before M19 owner acceptance and tracker closure.
