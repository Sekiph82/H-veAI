# M15D Post-Dispatch Result Placement UX Strict Re-Audit

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Audited implementation commit: `41b7f48100af2fe43665d2d85f600c14c59513a6`
Audited branch HEAD: `b7b49551f0be60fe82cf012ab155725ebf88bacd`

## Verdict

**TECHNICAL PASS / USER VISUAL PLACEMENT ACCEPTANCE REQUIRED**

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- M15D result-placement remediation: PASS technically

M15C handoff behavior is already user accepted.
M15D changes only the successful post-dispatch result placement.
M15 remains OPEN until the user confirms the new local placement is visually correct.
M16 remains blocked.
M21 remains planned/not started.
Strict completed roadmap progress remains `15 / 20 = 75%` until that final visual confirmation.

## Evidence independently reviewed

- M15D authoritative remediation prompt.
- Immutable builder log:
  `docs/H!veAI/codex-logs/M15D_POST_DISPATCH_RESULT_PLACEMENT_UX_REMEDIATION_LOG.md`
- Actual implementation commit:
  `41b7f48100af2fe43665d2d85f600c14c59513a6`
- Current branch HEAD:
  `b7b49551f0be60fe82cf012ab155725ebf88bacd`
- `src/PromptEnginePage.tsx`
- `src/styles.css`
- `tests/m15c-post-dispatch-handoff-focused.test.tsx`
- Roadmap/task documentation changes in the implementation diff.

## Placement change

Before M15D:

- successful dispatch text was rendered in the page-top/global `.safe-notice.prompt-notice`;
- `View result in Agents` was attached to that global notice.

After M15D:

- generic page notices remain global;
- dispatch success uses dedicated `dispatchNotice` state;
- the success surface renders as `.prompt-dispatch-result`;
- it is inserted immediately after `.prompt-dispatch-row` inside the Provider and dispatch panel;
- the exact `View result in Agents` action is rendered inside that same local result surface;
- no duplicate success result remains at page top.

The implementation therefore matches the requested hierarchy:

1. Codex / Claude provider controls
2. Dispatch action
3. Local dispatch result
4. `View result in Agents`
5. Version history / provenance

## Behavioral preservation

The remediation does not change provider dispatch semantics.

The following remain preserved:

- exact project/session handoff target from M15C;
- navigation-only `View result in Agents` behavior;
- no redispatch on handoff click;
- stale target clearing;
- M15A durable single-use dispatch;
- M15B narrow ACL;
- explicit Codex / Claude provider selection;
- project-neutral task picker behavior;
- M14E dedicated final assistant response;
- no shell/process expansion;
- governed publication.

The success target is cleared before generate/edit/approve/dispatch replacement actions and when the project changes, preventing stale local result handoffs.

## Focused tests

The implementation test suite directly verifies:

- successful CODEX dispatch result is contained inside `.prompt-dispatch-panel`;
- successful CLAUDE dispatch uses the same local surface;
- `.prompt-dispatch-result` is the immediate sibling after `.prompt-dispatch-row`;
- the old page-top `.safe-notice.prompt-notice` success surface is absent after dispatch;
- `View result in Agents` remains present;
- clicking it does not increase the `hiveai_prompt_dispatch` call count;
- exact project/session route targeting remains intact;
- stale handoff clears on new draft;
- stale handoff clears on project change;
- wrong-project session targeting is rejected;
- invalid session targeting fails safely;
- polling/manual selection behavior remains preserved.

## Regression and publication evidence

Builder evidence reports:

- full frontend: 121/121 PASS;
- focused Rust Prompt Engine: 10/10 PASS;
- full serialized Rust: 343/343 PASS;
- M15A race/replay/failure coverage: PASS;
- M15B ACL/capability coverage: 2/2 PASS;
- M14E final-response coverage: 4/4 PASS;
- TypeScript typecheck: PASS;
- frontend production build: PASS;
- npm audit high severity: 0 vulnerabilities;
- Rust fmt/all-targets/pty-support: PASS;
- `git diff --check`: PASS;
- publisher rollback harness: 9/9 PASS;
- governed stable publication: PASS;
- stable/candidate EXE SHA-256 equality:
  `04A5674614F97180C053E107DAFDECC6444FAAEF305370397D5B6E55618399A1`;
- expected PE/icon/stable target checks: PASS;
- governed no-console smoke path: PASS.

## Provenance

Implementation commit:

`41b7f48100af2fe43665d2d85f600c14c59513a6`

Log publication HEAD:

`b7b49551f0be60fe82cf012ab155725ebf88bacd`

The branch HEAD directly parents the implementation commit through the log publication commit. Provenance is coherent.

## Final acceptance gate

Only one visual confirmation remains:

1. Open the published stable H!veAI.
2. Prompt Engine -> Generate -> Approve -> Dispatch to Codex or Claude.
3. Confirm the successful dispatch message is **not** at the page top.
4. Confirm the successful dispatch message and `View result in Agents` button appear directly under the Provider and dispatch controls.
5. Confirm the result block is visually readable and does not interfere with Version history and provenance.
6. Confirm `View result in Agents` still opens the exact dispatched session.

The handoff behavior itself was already accepted in M15C, so no duplicate behavioral acceptance is required beyond verifying it was not visually/regressively broken.

If the user confirms this placement, M15 may be declared **PASS/CLOSED**, progress advances to `16 / 20 = 80%`, and M16 may be activated.

## Final boundary

M14: PASS/CLOSED.

M15A: PASS on technical evidence.

M15B: PASS + user native accepted.

M15C: PASS + user native accepted.

M15D: TECHNICAL PASS, pending visual placement confirmation only.

M15: OPEN pending that single confirmation.

M16-M20: blocked/planned.

M21: planned/not started.
