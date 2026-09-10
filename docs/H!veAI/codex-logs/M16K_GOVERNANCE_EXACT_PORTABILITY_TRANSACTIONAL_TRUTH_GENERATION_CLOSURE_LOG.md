# M16K Governance-Exact Portability + Transactional Truth Generation Closure Log

Date: 2026-09-09
Repository: Sekiph82/AI-Commerce-HQ
Branch: H!veAI
Status: implementation remediation complete; M16 remains OPEN pending independent whole-M16 strict re-audit and owner native/visual acceptance.

## Starting HEAD

- `git fetch origin H!veAI` completed.
- `git merge --ff-only origin/H!veAI` completed.
- Starting local and origin HEAD: `96a3383aa002b0372b5e3d0e34eecc8ea4245968`.
- The pre-existing untracked `start-demo.bat` and `task.md` were preserved and not touched.
- `H!veAI/GPT.md`, all M16A-J prompts/logs/audits, the M16J strict re-audit, the required source modules, tests, and all eight checked-out PROJECT/RULES contracts were read.

## Implementation commits

- `754715d640d40b1db9eaf8c38376c541f49b992f`: close M16K truth governance and generation findings.
- This immutable evidence log is committed separately after the final verification and publication gates.

## R32-R35 reproduction and closure

### UCP-R32: typed HANDOFF governance

Reproduction: the pre-fix code searched serialized PROJECT JSON for unrelated policy substrings and did not recognize the installed `governance.builderMayMutateHandoff=false` contract. It could therefore attempt an unauthorized HANDOFF mutation.

Closure: `ProjectGovernance`, `HandoffPolicy`, and `handoff_governance` now parse typed machine-readable fields. Precedence is explicit PROJECT deny, stricter RULES deny, then standard H!veAI allowance. Denied repositories skip HANDOFF rendering completely, materialize STATE, append permitted EVENTS, and surface a governance note. The actual ScrubBots-Level-Factory fixture proves malformed HANDOFF markers do not block STATE materialization and denied HANDOFF bytes remain unchanged.

### UCP-R33: portable PROJECT identity

Reproduction: adoption wrote the current machine Registry UUID into version-controlled PROJECT.json as `localRegistryId`.

Closure: `local_registry_id` remains backward-readable legacy metadata but is `skip_serializing`; adoption writes no local Registry identity. Registry mapping remains in SQLite. The cross-machine fixture proves stable project identity is retained without serializing either machine UUID.

### UCP-R34: transactional truth generation

Reproduction: domain rows committed first and post-commit materialization was best effort, leaving a crash window in which filesystem truth could remain falsely CURRENT.

Closure: migration v21 adds monotonic `truth_generation` and `truth_materialized_generation`. Every truth-changing domain transaction marks dirty before commit. Materialization reads generation G, writes bounded files, and marks generation G current only when G is still authoritative. Startup and safety retry select every active project where materialized generation is behind authoritative generation.

### UCP-R35: monotonic concurrency safety

Reproduction: concurrent materializers could let an older completion or failure overwrite newer truth-sync status.

Closure: PENDING, CURRENT, and DEGRADED transitions use generation guards and a retry-attempt compare-and-set token. Older runs become historical no-ops against current state. The regression covers stale completion, stale failure, retry recovery, no counter rollback, and same-generation concurrency.

## Typed governance architecture

- `PROJECT.json` governance is deserialized into typed optional fields; unknown or malformed governance fails closed.
- Supported policy values are explicit `automated`, `manual`, `owner-only`, and `owner-approved`.
- RULES is parsed line-by-line only for bounded authority phrases, never by searching serialized JSON.
- Governance denial is evaluated before any managed HANDOFF marker parsing or renderer call.
- Automation-allowed projects retain the M16J single managed block, repeat idempotency, and unmanaged-byte preservation behavior.

## Actual Level Factory governance proof

- Literal project key: `scrubbots-level-factory`.
- Literal canonical task source: `tasks.md`.
- Literal PROJECT field: `governance.builderMayMutateHandoff: false`.
- Literal RULES authority includes: `Codex may not rewrite HANDOFF or independent audit history unless an owner-approved prompt explicitly changes governance.`
- Result: HANDOFF automation denied; malformed markers are not parsed on the denied path; STATE materialization remains permitted; canonical EVENTS behavior remains available; HANDOFF bytes are preserved.

## PROJECT portability proof

- The checked-in fixture includes the relevant literal PROJECT identity/governance shapes and RULES evidence for all eight repositories.
- New PROJECT serialization omits `localRegistryId`.
- Stable portable identity is `projectKey`; local Registry UUID is not used in PROJECT bytes, canonical event identity, or materialized truth identity.
- No external repository was modified by the tests.

## Truth-generation architecture

- Schema version: 21, migration name `transactional_truth_generation`.
- `truth_generation` is authoritative dirty intent.
- `truth_materialized_generation` records the last filesystem generation known complete.
- CURRENT is valid only when the two generations match.
- PENDING/DEGRADED status and generation mismatch both outrank Git health and prevent HEALTHY projection.
- Retry selection is bounded to eight active projects per safety pass and ordered by generation.
- Existing M16J durable status, revision, error, trigger, timestamps, retry bounds, and restart behavior remain preserved.

## Lifecycle transaction matrix

| Domain operation | Transaction marks generation? | Post-commit materializer? | Restart recovery test? |
| --- | --- | --- | --- |
| workflow transition | Yes, before commit | Yes | Yes |
| workflow override | Yes, before commit | Yes | Yes |
| task refresh persistence | Yes, before commit | Yes | Yes |
| audit lifecycle | Yes, before commit | Yes | Yes |
| agent finish | Yes, before commit | Yes | Yes |
| agent failure | Yes, before commit | Yes | Yes |
| agent recovery | Yes, before commit | Yes | Yes |
| explicit reconcile | Existing durable path | Yes | Yes |
| adoption | Yes, before commit | Yes | Yes |
| remote fast-forward where truth changes | Yes, before merge commit | Yes | Yes |
| watcher-driven task truth refresh | Yes through task refresh transaction | Yes | Yes |

