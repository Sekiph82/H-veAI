# M19 Next Best Task AI + Engineering Brief V03 R02 — Independent Strict Re-audit

## Verdict

**CHANGES_REQUIRED**

M19 remains **ACTIVE**. M19 is **not** PASS/CLOSED. M20 must remain unopened.

Builder evidence reviewed:

- Start SHA: `908f8437915722610720dfda47a57748101d5404`
- Implementation SHA: `1fda1a4f35a40bd9e8b02444d4965a1ecf544367`
- Immutable builder-log commit: `08944994975c5d1a6cd5825837a642926fa99348`
- Builder log: `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_LOG.md`
- Authority: `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_REMEDIATION_PROMPT.md` plus the superseded V03 prompt whose requirements remained in force.

The implementation materially improves several areas, but source review still finds recommendation-safety defects and unclosed owner-observed acceptance blockers.

## Positive source findings

The following V03 R02 work is source-positive and should be preserved:

- M19 now calls a pure exact-root `TASKS.md` parser instead of the mutating all-source M09 parser.
- Snapshot comparison read and explicit history write are separated at Rust API level.
- Remote M19 admission now checks Registry identity, branch, age, remote HEAD, root hash, remote health, and per-task hash completeness.
- Rank-aware explanation remains present.
- Projects cards now expose visible `Local workspace` copy.
- A dedicated responsive card stylesheet adds bounded/wrapping action layout.
- Explicit Add Project can reactivate an archived row with the same normalized path and preserve the same project ID.
- GitHub API warnings are grouped and primary fan-out stops after an observed 403/429 in one acquisition cycle.
- The full Rust library suite reported 533/533 PASS, and the previously failing exact M16 observational-read test reported PASS after the pure-read refactor.

These improvements do not close the remaining findings below.

---

# Findings

## F-M19-V03R02-STRICT-001 — CRITICAL — Unfinished dependencies still do not block eligibility

### Evidence

`src-tauri/src/next_best_task.rs::collect_parsed_inputs()` resolves dependency references and adds blockers only when a dependency is missing or ambiguous. It does **not** add a blocker when the dependency exists but is unfinished.

`collect_remote_inputs()` has the same semantic gap. It checks that a dependency ID exists in `known_ids`, but does not add a blocker when that dependency exists with a non-complete status.

`build_candidate_set()` computes unmet dependencies only for the `dependency_unlock` score. `defer_or_retain()` does not consult unmet dependency state. Therefore a dependent task with an existing unfinished prerequisite can still enter `eligible`.

The new test `full_graph_scores_real_prerequisite_and_keeps_dependent_attention()` manually injects `dependent.blockers.push("dependency unfinished")`. That bypasses the production defect instead of proving that the graph derives the blocker itself.

### Required closure

Build per-project canonical task-state maps before eligibility and derive unmet dependencies from actual prerequisite completion state. A task with any uniquely-resolved but unfinished prerequisite must be ineligible and surfaced in attention without requiring a separate `Blocker:` line.

Add direct production-path tests for both local exact-root and remote root-TASKS inputs:

- B depends on unfinished A, no blocker text -> B ineligible;
- A completed -> B can become eligible;
- duplicate dependency reference deduped;
- missing and ambiguous IDs fail closed.

---

## F-M19-V03R02-STRICT-002 — CRITICAL — Remote failure-evidence lookup still fails open

### Evidence

Local input collection propagates `recent_failure_evidence()` errors into uncertainty/unavailable inputs.

Remote input collection does:

`recent_failure_evidence(database, &project.id, &task.id).unwrap_or_default()`

A database/query/timestamp error is therefore converted into an empty evidence set and `verified_failure_urgency = 0`. That is a favorable default rather than fail-closed uncertainty.

### Required closure

Use one shared structured failure-evidence function for local and remote candidates. Query/parsing failures must create explicit uncertainty/attention/unavailable input and must prevent a recommendation when the missing evidence is material under the accepted scoring contract.

Add tests for remote query failure, malformed timestamp, stale failure, resolved failure, unrelated failure, and fresh linked failure.

---

## F-M19-V03R02-STRICT-003 — CRITICAL — The explicit M19 history write exists but is not wired into the product lifecycle

### Evidence

Rust now exposes `hiveai_next_best_task_record_history` and `next_best_task::record_history()`.

However `src/commandCenter.ts` exposes `getCommandCenterSnapshot()`, `getNextBestTaskSnapshot()`, tracking refresh/select calls, and event listeners, but no history-recording invocation. `src/command_center_view.tsx` refreshes Command Center snapshots and GitHub tracking but does not call the history command.

As shipped, normal UI reads can therefore remain at `UNAVAILABLE_FIRST_SNAPSHOT` indefinitely unless some external/manual caller invokes the native history command.

