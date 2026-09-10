# M16E + Unified Project Control Plane — Independent Whole-System Strict Re-Audit

Date: 2026-09-08  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16E_AND_UNIFIED_PROJECT_CONTROL_PLANE_LIVE_SYNC_IMPLEMENTATION_LOG.md`  
Implementation commit reviewed: `9c2cefdf27660eb386c20437af5f6e35aa0895c9`  
Compatibility follow-up reviewed: `7266601c1ba27c2415ab301ce7ae99c989242fca`

## Verdict

**FAIL / CHANGES REQUIRED**

### M16 audit-engine portion
- R82: CLOSED
- R83: CLOSED
- R84: CLOSED
- R85: CLOSED

The M16 audit-engine remediation is materially present in production source.

### Unified Project Control Plane / Live Sync portion
- BLOCKER: 3
- MAJOR: 7
- MINOR: 1

The combined M16E run cannot be accepted because the newly implemented control plane is not yet compatible with the actual eight-project portfolio and does not yet provide the event-driven / safety-reconciled behavior requested by the owner.

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap completed progress remains `16 / 20 = 80%`.

---

# Confirmed M16 closures

## R82 — CLOSED
`persist_run(...)` now applies prior-finding dispositions/inheritance before the final `validate_semantic_evaluation(...)` call. Persisted PASS can therefore be rejected/downgraded after inherited OPEN release blockers are materialized.

## R83 — CLOSED
STAGED and COMMIT_RANGE source extraction now uses `git_engine::run_git_bounded(...)` and records explicit TRUNCATED/UNAVAILABLE source evidence.

## R84 — CLOSED
Builder-log ranking no longer contains the earlier exact-name preference for `M16C_REV2`; it uses prior audit/session/prompt/task relevance plus general milestone/log relevance.

## R85 — CLOSED
The Project Cockpit async dependency was narrowed from mutable project object identity to stable `project?.id`, and the builder reports one fully green 125-test frontend run.

---

# UCP-R01 — BLOCKER
## The actual tracked portfolio is not on the schema the new H!veAI runtime accepts

Production `control_plane.rs` accepts only:

`hiveai-project-control-plane/v1`

and expects `repository` to be an object:

```json
{"owner":"...","name":"...","branch":"..."}
```

However the already-merged control-plane files in the other tracked repositories currently use:

`hiveai-project/v1`

and a string repository value such as:

`"repository": "Sekiph82/Bulk-Edit"`

Confirmed current target-branch examples:

- Bulk-Edit
- fmcg-erp-system
- FormuLab
- PackLab
- PackLab-3D
- ScrubBots
- ScrubBots-Level-Factory

All seven use the older incompatible PROJECT.json shape.

The compatibility follow-up intentionally routes that schema back through the legacy dashboard path, so those projects are **not adopted control-plane v1 projects** even after the local Git pulls complete.

This defeats the primary product goal of making every tracked project speak one live state contract.

### Required fix

Implement a real portfolio migration/upgrader, then migrate all eight repositories to one accepted schema.

Do not merely add another compatibility branch.

---

# UCP-R02 — BLOCKER
## Existing HANDOFF/state files cannot be safely adopted by the current create-if-missing implementation

Production reads `.hiveai/HANDOFF.md` through generic JSON deserialization into `HandoffDocument`.

But several tracked repositories correctly contain human-readable Markdown HANDOFF files, including the current ScrubBots-Level-Factory operational handoff.

Examples in the portfolio are Markdown headings/tables, not JSON.

`adopt(...)` uses create-if-missing semantics, so an existing incompatible HANDOFF is preserved but remains unparsable. The project can therefore never become `adopted=true` through the current adoption command.

The same migration problem applies to the older STATE/PROJECT shapes already installed in the portfolio.

### Required fix

Choose and implement one explicit migration-safe contract.

Preferred:

- keep `HANDOFF.md` human-readable Markdown;
- parse a bounded standardized metadata section or derive resume facts from STATE.json;
- never require project-specific historical handoff prose to become JSON.

Existing stricter handoff/governance content must remain preserved.

---

# UCP-R03 — BLOCKER
## The live watcher ignores the declared canonical task source for adopted projects

`configure_project_watcher(...)` currently calls:

`control_plane::watcher_sources(root_path, &[])`

It passes an empty declared-source list.

`watcher_sources(...)` itself always starts with:

`root.join("TASKS.md")`

Therefore adopted projects whose canonical task source is:

- `tasks.md`
- `docs/FORMULAB_V1_TASK_TRACKER.md`
- another declared path

do not receive the required canonical-task watcher from the control-plane configuration.

This directly breaks the requested invariant:

> any project task/workflow state change must appear in H!veAI immediately.

### Required fix

Resolve the actual PROJECT.json first and attach exactly the bounded canonical task source + declared safe sources.

Do not hard-code `TASKS.md` as the portfolio-wide watcher source.

---

# UCP-R04 — MAJOR
## The one-time adoption/reconciliation phase required by the prompt was not executed

The builder log explicitly states:

> “A read-only scan of the current registry found eight ACTIVE projects. No registered root was modified by this run.”

and:

> “Owner adoption ... remains pending.”

But the authoritative M16E prompt explicitly required a safe one-time local adoption/reconciliation for every accessible registered repository.

The run implemented an adoption command but stopped before using it.

As a result, the product currently has a control-plane engine but the user's portfolio is not actually migrated into it.

### Required fix

After implementing the migration-safe schema, execute bounded local reconciliation for all registered roots that are safe to update.

Dirty/diverged repos must be reported, not modified.

---

# UCP-R05 — MAJOR
## No periodic 60-second safety reconciliation or background remote-fetch scheduler exists

The requested contract was:

- filesystem watcher as primary;
- 60-second reconciliation safety net;
- bounded remote fetch/reconciliation so GitHub-only changes eventually become visible locally.

The production watcher worker is event-driven only. No 60-second reconciliation timer is present in `watcher.rs`.

`sync_remote(...)` exists as a command, but there is no background scheduler that invokes safe fetch/reconciliation on registered projects.

Therefore:

- a missed filesystem event can remain stale indefinitely;
- a GitHub-only change remains invisible until the user or another process manually fetches/pulls.

### Required fix

Add:

- idempotent 60-second safety reconciliation;
- bounded remote fetch scheduler;
- affected-project refresh after fetch/fast-forward;
- lifecycle shutdown/cancellation safety.

---

# UCP-R06 — MAJOR
## Adopt / safe-sync / local-Git-repair functionality is not wired into the native UI

TypeScript helpers exist:

- `adoptControlPlane`
- `syncControlPlaneRemote`
- `getLocalRepairPlan`

but the current `pages.tsx` does not use those functions.

No owner-facing control-plane adoption action, safe fast-forward toggle, or “Connect local Git” workflow is present in the native pages.

The user therefore cannot operate the newly implemented lifecycle from H!veAI itself.

### Required fix

Wire explicit UI actions and Settings into the native product.

Safety rules remain:

- no reset;
- no rebase;
- no auto-stash;
- no destructive overwrite;
- dirty/ahead/diverged requires attention.

---

# UCP-R07 — MAJOR
## Known remote identity is still lost when local Git is missing

For non-Git local folders, `control_plane::snapshot(...)` leaves Git state at:

- local_status = UNKNOWN
- remote_status = UNKNOWN

unless `git_engine::snapshot(...)` succeeds.

But Project Registry may already know:

- GitHub owner;
- repository name;
- expected branch.

This is exactly the PackLab / Pixel Art Generator screenshot case.

`local_repair_plan(...)` knows that local Git is disconnected, but the primary ControlPlaneSnapshot does not project the known remote identity.

### Required fix

ControlPlaneSnapshot must separate:

- remote repository identity/status from Registry;
- local Git checkout status from Git Engine.

Expected native output:

- Remote: `Sekiph82/PackLab` / CONNECTED
- Local Git: NOT_CONNECTED

not “No remote detected”.

---

# UCP-R08 — MAJOR
## The control plane does not yet reconcile task/audit/agent/provider lifecycle actions into STATE/HANDOFF/EVENTS

The prompt required all meaningful actions to immediately update H!veAI's normalized state.

Current `control_plane.rs` contains:

- snapshot;
- adoption;
- Git sync planning/execution;
- local repair planning;

but no production state reconciler that:

- updates STATE after task/workflow changes;
- appends normalized EVENTS after Agent/Audit/Prompt transitions;
- imports SESSION_RESULT.json;
- reconciles an external Codex/Claude result;
- updates HANDOFF according to actor permissions.

`SESSION_RESULT.json` currently exists as documentation only; the production control-plane module does not read it.

Therefore the core “every AI behaves the same and H!veAI updates after every action” contract is not yet implemented.

### Required fix

Implement an actor-aware Project State Reconciler and wire all native lifecycle sources into it.

External provider result remains CLAIM_ONLY until corroborated.

---

# UCP-R09 — MAJOR
## Append-only EVENTS history becomes stale after the bounded limit

`read_events(...)` reads the entire EVENTS.jsonl only while the whole file is under `MAX_FILE_BYTES`, then uses:

`raw.lines().take(MAX_EVENTS)`

This keeps the **first** 128 events, not the newest 128.

Once more than 128 events exist:

- `last_event_at` points to an old event;
- recent lifecycle history is ignored.

Once the append-only file exceeds the overall file byte cap, the entire event source becomes unavailable.

This is incompatible with a long-lived append-only event stream.

### Required fix

Use bounded tail-oriented streaming:

- preserve append-only file;
- read/hash safely;
- parse the newest bounded N records;
- maintain latest-event identity without loading the full file;
- surface malformed/oversized individual records truthfully.

---

# UCP-R10 — MAJOR
## Normal Git commits are not reliably watched because the active branch ref is not in the control-plane watch set

The control-plane watch set includes:

- `.git/HEAD`
- `.git/index`

but not the resolved current branch ref such as:

`.git/refs/heads/main`

A normal commit usually changes the branch ref while `.git/HEAD` remains the same symbolic-ref text.

Therefore a commit can occur without the intended Git metadata watcher firing.

### Required fix

Watch the resolved HEAD ref safely, including packed-ref fallback/reconfiguration when branch changes.

Remote-tracking refs relevant to fetch reconciliation must also invalidate Git projection.

---

# UCP-R11 — MAJOR
## The original legacy dashboard parser remains non-delimited and can still reproduce false MALFORMED states

The M16E authority required old dashboard front matter, if retained, to use an explicit delimited header.

The current legacy `parse_manifest(...)` still starts in header mode and treats colon-delimited lines before the first `##` section as fields.

