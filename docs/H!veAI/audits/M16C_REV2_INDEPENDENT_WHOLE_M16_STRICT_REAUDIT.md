# M16C REV2 — Independent Whole-M16 Strict Re-Audit

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`
Builder log: `docs/H!veAI/codex-logs/M16C_REV2_COMPREHENSIVE_WHOLE_MILESTONE_CLOSURE_REMEDIATION_LOG.md`
Audited implementation commit: `b371bc2d7378ade8ec7d167c019476693a028b4a`
Evidence publication HEAD reviewed: `495774b6d629b9afa625b174a26a2b5daa306865`

## Verdict

**FAIL / CHANGES REQUIRED**

- BLOCKER: 0
- MAJOR: 7
- MINOR: 1
- NOTE: 1

This is a **whole-M16 release-gate re-audit**, not a previous-finding-only check.

The implementation materially closes R63-R73 from the prior comprehensive audit, but seven additional source-level defects remain across verdict semantics, re-audit identity, Git scope/freshness, and bounded evidence collection.

M15 remains PASS/CLOSED.
M16 remains OPEN.
M17 MUST NOT activate.
M21 MUST NOT start.
Roadmap completed progress remains `16 / 20 = 80%`.

---

# Prior comprehensive findings R63-R73

The re-audit confirms the intended remediation is materially present for:

- audit-scoped evidence/finding/coverage identity;
- semantic model-output guard;
- OPEN/STILL_OPEN lifecycle separation;
- working/staged/commit-range Git types;
- expanded freshness identities;
- claim-directed test metadata support;
- task picker/native task validation;
- task-intelligence availability status;
- canonical builder-log containment/relevance.

Those prior findings should not be reopened merely because new adjacent defects were discovered.

---

# M16-R74 — MAJOR
## Global weak-evidence logic makes legitimate PASS effectively impossible

### Production behavior

`validate_semantic_evaluation(...)` calculates `weak_evidence` using:

- weak coverage;
- degraded task requirements;
- **any evidence item in the entire input** whose status is CLAIM_ONLY, UNVERIFIED, PARTIAL, UNAVAILABLE, STALE, or TRUNCATED.

The audit input intentionally includes evidence that is secondary or not applicable.

Examples:

- builder logs are always `CLAIM_ONLY`;
- project/freeform audit emits `TASK_REQUIREMENTS:project-audit` as `UNAVAILABLE`;
- unrelated bounded evidence may legitimately be PARTIAL/TRUNCATED without being required for the model's PASS judgment.

Therefore a valid PASS based on fully VERIFIED required evidence is downgraded to CONDITIONAL merely because a secondary builder claim exists.

In a normal H!veAI project containing builder logs, this makes PASS practically unreachable.

### Contract violation

M16 requires builder logs to be secondary claims only. “Secondary” means they cannot prove PASS by themselves, not that their mere presence must veto an otherwise independently verified PASS.

### Required fix

Compute verdict admissibility from **required/referenced evidence and authority completeness**, not from every evidence row in the manifest.

- CLAIM_ONLY must never satisfy VERIFIED coverage.
- Irrelevant CLAIM_ONLY evidence must not veto PASS.
- NOT_APPLICABLE/freeform task evidence must not veto project-level PASS.
- TRUNCATED/PARTIAL evidence should veto PASS only when it is required to prove a requirement/finding.

Add a direct fixture containing verified required evidence plus an unrelated builder CLAIM_ONLY row and prove PASS remains eligible.

---

# M16-R75 — MAJOR
## Requirement coverage completeness is not enforced

### Production behavior

The semantic validator checks only coverage rows the model returned.

It does not prove:

- every task acceptance criterion has a coverage row;
- every required requirement reference is unique and known;
- a model cannot omit difficult requirements;
- finding `requirementRefs` actually refer to a real audited requirement.

A task can contain multiple verified requirements while the model returns:

- zero coverage rows, or
- only one favorable row,

and no deterministic completeness error is generated.

### Required fix

Build the canonical requirement set from audit input.

For task audits:

- assign deterministic requirement IDs before model execution;
- require exactly one valid coverage disposition per required criterion;
- reject/downgrade missing, duplicate, or unknown requirement refs;
- validate finding requirementRefs against the canonical requirement set.

For project/freeform audits:

- use an explicit project-audit requirement/authority contract or mark task criteria NOT_APPLICABLE;
- never fabricate task criteria.

PASS requires complete required coverage.

---

# M16-R76 — MAJOR
## Re-audit chain can drift to a different task or Git target

### Production/UI behavior

Audit Center's `Re-audit` button calls the same `startAudit(selected.id)` path using the **currently selected UI controls**:

- current `taskId`;
- current `gitScope`;
- current `baseRef`;
- current `headSha`.

It does not reconstruct the selected prior audit's immutable target.

Native `persist_run(...)` validates only that `prior_audit_id` belongs to the same project.

It does **not** require:

- same task/freeform target;
- same Git scope;
- compatible base/range;
- expected remediation/session provenance relationship.

Thus a prior task-A audit can be re-audited as task-B or freeform project audit while inheriting prior findings into the new chain.

### Required fix

Define immutable re-audit target compatibility.

At minimum:

- prior audit project must match;
- task identity must match unless an explicit separately approved scope-transition feature exists;
- Git scope/base semantics must be inherited or explicitly validated as compatible;
- Audit Center Re-audit must default to the selected audit's immutable target rather than mutable form state;
- native authority must enforce this even if frontend is bypassed.

Add cross-task, task→freeform, and incompatible-scope rejection tests.

---

# M16-R77 — MAJOR
## Inherited prior findings can carry stale evidence refs that do not exist in the current audit

### Production behavior

For an AVAILABLE re-audit, `apply_prior_finding_dispositions(...)` carries an omitted prior OPEN finding into the current evaluation.

For omitted findings it copies:

`evidence_refs = parse_list(prior.evidence_refs_json)`

The current audit then runs `validate_semantic_evaluation(...)`, whose reference validator requires every finding evidence ref to exist in the **current input evidence namespace**.

If the old evidence logical IDs are not recollected in the new run, the re-audit fails with `AUDIT_EVIDENCE_REFERENCE_NOT_FOUND`.

The same issue can affect a STILL_OPEN disposition with no current evidence refs because the implementation falls back to prior evidence refs.

### Required fix

Never masquerade prior-audit evidence refs as current-audit refs.

Use an explicit inherited-evidence/provenance contract, for example:

- current evidence refs only for evidence recollected now;
- prior audit/finding reference stored separately for inherited rationale;
- or copy immutable prior evidence into a typed inherited evidence class with current audit-scoped logical identity and provenance pointing back to the prior audit.

An omitted prior finding must remain OPEN without making re-audit persistence dependent on old logical IDs being recollected.

Add a test where prior evidence disappears/changes in the current audit and omission still persists a truthful inherited OPEN finding.

---

# M16-R78 — MAJOR
## STAGED and COMMIT_RANGE audits ingest out-of-scope working-tree source evidence

### Production behavior

`collect_input(...)` builds source candidates from:

`changed_paths(git_snapshot)`

That function always combines:

- staged files;
- unstaged files;
- untracked files.

Then it extends the list with the selected Git diff's `changed_files`.

Therefore:

- a STAGED audit also reads unstaged and untracked files as source evidence;
- a COMMIT_RANGE audit also reads current working-tree staged/unstaged/untracked files unrelated to the committed range.

The audit target says one scope while the source evidence planner silently mixes another scope into the model input.

### Freshness consequence

For STAGED/COMMIT_RANGE, freshness does not hash all of those out-of-scope file contents. So the accidentally included source evidence can change without the target-specific freshness identity necessarily changing.

### Required fix

Make source selection scope-specific.

- WORKING_TREE: only working-tree/untracked evidence appropriate to that scope.
- STAGED: staged/index change set plus explicitly separate contextual evidence.
- COMMIT_RANGE: base..HEAD changed files from committed range.

If contextual out-of-scope source is included, classify it explicitly as context and include its full content identity in freshness.

The default model evidence used to prove implementation must not mix scopes silently.

---

# M16-R79 — MAJOR
## Several “bounded” evidence paths read full files/content before applying bounds

The M16 contract requires bounded source/test/log collection.

Three production paths violate the spirit and operational safety of that boundary:

### Exact test body reader

`locate_test_body(...)` uses:

`fs::read_to_string(canonical)`

which loads the entire test file before extracting an 80-line window.

### Builder log reader

`read_builder_claims(...)` uses:

`fs::read_to_string(&path)`

and only afterwards applies `bound_text(...)`.

### Untracked freshness identity

`untracked_content_identity(...)` calls `fs::read(canonical)` for each untracked file, up to thousands of files.

A very large test/log/untracked file can therefore cause excessive memory/latency before the advertised evidence bounds take effect.

### Required fix

Use streaming/bounded readers and streaming hashes.

- locate test names without unbounded full-file allocation;
- read only bounded windows around matched test definitions;
- builder logs: read/hash at bounded size or stream safely;
- untracked identity: hash via buffered streaming with explicit per-file/total/time limits and truthful overflow state.

The freshness identity may hash full content, but it must do so by streaming rather than reading entire files into memory.

---

# M16-R80 — MAJOR
## Large Git diffs fail instead of degrading truthfully to bounded evidence

Git Engine increased `MAX_COMMAND_OUTPUT` to 8 MiB and computes a full hash from raw diff output.

If raw diff output exceeds that limit, `output_text(...)` returns:

`GIT_OUTPUT_LIMIT`

For WORKING_TREE/STAGED, initial `collect_input` may swallow the diff error and continue with `diff = None`.

But immediately before persistence, `current_freshness_token(...)` calls `git_engine::diff(...)` and propagates the same error, causing the entire audit run to fail.

For COMMIT_RANGE the initial collection fails immediately.

This contradicts the M16 bounded/truncated evidence contract for large legitimate change sets.

### Required fix

Separate:

- full internal change identity;
- bounded display/model diff.

Compute full identity by streaming or Git-native object/tree/diff identity without retaining unlimited output.

Return bounded/truncated diff evidence rather than failing solely because the display diff is large.

An overlarge legitimate repository change should become explicit TRUNCATED/PARTIAL evidence or another deterministic bounded state, not an inconsistent mid-run error.

Add >limit working/staged/commit-range fixtures.

---

# M16-R81 — MINOR
## Audit Center exposes arbitrary commit-range ref text instead of authority-derived target presets

The comprehensive remediation introduced free-text:

- Base ref
- Head SHA

fields.

Native Git resolution prevents shell injection and resolves the values to commits, so this is not an arbitrary-command vulnerability.

However, the authoritative M16 remediation contract required implementation/session audit ranges to be derived from project/session/prompt/prior-audit authority where available, not primarily from raw user-entered refs.

This weakens reproducibility and increases the chance that a user audits a range unrelated to the implementation session they intended to verify.

### Required fix

Prefer bounded presets derived from:

- selected implementation Agent session;
- prompt/version provenance;
- prior audit;
- registered branch/base policy.

Keep a manual commit-range mode only if explicitly labeled advanced/manual and persist that origin truthfully.

---

# M16-N03 — NOTE
## Production GPT provider remains intentionally UNAVAILABLE

Production still uses `UnavailableAuditModel`.

This remains allowed by M16 authority and is not a release defect by itself.

Native acceptance should verify the UI communicates:

- CONDITIONAL;
- UNAVAILABLE;
- no fabricated PASS.

---

# Whole-M16 remediation requirement

The next remediation must close **R74-R81 together in one run**.

Do not create one prompt per finding.

After implementation, perform another entire M16 adversarial sweep covering:

- evidence semantics;
- requirement completeness;
- re-audit identity;
- persisted history;
- Git scopes;
- freshness;
- source/test/log boundedness;
- ACL/security;
- remediation provenance;
- UI;
- degraded paths.

If the remediation itself exposes an adjacent BLOCKER/MAJOR during that sweep, fix it in the same run before producing the builder log.

Only after a clean whole-M16 independent re-audit should the user be asked for native/visual acceptance.
