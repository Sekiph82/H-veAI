# M19 Next Best Task AI + Engineering Brief V02 — Independent Strict Re-Audit

- Audit date: 2026-09-16
- Repository: `Sekiph82/H-veAI`
- Audited branch: `main`
- V02 implementation SHA: `91dee49724830c715ae17881c1d4db98e3e170f3`
- V02 builder-log commit: `8d4696e5e34aa9f925200624ca9e82a94f5783b1`
- V02 authoritative remediation prompt: `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_REMEDIATION_PROMPT.md`
- V02 builder log: `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_LOG.md`

## Verdict

**CHANGES_REQUIRED**

V02 closes several important V01 defects. Seed reconciliation is materially safer, non-seed projects remain registered, Add Project now distinguishes local-only and GitHub-linked success, rank wording is corrected, M19 attention is visible, the candidate universe is scored before output truncation, and a per-task remote metadata contract now exists.

However M19 still does not satisfy the full deterministic eligibility, freshness, evidence, Engineering Brief, and verification contract. The most serious remaining defect is that a task with a known but unfinished dependency can still become an executable recommendation. In addition, local M19 decisions still invoke the all-source mutating M09 parse path, remote CURRENT cache has no independent age/freshness bound at recommendation time, remote candidates silently receive zero failure urgency, dependency-unlock points can be awarded when completing the prerequisite would not actually unblock the dependent, and the Command Center still presents a legacy brief panel beside a separate M19 brief rather than one integrated Engineering Brief surface.

M19 remains OPEN. M20 remains BLOCKED. Owner-native final acceptance is not ready.

---

## Positive findings retained

1. `github_tracking::ensure_portfolio()` now uses `COALESCE`-style bootstrap filling instead of permanently rewriting established non-null seed settings/repository identity, and direct tests cover archive/remove and ninth/tenth persistence.
2. Local M19 now filters parsed task rows to exact repository-root `TASKS.md` with matching content hash and performs before/after root-file hashing.
3. `RemoteTaskRow` now contains per-task actor, dependency, blocker, gate/wait, priority, content hash, and metadata completeness fields.
4. Candidate truncation moved after global scoring, and deterministic tie-breaking no longer depends on project-name ordering.
5. Non-executable/unknown actors fail closed into attention; unavailable Codex/Claude provider readiness removes those tasks from executable recommendations.
6. Recommendation explanations are rank-aware.
7. Add Project visibly distinguishes GitHub-linked success from registered-local-only success.
8. M19 attention is rendered in the M19 UI surface.

These improvements are accepted as valid remediation work and must be preserved.

---

# Findings

## F-M19-V02-STRICT-001 — CRITICAL — Known unfinished dependencies do not make a task ineligible

In `next_best_task::collect_parsed_inputs()`, a dependency is added to `blockers` only when the dependency ID is not evidenced in canonical `TASKS.md`. If the dependency exists in the same canonical ledger, no completion-state check is performed.

The remote path has the same defect: `collect_remote_inputs()` checks whether a dependency ID exists but does not check whether that dependency task is complete.

Therefore a task may depend on another known unfinished task and still pass `defer_or_retain()` with an empty blocker list and become eligible for Codex/Claude execution.

### Required remediation

Build one canonical dependency graph containing status/completion for every task before eligibility. For every task:

- unresolved or ambiguous dependency => fail closed;
- known dependency that is not complete => task is blocked/ineligible and remains attention;
- completed dependency => satisfied;
- case-equivalent explicit IDs must resolve deterministically;
- duplicate explicit IDs must be treated as ambiguous, not last-write-wins.

Add direct local and remote tests where `TASK-B` depends on unfinished `TASK-A` without a separate `Blocker:` line and prove `TASK-B` cannot be recommended. Also prove it becomes eligible after `TASK-A` is complete.

---

## F-M19-V02-STRICT-002 — CRITICAL — Local M19 authority still depends on the all-source mutating M09 parse path

`collect_local_inputs()` reads root `TASKS.md`, but then invokes `task_intelligence::parse(database, project_id)` and filters its result afterward.

That M09 parse path discovers and parses TASKS, PLAN, PROGRESS, ROADMAP, HANDOFF, CUSTOM, and OTHER task sources under one project-wide `MAX_TASKS` budget, persists task/source/settings rows, and calls control-plane materialization best-effort.

Consequences:

1. Non-root auxiliary sources can consume the shared parser budget before M19 filters them out, so exact root `TASKS.md` truth can still be indirectly truncated or omitted by non-authoritative sources.
2. A Command Center / M19 decision read invokes a write-capable parser path and can mutate SQLite/task-intelligence/control-plane materialization state. This conflicts with the accepted observational-read boundary and makes the V02 full-Rust M16 observational failure especially relevant rather than safely dismissible.

### Required remediation

Create/reuse a pure exact-root `TASKS.md` parse adapter for M19 that:

- reads only the canonical root file;
- applies the parser logic/bounds needed by M19 without M08 discovery of auxiliary sources;
- performs no task-intelligence persistence, no workflow/control-plane materialization, and no project-file mutation;
- returns structured parse warnings/freshness/evidence directly to M19.