The final compatibility commit deliberately keeps `hiveai-project/v1` projects on this legacy parser.

Because the seven portfolio PROJECT.json files currently remain on that legacy schema, the old parser is still a live authority path.

### Required fix

Make the legacy compatibility parser deterministic:

- explicit delimited metadata, or
- a fixed allowlist of top-level pointer keys;
- body prose must never increase front-matter field count.

Remove the original `front-matter field limit reached (32)` false-positive class.

---

# UCP-R12 — MINOR
## Persisted control-plane sync metadata is written before Git enrichment

`snapshot(...)` currently calls `persist_metadata(...)` immediately after `resolve_project(...)`, before it obtains and merges the current Git snapshot.

The persisted `control_plane_sync_status` can therefore remain UNKNOWN/stale even when the returned runtime snapshot says IN_SYNC/BEHIND/AHEAD.

### Required fix

Persist metadata only after final Git-enriched health/sync computation.

---

# Test-gap finding

The new control-plane tests exercise the intended new schema, but the release gate did not include the **actual currently merged portfolio files** as fixtures.

That is why the suite could be fully green while all seven non-H!veAI repos still carried incompatible PROJECT/HANDOFF shapes.

The next remediation must include exact representative fixtures copied from current repository structures.

---

# Required closure strategy

Do not patch these one by one.

The next remediation must close UCP-R01 through UCP-R12 in one continuous run and then perform a fresh whole-system adversarial sweep.

Required end-state:

1. all eight GitHub repositories use one accepted control-plane schema;
2. all safely synchronized local repos are reconciled;
3. non-Git local folders preserve known remote identity and offer safe repair;
4. canonical task file may differ by project but is watched and parsed correctly;
5. task/workflow/audit/agent/provider transitions update normalized state immediately;
6. missed events recover within 60 seconds;
7. GitHub-only changes become visible through bounded safe fetch;
8. H!veAI Command Center and Project Cockpit consume the normalized state;
9. no stale legacy/fallback authority remains for an adopted project;
10. user can operate adoption/sync/repair from the native UI.

Only after this entire system passes independent strict re-audit and owner native/visual acceptance should M16 close and M17 activate.
