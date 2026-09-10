# M16E + Unified Project Control Plane + Live Project Synchronization — Consolidated Implementation Prompt

Date: 2026-09-08  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous implementation run.

It combines:

1. the remaining whole-M16 release-gate remediation discovered after M16D; and
2. the new owner-directed H!veAI product foundation: **Unified Project Control Plane + live project synchronization across every registered project**.

Do not stop after the M16 fixes.
Do not stop after the control-plane schema.
Do not ask for a new prompt between phases.

Finish all phases, all focused tests, full regression, governed publication, immutable builder log, commit, and push.

Do not activate M17.
Do not start M21.
Do not silently rewrite roadmap completion state.
M16 remains OPEN until independent whole-M16 re-audit + user native/visual acceptance.

---

# 1. Product direction authority

The owner has explicitly clarified H!veAI's primary product goal:

> H!veAI is the single project command center for all tracked projects. When the owner opens the laptop and H!veAI, every project must show its current real state. Clicking a project must let the owner continue from where that project actually stopped.

Prompt Engine, Audit Engine, Agents, Git Engine, Tasks, Activity and other modules are supporting engines.

The core user outcome is:

`Open H!veAI → see all projects live → click a project → see exact current task/workflow/progress/Git/audit/session state → continue work`.

The current screenshots prove the existing source/manifest/fallback system is not sufficient because different projects currently show inconsistent truth:

- stale current task;
- `FALLBACK_M08_M09`;
- `MANIFEST`;
- `CANONICAL`;
- `MALFORMED`;
- `PARTIAL`;
- `ABSENT`;
- `front-matter field limit reached (32)`;
- workflow unavailable;
- PackLab and Pixel Art Generator shown as non-Git despite having GitHub repositories;
- Project Cockpit tabs that look clickable but do not function as real navigation;
- Command Center and Project Cockpit can represent different current truth.

Treat these as one architectural problem: **project state normalization and synchronization**.

---

# 2. Required reading before edits

Read completely:

1. `H!veAI/GPT.md`
2. M16 unified implementation prompt
3. all M16A/B/C/D prompts, logs and strict audits
4. `H!veAI/docs/H!veAI/audits/M16D_INDEPENDENT_WHOLE_M16_STRICT_REAUDIT.md`
5. current Project Registry / Project Dashboard / Task Sources / Workflow / Watcher / Git Engine / Command Center / Project Cockpit production source
6. current direct test bodies for those modules
7. current control-plane bootstrap files under `H!veAI/.hiveai/`
8. the registered-project database schema/migrations
9. current stable publisher and rollback harness

Builder logs remain claims only.

---

# 3. Phase A — close the remaining M16 release-gate findings

Close all together:

- M16-R82 BLOCKER
- M16-R83 MAJOR
- M16-R84 MINOR
- M16-R85 MINOR

## 3.1 R82 — final semantic validation after prior-finding inheritance

Current defect:

The structured model evaluation is semantically validated before prior OPEN findings are inherited/applied. A model PASS can therefore be accepted first, then an inherited OPEN MAJOR/BLOCKER is appended later in persistence without re-running the final verdict guard.

Required invariant:

> No persisted PASS may contain an unresolved lifecycle OPEN BLOCKER or MAJOR, including inherited prior findings.

Required implementation:

- prior disposition/inheritance must be materialized before final verdict admissibility; or
- run the deterministic semantic guard again after prior findings are materialized and before any audit row/children are committed.

Also ensure:

- unresolved inherited `blocks_release=true` prevents PASS;
- STILL_OPEN remains lifecycle OPEN;
- omission never closes;
- degraded UNAVAILABLE/MALFORMED behavior from R62 remains truthful;
- STALE still takes precedence and applies zero prior dispositions.

Add direct tests that inspect the persisted verdict, not only finding status.

## 3.2 R83 — bounded staged/commit source extraction

Do not use the generic non-draining `run_git` path to load potentially large staged or committed file blobs.

For STAGED and COMMIT_RANGE source evidence:

- stream child stdout while process is alive;
- enforce fixed scan/capture budgets;
- retain only the needed bounded source window;
- return explicit TRUNCATED/UNAVAILABLE evidence when source cannot be fully/cleanly inspected;
- never silently omit a changed source file because `git show` exceeded a pipe/output budget;
- adversarially inspect remaining `run_git` call sites that can produce repository-sized output.

