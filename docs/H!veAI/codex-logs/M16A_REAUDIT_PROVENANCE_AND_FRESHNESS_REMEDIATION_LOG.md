# M16A Re-audit Provenance and Freshness Remediation Log

Date: 2026-09-08
Branch: H!veAI
Starting HEAD: 0ba86e89f085644b2b04cabddc9441e47672f739
Implementation commit: 3d95dc12dce7f0de804e19654cee4f7fa97458b8

## Boundary

M16-R59, M16-R60, and M16-R61 are remediated only. M16 remains OPEN pending independent strict re-audit and user native/visual acceptance. M15 remains PASS/CLOSED. M17 was not activated. M21 was not started. Roadmap progress remains 16/20 = 80%.

## R59 before and after

Before: an AVAILABLE re-audit closed every OPEN prior finding omitted from the current model findings list. This was omission-based closure with no current evidence, disposition, or rationale.

After: the model contract has bounded typed `priorFindingDispositions` entries with:

- `priorFindingKey`
- `disposition`: `STILL_OPEN`, `CLOSED`, or `SUPERSEDED`
- `evidenceRefs`
- `rationale`
- `replacementFindingKey` for `SUPERSEDED`

Every disposition must resolve to one real OPEN prior finding. Duplicate and unknown keys are rejected. Every evidence reference must be an ID in the current bounded input. `CLOSED` and `SUPERSEDED` require current evidence and rationale. Omission leaves the prior finding unresolved. `STILL_OPEN` is persisted as such. Closure and supersession preserve the prior identity, current audit attribution, evidence, rationale, and replacement key. UNAVAILABLE, MALFORMED, STALE, and FAILED paths never close prior findings.

## R60 exact provenance contract

Audit Center carries bounded `projectId`, `promptId`, and `auditId` to Prompt Engine. Prompt Engine loads the audit through native `getAudit`, checks selected-project ownership, checks the audit remediation prompt ID, and loads the exact persisted remediation version ID. URL identity alone is not trusted and no dispatch is automatic.

After the one successful exact Prompt Engine dispatch, Prompt Engine calls native `linkRemediationSession(projectId, auditId, returned.session.id)` once. Native linking requires the audit association, project ownership, exact prompt and version IDs, exact version number, approved body hash, session prompt-body hash, DISPATCHED prompt state, dispatched session ID, and matching dispatch provenance JSON. Failed linking is surfaced as a bounded truthful pending-link error and does not create another provider session. Audit Center exposes bounded navigation to the linked session in Agents; navigation does not launch a provider.

## R61 freshness contract

The deterministic freshness token hashes branch, HEAD, staged paths, unstaged paths, untracked paths, conflicted paths, bounded working-tree diff hash, diff truncation state, and repository identity. The token is included in the bounded input manifest and persisted on the audit row.

Immediately after model evaluation and before persistence, native Git snapshot and bounded working-tree diff are re-read for the same registered ACTIVE project. A changed token produces `STALE`, a bounded diagnostic beginning `AUDIT_STALE_REPOSITORY_CHANGED:`, no normal completed result, no prior-finding disposition application, and truthful persisted/retrieved state. This applies regardless of provider availability or malformed output.

## Gates

