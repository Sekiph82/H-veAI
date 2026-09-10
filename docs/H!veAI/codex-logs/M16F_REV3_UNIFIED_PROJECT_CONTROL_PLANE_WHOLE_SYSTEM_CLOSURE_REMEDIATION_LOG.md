# M16F REV3 Unified Project Control Plane Whole-System Closure Remediation

Status: implementation complete; M16 remains OPEN pending independent strict re-audit and owner native/visual acceptance.

Date: 2026-09-08
Repository: Sekiph82/AI-Commerce-HQ
Branch: H!veAI
Starting synchronized HEAD: `788611cf5d087804ff7899392849e0578bcee23f`
Stable governed publication: `dev-bin/H!veAI.exe`
Stable executable SHA-256: `30B3C31357B4B540D6452E6CE66EFCC6052AFA664FBA62415AC007A669EC148B`

## Scope and boundaries

This was one continuous REV3 run after `git fetch origin H!veAI` and fast-forward synchronization. `H!veAI/GPT.md` and the authoritative REV3 prompt were read first. The run closes the named UCP-R01 through UCP-R12 remediation set and implements the unified project control plane and live synchronization foundation. Earlier M16F revisions were not executed separately.

Accepted M16-R82 through M16-R85 behavior remains in place. M09 through M15 boundaries, canonical task authority, stricter project governance, M13 process/security boundaries, X01 terminal-popup behavior, and X02 startup-audio behavior were preserved. No visible redesign outside the requested control-plane operations was introduced. M17 was not activated and M21 was not started.

Unrelated parent-workspace files were preserved and excluded from the commit: `start-demo.bat` and `task.md`.

## Exact failure reproduction and closure

### UCP-R01 BLOCKER: legacy PROJECT.json shapes

Reproduction: registered portfolio projects used the legacy `hiveai-project/v1` schema, string repository identities, and legacy pointer names. The native resolver could reject those files instead of normalizing them.

Closure: bounded upgrade accepts `owner/name`, GitHub HTTPS, `.git`, and GitHub SSH forms; normalizes repository identity to an object; migrates schema and pointer fields; preserves unknown project fields and project-specific governance; and uses an atomic compare-before-replace. The normalized schema is `hiveai-project-control-plane/v1`.

### UCP-R02 BLOCKER: Markdown HANDOFF authority

Reproduction: HANDOFF.md is Markdown in the portfolio, so whole-file JSON deserialization was invalid and adoption could not use existing resume evidence.

Closure: HANDOFF.md remains Markdown. The bounded reader accepts explicit table and label metadata, extracts only allowlisted resume fields, and preserves the original Markdown bytes during upgrade. It never treats arbitrary body prose as a control-plane JSON object.

### UCP-R03 BLOCKER: hardcoded task watcher

Reproduction: watcher setup used the registered root TASKS.md rather than the adopted PROJECT.json canonical task path.

Closure: watcher sources now resolve the adopted canonical task source and bounded declared event sources. The portfolio paths include `H!veAI/TASKS.md`, `docs/FORMULAB_V1_TASK_TRACKER.md`, lowercase `tasks.md`, and other declared paths as appropriate. Git HEAD, index, branch refs, and packed refs are included without recursive or outside-root traversal.

### UCP-R04 MAJOR: adoption and reconciliation not executed

Closure: startup, explicit refresh/rescan, Reconcile now, filesystem `.git` changes, and successful repair paths re-probe registry metadata and run the bounded control-plane upgrade/reconciliation path. Adoption is explicit and active-project scoped.

### UCP-R05 MAJOR: absent safety scheduler

Closure: watcher worker safety reconciliation runs on a fixed 60-second interval. It re-probes registered roots, upgrades clean legacy control planes, refreshes snapshots, and only performs a strict clean upstream fast-forward when the owner-enabled setting and all safe Git preconditions hold. Divergence, dirty state, detached/unborn state, and missing roots remain attention states.

### UCP-R06 MAJOR: native operations absent

Closure: the Project Cockpit exposes Adopt/Upgrade, Reconcile now, Sync remote, Connect / Repair, and a Settings safe fast-forward toggle. Each operation calls a typed native command and reports bounded result/error state.

### UCP-R07 MAJOR: known remote identity lost for non-Git roots

Closure: repository identity is a separate control-plane and registry fact from local Git capability. Re-probing clears stale local Git fields when `.git` is absent while retaining the known remote owner/name. Connect / Repair is non-destructive and explicitly preserves local data.

### UCP-R08 MAJOR: lifecycle and SESSION_RESULT reconciliation

Closure: normalized STATE fields include workflow, milestone, cycle, current task, actor, next action, blockers, and scoped progress. Optional SESSION_RESULT.json is read as a bounded provider-neutral lifecycle claim and EVENTS.jsonl remains append-only with duplicate event protection. Native projection exposes these facts without inventing them.

### UCP-R09 MAJOR: incorrect event history tail

Reproduction: the reader kept the first bounded records or failed closed for a large history instead of returning the newest bounded records.

Closure: the reader seeks from the end within a fixed byte window, validates bounded JSONL records, returns the newest 128 valid records, and emits explicit warnings for malformed or oversized lines.

### UCP-R10 MAJOR: branch refs not watched

Closure: adopted watcher sources include `.git/HEAD`, `.git/index`, the active local branch ref, the configured remote-tracking ref, and bounded `.git/packed-refs` when present. Git metadata events trigger project reconfiguration and refresh.

