# M19 Next Best Task AI + Engineering Brief V04 - Independent Strict Re-Audit

## Verdict

**CHANGES_REQUIRED**

M19 is not ready for owner-native acceptance and must not be marked PASS/CLOSED. M20 must remain unopened.

This audit treats the V04 builder log as claims, not acceptance evidence, and reviews the implementation commit directly.

## Audited state

- V04 authoritative prompt commit: `9bd8f4aaa88c8d66b38b761beecb825bae4f29f4`
- V04 implementation commit: `89ad88c43d586dd233b72469e8c0d6856dee17c5`
- V04 immutable log commit: `7d24cdf9316abd00677179f1597b06d63b4e47aa`
- Implementation is exactly one commit ahead of the V04 prompt baseline.
- `TASKS.md` and `CODEX_ROADMAP.md` are absent from the implementation diff.
- No M20 implementation was observed in the V04 diff.

## Source-positive V04 closures

The following V04 work is materially source-positive and should be preserved:

- local and remote candidate inputs now enter one dependency graph and uniquely resolved unfinished prerequisites create derived eligibility blockers;
- local and remote failure-evidence lookup now use the same `Result` path instead of remote `unwrap_or_default()`;
- failure evidence now includes a stable evidence ID and latest passing evidence can suppress older failure urgency;
- Registry Add Project now returns an authoritative `CREATED`, `ALREADY_ACTIVE`, `RESTORED_ARCHIVED`, or `RESTORED_MISSING` disposition;
- same-path Registry recovery invokes the shared repository-identity validator before reactivation;
- frontend success copy uses the native registration disposition rather than guessing from the visible project list;
- the visible project-card action remains `Local workspace`;
- M19 snapshot computation remains separate from the explicit M19 history write command;
- the previous full-Rust builder run is reported as 537/537 passing.

These positives do not close the residual findings below.

---

## F-M19-V04-STRICT-001 - CRITICAL - Duplicate remote explicit task IDs still collapse instead of failing closed

### Evidence

`github_tracking::parse_root_tasks()` materializes every remote row with `RemoteTaskRow.id` equal to the explicit task ID parsed from root `TASKS.md`. It does not reject or preserve a separate unique row identity when the same explicit ID appears twice.

`next_best_task::build_candidate_set()` then:

1. builds `all_states` as a `HashMap<(project_id, task_id), state>`, so two remote rows with the same task ID overwrite each other;
2. builds `canonical_ids` as alias -> `Vec<String>`, but only pushes an ID if the identical string is not already present;
3. therefore two distinct remote rows both named `TASK-A` collapse to one string and appear uniquely resolved rather than ambiguous.

A dependent task can consequently resolve against duplicated remote IDs and use whichever duplicated state won the `HashMap` overwrite.

### Required remediation

Remote task materialization must retain a unique row identity separately from explicit task ID. Dependency resolution must index explicit IDs to unique canonical rows and fail closed whenever more than one canonical row has the same normalized explicit ID.

Direct remote production-path tests are required for duplicate explicit IDs with conflicting completion states and for a dependent that references the duplicated ID.

---

## F-M19-V04-STRICT-002 - CRITICAL - Unchanged remote repositories age out of M19 after five minutes even when HEAD was freshly revalidated

### Evidence

M19 remote candidate admission requires `RemoteTrackingSnapshot.fetched_at` to be within `REMOTE_M19_MAX_AGE`, currently five minutes.

However, `github_tracking::observe_project()` fetches current remote HEAD and, when `same_head_cache_is_reusable()` is true, immediately returns `snapshot.clone()` without updating `fetched_at` or persisting a new validation timestamp.

Therefore a quiet repository can be successfully revalidated by the scheduler every cycle while its cached `fetched_at` remains the original content-fetch time. Once that timestamp exceeds five minutes, M19 rejects the project as stale until the repository HEAD changes.

### Required remediation

