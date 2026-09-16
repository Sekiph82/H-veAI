# M19 Next Best Task AI + Engineering Brief V01 — Independent Strict Audit

- Audit date: 2026-09-16
- Repository: `Sekiph82/H-veAI`
- Audited branch: `main`
- Builder implementation SHA: `93b1dfbf016a8d47b65a109cfa11cd9da1a613c5`
- Builder log SHA: `daea1927de4e7571049fb30ab0c6b3459be8bfd6`
- Authoritative builder prompt: `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_PROMPT.md`
- Builder log: `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_LOG.md`

## Verdict

**CHANGES_REQUIRED**

M19 V01 contains useful M19.00 fixes, especially removal of the non-seed auto-archive ceiling, Add Project submit observability, and the Bulk-Edit / Pixel Art Generator GitHub resource-shape fix. However the new recommendation engine does not yet satisfy the accepted X04/root-TASKS authority boundary, remote per-task evidence is insufficient for safe eligibility, the bounded candidate implementation can starve later projects before scoring, the dependency-unlock score is structurally ineffective, and the required M19 Engineering Brief has not been implemented as an M19 factual/recommendation product surface. The test matrix is also materially below the V01 prompt requirements.

M19 MUST remain open. M20 MUST remain blocked. No owner-native final acceptance should be requested until the remediation below receives an independent source re-audit PASS.

---

## Positive findings retained

1. `github_tracking::ensure_portfolio()` no longer archives every ACTIVE non-seed project merely because it is absent from the historical eight-project bootstrap list.
2. The Add Project dialog now has a single guarded form-submit path, a pending state, bounded error display, immediate visible success, and Registry refresh/selection behavior.
3. The native path validator now differentiates non-existent, non-directory, and inaccessible project paths.
4. The GitHub integration now keeps the frontend cache contract bounded to the eight primary resource kinds while optional enrichment affects health independently. HTTP 429 is explicitly classified as rate-limited.
5. The M19 engine is native/deterministic, exposes score components and evidence fields, and does not self-close M19 or start M20.

These positives do not override the findings below.

---

# Findings

## F-M19-V01-STRICT-001 — CRITICAL — Seed bootstrap still rewrites existing Registry settings and repository identity

The V01 prompt explicitly says the historical eight projects may be bootstrap/migration defaults only and that existing eight-project IDs/paths/settings must be preserved.

Current `github_tracking::ensure_portfolio()` still executes, for already-existing seed projects:

- `UPDATE projects SET ... task_source_policy=?3 ...` with `GITHUB_TASKS_ONLY_POLICY`, even when the Registry already has a non-null user-owned setting;
- `UPDATE repositories SET remote_url=?, github_owner=?, github_repo=?, default_branch=?, is_git_repository=1 ...`, overwriting existing repository identity fields with seed constants.

The code comment says seed metadata must not rewrite user-owned identity, but the following SQL still does so.

### Required remediation

Seed reconciliation may create a missing bootstrap row and may fill genuinely absent migration fields, but it must not overwrite a non-null Registry setting or established repository identity merely because the project matches a historical seed. Add direct tests that mutate an existing seed project setting/branch/remote metadata, run `ensure_portfolio()`, and prove the established Registry values remain unchanged unless an explicit user action changes them.

---

## F-M19-V01-STRICT-002 — CRITICAL — Local candidate authority is not root `TASKS.md` only

The accepted M19 prompt carries the X04 authority rule forward: repository-root `TASKS.md` is the sole local current project/task/workflow-status authority.

`next_best_task::collect_local_inputs()` calls `task_intelligence::list()` or `task_intelligence::parse()` and then consumes **every** task in the returned `TaskIntelligenceSnapshot`.

But M09 `task_intelligence::is_parser_source()` intentionally accepts multiple M08 source kinds, including `TASKS`, `PLAN`, `PROGRESS`, `ROADMAP`, `HANDOFF`, `CUSTOM`, and `OTHER_TASK_SOURCE`. M19 adds no exact-root-`TASKS.md` filter before turning those parsed rows into recommendation candidates.

