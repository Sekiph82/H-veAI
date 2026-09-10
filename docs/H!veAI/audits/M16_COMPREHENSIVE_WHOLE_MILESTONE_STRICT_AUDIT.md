# M16 GPT Audit Engine — Comprehensive Whole-Milestone Strict Audit

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Scope: **entire M16 implementation and all M16A/M16B remediations as one release-gate audit**
Audited current code state: M16B implementation plus subsequent audit/prompt documentation through `8b2d0c56f1f29ac2706aa1268760a4e58086d627`

## Verdict

**FAIL / CHANGES REQUIRED**

- BLOCKER: 0
- MAJOR: 10
- MINOR: 1
- NOTE: 1

This audit intentionally supersedes the previous one-finding-at-a-time review pattern. It evaluates the whole M16 surface as one system: evidence collection, Git scope, source/test verification, result semantics, persistence, re-audit history, remediation provenance, UI target selection, security, and degraded operation.

M15 remains PASS/CLOSED.
M16 remains OPEN.
M17 MUST NOT activate.
M21 MUST NOT start.
Roadmap completed progress remains `16 / 20 = 80%`.

The previously created `M16C_PERSISTED_EVIDENCE_REFERENCE_INTEGRITY_REMEDIATION_PROMPT.md` is now **SUPERSEDED BEFORE EXECUTION** by the comprehensive remediation prompt produced from this audit. Do not execute the older M16C prompt separately.

---

# M16-R63 — MAJOR
## Persisted evidence IDs and evidence references use incompatible namespaces

M16B correctly avoided global `audit_evidence.id` collisions by persisting evidence rows as:

`<audit_id>:<logical_evidence_id>`

But findings, requirement coverage, and prior-finding disposition references still store unprefixed logical IDs.

Example:

- persisted row ID: `audit-123:TEST_RUN:proof`
- finding ref: `TEST_RUN:proof`

After `get(...)`, a consumer cannot resolve `finding.evidenceRefs` against `audit.evidence[].id`.

### Required fix

Introduce an explicit two-level evidence identity:

- globally unique persisted row ID;
- audit-scoped `logical_evidence_id`.

Use one deterministic same-audit resolver and validate all persisted refs before commit.

---

# M16-R64 — MAJOR
## Finding and coverage row IDs collide across immutable audits

Normal model findings are created with IDs such as:

`finding-0-<hash(finding_key)>`

Requirement coverage uses:

`coverage-0`, `coverage-1`, ...

Both database tables use globally unique primary keys.

Therefore two immutable audits that return the same finding key/index or coverage index can collide even though they belong to different audit runs.

This is especially dangerous for re-audits and task-scoped degraded audits with repeated coverage rows.

### Required fix

Use audit-scoped persisted row IDs and explicit logical IDs for:

- findings;
- requirement coverage;

or another equivalent globally unique durable identity scheme.

Add two-audit repeated-result fixtures that prove no PK collision.

---

# M16-R65 — MAJOR
## Model output is schema-validated but not semantically evidence-validated

`parse_model_output(...)` validates JSON shape and enums, but the resulting audit judgment is still trusted too deeply.

Current production can accept structurally valid output such as:

- `PASS` with an unresolved MAJOR/BLOCKER finding;
- `PASS` with FAILED/UNVERIFIED required coverage;
- HIGH confidence despite only CLAIM_ONLY/UNVERIFIED evidence;
- a finding citing a nonexistent evidence ID;
- coverage citing a nonexistent evidence ID;
- a finding citing only a builder-log CLAIM_ONLY item and still being treated as verified;
- a SUPERSEDED prior finding pointing to a replacement finding key that does not exist.

Only prior-finding disposition evidence refs currently receive direct current-input existence validation.

### Required fix

Add one central deterministic post-model semantic validator before persistence.

At minimum:

- PASS impossible with unresolved BLOCKER/MAJOR;
- PASS impossible when required coverage is FAILED/UNVERIFIED/PARTIAL under the accepted contract;
- PASS impossible when required authority classes are UNAVAILABLE/STALE/TRUNCATED in a way that prevents verification;
- builder CLAIM_ONLY cannot independently satisfy VERIFIED coverage;
- all finding/coverage/disposition evidence refs must resolve in the current audit evidence namespace;
- SUPERSEDED replacement key must exist in the current finding set;
- confidence/risk must be bounded/downgraded when evidence quality requires it.

Do not let the model override these deterministic truthfulness rules.

---

# M16-R66 — MAJOR
## Re-audit can report PASS while prior release-blocking findings remain unresolved, and STILL_OPEN findings become non-actionable

