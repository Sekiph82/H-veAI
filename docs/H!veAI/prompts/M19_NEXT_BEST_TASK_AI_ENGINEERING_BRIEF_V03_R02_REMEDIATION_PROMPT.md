# M19 Next Best Task AI + Engineering Brief V03 R02 - Authoritative Remediation Prompt

## 0. Supersession and authority

This file supersedes the earlier V03 remediation prompt for execution purposes:

`docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_REMEDIATION_PROMPT.md`

The earlier V03 prompt remains mandatory background and must be read in full. All of its requirements remain in force unless this R02 file explicitly strengthens or clarifies them.

Also read in full before changing source:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- root `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_STRICT_REAUDIT.md`
- `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_REMEDIATION_PROMPT.md`

`TASKS.md` and `CODEX_ROADMAP.md` are read-only for Codex. Do not close M19. Do not start M20.

Safely synchronize the standalone H!veAI checkout with current GitHub `main` before work. No reset, rebase, force-push, auto-stash, clean, or destructive sync.

This R02 adds owner-observed production regressions that are acceptance blockers in addition to every V03 finding already listed.

---

# 1. Owner-observed blocker A - ninth project is registered but invisible

## Reproduction observed in the published native app

The owner attempts to add the local folder:

`C:\Users\sekip\Desktop\Beach Cocktails - Merge`

The Add Project dialog returns:

`a project with this normalized path is already registered`

But that project is not visible on the normal Projects page under the default `Active and missing` view.

This is a release blocker. The owner must never be trapped in a state where H!veAI says a path is registered while the corresponding project cannot be found or restored from the normal UI.

## Source-level mechanism that must be investigated first

Current Registry source has an important asymmetry:

- `ensure_no_duplicate()` checks every matching normalized path without filtering by status;
- normal `list_projects()` excludes `ARCHIVED` rows unless `include_archived=true`.

That is consistent with the owner symptom: an archived row can block registration while remaining invisible in the default list.

Do not assume this is the only possible path. Reproduce against a real database fixture and prove the exact state transition that creates the invisible duplicate.

## Required behavior

Explicit Add Project is an owner action. Resolve duplicate normalized paths by current Registry state instead of returning one generic duplicate error.

Required semantics:

1. Existing ACTIVE row with same normalized path:
   - do not create a duplicate;
   - return or surface the existing project clearly;
   - ensure it is immediately visible in the current Projects result after refresh;
   - message should identify that it is already active, not present a dead-end error.

2. Existing MISSING row with same normalized path and the owner supplies a now-valid folder:
   - repair/reactivate the existing row through the safe path-validation and repository-identity rules;
   - preserve project identity, settings, history, and valid repository metadata;
   - show the restored project immediately.

3. Existing ARCHIVED row with same normalized path:
   - because Add Project is an explicit owner registration action, restore/reactivate that same Registry project rather than creating a second row;
   - preserve the same project ID, settings, task policy, builder/auditor preferences, valid repository identity, and historical data;
   - clear archived state intentionally;
   - remove only the exclusion state that conflicts with this explicit restoration when appropriate;
   - do not silently rewrite unrelated settings;
   - make the project visible immediately in Projects, Command Center project rail, project shortcuts, and any Registry-backed selector.

4. A truly unrelated project that merely collides through bad normalization:
   - fail closed with a specific identity/path diagnostic;
   - do not merge unrelated repositories.

5. Removed project rows:
   - keep explicit remove/exclusion semantics intact;
   - a later explicit owner Add Project action may register the path again after validation, but must not create duplicate remote identity rows.

## Required tests

Add direct Registry and mounted UI tests for:

- archived ninth project + Add Project same normalized path -> same ID becomes ACTIVE and visible;
- archived project remains archived across passive startup/refresh when owner did not explicitly re-add it;
- ACTIVE duplicate -> no duplicate row and user sees the existing project;
- MISSING duplicate with repaired valid path -> restored safely;
- settings and repository identity survive archived restoration;
- ninth and tenth projects remain visible after app restart;
- project appears in Projects page, Command Center rail, and project shortcuts after explicit restoration;
- no eight-project allow-list reappears anywhere.

