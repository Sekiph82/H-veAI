# M19 Next Best Task AI + Engineering Brief V07 R03 — Narrow Remote Authority and Fairness Remediation

## 0. Scope

Perform only the narrow remediation required by:

`docs/H!veAI/audits/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_R02_STRICT_REAUDIT.md`

Close:

- `F-M19-V07R02-STRICT-006`
- `F-M19-V07R02-STRICT-007`
- `F-M19-V07R02-STRICT-008`

Do not redesign accepted M19 work. Keep root `TASKS.md` and `CODEX_ROADMAP.md` read-only. Do not start M20 and do not self-close M19.

Before editing, safely synchronize local standalone H!veAI with GitHub `main` using fetch + fast-forward only when safe. No reset, rebase, force-push, clean, auto-stash, discard, or overwrite of owner work.

Read the V07 R02 prompt, R02 builder log, R02 lifecycle matrix, and R02 strict re-audit in full.

## 1. Fix production root-tracking fairness

The current production scheduler rotates the project list and then destroys that rotation by sorting again on `project.id`.

Required semantics:

- pending/manual refresh projects receive bounded priority;
- within each priority class, preserve the already-rotated order;
- repeated constrained portfolio windows must advance service across projects;
- under 20 projects and a 40-stage tracking cap, the same alphabetic subset must not permanently consume every window;
- budget-wait/failure timing must not create permanent starvation.

Use a stable partition or another explicit fair scheduling structure. Do not satisfy this with a detached helper.

Mandatory production-path tests:

1. 20 changed-HEAD projects, constrained 40-stage budget, repeated windows -> eventual service reaches every project;
2. a later-alphabet project cannot remain permanently budget-waited;
3. manual pending project gets priority without resetting/poisoning background rotation;
4. 8/9/10 project regressions remain bounded;
5. selected cadence and monotonic backoff remain unchanged.

## 2. Pin root TASKS to the observed immutable HEAD

Current changed-HEAD observation can read HEAD `H1` and then read `TASKS.md` through a moving branch ref.

Required contract:

- the authoritative TASKS bytes used to create task rows must be from the exact immutable revision represented by persisted `remote_head`;
- preferred implementation: fetch raw TASKS using the observed commit SHA `H1`;
- if another protocol is used, it must revalidate the revision and fail closed/retry on a branch movement;
- do not persist mixed H1/H2 content as CURRENT;
- `tasks_blob_sha`, task rows/content hashes, and `remote_head` must describe one revision;
- secondary commit metadata must not be allowed to falsify that authority relationship.

Mandatory deterministic tests:

1. observed HEAD H1 + TASKS for H1 -> CURRENT and provenance all H1;
2. branch advances to H2 between observations -> H1 snapshot still contains H1 TASKS if commit-pinned;
3. if exact H1 TASKS cannot be obtained -> fail closed, preserve last-good as degraded/stale as appropriate;
4. no mixed-revision CURRENT snapshot;
5. same-HEAD reuse and changed-HEAD validation timestamp semantics remain intact.

A small injectable tracking HTTP seam is acceptable if needed for deterministic race tests. Keep it narrow.

## 3. Verification

Run and record at minimum:

- `npm run typecheck`
- `npm run build`
- frontend tests single-worker and default
- existing real React Projects geometry gate
- `cargo check --manifest-path src-tauri/Cargo.toml`
- focused root-tracking fairness tests
- focused immutable-HEAD TASKS provenance/race tests
- V07 R02 lifecycle/generation/admission/deadline tests
- next_best_task freshness/history tests
- 8/9/10/20 tracking request-budget tests
- Navigation detailed-integration rotation regression
- 403/429/coalescing regressions
- duplicate-ID/dependency/Registry regressions
- exact M16 observational-read regression
- full Rust lib suite
- changed Rust rustfmt check
- `git diff --check`
- accepted publication helper and stable EXE SHA-256

Create immutable builder log:

`docs/H!veAI/codex-logs/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_R03_LOG.md`

The log must include exact start SHA, implementation SHA, changed files, direct disposition of findings 006-008, repeated-window fairness evidence, immutable-HEAD provenance race evidence, all test results, publication SHA-256, trackers unchanged, M20 not started, owner-native acceptance pending.

## 4. Stop gate

After implementation and tests:

1. verify trackers unchanged and M20 untouched;
2. commit implementation;
3. create/commit immutable V07 R03 log;
4. push non-force to `origin/main`;
5. verify clean worktree and `HEAD == origin/main == live GitHub main`;
6. stop for independent audit.

Do not perform or claim owner-native acceptance.
