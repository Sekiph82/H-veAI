# M19 Next Best Task AI + Engineering Brief V01 - Authoritative Implementation Prompt

## 0. Authority and execution boundary

You are implementing **M19 - Next Best Task AI + Engineering Brief** for H!veAI.

This is the single authoritative whole-M19 builder prompt. It includes an owner-observed portfolio-integrity preflight that must be closed before the recommendation engine can be considered truthful.

Before changing code:

1. Read `AGENTS.md`, `CONSTITUTION.md`, `ARCHITECTURE.md`, root `TASKS.md`, `CODEX_ROADMAP.md`, and `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`.
2. Read the accepted M18 evidence, especially:
   - `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V07_STRICT_REAUDIT.md`
   - `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V07_OWNER_NATIVE_FINAL_ACCEPTANCE.md`
3. Synchronize the local standalone H!veAI checkout with current GitHub `main` safely. Do not reset, rebase, force-push, or discard unrelated local work.
4. Record the exact starting GitHub `main` SHA in the builder log. The prompt-publication baseline before this prompt was `035692e21e2e8dedbc40eeaca01cceefc6e537ae`, but the authoritative starting point for implementation is the actual current `main` after this prompt and canonical tracker updates are published.
5. Run the existing baseline test suite before implementation and record any pre-existing failures separately.

`TASKS.md` and `CODEX_ROADMAP.md` are **read-only for Codex**. Do not close M19 or activate M20. ChatGPT owns canonical tracker transitions after independent audit and any required owner-native acceptance.

M20 is out of scope.

---

# 1. Non-negotiable product truth

M19 recommends what should happen next across the portfolio. Therefore M19 is invalid if the portfolio itself is silently truncated, if newly registered projects disappear, or if project GitHub evidence fails for reasons H!veAI could diagnose or contain more precisely.

The owner has observed three concrete defects that are mandatory M19 preflight work:

1. **A ninth project cannot remain visibly registered.** The Add Project dialog opens and accepts a local folder path, but pressing Enter appears to do nothing and the ninth project does not remain in the system.
2. **Bulk Edit GitHub evidence fails** in Project Cockpit while other projects appear healthy.
3. **Pixel Art Generator GitHub evidence fails** in Project Cockpit while other projects appear healthy. The registered Pixel Art Generator repository is `Sekiph82/ScrubBots-Level-Factory@main`.

These are not reasons to reopen M18. M18 remains PASS/CLOSED because its final native gate concerned GitHub-panel containment and the whole-application black-screen regression. M19 must now make the portfolio and evidence inputs reliable enough for portfolio recommendation.

---

# 2. M19.00 - Owner-observed portfolio truth, Registry extensibility, and GitHub acquisition closure

## M19.00.01 - Remove the exact-eight production portfolio ceiling

### Confirmed source defect

Current production `github_tracking::ensure_portfolio()` is built around a fixed seed/target set of eight projects. It reconciles that target set and archives registered projects not present in the fixed set. Because `ensure_portfolio()` is invoked from normal tracking/snapshot refresh paths, an owner-registered ninth project can be successfully inserted and then later disappear from the active portfolio by being archived.

This exact-eight behavior is no longer an acceptable production authority.

### Required behavior

Refactor the production portfolio model so that:

- The historical eight known projects may remain **seed/bootstrap/migration defaults only**.
- A seed list must never define the maximum portfolio size.
- A user-registered project that is not in the seed list must **not** be archived, deleted, hidden, deactivated, renamed, or have its identity replaced merely because it is not a seed target.
- `ensure_portfolio()`, GitHub tracking refresh, Command Center snapshot refresh, Project Cockpit refresh, application startup, and restart recovery must preserve arbitrary user-registered ACTIVE projects.
- Existing lifecycle operations such as explicit user archive/remove remain valid and must not be weakened.
- Existing eight project IDs/paths/settings must be preserved where already registered.
- Registry remains the project/root identity authority. No implementation may create a second portfolio database or revive hidden `.hiveai` current-state authority.
- No project files may be modified merely by registering/reading the project.

### Direct required tests

Add production-path tests proving at minimum:

1. Start from the existing eight seeded projects, register project 9 at a distinct valid path, run portfolio reconciliation, and prove project 9 remains ACTIVE and visible.
2. Register project 10 as well and prove both 9 and 10 remain ACTIVE.
3. Run the same sequence through the code paths used by GitHub tracking snapshot/refresh and prove neither project is archived.
4. Simulate restart/reopen and prove the non-seed projects remain registered with stable IDs.
5. Explicitly archive one project and prove the system respects the explicit archived state rather than auto-reactivating it.
6. Prove no existing eight-project regression.
7. Prove duplicate normalized paths still fail safely without mutating either project.

Do not satisfy these tests by simply deleting lifecycle validation or by turning all archived projects active.

---

## M19.00.02 - Repair Add Project UX so submission can never look like a silent no-op

The current dialog uses form submission, so this task is broader than adding an Enter handler. The user-observed behavior is that typing a valid local path and pressing Enter produces no visible result. The implementation must make the true outcome explicit.

### Required behavior

- Pressing **Enter** from the local-path input must submit exactly once.
- The primary **Add/Register Project** action must use the same submission path.
- Disable duplicate concurrent submissions while one registration is pending.
- Trim only UI whitespace around the entered path. Do not apply prose-style whitespace normalization to filesystem identity.
- Canonicalize/validate the path through the existing native Registry boundary.
- A valid existing local folder can be registered even if it is not a Git repository.
- A valid Git repository can be registered whether or not GitHub metadata is currently available.
- Successful registration must close or transition the dialog visibly, refresh the project list, and make the project immediately discoverable in Projects and relevant selectors.
- A successfully registered project must remain visible after Command Center refresh, Project Cockpit refresh, GitHub tracking refresh, and application restart.
- Never silently swallow a rejected native registration.

### Required visible error/result states

Provide bounded, human-readable feedback for at least:

- path does not exist;
- path is not a directory;
- path is unreadable/inaccessible;
- duplicate/equivalent registered path;
- archived/existing identity conflict where relevant;
- native/SQLite registration failure;
- project successfully registered but GitHub identity unavailable;
- registration success.

Do not expose raw secrets or uncontrolled stack traces. Keep a diagnostic code/details surface where useful for debugging.

### Required frontend/native tests

- Enter submits once.
- Button submits once.
- pending submission cannot double-register.
- native success refreshes visible project list.
- native failure renders a visible bounded error instead of appearing inert.
- duplicate-path error is visible.
- ninth project remains visible after subsequent refreshes.
- project-switch races do not replace the newly registered project with stale eight-project data.

---

## M19.00.03 - Bulk Edit and Pixel Art Generator GitHub acquisition reliability

### Known facts

The following repositories exist and are valid GitHub repositories with `main` as their default branch:

- `Sekiph82/Bulk-Edit@main`
- `Sekiph82/ScrubBots-Level-Factory@main` (Pixel Art Generator)

Therefore do not treat this task as a repository-name typo fix without evidence.

The current native GitHub implementation uses a bounded `curl.exe` REST transport and does not add a GitHub Authorization header. One Project Cockpit snapshot can also fan out into base resources plus optional enrichment calls. This makes unauthenticated rate limiting, endpoint/resource-specific failure, cache state, and request fan-out important suspects, but **the exact native cause for each failing project must be reproduced and evidenced before selecting a fix**.

### Mandatory reproduction and diagnostics

Using the production native path, reproduce GitHub snapshot acquisition for:

1. `Sekiph82/H-veAI@main` as a known working control;
2. `Sekiph82/Bulk-Edit@main`;
3. `Sekiph82/ScrubBots-Level-Factory@main`;
4. at least one additional currently working registered repository.

For each, capture in redacted structured evidence:

- requested repository identity and branch;
- resource kind;
- HTTP status class/code where available;
- whether the failure is transport, rate-limit, forbidden, not-found, malformed payload, timeout, cache validation, or optional enrichment failure;
- relevant GitHub rate-limit headers when available;
- cache hit/miss/stale/last-good status;
- bounded redacted stderr/error diagnostic;
- request count/budget consumed for the snapshot.

Never log Authorization values, credentials, cookies, access tokens, environment secrets, or unredacted sensitive URLs.

### Fix requirements

The fix must follow the reproduced cause, while preserving these invariants:

