# M16S Post-M21 Local Workspace and GPT Audit Provider Closure V01 Strict Audit

## 1. VERDICT

**FAIL**

M16 cannot be accepted as PASS/CLOSED from the current native owner evidence. Two material closure gaps remain after the standalone M21 migration:

1. the GitHub-first project model no longer exposes a usable owner-facing action for attaching or changing a local workspace on an already ACTIVE remote project; and
2. the production Audit Engine never invokes a real GPT provider because the public production `run()` path is hard-wired to `UnavailableAuditModel`.

The screenshots supplied by the owner are consistent with current production source: Audit Center truthfully returns `CONDITIONAL`, `LOW` confidence, `HIGH` regression risk, and `UNAVAILABLE` model status because no production GPT provider is wired.

M16 remains OPEN. M17 Claude Code Adapter must not be activated until this bounded closure work is implemented, independently audited, and owner-native accepted.

## 2. CONTRACT RECOVERY

The original M16 GPT Audit Engine contract required a production-grade evidence-first Audit Engine, while explicitly allowing a truthful `UNAVAILABLE` fallback when no direct GPT provider existed. That fallback prevented fake PASS results, but it did not itself deliver a working remote model provider.

The current owner is now exercising M16 as an actual product feature and expects real GPT audit execution. Therefore final M16 native acceptance requires a real configured production provider path, with the existing unavailable path retained only as a safe fallback.

The prior M16Q/M16R owner-native recovery contract also established that one logical GitHub project must be able to carry local workspace metadata on the same registry identity. Local path is metadata/capability, not a second project identity. The post-M21 GitHub-first portfolio now satisfies the one-project identity rule, but the ACTIVE remote-project UI no longer gives the owner a practical way to attach that local metadata.

M21-R03 standalone relocation has already been owner-confirmed. H!veAI's active local checkout is now outside the retired AI-Commerce parent, so the H!veAI project must be attachable to that standalone local checkout without creating a ninth/duplicate logical project.

## 3. BRANCH / HEAD / DIFF SCOPE

Repository: `Sekiph82/H-veAI`

Branch: `main`

Audited GitHub HEAD before this audit publication:

`accd51f422af7135c2ddaa36b82f6d6e43fa62de`

This audit is diagnostic only. No product implementation is changed by this audit commit.

Primary inspected implementation areas:

- `src-tauri/src/audit_engine.rs`
- `src/components/ProjectRegistryCard.tsx`
- `src/projectRegistry.ts`
- `src-tauri/src/projects/registry.rs`
- root `TASKS.md`
- `CODEX_ROADMAP.md`
- historical M16 whole-milestone contract
- historical M16Q/M16R native registry recovery contract

## 4. ACCEPTANCE CRITERIA MATRIX

| Requirement | Result | Evidence / note |
| --- | --- | --- |
| Audit evidence collection remains bounded/evidence-first | PASS | Owner UI shows verified/truncated/unavailable classifications rather than fabricated evidence. |
| Missing GPT provider never fabricates PASS | PASS | Current native behavior is truthful `CONDITIONAL`/`UNAVAILABLE`. |
| Real production GPT Audit provider executes configured audits | **FAIL** | Production `run()` directly calls `run_with_model(..., &UnavailableAuditModel)`. |
| Strict structured audit result validation retained | PARTIAL | Existing parser/contracts exist, but no production remote provider reaches them. |
| Provider auth/quota/network failures remain truthful | UNVERIFIED | No production network provider path exists to exercise these states. |
| API secret never persisted/logged/exposed to frontend | UNVERIFIED for future provider | No API integration currently exists; remediation must establish this safely. |
| ACTIVE GitHub-first project can attach local workspace | **FAIL** | UI only exposes Repair path when `status === MISSING`. |
| Local path backend capability exists | PASS | `repairProjectPath` frontend IPC and `repair_project_path` native implementation remain present. |
| Attached local path validates repository identity | PASS in backend | Repair path validates canonical path and rejects remote identity mismatch/ambiguity. |
| Attaching local workspace preserves one logical project | REQUIRED / currently inaccessible in normal ACTIVE UI | Must update same registry record, never Add Project duplicate. |
| H!veAI standalone checkout can be attached to H!veAI project | PENDING OWNER ACCEPTANCE | Must be performed through repaired UI after implementation. |
| Eight-project portfolio remains eight | PASS currently | Do not regress this while restoring local capability. |
| M21-R03 closure reflected in current tracker | **FAIL / stale** | Root tracker top still says M21-R03 awaits audit/owner confirmation even though both occurred. |
| Roadmap current-state text no longer says M21 not started | **FAIL / stale** | Current roadmap text retains obsolete pre-M21 forward-looking state. |
| M17 stays blocked until M16 closure | PASS currently | Must remain so during M16S. |

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

There is no new builder log for M16S yet. Current repository truth is sufficient to reproduce both owner-observed behaviors.

