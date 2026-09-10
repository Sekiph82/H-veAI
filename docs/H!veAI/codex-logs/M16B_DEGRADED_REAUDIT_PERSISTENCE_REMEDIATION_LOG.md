# M16B Degraded Re-audit Persistence Remediation Log

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Starting HEAD: `13d780564c2ddafb76f5baa459b35ba8fc217501`
Implementation commit: `404dca9`

## Required outcome

M16-R62 only is remediated. M16 remains OPEN pending independent strict re-audit and user native/visual acceptance. M15 remains PASS/CLOSED. M17 was not activated. M21 was not started. Roadmap progress remains `16 / 20 = 80%`.

## Before-fix reproduction

The pre-fix `persist_run(...)` path validated the prior audit project, then entered `apply_prior_finding_dispositions(...)` for every `COMPLETED` run. With a prior audit containing an OPEN finding, a fresh `UnavailableAuditModel` evaluation therefore reached the helper's provider guard and returned the exact error `AUDIT_PRIOR_DISPOSITION_PROVIDER_NOT_ELIGIBLE` instead of persisting. The equivalent malformed path produced `model_status = MALFORMED` and reached the same guard. This was reproduced from the production/domain path before changing the gate; the exact source was the unconditional `state == AuditState::Completed` condition in `persist_run` and the provider guard in `apply_prior_finding_dispositions`.

## Remediation

`persist_run(...)` now applies prior-finding dispositions only when both conditions hold:

`state == AuditState::Completed && evaluation.model_status == "AVAILABLE"`

Prior project ownership validation remains unconditional. Fresh `UNAVAILABLE` and `MALFORMED` evaluations now persist as truthful `COMPLETED` / `CONDITIONAL` history with low confidence, high risk, bounded diagnostics, the original `priorAuditId`, no disposition rows, and untouched prior findings. `STALE` continues to take precedence and never applies dispositions. Persisted evidence primary keys are scoped to their immutable audit row because logical input evidence IDs are stable across runs.

| Audit state | Model status | Persist | Prior dispositions |
| --- | --- | --- | --- |
| COMPLETED | AVAILABLE | YES | YES, explicit validated entries only |
| COMPLETED | UNAVAILABLE | YES | NO |
| COMPLETED | MALFORMED | YES | NO |
| STALE | any | YES | NO |
| FAILED/CANCELLED | supported existing behavior | truthful existing behavior | NO |

## Direct evidence

- `unavailable_fresh_reaudit_persists_and_preserves_prior_open_finding`: fresh degraded persistence, COMPLETED state, UNAVAILABLE status, CONDITIONAL verdict, prior linkage, queryable prior row, OPEN prior finding, and no synthetic current finding.
- `malformed_fresh_reaudit_persists_and_does_not_close_prior_finding`: fresh malformed persistence and prior OPEN preservation.
- `degraded_reaudits_keep_stale_precedence_after_repository_change`: UNAVAILABLE and MALFORMED repository changes persist as STALE with the stale diagnostic.
- `degraded_reaudit_keeps_cross_project_prior_validation`: cross-project prior audit remains rejected with `AUDIT_PRIOR_PROJECT_MISMATCH_OR_NOT_FOUND`.
- `explicit_reaudit_dispositions_are_required_and_validated`: AVAILABLE omission remains non-closing and explicit closure remains validated.
- Full Rust regression: `358 passed; 0 failed`.
- Full frontend regression: `15 files, 125 passed`.
- Focused audit engine suite: `15 passed; 0 failed`.
- Migration suite: `13 passed; 0 failed`.
- Focused Agent Session Center suite: `9 passed; 0 failed`.
- Focused Prompt Engine suite: `10 passed; 0 failed`.
- Focused Codex/process suite: `25 passed; 0 failed`.
- Focused Audit Center frontend suite: `3 passed; 0 failed`.
- Publisher rollback harness: `9/9 PASS`.
- TypeScript typecheck, production build, npm high audit, Rust fmt, all-targets, pty-support, and `git diff --check`: PASS.

## Governed publication

- Stable executable: `dev-bin/H!veAI.exe`
- Candidate SHA-256: `E608D5DE9B2976835AA08231E500DA8101ABA46AE8F6C8A6610920C8F06E6372`
- Stable SHA-256: `E608D5DE9B2976835AA08231E500DA8101ABA46AE8F6C8A6610920C8F06E6372`
- Candidate/stable size: `22,231,552` bytes each.
- PE prefix: `MZ`.
- PE subsystem: `2` / Windows GUI; publisher smoke found no visible console host.
- Desktop shortcut target and icon: PASS; target is the stable executable and icon is `dev-bin/H!veAI.ico,0`.
- Startup audio/X01/X02 behavior remained untouched and existing regression/publication evidence remained green.
- Native Audit Center open and native click/re-audit acceptance: PENDING because native CUA/user acceptance was unavailable in this run. Direct production/domain command coverage was executed instead; no real external provider operation was performed.

