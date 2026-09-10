# M16 GPT Audit Engine — Unified Whole-Milestone Implementation Prompt

Date: 2026-09-08
Product: H!veAI
Repository: `Sekiph82/AI-Commerce-HQ`
Branch: `H!veAI`
Milestone: **M16 — GPT Audit Engine**
Authority: **single authoritative whole-milestone builder prompt**

## 0. Execution mode: one continuous M16 run

This prompt is intentionally unified.

**Do not stop after M16.01, M16.02, M16.03, or any other subpackage.**
Do not return control after an intermediate subsection merely because that subsection is complete.
Proceed continuously through **M16.01 → M16.08** in one implementation run unless a genuine safety/blocking condition makes further execution impossible.

Subpackage numbers exist only for traceability and evidence mapping.

The required builder flow is:

`sync → close accepted M15 state → activate M16 → inspect current architecture → implement M16.01-M16.08 continuously → test → governed publish → create immutable M16 log → commit → push`

Do **not** activate M17.
Do **not** start M21.

The builder must not claim independent acceptance of its own work. M16 implementation may be complete at the end of this run, but final M16 PASS/CLOSED requires the independent strict audit that occurs after the builder log is produced.

---

# 1. Starting authority and milestone state

The following acceptance evidence is authoritative for this run:

- M14 and all accepted remediation stages are PASS/CLOSED.
- M15 Prompt Engine implementation and M15A-M15D remediation chain are complete.
- M15C `View result in Agents` handoff was manually accepted by the user.
- M15D final result placement was manually and visually accepted by the user.
- The user explicitly waived an additional manual duplicate-dispatch test; M15A automated race/replay/single-use tests remain the accepted evidence for that invariant.
- M15 may therefore be marked **PASS/CLOSED** at the start of this M16 run.
- Strict roadmap progress becomes **16 / 20 = 80%** when M15 is closed and M16 is activated.
- M17-M20 remain planned/blocked.
- M21 remains planned/not started.

Update canonical trackers/documentation truthfully at the start of the implementation so they reflect:

- M15 = PASS/CLOSED
- M16 = ACTIVE / IMPLEMENTING
- roadmap progress = 16/20 = 80%
- M17 not activated
- M21 not started

Do not rewrite historical immutable logs/audits.

---

# 2. M16 product objective

Implement a production-grade **GPT Audit Engine** that independently evaluates implementation evidence for registered H!veAI projects/tasks.

The Audit Engine must answer:

1. What exact requirements/acceptance criteria were supposed to be satisfied?
2. What actually changed in the repository?
3. What tests actually ran and what do their direct bodies/results prove?
4. What production symbols/configuration are actually present?
5. What architecture/governance constraints apply?
6. Are builder claims corroborated by source/test/Git evidence?
7. What is the resulting structured verdict?
8. If defects exist, what exact bounded findings should feed a remediation prompt?
9. Can the resulting implementation be re-audited without destroying the prior audit history?

The Audit Engine must be **evidence-first, source-level, reproducible, bounded, and independent from builder self-assessment**.

Builder logs are secondary claims only.

---

# 3. Mandatory inherited boundaries

Preserve every accepted M00-M15 boundary, especially:

- registered ACTIVE-project confinement;
- no arbitrary filesystem root from frontend;
- Git Engine remains authority for local Git evidence;
- Task Intelligence / Workflow remain authorities for task state;
- M08 remains source-discovery authority;
- Prompt Engine remains authority for prompt generation/versioning/review/approval/dispatch;
- Agent Session Center remains authority for provider execution/session evidence;
- prompt/session provenance is immutable;
- M15A durable single-use dispatch remains intact;
- M14E dedicated final assistant response remains intact;
- no arbitrary executable / shell / raw argv / PID control;
- no new broad Tauri permission;
- no secrets in prompt/query/log/UI evidence;
- builder self-report is never independent audit evidence;
- governed publication remains mandatory;
- no fake PASS from a mocked path when production/native evidence is unavailable;
- no M17/M21 implementation.

