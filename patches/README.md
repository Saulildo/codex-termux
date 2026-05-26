# Termux Patch Inventory

This fork tracks upstream OpenAI Codex and keeps only the compatibility delta
required to publish a working Android Termux package.

- Fork repo: `DioNanos/codex-termux`
- Upstream base for this release: `rust-v0.134.0`
- Current fork release target: `v0.134.0`

## Runtime patches

### Patch #1 - Browser login on Android
- File: `codex-rs/login/src/server.rs`
- Uses `termux-open-url` on Android instead of the desktop browser path.

### Patch #2 - Release profile for Termux builds
- File: `codex-rs/Cargo.toml`
- Keeps the Android release profile explicit for reproducible maintainer builds.

### Patch #4 - Update source points to fork releases
- File: `codex-rs/tui/src/updates.rs`
- Update checks point to `DioNanos/codex-termux` releases instead of `openai/codex`.

### Patch #5 - Version parser accepts Termux tag shapes
- Files: `codex-rs/tui/src/updates.rs`, `codex-rs/tui/src/update_versions.rs`
- Release parser strips both `rust-v` (upstream) and `v` (Termux) tag prefixes,
  and splits on `-` so suffixes like `-termux` collapse to a clean semver
  triple for comparison.

### Patch #6 - Correct package name for self-update
- File: `codex-rs/tui/src/update_action.rs`
- Uses `@mmmbuto/codex-cli-termux@latest` for npm/bun/Unix arms.

### Patch #6b - Fork identity across UI/doctor/npm surfaces
- Files: `codex-rs/cli/src/doctor/updates.rs`,
  `codex-rs/cli/src/doctor.rs`,
  `codex-rs/tui/src/npm_registry.rs`,
  `codex-rs/tui/src/update_prompt.rs`,
  `codex-rs/tui/src/history_cell/notices.rs`
- Replaces every upstream-identity reference on user-visible UI/doctor/registry
  surfaces with the fork identity. Covers: `GITHUB_LATEST_RELEASE_URL` →
  `DioNanos/codex-termux/releases/latest`, npm registry URL →
  `@mmmbuto/codex-cli-termux`, doctor labels and path joins →
  `@mmmbuto/codex-cli-termux`, update prompt and notice cells →
  `DioNanos/codex-termux` release URLs.

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
- Keeps voice and realtime audio disabled in published Termux builds through
  target-OS cfg gates, without workspace crate features.

### Patch #12 - Dynamic npm wrapper routing
- File: `npm-package/bin/codex.js`
- Detects root subcommands from `codex --help` and avoids misrouting valid
  commands to `codex exec`.

### Patch #13 - Fork-safe managed updates
- Files: `codex-rs/tui/src/update_action.rs`, `codex-rs/app-server-daemon/*`
- Keeps update commands on `@mmmbuto/codex-cli-termux@latest`, disables daemon
  auto-update fetches, neutralises `install_latest_standalone()` to `Ok(())`,
  and blocks the upstream installer URL from reappearing in the daemon's user
  guidance message.

### Patch #14 - Fork-owned public install surfaces
- Files: `scripts/install/*`, `scripts/stage_npm_packages.py`, `codex-rs/README.md`
- Keeps public install and release-staging guidance on `DioNanos/codex-termux`
  and `@mmmbuto/codex-cli-termux`.

### Patch #15 - Fork-owned feedback surfaces
- Files: `.github/ISSUE_TEMPLATE/*`, `.github/pull_request_template.md`, `codex-rs/tui/src/bottom_pane/feedback_view.rs`, `codex-rs/tui/src/tooltips.rs`
- Keeps public feedback, issue, contribution, and announcement-tip links on
  `DioNanos/codex-termux`.

