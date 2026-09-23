# H!veAI User Guide

## Install and first launch

Run the official installer on Windows 10/11 x64 with WebView2 available. The installer creates the Start Menu entry, desktop shortcut when selected, uninstall entry, and application-data directory. H!veAI is a native desktop app; Node.js, Rust, npm, Cargo, Vite, and a source checkout are not required after installation. The first launch initializes or migrates the local SQLite database.

## Projects and Command Center

Open Settings to register a project root, then select it under Project Shortcuts. Command Center shows portfolio KPIs, Engineering Brief, Active Work Queue, System Status, GitHub freshness, and selected-project truth. A stale or unavailable source is labelled rather than silently presented as current.

## Tasks and Prompt Engine

Project Cockpit is the selected-project workspace for tasks, source health, and bounded actions. Tasks are read from the governed project sources. Prompt Engine contains Prompt Builder, Sessions, and Audit. Sessions show provider and action provenance; Audit shows evidence and findings without changing the authoritative tracker.

## AI Assistant

AI Assistant supports Portfolio and Selected Project scopes. Provider-backed answers receive only the typed, bounded, sanitized evidence packet transmitted for that request. If no authenticated eligible provider is available, or a provider fails, H!veAI reports unavailable or failed truthfully and does not invent an answer.

## Search, Settings, and backup

Use the Search action or Ctrl/Cmd+K to open Command Palette. Settings is the gear beside the H!veAI version in the sidebar. Create a verified backup before restore. Restore validates integrity and schema, creates a safety backup, requires confirmation, and requires restart; keep the safety backup until the restarted state is checked.

## GitHub and troubleshooting signals

GitHub status can be fresh, stale, offline, unavailable, or failed. Treat stale or offline data as bounded evidence, not proof of current remote state. Registered-project access errors may be ACL or path failures and require correcting permissions or selecting a valid root. Before sharing diagnostics, remove tokens, cookies, API keys, personal paths, repository contents, and unrelated user data.

## Uninstall and update

Use Windows Installed Apps or the H!veAI uninstall entry. Uninstall removes installed binaries and shortcuts; application data and backups may require explicit owner cleanup. Install a newer official installer over the existing version after making a verified backup. Do not delete the safety backup until the update is confirmed.

## License

H!veAI-owned source is proprietary and is not open source. Official unmodified binaries may be used without a license fee for personal, professional, business/commercial, and internal organizational use. See [LICENSE](LICENSE), [EULA.txt](EULA.txt), and [third-party notices](THIRD_PARTY_NOTICES.md).