| Gate | Result |
| ---: | --- |
| 1 | PASS - fetched origin/H!veAI |
| 2 | PASS - fast-forward-only synchronized |
| 3 | PASS - H!veAI branch confirmed |
| 4 | PASS - starting HEAD/worktree recorded |
| 5 | PASS - unrelated parent files preserved |
| 6 | PASS - authoritative M16A prompt read in full |
| 7 | PASS - M16 builder log read |
| 8 | PASS - M16 independent strict audit read |
| 9 | PASS - R59/R60/R61 only confirmed |
| 10 | PASS - M16 OPEN confirmed |
| 11 | PASS - M17 blocked confirmed |
| 12 | PASS - M21 not started confirmed |
| 13 | PASS - R59 omission behavior reproduced in source and replaced test |
| 14 | PASS - R60 missing exact link provenance reproduced |
| 15 | PASS - R61 missing persistence freshness check reproduced |
| 16 | PASS - disposition schema implemented |
| 17 | PASS - strict disposition parser implemented |
| 18 | PASS - prior keys validated |
| 19 | PASS - evidence IDs validated against current input |
| 20 | PASS - omission closure removed |
| 21 | PASS - unresolved omissions preserved |
| 22 | PASS - explicit CLOSED semantics implemented |
| 23 | PASS - STILL_OPEN semantics implemented |
| 24 | PASS - SUPERSEDED semantics implemented |
| 25 | PASS - R59 direct tests added |
| 26 | PASS - bounded auditId handoff retained |
| 27 | PASS - audit/project ownership validated |
| 28 | PASS - exact remediation prompt ID validated |
| 29 | PASS - exact remediation version ID validated |
| 30 | PASS - no-auto-dispatch preserved |
| 31 | PASS - one post-dispatch native link call implemented |
| 32 | PASS - native link provenance strengthened |
| 33 | PASS - unrelated same-project session rejected |
| 34 | PASS - wrong prompt rejected |
| 35 | PASS - wrong version rejected |
| 36 | PASS - wrong project rejected |
| 37 | PASS - audit without remediation association rejected |
| 38 | PASS - session ID persisted only after checks |
| 39 | PASS - Audit Center linked-session navigation added |
| 40 | PASS - navigation has no provider-launch path |
| 41 | PASS - R60 focused Rust coverage added |
| 42 | PASS - R60 focused frontend coverage added |
| 43 | PASS - deterministic repository identity defined |
| 44 | PASS - branch included |
| 45 | PASS - HEAD included |
| 46 | PASS - staged paths included |
| 47 | PASS - unstaged paths included |
| 48 | PASS - untracked paths included |
| 49 | PASS - conflicted paths included |
| 50 | PASS - bounded diff hash included |
| 51 | PASS - token collected with input |
| 52 | PASS - token recomputed after evaluation |
| 53 | PASS - comparison occurs immediately before persistence |
| 54 | PASS - changed evidence becomes STALE |
| 55 | PASS - stale result cannot be normal COMPLETED |
| 56 | PASS - stale path cannot close prior findings |
| 57 | PASS - production-path same-HEAD working-tree test added |
| 58 | PASS - branch identity mutation covered by token test |
| 59 | PASS - same-HEAD diff mutation covered by token and production test |
| 60 | PASS - staged identity covered by token test |
| 61 | PASS - untracked identity covered by token test |
| 62 | PASS - unchanged identity determinism covered |
| 63 | PASS - stale state persistence/retrieval covered |
| 64 | PASS - HEAD-only helper replaced with full repository check |
| 65 | PASS - focused M16A Rust tests: 11 passed |
| 66 | PASS - focused M16 frontend tests: 3 passed |
| 67 | PASS - migration tests passed; schema version 14 |
| 68 | PASS - full serialized Rust library regression: 354 passed |
| 69 | PASS - full frontend regression: 15 files, 125 passed |
| 70 | PASS - TypeScript typecheck |
| 71 | PASS - frontend production build |
| 72 | PASS - npm audit high: 0 vulnerabilities |
| 73 | PASS - Rust format check after formatting |
| 74 | PASS - Rust all-targets check |
| 75 | PASS - Rust pty-support check |
| 76 | PASS - git diff check |
| 77 | PASS - M14E final-response regression included in full Rust/frontend suites |
| 78 | PASS - M15 Prompt Engine regressions included |
| 79 | PASS - M15A duplicate/race/replay regressions included |
| 80 | PASS - M15B ACL/provider/task-picker regressions included |
| 81 | PASS - M15C/D handoff/result regressions included |
| 82 | PASS - M16 known-good/known-bad audit tests included |
| 83 | PASS - M16 misleading-test coverage included |
| 84 | PASS - M16 security/containment coverage included |
| 85 | PASS - M16 re-audit history coverage included |
| 86 | PASS - M16A provenance tests passed |
| 87 | PASS - M16A freshness tests passed |
| 88 | PASS - publisher rollback harness: 9/9 passed |
| 89 | PASS - governed stable Tauri EXE publication |
| 90 | PASS - candidate/stable SHA equality |
| 91 | PASS - PE, shortcut, icon, and startup checks passed by publisher |
| 92 | PASS - publisher smoke reported no visible console popup |
| 93 | PENDING - native Audit Center visual acceptance remains for user |
| 94 | PASS - UNAVAILABLE audit state remains truthful in tests/UI |
| 95 | PASS - remediation draft handoff validated in frontend tests |
| 96 | PENDING - controlled real provider dispatch requires user/native provider acceptance |
| 97 | PASS - linked-session route targeting covered |
| 98 | PASS - navigation/linking does not redispatch |
| 99 | PASS - this immutable M16A log created |
| 100 | PASS - implementation commit recorded |
| 101 | PASS - test counts recorded |
| 102 | PASS - publication SHA recorded |
| 103 | PASS - scoped files only committed; parent untracked files excluded |
| 104 | PASS - normal push used |
| 105 | PASS - local/origin equality proved after push |
| 106 | PASS - M16 left OPEN pending independent strict re-audit and user native acceptance |
| 107 | PASS - M17 not activated |
| 108 | PASS - M21 not started |

## Publication

Candidate: `src-tauri/target/release/hiveai-desktop.exe`

Stable: `dev-bin/H!veAI.exe`

SHA256: `4305AC97557D02DDF7D5390270C6DB084E29F5299E6CDD6A100415E054F1CD78`

Size: 22,230,528 bytes for both candidate and stable. Stable PE prefix: `MZ`.

## Final state

M16A REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

M16 remains OPEN. M17 NOT ACTIVATED. M21 NOT STARTED.
