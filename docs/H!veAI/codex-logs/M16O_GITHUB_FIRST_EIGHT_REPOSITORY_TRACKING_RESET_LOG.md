# M16O GitHub-First Eight-Repository Tracking Reset Log

## scope

- Authoritative prompt: `docs/H!veAI/prompts/M16O_GITHUB_FIRST_EIGHT_REPOSITORY_TRACKING_RESET_PROMPT.md`.
- `H!veAI/GPT.md` was read and obeyed before execution.
- M16N was not executed; it was treated as superseded.
- M16 remains OPEN at 16/20 (80%). M17 was not activated. M21 was not started.
- The run migrated the eight literal GitHub targets to the v3 contract and made the H!veAI Command Center and Project Cockpit remote-first for current milestone, sprint, task, workflow, actor, next action, blockers, progress, last completion, and refresh provenance.

## implementation

- Added one shared Rust GitHub tracking service used by Command Center and Project Cockpit.
- Remote observation resolves one tracked branch HEAD, reads the four canonical blobs at that commit, validates v3 identity/schema/markers/progress/events, and persists the last successful snapshot in `github_sync_state`.
- Remote state is authoritative. Offline behavior is explicitly stale cached remote state or unavailable; local files, local Git status, and watcher events never replace remote truth.
- Remote snapshot provenance stores the actual Git blob IDs for `PROJECT.json`, `TASKS.md`, `RULES.md`, and `EVENTS.jsonl`.
- Startup portfolio seeding is deterministic and stores GitHub identity without inventing local workspace roots.
- The production readiness gate uses the native Tauri finished-page hook and retains the frontend readiness command as a harmless fallback.
- The exact v3 `PROJECT.json` shape uses top-level `tasksPath`, `rulesPath`, and `eventsPath`; each target has one root `CLAUDE.md` adapter and no lowercase duplicate.
- Legacy H!veAI live-authority sidecars were removed from migrated target repositories where safe. User-owned local files in the H!veAI worktree were preserved and excluded from the scoped commit.

## eight remote branch proofs

Each row passed exact repository/branch identity, v3 PROJECT/TASKS markers, canonical paths, GitHub-first rules, event schema, exact-or-null progress, and one canonical `CLAUDE.md` provider adapter. Blob values are the Git object IDs from the verified remote branch.

| repository | branch | remote HEAD | current milestone / sprint / task | progress | canonical blobs: PROJECT; TASKS; RULES; EVENTS |
|---|---|---|---|---:|---|
| `Sekiph82/AI-Commerce-HQ` | `H!veAI` | `048b6e4b5775f5e6c2ce4ac979314e4641f6872f` | `M16 / M16O / M16O` | 80 | `88e8bf16afed37b3871556752097b5a456100700; e657731c79848fe34bd809dfc3a0c24a05801989; 4552a48c016cc586db269dc3fc3b081026cea464; d20963ff8d8c5ef99af3842ed704d3bb3709cfec` |
| `Sekiph82/Bulk-Edit` | `main` | `9d43d382d3a3339bc40091d90710faea405443c8` | `M13 / M13.03 / M13.03` | null | `fa70c512aa680650b946fb6088e3c45307e43a8c; f66f0490a18ba3286db74a2802bc11726a402bae; ad3271781c5929ba9b35490cea051067917dea4c; 226a84edfd7ea69647ac77dd74c312a766817030` |
| `Sekiph82/fmcg-erp-system` | `main` | `392671e511ea9038669d105bf9635c921cc254f0` | `TASK-005 / TASK-005.1F / TASK-005.1F.3` | null | `7e3b31926ab517cb2d8ccf736f368f44d1558437; 2f97838445cf1f5b3bd33c359541211d6753cea2; ad3271781c5929ba9b35490cea051067917dea4c; c049652e26da56be73925b0e2bd3a716ac22f659` |
| `Sekiph82/FormuLab` | `feature/laboratory-stability` | `a4b0f41e6398a68f32aec1bc14e5eb1eec694443` | `FVL-05 / FVL-05 / FVL-05.012` | 78.57 | `db3653fe95fb901d8b873e05d42245a322adca33; bec5b354d5e7cb3aff7a090a787b98ced82d142c; ad3271781c5929ba9b35490cea051067917dea4c; a322756bfa9e10d825a30e28f77976f16847e08a` |
| `Sekiph82/PackLab` | `main` | `d17fb6db51a55911741707f14eec3f72bed94788` | `M00 / M00-S01 / PL-0001` | null | `44568585ea2643552f2753a93fd80e5b2ddad5a2; 0e47adcfb3acb3e8f7010ec64dbc69ee5c383a3f; ad3271781c5929ba9b35490cea051067917dea4c; afadc35a162603a56aa1807c6ad45f4731fe5a65` |
| `Sekiph82/PackLab-3D` | `main` | `df73d218fa1546ba88f65283736ee77700fcb8fd` | `M00 / M00-S01 / null` | null | `d297a0542e5d619c80b1e5ce12aa9f343b92ca75; 9f4d4363815031c0248b6901a046a366439a7b42; ad3271781c5929ba9b35490cea051067917dea4c; 87e9264fc0f602e37127cb182fb71425af8f8c0e` |
| `Sekiph82/Scrubbots` | `main` | `fe4e2e88754a9b9472536967937e042a7b581462` | `M19 — Scrubbot dispatcher orchestration / M19-C001 V02 — frozen full-surface dispatcher closure / M19-C001-V02` | 38.66 | `23b6d85588b8412e2f2e1d5be67810a644d48119; b720b445043eb3f9f117ea72b6e5bba684a60500; 12b10bd882b86a2723c483b2aad50299bf02f59e; 07567c71584d89dd41ace8154182ae56dee0bf80` |
| `Sekiph82/ScrubBots-Level-Factory` | `main` | `5ad81886669e4f8cfa2247a689439a2186ad1b50` | `PAG-M06 / PAG-M06-C002 / PAG-M06-C002` | 76.47 | `b9bd0d6779d31e31900b783e7f41254e6e911047; 0467b2bda50411bda9ea04eaaa25e0bb098581a9; ad3271781c5929ba9b35490cea051067917dea4c; 24415c9fdc594982fff8aff4531772f0c0e1b97c` |

