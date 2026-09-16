# M19 Next Best Task AI + Engineering Brief V05 — Builder Evidence Log

Date: 2026-09-17 (Europe/Istanbul)

## Authority and synchronization

- Authoritative prompt: `docs/H!veAI/prompts/M19_NEXT_BEST_TASK_AI_ENGINEERING_BRIEF_V05_REMEDIATION_PROMPT.md`
- Prior V04 remediation prompt, V04 immutable log, and V04 strict re-audit were read before source changes.
- Safe synchronization was `git fetch origin main` followed by `git merge --ff-only origin/main`.
- Synchronized start SHA / `origin/main`: `f45f5ad047a2cb8cacab271f2ba20eef8a3568e7`
- Implementation commit: `496d906` (`fix: complete M19 V05 residual remediation`)
- This file is created after the implementation commit and is intended to be immutable; it is not amended after publication.

## Changed files

- `src-tauri/src/github_tracking.rs`
- `src-tauri/src/next_best_task.rs`
- `src-tauri/src/github_integration.rs`
- `src-tauri/src/command_center.rs`
- `src-tauri/src/project_cockpit.rs`
- `src/commandCenter.ts`
- `src/command_center_view.tsx`
- `src/projectCockpit.ts`
- `tests/m11-command-center-focused.test.tsx`
- `scripts/m19-v05-browser-geometry.mjs`
- `package.json`

`TASKS.md` and `CODEX_ROADMAP.md` are absent from the implementation diff and remain unchanged.

## Finding dispositions

### F-M19-V04-STRICT-001 — remote duplicate explicit IDs

Implemented. Remote rows retain the owner-authored explicit ID and a deterministic canonical row ID derived from repository, branch, content hash, source line, and normalized row text. The M19 graph indexes normalized explicit ID to all canonical rows; zero, one, and multiple matches are distinct outcomes. Duplicate remote IDs, case-equivalent IDs, conflicting completion states, duplicate local IDs, and duplicate dependency edges are covered by native tests. A dependent on an ambiguous `TASK-A` is ineligible and produces one bounded ambiguity fact; no `HashMap` winner-by-overwrite is possible.

### F-M19-V04-STRICT-002 — same-HEAD validation freshness

Implemented. Persisted tracking now separates `content_fetched_at` from `validated_at`. A successful same-HEAD observation updates and persists only validation freshness, clears the obsolete transient error, and leaves content hash/rows/content-fetch time unchanged. M19 freshness reads successful validation time with branch, HEAD, and root-hash invariants. Legacy persisted snapshots migrate conservatively and fail closed when required materialization is absent. Native tests cover validation after more than five minutes, stale without validation, failed validation without timestamp advancement, changed HEAD, and legacy snapshot behavior.

### F-M19-V04-STRICT-003 — cached 403/429 circuit behavior

Implemented. Rate-limit classification is independent of presentation state, so cached last-known-good data may remain `STALE` while the machine cause is `RATE_LIMITED`. The process-wide circuit opens on the first 403 or 429, stops remaining primary and all optional calls, retains last-known-good data where available, and emits one grouped warning. Native production-path tests cover 403/429 with and without cache and assert one network request, circuit state, truthful stale/unavailable presentation, and warning grouping.

### F-M19-V04-STRICT-004 — fair quota-safe portfolio acquisition

Implemented with tiered planning and a process-wide governor. Idle uses durable cache plus bounded remote tracking and zero detailed primary/optional fan-out. Selected/manual acquisition is limited to one viewed project with eight primary resources and a separate optional allowance of eight. Navigation uses deterministic round-robin primary service and reserves primary capacity; optional work is shed first. The real process governor enforces 40 primary plus 8 optional requests per hour, while manual refresh cannot bypass it.

Exact scheduled request math exercised for 8, 9, 10, and 20 projects:

| Portfolio | Idle detailed primary / optional / tracking | Selected or manual primary / optional / tracking | Navigation primary / optional / tracking |
| --- | --- | --- | --- |
| 8 | 0 / 0 / 8 | 8 / 8 / 1 | 40 (5 each) / 8 / 8 |
| 9 | 0 / 0 / 9 | 8 / 8 / 1 | 36 (4 each) / 8 / 9 |
| 10 | 0 / 0 / 10 | 8 / 8 / 1 | 40 (4 each) / 8 / 10 |
| 20 | 0 / 0 / 20 | 8 / 8 / 1 | 40 (2 each) / 8 / 20 |

The fairness test proves every project receives a first round before any project repeats, optional work cannot consume primary capacity, PR-rich enrichment is shed, and the reset window is bounded. After exhausted external/shared quota, no new network request is made during the 90-second circuit backoff; last-good data is stale and not presented as current.

### F-M19-V04-STRICT-005 — real in-flight result coalescing

Implemented in the production-compiled path. A per-request-key `Arc` result slot plus condition variable makes the first caller the owner and later callers wait for the same bounded result. Owner success and failure are shared; timeout and panic/error paths settle and remove the in-flight entry. Slow transport tests cover 2 callers and 10 callers with exactly one transport request, shared CURRENT result, shared failure, and old-cache-plus-slow-owner behavior. The tests exercise the same governor code compiled for production; test fixtures serialize only their own global fixture state.

### F-M19-V04-STRICT-006 — refresh generation and history ordering

Implemented. Native refresh requests a generation and waits for bounded scheduler completion, including degraded/failure completion, instead of returning when work is merely queued. The frontend defers M19 history recording until the refresh promise resolves and then reads the new snapshot. A delayed mounted test proves no history write during in-flight refresh and correct ordering after completion; native tests cover generation ordering and bounded timeout.

### F-M19-V04-STRICT-007 — legacy/M19 attention identity

Implemented. Native `AttentionItem` serialization carries one stable `issueKey` derived from project/task and normalized operational evidence. The mounted test uses a real legacy-shaped item with detail and no evidence alongside an M19 item with evidence for the same fact and proves one rendered identity. Distinct blockers, provider-unavailable state, and distinct evidence IDs remain separate.

### F-M19-V04-STRICT-008 — actual browser geometry

Implemented. `scripts/m19-v05-browser-geometry.mjs` loads production CSS into a populated card fixture and evaluates actual Chromium layout; it does not mock `getBoundingClientRect`, `clientWidth`, `scrollWidth`, or computed layout. Requested widths 1536, 900, and 640 measured inner widths 1510, 874, and 614 respectively (the difference is the browser scrollbar). Every row passed containment, non-overlap, non-zero click boxes, focus visibility, `Local workspace` visibility, forbidden-copy absence, and document-overflow checks.

### F-M19-V04-STRICT-009 — dependency edge deduplication

Implemented. Dependency references are normalized and deduplicated before resolution, including case-equivalent references. A dependent/prerequisite pair has one canonical edge, one unfinished blocker/evidence fact, and one unlock-score contribution. Native tests cover `A, A` and `A, a`.

### F-M19-V04-STRICT-010 — production-path evidence matrix

Implemented for the native governed seam. The matrix below records the minimum repository set and the exercised classes; shell/public probes are not used as production evidence.

| Repository / branch | Class | Pre-cache and admission | Network / coalescing | Result and diagnostic |
| --- | --- | --- | --- | --- |
| `Sekiph82/H-veAI@main` | primary | cache-first; governor owner or coalesced | governed; one owner per key | CURRENT on fixture success; bounded sanitized error on failure |
| `Sekiph82/H-veAI@main` | optional | separate 8-request reservation; shed after reserve | governed; never spends primary reserve | PARTIAL/STALE when shed or rate-limited |
| `Sekiph82/H-veAI@main` | remote-tracking | bounded tracking refresh | one tracking observation | validation timestamp and root hash are persisted |
| `Sekiph82/Bulk-Edit@main` | primary + optional | same production seam and process window | governed; PR/action enrichment bounded | PARTIAL when optional work is shed; no warning storm |
| `Sekiph82/ScrubBots-Level-Factory@main` | primary + optional | same identity/branch validation | governed and bounded | same truthful CURRENT/PARTIAL/STALE contract |
| `Sekiph82/FormuLab@main` | primary + remote-tracking | same portfolio identity path | governed | same cache/freshness contract |

