# M16S Post-M21 Local Workspace and GPT Audit Provider Closure V02

## MANDATORY SYNC-FIRST AND GITHUB-FIRST CONTRACT

Before reading or changing any implementation file, safely synchronize the standalone H!veAI checkout with `Sekiph82/H-veAI` on `main`:

```powershell
git fetch origin main
git status --short
git rev-parse --show-toplevel
git rev-parse HEAD
git rev-parse origin/main
git rev-list --left-right --count HEAD...origin/main
```

If the working tree is clean and only behind `origin/main`, update only with:

```powershell
git merge --ff-only origin/main
```

Never use reset, rebase, force-push, destructive checkout, automatic stash, `git clean`, or any operation that discards owner work. If safe synchronization is impossible, stop with `SYNC_BLOCKED`.

After synchronization, read these current authorities before implementation:

- `AGENTS.md`
- `TASKS.md`
- `CODEX_ROADMAP.md`
- `docs/H!veAI/prompts/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V01_PROMPT.md`
- `docs/H!veAI/codex-logs/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V01_LOG.md`
- `docs/H!veAI/audits/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V02_AUDIT.md`

All Codex-facing work and the required builder log must be entirely in English.

All H!veAI repository changes made by this task must be committed and pushed before completion is claimed. At the end verify:

```powershell
git fetch origin main
git rev-parse HEAD
git rev-parse origin/main
git ls-remote origin refs/heads/main
git status --short
```

Completion requires local `HEAD == origin/main == live remote main` and a clean H!veAI working tree. Otherwise return `SYNC_BLOCKED`.

Owner-facing final response must contain only relevant GitHub H!veAI file URLs/paths, implementation/log commit SHA(s), final GitHub `main` SHA, and concise status. Do not dump ordinary local changed-file paths.

---

## WORK ITEM

- Work code: `M16S`
- Version: `V02`
- Repository: `Sekiph82/H-veAI`
- Branch: `main`
- Required log: `docs/H!veAI/codex-logs/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V02_LOG.md`
- Required independent audit after builder completion: ChatGPT, not Codex

This is a narrow remediation of three residual V01 findings. Do not redesign the Audit Engine, Project Registry, Settings shell, or GitHub tracking architecture. Do not activate M17 and do not implement Claude work.

The V01 implementation is materially correct and must be preserved. Fix only the residual defects below plus the focused tests/documentation needed to prove them.

---

# FINDING M16S-V02-F01 — MAJOR — REAL PROVIDER READINESS CHECK

## Current incorrect behavior

The Settings button labeled `Check readiness` calls `getAuditProviderReadiness()`. Native `audit_provider_readiness()` checks only whether:

- `OPENAI_API_KEY` exists and is non-empty; and
- a non-empty audit model setting exists.

It then reports `READY`.

This is configuration-presence detection, not provider readiness. A revoked/invalid credential, unavailable model, network outage, provider outage, or rate/quota condition may still be shown as READY.

The frontend type advertises:

- `READY`
- `NOT_CONFIGURED`
- `AUTH_ERROR`
- `RATE_LIMITED`
- `NETWORK_ERROR`

but the current Settings readiness action cannot produce the latter three.

## Required target behavior

Separate local configuration state from live provider readiness.

Keep a cheap local configuration inspection for ordinary page load. Do not make Settings page load automatically spend provider requests.

Add one explicit bounded native `Check readiness` operation invoked only by the user's button. It must validate the configured credential/model against OpenAI without creating or persisting an H!veAI audit run.

The readiness probe must:

- use native HTTPS, never browser JavaScript, curl, PowerShell, Python, or Node;
- never expose the raw credential to the frontend;
- have short explicit connect/total timeouts;
- bound response bytes;
- classify at minimum:
  - valid credential/model/provider reachable => `READY`;
  - missing key/model => `NOT_CONFIGURED`;
  - 401/403 => `AUTH_ERROR`;
  - 429 => `RATE_LIMITED`;
  - timeout/DNS/transport/5xx => `NETWORK_ERROR` or an equally truthful non-ready provider-unavailable category consistent with the existing frontend contract;
  - invalid/unavailable model => a truthful non-ready status/message, never READY;
