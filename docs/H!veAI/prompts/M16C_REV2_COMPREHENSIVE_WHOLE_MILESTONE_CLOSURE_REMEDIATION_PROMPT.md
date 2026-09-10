# M16C REV2 — Comprehensive Whole-Milestone Closure Remediation Prompt

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Milestone: M16 GPT Audit Engine
Authority: **single consolidated remediation prompt for the entire remaining M16 defect set**

## 0. Supersession and execution mode

This prompt **SUPERSEDES BEFORE EXECUTION**:

`docs/H!veAI/prompts/M16C_PERSISTED_EVIDENCE_REFERENCE_INTEGRITY_REMEDIATION_PROMPT.md`

Do not execute the older M16C prompt separately.

Read and obey:

`H!veAI/GPT.md`

This run must remediate the entire current M16 defect set in one continuous implementation.

**Do not stop after R63, R64, R65, or any individual finding.**
Proceed through all findings, full regression, governed publication, immutable log, commit, and push.

M15 remains PASS/CLOSED.
M16 remains OPEN until independent whole-milestone strict re-audit + user native/visual acceptance.
M17 MUST NOT activate.
M21 MUST NOT start.
Roadmap completed progress remains `16 / 20 = 80%`.

---

# 1. Source authority

Read in full before coding:

- `H!veAI/GPT.md`
- M16 unified implementation prompt
- M16 implementation log
- M16 whole-milestone independent audit history
- M16A prompt/log/re-audit
- M16B prompt/log/re-audit
- `M16_COMPREHENSIVE_WHOLE_MILESTONE_STRICT_AUDIT.md`

Treat builder logs as CLAIM evidence only.

The comprehensive audit is the current defect authority.

---

# 2. Findings to close in this one run

Close all of:

- M16-R63 — persisted evidence IDs vs logical refs
- M16-R64 — finding/coverage global PK collisions
- M16-R65 — missing deterministic semantic/evidence validation
- M16-R66 — unresolved prior finding PASS/actionability defect
- M16-R67 — incomplete Git implementation scope
- M16-R68 — incomplete repository freshness identity
- M16-R69 — non-claim-directed test/source verification
- M16-R70 — invalid/free-text task target behavior
- M16-R71 — falsely VERIFIED degraded task evidence
- M16-R72 — builder-log symlink/junction root escape
- M16-R73 — irrelevant/alphabetical builder-log selection

Also preserve:

- R59 explicit prior-finding disposition safety
- R60 exact remediation-session provenance
- R61 stale-result principle, strengthened by R68
- R62 degraded UNAVAILABLE/MALFORMED re-audit persistence
- M14E final response behavior
- all M15 Prompt Engine approval/single-use/handoff behavior

---

# 3. R63 + R64 — durable child identity and evidence-reference model

Implement one consistent durable identity strategy for all audit child rows.

## 3.1 Evidence identity

Add an additive migration with:

- `audit_evidence.logical_evidence_id TEXT`

For all new rows:

- DB primary row ID is globally unique and audit-scoped;
- logical evidence ID preserves the input/model contract ID;
- enforce same-audit uniqueness:
  `UNIQUE(audit_id, logical_evidence_id)`

Backfill:

- M16B-style `<audit_id>:<logical>` rows → logical = stripped exact audit prefix;
- older rows → logical = existing ID when no exact audit prefix exists.

Expose both:

- persisted row ID;
- logicalEvidenceId.

All model-facing/finding/coverage/disposition refs use logical evidence IDs scoped to the current audit.

## 3.2 Finding identity

Do not persist globally repeating model IDs directly as PKs.

Introduce, if needed:

- globally unique persisted finding row ID;
- logical finding key/id separately.

Maintain `finding_key` as stable semantic identity.

Two audits with the same `finding_key` must persist independently.

## 3.3 Coverage identity

Make requirement coverage row IDs audit-scoped/globally unique.

Preserve requirementRef as the semantic/logical identity.

Two audits with `coverage-0` or same requirementRef must not collide.

## 3.4 Reference integrity

Before transaction commit validate:

- every finding evidence ref resolves within current audit logical evidence;
- every coverage evidence ref resolves within current audit logical evidence;
- every disposition evidence ref resolves within current audit logical evidence;
- no cross-audit resolution;
- duplicate same-audit logical evidence IDs rejected/deduplicated deterministically.

Do not permit dangling evidence refs.

---

# 4. R65 — deterministic semantic validator

Add one central semantic validation/normalization stage after structured model parsing and before persistence.

The model may propose an audit judgment. Production code owns final admissibility.

## 4.1 Verdict invariants

`PASS` is forbidden if any of the following is true:

- unresolved BLOCKER finding;
- unresolved MAJOR finding;
- required requirement coverage is FAILED;
- required requirement coverage is UNVERIFIED;
- required requirement coverage is PARTIAL where the requirement cannot be proven;
- required evidence class needed for the judgment is UNAVAILABLE;
- required evidence is STALE;
- required evidence is TRUNCATED and missing portion prevents proof;
- unresolved prior release-blocking finding remains;
- required evidence refs are invalid.

Choose deterministic downgrade semantics:

- FAIL for verified correctness/security/release-blocking failure;
- CONDITIONAL for insufficient/unavailable/unverified evidence.

Test exact rules.

## 4.2 Evidence quality

Builder-log CLAIM_ONLY evidence cannot independently:

- mark requirement VERIFIED;
- close a prior finding;
- justify PASS.

A model finding/coverage row citing CLAIM_ONLY only must remain unverified/conditional unless independently corroborated.

## 4.3 Reference validation

Validate all normal finding/coverage evidence refs, not only prior-disposition refs.

Unknown refs make model output semantically invalid or deterministically degraded. Never silently ignore.

## 4.4 Supersession validation

A `SUPERSEDED` prior disposition must reference a replacement finding key that actually exists in the current audit's finding set.

---

# 5. R66 — re-audit unresolved-state truth and actionability

Separate lifecycle status from disposition metadata.

Recommended lifecycle:

- OPEN
- CLOSED
- SUPERSEDED

Disposition metadata:

- STILL_OPEN
- CLOSED
- SUPERSEDED

A prior finding disposition of STILL_OPEN must produce/retain:

- lifecycle `status = OPEN`;
- `disposition = STILL_OPEN`;
- current-audit provenance;
- remediation eligibility.

## 5.1 Omitted prior findings

For an AVAILABLE re-audit:

- omission must never close;
- unresolved prior release blockers must prevent PASS;
- current audit must make unresolved inherited state visible/resolvable.

Implement one deterministic strategy:

A. carry forward omitted prior OPEN findings as inherited OPEN rows in the current audit, or
B. persist a dedicated unresolved-prior structure and make verdict/UI/remediation logic consume it.

Strategy A is preferred because it keeps current Audit Center findings actionable.

## 5.2 Remediation eligibility

Audit Center and native remediation selection must accept lifecycle OPEN inherited/STILL_OPEN findings.

Do not require the user to select an older audit simply because the latest audit confirmed the finding remains open.

---

# 6. R67 — reproducible Git implementation scope

The Audit Engine must be able to audit:

- working-tree changes;
- staged/index changes;
- committed implementation changes.

## 6.1 Audit target contract

Add a typed bounded Git audit target, derived from registered/project/session/prompt authority, not arbitrary raw Git CLI from frontend.

Support at least:

- WORKING_TREE
- STAGED
- COMMIT_RANGE

For COMMIT_RANGE persist:

- validated base SHA/ref resolution;
- audited HEAD SHA;
- changed files;
- bounded diff;
- internal full diff/change-set identity.

## 6.2 Base/HEAD derivation

Prefer authoritative sources:

- audited Agent session/prompt provenance where available;
- explicit registered branch/base policy;
- prior audit audited HEAD for remediation re-audit where appropriate.

Do not trust arbitrary unvalidated user-provided ref text.

## 6.3 Audit provenance

Extend AuditRun persistence to link the audited implementation where available:

- audited agent session ID;
- audited prompt ID/version ID;
- audited base SHA;
- audited HEAD SHA;
- scope kind.

M16.04 requires project/task/session/test evidence linkage.

## 6.4 Git Engine

Extend the bounded Git Engine with a narrow commit-range diff operation if necessary.

No arbitrary Git command frontend primitive.