---

# 4. M16.01 — Audit input contract

Implement one explicit, typed, bounded audit input contract.

The audit input must be derived from existing H!veAI authorities and persisted evidence, not from unbounded freeform text.

## 4.1 Task requirements / acceptance criteria

For task-scoped audits include, where available:

- project ID;
- task ID;
- task title;
- task workflow state;
- task requirements / acceptance criteria;
- dependencies/blockers/gates;
- required actor/milestone;
- relevant source provenance.

For project/freeform audits:

- explicitly mark task-specific evidence unavailable/not applicable;
- do not fabricate acceptance criteria.

## 4.2 Actual Git diff / changed files

Use the existing Git Engine authority.

Capture bounded evidence such as:

- branch;
- HEAD SHA;
- staged files;
- unstaged files;
- untracked files;
- conflict state;
- bounded production diff;
- truncation state;
- repository identity if available.

Never accept an agent's changed-file claim as Git truth.

## 4.3 Test results

Create/extend a durable test-evidence contract sufficient for M16.

Audit inputs should distinguish:

- test command/suite identity where already persisted/available;
- pass/fail/skip counts;
- exit status;
- timestamp;
- bounded output/result summary;
- source/provenance of the test evidence;
- direct test-body inspection evidence where M16 performs it.

If historical test execution is not durably available, mark it **UNVERIFIED / UNAVAILABLE** rather than inventing PASS.

Do not create a generic arbitrary command runner just to execute tests.

## 4.4 Architecture / governance rules

Collect bounded relevant constraints from existing approved authorities such as:

- AGENTS / CONSTITUTION / Project Dashboard authority;
- M08 discovered instruction/source references;
- project-scoped governance;
- explicit task/milestone constraints.

Do not dump every governance file into every audit.

Materialize only relevant bounded evidence and retain provenance.

## 4.5 Builder logs as secondary claims only

Builder logs may be included only as **CLAIM** evidence.

They must never:

- satisfy a requirement by themselves;
- upgrade UNVERIFIED source/test evidence to VERIFIED;
- determine PASS without corroboration.

The input model must carry evidence classification, for example:

- VERIFIED
- CORROBORATED
- CLAIM_ONLY
- UNVERIFIED
- UNAVAILABLE
- STALE
- TRUNCATED

Use a deterministic enum/contract rather than vague prose.

## 4.6 Bounds

Define explicit constants for:

- max audit input bytes;
- max evidence items;
- max Git diff bytes;
- max source/test snippets;
- max builder-log claim bytes;
- max requirements;
- max findings;
- max persisted model-output bytes.

All truncation must be explicit and UTF-8 safe.

---

# 5. M16.02 — Structured audit result

Implement a typed structured audit result contract.

## 5.1 Verdict

Exactly:

- `PASS`
- `CONDITIONAL`
- `FAIL`

No hidden fourth success state.

Suggested semantics:

- PASS: no unresolved BLOCKER/MAJOR finding and requirements are sufficiently verified.
- CONDITIONAL: no proven blocker but material evidence remains UNVERIFIED/UNAVAILABLE or only MINOR/NOTE conditions prevent full confidence.
- FAIL: at least one verified BLOCKER/MAJOR correctness/security/contract failure, or implementation materially violates required acceptance behavior.

Codify the semantics and test them.

## 5.2 Finding severity

Exactly:

- `BLOCKER`
- `MAJOR`
- `MINOR`
- `NOTE`

Each finding must contain at minimum:

- stable finding ID;
- severity;
- title;
- concise detail;
- requirement/evidence references;
- source/test/Git locators where available;
- confidence;
- status/open/closed history semantics;
- remediation guidance;
- whether it blocks release.

Do not allow model-generated arbitrary severity labels.

## 5.3 Requirement coverage

Produce explicit requirement coverage rows:

- requirement ID/reference;
- requirement text;
- VERIFIED / PARTIAL / UNVERIFIED / FAILED / NOT_APPLICABLE;
- evidence refs;
- rationale.

## 5.4 Confidence

Use a bounded, explainable confidence representation.

Do not display fake precision. Prefer a small explicit enum such as:

- HIGH
- MEDIUM
- LOW

If a numeric score is also used internally, it must not be presented as scientifically calibrated probability.

## 5.5 Regression risk

Use an explicit enum, for example:

- LOW
- MEDIUM
- HIGH
- CRITICAL

Risk must cite the evidence/reason that produced it.

## 5.6 Machine contract

The GPT/model response must be parsed into a strict schema.

Do not persist arbitrary model prose as authoritative audit truth without schema validation.

If schema validation fails:

- audit run must fail truthfully or become CONDITIONAL/UNVERIFIED according to a deterministic contract;
- malformed model output must be preserved only as bounded diagnostic evidence;
- never invent missing findings/verdict.

---

# 6. M16.03 — Source-level verification

This is the defining requirement of M16.

The Audit Engine must not be a "summarize the builder log" feature.

## 6.1 Inspect production symbols / configuration

For evidence selected by the bounded audit planner, inspect actual production source/configuration.

Examples:

- function/type/module definitions;
- exact production branches;
- ACL/capability entries;
- DB migrations;
- route/UI production behavior;
- constants/bounds;
- adapter/process invocation code;
- Git/prompt/session provenance fields.

Use bounded file/range reads. Do not recursively ingest an entire repository.

## 6.2 Inspect direct test bodies

When a test is claimed to cover a requirement, inspect the actual test body or relevant bounded range, not merely the test file name or builder summary.

The Audit Engine must distinguish:

- test name claims behavior;
- test body actually exercises production behavior;
- test only mocks the behavior;
- test asserts the relevant invariant;
- test is unrelated/misleading.

## 6.3 Detect misleading test names / claims

Implement explicit heuristics/evidence classification for cases such as:

- test title says "rejects X" but no assertion proves rejection;
- test mocks the production boundary entirely;
- test asserts only rendered label text while claiming native/process correctness;
- skipped/ignored test appears in a builder claim;
- fixture proves a helper but not the production path;
- builder log claims count/path inconsistent with source/test evidence.

Do not automatically FAIL every mocked test. Classify what it does and does not prove.

## 6.4 Verify final branch / diff scope

Audit must record:

- audited repository;
- branch;
- HEAD;
- baseline/base reference when available;
- actual changed-file scope;
- whether unrelated files exist;
- whether audit evidence is stale relative to current HEAD.

If branch HEAD changes after audit input collection, mark the run stale/invalid rather than silently auditing the wrong code.

## 6.5 Source evidence reader

Implement a bounded, allowlisted source reader for audit purposes.

It must:

- operate only inside the registered project root;
- reject traversal/symlink/junction escape;
- obey file type and byte limits;
- avoid secret/config credential files by policy;
- never become a general unrestricted frontend file-read primitive;
- expose only audit-selected bounded evidence through narrow native commands.

---

# 7. M16.04 — Audit persistence

Use/extend the existing M04 audit schema through additive versioned migration(s).

## 7.1 Persist audit

Persist immutable audit-run identity including:

- audit ID;
- project ID;
- optional task ID;
- optional agent session ID / prompt version refs;
- audit type;
- audited branch;
- audited HEAD;
- input manifest/hash;
- audit result schema version;
- verdict;
- confidence;
- regression risk;
- created/start/end timestamps;
- auditor provider/model/version if a model is used;
- audit state.

## 7.2 Persist findings

Persist structured findings separately.

Preserve stable finding identity and status history.

Do not overwrite previous audit findings to simulate re-audit.

## 7.3 Link evidence

Persist/link bounded evidence references to:

- project;
- task;
- agent session;
- prompt/version;
- Git snapshot/diff;
- tests/test runs;
- source snippets/locators;
- builder-log claim evidence.