Preserve full Git change identity separately from bounded model/display evidence.

## 3.3 R84 — builder log relevance

Remove hard-coded preference for historical names such as `M16C_REV2`.

Selection order must be provenance-driven:

1. exact audited prompt/session/task/cycle linkage;
2. exact active milestone/task identity;
3. newest relevant artifact;
4. newest project log fallback.

Builder claims remain CLAIM_ONLY.

## 3.4 R85 — deterministic frontend suite

The full frontend suite must pass once, completely, without accepting an isolated retry as closure evidence.

Find and fix the Project Cockpit timing/race test or the underlying async state race.

---

# 4. Phase B — define the H!veAI Project Control Plane v1

Create one explicit versioned specification in the H!veAI repository.

Required canonical template for every tracked project:

```text
<project-root>/
├── <canonical task ledger declared by PROJECT.json>
├── AGENTS.md
├── CLAUDE.md
└── .hiveai/
    ├── PROJECT.json
    ├── RULES.md
    ├── STATE.json
    ├── HANDOFF.md
    ├── EVENTS.jsonl
    ├── SESSION_RESULT.json        # latest external-provider claim, optional
    ├── PROJECT_DASHBOARD.md       # compatibility pointer only
    ├── prompts/
    ├── logs/
    └── audits/
```

Do not require each repository to rename its existing canonical task ledger.

Examples:

- `TASKS.md`
- `tasks.md`
- `docs/FORMULAB_V1_TASK_TRACKER.md`
- `H!veAI/TASKS.md`

The exact canonical task path is declared in `.hiveai/PROJECT.json`.

## 4.1 PROJECT.json

Versioned static project-control contract.

Required fields include:

- schema;
- projectKey;
- displayName;
- repository owner/name;
- tracked branch;
- canonicalTaskSource;
- rulesSource;
- stateSource;
- handoffSource;
- eventSource;
- standard artifact directories;
- local-path registry identity where appropriate;
- refresh policy;
- Git policy;
- governance/mutation policy;
- optional project-specific context sources.

No current task/progress snapshot belongs here.

## 4.2 RULES.md

One shared semantic contract across Codex, Claude, ChatGPT and H!veAI, while preserving stricter project-specific governance.

Critical rule:

**Shared state semantics do not imply identical file-write permissions.**

Some repositories explicitly reserve:

- task closure;
- handoff mutation;
- independent audit creation;

to the owner or independent auditor.

Do not override those rules.

Define an actor-aware permission model, for example:

- OWNER
- HIVEAI_SYSTEM
- CODEX
- CLAUDE
- CHATGPT
- INDEPENDENT_AUDITOR

`PROJECT.json` may tighten which actor can mutate canonical task state, HANDOFF, audits, prompts, etc.

Provider-specific AGENTS/CLAUDE instructions may be stricter but may not redefine shared state enums/schema.

## 4.3 STATE.json

This is the normalized materialized state consumed by H!veAI.

Required fields:

- schema;
- projectKey;
- syncStatus;
- projectStatus;
- currentMilestone;
- currentSprint/cycle;
- currentTaskId;
- currentTaskTitle;
- workflowState;
- requiredActor;
- nextAction;
- blockers;
- progress completed/total/percent;
- Git branch/HEAD/dirty/ahead/behind/diverged;
- lastAudit verdict/id;
- lastAgentSession provider/id/status;
- updatedAt;
- updatedBy;
- evidence/provenance references.

Use one workflow enum:

- IDLE
- READY
- IN_PROGRESS
- AWAITING_AUDIT
- CHANGES_REQUIRED
- BLOCKED
- WAITING_OWNER
- COMPLETE

Never infer current task by scanning random historical prose when STATE + canonical task source provide authority.

## 4.4 HANDOFF.md

One human-readable resume pointer.

Must answer immediately:

- where are we?
- what is current?
- who acts next?
- what is the next action?
- what is blocked?
- what was the last audit/session?
- what canonical task source owns truth?

Do not turn HANDOFF into a second detailed task ledger.

## 4.5 EVENTS.jsonl

Append-only lifecycle stream.

Normalize event types such as:

- TASK_STARTED
- TASK_COMPLETED
- TASK_REOPENED
- WORKFLOW_CHANGED
- AUDIT_STARTED
- AUDIT_PASS
- AUDIT_FAIL
- REMEDIATION_STARTED
- REMEDIATION_COMPLETED
- AGENT_SESSION_STARTED
- AGENT_SESSION_COMPLETED
- BLOCKED
- UNBLOCKED
- OWNER_ACCEPTED
- RELEASED
- GIT_SYNCED
- GIT_SYNC_BLOCKED

