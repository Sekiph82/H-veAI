# H!veAI v1.0.0

H!veAI is a Windows-first local AI Development Command Center for registered projects, authoritative task tracking, GitHub-backed observation, bounded Portfolio and Selected Project AI Assistant chat, Prompt Engine sessions and audits, backup/restore, and human-confirmed actions.

## Highlights

- Native Windows desktop application with Command Center, Projects, Tasks, Project Cockpit, Prompt Builder, Sessions, and Audit.
- Explicit Portfolio and Selected Project chat scopes with bounded factual evidence and truthful provider-unavailable behavior.
- Local SQLite persistence, backup integrity/schema validation, safety backup before restore, and restart-safe recovery.
- GitHub tracking with explicit freshness/offline states and no silent promotion of stale evidence.
- Windows installer with Start Menu and uninstall integration.

## Install and upgrade

Use `H.veAI_1.0.0_x64-setup.exe` from the [v1.0.0 GitHub release](https://github.com/Sekiph82/H-veAI/releases/tag/v1.0.0) on Windows 10/11 x64 with WebView2. Node.js, Rust, npm, Cargo, Vite, and a source checkout are not required on the target machine. Make a verified backup before upgrading. The installer and executable are unsigned.

## Requirements and limitations

Windows 10/11 x64, WebView2, and filesystem permissions for selected project roots are required. Provider-backed chat additionally requires an eligible authenticated supported provider installation. GitHub observations require the configured network and authentication path. Clean-machine install, restart, and uninstall have not been independently verified: `CLEAN_MACHINE_UNVERIFIED — owner risk accepted for v1.0.0 publication`.

H!veAI-owned source is proprietary and is not included in this public distribution repository. Official unmodified binaries are available for no-fee personal, professional, business/commercial, and internal organizational use under [EULA.txt](EULA.txt). See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

See [User Guide](USER_GUIDE.md), [Security and Privacy](SECURITY_PRIVACY.md), [Backup and Restore](BACKUP_RESTORE.md), and [Troubleshooting](TROUBLESHOOTING.md).