Therefore a task parsed from a roadmap, handoff, plan, custom source, or other discovered task source can become a current M19 recommendation. This is a direct authority violation.

### Required remediation

Build M19 local candidates only from the canonical repository-root `TASKS.md` truth path accepted by X04. Non-root sources may supply historical/supporting evidence only and must never create or override current candidate/task/workflow state. Add direct mixed-source tests proving a conflicting ROADMAP/HANDOFF/CUSTOM task cannot enter or override the M19 candidate set.

---

## F-M19-V01-STRICT-003 — CRITICAL — Remote task rows do not carry enough per-task truth for safe eligibility

`github_tracking::RemoteTaskRow` currently contains only:

- `id`
- `title`
- `status`
- `source_path`
- `source_line`

In `collect_remote_inputs()`:

- every remote task receives `dependencies: Vec::new()`;
- only the task whose ID equals `remote.current_task_id` receives `remote.blockers`;
- every remote task receives the same snapshot-level `remote.required_actor`.

This means non-current remote tasks can be treated as dependency-free and blocker-free even when tracked-branch root `TASKS.md` says otherwise, and can inherit the current task's actor incorrectly.

### Required remediation

Extend the remote root-TASKS parsing/evidence contract to provide per-task actor, dependencies, blockers/gates, owner/external wait, and any eligibility metadata needed by M19. If complete per-task truth cannot be evidenced for a row, that row must fail closed rather than be recommended. Add tests where a non-current remote task is blocked/dependent/waiting and prove it cannot become eligible.

---

## F-M19-V01-STRICT-004 — CRITICAL — Candidate cap is applied before scoring and can starve later projects

`MAX_CANDIDATES` is 128. `snapshot()` receives Registry projects in the Registry's default name ordering, appends candidates project by project, then `build_candidate_set()` performs:

`set.eligible.truncate(MAX_CANDIDATES);`

This happens **before** `score_candidates()`.

Consequences:

- later alphabetically ordered projects may never reach the scoring stage;
- a high-priority task in a later project can be excluded solely because earlier projects contributed 128 tasks;
- the implementation violates the requirement to rank across all eligible ACTIVE Registry projects and to avoid starvation caused solely by stable alphabetical/project ordering.

The final score tie-break also falls back to lowercased project name before project/task IDs, so equal-scored work is still systematically alphabetically favored.

### Required remediation

Do not truncate the portfolio candidate universe before evidence/eligibility/scoring. Use a bounded but portfolio-fair strategy, for example bounded per-project extraction followed by global scoring, or score all safely bounded canonical rows and only truncate the output after ranking. Equal-score ordering must be deterministic without creating permanent project-name starvation. Add >128-candidate multi-project tests proving a high-priority later project remains eligible and rankable.

---

## F-M19-V01-STRICT-005 — MAJOR — Dependency-unlock scoring is structurally ineffective

`collect_parsed_inputs()` converts any unfinished/missing dependency into a blocker. `add_or_defer()` then removes that dependent task from `inputs` and moves it to attention.

Later, `build_candidate_set()` computes `candidate.unblocks` only by searching the remaining eligible `inputs` for tasks whose `dependencies` contain the candidate ID.

The tasks that are actually waiting on an unfinished dependency have already been removed, so the candidate that would unblock them generally receives an unlock count of zero.

### Required remediation

Build the dependency graph from the complete canonical task set before eligibility deferral. Score an eligible prerequisite by the canonical blocked/dependent tasks it would genuinely unblock. Preserve blocked dependents as attention items while retaining their graph edges for scoring. Add direct exact-score tests.

---

## F-M19-V01-STRICT-006 — CRITICAL — M19.06 Engineering Brief is not implemented/integrated