Each line is one bounded JSON object containing projectKey, eventId, timestamp, actor, taskId, workflowState, short factual summary and optional commit/audit/session identifiers.

Never rewrite historical lines.

## 4.6 SESSION_RESULT.json

For external Codex/Claude/ChatGPT sessions that were not launched through H!veAI, support one normalized provider result handoff.

It is a **claim**, not independent truth.

The file may contain:

- provider;
- session ID;
- task ID;
- claimed outcome;
- requested workflow transition;
- changed files;
- test summary;
- commit;
- next action;
- timestamp.

H!veAI reconciles this claim against Git/canonical task/governance evidence before changing materialized state.

When a session is launched by H!veAI, native Agent Session Center data is stronger provenance and this file may be unnecessary.

---

# 5. Phase C — replace legacy Dashboard/Fallback state inference

The old `.hiveai/PROJECT_DASHBOARD.md` must no longer be the primary live-state source.

Keep it only as a compatibility pointer map during migration.

## 5.1 Parser correction

The current malformed behavior such as:

`front-matter field limit reached (32)`

must be removed as a source of false project health.

If legacy front matter is still parsed:

- parse only explicitly delimited `---` YAML/front-matter region;
- never treat arbitrary body lines containing `:` as front-matter fields;
- bound safely;
- surface exact parse diagnostics.

## 5.2 Authority precedence

For adopted projects:

1. Registry project identity
2. `.hiveai/PROJECT.json`
3. canonical task source
4. `.hiveai/STATE.json`
5. `.hiveai/HANDOFF.md`
6. Git Engine live state
7. native Agent/Audit/Prompt persisted state
8. EVENTS history

Legacy M08/M09 dashboard fallback is allowed only for truly unmigrated projects.

Once a project adopts control-plane v1, do not display `FALLBACK_M08_M09` as its normal authority.

## 5.3 One-time reconciliation

Create a safe Project State Reconciler that can import current truth from:

- canonical task source;
- existing legacy PROJECT_DASHBOARD;
- existing handoff/cycle index;
- current Git;
- persisted H!veAI task/workflow/session/audit state.

It writes/repairs normalized STATE/HANDOFF without silently changing accepted task completion.

When evidence conflicts, show `NEEDS_RECONCILIATION` with explicit sources instead of inventing current task.

---

# 6. Phase D — event-driven live synchronization

The owner wants project state updated when something changes, not after manually rebuilding a dashboard.

Implement watcher-first synchronization.

## 6.1 Watch set per project

Watch:

- canonical task source from PROJECT.json;
- PROJECT.json;
- RULES.md;
- STATE.json;
- HANDOFF.md;
- EVENTS.jsonl;
- SESSION_RESULT.json if present;
- `.git/HEAD`;
- `.git/index`;
- relevant refs;
- standard prompt/log/audit directories;
- project-specific watcher sources explicitly declared by PROJECT.json.

## 6.2 Refresh timing

Primary:

- filesystem event-driven;
- debounce approximately 500 ms;
- coalesce duplicate events;
- recompute only affected project.

Safety net:

- reconciliation every 60 seconds.

Do not poll every second.

## 6.3 H!veAI internal events

Changes made through:

- Tasks;
- Prompt Engine;
- Agent sessions;
- Audit Center;
- workflow commands;

must immediately invalidate/reconcile the affected Project Cockpit and Command Center state.

Do not wait for a disk watcher if H!veAI already knows a state transition happened.

## 6.4 Remote Git synchronization

Filesystem watchers cannot see GitHub-only changes until local Git knows about them.

Implement safe remote reconciliation:

- periodic/background `git fetch --prune` at a reasonable bounded cadence;
- compute ahead/behind/diverged;
- if local branch is clean, has an upstream, ahead=0, and remote is a strict fast-forward:
  - H!veAI may perform an owner-enabled safe auto-fast-forward;
- never auto-reset;
- never auto-rebase;
- never auto-stash;
- never discard dirty/untracked work;
- never merge divergence automatically.

If dirty/ahead/diverged:

- show `SYNC_ATTENTION`;
- display exact reason;
- require explicit user action.

Add Settings toggle:

`Safe auto-fast-forward clean tracked projects`

