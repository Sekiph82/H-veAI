# M18 GitHub Integration V01 Plan

## Purpose

M18 turns H!veAI's existing GitHub tracking foundations into a governed project-scoped GitHub integration for repository/branch/commit truth, pull requests, issues, Actions/CI, releases/tags, bounded cache/rate-limit/offline behavior, and local/remote reconciliation.

M17 is accepted for closure by `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`. M18 may therefore activate. M19-M20 remain blocked by dependency order.

## Existing source material

M18 must begin by auditing and reusing the current GitHub foundations rather than creating a parallel stack.

At minimum inspect:

- `src-tauri/src/github_tracking.rs` and its current remote TASKS/head/cache model;
- `repositories`, `github_sync_state`, project registry, and local Git Engine data;
- existing Command Center / Project Cockpit remote-health surfaces;
- current GitHub-related Tauri commands, ACL/capabilities, frontend DTOs, tests, and migrations;
- exact-eight-project portfolio and `Sekiph82/FormuLab@main` branch contract;
- existing rate-limit, stale-cache, offline, timeout, and child-process helpers where reusable.

Do not destroy the accepted GitHub TASKS-only tracking behavior. General GitHub integration must extend it without making current project/task truth less reliable.

## Authentication and transport decision

GitHub integration must be least privilege and truthful.

Builder must first recover the current accepted authentication/transport boundary from repository source and installed local tools before choosing an implementation. Do not blindly add a token setting or a second unrelated transport.

Requirements:

- never persist plaintext GitHub secrets in H!veAI SQLite, logs, source, prompts, or UI;
- never inspect credential files directly;
- prefer an existing owner-managed authenticated mechanism if one is already governed and sufficient;
- public read-only resources may use bounded unauthenticated reads only where truthful rate-limit/offline handling is explicit;
- remote mutation requires an authenticated least-privilege path plus explicit human confirmation at the mutation boundary;
- never infer that a mutation succeeded from local intent; persist remote identity/result evidence only after confirmed remote success;
- sanitize/redact provider responses before persistence/UI exposure.

## M18.01 Repository / branch / commit reads

Provide project-scoped GitHub repository truth derived from Registry repository identity.

At minimum:

- repository owner/name/default branch;
- canonical remote branch HEAD SHA;
- bounded branch list where useful;
- bounded recent commit metadata;
- local Git Engine branch/HEAD/upstream/ahead/behind alongside remote truth;
- explicit CURRENT / STALE / UNAVAILABLE / RATE_LIMITED / AUTH_REQUIRED or equivalent health classification;
- no silent fallback that presents stale remote data as current.

Existing remote TASKS tracking must continue to work and should share/cache repository-head truth where practical.

## M18.02 Pull requests

Read project-relevant pull requests with bounded metadata and evidence.

At minimum support:

- open/closed/merged status;
- source/base branches and head/base SHAs;
- title/number/author/timestamps;
- bounded changed-file/diff summary and review/check status where available;
- bounded comments/reviews where supported by the chosen transport;
- explicit project/task/session links only when evidence exists.

Permission-gated PR creation may be added only behind explicit human confirmation. Creating a PR must never auto-commit, auto-push, rewrite local branches, or discard owner work.

## M18.03 Issues

Read project-relevant issues with bounded metadata.

At minimum:

- number/title/state/labels/author/timestamps;
- bounded body/comment evidence where useful;
- explicit task linkage only from deterministic references or an owner action;
- no semantic guessing that an issue owns a task merely because titles resemble each other.

Any issue mutation must be separately permission-gated and human-confirmed if implemented.

## M18.04 GitHub Actions / CI

Provide truthful CI evidence for project commits/PRs.

At minimum:

- workflow run identity/status/conclusion/event/branch/SHA;
- bounded jobs/steps summaries;
- bounded failed-log evidence where transport supports it;
- map runs to the exact commit/PR identity;
- distinguish no CI from unavailable CI;
- surface failed, cancelled, timed-out, queued, in-progress, and successful truth distinctly.

A workflow retry, if implemented, must be explicit human-confirmed remote mutation. Never loop retries automatically.

## M18.05 Releases / tags

Provide bounded release/tag context:

- latest releases;
- draft/prerelease/published status;
- tag/target commit where available;
- publication timestamps and release URL/identity;
- bounded tags when useful for local/remote reconciliation.

Do not create releases/tags in M18 unless a narrow explicitly approved mutation path is justified and tested. Read support is mandatory; creation is optional and permission-gated.

## M18.06 Cache / rate limits / offline behavior

