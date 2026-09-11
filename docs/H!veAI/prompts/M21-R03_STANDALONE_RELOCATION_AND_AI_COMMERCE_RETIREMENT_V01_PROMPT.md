# M21-R03 Standalone Relocation and AI-Commerce Retirement Readiness V01

## MANDATORY SYNC-FIRST AND GITHUB-FIRST CONTRACT

This is a destructive-retirement **readiness** task, not a deletion task.

Start from the currently active H!veAI checkout and safely synchronize `Sekiph82/H-veAI` on `main` before doing anything else:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the working tree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never use reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or any operation that discards owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

Read before execution:

- `AGENTS.md`
- `TASKS.md`
- `docs/H!veAI/audits/M21-R02_COCKPIT_TASKS_AND_AI_COMMERCE_RETIREMENT_V02_AUDIT.md`
- `docs/H!veAI/audits/M21-R02-ACC_OWNER_NATIVE_REACCEPTANCE_V01_AUDIT.md`
- `docs/H!veAI/audits/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V03_AUDIT.md`
- `docs/H!veAI/evidence/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V03_PRESERVATION_RECEIPT.json`
- `docs/H!veAI/SAFE_GIT_SYNC_POLICY.md`

All Codex-facing work and the required log must be entirely in English.

## WORK ITEM

