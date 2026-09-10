# M16D — Independent Whole-M16 Strict Re-Audit

Date: 2026-09-08  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16D_WHOLE_M16_FINAL_CLOSURE_REMEDIATION_LOG.md`  
Implementation commits reviewed: `24e91047addcd9beffe99e5fce7dc7ac7503de12`, `7033d0c0292f7e81572756fa302e126f8cd56d59`  
Current branch state also includes the H!veAI control-plane bootstrap merge.

## Verdict

**FAIL / CHANGES REQUIRED**

- BLOCKER: 1
- MAJOR: 1
- MINOR: 2
- NOTE: 1

This is a whole-M16 release-gate re-audit. R74-R81 are materially implemented, but M16 cannot close because one truthfulness defect can still produce a persisted PASS with an unresolved prior release-blocking finding.

M15 remains PASS/CLOSED.  
M16 remains OPEN.  
M17 MUST NOT activate yet.  
M21 MUST NOT start.  
Roadmap completed progress remains `16 / 20 = 80%`.

---

# M16-R82 — BLOCKER
## Prior findings are applied after semantic PASS validation, so an inherited unresolved MAJOR/BLOCKER can coexist with PASS

### Production order

`run_with_model(...)` currently does:

1. collect input;
2. `evaluate_with(...)`;
3. `validate_semantic_evaluation(...)` inside `evaluate_with`;
4. freshness check;
5. `persist_run(...)`;
6. only inside `persist_run(...)`, for AVAILABLE completed re-audits, call `apply_prior_finding_dispositions(...)`.

The deterministic semantic validator therefore runs **before** inherited/omitted prior findings are materialized into the current evaluation.

### Failure mode

A model can return:

- verdict = PASS;
- no current findings;
- no explicit prior disposition for an older OPEN MAJOR/BLOCKER.

The semantic validator sees no unresolved current blocker and permits PASS.

Later `apply_prior_finding_dispositions(...)` correctly carries the omitted prior finding into the current audit as lifecycle `OPEN`, but verdict/confidence/risk are not revalidated afterwards.

The persisted current audit can therefore contain:

- `PASS`;
- an inherited OPEN MAJOR/BLOCKER;
- `blocks_release = true`.

That contradicts M16 truthfulness and directly revives the core unresolved-prior-finding release-gate problem.

### Existing test gap

`explicit_reaudit_dispositions_are_required_and_validated` verifies that omission creates an OPEN inherited finding, but it does not assert that the evaluation verdict is downgraded/rejected after inheritance.

### Required remediation

Make prior-finding materialization part of the semantic decision before final verdict persistence.

Acceptable designs:

1. apply prior dispositions/inheritance before final semantic validation; or
2. run the full deterministic semantic validator again after prior materialization.

Required invariant:

> No persisted PASS may contain an unresolved lifecycle OPEN BLOCKER or MAJOR, including inherited prior findings.

Add direct AVAILABLE re-audit fixtures for omitted prior BLOCKER/MAJOR and STILL_OPEN prior findings.

---

# M16-R83 — MAJOR
## STAGED / COMMIT_RANGE source extraction still uses the unbounded non-draining Git runner

### Production behavior

`read_source_evidence(...)` uses:

- WORKING_TREE: bounded local file read;
- STAGED: `git_engine::run_git(root, ["show", ":<path>"])`;
- COMMIT_RANGE: `git_engine::run_git(root, ["show", "<head>:<path>"])`.

`run_git(...)` pipes stdout/stderr, waits for the child to terminate with `try_wait`, and only calls `wait_with_output` after process exit.

For commands producing enough output to fill the OS pipe buffer, the child can block waiting for its stdout to be drained while the parent waits for child exit. The result can be timeout rather than bounded evidence.

Even when it completes, the full file is captured before the Audit Engine applies its source snippet bound.

### Contract violation

M16D explicitly claims bounded/streaming evidence collection. WORKING_TREE source reads satisfy that, but STAGED and COMMIT_RANGE source reads do not.

A large staged or committed source file may therefore:

- allocate/capture far beyond the intended snippet budget;
- timeout;
- silently return no source evidence because `.ok()?`/Option flow drops the failure.

### Required remediation

Introduce a bounded/streamed Git blob reader for source evidence.

For STAGED and COMMIT_RANGE:

- stream stdout while process runs;
- capture only the bounded source scan/window budget;
- record explicit TRUNCATED / UNAVAILABLE evidence when the blob exceeds bounds or cannot be read;
- do not silently omit the changed source file;
- do not use generic `run_git` for potentially large blob content.

Also adversarially audit remaining `run_git` call sites for commands whose output can be repository-sized.

---

# M16-R84 — MINOR
## Builder-log relevance is still hard-coded to older M16 artifact names

`builder_log_relevance(...)` gives special priority to:

- `M16C_REV2`
- `M16_COMPREHENSIVE`

and only generic lower priority to other `M16` logs.

That means after M16D, an older M16C remediation/comprehensive artifact can outrank the current M16D builder log.

Builder logs remain CLAIM_ONLY, so this is not a verdict-truth blocker, but it defeats the intended “current/relevant log first” behavior.

### Required remediation

Remove milestone-name hard-coding.

Prefer durable provenance:

1. audited prompt version/session linkage;
2. explicit active task/cycle identity;
3. latest matching milestone/task log;
4. newest project log fallback.

---

# M16-R85 — MINOR
## Full frontend regression was not fully deterministic/green

The builder log reports:

- frontend: 124 passed;
- one existing timing-sensitive Project Cockpit test failed in the full parallel suite;
- isolated rerun passed 8/8.

An isolated retry is useful diagnostic evidence, but it is not equivalent to a fully green deterministic release suite.

This is especially relevant because the next product work is centered on Project Cockpit live-state synchronization.

### Required remediation

Fix the flaky Project Cockpit test or the underlying race. The governed closure run should produce one fully green full frontend suite without retry-dependent acceptance.

---

# M16-N04 — NOTE
## Production GPT audit provider remains intentionally UNAVAILABLE

This remains allowed. Native behavior must continue to show CONDITIONAL/UNAVAILABLE and must never fabricate PASS.

---

# Whole-M16 closure requirement

The next implementation must close R82-R85 together in one continuous run.

After implementation:

1. perform the whole-M16 adversarial sweep again;
2. run all focused + full regressions;
3. produce one fully green frontend suite;
4. governed publication;
5. independent whole-M16 re-audit;
6. user native/visual acceptance.

Only then may M16 close and progress become `17 / 20 = 85%`.
