# M18 GitHub Integration V07 — Native GitHub Surface Black-Screen Remediation Prompt

## AUTHORITY

Execute this prompt only after GitHub `main` contains:

- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V06_STRICT_REAUDIT.md` with source verdict PASS;
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V06_OWNER_NATIVE_ACCEPTANCE.md` with owner-native verdict PARTIAL PASS / CHANGES_REQUIRED;
- `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V06_LOG.md` as immutable builder history;
- M18 still OPEN and M19 still blocked.

Close **only**:

- `F-M18-V06-NATIVE-001` — Project Cockpit GitHub integration surface can collapse the whole React/WebView content to black after the loading state.

Do not broaden scope. Do not reopen accepted M00-M17, X03/X04, M18 V03-V06 architecture, or owner-accepted Prompt Engine/Settings/session UX.

`TASKS.md` and `CODEX_ROADMAP.md` are strictly READ-ONLY for Codex. ChatGPT owns all canonical tracker transitions.

## MANDATORY SAFE SYNC

Before reading or modifying implementation files:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If and only if the worktree is clean and local HEAD is strictly behind `origin/main`, update with:

```powershell
git merge --ff-only origin/main
```

Never reset, rebase, force-push, destructively checkout, automatically stash, run `git clean`, or discard owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED` and do not modify implementation files.

At completion, every H!veAI implementation/test/log change must be committed and pushed. Do not report complete unless local HEAD, `origin/main`, and live GitHub `refs/heads/main` are identical and the worktree is clean.

## REQUIRED CONTEXT

Read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md` READ-ONLY
- `CODEX_ROADMAP.md` READ-ONLY
- `docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V06_STRICT_REAUDIT.md`
- `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V06_OWNER_NATIVE_ACCEPTANCE.md`
- `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V06_LOG.md`
- `src/githubIntegration.ts`
- `src/pages.tsx`
- `src/App.tsx`
- `src-tauri/src/github_integration.rs`
- `src-tauri/src/lib.rs`
- existing Project Cockpit and M18-focused tests.

## VERIFIED DEFECT CHAIN

The owner-native failure is reproducible as:

1. open a registered project's Project Cockpit;
2. activate the `GitHub` tab;
3. bounded loading state/spinner appears;
4. when the native snapshot resolves/renders, the application React/WebView surface becomes completely black.

The current source contains a concrete Rust/TypeScript DTO mismatch:

- frontend `GitHubIntegrationSnapshot.reconciliation` expects camelCase keys such as `state`, `localBranch`, `localHead`, `remoteBranch`, `remoteHead`, `localDirty`, and `evidence`;
- Rust `GitHubReconciliation` is annotated with `#[serde(rename_all = "SCREAMING_SNAKE_CASE")]`, which renames **field keys** to `STATE`, `LOCAL_BRANCH`, `EVIDENCE`, etc.;
- `CockpitGitHubIntegration` passes `data.reconciliation.evidence` directly into `CockpitList`;
- `CockpitList` immediately evaluates `values.length`;
- therefore the malformed native payload can produce an uncaught render exception rather than a bounded GitHub-panel failure.

Treat this as the primary defect unless direct source/native evidence disproves it. If another contributing defect is discovered while reproducing this exact path, document it and fix it only when strictly necessary to close the same black-screen failure.

## REQUIRED REMEDIATION

### 1. Correct the Rust/TypeScript reconciliation wire contract

- Serialize `GitHubReconciliation` field **keys** in the exact camelCase contract consumed by `src/githubIntegration.ts`.
- Preserve reconciliation **values** and semantics. State values may remain uppercase domain values such as `ALIGNED`, `DIVERGED`, `REMOTE_UNAVAILABLE`, etc.
- Do not rename domain state values merely to make the UI pass.
- Do not add dual legacy/current field names to hide the mismatch unless a documented backward-compatibility requirement genuinely exists.
- Keep repository/branch identity exact and Registry-scoped.

### 2. Add fail-closed frontend validation at the GitHub panel boundary

Before rendering the current GitHub integration details, validate the minimum runtime shape needed by the panel, especially:

- `repository` object;
- `reconciliation` object;
- `reconciliation.evidence` array;
- bounded arrays used by rendering (`pullRequests`, `issues`, `actions`, `releases`, `tags`, `warnings`, `cache.resources`).

If the payload is malformed or incomplete:

- do not throw;
- do not fabricate an empty/current/healthy state;
- render a bounded `GitHub integration evidence is malformed/unavailable` diagnostic inside the GitHub panel;
- keep the rest of Project Cockpit and the H!veAI shell alive and interactive.

Use explicit guards / normalization only where semantically safe. Do not convert missing evidence to fabricated current evidence.

### 3. Contain render failures to the GitHub integration surface

Add a narrow error-containment boundary around the GitHub integration surface or an equivalent deterministic mechanism so that a future unexpected GitHub-panel render exception cannot blank the entire application shell.

Requirements:

- error containment must be local to the GitHub integration surface, not a blanket mechanism that silently hides arbitrary application failures;
- show a bounded truthful panel-level diagnostic;
- allow the user to leave the GitHub tab / Project Cockpit normally;
- do not auto-reload the whole application;
- do not navigate the WebView externally;
- do not swallow errors without observable diagnostic evidence.

### 4. Preserve loading lifecycle

- Loading state must terminate on both success and failure.
- No permanent spinner or full-screen overlay.
- A failed snapshot must render a bounded panel error.
- Switching project/tab during an in-flight request must not let stale completion overwrite another project's view.
- Existing active-request cleanup must remain effective.

