# M16T Codex-Only Audit Provider V03 Tracker-Truth Remediation

## MANDATORY SYNC-FIRST AND GITHUB-FIRST CONTRACT

Before reading or changing any repository file, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

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

After synchronization, read these current authorities before editing:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/prompts/M16T_CODEX_ONLY_AUDIT_PROVIDER_V02_PROMPT.md`
- `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V02_LOG.md`
- `docs/H!veAI/audits/M16T_CODEX_ONLY_AUDIT_PROVIDER_V02_AUDIT.md`
- `src-tauri/src/github_tracking.rs`

All Codex-facing work and the required builder log must be entirely in English.

All H!veAI repository changes made by this task must be committed and pushed before completion is claimed. At the end verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == live remote main` and a clean H!veAI working tree. Otherwise return `SYNC_BLOCKED`.

Owner-facing final response must contain only relevant GitHub H!veAI file URLs/paths, tracker-correction/log commit SHA(s), final GitHub `main` SHA, and concise status. Do not dump ordinary local changed-file paths.

---

## WORK ITEM

- Work code: `M16T`
- Version: `V03`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required builder log: `docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V03_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

This is a **tracker/governance-only remediation**. The V02 runtime/process implementation is accepted by the independent audit and must not be redesigned in this task.

The accepted architecture remains binding:

> H!veAI has no OpenAI API-key/direct-HTTP audit provider. The only production model-backed audit provider is the locally installed Codex CLI authenticated through Codex-managed ChatGPT login.

Do not activate M17 or implement Claude work.

---

# FINDING M16T-V03-F01 - MAJOR - CURRENT TASK MARKER FALSELY CLAIMS VALIDATED COMPLETION

## Current defect

Root `TASKS.md` currently contains a summary row equivalent to:

```markdown
- [x] M16T - Codex-only audit provider migration V02 (implementation complete; awaiting independent strict audit and owner native acceptance; M16 remains OPEN)
```

This contradicts the canonical legend in the same file:

```text
[x] validated complete
[~] active/in progress
```

It is also runtime-visible. `src-tauri/src/github_tracking.rs` treats `x`/`X` checklist markers as completed tasks, increments `completed_tasks`, and emits `TASK_COMPLETE` for the row.

Therefore M16T must not be `[x]` while independent acceptance and owner native acceptance remain pending.

## Required correction

At V03 builder completion, the current M16T row must remain active, normally:

```markdown
- [~] M16T - Codex-only audit provider migration V03 (... independent strict audit and owner native acceptance pending; M16 remains OPEN)
```

Use wording consistent with current repository style, but preserve these semantics:

- implementation/runtime remediation is complete;
- independent acceptance is not yet complete;
- owner native Codex acceptance is not yet complete;
- M16 remains OPEN;
- M17 remains NOT ACTIVATED/BLOCKED;
- M16T must not be counted as `TASK_COMPLETE` yet.

Do not change `src-tauri/src/github_tracking.rs` merely to accommodate an incorrect tracker marker. The parser semantics are correct; the tracker data is wrong.

---

# FINDING M16T-V03-F02 - MINOR - V02 IMMUTABLE LOG CONTAINS A MISTYPED IMPLEMENTATION SHA

## Current evidence defect

The immutable V02 builder log records:

`daa5aeec7f7c3bd15c59a64b378d88b3b683781`

That SHA does not exist.

The actual V02 implementation commit is:

`daa5aeeec7f7c3bd15c59a64b378d88b3b683781`

The V02 log commit is:

`911a6daba70786196354f80591c7b5b87e499544`

GitHub history proves the V02 log commit is directly parented by the actual implementation commit.

## Required handling

Do **not** rewrite or edit the immutable V02 log.

Instead:

1. record the corrected V02 implementation SHA in the V03 builder log;
2. explicitly state that V02 contained a one-character documentation typo and that repository ancestry identifies the actual commit;
3. ensure every new V03 SHA recorded in the V03 log is copied exactly from Git output;
4. after log publication, verify the log commit and final remote `main` SHA before returning.

---

# REQUIRED FINAL TRACKER STATE

After the V03 correction commit is complete, current/prospective truth in root `TASKS.md` must say:

- Current Milestone: `M16`;
- Current Sprint: `M16T-CODEX-ONLY`;
- Current Task: `M16T V03 - Codex-only audit provider tracker-truth remediation` or equivalent concise wording;
- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`;
- Next Task/Action: independent M16T V03 strict audit and owner native Codex acceptance;
- Required Actor: `HUMAN`;
- M16T summary row: `[~]`, not `[x]`;
- M16 remains `OPEN`;
- M17 remains `NOT ACTIVATED / BLOCKED`;
- milestone denominator remains `20`;
- strict completed milestone progress remains `16/20`;
- accepted M21 history remains unchanged.

