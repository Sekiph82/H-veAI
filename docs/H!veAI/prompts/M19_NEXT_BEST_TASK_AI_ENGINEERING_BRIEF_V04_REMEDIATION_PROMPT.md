# M19 Next Best Task AI + Engineering Brief V04 — Authoritative Remediation Prompt

## 0. Authority and execution boundary

You are remediating **M19 — Next Best Task AI + Engineering Brief** after the independent V03 R02 strict re-audit returned **CHANGES_REQUIRED**.

Authoritative audit:

`docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_STRICT_REAUDIT.md`

Before changing code:

1. Safely synchronize the standalone H!veAI workspace with current GitHub `main` using fetch + fast-forward only when safe.
2. Do not reset, rebase, force-push, auto-stash, clean, discard, or overwrite unrelated owner work.
3. Read in full:
   - `AGENTS.md`
   - `CONSTITUTION.md`
   - `ARCHITECTURE.md`
   - root `TASKS.md`
   - `CODEX_ROADMAP.md`
   - `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_REMEDIATION_PROMPT.md`
   - `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_REMEDIATION_PROMPT.md`
   - `docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_LOG.md`
   - `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_STRICT_REAUDIT.md`
4. Record exact synchronized `origin/main` SHA in the V04 builder log.
5. Preserve every V03 R02 source-positive fix unless this V04 explicitly strengthens it.

`TASKS.md` and `CODEX_ROADMAP.md` are **read-only for Codex**. ChatGPT owns tracker transitions. Do not mark M19 PASS/CLOSED. Do not start M20.

All findings `F-M19-V03R02-STRICT-001` through `011` must be closed with production-path tests and immutable evidence.

---

# 1. Preserve accepted V03 R02 work

Do not regress:

- pure exact-root `TASKS.md` parser for M19;
- read-only M19 snapshot computation;
- explicit history API separation;
- remote identity/branch/HEAD/root-hash/age validation;
- rank-aware recommendation explanations;
- global scoring before final candidate truncation;
- actor fail-closed semantics;
- one integrated M19 Engineering Brief surface;
- visible `Local workspace` project-card label;
- responsive footer containment CSS direction;
- archived same-path restoration preserving project ID;
- grouped GitHub rate-limit warning;
- stop-after-first-observed-rate-limit behavior within an acquisition cycle;
- Bulk-Edit and ScrubBots-Level-Factory repository-shape fixes;
- M18 panel-local error containment;
- secret redaction and mutation-denied GitHub policy;
- M20 stop boundary.

---

# 2. F-M19-V03R02-STRICT-001 — derive unfinished dependency blockers from canonical task state

This is the highest-priority correctness defect.

## Required behavior

For every project, build one canonical graph before candidate eligibility.

For every task:

- normalize explicit task IDs deterministically;
- retain completion state;
- resolve each dependency to exactly one canonical task;
- dedupe duplicate dependency references;
- missing dependency => ineligible + attention;
- ambiguous duplicate ID => ineligible + attention;
- uniquely resolved dependency that is not complete => ineligible + attention;
- only when every dependency is complete may dependency eligibility pass.

Do not require a separate `Blocker:` line to reflect an unfinished prerequisite.

Apply the same semantics to:

- local exact-root `TASKS.md` candidates;
- remote GitHub root-TASKS rows.

## Dependency-unlock scoring

Compute unlock credit from the same canonical graph. A prerequisite gets full unlock credit only when completing it would leave the dependent with zero unmet dependencies and no independent blocker/wait/non-executable actor gate.

## Direct tests

At minimum:

- local B depends on unfinished A, no blocker text -> B attention/ineligible;
- local A completed -> B eligible if other gates pass;
- remote equivalent unfinished/completed pair;
- missing dependency;
- ambiguous duplicate explicit ID;
- case-equivalent ID;
- duplicate dependency edge;
- two unmet prerequisites -> no false full unlock;
- independent blocker/wait -> no false full unlock.

Tests must derive blocker state from production graph logic. Do not manually inject `dependency unfinished` into fixtures.

---

# 3. F-M19-V03R02-STRICT-002 — one fail-closed structured failure-evidence path for local and remote tasks

Replace any remote `unwrap_or_default()` or equivalent favorable fallback.

Use one shared result contract containing, where available:

- source class;
- stable row/run ID;
- result/state;
- timestamp;
- age/freshness;
- project/task linkage;
- uncertainty/error.

