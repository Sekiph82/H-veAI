# M19 Next Best Task AI + Engineering Brief V03 — Authoritative Remediation Prompt

## 0. Authority and execution boundary

You are remediating **M19 — Next Best Task AI + Engineering Brief** after the independent V02 strict re-audit returned **CHANGES_REQUIRED**.

Authoritative audit:

`docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_STRICT_REAUDIT.md`

Before touching code:

1. Safely synchronize the standalone H!veAI checkout with current GitHub `main` using fetch + fast-forward only when safe.
2. Do not reset, rebase, force-push, auto-stash, clean, discard, or overwrite unrelated owner work.
3. Read in full:
   - `AGENTS.md`
   - `CONSTITUTION.md`
   - `ARCHITECTURE.md`
   - root `TASKS.md`
   - `CODEX_ROADMAP.md`
   - `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_PROMPT.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_REMEDIATION_PROMPT.md`
   - `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_STRICT_AUDIT.md`
   - `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_STRICT_REAUDIT.md`
   - `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_LOG.md`
4. Record exact starting `origin/main` SHA in the V03 builder log.
5. Run the relevant baseline tests before remediation, including the currently failing M16 observational-read test, and record baseline behavior before changing source.

`TASKS.md` and `CODEX_ROADMAP.md` are **read-only for Codex**. ChatGPT owns canonical tracker transitions. Do not mark M19 PASS/CLOSED. Do not activate or start M20.

This V03 prompt is a bounded remediation only for the residual V02 findings. Preserve valid V01/V02 fixes.

---

# 1. Non-negotiable outcome

M19 may recommend a task only when canonical current task truth proves all of the following:

- project is ACTIVE;
- task is not completed;
- every dependency is uniquely resolved and complete;
- no blocker/gate/Human/external wait prevents execution;
- required actor is a supported executable builder and currently available;
- local or remote TASKS evidence is fresh enough for the decision;
- required failure/CI/audit evidence is either truthfully available or truthfully marked unavailable;
- the task survived portfolio-wide deterministic scoring without project starvation.

The Command Center must expose **one integrated M19 Engineering Brief surface**. Recommendation reads must not mutate canonical task/control-plane/project-file state.

All V02 strict re-audit findings `F-M19-V02-STRICT-001` through `009` must be closed with production-path tests and immutable evidence.

---

# 2. Preserve already-accepted work

Do not regress:

- historical eight seeds acting as bootstrap defaults rather than a portfolio allow-list;
- established seed settings/repository identity preservation;
- ninth/tenth project persistence across refresh/restart;
- explicit archive/remove/exclusion behavior;
- Add Project exactly-once submit guard and visible bounded errors;
- GitHub-linked vs local-only registration success messaging;
- Bulk-Edit / Pixel Art Generator primary-resource shape fix;
- exactly eight primary GitHub cache resources;
- optional enrichment local/PARTIAL degradation;
- M18 GitHub panel-local error boundary and no whole-app black surface;
- rank-aware explanation wording;
- score-after-global-candidate-build behavior;
- deterministic non-alphabetic tie-breaking;
- Human/External/CI/GPT Audit/unknown actor fail-closed semantics;
- secret/token/cookie/credential redaction;
- read-only/default-denied GitHub mutation policy;
- M20 stop boundary.

---

# 3. F-M19-V02-STRICT-001 — unfinished dependencies must block eligibility

## Required canonical dependency graph

Build one task graph for each project before deriving eligibility.

For every canonical task, retain:

- stable task identity;
- explicit task ID where available;
- status/completion state;
- dependencies;
- blockers/gates;
- actor;
- Human/external wait;
- evidence/freshness.

Normalize dependency references consistently, including case-equivalent explicit IDs where the accepted parser semantics consider them equivalent.

## Eligibility rule

A task is executable only if every declared dependency resolves to exactly one canonical task and that dependency is complete.

- missing dependency => fail closed;
- ambiguous duplicate explicit ID => fail closed;
- dependency exists but is unfinished => dependent is blocked/ineligible;
- dependency completed => dependency satisfied;
- duplicate references to the same dependency count once;
- no separate `Blocker:` line is required for an unfinished dependency to block the task.

