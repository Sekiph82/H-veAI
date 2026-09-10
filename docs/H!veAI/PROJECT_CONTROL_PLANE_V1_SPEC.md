# Unified Project Control Plane v1

The project control plane is a normalized, provider-neutral read model rooted at the registered project path. It is additive to the existing canonical task ledger; it never replaces TASKS.md, CODEX_ROADMAP.md, or stricter project-specific governance.

## Files

- .hiveai/PROJECT.json identifies the project, repository, branch, canonical task source, registry identity, and declared event sources.
- .hiveai/RULES.md records the actor model: OWNER, HIVEAI_SYSTEM, CODEX, CLAUDE, CHATGPT, and INDEPENDENT_AUDITOR.
- .hiveai/STATE.json records the bounded workflow state and resume facts.
- .hiveai/HANDOFF.md records the durable resume pointer.
- .hiveai/EVENTS.jsonl is append-only lifecycle evidence.

The authority order is registry identity, PROJECT.json, canonical task source, STATE.json, HANDOFF.md, Git evidence, native persisted state, then events. Conflicts are surfaced as NEEDS_RECONCILIATION; files are not silently rewritten.

Adoption is explicit and create-if-missing only. It never overwrites existing project files or initializes Git. Remote reconciliation permits only an owner-enabled clean fast-forward. Dirty, ahead, diverged, missing, detached, or non-Git states require explicit owner attention.