The V03 log must record the reproduced pre-fix database state and the post-fix row/status/ID evidence for this exact class of bug.

---

# 2. Owner-observed blocker B - Projects card actions overflow outside the card

## Reproduction observed

At the owner desktop viewport, the Projects grid uses three cards per row. The archive/remove icons at the right edge visibly spill outside the card border. The delete/trash icon is partially or fully outside the card on multiple rows.

Current card composition places:

- `Open cockpit`;
- the long workspace action;
- archive icon;
- remove icon

inside one non-wrapping footer action row. The responsive three-column grid makes this footer too narrow.

## Required UI behavior

Every project card must contain all of its controls inside its own visual border at supported desktop widths. No action may overlap another card, the page gutter, or the vertical scrollbar.

Requirements:

- footer layout must be intrinsically bounded with `min-width:0` where needed;
- use a responsive grid, wrapping flex layout, or another deterministic layout that keeps all controls inside the card;
- icons must retain at least their normal clickable hit target;
- no horizontal page overflow may be introduced;
- card heights may grow when needed;
- 3-column, 2-column, and 1-column responsive modes must all remain usable;
- keyboard focus outlines must remain visible and not be clipped.

## Required visible copy change

The visible button text must be:

`Local workspace`

for all project cards.

Do not show `Change local workspace` as the visible button label anymore.

The accessible name/title may still describe the exact state, for example:

- `Change local workspace for <project>`;
- `Repair local workspace for <project>`;
- `Attach local workspace for <project>`.

But the visible card label is always `Local workspace`.

## Required tests

Add mounted UI/layout assertions at representative widths matching the native owner window, including approximately 1536 px outer desktop width and narrower breakpoints.

At minimum prove:

- footer controls are inside the card bounding box;
- remove icon is fully inside the card;
- archive and remove controls do not overlap;
- `Open cockpit` and `Local workspace` remain clickable;
- no horizontal document overflow;
- visible text `Change local workspace` is absent from project cards;
- visible text `Local workspace` is present on every non-archived project card.

Prefer a deterministic DOM/geometry test or Playwright/native-webview smoke if available. A screenshot-only claim is insufficient.

---

# 3. Owner-observed blocker C - GitHub pages enter a portfolio-wide 403 rate-limit warning storm

## Reproduction observed

Bulk-Edit and ScrubBots Level Factory GitHub data now load, so the earlier repository-shape problem is materially improved.

However the owner now sees the same GitHub warning repeated many times on GitHub pages across projects. The diagnostic text contains repeated variants of:

`GITHUB_HTTP_403_RATE_LIMITED: HTTP 403 ... API rate limit exceeded ...`

The pages then show `STALE` / `REMOTE_STALE` even when last-known-good data exists.

This is not acceptable as the steady-state GitHub UX.

## Existing source behavior that must be addressed

Current GitHub integration builds the user-facing warnings vector by collecting each resource error independently. When eight primary resources all fail for the same rate-limit condition, the same large diagnostic is repeated resource-by-resource.

The current native GitHub API transport also uses bounded curl REST reads. The implementation must be audited for request fan-out, portfolio scheduler cadence, selected-project refresh cadence, cache TTL, optional enrichment fan-out, and whether requests are authenticated.

Do not treat warning dedupe alone as the fix. Prevent unnecessary rate-limit exhaustion at the transport/scheduler layer as well.

## Required GitHub request architecture

Implement a bounded portfolio-safe acquisition policy.

Requirements:

1. Request coalescing and cache reuse
   - one logical refresh for the same repository/branch/resource within the freshness window must reuse the same cache/in-flight result;
   - opening multiple panels must not fan out duplicate network requests;
   - Command Center, project cockpit, M19, and GitHub tab reads should consume shared cached observations where possible.

2. Portfolio request budget
   - define and enforce a portfolio-level request budget, not only a per-snapshot subresource budget;
   - selected-project refresh must not cause all projects to refetch all resources;
   - optional enrichments are lower priority than primary resources;
   - stop optional fan-out immediately when rate-limit pressure is detected.

