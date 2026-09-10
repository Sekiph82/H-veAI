# M21 Standalone Migration Log

Status: implementation and automated verification complete; owner native/visual acceptance remains an external acceptance step.

Authoritative prompt: [M21 standalone migration and TASKS-only GitHub tracking architecture](https://github.com/Sekiph82/H-veAI/blob/main/M21_STANDALONE_MIGRATION_AND_TASKS_ONLY_GITHUB_TRACKING_ARCHITECTURE_PROMPT.md)

## Standalone migration

- Legacy source: `Sekiph82/AI-Commerce-HQ`, branch `H!veAI`, baseline HEAD `5159a9595053cc8d2ffeb9fe871b15527418dccd`.
- New target: `Sekiph82/H-veAI`, branch `main`.
- Initial standalone migration commit: `a606d46b38dd1b59143db73df391cdf7fece7dae`.
- Promotion method: the legacy `H!veAI` subtree was copied to the standalone repository root, then root-relative build/runtime paths were repaired. No nested `H!veAI/` wrapper was retained.
- Intentionally excluded from the promoted tree: generated dependencies, build targets, databases, local caches, and active legacy control-plane runtime files.
- Required historical prompt/audit/log material was retained under `docs/H!veAI/`; it is not a runtime task source.
- Final standalone HEAD: `e8eb44f6d0dbc84c393c1ecb17c8ced83c67d8e0`.

## Simplified tracking architecture

The production GitHub path now reads repository metadata, the tracked branch HEAD, GitHub Atom commit metadata, and the raw root `TASKS.md` over hidden bounded HTTP requests. It does not spawn Git CLI observation/fetch work for primary project truth, and network refresh is outside the startup path. The parser consumes one canonical root tracker, validates the explicit current task, counts only checkbox task rows, derives completed/total/progress, and preserves cached remote truth when refresh is unavailable.

The old `.hiveai` control-plane files are no longer authoritative project-tracking inputs. They were removed from the active eight-repository paths and preserved only as migration history where useful. Prompt, log, audit, and arbitrary Markdown files are not task sources. The app's GitHub source inventory is now one `TASKS.md` source per repository with `GITHUB_TASKS_ONLY` authority.

## Eight-repository matrix

All eight target repositories were normalized and pushed to their actual tracked branches. `TASKS.md` is the canonical path in every row. Counts below are the explicit checkbox rows in the normalized file; repositories with no checkbox rows report `0/0` rather than an invented percentage.

| Repository | Branch | Current milestone/task | Next action | Completed/total | Latest remote HEAD and message | Normalization commit |
|---|---|---|---|---:|---|---|
| `Sekiph82/H-veAI` | `main` | `M21` / `M21-01` Standalone root migration and GitHub TASKS-only architecture | Owner native/visual acceptance and independent retirement decision | 899/1044 | `e8eb44f6d0dbc84c393c1ecb17c8ced83c67d8e0` — simplify tracking to GitHub root TASKS | `e8eb44f6d0dbc84c393c1ecb17c8ced83c67d8e0` |
| `Sekiph82/Bulk-Edit` | `main` | `M13` / `M13.03` Real Etsy video upload architecture | Owner live Etsy video upload acceptance | 147/214 | `de5d4105223bef2feabf887d92ff901b0fb89df7` — adopt root TASKS tracking contract (#141) | `de5d4105223bef2feabf887d92ff901b0fb89df7` |
| `Sekiph82/fmcg-erp-system` | `main` | `TASK-005` / `TASK-005.1F.3` eTIMS card in invoice detail page | Implement or explicitly defer the task under the canonical ledger | 0/8 | `43d05b304b0a092c4149346aaf7d020366972f19` — adopt root TASKS tracking contract | `43d05b304b0a092c4149346aaf7d020366972f19` |
| `Sekiph82/FormuLab` | `feature/laboratory-stability` | `FVL-05` / `FVL-05.012` Train/validation/test partition rules | Begin after approval of laboratory-stability scope | 0/0 | `2c63c634ce4cad6ac377f1fe61732eb7e423f122` — adopt root TASKS tracking contract | `2c63c634ce4cad6ac377f1fe61732eb7e423f122` |
| `Sekiph82/PackLab` | `main` | `M00` / `PL-0001` Canonical repository structure specification | Implement PL-0001, then request audit | 1/436 | `a73ed2b80477cfd1ac03d286f123d5593630183e` — adopt root TASKS tracking contract | `a73ed2b80477cfd1ac03d286f123d5593630183e` |
| `Sekiph82/PackLab-3D` | `main` | `M00` / no exact task declared | Define the first approved PackLab 3D task | 0/0 | `73e3ac5118cf515202e42faefd14ea6a8f9642b8` — adopt root TASKS tracking contract | `73e3ac5118cf515202e42faefd14ea6a8f9642b8` |
| `Sekiph82/Scrubbots` | `main` | `M19` / `M19-C001-V05` Callback-order and direct-observability gaps | Independently audit M19-C001 V05 | 236/778 | `aaef3df03033223fe39153ebdb466152e95e454e` — adopt root TASKS tracking contract | `aaef3df03033223fe39153ebdb466152e95e454e` |
| `Sekiph82/ScrubBots-Level-Factory` | `main` | `PAG-M08` / `PAG-M08-C001` Output / Export Contract | Execute the authoritative task prompt and publish evidence | 259/392 | `29c5cb0f671388c392c30c36c63920298ce30067` — adopt root TASKS tracking contract | `29c5cb0f671388c392c30c36c63920298ce30067` |

`Sekiph82/AI-Commerce-HQ` is not an active portfolio project. The legacy parent repository remains intact and was not removed.

## Native publication

- `npm run typecheck`: passed.
- `npm test -- --run`: passed, 125 tests across 15 files.
- `npm run build`: passed.
- `cargo check --manifest-path src-tauri/Cargo.toml --all-targets`: passed.
- Full Rust regression: 417 passed, 0 failed, 0 ignored in 464.42s.
- `npm run tauri:build`: passed; native release executable and NSIS bundle produced.
- Publication helper: `scripts/publish-standalone.ps1` copied the release executable and icon, then ran a bounded native start/exit smoke.
- Published executable: `dev-bin/H!veAI.exe`.
- Published executable SHA-256: `4AD5E47ED6FE5E8996D2F244FC788124EB9F185CD3F9320CB321456784DB070E`.
- Opening-video SHA-256: `C57E30A84879D967A338AD54A2209CF471938BC869763E3C43766E5135982F58`, matching the legacy source bytes.
- Native smoke proved the published executable stayed alive after launch and exited within the bounded cleanup window. Startup video, visual project opening, and native click acceptance remain for the owner.

## Verification matrix

- Eight remote branches resolved and eight root `TASKS.md` files were fetched/normalized.
- Root parser tests cover current-task validation, status, next action, blockers, counts, and exact progress.
- Remote GitHub source inventory contains one canonical `TASKS.md` source per project.
- Duplicate/local-plus-remote identity prevention remains covered by the existing project identity and cockpit regression suites.
- Local `.hiveai` state is not used as current GitHub project truth.
- Slow/offline remote refresh is bounded and cached; startup does not wait on GitHub.
- Background HTTP uses hidden bounded `curl.exe`; no visible Git, cmd, PowerShell, or terminal window is opened by the production refresh path.
- Existing frontend and Rust regression suites were preserved and rerun after the standalone changes.
- Bulk-Edit normalization passed its required CodeQL, backend, frontend, and Docker checks and was merged as PR #141.
- Final standalone local `HEAD`, `origin/main`, and published remote `main` are required to equal the final commit after this log is committed and pushed.

## Retirement readiness

`AI-Commerce-HQ NOT YET SAFE TO RETIRE`

The standalone migration, build, publication, and eight-repository tracking conversion are complete, but the legacy parent is intentionally preserved. Owner native/visual acceptance and an explicit archival/retirement decision are still required before deleting or retiring `AI-Commerce-HQ`.
