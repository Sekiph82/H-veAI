# M21 Standalone Remediation Log

Status: IMPLEMENTATION COMPLETE / PUBLISHED
Repository: `Sekiph82/H-veAI`
Branch: `main`
Date: 2026-09-11

## Scope

This log records the bounded remediation of the independent strict-audit findings
M21-A01 and M21-A02. The work stayed in the standalone H-veAI repository. M17,
M18, M19, M20, and M21 follow-on work were not started.

## Findings closed

### M21-A01 - BLOCKER - stale active repository instructions

Root cause: active `AGENTS.md` still described the former
`AI-Commerce-HQ\H!veAI` child topology, the old `H!veAI` branch, and parent-root
release paths.

Remediation: `AGENTS.md`, `CONSTITUTION.md`, the protocol README, the migration
brief, the root `TASKS.md`, and the standalone architecture references now name
`Sekiph82/H-veAI` on `main`, use the standalone root for all commands and
artifacts, and identify the former parent only as migration/source history.
The root `TASKS.md` is the current project-status tracker and records M21-R01 as
PASS/CLOSED.

### M21-A02 - MAJOR - obsolete GitHub-v3 production tracking model

Root cause: `src-tauri/src/github_tracking.rs` still contained v3 resource
names, raw v3 parser/data-model types, and v3 policy metadata even though the
standalone product contract is GitHub branch metadata plus root `TASKS.md`.

Remediation: production tracking now uses the explicit `GITHUB_TASKS_ONLY`
policy and `GITHUB_TASKS_REMOTE` cache resource, with root TASKS parsing and no
legacy v3 parser/model types. Command Center, Task Sources, Watcher, and Project
Cockpit use the renamed TASKS-only identity predicate. The M16Q cockpit fixtures
were updated to seed the same production resource kind. Legacy control-plane,
dashboard, watcher, and source-discovery code remains bounded secondary telemetry
for historical/non-Git compatibility; GitHub-tracked portfolio projects
short-circuit to remote TASKS truth and cannot be contaminated by `.hiveai` or
local tracker discovery.

## Verification matrix

- `cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check`: PASS
- `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`: PASS
- `cargo test --manifest-path src-tauri/Cargo.toml --lib`: PASS, 414 passed, 0 failed
- `cargo test --manifest-path src-tauri/Cargo.toml --lib project_cockpit::tests::m16q_`: PASS, 4 passed, 0 failed
- `npm run typecheck`: PASS
- `npm test -- --run`: PASS, 15 files, 125 tests passed
- `npm run build`: PASS
- `npm run tauri:build`: PASS, Windows x64 executable and NSIS bundle produced
- `scripts/publish-standalone.ps1`: PASS, stable executable copied and native smoke launched responsively
- source scan for obsolete v3 production identifiers: PASS, none found in `src` or `src-tauri` outside excluded historical fixtures
- native process smoke: PASS, `H!veAI.exe` opened with title `H!veAI` and `Responding=True`; no terminal process was opened by the publisher
- canonical opening-video bytes: PRESERVED

Expected compiler warnings remain limited to pre-existing unused/dead-code
warnings; no warning was promoted to a build or test failure.

## GitHub portfolio observation

Direct `git ls-remote` verification after the standalone build observed:

| Repository | Branch | HEAD |
| --- | --- | --- |
| `Sekiph82/H-veAI` | `main` | `a8f904f8ba213b4047fc00b8fa6491296ac5ac08` |
| `Sekiph82/Bulk-Edit` | `main` | `de5d4105223bef2feabf887d92ff901b0fb89df7` |
| `Sekiph82/fmcg-erp-system` | `main` | `43d05b304b0a092c4149346aaf7d020366972f19` |
| `Sekiph82/FormuLab` | `feature/laboratory-stability` | `2c63c634ce4cad6ac377f1fe61732eb7e423f122` |
| `Sekiph82/PackLab` | `main` | `a73ed2b80477cfd1ac03d286f123d5593630183e` |
| `Sekiph82/PackLab-3D` | `main` | `fc6d732ee6f6d557b3b9f975520cfc275d2f88fd` |
| `Sekiph82/Scrubbots` | `main` | `f4b0d9835e8e0a812d98e51f2ae6b40ae705204d` |
| `Sekiph82/ScrubBots-Level-Factory` | `main` | `ed5d2c307c2abab14088f6997365085481661ae8` |

The H-veAI HEAD advanced during this run when the authoritative repository
received the follow-up prompt `a8f904f`; that remote update was fast-forwarded
before the implementation commit.

## Published artifacts

- Stable executable: `dev-bin/H!veAI.exe`
- Stable executable SHA-256: `FE3E7CF39949987219E4B5F64F55DC45D1DD9AC54240CB35CB8ACF8B9B26D5AF`
- Canonical opening video: `src/assets/H!veAI.mp4`
- Opening video SHA-256: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`
- NSIS bundle: `src-tauri/target/release/bundle/nsis/H!veAI_0.1.0_x64-setup.exe`

## Publication proof

- Implementation commit: `1a6d2424dc04753aa9d011c544efc82a78d6606e`
- This immutable log is published in the subsequent log commit and pushed to
  `origin/main`; the final publication proof reports its exact SHA and verifies
  that local `HEAD`, `origin/main`, and `git ls-remote` are equal.
- No active runtime, build, or development instruction depends on the former
  parent repository. The old parent checkout was intentionally preserved and
  was not deleted.

## Retirement verdict

**NOT YET SAFE TO RETIRE `AI-Commerce-HQ`.**

The standalone repository is independently buildable, testable, publishable, and
the active product/tracking model no longer depends on the former parent.
Retirement remains pending owner native/visual acceptance of the published
standalone application and the independent decision to archive/remove the
historical parent. No parent files were modified in this remediation.
