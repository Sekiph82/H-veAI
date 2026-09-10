# Migration From AI-Commerce-HQ

The former `AI-Commerce-HQ/H!veAI` application tree was promoted into the
root of `Sekiph82/H-veAI`. The standalone repository keeps its own `main`
branch, package paths, Tauri paths, tests, assets, and release configuration.

The historical parent repository is preserved and is not deleted by this run.
Its `H!veAI` branch remains an archival/source-history reference until the
owner independently retires it.

## Tracking Simplification

The old `.hiveai` control-plane files are no longer current project truth. All
eight tracked repositories were normalized to one root `TASKS.md` contract and
their prior tracker files were archived under
`docs/migration/legacy-task-trackers/` where useful. H!veAI reads only GitHub
branch metadata and root `TASKS.md` for current project state. Local registry,
Git, workflow, audit, and agent data is secondary telemetry.

The accepted opening-video asset was copied byte-for-byte from the legacy app.
The final migration builder log is
`docs/H!veAI/codex-logs/M21_STANDALONE_MIGRATION_LOG.md`.
