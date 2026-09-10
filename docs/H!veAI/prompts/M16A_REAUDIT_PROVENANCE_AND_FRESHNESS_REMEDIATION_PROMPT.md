# M16A GPT Audit Engine Strict Remediation Prompt

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Scope: close M16-R59 through M16-R61 only
Authority: authoritative remediation prompt

## 0. Starting state

Independent strict audit verdict for M16: **FAIL / CHANGES REQUIRED**.

Open findings:

- **M16-R59 MAJOR** — re-audit closes omitted prior findings without explicit proof.
- **M16-R60 MAJOR** — remediation session provenance is not wired end-to-end and native linking accepts any same-project session.
- **M16-R61 MAJOR** — repository freshness is not enforced between evidence collection and final audit persistence.

Do not redesign unrelated M16 functionality.
Do not add a production GPT provider in this remediation.
Do not activate M17.
Do not start M21.

M15 remains PASS/CLOSED.
M16 remains OPEN.
Roadmap completed progress remains `16 / 20 = 80%`.

---

# 1. R59 — explicit re-audit finding disposition

Replace omission-based closure with explicit proof-based re-audit disposition.

## Required model/result contract

Add a typed prior-finding disposition structure for re-audits, for example:

- `priorFindingKey`
- `disposition: STILL_OPEN | CLOSED | SUPERSEDED`
- `evidenceRefs`
- `rationale`

Requirements:

- every disposition must reference a real OPEN prior finding;
- duplicate prior-finding dispositions are rejected;
- unknown prior finding keys are rejected;
- every referenced evidence ID must exist in the current bounded audit input;
- CLOSED and SUPERSEDED require non-empty current evidence refs and rationale;
- absence from the current model's normal `findings` list does **not** close anything;
- a prior finding with no explicit disposition remains unresolved;
- UNAVAILABLE, MALFORMED, STALE, FAILED evaluation never closes prior findings.

## Persistence/history

Preserve immutable prior audit rows.

In the new audit:

- STILL_OPEN should be represented truthfully;
- CLOSED should preserve prior finding identity/reference and closure evidence;
- SUPERSEDED should identify replacement finding/reference where applicable;
- closure must be attributable to the current audit.

Do not mutate historical audit content to pretend the original finding was never open.

## Tests

Add direct tests for:

- omitted prior finding remains unresolved;
- explicit CLOSED with valid current evidence closes;
- explicit CLOSED with missing/unknown evidence is rejected;
- unknown prior finding key rejected;
- duplicate disposition rejected;
- UNAVAILABLE/MALFORMED/STALE cannot close;
- STILL_OPEN preserved;
- SUPERSEDED semantics deterministic.

Remove/update the existing test that treats model omission as closure proof.

---

# 2. R60 — exact remediation-session provenance

Wire the actual Audit → Prompt Engine → Dispatch → Agent session chain.

## Audit Center → Prompt Engine

The existing remediation draft handoff already carries:

- projectId
- promptId
- auditId

Preserve this bounded handoff.

## Prompt Engine

When opening a remediation draft from Audit Center:

- parse and retain bounded `auditId`;
- validate the audit belongs to the selected registered project;
- validate the loaded prompt/version is exactly the audit's persisted remediation prompt/version;
- do not trust URL identity alone;
- do not auto-dispatch.

After successful Prompt Engine dispatch of that exact audit remediation prompt/version:

- call the native audit session-link operation with the exact returned session ID;
- if audit linking fails, surface a truthful bounded error/diagnostic and do not falsely claim the remediation chain is fully linked;
- do not create a second provider session.

## Native link command

Strengthen `link_remediation_session(...)`.

It must require all of:

- audit belongs to project;
- audit has non-null remediation_prompt_id;
- audit has non-null remediation_prompt_version_id;
- session belongs to project;
- session.prompt_id equals audit.remediation_prompt_id;
- session.prompt_version_id equals audit.remediation_prompt_version_id;
- session contains exact prompt-version provenance expected from Prompt Engine dispatch.

