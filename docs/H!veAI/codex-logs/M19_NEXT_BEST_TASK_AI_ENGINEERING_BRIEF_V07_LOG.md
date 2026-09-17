# M19 Next Best Task AI + Engineering Brief V07 — Builder Evidence Log

Date: 2026-09-17 (Europe/Istanbul)

## Authority and safe synchronization

- Authoritative prompt: `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V07_REMEDIATION_PROMPT.md`
- The preceding V06 remediation prompt and V06 strict audit were read in full before source changes.
- Safe synchronization: `git fetch origin main` followed by `git merge --ff-only origin/main`.
- Synchronized start SHA / `origin/main`: `28b93c1e1fc97514b1830da2f3bc12a6201fb8fc`
- Implementation commit: `4288e232ac946a18432605a32d49d2255abcd82b` (`Complete M19 V07 production remediation`)
- This log is created after implementation and publication verification; it is immutable and will not be amended.

## Scope guard

- `TASKS.md` and `CODEX_ROADMAP.md` were unchanged and remain read-only for this work.
- M20 was not started.
- No reset, rebase, force-push, history rewrite, or remote GitHub mutation was used.
- Builder evidence does not claim owner-native interactive acceptance. Owner acceptance remains pending independent strict audit.

## V06 strict finding dispositions

### F-M19-V06-STRICT-001 — scheduler-compatible remote freshness

Closed with production-path evidence. Remote validation freshness now requires `validated_at`, uses the portfolio hard horizon derived from the one-hour background cadence plus the bounded changed-HEAD completion budget, and never falls back to content fetch time. The selected target horizon is separately derived from its five-minute cadence. Fake-clock tests cover the 1-hour/1-hour-plus-boundary behavior, unchanged HEAD validation, and failed validation not advancing freshness.

### F-M19-V06-STRICT-002 — monotonic inverse-failure backoff and fairness

Closed with production-path evidence. Failure retry delay is an explicit monotonic bounded schedule based on the selected/background cadence, with a 24-hour cap and reset on a CURRENT observation. Background failures cannot retry more aggressively than the hourly cadence; the tracking governor and bounded queue prevent one project or a five-minute budget herd from monopolizing the portfolio. Tests and the V07 matrix cover consecutive failures and 8/9/10/20-project demand.

### F-M19-V06-STRICT-003 — generation/project-scope binding

Closed with production-path evidence. Refresh generations carry immutable project scope through a bounded coordinator and completion map. Concurrent project A/B generations cannot release one another, while same-project overlapping generations coalesce only for the same scope. The scheduler settles each generation with the observation for its own project, and native tests cover scope preservation and failure propagation.

### F-M19-V06-STRICT-004 — changed-HEAD refresh deadline

Closed with production-path evidence. The refresh completion deadline is derived from three sequential 20-second HTTP stages, three 5-second process-grace intervals, and a bounded completion overhead: 80 seconds. Slow changed-HEAD fixtures complete inside this budget; over-deadline completion is settled without late history mutation. `refresh_and_compare` records the pre-refresh baseline only after successful scoped refresh completion.

### F-M19-V06-STRICT-005 — Navigation rotation across acquisition windows

Closed with production-path evidence. Navigation uses a process-wide public rotation cursor that advances between `portfolio_snapshots` windows and is not reset by Idle, Selected, or Manual calls. The real production orchestration test observes successive windows and verifies eventual primary-kind coverage across eight projects; cache hits do not consume admissions.

### F-M19-V06-STRICT-006 — complete V07 acquisition evidence

Closed with the generated artifact `docs/H!veAI/evidence/M19_V07_GITHUB_ACQUISITION_MATRIX.md`. The artifact is produced by `npm run generate:evidence:v07` from the native production `portfolio_snapshots_with_transport` orchestration probe, not a helper-only calculation. It contains the complete row schema, detailed Idle/Selected/Navigation/Manual demand rows, tracking unchanged/changed-head math, 8/9/10/20 request-budget evidence, failure/backoff evidence, and redacted provenance.

### F-M19-V06-STRICT-007 — real Projects React geometry

Closed with the real React browser gate. `npm run test:geometry:v07` starts Vite, navigates Chromium to the actual `/projects` route, and evaluates the production Projects surface with bounded Tauri seams. It runs at requested viewport classes approximately 1536/900/640 pixels (measured inner widths 1510/874/614), checks card/action geometry and keyboard focus, and walks ancestor overflow/clipping containers to prove focus visuals are not clipped.

## Verification gates

- `cargo check --manifest-path src-tauri/Cargo.toml` — PASS.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib --no-fail-fast --quiet -- --test-threads=1` — PASS, 564/564.
- Focused tracking, integration, next-best-task, scope/generation, backoff, deadline, navigation, circuit-breaker, and V07 evidence tests — PASS.
- `m19_snapshot_is_observational_until_history_is_explicitly_recorded` — PASS.
- `error_redaction_and_response_bounds_are_deterministic` — PASS.
- `npm run typecheck` — PASS.
- `npm run build` — PASS.
- `npm test -- --maxWorkers 1 --minWorkers 1` and default `npm test` — PASS, 19 files / 164 tests.
- `npm run test:geometry:v07` — PASS on the real React Projects route at all three requested viewport classes.
- `npm run generate:evidence:v07` — PASS; generated matrix is present and includes the complete V07 schema.
- `git diff --check` — PASS. `cargo fmt -- --check` reports pre-existing repository-wide formatting drift; no formatter rewrite was applied because it would change unrelated files.
- `powershell -ExecutionPolicy Bypass -File scripts/publish-dev-qa.ps1` — PASS; native production build and smoke publication completed.
- Published executable: `dev-bin/H!veAI.exe`.
- Published executable SHA-256: `13D83EA4C38217D2E4588224B6CF4662E9534978B9A572206F809A1DC2E379F9`.
- V07 acquisition artifact SHA-256 before final publication commit: `73AF46EF432B86E318362A73B8B568A4A87085C3E874CEF2F2BA712550BCF2B2`.

## Publication status

The final publication commit contains this immutable log and the generated V07 acquisition evidence artifact. It will be pushed non-force to `origin/main`; the final SHA and live GitHub/local equality are verified at handoff. M19 remains audit-pending until independent strict review and owner-native acceptance; no M20 work is included.
