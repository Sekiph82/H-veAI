# Pre-M18 Audit Evidence Authority Hotfix X04 — Independent Audit

## 1. Scope

Repository: `Sekiph82/H-veAI`
Branch: `main`
Work item: Pre-M18 Audit Evidence Authority Hotfix X04
Trigger: owner-native X03 acceptance followed by a fresh Bulk-Edit freeform audit.

## 2. Verdict

FAIL / CHANGES_REQUIRED.

X03 itself is PASS/CLOSED. The fresh audit exposed a separate authority-selection defect in the audit evidence planner.

## 3. Owner-native evidence

The owner demonstrated:

- Settings readiness remained `READY` after Settings -> Audit Center -> Settings in the same process.
- The old persisted `UNAVAILABLE` run stayed immutable and was correctly contextualized as historical.
- A new Bulk-Edit audit completed `COMPLETED / AVAILABLE` with Codex CLI.
- That fresh audit produced two findings based on local `.hiveai` control-plane/projection files and suggested validating the conflicting state against `.hiveai/PROJECT.json`.

## 4. Canonical Bulk-Edit governance

Bulk-Edit current `AGENTS.md` states:

- root `TASKS.md` is the only authoritative current project-status tracker consumed by H!veAI;
- GitHub repository metadata and latest commit are the other project-truth inputs;
- `.hiveai/PROJECT.json`, `.hiveai/TASKS.md`, `.hiveai/RULES.md`, `.hiveai/EVENTS.jsonl`, or equivalent competing current-state ledgers must not be created, revived, or updated.

Bulk-Edit's preserved legacy migration rules additionally state that local STATE, HANDOFF, watcher projections, first-open tasks, and provider self-assessment never override remote truth.

Therefore an audit remediation that effectively asks the owner to restore `.hiveai/PROJECT.json` conflicts with the repository's current canonical governance.

## 5. H!veAI source finding

`src-tauri/src/audit_engine.rs` selects working-tree source snippets from changed paths through `select_source_paths()` and `is_auditable_path()`.

The current auditable-path filter rejects traversal, absolute/path-colon cases and secret-like paths, then accepts ordinary `.rs`, `.ts`, `.tsx`, `.js`, `.jsx`, `.json`, `.toml`, and `.md` paths.

It does not distinguish current implementation evidence from superseded local H!veAI control-plane projections.

As a result, changed or untracked paths such as:

- `.hiveai/STATE.json`
- `.hiveai/HANDOFF.md`
- `.hiveai/EVENT_INDEX.json`

can be supplied to the audit model as verified source snippets during a working-tree audit even when the project's current governance explicitly says those local projections are not authoritative current truth.

## 6. F-X04-001 — Historical/local control-plane projections contaminate implementation audit evidence

Severity: MAJOR

The audit evidence planner can elevate superseded local `.hiveai` projection files into the model evidence set without authority classification. The model can then treat contradictions inside those files as current project defects and generate remediation guidance that conflicts with the repository's canonical tracker/governance.

Observed native consequence: the fresh Bulk-Edit audit generated `Project authority remains inconsistent` from STATE/dashboard disagreement and requested reconciliation against `.hiveai/PROJECT.json`, despite Bulk-Edit current governance forbidding that file from being revived.

## 7. F-X04-002 — Audit remediation can contradict repository governance

Severity: MAJOR

The model input contains a Project Dashboard governance evidence row, but source planning does not prevent non-authoritative local projections from being presented as ordinary verified source snippets. There is no fail-closed authority rule ensuring remediation guidance cannot require creation/revival of a source explicitly prohibited by current project governance.

## 8. Required remediation direction

Fix evidence authority selection, not Bulk-Edit.

The H!veAI audit engine should classify or exclude known local/generated/historical control-plane projections from implementation source evidence whenever current project authority identifies another canonical tracker/source.

At minimum, a root-TASKS/GitHub-first project must not treat local `.hiveai/STATE.json`, `.hiveai/HANDOFF.md`, `.hiveai/EVENT_INDEX.json`, or forbidden legacy ledger files as equal current implementation authority merely because they are changed/untracked `.json`/`.md` files.

The solution must preserve legitimate audits of real implementation files and must not delete, rewrite, or silently mutate project files.

## 9. Preserve immutable history

The Bulk-Edit audit that produced these findings is valid immutable historical evidence of what the engine saw at that moment. Do not rewrite or delete it.

## 10. Security/safety

Do not read credentials or secret files. Do not introduce direct OpenAI HTTP/API transport, API keys, GUI automation, or project-file mutation.

## 11. Regression boundary

Preserve X03 process-scoped readiness truth and historical Audit Center wording.
Preserve the accepted M00-M17 behavior.
Preserve exact eight-project portfolio truth and `Sekiph82/FormuLab@main`.
Do not activate M18 while X04 is open.

## 12. Required independent/native closure

After source remediation passes independent audit, owner-native acceptance must prove on Bulk-Edit that:

- Settings remains READY through route changes;
- a fresh working-tree audit no longer turns superseded local `.hiveai` projections into current authority defects;
- current real changed/untracked implementation files remain auditable;
- historical old audit rows remain immutable.

## 13. Current status

X03 PASS/CLOSED.
X04 OPEN / CHANGES_REQUIRED.
M17 PASS/CLOSED.
M18 NOT ACTIVATED.
