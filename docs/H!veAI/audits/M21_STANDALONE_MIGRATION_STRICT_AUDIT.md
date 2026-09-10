# M21 Standalone Migration — Independent Strict Audit

Date: 2026-09-10

## VERDICT

**CHANGES_REQUIRED / CONDITIONAL**

The standalone repository migration itself is real and the production tracking path now contains a direct GitHub root `TASKS.md` observer. However, the repository is not yet internally consistent with the new standalone architecture. Two material migration defects remain before owner-native acceptance and before `Sekiph82/AI-Commerce-HQ` can be considered safe to retire.

## Confirmed working implementation

- `Sekiph82/H-veAI` exists as a standalone repository and contains the migrated application at repository root.
- The production GitHub observer has an explicit eight-repository portfolio that now includes `Sekiph82/H-veAI` and excludes `Sekiph82/AI-Commerce-HQ`.
- The active observation path fetches the tracked branch HEAD remotely, fetches root `TASKS.md`, parses task rows, derives completed/total/progress, extracts current milestone/current sprint/current task/status/next action/required actor, and caches stale remote truth on network failure.
- The observer uses background `curl.exe` rather than Git CLI for primary remote project truth.
- `TASKS.md` is therefore genuinely present in the production read path rather than being only a documentation claim.

## Finding M21-A01 — BLOCKER — Standalone AGENTS.md still commands Codex to work in the deleted parent topology

### Observed repository truth

Current root `AGENTS.md` still states that Git commands must run from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ`

and that H!veAI application work must be placed under:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

It also explicitly says:

`Do not treat H!veAI as a separate Git repository.`

The same file still instructs Codex to fetch and compare `origin/H!veAI` and still contains stable executable paths rooted in the old parent repository.

### Why this is blocking

This directly contradicts the completed standalone migration and can cause the next Codex session to modify the old `AI-Commerce-HQ` checkout instead of `Sekiph82/H-veAI`. It also means the new repository is not self-governing as a standalone project.

### Required target behavior

Root agent/development instructions must describe the actual standalone repository, its `main` branch, standalone local root, current publication path, and current GitHub-only TASKS tracking contract. No active instruction may tell an agent that H!veAI is a subtree of AI-Commerce-HQ or that it must use `origin/H!veAI`.

## Finding M21-A02 — MAJOR — Legacy GitHub-v3 control-plane model remains embedded in production tracking code

### Observed repository truth

`src-tauri/src/github_tracking.rs` still retains substantial legacy concepts and production identifiers from the previous architecture, including:

- `RESOURCE_KIND = "GITHUB_TRACKING_V3"`
- `TASKS_START` / `TASKS_END` v3 markers
- `ProjectV3`, `TasksV3`, `ProgressV3`, `RemoteRaw`
- `parse_remote(...)` handling `PROJECT.json`, `.hiveai/TASKS.md`, `.hiveai/RULES.md`, `.hiveai/EVENTS.jsonl`
- `task_source_policy='GITHUB_REMOTE_V3'`
- metadata `{"tracking":"github-first-v3"}`
- `is_github_v3_project(...)`

The same file also contains the new root `TASKS.md` path and `parse_root_tasks(...)`, so the production module currently mixes both generations of the architecture.

### Why this matters

The migration requirement was to simplify project tracking to GitHub repository metadata plus root `TASKS.md` and remove the prior `.hiveai` control-plane as an active architecture. Even if the legacy parser is no longer on the normal observer path, leaving old authority names, v3 project policies, and `.hiveai` parser machinery in the production module preserves ambiguity and makes future regressions back into the old control-plane model likely.

### Required target behavior

The production tracking model should have one clear identity and one active architecture: direct GitHub repository tracking with canonical root `TASKS.md`. Legacy `.hiveai`/PROJECT/RULES/EVENTS parsing and v3 authority terminology should not remain as active production architecture unless there is a demonstrated compatibility reason, in which case it must be isolated and explicitly non-authoritative.

## Additional observations

The implementation log correctly leaves `AI-Commerce-HQ` in place and marks it `NOT YET SAFE TO RETIRE`. That is the correct state while owner-native acceptance and the two findings above remain open.

The automated build/test claims in the migration log are useful evidence, but native startup/video/UI behavior remains owner acceptance and cannot be independently verified from repository source alone.

## Required remediation before retirement decision

1. Repair all standalone agent/developer instructions so future work targets `Sekiph82/H-veAI` / `main` and no active instruction points back to the AI-Commerce-HQ subtree topology.
2. Remove or cleanly isolate the legacy GitHub-v3 / `.hiveai` project-control-plane machinery and terminology from the production tracking model so root `TASKS.md` is unambiguously the only project-management truth source.
3. Re-run standalone build/tests and publish a fresh native executable.
4. Owner performs native acceptance: startup video immediate, no visible terminal windows, exactly eight projects, project cards/cockpits/tasks load, task counts/progress/current/next values reflect GitHub root `TASKS.md`.
5. Only after the above passes should `AI-Commerce-HQ` be considered ready for retirement/deletion.

## FINAL VERDICT

**CONDITIONAL / CHANGES_REQUIRED**

The migration is substantially implemented, but the standalone repository still contains contradictory active governance and legacy control-plane production architecture. Do not delete `Sekiph82/AI-Commerce-HQ` yet.