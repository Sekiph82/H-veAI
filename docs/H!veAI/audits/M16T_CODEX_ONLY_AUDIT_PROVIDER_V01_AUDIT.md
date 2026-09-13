# M16T Codex-Only Audit Provider V01 Strict Audit

## 1. VERDICT

**CHANGES_REQUIRED**

The owner-directed provider architecture is materially implemented: the active OpenAI HTTP/API-key audit path is removed, the direct `reqwest` dependency was removed from H!veAI, the production audit boundary is now Codex CLI plus truthful unavailable fallback, and the active Settings surface describes local Codex CLI/managed ChatGPT login rather than API-key configuration.

However, two acceptance defects prevent PASS:

1. **MAJOR — bounded Codex stdout/stderr capture stops draining once the retention cap is reached.** A sufficiently verbose Codex `--json` stream can fill the OS pipe after the reader exits, block the child before it writes/exits cleanly, and cause a false timeout even though the dedicated final-message file is intended to be authoritative.
2. **MAJOR — canonical current tracking is internally contradictory.** The top of root `TASKS.md` still says `IMPLEMENTATION_IN_PROGRESS`, `Required Actor: CODEX`, and an implementation action, while deeper current truth and the immutable builder log say implementation is complete and awaiting independent audit/owner acceptance. `CODEX_ROADMAP.md` likewise still labels M16T active with Required Actor CODEX.

No OpenAI API-key provider remediation is required. M16T remains the correct architecture and M17 remains blocked.

## 2. CONTRACT RECOVERY

M16T V01 is a new architecture work item, not an M16S retry. Its binding owner decision is:

- no `OPENAI_API_KEY` product path;
- no direct H!veAI `api.openai.com` audit execution/readiness;
- no hidden OpenAI HTTP fallback;
- production model-backed audit provider is local Codex CLI using Codex-managed ChatGPT authentication;
- H!veAI must not read Codex auth files/tokens;
- audit execution is headless, bounded, read-only, ephemeral and schema-constrained;
- existing Codex runtime/process foundations should be reused rather than duplicated;
- M17/Claude must not be activated during M16T;
- completion requires independent strict audit plus owner native Codex acceptance.

## 3. BRANCH / HEAD / DIFF SCOPE

Repository: `Sekiph82/H-veAI`

Branch: `main`

Prompt/base SHA: `98606ce44199367a3974d9b5824962c0beb4b6d4`

Tracker transition: `060f2ebd9b110e81a2332ab50208d10ef4ca1a6e`

Implementation: `b61ed662a8f893c6127138430c33f066e3ebb891`

Completion tracker: `0b3c1ca90ac999993e725fe91db34e9dfd6dc016`

Builder log/main before this audit: `7843a6a33ce7b47e78466d93e8a0f9eadd43d555`

The implementation scope changes provider/runtime/audit/frontend/tracker/tests only. It adds `src-tauri/src/codex_runtime.rs`, removes the direct audit `reqwest` dependency, rewires `audit_engine.rs`, reduces duplicated resolver code in `codex_adapter.rs`, removes model-setting IPC/UI, updates provider labels/types/tests, and updates tracking.

## 4. ACCEPTANCE CRITERIA MATRIX

| Requirement | Result | Independent audit note |
| --- | --- | --- |
| No active `OPENAI_API_KEY` audit path | PASS | Active source/API-key commands and direct HTTP audit dependency are removed. |
| No direct `api.openai.com` audit provider | PASS | Active audit execution is process-based Codex CLI; direct reqwest dependency removed from H!veAI Cargo.toml. |
| No hidden OpenAI HTTP fallback | PASS | Production model resolution is Codex CLI or unavailable fallback. |
| Codex CLI executable/version foundation reused | PASS | Shared `codex_runtime.rs` factors executable resolution/version/login/bounded process primitives and existing adapter consumes the shared runtime. |
| ChatGPT-login policy enforcement | PASS at source level | Login parser distinguishes ChatGPT/API-key/not-logged/unknown and API-key state is policy-blocked. Host login result remains owner/native evidence. |
| Headless audit execution | PASS | Codex exec path is non-GUI and process based. |
| Read-only audit sandbox | PASS | Audit args require `--sandbox read-only`. |
| Ephemeral/no project-rule execution | PASS | Audit args include `--ephemeral`, `--ignore-user-config`, `--ignore-rules`, dedicated temp cwd and `--skip-git-repo-check`. |
| No hard-coded audit model | PASS | Audit provider uses CLI default (`CLI_DEFAULT`) and does not pass `--model`. Existing Codex agent hard-coded `gpt-5.5` pin was removed. |
| Schema-constrained final result | PASS | Audit uses `--output-schema` and dedicated `--output-last-message`; dedicated final file is authoritative. |
| Bounded stdout/stderr without child deadlock | **FAIL** | `read_bounded` stops reading after the retention cap instead of continuing to drain/discard. This can backpressure/block a verbose child. |
| Dedicated final output size bound | PASS | Final-message file is size-bounded before parsing. |
| Temporary audit directory cleanup | PASS | Dedicated directory is removed after the run closure. |
| Truthful unavailable/failure degradation | PASS in design | Existing non-PASS degradation remains and Codex-specific status categories were added. |
| Settings surface contains no API-key/model setup | PASS | Current Settings surface is `Codex Audit Provider`, local CLI/login/readiness only. |
| Current canonical tracker truthful | **FAIL** | Root tracker top and roadmap current status conflict with implementation-complete state. |
| M17 remains blocked | PASS | No Claude implementation is included. |
| Builder test/build/publication claims | CLAIM_ONLY | Log reports PASS; repository has no independent CI run/status for this commit. |
| Owner real native Codex audit acceptance | PENDING | Must happen only after source remediation/audit PASS. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