3. Rate-limit circuit breaker
   - on GitHub 403 rate-limit or 429, classify the response once;
   - capture reset/remaining metadata when the current transport exposes it;
   - do not continue hammering all remaining resources after the limit is known exhausted;
   - enter a bounded backoff/circuit-open state until reset or a conservative retry time;
   - last-known-good data remains available as STALE with explicit age;
   - automatic recovery occurs after backoff/reset without requiring app restart.

4. Authenticated read path
   - inspect the existing product/security architecture before adding anything new;
   - if a secure existing GitHub authentication mechanism already exists, use it for read requests;
   - if none exists and authenticated reads are necessary to make the product viable, add an explicit owner-controlled credential configuration using an appropriate secure OS-backed storage mechanism rather than plaintext repository files, logs, command history, or source code;
   - never print Authorization headers, PATs, tokens, cookies, credential-store values, or secrets in UI/logs/tests;
   - unauthenticated fallback may remain for public repositories, but it must obey the same budget/backoff policy and must not produce a permanent warning storm.

5. Conditional requests where supported
   - use ETag / Last-Modified or another safe conditional strategy if it materially reduces API consumption;
   - persist only non-secret validators;
   - classify 304/reuse truthfully.

6. Scheduler cadence
   - calculate worst-case hourly API consumption for 8, 9, 10, and at least 20 registered projects;
   - the normal idle application must not deterministically exhaust GitHub public unauthenticated limits;
   - if authenticated limits are used, normal idle behavior must still remain comfortably below the quota;
   - document the chosen cadence and request upper bound in the V03 log.

## Required warning UX

Do not render the same raw GitHub API JSON error eight or more times.

Normalize and deduplicate warnings by factual error identity, for example:

`GitHub API rate limit reached (403). Using last-known-good cache for 8 affected resources. Retry after <time if known>.`

Requirements:

- one concise warning per distinct failure class by default;
- optional expandable diagnostics may show affected resource names;
- never dump a repeated full GitHub JSON body into the main warning list;
- bounded diagnostics only;
- identical resource errors collapse into one grouped warning with affected-count/resource list;
- distinct errors remain distinct;
- stale cache age remains visible;
- if data is current, do not show stale/rate-limit warning leftovers from an older refresh.

## Required tests

Add production-path tests for:

- eight primary resources returning the same 403 rate-limit -> one grouped warning, not eight duplicates;
- 403 on first/early primary request stops unnecessary remaining fan-out according to the new circuit policy;
- optional enrichments do not continue after circuit open;
- stale last-known-good cache is retained and labeled STALE;
- retry after reset/backoff recovers to CURRENT;
- warning clears after successful recovery;
- multiple components requesting the same repo simultaneously are coalesced;
- request count for 8/10/20 project idle portfolio remains within the documented hourly budget;
- 429 equivalent behavior;
- non-rate-limit 403 is not falsely classified as rate limit;
- timeout/malformed response remains independently classified;
- no secret appears in warnings/logs/snapshots;
- Bulk-Edit, ScrubBots-Level-Factory, H-veAI, and one additional repository all remain functional after the change.

The V03 log must include a before/after request-count table and the owner-visible grouped-warning behavior.

---

# 4. Owner-observed blocker D - GitHub current/stale truth must be coherent

The owner screenshots show combinations such as a usable last-known-good remote HEAD plus `STALE`, or unavailable remote HEAD plus stale cached diagnostics.

That can be truthful, but the UI must make the state understandable.

Required semantics:

- `CURRENT`: all required primary evidence needed for the shown claim is current within the documented freshness horizon;
- `STALE`: last-known-good evidence is shown, with fetched time/age and the current failure reason;
- `UNAVAILABLE`: no usable last-known-good evidence exists for the required claim;
- local Git dirtiness is separate from remote freshness;
- `REMOTE_STALE` reconciliation must cite the exact freshness/rate-limit reason;
- a rate-limited refresh must not erase a known remote HEAD from the last-known-good cache;
- a successful refresh must immediately replace stale diagnostics and remove obsolete warnings.

Add direct tests for the state transitions CURRENT -> RATE_LIMITED/STALE -> CURRENT and CURRENT -> offline/STALE -> CURRENT.

---

# 5. Integration with the existing V03 findings

