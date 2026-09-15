# M18 GitHub Integration V04 Strict Remediation Log

## Scope

- Prompt executed: `docs/H!veAI/prompts/M18_GITHUB_INTEGRATION_V04_STRICT_REMEDIATION_PROMPT.md`
- Strict audit: `docs/H!veAI/audits/M18_GITHUB_INTEGRATION_V03_STRICT_AUDIT.md`
- Owner UX plan: `docs/H!veAI/plans/M18_V04_OWNER_PROMPT_AGENT_UX_CONSOLIDATION_PLAN.md`
- Scope closed: F-M18-V03-001 through F-M18-V03-005 and M18.10.01 through M18.10.05 only.
- M19 was not activated.
- `TASKS.md` and `CODEX_ROADMAP.md` were not modified. Root `TASKS.md` remains the sole current authority for local projects; tracked-branch root `TASKS.md` remains the sole current authority for GitHub-tracked projects.

## Safe synchronization

- Initial standalone checkout: clean `main` at `210684e10e1dbe8bb2c4627fd980c72b30ccb6a9`.
- `origin/main` after fetch: `a6520fb11a02fd14d06cb21f0148dfeec7d99ef1`.
- Divergence: local was an ancestor, eight commits behind, with no local owner changes.
- Synchronization: `git merge --ff-only origin/main` only.
- No reset, rebase, force-push, automatic stash, `git clean`, destructive checkout, or discarded owner work was used.

## Remediation disposition

### F-M18-V03-001 — closed

Production pull-request acquisition now enriches a bounded selected set with detail, changed files, reviews, issue comments, pull comments, commit check-runs, and commit status. The frontend exposes the resulting bounded facts and per-resource truth states rather than only list-row metadata.

### F-M18-V03-002 — closed

Production Actions acquisition now enriches a bounded selected set with jobs and steps, and fetches bounded logs only for failed, cancelled, timed-out, or action-required jobs. The UI distinguishes current empty Actions (`No CI runs`) from unavailable or failed Actions evidence and never treats unavailable data as proof of no CI.

### F-M18-V03-003 — closed

Successful remote payloads are sanitized before cache persistence and before frontend projection. Recursive credential-key/value redaction covers authorization/bearer forms, GitHub token families, token/access-token/api-key forms, and bounded text excerpts. Cache reload rejects payloads whose sanitized form differs, failing closed.

### F-M18-V03-004 — closed

Task and session links are emitted only for explicit canonical `TASK-...`, `SESSION-...`, and valid dotted `M<number>` references. Prose such as “M18 integration” and other non-canonical lookalikes is not promoted to a link.

### F-M18-V03-005 — closed

The production acquisition path is covered through an injectable transport seam, not a test-only endpoint shortcut. Fixture tests assert the exact enrichment paths, bounded request count, parsed facts, and failed-job log evidence. The normal full Rust suite was rerun after diagnosing the watcher failure as a timing-sensitive test harness assertion; the test now polls for the durable state transition while production watcher behavior is unchanged.

### M18.10.01 — closed

Prompt Engine is the primary user-facing surface with Prompt Builder and Sessions tabs. Agents was removed from primary navigation and the command palette.

### M18.10.02 — closed

Legacy `/agents?projectId=...&sessionId=...` links resolve into the integrated Prompt Engine Sessions surface, select the exact persisted project/session, preserve fail-closed cross-project and missing-session behavior, and do not redispatch. New Prompt Engine handoffs target the integrated sessions route.

### M18.10.03 — closed

Codex and Claude builder/session readiness is shown in a separate Settings Builder Providers section. It remains provider-neutral, reports version/state/diagnostics/resume/capabilities, and does not read credential files or expose API keys. The Codex-only Audit Provider contract remains a separate section.

### M18.10.04 — closed

Nullable `required_actor` is preserved as nullable through prompt context collection and generation; no default HUMAN actor is invented. A native regression test covers the real nullable database row and generated context.

### M18.10.05 — closed

Focused M15 handoff/legacy-route tests and M14 session tests cover exact integrated routing, no redispatch, fail-closed targets, provider behavior, and preserved session/provenance behavior. Existing M00-M17 and X03/X04 behavior remained in the full regression suite.

## Bounded GitHub evidence contract

- Base reads remain repository, branches, commits, pull requests, issues, Actions runs, releases, and tags.
- Selected PR enrichment uses `/pulls/{number}`, `/pulls/{number}/files`, `/pulls/{number}/reviews`, `/issues/{number}/comments`, `/pulls/{number}/comments`, `/commits/{head_sha}/check-runs`, and `/commits/{head_sha}/status`.
- Selected Actions enrichment uses `/actions/runs/{run_id}/jobs` and failed-job `/actions/jobs/{job_id}/logs`.
- Enrichment is selected and capped: five PRs, five Actions runs, 25 files, 10 reviews, 20 checks, 20 jobs, 25 steps, 512-byte log summaries, and 80 subresource requests maximum. The fixture transport records and asserts the topology and bound.
- Remote reads remain observational. PR creation, issue mutation, workflow retry, push, merge, checkout, reset, and rebase are not exposed.

## Verification evidence

- Implementation/test commit: `3469c7399a1a5f98593f7f731070d83b2a67a692`.
- Full Rust library regression: `509 passed; 0 failed; 0 ignored`.
- Full frontend regression: `18 files passed; 144 tests passed`.
- `npm run typecheck`: passed.
- `npm run build`: passed.
- Governed native publication: `scripts/publish-dev-qa.ps1` passed and smoke-tested `dev-bin/H!veAI.exe`.
- Published executable SHA-256: `8888A2724BFB2138BB8AC0CC09C9AC68246E2D012D931EE1AF1C09595326F4C4`.
- GitHub commit status/check-runs for the implementation commit: status endpoint returned `pending` with `total_count: 0`; check-runs endpoint returned `total_count: 0`. No hosted checks were available to wait for; local full regressions were the publication gate.
- Security negative scans found no implementation path that reads credential files or exposes provider/API keys; no prohibited Git mutation command was added.
- Final pre-log equality check: local HEAD, `origin/main`, and live GitHub `main` all resolved to the implementation SHA above; worktree was clean before adding this log.

## Changed files

Implementation/test commit changed only:

- `src-tauri/src/github_integration.rs`
- `src-tauri/src/prompt_engine.rs`
- `src-tauri/src/watcher.rs`
- `src/App.tsx`
- `src/PromptEnginePage.tsx`
- `src/agentNavigation.ts`
- `src/components/Shell.tsx`
- `src/githubIntegration.ts`
- `src/pages.tsx`
- `src/styles.css`
- `tests/m15c-post-dispatch-handoff-focused.test.tsx`

This builder log is the only additional V04 change. No registered external project repository was modified.

## Stop condition

V04 implementation, tests, log, and native publication are pushed to `Sekiph82/H-veAI@main`. Codex stops here for independent audit.