Use IDs/hashes/locators, not unbounded duplicate blobs where existing durable evidence is already referenced.

## 7.4 Preserve re-audit history

A re-audit must create a new audit run.

It may link:

- prior audit ID;
- remediation prompt ID/version;
- remediation agent session ID;
- superseded/closed findings;
- reopened findings;
- new findings.

Historical audits remain immutable.

Do not rewrite a failed prior audit into PASS.

## 7.5 State machine

Use explicit states, for example:

- PREPARING
- READY
- RUNNING
- COMPLETED
- FAILED
- STALE
- CANCELLED/STOPPED if actually supported

Avoid ambiguous booleans.

---

# 8. GPT/model execution architecture

M16 is named GPT Audit Engine, but the provider boundary must remain narrow and truthful.

Before implementing model execution:

1. inspect what OpenAI/GPT integration already exists in the repository;
2. do not invent unavailable credentials/API contracts;
3. do not add plaintext secret storage;
4. do not silently reuse Codex as if it were the GPT audit model unless the product contract explicitly chooses that path;
5. if no direct GPT API/provider exists, implement a provider-neutral AuditModel interface and a production-safe configuration boundary, with fixture/local deterministic audit model for tests only;
6. native user-facing audit must clearly report UNAVAILABLE if no configured GPT audit provider exists.

If a first-party OpenAI API/provider path is implemented:

- use least-privilege secret handling;
- never persist raw API key;
- never expose key to frontend;
- redact diagnostics;
- use bounded request/response sizes;
- set explicit model/config through settings rather than scattered constants;
- persist model/version used;
- enforce strict structured output/schema parsing;
- treat network/model failure truthfully.

The Audit Engine must remain architecturally separable from advertisements or unrelated network content.

---

# 9. M16.05 — Remediation loop

Implement the full bounded remediation loop, but preserve human review/approval.

## 9.1 Convert failed findings to remediation input

For a FAIL/CONDITIONAL audit with actionable findings:

- allow user to select relevant open findings;
- create bounded remediation context;
- include exact finding IDs/severity/detail/evidence refs;
- include task/project constraints required for a safe fix;
- do not include unrelated audit history by default.

## 9.2 Send through Prompt Engine

Do **not** directly launch Codex/Claude from Audit Center.

Use the existing Prompt Engine authority:

- create/generate `REMEDIATION` prompt draft;
- preserve exact audit/finding provenance;
- user reviews;
- user edits if desired;
- user explicitly approves;
- user selects Codex/Claude;
- Prompt Engine performs single-use dispatch.

## 9.3 Re-audit resulting implementation

After remediation session completion:

- expose a clear **Re-audit** action;
- create a new audit run;
- re-collect current Git/source/test evidence;
- compare to prior audit;
- mark which prior findings remain open, are closed, or are superseded;
- preserve full chain.

Do not auto-declare a finding closed because builder says it was fixed.

---

# 10. M16.06 — Audit Center UI

Replace the current Audit Center placeholder with production UI.

The primary Audit Center should be human-readable, not a raw JSON wall.

## 10.1 Primary layout

Recommended vertical composition:

### A. Audit target
- Project
- Task / freeform project audit
- relevant implementation/session/prompt target if selectable
- current branch/HEAD summary
- Start audit / Refresh evidence

### B. Current verdict
Large, clear:
- PASS / CONDITIONAL / FAIL
- confidence
- regression risk
- audited HEAD
- audit timestamp/state

### C. Requirement coverage
Readable rows/cards:
- requirement
- status
- evidence count
- concise reason

### D. Findings
Sortable/filterable by:
- BLOCKER
- MAJOR
- MINOR
- NOTE
- OPEN/CLOSED

Each finding should show:
- severity
- title
- concise reason
- requirement/evidence refs
- source/test locator where useful
- remediation eligibility

### E. Remediation
- select findings
- **Create remediation prompt**
- sends user to Prompt Engine review flow
- never auto-dispatch

