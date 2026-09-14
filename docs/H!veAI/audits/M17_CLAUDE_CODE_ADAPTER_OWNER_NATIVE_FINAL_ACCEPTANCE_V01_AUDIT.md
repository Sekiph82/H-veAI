# M17 Claude Code Adapter — Owner Native Final Acceptance V01

## 1. VERDICT

PASS.

M17 is accepted for closure. Independent V05 source re-audit passed, and the owner supplied native visual evidence from the governed H!veAI desktop build and explicitly accepted the remaining provider-quota-limited native flow.

## 2. CONTRACT RECOVERY

M17 required a first-class local Claude Code CLI adapter using the owner's Claude-managed login, truthful readiness, bounded structured streaming, project/task/cwd containment, persisted provider-native session identity, explicit attention/error states, owned-process Stop, exact Resume, crash/orphan recovery, truthful action projection, and preservation of all accepted M00-M16 behavior.

No `ANTHROPIC_API_KEY`, direct Anthropic HTTP/API transport, credential-file inspection, GUI/browser automation, automatic Claude installation/update, or blanket permission bypass was allowed.

## 3. BRANCH / HEAD / DIFF SCOPE

Repository: `Sekiph82/H-veAI`

Branch: `main`

Accepted implementation: M17 V01 through V05 remediation chain, ending with V05 implementation commit `e28d8f4db643056526ea73954940253d77bda6e5` and V05 builder-log commit `d1f20db781c0f39915c587b7b55fe64ba99ae7c7`.

Independent V05 strict re-audit commit: `75443528fdcf80a830863d2ae7246ed104f5ac52`.

## 4. ACCEPTANCE CRITERIA MATRIX

- Local Claude Code CLI readiness: PASS.
- Owner-managed Claude authentication: PASS.
- Installed Claude version/capabilities surfaced truthfully: PASS.
- Project-scoped native Start reaches Claude provider: PASS.
- Provider quota/usage-limit failure is surfaced truthfully: PASS.
- Provider-native session identity/provenance persisted sufficiently for exact Resume eligibility: PASS by source/test evidence and native Resume control visibility.
- Owned Stop lifecycle: PASS by independent source/test evidence; native live Stop exercise was blocked by provider weekly quota.
- Exact Resume lifecycle: PASS by independent source/test evidence; full native Resume execution was blocked by provider weekly quota.
- Crash/orphan/action-truth/diagnostic truth: PASS by V05 strict source audit.
- Security boundaries: PASS.
- M00-M16 regressions: PASS at source audit level; builder command counts remain builder claims where no independent CI status exists.

## 5. BUILDER CLAIMS VS REPOSITORY TRUTH

The V05 builder log claimed closure of stale diagnostic truth, direct mutation action-truth projection, production Stop escalation coverage, and dual-budget drain/finalization coverage. The independent V05 strict re-audit verified those implementation paths and tests in repository source and found no blocking source defect.

Native evidence then confirmed that the published application detects Claude readiness and reaches the provider in a real session.

## 6. FILE / SYMBOL EVIDENCE

Independent V05 source audit verified the production Claude adapter in `src-tauri/src/agent_session_center.rs`, including readiness/auth classification, current-session ownership projection, exact-resume eligibility, current diagnostic transitions, stop/retry behavior, structured stream finalization, and restart reconciliation.

## 7. FOCUSED TEST EVIDENCE

Repository tests verified, among other cases, current diagnostic clearing on resume/reconcile, direct mutation action truth, production stop escalation failure/retry/completion, dual-budget stream draining/finalization, resume setup cleanup, provider-session identity immutability, attention-state terminalization, and restart recovery.

Builder-reported aggregate counts remain claims rather than independent CI evidence.

## 8. REGRESSION EVIDENCE

The accepted M17 chain preserved the Codex provider, M16 Codex-only audit provider, exact eight-project portfolio, and `Sekiph82/FormuLab@main` tracking. No native evidence supplied for this final acceptance contradicts those accepted regressions.

## 9. SECURITY / SAFETY REVIEW

PASS.

Claude integration remains local-CLI-only and owner-login-managed. No direct Anthropic API transport, API-key fallback, auth-file inspection, GUI/browser automation, automatic install/update, shell-string prompt transport, unrelated-process termination, or blanket permission bypass is accepted.

## 10. ARCHITECTURE CONSISTENCY

PASS.

M17 now implements Claude as an owned local engineering-agent provider while preserving provider-neutral session/provenance concepts and the accepted Codex/audit architecture.

## 11. TRACKER / LOG / DOCUMENTATION TRUTHFULNESS

At the time of this acceptance record, root `TASKS.md` still reflects the pre-acceptance M17 V05 state. The next M18 builder prompt must perform the canonical tracker transition as its first governed action: M17 PASS/CLOSED, strict progress 18/20 = 90%, M18 ACTIVE, M19-M20 planned/blocked.

Historical M17 prompts, logs, and failed strict audits remain immutable evidence.

## 12. FINAL REPOSITORY STATE

The independent V05 strict audit was already committed on live GitHub `main`. This acceptance artifact is an additional owner-acceptance record and does not mutate application source.

## 13. OPEN CROSS-MILESTONE FINDINGS

No blocking M17 source finding remains.

The only incomplete manual subflow is a full live Start -> Stop -> exact Resume round-trip under available Claude quota. This is not treated as a code defect because the real provider rejected execution due the owner's weekly usage limit, the application surfaced that external condition truthfully, deterministic source-level lifecycle evidence is accepted, and the owner explicitly waived the quota-blocked manual step and approved closure.

## 14. DEFECTS BY SEVERITY

BLOCKER: 0.

MAJOR: 0.

MINOR: 0.

NOTE: Full live Stop/Resume round-trip could not be performed because Claude reported the owner's weekly usage limit.

## 15. TECHNICAL DEBT / UPGRADE OPPORTUNITIES

When Claude quota is naturally available again, a non-blocking smoke exercise of Start -> Stop -> exact Resume may be performed for additional operational confidence. It is not required to reopen M17 unless new contradictory evidence appears.

## 16. UNVERIFIED ITEMS

The live Stop and resumed-live execution portions were not manually exercised in this acceptance session because the provider returned a weekly-limit failure before a long-running session could exist.

These behaviors remain independently verified at source/deterministic-test level rather than current-quota native level.

## 17. REGRESSION RISK

LOW to MEDIUM.

The lifecycle surface is process- and provider-sensitive, but five remediation rounds plus independent V05 source audit materially reduce known risk. The remaining manual gap is caused by external provider quota rather than an observed H!veAI defect.

## 18. AUDIT CONFIDENCE

HIGH for source architecture and truthful provider-error handling.

MEDIUM-HIGH for the complete real-provider lifecycle because provider quota prevented a fresh full Stop/Resume round-trip in this final acceptance session.

## 19. FINAL VERDICT

PASS / M17 ACCEPTED FOR CLOSURE.

Native owner evidence shows Claude Code readiness as `READY`, Claude Code version `2.1.270`, authenticated readiness/capabilities including Resume, and a real project-scoped Claude session reaching the provider. The provider returned a weekly usage-limit message; H!veAI persisted the session as FAILED rather than fabricating success and exposed `Resume exact session`. The owner explicitly reviewed the UI, accepted the behavior, and authorized M17 closure despite the external quota limitation.

M18 may now activate.

## 20. REQUIRED REMEDIATION

None for M17 closure.

The next governed action is the M18 tracker transition and M18 GitHub Integration implementation. Do not reopen M17 unless new contradictory runtime or source evidence appears.
