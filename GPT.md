# GPT.md — H!veAI Collaboration and Audit Rules

This file records durable working decisions for ChatGPT/GPT-assisted H!veAI development.

## 1. Whole-milestone rule

- Milestone subpackages such as M16.01, M16.02, etc. are for traceability only.
- When the user asks for a milestone implementation prompt, produce one unified whole-milestone prompt unless a genuine blocker requires a later remediation.
- Codex must not stop after each numbered subpackage.

## 2. Strict audit rule

- Independent audits must inspect the whole active milestone as one system, not stop after finding the first defect.
- Before creating a remediation prompt, perform a comprehensive pass across:
  - production source;
  - DB/migrations;
  - native command/ACL boundaries;
  - UI/native workflow;
  - direct test bodies;
  - Git/provenance;
  - state machines;
  - persistence/history;
  - security/containment;
  - degraded/error paths;
  - cross-feature regressions.
- Collect all reasonably discoverable BLOCKER/MAJOR/MINOR findings in that pass.
- Do not intentionally create one remediation cycle per finding.
- A remediation prompt should close all findings from that comprehensive audit in one continuous run.
- After remediation, re-audit the entire milestone again, not only the previously named finding.

## 3. Builder logs are claims

- `docs/H!veAI/codex-logs/` contains builder claims/evidence, not independent acceptance.
- Always inspect actual source, actual diff/commit, and direct tests before accepting a builder claim.

## 4. Authoritative artifact workflow

- Authoritative Codex prompts: `docs/H!veAI/prompts/`
- Immutable builder logs: `docs/H!veAI/codex-logs/`
- Independent audits: `docs/H!veAI/audits/`
- Historical failed prompts/logs/audits remain immutable.
- If a prompt is superseded before execution, create a new REV/superseding prompt and explicitly mark the older prompt superseded. Do not rewrite history.

## 5. User handoff format

When giving the user a Codex task:
1. save the authoritative prompt to GitHub first;
2. provide the GitHub URL;
3. provide only a short English Codex starter prompt in chat.

Do not paste the whole authoritative prompt into chat unless the user asks.

## 6. Native/visual acceptance

- Automated tests do not replace user native/visual acceptance where a milestone requires it.
- The user is the final authority for visual/native acceptance.
- Do not require the user to repeat a native test they explicitly declined if an accepted automated invariant already covers it.

## 7. Roadmap boundaries

- Do not activate the next milestone until the active milestone passes independent strict audit and required user acceptance.
- Do not start M21 before the numbered M00-M20 roadmap is complete.
- M21 is outside the user-facing 20-milestone denominator.

## 8. Anti-regression checklist for ChatGPT

Before answering `log geldi` or a milestone implementation request:
- read this file;
- read the active milestone prompt;
- read the latest builder log;
- read prior audits/remediations for the active milestone;
- inspect actual implementation source and direct tests;
- perform a whole-milestone adversarial sweep;
- only then write one consolidated audit/remediation artifact.

The purpose of this file is to prevent drift back into fragmented one-finding-at-a-time workflows.

## Owner-locked product priority — GitHub-first portfolio tracking

Effective 2026-09-09. Supersedes the earlier local-first tracking wording.

H!veAI's primary product job is:

> Track the owner's GitHub projects from one desktop app, show the exact current milestone/task/next action/progress from each repository's canonical GitHub tracking file, and let the owner resume the correct project from that state.

This product rule overrides architecture drift toward local-worktree authority, historical-document inference, or audit-first complexity.

Mandatory rules:

1. **GitHub tracked branch is the project-truth authority.** H!veAI must read project tracking state from the configured GitHub repository and tracked branch, not from the local working tree.
2. Local folders are execution workspaces only. Local dirty/clean status, unpushed files, local STATE/HANDOFF files, or local heuristics must never override the GitHub project state shown in Command Center or Project Cockpit.
3. H!veAI must never display a current milestone/task/next action/progress derived from arbitrary legacy prose, the first unchecked checkbox, stale dashboard files, provider handoff prose, or local fallback.
4. All eight tracked repositories must use one identical GitHub tracking contract.
5. The canonical operational truth should be intentionally small and non-duplicated. Prefer one canonical task tracker over several competing STATE/HANDOFF/dashboard projections.
6. Every Codex/Claude/ChatGPT state-changing run must update the canonical tracking file, append the canonical event when applicable, **commit and push those tracking changes to the tracked GitHub branch before yielding final completion** unless the run is explicitly blocked from pushing.
7. H!veAI refreshes GitHub state automatically. GitHub API/remote branch data is primary; a local filesystem watcher is not the project-truth mechanism.
8. If GitHub is temporarily unavailable, H!veAI may show the last successful GitHub snapshot with a visible stale/offline timestamp. It must not silently substitute local worktree state.
9. Project Cockpit may expose local workspace telemetry only as secondary technical information. It must not affect current task, milestone, next action, progress, required actor, health, or workflow truth.
10. A stale task, stale milestone, wrong percentage, unsupported-schema warning against the canonical remote file, or NEEDS_RECONCILIATION caused only by H!veAI's own tracker drift is a product failure.
11. Before declaring a tracking milestone complete, verify all eight actual GitHub target branches and the native H!veAI display against them. Synthetic fixtures alone are insufficient.
12. Case-variant provider instruction files must not be duplicated. Preserve one actual Claude instruction filename per repo and one Codex/AGENTS adapter, both pointing to the same provider-neutral H!veAI rules.
13. When the owner asks to normalize all projects, treat it as a cross-repository **GitHub control-plane migration**, not a local-only reconciliation exercise.
14. Do not activate the next roadmap milestone while the eight GitHub projects are not accurately resumable from H!veAI.
