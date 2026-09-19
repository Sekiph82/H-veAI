# M19 Next Best Task AI + Engineering Brief V07 R02 — Independent Strict Re-audit

Date: 2026-09-19

## Verdict

**CHANGES_REQUIRED**

The V07 R02 lifecycle remediation materially closes the five findings it was asked to address, but independent source review found two production correctness defects in the root-TASKS tracking path plus one verification gap. M19 must remain open and owner-native acceptance must not start yet.

Reviewed implementation:

- Start SHA: `543ed52a17a8bfee1e3ea7a438e8620b54d95a51`
- Implementation SHA: `34294efba46853608f35307e592dd94713961f8b`
- Publication SHA: `c61a7e94420e0394542299997c1eaa372ec16597`

## R02 requested findings

The intended R02 fixes are source-positive:

- changed-HEAD success now separates content materialization time from successful validation time;
- a refresh generation is attached to the qualifying observation job rather than settled merely by project id;
- a pre-existing in-flight observation carries no later manual generation in its `refreshes` vector;
- admission wait and execution wait are separate bounded phases;
- timed-out/cancelled generations are removed so late results cannot authorize M19 history;
- M19 now uses one portfolio hard freshness horizon, while the 300-second selected interval is scheduler targeting only.

Those changes are accepted as implemented.

## New strict findings

### F-M19-V07R02-STRICT-006 — CRITICAL — root-tracking fairness rotation is neutralized before admission

In production `polling_loop`, projects are first sorted by id, rotated with `rotation_cursor`, and then immediately sorted again with:

```rust
projects.sort_by_key(|project| {
    (
        !lifecycle.has_pending_refresh(&project.id),
        project.id.clone(),
    )
});
```

The second sort restores alphabetical id order inside the pending/non-pending groups, erasing the rotation that was supposed to make repeated portfolio service fair.

This is correctness-relevant under the 40-stage/hour tracking governor. In a 20-project changed-HEAD window the scheduled demand is 60 stages while only 40 can be admitted. Projects that are consistently earlier in the effective order can repeatedly consume the window before later projects reach the governor. Because next-due times are then derived from completion/failure time, the same subset can remain structurally advantaged in later windows. That can keep later remote TASKS snapshots stale/unavailable and therefore distort M19's portfolio-wide candidate set.

Required repair:

- preserve the rotated order among projects while moving manual/pending-refresh projects ahead;
- use a stable partition or stable boolean-only priority step rather than re-sorting by project id;
- prove repeated 20-project constrained windows rotate service across projects and cannot permanently starve a later project.

### F-M19-V07R02-STRICT-007 — CRITICAL — changed-HEAD TASKS content is not cryptographically bound to the observed HEAD

The tracking path first resolves branch HEAD `H1` with `fetch_github_head()`. For a changed HEAD it then calls `fetch_github_root_tasks(repository, branch, H1)`.

Inside that function, root `TASKS.md` is fetched by **branch name**:

```
https://raw.githubusercontent.com/{repository}/{branch}/TASKS.md
```

The returned snapshot nevertheless stores `remote_head = H1`.

If the branch advances to `H2` after the first HEAD observation but before the raw branch request, the cached task content can come from H2 while provenance still says H1. The later Atom request does not verify equality with H1. That breaks the exact remote authority contract and can make M19 recommend from content that is not the content of the recorded remote HEAD.

Required repair:

- fetch authoritative root TASKS by the exact observed commit SHA, not a moving branch ref, **or** use an equivalent revalidation protocol that fails closed/retries when the branch changes during the observation;
- ensure the persisted `tasks_blob_sha`, task rows, and `remote_head` all refer to the same immutable revision;
- add a deterministic test where the branch advances between HEAD observation and TASKS retrieval and prove mixed-revision state cannot be persisted as CURRENT.

### F-M19-V07R02-STRICT-008 — MAJOR — verification does not exercise either failure mode above

The R02 lifecycle matrix is useful for generation/observation binding, but it does not verify production project-order fairness under governor pressure and it does not verify HEAD-to-content revision atomicity. The synthetic lifecycle epochs also should not be presented as wall-clock duration proof; duration acceptance must come from the actual timeout constants/seams.

Required verification:

- repeated constrained 20-project tracking windows with a fake governor/clock, proving eventual per-project service;
- pending/manual priority while preserving background rotation;
- moving-branch race fixture proving HEAD/content consistency;
- regression of all accepted R02 lifecycle tests and M19 freshness/history tests.

## Governance

- `TASKS.md` and `CODEX_ROADMAP.md` were not changed by the implementation diff.
- M20 was not started.
- Owner-native acceptance remains blocked until the narrow findings above pass independent source re-audit.