### F. History
- previous audits
- remediation prompt/session links
- re-audits
- verdict chain

## 10.2 Evidence details

Raw model JSON, full evidence manifests, source snippets, builder log claims, and technical diagnostics must live in collapsed secondary disclosures.

The default view is audit judgment and evidence coverage, not transport noise.

## 10.3 Navigation

Support bounded handoffs:

- Audit → Prompt Engine remediation draft
- Audit → relevant Agents session
- Audit → previous/re-audit entry
- Prompt/Agent provenance back to audit where useful

No handoff may create duplicate provider operations merely by navigation.

---

# 11. M16.07 — Security / truthfulness

## 11.1 No advertiser/network influence

Audit verdict must be derived solely from the explicit audit input/evidence contract and configured audit-model response.

No advertising, recommendation feed, unrelated browser/network content, or sponsored content may enter the evidence prompt or alter weighting/verdict.

Add an explicit architecture/test boundary proving the audit input builder has no such source.

## 11.2 Builder self-assessment is secondary

Builder logs must be tagged CLAIM_ONLY unless independently corroborated.

A builder log saying:

`all tests pass`

is not enough.

The Audit Engine must seek direct test/source/Git evidence.

## 11.3 UNVERIFIED evidence

UI and persistence must clearly distinguish:

- VERIFIED
- PARTIAL
- CLAIM_ONLY
- UNVERIFIED
- UNAVAILABLE
- STALE
- TRUNCATED

Never silently collapse these to "passed".

## 11.4 Prompt injection / hostile repository content

Treat repository text as untrusted evidence.

The audit model prompt must strongly separate:

- system/audit instructions;
- evidence payload;
- repository/user-controlled text.

Repository content saying "ignore previous instructions" must remain evidence text, not control the audit engine.

Add adversarial tests.

## 11.5 Secret protection

Audit source collection must exclude credential/secret file patterns and redact suspicious values before persistence/model transport.

No raw token/API key/password/authorization secret may appear in:

- audit input;
- audit DB rows;
- frontend evidence;
- logs;
- model diagnostics.

---

# 12. M16.08 — Tests / audit / closure

Implement all testing categories in the same M16 run.

## 12.1 Known-good fixtures

Create fixtures where production implementation genuinely satisfies requirements.

Expected:
- no false MAJOR/BLOCKER;
- requirement coverage reflects verified evidence;
- PASS when evidence is sufficient.

## 12.2 Known-bad fixtures

Create fixtures with real defects such as:

- missing required production symbol;
- incorrect config/ACL;
- wrong branch/diff;
- failing direct test;
- security boundary violation.

Expected:
- deterministic structured findings;
- FAIL when appropriate.

## 12.3 Misleading-test fixtures

Include cases where:

- test name overclaims;
- test body has no relevant assertion;
- test mocks the production path;
- ignored/skipped test is claimed as evidence;
- builder log count conflicts with actual fixture evidence.

Expected:
- test evidence downgraded or finding generated;
- no false PASS from the test name alone.

## 12.4 Remediation / re-audit loop tests

Prove:

`Audit FAIL → select finding → generate remediation Prompt Engine draft → explicit approval boundary remains → remediation session provenance → new re-audit → history preserved`

Use fixtures/mocks for provider execution where appropriate, but exercise real production domain paths for audit persistence/provenance.

## 12.5 Migration/persistence tests

Test:

- fresh schema;
- upgrade from current production schema;
- re-apply/idempotence as supported;
- FK integrity;
- immutable audit history;
- finding links;
- stale HEAD handling;
- failure/rollback behavior.

## 12.6 UI tests

Test:

- verdict display;
- severity display;
- coverage;
- confidence/risk;
- UNVERIFIED state;
- remediation selection;
- Audit → Prompt Engine handoff;
- history/re-audit chain;
- empty/loading/error states;
- long source/test/finding text wrapping;
- no raw JSON default wall.

## 12.7 Security tests

