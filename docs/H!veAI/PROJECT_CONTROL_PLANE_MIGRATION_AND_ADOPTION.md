# Project Control Plane v1 Migration and Adoption

Adoption is explicit and idempotent. H!veAI may create missing control-plane files only for an ACTIVE registered project and only with create-if-missing semantics. Existing canonical task, handoff, audit, prompt, log, and session files are preserved.

The migration order is:

1. Validate the registered local path and repository identity.
2. Read PROJECT.json and reject malformed, escaped, or mismatched identity.
3. Initialize missing RULES.md, STATE.json, HANDOFF.md, and EVENTS.jsonl without overwriting existing content.
4. Reconcile the declared canonical task source, Git read model, workflow history, audits, sessions, and prompts into a normalized read model.
5. Mark conflicts as NEEDS_RECONCILIATION and require owner action where automatic repair would change project truth.

The v18 database migration adds only control-plane metadata and indexes to the registered project row. It does not rewrite canonical task content or historical evidence. Re-running adoption and migration is safe and deterministic.

Project-specific governance remains stronger than this shared contract. Task closure, handoff mutation, audit persistence, provider dispatch, and Git writes remain subject to each project RULES.md and the existing H!veAI authorization boundaries.