Reject:

- unrelated same-project session;
- session for another prompt;
- session for another version;
- session from another project;
- audit without a remediation prompt;
- stale/rewritten prompt association.

Persist `remediation_session_id` only after all checks pass.

## UI/history

Audit Center history/details should expose the linked remediation session when present and provide a bounded navigation action to Agents.

Navigation must not start a provider.

## Tests

Add direct tests proving:

- exact remediation session links successfully;
- unrelated same-project session is rejected;
- right prompt/wrong version rejected;
- wrong project rejected;
- no remediation prompt rejected;
- successful Prompt Engine dispatch links once;
- handoff/linking does not redispatch;
- Audit history shows linked session.

---

# 3. R61 — repository freshness enforcement

Audit results must not be persisted as normal COMPLETED results if repository evidence changed after collection.

## Freshness token

At input collection build a deterministic freshness identity that covers at least:

- current branch;
- HEAD SHA;
- staged file identities;
- unstaged file identities;
- untracked file identities;
- conflicted file identities;
- bounded working-tree diff hash;
- relevant Git snapshot identity.

Persist/include the token in the audit input manifest/provenance.

## Pre-persistence validation

Immediately after model evaluation and immediately before final audit persistence:

1. re-read Git authority for the same registered project;
2. recompute freshness identity;
3. compare to the collected identity.

If it changed:

- persist/return an audit with `state = STALE` according to a deterministic schema, OR fail without a normal audit verdict if that is the chosen contract;
- do not persist a normal COMPLETED PASS/CONDITIONAL/FAIL as though evidence were current;
- do not reconcile/close prior findings;
- expose a bounded diagnostic explaining repository evidence changed and audit must be re-run.

The same rule applies even when model status is UNAVAILABLE or MALFORMED. Stale evidence is stale regardless of provider availability.

## Required tests

Add production-path tests for:

- HEAD changes during evaluation;
- branch changes during evaluation;
- working-tree content/diff changes with same HEAD;
- staged changes with same HEAD;
- untracked-file set changes;
- unchanged repository remains eligible;
- stale re-audit cannot close prior finding;
- stale state persisted/retrieved truthfully.

The existing unused `current_head_is_fresh` helper should be replaced, extended, or removed. Do not leave a misleading partial helper that checks only HEAD.

---

# 4. Preserve all accepted M16 functionality

Do not regress:

- typed evidence classifications;
- builder logs CLAIM_ONLY;
- persisted tests UNVERIFIED;
- strict model schema parsing;
- bounded source/test evidence;
- secret/traversal policy;
- migration 13 compatibility;
- Audit Center structured UI;
- Prompt Engine review/edit/approve;
- M15A single-use dispatch;
- M15C/D handoff behavior;
- M14E final assistant response;
- narrow Tauri ACL;
- governed publication.

No arbitrary shell/process/file primitive.

---

# 5. Explicit execution gates

