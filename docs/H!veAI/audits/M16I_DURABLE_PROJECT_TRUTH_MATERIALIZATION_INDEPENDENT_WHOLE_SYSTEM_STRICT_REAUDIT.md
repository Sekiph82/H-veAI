# M16I Durable Project Truth Materialization — Independent Whole-System Strict Re-Audit

Date: 2026-09-09  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16I_DURABLE_PROJECT_TRUTH_MATERIALIZATION_HEALTH_REMOTE_OBSERVATION_CLOSURE_LOG.md`  
Implementation commit reviewed: `994167ef7d2e0f45812ac47f3b8a5b79db620d60`

## Verdict

**FAIL / CHANGES REQUIRED**

### Previously named M16I findings
- UCP-R22: PARTIALLY CLOSED
- UCP-R23: CLOSED
- UCP-R24: CLOSED
- UCP-R25: PARTIALLY CLOSED
- UCP-R26: CLOSED

### New whole-system findings
- BLOCKER: 3
- MAJOR: 2
- MINOR: 0

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap progress remains `16 / 20 = 80%`.

M16I materially improves the system. A shared materializer now exists, health precedence is repaired, multiple active workflow ambiguity fails closed, remote-observation state is persisted, and workflow/audit/agent/task paths attempt materialization. However, strict inspection of the actual eight-repository contract exposed three cross-repository integrity blockers that fixture-only tests did not catch.

---

# Confirmed closures

## UCP-R23 — CLOSED

`health_for(...)` now preserves `NEEDS_RECONCILIATION` before Git cleanliness and remote-sync health. Clean/in-sync Git can no longer overwrite unresolved truth as HEALTHY.

## UCP-R24 — CLOSED

The resolver no longer ranks multiple active workflow candidates by latest event/attention/task ID. Ambiguity is explicit and fails closed unless a valid explicit identity disambiguates under the conflict policy.

## UCP-R26 — CLOSED

Migration v19 persists remote observation status/error/upstream/ahead/behind/divergence. `sync_remote(...)` records success and degraded fetch failures and the project surfaces consume that projection.

---

# UCP-R27 — BLOCKER
## The materializer corrupts the shared cross-repository projectKey identity

Every actual tracked repository currently has a stable repository-owned `projectKey` in both `.hiveai/PROJECT.json` and `.hiveai/STATE.json`.

Examples independently verified on the target branches:

- AI-Commerce-HQ → `ai-commerce-hq`
- Bulk-Edit → `bulk-edit`
- fmcg-erp-system → `fmcg-erp-system`
- FormuLab → `formulab`
- PackLab → `packlab`
- PackLab-3D → `packlab-3d`
- ScrubBots → `scrubbots`
- ScrubBots-Level-Factory → `scrubbots-level-factory`

Production `materialize_project_truth(...)` instead executes:

`state_object.insert("projectKey", project_id)`

where `project_id` is H!veAI's local Registry UUID.

The emitted materialization event also uses:

`project_key: project_id`

This conflates:

1. stable repository control-plane identity; and
2. machine-local H!veAI Registry identity.

After the first local materialization of an existing adopted repository, STATE can therefore be rewritten from a stable repo key such as `scrubbots` to a machine-local UUID. The appended EVENTS row receives the same wrong identity.

This breaks the owner's central contract that every repository carries the same portable file format and can be read consistently by Codex, Claude, ChatGPT, GitHub, and H!veAI on another machine.

### Required remediation

- Read the authoritative `ProjectDocument.project_key`.
- Preserve that exact stable key in STATE, HANDOFF references, and EVENTS.
- Keep local Registry UUID only in Registry DB / `localRegistryId`.
- Never rewrite a repository's stable projectKey from local registration identity.
- Add direct migration/materialization tests using the actual eight projectKey slugs, not only newly adopted UUID-shaped fixtures.
- Add cross-machine re-registration regression proving a new local Registry UUID leaves repository files byte-equivalent except legitimate state changes.

---

# UCP-R28 — BLOCKER
## H!veAI appends EVENTS rows that do not conform to the shared event schema already installed in all repositories

The shared `.hiveai/RULES.md` contract defines event rows with:

- `schema: "hiveai-event/v1"`
- `eventId`
- `projectKey`
- `type`
- `at`
- `actor`
- `taskId`
- `workflowState`
- `summary`
- `commit`
- `auditId`
- `sessionId`

Actual committed EVENTS files use that schema.

Production `EventRecord` used by H!veAI materialization currently contains only:

- eventId
- type
- projectKey
- at
- actor
- summary
- taskId
- evidenceRefs

It has no `schema`, `workflowState`, `commit`, `auditId`, or `sessionId`.

Therefore H!veAI can append a structurally different row into the same standardized EVENTS.jsonl file, producing mixed-format history.

This directly defeats the user's requirement that every AI sees and writes the same project-tracking format.

### Required remediation

Create one canonical typed `hiveai-event/v1` writer contract.

At minimum every newly emitted row must serialize:

`schema,eventId,projectKey,type,at,actor,taskId,workflowState,summary,commit,auditId,sessionId`

Optional values serialize as null where appropriate.

Additional bounded `evidenceRefs` may exist only as a backward-compatible extension.

Reader compatibility may continue accepting historical variants, but all new writes must be canonical v1.

Add exact round-trip tests against real existing event rows from:

- AI-Commerce-HQ
- ScrubBots-Level-Factory

and assert new H!veAI materialization rows have the same core schema.

---

# UCP-R29 — BLOCKER
## State-changing operations still report success when durable project-truth materialization fails

M16I wires many lifecycle paths to the materializer, but the calls are deliberately discarded:

- workflow transition: `let _ = materialize_project_truth(...)`
- workflow override: same
- task-intelligence refresh: same
- audit lifecycle persistence: same
- agent completion/failure/recovery: same

These operations first commit their native DB/domain state and then ignore any STATE/HANDOFF/EVENTS materialization error.

Failure examples include:

- STATE malformed
- file locked
- atomic replace failure
- governed HANDOFF conflict
- event append/index failure
- disk full / permission failure

The caller still receives success, while the user's portable repository control files remain stale.

That violates the explicit contract:

> after every meaningful action the tracked project files update immediately and H!veAI shows the same truth.

This is not solved by retrying only when somebody later opens a snapshot.

### Required remediation

Do not attempt an unsafe distributed rollback between SQLite and the filesystem.

Instead implement durable convergence state/outbox semantics.

After a domain transition commits:

1. attempt one materialization;
2. if it succeeds, persist materialization status CURRENT;
3. if it fails, persist `TRUTH_SYNC_PENDING/DEGRADED` with bounded error and trigger/revision;
4. surface that state in Command Center/Cockpit immediately;
5. enqueue/retry through watcher/safety reconciliation;
6. clear pending only after the exact later truth revision is materialized successfully.

A lifecycle operation may return its domain success, but it must also expose/record that project truth synchronization is pending. It must never silently claim fully synchronized success.

Add failpoint tests for workflow, audit, agent, and task-refresh materialization failures, including restart recovery.

---

# UCP-R30 — MAJOR
## Project-specific HANDOFF content is lost on the second automated materialization

`render_materialized_handoff(...)` preserves an existing handoff only when it does **not** already contain the normalized marker.

First run:

- old project-specific HANDOFF is appended under `## Project-specific notes`.