---

# 7. R68 — complete freshness identity

Freshness must use full internal identities, not only bounded display diff.

Include content-sensitive identity for:

- branch;
- HEAD;
- base SHA where relevant;
- staged/index state/content;
- working-tree tracked changes;
- selected/untracked evidence file content;
- conflicts;
- committed range identity;
- repository identity.

Do not hash only a truncated diff.

Recommended internal primitives may include:

- index tree hash / staged diff hash;
- full diff hash computed before truncation;
- selected changed-file content SHA;
- untracked selected-file SHA;
- base..HEAD tree/range identity.

Displayed/model evidence remains bounded. Freshness identity may use hashes of full local data without transporting that data.

Add mutation-during-model tests for:

- staged content same path;
- untracked content same path;
- working diff mutation beyond displayed truncation;
- committed HEAD/range change;
- unchanged identity.

STALE must dominate all provider statuses and must never apply prior dispositions.

---

# 8. R69 — claim-directed direct source/test verification

Replace file-level heuristic-only verification with a bounded evidence planner.

## 8.1 Production symbols

From:

- Git diff hunks;
- task acceptance criteria;
- finding locators;
- relevant governance/config targets;

identify specific production files/symbols/ranges.

Read bounded windows around selected symbol/hunk/range, not just first 8KB.

## 8.2 Test bodies

When test evidence claims a suite/test:

- derive test file/name where possible from durable test metadata;
- locate exact definition/range;
- inspect direct test body;
- identify relevant assertions;
- identify whether the tested production boundary is real or mocked;
- distinguish unrelated mocks elsewhere in the file.

Do not mark a test CORROBORATED merely because the file contains any `assert`/`expect`.

Do not mark it PARTIAL merely because another test in the file contains a mock.

If exact mapping is impossible, remain UNVERIFIED.

## 8.3 Misleading test detection

Cover:

- overclaiming test name;
- no relevant assertion;
- fully mocked production boundary;
- skipped/ignored target test;
- helper-only test claimed as production proof;
- test count claim inconsistent with durable test result;
- filename-only claim.

---

# 9. R70 + R71 — task authority correctness

## 9.1 UI

Replace free-text Task ID input with the same project-neutral workflow task-picker pattern already accepted in Prompt Engine.

Include:

- Freeform project audit option;
- readable long-title handling;
- exact task ID as underlying value;
- no project-specific branches.

## 9.2 Native validation

If taskId is supplied:

- require exact task exists;
- require task belongs to selected project;
- reject unknown/wrong-project ID immediately;
- never silently reinterpret it as freeform project audit.

## 9.3 Task-intelligence availability

Do not swallow material task intelligence errors with `.ok()` and then label empty requirements VERIFIED.

Represent separately:

- workflow task authority available;
- task-intelligence parse available/unavailable;
- requirements provenance available/unavailable.

Only mark requirements VERIFIED when the responsible authority succeeded.

If workflow task exists but acceptance criteria authority is unavailable:

- task identity may remain VERIFIED;
- requirements evidence must be PARTIAL/UNAVAILABLE/UNVERIFIED.

---

# 10. R72 + R73 — builder-log containment and relevance

## 10.1 Root containment

Canonicalize:

- registered root;
- builder-log directory;
- each candidate log file.

Read a candidate only if canonical path remains within canonical registered root.

Reject:

- symlink escape;
- junction escape;
- outside-root file target.

Record EXCLUDED metadata without reading external content.

## 10.2 Selection

Do not alphabetically take the first four.

Priority:

1. builder log explicitly linked to audited prompt/session/task if durable provenance exists;
2. relevant milestone/task/project logs;
3. newest bounded project logs as fallback.

All remain CLAIM_ONLY.

---

# 11. Persistence migration requirements

Use additive migration(s), preserving schema v14 data.

Migration must:

- add logical evidence identity;
- backfill safely;
- add indexes/uniqueness;
- support audit-scoped finding/coverage row identities;
- preserve existing audit history;
- not rewrite historical verdicts/findings;
- be upgrade-safe from v13 and v14;
- be idempotent through migration history semantics.

No destructive table reset.

---

# 12. Audit Center requirements

Preserve current accepted overall style.

