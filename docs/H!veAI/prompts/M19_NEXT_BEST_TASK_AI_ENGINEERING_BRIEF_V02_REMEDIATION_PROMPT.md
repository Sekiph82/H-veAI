# M19 Next Best Task AI + Engineering Brief V02 — Authoritative Remediation Prompt

## 0. Authority and execution boundary

You are remediating **M19 — Next Best Task AI + Engineering Brief** after the independent V01 strict audit returned **CHANGES_REQUIRED**.

Before touching code:

1. Safely synchronize the standalone H!veAI local checkout with current GitHub `main` using fetch + fast-forward only when safe.
2. Do not reset, rebase, force-push, auto-stash, clean, discard, or overwrite unrelated owner work.
3. Read in full:
   - `AGENTS.md`
   - `CONSTITUTION.md`
   - `ARCHITECTURE.md`
   - root `TASKS.md`
   - `CODEX_ROADMAP.md`
   - `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_PROMPT.md`
   - `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_LOG.md`
   - `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_STRICT_AUDIT.md`
4. Record the exact starting `origin/main` SHA in the V02 builder log.
5. Run the relevant baseline tests before remediation and record pre-existing failures separately.

`TASKS.md` and `CODEX_ROADMAP.md` are **read-only for Codex**. ChatGPT owns canonical tracker transitions. Do not mark M19 PASS/CLOSED. Do not activate or start M20.

This V02 prompt is a bounded remediation of M19 V01. Preserve valid V01 fixes unless this prompt explicitly requires a change.

---

# 1. Non-negotiable outcome

M19 must recommend work only from canonical current task truth, across the complete ACTIVE Registry portfolio, without silent project starvation, stale-as-current evidence, cross-source authority leakage, or fabricated dependency/actor state.

The Engineering Brief must become a real deterministic M19 product surface rather than a separate legacy brief next to a recommendation card.

All 12 V01 strict-audit findings must be closed with direct tests and immutable evidence.

---

# 2. Preserve already-good V01 work

Do not regress:

- non-seed projects surviving Registry/GitHub refresh and restart;
- Add Project exactly-once submit protection and visible bounded native errors;
- valid local non-Git registration;
- Bulk-Edit and Pixel Art Generator GitHub acquisition fix;
- exact eight primary GitHub cache resource contract;
- optional enrichment local degradation;
- M18 V07 panel-local GitHub error containment and no whole-app black surface;
- read-only/default-denied GitHub mutation policy;
- secret/token/cookie/credential redaction;
- native deterministic recommendation architecture;
- M20 stop boundary.

---

# 3. F-M19-V01-STRICT-001 — make historical seeds bootstrap-only in reality

`github_tracking::ensure_portfolio()` must stop overwriting established Registry-owned values for already-existing seed projects.

## Required behavior

For the historical eight seed identities:

- creating a genuinely missing bootstrap row is allowed;
- filling a genuinely absent migration field may be allowed when unambiguous;
- a non-null existing `task_source_policy` must not be overwritten merely because the project is a seed;
- established repository `remote_url`, `github_owner`, `github_repo`, branch/default branch, Git capability state, and other user-owned Registry identity/settings must not be rewritten from seed constants merely because `ensure_portfolio()` runs;
- an explicitly archived seed project must remain archived;
- an explicitly removed seed project must remain excluded until explicit registration/recovery;
- a user-registered non-seed project must remain untouched by seed reconciliation.

If an old migration requires normalization, make it explicit, one-way, testable, and evidence-based rather than a permanent every-refresh rewrite.

## Direct tests

Add tests proving:

1. existing seed custom setting survives `ensure_portfolio()`;
2. existing seed repository identity/branch metadata survives reconciliation;
3. missing seed bootstrap still works;
4. archived seed remains archived;
5. excluded/removed seed remains excluded;
6. ninth/tenth projects remain ACTIVE;
7. repeated reconciliation is idempotent.

---

# 4. F-M19-V01-STRICT-002 and 008 — canonical root `TASKS.md` only, with proven freshness

The accepted X04 authority model is mandatory.

## Local authority rule

For local/attached workspaces, **repository-root `TASKS.md` is the sole current task/workflow/status authority for M19**.

M09 may continue to parse PLAN/PROGRESS/ROADMAP/HANDOFF/CUSTOM/OTHER sources for historical or auxiliary intelligence elsewhere, but M19 candidate creation must not treat those sources as current task truth.

## Implementation requirement

Use one of these safe designs:

