# M16F REV3 + Unified Project Control Plane Whole-System Closure Remediation Prompt

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous whole-system remediation run.

Read and close every finding in:

`H!veAI/docs/H!veAI/audits/M16E_UNIFIED_PROJECT_CONTROL_PLANE_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`

Close all:

- UCP-R01
- UCP-R02
- UCP-R03
- UCP-R04
- UCP-R05
- UCP-R06
- UCP-R07
- UCP-R08
- UCP-R09
- UCP-R10
- UCP-R11
- UCP-R12

Do not stop after any individual finding.

M16 audit-engine findings R82-R85 are already closed. Preserve them.

After all named fixes, perform a fresh adversarial sweep across the entire M16 + control-plane + watcher + Command Center + Project Cockpit + Git sync surface. If an adjacent BLOCKER/MAJOR is found, fix it in the same run and add a direct regression.

Do not activate M17.
Do not start M21.
M16 remains OPEN until independent whole-system strict re-audit and owner native/visual acceptance.

---

# 1. Product authority

H!veAI is the owner's single project command center.

The required user outcome is:

`Open laptop → open H!veAI → all projects show current real state → click one project → continue exactly where that project stopped`.

The control plane exists to normalize project state, not to create a second roadmap or duplicate task ledger.

Every tracked project may keep its own canonical task filename/path, but every project must expose the same normalized H!veAI semantics.

---

# 2. Read before editing

Read completely:

1. `H!veAI/GPT.md`
2. M16 implementation prompt
3. all M16A-E prompts/logs/audits
4. `M16E_UNIFIED_PROJECT_CONTROL_PLANE_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`
5. current `control_plane.rs`
6. current `project_dashboard.rs`
7. current `watcher.rs`
8. current Project Registry / Task Sources / Task Intelligence / Workflow / Git Engine
9. current Command Center / Project Cockpit frontend and native source
10. current migration SQL
11. current direct tests
12. current live portfolio control-plane files from all eight repositories

Builder logs remain claims only.

---


---

# 2A. Native screenshot evidence that MUST be reproduced before fixing

Owner native screenshot from 2026-09-08 exposes concrete current failures in:

`Scrubbots - Pixel Art Generator`

Registered path shown by H!veAI:

`C:\Users\sekip\Desktop\Scrubbots - Pixel Art Generator`

The native banner currently says:

`PROJECT.json is malformed: invalid type: string "https://github.com/Sekiph82/ScrubBots-Level-Factory", expected struct RepositoryIdentity ... | NON_GIT_PROJECT: registered project is not a Git repository`

The same screen also incorrectly shows:

- Current task: `**PAG-M00 — Repository Bootstrap & Governance**`
- Milestone progress: `4%`
- Health: `UNKNOWN`
- Repository: `Unavailable`
- Current milestone: `Unknown`
- Required actor: `Unknown`
- Next action: `Next action unavailable`

These are not cosmetic issues. They prove migration and reconciliation are still using stale/incompatible authority.

## Mandatory screenshot-specific closures

### A. Legacy repository string normalization

The upgrader must accept and normalize all bounded legacy repository string forms that exist or are reasonably expected in the portfolio:

- `Sekiph82/RepoName`
- `https://github.com/Sekiph82/RepoName`
- `https://github.com/Sekiph82/RepoName.git`
- `git@github.com:Sekiph82/RepoName.git`

Normalize to:

```json
{
  "owner": "Sekiph82",
  "name": "RepoName",
  "branch": "main"
}
```

Do not accept arbitrary hosts or escaped paths.

Add the exact ScrubBots-Level-Factory HTTPS URL fixture from the screenshot.

### B. Registry Git capability must be re-detected after local repository synchronization

The screenshot still says:

`NON_GIT_PROJECT: registered project is not a Git repository`

even though the local ScrubBots-Level-Factory working copy has subsequently been synchronized successfully and is a clean Git repository.

Do not treat `is_git_repository` or repository metadata captured at original registration time as permanently authoritative.

On:

- application startup;
- explicit Refresh;
- Reconcile now;
- filesystem appearance/change of `.git`;
- successful clone/connect/repair;
- 60-second safety reconciliation;

re-probe the actual registered project root and refresh Registry repository metadata safely.

Required transition test:

1. register a normal folder as non-Git;
2. initialize/connect a valid matching Git checkout in the same registered root;
3. run reconcile/refresh;
4. H!veAI must change local Git state to CONNECTED without deleting/re-registering the project.

The reverse case must also be detected truthfully if `.git` disappears or becomes invalid.

### C. Current task precedence and heading hygiene

