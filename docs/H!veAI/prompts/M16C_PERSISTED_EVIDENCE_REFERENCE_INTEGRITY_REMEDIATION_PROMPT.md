# M16C Persisted Evidence Reference Integrity Remediation Prompt

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Scope: close M16-R63 only
Authority: authoritative remediation prompt

## 0. Starting state

Independent M16B strict re-audit verdict: **FAIL / CHANGES REQUIRED**.

Closed findings that must remain closed:

- M16-R59
- M16-R60
- M16-R61
- M16-R62

Only open finding:

- **M16-R63 MAJOR** — persisted evidence row IDs are audit-prefixed to avoid PK collisions, but persisted finding/coverage/disposition evidence refs still use unprefixed logical IDs, so historical references no longer resolve against returned persisted evidence rows.

M15 remains PASS/CLOSED.
M16 remains OPEN.
Roadmap completed progress remains `16 / 20 = 80%`.
M17 MUST NOT activate.
M21 MUST NOT start.

Do not redesign the Audit Engine.
Do not add a production GPT provider.
Do not change unrelated M16 behavior.

---

# 1. Reproduce R63 before modifying code

Create a direct persistence fixture:

1. produce an audit input containing logical evidence ID `TEST_RUN:proof`;
2. persist a finding/coverage row that references `TEST_RUN:proof`;
3. persist the audit;
4. read the audit back through production `get(...)`;
5. prove:
   - persisted evidence row ID is currently `<audit_id>:TEST_RUN:proof`;
   - finding/coverage reference remains `TEST_RUN:proof`;
   - direct ID resolution therefore fails.

Repeat with a second immutable audit using the same logical evidence ID to prove why globally unique DB row IDs are needed.

Record exact before-fix behavior in the log.

---

# 2. Required durable evidence identity contract

Implement an explicit two-level identity model.

## 2.1 Global persisted row ID

Keep a globally unique primary key for `audit_evidence.id`.

Recommended:

`<audit_id>:<logical_evidence_id>`

Equivalent deterministic globally unique audit-scoped form is acceptable.

## 2.2 Logical evidence ID

Add an additive migration with:

`logical_evidence_id TEXT`

Requirements:

- non-null for all new M16C evidence rows;
- original M16 logical evidence identity is stored unchanged;
- backfill existing audit_evidence rows deterministically where possible:
  - for M16B-prefixed rows, strip the exact owning `audit_id + ":"` prefix;
  - for pre-M16B historical rows, logical ID may equal existing `id`;
- add an index/uniqueness rule sufficient for efficient same-audit resolution.

Preferred invariant:

`UNIQUE(audit_id, logical_evidence_id)`

Do not require logical IDs to be globally unique across audits.

---

# 3. External evidence-reference contract

Choose **logical IDs scoped to one audit** as the external/reference contract unless actual repository constraints require otherwise.

That means:

- `AuditEvidence` returned to frontend exposes:
  - persisted row ID;
  - `logicalEvidenceId`;
- finding `evidenceRefs`;
- coverage `evidenceRefs`;
- prior-finding disposition `dispositionEvidenceRefs`;

continue to use logical evidence IDs.

Resolution rule:

`(audit_id, logical_evidence_id) -> exactly one audit_evidence row`

No cross-audit lookup.

Document this contract in code and tests.

---

# 4. Persistence invariant enforcement

Before final transaction commit, validate every evidence reference that should resolve.

For the current audit:

- every finding `evidenceRefs`;
- every requirement coverage `evidenceRefs`;
- every disposition `dispositionEvidenceRefs`;

must resolve to a current audit logical evidence ID, unless the contract explicitly permits a typed non-evidence reference class. If some refs intentionally refer to non-`AuditEvidence` authority tokens, split that contract explicitly rather than silently mixing namespaces.

Do not accept dangling refs.

Use deterministic errors such as:

- `AUDIT_EVIDENCE_REFERENCE_NOT_FOUND`
- `AUDIT_EVIDENCE_REFERENCE_DUPLICATE`

Exact names may differ if consistent.

---

# 5. Historical compatibility

Migration must preserve existing M16/M16A/M16B audit history.

Required behavior:

- old evidence rows remain readable;
- old audits remain immutable;
- no historical finding/verdict is rewritten;
- returned old AuditRun entries expose resolvable logical evidence identity where data permits;
- no cross-audit collision occurs when multiple audits share the same logical evidence ID.

Add migration tests from schema v13 and v14 to the new schema.

---

# 6. Frontend contract

Update `src/auditEngine.ts` types so `AuditEvidence` exposes `logicalEvidenceId`.

Audit Center visual redesign is not required.

If useful, technical evidence details may display logical ID while retaining the persisted row ID as secondary provenance.

Do not change primary Audit Center layout.

---

# 7. Preserve R59-R62

Do not regress:

## R59
- no closure by omission;
- explicit AVAILABLE dispositions only.

## R60
- exact Audit → Prompt Engine → Agent session provenance.

## R61
- branch/HEAD/working-tree freshness enforcement.

## R62
- fresh UNAVAILABLE/MALFORMED re-audits persist truthfully;
- degraded runs apply zero prior dispositions;
- STALE remains higher priority.

---

# 8. Required direct tests

Add production/domain tests for:

1. first audit persists logical evidence ID and unique row ID;
2. second audit with same logical evidence ID also persists successfully;
3. row IDs differ across audits;
4. logical IDs remain equal across audits;
5. finding evidence ref resolves within audit 1;
6. same ref resolves within audit 2;
7. audit 1 ref cannot resolve to audit 2 evidence by scoped resolver;
8. coverage evidence refs resolve;
9. disposition evidence refs resolve;
10. dangling finding evidence ref rejected before commit;
11. dangling coverage evidence ref rejected;
12. dangling disposition evidence ref rejected;
13. duplicate logical evidence ID in one audit rejected or deterministically deduplicated according to contract;
14. migration backfills M16B-prefixed rows correctly;
15. migration handles pre-M16B rows;
16. historical `get(...)` returns logicalEvidenceId;
17. UNAVAILABLE re-audit still persists;
18. MALFORMED re-audit still persists;
19. STALE path remains intact;
20. exact remediation provenance suite remains green.

---

# 9. Full regression

Run:

- focused M16C Rust tests;
- M16/M16A/M16B audit-engine tests;
- Audit Center frontend tests;
- migration tests;
- full serialized Rust regression;
- full frontend regression;
- TypeScript typecheck;
- production frontend build;
- `npm audit --audit-level=high`;
- Rust fmt;
- Rust all-targets;
- Rust pty-support;
- `git diff --check`;
- M14E regressions;
- M15 Prompt Engine regressions;
- M15A replay/race;
- M15B ACL/provider/task picker;
- M15C/D handoff/result;
- M16 R59-R62 regressions;
- publisher rollback harness;
- governed stable publication;
- candidate/stable SHA equality;
- PE/shortcut/icon/startup/no-console checks.

---

# 10. Explicit execution gates

1. Fetch `origin/H!veAI`.
2. Fast-forward-only synchronize.
3. Confirm branch `H!veAI`.
4. Record starting HEAD/worktree.
5. Preserve unrelated files.
6. Read M16 prompt.
7. Read M16 independent audit.
8. Read M16A prompt/log/re-audit.
9. Read M16B prompt/log/re-audit.
10. Confirm R59-R62 closed.
11. Confirm only R63 open.
12. Reproduce broken evidence-reference resolution.
13. Reproduce same-logical-ID cross-audit collision motivation.
14. Design two-level identity contract.
15. Add migration version.
16. Add `logical_evidence_id` column.
17. Backfill M16B-prefixed rows.
18. Backfill pre-M16B rows.
19. Add same-audit logical-ID uniqueness/index.
20. Update Rust AuditEvidence type.
21. Update TS AuditEvidence type.
22. Persist global row ID.
23. Persist logical evidence ID.
24. Read both IDs in `get(...)`.
25. Implement same-audit logical resolver.
26. Validate finding refs.
27. Validate coverage refs.
28. Validate disposition refs.
29. Reject dangling refs.
30. Prevent cross-audit resolution.
31. Add two-audit same-logical-ID test.
32. Add finding resolution test.
33. Add coverage resolution test.
34. Add disposition resolution test.
35. Add dangling finding ref test.
36. Add dangling coverage ref test.
37. Add dangling disposition ref test.
38. Add duplicate logical ID test.
39. Add migration v14→new test.
40. Add migration v13→new test.
41. Add historical get compatibility test.
42. Preserve R59 suite.
43. Preserve R60 suite.
44. Preserve R61 suite.
45. Preserve R62 suite.
46. Run focused M16C Rust tests.
47. Run focused Audit Center frontend tests.
48. Run migration tests.
49. Run full Rust regression.
50. Run full frontend regression.
51. Run TypeScript typecheck.
52. Run frontend build.
53. Run npm high audit.
54. Run Rust fmt.
55. Run Rust all-targets.
56. Run Rust pty-support.
57. Run `git diff --check`.
58. Run M14E regression.
59. Run M15 regressions.
60. Run M15A replay/race regression.
61. Run M15B regression.
62. Run M15C/D regression.
63. Run M16 R59-R62 regression.
64. Run publisher rollback harness.
65. Governed-publish stable EXE.
66. Verify candidate/stable SHA equality.
67. Verify PE/shortcut/icon/startup.
68. Verify no visible console popup.
69. Native-open Audit Center if feasible.
70. Verify historical audit technical evidence remains readable.
71. Verify no broken evidence IDs appear in UI if visually inspectable.
72. Create immutable M16C log.
73. Record implementation commit.
74. Record migration version.
75. Record test counts.
76. Record publication SHA.
77. Commit scoped files only.
78. Push normally, no force.
79. Verify local/origin equality.
80. Leave M16 OPEN pending independent strict re-audit + user native/visual acceptance.
81. Do not activate M17.
82. Do not start M21.

---

# 11. Required log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16C_PERSISTED_EVIDENCE_REFERENCE_INTEGRITY_REMEDIATION_LOG.md`

Record:

- exact R63 reproduction;
- final durable evidence identity contract;
- migration version and backfill behavior;
- resolver semantics;
- dangling-reference validation;
- cross-audit isolation proof;
- direct test names/counts;
- R59-R62 regression proof;
- native evidence/limitations;
- publication SHA and size;
- implementation commit;
- final milestone state.

## Completion boundary

End with:

`M16C REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