Separate content-fetch time from remote-validation time, or truthfully refresh a dedicated `validated_at` observation timestamp after a successful same-HEAD check. M19 decision freshness must use current successful validation evidence, not require repository content to change every five minutes.

Add tests proving a repository with unchanged HEAD remains recommendation-eligible across repeated validation cycles beyond five minutes, while an actually unvalidated/stale snapshot still fails closed.

---

## F-M19-V04-STRICT-003 - CRITICAL - Cached 403/429 responses do not open the portfolio rate-limit circuit

### Evidence

Production `guarded_resource()` sends request failures through `stale_or_unavailable()`.

When last-known-good cache exists, `stale_or_unavailable()` returns:

- `state = "STALE"`;
- the original 403/429 rate-limit error in `error`.

But the primary acquisition loop opens the process-wide circuit only when:

`resource.state == "RATE_LIMITED"`

The optional-enrichment request budget likewise sets `rate_limit_open` only when the returned state equals `RATE_LIMITED`.

This means the most important production case, a rate-limited request with last-known-good cache, is labeled STALE and does **not** trip the circuit. Remaining primary and optional resources can continue making requests after GitHub has already reported quota exhaustion.

The grouped-warning code can hide the repetition in UI, but it does not stop the network fan-out.

### Required remediation

Rate-limit detection must be based on structured failure classification independent of presentation/cache state. A cached response may be presented as STALE while still carrying a machine-readable `RATE_LIMITED` cause that immediately opens the process-wide circuit and halts all remaining fan-out.

Tests must cover 403 and 429 with an existing last-known-good cache, not only the empty-cache case.

---

## F-M19-V04-STRICT-004 - CRITICAL - The 48-request governor is a starvation cap, not a fair portfolio-safe acquisition policy

### Evidence

Production constants currently allow 48 GitHub API requests per process hour while one cold repository primary snapshot can require eight primary requests and optional enrichment can request up to 40 more resources.

Consequences:

- eight cold projects require 64 primary requests before optional enrichment;
- only six complete eight-resource primary snapshots fit inside 48 requests;
- later projects can receive `GITHUB_PROCESS_REQUEST_BUDGET_EXHAUSTED` for the remainder of the hour;
- a single PR/action-rich repository can consume 8 primary + up to 40 optional requests, exhausting the entire process budget by itself;
- the 30-second cache freshness window makes repeat consumption easy unless every read is perfectly coalesced.

The V04 budget test does not prove portfolio service. It computes `min(cold_primary_demand, 48)` and asserts the result is at most 48 and below 60. That is tautologically true and explicitly permits project starvation.

The builder log table similarly labels `min(64,48)=48` as a quota success for eight projects without proving that all eight projects receive current primary evidence.

### Required remediation

Implement a fair scheduler/admission design that reserves primary capacity across the portfolio and prevents optional enrichment from consuming capacity required for primary health. Optional evidence should be lazy/on-demand or governed by a materially smaller separate budget.

The budget proof must demonstrate service/fairness, not merely a global cap. For 8, 9, 10, and 20 projects, prove which primary resources can be current, the refresh cadence, selected-project behavior, manual refresh behavior, and the maximum network calls without deterministic starvation.

---

## F-M19-V04-STRICT-005 - MAJOR - Production request coalescing does not reliably share the in-flight result and is not exercised by the unit transport tests

### Evidence

The actual governor/coalescing implementation is behind `#[cfg(not(test))]`. Under tests, `load_or_fetch()` bypasses `guarded_resource()` and directly uses the fixture transport. Therefore ordinary FixtureTransport unit tests cannot exercise production `RequestAdmission`, the in-flight set, or the production coalescing wait path.

In production, a coalesced caller polls SQLite cache only 20 times at 5 ms, approximately 100 ms total. Network requests may take seconds and the configured HTTP timeout is far longer. If no cache exists, a legitimate concurrent caller can return `GITHUB_REQUEST_COALESCED_NO_CACHE` before the owner request finishes. If old cache exists, the coalesced caller can immediately return that old cache as STALE rather than receive the owner request's eventual current result.

