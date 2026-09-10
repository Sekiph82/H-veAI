# M16D — Whole-M16 Final Closure Remediation Prompt

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`
Scope: close all findings from `M16C_REV2_INDEPENDENT_WHOLE_M16_STRICT_REAUDIT.md`
Authority: single consolidated whole-milestone remediation prompt

## 0. Execution mode

This is one continuous M16 remediation run.

Do not stop after R74, R75, or any individual finding.
Do not ask for another prompt between findings.

Close all:

- M16-R74
- M16-R75
- M16-R76
- M16-R77
- M16-R78
- M16-R79
- M16-R80
- M16-R81

Then perform one fresh whole-M16 adversarial sweep. If that sweep reveals an adjacent BLOCKER or MAJOR, fix it in the same run and add a direct regression before completing.

M15 remains PASS/CLOSED.
M16 remains OPEN until independent whole-M16 strict re-audit + user native/visual acceptance.
M17 MUST NOT activate.
M21 MUST NOT start.
Roadmap completed progress remains `16 / 20 = 80%`.

---

# 1. Required preflight authority

Before coding, read in full:

- `H!veAI/GPT.md`
- M16 unified implementation prompt
- M16 implementation log
- M16 comprehensive whole-milestone audit
- M16A prompt/log/re-audit
- M16B prompt/log/re-audit
- M16C REV2 prompt/log
- `M16C_REV2_INDEPENDENT_WHOLE_M16_STRICT_REAUDIT.md`

Builder logs remain CLAIM evidence only.

---

# 2. R74 — required-evidence semantic truth instead of global veto

Refactor `validate_semantic_evaluation(...)` so verdict admissibility is based on evidence actually required to prove requirements/findings and on mandatory authority completeness.

Do not let any unrelated weak evidence row veto PASS globally.

Required rules:

- CLAIM_ONLY cannot satisfy VERIFIED coverage.
- An unrelated builder-log CLAIM_ONLY row does not veto PASS.
- Project/freeform task-not-applicable evidence does not veto project-level PASS.
- PARTIAL/TRUNCATED/UNAVAILABLE evidence only affects verdict if it is referenced by a required coverage/finding or represents a required missing authority.
- Required missing task requirements authority still prevents task-level PASS.
- HIGH confidence must downgrade when the evidence supporting the actual judgment is weak.
- Regression risk must remain deterministic.

Add a fixture:

`verified required evidence + unrelated builder CLAIM_ONLY → PASS remains eligible`.

Also test:

`verified project audit + task-specific NOT_APPLICABLE/UNAVAILABLE marker → PASS remains eligible when project requirements are fully verified`.

---

# 3. R75 — complete canonical requirement coverage

Create a canonical requirement set before model execution.

For task audits:

- derive deterministic IDs from ordered task acceptance criteria;
- persist/include them in audit input;
- require exactly one coverage row for every required criterion;
- reject/downgrade missing coverage;
- reject unknown coverage refs;
- reject duplicates;
- validate finding requirementRefs against the canonical requirement set.

Suggested contract:

- `task-requirement-0`
- `task-requirement-1`
- ...

Do not let model invent requirement identity.

For project/freeform audits:

- use explicit project-level/governance requirements if product authority provides them;
- otherwise task requirements are NOT_APPLICABLE, not silently missing.

PASS requires complete required coverage.

Add direct tests for:

- all requirements covered → eligible;
- one omitted requirement → CONDITIONAL/FAIL according to evidence;
- unknown requirementRef rejected;
- duplicate coverage rejected;
- finding cites unknown requirementRef rejected.

---

# 4. R76 — immutable re-audit target compatibility

Re-audit must be a re-evaluation of the intended prior target, not an arbitrary new audit with a priorAuditId.

## 4.1 Native authority

When `prior_audit_id` is supplied, load the prior audit before collecting/persisting.

Enforce:

- same project;
- same taskId, including null/freeform identity;
- compatible Git scope;
- compatible base/head semantics;
- compatible audited session/prompt provenance when present.

Preferred rule:

- inherit immutable target from prior audit by default;
- allow only deterministic target evolution needed for remediation:
  - prior audited base remains stable;
  - audited HEAD may advance to current remediation result under explicit re-audit contract;
  - session/prompt target must match linked remediation provenance when transitioning from pre-remediation audit to post-remediation re-audit.

Do not let frontend bypass native validation.

## 4.2 UI

`Re-audit` must use the selected audit's immutable target/provenance, not mutable current form controls.

If the UI allows “start new audit with different target,” that must be a separate Start audit action without priorAuditId.

Add tests:

- task A prior → task B re-audit rejected;
- task prior → freeform re-audit rejected;
- incompatible scope rejected;
- compatible remediation HEAD advance accepted;
- selected audit target restored/used by Re-audit.

---

# 5. R77 — inherited prior evidence provenance

Do not copy prior logical evidence refs into the current evidence namespace unless the evidence is actually recollected or explicitly imported with provenance.

Required design:

- inherited prior finding remains lifecycle OPEN;
- store prior audit ID/finding key;
- current `evidenceRefs` contains only current-audit logical IDs;
- prior evidence linkage is stored separately as inherited provenance, OR prior evidence is copied into typed `INHERITED_EVIDENCE` rows with:
  - current audit-scoped logical ID;
  - prior audit ID;
  - prior logical evidence ID;
  - explicit verification semantics.

Omission must never fail simply because old evidence IDs are absent in current input.

STILL_OPEN without current proof must remain OPEN and may use inherited provenance without masquerading as current verification.

Add tests:

- prior evidence logical ID absent in current input;
- omission persists inherited OPEN finding;
- no dangling current evidence refs;
- remediation eligibility remains;
- later explicit closure requires current validated evidence.

---

# 6. R78 — scope-pure evidence planning and freshness

Make audit source evidence match the selected Git scope.

## WORKING_TREE

Use:

- unstaged tracked changes;
- untracked files;
- any explicitly included staged context only if separately labeled.

## STAGED

Use:

- index/staged changed files;
- staged content;
- do not silently include unstaged/untracked source as implementation proof.

## COMMIT_RANGE

Use:

- base..HEAD changed files;
- committed range content;
- do not silently include current working-tree changes as implementation proof.

Contextual evidence outside scope may be included only if:

- typed as context;
- clearly not used as direct implementation proof;
- freshness covers it if transported to model.

Refactor `changed_paths`/source planner to be target-aware.

Add scope-isolation tests.

---

# 7. R79 — bounded/streaming readers and hashes

Remove unbounded whole-file allocation from M16 evidence paths.

## Exact test-body locator

Do not `fs::read_to_string` an unlimited file.

Use:

- buffered line streaming;
- fixed max scan bytes/lines/time;
- exact name match;
- bounded body window.

If target definition lies beyond scan bound:

- return UNVERIFIED/TRUNCATED evidence;
- do not allocate entire file.

## Builder logs

Use buffered bounded read.

- max bytes per log;
- max total builder claim bytes;
- truncation explicit;
- canonical containment remains.

## Untracked freshness

Use streaming SHA-256 per file.

Define:

- max file count;
- max total bytes/time for identity calculation;
- deterministic overflow state.

If complete content identity cannot be safely computed within policy:

- do not pretend freshness is complete;
- return explicit bounded error/UNAVAILABLE identity state or mark audit unable to claim fresh PASS.

No full `fs::read` on arbitrary huge files.

---

# 8. R80 — large Git change sets must degrade truthfully

Refactor Git diff identity/display separation.

Current requirement:

- full change identity must not depend on retaining the entire raw diff in memory;
- display/model diff remains bounded to MAX_DIFF_BYTES/MAX_DIFF_LINES;
- legitimate >8 MiB change sets must not fail merely because display output exceeds internal command buffer.

Preferred approaches:

- stream Git stdout into:
  - SHA-256 hasher;
  - bounded display buffer;
  - changed-file collector;
- or use Git-native object/tree/index identities plus separate bounded diff command.

Support all:

- WORKING_TREE
- STAGED
- COMMIT_RANGE

Overlarge result must expose explicit `truncated = true` while retaining reliable freshness/change identity.

Add >current-limit fixtures for each scope without allocating absurd test sizes.

---

# 9. R81 — authority-derived commit-range target UX

Do not make free-text Base ref / Head SHA the primary commit-range workflow.

Add bounded presets sourced from actual H!veAI authority where available:

- selected Agent implementation session;
- selected prompt/version provenance;
- prior audit;
- registered branch/base policy.

UI should show human-readable choices such as:

- “Current working tree”
- “Staged changes”
- “Implementation session <short-id>”
- “Previous audit → current remediation”
- “Manual commit range (advanced)”

If manual mode remains:

- clearly label it advanced/manual;
- native validation resolves commits;
- persist `targetOrigin = MANUAL` or equivalent;
- do not imply the range is session-derived.

No arbitrary Git argv or path input.

---

# 10. Preserve R59-R73

Do not regress:

- omission never closes;
- exact remediation-session provenance;
- degraded UNAVAILABLE/MALFORMED persistence;
- audit-scoped IDs;
- semantic reference validation;
- task picker/native task ownership;
- builder-log root containment;
- current M14/M15 behavior.

---

# 11. Additional whole-M16 adversarial sweep

After implementing R74-R81, inspect actual production code for adjacent failures.

Mandatory sweep areas:

1. every `.ok()`, `unwrap_or_default()`, and swallowed error in audit authority paths;
2. every file read/hash path for bounds and root containment;
3. every PASS path for complete requirement/evidence validation;
4. every re-audit path for target/task/provenance consistency;
5. every persisted child ID/reference namespace;
6. every Git target scope for contamination;
7. every freshness token input for content sensitivity;
8. every frontend action that can mutate audit identity;
9. every native command ACL exposure;
10. every degraded provider state;
11. every DB transaction failure/rollback path;
12. every model-output semantic bypass.

If you find a new BLOCKER/MAJOR, fix it in this same run and record it in the log as `ADJ-M16-...`.

Do not stop and ask for another prompt.

---

# 12. Required tests

Add/update direct tests for at least:

1. verified required evidence + unrelated CLAIM_ONLY still allows PASS;
2. freeform NOT_APPLICABLE task marker does not veto project PASS;
3. missing one of N task requirements prevents PASS;
4. unknown coverage requirement ref rejected;
5. finding unknown requirement ref rejected;
6. task A → task B prior chain rejected;
7. task → freeform prior chain rejected;
8. incompatible Git scope re-audit rejected;
9. valid remediation re-audit target accepted;
10. Re-audit UI ignores unrelated current form changes;
11. omitted prior finding with absent prior evidence refs persists OPEN;
12. inherited prior refs never dangle in current evidence namespace;
13. explicit closure still requires current evidence;
14. WORKING_TREE source set excludes staged-only proof unless typed context;
15. STAGED source set excludes unstaged/untracked proof;
16. COMMIT_RANGE source set excludes dirty working tree proof;
17. source evidence mutation included in freshness for every transported evidence class;
18. huge test file is scanned within fixed bounds;
19. huge builder log is bounded before allocation;
20. huge untracked file is streamed/hashed within policy;
21. >limit working-tree diff truncates without audit failure;
22. >limit staged diff truncates without audit failure;
23. >limit commit-range diff truncates without audit failure;
24. full change identity changes beyond display truncation;
25. manual commit range persisted as MANUAL origin;
26. session-derived range validates exact session/prompt provenance;
27. wrong-project session target rejected;
28. R59-R73 suites remain green.

---

# 13. Explicit execution gates

## Preflight
1. Read `H!veAI/GPT.md`.
2. Fetch origin.
3. Fast-forward-only synchronize.
4. Confirm `H!veAI`.
5. Record HEAD/worktree.
6. Preserve unrelated files.
7. Read all M16 prompts/logs/audits.
8. Read M16C REV2 whole re-audit.
9. Confirm R74-R81 are the only named open findings.
10. Confirm M16 OPEN.
11. Confirm M17 blocked.
12. Confirm M21 not started.

## Reproduce
13. Reproduce global CLAIM_ONLY PASS veto.
14. Reproduce freeform task marker PASS veto.
15. Reproduce missing-coverage acceptance.
16. Reproduce unknown requirement ref acceptance.
17. Reproduce cross-task re-audit chain.
18. Reproduce incompatible-scope re-audit chain.
19. Reproduce inherited old evidence ref persistence failure.
20. Reproduce STAGED scope contamination.
21. Reproduce COMMIT_RANGE dirty-worktree contamination.
22. Reproduce unbounded exact-test file read.
23. Reproduce unbounded builder-log read.
24. Reproduce unbounded untracked file read/hash.
25. Reproduce >limit Git diff failure.
26. Record current manual commit-range UX.

## R74 semantic evidence
27. Refactor weak-evidence calculation.
28. Build required/referenced evidence quality set.
29. Exclude irrelevant CLAIM_ONLY from veto.
30. Keep CLAIM_ONLY unable to verify coverage.
31. Handle project/freeform task-not-applicable correctly.
32. Add PASS eligibility fixtures.

## R75 requirements
33. Build canonical task requirement IDs.
34. Add them to input/model contract.
35. Validate complete coverage.
36. Reject missing required coverage.
37. Reject unknown coverage refs.
38. Reject duplicate coverage.
39. Validate finding requirement refs.
40. Add task/project requirement fixtures.

## R76 re-audit target
41. Load prior immutable target natively.
42. Validate same project.
43. Validate same task/freeform identity.
44. Validate compatible scope.
45. Validate base/range compatibility.
46. Validate remediation provenance transition.
47. Make UI Re-audit use prior target.
48. Separate Start new audit from Re-audit.
49. Add target-chain tests.

## R77 inherited evidence
50. Define inherited evidence/provenance contract.
51. Remove old-ref-as-current-ref fallback.
52. Preserve prior audit/finding linkage.
53. Keep omitted finding OPEN.
54. Keep STILL_OPEN lifecycle OPEN.
55. Require current evidence for closure.
56. Add absent-old-evidence tests.
57. Verify remediation selection.

## R78 scope purity
58. Make changed-source planner scope-aware.
59. WORKING_TREE source set.
60. STAGED source set.
61. COMMIT_RANGE source set.
62. Type contextual out-of-scope evidence if retained.
63. Include any transported context in freshness.
64. Add contamination tests.

## R79 bounded readers
65. Add bounded test-file scanner constants.
66. Implement buffered test definition scan.
67. Implement bounded test body window.
68. Explicit UNVERIFIED/TRUNCATED on scan overflow.
69. Add bounded builder-log reader.
70. Add builder per-file limit.
71. Add builder total limit.
72. Stream untracked file hashes.
73. Add untracked count/byte/time bounds.
74. Add truthful overflow behavior.
75. Add stress fixtures.

## R80 Git large-diff
76. Separate full identity from display buffer.
77. Stream/hash working-tree diff.
78. Stream/hash staged diff.
79. Stream/hash commit-range diff.
80. Preserve binary sanitization.
81. Preserve changed-file metadata.
82. Preserve display bounds.
83. Return truncated evidence, not OUTPUT_LIMIT for legitimate large change.
84. Add large diff tests.
85. Re-run freshness mutation tests.

## R81 target UX
86. Add authority-derived target presets.
87. Add implementation-session option.
88. Add prior-audit/remediation option.
89. Add registered policy option where applicable.
90. Make manual range advanced.
91. Persist target origin.
92. Validate session/prompt-derived target natively.
93. Add UI/native tests.

## Whole M16 persistence/security
94. Verify migration need.
95. Add additive migration only if schema changes required.
96. Preserve historical audits.
97. Verify all child refs resolve.
98. Verify no cross-audit leakage.
99. Verify task/session wrong-project isolation.
100. Verify source/log root containment.
101. Verify secrets stay redacted.
102. Verify no broad ACL addition.

## Focused tests
103. Run R74 tests.
104. Run R75 tests.
105. Run R76 tests.
106. Run R77 tests.
107. Run R78 tests.
108. Run R79 tests.
109. Run R80 tests.
110. Run R81 tests.
111. Run R59-R73 regression suite.
112. Run Audit Center focused frontend.
113. Run Prompt Engine remediation provenance.
114. Run Git Engine focused suite.
115. Run migration suite.

## Full regression
116. Run full serialized Rust suite.
117. Run full frontend suite.
118. TypeScript typecheck.
119. Frontend production build.
120. npm high audit.
121. Rust fmt.
122. Rust all-targets.
123. Rust pty-support serialized.
124. `git diff --check`.
125. M14E regression.
126. all M15 regressions.
127. publisher rollback harness.

## Final adversarial sweep
128. Inspect production audit source line by line around changed paths.
129. Inspect new test bodies directly.
130. Inspect migration SQL directly.
131. Inspect Git Engine production diff path.
132. Inspect Audit Center re-audit path.
133. Inspect Prompt Engine provenance path.
134. Search swallowed authority errors.
135. Search unbounded file reads.
136. Search global weak-evidence assumptions.
137. Search missing requirement completeness.
138. Search priorAuditId without target validation.
139. Search scope-independent changed paths.
140. Search freshness using display-truncated data.
141. Search dangling prior evidence refs.
142. Search arbitrary session/task cross-project joins.
143. Fix any new BLOCKER/MAJOR in same run.
144. Add direct regression for each adjacent fix.

## Publication
145. Governed Tauri no-bundle release build.
146. Publisher rollback 9/9.
147. Publish stable EXE.
148. Verify candidate/stable SHA equality.
149. Verify PE subsystem.
150. Verify shortcut/icon/startup.
151. Verify no visible console popup.

## Native preparation
152. Native-open Audit Center if feasible.
153. Verify task picker.
154. Verify target preset UX.
155. Verify UNAVAILABLE Start.
156. Verify UNAVAILABLE Re-audit.
157. Verify immutable prior target on Re-audit.
158. Verify history remains readable.
159. Verify remediation handoff.
160. Record user-owned pending acceptance truthfully.

## Evidence/commit
161. Create immutable M16D log.
162. Record R74-R81 before/after.
163. Record any ADJ-M16 fixes.
164. Record schema changes.
165. Record bounds/constants.
166. Record focused test counts.
167. Record full regression counts.
168. Record adversarial sweep.
169. Record candidate/stable hashes.
170. Record implementation commit.
171. Commit scoped files only.
172. Push normally.
173. Verify local/origin equality.
174. Leave M16 OPEN pending independent whole-M16 strict re-audit + user native acceptance.
175. Do not activate M17.
176. Do not start M21.

---

# 14. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16D_WHOLE_M16_FINAL_CLOSURE_REMEDIATION_LOG.md`

Record:

- starting HEAD;
- implementation commit;
- final pushed HEAD;
- changed files;
- exact R74-R81 reproductions;
- exact final contracts;
- canonical requirement model;
- re-audit target compatibility rules;
- inherited-evidence provenance;
- Git scope isolation;
- freshness identity;
- bounded reader/hash constants;
- large-diff handling;
- target-origin UX;
- all direct test names/counts;
- all regressions;
- final adversarial sweep and any adjacent fixes;
- publisher SHA/size;
- native evidence/limitations.

End with:

`M16D WHOLE-M16 REMEDIATION COMPLETE / PENDING INDEPENDENT WHOLE-M16 STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