Remote observation that only records remote counts and does not change local project truth does not increment the generation; any subsequent fast-forward or local truth mutation does.

## Crash-window proof

The direct regression commits a representative domain mutation and generation increment, omits materializer invocation, verifies authoritative generation is ahead of materialized generation and health is not CURRENT/HEALTHY, then invokes startup-style safety retry and verifies generation equality and CURRENT. The same durable pattern is wired into audit and agent lifecycle transactions. A crash after domain commit therefore leaves durable dirty intent rather than a falsely current projection.

## Concurrency monotonicity proof

The direct regression advances from generation 20 to 21, completes generation 21, then completes and fails the older generation. Current state remains generation 21 CURRENT. Guarded retry tokens prevent two same-generation attempts from overwriting one another, and all generation increments are monotonic.

## Restart recovery proof

Migration and control-plane tests verify missing STATE recovery, persisted DEGRADED/PENDING retry, generation mismatch precedence, migration idempotency, bounded retries, and database restart-style re-opening. A missing or malformed filesystem projection cannot be reported as healthy while its generation is incomplete.

## Eight literal portfolio fixtures

`src-tauri/fixtures/m16k/portfolio.json` records the requested stable key/source matrix, source checkout SHA, relevant PROJECT shape, and RULES evidence:

| Repository | Stable key | Required canonical source | Source SHA recorded |
| --- | --- | --- | --- |
| AI-Commerce-HQ | `ai-commerce-hq` | `H!veAI/TASKS.md` | `96a3383aa002b0372b5e3d0e34eecc8ea4245968` |
| Bulk-Edit | `bulk-edit` | `TASKS.md` | `826299af185a977ef546a025777a78aa114bfc52` |
| fmcg-erp-system | `fmcg-erp-system` | `TASKS.md` | `e61236bf1924b2f70c0d61279705e1c7b9bd9f2a` |
| FormuLab | `formulab` | `docs/FORMULAB_V1_TASK_TRACKER.md` | `63a3ebb50273418a9da63165b7c1e4cbf1c39c92` |
| PackLab | `packlab` | `TASKS.md` | `4c7c33aebfcf6d70c8d6e6f4b03b5e589b796f3d` |
| PackLab 3D | `packlab-3d` | `tasks.md` | `534df555e46e9d18781a05a08508a317ec74ec4a` |
| ScrubBots | `scrubbots` | `tasks.md` | `b4ecabb34b8f8f46f2999a0ecc83dfa9c61f02cd` |
| ScrubBots-Level-Factory | `scrubbots-level-factory` | `tasks.md` | `134f350d4b0500fe4ae362f24b724232d627086e` |

## Full tests

- Focused M16K control-plane tests: PASS, 4/4.
- Full serialized Rust library regression: PASS, 402/402.
- Default all-targets Rust regression: PASS, 402/402 plus 0 main tests.
- `pty-support` all-targets regression: PASS, 403/403 plus 0 main tests.
- Frontend Vitest: PASS, 125/125 across 15 files.
- TypeScript typecheck: PASS.
- Production frontend build: PASS.
- `npm audit --audit-level=high`: PASS for the requested threshold; no high/critical findings. Existing moderate Vitest advisory remains and requires a breaking upgrade.
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.
- Publisher failure/rollback harness: PASS, 9/9.

All prior UCP and M16 R82-R85 regressions remain green in the full suites, including canonical events, progress scope, workflow, audit, agent, watcher, Command Center, Cockpit, ACL, degraded paths, and M16 audit fixtures.

## Whole-M16 adversarial sweep

PASS across exact portfolio governance, typed HANDOFF authority, portable identity, local-only Registry identity, generation and migration integrity, crash boundaries, concurrency, canonical events, resolver/materializer, HANDOFF preservation, task intelligence, workflow, audit, agent sessions, watcher, remote observation, Command Center, Cockpit, database migrations, ACL/native command boundaries, degraded paths, tests, and publication. No adjacent BLOCKER or MAJOR defect was found after the named fixes.

## Publication SHA

- Governed `publish-dev-qa.ps1` completed with a Tauri `--no-bundle` production build.
- Candidate and stable executable hashes matched during the atomic swap.
- Published stable executable: `H!veAI/dev-bin/H!veAI.exe`.
- Stable executable SHA-256: `5B157BAB4D56C76E9887580B751BEAF78FAACFFC695EF7DB5DDCB14E662040F6`.
- PE signature check: PASS (`MZ`).
- Startup readiness, shortcut target, icon target, no development ports, no visible console host, and cleanup checks: PASS.
- Stable icon SHA-256: `F0C1CC62F000C959AB10493B902E1A7B2F4E10E1E4BB9C837BF3841BC194C5AD`.
- Canonical opening-video SHA-256 preserved: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`.

## Final pushed HEAD

- The implementation commit is `754715d640d40b1db9eaf8c38376c541f49b992f`.
- The final log-only commit and pushed HEAD are recorded after this file is committed.
- Local `H!veAI` and `origin/H!veAI` are required to be byte-for-byte equal at the final proof.

## Native acceptance pending

Automated native publication and smoke gates passed. Independent whole-M16 strict re-audit and owner native/visual acceptance remain pending. No visible UI redesign was made. M16 was not self-closed.

M16K WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.

M16 remains OPEN.

M17 NOT ACTIVATED.

M21 NOT STARTED.