- consume an existing canonical root-TASKS truth API that already proves root identity and freshness; or
- build an M19-specific exact-root-TASKS adapter over the existing parser primitives; or
- filter a parsed snapshot to exact canonical root `TASKS.md` only **and** validate that the root file hash/current identity matches the parsed evidence at decision time.

Do not simply call `task_intelligence::list()` and label the persisted snapshot CURRENT.

## Freshness behavior

Before a local row may be labeled `CURRENT`, prove that its evidence corresponds to the current canonical root `TASKS.md` content/hash.

If the root file:

- is missing;
- changed after the cached parse;
- is malformed;
- cannot be read;
- cannot be mapped to canonical task truth;

then fail closed and surface `STALE`, `MALFORMED`, or `UNAVAILABLE` as appropriate. Do not recommend from stale rows.

## Direct tests

Cover:

- root TASKS + conflicting ROADMAP task;
- root TASKS + conflicting HANDOFF task;
- root TASKS + conflicting CUSTOM source;
- cached parse followed by root TASKS mutation;
- missing root TASKS;
- malformed root TASKS;
- valid fresh root TASKS;
- ninth non-seed project with fresh canonical root TASKS.

Only the root task truth may produce M19 candidates.

---

# 5. F-M19-V01-STRICT-003 — per-task remote truth must be complete or fail closed

`RemoteTaskRow` is currently too weak for M19 eligibility.

## Required remote task contract

For each tracked-branch root `TASKS.md` task that may become a candidate, provide per-task evidence for at least:

- ID;
- title;
- status;
- source path/line/evidence locator;
- required actor;
- dependencies;
- blockers/gates;
- owner gate;
- external wait;
- authoritative task priority if available.

Do not apply snapshot-level `required_actor` to every task. Do not apply current-task blockers only to the current task while assuming all other tasks are blocker-free.

If the existing GitHub tracker parser cannot provide these fields, extend it deterministically from tracked-branch root `TASKS.md`.

If a row lacks evidence required for safe eligibility, it must not be recommended. Route the uncertainty to unavailable/attention rather than inventing empty dependencies/blockers.

## Direct tests

Include:

- non-current remote task with unfinished dependency;
- non-current remote task with blocker;
- non-current remote Human wait;
- two remote tasks with different actors;
- malformed per-task metadata;
- stale remote root TASKS;
- valid current remote task set.

---

# 6. F-M19-V01-STRICT-004 and 005 — portfolio-fair candidate graph and real dependency unlock scoring

## Build the full canonical graph first

Construct the canonical task graph before eligibility deferral.

The graph must retain:

- eligible tasks;
- completed tasks;
- blocked tasks;
- dependency edges;
- human/external waits;
- per-task evidence/freshness.

Then derive candidate and attention sets from that graph.

Do not remove blocked dependents before computing which prerequisite tasks they depend on.

## Dependency unlock

An eligible prerequisite should receive an unlock component only when canonical evidence proves that completing it would unblock another unfinished task.

Do not count unrelated references. Do not double-count duplicated edges. Keep the blocked dependent in attention while using its edge for scoring.

## Candidate bound and fairness

The current `truncate(MAX_CANDIDATES)` before scoring is prohibited.

Implement a bounded strategy that cannot exclude a whole later project solely because earlier alphabetically ordered projects contributed more rows.

Acceptable design characteristics:

- bound per-project canonical extraction first, then score globally; or
- safely score all canonical rows under an explicit total parser bound and truncate only ranked output; or
- another deterministic strategy with direct starvation-proof evidence.

Equal-score ordering must be deterministic but must not create permanent project-name starvation. Do not use random ranking.

## Required tests

- >128 candidates across several projects;
- later-alphabet project contains highest-priority task;
- each eligible ACTIVE project remains representable before final output truncation;
- real prerequisite receives non-zero unlock points;
- dependent remains attention item;
- duplicate dependency edge is counted once;
- deterministic repeated ordering.

---

# 7. F-M19-V01-STRICT-009 — complete scoring and fail-closed semantics

Use an explicit, testable scoring model that implements the V01 contract.

At minimum account for, when authoritative/evidenced:

- project priority;
- task priority;
- dependency criticality / unblocks;
- fresh linked audit/CI/test failure urgency;
- blocker/gate eligibility;
- Human/external wait handling;
- required actor/provider readiness;
- bounded context-switch cost;
- explicit owner focus if an existing authoritative owner-focus setting exists;
- evidence uncertainty.