### Required closure

Wire history recording to one explicit intentional lifecycle action, for example a successful user/manual refresh or a bounded post-refresh commit step. The snapshot computation itself must remain observational. First snapshot must be unavailable, an explicit record must establish history, and the next comparable snapshot must truthfully report no-change/change.

Add mounted/native-path tests that exercise the actual UI/native refresh lifecycle, not only direct Rust unit calls to `record_history()`.

---

## F-M19-V03R02-STRICT-004 — CRITICAL — GitHub rate-limit architecture still lacks the required proactive portfolio budget/coalescing and can still deterministically exhaust unauthenticated API quota

### Evidence

`src-tauri/src/github_integration.rs` still defines a per-resource cache freshness of 30 seconds and a per-snapshot enrichment budget. Each uncached snapshot can request eight primary API resources before optional enrichment.

The new process-wide circuit opens only **after** a 403/429 has already occurred. It is a reactive backoff, not the required proactive portfolio-level request budget.

There is no source-level in-flight request coalescing for concurrent callers of the same repository/branch/resource, no portfolio token/request budget before the limit is hit, no ETag/Last-Modified conditional request implementation, and the production `curl.exe` transport still sends only Accept/API-version/User-Agent headers with no authenticated-read integration.

The builder log does not provide the required worst-case hourly request calculations for 8, 9, 10, and 20 projects.

The existing test proves only that one acquisition stops after the first synthetic 403. It does not prove that normal idle operation avoids reaching that 403 in the first place.

### Required closure

Implement a real portfolio request governor shared by GitHub integration callers:

- bounded process-wide request budget/token bucket or equivalent;
- per-repository/resource in-flight coalescing;
- shared cache reuse across panels/readers;
- optional enrichment priority below primary reads;
- cadence that is mathematically below the intended quota for 8/9/10/20 projects;
- reset-aware backoff when response metadata is available;
- conditional validators where practical;
- use a secure existing authenticated mechanism if one exists, otherwise document a viable explicit owner-controlled auth path or a quota-safe unauthenticated design.

Add deterministic request-count tests and publish the hourly budget math.

---

## F-M19-V03R02-STRICT-005 — MAJOR — Explicit archived/MISSING recovery bypasses stored repository-identity validation

### Evidence

`src-tauri/src/projects/registry.rs::register_project()` detects current Git metadata and, when a matching normalized path exists, immediately updates the existing project row to `ACTIVE` and clears archive state.

That recovery path does not compare the newly detected Git repository identity against the existing stored `repositories` row before reactivation. The stricter identity checks implemented by `repair_project_path()` are therefore bypassed.

If the contents at the same normalized filesystem path have been replaced by an unrelated Git repository while the Registry row is archived/MISSING, explicit Add can reactivate the old project identity while retaining old repository metadata.

This violates the R02 requirement that unrelated repository/path collisions fail closed.

### Required closure

Factor repository-identity validation into a reusable helper and apply it to explicit duplicate recovery. Preserve the same ID only when identity is compatible. Reject unrelated Git identity, remote disappearance/appearance contradictions, and ambiguous legacy cases according to the same durable rules as repair.

Add archived and MISSING recovery identity-matrix tests, including same-path unrelated repo replacement.

---

## F-M19-V03R02-STRICT-006 — MAJOR — Owner-facing restored-project messaging cannot identify the archived case in the normal default view

### Evidence

`src/pages.tsx` determines whether a registration was an archived restoration using:

`const existing = records.find((item) => item.id === registered.id);`

The default Projects query excludes archived rows. Therefore the exact owner-observed archived ninth-project recovery case normally has no matching `existing` record in frontend state. The success message falls through to a generic newly registered GitHub/local message instead of the intended restoration message.

### Required closure

Return an explicit registration disposition from native registration, or otherwise query status before mutation, with bounded values such as `CREATED`, `ALREADY_ACTIVE`, `RESTORED_ARCHIVED`, `RESTORED_MISSING`. Render truthful owner-facing copy from that authoritative disposition.

---

## F-M19-V03R02-STRICT-007 — MAJOR — Semantic attention dedupe remains category/title based rather than factual-identity based

### Evidence

`src/command_center_view.tsx::semanticKey()` builds its dedupe key from project ID, task ID, normalized category/state family, and normalized title.

It does not incorporate normalized blocker/wait/evidence identity. The same underlying issue can remain duplicated when legacy and M19 titles differ, while two genuinely distinct issues with the same project/task/category/title can be incorrectly collapsed.

### Required closure

Build dedupe identity from the factual issue: project, canonical task, normalized blocker/wait/failure identity, and stable source/evidence ID where available. Preserve distinct issues while collapsing the same issue across legacy/M19 categories.

Add mounted UI fixtures for both duplicate-collapse and distinct-issue preservation.

