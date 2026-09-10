# H!veAI Native Product Repair Log

## Publication

- Authoritative prompt: `H!veAI/docs/H!veAI/prompts/PROJECT_COCKPIT_AND_SOURCE_SCOPE_NATIVE_FIX_BEHAVIOR_ONLY_PROMPT.md`
- Synchronization merge: `ab6b861` (`origin/H!veAI` was fetched and merged before implementation).
- Previous accepted repair log: `H!veAI/docs/H!veAI/codex-logs/NATIVE_REGRESSION_REPAIR_LOG.md` at `81972d2`.
- Implementation commit: `fffc8a1`.
- Log commit and final pushed HEAD: the commit that adds this immutable log; the concrete SHA is returned in the final publication proof.
- Scope: cockpit identity/navigation, GitHub source scope, Command Center project views, and live Tasks behavior only. No M17 or M21 work was started.

## Root Causes And Repairs

1. Project Cockpit opened blank because GitHub project IDs contain `/` and were interpolated into `/projects/:id` routes without URL encoding. Every project-card, Command Center, Shell shortcut, and cockpit navigation path now encodes the logical ID. The route resolves the original ID and opens the existing live cockpit.
2. GitHub projects were exposing a broad local filesystem discovery crawl. GitHub v3 projects now use exactly four remote contract sources: `.hiveai/PROJECT.json`, `.hiveai/TASKS.md`, `.hiveai/RULES.md`, and `.hiveai/EVENTS.jsonl`. Local custom source mutation is rejected for these projects. Startup reconciliation removes legacy owned source rows, so the database is repaired instead of merely hidden by the frontend.
3. Command Center Cockpit, Tasks, Workflow, Audit, and Logs were static labels. They are now real bounded tabs over the selected project's current remote snapshot, with explicit project targeting and a compact recent-activity surface.
4. Portfolio/project summary cards were tied to incomplete legacy/local evidence and rendered placeholder state. The selected project and Command Center now read the same GitHub-authoritative snapshot for current task, next action, workflow/actor, health, blockers, progress, and exact counts when declared; unavailable values remain explicitly unavailable instead of becoming fabricated zeros.
5. Tasks was source-inventory-first and exposed legacy `M09 refresh` terminology. It now leads with current task, next action, required actor, active/open, running, completed, total, completion, milestone, execution state, blockers, and last completed work, refreshed from GitHub tracking events without requiring an app restart. Source inventory remains a bounded supporting section.

## Native Evidence

The published stable application was smoke-tested by `scripts/publish-dev-qa.ps1`, including frontend readiness, no development ports, shortcut validation, and no newly-created visible console host.

The real installed SQLite registry, read after publication, proved:

```text
active_projects=8
distinct_github_repositories=8
duplicate_repository_groups=0
legacy_project_sources=0
github_contract_sources=32
```

The source count is exactly four remote contract sources for each of the eight logical repositories. All eight project identities therefore remain unique and cockpit navigation has one working target per repository.

- Stable executable: `H!veAI/dev-bin/H!veAI.exe`
- Stable executable SHA-256: `4EC920F39FA7DA1BFF7620D9160F185268F4BFD512DF979D6CCBE41C1C558C2E`
- Canonical opening video SHA-256, unchanged: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`
- Desktop shortcut target: `H!veAI/dev-bin/H!veAI.exe`

## Verification

- `npm test -- --run`: 125 passed, 0 failed.
- `npm run typecheck`: passed.
- `npm run build`: passed.
- `cargo fmt --all -- --check`: passed.
- `cargo check --lib`: passed.
- `cargo test --lib task_sources::tests -- --nocapture`: 35 passed, 0 failed.
- `cargo test --all-targets -- --nocapture`: 416 passed, 0 failed; main target had 0 tests.
- Governed native publication and readiness smoke: passed.

The existing React test suite emits non-fatal `act(...)` warnings in legacy tests; they do not change the zero-failure result. Existing Rust compiler warnings likewise remain non-fatal and unrelated to this scoped repair.

## Final Push Proof

This file is immutable after its log commit. The final local SHA, `origin/H!veAI` SHA, and equality proof are recorded in the completion response after the normal push.
