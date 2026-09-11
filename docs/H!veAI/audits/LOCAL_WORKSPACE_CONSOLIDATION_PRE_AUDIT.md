# H!veAI Local Workspace Consolidation — Pre-Audit

## VERDICT

**CHANGES_REQUIRED / OWNER-DIRECTED TOPOLOGY CHANGE**

The latest launcher remediation log is internally consistent with the repository governance that existed at that moment, but the owner has now explicitly changed the desired local workspace topology.

The GitHub repository identity remains:

`https://github.com/Sekiph82/H-veAI`

The owner now requires the primary local H!veAI workspace on the current laptop to be:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

This local path change must not change the GitHub repository identity or project-tracking architecture.

## Audit of the latest launcher remediation log

Latest remediation log commit:

`8295d5244ee0e65d5e25cc9c739bb5f13e9399f2`

The log reports that the Desktop shortcut was moved from the historical parent-child executable to:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI\dev-bin\H!veAI.exe`

and that the launched process was responsive with no visible shell children.

The log itself is a documentation-only GitHub commit. The claimed Windows shortcut mutation is machine-local state and cannot be independently proven from GitHub source alone. It is therefore accepted only as builder/native evidence, not as portable repository proof.

The repository `AGENTS.md` also currently declares the standalone local root as:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI`

That is now superseded by the owner's new local-workspace decision.

## Required local topology

The desired canonical local workspace is now:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

That directory must contain the complete standalone H!veAI working tree and its own `.git` metadata for `Sekiph82/H-veAI` on `main`.

The fact that this directory is temporarily nested inside the local `AI-Commerce-HQ` checkout must not change H!veAI into an AI-Commerce-HQ subtree, branch, or submodule. It remains a standalone Git repository whose remote is `Sekiph82/H-veAI`.

The owner intends to move this complete `H!veAI` directory elsewhere later and then delete the surrounding `AI-Commerce-HQ files` directory. The H!veAI repository therefore must remain self-contained and relocatable.

## Candidate duplicate / temporary directories requiring inspection

The owner identified these current laptop paths:

- `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`
- `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI`
- `C:\Users\sekip\Desktop\AI-Commerce-HQ files\m16o-remediation`
- `C:\Users\sekip\Desktop\AI-Commerce-HQ files\M21-portfolio`

Their contents and Git identities must be inspected before any deletion or merge.

Do not assume solely from directory names that every file belongs in the final H!veAI repository. Determine for each candidate directory:

- Git repository/worktree identity, if any;
- remote URL and branch;
- HEAD SHA;
- tracked/untracked/ignored state;
- whether it is a temporary worktree, remediation checkout, asset staging folder, or duplicate clone;
- whether it contains files not already committed to `Sekiph82/H-veAI`;
- whether it contains unique owner assets or uncommitted work that must be preserved.

Only H!veAI-related material should be consolidated into the final workspace. Never overwrite a newer or unique file merely because another folder contains a file with the same name.

## Laptop-wide H!veAI artifact discovery

The consolidation should also perform a bounded search of the owner's laptop for other H!veAI-specific development artifacts that may have been created during the migration/remediation work.

The search must target strong H!veAI identifiers, repository metadata, known build/publication artifacts, worktrees, logs, prompts, assets, and development copies. It must not indiscriminately move unrelated files that happen to contain generic words such as `hive`, `AI`, `tasks`, or `project`.

Any discovered candidate must be classified before consolidation.

## Portability requirement

Version-controlled H!veAI files must no longer depend on this laptop's user-specific absolute paths.

Portable repository references should use one of these forms:

1. repository-relative paths when referring to files inside the H!veAI repository;
2. GitHub URLs when referring to authoritative files, branches, prompts, audits, logs, or repository resources that should be accessible from another computer;
3. runtime-derived paths when code/scripts must operate on the current local checkout.

Do not hardcode `C:\Users\sekip\...` into portable source, documentation, governance, tests, scripts, or configuration merely to describe repository files.

A GitHub URL is not a replacement for an actual local executable path at runtime. For launchers/build helpers that necessarily operate on local files, derive the repository root or executable location dynamically rather than embedding the current laptop's absolute path in version-controlled source.

The Desktop shortcut itself is machine-local state and may point to the current local executable, but the repository should contain a relocation-safe helper/process that can recreate or refresh it after the repo is moved or cloned on another computer.

## GitHub-first project tracking must remain unchanged

This local workspace consolidation must not alter the approved H!veAI project-tracking model:

- GitHub is the project truth source;
- root `TASKS.md` is the task truth source for tracked projects;
- no `.hiveai/PROJECT.json` runtime dependency may return;
- local workspace paths are not project-status truth;
- moving the H!veAI clone to another computer must not change project state.

## Safety requirements

Before deleting or retiring any duplicate/local directory:

- verify the target standalone repo is clean or intentionally dirty with all desired changes accounted for;
- verify its `origin` is `Sekiph82/H-veAI`;
- verify local HEAD matches the intended remote `main` HEAD or explicitly preserve any divergence;
- preserve unique/uncommitted files;
- compare duplicate/conflicting files before choosing a winner;
- do not delete the source directory until the consolidated target has been validated;
- do not delete `AI-Commerce-HQ` GitHub content as part of this task;
- do not force-push;
- do not mutate unrelated projects.

## Acceptance target

After remediation, the owner should have one authoritative local H!veAI development workspace on this laptop at:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

That directory must be a standalone clone/worktree of:

`https://github.com/Sekiph82/H-veAI`

and should be safe to move later to another path or another computer without rewriting project source or governance files.