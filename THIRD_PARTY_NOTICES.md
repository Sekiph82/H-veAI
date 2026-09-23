# H!veAI Third-Party Notices

This file records the directly declared frontend and Rust components used to
build H!veAI. Versions are the resolved versions observed in the current lock
files/package installation. Each component keeps its own license; the H!veAI
proprietary license does not replace those rights. License texts are available
from the referenced upstream repositories or the package source distribution.

## Frontend and build components

| Component | Version | License | Notice/source |
|---|---:|---|---|
| @tauri-apps/api | 2.11.1 | Apache-2.0 OR MIT | https://github.com/tauri-apps/tauri |
| @tauri-apps/plugin-notification | 2.3.3 | MIT OR Apache-2.0 | https://github.com/tauri-apps/plugins-workspace |
| react | 19.2.8 | MIT | https://react.dev/ |
| react-dom | 19.2.8 | MIT | https://react.dev/ |
| react-router-dom | 7.18.2 | MIT | https://github.com/remix-run/react-router |
| framer-motion | 12.23.28 | MIT | https://github.com/motiondivision/motion |
| motion-dom | 12.23.28 | MIT | https://github.com/motiondivision/motion |
| motion-utils | 12.23.28 | MIT | https://github.com/motiondivision/motion |
| lucide-react | 0.468.0 | ISC | https://lucide.dev/ |
| @xterm/xterm | 6.0.0 | MIT | https://github.com/xtermjs/xterm.js |
| @xterm/addon-fit | 0.11.0 | MIT | https://github.com/xtermjs/xterm.js |
| @tauri-apps/cli | 2.11.4 | Apache-2.0 OR MIT | https://github.com/tauri-apps/tauri |
| typescript | 7.0.2 | Apache-2.0 | https://www.typescriptlang.org/ |
| vite | 8.2.2 | MIT | https://vite.dev/ |
| @vitejs/plugin-react | 6.1.0 | MIT | https://github.com/vitejs/vite-plugin-react |

The test-only packages (`@testing-library/jest-dom` 6.8.0, `@testing-library/react`
16.3.0, `@testing-library/user-event` 14.6.1, `@types/react` 19.2.18,
`@types/react-dom` 19.2.5, `jsdom` 26.1.0, and `vitest` 3.2.7) are MIT-licensed
and are development/test dependencies, not runtime application components.

## Rust/Tauri components

| Component | Version | License | Notice/source |
|---|---:|---|---|
| tauri | 2.11.5 | Apache-2.0 OR MIT | https://github.com/tauri-apps/tauri |
| tauri-build | 2.6.3 | Apache-2.0 OR MIT | https://github.com/tauri-apps/tauri |
| tauri-plugin-log | 2.9.0 | Apache-2.0 OR MIT | https://github.com/tauri-apps/plugins-workspace |
| tauri-plugin-notification | 2.3.3 | Apache-2.0 OR MIT | https://github.com/tauri-apps/plugins-workspace |
| log | 0.4.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/log |
| serde | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_json | 1.0.151 | MIT OR Apache-2.0 | https://github.com/serde-rs/json |
| rusqlite | 0.32.1 | MIT | https://github.com/rusqlite/rusqlite |
| uuid | 1.25.0 | Apache-2.0 OR MIT | https://github.com/uuid-rs/uuid |
| notify | 8.2.0 | CC0-1.0 | https://github.com/notify-rs/notify |
| chrono | 0.4.45 | MIT OR Apache-2.0 | https://github.com/chronotope/chrono |
| sha2 | 0.10.9 | MIT OR Apache-2.0 | https://github.com/RustCrypto/hashes |
| tempfile | 3.27.0 | MIT OR Apache-2.0 | https://github.com/Stebalien/tempfile |

`portable-pty` is an optional declared capability dependency and is not enabled
by the default release feature set. Its upstream license must be retained if a
future distribution enables that feature.

## Compatibility review

The directly declared resolved components reviewed above use MIT, Apache-2.0,
ISC, or CC0-1.0 terms. No copyleft dependency was identified in this direct
inventory. Transitive dependencies remain governed by their own package
metadata and notices; `Cargo.lock` and `package-lock.json` are retained as the
resolution records. The Windows WebView2 runtime is an external platform
dependency, not an H!veAI-owned source component; its Microsoft terms apply
when installed on the target machine.
