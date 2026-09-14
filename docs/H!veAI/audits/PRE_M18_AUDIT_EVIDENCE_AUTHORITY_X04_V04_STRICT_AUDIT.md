# Pre-M18 Audit Evidence Authority X04 V04 — Independent Strict Audit

## 1. Verdict

**FAIL / CHANGES_REQUIRED**

X04 V04 closes the two direct V03 projection leaks in the Project Dashboard and Project Cockpit paths, and it suppresses persisted workflow rows and dashboard materialization inside the per-project ROOT_TASKS Command Center summary path. However, the end-to-end owner contract is still not fully true at the outer Command Center boundary.

Four MAJOR findings remain:

1. global database evidence is still merged into current `Needs Your Attention` / `Active Work Queue` state for ROOT_TASKS local projects and then counted/misattributed as TASKS-derived current truth;
2. local ROOT_TASKS milestone/required-actor/blocker truth is not carried through the Command Center project contract and the frontend still renders those fields through remote-only branches;
3. the Command Center degraded fallback still reintroduces the legacy control-plane as a current-facing reconciliation channel;
4. the V04 deterministic test evidence is incomplete and several existing Rust tests were shortened with unconditional `return;`, leaving dead assertions instead of maintaining a truthful executable regression suite.

M18 must remain blocked. Do not run owner-native X04 acceptance yet.

## 2. Scope

Repository: `Sekiph82/H-veAI`

Branch: `main`

V04 starting SHA:

- `2aa7126aa488d1478dac084839c6fe650e952185`

Builder implementation/test commit:

- `9b61c9b83eef95122ba660c9132cab226206ce6a`

Builder log commits / audited live main before this audit:

- initial log: `6da82f70ba579b2528b45492e6dc073eb9e40938`
- finalized log / live main: `033be3ce2a1ceae20540dcc0885144080c729ffc`

Authoritative prompt:

- `docs/H!veAI/prompts/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V04_CONSUMER_PRESENTATION_REMEDIATION_PROMPT.md`

