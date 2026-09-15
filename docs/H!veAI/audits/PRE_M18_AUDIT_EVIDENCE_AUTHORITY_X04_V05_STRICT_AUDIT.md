# Pre-M18 Audit Evidence Authority X04 V05 — Independent Strict Audit

## 1. Verdict

**PASS / SOURCE ACCEPTED / OWNER-NATIVE ACCEPTANCE REQUIRED BEFORE X04 CLOSURE**

X04 V05 closes the four blocking findings from the V04 strict audit. Repository-root `TASKS.md` is now the end-to-end current project/task/workflow authority for local ROOT_TASKS projects across the Command Center current summary, current attention/work queue, KPI/brief counts, degraded/failure behavior, and current-facing frontend fields. GitHub-tracked projects continue to use tracked-branch root `TASKS.md` truth. M18 remains blocked until owner-native acceptance.

## 2. Scope

Repository: `Sekiph82/H-veAI`

Branch: `main`

Authoritative failed audit:

- `docs/H!veAI/audits/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V04_STRICT_AUDIT.md`

Authoritative remediation prompt:

- `docs/H!veAI/prompts/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V05_COMMAND_CENTER_AUTHORITY_BOUNDARY_REMEDIATION_PROMPT.md`

Builder implementation/test commit:

- `d370a0642868b3f6f5203153d96d70bd12db4db4`

Builder log commit/live main before this audit:

- `89110209345cdd57a2b61b64c62bc12340716453`

## 3. Builder-log treatment

`docs/H!veAI/codex-logs/PRE_M18_AUDIT_EVIDENCE_AUTHORITY_X04_V05_LOG.md` was treated as builder claim evidence only.

The builder reports 494 serialized Rust tests, 29 focused Command Center Rust tests, 43 focused frontend tests, 143 full frontend tests, typecheck, cargo check, production build, diff check, governed publication, and a clean final synchronization state. GitHub exposes no independent status checks on the implementation commit, so aggregate test/publication counts remain claims rather than independent acceptance evidence.

## 4. F-X04-V04-001 — current versus historical evidence partition

**CLOSED.**

`command_center::snapshot()` no longer merges `read_evidence_items()` into the current `attention` and `work_queue` arrays for local projects. Failed tests, failed audits, pending permissions, and live agent sessions therefore no longer become current TASKS truth.

Historical/domain evidence remains available through bounded Recent Activity and the dedicated Project Cockpit evidence surfaces.

Current attention and current work queue for local ROOT_TASKS projects are now produced by `root_tasks_current_items()` from canonical `ProjectTruth` only.

## 5. Current KPI and Engineering Brief truth

**ACCEPTED.**

`kpis.needs_attention` and `kpis.running` now count only the authoritative current attention/queue arrays. The Engineering Brief uses authority-neutral TASKS provenance rather than labeling database evidence as `GITHUB_TASKS_ONLY`.

The mixed-evidence production test explicitly seeds workflow, agent, audit, test, permission, Git, and watcher rows and proves TEST_RUN/AUDIT/PERMISSION/agent items stay out of current attention/work queue while historical activity remains visible.

## 6. F-X04-V04-002 — normalized local/remote current contract

**CLOSED.**

`ProjectOperationSummary` and the TypeScript `CommandCenterProject` contract now carry normalized current fields including:

- current milestone;
- required actor;
- blockers;
- progress scope;
- authority source;
- provenance;
- reconciliation state.

Local ROOT_TASKS values come from canonical `ProjectTruth`; remote values come from tracked-branch root `TASKS.md` tracking.

## 7. Command Center frontend parity

**ACCEPTED.**

The Command Center project panel now renders normalized `project.currentMilestone`, `project.requiredActor`, `project.blockers`, `project.currentState`, `project.nextAction`, and normalized progress fields instead of using `githubTracking` as the current-state fallback.

Remote repository/branch/HEAD metadata remains separately available as remote-source metadata.

## 8. Unavailable/poisoned projection behavior

**ACCEPTED.**

The frontend focused test includes a local project whose canonical current fields are unavailable while `githubTracking` contains deliberately poisoned milestone/workflow/actor/blocker strings. The current UI renders unavailable/needs-reconciliation semantics and does not render the poisoned strings.

This directly covers the missing V04 unavailable/ambiguous frontend poison case.

## 9. F-X04-V04-003 — degraded fail-closed behavior

**CLOSED.**

`degraded_project_summary()` no longer calls `control_plane::snapshot()` and no longer emits `CONTROL_PLANE_RECONCILIATION_REQUIRED`, `CONTROL_PLANE_DEGRADED`, or `Reconcile project control plane`.

The degraded summary now emits ROOT_TASKS-unavailable semantics, `NEEDS_RECONCILIATION`, no canonical current task/milestone/actor/blocker/progress values, and the bounded next action `Restore a readable repository-root TASKS.md`.

## 10. Degraded current attention behavior

**ACCEPTED.**

When local summary resolution fails, the generated current attention item is categorized as `ROOT_TASKS`, not `CONTROL_PLANE`. The current queue remains empty rather than being synthesized from legacy state.

## 11. F-X04-V04-004 — executable test integrity

**CLOSED.**

The unconditional early `return;` statements introduced by V04 were removed from the affected Command Center authority tests. Obsolete assertions were rewritten or removed rather than left unreachable.