Blocked dependents remain in attention with evidence identifying the unmet dependency.

## Direct tests

Add both local exact-root and remote root-TASKS tests for:

1. `TASK-B` depends on unfinished `TASK-A`, no blocker line -> B is not eligible;
2. same graph after A is complete -> B can become eligible if all other gates pass;
3. dependency ID case variation resolves consistently;
4. duplicate explicit IDs make dependency resolution ambiguous and fail closed;
5. missing dependency fails closed;
6. duplicate dependency edge is deduplicated.

---

# 4. F-M19-V02-STRICT-002 — remove the mutating all-source M09 parse from M19 decision reads

## Problem to remove

M19 must not call the full `task_intelligence::parse()` merely to establish current recommendation truth. That function discovers auxiliary sources, shares a global parser budget across them, persists rows/settings, and may invoke control-plane materialization.

## Required architecture

Create or reuse a **pure exact-root TASKS parser path** for M19.

Acceptable architecture:

- factor reusable text-parser primitives from M09 into a pure helper; then
- M19 reads only `<registered-root>/TASKS.md`, hashes it, parses only that text as source kind TASKS, and validates a second hash/read before accepting CURRENT evidence.

Requirements:

- no M08 auxiliary source discovery for M19 decisions;
- no ROADMAP/HANDOFF/CUSTOM/PLAN/PROGRESS parsing in this path;
- no task-intelligence SQLite persistence;
- no workflow/control-plane materialization;
- no project-file write;
- no hidden `.hiveai` current-state authority;
- preserve existing M09 behavior for its own callers unless required to expose a pure helper.

The parser must remain bounded and deterministic, including source size, task count, scalar bounds, invalid UTF-8, malformed structured metadata, dependency ambiguity, and changed-during-read behavior.

## Read-purity tests

Create a real temp project and database, then repeatedly call the production M19 snapshot / Command Center snapshot path. Prove that observational reads do not change:

- repository/project files;
- `.hiveai` STATE/HANDOFF/EVENTS/EVENT_INDEX files;
- task rows/task_sources merely because M19 was viewed;
- task_events;
- workflow state;
- control-plane generation/materialization state;
- unrelated settings.

If the M19 comparison fingerprint must be persisted, separate its write from the observational snapshot function. Use an explicit/bounded M19 history-recording operation or refresh lifecycle that stores only M19-owned non-authoritative cache state. The read function itself must be pure.

Also add an adversarial fixture with enough auxiliary task-source rows to hit the old shared M09 budget and prove root `TASKS.md` remains unaffected because M19 never enters that all-source path.

---

# 5. F-M19-V02-STRICT-003 — enforce recommendation-time freshness for remote TASKS

## Required freshness contract

A cached GitHub root-TASKS snapshot is not executable merely because serialized `remote_health == CURRENT`.

Define an explicit bounded freshness horizon for M19 decisions. Use one of these approaches:

- validate remote HEAD before recommending; or
- accept a scheduler-produced snapshot only when its `fetched_at` / persisted sync timestamp is within a documented short freshness window and all identity/hash invariants match; otherwise mark stale and request/await refresh.

Requirements:

- Registry owner/repo/branch must match snapshot repository/branch;
- `tasks_blob_sha` and every candidate row hash must agree;
- stale/old CURRENT cache must become STALE/UNAVAILABLE for recommendation purposes;
- stale evidence may remain visible as last-known evidence but may not be labeled current or drive executable recommendation;
- offline/rate-limited state with last-good data must remain explicitly stale;
- startup before first successful scheduler refresh must fail closed.

## Tests

- CURRENT snapshot fetched within freshness horizon -> eligible if other gates pass;
- old persisted CURRENT snapshot -> not eligible;
- Registry branch changed but cache branch old -> not eligible;
- root hash mismatch -> not eligible;
- stale last-good cache -> visible but not executable;
- fresh scheduler/HEAD refresh recovers eligibility.