Default may be ON for the owner's current local single-user setup only if all safety guards above are implemented and clearly visible.

---

# 7. Phase E — local Git repository identity and repair

Fix the screenshot case where a registered folder has a known GitHub repository but local `.git` is missing.

Do not collapse these into one `Non-Git folder / No remote detected` statement.

Represent separately:

- Remote repository: CONNECTED / UNKNOWN
- Local Git repository: CONNECTED / NOT_CONNECTED / INVALID
- Branch: actual branch or unavailable
- Upstream: actual tracking ref or unavailable

For a project with repository metadata in Registry but no local `.git`:

show:

`Remote: Sekiph82/PackLab`
`Local Git: Not connected`

and similarly for ScrubBots Level Factory / Pixel Art Generator.

Add an explicit **Repair / Connect local Git** workflow.

Safety rules:

- never run destructive Git operations;
- never overwrite an existing non-empty local folder automatically;
- never `git init` and force-connect an arbitrary populated folder without proof;
- offer a safe clone-to-new-folder / backup-and-replace path;
- validate repository owner/name;
- validate expected canonical path;
- preserve local-only data;
- show exact actions before execution.

---

# 8. Phase F — Project Cockpit and Command Center UX repair

Use the owner's screenshots as native acceptance requirements.

## 8.1 Command Center

Every project card/list selection must show live normalized state:

- current milestone/sprint/cycle;
- current task;
- workflow state;
- next action;
- progress;
- health;
- Git status;
- last audit;
- last agent session.

No stale M08/M09 fallback after control-plane adoption.

## 8.2 Project Cockpit

Current task must come from reconciled current state, not “first open task in a giant task file” or historical prose.

ScrubBots must not show an old M02 task if the canonical current work is later.

PackLab, PackLab3D, FormuLab, FMCG, Bulk Edit, Pixel Generator, AI-Commerce-HQ must all use the same projection rules.

## 8.3 Tabs

The visible:

- Cockpit
- Tasks
- Workflow
- Audit
- Logs

tabs must be real interactive controls.

At minimum:

- Cockpit → overview
- Tasks → canonical task view
- Workflow → current workflow/cycle state
- Audit → project-filtered Audit Center
- Logs → project activity/agent/build log view

Keyboard accessible.
Selected state obvious.
No dead text that visually impersonates navigation.

## 8.4 Health

Health must report actionable states such as:

- HEALTHY
- NEEDS_RECONCILIATION
- SYNC_ATTENTION
- LOCAL_GIT_NOT_CONNECTED
- BLOCKED
- MALFORMED_CONTROL_PLANE

Do not call an intentionally missing optional legacy manifest a project failure after v1 adoption.

---

# 9. Phase G — standardize all currently registered projects safely

The currently tracked portfolio contains eight projects.

Use H!veAI Registry as the local-path authority, not guessed paths.

Expected GitHub identities include:

- `Sekiph82/AI-Commerce-HQ`
- `Sekiph82/Bulk-Edit`
- `Sekiph82/fmcg-erp-system`
- `Sekiph82/FormuLab`
- `Sekiph82/PackLab`
- `Sekiph82/PackLab-3D`
- `Sekiph82/Scrubbots`
- `Sekiph82/ScrubBots-Level-Factory`

Remote control-plane bootstrap commits/merges already exist. Do not rewrite or discard them.

Perform a one-time local adoption/reconciliation for every safely accessible registered local repository.

Rules:

1. record project local path;
2. record Git status before touching files;
3. if Git repo is dirty, never pull/reset automatically;
4. if clean, fetch and fast-forward only;
5. if local folder is not Git, mark it for explicit safe repair rather than inventing repository state;
6. preserve project-specific governance files;
7. do not overwrite existing canonical task truth;
8. migrate old H!veAI state non-destructively;
9. keep historical audits/logs/prompts;
10. never delete project-specific docs merely for symmetry.

The goal is identical **control-plane semantics and file roles**, not deleting useful project documentation.

---

# 10. Phase H — provider-neutral AI behavior

Make the control-plane contract provider-neutral.

Codex, Claude, ChatGPT and future providers must all receive the same state semantics.

AGENTS.md and CLAUDE.md adapters should say:

- read PROJECT/RULES/STATE/HANDOFF/canonical task source;
- obey project-specific stricter governance;
- do not invent task closure;
- return normalized session result facts.

