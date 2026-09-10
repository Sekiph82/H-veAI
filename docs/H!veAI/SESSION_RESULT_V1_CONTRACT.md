# SESSION_RESULT v1 and Provider-Neutral Result Contract

`SESSION_RESULT.json` is an optional latest-result claim written by an external provider adapter. It is not a replacement for the canonical task ledger, workflow history, audit records, or append-only event history.

The document is JSON with these required fields:

- `schema`: `hiveai-session-result/v1`;
- `projectKey`: the PROJECT.json project key;
- `sessionId`: the durable Agent Session identifier;
- `provider`: an allowlisted provider name such as `CODEX` or `CLAUDE`;
- `state`: a truthful terminal or live state;
- `finalResponse`: the bounded semantic assistant response, when available;
- `capturedAt`: an ISO-8601 timestamp;
- `provenance`: provider, command, and capture metadata without credentials.

Generic stdout, stderr, tool activity, and provider progress are separate bounded evidence. They must not replace `finalResponse`, and truncation must remain explicit. The provider-neutral projection preserves provider identity, session identity, project identity, task identity, lifecycle state, final response, bounded activity, and evidence status. It never grants a provider permission to mutate project truth.

Unknown, unavailable, malformed, or truncated values remain explicit. Sanitization occurs before persistence, protected markers are never persisted, and reload must preserve the meaningful final response when the generic transport cap is reached.