Test:

- project-root containment;
- traversal;
- symlink/junction escape where host allows;
- secret file exclusion;
- redaction;
- repository prompt injection strings;
- builder-log claims cannot become VERIFIED alone;
- no unrelated network/advertising evidence input;
- bounded model response parsing;
- malformed schema response;
- overlarge source/diff/test/log evidence;
- stale branch HEAD.

## 12.8 Full regression/publication

Run all required H!veAI gates:

- focused M16 Rust tests;
- focused M16 frontend tests;
- migration tests;
- full serialized Rust regression;
- full frontend regression;
- TypeScript typecheck;
- frontend production build;
- `npm audit --audit-level=high`;
- Rust fmt;
- Rust all-targets check;
- Rust `pty-support` check;
- `git diff --check`;
- existing M14/M15 focused regressions;
- Prompt Engine dispatch/replay tests;
- publisher rollback harness;
- governed Tauri `--no-bundle` stable publication;
- candidate/stable SHA equality;
- PE/startup/shortcut/icon checks;
- no visible console popup.

## 12.9 Independent release-gate audit boundary

The builder must prepare all evidence required for the independent release-gate audit of M16.

However, **the builder must not call its own M16 implementation independently audited/PASS**.

At completion, write:

`M16 IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE`

The independent audit is performed outside this builder run using the immutable M16 log plus actual source/commit/test evidence.

Do not activate M17.

---

# 13. Execution gates — complete all in one run

Execute and record every gate individually.

## Synchronization / authority

1. `git fetch origin H!veAI`.
2. Fast-forward-only synchronize.
3. Confirm exact branch `H!veAI`.
4. Record starting HEAD and worktree.
5. Preserve unrelated user-owned files.
6. Read `H!veAI/TASKS.md`.
7. Read `H!veAI/CODEX_ROADMAP.md`.
8. Read M15 authoritative prompt.
9. Read M15A prompt/log/re-audit.
10. Read M15B prompt/log/re-audit.
11. Read M15C prompt/log/re-audit.
12. Read M15D prompt/log/re-audit.
13. Record user M15C/M15D native acceptance authority.
14. Mark M15 PASS/CLOSED in canonical trackers.
15. Activate M16.
16. Record progress 16/20 = 80%.
17. Confirm M17 not activated.
18. Confirm M21 not started.

## Architecture discovery

19. Inspect existing audit DB tables/migrations.
20. Inspect current Audit Center placeholder.
21. Inspect Git Engine contracts.
22. Inspect task/workflow contracts.
23. Inspect task/source discovery contracts.
24. Inspect Project Dashboard authority.
25. Inspect Prompt Engine contracts.
26. Inspect Agent Session Center provenance.
27. Inspect test-run persistence currently available.
28. Inspect existing OpenAI/GPT/provider/config capability.
29. Decide/document production AuditModel boundary from actual repository evidence.
30. Define exact M16 schema versions and bounds before coding.

## M16.01

31. Implement typed audit input contract.
32. Implement task requirement collection.
33. Implement Git diff/changed-file collection.
34. Implement test-evidence collection.
35. Implement governance/architecture collection.
36. Implement builder-log CLAIM_ONLY evidence.
37. Implement evidence verification-status enum.
38. Implement deterministic evidence ordering.
39. Implement explicit byte/item bounds.
40. Implement UTF-8-safe truncation markers.
41. Implement input manifest/hash.
42. Add collector unit/domain tests.

## M16.02

43. Implement verdict enum.
44. Implement severity enum.
45. Implement finding schema.
46. Implement requirement coverage schema.
47. Implement confidence schema.
48. Implement regression-risk schema.
49. Implement strict model output schema validation.
50. Implement malformed/unavailable model response truth.
51. Add verdict/severity/coverage tests.

## M16.03