Do not force builders to mutate files they are forbidden to own.

If launched through H!veAI:

- H!veAI captures final assistant response;
- H!veAI writes/reconciles session state itself.

If launched externally:

- provider may write/update SESSION_RESULT.json if allowed;
- otherwise final response must include the normalized result block that H!veAI can import manually.

Define one provider-neutral final result schema.

---

# 11. Data ownership and anti-loop rules

Avoid watcher feedback storms.

If H!veAI writes STATE.json:

- tag writer/revision;
- suppress self-generated duplicate watcher loops;
- one logical state transition should create one materialized state update and one event.

Do not commit generated state every few seconds.

Only meaningful lifecycle transitions belong in tracked history.

Live Git dirtiness/ahead/behind can remain runtime registry state and need not create a Git commit for every refresh.

Clearly separate:

- tracked durable state;
- runtime ephemeral Git telemetry.

---

# 12. Migration / DB requirements

Add additive migrations as required.

Persist normalized project-control metadata without duplicating canonical task content.

Consider fields/tables for:

- control-plane schema/version;
- current normalized project state;
- last reconciliation source hashes;
- last filesystem event;
- safe auto-sync setting;
- remote sync status;
- local Git connection state;
- SESSION_RESULT import provenance.

No destructive reset.

Existing projects and history must remain readable.

---

# 13. Tests

Add direct fixtures for all eight major project-shape families.

At minimum:

1. canonical `TASKS.md`;
2. lowercase `tasks.md`;
3. nested FormuLab task tracker path;
4. existing handoff/cycle-index governance;
5. no legacy dashboard;
6. malformed old dashboard;
7. oversized historical dashboard;
8. non-Git local folder with known remote;
9. clean Git repo behind upstream;
10. dirty Git repo behind upstream;
11. ahead local branch;
12. diverged branch;
13. filesystem task change triggers one project refresh;
14. STATE change triggers Command Center refresh;
15. Audit completion triggers project refresh;
16. Agent completion triggers project refresh;
17. Prompt remediation creation does not falsely close task;
18. provider SESSION_RESULT is claim-only until reconciled;
19. project-specific governance prevents unauthorized task closure;
20. old front-matter body colon lines do not count as front-matter;
21. cockpit tabs navigate correctly;
22. selected project retains correct state;
23. two projects with different task file names project to same normalized UI;
24. remote repository shown even when local Git is missing;
25. safe FF only when clean + behind-only;
26. no auto mutation when dirty/ahead/diverged;
27. watcher self-write does not infinite-loop;
28. 60-second safety reconciliation is idempotent.

Also retain all M16 direct regression suites, especially R59-R85.

---

# 14. Full regression and publication

Run:

- all M16 focused Rust tests;
- all Git Engine tests;
- Project Registry tests;
- Project Dashboard parser tests;
- task-source/task-intelligence tests;
- watcher tests;
- workflow tests;
- Project Cockpit frontend tests;
- Command Center frontend tests;
- Audit Center frontend tests;
- Agent Session Center tests;
- Prompt Engine tests;
- full serialized Rust suite;
- full frontend suite with zero flaky/retry-dependent failures;
- TypeScript typecheck;
- production frontend build;
- npm audit high;
- Rust fmt;
- Rust all-targets;
- pty-support suite;
- git diff --check;
- publisher rollback harness;
- governed stable publication;
- candidate/stable SHA equality;
- PE/shortcut/icon/startup/no-console checks.

---

# 15. Native acceptance preparation

If native UI access is available, verify at least:

1. all eight projects load;
2. adopted projects no longer show legacy fallback authority as normal state;
3. ScrubBots does not show stale M02 task;
4. PackLab remote repo is visible even if local Git needs repair;
5. Pixel Art Generator remote repo is visible even if local Git needs repair;
6. Project Cockpit tabs work;
7. editing a watched task/state file updates H!veAI without restarting;
8. Agent completion updates project state;
9. Audit completion updates project state;
10. safe Git sync state is visible.

If native automation is unavailable, record that honestly. User remains final visual authority.

---

# 16. Final adversarial sweep

Before writing the builder log, inspect the entire changed surface for:

- stale/fallback authority leaks;
- duplicated task ledgers;
- project-specific governance being overwritten;
- watcher races/feedback loops;
- non-Git path confusion;
- unsafe auto-pull/reset/rebase/stash;
- remote/local repository identity mismatch;
- cross-project state leakage;
- task-source case sensitivity;
- malformed/huge control-plane files;
- provider claims treated as acceptance;
- H!veAI internal event not invalidating UI;
- frontend tabs that look clickable but are not;
- persisted M16 PASS with inherited OPEN MAJOR/BLOCKER;
- staged/commit large source Git pipe deadlock;
- full-suite flakiness.

If any additional BLOCKER/MAJOR is found, fix it in the same run and add a direct regression.

Do not stop for another prompt.

---

# 17. Explicit execution gates

1. Read GPT.md.
2. Fetch origin/H!veAI.
3. Fast-forward-only synchronize.
4. Confirm branch H!veAI.
5. Record starting HEAD/worktree.
6. Preserve unrelated files.
7. Read all M16 authority.
8. Reproduce R82.
9. Reproduce R83.
10. Reproduce R84.
11. Reproduce R85.
12. Fix R82.
13. Add persisted PASS + inherited MAJOR test.
14. Add inherited BLOCKER test.
15. Add STILL_OPEN test.
16. Fix R83 bounded Git blob reading.
17. Add large staged source test.
18. Add large commit-range source test.
19. Search large-output run_git call sites.
20. Fix R84 provenance-driven log selection.
21. Fix R85 deterministic frontend race.
22. Run focused M16 tests.
23. Write Project Control Plane v1 spec.
24. Define PROJECT.json schema.
25. Define RULES actor model.
26. Define STATE.json schema.
27. Define HANDOFF format.
28. Define EVENTS JSONL schema.
29. Define SESSION_RESULT schema.
30. Define provider-neutral result contract.
31. Add versioned template directory.
32. Refactor dashboard compatibility parser.
33. Enforce explicit front-matter delimiters.
34. Remove body-colon false parsing.
35. Add malformed/oversized parser fixtures.
36. Implement authority precedence.
37. Add adopted-v1 detection.
38. Disable legacy fallback for adopted projects.
39. Implement one-time reconciler.
40. Reconcile canonical task source.
41. Reconcile existing handoff.
42. Reconcile existing cycle index.
43. Reconcile Git.
44. Reconcile persisted H!veAI workflow.
45. Reconcile sessions.
46. Reconcile audits.
47. Emit NEEDS_RECONCILIATION on conflict.
48. Implement per-project watcher set.
49. Add 500ms debounce.
50. Coalesce duplicate events.
51. Add affected-project-only refresh.
52. Add 60-second safety reconciliation.
53. Wire Tasks mutation invalidation.
54. Wire Prompt Engine invalidation.
55. Wire Agent invalidation.
56. Wire Audit invalidation.
57. Wire workflow invalidation.
58. Implement bounded remote fetch scheduler.
59. Compute ahead/behind/diverged.
60. Add safe-auto-FF setting.
61. Require clean branch.
62. Require upstream.
63. Require ahead=0.
64. Require behind>0.
65. Require no divergence.
66. Use FF-only update.
67. Never reset.
68. Never rebase.
69. Never auto-stash.
70. Never discard untracked.
71. Emit sync-attention reason.
72. Separate remote repo state from local Git state.
73. Show known remote for non-Git local folder.
74. Implement safe repair/connect workflow.
75. Refuse destructive populated-folder attachment.
76. Add backup/clone plan preview.
77. Validate repo owner/name.
78. Validate target path.
79. Update Projects card UI.
80. Update Command Center normalized projection.
81. Update Project Cockpit normalized projection.
82. Remove stale current-task heuristic.
83. Show current milestone.
84. Show current sprint/cycle.
85. Show current task.
86. Show workflow.
87. Show next action.
88. Show progress.
89. Show Git.
90. Show last audit.
91. Show last agent session.
92. Implement Cockpit tab navigation.
93. Implement Tasks tab.
94. Implement Workflow tab.
95. Implement Audit tab.
96. Implement Logs tab.
97. Make tabs keyboard accessible.
98. Add project-filtered navigation.
99. Add normalized health enum.
100. Replace malformed optional legacy-manifest health logic.
101. Add control-plane adoption UI/state.
102. Read Registry list of eight projects.
103. Record each local path.
104. Record each repository identity.
105. Record each tracked branch.
106. Safe-fetch each Git repo when clean.
107. Never pull dirty repos.
108. Mark non-Git folders for repair.
109. Preserve canonical task files.
110. Preserve historical audits/logs/prompts.
111. Preserve stricter project governance.
112. Reconcile state non-destructively.
113. Initialize missing PROJECT.json.
114. Initialize missing RULES.md.
115. Initialize missing STATE.json.
116. Initialize missing HANDOFF only if absent; otherwise migrate truthfully.
117. Initialize EVENTS.jsonl append-only.
118. Initialize standard artifact dirs.
119. Prepend/merge AGENTS adapter without deleting existing content.
120. Prepend/merge CLAUDE adapter without deleting existing content.
121. Do not duplicate adapters.
122. Add actor permission tests.
123. Add provider-neutral result tests.
124. Add external SESSION_RESULT tests.
125. Add internal Agent Session reconciliation tests.
126. Add AI final-response contract docs.
127. Add watcher task-change test.
128. Add watcher state-change test.
129. Add watcher self-loop suppression test.
130. Add reconciliation idempotency test.
131. Add clean-behind auto-FF test.
132. Add dirty-behind refusal test.
133. Add ahead refusal test.
134. Add divergence refusal test.
135. Add non-Git known-remote test.
136. Add wrong-remote test.
137. Add PackLab-style fixture.
138. Add Pixel Generator governance fixture.
139. Add ScrubBots legacy-dashboard fixture.
140. Add Bulk-Edit large-task fixture.
141. Add FormuLab nested-tracker fixture.
142. Add AI-Commerce-HQ nested-H!veAI task fixture.
143. Add FMCG TASKS fixture.
144. Add PackLab3D lowercase-tasks fixture.
145. Run focused control-plane Rust tests.
146. Run focused watcher tests.
147. Run focused Project Cockpit tests.
148. Run focused Command Center tests.
149. Run full M16 regression.
150. Run all Git tests.
151. Run task intelligence regression.
152. Run workflow regression.
153. Run Agent regression.
154. Run Prompt regression.
155. Run Audit regression.
156. Run full serialized Rust suite.
157. Run full frontend suite once with zero failures.
158. Run typecheck.
159. Run production build.
160. Run npm audit high.
161. Run Rust fmt.
162. Run all-targets.
163. Run pty-support.
164. Run git diff --check.
165. Inspect migration SQL.
166. Inspect ACL changes.
167. Inspect final source directly.
168. Inspect new tests directly.
169. Perform final adversarial sweep.
170. Fix any adjacent BLOCKER/MAJOR same run.
171. Run regressions again after adjacent fixes.
172. Run publisher rollback harness.
173. Governed no-bundle release build.
174. Publish stable EXE.
175. Verify candidate/stable SHA.
176. Verify PE.
177. Verify shortcut/icon/startup.
178. Verify no console popup.
179. Native smoke if feasible.
180. Create immutable builder log.
181. Record M16 R82-R85 before/after.
182. Record control-plane schemas.
183. Record migration behavior.
184. Record all eight project adoption results.
185. Record repositories blocked by dirty/non-Git local state.
186. Record watcher/refresh evidence.
187. Record safe Git sync evidence.
188. Record screenshot-issue before/after mapping.
189. Record test counts.
190. Record publication SHA/size.
191. Record implementation commit(s).
192. Commit scoped files only.
193. Push normally.
194. Verify local/origin equality.
195. Leave M16 OPEN pending independent whole-M16 strict re-audit.
196. Leave control-plane native acceptance pending owner screenshots.
197. Do not activate M17.
198. Do not start M21.

---

# 18. Required artifacts

Create/update versioned documentation for:

- Project Control Plane v1 specification;
- schemas/examples/templates;
- migration/adoption rules;
- provider-neutral result contract;
- safe Git sync policy.

Create immutable builder log exactly:

`H!veAI/docs/H!veAI/codex-logs/M16E_AND_UNIFIED_PROJECT_CONTROL_PLANE_LIVE_SYNC_IMPLEMENTATION_LOG.md`

The log must include:

- starting HEAD;
- exact M16 R82-R85 reproduction and closure evidence;
- control-plane architecture;
- watcher architecture;
- remote/local Git reconciliation;
- per-project adoption status;
- screenshot issue mapping;
- focused/full tests;
- final adversarial sweep;
- native limitations;
- publication SHA;
- implementation commits;
- final pushed HEAD.

End with truthful state:

`M16E + UNIFIED PROJECT CONTROL PLANE IMPLEMENTATION COMPLETE / PENDING INDEPENDENT WHOLE-M16 STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
