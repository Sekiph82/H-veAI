# Safe Git Synchronization Policy

Remote reconciliation is a bounded, owner-controlled operation over a registered ACTIVE Git project.

- Fetch is bounded and uses the sanitized configured upstream remote.
- Automatic update is allowed only for a clean branch with an upstream, ahead count zero, behind count greater than zero, and no divergence.
- The update is `git merge --ff-only @{upstream}` after a fresh snapshot.
- Dirty, conflicted, detached, unborn, ahead, diverged, or root-mismatched repositories return a structured SYNC_ATTENTION result.
- The engine never resets, rebases, auto-stashes, discards untracked files, or overwrites canonical task files.
- A non-Git local folder may retain a known remote identity, but repair or connect is an explicit owner workflow with backup or clone preview.

Local Git state and known remote repository identity are separate fields. A known remote does not imply a connected local checkout, and a healthy local checkout does not imply that a remote fetch was performed.