---

## F-M19-V03R02-STRICT-008 — MAJOR — Required frontend geometry and owner-facing mounted tests were not added

### Evidence

The implementation adds `src/registry-card.css` and changes `ProjectRegistryCard.tsx`, but the implementation commit contains no new/modified frontend test file for the mandatory card geometry scenarios.

The R02 authority explicitly required deterministic mounted/geometry assertions around owner-like ~1536 px width, narrower breakpoints, delete-icon containment, action overlap, horizontal overflow, and visible copy.

The builder log reports the pre-existing/full Vitest total, but does not provide geometry assertions or a native-webview/Playwright equivalent for the new layout acceptance gates.

### Required closure

Add real mounted browser/native-webview geometry tests at 3/2/1-column widths. Assert every action bounding box is inside its card, archive/remove do not overlap, document has no horizontal overflow, focus outline remains usable, and every non-archived workspace button visibly reads `Local workspace`.

---

## F-M19-V03R02-STRICT-009 — MAJOR — The mandatory V03/V03-R02 verification matrix remains materially incomplete

### Evidence

`next_best_task.rs` has only a small set of direct tests. The dependency test injects the missing blocker manually. Direct coverage required by the authoritative prompts is still absent for many production semantics, including:

- local and remote unfinished dependency derived from task completion state;
- remote failure-evidence query failure;
- stale/fresh remote recovery matrix with identity/branch/hash cases;
- provider available/unavailable matrix for Codex and Claude through actual readiness application;
- explicit product-lifecycle history recording;
- comparison change/no-change through mounted/native refresh;
- semantic attention dedupe;
- frontend card geometry;
- concurrent GitHub request coalescing;
- 8/10/20-project request-budget bounds;
- 403/429 recovery-to-CURRENT and warning clearing.

### Required closure

Implement the complete required matrix as direct production-path tests. Passing an unrelated full suite does not replace missing acceptance-specific coverage.

---

## F-M19-V03R02-STRICT-010 — MAJOR — The published GitHub acquisition matrix is not the evidence matrix required by the prompt

### Evidence

The builder log contains a four-row repository summary with eight primary-resource counts and prose statements.

It does not record, per repository/resource stage, the required HTTP status/classification, cache hit/miss/current/stale state, rate-limit limit/remaining/reset or explicit transport-unavailable marker, snapshot request count/budget, bounded diagnostic, and final resource health.

It also omits the required before/after request-count table and worst-case hourly consumption math for 8, 9, 10, and 20 projects.

### Required closure

Publish the exact immutable, redacted matrix and request-count/budget tables required by R02. Test-count prose is not evidence.

---

## F-M19-V03R02-STRICT-011 — MAJOR — Required publication/native owner-window smoke evidence is missing from the immutable log

### Evidence

R02 required the published EXE to prove:

- restored ninth project visible immediately and after restart;
- footer delete icon contained;
- visible `Local workspace` copy;
- no repeated 403 warning wall;
- grouped stale warning on rate limit;
- stale warning clears after recovery.

The V03 R02 log contains no published EXE path/SHA-256, no accepted publication-command result, no owner-window/native smoke steps, and no result table for these scenarios. Its `Publication guard` only records tracker/M20 boundaries.

### Required closure

Run the accepted publication path and record stable EXE SHA-256 plus a bounded native QA evidence table for every R02 owner-observed scenario. Builder smoke remains builder evidence, not owner acceptance.

---

# Gate assessment

| Gate | Result |
|---|---|
| Pure exact-root local parser | SOURCE PASS |
| M19 observational read separation | SOURCE PASS, lifecycle integration incomplete |
| Unfinished dependency eligibility | FAIL |
| Remote failure evidence fail-closed | FAIL |
| Remote freshness/identity admission | SOURCE PARTIAL PASS |
| Exact dependency unlock | PARTIAL, depends on broken eligibility graph |
| Single Engineering Brief surface | SOURCE PASS |
| Semantic attention dedupe | FAIL |
| Archived ninth-project recovery | PARTIAL PASS, identity-validation and disposition gaps remain |
| Project-card containment/copy | SOURCE POSITIVE, REQUIRED GEOMETRY EVIDENCE MISSING |
| GitHub warning grouping/circuit | PARTIAL PASS |
| Portfolio-safe API consumption/coalescing | FAIL |
| GitHub evidence matrix | FAIL |
| Full required test matrix | FAIL |
| Publication/native R02 smoke evidence | FAIL |
| Owner-native acceptance readiness | NOT READY |

## Final re-audit verdict

**CHANGES_REQUIRED**

Do not request owner-native M19 acceptance yet. Close `F-M19-V03R02-STRICT-001` through `011`, rerun complete regressions/publication, publish a new immutable remediation log, and return for another independent strict source re-audit.
