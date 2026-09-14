# Pre-M18 Native Audit Provider Route-Truth Hotfix X03 — Owner-Native Audit

## 1. Scope

Owner-native regression report after M17 closure, before M18 implementation.

Scope is limited to the accepted M16T Codex-only audit-provider UX/truth boundary:

- Settings -> Codex Audit Provider readiness state across route navigation;
- Audit Center presentation of immutable historical audit records versus current live provider readiness.

This audit does not reopen the accepted Codex structured-output contract, model-result semantics, M17 Claude adapter, or M18.

## 2. Owner-native evidence

The owner supplied native screenshots showing this sequence:

1. Open Settings.
2. Codex CLI baseline shows `AUTH_UNVERIFIED` until explicit readiness is checked.
3. Press `Check readiness`.
4. Settings correctly shows `codex-cli 0.154.0`, `ChatGPT authenticated`, `READY`.
5. Navigate to Audit Center.
6. Audit Center selects an immutable historical Bulk-Edit audit created on 2026-09-08 whose model status is `UNAVAILABLE` and whose historical provider/model label is `OPENAI_GPT UNCONFIGURED`.
7. That historical record is rendered with a present-tense yellow warning: `Codex CLI audit provider is not configured.`
8. Navigate back to Settings.
9. The previously verified live `READY` state is lost and Settings again shows `AUTH_UNVERIFIED`.

## 3. Source evidence

Current Settings implementation stores readiness only in component-local React state. On every remount it calls `getAuditProviderReadiness()` and replaces the prior checked result. The explicit `checkAuditProviderReadiness()` result is not retained across route unmount/remount.

Current Audit Center automatically selects the first persisted audit for the selected project. For any persisted audit with `modelStatus !== AVAILABLE`, it renders the selected run's model-status warning. For historical `UNAVAILABLE` records, the text is phrased as current provider truth rather than historical run truth.

## 4. Finding F-X03-001 — verified readiness is lost on route remount

Severity: MAJOR

The explicit production-equivalent readiness check can return `READY`, but navigating away from Settings and back remounts `AuditProviderSettings`, discards the checked state, invokes the baseline readiness command again, and presents `AUTH_UNVERIFIED`.

This produces contradictory UX inside one native application process and makes the user repeat an expensive explicit readiness probe merely to restore the status they just verified.

### Required remediation

Introduce one governed current audit-provider readiness truth shared across routes.

Preferred implementation is a backend/process-scoped last-verified readiness cache or another single source of truth that survives React route remounts. A frontend-only global context is acceptable only if it cannot disagree with backend audit execution truth.

Requirements:

- after an explicit successful readiness check, route navigation away and back must continue to show the last verified `READY` result within the same native process;
- non-READY explicit check results must likewise remain truthful rather than being silently replaced by a weaker baseline state;
- native application restart may legitimately reset explicit end-to-end verification and require a fresh check;
- if executable identity/version materially changes, retained readiness must not falsely remain READY;
- do not persist credentials/tokens or inspect Codex auth files;
- do not weaken the production structured-output readiness probe;
- no API-key/OpenAI HTTP fallback.

## 5. Finding F-X03-002 — historical audit warning is phrased as live provider truth

Severity: MAJOR

Audit Center correctly preserves an old 2026-09-08 `UNAVAILABLE` run, but its yellow warning states `Codex CLI audit provider is not configured` in present tense.

The selected audit is immutable historical evidence. It must not be rewritten to AVAILABLE, but the UI must not imply that the historical model status is the current live provider readiness.

### Required remediation

Separate historical run truth from live provider truth in presentation.

At minimum:

- rename or contextualize `Current verdict` so that a selected historical run is clearly identified as the selected/persisted audit verdict;
- historical `UNAVAILABLE`, `MALFORMED`, `USAGE_LIMITED`, etc. warnings must be phrased as conditions of that audit run, not current Settings/provider truth;
- visibly retain the audit timestamp and immutable model status;
- if current readiness is displayed in Audit Center, it must come from the same governed current-readiness source as Settings;
- do not mutate or rewrite historical audit rows.

## 6. Governance finding / owner instruction

The owner explicitly changed tracker governance:

- ChatGPT, after independent audits/owner acceptance, owns `TASKS.md` and `CODEX_ROADMAP.md` status transitions.
- Codex must treat canonical tracker files as read-only and only synchronize local files with GitHub authoritative state.

Authoritative governance:

`docs/H!veAI/governance/TRACKER_TRANSITION_OWNERSHIP_V01.md`

## 7. Regression requirements

The hotfix must preserve all accepted M16T behavior:

- local Codex CLI only;
- owner-managed ChatGPT login;
- representative production structured-output readiness probe;
- freeform/task schema semantics;
- AVAILABLE-only authoritative completion;
- explicit failure classes and bounded diagnostics;
- immutable audit history;
- exact eight-project portfolio;
- `Sekiph82/FormuLab@main`;
- M17 Claude adapter acceptance.

## 8. Required deterministic tests

At minimum:

1. explicit readiness `READY` -> Settings unmount/remount -> still READY in same process;
2. explicit non-READY result -> remount -> same verified status retained;
3. process restart/no verified cache -> baseline AUTH_UNVERIFIED behavior remains truthful;
4. changed executable/version invalidates retained verified readiness or forces fresh verification;
5. historical `UNAVAILABLE` audit remains immutable and is labeled historical/selected-run truth;
6. historical audit warning never claims current provider is unconfigured when current live readiness is READY;
7. fresh audit behavior and M16T readiness/audit regressions stay green.

## 9. Verdict

**CHANGES_REQUIRED**

F-X03-001 and F-X03-002 must close before M18 implementation begins.

M17 remains PASS/CLOSED. The strict completed roadmap count remains 18/20 = 90%. X03 is a non-numbered pre-M18 hotfix and does not change the milestone denominator. M18 is not yet activated.
