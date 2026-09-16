# M18 GitHub Integration V07 — Independent Strict Re-Audit

## 01. Audit identity

- Milestone: M18 GitHub Integration
- Remediation generation: V07
- Closed finding target: `F-M18-V06-NATIVE-001`
- Builder implementation/test commit: `5039ed53ba5f7b03ac6ee93f1cc0034e0432d33e`
- Builder-log commit: `cce06bdf18939117dc6dffb9f0f9fa0fc7654add`
- Builder log: `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V07_LOG.md`
- Authoritative prompt: `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V07_NATIVE_GITHUB_BLACK_SCREEN_REMEDIATION_PROMPT.md`
- Prior owner-native acceptance: `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V06_OWNER_NATIVE_ACCEPTANCE.md`

## 02. Verdict

**PASS — SOURCE_ACCEPTED / OWNER_NATIVE_REACCEPTANCE_REQUIRED**

V07 closes `F-M18-V06-NATIVE-001` at source/test-contract level. The concrete Rust/TypeScript reconciliation wire mismatch is corrected, malformed GitHub integration data is now guarded before normal rendering, and an unexpected GitHub-panel render exception is contained locally rather than being allowed to collapse the full H!veAI React/WebView surface.

No new BLOCKER, MAJOR, or MINOR source defect was found in the bounded V07 change set.

M18 is **not PASS/CLOSED yet**. The remaining gate is the owner-native multi-project GitHub surface re-test defined by the accepted V06 owner-native record. M19 remains blocked until that gate passes and ChatGPT performs the canonical tracker transition.

## 03. Severity summary

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- Closed V06 native blocker: 1 / 1

## 04. Scope and change-boundary verification

The V07 range from prompt commit `c4841ba299eaef41b51f78986b6209d314762b46` through builder-log commit `cce06bdf18939117dc6dffb9f0f9fa0fc7654add` contains exactly two builder commits:

1. `5039ed53ba5f7b03ac6ee93f1cc0034e0432d33e` — implementation/tests;
2. `cce06bdf18939117dc6dffb9f0f9fa0fc7654add` — immutable builder log.

The changed files are bounded to:

- `src-tauri/src/github_integration.rs`
- `src/githubIntegration.ts`
- `src/pages.tsx`
- `tests/m12-project-cockpit-focused.test.tsx`
- `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V07_LOG.md`

`TASKS.md` and `CODEX_ROADMAP.md` are absent from the Codex diff. No M19 implementation is present.

## 05. Root-cause confirmation

**CONFIRMED.**

Before V07, production Rust `GitHubReconciliation` used `#[serde(rename_all = "SCREAMING_SNAKE_CASE")]`, while the TypeScript contract consumed camelCase keys including `state`, `localBranch`, `localHead`, `remoteBranch`, `remoteHead`, `localDirty`, and `evidence`.

The GitHub cockpit renderer passed `data.reconciliation.evidence` into list rendering. The native payload therefore could omit the expected camelCase `evidence` property, producing the owner-observed loading-spinner-to-black-screen failure through an uncaught render exception.

The V07 change directly addresses that verified chain rather than masking it with fabricated fallback data.

## 06. Rust reconciliation wire contract

**PASS.**

Production `GitHubReconciliation` now uses:

```rust
#[serde(rename_all = "camelCase")]
```

The struct continues to preserve the same semantic fields and domain-state values. This changes field-key serialization only; values such as `DIVERGED`, `LOCAL_AHEAD`, or other reconciliation domain states remain unchanged.

No dual legacy/current reconciliation key family was added.

## 07. Direct serialization evidence

**PASS.**

A direct production-struct `serde_json::to_value` test asserts all seven exact frontend keys:

- `state`
- `localBranch`
- `localHead`
- `remoteBranch`
- `remoteHead`
- `localDirty`
- `evidence`

The same test asserts incorrect historical forms `STATE`, `LOCAL_BRANCH`, and `EVIDENCE` are absent and confirms the uppercase domain value `DIVERGED` is preserved.

This is direct contract evidence against the exact V06 native root cause.

## 08. Runtime-shape validation boundary

**PASS.**

`src/githubIntegration.ts` now validates the native snapshot as runtime data rather than trusting TypeScript compile-time shape alone. The validator covers repository identity, reconciliation, local evidence, cache/mutation policy, warnings, and all bounded render collections used by the GitHub surface.

In particular, `reconciliation.evidence` must be a bounded string array. Missing evidence, malformed reconciliation state, null/invalid bounded arrays, malformed nested PR/Issue/Actions data, and over-limit collections do not get normalized into fabricated healthy/current evidence.

## 09. Project identity confinement

**PASS.**

The mounted GitHub surface additionally verifies the returned `projectId` against the currently requested registered project before accepting the snapshot for rendering. Existing project-scoped native command routing remains unchanged.

V07 does not broaden repository/host selection and does not introduce arbitrary frontend-controlled GitHub hosts.

## 10. Panel-local error containment

**PASS.**

A dedicated `GitHubIntegrationErrorBoundary` wraps only the Project Cockpit GitHub surface. Unexpected render exceptions resolve to the bounded panel-level diagnostic:

`GitHub integration evidence is malformed/unavailable`

The boundary does not wrap unrelated application surfaces, does not reload the application, and does not navigate the WebView externally. The Project Cockpit shell remains outside this boundary and therefore remains mounted and navigable.

The boundary resets naturally on unmount/re-entry and explicitly resets when the project identity changes.

## 11. Loading/failure lifecycle

**PASS.**