### Patch #16 - Android remote-control daemon support
- Files: `codex-rs/app-server-daemon/src/managed_install.rs`, `codex-rs/app-server-daemon/src/backend/pid.rs`, `codex-rs/cli/src/remote_control_cmd.rs`
- Enables `codex remote-control` daemon mode (`start`/`stop`) on Android/Termux.
  Three sub-fixes, all gated on `#[cfg(target_os = "android")]`:
  1. **`managed_codex_bin`** (`managed_install.rs`): on Android, resolves the daemon
     binary via `CODEX_SELF_EXE` (set by the npm launcher, Patch #10) instead of
     the standalone installer path `~/.codex/packages/standalone/current/codex`
     which does not exist on npm-based Termux installs. The ELF resolves
     `libc++_shared.so` via `RUNPATH=$ORIGIN` (Patch #10b).
  2. **`read_process_start_time`** (`pid.rs`): on Android, reads process start time
     from `/proc/<pid>/stat` field 22 (starttime in jiffies since boot) instead of
     `ps -o lstart=`, which is not available in Android toybox.
  3. **Foreground socket dir** (`remote_control_cmd.rs`): uses `std::env::temp_dir()`
     (honours `$TMPDIR`) instead of hardcoding `/tmp`, which does not exist on
     stock Android. Applied unconditionally; correct on all Unix platforms.

### Patch #17 - flock ENOTSUP/EOPNOTSUPP tolerance for Termux storage
- Files: `codex-rs/app-server-daemon/src/backend/pid.rs`, `codex-rs/app-server-daemon/src/lib.rs`
- Some Android/Termux storage backends rooted at `/data/data/com.termux/...`
  reject `flock(2)` with `ENOTSUP` / `EOPNOTSUPP` instead of acquiring or
  refusing the lock. Both `try_lock_file` helpers — the daemon operation lock
  and the pid reservation lock — match the same permissive degradation already
  used elsewhere (see Patch #18 on `installation_id.rs`) and treat the
  unsupported class as "lock acquired" so `codex remote-control` proceeds.
  Linux ext4/btrfs/xfs continue to enforce `flock` unchanged; Windows is
  unaffected (`#[cfg(not(unix))]` paths untouched).

### Patch #18 - Android runtime compatibility shims
- Files: `codex-rs/arg0/src/lib.rs`, `codex-rs/core/src/installation_id.rs`, `codex-rs/utils/pty/src/pty.rs`
- Three Android-specific runtime shims, all gated on `#[cfg(target_os = "android")]`:
  1. **`arg0/src/lib.rs`**: `CODEX_SELF_EXE` resolution so subprocess re-exec
     flows pick up the npm-launcher-provided real binary path (paired with
     Patch #10), plus permissive degradation when `try_lock` on the codex
     aliases lock file returns `ErrorKind::Unsupported`.
  2. **`core/src/installation_id.rs`**: tolerates `ErrorKind::Unsupported` on
     the installation-id lockfile so first-run on Termux storage does not
     abort installation-id bootstrap. This is the original source of the
     `is_unsupported_file_lock_error` pattern reused by Patches #17 and the
     `arg0` shim above.
  3. **`utils/pty/src/pty.rs`**: provides an `openpty` C symbol on Android,
     since Bionic does not export it. The fork implementation uses
     `posix_openpt` + `grantpt` + `unlockpt` + `ptsname_r` + `open` to
     produce master/slave fds compatible with the upstream pty handling.

### Patch #19 - Android UI cfg gates
- Files: `codex-rs/tui/src/clipboard_paste.rs`, `codex-rs/tui/src/app_event.rs`
- Adds `#[cfg(not(target_os = "android"))]` around clipboard paste paths that
  depend on platform clipboard primitives unavailable on Termux, and extends
  the existing dead-code allow attribute on `app_event` variants to include
  `target_os = "android"` so the no-voice (Patch #11) and no-clipboard build
  configurations link cleanly.

### Patch #20 - Android code-mode stubs
- Files: `codex-rs/code-mode/src/lib.rs`, `codex-rs/code-mode/src/runtime_stub.rs`, `codex-rs/code-mode/src/service_stub.rs`, `codex-rs/code-mode/Cargo.toml`
- The upstream `code-mode` runtime/service modules pull dependencies that do
  not cross-build to `aarch64-linux-android`. On Android the build switches
  to fork-owned `runtime_stub.rs` and `service_stub.rs` which expose the same
  public surface (`CodeModeNestedToolCall`, service entry points) with
  no-op implementations. `code-mode/Cargo.toml` gates the heavy deps on
  `cfg(not(target_os = "android"))`. `code-mode/src/lib.rs` routes
  `mod runtime` vs `mod runtime_stub` (and `service` vs `service_stub`)
  through the same cfg.

### Patch #21 - Android cross-build vendored OpenSSL
- File: `codex-rs/core/Cargo.toml`
- Adds a `[target.aarch64-linux-android.dependencies]` block that pulls
  `openssl-sys` with the `vendored` feature, so Android cross-builds compile
  OpenSSL from source instead of looking for system libssl. Non-Android
  targets are unaffected.

### Patch #22 - V8 Android prebuilt infrastructure
- Files: `scripts/fetch_rusty_v8_android.py`, `scripts/prepare_rusty_v8_android_source.py`, `third_party/v8/android-artifacts.toml`
- `rusty_v8` does not provide official Android arm64 binary releases. The
  fork ships its own prebuilt path: `fetch_rusty_v8_android.py` reads the
  required `rusty_v8` version from `Cargo.lock`, looks it up in
  `android-artifacts.toml`, downloads the matching prebuilt static library
  and binding from a fork-owned GitHub release, verifies SHA256, and exports
  `RUSTY_V8_ARCHIVE` + `RUSTY_V8_SRC_BINDING_PATH` so Cargo skips compiling
  V8 from source. `prepare_rusty_v8_android_source.py` is the maintainer-side
  companion used to produce a new prebuilt when upstream bumps the V8 pin.

### Patch #23 - Fork-owned workflows and CI guards
- Files: `.github/workflows/ci.yml`, `.github/workflows/termux-npm-build-publish.yml`, `.forgejo/workflows/termux-next-smoke.yml`
- Upstream `ci.yml` stages an npm package and uploads it as an artifact;
  these steps are gated with
  `if: ${{ github.repository == 'openai/codex' }}`
  so a fork clone of CI never publishes upstream-flavoured artifacts.
  `termux-npm-build-publish.yml` is the fork's release pipeline:
  workflow_dispatch, builds the Android arm64 binaries with the V8 prebuilt
  flow (Patch #22), assembles the npm package, publishes
  `@mmmbuto/codex-cli-termux` to npm, and (optionally) cuts a GitHub release.
  `.forgejo/workflows/termux-next-smoke.yml` is the Forge mirror used for
  develop-side smoke tests.

## Verification

Run from repo root:

```bash
bash verify-patches.sh
```