M16A correctly stopped omission from auto-closing a prior finding. However, omission now simply leaves the finding in the previous audit while the current model verdict remains untouched.

Therefore a valid current model response can say PASS while a prior MAJOR/BLOCKER is still unresolved because it was omitted from `priorFindingDispositions`.

There is a second actionability defect:

`STILL_OPEN` dispositions are persisted with:

`status = "STILL_OPEN"`

But Audit Center enables finding selection only when:

`finding.status === "OPEN"`

and `create_remediation_prompt(...)` also accepts only `status == "OPEN"`.

Thus a finding explicitly confirmed STILL_OPEN in the latest re-audit cannot be selected for remediation from that latest audit.

### Required fix

Separate finding lifecycle status from re-audit disposition.

Recommended:

- lifecycle status: OPEN | CLOSED | SUPERSEDED;
- disposition metadata: STILL_OPEN | CLOSED | SUPERSEDED.

STILL_OPEN must remain lifecycle OPEN and remain remediation-eligible.

For every prior OPEN release-blocking finding, a new AVAILABLE re-audit must either:

- explicitly keep it OPEN;
- explicitly close/supersede it with validated evidence;

or deterministic result logic must prevent PASS.

Omission alone may not produce a PASS state while unresolved release blockers exist.

---

# M16-R67 — MAJOR
## Audit Git scope does not capture the actual implementation once changes are staged or committed

The Audit Engine always requests:

`GitDiffScope::WorkingTree`

The Git Engine supports only:

- STAGED
- WORKING_TREE

There is no commit-range/baseline diff scope.

Consequences:

- staged changes are not present in the audit's working-tree diff text;
- committed implementation changes disappear from diff evidence entirely;
- `baseline_ref` is stored but not used to compute the audited change set;
- an implementation that Codex commits before audit can appear as a clean repository with no actual implementation diff;
- the Audit Center has no explicit audited implementation/session/commit target to reconstruct the intended change set.

This breaks the core M16 contract of auditing **actual Git diff / changed files**.

### Required fix

Create an explicit audit Git target/scope contract.

Support at least:

- working tree;
- staged/index;
- committed range from a validated base to audited HEAD.

For implementation/session audits, persist the target baseline/base and audited HEAD/session/prompt provenance so the exact committed implementation can be reproduced later.

Do not guess arbitrary Git refs from frontend input.

---

# M16-R68 — MAJOR
## Repository freshness token does not cover all mutable repository evidence

M16A added freshness protection, but its identity is still incomplete.

It hashes:

- branch;
- HEAD;
- staged path list;
- unstaged path list;
- untracked path list;
- conflicted path list;
- **bounded working-tree diff hash**;
- truncation flag;
- repository identity.

Missing content identity includes:

- staged/index content changes when staged path remains the same;
- untracked file content changes when path remains the same;
- working-tree changes beyond the bounded/truncated diff prefix when path set remains the same;
- committed audit-range identity beyond HEAD/base semantics because committed range is not represented.

Therefore a repository can materially change after evidence collection without changing the freshness token.

### Required fix

Use content-sensitive Git identities that do not require transporting unbounded content.

Examples:

- index tree/hash or staged diff hash;
- deterministic changed-file content hashes for bounded selected files;
- untracked selected-file hashes;
- full change-set identity computed internally before truncating user/model-facing diff;
- explicit base..HEAD identity for committed range.

The freshness token must be computed from full identities, while displayed evidence may remain bounded.

---

# M16-R69 — MAJOR
## “Direct test-body inspection” and misleading-test detection are not actually claim-directed

Current source selection includes:

- changed paths;
- four fixed production files;
- discovered project sources.

A persisted test run is read as metadata only. There is no production mapping from the claimed test suite/command/test result to the actual test file/body that supposedly proves the requirement.

A test body is inspected only if its file happens to enter the generic source selection list.

The classifier then uses broad whole-file substring heuristics:

- contains `vi.mock` / `jest.mock` / `mock(` → PARTIAL;
- contains `assert` / `expect(` → CORROBORATED;
- contains skip marker → STALE.

This can misclassify a file because an unrelated test elsewhere in the first bounded chunk contains an assertion or mock.

Also each source file is read only from the beginning up to the fixed snippet bound, so a named test/symbol later in a large file may never be inspected.

### Required fix

Implement a bounded claim-directed verification planner:

- identify claimed test file/test name/suite from durable test evidence or explicit audit target metadata;
- locate the specific test definition/range;
- inspect the relevant direct body;
- record whether it reaches real production code or only mocks the boundary;
- record assertion(s) relevant to the claimed invariant;
- distinguish file-level mock presence from the specific test's behavior;
- use bounded ranges around matched symbols/tests, not only the first bytes of a file.

If exact test-body mapping cannot be established, evidence must remain UNVERIFIED rather than upgraded by filename/body substrings.

---

# M16-R70 — MAJOR
## Task target validation and UI are inconsistent with task authority

Audit Center uses a free-text input:

`Task / freeform project audit <input placeholder="Optional task ID">`

This exposes internal task IDs directly instead of the workflow/task picker behavior already established elsewhere.

More importantly, native `collect_input(...)` handles a nonempty unknown task ID by:

- failing to resolve a workflow task;
- producing `task = None`;
- preserving the original `request.task_id` in `AuditInput`.

Persistence then attempts to store that invalid task ID under the audit FK, causing a later database failure instead of a bounded target-validation error.

### Required fix

- use one project-neutral workflow task picker with a freeform-project option;
- validate task ownership/existence natively before evidence collection;
- unknown/wrong-project task ID must fail immediately with a deterministic error;
- never reinterpret an invalid task ID as a project/freeform audit.

---

# M16-R71 — MAJOR
## Task evidence can be labeled VERIFIED when task-intelligence parsing failed

`task_intelligence::list(...)` is converted with `.ok()`.

If task intelligence fails, a valid workflow task can still be turned into `AuditTaskEvidence` with empty acceptance criteria/dependencies/blockers.

Then `task_evidence_item(Some(task))` labels the evidence:

`VerificationStatus::Verified`

The engine therefore cannot distinguish:

- “the task genuinely has no acceptance criteria”
from
- “the task intelligence authority failed and criteria were unavailable.”

This violates M16 truthfulness.

### Required fix

Represent task-source/parse availability explicitly.

Do not mark task requirements VERIFIED unless the authority that supplies them succeeded and provenance/hash is present as required.

Use PARTIAL/UNAVAILABLE/UNVERIFIED for degraded task-intelligence input.

Do not silently swallow task-source/intelligence errors that materially affect requirement coverage.

---

# M16-R72 — MAJOR
## Builder-log reader can escape the registered project root through a directory symlink/junction

`read_source_evidence(...)` correctly canonicalizes candidate paths and verifies they remain inside the canonical registered root.

`read_builder_claims(...)` does not.

It constructs:

`<registered-root>/docs/H!veAI/codex-logs`

and directly calls `read_dir` / `read_to_string`.

If that directory or a child markdown file is a symlink/junction pointing outside the registered root, the Audit Engine can ingest external files as builder-log evidence.

The line sanitizer is not a substitute for root containment.

### Required fix

Apply the same canonical-root containment policy to builder-log discovery and every file opened by it.

Reject symlink/junction escape and record EXCLUDED evidence without reading external content.

Add platform-appropriate tests where supported.

---

# M16-R73 — MINOR
## Builder-log selection is deterministic but not relevant/current

`read_builder_claims(...)` sorts paths alphabetically and takes the first four.

This can select old unrelated logs and omit the current implementation log.

Because builder logs are secondary CLAIM_ONLY evidence this is not a release blocker by itself, but it reduces audit usefulness and can create confusing evidence manifests.

### Required fix

Select bounded builder claims by explicit audit/session/prompt provenance when available, then fall back to recent/relevant project logs. Preserve CLAIM_ONLY status.

---

# M16-N02 — NOTE
## Production GPT provider remains intentionally UNAVAILABLE

Current production uses `UnavailableAuditModel`.

That is still allowed by the original M16 authority as long as native behavior remains truthful:

- CONDITIONAL;
- UNAVAILABLE;
- no fabricated PASS;
- evidence remains visible.

Do not add a production GPT provider merely to close this audit unless the product owner separately requests it.

---

# Whole-M16 acceptance requirements after remediation

The next remediation must close **all** open findings in one run:

- R63
- R64
- R65
- R66
- R67
- R68
- R69
- R70
- R71
- R72
- R73

The remediation must also preserve already-fixed:

- R59 explicit re-audit disposition behavior;
- R60 exact remediation-session provenance;
- R61 intended stale protection, strengthened by R68;
- R62 degraded re-audit persistence;
- all M14/M15 accepted behavior.

After implementation, run one **whole-M16 independent strict re-audit**, not a single-finding re-audit.

Only after the entire M16 passes technically should native/user acceptance be requested.