Rules:

- query/database failure => explicit uncertainty/unavailable input;
- malformed or missing timestamp => uncertainty;
- no linked evidence genuinely exists => proven absence may score zero;
- stale fail => no urgency bonus;
- later pass/closure supersedes older fail;
- only fresh unresolved linked failure earns urgency;
- remote and local candidates use identical semantics.

Add direct local + remote tests for fresh fail, stale fail, fail then pass, malformed timestamp, query failure, unrelated evidence, and exact score evidence.

---

# 4. F-M19-V03R02-STRICT-003 — wire explicit M19 history recording into a real intentional product lifecycle

Keep `snapshot()` pure.

The explicit history mutation must be reachable through normal product behavior, not only a Rust unit test or unused native command.

Acceptable pattern:

1. Command Center computes/reads a pure snapshot.
2. On an intentional successful user/manual refresh or another clearly defined lifecycle event, the frontend/native lifecycle explicitly records that already-computed M19 fingerprint/history.
3. The next snapshot compares against that prior record.

Requirements:

- first snapshot => `UNAVAILABLE_FIRST_SNAPSHOT`;
- pure repeated reads alone do not write;
- explicit lifecycle record writes only M19-owned non-authoritative history;
- next comparable snapshot can report no-change/change;
- incompatible schema remains unavailable;
- persistence failure is visible/bounded and does not corrupt current recommendation truth.

Add mounted/native-path tests that invoke the actual product lifecycle, not just `record_history()` directly.

---

# 5. F-M19-V03R02-STRICT-004 — implement a proactive portfolio-safe GitHub request governor

The existing 403/429 circuit is reactive and must remain, but it is not sufficient.

## Required architecture

Implement one process-wide GitHub API request governor shared by production integration callers.

It must provide:

### A. Portfolio request budget

- explicit bounded request allowance over time;
- normal idle usage mathematically below intended quota;
- no project-count multiplier that deterministically exhausts unauthenticated limits;
- optional enrichment lower priority than primary repository state;
- selected-project refresh must not trigger full-portfolio API fan-out.

### B. Request coalescing

Concurrent requests for the same repository/branch/resource inside the freshness/in-flight window must share one network result.

Command Center, Projects/GitHub cockpit, M19-related reads, and other panel readers must consume shared cached observations where possible.

### C. Cache/freshness policy

Choose a realistic primary-resource cache freshness horizon consistent with the portfolio budget. The current 30-second API cache may be retained only if the published worst-case math proves it safe for the target portfolio size.

### D. Conditional requests

Where practical, use ETag/Last-Modified or another safe validator. Persist only non-secret validators. Treat 304 as cache reuse/current according to the documented freshness contract.

### E. Rate-limit metadata/backoff

When transport exposes headers, capture bounded rate-limit limit/remaining/reset. If the current curl transport cannot expose them, extend it safely or record explicit unavailability. Never print secrets.

403 rate-limit/429 must:

- open one process-wide circuit;
- stop primary/optional fan-out;
- retain last-known-good data as STALE;
- retry only after reset/backoff;
- recover automatically to CURRENT and clear obsolete warnings.

### F. Authentication viability

Inspect existing security architecture. If a secure existing GitHub auth/token mechanism exists, use it for reads. If not, do not invent plaintext credentials. Either implement an explicit owner-controlled OS-backed credential path or prove a quota-safe unauthenticated design.

## Mandatory request-budget evidence

Calculate and publish worst-case hourly primary + optional request consumption for:

- 8 projects;
- 9 projects;
- 10 projects;
- 20 projects.

Include idle, selected-project, and manual-refresh scenarios.

## Tests

- two concurrent same-resource readers => one network request;
- cache reuse inside freshness window;
- 8/9/10/20 project deterministic request-count budget;
- optional enrichment sheds first under pressure;
- first 403/429 stops remaining fan-out;
- stale last-good retained;
- recovery after backoff/reset => CURRENT;
- warning disappears after recovery;
- generic non-rate-limit 403 stays non-rate-limit;
- timeout/malformed/offline remain distinct;
- no secret leakage.

---

# 6. F-M19-V03R02-STRICT-005 — validate repository identity before archived/MISSING same-path recovery

Explicit Add Project recovery must not bypass the identity safeguards already used by path repair.

Factor one reusable repository-identity compatibility validator.

For an existing normalized-path row:

- ACTIVE + compatible identity => return same ID without destructive rewrite;
- ARCHIVED + compatible identity => restore same ID;
- MISSING + compatible identity => restore same ID;
- stored Git vs detected non-Git contradiction => fail closed;
- stored non-Git vs detected Git contradiction => fail closed unless the accepted legacy contract explicitly allows it and evidence is sufficient;
- different remote identity => fail closed;
- no-remote Git replacement must satisfy accepted HEAD/ancestry identity rules;
- ambiguous legacy repository identity => fail closed;
- only compatible restoration may clear the relevant exclusion.

Preserve settings/history/valid repository metadata. Do not merge unrelated repository content merely because the path string matches.

Add archived/MISSING/ACTIVE recovery identity tests including a same-path repo replacement fixture.

---

# 7. F-M19-V03R02-STRICT-006 — return an authoritative registration disposition

Do not infer restoration status from the frontend's currently visible records because archived rows are intentionally absent from the default list.

Extend the native registration result with a bounded disposition such as:

- `CREATED`;
- `ALREADY_ACTIVE`;
- `RESTORED_ARCHIVED`;
- `RESTORED_MISSING`.

Frontend success copy must use this disposition.

Required owner-visible semantics:

- archived recovery explicitly says restored/reactivated;
- active duplicate says already registered/active;
- missing recovery says restored/repaired;
- new local-only project says registered locally;
- new GitHub-linked project says registered with GitHub identity.

The same project must be visible immediately after successful restoration and remain after restart.

---

# 8. F-M19-V03R02-STRICT-007 — factual semantic attention dedupe

Replace the current category/title heuristic with an explicit normalized issue identity.

Use, as applicable:

- project ID;
- canonical task ID;
- normalized blocker/dependency/wait identity;
- structured failure evidence ID;
- stable source/evidence identity.

Do not collapse distinct blockers merely because title/category match. Do collapse the same factual issue emitted through legacy + M19 categories.

Add mounted fixtures for:

- same issue, different category/title wording -> once;
- two distinct blockers on same task -> both visible;
- same task plus separate provider issue -> both visible.

---

# 9. F-M19-V03R02-STRICT-008 — real project-card geometry tests

Preserve the `Local workspace` visible copy and responsive CSS, but prove it.

Add deterministic mounted geometry or Playwright/native-webview tests at representative widths including approximately 1536 px outer owner window and narrower 2/1-column breakpoints.

Assert:

- each card footer/action bounding box is inside its card;
- remove icon fully inside card;
- archive/remove do not overlap;
- Open cockpit clickable;
- Local workspace clickable;
- no horizontal document overflow;
- keyboard focus outline is not clipped;
- `Change local workspace` absent as visible card copy;
- `Local workspace` visible on every non-archived card.

A CSS/source assertion or screenshot alone is not sufficient.

---

# 10. F-M19-V03R02-STRICT-009 — complete the required production-path verification matrix

Add direct coverage for all remaining V03/V03-R02 requirements, including:

## M19 graph and freshness

- local + remote unfinished dependency;
- completion recovery;
- ambiguity/missing/case/duplicate edges;
- stale remote snapshot;
- branch mismatch;
- root hash mismatch;
- stale last-good;
- fresh recovery.

## Actors and scoring

- Codex available/unavailable;
- Claude available/unavailable;
- Human/CI/GPT Audit/External/unknown fail closed;
- project priority;
- task priority;
- exact unlock;
- context-switch cost;
- structured failure evidence local + remote;
- >128 fairness;
- deterministic ties.

## Engineering Brief/history

- one mounted Engineering Brief surface;
- semantic attention dedupe;
- first snapshot unavailable;
- explicit product-lifecycle record;
- later no-change/change;
- incompatible schema unavailable;
- persistence failure bounded.

## Registry/UI

- archived ninth same ID restored + visible;
- restart persistence;
- tenth project remains;
- recovery identity matrix;
- registration disposition copy;
- geometry tests.

## GitHub

- Bulk-Edit;
- ScrubBots-Level-Factory;
- H-veAI;
- one additional control;
- warning grouping;
- coalescing;
- budget;
- 403/429 circuit + recovery;
- stale/current transitions;
- redaction.

---

# 11. F-M19-V03R02-STRICT-010 — publish the actual GitHub evidence matrix

The V04 log must contain or link to an immutable checked-in redacted artifact with one row per repository/resource/fetch stage for:

- `Sekiph82/H-veAI@main`;
- `Sekiph82/Bulk-Edit@main`;
- `Sekiph82/ScrubBots-Level-Factory@main`;
- one additional working control.

Record when available:

- repository;
- branch;
- resource/stage;
- HTTP code/status class;
- failure classification;
- cache hit/miss/current/stale/last-good;
- fetched-at/age;
- rate-limit limit/remaining/reset or `UNAVAILABLE FROM CURRENT TRANSPORT`;
- actual request count and governor budget consumed;
- coalesced/reused boolean;
- bounded sanitized diagnostic;
- final resource health.

Also include before/after request-count tables and the exact 8/9/10/20-project hourly calculations.

---

# 12. F-M19-V03R02-STRICT-011 — publication and builder-native owner-window smoke evidence

Run the accepted publication pipeline and record:

- exact published EXE path;
- stable EXE SHA-256;
- publication command/result;
- shortcut target/icon verification where required by existing publication contract;
- hidden/native smoke result.

Then perform builder-native QA against the published build and record bounded evidence for:

1. archived ninth project + explicit Add -> same ID ACTIVE and visible;
2. app restart -> ninth still visible;
3. tenth project -> both visible;
4. project card footer controls contained at owner-like width;
5. every visible workspace button says `Local workspace`;
6. Bulk-Edit GitHub page loads without repeated-warning wall;
7. ScrubBots-Level-Factory GitHub page same;
8. forced rate limit -> one grouped warning + stale last-good;
9. recovery -> warning clears + CURRENT;
10. H-veAI + one control repo same behavior;
11. no M18 whole-app black-surface regression.

Builder QA is not owner acceptance. Do not self-claim owner acceptance.

---

# 13. Regression gates

Run and record at minimum:

1. `npm run typecheck`;
2. `npm run build`;
3. focused mounted M19/Engineering Brief/Projects/Add Project/GitHub tests;
4. full frontend tests single-worker;
5. normal/default frontend test invocation;
6. `cargo check --manifest-path src-tauri\Cargo.toml`;
7. focused Rust tests for next_best_task, task_intelligence exact-root helper, registry, github_tracking, github_integration, command_center;
8. exact M16 observational-read test;
9. full `cargo test --manifest-path src-tauri\Cargo.toml --lib --no-fail-fast` with sufficient timeout;
10. formatting check on changed Rust files;
11. full cargo fmt check with any unrelated baseline drift clearly distinguished;
12. `git diff --check`;
13. security/redaction/request-governor tests;
14. accepted publication pipeline;
15. stable EXE SHA-256;
16. builder-native R02 smoke matrix.

Do not claim full PASS when a touched-path failure remains unexplained.

---

# 14. Required immutable V04 builder log

Create only after implementation, tests, publication, and builder-native smoke are complete:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V04_LOG.md`

It must include:

- synchronized start SHA;
- final implementation SHA;
- final log commit SHA in handoff;
- changed files;
- explicit disposition of `F-M19-V03R02-STRICT-001` through `011`;
- direct unfinished-dependency evidence local + remote;
- failure-evidence fail-closed evidence local + remote;
- explicit product-lifecycle history evidence;
- archived/MISSING recovery identity evidence;
- registration disposition evidence;
- project-card geometry assertions;
- semantic attention dedupe evidence;
- GitHub request-governor/coalescing tests;
- 8/9/10/20-project hourly budget calculations;
- full per-resource GitHub evidence matrix;
- 403/429 stale + recovery evidence;
- frontend and Rust full test results;
- M16 exact test result;
- publication result and stable EXE SHA-256;
- builder-native smoke matrix;
- explicit statement `TASKS.md` and `CODEX_ROADMAP.md` unchanged;
- explicit statement M20 not started;
- explicit statement owner-native acceptance not self-claimed.

The builder log is a claim, not independent acceptance evidence.

---

# 15. Final repository stop gate

After all work:

1. verify `TASKS.md` and `CODEX_ROADMAP.md` unmodified;
2. verify M20 source/scope not started;
3. commit implementation;
4. create/commit immutable V04 log after implementation SHA is final;
5. push non-force to `origin/main`;
6. verify clean worktree;
7. verify local `HEAD == origin/main == live GitHub main`;
8. verify V04 log reachable on GitHub;
9. stop for independent strict re-audit.

Do not request owner-native acceptance until the independent V04 source audit returns PASS.