For ScrubBots-Level-Factory, current control-plane state already identifies the active cycle/task as:

`PAG-M00-C003`

`Bootstrap Reliability & Offline Enforcement Remediation`

The Project Cockpit must not fall back to a top-level milestone heading such as:

`**PAG-M00 — Repository Bootstrap & Governance**`

when STATE/workflow/handoff provides a stronger current task pointer.

Required precedence for displayed current task:

1. verified active native workflow task, when applicable;
2. valid STATE.json currentTaskId/currentTaskTitle;
3. valid HANDOFF resume/current task pointer;
4. canonical task source lookup by that exact ID;
5. only then a bounded fallback heuristic for unmigrated projects.

Do not select a milestone heading as a task merely because it is the first open-looking heading.

Strip Markdown presentation markers from labels rendered in native UI. A task title must not visibly include surrounding `**`.

### D. Milestone progress must be provenance-correct

Do not display a synthetic `4%` simply because a parser counted arbitrary global checkboxes or headings.

Progress must come from, in order:

1. explicit valid STATE progress for the current milestone/cycle;
2. canonical task ledger counts scoped to the current milestone/cycle;
3. otherwise unavailable/unknown.

Never mix all historical project tasks into current milestone progress.

Add exact Level Factory fixture proving the current milestone progress cannot silently become 4% from unrelated/historical task counts.

### E. Current milestone / actor / next action must recover after migration

Once the legacy PROJECT.json is upgraded and the existing state/handoff are reconciled, the same native screenshot scenario must resolve to meaningful values including:

- current milestone `PAG-M00`;
- current cycle/task `PAG-M00-C003`;
- required actor `CODEX`;
- next action from the existing normalized state/handoff;
- known remote repository `Sekiph82/ScrubBots-Level-Factory`;
- local Git CONNECTED when the registered folder contains the synchronized checkout.

No stale `Unknown`, `Unavailable`, or malformed banner may remain after a successful migration/reconciliation.

### F. Native acceptance fixture

Add one end-to-end native/backend fixture that starts from the exact screenshot-era legacy files and repository URL string, performs upgrade + re-probe + reconcile, and asserts the final normalized snapshot.

This exact screenshot case is a release-gate fixture, not merely a manual note.


---

# 2B. Full native portfolio screenshot matrix — release-gate evidence

The owner supplied native screenshots for all tracked project cockpit states plus Command Center. Treat this entire matrix as required reproduction evidence before coding and as final native acceptance evidence after coding.

## 2B.1 AI-Commerce-HQ

Native state currently shows:

- banner: `PROJECT.json is malformed: invalid type: string "Sekiph82/AI-Commerce-HQ", expected struct RepositoryIdentity`
- Current task: unavailable
- Milestone progress: Unknown
- Health: UNKNOWN
- Registry repository identity is visible
- Current milestone: Unknown
- Required actor: Unknown
- Next action: unavailable

Required closure:

- migrate the legacy repository string;
- resolve the nested canonical task source correctly;
- recover normalized task/milestone/workflow/next-action state when authoritative evidence exists;
- do not let the nested `H!veAI/TASKS.md` layout become invisible merely because the registered root is the repository root.

Add an exact AI-Commerce-HQ nested-control-plane fixture.

## 2B.2 Bulk-Edit

Native state currently shows:

- malformed repository string `Sekiph82/Bulk-Edit`;
- Current task displayed as `Stripe production workflow review (webhook endpoint status manually re-verified in Stripe dashboard).`;
- milestone progress `69%`;
- Health UNKNOWN;
- Current milestone Unknown;
- Required actor Unknown;
- Next action unavailable.

Required closure:

- migrate the repository string;
- preserve the current task only if it resolves to an actual canonical task identity/state;
- derive milestone/cycle and progress from the same canonical scope as the current task;
- recover next action and required actor from workflow/state/handoff when present;
- if the displayed sentence is not a canonical task node, do not treat prose as task identity.

Add an exact Bulk-Edit fixture using the current task ledger shape.

## 2B.3 fmcg-erp-system

Native state currently shows:

- malformed repository string `Sekiph82/fmcg-erp-system`;
- Current task displayed as:
  `TASK-014.3 closed. TASK-014.4 Graphify refresh completed next.`
- state shown OPEN;
- milestone progress `0%`;
- milestone/actor unknown;
- next action unavailable.

This is strong evidence that the task parser can promote status prose / transition prose into a task title.

Required closure:

- current task must resolve to a canonical task item with a stable ID;
- a sentence containing a closed task and a “next” task must not itself become one synthetic current task;
- if `TASK-014.3` is closed and `TASK-014.4` is the next actionable task, resolve the actual canonical `TASK-014.4` item, not the prose sentence;
- progress must use the correct milestone/sprint scope and must not default to 0% from parse failure.