- return only non-secret metadata;
- not persist the API key, Authorization header, raw provider body, or secret-bearing diagnostics;
- not create an audit record or alter audit history;
- not silently fall back to a fake fixture.

Use an official low-cost/no-generation provider capability/model validation endpoint if the current OpenAI API provides one suitable for validating the configured model and credential. If no suitable official endpoint exists, use the smallest explicit user-triggered Responses API readiness probe practical, with minimal bounded output. Do not run the full audit prompt merely to test readiness.

Verify current official OpenAI API behavior before choosing the endpoint. Do not rely on obsolete historical API assumptions.

## UX behavior

Settings should distinguish at least:

- local configuration present but not yet live-checked;
- READY after successful explicit check;
- NOT_CONFIGURED;
- AUTH_ERROR;
- RATE_LIMITED;
- NETWORK_ERROR/provider unavailable;
- invalid/unavailable model where applicable.

Do not label configuration presence alone as READY.

The existing environment-only credential fallback remains permitted. Add concise non-secret setup guidance explaining that `OPENAI_API_KEY` must exist in the Windows environment visible to the H!veAI process and that H!veAI never displays the key. Do not ask the owner to paste a key into Codex/GitHub/source.

---

# FINDING M16S-V02-F02 — MAJOR — RUNTIME-AUTHORITATIVE PROVIDER/MODEL/VERSION PERSISTENCE

## Current incorrect behavior

A successful structured model response currently contains `model` and `modelVersion` fields, and `parse_model_output()` assigns:

- `auditor_provider = OPENAI_GPT`
- `auditor_model = parsed.model`
- `auditor_version = parsed.model_version`

Those latter values are generated content. The model can return `null`, a typo, or an arbitrary/spoofed identity. The runtime already knows the exact `AuditModel` instance, configured model name, and provider adapter version that actually executed the request.

The M16S V01 contract required the provider/model/version actually used to be persisted.

## Required target behavior

Provider execution identity must come only from trusted runtime authority.

On a successful model result:

- `auditor_provider` must come from `model.provider()`;
- `auditor_model` must come from `model.model()`;
- `auditor_version` must come from `model.version()`;
- model-generated JSON must not be able to override, erase, or spoof those fields.

Preferred cleanup:

- remove `model` and `modelVersion` from the required Structured Output schema and raw model-result contract if they serve no independent semantic purpose; or
- ignore them completely for persisted execution provenance if retaining them is necessary for backward compatibility.

Do not create a second audit result schema.

The same trusted metadata semantics must apply to successful OpenAI runs, fixture tests where relevant, malformed/degraded results, and future provider implementations through the `AuditModel` boundary.

## Required direct tests

At minimum prove:

1. a successful OpenAI mock whose generated JSON claims a different model cannot alter persisted/evaluated `auditor_model`;
2. a generated null/missing model identity cannot erase runtime model provenance;
3. `auditor_provider`, `auditor_model`, and `auditor_version` exactly equal the executing provider object's values;
4. persistence round-trip returns those trusted values;
5. malformed/provider failure remains truthful and does not invent successful provenance.

---

# FINDING M16S-V02-F03 — MINOR — MAKE LOCAL WORKSPACE CONTROL VISIBLY DISCOVERABLE

## Current state

The V01 functional defect is fixed: all non-archived project cards now expose the existing repair callback and correctly compute:

- Attach local workspace;
- Change local workspace;
- Repair local workspace.

However, the visible card renders only a MapPin icon. The truthful label exists in `aria-label`/`title`, and the dialog still uses generic `Path repair` / `Repair project path` copy even for a first attach or normal change.

The owner originally reported that the local-files configuration area appeared to have disappeared. An icon-only repair affordance is too easy to miss for this product requirement.

## Required target behavior

Without redesigning Projects:

- expose a compact visible local-workspace action/label on the project card or immediately adjacent project metadata;
- show the correct visible wording for Attach / Change / Repair according to current state;
- when the dialog opens, its heading/helper text must match the operation instead of always saying Repair;
- continue using the exact existing safe `repairProjectPath` backend;
- do not introduce a second path mutation route;
- do not add a ninth project or use Add Project for attachment;
- preserve project identity, history, GitHub TASKS truth, and portfolio count.