- Work code: `M21-R03`
- Version: `V01`
- GitHub repository receiving changes: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M21-R03_STANDALONE_RELOCATION_AND_AI_COMMERCE_RETIREMENT_V01_LOG.md`
- Required receipt: `docs/H!veAI/evidence/M21-R03_STANDALONE_RELOCATION_AND_AI_COMMERCE_RETIREMENT_V01_RECEIPT.json`

The owner has completed M21-R02 native re-acceptance successfully. Project Cockpit Tasks now renders canonical remote rows, the portfolio shows exactly eight projects, and the previous `No parsed tasks` failure is closed.

The owner now wants to retire both:

1. the historical local AI-Commerce parent tree; and
2. the GitHub repository `Sekiph82/AI-Commerce-HQ`.

This V01 task must make those deletions safe and auditable, but **must not perform either deletion**.

## SAFETY PRINCIPLE

Never destroy the only copy of anything.

The final active H!veAI checkout must live outside the historical AI-Commerce parent before that parent can be considered deletion-ready.

The complete Git history/refs of `Sekiph82/AI-Commerce-HQ` must have a durable local mirror outside the historical parent and outside temporary folders before the GitHub repository can be considered deletion-ready.

If any required preservation, relocation, build, shortcut, ref-comparison, or integrity gate cannot be proven, return `RETIREMENT_BLOCKED` and preserve everything.

## PHASE 1 — DISCOVER AND CLASSIFY CURRENT PATHS

Resolve known Windows folders dynamically:

```powershell
$desktop   = [Environment]::GetFolderPath('Desktop')
$documents = [Environment]::GetFolderPath('MyDocuments')
```

Preferred final standalone H!veAI checkout:

```text
<Desktop>\H!veAI
```

Preferred durable retirement preservation root:

```text
<Documents>\H!veAI-Preservation\M21-R03
```

Expected historical parent from the prior migration may be under:

```text
<Desktop>\AI-Commerce-HQ files\AI-Commerce-HQ
```

Do not blindly trust that expected path. Discover and verify the actual current H!veAI root and historical AI-Commerce parent using Git metadata and filesystem evidence.

Record logical path roles in the log, but do not commit private absolute Windows paths into the receipt.

Before relocation:

- prove the current H!veAI checkout is `Sekiph82/H-veAI` on `main`;
- prove it is clean and synchronized;
- enumerate non-ignored untracked files with `git ls-files --others --exclude-standard`;
- inspect ignored/untracked categories read-only enough to distinguish reproducible build/cache output from potentially unique owner files;
- if unique owner files exist in the current H!veAI checkout outside Git, preserve them safely or stop with `RETIREMENT_BLOCKED`;
- do not delete anything.

## PHASE 2 — CREATE THE FINAL STANDALONE H!VEAI CHECKOUT OUTSIDE THE HISTORICAL PARENT

The target is `<Desktop>\H!veAI`.

If the target does not exist, clone from GitHub:

```powershell
git clone https://github.com/Sekiph82/H-veAI.git <Desktop>\H!veAI
```

If the target already exists, do not overwrite it. Verify that it is the correct `Sekiph82/H-veAI` checkout, inspect its status/divergence, and only use it if it can be reconciled safely under `AGENTS.md` rules. Otherwise stop with `RETIREMENT_BLOCKED`.

The new checkout must satisfy all of the following:

- repository identity is exactly `Sekiph82/H-veAI`;
- branch is `main`;
- `HEAD == origin/main` at relocation time;
- working tree is clean before build/publication;
- path is not equal to, inside, or below the historical AI-Commerce parent;
- path is not in `%TEMP%` or another temporary directory.

Once the new standalone checkout is verified, treat it as the canonical local H!veAI workspace for the rest of this task. Make all M21-R03 repository edits/commits/pushes from this new checkout only.

Do not move or delete the old nested H!veAI checkout yet.

## PHASE 3 — REBUILD/PUBLISH H!VEAI FROM THE NEW CHECKOUT

From `<Desktop>\H!veAI`:

- restore/install dependencies using the repository's normal deterministic process;
- run the minimum relevant validation required by the publication helper;
- run the existing safe native publication process `scripts/publish-dev-qa.ps1`;
- publish `dev-bin/H!veAI.exe` under the **new** standalone checkout;
- regenerate/validate `dev-bin/H!veAI.ico` if the existing publication helper requires it;
- update/validate the stable Desktop `H!veAI.lnk` so its target is the new `<Desktop>\H!veAI\dev-bin\H!veAI.exe`;
- the shortcut must target the executable directly, never PowerShell/cmd/npm/cargo/browser/scripts;
- run the existing native smoke/readiness checks from the new checkout;
- confirm no visible console-host regression in automated publication checks;
- record the published executable SHA-256.

Do not claim owner visual acceptance for the relocated build. Automated smoke and shortcut validation are required now; owner double-click confirmation is a later final deletion gate.

## PHASE 4 — PROVE ACTIVE H!VEAI NO LONGER DEPENDS ON AI-COMMERCE-HQ

Inspect **active production/runtime/configuration paths** in H!veAI for live dependencies on the historical repository or parent. At minimum inspect current source/configuration/scripts that can affect runtime/bootstrap/publication, including relevant files under:

- `src/`
- `src-tauri/`
- `scripts/`
- active package/build/Tauri configuration

Historical prompts, logs, audits, migration notes, and legacy assets are allowed to mention AI-Commerce-HQ for provenance. Do not rewrite historical evidence merely to remove the text.

The deletion-readiness gate requires:

- no active default portfolio target for `Sekiph82/AI-Commerce-HQ`;
- no production bootstrap path that fetches or requires it;
- no launcher/shortcut target inside the historical parent;
- no active build/publication path that depends on the old checkout;
- no required current tracker/control-plane truth hosted only in AI-Commerce-HQ;
- H!veAI canonical remote remains `Sekiph82/H-veAI` on `main`.

If an active dependency remains, stop with `RETIREMENT_BLOCKED` rather than deleting or hiding it.

## PHASE 5 — CREATE A DURABLE MIRROR OF THE AI-COMMERCE-HQ GITHUB REPOSITORY

This phase is explicitly authorized as **read-only inspection/preservation** of `Sekiph82/AI-Commerce-HQ` because the owner intends to delete that GitHub repository after this gate.

Do not modify, push to, rewrite, or delete `Sekiph82/AI-Commerce-HQ`.

Create a durable mirror outside the historical parent and outside temp, preferably:

```text
<Documents>\H!veAI-Preservation\M21-R03\AI-Commerce-HQ.git
```

Use a real mirror clone/fetch model that preserves repository refs and objects, for example:

```powershell
git clone --mirror https://github.com/Sekiph82/AI-Commerce-HQ.git <mirror-path>
```

If a verified mirror already exists at the intended durable location, update it safely with mirror semantics rather than overwriting unknown content.

Then verify at minimum:

- `git --git-dir=<mirror-path> fsck --full` passes;
- remote heads/tags/refs are enumerated read-only from GitHub;
- mirror heads/tags/refs are enumerated locally;
- every deletion-relevant remote branch/tag ref and SHA is present in the mirror;
- mirror is not inside `%TEMP%`, the historical AI-Commerce parent, or the new H!veAI checkout;
- mirror is non-empty and readable after creation;
- no credentials/secrets are copied into H!veAI GitHub evidence.

Do not commit the mirror itself to `Sekiph82/H-veAI`.

The receipt may record only privacy-safe summary evidence such as remote/mirror ref counts, equality booleans, mirror fsck result, and aggregate/ref-set hashes. Do not commit private absolute paths or giant ref listings.

## PHASE 6 — RECHECK EXISTING LOCAL PRESERVATION

Re-run or reuse the existing HVA-LWC deterministic preservation verifier as appropriate to ensure the previously preserved historical material still validates.

Do not delete or relocate those preservation trees during M21-R03.

If preservation verification no longer passes, stop with `RETIREMENT_BLOCKED`.

## PHASE 7 — UPDATE CURRENT TRACKER TRUTH

Update root `TASKS.md` to reflect the real current state.

During work, M21-R03 is active. After successful implementation/publication/preservation, the correct state is:

- M21-R02 = PASS/CLOSED with owner native re-acceptance complete;
- M21-R03 = implementation/readiness complete, awaiting independent retirement audit and final owner shortcut launch confirmation;
- deletion has **not** yet occurred;
- Required Actor = HUMAN after builder completion because independent audit + owner final confirmation remain.

Preserve the user-facing 20-milestone denominator and historical M21/M21-R01/M21-R02 evidence.

## REQUIRED RECEIPT

Create exactly:

`docs/H!veAI/evidence/M21-R03_STANDALONE_RELOCATION_AND_AI_COMMERCE_RETIREMENT_V01_RECEIPT.json`

The receipt must be compact, machine-readable, privacy-safe, and include at minimum:

```json
{
  "workCode": "M21-R03",
  "version": "V01",
  "newStandaloneCheckoutVerified": true,
  "newCheckoutOutsideHistoricalParent": true,
  "newCheckoutHead": "<sha>",
  "desktopShortcutTargetsNewCheckout": true,
  "nativePublicationPassed": true,
  "publishedExecutableSha256": "<sha256>",
  "activeAiCommerceRuntimeReferences": 0,
  "aiCommerceMirrorCreated": true,
  "aiCommerceMirrorFsckPassed": true,
  "aiCommerceRemoteRefCount": 0,
  "aiCommerceMirrorRefCount": 0,
  "aiCommerceRefsMatch": true,
  "priorPreservationVerificationPassed": true,
  "oldParentStillPresent": true,
  "githubAiCommerceRepoStillPresent": true,
  "localParentDeletionReady": true,
  "githubRepositoryDeletionReady": true,
  "ownerFinalLaunchConfirmationRequired": true
}
```

Use real values. If any readiness boolean cannot be proven true, set the applicable readiness result false and return `RETIREMENT_BLOCKED`.

Do not fabricate successful deletion readiness.

## REQUIRED VALIDATION

At minimum verify:

- new standalone clone identity/branch/cleanliness;
- final new-checkout `HEAD == origin/main == git ls-remote origin refs/heads/main` after all H!veAI commits are pushed;
- new checkout is outside the historical parent;
- H!veAI native publication from the new checkout passes;
- Desktop shortcut target/icon validation passes and target is under the new checkout;
- H!veAI active production/configuration paths have no live AI-Commerce dependency;
- default portfolio still contains exactly the intended eight repositories;
- AI-Commerce Git mirror integrity/ref preservation passes;
- existing HVA-LWC preservation verification remains PASS;
- `git diff --check` passes;
- no secrets, local DB, cache, build output, preservation tree, mirror repository, or machine-specific private artifact is committed;
- historical prompt/log/audit artifacts remain immutable.

## DESTRUCTIVE ACTIONS EXPRESSLY PROHIBITED IN V01

Do **not**:

- delete the historical local AI-Commerce parent;
- delete the old nested H!veAI checkout;
- delete any preservation tree;
- delete `Sekiph82/AI-Commerce-HQ` from GitHub;
- modify or push to `Sekiph82/AI-Commerce-HQ`;
- reset/rebase/force-push/clean/stash away owner work;
- rewrite historical H!veAI evidence;
- mark deletion as completed.

This is a readiness gate. The owner will perform/authorize final deletion only after an independent M21-R03 audit and final launch confirmation from the relocated shortcut.

## REQUIRED CODEX LOG

Create exactly:

`docs/H!veAI/codex-logs/M21-R03_STANDALONE_RELOCATION_AND_AI_COMMERCE_RETIREMENT_V01_LOG.md`

The log must include:

- synchronized starting H!veAI SHA;
- old/current checkout logical role and new standalone checkout logical role;
- proof the new checkout is outside the historical parent;
- new checkout repository/branch/HEAD proof;
- publication/build/smoke results from the new checkout;
- Desktop shortcut target validation result;
- executable SHA-256;
- active AI-Commerce dependency scan result and any allowed historical-only references;
- AI-Commerce mirror method, ref-count/equality evidence, and `git fsck --full` result;
- prior preservation verifier result;
- receipt path and summary;
- exact H!veAI implementation/evidence commit SHA(s) known before log publication;
- explicit statement that the old local parent and GitHub AI-Commerce repository still exist and were not modified/deleted;
- final readiness verdict for local parent deletion and GitHub repository deletion;
- explicit owner final-launch confirmation requirement;
- final GitHub synchronization verification.

The log must not attempt to contain its own creating commit SHA. Return the log commit SHA in the final response after publication.

## COMPLETION STATUS

Return exactly one:

- `READY_FOR_INDEPENDENT_RETIREMENT_AUDIT`
- `SYNC_BLOCKED`
- `RETIREMENT_BLOCKED`

`READY_FOR_INDEPENDENT_RETIREMENT_AUDIT` is allowed only when both deletion-readiness gates are supported by evidence, all H!veAI repository changes are committed/pushed from the new standalone checkout, and new-checkout local `HEAD`, `origin/main`, and live remote `main` are identical.

Final owner-facing response must contain only relevant GitHub H!veAI file URLs/paths, implementation/log commit SHA(s), final H!veAI `main` SHA, and concise status. Do not dump ordinary local file changes or private absolute paths.