Add an exact regression fixture for this prose-to-task failure.

## 2B.4 FormuLab

Native state currently shows:

- malformed repository string `Sekiph82/FormuLab`;
- Current task unavailable;
- progress Unknown;
- milestone/actor unknown;
- next action unavailable.

FormuLab's canonical task source is nested:

`docs/FORMULAB_V1_TASK_TRACKER.md`

Required closure:

- migrate repository string;
- load and watch the nested canonical task source;
- prove task intelligence and live refresh work from that nested path;
- recover current state from the nested tracker + normalized STATE/HANDOFF authority.

## 2B.5 PackLab

Native state currently shows:

- malformed repository string `Sekiph82/PackLab`;
- `task intelligence has not been parsed for this project`;
- `NON_GIT_PROJECT: registered project is not a Git repository`;
- Current task unavailable;
- progress Unknown;
- Repository unavailable;
- milestone/actor unknown;
- next action unavailable.

Required closure:

- migrate repository string;
- separate Registry-known remote identity from local Git status;
- show `Remote: Sekiph82/PackLab` even if local Git is not connected;
- parse/watch canonical `TASKS.md`;
- expose a safe Connect/Repair local Git action;
- after a valid checkout exists at the registered root, re-probe and automatically transition local Git state to CONNECTED.

## 2B.6 PackLab 3D

Native state currently shows:

- malformed repository string `Sekiph82/PackLab-3D`;
- `task intelligence has not been parsed for this project`;
- Current task unavailable;
- progress Unknown;
- Registry repository identity visible;
- milestone/actor unknown;
- next action unavailable.

Canonical task source is lowercase:

`tasks.md`

Required closure:

- migrate repository string;
- support lowercase canonical task source exactly as declared;
- task intelligence must not assume `TASKS.md`;
- watcher must attach to lowercase `tasks.md`;
- recover normalized current state from canonical evidence.

## 2B.7 ScrubBots

Native state currently shows:

- malformed repository string `Sekiph82/Scrubbots`;
- stale Current task:
  `SB-M02-017 Add RESERVED only when reservation architecture is designed (see M14).`
- milestone progress `23%`;
- current milestone Unknown;
- required actor Unknown;
- next action unavailable.

This project is the strongest stale-task regression.

Required closure:

- migrate repository string;
- do not select “first open task” or an old open historical item as current task;
- current task selection must follow verified workflow/STATE/HANDOFF/current-cycle precedence;
- progress must correspond to current milestone/cycle, not whole-file checkbox percentage;
- if no authoritative current task exists, say NEEDS_RECONCILIATION rather than inventing/stale-selecting one.

Add exact stale-`SB-M02-017` regression.

## 2B.8 ScrubBots - Pixel Art Generator / Level Factory

Native state currently shows:

- malformed HTTPS repository string;
- NON_GIT_PROJECT despite the local repo having since been synchronized successfully;
- wrong milestone heading rendered as current task with Markdown `**`;
- synthetic `4%` progress;
- Repository unavailable;
- milestone/actor unknown;
- next action unavailable.

Keep and enforce all screenshot-specific requirements already defined in section 2A.

---

# 2C. Command Center native failure — global fault isolation is mandatory

The owner also supplied a Command Center screenshot showing:

`Live Command Center snapshot is unavailable; showing Registry identity only.`

The entire portfolio degrades to registry-only because one or more projects have malformed/unavailable state.

Current screen also shows:

- 8 registered
- Active tasks unavailable
- Snapshot unavailable
- `FALLBACK_M08_M09`
- no meaningful portfolio health/work queue

This is unacceptable for a portfolio command center.

## Required fault-isolation invariant

> One malformed, unavailable, dirty, non-Git, or unreconciled project must NEVER make the entire Command Center snapshot unavailable.

Implement per-project isolation:

- each project reconciliation returns a typed success/degraded/error result;
- Command Center aggregates all healthy/degraded projects independently;
- malformed project contributes one `NEEDS_ATTENTION` item with its exact reason;
- the other projects remain fully visible and live;
- portfolio counters derive from available projects plus explicit degraded counts;
- global snapshot only becomes unavailable for a true platform/DB/runtime-wide failure, never a single-project parse failure.

Required test:

1. eight registered projects;
2. one malformed legacy PROJECT.json;
3. seven valid/adopted projects;
4. Command Center must render the seven valid live states plus one degraded project;
5. no registry-only global fallback.

## Remove normal-use FALLBACK_M08_M09