## REQUIRED DIRECT TESTS

### Rust wire-contract tests

Add direct serialization evidence for `GitHubReconciliation` using the production struct and `serde_json`, asserting exact keys:

- `state`
- `localBranch`
- `localHead`
- `remoteBranch`
- `remoteHead`
- `localDirty`
- `evidence`

Assert that incorrect field-key forms such as `STATE`, `LOCAL_BRANCH`, and `EVIDENCE` are absent.

Also preserve reconciliation domain-state values unchanged.

### Mounted frontend Project Cockpit tests

Exercise the actual mounted Project Cockpit / GitHub tab, not helper-only functions.

Required cases:

1. exact valid native-shape snapshot renders repository/reconciliation/PR/Issue/Actions sections and leaves the H!veAI shell mounted;
2. reconciliation evidence renders from camelCase native payload;
3. missing `reconciliation.evidence` fails closed to a bounded GitHub-panel diagnostic, not a thrown test/render exception;
4. malformed `reconciliation` object fails closed;
5. missing/null bounded arrays fail closed or are handled only where semantically safe, without fabricating CURRENT/HEALTHY state;
6. native command rejection renders the existing bounded error state;
7. switch away from GitHub while request is in flight and prove stale completion cannot black/overwrite the new tab;
8. switch project while request is in flight and prove stale project data cannot render under the new project;
9. after a GitHub panel error, navigate to another cockpit tab and prove the shell remains interactive;
10. no same-WebView external navigation or application-wide reload occurs.

### Regression preservation

Prove at minimum:

- exact eight-project portfolio remains exactly eight;
- `Sekiph82/FormuLab@main` remains canonical;
- X04 TASKS-only current-state authority remains intact;
- Command Center current task/workflow/actor remains TASKS-derived;
- Prompt Engine Builder + Sessions integration remains intact;
- no standalone top-level Agents navigation returns;
- Settings `Builder Providers` remains separate from `Codex Audit Provider`;
- nullable `required_actor` behavior remains safe;
- accepted M14/M15/M17 provider/session/provenance behavior remains green;
- accepted V06 sanitizer/cache/Actions/linkage tests remain green.

## SECURITY / NEGATIVE GATES

Do not add:

- GitHub PAT/API-key settings;
- credential/auth-store reads;
- direct OpenAI HTTP audit transport;
- arbitrary frontend-controlled GitHub hosts;
- shell command-string construction;
- automatic push/merge/rebase/reset/checkout/tag/release/issue mutation/workflow retry;
- hidden `.hiveai` current-truth fallback;
- Codex edits to `TASKS.md` or `CODEX_ROADMAP.md`;
- M19 activation;
- standalone top-level Agents workspace/navigation.

Do not weaken V06 sanitizer, request bounds, cache schema, or mutation-denied defaults.

## FULL REQUIRED VERIFICATION

After the targeted fix, run and record exact results for:

- focused Rust GitHub integration tests including the new serialization contract test;
- focused mounted Project Cockpit/GitHub frontend tests;
- V06 sanitizer/cache/Actions/project-owned-linkage regressions;
- X04 TASKS-only regressions;
- M14/M15/M17 Prompt Engine/session/provider regressions;
- normal `cargo test --lib`;
- full frontend test suite;
- `npm run typecheck`;
- `npm run build`;
- `cargo check --manifest-path src-tauri/Cargo.toml`;
- `git diff --check`;
- tracker immutability check for `TASKS.md` and `CODEX_ROADMAP.md`;
- final security/negative searches.

Check GitHub hosted status/check/workflow availability for the implementation commit and record the result without claiming hosted CI when none exists.

## GOVERNED PUBLICATION

Do not publish until every required focused/full verification gate is green.

Then use only:

`scripts/publish-dev-qa.ps1`

Preserve:

- `dev-bin/H!veAI.exe` stable path;
- Desktop `H!veAI.lnk` direct target;
- stable icon contract;
- no dev server;
- no visible terminal/console regression.

Record the final executable SHA-256.

Do not perform owner-native acceptance. Stop for independent V07 source re-audit first.

## REQUIRED BUILDER LOG

Create:

`docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V07_LOG.md`

Record at minimum:

- synchronized starting SHA;
- implementation/test SHA(s);
- exact changed files;
- root-cause confirmation;
- exact Rust serialization before/after contract;
- frontend runtime-shape/error-containment design;
- focused serialization test results;
- mounted Project Cockpit/GitHub test results;
- project/tab stale-request evidence;
- full Rust/frontend/typecheck/build/cargo-check/diff/security results;
- hosted-status observation;
- governed publication result and EXE SHA-256;
- final local HEAD / origin/main / live GitHub main equality and clean worktree.

Historical V03-V06 prompts/logs/audits are immutable. Do not edit them.

## STOP CONDITION

At completion:

- commit and push all implementation/test/log changes;
- keep `TASKS.md` and `CODEX_ROADMAP.md` unchanged;
- keep M18 OPEN;
- keep M19 blocked;
- stop for independent strict V07 source re-audit.

Do not claim M18 PASS/CLOSED and do not perform owner-native acceptance.

## OWNER-FACING FINAL RESPONSE

Return only:

- GitHub V07 log URL/path;
- implementation/test SHA(s);
- builder-log SHA;
- final GitHub main SHA;
- concise status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_V07_STRICT_REAUDIT`.