Second run:

- existing now contains `<!-- H!veAI normalized handoff v1 -->`;
- `preserved = String::new()`;
- the previously preserved project-specific notes disappear.

This is a deterministic two-pass data-loss bug.

It is especially dangerous because watcher re-entry is expected after the first STATE/HANDOFF write.

### Required remediation

Use explicit managed block boundaries, for example:

- `<!-- HIVEAI:BEGIN MANAGED -->`
- `<!-- HIVEAI:END MANAGED -->`

Replace only the managed block.

Everything outside the block must remain byte-preserved across unlimited materializations.

Add:

1. first materialization preservation test;
2. second materialization preservation test;
3. ten-repeat idempotency test;
4. owner-governed HANDOFF never-mutated test;
5. ScrubBots-Level-Factory governance regression.

---

# UCP-R31 — MAJOR
## Invalid progressScope can still produce a progressPercent with no corresponding scope

M16I correctly detects a stale/mismatched requested progress scope and sets:

- `progress_scope_valid = false`
- `progress_scope = None`

But `progress_percent` is then built with `.or_else(...)`.

That fallback still computes milestone progress from canonical tasks without requiring `progress_scope_valid`, and it does not restore a generated `progressScope`.

A result can therefore contain:

- `progressPercent = 42`
- `progressScope = null`

after an invalid/stale scope was explicitly detected.

That contradicts the exact-scope invariant and makes the persisted state ambiguous.

### Required remediation

Enforce one invariant:

`progressPercent.is_some() => progressScope.is_some() && scope exactly matches resolved milestone/cycle`

If progress is computed from canonical milestone/cycle grouping, set the corresponding exact generated scope.

If a stale explicit scope is treated as reconciliation evidence, either:

- fail closed with no percent until reconciled; or
- compute from authoritative scope and explicitly replace the stale scope with the exact generated authoritative scope.

Never emit percent-without-scope.

Add direct invariant/property tests.

---

# Eight-repository contract verification

Independent target-branch inspection confirms all eight current PROJECT.json files use:

`hiveai-project-control-plane/v1`

and preserve heterogeneous canonical task sources:

| Project | Canonical task source |
|---|---|
| AI-Commerce-HQ | `H!veAI/TASKS.md` |
| Bulk-Edit | `TASKS.md` |
| fmcg-erp-system | `TASKS.md` |
| FormuLab | `docs/FORMULAB_V1_TASK_TRACKER.md` |
| PackLab | `TASKS.md` |
| PackLab-3D | `tasks.md` |
| ScrubBots | `tasks.md` |
| ScrubBots-Level-Factory | `tasks.md` |

The failure is therefore not absence of the standardized files. The remaining problem is that M16I's local writer does not yet preserve the standardized identities and event format already installed in those repositories.

---

# Required final remediation strategy

Do not patch these findings separately.

The next remediation must close UCP-R27 through UCP-R31 in one continuous run and then perform a fresh whole-M16 adversarial sweep.

Final required invariants:

1. stable repository projectKey is portable and never replaced by local Registry UUID;
2. localRegistryId remains machine-local metadata only;
3. every new EVENTS row conforms to canonical `hiveai-event/v1`;
4. state-changing operations can never silently lose materialization failures;
5. pending truth sync is durable, visible, restart-safe, and retried;
6. HANDOFF unmanaged content survives unlimited repeated materializations;
7. percent can never exist without exact progress scope;
8. Command Center, Cockpit, STATE, HANDOFF, EVENTS, and fresh restart converge on the same truth;
9. all eight actual repository contract fixtures pass;
10. M16 R82-R85 and all UCP R13-R26 regressions remain green.

Only after an independent whole-system PASS plus owner native/visual acceptance may M16 close and M17 activate.