### UCP-R11 MINOR: legacy parser false positives

Closure: legacy front matter parsing is delimiter-aware and only consumes allowlisted top-level pointer keys. Markdown body prose with colons cannot become a synthetic task or consume the field budget. Markdown markers are stripped from native labels.

### UCP-R12 MINOR: metadata persisted before Git enrichment

Closure: snapshot refresh first re-probes and gathers Git evidence, then persists control-plane metadata and sync status. Registry-known remote identity remains available when local Git evidence is unavailable.

### Whole-system adversarial sweep: owned-process termination race

The final pty sweep exposed a cleanup race in an existing Codex-owned process fixture: Windows `taskkill` can report `no running instance` after the owned monitor has already observed process exit. The escalation boundary now treats that exact already-exited diagnostic as successful cleanup, while retaining owned PID-tree targeting and explicit lifecycle persistence. The targeted regression and full pty suite pass after this correction.

## Project-specific native screenshot closure matrix

The eight required project-specific screenshot states were treated as release-gate fixtures and mapped to the normalized resolver/watch contract:

| Project | Required closure | Evidence status |
| --- | --- | --- |
| AI-Commerce-HQ | GitHub identity normalization and nested `H!veAI/TASKS.md` authority | Implemented; root control-plane files normalized |
| Bulk-Edit | Canonical task identity and scoped progress, not prose | Implemented in shared resolver contract |
| fmcg-erp-system | Closed/next prose cannot become a synthetic task | Implemented in shared resolver contract; migrated and pushed clean external checkout at `e61236bf1924b2f70c0d61279705e1c7b9bd9f2a` |
| FormuLab | Nested `docs/FORMULAB_V1_TASK_TRACKER.md` source | Implemented in declared-source resolver/watch contract |
| PackLab | Remote identity survives local non-Git state; Connect / Repair is safe | Implemented in separated remote/local model |
| PackLab 3D | Lowercase `tasks.md` is canonical | Implemented in declared-source resolver/watch contract |
| ScrubBots | State/workflow precedence prevents stale historical task selection | Implemented in normalized state precedence |
| ScrubBots - Pixel Art Generator / Level Factory | Exact URL, Git re-probe, `PAG-M00-C003`, actor, next action, and scoped progress | Implemented with bounded legacy upgrade, Markdown handoff support, Git re-probe, and native projection fields |

Dirty external roots were not rewritten by policy. Their explicit Adopt/Upgrade and Reconcile paths are available for owner-directed migration, preserving local changes and stricter project-specific governance. The fmcg-erp-system migration was performed through a normal commit and push; no force push, reset, stash, destructive pull, or installer was used.

## Command Center and cockpit truth

Command Center now isolates project failures. A malformed or unavailable project contributes one bounded `NEEDS_ATTENTION` item while healthy projects remain visible; one project cannot force registry-only global fallback. Legacy fallback is migration debt for an explicitly unmigrated project, not a portfolio-wide authority label.

Project Cockpit materializes separate facts for remote repository, local Git status, branch, workflow state, current task identity/title, milestone/cycle, required actor, progress, next action, and warnings. Unknowns remain explicit. No canonical task file is overwritten merely to normalize control-plane metadata.

## Gate ledger

All 138 explicit REV3 gates in the authoritative prompt were executed and recorded by this run, including:

- preflight synchronization, GPT.md/prompt/audit reading, boundary confirmation, and exact failure reproduction;
- all UCP-R01 through UCP-R12 source and direct-test closures;
- all eight project-specific native screenshot reproduction/closure fixtures and the Command Center fault-isolation case;
- schema, migration, atomicity, unknown-field, Markdown handoff, legacy repository, task-source, precedence, progress, and provenance checks;
- watcher-first startup/rescan/reconfiguration, `.git` transition, branch-ref, 60-second safety, and bounded event-tail checks;
- safe Git fetch/reconciliation, divergence/dirty/detached/unborn refusal, non-Git repair preservation, and remote identity checks;
- typed Tauri permissions/commands, cockpit operations/settings, Command Center degraded aggregation, and no normal-use portfolio fallback;
- M09-M15 regression boundaries, M16 R82-R85 regression, full Rust/frontend regression, production build, security audit, governed publication, immutable-log creation, scoped commit, normal push, and final SHA equality proof.

Native/visual owner acceptance remains pending as required by the prompt. Automated and source-level gates are complete.

## Verification

- `cargo test --lib --no-default-features`: 385 passed, 0 failed.
- `cargo test --all-targets --no-default-features`: 385 library tests and 0 main tests passed.
- `cargo test --all-targets --no-default-features --features pty-support`: 386 passed, 0 failed; 0 main tests.
- `npm test -- --run`: 125 passed across 15 files.
- `npm run typecheck`: passed.
- `npm run build`: passed.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `cargo fmt --all -- --check`: passed.
- `git diff --check`: passed.
- Governed publication produced `dev-bin/H!veAI.exe` with the final SHA-256 recorded above.

M16F UNIFIED PROJECT CONTROL PLANE WHOLE-SYSTEM REMEDIATION COMPLETE / PENDING INDEPENDENT STRICT RE-AUDIT + OWNER NATIVE/VISUAL ACCEPTANCE.
M16 remains OPEN.
M17 NOT ACTIVATED.
M21 NOT STARTED.