52. Implement bounded source-level evidence planner.
53. Implement registered-root-contained audit source reader.
54. Implement production-symbol/config inspection.
55. Implement direct test-body inspection.
56. Implement misleading-test evidence classification.
57. Implement final branch/HEAD/diff-scope verification.
58. Implement stale-HEAD detection.
59. Implement secret-file exclusion.
60. Implement source/test locator provenance.
61. Add known misleading-test fixtures.

## Model execution

62. Implement/extend AuditModel interface.
63. Implement production provider boundary only from actual available repository capability.
64. Implement truthful UNAVAILABLE state when no provider configured.
65. Keep secrets native-only.
66. Bound request size.
67. Bound response size.
68. Implement structured response parser.
69. Persist provider/model/version used.
70. Add provider failure/timeout/malformed response tests.
71. Add repository prompt-injection adversarial tests.

## M16.04

72. Add additive DB migration(s).
73. Persist immutable audit runs.
74. Persist structured findings.
75. Persist evidence links.
76. Persist requirement coverage.
77. Persist confidence/risk.
78. Persist audit input hash/HEAD.
79. Persist prior-audit/re-audit links.
80. Preserve immutable prior audits.
81. Implement explicit audit state machine.
82. Add migration upgrade tests.
83. Add persistence failure/rollback tests.
84. Add re-audit history tests.

## M16.05

85. Build selected-finding remediation context.
86. Preserve exact audit/finding provenance.
87. Integrate remediation draft generation through Prompt Engine.
88. Require existing review/edit/approve boundary.
89. Preserve provider selection in Prompt Engine.
90. Preserve M15A single-use dispatch.
91. Link remediation prompt/session back to audit.
92. Add Re-audit action/domain command.
93. Re-collect fresh evidence for re-audit.
94. Compare prior/open/closed/new findings truthfully.
95. Add remediation-loop domain tests.

## M16.06

96. Replace Audit Center placeholder.
97. Implement target selector.
98. Implement current verdict panel.
99. Implement requirement coverage UI.
100. Implement findings/severity UI.
101. Implement confidence/risk UI.
102. Implement evidence verification badges.
103. Implement remediation selection UI.
104. Implement Create remediation prompt action.
105. Implement audit history UI.
106. Implement remediation/re-audit chain UI.
107. Implement bounded technical details disclosures.
108. Implement empty/loading/error states.
109. Implement long-content wrapping.
110. Add Audit → Prompt Engine navigation.
111. Add Audit → Agent session navigation where relevant.
112. Ensure navigation itself performs no provider action.
113. Add focused UI tests.

## M16.07

114. Prove builder logs cannot independently verify requirements.
115. Prove unrelated network/advertiser content has no audit input path.
116. Add UNVERIFIED/UNAVAILABLE/STALE/TRUNCATED UI/persistence handling.
117. Add prompt-injection separation tests.
118. Add secret redaction tests.
119. Add traversal/root-containment tests.
120. Add over-bound evidence tests.
121. Add wrong-project/task/session evidence isolation tests.

## M16.08 fixtures

122. Add known-good audit fixture.
123. Add known-bad missing-symbol fixture.
124. Add known-bad config/security fixture.
125. Add misleading-test-name fixture.
126. Add mocked-production-path fixture.
127. Add skipped-test-claim fixture.
128. Add builder-log-conflict fixture.
129. Add stale-HEAD fixture.
130. Add malformed-model-response fixture.
131. Add remediation/re-audit fixture.
132. Prove deterministic audit outcome from same bounded evidence.

## Full verification