All previous V03 requirements remain mandatory, including:

- unfinished dependency blocks eligibility;
- pure exact-root TASKS parser for M19;
- read-pure M19/Command Center snapshot computation;
- remote recommendation-time freshness;
- exact dependency-unlock scoring;
- structured failure evidence for local and remote candidates;
- one integrated Engineering Brief surface;
- complete verification matrix;
- GitHub acquisition evidence matrix;
- comparison schema/freshness/hash truthfulness;
- explicit M19 history recording separate from observational reads.

The new owner-observed blockers are not optional cosmetic work. They are part of M19 owner-native acceptance readiness.

---

# 6. Additional mandatory regression matrix for R02

In addition to every test required by the prior V03 prompt, execute and record these owner-facing scenarios:

1. Start with the historical seed portfolio and an archived ninth project whose normalized path is `C:\Users\sekip\Desktop\Beach Cocktails - Merge` or an equivalent temp fixture.
2. Default Projects view does not show archived project before explicit action.
3. Add Project with that exact path.
4. The existing archived row is restored rather than duplicated.
5. Same project ID is now ACTIVE and visible immediately.
6. Restart the native app/database and prove it remains visible.
7. Add a tenth unrelated project and prove both remain.
8. Verify Projects page card actions are fully contained at owner-like desktop width.
9. Verify visible card copy is `Local workspace` for all active/missing projects.
10. Open Bulk-Edit GitHub page and ScrubBots-Level-Factory GitHub page and prove primary data can load without a repeated warning wall.
11. Force a 403 rate-limit fixture and prove exactly one grouped warning plus stale last-known-good behavior.
12. Recover the fixture and prove the warning disappears and health returns CURRENT.
13. Confirm H-veAI GitHub page and at least one additional project follow the same behavior.
14. Confirm M18 panel-local containment remains intact and no full black surface occurs.

These must be real assertions against production paths, not comments or screenshots alone.

---

# 7. Publication and native QA gate

Run all publication gates from the prior V03 prompt plus a native owner-window smoke focused on the three newly reported regressions.

Before declaring implementation ready for re-audit, prove in the published EXE:

- ninth restored project is visible after explicit Add Project and after restart;
- no footer delete icon is outside its project card;
- every visible workspace button says `Local workspace`;
- GitHub tabs do not display a wall of repeated identical 403 warnings;
- when rate-limited, one concise grouped warning is shown and last-known-good data remains truthful;
- after recovery, stale warning state clears.

Codex must not self-claim owner acceptance. This is only builder evidence before independent source audit and later owner-native acceptance.

---

# 8. Required V03 R02 immutable builder log

Create only after all implementation, tests, and publication are complete:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_LOG.md`

The log must contain everything required by the earlier V03 log specification plus:

- exact reproduced reason the ninth project was invisible;
- pre-fix project row status/ID and post-fix restored status/ID evidence;
- proof no duplicate Registry row was created;
- restart persistence evidence for ninth/tenth projects;
- mounted/native card containment evidence;
- confirmation that visible workspace button copy is `Local workspace`;
- GitHub request-count analysis before/after;
- rate-limit circuit/backoff behavior;
- grouped-warning dedupe evidence;
- stale last-known-good preservation and successful recovery evidence;
- explicit results for Bulk-Edit, ScrubBots-Level-Factory, H-veAI, and one extra control project;
- secret-redaction evidence;
- full focused and regression test results;
- published EXE SHA-256;
- statement that `TASKS.md` and `CODEX_ROADMAP.md` were unchanged;
- statement that M20 was not started;
- statement that owner-native acceptance is not self-claimed.

Builder log remains a claim, not acceptance evidence.

---

# 9. Final stop gate

After implementation:

1. verify `TASKS.md` and `CODEX_ROADMAP.md` remain untouched;
2. verify M20 was not started;
3. commit implementation;
4. create and commit `M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_LOG.md` only after implementation SHA is final;
5. push non-force to `origin/main`;
6. verify clean worktree;
7. verify local `HEAD == origin/main == live GitHub main`;
8. verify the R02 log is reachable on GitHub;
9. stop for independent strict re-audit.