## Missing evidence rule

Missing or unreadable evidence must never silently become a favorable numeric zero.

Audit/test DB read errors, malformed timestamps, missing linkage, or unknown freshness must be surfaced in candidate uncertainty/unavailable state.

A genuine proven absence of a failure may score zero. An evidence-read failure may not.

## Actor model

Model at least:

- CODEX;
- CLAUDE;
- HUMAN;
- CI;
- GPT AUDIT / GPT_AUDIT according to accepted project vocabulary;
- EXTERNAL;
- UNKNOWN/unavailable.

Requirements:

- unavailable CODEX/CLAUDE must not be recommended for immediate execution;
- HUMAN/EXTERNAL work remains separate attention;
- CI/GPT Audit semantics must be explicit and not silently treated as available merely because they are not Codex/Claude;
- UNKNOWN actor fails closed from executable recommendation and is surfaced as uncertainty/attention.

## Failure freshness

Closed/historical failures must not indefinitely add urgency. A failure bonus requires project/task linkage and a bounded fresh unresolved signal.

Add direct tests where an older fail is superseded/closed by later evidence.

---

# 8. F-M19-V01-STRICT-006 and 007 — implement the real M19 Engineering Brief and surface M19 attention

The current legacy `CommandCenterSnapshot.engineering_brief` cannot remain disconnected from M19.

## Required Engineering Brief content

Provide a deterministic bounded brief containing:

- portfolio state summary;
- current recommended next task/action;
- a small bounded number of lower-ranked alternatives with truthful rank/difference when useful;
- M19 attention items;
- blocked/Human/external waits;
- provider/actor availability that materially affects actionability;
- fresh verified audit/CI/GitHub issues;
- unavailable/partial/stale evidence notes;
- what changed since the previous relevant M19 snapshot/visit, **only when comparable persisted evidence exists**.

## Change-since-last truth

Do not fabricate change history.

Persist or otherwise retain a bounded prior M19 factual snapshot/fingerprint with timestamp/provenance sufficient to compare:

- recommendation identity;
- attention set identity/state;
- relevant provider state;
- key portfolio counts;
- relevant fresh failure identities.

On first run or incomparable schema/state, say change comparison is unavailable.

Do not let historical hidden `.hiveai` files become current-state authority.

## UI integration

The Command Center must display the M19 Engineering Brief, not an unrelated legacy facts block plus a detached recommendation.

`m19.attention` must be visibly reachable with project/task/category/evidence. Merge/dedupe against legacy attention carefully so the same issue is not shown twice.

The M18 error-containment boundary must remain intact.

---

# 9. F-M19-V01-STRICT-010 — rank-aware explanation correctness

Change recommendation conversion so it receives actual rank/context.

Only rank 1 may say "ranked first".

Alternative explanations must state their actual rank or bounded score difference using the same deterministic score object.

Do not invent persuasive prose. Facts, score components, and explanation must remain consistent.

Add direct tests for rank 1, rank 2, and tie cases.

---

# 10. F-M19-V01-STRICT-012 — explicit registered-local-only success state

Preserve successful registration for non-Git and non-GitHub projects.

After Add Project succeeds, inspect the returned Repository record and show a bounded success result that distinguishes:

- registered + GitHub identity available;
- registered local project + GitHub identity unavailable/not applicable.

Do not convert missing GitHub identity into registration failure.

Add frontend tests for both success states.

---

# 11. GitHub acquisition evidence completion

Do not regress the V01 Bulk-Edit / Pixel Art Generator fix.

Repeat the production-native acquisition matrix for:

- `Sekiph82/H-veAI@main` control;
- `Sekiph82/Bulk-Edit@main`;
- `Sekiph82/ScrubBots-Level-Factory@main`;
- one additional working control.

Record redacted bounded evidence per repository/resource:

- repository + branch;
- resource kind;
- HTTP status/class where available;
- failure classification if any;
- rate-limit headers/budget where available;
- cache hit/miss/stale/last-good state;
- request count/budget consumed;
- bounded redacted error/diagnostic.

Never record tokens, Authorization values, cookies, credentials, credential-store content, or browser/session secrets.

The base repository success + optional enrichment failure path must remain PARTIAL/local rather than `github.repo_failed`.

---

# 12. Mandatory V02 test matrix

Do not stop at three `next_best_task` unit tests.

Add deterministic direct tests covering at minimum:

## Registry/portfolio