After project control-plane migration:

- `FALLBACK_M08_M09` must not be a normal authority label in Command Center or Cockpit;
- legacy fallback may exist only for explicitly unmigrated projects and must be visibly marked as migration debt;
- one legacy project must not force the whole portfolio into fallback.

---

# 2D. Cross-project task intelligence contract

The screenshots prove current task intelligence is inconsistent across project shapes.

Define one canonical resolver used by both Command Center and Project Cockpit.

## Resolution order

1. verified active native workflow task;
2. normalized STATE currentTaskId;
3. HANDOFF explicit current/resume task ID;
4. canonical task ledger exact ID lookup;
5. project-specific declared current-cycle pointer;
6. bounded legacy fallback only for truly unmigrated projects.

Never use:

- first unchecked checkbox globally;
- first OPEN item globally;
- milestone heading;
- prose sentence containing multiple task transitions;
- historical unresolved task from an old milestone;

as current task when stronger evidence exists.

## Required normalized output

A current task projection must carry:

- taskId
- title
- lifecycle state
- milestoneId
- sprint/cycleId
- authority source
- provenance path
- optional required actor
- optional next action

If taskId cannot be established, report `NEEDS_RECONCILIATION`, not a fabricated title.

---

# 2E. Progress semantics across all projects

The screenshots currently show:

- Bulk-Edit 69%
- fmcg 0%
- ScrubBots 23%
- Pixel Generator 4%
- others Unknown

Do not preserve these numbers merely because they were previously displayed.

Progress is valid only when:

1. explicit normalized STATE progress is fresh and scoped to the current milestone/cycle; or
2. canonical task counts can be deterministically scoped to that milestone/cycle.

Otherwise show unavailable.

Do not compute progress from:

- all historical checkboxes in the file;
- prose bullets;
- closed archived milestones;
- unrelated cycles;
- parse-failure defaults.

Add cross-project progress fixtures for all eight project shapes.

---

# 2F. Native portfolio acceptance matrix

The final release-gate must verify each registered project individually.

For every project, record:

- registered local path;
- remote identity;
- local Git status;
- control-plane schema/version;
- canonical task source;
- current milestone/cycle;
- current task ID/title or truthful NEEDS_RECONCILIATION;
- progress or truthful unavailable;
- required actor or truthful unavailable;
- next action or truthful unavailable;
- health;
- sync state;
- last audit/session where applicable.

Required final screenshots/native evidence must prove:

1. AI-Commerce-HQ no malformed banner.
2. Bulk-Edit no malformed banner and task identity is canonical.
3. fmcg does not show transition prose as one synthetic task.
4. FormuLab reads nested tracker.
5. PackLab shows known remote even if local Git requires repair.
6. PackLab3D reads lowercase tasks.md.
7. ScrubBots no longer shows stale SB-M02-017 unless that task is truly authoritative now.
8. Pixel Generator no Markdown task heading, no stale 4%, and Git state re-probes correctly.
9. Command Center remains live even if one project is degraded.
10. Command Center and each Cockpit agree on the same normalized project state.

This matrix is mandatory user-facing closure evidence.

# 3. One canonical control-plane schema

Standardize on:

`hiveai-project-control-plane/v1`

Do not keep the currently deployed seven-repository `hiveai-project/v1` files as the normal runtime format.

## 3.1 PROJECT.json

Final supported shape:

```json
{
  "schema": "hiveai-project-control-plane/v1",
  "projectKey": "stable-key",
  "displayName": "Project name",
  "repository": {
    "owner": "Sekiph82",
    "name": "RepoName",
    "branch": "main"
  },
  "canonicalTaskSource": "TASKS.md",
  "rules": ".hiveai/RULES.md",
  "state": ".hiveai/STATE.json",
  "handoff": ".hiveai/HANDOFF.md",
  "events": ".hiveai/EVENTS.jsonl",
  "eventSources": []
}
```

Support additive optional governance fields.

Do not require every repo to rename its task ledger.

Required portfolio task mappings include:

- AI-Commerce-HQ/H!veAI → `TASKS.md` inside H!veAI root
- Bulk-Edit → `TASKS.md`
- fmcg-erp-system → `TASKS.md`
- FormuLab → `docs/FORMULAB_V1_TASK_TRACKER.md`
- PackLab → `TASKS.md`
- PackLab-3D → `tasks.md`
- ScrubBots → `tasks.md`
- ScrubBots-Level-Factory → `tasks.md`

---

# 4. Migration/upgrader instead of create-if-missing-only

Implement a bounded deterministic control-plane upgrader.

It must safely convert:

- `hiveai-project/v1`
- repository string → repository object
- old STATE shape → normalized current STATE shape
- existing Markdown HANDOFF → preserved Markdown + normalized machine-readable state
- old PROJECT_DASHBOARD pointer data → new PROJECT.json authority where safe

Rules:

- preserve canonical task truth;
- preserve project-specific governance;
- preserve human-written handoff prose;
- preserve audits/prompts/logs/history;
- never overwrite unknown project-specific content;
- if migration is ambiguous, mark NEEDS_RECONCILIATION and show exact conflict.

Do not require deleting/replacing existing control-plane files manually.

---

# 5. HANDOFF contract

`.hiveai/HANDOFF.md` remains human-readable Markdown.

Do not deserialize the whole Markdown file as JSON.

Machine state belongs in `STATE.json`.

Define a bounded optional metadata section if needed, but the human handoff body must remain readable and project-specific governance-safe.

H!veAI derives:

- resume pointer;
- current task;
- next actor;
- next action;

primarily from STATE.json and canonical task/workflow evidence.

HANDOFF is supporting resume context, not the sole machine state source.

---

# 6. Fix watcher source authority

The watcher must resolve the actual adopted PROJECT.json and use:

- canonicalTaskSource;
- PROJECT.json;
- RULES.md;
- STATE.json;
- HANDOFF.md;
- EVENTS.jsonl;
- SESSION_RESULT.json if present;
- safe declared eventSources;
- Git HEAD symbolic ref target;
- Git index;
- relevant remote-tracking ref where safe.

Never pass an empty declared-source list to control-plane watcher setup.

Never hard-code root `TASKS.md` as the only canonical task candidate.

Add direct fixtures for lowercase and nested task paths.

---

# 7. Watch actual Git refs

Resolve symbolic HEAD.

If HEAD points to:

`refs/heads/main`

watch:

- `.git/HEAD`
- resolved branch ref or packed-ref equivalent;
- `.git/index`;
- relevant upstream tracking ref after fetch.

Reconfigure watches when branch changes.

A normal local commit must invalidate project Git state even when .git/HEAD text itself is unchanged.

---

# 8. 60-second safety reconciliation

Add an idempotent reconciliation scheduler.

Primary path:

- filesystem events;
- debounce/coalescing;
- immediate affected-project refresh.

Safety path:

- every 60 seconds;
- reconcile every ACTIVE registered project;
- no duplicate state/event churn when nothing changed.

Lifecycle:

- starts with application;
- cleanly stops on shutdown;
- bounded concurrency;
- no runaway threads.

---

# 9. Remote fetch and safe fast-forward scheduler

Add bounded background remote reconciliation.

For valid local Git projects:

1. fetch/prune safely on a bounded cadence;
2. recompute upstream/ahead/behind/diverged;
3. if owner setting enabled AND:
   - worktree clean;
   - no conflicts;
   - upstream configured;
   - ahead == 0;
   - behind > 0;
   - no divergence;
   then update only via fast-forward.
4. otherwise show SYNC_ATTENTION.

Never:

- reset;
- rebase;
- auto-stash;
- force checkout;
- discard untracked;
- auto-merge divergence.

After successful FF, immediately reconcile the project and update Command Center / Cockpit.

---

# 10. Native UI actions

Wire existing native capabilities into visible UX.

## Projects / Cockpit

Add visible actions where applicable:

- Adopt / Upgrade control plane
- Reconcile now
- Sync remote now
- Connect / Repair local Git

## Settings

Add:

`Safe auto-fast-forward clean tracked projects`

Persist the setting.

## Status

Show separately:

- Remote repository
- Remote status
- Local Git status
- Branch
- Upstream
- Ahead
- Behind
- Dirty/conflicted
- Sync action/reason

Do not collapse remote identity and local checkout state.

---

# 11. Known remote for non-Git folders

If Registry knows:

`Sekiph82/PackLab`

but local path is not a Git repository, snapshot must say:

- Remote repository: Sekiph82/PackLab
- Remote status: KNOWN/CONNECTED_TO_REGISTRY
- Local Git: NOT_CONNECTED

Do the same for ScrubBots-Level-Factory / Pixel Art Generator and any equivalent project.

Never display “No remote detected” when Registry already has remote identity.

Repair workflow must preserve local data.

---

# 12. Project State Reconciler

Implement one actor-aware reconciler.

Inputs:

- canonical task source;
- STATE.json;
- HANDOFF.md;
- EVENTS.jsonl;
- SESSION_RESULT.json;
- native workflow records;
- native agent sessions;
- native audit records;
- prompt/remediation provenance;
- Git state;
- project-specific governance.

Outputs:

- normalized ControlPlaneSnapshot;
- updated STATE.json only when an allowed state transition occurred;
- append-only event when a meaningful lifecycle event occurred;
- optional HANDOFF update only when actor governance permits;
- runtime registry projection.

Do not create a second task ledger.

---

# 13. Internal lifecycle wiring

Wire these H!veAI-native transitions into the reconciler:

- task start/complete/reopen;
- workflow transition;
- Prompt Engine remediation creation;
- Agent session start/end;
- Audit start/pass/fail;
- owner acceptance;
- release/publication where project governance uses it.

One logical transition produces at most one normalized event.

Prevent watcher feedback loops from H!veAI's own writes.

---

# 14. External provider SESSION_RESULT

Implement production support for optional:

`.hiveai/SESSION_RESULT.json`

Bounded schema:

- provider
- sessionId
- taskId
- state
- finalResponse/summary
- changedFiles
- tests
- commit
- requested workflow transition
- capturedAt

This is CLAIM_ONLY until reconciled against Git/task/governance evidence.

External Codex/Claude must not gain permission to close tasks merely by writing SESSION_RESULT.

If project governance forbids provider file mutation, support manual import of the normalized final result instead.

---

# 15. EVENTS tail reader

Fix append-only event history.

Requirements:

- newest bounded N events, not first N;
- bounded streaming/tail read;
- support files larger than 128 KiB;
- do not load entire unbounded history;
- latest event timestamp must remain current;
- malformed individual lines are reported without dropping all valid recent history;
- append-only integrity preserved.

Add >128-event and >128KiB fixtures.

---

# 16. Legacy dashboard parser closure

The legacy compatibility parser must not infer arbitrary front matter from prose.

Implement either:

A. explicit `---` delimited metadata; or
B. fixed allowlisted top-level pointer keys before the first section.

Body lines containing `:` must never count toward a front-matter field limit.

Add exact regression for:

`front-matter field limit reached (32)`

and prove the false-positive class is gone.

Once a project is upgraded to control-plane v1, legacy dashboard state must not drive current task/progress/workflow.

---

# 17. Persist metadata after Git enrichment

Move control-plane metadata persistence until after:

- Git snapshot enrichment;
- final health;
- final sync status.

Persisted and returned control-plane sync state must match.

---

# 18. Migrate all eight GitHub repositories

The current remote portfolio already contains older bootstrap files.

Upgrade all eight repository control-plane files to the final accepted schema.

Do this safely through normal commits/PRs respecting branch protection.

Do not overwrite canonical task ledgers.

Repositories:

- Sekiph82/AI-Commerce-HQ
- Sekiph82/Bulk-Edit
- Sekiph82/fmcg-erp-system
- Sekiph82/FormuLab
- Sekiph82/PackLab
- Sekiph82/PackLab-3D
- Sekiph82/Scrubbots
- Sekiph82/ScrubBots-Level-Factory

For each, record:

- target branch;
- previous schema;
- new schema;
- canonical task source;
- governance preservation;
- commit/PR;
- final merged SHA.

If a branch is protected, use PR. Never force.

---

# 19. Reconcile registered local roots

After the H!veAI runtime upgrader is implemented:

- inspect Registry's actual local path for every project;
- inspect local Git status;
- if clean and safely behind only, fetch/FF;
- if dirty/ahead/diverged, report and do not mutate;
- if non-Git, keep known remote and create repair plan;
- run control-plane migration/reconciliation only when safe.

Do not guess paths from project names.

Record exact status for all eight local roots.

---

# 20. Command Center / Project Cockpit truth

For adopted projects, primary displayed current state comes from the normalized control-plane reconciler plus canonical task/workflow authority.

Do not show:

- FALLBACK_M08_M09
- stale first-open-task heuristics
- legacy dashboard progress

as normal truth after adoption.

Show:

- milestone/cycle
- current task
- workflow
- next action
- progress
- blockers
- remote/local Git
- audit
- latest agent/session
- health/sync

Command Center and Project Cockpit must project the same project truth.

---

# 21. Tests

Add exact portfolio-shape fixtures for the **currently deployed** repository files, not only ideal templates.

Minimum direct tests:

1. old `hiveai-project/v1` Bulk-Edit PROJECT upgrade;
2. repository string → object migration;
3. Markdown HANDOFF preserved;
4. old STATE shape migrated;
5. FormuLab nested canonical task source;
6. lowercase PackLab3D tasks.md;
7. ScrubBots tasks.md;
8. Level Factory stricter governance preserved;
9. H!veAI nested root TASKS.md;
10. watcher attaches declared nested task path;
11. watcher attaches lowercase task path;
12. watcher observes local commit branch ref;
13. watcher branch-switch reconfiguration;
14. 60-second reconciliation idempotent;
15. missed event recovered by safety pass;
16. remote fetch refreshes behind count;
17. clean-behind auto-FF;
18. dirty-behind refusal;
19. ahead refusal;
20. divergence refusal;
21. non-Git known remote remains visible;
22. repair plan does not mutate populated folder;
23. UI adopt action;
24. UI reconcile action;
25. UI sync action;
26. Settings auto-FF toggle;
27. SESSION_RESULT claim-only import;
28. internal Agent completion reconciliation;
29. Audit result reconciliation;
30. task/workflow event reconciliation;
31. watcher self-write loop suppression;
32. EVENTS newest 128 after >128 records;
33. EVENTS >128KiB tail remains readable;
34. malformed recent event does not destroy all history;
35. legacy 32-field false-positive regression;
36. persisted sync status equals returned Git-enriched state;
37. all R82-R85 regressions remain green.

---

# 22. Full verification

Run:

- focused control-plane Rust tests;
- watcher tests;
- Git Engine tests;
- Registry tests;
- dashboard parser tests;
- task-source/intelligence tests;
- workflow tests;
- Agent tests;
- Prompt Engine tests;
- Audit tests;
- Command Center frontend tests;
- Project Cockpit frontend tests;
- Settings tests;
- full serialized Rust suite;
- all-targets default;
- all-targets pty-support;
- full frontend suite once, zero retry-dependent failures;
- TypeScript typecheck;
- Vite production build;
- npm audit high;
- cargo fmt;
- git diff --check;
- publisher rollback 9/9;
- governed no-bundle publication;
- candidate/stable SHA equality;
- PE/icon/shortcut/startup/no-console checks.

---

# 23. Native acceptance preparation

If native access is available, verify:

1. all eight registered projects;
2. all migrated projects say ADOPTED;
3. no adopted project shows FALLBACK_M08_M09 as authority;
4. ScrubBots does not show stale old task;
5. FormuLab reads nested tracker;
6. PackLab shows known remote even if local Git repair is needed;
7. Pixel Generator shows known remote/local Git separately;
8. Task file edit updates within ~1 second;
9. local commit updates Git projection;
10. missed event is recovered within 60 seconds;
11. safe remote FF updates UI;
12. dirty repo is never modified;
13. Adopt/Reconcile/Sync/Repair UI works;
14. Command Center and Cockpit agree;
15. Cockpit tabs remain functional.

User is final visual authority.

---

# 24. Final adversarial sweep

Before builder log:

- inspect every migration path;
- inspect every portfolio repository file;
- inspect every watcher source;
- inspect every timer/thread;
- inspect every Git mutation;
- inspect every actor permission;
- inspect every state/event writer;
- inspect SESSION_RESULT claim boundary;
- inspect legacy fallback reachability;
- inspect Command Center/Cockpit precedence;
- inspect large EVENTS history;
- inspect non-Git remote identity;
- inspect R82-R85 regressions.

If a new BLOCKER/MAJOR is found, fix it now and add a regression. Do not ask for another prompt.

---

# 25. Explicit gates

