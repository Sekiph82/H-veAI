# Urgent PROJECT.json Runtime Removal — Independent Strict Audit

## VERDICT

**CONDITIONAL / CHANGES_REQUIRED**

The critical legacy `.hiveai/PROJECT.json` production dependency appears closed in the committed H-veAI source, and the nine-project persisted-state convergence is materially implemented. However, one portfolio-level task-truth defect remains before this can be considered fully accepted: `Sekiph82/AI-Commerce-HQ@H!veAI` does not yet conform to the TASKS.md shape that the new root-task parser uses for counts/current-state fields.

Owner native/visual acceptance also remains required.

## What was verified

### 1. Legacy PROJECT.json production path

PASS at source level.

The urgent implementation commit `0b5e155977b8a190c4039c6c6c24b8bce99e8803` routes GitHub-tracked projects to remote GitHub/root-TASKS snapshots before the former local control-plane/truth paths can run. Production-facing control-plane and truth resolver functions now short-circuit for `GITHUB_TASKS_ONLY` projects.

A code search of the current default branch returned no active `PROJECT.json` matches in the indexed source.

### 2. Portfolio corrected to nine projects

PASS at source level.

`ensure_portfolio` now defines nine canonical GitHub projects and explicitly includes both:

- `Sekiph82/H-veAI@main`
- `Sekiph82/AI-Commerce-HQ@H!veAI`

The implementation archives non-portfolio active persisted rows and deletes obsolete GitHub sync resource rows, so prior persisted state can converge without asking the owner to delete the local database manually.

### 3. Persisted-state migration

PASS at implementation level, native evidence claimed by builder.

The code adds existing-state convergence and regression tests. The builder log reports `ACTIVE_COUNT=9`, `REMOTE_CACHE_ROWS=9`, and `LEGACY_CACHE_ROWS=0` from the real application state.

### 4. Hidden/background GitHub observation

PASS at source/log level.

The remediation addresses the pipe-buffer deadlock by draining hidden HTTP child output concurrently and retains bounded hidden background execution. The builder reports no visible terminal children during native polling.

### 5. TASKS status-block compatibility

PASS for the specific failure that triggered `0c5324155d4097400d7d3a29146d36727726f772`.

The parser no longer requires every valid status-block current task to also be duplicated as a checklist row.

## Open finding

### UPRR-A01 — MAJOR — AI-Commerce-HQ TASKS.md is not normalized to the new nine-project TASKS contract

The newly re-added ninth portfolio project is `Sekiph82/AI-Commerce-HQ@H!veAI`.

Its root `TASKS.md` currently uses an older human-oriented format:

- `## Status: ✅ ALL TASKS COMPLETE`
- `## Completed Tasks (20/20)`
- task lines use `- ✅ ...`

It does **not** currently provide the standardized Project Status fields expected by the new H!veAI parser, and its 20 completed task rows are not Markdown checkbox rows such as `- [x] ...`.

The production parser counts checkbox task rows. Therefore this repository can legitimately be fetched from GitHub while still producing `0/0`, unavailable progress, and missing current/next fields in H!veAI.

This is no longer a `PROJECT.json` problem. It is a canonical TASKS-format compatibility problem introduced by expanding the portfolio from eight normalized repositories to nine.

### Required behavior

`AI-Commerce-HQ` must participate in exactly the same GitHub + root `TASKS.md` model as the other eight projects.

Its existing root tracker should be normalized without losing historical task content so H!veAI can truthfully derive at minimum:

- total tasks = 20 for the existing completed list unless additional canonical task rows exist;
- completed tasks = 20;
- remaining/open = 0;
- completion = 100%;
- project status = completed/closed;
- current task = none/completed in a truthful explicit form rather than fabricated work;
- next task/action = none unless the tracker actually defines future work.

Do not invent a current active task merely to satisfy the parser.

The same nine-project validation must confirm that every repository's root `TASKS.md` is either directly parseable into meaningful counts/state or truthfully represents an empty/not-yet-planned project.

## Automated evidence reported by builder

- Rust: 415 passed, 0 failed.
- Frontend: 125 passed across 15 files.
- Typecheck: PASS.
- Build: PASS.
- Tauri build: PASS.
- Direct remote gate: all nine branches and root TASKS.md files returned non-empty content.
- Published EXE SHA-256: `DA063EE2A1883C7F49C7E123F4D8CA9E88F6C75C2551137B2324C02A73709F09`.

These results support implementation quality but do not override the ninth repository TASKS-format mismatch above.

## Owner-native acceptance still required

After UPRR-A01 is closed, the owner should verify the published executable shows:

1. exactly 9 projects;
2. no `.hiveai/PROJECT.json` error anywhere;
3. AI-Commerce-HQ and H-veAI as separate projects;
4. populated task/progress values for repositories with task data;
5. AI-Commerce-HQ as 20/20 and 100% complete if its repository truth remains unchanged;
6. Project Cockpit current/milestone/progress fields from root TASKS.md;
7. immediate startup video;
8. no terminal flashing.

## FINAL VERDICT

**CONDITIONAL / CHANGES_REQUIRED**

The original BLOCKER, production requests for `.hiveai/PROJECT.json`, appears technically remediated. The nine-project portfolio is also implemented. One MAJOR data-contract gap remains for the newly retained `AI-Commerce-HQ` ninth project, plus owner native acceptance. Do not declare full native closure until both are complete.