Cockpit Settings support is optional for this V02 closure if the Projects surface is clearly discoverable and functional.

---

# TRACKER STATE

Update current/prospective tracking only after implementation:

- M16 remains OPEN;
- M16S V02 becomes `IMPLEMENTATION_COMPLETE / AWAITING_INDEPENDENT_STRICT_AUDIT_AND_OWNER_NATIVE_ACCEPTANCE`;
- Required Actor becomes HUMAN after builder completion;
- M21 and M21-R01 through M21-R03 remain PASS/CLOSED;
- M17 remains NOT ACTIVATED/BLOCKED;
- preserve all historical V01 artifacts unchanged.

Do not change the 20-milestone denominator.

---

# REQUIRED VALIDATION

Run the narrowest focused tests first, then required regressions.

At minimum:

1. Rust readiness-probe tests with injected/mock transport covering READY, NOT_CONFIGURED, AUTH_ERROR, RATE_LIMITED, network/5xx, invalid model and bounded response handling.
2. Rust successful audit identity tests proving generated model/modelVersion cannot spoof runtime provider identity.
3. Persistence round-trip test for trusted auditor provider/model/version.
4. Existing provider request/privacy/structured-output/freshness tests.
5. Frontend Settings readiness-state tests proving the button invokes live readiness rather than only local configuration presence.
6. Frontend Projects tests proving visible Attach/Change/Repair wording and callback behavior.
7. Existing registry identity/duplicate/path-repair tests.
8. Full frontend test suite.
9. Relevant full Rust test suite under repository policy.
10. TypeScript typecheck.
11. Production frontend build.
12. `git diff --check`.
13. Governed native QA publication through the existing safe publisher after all tests pass.
14. Validate the stable `Desktop\H!veAI.lnk` target/icon and no terminal flash regression.

Do not use a real owner API key in automated tests. Use deterministic mocked provider transport.

A real provider check is an OWNER native acceptance step after independent source audit PASS, unless a credential already exists securely and testing it cannot expose or consume sensitive owner state.

---

# PROHIBITED SHORTCUTS

Do not:

- make READY mean only key/model strings exist;
- persist a model name generated by the model as execution provenance;
- hard-code a fake successful readiness state;
- add a fake provider fixture to production;
- store the API key in SQLite, frontend state, localStorage, source, committed `.env`, logs, prompt files, audit rows, or diagnostics;
- print an Authorization header;
- weaken strict audit schema/evidence/freshness validation;
- create a second audit schema;
- create a second local-workspace backend path;
- create a ninth project;
- modify another GitHub repository;
- activate M17 or implement Claude work;
- rewrite historical V01 prompt/log/audit artifacts.

---

# REQUIRED BUILDER LOG

Create exactly:

`docs/H!veAI/codex-logs/M16S_POST_M21_LOCAL_WORKSPACE_AND_GPT_AUDIT_PROVIDER_CLOSURE_V02_LOG.md`

The log must include:

- synchronized starting GitHub SHA;
- implementation commit SHA(s) known before log publication;
- exact source symbols changed for each V02 finding;
- chosen official readiness endpoint/contract and why it is appropriate;
- proof that readiness does not persist an H!veAI audit or expose the credential;
- proof that trusted runtime provider/model/version wins over spoofed/null generated fields;
- visible local-workspace UX result;
- focused test commands/counts;
- full frontend/Rust/typecheck/build/diff-check results;
- native publication evidence;
- explicit statement that M17 remains blocked and no Claude implementation was done.

Do not require the log file to contain the SHA of the commit that first creates itself. After publishing the log, verify its commit and final remote `main` and return those SHA values in the final response.

---

# COMPLETION GATE

Return `COMPLETE` only when all V02 source/test/publication requirements pass and GitHub synchronization is proven.

Return `SYNC_BLOCKED` if safe Git synchronization cannot be completed.

Return `PROVIDER_REMEDIATION_BLOCKED` if the official API cannot support a truthful bounded readiness implementation without violating the security contract; explain the exact blocker and do not fake READY.

Final owner-facing response must show only:

- GitHub V02 log URL/path;
- implementation commit SHA(s);
- V02 log commit SHA;
- final GitHub `main` SHA;
- concise status.