The M19/Command Center read path must not call the mutating all-source `task_intelligence::parse()` merely to establish current recommendation truth.

Add adversarial tests where hundreds/thousands of auxiliary source tasks exist and root `TASKS.md` remains fully authoritative and recommendable. Add a read-purity test proving repeated M19/Command Center snapshots do not alter project files, task/control-plane rows, task events, or unrelated settings.

---

## F-M19-V02-STRICT-003 — CRITICAL — Remote `CURRENT` cache has no recommendation-time freshness bound

`collect_remote_inputs()` accepts `github_tracking::cached_snapshot()` whenever `remote.remote_health == "CURRENT"`.

`cached_snapshot()` only deserializes the persisted cache. It does not verify age, `last_synced_at`, remote HEAD, or a recommendation-specific freshness horizon. `refresh_project()` also returns any cached snapshot immediately.

The background scheduler normally observes repositories every 10/30 seconds, but the recommendation engine cannot assume that the scheduler has already run successfully. A previously persisted CURRENT snapshot can remain CURRENT across startup, scheduler delay/failure, or direct M19 invocation and can be recommended without a bounded age check.

### Required remediation

Define and enforce a recommendation-time remote freshness contract. A GitHub task snapshot may be executable only when:

- its tracked-branch identity matches the Registry identity;
- its cache age is within a bounded accepted horizon, or a current remote-HEAD validation proves it is current;
- the root TASKS hash/head evidence is internally consistent.

Otherwise route it to STALE/UNAVAILABLE attention and do not recommend from it. Add tests using an old persisted `fetched_at`/sync timestamp that still says CURRENT and prove M19 refuses it.

---

## F-M19-V02-STRICT-004 — MAJOR — Dependency-unlock scoring can overstate work that would not actually unblock a dependent

V02 correctly builds dependency counts before eligibility deferral, but `dependent_counts` counts every unfinished dependent that references a prerequisite, even when that dependent also has an unrelated blocker, Human/external wait, unavailable actor, or another unfinished prerequisite.

That means a prerequisite can receive `dependency_unlock` points even though completing it would not make the dependent executable.

There is also inconsistent case handling between some dependency-existence checks and graph membership.

### Required remediation

Award unlock points only when canonical evidence proves completion of the prerequisite would remove the final dependency blocker for another unfinished task and no independent blocker/gate/wait still prevents eligibility. Deduplicate edges and normalize explicit IDs consistently. Add exact-score tests for:

- sole unfinished prerequisite => unlock points;
- two unfinished prerequisites => completing one does not yet count as fully unblocking unless the scoring definition explicitly models partial criticality separately;
- unrelated blocker/wait => no false unlock;
- case-equivalent dependency ID;
- duplicate edge counted once.

---

## F-M19-V02-STRICT-005 — MAJOR — Remote projects silently lose fresh audit/test urgency and failure provenance is too weak

Local candidates call `recent_failure_urgency()`. Remote/GitHub candidates hard-code `verified_failure_urgency: 0`.

Most historical portfolio projects are GitHub-task projects, so fresh linked audit/test failures can be ignored by the portfolio ranking solely because the task source is remote.

Additionally the score component exposes only generic text such as `Latest linked audit/test evidence within seven days`, not the actual evidence row/result/timestamp/freshness used to award urgency. CI/GitHub issue failure inputs requested by the M19 contract are not represented as structured score evidence.

### Required remediation

Use the same fail-closed linked failure lookup for eligible local and remote candidates when project/task linkage is valid. Return structured evidence, not just an integer, including source type, stable evidence ID where available, result/state, timestamp, age/freshness, and project/task linkage. Missing/query/malformed evidence must remain uncertainty, not zero.

If CI/GitHub failure linkage is unavailable, explicitly expose that input as unavailable rather than claiming a complete Engineering Brief.

Add tests for remote fresh failure, later passing/closed result, stale failure, malformed timestamp, DB query failure, and evidence text consistency with score points.

---

## F-M19-V02-STRICT-006 — MAJOR — Engineering Brief remains visually split and attention deduplication is category-exact rather than semantic

The native Command Center now embeds M19 into `engineering_brief`, which is progress. The frontend still renders two separate right-rail panels:

1. legacy `AI Engineering Brief` factual inputs;
2. separate `M19 Engineering Brief` recommendation/attention panel.

This is the exact presentation shape the V02 prompt asked to replace with one integrated M19 Engineering Brief.

The M19 attention dedupe key is only `projectId:taskId:category`. The legacy Command Center and M19 often describe the same underlying blocker/wait under different categories, so the same owner-visible issue can still appear twice.

The UI also does not clearly surface structured fresh audit/CI/GitHub issue facts with freshness/provenance as required by M19.06.

### Required remediation

Render one integrated Engineering Brief surface. Merge deterministic factual summary, recommendation, alternatives, change-since-last, provider state, M19 attention, fresh linked failures, and unavailable/partial evidence into that surface.

