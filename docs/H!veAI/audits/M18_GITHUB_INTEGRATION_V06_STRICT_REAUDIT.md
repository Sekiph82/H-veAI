# M18 GitHub Integration V06 — Independent Strict Re-Audit

## 01. Audit identity

- Milestone: M18 GitHub Integration
- Remediation generation: V06
- Builder implementation/test commit: `94bbc45705511ae503befe8ea1f5c2905ee6bf1f`
- Builder-log commit: `08b9d4fd2d301d66e63514b55dd81914781ee45e`
- Builder log: `docs/H!veAI/codex-logs/M18_GITHUB_INTEGRATION_V06_LOG.md`
- Authoritative prompt: `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V06_STRICT_REMEDIATION_PROMPT.md`
- Prior failed audit: `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V05_STRICT_REAUDIT.md`

## 02. Verdict

**PASS — SOURCE_ACCEPTED / OWNER_NATIVE_ACCEPTANCE_REQUIRED**

V06 closes all three residual findings from the V05 strict re-audit at source/test-contract level. No new BLOCKER, MAJOR, or MINOR source defect was found in the bounded V06 change set.

M18 is **not** PASS/CLOSED yet. M19 remains blocked until owner-native acceptance is completed and ChatGPT performs the canonical tracker transition.

## 03. Severity summary

- BLOCKER: 0
- MAJOR: 0
- MINOR: 0
- Closed V05 residual findings: 3 / 3

## 04. Scope and change-boundary verification

The V06 range from `46eba0b492c14b2a8a925cd0e0ea7525efedf7be` to `08b9d4fd2d301d66e63514b55dd81914781ee45e` contains exactly two commits: one implementation/test commit and one builder-log commit.

Changed implementation/test files are bounded to GitHub integration, test-only task-intelligence isolation, watcher regression stabilization, and focused frontend regression tests. `TASKS.md` and `CODEX_ROADMAP.md` are absent from the Codex diff. No M19 implementation is present.

## 05. F-M18-V05-001 — quoted Authorization redaction

**CLOSED.**

`authorization_span()` now tolerates a closing quote between the `Authorization` key and its colon, preserving the bounded scanner model while recognizing JSON-looking retained text such as `"Authorization":"Basic ..."`. It also preserves support for spaced/no-space Bearer and Basic forms.

The sanitizer remains deterministic and byte-scanned rather than regex/backtracking based. Cache schema version 2 remains in force and the existing schema-1 rejection boundary is preserved.

## 06. End-to-end cache/DTO secret evidence

**PASS at source/test-contract level.**

The production-path test now drives representative synthetic secrets through `snapshot_with_transport`, then checks both the projected issue DTO and persisted `github_sync_state.metadata_json` for absence of plaintext values. Coverage includes URL `access_token`, URL `token`, spaced/no-space Bearer, spaced/no-space Basic, JSON-looking quoted Authorization, quoted token/api-key values, and all supported GitHub token prefix families.

The same test also verifies a sanitized schema-2 cache entry can be reloaded. Existing direct tests continue to reject schema-1 and malformed cache rows and preserve branch/repository/resource scoping.

## 07. Sanitizer idempotence and fail-closed cache behavior

**PASS.**

The V06 scanner explicitly recognizes the `[REDACTED]` marker so already-sanitized payloads remain stable when revalidated. `load_cache()` still rejects rows whose payload changes under the sanitizer, preventing an unsanitized legacy/raw payload from being promoted as trusted current evidence.

No credential store, environment secret, Git auth store, or provider credential file access was added.

## 08. F-M18-V05-002A — Actions production-path matrix

**CLOSED.**

V06 adds direct `snapshot_with_transport` coverage proving:

- a successful job before a failed job does not cause a success log request;
- a third failed job after success/skipped jobs is selected;
- cancelled, timed-out, and action-required jobs are eligible;
- success/skipped jobs are excluded;
- first failed log unavailable + second current yields `PARTIAL` and retains second-job provenance;
- unavailable jobs resource produces no fabricated log evidence;
- current jobs with no eligible failure produces `NOT_APPLICABLE`;
- failed-log acquisition is bounded to two jobs;
- excerpts are truncated to the configured bound;
- a third failed-job log is not requested.

The previously accepted global enrichment budget test still proves no N+1 acquisition after budget exhaustion.

## 09. Actions provenance and truth semantics

**PASS.**

Retained failed-log evidence continues to carry job ID and job name. A run-level summary is derived from an actually eligible failed/attention job rather than from arbitrary job ordering. Partial/unavailable states remain explicit instead of being silently promoted to verified-empty success.

## 10. F-M18-V05-002B — project-owned reference matrix

**CLOSED.**

The selected H!veAI project is now tested alongside a real registered foreign portfolio project, `Sekiph82/FormuLab@main`, using syntactically valid foreign task and session IDs. Only selected-project canonical TASKS-derived task IDs and sessions persisted under the selected project become validated links. Foreign and unknown syntactically valid references remain raw evidence only.

No title-similarity ownership inference or hidden `.hiveai` current-truth fallback is introduced.