## Explicit gate record

1. PASS — fetched `origin/H!veAI`.
2. PASS — fast-forward-only synchronized to `13d7805`.
3. PASS — branch confirmed `H!veAI`.
4. PASS — recorded starting HEAD and worktree; only parent `start-demo.bat` and `task.md` were unrelated untracked files.
5. PASS — unrelated parent files preserved and unstaged.
6. PASS — read authoritative M16 implementation prompt.
7. PASS — read M16 independent strict audit.
8. PASS — read authoritative M16A remediation prompt.
9. PASS — read M16A remediation log.
10. PASS — read M16A strict re-audit; verdict FAIL / CHANGES REQUIRED.
11. PASS — M16-R59, M16-R60, and M16-R61 confirmed closed.
12. PASS — only M16-R62 treated as open.
13. PASS — reproduced fresh UNAVAILABLE prior-disposition failure and exact error.
14. PASS — reproduced equivalent MALFORMED provider-eligibility failure path.
15. PASS — encoded the eligibility truth table in production gate, comments, and tests.
16. PASS — prior project ownership validation retained.
17. PASS — disposition processing gated on COMPLETED plus AVAILABLE.
18. PASS — fresh COMPLETED plus UNAVAILABLE persists.
19. PASS — fresh COMPLETED plus MALFORMED persists.
20. PASS — `priorAuditId` linkage retained.
21. PASS — prior historical rows remain queryable and untouched.
22. PASS — prior OPEN finding remains OPEN.
23. PASS — no degraded synthetic closure rows are created.
24. PASS — AVAILABLE explicit closure behavior retained.
25. PASS — AVAILABLE omission behavior retained.
26. PASS — STALE precedence retained.
27. PASS — UNAVAILABLE persistence test added and green.
28. PASS — UNAVAILABLE prior-open preservation test added and green.
29. PASS — MALFORMED persistence test added and green.
30. PASS — MALFORMED prior-open preservation test added and green.
31. PASS — AVAILABLE explicit closure regression green.
32. PASS — AVAILABLE omission regression green.
33. PASS — UNAVAILABLE plus repository change becomes STALE.
34. PASS — MALFORMED plus repository change becomes STALE.
35. PASS — cross-project prior validation regression green.
36. PASS — focused M16B Rust/domain tests green.
37. PASS — focused M16/M16A Rust tests green.
38. PASS — focused Audit Center frontend tests green.
39. PASS — migration tests green.
40. PASS — full serialized Rust regression: 358 passed.
41. PASS — full frontend regression: 125 passed.
42. PASS — TypeScript typecheck.
43. PASS — frontend production build.
44. PASS — `npm audit --audit-level=high`, 0 vulnerabilities.
45. PASS — Rust format check.
46. PASS — Rust all-targets check.
47. PASS — Rust pty-support check.
48. PASS — `git diff --check`.
49. PASS — M14E regressions included in full regression and focused Agent Session Center suite.
50. PASS — M15 Prompt Engine regressions included in full regression and focused Prompt Engine suite.
51. PASS — M15A duplicate/race/replay regressions green.
52. PASS — M15B ACL/provider/task-picker regressions green.
53. PASS — M15C/D handoff/result regressions green in frontend suite.
54. PASS — M16 R59 explicit-disposition suite green.
55. PASS — M16 R60 exact remediation provenance suite green.
56. PASS — M16 R61 freshness suite green.
57. PASS — publisher rollback harness 9/9.
58. PASS — governed stable executable publication.
59. PASS — candidate/stable SHA equality.
60. PASS — PE, shortcut, icon, startup, and publication smoke checks.
61. PASS — no visible console popup in publisher smoke; PE subsystem is Windows GUI.
62. PENDING — native Audit Center open/user native acceptance unavailable in this run.
63. PASS — production UNAVAILABLE Start path covered by `UnavailableAuditModel` production/domain tests.
64. PASS — production UNAVAILABLE Re-audit persistence path covered directly; native click unavailable.
65. PASS — prior audit remains visible/queryable in direct persisted read model tests.
66. PASS — no prior finding false closure.
67. PASS — this immutable M16B remediation log created.
68. PASS — implementation commit recorded as `404dca9`.
69. PASS — test counts recorded above.
70. PASS — publication SHA recorded above.
71. PASS — scoped implementation commit contains only M16B source/tests/tracker files.
72. PASS — changes pushed normally to `origin/H!veAI`.
73. PASS — final local/origin SHA equality proved after push.
74. PASS — M16 remains OPEN pending independent strict re-audit and user native/visual acceptance.
75. PASS — M17 NOT ACTIVATED.
76. PASS — M21 NOT STARTED.

M16B REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.
M16 REMAINS OPEN.
M17 NOT ACTIVATED.
M21 NOT STARTED.
