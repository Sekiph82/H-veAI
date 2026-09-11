# M21-R02 Owner Native Acceptance and Retirement Gate V01 Strict Audit

## 1. VERDICT

**FAIL / REMEDIATION_REQUIRED**

The owner performed the required M21 native/visual acceptance on 2026-09-11. Most acceptance criteria passed, but the Project Cockpit `Tasks` tab fails for GitHub-root-`TASKS.md` projects even while the same project's Command Center and Cockpit Overview show correct remote task truth.

The owner also requested retirement of the historical `Sekiph82/AI-Commerce-HQ` GitHub repository and its old local parent tree. That retirement must not be performed yet because current H!veAI production code still contains `Sekiph82/AI-Commerce-HQ` in the default GitHub portfolio target set, and the last independently known active standalone H!veAI checkout remains physically below the historical local parent tree until relocation is separately proven.

This audit opens bounded remediation work item `M21-R02`.

## 2. CONTRACT RECOVERY

M21 owner acceptance required:

- startup video begins immediately;
- no visible terminal/console windows;
- exactly eight projects appear;
- project cards and Cockpit load;
- Cockpit Tasks loads canonical task truth;
- task counts/progress/current/next reflect GitHub root `TASKS.md`;
- no obvious visual regression.

Retirement additionally requires that H!veAI no longer depends on or automatically resurrects the historical AI-Commerce project/repository and that deleting the old local parent cannot delete the active H!veAI checkout.

## 3. BRANCH / HEAD / DIFF SCOPE

Repository: `Sekiph82/H-veAI`

Branch: `main`

Repository HEAD inspected before this audit publication:

`83e9b8036ebf4f371134383127deb0957585f2fc`

This is an independent acceptance audit. No production implementation change is accepted by this file.

## 4. ACCEPTANCE CRITERIA MATRIX

| Requirement | Result | Evidence |
| --- | --- | --- |
| Native H!veAI launch | PASS | Owner native acceptance. |
| Startup video immediate | PASS | Owner native acceptance. |
| No visible terminal flash | PASS | Owner native acceptance. |
| Exactly eight projects | PASS | Owner native acceptance screenshot shows 8 projects. |
| Command Center remote task truth | PASS | ScrubBots current task/count/progress/next action are populated from GitHub root TASKS truth. |
| Project Cockpit Overview remote truth | PASS | ScrubBots Overview shows remote HEAD, milestone, sprint, current task, workflow, actor, next action and progress. |
| Project Cockpit Tasks canonical task rendering | **FAIL** | Tasks tab shows `Task intelligence unavailable`, `No parsed tasks`, `CURRENT Unknown`, and `NEXT Unknown` for the same ScrubBots project. |
| General visual integrity | PASS | Owner reports no other visual issue in this acceptance pass. |
| AI-Commerce absent from current visible portfolio | PASS | Current owner screenshot shows eight projects and no AI-Commerce entry. |
| AI-Commerce absent from default production target definition | **FAIL** | `github_tracking.rs::ensure_portfolio` still defines `Sekiph82/AI-Commerce-HQ` inside `TARGETS`; the current eight-project state depends on exclusion state. |
| Historical local parent safe to delete | **UNVERIFIED / BLOCKED** | Last audited active H!veAI checkout was physically under the historical AI-Commerce parent. No new relocation proof has been provided. |

## 5. BUILDER CLAIMS VS REPOSITORY / OWNER TRUTH

The prior V03 consolidation audit remains valid for preservation/governance closure. It did not assert owner native acceptance.

The owner acceptance now provides direct runtime evidence that Command Center and Cockpit Overview work, but Cockpit Tasks does not. Therefore the owner acceptance gate cannot be marked PASS.

Current source independently confirms a structural reason for the observed failure: GitHub-tasks-only projects are routed through `snapshot_remote_primary(...)`, where `task_intelligence` is explicitly set to `None` and `workflow.tasks` is explicitly initialized empty while remote summary/projection data is populated for Overview.

## 6. FILE / SYMBOL EVIDENCE

### `src-tauri/src/project_cockpit.rs`

`ProjectCockpitSnapshot` exposes both legacy/persisted task intelligence fields and remote-primary fields.

For GitHub-tasks-only projects, `snapshot(...)` dispatches to `snapshot_remote_primary(...)`. That function currently initializes:

- `task_intelligence = None`;
- `task_intelligence_error = None`;
- an empty `WorkflowProjectList`.

This matches the owner-visible discrepancy: Overview can render `remote_primary`, while the Tasks surface has no canonical rows to render.

### `src/projectCockpit.ts`

The frontend snapshot contract contains `taskIntelligence`, `workflow`, `githubTracking`, and `remotePrimary` simultaneously. The UI therefore has multiple task representations, and parity for the GitHub-root-`TASKS.md` authority must be explicit rather than assumed.

### `src-tauri/src/github_tracking.rs`

`ensure_portfolio(...)` currently declares a nine-entry `TARGETS` array including:

`Sekiph82/AI-Commerce-HQ` on branch `H!veAI`.

An exclusion record can hide/skip that target in the current database, which explains the owner's correct visible count of eight. However, a fresh database without that exclusion can attempt to recreate the obsolete project target. This is incompatible with permanent retirement/deletion of the AI-Commerce repository.

## 7. FOCUSED TEST EVIDENCE

No current test evidence overrides the direct runtime failure.

M21-R02 must add focused tests that reproduce the exact mismatch:

1. a GitHub-tasks-only project has populated remote root `TASKS.md` truth while legacy/persisted task intelligence is empty;
2. Cockpit Overview and Cockpit Tasks must both expose/render the canonical remote task truth;
3. counts/current/next/progress for the same remote HEAD must remain consistent;
4. a fresh portfolio initialization must contain exactly the intended eight repositories and must not contain `Sekiph82/AI-Commerce-HQ`;
5. existing explicit-exclusion/history data must not cause unrelated projects to disappear.

## 8. REGRESSION EVIDENCE

Owner acceptance confirms that startup, terminal suppression, eight-project display, Command Center, Cockpit Overview and general visual presentation remain working. M21-R02 must preserve these behaviors.

## 9. SECURITY / SAFETY REVIEW

No local parent directory or GitHub repository deletion is authorized during M21-R02 implementation.

Do not delete, move, reset, clean, force-push, or rewrite any unrelated repository or preserved owner data. The old AI-Commerce repository may be retired only after H!veAI runtime no longer targets it and the active standalone checkout is proven outside the local parent intended for deletion.

## 10. ARCHITECTURE CONSISTENCY

The observed Tasks-tab behavior violates the post-M21 architecture in which GitHub repository metadata plus root `TASKS.md` are the canonical project truth for tracked repositories.

A GitHub-tasks-only project must not display an unavailable/empty task view merely because legacy persisted task-intelligence rows are absent when valid canonical remote task rows exist.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Current root `TASKS.md` still states that M21-R01 is PASS/CLOSED and owner acceptance is pending. This became stale when the owner acceptance produced a concrete failure.

M21-R02 implementation must update current Project Status truth to reflect the opened remediation and later close it only after independent audit plus owner re-acceptance.

Historical logs/audits remain immutable.

## 12. FINAL REPOSITORY STATE

Before publishing this audit, GitHub `main` was `83e9b8036ebf4f371134383127deb0957585f2fc`.

The audit publication commit will become the newer remote HEAD.

## 13. OPEN CROSS-MILESTONE FINDINGS

- M21-R02-F01: Project Cockpit Tasks does not surface canonical GitHub root `TASKS.md` task truth for remote-primary projects.
- M21-R02-F02: the obsolete `Sekiph82/AI-Commerce-HQ` repository remains in the production default portfolio target definition.
- M21-R02-F03: active H!veAI relocation outside the historical local parent is not yet independently proven, so local parent deletion remains blocked.

## 14. DEFECTS BY SEVERITY

- **MAJOR — M21-R02-F01:** Cockpit Tasks canonical truth parity failure.
- **MAJOR — M21-R02-F02:** obsolete AI-Commerce repository remains a default production portfolio target.
- **BLOCKER FOR LOCAL DELETION — M21-R02-F03:** active checkout relocation outside the deletion parent is unverified.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Prefer one normalized remote task-row representation shared by Command Center and Project Cockpit rather than fabricating UI-only fallback data. Avoid reintroducing `.hiveai`, `PROJECT.json`, or local filesystem task files as primary authority.

## 16. UNVERIFIED ITEMS

- exact current physical path of the active H!veAI standalone checkout after the owner's latest acceptance;
- current Desktop shortcut target path;
- whether the historical local parent now contains anything not already preserved outside it.

## 17. REGRESSION RISK

**MEDIUM**

The fix touches project-truth presentation and portfolio initialization. Focused parity and fresh-database tests are required.

## 18. AUDIT CONFIDENCE

**HIGH** for the Tasks defect and default-target defect because owner runtime evidence and current source independently agree.

**MEDIUM** for local deletion readiness because the current host filesystem cannot be independently inspected from GitHub.

## 19. FINAL VERDICT

**FAIL / REMEDIATION_REQUIRED**

M21 owner native/visual acceptance is not complete. M21-R02 must close the Cockpit Tasks parity defect and permanently remove AI-Commerce-HQ from the default H!veAI portfolio architecture before repository retirement. Local parent deletion additionally requires proven relocation of the active H!veAI checkout outside that parent.

## 20. REQUIRED REMEDIATION

Create and execute the bounded English Codex prompt:

`docs/H!veAI/prompts/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V01_PROMPT.md`

Expected builder log:

`docs/H!veAI/codex-logs/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V01_LOG.md`

Do not delete the local historical parent or the GitHub `Sekiph82/AI-Commerce-HQ` repository as part of this implementation cycle.