Update `CODEX_ROADMAP.md` current/prospective wording only if needed to keep it aligned with the V03 state. Do not rewrite historical milestone notes merely to replace old version labels.

Do not mark M16T `[x]` until an independent audit has passed and the owner native Codex acceptance has actually been recorded.

---

# ABSOLUTE PROVIDER GUARDRAIL

V03 must not modify production provider code unless a genuinely new, independently reproducible production defect is discovered. The expected task is tracker/governance-only.

Do not reintroduce or use:

- `OPENAI_API_KEY`;
- direct H!veAI calls to `api.openai.com` for audit execution/readiness;
- OpenAI Responses API transport in H!veAI;
- API-key setup UI;
- API-key authentication as an accepted Codex audit mode;
- Codex auth-file/token inspection;
- ChatGPT desktop GUI automation;
- Claude/M17 implementation.

Do not change the accepted bounded Codex stream-drain/final-message implementation merely because this tracker task exists.

---

# REQUIRED VALIDATION

Because this is expected to be tracker/governance-only, do not manufacture unrelated source edits or expensive regression work.

At minimum:

1. inspect `src-tauri/src/github_tracking.rs::task_row` and the completed-count loop to confirm that `[x]` is runtime-visible `TASK_COMPLETE` truth;
2. verify the corrected M16T row is `[~]` and no current/prospective M16T row falsely uses `[x]` before acceptance;
3. verify top `TASKS.md` status/next-action/actor fields are internally consistent;
4. verify `CODEX_ROADMAP.md` current status is consistent with root `TASKS.md`;
5. verify M16 remains OPEN and M17 remains blocked;
6. run `git diff --check`;
7. verify no production source file changed unless a real new defect forced it;
8. verify no OpenAI API-key/direct HTTP audit provider was introduced;
9. verify the V03 log records the exact corrected V02 implementation SHA and exact V03 commit SHA(s).

If production source unexpectedly changes, stop and justify the defect before making the change, then run the relevant focused regressions required by repository policy.

No real Codex audit turn/quota is required by this tracker-only remediation. Owner-native Codex acceptance occurs after independent V03 audit PASS.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16T_CODEX_ONLY_AUDIT_PROVIDER_V03_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- exact tracker-correction commit SHA(s) known before log publication;
- the corrected V02 implementation SHA `daa5aeeec7f7c3bd15c59a64b378d88b3b683781`;
- statement that the immutable V02 log had a one-character SHA typo and was not rewritten;
- final M16T checkbox marker and why `[~]` is required until acceptance;
- confirmation that GitHub tracking parses `[x]` as completed and therefore the correction is runtime-visible truth;
- final top tracker fields;
- roadmap consistency result;
- `git diff --check` result;
- provider guardrail search result;
- statement that no production source/provider code changed, if that remains true;
- statement that M16 remains OPEN and M17 remains blocked.

Do not require the log file to contain the SHA of the commit that first creates itself. After publishing the log, verify its commit and final remote `main` and return those SHA values in the final response.

---

# COMPLETION GATE

Return `COMPLETE` only when:

- M16T is no longer falsely represented as validated complete;
- canonical current tracker and roadmap truth are consistent;
- the V02 SHA typo is prospectively corrected in V03 evidence without rewriting history;
- M16 remains OPEN;
- M17 remains blocked;
- GitHub synchronization is proven.

Return `SYNC_BLOCKED` if safe synchronization cannot be completed.

Final owner-facing response must show only:

- GitHub V03 log URL/path;
- tracker-correction commit SHA(s);
- V03 log commit SHA;
- final GitHub `main` SHA;
- concise status.
