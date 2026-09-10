# M16F REV3 Unified Project Control Plane — Independent Whole-System Strict Re-Audit

Date: 2026-09-08  
Repository: `Sekiph82/AI-Commerce-HQ`  
Branch: `H!veAI`  
Rules authority: `H!veAI/GPT.md`  
Builder log: `H!veAI/docs/H!veAI/codex-logs/M16F_REV3_UNIFIED_PROJECT_CONTROL_PLANE_WHOLE_SYSTEM_CLOSURE_REMEDIATION_LOG.md`  
Implementation commit reviewed: `dc6fe12aa258835c1dd57bc7eacbe498fb45a3c8`

## Verdict

**FAIL / CHANGES REQUIRED**

### M16 audit engine
R82-R85 remain CLOSED.

### Unified Project Control Plane
- BLOCKER: 3
- MAJOR: 2
- MINOR: 1

M16 remains OPEN.  
M17 MUST NOT activate.  
M21 MUST NOT start.  
Roadmap completed progress remains `16 / 20 = 80%`.

The builder log reports 385/385 Rust, 386/386 pty-support, 125/125 frontend and governed publication. Those are useful regression claims, but the whole-system source and live repository state still contain release-blocking contract mismatches.

---

# UCP-R13 — BLOCKER
## The final PROJECT.json schema is internally inconsistent: `events` is emitted as a string, while production deserialization requires a string array

Production `ProjectDocument` defines the field as:

`pub event_sources: Vec<String>`

with serde names/aliases:

- `events`
- `eventSources`
- `eventSource`

That means all accepted aliases deserialize into a `Vec<String>`.

But the current normalized AI-Commerce-HQ PROJECT.json and the newly migrated fmcg-erp-system PROJECT.json both contain:

`"events": ".hiveai/EVENTS.jsonl"`

as a scalar string.

The upgrader also migrates old `eventSource` directly into `events` without converting a string into an array.

Therefore the exact files claimed as normalized by this run can fail `serde_json::from_str::<ProjectDocument>` after migration.

This can immediately re-create a `MALFORMED_CONTROL_PLANE` state after a successful-looking upgrade.

### Required remediation

Split semantics explicitly:

- `events`: one string path to the canonical append-only EVENTS.jsonl file; and
- `eventSources`: optional array of additional safe watcher sources.

Do not alias both concepts into one Vec.

Add exact fixtures for:
1. current AI-Commerce-HQ PROJECT.json;
2. current migrated fmcg PROJECT.json;
3. old `eventSource` scalar;
4. explicit `eventSources` array.

The upgrader must output a document that the production parser can immediately re-read successfully.

---

# UCP-R14 — BLOCKER
## The required eight-repository remote migration was not executed

The authoritative REV3 prompt required all eight tracked GitHub repositories to be migrated to the final control-plane schema.

Actual target-branch inspection after the run:

- AI-Commerce-HQ: migrated to `hiveai-project-control-plane/v1`
- fmcg-erp-system: migrated to `hiveai-project-control-plane/v1`
- Bulk-Edit: still `hiveai-project/v1`
- FormuLab: still `hiveai-project/v1`
- PackLab: still `hiveai-project/v1`
- PackLab-3D: still `hiveai-project/v1`
- ScrubBots: still `hiveai-project/v1`
- ScrubBots-Level-Factory: still `hiveai-project/v1`

The builder log itself admits only fmcg was migrated and pushed, while dirty external roots were not rewritten.

Dirty **local** roots are a valid reason not to mutate a working tree, but they are not a reason to leave the authoritative GitHub repository control-plane files on the old schema. The prompt explicitly allowed normal PRs respecting branch protection.

### Required remediation

Migrate all remaining remote repositories independently of local dirty state.

For each repository:

- preserve canonical task ledger;
- preserve project-specific governance;
- preserve HANDOFF prose;
- update only the control-plane contract files;
- use a normal branch/PR where protection requires it;
- verify the final target branch actually contains the final schema;
- record merged commit SHA.

Do not mark this gate complete until all eight target branches are verified.

---

# UCP-R15 — BLOCKER
## The generic adoption path still creates the wrong contract for heterogeneous projects

`adopt(...)` still hard-codes:

- `canonical_task_source: "TASKS.md"`
- event sources including `"TASKS.md"`

and creates `.hiveai/HANDOFF.md` by JSON-serializing `HandoffDocument`.

This contradicts the accepted portfolio model:

- FormuLab canonical source: `docs/FORMULAB_V1_TASK_TRACKER.md`
- PackLab-3D / ScrubBots / Level Factory canonical source: `tasks.md`
- HANDOFF.md is required to remain human-readable Markdown.

For a project missing/repairing control-plane files, clicking Adopt can therefore create a new control plane that is wrong on first write.

### Required remediation

The adoption path must:

1. resolve canonical task source from existing PROJECT metadata, Registry/task-source policy, or deterministic bounded project discovery;
2. refuse adoption if canonical source is ambiguous;
3. create HANDOFF.md as the standard human-readable Markdown template, never JSON disguised as .md;
4. create PROJECT.json using the same schema that production immediately re-parses;
5. preserve existing project-specific governance and never overwrite existing HANDOFF/task truth.

Add direct FormuLab, lowercase tasks.md, and missing-control-plane adoption fixtures.

---

# UCP-R16 — MAJOR
## Upgrade changes schema shape but does not perform the promised state reconciliation

`upgrade_control_plane(...)` currently:

- changes PROJECT schema/repository/pointers;
- changes STATE schema;
- inserts `workflowState = NEEDS_RECONCILIATION` only if absent.

It does not itself reconcile:

- currentTaskId/currentTaskTitle;
- milestone/cycle;
- required actor;
- next action;
- scoped progress;

from canonical task source, existing HANDOFF, workflow records, or native session/audit state.

This is why a schema upgrade can succeed while Cockpit remains mostly Unknown/Unavailable.

The REV3 acceptance requirement was not merely “schema parses”; it was “the project becomes operationally resumable.”

### Required remediation

Implement one deterministic post-upgrade reconciliation pass and call it from:

- upgrade;
- explicit Reconcile;
- startup/safety pass;
- successful Git repair/sync.

The reconciler must follow the shared task-resolution precedence and write normalized state only when evidence is authoritative.

If evidence conflicts, keep NEEDS_RECONCILIATION with explicit reasons instead of inventing a task.

---

# UCP-R17 — MAJOR
## HANDOFF metadata extraction conflates cycle identity with task identity

`read_handoff(...)` currently maps all of:

- `current task`
- `current task id`
- `active cycle`

into `current_task_id`.

A cycle/milestone pointer is not necessarily a task ID.

This can cause the Project Cockpit resolver to treat a cycle heading as the current task, which is exactly the class of failure seen in the Pixel Generator screenshot.

### Required remediation

Parse HANDOFF fields into distinct typed values:

- currentTaskId
- currentTaskTitle
- currentMilestone
- currentCycle
- requiredActor
- nextAction
- resumePointer

Never promote `active cycle` into `current_task_id`.

Add a fixture where cycle and task IDs differ.

---

# UCP-R18 — MINOR
## Append-only event duplicate protection is bounded only to the newest 128 events

`append_event(...)` calls `read_events(...)`, and `read_events` intentionally returns only the newest 128 records.

Therefore an event ID that occurred earlier than the newest 128 can be appended again later and bypass duplicate detection.

The tail reader itself is correctly bounded, but global event-ID uniqueness is not guaranteed.

### Required remediation

Keep the bounded tail reader for display/reconciliation, but use a separate bounded/indexed dedupe mechanism for append protection, for example a persisted recent/event-id index or deterministic transition idempotency key.

Do not scan an unbounded file for every append.

---

# Whole-system closure requirement

The next remediation must close UCP-R13 through UCP-R18 together in one continuous run.

It must also re-run the complete REV3 acceptance matrix across:

- all eight target GitHub repositories;
- all eight registered local project roots;
- Command Center;
- Project Cockpit;
- watcher/safety reconciliation;
- remote/local Git status;
- task/milestone/progress resolution;
- M16 R82-R85 regressions.

The final audit must inspect the actual target-branch control-plane files again, not only the H!veAI runtime tests.

Only after:
1. independent whole-system PASS, and
2. owner native/visual acceptance

may M16 close and M17 activate.
