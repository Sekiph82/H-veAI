# M16G Unified Project Control Plane Final Whole-System Closure Remediation Prompt

Date: 2026-09-08
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Rules authority: `H!veAI/GPT.md`

## 0. Execution mode

This is one continuous final whole-system remediation run for the Unified Project Control Plane.

Read and close every finding in:

`H!veAI/docs/H!veAI/audits/M16F_REV3_UNIFIED_PROJECT_CONTROL_PLANE_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`

Close all together:

- UCP-R13 BLOCKER
- UCP-R14 BLOCKER
- UCP-R15 BLOCKER
- UCP-R16 MAJOR
- UCP-R17 MAJOR
- UCP-R18 MINOR

Do not stop after any individual finding.

Preserve the already-closed M16 audit-engine findings R82-R85.

After all named fixes, perform a fresh whole-M16 + whole-control-plane adversarial sweep. If any additional BLOCKER or MAJOR is discovered, fix it in the same run and add a direct regression.

Do not activate M17.
Do not start M21.
M16 remains OPEN until independent strict re-audit + owner native/visual acceptance.

---

# 1. Required reading before edits

Read completely:

1. `H!veAI/GPT.md`
2. all M16A-F prompts/logs/audits
3. `M16F_REV3_UNIFIED_PROJECT_CONTROL_PLANE_INDEPENDENT_WHOLE_SYSTEM_STRICT_REAUDIT.md`
4. current `control_plane.rs`
5. current `watcher.rs`
6. current `project_dashboard.rs`
7. current Project Registry / Task Sources / Task Intelligence / Workflow / Git Engine
8. current Command Center / Project Cockpit source
9. current direct tests
10. current target-branch control-plane files from all eight GitHub repositories

Builder logs are claims only.

---

# 2. UCP-R13 — split canonical events path from additional event sources

The production ProjectDocument currently aliases:

- `events`
- `eventSource`
- `eventSources`

into one `Vec<String>`.

That is wrong.

Implement distinct fields:

- `events: String`
- `eventSources: Vec<String>`

Backward compatibility:

- legacy `eventSource: String` migrates to `events`
- legacy `eventSources: []` migrates to `eventSources`
- current `events: String` remains valid
- reject wrong scalar/array type combinations truthfully

Do not alias a scalar path into a Vec.

Required exact fixtures:

1. current AI-Commerce-HQ PROJECT.json
2. current fmcg PROJECT.json
3. old `eventSource` scalar
4. explicit `eventSources` array
5. one document containing both canonical events path and extra watcher sources

Required invariant:

> the exact upgrader output must immediately deserialize through the normal production ProjectDocument path.

---

# 3. UCP-R14 — actually migrate all eight target GitHub repositories

The previous run migrated only:

- AI-Commerce-HQ
- fmcg-erp-system

The following target branches still carry `hiveai-project/v1`:

- Bulk-Edit / main
- FormuLab / feature/laboratory-stability
- PackLab / main
- PackLab-3D / main
- ScrubBots / main
- ScrubBots-Level-Factory / main

Migrate all remaining repositories to the final accepted schema.

Do not use local dirty state as a reason to skip remote GitHub migration.

For each repo:

1. read current PROJECT/STATE/RULES/HANDOFF/EVENTS;
2. preserve canonical task file;
3. preserve stricter governance;
4. preserve human handoff prose;
5. change only required control-plane contract files;
6. use normal branch/PR if protected;
7. wait for/verify required checks where available;
8. merge normally;
9. verify target branch after merge;
10. record final target-branch SHA.

No force push.

Final requirement:

> all eight target branches must contain the same supported PROJECT schema.

---

# 4. UCP-R15 — make Adopt safe for heterogeneous projects

Remove hard-coded `TASKS.md` from generic adoption.

Canonical task source resolution order:

1. existing PROJECT.json canonicalTaskSource if valid;
2. Registry/task-source policy;
3. known project mapping during current eight-project migration;
4. bounded deterministic discovery:
   - TASKS.md
   - tasks.md
   - declared nested tracker
5. if multiple candidates conflict: refuse with NEEDS_RECONCILIATION.

Do not guess.

## HANDOFF creation

HANDOFF.md must be Markdown.

Never create JSON content in a `.md` handoff.

Create the standard human-readable handoff template.

Machine state belongs in STATE.json.

Existing HANDOFF.md must never be overwritten by Adopt.

Add adoption tests for:

- FormuLab nested tracker
- PackLab3D lowercase tasks.md
- ScrubBots lowercase tasks.md
- missing-control-plane project
- ambiguous multiple task ledgers

---

# 5. UCP-R16 — implement real post-upgrade state reconciliation

Schema migration is not enough.

After upgrade, run one actor-aware Project State Reconciler.

Evidence precedence:

1. verified active native workflow task
2. valid STATE currentTaskId/currentTaskTitle
3. explicit HANDOFF current task ID
4. exact canonical task lookup
5. declared current cycle pointer
6. bounded legacy fallback only for truly unmigrated projects

Reconcile:

- current milestone
- current cycle/sprint
- current task ID
- current task title
- workflow state
- required actor
- next action
- blockers
- scoped progress
- last audit/session references where available

Do not write invented facts.

If evidence conflicts, write/return:

`NEEDS_RECONCILIATION`

with explicit source conflicts.

Invoke the reconciler from:

- upgrade
- explicit Reconcile
- startup
- 60-second safety pass
- successful local Git repair
- successful safe Git fast-forward
- internal task/workflow/audit/agent transitions

---

# 6. UCP-R17 — type HANDOFF fields correctly

Do not map `active cycle` into currentTaskId.

Parse distinct fields:

- currentTaskId
- currentTaskTitle
- currentMilestone
- currentCycle
- requiredActor
- nextAction
- resumePointer
- lastAudit
- lastAgentSession where present

Support bounded:

- Markdown table rows
- allowlisted label/value lines

Ignore unrelated prose.

Add direct fixture:

- current cycle = `PAG-M00-C003`
- current task ID = a different explicit task ID

and prove no identity conflation.

---

# 7. UCP-R18 — durable event idempotency beyond the display tail

Keep the newest-N tail reader.

But append dedupe cannot depend only on the newest 128 events.

Implement bounded durable idempotency using one of:

- DB-backed event ID index keyed by project + eventId
- compact bounded sidecar index
- deterministic transition idempotency key persisted in DB

Requirements:

- old event ID cannot be appended again after >128 newer events
- no unbounded EVENTS scan on every append
- append-only EVENTS file remains unchanged historically
- migration for existing projects is additive

Add >256 event duplicate-replay test.

---

# 8. Re-run all eight project native screenshot cases

After remediation, reproduce the complete portfolio matrix again.

## AI-Commerce-HQ
Must:
- parse PROJECT.json
- resolve nested `H!veAI/TASKS.md`
- show no malformed banner

## Bulk-Edit
Must:
- parse final schema
- canonical task identity only
- milestone/progress scope aligned

## fmcg-erp-system
Must:
- not show transition prose as synthetic task
- resolve actual canonical next task if authoritative

## FormuLab
Must:
- read nested tracker
- watcher attached to nested path

## PackLab
Must:
- remote identity visible even if local Git absent
- canonical task intelligence available
- repair action safe

## PackLab-3D
Must:
- lowercase tasks.md canonical
- watcher + intelligence use that exact path

## ScrubBots
Must:
- no stale SB-M02-017 unless truly authoritative
- task resolver follows workflow/state/handoff precedence

## ScrubBots-Level-Factory
Must:
- no malformed PROJECT
- local Git re-probe works
- no Markdown task-heading leakage
- correct cycle/task/actor/next-action projection
- no synthetic 4% progress

---

# 9. Command Center whole-portfolio invariants

One bad project must never collapse the full snapshot.

Verify:

- seven healthy + one malformed still returns live portfolio snapshot
- each degraded project gets one needs-attention item
- healthy projects remain interactive/live
- no registry-only global fallback due to a single project
- adopted projects do not show FALLBACK_M08_M09

Command Center and Project Cockpit must use the same normalized current-task resolver.

---

# 10. Local portfolio reconciliation

Inspect Registry's actual local path for all eight projects.

For each:

- re-probe Git
- record remote
- record local Git
- record branch
- record dirty/ahead/behind/diverged
- run upgrade/reconcile only when safe

Rules:

- never reset
- never rebase
- never auto-stash
- never discard untracked
- never force checkout
- never auto-merge divergence

Dirty/ahead/diverged:
report only.

Clean behind:
safe FF only under accepted setting.

Non-Git:
known remote remains visible and repair plan remains explicit.

---

# 11. Direct tests

Add or update direct regressions for:

1. events scalar vs eventSources array
2. upgrader output re-parses
3. all eight current PROJECT shapes
4. all eight final PROJECT shapes
5. FormuLab nested adoption
6. lowercase tasks adoption
7. Markdown handoff creation
8. existing Markdown handoff preservation
9. ambiguous canonical source refusal
10. post-upgrade reconciler
11. cycle != task identity
12. stale ScrubBots task rejection
13. fmcg transition prose rejection
14. scoped progress
15. event replay after >256 records
16. one-bad-project Command Center isolation
17. local Git false→true re-probe
18. remote-known/local-non-Git projection
19. all M16 R82-R85 regressions

---

# 12. Remote repository verification gate

Before writing the final log, fetch the actual target branch PROJECT.json from all eight repositories.

The builder log must include a table:

| Repository | Target branch | Schema | Canonical task source | Remote final SHA |
| ... |

No row may remain `hiveai-project/v1`.

If one cannot be migrated:
- do not claim closure;
- record exact blocker;
- leave M16 open.

---

# 13. Full verification

Run:

- focused control-plane Rust tests
- watcher tests
- Registry tests
- Task Source/Intelligence tests
- Workflow tests
- Git Engine tests
- Command Center tests
- Project Cockpit tests
- Agent tests
- Prompt Engine tests
- Audit Engine tests
- full serialized Rust suite
- all-targets default
- all-targets pty-support
- full frontend suite once with zero retry-dependent failure
- typecheck
- production build
- npm audit high
- cargo fmt
- git diff --check
- publisher rollback
- governed publication
- candidate/stable SHA equality
- PE/startup/shortcut/icon/no-console checks

---

# 14. Final adversarial sweep

Before the builder log, re-audit the whole changed system.

Inspect:

- schema read/write symmetry
- migration round-trip
- canonical task heterogeneity
- Markdown handoff preservation
- current-task precedence
- progress scope
- event idempotency
- watcher sources
- 60s scheduler
- remote fetch/FF
- Registry re-probe
- Command Center fault isolation
- Cockpit convergence
- all eight remote repository files
- all eight local project statuses
- M16 R82-R85

If any adjacent BLOCKER or MAJOR exists, fix it in this same run.

Do not stop for another prompt.

---

# 15. Explicit gates

1. Read GPT.md.
2. Sync H!veAI FF-only.
3. Record starting HEAD.
4. Read M16F REV3 audit.
5. Confirm R82-R85 remain closed.
6. Reproduce R13.
7. Reproduce R14.
8. Reproduce R15.
9. Reproduce R16.
10. Reproduce R17.
11. Reproduce R18.
12. Split events/eventSources schema.
13. Fix legacy eventSource migration.
14. Add round-trip parser test.
15. Fix Adopt canonical source resolution.
16. Fix Adopt Markdown HANDOFF.
17. Add ambiguous-source refusal.
18. Implement post-upgrade reconciler.
19. Wire reconciler into upgrade.
20. Wire reconciler into startup.
21. Wire reconciler into 60s pass.
22. Wire reconciler into Git repair/sync.
23. Type HANDOFF fields separately.
24. Add cycle/task distinction test.
25. Add durable event idempotency.
26. Add >256 replay test.
27. Migrate Bulk-Edit remote.
28. Migrate FormuLab remote.
29. Migrate PackLab remote.
30. Migrate PackLab-3D remote.
31. Migrate ScrubBots remote.
32. Migrate Level Factory remote.
33. Verify AI-Commerce remote.
34. Verify fmcg remote.
35. Fetch all eight target PROJECT.json files.
36. Prove all eight schema values match.
37. Verify all eight canonical task paths.
38. Preserve all task ledgers.
39. Preserve all governance.
40. Preserve all handoff prose.
41. Reconcile all safe local roots.
42. Record dirty roots without mutation.
43. Record non-Git roots with known remote.
44. Re-run screenshot fixtures.
45. Re-run Command Center isolation fixture.
46. Re-run stale-task fixtures.
47. Re-run scoped-progress fixtures.
48. Run focused Rust.
49. Run focused frontend.
50. Run R82-R85 regression.
51. Run full Rust.
52. Run pty.
53. Run full frontend once green.
54. Run typecheck/build/audit/fmt/diff.
55. Perform final adversarial sweep.
56. Fix adjacent BLOCKER/MAJOR same run.
57. Re-run affected tests.
58. Governed publish.
59. Verify stable SHA.
60. Native smoke if feasible.
61. Create immutable builder log.
62. Record all remote target SHAs.
63. Record all local reconciliation statuses.
64. Commit scoped changes.
65. Push normally.
66. Verify local/origin equality.
67. Leave M16 OPEN pending independent strict re-audit + owner native acceptance.
68. Do not activate M17.
69. Do not start M21.

---

# 16. Required builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16G_UNIFIED_PROJECT_CONTROL_PLANE_FINAL_WHOLE_SYSTEM_CLOSURE_REMEDIATION_LOG.md`

Required sections:

- starting HEAD
- implementation commits
- R13-R18 reproduction
- R13-R18 closure
- schema round-trip proof
- eight-repository remote migration matrix
- eight-project local reconciliation matrix
- handoff preservation proof
- canonical task source proof
- Command Center isolation proof
- screenshot fixture results
- event idempotency proof
- full tests
- adversarial sweep
- publication SHA
- final pushed HEAD
- native acceptance still pending

End exactly with:

`M16G UNIFIED PROJECT CONTROL PLANE FINAL WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.`

`M16 remains OPEN.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`
