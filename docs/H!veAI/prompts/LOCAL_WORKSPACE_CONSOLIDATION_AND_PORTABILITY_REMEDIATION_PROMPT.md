# H!veAI Local Workspace Consolidation + Portability Remediation

Work from the current `Sekiph82/H-veAI` repository state on `main`.

First read:

- `AGENTS.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `docs/H!veAI/audits/LOCAL_WORKSPACE_CONSOLIDATION_PRE_AUDIT.md`
- `docs/H!veAI/codex-logs/NATIVE_PRODUCT_POLISH_AND_PORTFOLIO_BEHAVIOR_A01_REMEDIATION_LOG.md`

This task changes the owner's preferred local development topology while preserving the standalone GitHub repository and GitHub-first project-tracking model.

## Owner decision

The authoritative GitHub repository remains:

`https://github.com/Sekiph82/H-veAI`

The owner now wants the one primary H!veAI development workspace on this laptop to be:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

This directory must become the complete standalone local checkout of `Sekiph82/H-veAI` on `main`.

Do not interpret the path as meaning H!veAI belongs to the `AI-Commerce-HQ` Git repository again. The local H!veAI directory must keep its own standalone `.git` repository metadata and `origin` must remain `https://github.com/Sekiph82/H-veAI`.

Do not convert it into an AI-Commerce-HQ subtree or submodule.

The owner intends to move this entire `H!veAI` directory somewhere else later and then delete the surrounding `AI-Commerce-HQ files` directory from the laptop. Therefore the H!veAI workspace must be self-contained and relocatable.

---

# 1. Audit and consolidate all known H!veAI local directories

Inspect these paths before moving, merging, or deleting anything:

1. `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`
2. `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`
3. `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI`
4. `C:\Users\sekip\Desktop\AI-Commerce-HQ files\m16o-remediation`
5. `C:\Users\sekip\Desktop\AI-Commerce-HQ files\M21-portfolio`

For every directory, determine what it actually is before touching it:

- Git repository or worktree identity;
- remote URL;
- branch;
- HEAD SHA;
- git status;
- tracked/untracked/ignored files;
- whether it is a duplicate clone, temporary worktree, remediation checkout, migration staging directory, asset folder, or unrelated material;
- whether it contains any H!veAI file that is unique, newer, uncommitted, or missing from `Sekiph82/H-veAI@main`.

Do not blindly merge folders by filename.

If two files conflict, inspect content/history and preserve the correct/current/unique material. Never silently overwrite owner work.

If `m16o-remediation` or `M21-portfolio` are temporary H!veAI worktrees/checkouts, recover any unique useful content first and retire them only after the final target is verified.

If any of the named folders are not actually H!veAI-related, leave them alone and report that fact.

---

# 2. Find other H!veAI development copies on this laptop

Perform a bounded search for other H!veAI-specific development artifacts that may have been created during the long migration/remediation process.

Look for strong indicators such as:

- a Git remote pointing to `Sekiph82/H-veAI`;
- H!veAI-specific `AGENTS.md`, `TASKS.md`, `src-tauri`, `dev-bin`, startup-video assets, prompts, audits, Codex logs, migration worktrees, or publication copies;
- worktree metadata associated with the H-veAI repository;
- folders with clearly H!veAI-specific names and matching repository contents.

Do not sweep unrelated user files merely because they contain generic words such as `AI`, `hive`, `task`, `project`, or `log`.

Classify every candidate before moving anything.

The goal is to eliminate accidental duplicate development roots, not to vacuum the laptop indiscriminately.

---

# 3. Build one canonical local workspace

The final local development root for this laptop must be:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

It must contain the complete current standalone H!veAI repository and all required development/runtime assets.

Required properties:

- its own `.git` directory belongs to `Sekiph82/H-veAI`;
- branch is `main`;
- `origin` is the standalone H-veAI GitHub repository;
- all desired committed source from current `main` is present;
- any legitimate unique local H!veAI work discovered during consolidation is preserved intentionally;
- no obsolete duplicate clone is still treated as an active development root;
- the application builds and runs from this directory;
- `dev-bin/H!veAI.exe` is produced/published under this directory;
- the Desktop `H!veAI.lnk` launches the executable from this directory after consolidation.

Do not require the owner to remember which of several old folders is the real project after this task.

---

# 4. Remove machine-specific absolute paths from portable repository truth

The repository must be usable tomorrow from another computer where `C:\Users\sekip\...` does not exist.

Audit all active version-controlled files for machine-specific absolute local paths, including at minimum:

- governance/instruction files;
- architecture documents;
- prompts and active operational docs;
- build/publish scripts;
- frontend/backend/native configuration;
- tests and fixtures that influence production behavior;
- launcher/publication helpers;
- comments or constants that production tooling actually consumes.

Historical immutable audit/log evidence may retain old paths when they are part of the historical record. Do not rewrite immutable historical logs simply to cosmetically remove past paths. Current active instructions and runtime/configuration must be portable.

Use the correct portable reference type:

### Repository-internal file references

Use repository-relative paths, for example:

`src/assets/H!veAI.mp4`

not:

`C:\Users\sekip\Desktop\...\src\assets\H!veAI.mp4`

### Authoritative remote references

When documentation, prompts, audits, or operational instructions need a location that should work from any computer, use the canonical GitHub repository URL or a repository-relative path.

Canonical remote:

`https://github.com/Sekiph82/H-veAI`

### Runtime local file operations

Do not replace required local executable/file operations with GitHub URLs.

Instead, code and scripts must derive paths from the current repository root, current executable location, script location, environment, or another relocation-safe mechanism.

There should be no production need to know the username `sekip` or the current Desktop hierarchy.

The repository should remain functional if cloned to, for example:

`D:\Projects\H!veAI`

without editing source files merely to change path strings.

---

# 5. Update active governance to separate GitHub identity from local checkout location

The current `AGENTS.md` and any other active instruction documents must stop treating one laptop-specific absolute path as repository identity.

The durable rule should be conceptually:

- canonical GitHub repository: `https://github.com/Sekiph82/H-veAI`;
- canonical branch: `main`;
- local checkout: wherever the repository is currently cloned;
- on the owner's current laptop, the preferred checkout is presently `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`, but active automation must not require that exact string to function.

Do not restore the old architectural meaning that H!veAI is part of the AI-Commerce-HQ repository.

The local path is only a current workstation preference.

---

# 6. Make publication/launcher behavior relocation-safe

After consolidation, the owner's Desktop shortcut must launch the executable from the new current workspace:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI\dev-bin\H!veAI.exe`

However, do not embed this exact machine path throughout version-controlled code.

The repository must provide a safe way to rebuild/publish and refresh the Desktop shortcut based on the current checkout location.

If the owner later moves the whole H!veAI directory elsewhere, only machine-local launcher state should need refreshing. The repository source itself should not require a mass search/replace.

Validate the shortcut target, working directory, icon, native startup, startup video, and hidden background process behavior after the move.

---

# 7. Preserve GitHub-first project tracking

Do not regress the project tracking system while reorganizing local files.

Preserve:

- GitHub as the project truth source;
- root `TASKS.md` as task truth;
- no `.hiveai/PROJECT.json` runtime dependency;
- current project portfolio behavior;
- current task/progress parsing;
- Command Center and Project Cockpit behavior;
- project removal persistence;
- Builder/Auditor settings behavior;
- two-decimal progress formatting;
- hidden background GitHub polling;
- immediate startup video;
- no visible terminal flashing.

Local filesystem consolidation must not become another project-state authority.

---

# 8. Retire duplicate local roots only after verification

Once the target workspace is verified, retire obsolete H!veAI-specific duplicate folders/worktrees that are confirmed redundant.

Do not delete any directory until all of the following are true:

- unique/uncommitted files have been preserved or intentionally rejected with evidence;
- target repository `origin` and branch are correct;
- target checkout contains the intended source;
- build and tests pass from the target;
- published EXE launches from the target;
- Desktop shortcut launches the target EXE;
- no active script/config/document points to the retired path;
- no Git worktree metadata is left broken;
- the owner would not lose material by deleting the old directory.

For every retired directory, record why it was safe to retire.

If a directory cannot safely be removed, keep it and clearly report why.

---

# 9. Verify future portability

Simulate/verify that the repository does not depend on its present absolute path.

At minimum verify active repository content does not require:

- `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H-veAI`
- `C:\Users\sekip\Desktop\AI-Commerce-HQ files\H!veAI`
- `C:\Users\sekip\Desktop\AI-Commerce-HQ files\m16o-remediation`
- `C:\Users\sekip\Desktop\AI-Commerce-HQ files\M21-portfolio`
- the old `AI-Commerce-HQ\H!veAI` path as repository identity rather than merely the owner's current checkout location.

Current-machine launcher state may naturally resolve to the current checkout, but repository code must derive it rather than depend on a fixed username/path.

The finished H!veAI directory must be movable/copied/cloned to another computer and continue development from GitHub without needing access to this laptop's other folders.

---

# 10. Required validation

Before declaring completion, perform and record at minimum:

1. Final canonical local path exists: `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`.
2. That directory is a standalone Git repository for `Sekiph82/H-veAI`.
3. Branch is `main`.
4. Local HEAD / `origin/main` relationship is explicitly verified.
5. All five owner-named candidate directories were inspected and classified.
6. Other strongly identified H!veAI local copies/worktrees were searched for and classified.
7. Any unique local material was preserved before cleanup.
8. Active repo files no longer hardcode user-specific absolute paths where portable relative/GitHub/dynamic resolution is appropriate.
9. Active governance no longer treats one laptop path as repository identity.
10. Build/typecheck/frontend tests/Rust tests relevant to the current application remain green.
11. Native Tauri build succeeds from the final target directory.
12. `dev-bin/H!veAI.exe` is published under the final target directory.
13. Desktop `H!veAI.lnk` launches that executable.
14. Startup video remains immediate.
15. No visible Git/cmd/PowerShell/Terminal windows return.
16. GitHub + root `TASKS.md` tracking remains functional.
17. No retired duplicate path remains referenced by active source/config/governance.
18. The final workspace is demonstrably safe to move later without source-code path rewrites.

If one long-running historical observational test remains genuinely unbounded, report it truthfully rather than pretending it passed. Do not let that prevent focused verification of this filesystem/portability task if all directly relevant gates pass.

---

# 11. Logging

Create a new immutable log:

`docs/H!veAI/codex-logs/LOCAL_WORKSPACE_CONSOLIDATION_AND_PORTABILITY_REMEDIATION_LOG.md`

The log must include:

- starting local topology;
- classification of every owner-named directory;
- any additional H!veAI copies/worktrees discovered;
- files/content recovered from duplicates;
- directories retired and evidence that retirement was safe;
- final canonical local workspace;
- final Git remote/branch/HEAD;
- active absolute-path references found and how each was corrected;
- historical paths intentionally retained only in immutable evidence;
- launcher/shortcut result;
- build/test/native verification;
- GitHub + TASKS tracking regression result;
- published EXE SHA-256;
- implementation commit SHA;
- log commit SHA;
- final `origin/main` HEAD.

Do not declare success merely because files were copied. Success means the owner has one clear local H!veAI development workspace, no H!veAI work was lost, active repository content is portable, and continued development from another computer depends on GitHub rather than this laptop's directory layout.