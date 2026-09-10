# M16C Persisted Evidence Reference Integrity Remediation Log

Date: 2026-09-08  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Starting HEAD: `8b2d0c56f1f29ac2706aa1268760a4e58086d627`  
Implementation commit: `283ce2cc349ade2cb645f313872b3475f71d2ca0`

## Required outcome

M16-R63 only is remediated. M16 remains OPEN pending independent strict re-audit and user native/visual acceptance. M15 remains PASS/CLOSED. M17 was not activated. M21 was not started. Roadmap progress remains `16 / 20 = 80%`.

## Before-fix reproduction

The pre-fix M16B persistence path stored each evidence row with an audit-scoped primary key of the form `<audit_id>:<logical_id>`, while findings, coverage, and prior-finding disposition records retained the logical `<kind>:<locator>` reference. The persisted read model therefore returned IDs that did not equal the references it exposed, and there was no `(audit_id, logical_id)` resolver. Repeating the same immutable audit input also attempted to reuse the logical evidence identity as a database identity, which made the collision-avoidance encoding part of the public evidence ID rather than a separate persistence identity. This was reproduced by tracing the pre-fix insert/read/reference path at starting HEAD `8b2d0c5`.

## Remediation

M16C uses two durable identity levels:

- `audit_evidence.id` is the globally unique immutable persisted row ID, generated as `<audit_id>:<logical_id>` for new rows.
- `audit_evidence.logical_evidence_id` is the stable audit-scoped logical reference (`<kind>:<locator>`), preserved in findings, coverage, and disposition evidence references.

Migration v15 (`audit_evidence_logical_identity`) adds the logical column, backfills M16B-prefixed IDs by removing the `<audit_id>:` prefix, backfills pre-M16B rows with their existing logical row ID, and creates the unique `(audit_id, logical_evidence_id)` index. New rows are validated as non-empty and unique before persistence. The resolver requires both `audit_id` and `logical_evidence_id`; it never resolves an evidence reference across audits. Finding, coverage, and disposition references are validated against the current audit input before the transaction can commit. Historical reads fall back to the existing row ID when a legacy logical column is null.

## Direct evidence

- `persisted_evidence_has_global_and_audit_scoped_logical_identity`: two immutable audits persist the same logical evidence reference; global row IDs differ, logical IDs remain equal, finding/coverage/disposition references remain logical, and the scoped resolver returns only the matching audit row.
- `persisted_evidence_rejects_dangling_and_duplicate_logical_references`: dangling finding, coverage, and disposition references plus duplicate logical IDs are rejected with bounded errors and leave no audit row committed.
- Migration v14 -> v15 and v13 -> v15 fixtures prove M16B-prefixed and pre-M16B historical backfill without rewriting persisted row IDs.
- Focused audit engine suite: `17 passed; 0 failed`.
- Migration suite: `15 passed; 0 failed`.
- Full Rust regression: `362 passed; 0 failed`.
- Full frontend regression: `15 files, 125 passed`.
- Focused Agent Session Center suite: `9 passed; 0 failed`.
- Focused Prompt Engine suite: `10 passed; 0 failed`.
- Focused capability suite: `2 passed; 0 failed`.
- Publisher rollback harness: `9/9 PASS`.
- TypeScript typecheck, production build, npm high audit, Rust fmt, all-targets, pty-support, and `git diff --check`: PASS.

## Governed publication

- Stable executable: `dev-bin/H!veAI.exe`
- Candidate SHA-256: `1A26AD95597B6D3C9B348AF8C255E858A9A57AF8C5E21AF1A40FD7E4CA6F41DC`
- Stable SHA-256: `1A26AD95597B6D3C9B348AF8C255E858A9A57AF8C5E21AF1A40FD7E4CA6F41DC`
- Candidate/stable size: `22,293,504` bytes each.
- PE prefix: `MZ`.
- PE subsystem: `2` / Windows GUI; publisher smoke found no visible console host.
- Desktop shortcut target and icon: PASS; target is the stable executable and icon is `dev-bin/H!veAI.ico,0`.
- M09 and M16 R59-R62 behavior remained covered by regression; visible UI was not changed.
- Native Audit Center open and native visual acceptance: PENDING for independent/user evidence. Direct production/domain and frontend test evidence passed; no external provider operation was required for M16C.

## Explicit gate record

