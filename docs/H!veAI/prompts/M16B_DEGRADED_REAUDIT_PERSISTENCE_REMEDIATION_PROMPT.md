# M16B Degraded Re-audit Persistence Remediation Prompt

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Scope: close M16-R62 only
Authority: authoritative remediation prompt

## 0. Starting state

Independent M16A strict re-audit verdict: **FAIL / CHANGES REQUIRED**.

Closed findings that must remain closed:

- M16-R59
- M16-R60
- M16-R61

Only open finding:

- **M16-R62 MAJOR** — fresh re-audits with `UNAVAILABLE` or `MALFORMED` model status fail instead of persisting truthful degraded audit history.

M15 remains PASS/CLOSED.
M16 remains OPEN.
Roadmap completed progress remains `16 / 20 = 80%`.
M17 MUST NOT activate.
M21 MUST NOT start.

Do not redesign M16.
Do not add a production GPT provider.
Do not alter R59-R61 contracts except where needed to preserve their behavior.

---

# 1. Reproduce R62 before changing code

Reproduce the production/domain path:

1. create/load a prior audit with at least one OPEN finding;
2. run a fresh repository re-audit using `UnavailableAuditModel`;
3. confirm current behavior returns `AUDIT_PRIOR_DISPOSITION_PROVIDER_NOT_ELIGIBLE` instead of persisting a re-audit;
4. reproduce the equivalent MALFORMED evaluation path.

Record exact before-fix evidence in the log.

---

# 2. Required state-machine rule

Disposition application must be gated by both audit state and model eligibility.

Required truth table:

| Audit state | Model status | Persist run? | Apply prior dispositions? |
| --- | --- | --- | --- |
| COMPLETED | AVAILABLE | YES | YES, explicit validated dispositions only |
| COMPLETED | UNAVAILABLE | YES | NO |
| COMPLETED | MALFORMED | YES | NO |
| STALE | any | YES | NO |
| FAILED/CANCELLED | as supported | truthful existing behavior | NO |

The implementation must not convert UNAVAILABLE/MALFORMED re-audit into an error merely because a prior audit exists.

---

# 3. Production change

Refactor `persist_run(...)` or its caller so:

- prior project ownership validation still occurs;
- explicit prior-finding dispositions are processed **only** when:
  - `state == COMPLETED`, and
  - `evaluation.model_status == "AVAILABLE"`;
- UNAVAILABLE/MALFORMED COMPLETED re-audits persist normally without calling `apply_prior_finding_dispositions`;
- prior OPEN findings remain open in historical prior audit evidence;
- no synthetic CLOSED/STILL_OPEN/SUPERSEDED row is required when the degraded model produced no eligible disposition;
- audit history still links `prior_audit_id`;
- current audit retains truthful:
  - CONDITIONAL verdict,
  - LOW confidence,
  - appropriate risk,
  - modelStatus UNAVAILABLE or MALFORMED,
  - bounded diagnostic,
  - COMPLETED state if repository remained fresh.

Do not mutate the prior audit row.

---

# 4. Preserve R59

AVAILABLE re-audits must still:

- require explicit priorFindingDispositions;
- reject duplicate/unknown prior keys;
- reject invalid evidence refs;
- require closure proof for CLOSED/SUPERSEDED;
- never close by omission.

Add a regression proving an AVAILABLE re-audit with omitted prior finding still persists without closing it.

---

# 5. Preserve R61

Freshness remains higher priority than model degraded state.

If repository changes after collection:

- state = STALE;
- no disposition application;
- stale diagnostic retained;
- do not persist normal COMPLETED degraded result.

Add a regression for UNAVAILABLE + repository change → STALE, not COMPLETED.

---

# 6. Audit Center behavior

No redesign required.

Verify the existing Re-audit button now works in the actual current production provider state:

- production provider is UNAVAILABLE;
- clicking Re-audit should create a new immutable audit row;
- UI should show CONDITIONAL / COMPLETED / UNAVAILABLE truthfully;
- prior audit remains in history;
- no prior finding is falsely closed.

If native CUA is unavailable, record limitation and cover the production command path directly in tests.

---

# 7. Required tests

Add direct Rust/domain tests for:

1. prior OPEN finding + UNAVAILABLE fresh re-audit persists;
2. returned audit has `state == COMPLETED`;
3. returned audit has `model_status == UNAVAILABLE`;
4. verdict remains CONDITIONAL;
5. prior audit remains queryable;
6. prior OPEN finding remains OPEN;
7. no closure disposition row is synthesized;
8. MALFORMED fresh re-audit persists;
9. MALFORMED re-audit does not close prior finding;
10. AVAILABLE explicit CLOSED disposition still works;
11. AVAILABLE omission still does not close;
12. UNAVAILABLE stale re-audit becomes STALE;
13. MALFORMED stale re-audit becomes STALE if applicable;
14. cross-project prior audit validation still rejects.