Required updates:

- workflow task picker;
- explicit audit Git target/scope summary;
- audited base/HEAD;
- unresolved inherited findings visible/actionable;
- evidence technical details show logical evidence identity;
- linked audited session/prompt where available;
- clear STALE/UNAVAILABLE/UNVERIFIED states.

Do not turn primary UI into raw JSON.

---

# 13. Security/truthfulness sweep

Before completion, adversarially verify entire M16 for:

- path traversal;
- symlink/junction escapes;
- secret file reads;
- secret content redaction;
- prompt-injection repository text;
- wrong-project task/session/audit refs;
- cross-audit evidence resolution;
- stale repository mutations;
- dangling evidence refs;
- malformed model output;
- model PASS over unverified evidence;
- builder CLAIM_ONLY influence;
- huge diff/source/test content;
- duplicate IDs;
- DB transaction rollback.

Do not stop after all named findings pass. Perform one final adversarial sweep for adjacent defects introduced by the remediation.

---

# 14. Required direct tests

Add explicit production/domain tests for at least:

1. two audits reuse same logical evidence ID safely;
2. two audits reuse same finding key safely;
3. two audits reuse same coverage index/ref safely;
4. finding refs resolve to same-audit evidence;
5. coverage refs resolve;
6. disposition refs resolve;
7. cross-audit evidence resolution rejected;
8. dangling ref rejected;
9. PASS + MAJOR rejected/downgraded;
10. PASS + BLOCKER rejected/downgraded;
11. PASS + FAILED coverage rejected/downgraded;
12. PASS + UNVERIFIED required coverage rejected/downgraded;
13. CLAIM_ONLY-only proof cannot verify requirement;
14. unknown evidence ref rejected;
15. SUPERSEDED replacement key must exist;
16. prior MAJOR omitted cannot yield PASS;
17. STILL_OPEN remains lifecycle OPEN;
18. STILL_OPEN current audit finding is remediation-selectable;
19. degraded UNAVAILABLE re-audit persists;
20. malformed re-audit persists;
21. stale degraded re-audit stays STALE;
22. staged diff is collected;
23. committed range diff is collected;
24. clean working tree with committed implementation still has auditable committed change set;
25. staged content mutation same path triggers STALE;
26. untracked content mutation same path triggers STALE;
27. mutation beyond displayed diff truncation triggers STALE;
28. exact test definition range found;
29. unrelated mock elsewhere does not downgrade target test;
30. target test with mocked production path marked PARTIAL/UNVERIFIED;
31. test with no relevant assertion not corroborated;
32. skipped exact target test not proof;
33. invalid task ID rejected before persistence;
34. wrong-project task rejected;
35. task intelligence failure does not produce VERIFIED requirements;
36. builder-log directory symlink/junction escape rejected;
37. builder-log child symlink escape rejected;
38. relevant/recent builder log selection;
39. R60 exact remediation-session provenance remains green;
40. M15 single-use dispatch remains green.

---

# 15. Explicit execution gates

## Preflight
1. Read `H!veAI/GPT.md`.
2. Fetch `origin/H!veAI`.
3. Fast-forward-only synchronize.
4. Confirm exact branch `H!veAI`.
5. Record starting HEAD/worktree.
6. Preserve unrelated files.
7. Read M16 unified prompt.
8. Read M16/M16A/M16B logs.
9. Read all M16 independent audits.
10. Read comprehensive M16 audit.
11. Confirm older M16C prompt is superseded and must not be executed separately.
12. Confirm M16 OPEN.
13. Confirm M17 blocked.
14. Confirm M21 not started.

## Reproduction
15. Reproduce R63.
16. Reproduce R64 finding collision.
17. Reproduce R64 coverage collision.
18. Reproduce PASS+MAJOR semantic hole.
19. Reproduce dangling normal evidence ref.
20. Reproduce unresolved prior PASS hole.
21. Reproduce STILL_OPEN remediation ineligibility.
22. Reproduce staged diff omission.
23. Reproduce committed diff omission.
24. Reproduce staged-content freshness hole.
25. Reproduce untracked-content freshness hole.
26. Reproduce truncation freshness hole.
27. Reproduce non-claim-directed test classification.
28. Reproduce invalid task ID failure.
29. Reproduce task-intelligence false VERIFIED case.
30. Reproduce builder-log root escape where platform supports.

