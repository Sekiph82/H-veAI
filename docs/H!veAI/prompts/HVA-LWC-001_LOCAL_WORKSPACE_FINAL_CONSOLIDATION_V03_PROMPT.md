# HVA-LWC-001 Local Workspace Final Consolidation V03 Prompt

## MANDATORY SYNC-FIRST CONTRACT

Repository: `https://github.com/Sekiph82/H-veAI`

Branch: `main`

Before reading or editing the active task, synchronize the standalone H!veAI checkout safely:

```powershell
git fetch origin main
git status --short
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the working tree is clean and local HEAD is only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never use reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or any operation that discards dirty/divergent owner work.

Do not start V03 until the local checkout contains this prompt and the V02 strict audit from GitHub `main`.

At completion, all H!veAI repository changes must be committed and pushed. Verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Do not report `COMPLETE` unless local `HEAD`, `origin/main`, and remote `refs/heads/main` are identical and the H!veAI working tree is clean.

## ENGLISH-ONLY CODEX PROMPT RULE

All current and future prompts intended for Codex must be written entirely in English.

This V03 prompt is in English. During V03, add a permanent active rule to `AGENTS.md` stating that every Codex-facing prompt must be written entirely in English. Do not rewrite historical immutable prompts merely because they contain another language.

## Work-item identity

- Work code: `HVA-LWC-001`
- Version: `V03`
- Scope: bounded closure of the V02 strict-audit findings only
- Repository: `Sekiph82/H-veAI`
- Branch: `main`

Read first after synchronization:

- `AGENTS.md`
- `TASKS.md`
- `docs/H!veAI/audits/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V02_AUDIT.md`
- `docs/H!veAI/codex-logs/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V02_LOG.md`
- `docs/H!veAI/prompts/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V02_PROMPT.md`
- `docs/H!veAI/HVA-LWC-001_ARTIFACT_NAMING_GOVERNANCE_V01_GOVERNANCE.md`

Do not inspect or modify another GitHub repository as part of this work item. The preserved copies of other repositories are evidence/data-preservation subjects only. Do not develop, reset, rebase, merge, push, clean, or rewrite them.

## V03 objective

Close exactly these V02 strict-audit findings:

- `HVA-LWC-001-V02-F01`: durable preservation is still supported only by builder self-report;
- `HVA-LWC-001-V02-F02`: canonical root `TASKS.md` contains contradictory current M21 truth and actor/action mismatch;
- `HVA-LWC-001-V02-F03`: permanent English-only Codex-prompt governance must be encoded.

No UI redesign, product feature work, M17-M20 implementation, installer work, destructive parent cleanup, or unrelated repository remediation is allowed.

## 1. Create an independently inspectable read-only preservation verifier

Add a narrowly scoped PowerShell verifier:

`scripts/verify-hva-lwc-preservation.ps1`

The verifier exists only to prove the V01/V02 preservation copies. It is not a cleanup, migration, synchronization, or deletion utility.

### Required safety properties

The script must be read-only with respect to all source and destination preservation trees.

It must not:

- delete, move, rename, copy, overwrite, normalize, checkout, reset, clean, stash, commit, merge, rebase, push, fetch, or otherwise mutate preserved repositories/files;
- modify Git metadata;
- follow directory reparse points/junctions/symlinks in a way that can escape roots or create recursion loops;
- print file contents;
- commit or expose secrets/private file contents;
- require a hard-coded Windows username.

Resolve Windows known folders dynamically. The durable destination root must be derived from the user's Documents known folder, consistent with V02.

The script may locate the V01 source preservation roots only for verification. If a required source root has disappeared, it must fail clearly and produce a non-PASS result rather than silently validating destination-only data.

### Deterministic verification model

For every required V01 source/durable-destination pair, compute and compare at minimum:

- recursive regular-file count;
- recursive total byte count;
- deterministic cryptographic aggregate manifest digest.

The aggregate digest must be derived from every regular file, not merely representative samples.

For each regular file, derive a deterministic record from at least:

- repository/preservation-root-relative path;
- exact file length;
- SHA-256 of file contents.

Sort records deterministically using ordinal semantics before deriving the aggregate SHA-256 manifest digest.

Use a normalized relative-path separator inside the digest computation so the receipt is deterministic on the current Windows host. Preserve path identity; do not silently collapse distinct files.

Do not follow reparse points. If an inaccessible file or unsafe filesystem object prevents complete verification, fail the affected pair rather than ignoring it.

The verifier must compare source and destination aggregate manifest digests and report `PASS` only when count, bytes, and manifest digest all match.

### Privacy-safe committed receipt

The script must be able to write a compact JSON receipt to a caller-specified output path. Commit the resulting receipt at:

`docs/H!veAI/evidence/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V03_PRESERVATION_RECEIPT.json`

The committed receipt must include for each verified preservation class:

- stable logical source label;
- stable logical destination label;
- source file count;
- destination file count;
- source total bytes;
- destination total bytes;
- source aggregate SHA-256 manifest digest;
- destination aggregate SHA-256 manifest digest;
- count-match boolean;
- bytes-match boolean;
- manifest-match boolean;
- final result;
- verifier version/schema;
- verification timestamp;
- explicit statement that reparse points were not followed.

Do not include giant per-file filename inventories, private file contents, credentials, `.env` values, or machine-specific absolute paths in the committed receipt.

The receipt must cover at minimum the V02 preservation classes:

- `HVA-LWC-001-V01-parent-preservation`;
- `H-veAI-consolidation-retired-20260911`;
- `HVA-LWC-001-V01-deduplicated`.

The empty retired-loose source may be reported separately as an empty/non-content class, but do not manufacture a meaningful content digest for a nonexistent tree.

If full cryptographic verification cannot complete, V03 must report `PRESERVATION_BLOCKED`, not PASS.

## 2. Test the preservation verifier itself

Add focused tests for the verifier or an equivalent deterministic test harness that exercises the real verification logic against temporary fixture trees.

At minimum prove:

1. identical trees PASS;
2. same path and same size but different file contents FAIL due to SHA-256 mismatch;
3. missing file FAILS;
4. extra file FAILS;
5. same files with deterministic enumeration-order differences still produce the same aggregate digest;
6. reparse-point/symlink/junction behavior is rejected or safely skipped according to the documented no-follow contract, never recursively escaped;
7. receipt output contains no absolute test-root path if privacy-safe mode is used.

Do not use mocked comparison results in place of the actual digest logic.

## 3. Repair canonical root TASKS.md current truth

`TASKS.md` is the only current project-status tracker. Repair only its current operational truth; do not rewrite immutable historical audits/logs.

Current defects to fix:

### Contradictory M21 state

The active `Current truth` section currently contains both:

- wording that says M21 remains planned/not started; and
- wording that says M21 standalone migration / M21-R01 are PASS/CLOSED.

Remove the stale current-truth contradiction. Preserve accurate historical context without leaving mutually exclusive current-state claims.

### Required actor mismatch

The top Project Status says the next action is owner native/visual acceptance and independent retirement decision, but `Required Actor` is `CODEX`.

Make the required actor truthful. Use the repository's established human/owner semantics. Do not invent that Codex can perform the owner's acceptance/retirement decision.

### Current task semantics

Do not fabricate a completed owner acceptance. The tracker must clearly show:

- M21 standalone migration and M21-R01 remediation are implementation/audit closure states already reached where supported;
- owner native/visual acceptance and retirement decision remain pending if they have not actually occurred;
- no parent deletion is authorized merely by this task;
- the 20-roadmap-milestone completion denominator remains unchanged if that is still the active product accounting convention.

Keep the edit minimal and internally consistent.

## 4. Permanently encode English-only Codex prompts

Update active `AGENTS.md` with a concise permanent rule:

> Every prompt intended for Codex must be written entirely in English.

The rule must apply to future implementation, remediation, audit-follow-up, migration, cleanup, and governance prompts intended for Codex.

Do not rewrite historical prompt artifacts.

Do not require owner-facing conversation with ChatGPT to be English; this rule is specifically about Codex-facing prompt artifacts/instructions.

## 5. Re-verify V02 governance remains intact

Confirm after edits that `AGENTS.md` still requires:

- fetch/reconcile before reading the active prompt;
- no destructive reconciliation of dirty/divergent local work;
- commit/push before completion;
- final `HEAD == origin/main == ls-remote` verification;
- GitHub-first owner-facing completion reporting;
- repository-relative `docs/H!veAI/codex-logs/` path.

Do not weaken any V02 rule while adding the English-only rule.

## 6. Repository validation

At minimum:

- run `git diff --check`;
- run focused verifier tests;
- run the real preservation verifier against the V01 source and V02 durable preservation roots and generate the committed JSON receipt;
- independently confirm the receipt reports matching count/bytes/aggregate digests for every required non-empty pair;
- confirm `TASKS.md` has no contradictory M21 current-state statements;
- confirm the top Required Actor matches the pending owner action;
- confirm all Codex-facing prompts created by V03 are English;
- confirm V03 did not add preservation trees, private manifests, secrets, local databases, caches, or unrelated repository contents to H!veAI;
- confirm V01 legacy archive remains unchanged;
- confirm V02 audit/log/prompt historical artifacts remain immutable.

Because the verifier reads many files, do not turn a verification failure into a destructive repair action. Report the failure exactly.

## 7. Required V03 builder log

Create:

`docs/H!veAI/codex-logs/HVA-LWC-001_LOCAL_WORKSPACE_FINAL_CONSOLIDATION_V03_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- exact V03 changed-file scope;
- V02 finding-by-finding closure table;
- verifier design summary and safety constraints;
- focused verifier test results;
- real preservation verification result for each required class;
- GitHub path to the committed preservation receipt;
- receipt aggregate digests copied exactly from the generated artifact;
- `TASKS.md` before/after contradiction resolution summary;
- `AGENTS.md` English-only governance addition;
- confirmation that V02 sync-first/GitHub-first governance remains intact;
- `git diff --check` result;
- implementation commit SHA(s) known before log publication;
- explicit statement that the log's own commit SHA is verified only after publication and is not self-fabricated.