The two protected migrations were governed normally: Bulk-Edit path-contract update merged as PR #140 after all five required checks passed; ScrubBots-Level-Factory path-contract update merged as PR #3. No protected branch bypass was used.

## verification

- Full Rust regression: `cargo test --all-targets -- --test-threads=1` passed **407/407** at the final implementation commit before the final case-insensitive rules micro-fix; the post-fix targeted GitHub parser suite passed **2/2** and `cargo fmt --all -- --check` passed.
- Frontend regression: **125/125** Vitest tests passed across 15 files after the readiness fix; TypeScript typecheck and Vite production build passed.
- `npm audit --audit-level=high` passed. The dependency report contains only the known two moderate transitive Vitest advisories; no high/critical advisory was reported at this threshold.
- Publisher failure harness passed **9/9**, including invalid candidate, build/provenance, readiness, shortcut/icon, post-swap rollback, locked stable, process cleanup, successful swap, and no-bypass scenarios.
- Governed publisher `scripts/publish-dev-qa.ps1` passed after the native readiness hook fix: release PE build, embedded frontend readiness, zero forbidden listeners on ports 5173/8765, no visible console host, shortcut target/icon checks, candidate smoke, stable swap, and rollback safeguards.
- Stable executable: `H!veAI/dev-bin/H!veAI.exe`; PE prefix `MZ`; SHA-256 `E64C64E86EECF24597D5225091F2B852C92F6395A9191B411EA5E97A65B85157`.
- Automated native Project Cockpit read-model verification passed for all eight target branches using the same remote v3 snapshots and exact current-state fields. Owner native/visual walkthrough remains pending.
- Source branch was fetched and proven equal before log publication: local and `origin/H!veAI` both resolved to `d156c90390985163f10d7eaefff78df041036fe3`.

## scoped commits

- `519c70f` — `feat(H!veAI): make portfolio tracking GitHub-first`
- `61317f9` — `fix(H!veAI): align remote tracker schema and provenance`
- `048b6e4` — `fix(H!veAI): accept canonical GitHub-first rule casing`
- `d156c90` — `fix(H!veAI): emit native readiness marker in production`
- The immutable-log commit SHA is captured in the final equality proof after this file is committed and pushed.

M16O GITHUB-FIRST EIGHT-REPOSITORY TRACKING RESET COMPLETE / PENDING OWNER NATIVE ACCEPTANCE.

M16 remains OPEN.

M17 NOT ACTIVATED.

M21 NOT STARTED.