This is not true result sharing.

### Required remediation

Use a real per-key shared completion primitive/result slot so concurrent callers wait on or subscribe to the same bounded in-flight result. Production governor/coalescing logic must be testable, for example by moving it out of `cfg(not(test))` and injecting clock/transport state.

Add a deterministic slow-owner concurrent test proving two same-key callers cause one network request and both receive the same final CURRENT result.

---

## F-M19-V04-STRICT-006 - MAJOR - Manual refresh history recording still occurs before the native GitHub refresh completes

### Evidence

Frontend manual refresh performs:

1. `refreshGitHubTracking()`;
2. `recordNextBestTaskHistory()`;
3. refresh the displayed Command Center snapshot.

But native `GitHubTrackingManager::refresh_now()` merely sets `refresh_requested` and immediately returns `Ok(0)`. Actual GitHub observation is performed asynchronously later by the polling loop.

Therefore the frontend promise resolving does not mean GitHub refresh succeeded or completed. M19 history can be recorded from pre-refresh evidence.

The mounted test checks only invoke ordering. Its mock resolves `hiveai_github_tracking_refresh` immediately, so it does not prove scheduler completion or successful fresh evidence.

### Required remediation

Define an explicit refresh generation/completion contract. The user-triggered lifecycle must either:

- capture the prior fingerprint intentionally, await the requested remote refresh generation, then compute the post-refresh comparison; or
- otherwise provide a clear, deterministic previous-vs-current history contract.

Do not equate "refresh requested" with "refresh completed".

---

## F-M19-V04-STRICT-007 - MAJOR - Semantic attention dedupe is still asymmetric between real legacy and M19 item shapes

### Evidence

`semanticAttentionKey()` chooses `family = "evidence"` whenever either evidence or detail is present. It then uses `evidence || detail` as the identity payload.

Real legacy `AttentionItem` objects carry `detail` but no evidence array, while M19 attention objects commonly carry evidence. The same factual issue can therefore generate:

- a legacy key based on normalized detail;
- an M19 key based on evidence path/identity.

Those keys differ and the issue remains duplicated.

The new test does not represent this real shape mismatch. It compares two objects that both have the same evidence array and only changes category/title wording.

### Required remediation

Expose or derive a stable factual issue identity that both legacy and M19 projections can share. Tests must use one actual legacy-shaped item without evidence and one M19-shaped item with evidence for the same issue, then prove one rendered item. Distinct blockers/evidence/provider issues must remain distinct.

---

## F-M19-V04-STRICT-008 - MAJOR - The project-card "geometry test" mocks the geometry it later asserts

### Evidence

`tests/m19-v04-project-card-geometry.test.tsx` replaces `HTMLElement.prototype.getBoundingClientRect` with hand-authored rectangles. Those mocked rectangles already place the footer and delete button inside the mocked card, then the test asserts that the mocked values are contained.

The test therefore does not measure the actual CSS/layout engine and can pass even if the real card overflows.

It also does not prove all V04 geometry requirements, including actual archive/remove non-overlap, horizontal document overflow, focus-outline clipping, and real click targets under the native owner-sized layout.

### Required remediation

Use a real layout engine such as Playwright/Chromium or a deterministic native-webview/browser smoke that evaluates actual computed geometry. Do not mock `getBoundingClientRect` for the acceptance geometry test.

---

## F-M19-V04-STRICT-009 - MAJOR - Duplicate dependency references are not fully deduplicated in the canonical graph

### Evidence

During dependency normalization, each dependency reference pushes the resolved prerequisite into `dependency_task_ids` and can append the same generated unfinished blocker. Duplicate references therefore remain duplicated in the canonical candidate data and explanation/blocker surface.