### Audit provider truth

The production source defines an `AuditModel` abstraction and an `UnavailableAuditModel`. The unavailable model identifies itself as `OPENAI_GPT / UNCONFIGURED / UNAVAILABLE` and returns `AUDIT_MODEL_UNAVAILABLE`.

Critically, production `pub fn run(...)` does not resolve configuration and does not instantiate an HTTP/OpenAI provider. It always executes:

`run_with_model(database, request, &UnavailableAuditModel)`

Fixture models exist for tests, but test fixtures are not production capability.

Therefore the native message `GPT audit provider is not configured` is not caused by the owner missing a hidden UI setting. The application currently has no wired production GPT provider.

### Local workspace truth

The registry backend and frontend IPC still support path repair/rebinding. However `ProjectRegistryCard` renders the Repair path action only when `project.status === 'MISSING'`.

The GitHub-first seeded projects are normally `ACTIVE`, including remote-only projects whose local path is blank/unbound. That state has no attach/change-local-workspace action in the card UI. The generic Add Project flow still exists, but using Add Project for an already tracked GitHub identity is not the correct UX or acceptance contract because it risks duplicate/merge behavior and does not explicitly communicate attachment to the existing logical project.

## 6. FILE / SYMBOL EVIDENCE

### `src-tauri/src/audit_engine.rs`

- `AuditModel` is provider-neutral.
- `UnavailableAuditModel` is the only production model implementation currently identified.
- `FixtureAuditModel` is test-only.
- production `run()` is hard-wired to `UnavailableAuditModel`.
- bounded audit input/output/evidence constants and structured result types already exist and should be reused rather than redesigned.

### `src/components/ProjectRegistryCard.tsx`

The action is conditionally rendered only for `MISSING` projects:

`project.status === 'MISSING' ? ... onRepair ... : null`

An ACTIVE remote-only project therefore cannot attach a local checkout from this card.

### `src/projectRegistry.ts`

`repairProjectPath(projectId, path)` still invokes native `hiveai_project_repair_path`, so the frontend/native contract required for safe rebinding already exists.

### `src-tauri/src/projects/registry.rs`

`repair_project_path`:

- canonicalizes/validates the replacement path;
- rejects duplicate local paths;
- probes Git metadata;
- compares repository type and remote identity;
- rejects mismatching/ambiguous remotes;
- updates the existing project row rather than creating a second logical project;
- marks project truth dirty for reconciliation.

This is a strong implementation base for an explicit Attach/Change Local Workspace UI.

## 7. FOCUSED TEST EVIDENCE

Existing tests prove the unavailable audit fallback and fixture model behavior, but that is insufficient to prove a real production GPT provider.

M16S must add direct focused tests for:

- configured provider request construction;
- strict structured-result decoding;
- malformed response rejection;
- authentication failure;
- quota/rate-limit failure;
- timeout/network failure;
- secret redaction;
- unavailable fallback when no credential is configured;
- ACTIVE remote-only project local attachment;
- ACTIVE local project local-path change;
- remote identity mismatch rejection;
- duplicate project prevention;
- eight-project portfolio invariance.

## 8. REGRESSION EVIDENCE

Current owner screenshots show that the accepted eight-project remote-truth UI, Tasks, and Cockpit continue to function. M16S must preserve these accepted M21-R02/M21-R03 behaviors.

The local workspace fix must not revert to the historical 8+8 duplicate registry failure. The real GPT provider must not weaken existing evidence classifications, freshness gates, audit history, or remediation provenance.

## 9. SECURITY / SAFETY REVIEW

The future GPT provider is a security-sensitive network boundary.

Requirements:

- raw API keys must never be committed, written to SQLite, localStorage, audit rows, logs, diagnostics, prompts, or frontend-readable state;
- prefer OS-backed secure credential storage on Windows; permit `OPENAI_API_KEY` environment input as a non-persisted fallback if needed;
- expose only readiness metadata such as configured/not configured, provider, model and last error category;
- use HTTPS only;
- set explicit connect/request timeouts and bounded request/response sizes;
- redact Authorization values and provider response diagnostics;
- classify 401/403, 429, timeout/network, 5xx, malformed/invalid structured output separately and truthfully;
- never convert provider failure into PASS;
- use the existing strict audit result contract and reject schema-invalid output;
- do not enable unrelated OpenAI tools such as web/file search for M16 audit execution unless separately specified;
- default remote audit requests to non-persistent API storage where the provider supports it.

Local workspace attachment must retain canonical-path validation, containment/repository checks and duplicate prevention. It must never move/delete owner files or auto-reset/reconcile the attached Git checkout.

## 10. ARCHITECTURE CONSISTENCY

The required remediation fits existing architecture:

- GitHub/root `TASKS.md` remains canonical project/task truth.
- Local workspace is optional capability metadata for local Git, Files, Agents and worktree operations.
- One GitHub identity remains one logical project.
- `AuditModel` remains provider-neutral; OpenAI is one production implementation, not a hard-coded architecture throughout the Audit Engine.
- M17 remains the Claude Code Adapter milestone and must not be pulled into M16S.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

Current tracking has become stale after the owner completed M21-R03 final launch acceptance and deleted the retired local parent.

M16S must reconcile only current/prospective tracker and roadmap state:

- mark M21-R03 and the standalone retirement/local-parent closure as accepted/completed;
- set M16S as the active closure work while M16 remains OPEN;
- keep M17 NOT ACTIVATED until independent M16S audit + owner native acceptance;
- remove obsolete current/future statements that claim M21 is not started;
- preserve historical immutable prompts/logs/audits containing their original temporal statements.

## 12. FINAL REPOSITORY STATE

Before this diagnostic audit publication, H!veAI `main` is `accd51f422af7135c2ddaa36b82f6d6e43fa62de`.

The new audit commit will become the next remote `main` HEAD. No product source changes are made by this audit.

## 13. OPEN CROSS-MILESTONE FINDINGS

- M16S-F01 BLOCKER: no real production GPT Audit provider path.
- M16S-F02 MAJOR: ACTIVE GitHub-first projects cannot explicitly attach/change a local workspace through normal project UI.
- M16S-F03 MAJOR: canonical current tracker/roadmap state is stale after M21 completion.

M17 remains blocked by M16 closure.

## 14. DEFECTS BY SEVERITY

### M16S-F01 — BLOCKER — Production Audit Engine is permanently UNAVAILABLE

`audit_engine::run()` always uses `UnavailableAuditModel`. A configured real GPT provider cannot currently execute.

### M16S-F02 — MAJOR — Local workspace attachment UX is missing for ACTIVE remote projects

Backend rebinding exists, but normal ACTIVE GitHub-first cards hide the only path-repair action. The owner cannot explicitly bind `Desktop\H!veAI` to the existing H!veAI project identity.

### M16S-F03 — MAJOR — Post-M21 current tracking is stale

The canonical top status still awaits gates that have already passed; current roadmap status contains obsolete `M21 not started` language.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

Do not turn M16S into a generic multi-provider AI-settings redesign. Implement one clean production OpenAI audit provider behind the existing `AuditModel` boundary, with configuration/readiness primitives that can support future providers.

Likewise, do not redesign Project Registry. Restore explicit local-workspace attachment/change semantics on the existing logical project record.

## 16. UNVERIFIED ITEMS

- Whether the owner already has an OpenAI API key with API billing/credits available.
- Exact preferred GPT model for audit execution. The model should be an explicit configurable setting rather than a scattered source constant.
- Whether Windows Credential Manager integration can be added without unacceptable dependency/runtime cost; Codex must inspect current project constraints and use the safest supported OS-backed mechanism.

These unknowns must not be papered over by storing secrets insecurely.

## 17. REGRESSION RISK

**MEDIUM**

The local attachment change touches Registry UX and project identity behavior, where duplicate-project regressions were previously severe. The GPT provider introduces a new network/credential boundary. Both are bounded but security-sensitive.

## 18. AUDIT CONFIDENCE

**HIGH**

Both primary findings are directly reproducible from owner-native screenshots and current production source. The hard-wired unavailable provider is explicit. The hidden ACTIVE-project repair action is explicit. The stale tracker state is directly visible in root `TASKS.md` and current roadmap text.

## 19. FINAL VERDICT

**FAIL**

Do not close M16 and do not activate M17 yet.

M16S must restore explicit local-workspace attachment for the existing GitHub-first logical project, wire a real secure production OpenAI GPT Audit provider while retaining truthful unavailable fallback behavior, and reconcile post-M21 current tracker truth.

## 20. REQUIRED REMEDIATION

Create one bounded M16S V01 implementation cycle covering exactly:

1. **M16S-F01:** real secure OpenAI GPT Audit provider behind existing `AuditModel`, strict structured output, explicit readiness/error states, no secret persistence/exposure, existing unavailable fallback preserved;
2. **M16S-F02:** Attach/Change Local Workspace action for existing ACTIVE GitHub-first projects using the existing safe path-repair identity validation, with no duplicate logical project creation;
3. **M16S-F03:** current tracker/roadmap reconciliation after accepted M21 standalone migration/retirement.

Required owner acceptance after implementation:

- attach the standalone H!veAI checkout to the existing H!veAI project through the native UI and prove portfolio count remains 8;
- run a real native GPT audit with a configured provider and prove `auditor_provider/model` are real, model status is not `UNAVAILABLE`, and a strict structured verdict/coverage/findings result is persisted without exposing the API key;
- verify unavailable/auth/quota/network failures still fail truthfully rather than fabricating PASS.