---

# 6. F-M19-V02-STRICT-004 — make dependency criticality semantically exact

`dependency_unlock` may score only work that is proven to remove the last dependency blocker for another unfinished task.

## Required scoring semantics

For each dependent task:

- calculate all unmet dependencies;
- calculate independent blockers/gates/waits/actor constraints separately;
- award a prerequisite full unlock credit only if completing that prerequisite would leave zero unmet dependencies **and** no independent blocker/gate/wait would still prevent the dependent from becoming otherwise eligible;
- if you want partial critical-path credit for one of several unfinished prerequisites, define a separate component with an explicit smaller weight and test it. Do not label partial criticality as an actual unlock.

Normalize dependency IDs consistently and dedupe duplicate references.

## Exact tests

- one unmet prerequisite, no other blocker -> exact unlock points;
- two unmet prerequisites -> no false full-unlock credit for either, unless a separately named/tested partial component is intentionally added;
- unrelated blocker -> no full-unlock credit;
- Human/external wait -> no full-unlock credit;
- duplicate edge -> one edge;
- case-equivalent explicit ID -> stable same result;
- deterministic repeated scores.

---

# 7. F-M19-V02-STRICT-005 — use structured fresh failure evidence for local and remote candidates

## Shared failure lookup

Replace the integer-only failure lookup with a structured result such as:

- urgency points;
- source class (`AUDIT`, `TEST`, `CI`, `GITHUB_ACTION`, etc. where actually supported);
- stable evidence row/run ID when available;
- result/state;
- occurred/finished timestamp;
- age/freshness;
- project/task linkage;
- uncertainty/error if lookup failed.

Apply the same linked lookup to both local and GitHub-task candidates. A remote task must not silently receive zero merely because its TASKS authority is remote.

## Fail closed

- DB/query failure => uncertainty / attention or explicit unavailable input;
- malformed/missing failure timestamp => uncertainty;
- no linked row genuinely exists => proven absence may score zero;
- stale failure outside freshness window => no urgency bonus but may remain factual history;
- later passing/closed evidence supersedes older fail;
- only fresh unresolved linked failure earns urgency.

If current architecture cannot link GitHub Actions/CI to a canonical task safely, surface that as unavailable. Do not fabricate task linkage.

## Tests

At minimum:

- local fresh audit fail;
- remote fresh audit/test fail;
- stale fail;
- old fail followed by pass/closure;
- malformed timestamp;
- query failure;
- unrelated project/task failure has no effect;
- score component evidence exactly matches the structured evidence used for points.

---

# 8. F-M19-V02-STRICT-006 — one integrated Engineering Brief surface

## Native contract

Keep M19 embedded in the native Command Center snapshot. Do not restore detached second polling.

The integrated brief must contain:

- portfolio factual summary;
- current rank-1 recommendation;
- bounded alternatives and truthful rank/score difference;
- change-since-last comparison if comparable;
- M19 attention;
- Human/external waits;
- provider readiness materially affecting actionability;
- fresh linked audit/test/CI/GitHub issues where actually evidenced;
- unavailable/partial/stale evidence;
- evidence/provenance for important claims.

## Frontend

Render **one** Engineering Brief panel. Remove the current visual split between legacy `AI Engineering Brief` facts and a separate `M19 Engineering Brief` panel.

Legacy facts that are still useful may be merged into the one M19 brief under a factual section, but must not remain as a sibling brief panel.

## Semantic attention dedupe

Build normalized attention identity from factual content, for example:

- project ID;
- canonical task ID if available;
- normalized blocker/wait/evidence identity;
- source/evidence row when useful.

Do not dedupe only by exact category, because legacy and M19 categories may differ for the same underlying issue.

Preserve genuinely distinct issues.

## Mounted UI tests

Provide a real `hiveai_command_center_snapshot` mock/fixture containing M19 data and assert:

1. exactly one Engineering Brief region/panel;
2. rank-1 recommendation visible;
3. alternative rank and score difference visible;
4. M19 attention visible with evidence;
5. semantically duplicate legacy + M19 issue rendered once;
6. a distinct second issue remains visible;
7. provider state visible when material;
8. stale/unavailable evidence visible;
9. first-snapshot comparison unavailable is visible truthfully;
10. no M18 whole-app black-surface regression.

---

# 9. F-M19-V02-STRICT-007 — complete the V03 verification matrix and requalify the Rust regression honestly

The current seven `next_best_task` tests are not enough.

Add direct production-path coverage for every residual finding in this prompt, including:

## Dependency and authority

- unfinished dependency local + remote;
- completion recovery local + remote;
- ambiguous duplicate ID;
- missing dependency;
- exact-root parser purity;
- auxiliary-source budget cannot starve root TASKS;
- changed-during-decision root file;
- malformed priority/metadata fail closed.

## Freshness

- stale serialized CURRENT remote cache;
- branch mismatch;
- hash mismatch;
- stale last-good;
- fresh recovery.

## Scoring

- exact full-unlock conditions;
- no false unlock under unrelated blocker/wait;
- project priority;
- task priority;
- context-switch cost;
- structured failure urgency local + remote;
- stale/resolved failure;
- deterministic tie behavior;
- >128 portfolio fairness remains protected.

## Actor readiness

Exercise actual readiness application for:

- Codex available/unavailable;
- Claude available/unavailable;
- Human;
- CI;
- GPT Audit;
- External;
- unknown.

## Engineering Brief / comparison

- first snapshot unavailable;
- later comparable snapshot no-change/change;
- incompatible fingerprint schema -> comparison unavailable, not compared;
- persistence failure is bounded/unavailable;
- integrated mounted UI;
- semantic attention dedupe.

## Registry/GitHub regressions

Preserve ninth/tenth, archive/remove, Bulk-Edit, Pixel Art Generator, H!veAI control, optional enrichment PARTIAL, 403/429, timeout, stale cache, malformed response, redaction, and M18 error boundary.

## Full-Rust regression rule

Before source changes, run:

`cargo test --manifest-path src-tauri\Cargo.toml --lib control_plane::tests::m16l_current_command_center_cockpit_and_control_reads_are_observational -- --exact --nocapture`

Record whether it passes/fails on the V03 starting baseline.

After remediation run it again plus the full Rust library suite with a timeout long enough for the known long tests. If it fails only after V03 or fails because a touched M19/Command Center path writes state, it is a V03 blocker. Do not label a touched-path failure as pre-existing without before/after proof.

---

# 10. F-M19-V02-STRICT-008 — publish the actual GitHub acquisition evidence matrix

The V03 immutable log must contain, or link to an immutable checked-in evidence artifact containing, a bounded redacted matrix for:

- `Sekiph82/H-veAI@main`;
- `Sekiph82/Bulk-Edit@main`;
- `Sekiph82/ScrubBots-Level-Factory@main`;
- one additional working control.

For every tested resource/fetch stage record when available:

- repository;
- branch;
- resource kind/stage;
- HTTP status/code or status class;
- failure classification;
- cache state: hit/miss/current/stale/last-good;
- rate-limit limit/remaining/reset or explicit unavailable if the transport does not expose it;
- snapshot request count/budget;
- bounded redacted diagnostic;
- final resource health.

If a transport does not expose a required header, state `UNAVAILABLE FROM CURRENT TRANSPORT`; do not invent it.

No Authorization value, token, cookie, credentials, browser/session secrets, or credential-store data may appear.

Test counts alone are not the evidence matrix.

---

# 11. F-M19-V02-STRICT-009 — small comparison/evidence truthfulness fixes

Close all three:

1. fingerprint schema mismatch/incompatible schema => `UNAVAILABLE_INCOMPATIBLE_SCHEMA` or equivalent, never normal comparison;
2. `UNAVAILABLE_FIRST_SNAPSHOT` / incompatible / persistence unavailable states must carry unavailable freshness in facts/UI;
3. normalize remote content-hash evidence so `sha256:` appears exactly once.

Add direct tests.