1. PASS - fetched `origin/H!veAI`.
2. PASS - fast-forward-only synchronized to `8b2d0c5`.
3. PASS - branch confirmed `H!veAI`.
4. PASS - recorded starting HEAD and worktree; only parent `start-demo.bat` and `task.md` were unrelated untracked files.
5. PASS - unrelated parent files preserved and unstaged.
6. PASS - read the authoritative M16C remediation prompt in full.
7. PASS - read the M16 independent strict audit.
8. PASS - read the M16A remediation prompt, log, and strict re-audit.
9. PASS - read the M16B remediation prompt, log, and strict re-audit.
10. PASS - M16-R59, M16-R60, M16-R61, and M16-R62 confirmed closed.
11. PASS - only M16-R63 treated as open.
12. PASS - reproduced the pre-fix unresolved persisted evidence-reference identity path.
13. PASS - reproduced the pre-fix same-logical-ID collision motivation.
14. PASS - defined the two-level global-row/audit-logical identity contract.
15. PASS - added schema migration v15.
16. PASS - added `logical_evidence_id` to persisted evidence.
17. PASS - backfilled M16B-prefixed evidence IDs.
18. PASS - backfilled pre-M16B historical evidence IDs.
19. PASS - added same-audit uniqueness and index enforcement.
20. PASS - updated Rust `AuditEvidence` contract.
21. PASS - updated frontend `AuditEvidence` contract.
22. PASS - persisted globally unique evidence row IDs.
23. PASS - persisted stable audit-scoped logical evidence IDs.
24. PASS - returned both identities from persisted `get` reads.
25. PASS - added `(audit_id, logical_evidence_id)` resolver.
26. PASS - validated finding evidence references before commit.
27. PASS - validated coverage evidence references before commit.
28. PASS - validated prior disposition evidence references before commit.
29. PASS - rejected dangling evidence references.
30. PASS - prevented cross-audit evidence resolution.
31. PASS - two immutable audits with the same logical ID persist independently.
32. PASS - finding references resolve within their audit.
33. PASS - coverage references resolve within their audit.
34. PASS - disposition references resolve within their audit.
35. PASS - dangling finding reference test green.
36. PASS - dangling coverage reference test green.
37. PASS - dangling disposition reference test green.
38. PASS - duplicate logical evidence reference test green.
39. PASS - v14 -> v15 migration/backfill test green.
40. PASS - v13 -> v15 migration/backfill test green.
41. PASS - historical read compatibility preserved.
42. PASS - M16-R59 explicit disposition regression green.
43. PASS - M16-R60 exact provenance regression green.
44. PASS - M16-R61 freshness regression green.
45. PASS - M16-R62 degraded persistence regression green.
46. PASS - focused M16C Rust audit suite green.
47. PASS - Audit Center frontend coverage included in the frontend suite.
48. PASS - focused migration suite green.
49. PASS - full serialized Rust regression: `362 passed`.
50. PASS - full frontend regression: `15 files, 125 passed`.
51. PASS - TypeScript typecheck.
52. PASS - frontend production build.
53. PASS - `npm audit --audit-level=high`, 0 vulnerabilities.
54. PASS - Rust format check.
55. PASS - Rust all-targets check.
56. PASS - Rust pty-support check.
57. PASS - `git diff --check`.
58. PASS - M14E regressions included in full regression and focused Agent Session Center suite.
59. PASS - M15 Prompt Engine regressions included in full regression and focused Prompt Engine suite.
60. PASS - M15A duplicate/race/replay regressions green.
61. PASS - M15B ACL/provider/task-picker regressions green.
62. PASS - M15C/D handoff/result regressions green in frontend suite.
63. PASS - M16 R59-R62 regressions green in the focused audit suite.
64. PASS - publisher rollback harness `9/9`.
65. PASS - governed stable executable publication.
66. PASS - candidate/stable SHA equality.
67. PASS - PE, shortcut, icon, and startup publication checks.
68. PASS - no console popup in publisher smoke; PE subsystem is Windows GUI.
69. PENDING - native Audit Center open/user visual acceptance requires independent/user native evidence.
70. PASS - historical evidence identity/readability verified by direct persisted read and resolver tests.
71. PENDING - final no-broken-evidence-ID UI inspection requires independent/user native visual evidence.
72. PASS - immutable M16C remediation log created.
73. PASS - implementation commit recorded as `283ce2cc349ade2cb645f313872b3475f71d2ca0`.
74. PASS - migration version and backfill behavior recorded.
75. PASS - direct, focused, full, frontend, and publication test counts recorded.
76. PASS - governed publication SHA and size recorded.
77. PASS - scoped implementation commit contains only M16C source, migration, type, and tracker files.
78. PASS - immutable evidence log is the only subsequent scoped documentation commit.
79. PASS - normal push to `origin/H!veAI` completed.
80. PASS - final local/origin SHA equality proved after push.
81. PASS - M16 remains OPEN pending independent strict re-audit and user native/visual acceptance.
82. PASS - M17 NOT ACTIVATED and M21 NOT STARTED.

M16C REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.
M16 REMAINS OPEN.
M17 NOT ACTIVATED.
M21 NOT STARTED.
