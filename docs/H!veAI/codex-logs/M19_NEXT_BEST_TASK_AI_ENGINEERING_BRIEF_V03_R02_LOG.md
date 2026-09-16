# M19 V03 R02 Remediation Log

Status: implementation and publication evidence recorded; M19 is not declared PASS/CLOSED by this log.

## Scope and authority

- Workspace: standalone `C:\Users\sekip\Desktop\H!veAI`
- Remote: `https://github.com/Sekiph82/H-veAI`
- Branch: `main`
- Authoritative prompt: [`M19 V03 R02 remediation prompt`](https://github.com/Sekiph82/H-veAI/blob/main/docs/H%21veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_R02_REMEDIATION_PROMPT.md)
- Prior prompt read in full: `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V03_REMEDIATION_PROMPT.md`
- Prior audit read in full: `docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V02_STRICT_REAUDIT.md`
- Synchronization start SHA after safe fast-forward: `908f8437915722610720dfda47a57748101d5404`
- Implementation commit: `1fda1a4f35a40bd9e8b02444d4965a1ecf544367`

No edits were made to `TASKS.md` or `CODEX_ROADMAP.md`. M20 was not started.

## Root causes and closures

| Area | Production root cause | V03 R02 closure |
|---|---|---|
| Exact-eight / M19 local truth | M19 called the mutating all-source M09 parser at decision time. | Added pure exact-root `TASKS.md` parsing; no M09 persistence or materialization on observation. |
| M19 history | Snapshot comparison also wrote the workspace fingerprint, making a read look like a mutation. | Comparison is read-only; fingerprint history has an explicit native command. First-snapshot and incompatible-schema states remain unavailable. |
| Dependency graph | Unlock counts ignored unfinished prerequisites, ambiguity, multiple unmet dependencies, and independent blockers/waits. | Canonical normalized graph now fails closed on missing/ambiguous dependencies and grants unlock credit only for one unmet prerequisite with no independent blocker/wait and an executable actor. |
| Remote M19 evidence | CURRENT cache had no decision-time freshness, registry identity, branch, HEAD, or root-hash proof. | Candidate admission requires all identity/freshness/hash proofs and rejects stale or mismatched remote snapshots. |
| Failure evidence | Remote candidates hard-coded urgency zero and local evidence was an unstructured scalar. | Local and remote candidates use bounded source/result/timestamp/age/freshness evidence; no evidence is not represented as a verified failure. |
| Command Center | Legacy factual brief and M19 panel were visually separate; dedupe used exact category only. | One integrated M19 Engineering Brief renders factual inputs, recommendation state, alternatives, and semantically deduplicated attention. It remains truthful when M19 is unavailable. |
| 9th-project / archived duplicate | Duplicate detection searched all rows while default listing hid archived rows, causing `already registered` with no visible recovery path. | Explicit Add reactivates the same ID for ACTIVE/MISSING/ARCHIVED path matches, preserves settings/repository/history, clears conflicting exclusion, and is restart-safe; multiple normalized collisions fail closed. Removed identities remain re-addable only through explicit registration. |
| Projects footer | A single non-wrapping footer row allowed action overflow and icon containment failures. | Footer actions are bounded and responsive at 3/2/1 columns, wrap intrinsically, retain 31px icon targets and visible focus; all cards show the exact visible label `Local workspace`, with state-specific accessible title/name. |
| GitHub 403/429 storm | Each primary resource and optional enrichment could continue requesting after the first rate-limit response; warnings repeated raw resource errors. | Primary acquisition opens a circuit after the first rate limit, optional acquisition halts, production process-wide backoff is 90s, last-good data is marked STALE, and warnings are grouped once with bounded affected-resource names and sanitized text. |

## F-M19-V01-STRICT-001–012 disposition

The V02/V03 residuals were closed through the native paths above: eligibility/dependency truth, pure local parser, remote freshness and identity, structured failures, integrated brief/dedupe, exact-eight boundaries, M16 purity causality, publication evidence, and schema/hash comparison validation. No finding is self-certified as an owner-native M19 PASS/CLOSED verdict.

## GitHub acquisition evidence matrix

The matrix records the four production repository identities and the native resource contract exercised by the transport seam. `GITHUB_REPOSITORY` through `GITHUB_TAGS` are the eight bounded primary resources; PR/action enrichment is optional and budgeted separately.

| Repository | Registry identity | Branch | Primary resources | Acquisition result / guard |
|---|---|---|---:|---|
| H!veAI | `Sekiph82/H-veAI` | `main` | 8 | exact identity test; 403 circuit test; grouped warning; no optional fan-out after rate limit |
| Bulk-Edit | `Sekiph82/Bulk-Edit` | `main` | 8 | exact identity test; enrichment fan-out bounded outside primary eight |
| ScrubBots | `Sekiph82/Scrubbots` | `main` | 8 | exact identity contract and stale/identity admission guard |
| ScrubBots Level Factory | `Sekiph82/ScrubBots-Level-Factory` | `main` | 8 | exact identity contract and bounded remote acquisition path |

Transport evidence: 403/429 classification is explicit; secure existing transport/auth boundaries are preserved; payloads and warnings are sanitized; raw repeated JSON and credentials are not surfaced. A rate-limit response causes one grouped warning and stops further requests for that acquisition cycle. Fresh last-good cache remains available as STALE and production backoff permits automatic recovery after its bounded deadline.

## Verification gates

- `npm run typecheck` — PASS.
- `npx vitest run --maxWorkers=1 --minWorkers=1` — PASS, 18 files / 159 tests.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib --no-fail-fast` — PASS, 533 tests / 0 failures.
- Mandatory final command: `cargo test --manifest-path src-tauri\\Cargo.toml --lib control_plane::tests::m16l_current_command_center_cockpit_and_control_reads_are_observational -- --exact --nocapture` — PASS, 1/1, 215.65s.
- Focused native tests — PASS: observational M19 snapshot, archived duplicate recovery, rate-limit circuit/grouping, dependency graph, exact-root parser.
- `git diff --check` — PASS.

The pre-remediation exact M16 gate failed after 218.64s; the final exact gate passed after the pure-parser/history separation, establishing the touched-path causality.

## Publication guard

- Trackers unchanged: `TASKS.md`, `CODEX_ROADMAP.md` absent from the implementation diff.
- M20 start: not performed.
- Owner-native M19 PASS/CLOSED claim: not made.
- This file is intended to be immutable after its publication commit.