---

# 12. M19 comparison persistence without violating observational reads

The Engineering Brief needs history, but accepted Command Center reads must remain observational.

Implement a clear two-phase boundary:

- **Pure snapshot computation**: reads current canonical evidence and previous persisted M19 fingerprint, computes recommendation/brief/comparison, performs no writes.
- **Explicit M19 history recording**: persists the newly computed bounded fingerprint through a distinct mutation path tied to an intentional refresh/lifecycle action, not hidden inside the read function.

The stored fingerprint is M19-owned non-authoritative history only. It must never override current root TASKS truth.

At first snapshot with no prior record, comparison is unavailable. Recording that fingerprint makes the next explicitly refreshed snapshot comparable.

Add tests proving pure snapshot repetition is byte/DB observational and explicit history recording changes only the M19-owned history row.

---

# 13. Regression and publication gates

Run and record at minimum:

1. baseline exact M16 observational test before remediation;
2. `npm run typecheck`;
3. `npm run build`;
4. focused mounted frontend M19/Engineering Brief/Add Project/M18 containment tests;
5. full `npm test -- --reporter=dot --maxWorkers=1`;
6. also run normal/default frontend invocation and record resource-sensitive timeout differences truthfully;
7. `cargo check --manifest-path src-tauri\Cargo.toml`;
8. focused Rust suites for:
   - `next_best_task`;
   - exact-root pure parser helper;
   - `github_tracking`;
   - `github_integration`;
   - `command_center`;
   - M16 observational-read regression;
9. full `cargo test --manifest-path src-tauri\Cargo.toml --lib` with sufficient timeout;
10. formatting check on every changed Rust file;
11. full `cargo fmt --manifest-path src-tauri\Cargo.toml -- --check`, distinguishing pre-existing unrelated drift from introduced drift;
12. `git diff --check`;
13. security/redaction/request-budget tests;
14. accepted `scripts/publish-dev-qa.ps1` publication path;
15. release `--no-bundle` build / hidden smoke / readiness / no-visible-console / forbidden-port / stable swap / shortcut target+icon verification;
16. stable published EXE SHA-256.

Do not claim a full regression PASS if the full suite contains an unexplained failure on a touched execution path.

---

# 14. Required V03 immutable builder log

Create only after implementation/tests/publication are complete:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_LOG.md`

The log must include:

- exact starting synchronized SHA;
- ending implementation SHA;
- final log commit SHA in final handoff;
- changed files;
- disposition of every `F-M19-V02-STRICT-001` through `009`;
- before/after result of the exact M16 observational test;
- proof that pure M19/Command Center snapshot reads do not mutate project/control/task state;
- local dependency eligibility evidence;
- remote dependency eligibility evidence;
- remote freshness-age/branch/hash evidence;
- exact unlock-score fixtures;
- structured fresh-failure evidence local and remote;
- mounted one-panel Engineering Brief evidence;
- semantic attention-dedupe evidence;
- comparison schema/freshness/hash-format evidence;
- full per-repository/per-resource GitHub acquisition matrix required above;
- focused and full test results;
- any baseline/pre-existing failure with before/after causal evidence;
- publication result and EXE SHA-256;
- explicit statement that `TASKS.md` and `CODEX_ROADMAP.md` were unchanged;
- explicit statement that M20 was not started;
- explicit statement that owner-native acceptance is not self-claimed.

Builder log is a claim, not acceptance evidence.

---

# 15. Final repository and stop gate

After implementation:

1. verify `TASKS.md` and `CODEX_ROADMAP.md` are unmodified by Codex;
2. verify M20 source/scope was not started;
3. commit implementation;
4. create and commit immutable V03 log only after the implementation SHA is final;
5. push non-force to `origin/main`;
6. verify clean worktree;
7. verify local `HEAD == origin/main == live GitHub main`;
8. verify V03 log is reachable on GitHub;
9. stop.

Do not update canonical tracker state. Do not claim M19 PASS/CLOSED. Do not request owner-native acceptance. Wait for independent strict V03 re-audit.