## Identity/persistence
31. Add additive migration.
32. Add logical evidence ID.
33. Backfill old evidence IDs.
34. Add same-audit evidence uniqueness.
35. Audit-scope finding persisted IDs.
36. Audit-scope coverage persisted IDs.
37. Preserve semantic finding_key.
38. Preserve semantic requirementRef.
39. Update Rust DTOs.
40. Update TS DTOs.
41. Implement same-audit evidence resolver.
42. Validate finding refs.
43. Validate coverage refs.
44. Validate disposition refs.
45. Reject dangling refs.
46. Reject cross-audit refs.
47. Add migration compatibility tests.

## Semantic validator
48. Implement central post-model semantic validation.
49. Enforce PASS blocker rule.
50. Enforce PASS major rule.
51. Enforce coverage sufficiency rule.
52. Enforce evidence availability rule.
53. Enforce CLAIM_ONLY rule.
54. Enforce ref integrity.
55. Enforce superseded replacement existence.
56. Bound confidence downgrades.
57. Bound regression risk normalization.
58. Preserve malformed/unavailable truthfulness.

## Re-audit
59. Separate lifecycle status/disposition.
60. Make STILL_OPEN lifecycle OPEN.
61. Carry/represent unresolved omitted prior findings.
62. Prevent PASS with unresolved release blocker.
63. Preserve explicit CLOSED.
64. Preserve explicit SUPERSEDED.
65. Preserve omission-never-closes.
66. Make inherited current OPEN findings remediation-eligible.
67. Preserve immutable prior rows.

## Git scope
68. Define typed audit Git target.
69. Add working tree scope.
70. Add staged scope.
71. Add commit-range scope.
72. Validate base/HEAD authority.
73. Persist scope/base/HEAD.
74. Link audited prompt/session provenance.
75. Collect changed files for each scope.
76. Collect bounded display diff.
77. Compute full internal change identity.
78. Add Git Engine narrow command(s) if required.
79. Do not expose arbitrary Git argv.

## Freshness
80. Include branch.
81. Include HEAD.
82. Include base/range.
83. Include staged/index content identity.
84. Include working tracked content identity.
85. Include untracked selected-file content identity.
86. Include conflict identity.
87. Use full internal diff/change hash before truncation.
88. Recompute after model evaluation.
89. STALE on any material identity change.
90. Apply zero prior dispositions when STALE.
91. Add mutation tests.

## Source/test planner
92. Implement claim-directed source planner.
93. Locate diff hunks/symbols.
94. Read bounded symbol windows.
95. Parse durable test claim metadata.
96. Resolve test file/name where possible.
97. Locate exact test body range.
98. Classify exact test, not whole file.
99. Detect relevant assertion.
100. Detect mocked production boundary.
101. Detect skipped target test.
102. Leave unresolved mapping UNVERIFIED.
103. Add misleading-test fixtures.

## Task target
104. Add reusable workflow task picker to Audit Center.
105. Preserve freeform project audit option.
106. Validate native task ownership.
107. Reject unknown task.
108. Reject wrong-project task.
109. Separate task identity from task-intelligence availability.
110. Remove false VERIFIED task requirements.
111. Add degraded task authority UI/evidence.

## Builder logs
112. Canonicalize builder log root.
113. Canonicalize each file.
114. Enforce registered-root containment.
115. Reject symlink/junction escape.
116. Preserve CLAIM_ONLY.
117. Prefer explicitly linked/relevant log.
118. Use recent fallback rather than alphabetical first four.

## UI/provenance
119. Show audit scope.
120. Show base/HEAD.
121. Show audited session/prompt provenance if present.
122. Show inherited OPEN findings.
123. Preserve current verdict panel.
124. Preserve coverage/confidence/risk.
125. Preserve history chain.
126. Preserve remediation Prompt Engine handoff.
127. Preserve linked remediation session Agents handoff.
128. Preserve collapsed technical details.