133. Run focused M16 Rust tests.
134. Run focused M16 frontend tests.
135. Run migration tests.
136. Run full serialized Rust regression.
137. Run full frontend regression.
138. Run TypeScript typecheck.
139. Run frontend production build.
140. Run npm high-severity audit.
141. Run Rust fmt check.
142. Run Rust all-targets check.
143. Run Rust pty-support check.
144. Run `git diff --check`.
145. Run M14E final-response regressions.
146. Run M15 Prompt Engine regressions.
147. Run M15A duplicate/race/replay regressions.
148. Run M15B ACL/provider/task-picker regressions.
149. Run M15C/M15D handoff/result-placement regressions.
150. Run M16 source-containment/security suite.
151. Run M16 prompt-injection/secret suite.
152. Run M16 misleading-test suite.
153. Run M16 remediation/re-audit loop suite.
154. Run publisher rollback harness.
155. Governed-build production Tauri `--no-bundle`.
156. Publish stable `H!veAI.exe`.
157. Verify candidate/stable SHA equality.
158. Verify PE target.
159. Verify shortcut/icon/startup.
160. Verify no visible console popup.

## Native acceptance evidence

161. Open stable native H!veAI if available.
162. Open Audit Center.
163. Run one known-good/controlled audit if provider/config available.
164. Verify verdict UI.
165. Verify coverage/confidence/risk UI.
166. Verify finding display on controlled bad fixture if safely feasible.
167. Verify UNVERIFIED evidence is visually explicit.
168. Verify Create remediation prompt lands in Prompt Engine without auto-dispatch.
169. Verify audit history/re-audit chain UI.
170. If GPT provider is unavailable, record exact truthful native limitation instead of fabricating success.

## Evidence / commit

171. Create immutable M16 implementation log.
172. Record all migration/schema versions.
173. Record exact model/provider configuration behavior.
174. Record all constants/bounds.
175. Record test counts.
176. Record fixture outcomes.
177. Record publication SHA.
178. Record native acceptance evidence/limitations.
179. Commit scoped files only.
180. Push normally, no force.
181. Verify local HEAD equals `origin/H!veAI`.
182. Verify historical logs/audits were not rewritten.
183. Mark M16 implementation complete pending independent audit.
184. Do not mark M16 PASS/CLOSED.
185. Do not activate M17.
186. Do not start M21.

---

# 14. Required M16 builder log

Create exactly:

`H!veAI/docs/H!veAI/codex-logs/M16_GPT_AUDIT_ENGINE_IMPLEMENTATION_LOG.md`

The log must include:

## Repository/provenance
- starting HEAD;
- implementation commit SHA;
- final pushed HEAD;
- local/origin equality;
- changed files.

## Architecture
- final AuditModel/provider architecture;
- source/evidence collector architecture;
- exact evidence verification classifications;
- audit state machine;
- re-audit chain model.

## Persistence
- migration version(s);
- tables/columns/indexes;
- immutable history behavior;
- failure/rollback behavior.

## Bounds
List every relevant byte/item/time bound.

## Security
- containment;
- secret exclusion/redaction;
- prompt-injection separation;
- builder-log claim handling;
- unrelated network/advertising isolation.

## Tests
Exact counts and commands for:

- M16 focused;
- known-good;
- known-bad;
- misleading tests;
- remediation/re-audit loop;
- security;
- migration;
- full Rust;
- full frontend;
- publisher rollback.

## Native evidence
- actual native Audit Center evidence if feasible;
- otherwise exact reason it could not be completed.

## Publication
- candidate SHA;
- stable SHA;
- size;
- shortcut/icon/startup/no-console evidence.

## Final milestone state

End exactly with truthful state equivalent to:

`M16 IMPLEMENTATION COMPLETE / PENDING INDEPENDENT STRICT AUDIT + USER NATIVE/VISUAL ACCEPTANCE.`

`M17 NOT ACTIVATED.`

`M21 NOT STARTED.`

---

# 15. Completion rule

Complete **all M16.01-M16.08 implementation work in this one run**.

Do not stop between subpackages.

Do not ask for a new prompt after M16.01, M16.02, etc.

Only stop early if a genuine blocker makes the remaining milestone impossible to implement safely. If that occurs:

- record the exact blocker;
- do not fabricate completion;
- complete every unaffected M16 task that can still be safely completed;
- write the log with exact pending gates.

Otherwise continue through M16.08, full regression, governed publication, immutable log, commit, and push in one uninterrupted milestone execution.
