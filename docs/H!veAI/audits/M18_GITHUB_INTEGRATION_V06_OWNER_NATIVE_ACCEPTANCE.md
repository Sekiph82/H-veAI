# M18 GitHub Integration V06 Owner-Native Acceptance

## Verdict

**PARTIAL PASS / CHANGES_REQUIRED**

M18 V06 passed the independent source-level strict re-audit, and the owner-native acceptance evidence confirms the majority of the milestone behaves correctly in the governed Windows build. One native release-blocking defect remains: opening the Project Cockpit `GitHub` integration surface causes the H!veAI React/WebView content to become completely black after the bounded-loading spinner.

M18 remains OPEN. M19 remains blocked.

## Evidence basis

This acceptance record is based on the owner's 28 native screenshots from the governed Windows application plus the owner's direct report of the GitHub-tab failure. It is owner-native evidence, not an independent machine reproduction.

## Accepted native surfaces

The owner-native screenshots accept the following V04-V06 behavior:

- top-level `Agents` navigation is removed;
- Prompt Engine exposes `Prompt Builder` and `Sessions` as one integrated workspace;
- Prompt Builder loads project/task context without the historical nullable `required_actor` SQLite exception;
- prompt generation, exact-version approval, provider selection, and dispatch controls render correctly;
- Prompt Engine Sessions lists persisted sessions and opens session detail in the integrated surface;
- provider failures remain truthful, including a Claude weekly-limit failure shown as a failed persisted session rather than a fabricated success;
- Settings exposes `Builder Providers` separately from `Codex Audit Provider`;
- Codex Audit Provider remains READY with the managed ChatGPT login;
- exact eight-project portfolio remains visible;
- duplicate normalized-path registration fails safely without mutating project files;
- Command Center continues to show GitHub/root-TASKS-only current truth for registered projects;
- accepted sidebar, shell, background, project cards, and Prompt Engine presentation remain intact.

## F-M18-V06-NATIVE-001 — BLOCKER — Project Cockpit GitHub surface blacks out the whole app

### Owner-observed behavior

1. Open a registered project from Projects / Project Cockpit.
2. Activate the `GitHub` cockpit tab.
3. H!veAI first shows the bounded GitHub loading spinner.
4. After the snapshot resolves/renders, the entire application content becomes black. The native window frame remains visible, but the React application surface is gone.
5. The app must be recovered by leaving/restarting rather than by a bounded panel-level error state.

### Source-level root cause

The frontend contract in `src/githubIntegration.ts` expects `GitHubIntegrationSnapshot.reconciliation` fields in camelCase, including:

- `state`
- `localBranch`
- `localHead`
- `remoteBranch`
- `remoteHead`
- `localDirty`
- `evidence`

`CockpitGitHubIntegration` passes `data.reconciliation.evidence` directly into `CockpitList`. `CockpitList` assumes an array and immediately evaluates `values.length`.

The Rust DTO `GitHubReconciliation` in `src-tauri/src/github_integration.rs` is incorrectly annotated with:

```rust
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
```

That renames field **keys** to values such as `STATE`, `LOCAL_BRANCH`, and `EVIDENCE`, while the TypeScript/native UI contract expects camelCase keys. Therefore `data.reconciliation.evidence` is `undefined` in the native payload and the render path can throw when `CockpitList` reads `values.length`. H!veAI currently has no route/panel error boundary that converts this render exception into a bounded GitHub-panel error, so the React surface can collapse to the observed black WebView.

This is an end-to-end native contract defect, not a failure of the accepted TASKS-only authority architecture.

## Required closure

The remediation must:

1. align the Rust/TypeScript reconciliation serialization contract without changing the semantic reconciliation-state values;
2. prove the real serialized Rust DTO uses the frontend contract keys;
3. make the GitHub cockpit presentation fail closed if malformed/partial payload data nevertheless reaches the UI;
4. prevent one GitHub integration panel render failure from blanking the entire application;
5. preserve all accepted V06 GitHub security, cache, Actions, linkage, provider/session, exact-eight-project, FormuLab-main, and TASKS-only behavior;
6. republish only after focused and full regressions are green;
7. obtain owner-native re-acceptance by opening the GitHub surface for multiple registered projects and then navigating back to other H!veAI surfaces without a black screen.

## Native re-test gate after source PASS

After independent V07 source audit PASS, owner should test at minimum:

- H!veAI Project Cockpit -> GitHub;
- Bulk-Edit Project Cockpit -> GitHub;
- one additional GitHub-tracked project -> GitHub;
- return to Command Center, Prompt Engine, and Settings after each GitHub visit.

Expected result: GitHub evidence or a bounded panel-level unavailable/error state renders; the H!veAI application shell remains alive and interactive throughout.

## Milestone state

- M18 V06 source audit: **PASS**.
- M18 V06 owner-native acceptance: **PARTIAL PASS / CHANGES_REQUIRED**.
- Open blocker: **F-M18-V06-NATIVE-001** only.
- M18: **OPEN / ACTIVE**.
- M19: **BLOCKED**.
