# M16D Whole-M16 Final Closure Remediation Log

Status: implementation remediation complete; M16 remains OPEN pending independent whole-M16 strict re-audit and user native/visual acceptance.

Date: 2026-09-08
Branch: H!veAI
Starting synchronized HEAD: `93906d535bc3419ec4ff84696226beeeffb65738`
Implementation commit: `24e91047addcd9beffe99e5fce7dc7ac7503de12`
Adversarial-sweep follow-up commit: `7033d0c0292f7e81572756fa302e126f8cd56d59`

## Scope

This run obeyed `H!veAI/GPT.md`, synchronized with `origin/H!veAI` using fast-forward-only history, and executed the authoritative M16D prompt as one whole-milestone remediation. It closed the named open set M16-R74 through M16-R81 and did not close M16, activate M17, or start M21.

Unrelated parent-workspace files were preserved and excluded from the commits: `../start-demo.bat` and `../task.md`.

## Findings Closed

### M16-R74: required-evidence semantic truth

- Added a required/referenced evidence quality set for PASS eligibility.
- Unrelated `CLAIM_ONLY` builder claims no longer veto a PASS supported by verified required evidence.
- Claim-only evidence remains unable to verify coverage when it is required or referenced.
- Project-level audits use an explicit `project-audit` `NOT_APPLICABLE` coverage row when no task requirements apply.

### M16-R75: canonical requirement coverage

- Task requirements materialize as canonical `task-requirement-{index}` IDs in the audit input contract.
- Required coverage must contain exactly one row per canonical requirement.
- Unknown, duplicate, missing, and finding-level unknown requirement references are rejected deterministically.

### M16-R76: immutable re-audit target compatibility

- Re-audit loads the prior immutable target natively and validates project, task/freeform identity, Git scope, commit-range base, and session/prompt provenance.
- Re-audit ignores unrelated current-form target changes and uses the selected prior target.
- A permitted remediation-session/prompt provenance transition is explicit and exact.

### M16-R77: inherited prior evidence provenance

- Omitted prior findings remain OPEN and are persisted with `inherited_prior_audit_id` plus separate inherited prior evidence references.
- Prior evidence references never enter the current audit evidence namespace.
- Closure and supersession still require current evidence and rationale.
- Migration v17 adds durable inherited-provenance columns and an index without rewriting prior rows.

### M16-R78: scope-pure evidence planning and freshness

- `WORKING_TREE` uses unstaged and untracked evidence plus the selected diff.
- `STAGED` uses staged evidence and index content.
- `COMMIT_RANGE` uses only the selected committed head content.
- Same-HEAD working-tree content changes participate in freshness identity.
- Source evidence is bounded, contained, and read from the selected authority rather than unrelated project context.

### M16-R79: bounded readers and hashes

- Test-body scanning uses fixed byte and line limits with explicit truncation.
- Builder-log claims use bounded per-file and total budgets before allocation.
- Untracked content identity uses streaming SHA-256 with fixed file-count, total-byte, and time limits.
- Working-tree tracked identity uses streaming SHA-256 with a fixed aggregate byte limit and explicit incomplete identity state.

### M16-R80: large Git change sets

- Git patch capture is bounded independently from the full change identity.
- Legitimate large diffs in `WORKING_TREE`, `STAGED`, and `COMMIT_RANGE` return successfully with `truncated = true` while preserving a full change-set identity.
- Identity-incomplete states are visible to audit freshness and cannot claim a high-confidence PASS.

### M16-R81: authority-derived target UX

- Audit target choices are explicit: current working tree, staged changes, implementation session, previous audit/current remediation, and manual commit range.
- Manual commit range is labeled advanced/manual and persists `targetOrigin = MANUAL`.
- Session and prior-audit targets carry exact persisted provenance; arbitrary Git argv/path input is not introduced.

## Adjacent Adversarial Sweep

The required whole-M16 sweep covered audit error handling, file containment and bounds, PASS eligibility, re-audit consistency, persisted namespaces, Git scope isolation, freshness inputs, UI audit actions, native ACL exposure, degraded provider states, transaction rollback, and model semantic validation.

The sweep found one additional major-risk condition: working-tree tracked-file hashing was streamed but lacked an aggregate byte ceiling. `ADJ-M16-TRACKED-IDENTITY-BOUND` was fixed in commit `7033d0c0292f7e81572756fa302e126f8cd56d59`; no additional BLOCKER or MAJOR defects remained after the fix.

## Direct Evidence

- Audit-focused Rust tests: 28 passed, 0 failed, serialized.
- Git-focused Rust tests: 28 passed, 0 failed, serialized.
- Full default Rust library regression: 377 passed, 0 failed, serialized.
- Full `--all-targets` default regression: 377 passed, 0 failed, serialized.
- Full `pty-support` regression: 378 passed, 0 failed, serialized.
- Frontend regression: 124 passed, 1 existing timing-sensitive project-cockpit test failed in the full parallel run; the isolated rerun passed 8/8. No M16D-focused frontend test failed.
- TypeScript typecheck: passed.
- Production frontend build: passed.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- Publisher rollback harness: all 9 scenarios passed.
- `git diff --check`: passed.

## Governed Publication

The governed publisher built `src-tauri/target/release/hiveai-desktop.exe`, performed candidate and stable PE validation, frontend readiness smoke, forbidden-port checks, visible-console-host checks, shortcut/icon checks, and exact candidate/stable hash equality.

Published stable executable: `dev-bin/H!veAI.exe`

- PE signature: `MZ`
- PE subsystem: `2`
- Size: `22,470,656` bytes
- SHA-256: `42AA1D34F43613F83975085C338BFFDA7EE25C29B08B61126C698F6CC71AE22C`

Native/visual acceptance remains the user's responsibility where required by the independent audit contract; automated publication does not close M16.

## Publication Boundary

All scoped source, migration, UI contract, direct-test, and this immutable log change were committed and pushed normally. The final pushed HEAD and `origin/H!veAI` were compared after push and are equal; the concrete SHA is reported with the final handoff because a commit cannot contain its own SHA before creation.

M16D WHOLE-M16 REMEDIATION COMPLETE / PENDING INDEPENDENT WHOLE-M16 STRICT RE-AUDIT + USER NATIVE/VISUAL ACCEPTANCE.

M16 remains OPEN.

M17 NOT ACTIVATED.

M21 NOT STARTED.
