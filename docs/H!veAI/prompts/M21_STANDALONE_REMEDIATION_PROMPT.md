# M21 Standalone Remediation Prompt

Work only in `Sekiph82/H-veAI` on branch `main`.

Read and remediate every finding from:

`docs/H!veAI/audits/M21_STANDALONE_MIGRATION_STRICT_AUDIT.md`

Do not treat the previous M21 completion log as proof. Inspect the current repository, actual production code, active instructions, tests, build scripts, and runtime behavior yourself.

## Problem 1 — Standalone repository instructions still point back to AI-Commerce-HQ

### Current incorrect behavior

The standalone `H-veAI` repository still contains active development instructions that tell Codex to work from the old `AI-Commerce-HQ` local root, write H!veAI code under the old nested `AI-Commerce-HQ/H!veAI` tree, fetch `origin/H!veAI`, and treat H!veAI as not being a separate repository.

That is incompatible with the completed standalone migration and can cause future work to be written to the wrong repository or branch.

### Correct behavior

All active repository instructions must consistently describe the current standalone reality:

- repository: `Sekiph82/H-veAI`
- production branch: `main`
- H!veAI is a standalone repository root
- Git operations for H!veAI development must target the standalone repository
- no active instruction may tell Codex/Claude/developers to write H!veAI code into `AI-Commerce-HQ/H!veAI`
- no active instruction may require `origin/H!veAI` as the H!veAI production branch
- historical documentation may mention the old repository only when clearly marked as historical/migration context

Inspect all active root instructions and development-governance documents, not only one file.

## Problem 2 — Legacy `.hiveai` / GitHub-v3 tracking architecture still exists in active production code

### Current incorrect behavior

The new GitHub root `TASKS.md` tracking path exists, but active production code still contains substantial legacy project-tracking concepts from the previous architecture, including old v3 tracker policy names, `.hiveai` project/rules/events paths, old PROJECT/TASKS v3 parsing structures, reconciliation-era naming, and compatibility logic that can make the product architecture ambiguous.

The owner decision is now explicit: H!veAI project tracking is GitHub-first and `TASKS.md`-only.

### Correct behavior

For the eight tracked repositories, the active production project-tracking model must be unambiguous:

- GitHub repository/branch metadata is remote repository truth
- root `TASKS.md` is the only project-management truth source
- H!veAI derives total tasks, completed tasks, remaining/open tasks, completion percentage, current milestone, current sprint when present, current task, current status, next task/action, and related task summary directly from root `TASKS.md`
- `.hiveai/PROJECT.json`, `.hiveai/STATE.json`, `.hiveai/TASKS.md`, `.hiveai/RULES.md`, `.hiveai/EVENTS.jsonl`, handoff/control-plane manifests, and legacy reconciliation documents must not be active project-state inputs
- prompt files, logs, audits, historical docs, and arbitrary Markdown files must never become project task sources
- the normal GitHub task-source inventory for each project is one canonical root `TASKS.md`
- local workspace files must not override GitHub project truth

Remove obsolete active production logic where it is no longer needed. If a legacy symbol or compatibility path must remain temporarily for a genuinely unrelated feature, clearly isolate it so it cannot participate in current GitHub project truth or source discovery.

Do not preserve dead architecture merely because old tests reference it. Update tests to the current product contract.

## Problem 3 — Naming and policy terminology must reflect the simplified architecture

### Current incorrect behavior

The production code still uses names such as `GITHUB_REMOTE_V3`, `GITHUB_TRACKING_V3`, and other terminology that belongs to the superseded `.hiveai` v3 control-plane design.

Even where current runtime behavior uses root `TASKS.md`, this naming makes the implementation misleading and increases the risk that future code restores old behavior.

### Correct behavior

Active production names, policies, comments, data-flow labels, and tests should clearly describe the current model: direct GitHub repository tracking with root `TASKS.md` as the sole task tracker.

Do not rename historical logs or immutable historical evidence. Clean up only active implementation/governance surfaces where old terminology incorrectly describes current behavior.

## Problem 4 — Verify the standalone application is truly independent before retirement

### Correct behavior

After remediation, the standalone repository must remain independently buildable and runnable without depending on the old `AI-Commerce-HQ` repository layout.

Verify at minimum:

- frontend typecheck
- frontend tests
- frontend production build
- Rust checks/tests
- Tauri/native build
- published native executable path used by the standalone repo
- startup video preserved and still starts promptly
- no visible Git/cmd/PowerShell/Terminal windows caused by project refresh
- all eight configured GitHub projects load from their tracked branches
- each project exposes exactly one canonical task source: root `TASKS.md`
- project task totals/progress/current task/next task are derived from that file
- local project folders are not project-truth sources
- standalone instructions no longer redirect work into AI-Commerce-HQ

## Repository-retirement gate

Do not delete `Sekiph82/AI-Commerce-HQ` as part of this remediation.

At the end, give one explicit verdict:

- `AI-Commerce-HQ READY FOR OWNER RETIREMENT`

or

- `AI-Commerce-HQ NOT YET SAFE TO RETIRE`

Use `READY FOR OWNER RETIREMENT` only if the standalone repository is independently verified and no active H!veAI runtime/build/development instruction depends on the old parent repository.

## Logging

Create a new immutable remediation log at:

`docs/H!veAI/codex-logs/M21_STANDALONE_REMEDIATION_LOG.md`

Include:

- exact findings remediated
- root causes
- files changed
- legacy tracking code/instructions removed or isolated
- test/build/native verification results
- standalone repository HEAD
- published executable SHA-256 if rebuilt
- startup video SHA-256
- eight-repository verification summary
- final AI-Commerce-HQ retirement verdict

Commit and push the implementation and immutable log to `Sekiph82/H-veAI` `main`.

Return the exact implementation commit SHA, log commit SHA, final `origin/main` HEAD, and log GitHub URL.