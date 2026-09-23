# H!veAI Troubleshooting

- **Provider unavailable or login required:** confirm the supported provider installation and authentication. H!veAI states unavailable instead of fabricating an answer. A timeout or stop failure is fail-closed.
- **GitHub offline or stale:** check network/authentication and the configured repository. Treat cached evidence as stale until a successful refresh proves freshness.
- **Registered project missing:** open Settings, verify the path exists and is readable, check ACLs, and confirm the project is not archived. Re-register only the intended root.
- **ACL or permission issue:** grant the Windows account the minimum required access for the requested operation; do not run the app elevated as a first workaround.
- **Installer or startup issue:** confirm Windows and WebView2 prerequisites, inspect the installed shortcut target, and retry after a restart. No Node.js, Rust, or development server is required for an installed build.
- **Backup/restore error:** do not delete the live database. Keep the verified and safety backups, inspect the reported integrity or schema error, and retry only with a compatible backup.
- **No current task:** this is a truthful source state, not a generated task. Refresh the source or correct the governed project tracker.

Application logs are under the H!veAI application-data log directory. Before sharing diagnostics, remove secrets, tokens, cookies, API keys, personal paths, repository contents, and unrelated user data. Include app version, failure class, timestamp, and only the redacted relevant log lines.