Deduplicate attention using a normalized semantic identity, not exact category equality alone. Preserve project/task/evidence details and allow genuinely distinct issues to remain separate.

Add mounted frontend tests that supply a real M19 Command Center fixture and prove:

- exactly one Engineering Brief surface;
- recommendation visible;
- alternative rank/difference visible;
- M19 attention visible;
- duplicate legacy/M19 representation collapses once;
- unavailable/stale evidence visible;
- first-snapshot comparison truth visible.

---

## F-M19-V02-STRICT-007 — MAJOR — Mandatory V02 verification matrix remains incomplete and the full-Rust failure is not safely requalified

The new `next_best_task` module contains seven focused tests, far below the explicit V02 matrix. Missing direct coverage includes, among others:

- unfinished known dependency blocks local and remote work;
- dependency becomes satisfied after completion;
- ambiguous duplicate IDs;
- stale CURRENT remote cache age;
- Codex available/unavailable and Claude available/unavailable through the actual readiness application path;
- remote fresh/resolved failure urgency;
- DB evidence-read failure;
- semantic attention dedupe;
- mounted integrated M19 Engineering Brief UI;
- comparison schema incompatibility;
- exact production-path ninth-project recommendation eligibility.

The builder log reports full Rust as `529 passed, 1 known pre-existing M16 observational failure`. V02 directly changed the Command Center snapshot path so it now invokes M19, and M19 invokes write-capable parsing/persistence. Therefore that observational failure cannot be dismissed merely by label without reproducing it on the V02 starting baseline and proving V02 did not introduce or worsen it.

### Required remediation

Expand the direct test matrix to the prompt contract. Re-run the full suite with sufficient timeout. If any failure remains, reproduce the same test on the V03 starting baseline and on the final source, explain the causal path, and do not classify it as unrelated when a touched path is involved.

---

## F-M19-V02-STRICT-008 — MAJOR — Immutable V02 log still does not contain the required per-repository/per-resource GitHub acquisition matrix

The V02 prompt requires redacted structured evidence per repository/resource for the H!veAI control, Bulk-Edit, Pixel Art Generator, and one additional control, including:

- resource kind;
- HTTP status/class where available;
- failure classification;
- rate-limit headers/budget where available;
- cache hit/miss/stale/last-good state;
- request count/budget;
- bounded redacted diagnostic.

The V02 log provides one general paragraph saying the focused tests exercise those identities and classes, but it does not record the required per-repository/per-resource matrix.

### Required remediation

In the V03 immutable log, include the actual bounded redacted matrix or a checked-in immutable evidence artifact referenced by the log. Do not expose tokens/cookies/credentials. Test counts alone are not a substitute for the requested production-path evidence.

---

## F-M19-V02-STRICT-009 — MINOR — Comparison and evidence presentation have small truthfulness defects

1. Persisted M19 fingerprint deserialization does not reject an incompatible `schema` value before comparison. A deserializable future/foreign schema could be treated as comparable.
2. `UNAVAILABLE_FIRST_SNAPSHOT` is not classified as unavailable in the fact freshness expression, so the first-snapshot comparison fact can be labeled `CURRENT` despite explicitly being unavailable.
3. Remote evidence formatting prefixes `sha256:` around a `content_hash` value that is already stored as `sha256:<hex>`, producing `sha256:sha256:<hex>` in evidence text.

### Required remediation

Fail comparison closed on schema mismatch, classify first-snapshot/incomparable comparison truthfully as unavailable, and normalize remote hash presentation once. Add direct tests.

---

# Gate assessment

| Gate | V02 re-audit result |
|---|---|
| Seed/bootstrap-only Registry preservation | PASS |
| Ninth/tenth project persistence | PASS |
| Add Project submit + success-state distinction | PASS |
| Bulk-Edit / Pixel Art resource-shape fix | SOURCE PASS, evidence matrix still incomplete |
| Exact-root local filtering/hash check | PARTIAL, all-source mutating parser dependency remains |
| Dependency eligibility | FAIL |
| Remote per-task metadata | PARTIAL PASS, dependency completion/freshness still fail |
| Portfolio fairness before truncation | PASS |
| Dependency criticality score | PARTIAL / FAIL |
| Actor fail-closed semantics | PASS at unit/source level |
| Fresh failure urgency | FAIL for remote portfolio and provenance |
| Rank-aware explanation | PASS |
| M19 Engineering Brief | PARTIAL / FAIL due split surface/incomplete issue evidence/dedupe |
| Change-since-last persistence | PARTIAL, schema/freshness defects remain |
| Mandatory V02 test/evidence matrix | FAIL / INCOMPLETE |
| Full regression | NOT CLEAN; one Rust failure remains and causal independence is not established |
| Owner-native acceptance eligibility | NOT READY |

## Final decision

**M19 V02 = CHANGES_REQUIRED.**

Do not mark M19 PASS/CLOSED. Do not activate M20. Execute the authoritative V03 remediation prompt, publish a new immutable V03 builder log, then perform another independent strict source re-audit. Owner-native acceptance should be requested only after that re-audit returns PASS.
