# H!veAI Security and Privacy

H!veAI is local-first. The native desktop process owns filesystem, process, SQLite, Git, GitHub, backup, and provider-boundary operations; the frontend does not receive an unrestricted shell.

## Local data and provider boundaries

SQLite stores registry metadata, bounded task and source observations, sessions, provider events, audit records, action provenance, and backup metadata under the application-data directory. Assistant evidence is a typed bounded packet, recursively sanitized before provider invocation and persistence. It contains factual project or portfolio fields and provenance, not complete repositories or arbitrary files. Authorization headers, API keys, passwords, cookies, token prefixes, and secret-like values are redacted. Do not paste secrets into chat.

Provider authentication is owned by the provider's authenticated runtime boundary. H!veAI does not add a plaintext API-key field. Provider failure, timeout, stop failure, missing authentication, or missing evidence fails closed; no fake assistant response is generated.

## Permissions and network

Application capabilities allow only the commands required by the product. Provider processes receive bounded input and do not receive registered-project source working directories. GitHub access is limited to configured tracking and observation flows and may be stale or unavailable. Local logs are bounded and redacted; diagnostics must be scrubbed before sharing.

## Backups and dependencies

Backups contain supported SQLite state and integrity metadata, not source repositories, provider credentials, or arbitrary secrets. Restore validates schema and integrity and creates a safety backup first. WebView2, authenticated provider installation, GitHub availability, filesystem ACLs, and Windows installer services are external dependencies. See [Backup and Restore](BACKUP_RESTORE.md) and [Troubleshooting](TROUBLESHOOTING.md).

H!veAI is not open source. H!veAI-owned source is reserved under [LICENSE](LICENSE), while official unmodified binaries are available for no-fee personal, professional, business/commercial, and internal organizational use under [EULA.txt](EULA.txt). Third-party rights remain separate; see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