M19 V01 requires a deterministic Engineering Brief containing portfolio state, evidenced changes since the previous relevant snapshot/visit, attention/waits, verified audit/CI/GitHub issues with freshness, recommended next actions, provider availability, and explicit unavailable/partial notes.

The M19 implementation adds a separate `M19Snapshot`, but does not modify the native `command_center::EngineeringBrief` implementation. Current `command_center.rs` still returns:

`engineering_brief: EngineeringBrief { facts, recommendation: None }`

The Command Center UI continues to render the old `data.engineeringBrief.facts` surface, while M19 only adds a separate compact Next Best Task panel. There is no M19 change-since-last comparison, no M19 recommendation inside the Engineering Brief, and no provider/uncertainty integration into that brief.

### Required remediation

Implement M19.06 as a real deterministic product surface. Persist or otherwise maintain a truthful comparable prior M19 snapshot boundary so "what changed" is only emitted when comparison evidence exists. Integrate M19 recommendation, attention, provider readiness, freshness, and unavailable/partial facts into the Engineering Brief without fabricating history.

---

## F-M19-V01-STRICT-007 — MAJOR — M19 attention items are generated but not surfaced in the M19 UI

`M19Snapshot` contains `attention`, and the engine places blocked, Human/External wait, and unavailable-provider work there.

`NextBestTaskPanel` renders the recommendation and `unavailableInputs`, but does not render `m19.attention`. The right-side "Needs Your Attention" panel renders the older Command Center `data.attention` collection, not the new M19 attention set.

Therefore M19-specific Human/External/actor-unavailable items can be computed correctly yet remain invisible to the owner.

### Required remediation

Expose the bounded M19 attention set in the user-facing M19/Engineering Brief surface, with project/task/category/evidence and without duplicating identical Command Center items.

---

## F-M19-V01-STRICT-008 — MAJOR — Local task freshness is asserted as CURRENT without validating the source at recommendation time

`collect_local_inputs()` prefers `task_intelligence::list()` and only reparses on an error. `task_intelligence::list()` deserializes the persisted snapshot from SQLite; it does not validate that the current root `TASKS.md` hash still matches that snapshot before M19 consumes it.

`collect_parsed_inputs()` then hard-codes:

`evidence_freshness: "CURRENT"`

for those rows.

A previously parsed task snapshot can therefore be labeled CURRENT by M19 after the authoritative root file has changed but before another subsystem has refreshed M09.

### Required remediation

At M19 decision time, validate the canonical root `TASKS.md` identity/hash against the parsed snapshot or route through a canonical truth API that already guarantees freshness. If freshness cannot be proven, fail closed or mark stale/unavailable. Never hard-code CURRENT for an unvalidated cached snapshot.

---

## F-M19-V01-STRICT-009 — MAJOR — Scoring and fail-closed behavior are incomplete relative to the V01 contract

The score currently contains only:

- project priority;
- dependency unlock;
- verified failure;
- Codex/Claude readiness bonus;
- uncertainty penalty.

Missing or unsafe areas include:

1. no explicit task-priority component where authoritative task priority exists;
2. no bounded context-switch-cost component;
3. no owner-focus component if/when an authoritative owner-focus setting is present;
4. unknown required actor remains eligible with only an uncertainty penalty instead of failing closed or being routed to attention;
5. actor availability logic is explicit only for Codex/Claude; CI/GPT Audit/Human/External/unknown states are not modeled with equivalent decision semantics;
6. `recent_failure_urgency()` converts database/query/timestamp read failures into `0` through `ok()/flatten()` paths, which silently makes missing failure evidence favorable instead of surfacing uncertainty.

### Required remediation

Bring the scoring model to the full V01 contract. Missing evidence must be represented as uncertainty/unavailable, never silently as a favorable zero. Unknown required actor must not become an executable recommendation. Add exact component and fail-closed tests.

---