The native tests additionally assert request-count budget before/after, admission owner/coalesced/budget-exhausted behavior, cache retention, grouped rate-limit cause, and bounded timestamps. Any unavailable live GitHub rate-limit headers are recorded as unavailable by the fixture seam rather than invented.

### F-M19-V04-STRICT-011 — published builder smoke

The accepted publication helper completed production build, candidate executable validation, published stable swap, shortcut target/icon validation, and stable executable smoke. The published executable is `dev-bin/H!veAI.exe` with SHA-256 `B19464E0C443B949DDA3DA44B4A98A2054E9E3BCC4B634897E3A516D4B2AB591`.

The following evidence is directly covered by production-path native tests and the published-build helper: exact-eight plus ninth/tenth persistence contract, authoritative Add Project dispositions, repository identity checks, card geometry, `Local workspace` copy, bounded Bulk-Edit/ScrubBots acquisition, 403/429 circuit behavior, recovery semantics, H-veAI/control identity, and M18 panel containment.

The interactive owner-like window scenarios below are deliberately **not self-claimed** because this run has no owner-native UI acceptance channel. They remain required for the independent builder/native smoke pass:

| Scenario | Status in this log |
| --- | --- |
| archived ninth explicit Add, same-ID ACTIVE and visible | not self-claimed; native persistence/recovery tests pass |
| restart published app retains ninth | not self-claimed; persistence tests pass |
| tenth unrelated project leaves ninth and tenth visible | not self-claimed; exact portfolio tests pass |
| owner-sized card action containment | PASS via real Chromium geometry; published window click-through not self-claimed |
| every workspace button says `Local workspace` | PASS via real Chromium geometry; published window click-through not self-claimed |
| Bulk-Edit and ScrubBots pages without repeated warning wall | not self-claimed as interactive pages; governed native tests pass |
| forced cached 403/429 fixture and recovery | PASS via native production-path fixture tests |
| H-veAI plus control project | PASS via native identity/acquisition tests |
| no M18 full-app black surface | not self-claimed as interactive window; existing frontend regression and published helper smoke pass |

This is builder evidence, not owner acceptance. The log remains a claim until independent V05 source re-audit and the missing owner-like interactive smoke are completed.

## Regression and publication gates

- `npm run typecheck`: PASS
- `npm run build`: PASS (Vite build; existing chunk-size/dynamic-import warnings only)
- focused mounted frontend tests: PASS
- normal/default frontend invocation: PASS, 164/164
- `npm test -- --maxWorkers 1 --minWorkers 1`: PASS, 19 files / 164 tests
- `npm run test:geometry:browser`: PASS at requested 1536/900/640
- `cargo check --manifest-path src-tauri/Cargo.toml`: PASS (existing warnings only)
- focused native V05 suites: PASS — github tracking 17, next-best-task 13, GitHub integration 29
- production governor/coalescing, 403/429, same-HEAD, duplicate-ID, fairness, refresh-generation, and dependency-dedup tests: PASS
- exact M16L observational read: PASS, 1 test in 218.63s
- full native `cargo test --manifest-path src-tauri/Cargo.toml --lib --no-fail-fast --quiet`: PASS, 548/548
- changed Rust files `rustfmt --check`: PASS
- full cargo fmt check: existing unrelated baseline formatting drift remains outside the changed-file check; no tracker files were touched
- `git diff --check`: PASS
- accepted publication helper: PASS
- stable executable SHA-256: recorded above

## Stop boundary

`TASKS.md` and `CODEX_ROADMAP.md` remain read-only and unchanged. M20 was not started, opened, or modified. No M19 PASS/CLOSED transition was made. Owner-native acceptance is not self-claimed. The next action after this immutable log is independent V05 strict source re-audit.