1. Read GPT.md.
2. Sync origin/H!veAI FF-only.
3. Record starting HEAD.
4. Read whole-system audit.
5. Confirm R82-R85 closed.
6. Reproduce UCP-R01.
7. Reproduce UCP-R02.
8. Reproduce UCP-R03.
9. Reproduce UCP-R04.
10. Reproduce UCP-R05.
11. Reproduce UCP-R06.
12. Reproduce UCP-R07.
13. Reproduce UCP-R08.
14. Reproduce UCP-R09.
15. Reproduce UCP-R10.
16. Reproduce UCP-R11.
17. Reproduce UCP-R12.
18. Finalize v1 schema.
19. Implement old-schema upgrader.
20. Implement repository-string migration.
21. Implement state migration.
22. Preserve Markdown handoff.
23. Preserve governance.
24. Fix adoption for heterogeneous task sources.
25. Remove hardcoded TASKS watcher assumption.
26. Load declared event sources.
27. Watch resolved local branch ref.
28. Watch relevant upstream ref.
29. Reconfigure on branch change.
30. Add 60s scheduler.
31. Add clean shutdown.
32. Add bounded remote fetch scheduler.
33. Add safe FF guard.
34. Add auto-FF setting persistence.
35. Add Adopt UI.
36. Add Reconcile UI.
37. Add Sync UI.
38. Add Repair/Connect UI.
39. Separate remote/local Git status.
40. Project known remote for non-Git.
41. Implement state reconciler.
42. Wire Task lifecycle.
43. Wire Workflow lifecycle.
44. Wire Prompt lifecycle.
45. Wire Agent lifecycle.
46. Wire Audit lifecycle.
47. Add event dedupe.
48. Add self-write suppression.
49. Implement SESSION_RESULT parser.
50. Enforce CLAIM_ONLY semantics.
51. Implement recent EVENTS tail reader.
52. Add large event-history support.
53. Fix legacy parser.
54. Add 32-field regression.
55. Persist metadata after Git enrichment.
56. Upgrade AI-Commerce-HQ control plane.
57. Upgrade Bulk-Edit.
58. Upgrade fmcg-erp-system.
59. Upgrade FormuLab.
60. Upgrade PackLab.
61. Upgrade PackLab-3D.
62. Upgrade ScrubBots.
63. Upgrade Level Factory.
64. Preserve all canonical task files.
65. Preserve all project governance.
66. Merge/PR without force.
67. Record remote final SHAs.
68. Inspect Registry local paths.
69. Record local status for all eight.
70. Safe fetch clean repos.
71. FF-only clean behind repos when allowed.
72. Do not mutate dirty repos.
73. Do not mutate ahead/diverged repos.
74. Mark non-Git repair.
75. Reconcile safely accessible local roots.
76. Update Command Center precedence.
77. Update Cockpit precedence.
78. Remove adopted fallback authority.
79. Show normalized milestone/cycle/task.
80. Show normalized workflow/next action.
81. Show normalized progress/blockers.
82. Show normalized Git/audit/session.
83. Add all migration fixtures.
84. Add all watcher fixtures.
85. Add scheduler fixtures.
86. Add Git sync fixtures.
87. Add non-Git remote fixtures.
88. Add lifecycle reconciler fixtures.
89. Add SESSION_RESULT fixtures.
90. Add EVENTS tail fixtures.
91. Add legacy parser fixtures.
92. Run focused Rust suites.
93. Run focused frontend suites.
94. Run R82-R85 regression.
95. Run full Rust.
96. Run all-targets.
97. Run pty-support.
98. Run full frontend once green.
99. Run typecheck.
100. Run build.
101. Run npm audit.
102. Run fmt.
103. Run diff check.
104. Perform full adversarial sweep.
105. Fix adjacent BLOCKER/MAJOR in same run.
106. Re-run affected regressions.
107. Run publisher rollback.
108. Governed publish.
109. Verify stable SHA.
110. Native smoke if feasible.
111. Create immutable builder log.
112. Record all UCP findings before/after.
113. Record all eight remote migrations.
114. Record all eight local reconciliation statuses.
115. Record test counts.
116. Record publication SHA.
117. Commit scoped files.
118. Push normally.
119. Verify local/origin equality.
120. Leave M16 OPEN pending independent strict re-audit + owner native acceptance.
121. Do not activate M17.
122. Do not start M21.
123. Reproduce all eight project screenshots before migration.
124. Add exact AI-Commerce-HQ nested-source fixture.
125. Add exact Bulk-Edit canonical-task fixture.
126. Add exact fmcg synthetic-prose-task fixture.
127. Add exact FormuLab nested-tracker fixture.
128. Add exact PackLab known-remote/non-Git fixture.
129. Add exact PackLab3D lowercase-task fixture.
130. Add exact ScrubBots stale-SB-M02-017 fixture.
131. Add exact Pixel Generator screenshot fixture.
132. Add Command Center one-bad-project fault-isolation fixture.
133. Remove portfolio-wide registry-only fallback caused by a single project.
134. Implement one shared current-task resolver for Cockpit + Command Center.
135. Validate project-scoped progress across all eight project shapes.
136. Produce final per-project native acceptance matrix.
137. Re-run full regressions after portfolio-specific fixes.
138. Record final native/owner acceptance pending truthfully.

---

# 26. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16F_REV3_UNIFIED_PROJECT_CONTROL_PLANE_WHOLE_SYSTEM_CLOSURE_REMEDIATION_LOG.md`

Record:

- starting HEAD;
- implementation commits;
- final pushed HEAD;
- UCP-R01 through R12 reproduction and closure;
- final schemas;
- portfolio migration matrix;
- local reconciliation matrix;
- watcher sources;
- scheduler behavior;
- Git sync safety;
- UI actions;
- reconciler lifecycle sources;
- EVENTS tail behavior;
- legacy parser closure;
- test names/counts;
- adversarial sweep;
- publication SHA/size;
- native evidence/limitations.

End with:

`M16F UNIFIED PROJECT CONTROL PLANE WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