## 3. Builder-log treatment

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V04_LOG.md` was treated as builder claim evidence only.

The builder reports 490 serialized Rust tests, 142 frontend tests, typecheck, cargo check, production build, `git diff --check`, security review, governed publication, and final local/origin/live equality as passing. GitHub exposes no independent status checks on implementation commit `9b61c9b83eef95122ba660c9132cab226206ce6a`, so aggregate pass counts and native-publication claims remain builder claims pending independent evidence.

Acceptance in this audit is based on live GitHub source, direct test bodies, implementation diff, repository state, and the owner ROOT_TASKS-only current-state contract.

## 4. Accepted closure — Project Dashboard ROOT_TASKS materialization

**ACCEPTED.** `project_dashboard::root_tasks_resolution()` no longer copies `.hiveai/PROJECT_DASHBOARD.md` materialized current-state fields into the public ROOT_TASKS current-state object.

For a ROOT_TASKS local project:

- `tracking_mode` is fixed to `ROOT_TASKS_ONLY`;
- `dashboard_mode` is `ROOT_TASKS_ONLY`;
- `canonical_task_source` is `TASKS.md`;
- `materialized` is `MaterializedDashboardStatus::default()`.

The dashboard may still be parsed for bounded warnings / role context, but its milestone/task/workflow/actor/progress/waiting/current-work/blocker projection values are no longer emitted as current materialization.

The V04 test `resolver_removes_dashboard_current_materialization_from_root_tasks_contract` directly verifies this behavior.

## 5. Accepted closure — Project Cockpit legacy current-state channel

**ACCEPTED.** For local ROOT_TASKS Project Cockpit snapshots, the legacy `control_plane` snapshot is now omitted (`None`) rather than being serialized as a parallel current-state channel.

The V04 backend adversarial test poisons `.hiveai/PROJECT_DASHBOARD.md` and `.hiveai/PROJECT.json`, then verifies:

- `cockpit.control_plane.is_none()`;
- dashboard tracking remains `ROOT_TASKS_ONLY`;
- dashboard materialized state is default/empty;
- poisoned values do not enter project-summary warnings/current operational output.

This closes the direct backend payload portion of F-X04-V03-002.

## 6. Accepted closure — Project Cockpit frontend current fields

**ACCEPTED for the canonical-present case.** The current Project Cockpit overview now reads local current milestone, workflow, required actor, next action, blockers, progress, and current-work title from canonical `snapshot.truth` rather than dashboard materialization or legacy control-plane fallbacks.

The frontend test `renders ROOT_TASKS current state without dashboard or control-plane fallbacks` deliberately poisons dashboard and workflow values and verifies the canonical TASKS-derived title, milestone, and next action render while poison strings do not.

Workflow rows are now labeled as historical evidence in the Tasks / Workflow surfaces rather than being labeled as canonical current state.

## 7. Accepted closure — direct workflow/dashboard operational rows inside `summarize_project`

**ACCEPTED.** For `root_tasks_only` projects, `command_center::summarize_project()` now:

- does not use workflow rows to calculate completion;
- does not choose a current workflow row;
- does not build `last_action` from a workflow row;
- uses TASKS-derived required actor;
- ignores workflow rows when generating per-project attention/work-queue entries;
- does not call `materialized_operational_evidence()`;
- suppresses Project Dashboard authority-warning attention.

The updated `m11a_r01_snapshot_reads_real_m10_workflow_rows_with_bound` test verifies conflicting workflow rows no longer become ROOT_TASKS current attention/queue.

## 8. Finding F-X04-V04-001 — outer Command Center evidence merge still creates non-TASKS current attention/work queue

**MAJOR / OPEN.** V04 fixed the per-project `summarize_project()` loop, but the outer `command_center::snapshot()` still sets `has_legacy_projects = true` for every non-GitHub project and then globally executes:

- `read_activity(...)`;
- `read_evidence_items(...)`;
- `attention.extend(evidence_attention)`;
- `queue.extend(evidence_queue)`.

`read_evidence_items()` creates current `AttentionItem` / `WorkQueueItem` entries from persisted database evidence including:

- failed `test_runs`;
- failed `audits`;
- pending `permission_requests`;
- live `agent_sessions`.

Those are legitimate evidence in their own domains, but V04 requirement 8 was explicit: if Command Center exposes **current attention/work queue for a ROOT_TASKS project**, those current items must be derived from canonical TASKS / `ProjectTruth`, not another database ledger.

## 9. Direct evidence for F-X04-V04-001

The current production code merges `read_evidence_items()` into the same `attention` and `work_queue` arrays rendered by the Command Center as:

- **Needs Your Attention**;
- **Active Work Queue**.

The test `m11a_r06_mixed_evidence_attention_queue_and_activity_are_real_and_bounded` uses a normal fixture that writes repository-root `TASKS.md`, then inserts an agent session, failed audit, failed test, and permission request. The test explicitly expects:

- `TEST_RUN` in `snapshot.attention`;
- `AUDIT` in `snapshot.attention`;
- `PERMISSION` in `snapshot.attention`;
- `agent:session-1` in `snapshot.work_queue`.

Therefore the remaining leak is not theoretical; it is asserted current behavior for a ROOT_TASKS fixture.

The same arrays drive `kpis.needs_attention` and `kpis.running`. The Engineering Brief then labels those aggregate values as `GitHub root TASKS.md workflow` / `GITHUB_TASKS_ONLY`, even though the first attention/queue item may actually be a failed test, audit, permission request, or agent-session row. This is both an authority violation and a provenance error.

Historical test/audit/agent/permission evidence may remain visible in Activity, Audit, Agents, Tests, or another explicitly non-current evidence surface. It may not silently become ROOT_TASKS current project/task/workflow attention or work-queue truth.

## 10. Required closure for F-X04-V04-001

V05 must make the outer portfolio aggregation authority-aware.

For a local ROOT_TASKS project:

1. current `attention` and current `work_queue` must come only from canonical `ProjectTruth` / root `TASKS.md` when enough unambiguous truth exists;
2. failed tests, failed audits, permission requests, agent-session state, immutable workflow history, watcher snapshots, and dashboard projections must remain historical/domain-specific evidence and must not be merged into ROOT_TASKS current attention/queue;
3. `kpis.needs_attention` and `kpis.running` must count the authoritative current items, not mixed evidence ledgers;
4. Engineering Brief facts must use truthful provenance and must never label non-TASKS evidence as `GITHUB_TASKS_ONLY` / root TASKS workflow;
5. GitHub-tracked remote TASKS behavior must remain unchanged.

It is acceptable for historical/domain-specific evidence to remain in `recent_activity` and dedicated detail pages/tabs.

## 11. Finding F-X04-V04-002 — local ROOT_TASKS milestone/actor/blockers are still missing from Command Center current presentation

**MAJOR / OPEN.** V04 corrected Project Cockpit current presentation, but Command Center current project presentation is still remote-shaped.

Rust `ProjectOperationSummary` contains `current_task`, `current_state`, `next_action`, `allowed_actors`, and progress, but it does not expose canonical current milestone or blockers as first-class normalized fields.

The TypeScript `CommandCenterProject` contract likewise has no `currentMilestone`, `requiredActor`, `blockers`, `progressScope`, `authoritySource`, or `reconciliationState` fields even though some of that truth already exists on the Rust side.

`CommandCenterProjectPanel` renders current milestone, required actor, and blockers through `project.githubTracking` / `remote` only:

- milestone: `remote?.currentMilestone ?? "Unavailable"`;
- required actor: `remote?.requiredActor ?? "Unavailable"`;
- workflow-panel required actor: remote-only;
- blockers: `remote?.blockers ?? []` and the empty text says `No remote blockers declared.`

For a local ROOT_TASKS project with a canonical TASKS milestone/actor/blocker, the UI therefore reports those canonical fields as unavailable or absent instead of presenting TASKS truth.

This does not leak legacy state, but it still violates the end-to-end requirement that current-facing Command Center fields use canonical local `ProjectTruth` for local ROOT_TASKS projects.

## 12. Required closure for F-X04-V04-002

Normalize the Command Center project contract so both remote GitHub TASKS and local ROOT_TASKS projects expose the same current-truth fields without frontend authority branching.

At minimum, the normalized current contract should carry canonical:

- current milestone;
- required actor;
- blockers;
- reconciliation state / authority provenance where needed.

For local ROOT_TASKS, populate those fields from `ProjectTruth` only. For GitHub-tracked projects, populate the same normalized fields from remote root `TASKS.md` truth.

The Command Center frontend must render the normalized canonical fields rather than assuming that milestone/actor/blockers exist only under `githubTracking`.

Ambiguous or unavailable TASKS truth must render explicit unavailable/needs-reconciliation, never hidden/control-plane/dashboard/workflow fallback.

## 13. Finding F-X04-V04-003 — degraded Command Center fallback still revives the legacy control plane

**MAJOR / OPEN.** `degraded_project_summary()` still calls `control_plane::snapshot(...)` and emits a current-facing degraded summary with:

- `task_authority = CONTROL_PLANE_RECONCILIATION_REQUIRED`;
- `provenance_mode = CONTROL_PLANE_DEGRADED`;
- `next_action = Reconcile project control plane`;
- serialized `control_plane` summary / `truth_sync` when available.

This helper is used when `summarize_project()` fails. The owner contract requires current-state failures to fail closed around repository-root TASKS truth; an error path must not resurrect the legacy control-plane as the remediation/current-action channel.

A fallback path is still part of the runtime contract. ROOT_TASKS-only authority cannot depend on the happy path succeeding.

## 14. Required closure for F-X04-V04-003

For non-GitHub local projects, degraded Command Center summary must remain ROOT_TASKS-oriented and fail closed.

It must not:

- load or serialize legacy control-plane current state;
- advertise `CONTROL_PLANE_*` as current authority/provenance;
- set `Reconcile project control plane` as current next action.

Use a bounded ROOT_TASKS unavailable/reconciliation representation instead, with empty legacy current-state channels and an action such as retry/restore/reconcile repository-root `TASKS.md` evidence as appropriate to the actual failure.

Add a deterministic test for this degraded/failure path.

## 15. Finding F-X04-V04-004 — required adversarial test matrix is incomplete and existing tests contain deliberate dead code

**MAJOR / OPEN.** The V04 prompt required direct adversarial production-path tests, including a frontend case where canonical TASKS truth is unavailable/ambiguous while legacy projection values are poisoned.

The new frontend poison test covers only the canonical-present case. It does **not** exercise the required unavailable/ambiguous canonical truth case.

More seriously, implementation commit `9b61c9b83eef95122ba660c9132cab226206ce6a` inserts unconditional `return;` statements into multiple existing Command Center Rust tests immediately after new broad assertions. The detailed assertions below those returns are unreachable dead code.

Examples occur in the materialized-dashboard blocker/activity identity, deduplication, collision, and ordering test family. A passing test count cannot be treated as strong regression evidence when assertions remain in source but can no longer execute.

## 16. Required closure for F-X04-V04-004

V05 must restore executable test integrity:

1. remove the unconditional early `return;` statements introduced by V04;
2. delete or rewrite obsolete assertions instead of leaving unreachable code;
3. preserve meaningful helper-level tests for any historical/contextual functionality that remains supported;
4. add the missing frontend unavailable/ambiguous TASKS + poisoned legacy projection test;
5. add a direct Command Center test proving test/audit/permission/agent-session DB evidence remains in historical/domain-specific surfaces but does not become ROOT_TASKS current attention/work queue/KPI truth;
6. add a direct local ROOT_TASKS Command Center render test for canonical milestone/actor/blockers and fail-closed ambiguity;
7. add a degraded-summary/failure-path test proving no control-plane fallback;
8. keep the existing canonical-present poison test.

The final test source should contain no intentionally unreachable assertion blocks.

## 17. Security / scope / accepted-regression boundary

The bounded V04 diff does not modify the Codex/Claude provider implementation, GitHub tracking implementation, FormuLab branch mapping, audit transport, or canonical tracker files.

No evidence was found in the V04 changed-file set of:

- `OPENAI_API_KEY` / direct OpenAI Responses transport;
- `ANTHROPIC_API_KEY` / direct Anthropic transport;
- new provider auth-file inspection;
- GUI/browser automation;
- blanket permission bypass;
- M18 implementation.

The exact eight-project portfolio and `Sekiph82/FormuLab@main` source were outside the V04 implementation diff and must remain preserved by V05.

## 18. Tracker / CI / completion evidence

Comparison from `2aa7126aa488d1478dac084839c6fe650e952185` through live main `033be3ce2a1ceae20540dcc0885144080c729ffc` changes only the V04 log plus:

- `src-tauri/src/command_center.rs`
- `src-tauri/src/project_cockpit.rs`
- `src-tauri/src/project_dashboard.rs`
- `src-tauri/src/watcher.rs`
- `src/pages.tsx`
- `src/projectCockpit.ts`
- `tests/m07.06-focused.test.tsx`
- `tests/m12-project-cockpit-focused.test.tsx`

`TASKS.md` and `CODEX_ROADMAP.md` were not changed by the V04 builder diff. M18 was not activated by the builder.

GitHub exposes no independent commit-status checks on `9b61c9b83eef95122ba660c9132cab226206ce6a`; the 490/142 test counts remain builder claims. The final log states post-log local/origin/live equality and clean worktree, while live GitHub main is independently observed at `033be3ce2a1ceae20540dcc0885144080c729ffc`.

## 19. Native gate

**DO NOT RUN OWNER-NATIVE X04 ACCEPTANCE YET.**

V04 is materially closer, but a native test can still appear correct in Project Cockpit while Command Center mixes database evidence into current attention/work queue, omits local canonical milestone/actor/blockers, or enters the control-plane degraded fallback on an error path.

Owner-native acceptance resumes only after V05 independently closes F-X04-V04-001 through F-X04-V04-004.

## 20. Closure decision

X04 V04 is **CHANGES_REQUIRED**.

Accepted and retained:

- ROOT_TASKS Project Dashboard materialized current-state fields are cleared;
- ROOT_TASKS tracking mode no longer inherits `single-dashboard-watch`;
- local ROOT_TASKS Project Cockpit omits the legacy control-plane current-state snapshot;
- Project Cockpit current fields use canonical TASKS truth;
- persisted workflow/dashboard rows no longer create current items inside the per-project ROOT_TASKS `summarize_project()` path.

Still blocking:

- outer Command Center mixed-evidence current attention/work queue/KPI/brief aggregation;
- missing local canonical milestone/actor/blocker parity in the Command Center project contract/UI;
- legacy control-plane degraded fallback;
- incomplete / partially bypassed deterministic regression tests.

Required next action: execute a narrowly bounded X04 V05 Command Center authority-boundary remediation. Preserve all accepted X04 V02/V03/V04 closures, X03, M00-M17, the Codex-only audit provider, Claude adapter behavior, exact eight-project portfolio, and `Sekiph82/FormuLab@main`. `TASKS.md` and `CODEX_ROADMAP.md` remain READ-ONLY for Codex. M18 remains blocked.