- seed settings/identity preservation;
- 9th and 10th project persistence;
- explicit archive/remove preservation;
- repeated reconciliation idempotence.

## Authority/freshness

- exact root TASKS only;
- conflicting ROADMAP/HANDOFF/CUSTOM ignored for current recommendation;
- missing root TASKS;
- malformed root TASKS;
- stale local cached parse;
- stale GitHub TASKS;
- current fresh local/remote TASKS.

## Remote per-task truth

- actor differences;
- dependencies;
- blocker/gate;
- Human/external wait;
- incomplete metadata fails closed.

## Candidate/scoring

- >128 multi-project fairness;
- later project not starved;
- exact dependency unlock points;
- project priority;
- task priority where authoritative;
- context-switch cost;
- owner focus if supported;
- unresolved fresh failure;
- resolved/closed historical failure;
- evidence read failure becomes uncertainty;
- deterministic tie behavior.

## Actor readiness

- Codex available/unavailable;
- Claude available/unavailable;
- Human;
- CI;
- GPT Audit;
- External;
- Unknown.

## Engineering Brief/UI

- first snapshot = comparison unavailable;
- comparable later snapshot = truthful bounded changes;
- recommendation visible;
- M19 attention visible;
- unavailable/partial evidence visible;
- no duplicate attention;
- rank 1 vs alternatives wording;
- registered GitHub-linked success;
- registered local-only success.

## M18/GitHub regression

- Bulk-Edit;
- Pixel Art Generator;
- H!veAI control;
- optional enrichment partial;
- 403/429;
- timeout;
- stale last-good cache;
- malformed response;
- secret redaction;
- no whole-app black-surface regression.

---

# 13. Regression and publication gates

Run and record:

1. `npm run typecheck`
2. `npm run build`
3. focused frontend tests for M19/Add Project/M18 containment
4. full `npm test -- --reporter=dot`
5. `cargo check --manifest-path src-tauri\Cargo.toml`
6. focused Rust suites for:
   - `github_tracking`
   - `github_integration`
   - `next_best_task`
   - any new Engineering Brief persistence/comparison module
7. full `cargo test --manifest-path src-tauri\Cargo.toml --lib`
8. formatting check on **every changed Rust file**; also run full cargo fmt check and distinguish genuine pre-existing unrelated drift from new drift
9. `git diff --check`
10. release/no-bundle publication helper used by the accepted QA flow
11. hidden smoke launch/readiness and desktop shortcut/binary verification

Do not terminate the full Rust regression merely because the known M16 observational test crosses 60 seconds. Use an appropriate bounded timeout that gives the established suite a fair chance to complete. If it truly hangs, isolate and record exact evidence rather than reporting the full regression as complete.

If the two frontend timeout tests remain, prove they are identical to the pre-remediation baseline and unrelated to M19. Do not label a new failure pre-existing without baseline evidence.

---

# 14. V02 immutable builder log

Create:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_LOG.md`

The log must contain:

- starting SHA;
- ending implementation SHA;
- exact changed files;
- closure evidence for every `F-M19-V01-STRICT-001` through `012`;
- exact root-TASKS authority proof;
- local freshness proof;
- remote per-task proof;
- >128 portfolio fairness proof;
- dependency-unlock exact score proof;
- actor/readiness matrix;
- Engineering Brief before/after evidence;
- GitHub acquisition matrix;
- focused and full test results;
- publication binary/hash/result;
- explicit statement that TASKS/ROADMAP were not modified;
- explicit statement that M20 was not started;
- explicit statement that owner-native final acceptance is not self-claimed.

Commit/push implementation and immutable log non-force to GitHub `main`.

At final handoff verify:

- clean worktree;
- local HEAD = `origin/main` = live GitHub `main`;
- V02 log exists on GitHub;
- no tracker mutation by Codex;
- no M20 work.

Then stop for independent strict re-audit.

---

# 15. Owner-native gate remains deferred

Do **not** ask the owner to run final M19 native acceptance until an independent V02 source re-audit returns PASS.

After source PASS, the owner-native gate must still include at minimum:

- adding a real ninth project and confirming persistence after restart/refresh;
- Bulk Edit GitHub panel;
- Pixel Art Generator GitHub panel;
- H!veAI GitHub control;
- Next Best Task across multiple projects;
- Human/External attention visibility;
- Engineering Brief truthfulness/change state;
- navigation through Command Center, Projects, Prompt Engine, Settings and back;
- no full black React/WebView surface.

M19 stays open until both source audit and required owner-native acceptance are complete.