Confirmed independently:

- implementation commit exists at the claimed SHA;
- direct `reqwest` dependency was removed from `src-tauri/Cargo.toml`;
- active audit source uses the shared Codex runtime;
- Codex auth classification includes a ChatGPT-only accepted mode and API-key policy block;
- audit arguments include read-only/ephemeral/isolation/schema/final-output controls;
- active Settings UI is Codex-oriented and no longer exposes OpenAI model/API-key configuration;
- current `main` contains the claimed immutable builder log;
- M17 remains unimplemented/blocked.

Not independently established by GitHub-only audit:

- local CLI version `0.153.4`;
- actual host `codex login status` output;
- exact native executable hash/shortcut target;
- claimed 32/32 focused Rust, 426/426 full Rust, 135/135 frontend test execution;
- governed native smoke on the owner's machine.

These remain builder/native claims until owner acceptance or independent execution evidence.

## 6. FILE / SYMBOL EVIDENCE

### `src-tauri/src/codex_runtime.rs`

The new shared runtime provides executable resolution, native PE validation, bounded version/login probes, and `run_bounded_process`.

The critical defect is `read_bounded`:

- it retains bytes up to `max`;
- when an incoming chunk exceeds remaining retention space it marks `truncated = true`;
- then it **breaks** from the read loop.

That is a retention bound, but not a safe stream-drain implementation. After the reader exits, the child still owns its stdout/stderr pipe. If the child continues emitting Codex JSON events or diagnostics, the pipe can fill and the child can block indefinitely until the parent timeout kills it.

The correct behavior is to continue draining the stream to EOF while discarding bytes after the retention cap, keeping `truncated=true`. A bounded retained buffer does not require a bounded amount of data read from the pipe.

### `src-tauri/src/audit_engine.rs`

The audit provider now uses Codex CLI and no longer constructs OpenAI HTTPS requests. It passes no explicit model, uses the dedicated final result file, maintains schema validation and preserves runtime-authoritative provider identity.

### `src-tauri/Cargo.toml`

The direct `reqwest` dependency introduced for M16S is removed.

### `src/pages.tsx` / `src/auditEngine.ts`

The Settings provider panel is now `Codex Audit Provider`; it displays executable/version/login/status and invokes the Codex readiness commands. API-key/model-setting mutation is absent from the active frontend contract.

### `TASKS.md`

The detailed current-truth paragraph says M16T is implementation-complete pending independent audit and owner acceptance. However the authoritative top block still says:

- `Current Task Status: IMPLEMENTATION_IN_PROGRESS`;
- implementation-removal next action;
- `Required Actor: CODEX`;
- task row says implementation in progress.

Those statements are stale after the completion-tracker commit.

### `CODEX_ROADMAP.md`

Top current status still says M16T is active and Required Actor CODEX despite deeper text recording implementation completion.

## 7. FOCUSED TEST EVIDENCE

The repository contains direct tests for:

- Codex audit exec argument safety;
- readiness/failure classifications;
- API-key auth policy blocking;
- runtime-authoritative provider identity;
- schema/freshness behavior;
- frontend provider states.

The builder reports all focused/full regressions PASS. No GitHub Actions workflow/status is attached to the implementation commit, so execution counts are not independently promoted beyond builder evidence.

Critically, the current test set does not prove that a child which writes **more than the retained stdout/stderr limit and continues running** can finish without blocking. That missing stress case directly maps to Finding F01.

## 8. REGRESSION EVIDENCE

Architecture regression coverage is generally strong: removal of OpenAI HTTP settings, preservation of audit schema semantics, local workspace behavior and frontend labels are all represented in source/tests.

