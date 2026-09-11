# HVA-LWC-001 — Local Workspace Final Consolidation — V01 Prompt

## Work-item identity

- Work code: `HVA-LWC-001`
- Version: `V01`
- Repository: `https://github.com/Sekiph82/H-veAI`
- Branch: `main`
- Canonical local workspace required by owner:
  `C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

This task is the first H!veAI task using the new versioned artifact naming system.

For this work item, all new artifacts must use the same work code and version:

- Prompt: `HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_PROMPT.md`
- Builder log: `HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_LOG.md`
- Audit: `HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_AUDIT.md`

If this version does not fully pass, do **not** create another `RERUN` file. The next remediation becomes `V02`, then `V03`, etc.

Read the independent audit first:

`docs/H!veAI/audits/HVA-LWC-001_LOCAL_WORKSPACE_CONSOLIDATION_RERUN_AUDIT_V01.md`

## Owner goal

The owner's one and only active H!veAI local development workspace on this laptop must be:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

That directory must remain a standalone clone of:

`https://github.com/Sekiph82/H-veAI`

on branch:

`main`

The owner intends later to move this entire H!veAI directory elsewhere and delete the surrounding:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files`

directory from the laptop.

Therefore this task is not complete merely because the current checkout builds. The surrounding tree must be audited so no unique H!veAI material or unrelated unpushed owner work is accidentally lost.

## 1. Keep the canonical workspace exactly where the owner requested

Do not move the active H!veAI checkout away from:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

During this task.

Verify:

- it has its own `.git`;
- `origin` is `https://github.com/Sekiph82/H-veAI.git`;
- active branch is `main`;
- local HEAD and `origin/main` are synchronized before final publication;
- this directory contains the complete active H!veAI source, assets, tests, scripts, docs, dev-bin publication output and repository metadata required for continued development.

## 2. Finish consolidation of the loose H!veAI archive

The previous rerun found a non-Git loose H!veAI archive under the surrounding parent tree containing approximately 35 files / 75 MB of logos, scenes, prompts, videos and related material.

Inspect every file in that archive.

For each file, classify it as:

- exact duplicate of canonical repository content;
- superseded historical copy;
- unique H!veAI source/config/script/document;
- unique H!veAI asset;
- generated build/cache/runtime output;
- unrelated file;
- secret/private/local-only data that must not be committed.

For every genuinely H!veAI-owned unique and useful file:

- integrate it into an appropriate location inside the canonical workspace;
- preserve original useful content;
- avoid duplicate/conflicting source-of-truth copies;
- commit project-critical portable content to `Sekiph82/H-veAI` when appropriate so another computer can obtain it from GitHub;
- do not silently lose or overwrite unique owner material.

If a file is a duplicate or obsolete historical copy, record that fact before removing the external copy.

After successful consolidation, the loose external H!veAI archive should no longer be needed as an independent project source.

## 3. Do not mix unrelated portfolio repositories into H!veAI

The previous audit found that:

- `m16o-remediation`
- `M21-portfolio`

are staging areas containing independent Git repositories for other projects. They are not H!veAI source directories.

Do not copy these repositories into the H!veAI repository.

Instead, inspect each contained repository with Git-aware checks:

- repository identity / remote;
- branch;
- HEAD;
- relationship to its remote branch;
- uncommitted changes;
- untracked files;
- local-only commits;
- divergent history.

For each staging repository, determine whether all meaningful work is safely represented on its correct GitHub repository.

If unique/unpushed owner work exists, preserve it safely. Do not delete it merely to clean the parent tree.

If the work can be safely pushed/merged without rewriting history or damaging the target repository, do so only after confirming the correct repository/branch and normal project governance. Otherwise relocate the staging copy outside the soon-to-be-deleted parent tree and record why it still exists.

The final H!veAI consolidation must never contaminate `Sekiph82/H-veAI` with unrelated project histories.

## 4. Re-audit the retired H-veAI copy

The previous run moved an older H-veAI checkout to a temporary retirement location.

Re-check that copy for:

- uncommitted changes;
- untracked files;
- commits absent from `origin/main`;
- unique assets or docs absent from the canonical workspace.

If it is fully redundant, it may be safely removed.

If anything unique exists, integrate/preserve it first.

Do not delete solely because a prior log claimed it was redundant.

## 5. Search for additional H!veAI copies or strong indicators

Perform a bounded search of the owner's relevant development directories for strong H!veAI indicators such as:

- Git remotes pointing to `Sekiph82/H-veAI`;
- directories named `H!veAI`, `H-veAI`, `hiveai` or clearly named migration/remediation copies;
- H!veAI `AGENTS.md`, `TASKS.md`, startup video, logos or build artifacts paired with source trees;
- worktrees or retired clones.

Do not crawl unrelated personal files indiscriminately.

Classify every strong match. The final report must state whether it is:

- canonical;
- duplicate;
- historical archive;
- unrelated;
- contains unique work;
- safe to remove;
- must be preserved elsewhere.

## 6. Make the repository portable across computers

The GitHub repository must be the portable source of truth for development documentation and project files.

Active version-controlled files must not depend on this laptop's absolute path.

Use these rules:

### Repository-internal references

Use repository-relative paths whenever a file references another file inside H!veAI.

Example:

`docs/H!veAI/...`

not:

`C:\Users\sekip\Desktop\...`

### Remote references

