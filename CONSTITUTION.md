# H!veAI Repository Constitution

## Purpose
H!veAI is a GitHub-first AI Development Command Center. It must always show:
- every managed project and its health,
- current task,
- last completed task/action,
- next required action,
- required actor (Human, Codex, Claude, GPT Audit, CI, External),
- supporting evidence,
- an actionable control whenever the next step can be executed.

## Non-negotiable principles
1. **GitHub-first:** each tracked repository's branch metadata and root `TASKS.md` are authoritative for project-management state; local folders and SQLite records are execution telemetry only.
2. **Evidence-first:** AI self-report never proves task completion. Completion needs repository/test/audit evidence.
3. **Human override wins:** user corrections override inferred state and are recorded as events.
4. **Separation of duties:** builder and auditor are separate roles. Auditor inspects actual diff, task requirements, tests and architecture rules.
5. **Explicit state machine:** UI and backend use shared enums, never arbitrary status strings.
6. **Actionable status:** PROMPT_REQUIRED → Generate Prompt; READY_FOR_IMPLEMENTATION → Run Builder; AUDIT_REQUIRED → Run Audit; FIX_REQUIRED → Fix with Builder; WAITING_OWNER → Review Decision.
7. **Safe by default:** scanning is read-only. Dependency install, deletion, push, PR creation and destructive shell actions require policy approval. Merge always requires explicit human approval.
8. **No hidden repo mutation:** registering/scanning a project must never edit it.

## Target stack
- Tauri 2
- Rust native core
- React + TypeScript + Vite
- Tailwind + shadcn/ui
- Framer Motion
- SQLite
- xterm.js
- Git
- Codex CLI/SDK adapter
- Claude Code adapter
- OpenAI/Codex audit/planning adapter
- GitHub integration

## Canonical task states
BACKLOG, PLANNING_REQUIRED, PROMPT_REQUIRED, PROMPT_READY,
READY_FOR_IMPLEMENTATION, CODEX_RUNNING, CLAUDE_RUNNING,
IMPLEMENTATION_COMPLETE, AUDIT_REQUIRED, GPT_AUDIT_RUNNING,
AUDIT_PASSED, AUDIT_FAILED, FIX_REQUIRED, RE_AUDIT_REQUIRED,
VERIFY_REQUIRED, VERIFY_RUNNING, WAITING_OWNER, DESIGN_GATE,
WAITING_EXTERNAL, BLOCKED, PAUSED, FAILED, TASK_COMPLETE.

## Canonical agent states
IDLE, QUEUED, STARTING, RUNNING, WAITING_PERMISSION, WAITING_USER,
STOPPING, COMPLETED, FAILED, CRASHED, ORPHANED, RATE_LIMITED.

## Event ledger
Every material transition is immutable and records:
id, project_id, task_id, timestamp, actor_type, actor_id, event_type,
from_state, to_state, summary, evidence_json, session_id, commit_sha, source.

## Repository-native sources
For the eight tracked GitHub projects, H!veAI reads repository metadata and the
root `TASKS.md` only for project-management state. Provider instructions and
other repository documents may support execution but cannot become task truth.
Legacy `.hiveai` control-plane files are historical/secondary telemetry only and
must never override the remote root tracker.

## Definition of done
A milestone is complete only when implementation exists, relevant tests pass,
regressions pass, security is reviewed, recovery/error states exist,
documentation and TASKS.md reflect reality, diff is reviewed, and no secrets,
user data, caches or build junk are committed.

## Codex session rules
Every Codex session must read AGENTS.md, CONSTITUTION.md, ARCHITECTURE.md and
TASKS.md, inspect branch/HEAD/status, verify the current milestone from repo
evidence, run baseline tests before risky work, stay inside current scope,
add tests, verify, update docs/tasks, review diff, never force-push and never
silently modify another managed repository.

## Standalone repository rule
H!veAI is developed and released from the standalone `Sekiph82/H-veAI` repository
on `main`. The historical `AI-Commerce-HQ` repository is migration/source
history only and must not be an active runtime, build, or development root.
Retain reusable infrastructure only after audit. Do not destroy old local data
without an explicit backup and migration policy.

## V1 non-goals
No graph database, knowledge graph, 3D project graph, automatic PR merge,
cloud multi-user tenancy, mobile companion or remote execution farm.
