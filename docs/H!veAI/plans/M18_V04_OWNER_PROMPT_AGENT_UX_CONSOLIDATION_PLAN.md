# M18 V04 Owner-Directed Prompt Engine / Agent Session UX Consolidation Plan

Date: 2026-09-15
Repository: `Sekiph82/H-veAI`
Milestone: M18 remains OPEN

## Purpose

The owner supplied native H!veAI screenshots after the M18 V03 strict audit and requested a bounded UI/runtime cleanup before M18 acceptance. This plan is additive to the five V03 audit findings and does not activate M19.

The requested result is one coherent Prompt Engine workflow instead of two competing top-level workspaces, with provider diagnostics moved to Settings and the observed Prompt Engine nullable-column crash fixed.

## Source observations

Current source confirms the screenshots:

- `src/components/Shell.tsx` exposes separate top-level `Agents` and `Prompt Engine` navigation items.
- `src/App.tsx` exposes separate `/agents` and `/prompts` routes.
- `src/pages.tsx` renders `Agent Session Center` as a standalone page and renders a large `Claude Code readiness` capability card above session operations.
- `src/pages.tsx` Settings currently contains application lifecycle and the distinct `Codex Audit Provider` readiness panel.
- `src/PromptEnginePage.tsx` dispatches an exact prompt version and then links to the standalone Agents route through `View result in Agents`.
- `src-tauri/src/prompt_engine.rs::collect_context()` reads `tasks.required_actor` as a non-null `String`; real task rows may contain SQL NULL, matching the native error `Invalid column type Null at index: 2, name: required_actor`.

## Target information architecture

### 1. One top-level Prompt Engine workspace

Remove `Agents` from the primary sidebar and command-palette navigation. `Prompt Engine` remains the single top-level entry for prompt creation and provider session work.

Inside Prompt Engine provide two clear internal surfaces, tabs, or equivalent segmented navigation:

- `Prompt Builder`
- `Sessions`

The existing M14 Agent Session Center functionality moves into the `Sessions` surface without losing lifecycle behavior.

### 2. Preserve backward-compatible session deep links

Do not break accepted M15C/M15D provenance or old links.

- Existing `/agents?...` links must remain safe and resolve/redirect into the integrated Prompt Engine Sessions surface while preserving validated `projectId` and `sessionId`.
- New post-dispatch links must target the integrated Prompt Engine Sessions surface directly.
- Opening a dispatched result must select the exact persisted session without relaunching or redispatching a provider.
- Invalid, cross-project, missing, or stale session targets must still fail closed.

### 3. Provider readiness belongs in Settings

Move the large Claude readiness/capability presentation out of the session-operation surface.

Settings should gain a dedicated builder-provider diagnostics section that truthfully shows both `CODEX` and `CLAUDE` readiness/capabilities using the existing provider-neutral readiness command. Keep this conceptually separate from the existing `Codex Audit Provider` panel, because builder/session readiness and audit-provider readiness are different contracts.

The Sessions surface may still use readiness internally to enable/disable Start and may show a concise unavailable diagnostic or Settings link, but must not repeat the large readiness capability card/chip wall.

### 4. Fix nullable Prompt Engine task context

`required_actor` is nullable in real task data. Prompt context collection must treat it as optional rather than failing SQLite row decoding.

At minimum:

- read `required_actor` as `Option<String>`;
- preserve `null`/unavailable truth in the context manifest rather than inventing an actor;
- verify nullable milestone/other already-optional task fields remain safe;
- add a direct Rust regression that inserts/selects a real task with `required_actor = NULL` and proves context collection and prompt generation do not raise `Invalid column type Null`.

### 5. Preserve accepted M14/M15 behavior

The UI consolidation is not a rewrite of the provider/session backend. Preserve:

- project confinement;
- exact task/session identity;
- start/stop/retry/resume semantics;
- Claude exact-resume contract;
- PTY/output/final-response rendering;
- polling and persisted-session selection;
- permission/wait diagnostics;
- Git/diff evidence;
- immutable prompt-version dispatch provenance;
- no automatic dispatch;
- no provider relaunch merely because a session is opened.

## Required focused evidence

Add deterministic tests proving:

1. primary navigation contains `Prompt Engine` but no separate `Agents` item;
2. Prompt Engine exposes both Prompt Builder and Sessions surfaces;
3. a legacy `/agents?projectId=...&sessionId=...` target reaches the exact integrated session safely;
4. new post-dispatch handoff targets the integrated Sessions surface and never redispatches;
5. wrong-project / missing session targets still fail closed;
6. Settings renders builder-provider readiness for Codex and Claude separately from the Codex Audit Provider panel;
7. Agent Sessions no longer renders the large Claude readiness capability panel;
8. provider readiness still gates Start truthfully;
9. a task with `required_actor = NULL` can be selected, context-refreshed, and used to generate a prompt without the observed native error;
10. existing M14/M15 focused regressions remain green.

## Visual acceptance target

After independent source re-audit passes, owner-native acceptance should verify:

- only one top-level `Prompt Engine` navigation entry exists for prompt/session work;
- Prompt Builder and Sessions are easy to switch between;
- provider readiness diagnostics live under Settings;
- the Sessions surface begins with session operations/current conversation rather than the large readiness card;
- Prompt Engine no longer displays `Invalid column type Null at index: 2, name: required_actor` for tasks with no required actor;
- no visual regression to the accepted H!veAI shell, background, topbar, session reader, or Prompt Engine vertical workflow.

## Scope boundary

This is a bounded owner-directed M18 V04 addendum. It does not reopen M14 or M15, does not alter X04 TASKS-only authority, does not authorize destructive Git/GitHub mutation, and does not activate M19.