When documentation intends to point to a repository, branch, prompt, audit, or source that should be reachable from another computer, prefer GitHub repository URLs or repository-relative references.

### Runtime/local filesystem paths

Do **not** replace executable filesystem paths with GitHub URLs. A native executable cannot launch from a GitHub webpage.

Instead, scripts and runtime code must derive paths from the current checkout/install location dynamically.

The repository must continue working if tomorrow it is cloned to a completely different folder or another Windows username.

### Historical evidence

Old immutable logs/audits may contain historical absolute paths when those paths are evidence of what happened at that time. Do not rewrite history merely to make old evidence look current.

Current governance/docs/scripts must not instruct future agents to depend on old machine-specific paths.

## 7. Update active governance to the owner's current canonical local topology without making it machine-dependent

The owner currently chooses this laptop workspace:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files\AI-Commerce-HQ\H!veAI`

This is the current local checkout location, not a portable repository contract.

Review current active guidance such as `AGENTS.md`, architecture/governance docs, publication helpers, and developer instructions.

They should clearly distinguish:

- GitHub canonical repository: `https://github.com/Sekiph82/H-veAI`;
- branch: `main`;
- current laptop checkout: the owner-selected path above;
- portable behavior: another computer may clone the same repository anywhere.

No future Codex task should fail simply because the clone is not under `C:\Users\sekip`.

## 8. Launcher and dev publication behavior

On this laptop, the Desktop H!veAI shortcut should launch the validated executable from the owner's current canonical workspace.

But the publication helper must remain relocation-safe and derive its root from the repository itself.

Verify after final consolidation:

- stable `dev-bin/H!veAI.exe` exists in the canonical workspace;
- Desktop shortcut resolves to that executable on this laptop;
- shortcut working directory/icon are valid;
- no old retired copy is accidentally launched;
- moving/cloning the repository elsewhere would require only rerunning the publication/shortcut helper, not editing hard-coded source paths.

## 9. Parent-tree deletion-readiness report

The owner eventually wants to move the canonical H!veAI folder elsewhere and delete:

`C:\Users\sekip\Desktop\AI-Commerce-HQ files`

Do **not** delete that entire directory in this task unless the owner has separately and explicitly requested the final destructive deletion after reviewing the readiness report.

Instead produce a precise deletion-readiness inventory of the surrounding tree.

For every meaningful remaining top-level project/staging/archive item state one of:

- `SAFE_ON_GITHUB`
- `PRESERVED_ELSEWHERE`
- `REDUNDANT_SAFE_TO_DELETE`
- `UNIQUE_WORK_BLOCKS_DELETION`
- `OWNER_DECISION_REQUIRED`

The final result must make it obvious what, if anything, still prevents the owner from deleting the parent directory.

## 10. Introduce permanent versioned artifact naming governance

Create/update current governance so future H!veAI prompts, logs and audits use a stable work code and two-digit version.

Canonical filename shape:

`<WORK_CODE>_<SHORT_DESCRIPTION>_VNN_<TYPE>.md`

Examples:

`HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_PROMPT.md`

`HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_LOG.md`

`HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_AUDIT.md`

Rules:

- `V01` is the first execution of a work item;
- if remediation is required, use `V02`;
- another remediation uses `V03`, etc.;
- never create vague suffixes such as `RERUN`, `RETRY`, `FINAL2`, `NEW`, `LATEST` for future active work;
- old historical files keep their historical names;
- a materially new task gets a new work code;
- prompt, builder log and independent audit for one execution version must share the same work code and `VNN`;
- every builder completion response must print the exact work code/version.

Create a short current governance document for this convention and reference it from active agent instructions.

## Validation

Before completion verify at minimum:

1. canonical workspace is the owner-requested nested directory;
2. its origin/branch/HEAD are correct;
3. loose H!veAI archive has been fully inventoried and all unique useful H!veAI content has been integrated/preserved;
4. no unrelated project repository was mixed into H!veAI;
5. staging directories with unique/unpushed portfolio work are preserved or safely resolved;
6. retired H-veAI copy contains no unique work before removal;
7. active repository content has no operational dependency on one Windows username/path;
8. GitHub/relative references are used appropriately for portable documentation;
9. build/typecheck/tests pass;
10. native publication succeeds from the canonical workspace;
11. Desktop shortcut launches the exact canonical executable;
12. no terminal flashing regression appears;
13. GitHub + root TASKS.md project tracking remains unchanged;
14. deletion-readiness inventory is complete;
15. new artifact naming governance exists and is referenced by active instructions;
16. all new artifacts for this task use `HVA-LWC-001 ... V01` naming.

## Required log

Create:

`docs/H!veAI/codex-logs/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_LOG.md`

The log must include:

- exact canonical local path;
- origin, branch, initial HEAD and final HEAD;
- full classification of each known candidate directory;
- loose H!veAI archive inventory result;
- unique files integrated and destination paths;
- staging repo divergence/unpushed-work disposition;
- retired-copy disposition;
- additional-copy search result;
- machine-specific-path scan result;
- build/test/native validation;
- shortcut target result;
- parent-tree deletion-readiness table;
- artifact naming governance created/updated;
- implementation commit SHA(s);
- log commit SHA;
- exact final `origin/main` HEAD;
- EXE SHA-256.

Do not declare PASS while any unique H!veAI material remains outside the canonical workspace without an explicit preservation reason, or while any remaining parent-tree item with unique owner work is mislabeled safe to delete.