Extend `github_sync_state` or another narrow governed persistence model rather than creating uncontrolled caches.

Required behavior:

- bounded response/cache size;
- resource-specific cache identity and schema/version awareness;
- fetched-at / last-known-good / remote-health truth;
- stale cache may be shown only as STALE with age/provenance;
- rate-limit exhaustion classified separately from generic network errors;
- offline/network failure must not erase the last known good snapshot;
- malformed remote payload must not overwrite good truth as successful current data;
- bounded timeout and concurrency behavior;
- no unbounded polling or request storms.

Selected-project vs portfolio refresh cadence must remain bounded and should reuse current remote-observation scheduling where practical.

## M18.07 Local / remote reconciliation

Build one project-scoped reconciliation model combining local Git Engine and GitHub remote truth.

At minimum classify:

- local HEAD equals tracked remote HEAD;
- local ahead;
- local behind;
- diverged;
- detached/no-upstream/unknown;
- remote branch missing;
- stale remote evidence;
- local dirty state separately from commit divergence.

Never fetch/merge/rebase/reset/checkout/push merely to make states agree. M18 reconciliation is observational by default.

Any future remote/local mutation must remain explicit and human-confirmed.

## M18.08 Security / permission boundary

Required invariants:

- Registry repository identity is authoritative for project scope;
- no arbitrary repository URL/owner/repo injection from untrusted frontend text;
- no arbitrary API URL construction outside approved GitHub hosts/endpoints;
- no shell command-string construction;
- no secrets in logs/cache/UI;
- bounded response sizes and pagination;
- remote mutation default-denied;
- explicit human confirmation for PR creation or workflow retry if those mutations are implemented;
- no automatic push, merge, branch rewrite, tag creation, release creation, issue mutation, or workflow retry;
- project/task/session mappings are evidence-backed;
- stale/offline/auth/rate-limit truth is fail-closed.

## M18.09 UI / tests / publication / closure

Integrate GitHub truth into the existing H!veAI visual language without creating a separate app-within-an-app.

At minimum provide useful project-scoped GitHub visibility in Project Cockpit and/or a governed GitHub panel reachable from existing project context:

- repository/branch/commit status;
- local/remote reconciliation;
- PRs;
- issues;
- CI/Actions;
- releases/tags;
- stale/offline/rate-limit/auth diagnostics.

Do not redesign global navigation unless necessary. Preserve dashboard layout governance.

Required deterministic evidence must include at least:

- repository/branch/commit fixtures;
- local-vs-remote equal/ahead/behind/diverged fixtures;
- PR metadata/diff/review/check fixtures;
- issue/task explicit-link and no-guess behavior;
- Actions status/conclusion and failed-log fixtures;
- releases/tags fixtures;
- rate-limit/offline/stale-cache behavior;
- malformed payload and timeout behavior;
- pagination/size bounds;
- permission-gated mutation denial/confirmation tests for every mutation actually implemented;
- secret redaction;
- cross-project isolation;
- existing GitHub TASKS tracking regression;
- exact eight-project portfolio and `Sekiph82/FormuLab@main` regression;
- local Git Engine regression;
- M13-M17 agent/audit regressions relevant to project identity;
- full Rust/frontend/typecheck/build regression;
- governed native publication;
- independent 20-section strict audit;
- owner-native visual/operational acceptance where UI or mutation controls require it.

## Tracker transition

The first M18 builder action must reconcile canonical tracker truth because the owner acceptance record closes M17 after the last source build.

Prospective truth at M18 start:

- Current Milestone: M18;
- Current Sprint: M18-GITHUB-INTEGRATION;
- Current Task: M18 V01 — GitHub Integration;
- Current Task Status: IMPLEMENTATION_IN_PROGRESS;
- Required Actor: CODEX;
- M17: PASS/CLOSED;
- M17 V05: source strict re-audit PASS;
- M17 owner-native acceptance: PASS, with the live Stop/Resume subflow quota-blocked but explicitly owner-waived after truthful weekly-limit handling;
- strict completed milestone count: 18/20 = 90%;
- M18: `[~]` active;
- M19-M20: planned/blocked.

At builder completion M18 remains `[~]` and becomes `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`. Do not mark M18 `[x]` before independent audit plus any required owner acceptance.

## Closure

M18 closes only when repository truth, local/remote reconciliation, PR/issues/Actions/releases visibility, cache/rate-limit/offline behavior, security boundaries, deterministic tests, governed publication, independent strict audit, and required owner-native acceptance all pass.

Only then may M19 activate.
