# H!veAI Canonical Tracker Transition Ownership V01

## Purpose

This governance note defines who is allowed to change canonical roadmap/task status after builder work, independent audits, and owner-native acceptance.

## Canonical rule

`TASKS.md` is the canonical detailed project-status tracker. `CODEX_ROADMAP.md` mirrors milestone-level status.

Canonical status transitions are owned by ChatGPT acting as the independent auditor/owner-facing coordinator, not by Codex acting as the implementation builder.

From this governance revision forward:

- Codex MAY read `TASKS.md` and `CODEX_ROADMAP.md` as source-of-truth inputs.
- Codex MUST NOT edit, rewrite, tick, close, activate, or otherwise mutate `TASKS.md` or `CODEX_ROADMAP.md` as part of implementation or remediation work.
- Codex MUST NOT mark its own work PASS/CLOSED or activate the next milestone.
- Builder logs MAY report implementation-complete claims and proposed next state, but they do not change canonical tracker truth.
- ChatGPT updates canonical tracker truth only after independently reviewing builder evidence/source and, where required, receiving owner-native acceptance.
- After ChatGPT pushes a tracker transition to GitHub `main`, Codex synchronizes its local standalone checkout from GitHub using the safe sync-first contract and treats that GitHub tracker state as authoritative.
- If Codex finds a local tracker different from GitHub `main`, it must resolve only by safe synchronization when the worktree is clean and strictly behind. It must never invent or independently author a tracker transition.
- Historical prompt/log/audit artifacts remain immutable.

## Builder sync-only responsibility

At the beginning and end of builder work, Codex is responsible for repository synchronization truth only:

1. fetch `origin/main`;
2. inspect worktree and divergence;
3. fast-forward only when safe;
4. never reset, rebase, force-push, automatically stash, run `git clean`, destructively checkout, or discard owner work;
5. commit/push implementation and the required builder log;
6. verify local HEAD = `origin/main` = live GitHub `main` and a clean worktree before reporting completion.

Codex does not own canonical tracker transitions.

## Audit/acceptance responsibility

ChatGPT owns:

- strict-audit verdict recording;
- owner-native acceptance recording;
- milestone PASS/CLOSED decisions;
- activation of the next milestone;
- root `TASKS.md` status updates;
- corresponding `CODEX_ROADMAP.md` status updates;
- preparation of the next authoritative builder prompt after tracker truth has been updated.

## Effective scope

This rule applies immediately to post-M17 work, including the pre-M18 audit-provider hotfix and M18-M20.

Any older builder prompt that instructs Codex to mutate canonical tracker files is superseded on this point by this governance note and must not be executed without an updated prompt that treats those files as read-only.