The GitHub panel still clears previous data/error state, starts loading only for a native request, ends loading through `finally`, and uses the existing active-request cleanup guard.

Malformed payloads become a bounded panel error. Native command rejections remain visible as bounded error diagnostics. No permanent spinner or application-wide overlay was introduced.

## 12. Stale tab/project completion safety

**PASS at source/test-contract level.**

Mounted tests exercise both required stale-completion classes:

- leaving the GitHub tab while the request is pending, then resolving the old request;
- changing from Project Alpha to Project Beta while Alpha's GitHub request remains pending, then resolving Alpha later.

The active cleanup guard prevents stale completion from rendering over the current tab/project.

## 13. Mounted valid GitHub rendering evidence

**PASS at source/test-contract level.**

The mounted Project Cockpit fixture uses a complete native-shaped snapshot and verifies repository identity, reconciliation state, PR surface, mutation-denied text, and project-scoped invocation. The fixture includes branches, commits, PRs, issues, Actions/jobs/steps, releases, tags, local Git evidence, reconciliation evidence, cache metadata, mutation policy, and warnings.

This is not a helper-only validator test; it exercises the actual mounted Project Cockpit GitHub tab.

## 14. Mounted malformed/error evidence

**PASS at source/test-contract level.**

The mounted suite verifies:

- missing `reconciliation.evidence`;
- a malformed/null bounded `actions` collection;
- native snapshot rejection;
- an intentionally thrown unexpected GitHub-panel render exception.

In the render-exception fixture the Project Cockpit heading remains mounted and the route remains `/projects/alpha`, directly testing the V07 containment objective.

## 15. Navigation after failure

**PASS at source/test-contract level.**

After malformed GitHub evidence, the mounted test navigates from GitHub to the Tasks tab and confirms the `Canonical tasks` surface renders. This establishes that the shell remains interactive after a GitHub-panel failure rather than collapsing into the previous black WebView state.

No application-wide reload or same-WebView external navigation was added by V07.

## 16. Regression preservation

**PASS at change-boundary/source level.**

V07 does not modify portfolio registration, root-TASKS authority, Command Center current-truth resolution, Prompt Engine Builder/Sessions architecture, Settings Builder Providers, the Codex-only audit provider, Claude adapter behavior, or M14/M15/M17 provenance/session implementation.

The exact accepted V06 architecture is therefore not reopened by this bounded diff. `TASKS.md` still declares M18 active and M19 blocked; canonical trackers were correctly left untouched by Codex pending owner-native acceptance and ChatGPT transition ownership.

The builder log reports focused M12/M13/M14/M15C/M17 and V06 GitHub regressions green. Those execution counts remain builder claims rather than independent hosted-CI evidence.

## 17. Verification-evidence assessment

The builder log reports:

- focused Rust GitHub integration: `20 passed; 0 failed`;
- mounted Project Cockpit/GitHub: `15 passed; 0 failed`;
- focused M12/M13/M14/M15C/M17 frontend regression: `49 passed; 0 failed`;
- full frontend: `18 files; 156 tests passed; 0 failed`;
- normal parallel Rust library gate: `516 passed; 0 failed`;
- `npm run typecheck`: passed;
- `npm run build`: passed;
- `cargo check --manifest-path src-tauri/Cargo.toml`: passed;
- `git diff --check`: passed;
- tracker immutability check: passed.

These console results are not independently re-executed by this audit and are therefore treated as builder evidence. Independent acceptance here rests on the inspected production source, direct test bodies, bounded diff, and repository state.

## 18. Hosted status and security boundary

Independent GitHub inspection of implementation commit `5039ed53ba5f7b03ac6ee93f1cc0034e0432d33e` returns zero commit statuses and zero workflow runs. No hosted-green-CI claim is made.

The V07 diff adds no PAT/API-key setting, credential/auth-store read, direct OpenAI HTTP audit transport, shell command-string construction, GitHub mutation fallback, hidden `.hiveai` current-truth fallback, standalone top-level Agents navigation, application reload, external WebView navigation, or M19 activation.

GitHub integration remains observational/read-only under the accepted V06 mutation boundary.

## 19. Publication and repository state

The builder log reports governed publication through `scripts/publish-dev-qa.ps1` after local gates passed, with stable executable:

`dev-bin/H!veAI.exe`

and SHA-256:

`98238D769C0A88D802D877694DAB71CAAA20213814E36CC6710EE5F1FF8C47F9`

Independent GitHub inspection confirms the builder range ends at `cce06bdf18939117dc6dffb9f0f9fa0fc7654add` before this audit record, with implementation/test commit as its direct parent. Canonical trackers were not modified by the builder.

## 20. Acceptance decision and next gate

**Independent V07 source decision: PASS.**

`F-M18-V06-NATIVE-001` is source-closed. No V08 remediation is required from the inspected source/test contract.

The remaining required gate is the narrowly scoped owner-native re-acceptance already defined by the V06 owner-native record:

1. open **H!veAI Project Cockpit → GitHub**;
2. open **Bulk-Edit Project Cockpit → GitHub**;
3. open **one additional GitHub-tracked project → GitHub**;
4. after each visit, navigate back to **Command Center**, **Prompt Engine**, and **Settings** as practical and confirm the shell remains alive.

Expected result: each GitHub surface renders repository evidence or a bounded panel-local unavailable/error state; no full black React/WebView surface occurs.

If that owner-native re-test passes, ChatGPT should record final M18 owner-native acceptance and then perform the canonical `TASKS.md` / `CODEX_ROADMAP.md` transition to close M18 and activate M19. Codex must not perform that transition.