1. Fetch `origin/H!veAI`.
2. Fast-forward-only synchronize.
3. Confirm `H!veAI`.
4. Record starting HEAD/worktree.
5. Preserve unrelated files.
6. Read M16 authoritative prompt.
7. Read M16 builder log.
8. Read M16 independent strict audit.
9. Confirm R59/R60/R61 only.
10. Confirm M16 OPEN.
11. Confirm M17 blocked.
12. Confirm M21 not started.
13. Reproduce R59 from production source/test.
14. Reproduce R60 provenance gap.
15. Reproduce R61 freshness gap.
16. Design prior-finding disposition schema.
17. Implement strict disposition parser.
18. Validate prior finding keys.
19. Validate disposition evidence refs.
20. Remove omission-as-closure logic.
21. Preserve unresolved prior findings.
22. Implement explicit CLOSED semantics.
23. Implement STILL_OPEN semantics.
24. Implement SUPERSEDED semantics.
25. Add R59 focused tests.
26. Parse bounded auditId handoff in Prompt Engine.
27. Validate audit/project ownership before using handoff.
28. Validate exact remediation prompt ID.
29. Validate exact remediation prompt version ID.
30. Preserve no-auto-dispatch.
31. After successful exact dispatch, invoke audit-session linking once.
32. Strengthen native link validation.
33. Reject unrelated same-project session.
34. Reject wrong prompt.
35. Reject wrong version.
36. Reject wrong project.
37. Reject audit without remediation prompt.
38. Persist exact remediation_session_id only after validation.
39. Add Audit → Agents linked-session navigation.
40. Prove navigation does not launch provider.
41. Add R60 focused Rust tests.
42. Add R60 focused frontend tests.
43. Define deterministic repository freshness identity.
44. Include branch.
45. Include HEAD.
46. Include staged paths/state.
47. Include unstaged paths/state.
48. Include untracked paths.
49. Include conflicts.
50. Include bounded diff hash.
51. Collect identity with audit input.
52. Recompute identity after evaluation.
53. Compare immediately before persistence.
54. Mark changed evidence STALE.
55. Prevent normal COMPLETED verdict on stale evidence.
56. Prevent prior-finding closure on stale re-audit.
57. Add HEAD-change test.
58. Add branch-change test.
59. Add same-HEAD diff-change test.
60. Add staged-change test.
61. Add untracked-change test.
62. Add unchanged-evidence test.
63. Add stale persistence/retrieval test.
64. Remove/replace misleading HEAD-only helper.
65. Run focused M16A Rust tests.
66. Run focused M16 frontend tests.
67. Run migration tests.
68. Run full serialized Rust regression.
69. Run full frontend regression.
70. Run TypeScript typecheck.
71. Run frontend production build.
72. Run npm high audit.
73. Run Rust fmt.
74. Run Rust all-targets.
75. Run Rust pty-support.
76. Run `git diff --check`.
77. Run M14E regression.
78. Run M15 Prompt Engine regressions.
79. Run M15A duplicate/race/replay regression.
80. Run M15B ACL/provider/task-picker regression.
81. Run M15C/D handoff/result regression.
82. Run M16 known-good/known-bad fixtures.
83. Run M16 misleading-test fixtures.
84. Run M16 security/containment suite.
85. Run M16 re-audit history suite.
86. Run M16A provenance suite.
87. Run M16A freshness suite.
88. Run publisher rollback harness.
89. Governed-publish stable Tauri EXE.
90. Verify candidate/stable SHA equality.
91. Verify PE/shortcut/icon/startup.
92. Verify no visible console popup.
93. Native-open Audit Center if feasible.
94. Verify UNAVAILABLE state remains truthful.
95. Verify remediation draft handoff.
96. If safe, dispatch one controlled remediation and prove exact session link.
97. Verify linked session opens in Agents.
98. Verify no second provider launch from navigation/linking.
99. Create immutable M16A remediation log.
100. Record actual implementation commit.
101. Record all test counts.
102. Record publication SHA.
103. Commit scoped files only.
104. Push normally, no force.
105. Verify local/origin HEAD equality.
106. Leave M16 OPEN pending independent re-audit/user native acceptance.
107. Do not activate M17.
108. Do not start M21.

---

# 6. Required log

Create:

`H!veAI/docs/H!veAI/codex-logs/M16A_REAUDIT_PROVENANCE_AND_FRESHNESS_REMEDIATION_LOG.md`

Record:

- actual implementation commit SHA;
- R59 before/after behavior;
- exact prior-finding disposition schema;
- R60 exact remediation provenance contract;
- Prompt Engine/Audit linking path;
- rejection cases;
- R61 freshness identity fields/hash;
- stale behavior;
- focused/full test counts;
- native evidence/limitations;
- publication SHA/size;
- final milestone state.

## Completion boundary

Expected builder final state:

`M16A REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
