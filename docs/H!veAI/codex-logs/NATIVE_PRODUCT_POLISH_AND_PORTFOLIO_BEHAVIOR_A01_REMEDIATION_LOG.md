# Native Product Polish and Portfolio Behavior A01 Remediation Log

Date: 2026-09-11
Repository: `Sekiph82/H-veAI`
Branch: `main`

## Scope

This follow-up closes the remaining launcher-publication findings from the strict audit without redesigning the completed product-polish work. The historical product-polish log remains immutable.

## Findings

- **NPPA-A01:** The owner Desktop shortcut still launched the historical `AI-Commerce-HQ\\H!veAI` executable instead of the standalone canonical executable.
- **NPPA-A02:** The prior immutable log did not include its final publication commit and final `origin/main` SHA. This follow-up records both values without mutating that prior log.
- **NPPA-A03:** The prior bounded run recorded one long-running observational test as incomplete. It remains explicitly unverified and is not relabeled as a pass.

## Root Cause and Remediation

The owner shortcut had stale `TargetPath`, `WorkingDirectory`, and `IconLocation` values inherited from the historical parent checkout. The shortcut was rewritten through the Windows Shell Link API to use the standalone canonical publication:

- Shortcut: `C:\Users\sekip\Desktop\H!veAI.lnk`
- Target: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI\dev-bin\H!veAI.exe`
- Working directory: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI\dev-bin`
- Icon source: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI\dev-bin\H!veAI.exe,0`
- Arguments: empty

The historical parent executable is no longer an owner-launch target. No source-level GitHub tracking or root `TASKS.md` behavior was changed, and no legacy runtime dependency was restored.

## Native Verification

- Shortcut target exists and resolves to the standalone EXE: PASS.
- Shortcut working directory and icon source resolve inside the standalone `H-veAI` tree: PASS.
- Launching the actual Desktop shortcut produced `H!veAI.exe` from the standalone path: PASS.
- Native process was responsive after startup: `Responding=True`, window title `H!veAI`.
- The launched app had only `msedgewebview2.exe` as a child process: PASS.
- No Git, cmd, PowerShell, or Terminal child process belonged to the launched H!veAI process: PASS. Ambient Codex-host command shells observed during this verification were not app children.
- Canonical opening-video behavior and the standalone executable bytes were preserved.

## Preserved Product Truth

The GitHub plus root `TASKS.md` tracking model remains unchanged. The standalone published EXE remains:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI\dev-bin\H!veAI.exe`

Its SHA-256 is:

`200D316F5A2D87CEFEE3FBCF9C9BCBA12CF43426138F5D1FFAAA6EEB7BFC45BE`

The accepted source implementation remains at `62776206395832ba7eaceb28cca3343f12a92797`. The audit synchronization baseline was `c7cdc4fe9acd5a2dd6c0de27af080fd67ecf3851`.

## Publication

This log is immutable after publication. The exact log publication commit, log blob SHA, and final remote head are returned in the publication receipt for this immutable file after commit and push, following the repository's established immutable-log convention.
