# M18 GitHub Integration V01 — Whole Milestone Builder Prompt

## MANDATORY SYNC-FIRST / GITHUB-FIRST CONTRACT

Before reading or changing any repository file, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the worktree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never reset, rebase, force-push, destructively checkout, automatically stash, run `git clean`, or discard/reconcile owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED` and report the exact evidence.

Then read at minimum:

- `AGENTS.md`
- `CONSTITUTION.md`
- `ARCHITECTURE.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`
- `docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_V05_STRICT_REAUDIT.md`
- current `src-tauri/src/github_tracking.rs`
- current Registry, Local Git Engine, GitHub sync/cache schema/migrations, Tauri commands/ACL, Project Cockpit/Command Center GitHub surfaces, and focused tests.

Every Codex-facing artifact and builder log must be entirely in English.

All H!veAI repository changes must be committed and pushed before completion. Final completion requires:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Local `HEAD`, `origin/main`, and live GitHub `main` must be identical and the worktree must be clean.

---

## WORK ITEM

- Work code: `M18`
- Version: `V01`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Plan: `docs/H!veAI/plans/M18_GITHUB_INTEGRATION_V01_PLAN.md`
- Prior accepted milestone: M17 PASS/CLOSED by owner-native final acceptance
- Required log: `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V01_LOG.md`
- Independent audit after builder completion: ChatGPT, not Codex

Execute the entire M18 milestone. Do not activate M19.

---

# FIRST ACTION — RECONCILE CANONICAL TRACKER TRUTH

Before substantive source changes, update `TASKS.md` and `CODEX_ROADMAP.md` so prospective truth becomes:

- Current Milestone: `M18`
- Current Sprint: `M18-GITHUB-INTEGRATION`
- Current Task: `M18 V01 — GitHub Integration`
- Current Task Status: `IMPLEMENTATION_IN_PROGRESS`
- Required Actor: `CODEX`
- M17: `PASS/CLOSED`
- M17 V05 independent strict re-audit: PASS
- M17 owner-native acceptance: PASS
- The owner explicitly accepted closure despite the live Start -> Stop -> Resume smoke path being blocked by an external Claude weekly quota; the application truthfully surfaced that provider condition.
- Strict completed milestone count: `18 / 20 = 90%`
- M18: `[~]` ACTIVE
- M19-M20: planned/blocked.

Reference:

`docs/H!veAI/audits/M17_CLAUDE_CODE_ADAPTER_OWNER_NATIVE_FINAL_ACCEPTANCE_V01_AUDIT.md`

Commit and push this tracker transition before substantive M18 implementation.

At builder completion, if and only if all M18 implementation gates pass, transition to:

- Current Task Status: `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`
- Required Actor: `HUMAN`
- M18 remains `[~]`
- M19 remains blocked.

Do not mark M18 `[x]`.

---

# ABSOLUTE PRESERVATION / SECURITY GUARDRAILS

Preserve all accepted M00-M17 behavior, especially:

- standalone `Sekiph82/H-veAI` root and GitHub TASKS-only canonical tracker model;
- exact eight-project portfolio;
- `Sekiph82/FormuLab@main` tracking;
- Registry project identity and local workspace repair semantics;
- Local Git Engine read-only-by-default safety;
- watcher/snapshot/task/workflow truth;
- Command Center and Project Cockpit accepted behavior;
- Codex adapter and Claude Code adapter;
- M16 Codex-only audit provider;
- governed `dev-bin/H!veAI.exe` publication and Desktop shortcut;
- historical prompt/log/audit immutability.

GitHub integration guardrails:

- never persist plaintext GitHub secrets in SQLite, logs, UI, prompts, or source;
- never read GitHub credential files directly;
- do not invent an API-key/PAT setting unless existing architecture and explicit milestone requirements prove it necessary; recover the current accepted transport/auth boundary first;
- never build arbitrary shell command strings;
- never accept arbitrary frontend-controlled GitHub hosts/endpoints;
- Registry repository identity is authoritative for project scope;
- all remote mutation is default-denied;
- PR creation and Actions retry, if implemented, require explicit human confirmation at the mutation boundary;
- do not auto-push, auto-merge, auto-rebase, auto-reset, auto-checkout, auto-tag, auto-release, mutate issues, or retry workflows without an explicit governed human action;
- never erase local work to reconcile local/remote state;
- stale, offline, auth-required, rate-limited, malformed, and unavailable remote states must remain distinct and truthful;
- sanitize/redact remote error/output evidence before persistence/UI exposure;
- enforce bounded pagination, response sizes, concurrency, polling, and cache growth;
- M19 must remain NOT ACTIVATED.

---

# M18.01 — REPOSITORY / BRANCH / COMMIT TRUTH

Extend the existing GitHub tracking foundation instead of creating a second unrelated remote stack.

Implement project-scoped remote truth including at minimum:

1. repository owner/name and canonical default/tracked branch;
2. canonical remote branch HEAD SHA;
3. bounded branch metadata where useful;
4. bounded recent commit metadata;
5. explicit fetched-at/provenance/remote-health truth;
6. CURRENT / STALE / UNAVAILABLE / RATE_LIMITED / AUTH_REQUIRED or equivalent explicit classes;
7. reuse of existing remote HEAD/cache evidence where safe;
8. no regression to root `TASKS.md` remote tracking.

Remote branch/head data must reconcile with, not replace, Local Git Engine truth.

---

# M18.02 — PULL REQUESTS

Implement bounded project-scoped PR inspection.

At minimum expose:

- PR number/title/state/draft/merged truth;
- author/timestamps;
- base/head branches and SHAs;
- bounded changed-file/diff summary;
- review/check status where available;
- bounded comments/reviews where supported;
- deterministic project linkage;
- task/session linkage only from explicit evidence.

Implement permission-gated PR creation only if the chosen authenticated boundary supports it safely. Any PR-create UI/action must require explicit human confirmation and must not commit, push, switch branches, or alter local work automatically.

If safe authenticated mutation support is not available in the existing environment, keep PR creation truthfully unavailable rather than adding an unsafe credential system.

---

# M18.03 — ISSUES

Implement bounded project-relevant issue reads:

- number/title/state/labels/author/timestamps;
- bounded body/comment evidence where useful;
- deterministic repository scope;
- explicit task linkage from IDs/references or owner action only;
- no title-similarity ownership guessing.

Issue mutation is not mandatory. If added, it must use the same human-confirmed least-privilege mutation boundary.

---

# M18.04 — GITHUB ACTIONS / CI

Implement bounded CI inspection:

- workflow run identity/name/event/status/conclusion;
- branch/SHA/PR association;
- bounded jobs/steps summaries;
- bounded failed-log evidence where transport supports it;
- distinguish no CI from CI unavailable;
- distinguish queued/in-progress/success/failure/cancelled/timed-out/skipped truth.

If workflow retry is supported, require explicit human confirmation and retry only the selected governed run. Never retry in a loop.

---

# M18.05 — RELEASES / TAGS

Implement project-scoped read support for:

- bounded releases;
- latest/published/draft/prerelease status;
- tag name and target identity where available;
- publication metadata;
- bounded tag history useful for reconciliation.

Release/tag creation is not required. Do not add it merely to expand scope.

---

# M18.06 — CACHE / RATE LIMIT / OFFLINE

Extend the existing governed GitHub cache model narrowly.

Required behavior:

- resource-specific cache keys/kinds;
- bounded payloads and pagination;
- fetched-at and last-known-good truth;
- stale cache displayed only as STALE with age/provenance;
- rate-limit classification distinct from network/offline/auth/malformed failures;
- network/offline failures preserve last-known-good data without labeling it current;
- malformed responses cannot replace last-known-good truth as successful current data;
- bounded timeout/concurrency/refresh cadence;
- no request storms;
- branch identity changes invalidate incompatible resource caches;
- migration/backward-compatibility tests for any schema change.

---

# M18.07 — LOCAL / REMOTE RECONCILIATION

Create one deterministic project-level reconciliation model combining Local Git Engine and GitHub remote evidence.

At minimum classify:

- EQUAL / SYNCED;
- LOCAL_AHEAD;
- LOCAL_BEHIND;
- DIVERGED;
- DETACHED_OR_NO_UPSTREAM;
- REMOTE_BRANCH_MISSING;
- REMOTE_STALE;
- REMOTE_UNAVAILABLE;
- UNKNOWN where evidence is insufficient.

Keep local dirty state separate from commit divergence.

This milestone is observational by default. Do not fetch/merge/rebase/reset/checkout/push merely to make state agree.

Expose enough evidence to explain the classification: local branch, local HEAD, upstream/ahead/behind where available, remote branch, remote HEAD, remote fetched-at/health.

---

# M18.08 — SECURITY / PERMISSION BOUNDARY

Add deterministic evidence proving:

- cross-project repository access is rejected;
- frontend cannot redirect a project operation to an arbitrary repo/host;
- response/pagination/body sizes are bounded;
- secrets/tokens are redacted before persistence/UI;
- remote mutation functions are absent or default-denied without explicit confirmation;
- confirmation cannot be reused across a different repository/action/identity;
- no local destructive Git mutation occurs through M18;
- rate-limit/offline/auth/malformed states fail closed;
- GitHub TASKS tracking and current eight-project portfolio remain unchanged.

---

# M18.09 — UI / TESTS / PUBLICATION

Integrate GitHub truth into existing H!veAI project context without redesigning the global shell.

Provide a coherent Project Cockpit GitHub experience, or an equivalent existing project-scoped surface, showing at minimum:

- repository and remote branch/HEAD;
- local/remote reconciliation;
- recent PRs;
- issues;
- Actions/CI;
- releases/tags;
- remote health/staleness/rate-limit/auth/offline diagnostics;
- explicit mutation controls only where safely supported.

Follow `docs/H!veAI/UI_LAYOUT_GOVERNANCE.md` for every visual change.

Do not overload the one-screen shell with raw API payloads. Prefer bounded dense cards/tables/drawers consistent with current H!veAI design.

---

# REQUIRED DETERMINISTIC TEST MATRIX

At minimum add or preserve deterministic coverage for:

1. repository/default-branch/remote-HEAD reads;
2. branch and bounded commit metadata;
3. local/remote EQUAL, AHEAD, BEHIND, DIVERGED;
4. detached/no-upstream and missing remote branch;
5. PR open/closed/merged/draft and bounded diff/review/check evidence;
6. explicit task/session link and no-guess behavior;
7. issue read and no implicit title-match ownership;
8. Actions queued/running/success/failure/cancelled/timed-out and no-CI vs unavailable-CI;
9. bounded failed-log evidence;
10. releases/draft/prerelease/tags;
11. rate-limit, offline/network, auth-required, timeout, malformed response;
12. last-known-good cache retained as STALE;
13. cache invalidation on tracked branch change;
14. pagination/response/cache/concurrency bounds;
15. secret/token redaction;
16. cross-project/host containment;
17. remote mutation denied without confirmation;
18. confirmation identity binding for every mutation implemented;
19. existing root GitHub TASKS tracking regression;
20. exact eight-project portfolio regression;
21. `Sekiph82/FormuLab@main` regression;
22. Local Git Engine regression;
23. M13-M17 provider/session/audit regressions relevant to project identity;
24. Project Cockpit/Command Center frontend regressions;
25. focused M18 mounted frontend tests;
26. full Rust library tests;
27. full frontend Vitest tests;
28. `npm run typecheck`;
29. `cargo check --manifest-path src-tauri/Cargo.toml`;
30. `npm run build`;
31. `git diff --check`.

Use mocked/fixture remote responses for deterministic tests. Do not consume external GitHub mutation quota or create real PRs/issues/workflow retries merely to satisfy automated tests.

---

# BUILDER CLAIMS / CI TRUTH

Record exact test commands/results in the builder log, but do not call builder output independent acceptance evidence.

If GitHub Actions status checks are absent on the implementation commit, state that explicitly.

If external live GitHub behavior cannot be exercised safely, keep it UNVERIFIED for owner/native acceptance rather than fabricating PASS.

---

# NATIVE QA PUBLICATION

If and only if implementation/regression gates pass:

- publish with the governed `scripts/publish-dev-qa.ps1` production `--no-bundle` path;
- keep `dev-bin/H!veAI.exe` stable;
- keep the Desktop `H!veAI.lnk` target stable;
- keep `dev-bin/H!veAI.ico` contract stable;
- record executable SHA-256;
- verify no standalone browser or visible console is introduced;
- do not activate M19.

---

# REQUIRED BUILDER LOG

Create:

`docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V01_LOG.md`

Record at minimum:

- starting synchronized SHA;
- tracker-transition SHA;
- implementation/test SHA(s);
- exact files changed;
- recovered GitHub authentication/transport architecture and why it is least privilege;
- schema/migration changes;
- implementation summary for M18.01-M18.09;
- remote mutation surface actually implemented, if any;
- deterministic test names/results;
- full regression/build/security results;
- GitHub status-check availability on implementation commit;
- native QA publication result and executable SHA-256;
- final tracker state;
- final local HEAD / `origin/main` / live GitHub main equality and clean-worktree evidence.

The log is builder evidence, not independent acceptance.

---

# FINAL COMPLETION CONTRACT

Before saying COMPLETE:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

All three main SHAs must match and the worktree must be clean.

Final owner-facing response must contain only:

- GitHub URL/path for `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V01_LOG.md`;
- tracker-transition SHA;
- implementation/test SHA(s);
- builder-log SHA;
- final GitHub `main` SHA;
- concise status.

Do not activate M19 and do not claim M18 PASS/CLOSED. Independent strict audit and owner-native acceptance remain separate gates.