- A successful base repository identity fetch must not be converted into `github.repo_failed` merely because an **optional** resource/enrichment endpoint failed.
- Repository identity, branches/commits, PRs, issues, Actions, releases, and tags must each retain truthful independent availability/current/partial/stale state where architecture permits.
- Optional enrichment failures must degrade locally rather than blanking or poisoning the whole GitHub panel.
- Preserve the M18 V07 panel-local error boundary and no-whole-app-black-screen behavior.
- Preserve read-only/default-denied mutation policy.
- Preserve secret/token redaction.
- Do not add a PAT/API-key Settings field as a shortcut.
- Do not scrape Git credential stores, browser cookies, `.git-credentials`, provider credentials, or unrelated environment secrets.
- Do not introduce arbitrary frontend-controlled GitHub hosts.
- Do not fabricate GitHub data when the remote source is unavailable.

If the reproduced cause is unauthenticated rate limiting or excessive fan-out, implement a bounded architecture-appropriate remedy such as request consolidation, conditional requests, cache reuse, per-resource retry timing, last-good stale fallback with explicit freshness, or lower enrichment fan-out. Do not hammer GitHub with retry loops.

If a base endpoint fails while valid last-good cache exists, the UI may display explicit stale evidence with age/source provenance. It must never label stale cache as current.

If a resource has never been successfully fetched, show truthful unavailable/partial status instead of fake empty data.

### Direct required tests

Add deterministic transport/cache tests covering at minimum:

- repository base success + optional endpoint failure;
- repository base success + optional enrichment failure;
- 403 rate-limit response;
- 429 response;
- timeout/transport failure;
- stale last-good cache fallback;
- malformed response;
- successful recovery after prior failure;
- request-budget bound;
- no secret leakage;
- Bulk Edit fixture identity;
- Pixel Art Generator / `ScrubBots-Level-Factory` fixture identity;
- H!veAI working-control regression;
- M18 black-screen/error-boundary regression.

The builder log must state the actual reproduced native cause for Bulk Edit and Pixel Art Generator separately. If the two causes differ, fix and report them separately.

---

## M19.00.04 - Portfolio-native acceptance gate

After source implementation, full regression, governed publication, and independent strict source audit PASS, the owner-native acceptance gate for M19 must include at minimum:

1. Register a ninth real local project through **Add Project** using Enter or the primary button.
2. Confirm it appears in Projects immediately and remains after navigation/restart/refresh.
3. Open Bulk Edit -> Project Cockpit -> GitHub and confirm repository evidence is usable or, if an external live condition prevents full data, the exact bounded diagnostic is truthful and the known implementation defect is not recurring.
4. Open Pixel Art Generator -> Project Cockpit -> GitHub and perform the same check.
5. Open H!veAI GitHub as a control.
6. Navigate to Command Center, Prompt Engine, Settings, and back to Projects after these operations.
7. Confirm no full black React/WebView surface.
8. Confirm the newly registered ninth project participates in the portfolio surfaces and M19 recommendation candidate universe when eligible.

Codex must not self-declare this owner-native gate passed.

---

# 3. M19.01 - Candidate task eligibility

Implement a deterministic candidate-building layer for **all ACTIVE Registry projects**, not a fixed seed list.

Requirements:

- Use canonical project/task truth under the accepted X04 authority model.
- Repository-root `TASKS.md` remains the sole local current project/task/workflow-status authority.
- Tracked-branch root `TASKS.md` remains the sole GitHub-tracked remote current-state authority where remote evidence is available.
- Historical hidden control-plane files, Project Dashboard materialization, audit rows, session rows, watcher projections, and legacy state may provide evidence/history but must not override current TASKS truth.
- Exclude completed tasks.
- Exclude tasks whose dependencies/blockers/gates make them ineligible.
- Preserve Human-required/external-wait tasks as separate attention items rather than silently deleting them.
- Unknown/unavailable truth fails closed and is visibly marked.
- A newly registered project with valid canonical TASKS must be eligible without adding source-code constants.
- A project without canonical TASKS must remain registered but must not receive fabricated task recommendations.

Add direct candidate-set fixtures covering multiple projects, ninth/non-seed project inclusion, completed tasks, blockers, external waits, missing TASKS, malformed TASKS, and stale GitHub evidence.

---

# 4. M19.02 - Deterministic priority scoring

Build a transparent scoring model using bounded factual inputs. At minimum consider:

- explicit project/task priority where authoritative;
- dependency critical path / work that unblocks other eligible work;
- verified audit/CI failure urgency;
- blocker/gate status;
- Human/external wait penalty;
- selected provider/actor availability where applicable;
- bounded context-switch cost;
- explicit owner focus when set.

Requirements:

- Scoring weights/constants must be explicit and testable.
- Missing evidence must not silently receive a favorable default.
- Historical failures that are already closed must not keep inflating urgency.
- A live CI/audit failure may influence priority only if its project/task linkage and freshness are evidenced.
- GitHub partial/unavailable state must be represented as uncertainty, not as zero failures.
- No randomized ranking.
- Deterministic tie-breaking.

Tests must prove exact score components and stable ordering for representative fixtures.

---

# 5. M19.03 - Agent/actor availability awareness

Use accepted Codex/Claude provider readiness and task actor requirements without allowing provider status to override canonical task truth.

Requirements:

- Distinguish Codex, Claude, Human, CI, GPT Audit, External, and unavailable/unknown actor states as appropriate to the existing model.
- Avoid recommending immediate execution of work that requires an unavailable builder provider.
- Keep Human-required work visible in a dedicated attention grouping.
- A provider quota/rate limitation is not task completion.
- Do not fabricate readiness from historical sessions.
- Reuse the accepted Settings/provider contracts rather than creating a third readiness model.

---

# 6. M19.04 - Explainable next-task recommendation

For the selected recommendation expose:

- project;
- task identity/title;
- current factual state;
- eligibility reason;
- score and component breakdown;
- dependencies/blockers/gates;
- required actor/provider readiness;
- evidence/freshness references;
- uncertainty/unavailable inputs;
- concise explanation of why this task is currently recommended.

Where useful, show bounded reasons why a small number of alternatives ranked lower. Do not dump the full portfolio scoring matrix into the primary UI by default.

The explanation must be derived from the same deterministic score/evidence object used for ordering. Do not generate persuasive prose disconnected from the actual ranking inputs.

---

# 7. M19.05 - Portfolio recommendation

Requirements:

- Rank across all eligible ACTIVE Registry projects, including user-added non-seed projects.
- Avoid starvation caused solely by stable alphabetical/project ordering.
- Respect explicit owner project focus where such a setting already exists and is authoritative.
- Do not infer owner preference from past conversations, project names, or unrelated activity.
- Keep blocked/Human/external-wait work visible separately.
- Project GitHub failures may reduce evidence confidence but must not make the entire registered project disappear.
- The recommendation UI must distinguish factual ranking inputs from recommendation output.

---

# 8. M19.06 - Engineering Brief

Implement a truthful bounded Engineering Brief containing at least:

- portfolio state summary;
- what changed since the last relevant visit/snapshot where evidence exists;
- items needing attention;
- blocked/Human/external waits;
- verified audit/CI/GitHub issues with freshness;
- current recommended next actions;
- provider availability that materially affects actionability;
- explicit unavailable/partial evidence notes.

Do not invent a change when no prior comparable evidence exists. Use “unavailable” or equivalent truthful state.

The brief should be useful at startup/morning and on return after time away without requiring a cloud LLM to fabricate connective narrative. If an AI wording layer is used, the factual payload and citations must remain independently inspectable and deterministic.

---

# 9. M19.07 - Truthfulness, evidence, and AI boundary

This is a hard acceptance boundary.

- Separate **facts** from **recommendations** visibly and structurally.
- Every important factual assertion must trace to project/task/Git/audit/CI/provider evidence already allowed by architecture.
- Include freshness/source state.
- Mark unavailable, stale, partial, malformed, or uncertain data explicitly.
- Never promote builder logs to independent audit truth.
- Never revive hidden `.hiveai` files as current-state authority.
- Never infer task completion from an agent claiming success.
- Never infer CI success from no returned data.
- Never infer GitHub repository health from an empty optional endpoint.
- Never infer user priority from conversational memory.
- Do not perform remote mutation as part of recommendation.

Add tests specifically targeting false-positive “healthy/current/completed” promotion from missing or stale inputs.

---

# 10. M19.08 - Tests, regression, publication, audit, and stop condition

## Focused tests