## 11. Portfolio and X04 authority preservation

**PASS.**

The implementation continues to consume the accepted GitHub tracking snapshot whose canonical task rows originate from tracked-branch root `TASKS.md`. V06 does not revive `STATE`, `HANDOFF`, `EVENT_INDEX`, `PROJECT`, or other hidden control-plane files as current project truth.

The accepted exact eight-project portfolio remains unchanged, including `Sekiph82/FormuLab@main`.

## 12. F-M18-V05-002C — legacy `/agents` route matrix

**CLOSED.**

Mounted frontend tests now separately cover bare `/agents`, exact target, project-only, session-only, duplicate project/session IDs, malformed ID, overlong ID, wrong-project session, and missing session behavior. The legacy route enters the integrated Prompt Engine Sessions surface and none of these normalization paths dispatches/relaunches a provider.

The accepted V05 architecture remains intact: no primary-navigation Agents workspace, no top-level legacy Claude readiness wall, and Prompt Builder + Sessions remain the unified Prompt Engine workspace.

## 13. M14/M15/M17 regression preservation

**PASS at source/test-contract level.**

Focused frontend regression files retain exact prompt/session provenance, target-session selection, later manual session selection, polling updates, provider separation, and post-dispatch handoff behavior. V06 does not reopen the accepted Claude adapter or Codex provider architecture.

## 14. F-M18-V05-003 — parallel Rust test isolation

**CLOSED.**

The task-intelligence failpoints were process-global mutable state in V05. V06 changes only the test instrumentation boundary to thread-local `RefCell` failpoints. Production parser code paths and accepted M09 semantics are unchanged.

The affected retry/containment tests set and consume failpoints on their own test thread, preventing unrelated parallel tests from consuming or poisoning shared state. No global test serialization, ignored-test addition, production semantic weakening, or arbitrary sleep was introduced for this fix.

## 15. Watcher regression stabilization

**PASS.**

The watcher remediation is confined to the test expectation around an asynchronous refresh, using the existing bounded polling helper rather than weakening production watcher behavior. The live watcher implementation remains unchanged by the V06 bounded remediation.

## 16. Verification-evidence assessment

The builder log reports:

- focused GitHub integration: 19 passed, 0 failed;
- two consecutive normal parallel `cargo test --lib` runs: 515 passed, 0 failed;
- frontend: 18 files / 151 tests passed;
- focused M13/M14/M15C/M17: 4 files / 34 tests passed;
- typecheck, build, cargo check, and diff check passed.

These local execution counts remain builder claims because no independent hosted CI/status/check is attached to the implementation commit. The important acceptance distinction is preserved: this audit independently verifies the source structure and test contracts, not the builder machine's console transcript.

## 17. GitHub hosted-status observation

Independent GitHub inspection of implementation commit `94bbc45705511ae503befe8ea1f5c2905ee6bf1f` returns no commit statuses and no workflow runs. Therefore this audit makes no hosted-green-CI claim.

Absence of hosted CI does not itself reopen a source finding because the repository currently has no required hosted check configured for this branch, but owner-native acceptance remains mandatory before M18 closure.

## 18. Security and mutation boundary

**PASS.**

No GitHub PAT/API-key setting, credential/auth-store read, arbitrary frontend-controlled GitHub host, hidden `.hiveai` current-truth fallback, automatic remote Git mutation, M19 activation, or standalone Agents workspace was introduced by the V06 diff.

Synthetic secrets remain test fixtures only. GitHub integration stays read-only/default-denied on mutation boundaries.

## 19. Publication and repository state

The builder log reports governed publication through `scripts/publish-dev-qa.ps1` only, with executable SHA-256:

`DC0E01D746C89742A5570C999D46528A343D5279EAC226F9766906C116759ACD`

Independent GitHub inspection confirms live `main` at the time of this audit contains builder-log commit `08b9d4fd2d301d66e63514b55dd81914781ee45e`. The V06 Codex range did not modify canonical trackers.

## 20. Acceptance decision and next gate

**Independent V06 source decision: PASS.**

All V05 residual source findings are closed. There is no V07 remediation prompt at this stage.

Required next gate is **owner-native M18 acceptance** using the governed native executable. At minimum verify:

1. H!veAI launches normally from the stable desktop entry without a development server or terminal window.
2. Prompt Engine contains both **Prompt Builder** and **Sessions** in one workspace.
3. The old standalone **Agents** navigation item is absent.
4. Settings exposes **Builder Providers** separately from **Codex Audit Provider**, with expected Codex/Claude readiness truth.
5. A normal Prompt Engine project/task selection loads without the prior nullable `required_actor` error.
6. A dispatched/available session can be opened in the integrated Sessions surface without redispatch.
7. A Project Cockpit GitHub surface loads bounded repository/PR/Actions evidence without inventing unavailable CI truth.
8. Existing Audit Center provider/history behavior from X03/X04 remains intact.

If owner-native acceptance passes, ChatGPT must record the owner acceptance and then update `TASKS.md` and `CODEX_ROADMAP.md` to close M18 and activate the next roadmap transition. Codex must not perform that tracker transition.