The shared runtime extraction introduces a new cross-cutting process primitive. Because the drain defect sits in that primitive, its regression risk is broader than a cosmetic audit bug even though the most immediate effect is Codex audit/readiness execution.

## 9. SECURITY / SAFETY REVIEW

The architectural security direction is good:

- no H!veAI API-key credential path;
- no auth-file scraping;
- ChatGPT login remains owned by Codex CLI;
- API-key login is refused by policy;
- no shell wrapper for audit execution;
- read-only sandbox;
- ignored user/project rules for audits;
- dedicated temporary cwd;
- bounded retained output and final result.

The drain bug is primarily reliability/availability, not credential exposure. Remediation must preserve all current safety restrictions.

## 10. ARCHITECTURE CONSISTENCY

PASS with one runtime reliability correction required.

The resulting topology matches the owner decision:

`Audit Engine -> AuditModel -> local Codex CLI | UnavailableAuditModel`

No active OpenAI API-provider branch should be reintroduced in V02.

The shared runtime extraction is preferable to duplicating executable resolution between Codex agent sessions and audit sessions.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

**FAIL.**

The immutable builder log truthfully states implementation-complete/pending independent audit and owner acceptance. Deeper TASKS/roadmap text also says this. But the most prominent canonical current-status blocks remain at implementation-in-progress with Required Actor CODEX.

This violates the project's tracker-truthfulness rule and can cause H!veAI itself to display the wrong current task/action/actor.

## 12. FINAL REPOSITORY STATE

Before this independent audit publication, `main` is:

`7843a6a33ce7b47e78466d93e8a0f9eadd43d555`

The commit contains the immutable M16T V01 builder log and descends linearly from the M16T prompt/plan and implementation commits.

## 13. OPEN CROSS-MILESTONE FINDINGS

- M16 remains OPEN.
- M17 remains blocked and must not begin during M16T V02.
- Owner native Codex audit acceptance remains pending.

## 14. DEFECTS BY SEVERITY

### F-M16T-V01-001 — MAJOR — bounded stream capture can deadlock the Codex child

`read_bounded` exits the reader thread when retained output reaches the cap. The reader must instead continue draining/discarding until EOF while keeping only the bounded retained prefix and the `truncated` flag.

### F-M16T-V01-002 — MAJOR — canonical current tracker contradicts implementation completion

`TASKS.md` top status and `CODEX_ROADMAP.md` top current status still present M16T as implementation-in-progress/actor CODEX after the completion commit and log publication.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

After the bounded drain fix, the shared process primitive should remain minimal and generic. Do not add terminal scraping, auth-file inspection, or provider-specific credential storage.

A future provider/runtime test harness can reuse the over-cap drain stress test for Claude CLI in M17.

## 16. UNVERIFIED ITEMS

- owner's actual Codex ChatGPT login state;
- real end-to-end readiness turn on the current host;
- real native audit generation through Codex CLI;
- local publication executable/shortcut hash;
- builder-reported test execution counts.

These belong to owner/native acceptance after source-level V02 PASS.

## 17. REGRESSION RISK

**MEDIUM.**

The provider architecture itself is appropriately constrained, but the stream-drain defect can turn valid verbose CLI executions into timeouts and sits inside a shared runtime primitive.

## 18. AUDIT CONFIDENCE

**HIGH for source/repository findings; MEDIUM for host-native claims.**

Both blocking findings are directly visible in repository source/tracker state and do not depend on builder testimony.

## 19. FINAL VERDICT

**CHANGES_REQUIRED**

M16T V01 is not ready for owner native acceptance yet.

The Codex-only architecture is accepted in principle and should be preserved. V02 must fix only the shared bounded-stream draining behavior and canonical current tracker truth, plus direct stress/regression tests. It must not reintroduce any OpenAI API-key or direct HTTP audit provider and must not activate M17.

## 20. REQUIRED REMEDIATION

Create M16T V02 with exactly these bounded goals:

1. change shared bounded stdout/stderr capture so it retains at most the configured cap **while continuing to drain the underlying stream until EOF**;
2. add direct stress tests where stdout and stderr each exceed their caps and the child nevertheless exits normally without timeout/deadlock, with retained sizes bounded and truncation flags true;
3. keep dedicated final-message output authoritative and prove over-cap generic event output does not prevent a valid final result;
4. reconcile `TASKS.md` and `CODEX_ROADMAP.md` current/top state to implementation-complete / awaiting independent strict audit and owner native acceptance, Required Actor HUMAN;
5. preserve M16 OPEN and M17 blocked;
6. rerun focused/full regressions and governed native publication;
7. preserve the absolute prohibition on active OpenAI API-key/direct HTTP audit paths.