## Focused tests
129. Run identity/reference tests.
130. Run semantic validator tests.
131. Run prior-finding lifecycle tests.
132. Run Git scope tests.
133. Run freshness mutation tests.
134. Run direct test-body planner tests.
135. Run task target tests.
136. Run task-intelligence degraded tests.
137. Run builder-log containment tests.
138. Run remediation provenance tests.
139. Run degraded re-audit tests.

## Full regression
140. Run full serialized Rust suite.
141. Run full frontend suite.
142. Run migration suite.
143. Run TypeScript typecheck.
144. Run frontend production build.
145. Run `npm audit --audit-level=high`.
146. Run Rust fmt.
147. Run Rust all-targets.
148. Run Rust pty-support.
149. Run `git diff --check`.
150. Run M14E regressions.
151. Run all M15 regressions.
152. Run M15A replay/race.
153. Run M15B ACL/provider/task-picker.
154. Run M15C/D handoff/result.
155. Run R59-R62 regressions.
156. Run all R63-R73 focused regressions.

## Final adversarial sweep
157. Inspect actual production source after tests.
158. Inspect direct new test bodies, not names only.
159. Inspect migration SQL directly.
160. Inspect ACL/capability diff directly.
161. Inspect final Git diff/changed files.
162. Search for duplicate-ID assumptions.
163. Search for dangling evidence namespaces.
164. Search for remaining `.ok()` / `unwrap_or_default()` that can silently downgrade required audit authority.
165. Search for uncanonicalized audit file reads.
166. Search for PASS paths bypassing semantic validator.
167. Search for re-audit states bypassing prior finding truth.
168. Search for working-tree-only assumptions.
169. Search for stale token inputs using truncated data.
170. If any adjacent MAJOR/BLOCKER is discovered, fix it in this same run and add a regression before continuing.

## Publication
171. Run publisher rollback harness.
172. Governed production Tauri `--no-bundle`.
173. Publish stable `H!veAI.exe`.
174. Verify candidate/stable SHA equality.
175. Verify PE target.
176. Verify shortcut/icon/startup.
177. Verify no console popup.

## Native acceptance preparation
178. Native-open Audit Center if feasible.
179. Verify project/task picker.
180. Verify UNAVAILABLE Start audit.
181. Verify UNAVAILABLE Re-audit creates history row.
182. Verify latest inherited OPEN finding stays actionable where fixture exists.
183. Verify technical evidence identifiers are readable/resolvable.
184. Verify remediation handoff remains human-reviewed.
185. Record exact native limitations if CUA unavailable.

## Evidence/commit
186. Create immutable comprehensive remediation log.
187. Record all findings R63-R73 before/after.
188. Record migration version/schema.
189. Record test counts.
190. Record adversarial sweep results.
191. Record publication SHA/size.
192. Record actual implementation commit.
193. Commit scoped files only.
194. Push normally, no force.
195. Verify local/origin equality.
196. Leave M16 OPEN pending independent **whole-M16** strict re-audit + user native/visual acceptance.
197. Do not activate M17.
198. Do not start M21.

---

# 16. Required log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16C_REV2_COMPREHENSIVE_WHOLE_MILESTONE_CLOSURE_REMEDIATION_LOG.md`

The log must include:

- starting HEAD;
- actual implementation commit;
- final pushed HEAD;
- exact changed files;
- R63-R73 reproduction evidence;
- R63-R73 final production behavior;
- migration/schema changes;
- identity/reference model;
- semantic validator truth table;
- prior finding lifecycle contract;
- Git audit target/scope contract;
- freshness identity fields;
- test-body planner behavior;
- task authority behavior;
- builder-log containment behavior;
- direct focused tests and counts;
- full regressions and counts;
- final adversarial sweep findings/fixes;
- publisher hashes;
- native evidence/limitations.

End exactly with equivalent truthful state:

`M16C REV2 COMPREHENSIVE REMEDIATION COMPLETE / PENDING INDEPENDENT WHOLE-M16 STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`

---

# 17. Completion boundary

This is a **whole-M16 closure remediation**, not an R63-only patch.

Do not stop after the first fixed defect.

Do not ask for another prompt between findings.

Before finishing, execute the final adversarial sweep and fix any adjacent release-blocking M16 defect discovered by that sweep in this same run.

Do not self-close M16.