Add/extend focused Rust and frontend tests for all M19.00-M19.07 contracts, including the exact owner-observed regressions.

At minimum include:

- ninth/tenth project survives portfolio reconciliation;
- Add Project Enter/button/error/success behavior;
- arbitrary non-seed project participates in recommendation eligibility;
- Bulk Edit GitHub acquisition fixture/path;
- Pixel Art Generator / `ScrubBots-Level-Factory` GitHub acquisition fixture/path;
- GitHub partial/resource-specific degradation;
- rate-limit/cache/request-budget behavior;
- M18 V07 panel-local containment regression;
- deterministic eligibility/scoring/tie-breaks;
- blocker/dependency/gate behavior;
- Human/external wait separation;
- Codex/Claude availability handling;
- explanation-to-score consistency;
- evidence/freshness/uncertainty rendering;
- Engineering Brief change/no-change/unavailable semantics;
- cross-project isolation;
- project-switch stale-response safety.

## Full gates

Run the repository’s normal full gates, including at minimum where applicable:

- focused frontend tests;
- full frontend tests;
- typecheck;
- production frontend build;
- focused Rust tests;
- normal parallel full Rust library test gate;
- `cargo check`;
- formatting/diff checks;
- security/redaction regression;
- governed production QA publication using the accepted publisher only;
- stable executable/launcher validation;
- no development server requirement;
- no installer work in M19.

Do not weaken or skip existing tests to obtain green output.

## Immutable builder evidence

Create an immutable builder log under:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V01_LOG.md`

The log must include:

- exact starting and ending Git SHAs;
- complete changed-file list;
- explicit M19.00 root-cause findings;
- exact native reproduction result for Bulk Edit;
- exact native reproduction result for Pixel Art Generator;
- before/after behavior for ninth-project registration;
- focused/full test commands and results;
- publication result and executable hash if published;
- any unavailable/unverified evidence;
- confirmation that `TASKS.md` and `CODEX_ROADMAP.md` were not modified by Codex;
- confirmation that M20 was not started.

## Final builder stop condition

After implementation, tests, governed publication, immutable builder log, safe commit/push, and remote equality verification:

**STOP.**

Do not write an independent audit verdict. Do not mark M19 PASS/CLOSED. Do not activate M20. Return the builder-log path and final commit SHA for independent strict audit by ChatGPT.

---

# 11. Scope preservation

Preserve all accepted behavior from M00-M18 unless a change is strictly required by this prompt.

Especially preserve:

- X04 root-TASKS-only current-state authority;
- M13/M14/M15 project/session/prompt provenance;
- M16 Codex-only independent audit provider architecture;
- M17 Claude builder adapter behavior;
- M18 GitHub least-privilege/read-only default, cache truthfulness, redaction, project confinement, and V07 panel-local error containment;
- Prompt Engine + Sessions consolidated UX;
- separate Builder Providers and Codex Audit Provider Settings surfaces;
- no standalone primary Agents navigation;
- no whole-app reload or black-screen fallback;
- Registry identity stability;
- no silent project-file mutation.

Do not introduce M20 chat, installer, release, or unrelated redesign work.

---

# 12. Acceptance summary

M19 V01 is implementation-complete only when all of the following are true at source/test/publication level:

1. The portfolio is no longer capped to exactly eight user-visible projects.
2. A ninth user project can be registered and survives normal refresh/restart paths.
3. Add Project never appears to silently ignore Enter/submission; success and failure are explicit.
4. Bulk Edit GitHub acquisition has a reproduced, evidenced root cause and implemented fix/contained behavior.
5. Pixel Art Generator GitHub acquisition has a reproduced, evidenced root cause and implemented fix/contained behavior.
6. GitHub optional-resource failures cannot poison an otherwise valid repository base snapshot.
7. M18 V07 full-black-screen regression remains closed.
8. M19 candidate eligibility includes all eligible ACTIVE Registry projects without hard-coded project membership.
9. Scoring is deterministic and explainable.
10. Actor/provider availability is truthful.
11. Engineering Brief facts are evidence-backed and freshness-aware.
12. Facts and recommendations remain structurally separated.
13. Full regression and governed publication gates pass or any real blocker is reported truthfully.
14. Immutable builder evidence is committed.
15. Codex stops for independent strict audit without editing canonical trackers or starting M20.
