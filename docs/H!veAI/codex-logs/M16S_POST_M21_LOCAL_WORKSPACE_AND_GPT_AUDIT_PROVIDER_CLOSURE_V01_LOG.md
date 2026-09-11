# M16S Post-M21 Local Workspace and GPT Audit Provider Closure V01

## Execution identity

- Work code: `M16S`
- Version: `V01`
- Base synchronization: local `main` was clean and safely fast-forwarded from `origin/main` at `c585a68ec5a419fd4326c4a23b9d74456e4939c9`.
- Implementation commit: `62a11de` (`feat: close M16S provider and workspace remediation`)
- Scope: M16S-F01, M16S-F02, and M16S-F03 only.
- M16 remains OPEN pending independent M16S strict audit and owner native acceptance. M17 is not activated.

## M16S-F01: production GPT audit provider

### Root cause

The production audit command always passed `UnavailableAuditModel` to the existing audit engine. The existing structured parser and persistence contract were present, but no production provider could ever be selected.

### Remediation

- Added a production `AuditModel` implementation for OpenAI's current HTTPS Responses API behind the existing audit engine boundary.
- Added a bounded reqwest transport with HTTPS-only endpoint, bounded connect/total timeouts, bounded response bytes, and sanitized categorized failures.
- Requests use `store:false`, no tools, bounded audit evidence, bounded output tokens, and strict JSON Schema structured output with `additionalProperties:false`.
- OpenAI output is accepted only after the existing strict semantic parser/validator succeeds. Malformed, truncated, auth, rate-limit, transport, and HTTP failures remain truthful non-PASS outcomes and never invent findings.
- Provider resolution retains the existing unavailable fallback whenever the model or credential is not configured or transport setup fails.
- The only credential source is the native process environment variable `OPENAI_API_KEY`; the credential is held in memory for the request and is never stored in SQLite, included in frontend state, returned by readiness, or written to logs.
- Added a bounded non-secret workspace model setting and truthful readiness command/UI. No credential input, save, remove, or display control was added because secure OS-backed credential storage is not present in this bounded change.

### Direct evidence

- Mocked transport tests assert the private/bounded request contract, strict schema, malformed/schema-invalid rejection, and non-PASS status handling for auth, rate-limit, and server failures.
- Readiness/settings tests assert model persistence contains no credential material and readiness reports configuration truthfully.

## M16S-F02: identity-preserving workspace attachment

### Root cause

The safe backend `repair_project_path` command already preserved project identity and rejected unsafe identity changes, but the Projects card exposed a path action only for `MISSING` projects. Active remote-only projects therefore had no user action to attach a local workspace, and valid active projects had no change action.

### Remediation

- Added one bounded Projects-card action for every non-archived project: `Attach local workspace`, `Change local workspace`, or `Repair local workspace` according to the persisted state.
- Reused the existing safe path-repair command and the existing project ID; no registration or duplicate-project path was introduced.
- Preserved the existing project cockpit/settings repair flow and active-project identity rules.

### Direct evidence

- Focused frontend tests cover active remote-only attach and active valid-path change actions and assert the existing repair callback is used.
- Existing registry identity, duplicate, path containment, non-Git/Git, missing-root, and project-scoped cockpit tests remain green.

## M16S-F03: current tracking truth

### Root cause

After accepted M21-R03 owner evidence, the current TASKS and roadmap headers still described M21 as not started and did not identify M16S as the active closure package.

### Remediation

- Updated current/prospective `TASKS.md` and `CODEX_ROADMAP.md` truth to record M21 and M21-R01 through M21-R03 as PASS/CLOSED on accepted evidence.
- Recorded the historical local AI-Commerce parent as retired from active use while retaining the GitHub repository for preservation; no deletion was performed.
- Recorded M16S implementation completion pending independent strict audit and owner native acceptance, with Required Actor `HUMAN`; M16 remains OPEN and M17 remains NOT ACTIVATED/BLOCKED.
- Historical package notes remain historical and are explicitly separated from the current truth header.

## Validation matrix

- Focused Rust audit tests: `32 passed; 0 failed`.
- Focused frontend tests: `3 files, 13 tests passed`.
- Full frontend regression: `17 files, 133 tests passed`.
- Full Rust regression: `425 passed; 0 failed`; main test binary, doc tests, and zero-test binaries completed successfully.
- TypeScript typecheck: passed.
- Production frontend build: passed; Vite transformed 2005 modules.
- Diff/whitespace check: passed; only expected Windows line-ending normalization warnings were emitted.
- Governed native publication: passed from the implementation commit. Candidate and stable executable smoke checks proved PE format, embedded frontend readiness, no forbidden development ports, expected `H!veAI` window title, no newly visible console host, stable/candidate hash equality, and shortcut target/icon correctness.
- Published executable SHA-256: `E84F0940BD0D1DED78D9E9980B2A3186B5B59C6B13C769B7D073DBFF527D5E69`.

## Boundary attestations

- No OpenAI API credential was requested, printed, persisted, or exposed by this run.
- No visible H!veAI UI redesign was performed.
- No M17 or Claude implementation/activation was performed.
- No installer was created.
- No historical parent or preservation repository was deleted.
- Owner native acceptance remains pending as required; this log records automated native smoke/publication evidence only.

## Publication

The implementation was pushed normally to `origin/main` before this log was created. This file is the immutable M16S V01 evidence artifact; its creating commit is intentionally not included in the log identity fields.