Add/update frontend test if needed to prove Re-audit degraded result is rendered rather than surfaced as an error.

---

# 8. Full regression

Run:

- focused M16B Rust tests;
- focused M16/M16A Rust tests;
- focused Audit Center frontend tests;
- migration tests;
- full serialized Rust regression;
- full frontend regression;
- TypeScript typecheck;
- frontend production build;
- `npm audit --audit-level=high`;
- Rust fmt;
- Rust all-targets;
- Rust pty-support;
- `git diff --check`;
- M14E regression;
- M15 Prompt Engine regressions;
- M15A duplicate/race/replay;
- M15B ACL/provider/task-picker;
- M15C/D handoff/result;
- M16 R59 explicit-disposition suite;
- M16 R60 exact remediation provenance suite;
- M16 R61 freshness suite;
- publisher rollback harness;
- governed stable Tauri publication;
- candidate/stable SHA equality;
- PE/shortcut/icon/startup/no-console checks.

---

# 9. Explicit execution gates

1. Fetch `origin/H!veAI`.
2. Fast-forward-only synchronize.
3. Confirm branch `H!veAI`.
4. Record starting HEAD/worktree.
5. Preserve unrelated files.
6. Read M16 prompt.
7. Read M16 independent audit.
8. Read M16A prompt.
9. Read M16A log.
10. Read M16A strict re-audit.
11. Confirm R59-R61 CLOSED.
12. Confirm only R62 open.
13. Reproduce UNAVAILABLE re-audit failure.
14. Reproduce MALFORMED re-audit failure.
15. Define eligibility truth table in code/comments/tests.
16. Keep prior-project validation.
17. Gate disposition processing on COMPLETED+AVAILABLE.
18. Persist COMPLETED+UNAVAILABLE re-audit.
19. Persist COMPLETED+MALFORMED re-audit.
20. Preserve priorAuditId linkage.
21. Preserve prior historical rows.
22. Preserve prior OPEN finding.
23. Prevent synthetic closure rows in degraded runs.
24. Preserve AVAILABLE explicit closure.
25. Preserve AVAILABLE omission behavior.
26. Preserve STALE precedence.
27. Add UNAVAILABLE persistence test.
28. Add UNAVAILABLE prior-open preservation test.
29. Add MALFORMED persistence test.
30. Add MALFORMED prior-open preservation test.
31. Add AVAILABLE explicit closure regression.
32. Add AVAILABLE omission regression.
33. Add UNAVAILABLE+STALE regression.
34. Add MALFORMED+STALE regression if supported.
35. Add cross-project prior audit regression.
36. Run focused M16B Rust tests.
37. Run focused M16/M16A Rust tests.
38. Run focused Audit Center frontend tests.
39. Run migration tests.
40. Run full Rust regression.
41. Run full frontend regression.
42. Run TypeScript typecheck.
43. Run frontend build.
44. Run npm high audit.
45. Run Rust fmt.
46. Run Rust all-targets.
47. Run Rust pty-support.
48. Run `git diff --check`.
49. Run M14E regressions.
50. Run M15 regressions.
51. Run M15A replay/race regressions.
52. Run M15B regressions.
53. Run M15C/D regressions.
54. Run M16 R59 suite.
55. Run M16 R60 suite.
56. Run M16 R61 suite.
57. Run publisher rollback harness.
58. Governed-publish stable EXE.
59. Verify candidate/stable SHA equality.
60. Verify PE/shortcut/icon/startup.
61. Verify no visible console popup.
62. Native-open Audit Center if feasible.
63. Verify production UNAVAILABLE Start audit.
64. Verify production UNAVAILABLE Re-audit creates a new history row if feasible.
65. Verify prior audit remains visible.
66. Verify no prior finding false closure.
67. Create immutable M16B log.
68. Record implementation commit.
69. Record test counts.
70. Record publication SHA.
71. Commit scoped files only.
72. Push normally, no force.
73. Verify local/origin equality.
74. Leave M16 OPEN pending independent strict re-audit + user native/visual acceptance.
75. Do not activate M17.
76. Do not start M21.

---

# 10. Required log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16B_DEGRADED_REAUDIT_PERSISTENCE_REMEDIATION_LOG.md`

Record:

- exact R62 reproduction before fix;
- exact eligibility truth table;
- changed production symbols;
- direct test names/counts;
- UNAVAILABLE behavior after fix;
- MALFORMED behavior after fix;
- prior finding preservation proof;
- R59-R61 regression proof;
- native evidence/limitations;
- publication SHA and size;
- actual implementation commit;
- final milestone state.

## Completion boundary

End with:

`M16B REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