## F-M19-V01-STRICT-010 — MAJOR — Alternative recommendation explanations are factually wrong

`score_candidates()` produces an ordered list, then every scored candidate is passed through the same `to_recommendation()` function.

`to_recommendation()` always emits wording beginning with either:

- "Ranked first among ..."
- or "Ranked first from ..."

This is also applied to candidates returned in `alternatives`.

### Required remediation

Generate rank-aware explanations from the same score object. Only rank 1 may claim first place. Alternatives must explain their actual rank/difference without persuasive prose disconnected from score evidence.

---

## F-M19-V01-STRICT-011 — MAJOR — Required verification/evidence matrix is incomplete

The V01 prompt demanded direct candidate fixtures for multiple projects, ninth/non-seed inclusion, completed tasks, blockers, external waits, missing/malformed TASKS, stale GitHub evidence, exact scoring, actor readiness, and related truthfulness boundaries.

The new `next_best_task` module contains only three direct unit tests:

1. non-seed + completed exclusion;
2. blocker/Human wait attention;
3. deterministic score/tie behavior.

These do not cover the authority, remote per-task, freshness, >128 fairness, provider state, Engineering Brief, malformed/missing TASKS, or failure-evidence boundaries above.

Additionally, the builder log records:

- full Rust library regression as **incomplete** after a >60s M16 control-plane test;
- full frontend suite with two timeout failures described as pre-existing;
- full `cargo fmt --check` failure described as pre-existing.

The mandatory GitHub reproduction section in the V01 prompt also requested per-resource structured status, cache hit/miss/stale state, bounded diagnostic, and request-budget evidence for each control/failing repository. The log records repository-level HTTP 200/count data and one overall remaining rate budget, but not the full requested per-resource diagnostic matrix.

### Required remediation

Add the missing direct tests and run the complete required regression with a timeout appropriate for the known long-running suite rather than terminating the whole library run at 60 seconds. Record baseline-vs-remediation failures precisely. Record the required bounded GitHub acquisition matrix without secrets.

---

## F-M19-V01-STRICT-012 — MINOR — Add Project success does not explicitly distinguish unavailable GitHub identity

The V01 prompt requires a visible result state for "project successfully registered but GitHub identity unavailable."

Current success messaging states only that the project was registered and is visible. It does not explicitly surface when registration succeeded but GitHub identity could not be derived.

### Required remediation

After successful registration, use the returned Repository record to distinguish normal GitHub-linked success from registered-local-only / GitHub-identity-unavailable success. Registration must still succeed for non-Git and non-GitHub projects.

---

# Gate assessment

| Gate | Audit result |
|---|---|
| M19.00 ninth/non-seed preservation | PARTIAL PASS; non-seed auto-archive fixed, seed-setting overwrite remains |
| M19.00 Add Project usability | PARTIAL PASS; core submit/error path fixed, GitHub-unavailable success state missing |
| M19.00 Bulk-Edit / Pixel Art Generator GitHub shape defect | SOURCE PASS with evidence-gap follow-up |
| M19.01 candidate authority/eligibility | FAIL |
| M19.02 deterministic scoring | FAIL |
| M19.03 actor availability | FAIL |
| M19.04 explainability | PARTIAL / FAIL due rank-inaccurate alternatives and incomplete truth inputs |
| M19.05 portfolio recommendation | FAIL due pre-score truncation/starvation |
| M19.06 Engineering Brief | FAIL |
| M19.07 truthfulness/evidence boundary | FAIL |
| M19 regression/evidence gate | FAIL / INCOMPLETE |
| Owner-native acceptance eligibility | NOT READY |

## Final decision

**M19 V01 = CHANGES_REQUIRED.**

Do not update M19 to PASS/CLOSED. Do not activate M20. Execute the authoritative V02 remediation prompt, publish a new immutable V02 builder log, then perform a fresh independent source re-audit. Only after a source re-audit PASS should the owner-native M19 acceptance gate be run.