The later unlock calculation uses a `HashSet`, so unlock score happens to deduplicate the prerequisite for that one calculation. The test only checks that `unblocks == 1`; it does not prove one canonical edge and one blocker/evidence identity.

### Required remediation

Deduplicate normalized dependency references before graph resolution and retain one canonical edge per dependent/prerequisite pair. Test both eligibility/blocker output and unlock scoring.

---

## F-M19-V04-STRICT-010 - MAJOR - The mandatory verification/evidence matrix is still incomplete

### Evidence

The source-positive full Rust and frontend suite results are useful, but the required targeted matrix remains incomplete.

Examples:

- no direct remote duplicate-ID ambiguity test;
- no unchanged-HEAD-over-five-minutes M19 freshness test;
- no cached-403/cached-429 circuit-stop test;
- no real production governor/coalescing concurrency test because the governor is compiled out under unit tests;
- the portfolio budget test proves only a numeric cap, not fair current evidence for all projects;
- the geometry test uses mocked rectangles rather than actual layout;
- the semantic attention test does not use actual legacy-vs-M19 item shapes.

The builder log's public acquisition matrix is also missing several fields required by V04 on a per-row basis, such as fetched-at/age, production governor budget consumed, and full rate-limit metadata or explicit per-row unavailability. The shell probe correctly labels itself outside the production governor, so it cannot substitute for production acquisition evidence.

### Required remediation

Complete the direct production-path matrix for every residual finding above and publish an evidence matrix that distinguishes shell/public probes from actual app-governed acquisition.

---

## F-M19-V04-STRICT-011 - MAJOR - Required builder-native owner-window smoke was not performed as specified

### Evidence

V04 required builder-native QA against the published executable for the actual owner-facing acceptance blockers: ninth-project restoration and restart persistence, tenth project visibility, real card containment, `Local workspace` copy, Bulk-Edit and ScrubBots-Level-Factory GitHub pages, forced rate-limit stale behavior, recovery to CURRENT, H-veAI/control repository behavior, and M18 black-surface containment.

The log records publication smoke, window title/ready marker, shortcut, port and console checks, but does not record those eleven interactive builder-native scenarios. It explicitly states that the recorded smoke is builder evidence and owner acceptance is pending, which is correct, but the mandatory V04 builder-native matrix itself is still absent.

### Required remediation

Run and record the bounded published-build smoke matrix before requesting independent source PASS. This remains builder QA, not owner acceptance.

---

# Gate assessment

| Gate | Result |
|---|---|
| V04 implementation/log chain | PASS |
| Tracker/M20 boundary | PASS |
| Dependency unfinished-state derivation | PARTIAL PASS - unique IDs improved, remote duplicate ambiguity and duplicate-edge normalization remain defective |
| Failure evidence local/remote fail-closed | SOURCE PASS |
| M19 pure snapshot | PASS |
| M19 history product lifecycle | FAIL |
| Remote recommendation freshness | FAIL |
| Registration identity validation | SOURCE PASS |
| Authoritative registration disposition | SOURCE PASS |
| Project card visible copy | SOURCE PASS |
| Project card real geometry proof | FAIL |
| Attention semantic dedupe | FAIL |
| GitHub 403/429 circuit | FAIL in cached-last-good case |
| GitHub portfolio request governor | FAIL - bounded but starving/unfair |
| Request coalescing | FAIL/UNPROVEN |
| GitHub evidence matrix | INCOMPLETE |
| Builder-native published smoke matrix | FAIL/INCOMPLETE |
| Owner-native acceptance readiness | NOT READY |

# Final disposition

**CHANGES_REQUIRED**

Do not run owner-native acceptance yet. M19 remains active. M20 remains blocked.

The next remediation should be narrow and source-driven. Preserve the V04 source-positive Registry, failure-evidence, exact-root, Engineering Brief, and visible-copy work. Correct the eleven findings above, complete the real production-path evidence, publish a new immutable builder log, and return for independent source re-audit.