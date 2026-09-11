# HVA-LWC-001 Local Workspace Final Consolidation V02 Prompt

## MANDATORY SYNC-FIRST AND GITHUB-ONLY REPORTING CONTRACT

This section is mandatory and must be executed before reading or editing any task file.

Repository: `https://github.com/Sekiph82/H-veAI`

Branch: `main`

### Start-of-session synchronization

From the standalone H!veAI repository root:

```powershell
git fetch origin main
git status --short
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

Rules:

1. Do not begin V02 from a stale local checkout.
2. If the working tree is clean and local HEAD is behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

3. Never use reset, rebase, force-push, destructive checkout, automatic stash, or file deletion to manufacture synchronization.
4. If local H!veAI changes exist, preserve them. Determine whether they are already represented by a local commit or by GitHub before reconciling. Do not overwrite owner work.
5. Before V02 implementation starts, the intended base must be the current GitHub `main` containing the V01 strict audit and this V02 prompt.
6. Record the synchronized starting SHA in the V02 log.

### End-of-session synchronization

Local edits do not count as completed work until they are committed and pushed to `origin/main`.

Before reporting completion:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires:

- local `HEAD` equals `origin/main`;
- `git ls-remote` reports the same SHA for `refs/heads/main`;
- the H!veAI working tree is clean;
- every repository file created or changed by V02 is visible on GitHub.

If any of these conditions fail, do not say COMPLETE. Report `SYNC_BLOCKED` with the exact reason and do not hide local-only work.

### Owner-facing final response

The final Codex chat response must be GitHub-first.

Show only:

- GitHub file URLs/paths for V02 repository artifacts;
- implementation/log commit SHA(s);
- final GitHub `main` SHA;
- concise PASS/BLOCKED status.

Do **not** dump local changed-file paths or narrate ordinary local filesystem edits in the final response. Local preservation paths and local evidence belong in the committed V02 log when needed for auditability. The exception is a synchronization failure that cannot be explained without naming the local blocker.

## Work-item identity

- Work code: `HVA-LWC-001`
- Version: `V02`
- Scope: bounded remediation of V01 strict-audit findings only
- Repository: `https://github.com/Sekiph82/H-veAI`
- Branch: `main`

Read first after synchronization:

- `AGENTS.md`
- `TASKS.md`
- `docs/H!veAI/audits/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_AUDIT.md`
- `docs/H!veAI/codex-logs/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_LOG.md`
- `docs/H!veAI/prompts/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V01_PROMPT.md`
- `docs/H!veAI/HVA-LWC-001_ARTIFACT_NAMING_GOVERNANCE_V01_GOVERNANCE.md`

Do not inspect or modify another GitHub repository as part of this V02 work item. Other project repositories mentioned by historical H!veAI evidence are not V02 GitHub scope.

## V02 objective

Close exactly these V01 findings:

- `HVA-LWC-001-V01-F01`: unique owner work is preserved only under Windows Temp;
- `HVA-LWC-001-V01-F02`: active `AGENTS.md` contains a stale standalone-root Codex-log path;
- `HVA-LWC-001-V01-F03`: exhaustive 35-file archive evidence and normalized deletion-readiness matrix are missing;
- `HVA-LWC-001-V01-F04`: completion evidence must avoid impossible self-referential SHA requirements;
- `HVA-LWC-001-V01-F05`: permanent sync-first and GitHub-only reporting governance requested by the owner.

No UI redesign, product milestone work, installer work, M17-M20 implementation, unrelated repository development, branch rewrite, or destructive parent deletion is allowed.

## 1. Durable preservation outside Windows Temp

V01 reports that staging trees containing local-only or divergent owner work were preserved under a Windows Temp location.

That is not durable enough.

Create a durable preservation root outside:

- `%TEMP%`;
- `%LOCALAPPDATA%\Temp`;
- the `AI-Commerce-HQ files` parent intended for eventual deletion;
- the H!veAI Git repository itself.

Prefer deriving the user's Documents folder dynamically, for example through the Windows known-folder API or PowerShell `[Environment]::GetFolderPath('MyDocuments')`, and create a clearly named preservation directory such as:

`<Documents>\H!veAI-Preservation\HVA-LWC-001-V02\`

Do not hard-code a Windows username into repository source or governance.

Preserve complete directory trees for every V01 item whose safety depends on an external preservation copy, especially any staging tree reported to contain local-only or divergent work.

Safety rules:

- copy first, verify second;
- do not delete the source preservation copy as part of V02;
- do not reset, rebase, squash, clean, checkout over, or force-push any unrelated repository;
- do not open a remediation cycle in those unrelated repositories;
- do not mix their histories or files into `Sekiph82/H-veAI`;
- preserve `.git` metadata, untracked files, and working-tree state as part of the complete tree copy;
- verify copy completeness using deterministic file counts/sizes and, where practical, hashes for ordinary files plus Git identity/status metadata recorded from the preserved copy;
- if a source preservation tree no longer exists, do not fabricate success. Record the exact missing source and mark the relevant item BLOCKED.

The V02 log may record local preservation paths as evidence. The final Codex chat response should not list them unless a blocker requires it.

## 2. Correct standalone-root governance paths

Fix active `AGENTS.md` so the Codex-log path is repository-relative from the standalone root:

Correct:

`docs/H!veAI/codex-logs/`

Incorrect:

`H!veAI/docs/H!veAI/codex-logs/`

Perform a bounded scan of **active current H!veAI governance/instruction documents** for equivalent stale assumptions that prepend an extra `H!veAI/` directory to current repository-relative paths.

Do not rewrite immutable historical prompts, logs, or audits merely because they contain historical paths.

## 3. Make sync-first and GitHub-only reporting permanent in AGENTS.md

Update active `AGENTS.md` with a concise permanent rule that future Codex sessions must:

1. fetch/reconcile `origin/main` before reading the active prompt;
2. never silently overwrite dirty/divergent local work;
3. commit and push all repository changes before claiming completion;
4. verify `HEAD == origin/main == ls-remote refs/heads/main` before final completion;
5. treat unsynchronized local edits as incomplete;
6. present owner-facing completion primarily through GitHub file URLs/paths and commit SHAs, not local changed-file dumps;
7. show local paths in the final response only when needed to explain a blocker or a specifically requested local artifact.

Keep the existing safe fast-forward policy. Do not weaken the prohibition on reset/rebase/force-push/destructive checkout.

## 4. Publish the exhaustive 35-file loose-archive evidence matrix

V01 reported a 35-file loose H!veAI archive:

- 29 files integrated into GitHub;
- 6 duplicates mapped to canonical repository assets.

V02 must publish a complete one-row-per-original-file matrix in its immutable log.

Exactly account for all 35 original loose-archive files with columns equivalent to:

| Original loose-archive path/name | Classification | Duplicate/canonical match if any | Final GitHub destination or preservation disposition | Verification evidence |
| --- | --- | --- | --- | --- |

Requirements:

- enumerate all 35 files individually;
- do not replace rows with category prose;
- for the 29 integrated files, identify the exact `docs/H!veAI/legacy-assets/HVA-LWC-001_V01/...` GitHub destination;
- for the 6 duplicates, identify the exact canonical repository file they duplicate;
- distinguish historical document, unique asset, prompt/text, exact duplicate, and any other real classification;
- use repository evidence from V01 commits and any surviving local source inventory;
- if evidence for a specific row is genuinely unavailable, mark that row `UNVERIFIED`; do not guess.

The objective is evidence completeness, not moving the already committed archive again.

## 5. Publish normalized parent-tree deletion-readiness evidence

Re-check the relevant surrounding parent tree without deleting it.

For every meaningful remaining top-level project/staging/archive item, publish one normalized status:

- `SAFE_ON_GITHUB`
- `PRESERVED_ELSEWHERE`
- `REDUNDANT_SAFE_TO_DELETE`
- `UNIQUE_WORK_BLOCKS_DELETION`
- `OWNER_DECISION_REQUIRED`

The table must include at minimum the active H!veAI workspace/container and every V01 candidate whose disposition affected parent deletion readiness.

Do not declare the parent safe to delete while the active owner-selected H!veAI workspace still physically resides inside it.

Do not delete the parent directory in V02.

## 6. Evidence and SHA semantics

Do not require a newly created immutable log file to contain the SHA of the same commit that first creates that log. That is self-referential.

Instead:

- record the synchronized starting GitHub SHA in the V02 log;
- record all implementation commit SHA(s) that exist before the log-publication commit;
- commit and push the V02 log;
- after the push, verify the exact log commit SHA and exact final `origin/main` SHA;
- return those exact SHAs in the final GitHub-only Codex response.

Do not create vague `RERUN`, `RETRY`, `FINAL2`, `NEW`, or `LATEST` artifacts.

## 7. Repository validation

At minimum:

- `git diff --check` must pass;
- confirm no accidental nested `H!veAI/docs/H!veAI/...` active path remains in current agent/governance instructions;
- confirm V02 did not add unrelated project source trees to H!veAI;
- confirm root `TASKS.md` remains the canonical tracker and is not replaced by a competing tracker;
- confirm legacy archive files from V01 remain present;
- confirm no secrets, local databases, caches, build outputs, or preservation copies are committed;
- if no production source code changed, do not manufacture unnecessary product-code changes merely to rerun a large test suite;
- if scripts or production source are changed unexpectedly, run the relevant focused tests plus regression checks and explain why the scope expanded.

## 8. Required V02 log

Create:

`docs/H!veAI/codex-logs/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V02_LOG.md`

The log must include:

- synchronized starting `main` SHA;
- V01 finding-by-finding closure table;
- durable-preservation verification summary;
- exact list of preserved V01 candidate trees and durability result;
- complete 35-row loose-archive evidence matrix;
- active-governance path scan result;
- exact `AGENTS.md` governance changes;
- normalized parent-tree deletion-readiness matrix;
- repository changed-file scope for V02;
- `git diff --check` result;
- any focused validation performed;
- implementation commit SHA(s) known before log publication;
- explicit statement that the log's own commit SHA will be verified after publication rather than fabricated inside the file.

## 9. Required completion state

V02 may report `COMPLETE` only when:

- durable non-Temp preservation exists for every unique/divergent owner-work tree that V01 relied upon for safety;
- the stale standalone-root log path is corrected;
- permanent sync-first/GitHub-only reporting governance is present in `AGENTS.md`;
- all 35 loose-archive files are individually accounted for;
- the deletion-readiness matrix is complete;
- all V02 repository artifacts are committed and pushed;
- local H!veAI `HEAD`, `origin/main`, and remote `refs/heads/main` are identical;
- the H!veAI working tree is clean.

If any preservation source is missing or any required safety condition cannot be proved, report `SYNC_BLOCKED` or `PRESERVATION_BLOCKED` as appropriate. Do not convert missing evidence into PASS.

## 10. Final response format

Keep the final Codex response short.

Return only:

- status: `COMPLETE`, `SYNC_BLOCKED`, or `PRESERVATION_BLOCKED`;
- GitHub URL/path to the V02 log;
- GitHub URL/path to the updated `AGENTS.md`;
- implementation commit SHA(s);
- V02 log commit SHA;
- exact final GitHub `main` SHA.

Do not list ordinary local file changes in the final chat response.