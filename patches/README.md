# Termux Patch Inventory (vs Upstream)

This file tracks fork-specific changes against upstream OpenAI Codex.

- Fork repo: `DioNanos/codex-termux`
- Upstream repo: `openai/codex`
- Baseline used for this inventory: `rust-v0.118.0`
- Current fork release: `v0.118.0-termux`
- Last update: 2026-04-01

Scope note:
- This inventory is Termux-fork only.

## 1) Runtime patches (Termux fork behavior)

These are the practical fork deltas most relevant for end users.

### Patch #1 - Browser login on Android/Termux
- File: `codex-rs/login/src/server.rs`
- Change: on Android target, use `termux-open-url` instead of desktop browser path.
- Goal: avoid Android/Termux browser login crash path.

### Patch #2 - Release profile for constrained devices
- File: `codex-rs/Cargo.toml`
- Change: release profile tuned for Termux build constraints (`lto = false`, higher codegen units).
- Goal: improve build reliability on mobile hardware.

### Patch #4 - Update source points to fork releases
- File: `codex-rs/tui/src/updates.rs`
- Change: update-check endpoint references `DioNanos/codex-termux` releases.
- Goal: update notifications target fork releases, not upstream-only tags.

### Patch #5 - Version parser compatibility (`-termux` suffix)
- File: `codex-rs/tui/src/updates.rs`
- Change: parser accepts termux-suffixed versions and strips suffix for compare.
- Goal: avoid false negatives in update detection.

### Patch #6 - Correct npm package name for auto-update
- File: `codex-rs/tui/src/update_action.rs`
- Change: update command uses `@mmmbuto/codex-cli-termux`.
- Goal: avoid accidental install path to upstream package.

### Patch #9 - Execute update action path
- File: `codex-rs/cli/src/main.rs`
- Change: returned update action is executed by CLI flow.
- Goal: ensure accepted update request actually runs.

### Patch #10 - Launcher hardening for direct binary invocation (0.106.2)
- Files:
  - `npm-package/bin/codex`
  - `npm-package/bin/codex-exec`
  - `npm-package/bin/codex.bin`
  - `npm-package/bin/codex-exec.bin`
  - `npm-package/package.json`
- Change:
  - `codex`/`codex-exec` are launcher scripts.
  - real ELF binaries moved to `codex.bin` / `codex-exec.bin`.
  - launcher exports safe `LD_LIBRARY_PATH` before `exec`.
- Goal: fix failures like:
  - `CANNOT LINK EXECUTABLE ... libc++_shared.so not found`
  when tools invoke binaries directly without Node wrapper env.

### Patch #10a - Wrapped self-exe propagation for helper aliases (0.117.0)
- Files:
  - `npm-package/bin/codex`
  - `npm-package/bin/codex-exec`
  - `npm-package/bin/codex.js`
  - `npm-package/bin/codex-exec.js`
  - `codex-rs/arg0/src/lib.rs`
- Change:
  - launchers export `CODEX_SELF_EXE` to the wrapped entrypoint path.
  - `arg0` prefers that override instead of `current_exe()` when recording `codex_self_exe` and when materializing helper aliases like `apply_patch`.
- Goal: ensure helper re-exec paths go back through the wrapper so bundled `libc++_shared.so` remains reachable on Termux.

### Patch #11 - Disable voice/realtime audio in published Termux builds (0.111.0)
- Files:
  - `codex-rs/cli/Cargo.toml`
  - `codex-rs/cloud-tasks/Cargo.toml`
- Change:
  - Android consumers of `codex-tui` use `default-features = false`.
  - This keeps `voice-input`, `cpal`, `oboe`, and `oboe-sys` out of the published Termux path.
- Goal: avoid runtime linker failures like:
  - `CANNOT LINK EXECUTABLE ... cannot find "libOpenSLES.so"`
  on Termux devices that do not expose that Android audio dependency to the packaged ELF.

### Patch #13 - Android network-proxy stub refresh for upstream 0.118.0
- Files:
  - `codex-rs/network-proxy/src/android_stub.rs`
  - `codex-rs/network-proxy/src/lib.rs`
- Change:
  - Android stub types were refreshed to match upstream `0.118.0` network proxy
    config models (`domains`, `unix_sockets`, permission entries, and helper accessors).
  - Android target keeps exporting the stubbed surface while non-Android targets
    continue to use upstream implementation types directly.
- Goal: keep Android/Termux builds compiling after upstream network proxy schema
  expansion without pretending the desktop proxy implementation is supported there.

### Patch #12 - Dynamic npm wrapper command routing (0.112.0)
- File: `npm-package/bin/codex.js`
- Change:
  - `codex.js` now discovers root subcommands from `codex --help` at runtime.
  - command aliases from help output are recognized.
  - unknown first arg still falls back to `exec` for prompt convenience.
  - if help parsing fails, launcher uses pass-through (safe fallback, no misrouting).
- Goal: avoid routing valid commands (for example `fork` or `debug`) to
  `codex exec`, which caused incorrect behavior in previous npm builds.

## 2) Historical patches

### Patch #7 - Manual update instruction fallback
- Historical for older 0.55.x line.
- Kept for context; no longer primary mechanism.

### Patch #8 - Bash execution workaround
- Historical workaround for older upstream behavior.
- No longer required on current upstream line, kept only as reference.

## 3) Build/toolchain patch files in `patches/`

These are repository patch assets used for Bazel/toolchain/dependency build paths.
They are not all runtime behavior patches.

### Declared in `MODULE.bazel` (active in module graph)
- `toolchains_llvm_bootstrapped_resource_dir.patch`
- `aws-lc-sys_memcmp_check.patch`
- `windows-link.patch`

### Present in `patches/` but currently not declared in `MODULE.bazel`
- `rules_rust.patch`
- `rules_rust_musl.patch`
- `rules_rust_windows_gnu.patch`

These are retained for compatibility/reference and should be reviewed when Bazel/toolchain rules are updated.

## 4) Quick verification

Run from repo root:

```bash
bash verify-patches.sh
```

The script verifies critical runtime patches and checks that patch files declared in `MODULE.bazel` exist.

## 5) Diff workflow against upstream

Recommended audit commands:

```bash
git fetch upstream main
git fetch upstream refs/tags/rust-v0.118.0:refs/tags/rust-v0.118.0
git log --oneline rust-v0.118.0..main
git diff --name-status rust-v0.118.0..main
```

Use this output to decide whether a delta is:
- runtime patch (user-facing behavior),
- packaging patch,
- docs/test evidence,
- or toolchain/build patch.
