# H!veAI — Windows AI Development Command Center

H!veAI is a native Windows desktop command center for managing development projects, tasks, AI sessions, audits, GitHub tracking, and evidence-aware actions.

## v1.0.0 download

Download the official Windows release from [GitHub Releases](https://github.com/Sekiph82/H-veAI/releases/tag/v1.0.0). The release includes the installer and standalone production executable.

H!veAI supports Windows 10/11 x64 with a supported WebView2 runtime. The installer and executable are currently unsigned. Clean-machine install, restart, and uninstall validation was not performed: `CLEAN_MACHINE_UNVERIFIED — owner risk accepted for v1.0.0 publication`.

## Features

- Command Center portfolio health, current work, attention, and next actions.
- Projects and Tasks grounded in each tracked repository's authoritative task source.
- Prompt Engine with Prompt Builder, Sessions, and Audit surfaces.
- AI Assistant with Portfolio and Selected Project scopes and bounded evidence.
- GitHub repository, branch, commit, issue, pull-request, release, and workflow tracking.
- Verified backup and restore with integrity checks and a safety backup before restore.
- Native Windows desktop runtime with no Node.js, Rust, npm, Cargo, Vite, or source checkout required after installation.

## Public distribution and license

This repository is a binary-distribution repository. H!veAI-owned source code is proprietary and is not included here. Official unmodified H!veAI binaries may be used without a license fee for personal, professional, business/commercial, and internal organizational use under the [EULA](EULA.txt). H!veAI-owned rights remain reserved under [LICENSE](LICENSE). Third-party rights are described in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

## Verify downloads

Download [SHA256SUMS.txt](SHA256SUMS.txt) from this repository or the release and compare the SHA-256 digest of the downloaded installer or executable with the listed value. On Windows PowerShell:

```powershell
Get-FileHash .\H.veAI_1.0.0_x64-setup.exe -Algorithm SHA256
Get-FileHash .\H.veAI_1.0.0_x64.exe -Algorithm SHA256
```

The release also provides [EULA.txt](EULA.txt), [LICENSE](LICENSE), and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). GitHub normalizes the exclamation mark in uploaded executable filenames to a period; use the actual names shown in the release assets list.

## Documentation

- [User Guide](USER_GUIDE.md)
- [Security and Privacy](SECURITY_PRIVACY.md)
- [Backup and Restore](BACKUP_RESTORE.md)
- [Troubleshooting](TROUBLESHOOTING.md)
- [Release Notes](RELEASE_NOTES.md)
