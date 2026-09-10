# M16J Portable Project Identity + Canonical Events + Durable Truth Sync Closure Log

Date: 2026-09-09
Repository: Sekiph82/AI-Commerce-HQ
Branch: H!veAI

## Execution and synchronization

- H!veAI/GPT.md was read first and obeyed.
- The authoritative M16J prompt and the complete M16I independent strict re-audit were read.
- git fetch origin H!veAI completed before implementation.
- Starting synchronized HEAD: 5250e9ea1763ccc58d4fe79bc29c078f6fcbcae0.
- Starting local and origin HEADs were equal.
- Implementation commit: f546033135c454d4da29ee4802142ee15db24915.
- M16 remains OPEN. M17 was not activated. M21 was not started.

## UCP-R27 reproduction and closure

Reproduction: the pre-fix materializer wrote the local Registry UUID into STATE.json, and materialization event identity used that UUID. The checked-in AI-Commerce-HQ control-plane contract also used the branch label H!veAI instead of the required portable repository key.

Closure:

- ProjectDocument.project_key is the portable repository identity.
- The local Registry UUID remains a SQLite/in-memory identity and is retained separately as localRegistryId only when policy permits.
- STATE, HANDOFF managed content, EVENTS, and truth revisions use the stable repository key.
- AI-Commerce-HQ is normalized to ai-commerce-hq.
- Cross-machine re-registration is covered by a portable-key fixture and the materializer never derives identity from the Registry UUID when a project document is present.

## UCP-R28 reproduction and closure

Reproduction: EventRecord serialization emitted the older internal shape without the canonical schema, workflowState, commit, auditId, and sessionId core fields.

Closure:

- One canonical writer emits hiveai-event/v1 for every new H!veAI event.
- The common core always includes eventId, projectKey, type, at, actor, taskId, workflowState, summary, commit, auditId, and sessionId.
- Bounded evidenceRefs remains an extension.
- Readers continue to parse legacy internal rows.
- m16j_canonical_event_round_trip_uses_portable_identity proves portable identity, canonical core fields, and legacy reader compatibility.
- Materialization event IDs use the stable project key and truth revision.

## UCP-R29 reproduction and closure

Reproduction: lifecycle hooks used let _ = materialize_project_truth(...), so filesystem synchronization failures were discarded after the authoritative database transition.

Closure:

- Migration v20 adds bounded per-project truth-sync status, revision, trigger, error, attempted/completed timestamps, and retry count.
- Post-commit lifecycle hooks use the durable best-effort materialization path; failures persist as DEGRADED with bounded diagnostics.
- Success marks CURRENT only for the exact revision being materialized.
- Watcher safety reconciliation retries up to eight pending/degraded projects per pass.
- Retry is idempotent, revision-aware, restart-safe, non-destructive, and retry-count bounded.
- Workflow transition, workflow override, task refresh, audit lifecycle, agent finish/failure/recovery, adoption, explicit reconcile, remote observation/fast-forward, and watcher reconciliation are covered by the shared path.
- m16j_truth_sync_failure_is_durable_and_restart_retry_recovers proves durable failure, restart-style retry, and CURRENT recovery.
- Command Center and Cockpit project summaries expose CURRENT, PENDING, or DEGRADED truth-sync state through the shared typed projection.

## UCP-R30 reproduction and closure

Reproduction: materialization rebuilt the complete HANDOFF body and erased project-specific headings and notes on repeat runs.

Closure:

- HANDOFF uses exactly one HIVEAI:BEGIN/END MANAGED CURRENT block.
- Only the managed block is replaced.
- Existing unmanaged bytes and notes are preserved across repeated runs.
- Missing markers cause safe insertion without destroying the existing document.
- Multiple or unmatched markers fail closed.
- PROJECT machine-readable governance and RULES governance are both consulted; owner-governed repositories remain byte-identical.
- m16j_handoff_managed_block_is_lossless_and_fails_closed covers repeat replacement, preserved notes, marker failure, stable identity, and explicit owner governance.

## UCP-R31 reproduction and closure

Reproduction: the resolver could return a computed percentage while returning no exact progress scope.

Closure:

- Explicit percentages require a matching MILESTONE:<id> or CYCLE:<id> scope.
- Stale or mismatched scopes reject the percentage and emit a warning.
- Deterministic computed milestone progress generates the matching scope at the same time.
- A final invariant guard rejects every percent/scope-less result.
- Global historical ratios cannot leak into current scoped progress.

## Actual eight-project contract matrix

Read-only contract verification passed for the actual checked-out projects and branches:

| Project | Portable projectKey | Branch |
| --- | --- | --- |
| AI-Commerce-HQ | ai-commerce-hq | H!veAI |
| Bulk-Edit | bulk-edit | main |
| fmcg-erp-system | fmcg-erp-system | main |
| FormuLab | formulab | feature/laboratory-stability |
| PackLab | packlab | main |
| PackLab-3D | packlab-3d | main |
| ScrubBots | scrubbots | main |
| ScrubBots-Level-Factory | scrubbots-level-factory | main |

PROJECT and STATE parsing, key preservation, branch observation, event schema checks, and the percent/scope invariant were verified without mutating any remote repository.

## Verification

- Focused control-plane suite: PASS, 21 tests.
- Full serialized Rust/all-targets suite: PASS, 398 tests.
- Full frontend Vitest suite: PASS, 125 tests in 15 files.
- TypeScript typecheck: PASS.
- Production Vite build: PASS.
- npm audit --audit-level=high: PASS; no high/critical findings. Existing moderate Vitest advisory remains and requires a breaking upgrade.
- Cargo format check: PASS.
- git diff --check: PASS.
- Publisher failure/rollback harness: PASS, 9/9.
- Governed Tauri no-bundle publication: PASS, candidate native smoke, stable swap, shortcut target, and icon checks.
- Published stable executable SHA-256: B425AA2007D3BB9D30D37A5795476F6EF772275F17A7A990C67013A287AA43CE.
- Canonical opening-video SHA-256 preserved: C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58.
- Whole-M16 adversarial sweep: PASS across identity, schemas, events, resolver/materializer, durable sync, migrations, restart recovery, HANDOFF governance, progress, workflow, audit, agent sessions, watcher, Git/remote observation, Command Center, Cockpit, ACL/native boundaries, degraded paths, tests, and publication.

Native/visual owner acceptance remains pending where required. No installer was created. No visible UI redesign was made. M17 and M21 remain untouched.

M16J WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.

M16 remains OPEN.

M17 NOT ACTIVATED.

M21 NOT STARTED.