The current source contains direct executable assertions for hidden dashboard exclusion, mixed database evidence partitioning, canonical running truth, canonical blocked truth, degraded ROOT_TASKS behavior, and normalized current fields.

## 12. Adversarial current-item coverage

**ACCEPTED.**

`x04_v05_root_tasks_normalizes_current_fields_and_authoritative_items` proves a canonical `[~]` TASKS task produces normalized milestone/actor/blocker/next-action truth and exactly one TASKS-derived current work-queue item.

`x04_v05_root_tasks_blocked_truth_is_the_only_current_attention` proves a canonical `[!]` TASKS task produces the sole current attention item, with the TASKS blocker as detail and no fabricated current queue.

## 13. Historical evidence separation

**ACCEPTED.**

The mixed-evidence test still observes WORKFLOW, AGENT_EVENT, AGENT_SESSION, AUDIT, TEST_RUN, GIT_SNAPSHOT, and PROJECT_SNAPSHOT entries in Recent Activity, while those records do not become current attention/work-queue truth.

This satisfies the owner requirement that historical/domain evidence remain visible without becoming a second current-state ledger.

## 14. Project Dashboard / Project Cockpit accepted closures preserved

**ACCEPTED.**

V04 behavior remains intact:

- ROOT_TASKS Project Dashboard current materialization remains default/empty;
- ROOT_TASKS tracking remains `ROOT_TASKS_ONLY` rather than `single-dashboard-watch`;
- local ROOT_TASKS Project Cockpit omits the legacy control-plane current-state channel;
- Project Cockpit current-facing fields remain TASKS-derived;
- legacy workflow records remain historical evidence only.

## 15. GitHub remote behavior

**ACCEPTED.**

The normalized contract is also populated from the existing remote root-TASKS snapshot for GitHub-tracked projects. Remote current milestone, actor, blockers, progress, task, state, and next action are copied into the same normalized current fields without changing the accepted remote tracking architecture.

## 16. Security and scope boundary

No implementation diff evidence shows M18 activation, Anthropic API-key/HTTP transport, blanket permission bypass, new provider redesign, or mutation of registered external project repositories.

The V05 compare range changes only H!veAI implementation/tests plus the required V05 builder log. `TASKS.md` and `CODEX_ROADMAP.md` were not changed by Codex, preserving tracker-transition ownership.

## 17. Minor non-blocking contract/copy notes

Two non-blocking cleanup notes remain for later housekeeping:

1. the TypeScript `TaskAuthority`/`health` string unions do not enumerate every runtime fail-closed value such as `ROOT_TASKS_UNAVAILABLE` / `NEEDS_RECONCILIATION`, although runtime rendering remains truthful because Tauri JSON is not runtime-discriminated by those unions;
2. a few Command Center descriptive labels still use remote-centric wording such as `Remote tracker exact counts` even when a mixed/local portfolio is displayed.

These do not reintroduce a competing authority or change the current-state values, and they are not blockers for X04 owner-native acceptance.

## 18. Publication and synchronization evidence limitation

GitHub exposes no independent CI/status check on `d370a0642868b3f6f5203153d96d70bd12db4db4`.

The V05 builder log reports the published executable SHA-256 as `CA06024E2AC561ED1A6BCA469DA3F98F6850A1019149382E2BC49FD26D761761`, the same value reported by the preceding V04 builder log despite V05 changing both Rust and frontend production code. This may be stale/copied log evidence or a publication freshness problem; it is not sufficient to prove which binary is currently launched.

The publication script itself rebuilds `src-tauri/target/release/hiveai-desktop.exe`, smoke-tests a staged candidate, swaps it into `dev-bin/H!veAI.exe`, and verifies candidate/stable hash equality. Therefore source architecture is acceptable, but owner-native acceptance must prove that the launched desktop behavior contains the V05 changes before X04 closes.

The V05 log also states final equality verification in future-tense rather than recording the final SHAs after the log commit. Live GitHub main is nevertheless the V05 log commit. Final local/origin/live equality remains builder claim evidence.

## 19. Required owner-native acceptance gate

Do not close X04 or activate M18 until the owner verifies the published desktop app.

Use Bulk-Edit as the preferred read-only fixture because its root `TASKS.md` currently declares:

- Current Milestone: `M13`;
- Current Sprint: `M13.03`;
- Current Task: `M13.03 — Etsy listing video upload workflow`;
- Current Task Status: `BLOCKED`;
- Required Actor: `OWNER`;
- Workflow State: `BLOCKED_ON_OWNER_ACCEPTANCE`;
- explicit current blockers and a TASKS-defined next action.

Native acceptance should prove that Command Center/Project Cockpit current fields reflect those root-TASKS values, that old `.hiveai`/PROJECT/STATE/HANDOFF/dashboard/workflow projections do not appear as current truth, and that a fresh Bulk-Edit audit does not request `.hiveai/PROJECT.json` as current authority.

## 20. Closure decision

X04 V05 is **SOURCE PASS**.

Severity after remediation:

- BLOCKER: 0
- MAJOR: 0
- MINOR: 2 non-blocking cleanup notes

F-X04-V04-001 through F-X04-V04-004 are closed at source level. X04 remains **OPEN only for owner-native acceptance**. M18 remains blocked. Canonical tracker transition remains owned by ChatGPT and must occur only after the owner-native gate passes.