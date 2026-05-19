# Termux Patch Inventory

This fork tracks upstream OpenAI Codex and keeps only the compatibility delta
required to publish a working Android Termux package.

- Fork repo: `DioNanos/codex-termux`
- Upstream base for this release: `rust-v0.131.0`
- Current fork release target: `v0.131.0`

## Runtime patches

### Patch #1 - Browser login on Android
- File: `codex-rs/login/src/server.rs`
- Uses `termux-open-url` on Android instead of the desktop browser path.

### Patch #2 - Release profile for Termux builds
- File: `codex-rs/Cargo.toml`
- Keeps the Android release profile explicit for reproducible maintainer builds.

### Patch #4 - Update source points to fork releases
- File: `codex-rs/tui/src/updates.rs`
- Update checks point to `DioNanos/codex-termux` releases.

### Patch #5 - Version parser accepts `-termux`
- File: `codex-rs/tui/src/updates.rs`
- Release parsing strips the `-termux` suffix for semantic comparison.

### Patch #6 - Correct package name for self-update
- File: `codex-rs/tui/src/update_action.rs`
- Uses `@mmmbuto/codex-cli-termux@latest`.

### Patch #10 - Launcher hardening
- Files: `npm-package/bin/codex`, `npm-package/bin/codex-exec`, `npm-package/bin/*.js`
- Packaged launchers preserve `LD_LIBRARY_PATH` and `CODEX_SELF_EXE` so direct
  helper re-exec flows keep bundled `libc++_shared.so` reachable.

### Patch #10b - Android ELF runpath hardening
- File: `codex-rs/.cargo/config.toml`
- Adds `-Wl,-rpath,$ORIGIN` so packaged Android ELFs can resolve sibling
  `libc++_shared.so` even without wrapper-provided `LD_LIBRARY_PATH`.

### Patch #11 - Android no-voice policy
- Files: `codex-rs/tui/Cargo.toml`, `codex-rs/tui/src/*`, `codex-rs/cli/Cargo.toml`, `codex-rs/cloud-tasks/Cargo.toml`
- Keeps voice and realtime audio disabled in published Termux builds.

### Patch #12 - Dynamic npm wrapper routing
- File: `npm-package/bin/codex.js`
- Detects root subcommands from `codex --help` and avoids misrouting valid
  commands to `codex exec`.

### Patch #13 - Fork-safe managed updates
- Files: `codex-rs/tui/src/update_action.rs`, `codex-rs/app-server-daemon/*`
- Keeps update commands on `@mmmbuto/codex-cli-termux@latest`, disables daemon
  auto-update fetches, and blocks the upstream installer URL from reappearing.

## Verification

Run from repo root:

```bash
bash verify-patches.sh
```