## 8. Required completion state

Report `COMPLETE` only if:

- every required non-empty preservation source/destination pair cryptographically verifies with count, bytes, and deterministic aggregate SHA-256 manifest equality;
- the committed privacy-safe receipt exists on GitHub;
- focused verifier tests pass;
- canonical `TASKS.md` current truth is internally consistent;
- Required Actor truthfully reflects the pending owner action;
- permanent English-only Codex-prompt governance exists in `AGENTS.md`;
- V02 sync-first/GitHub-first rules remain intact;
- all V03 repository changes are committed and pushed;
- local H!veAI `HEAD`, `origin/main`, and remote GitHub `main` are identical;
- H!veAI working tree is clean.

If preservation verification cannot be completed or does not match, report `PRESERVATION_BLOCKED`.

If GitHub synchronization cannot be completed safely, report `SYNC_BLOCKED`.

Do not claim PASS by relying only on the V03 builder log.

## 9. Final Codex response format

Keep the final Codex response short and GitHub-first.

Return only:

- status: `COMPLETE`, `PRESERVATION_BLOCKED`, or `SYNC_BLOCKED`;
- GitHub URL/path to the V03 log;
- GitHub URL/path to the V03 preservation receipt;
- GitHub URL/path to updated `AGENTS.md`;
- GitHub URL/path to updated `TASKS.md`;
- implementation commit SHA(s);
- V03 log commit SHA;
- exact final GitHub `main` SHA.

Do not list ordinary local changed-file paths or preservation directories in the final chat response unless required to explain a blocker.
