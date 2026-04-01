# [0.118.0-termux] - 2026-04-01

### Upstream
- OpenAI Codex `rust-v0.118.0` release: https://github.com/openai/codex/releases/tag/rust-v0.118.0
- Rebased the latest Termux line onto `rust-v0.118.0`.

### Termux Patches
- Kept Android browser login via `termux-open-url`.
- Kept Android no-voice policy for published `codex-tui` consumers.
- Kept wrapped launcher propagation via `CODEX_SELF_EXE` so helper aliases stay on
  the npm wrapper path and keep `libc++_shared.so` visible on Termux.
- Carried forward fork update-channel behavior so release detection continues to
  target `DioNanos/codex-termux` and `@mmmbuto/codex-cli-termux`.
- Updated the Android network-proxy stub to the upstream `0.118.0` config surface
  so Android builds continue to compile without upstream desktop-only proxy support.
- Continued using the Android API 29 cross-build path for ARM64 Bionic-safe TLS
  output without post-build ELF patching.

### Version
- npm package target: `@mmmbuto/codex-cli-termux@0.118.0-termux`
- Upstream base: `rust-v0.118.0`

### Verification
- Merge + patch audit performed against `rust-v0.118.0`.
- Cross-build/package verification is run on the maintainer host before publish.
- Final native Termux validation remains a post-release manual gate.

---

# [0.117.2-termux] - 2026-03-28

### Release
- Switched the latest-line Android build target from API 28 to API 29.
- This release validates the native ARM64 TLS layout emitted by the Android 29
  linker, replacing the previous post-build ELF patch attempt.
- Promoted `0.117.2-termux` to the npm `latest` dist-tag after packaging and
  launcher validation.

### Version
- npm package target: `@mmmbuto/codex-cli-termux@0.117.2-termux`
- Upstream base: `rust-v0.117.0`

### Verification
- Rebuild Android ARM64 binaries with the API 29 linker.
- Confirm `PT_TLS` layout is accepted by Termux/Bionic without a post-build patch.

---

# [0.117.1-termux] - 2026-03-28

### Hotfix
- Fixed Android ARM64 packaged executables that failed on Termux/Bionic with:
  `executable's TLS segment is underaligned ... needs to be at least 64`.
- Added a maintainer-side ELF patch step for Android ARM64 binaries so
  `codex.bin` and `codex-exec.bin` ship with a Bionic-compatible TLS alignment.

### Version
- npm package target: `@mmmbuto/codex-cli-termux@0.117.1-termux`
- Upstream base: `rust-v0.117.0`

### Verification
- Android ARM64 binaries rebuilt and patched for TLS alignment before npm pack.
- Manual on-device Termux validation remains the final post-publish gate.

---

# [0.117.0-termux] - 2026-03-27

### Upstream
- OpenAI Codex `rust-v0.117.0` release: https://github.com/openai/codex/releases/tag/rust-v0.117.0
- Upstream includes enhanced plugin workflows, improved sub-agent communication, and expanded app-server capabilities.

### Termux Patches
- Rebased the latest Termux line onto `rust-v0.117.0`.
- Kept Android browser login patch (`termux-open-url`) for auth flow.
- Kept Android no-voice policy for `codex-tui` consumers on Android.
- Kept launcher hardening (`codex.bin` / `codex-exec.bin` + sanitized `LD_LIBRARY_PATH`).
- Fixed `apply_patch` helper re-exec on Termux so alias/symlink launches use the wrapped launcher path instead of `codex.bin` directly.
- Revalidated update logic for the fork release channel (`DioNanos/codex-termux`) and `-termux` version parsing.
- Added the optional `termux-tts` skill for Termux text-to-speech workflows.
- Added Android ARM64 maintainer handling for fork-owned `rusty_v8` artifacts when upstream does not ship a usable pair.
- Fixed Android cross-build linkage for `rusty_v8` consumers by moving the Termux release target to Android API 28 and providing an AArch64 `__clear_cache` shim for V8.
- Updated version references to `0.117.0-termux` throughout project and npm metadata.

### Version
- npm package target: `@mmmbuto/codex-cli-termux@0.117.0-termux`
- Upstream base: `rust-v0.117.0`

### Verification
- `bash verify-patches.sh` passes for the latest Termux patch set.
- Rust-side launcher regression coverage was added for `CODEX_SELF_EXE` override handling.
- Android ARM64 package binaries were rebuilt on the maintainer host and packed into the npm wrapper together with `libc++_shared.so`.
- Fork-owned `rusty_v8` Android artifacts were produced locally for `v8 = 146.4.0` and used for the successful release build.
- Manual on-device Termux validation remains the final post-publish gate.

---


# Changelog - Codex Termux

All notable changes to this project will be documented in this file.

## [0.116.2-termux] - 2026-03-20

### Fixes
- Fixed auto-update detection in the `tui_app_server` path to track Termux releases (`DioNanos/codex-termux`) instead of upstream-only latest.
- Fixed tag parsing in `tui_app_server` updates to accept both `rust-vX.Y.Z` and `vX.Y.Z-termux`.
- Fixed Termux update command in `tui_app_server` to use `@mmmbuto/codex-cli-termux@latest` for npm/bun global updates.
- Unified update/release-note links in both TUI implementations to the Termux release channel.

### Version
- npm package target: `@mmmbuto/codex-cli-termux@0.116.2`
- Upstream base: `rust-v0.116.0`

---

## [0.116.0-termux] - 2026-03-20

### Upstream
- OpenAI Codex `rust-v0.116.0` release: https://github.com/openai/codex/releases/tag/rust-v0.116.0
- Upstream includes app-server/TUI auth improvements, plugin install flow updates, and realtime stability fixes.

### Termux Patches
- Rebased the latest Termux line onto `rust-v0.116.0`.
- Kept Android browser login patch (`termux-open-url`) for auth flow.
- Kept Android no-voice policy for `codex-tui` consumers on Android.
- Kept launcher hardening (`codex.bin` / `codex-exec.bin` + sanitized `LD_LIBRARY_PATH`).
- Kept Android PTY linker shim (`openpty`) in `codex-rs/utils/pty/src/pty.rs`.

### Version
- npm package target prepared: `@mmmbuto/codex-cli-termux@0.116.0-termux`
- Upstream base: `rust-v0.116.0`

### Verification (prep cycle)
- Merge commit created: `Merge upstream rust-v0.116.0 into Termux`.
- `bash verify-patches.sh` executed in this cycle.
- Local prep only: no GitHub release, no npm publish.

---

## [0.115.0-termux] - 2026-03-17

### Upstream
- OpenAI Codex `rust-v0.115.0` release: https://github.com/openai/codex/releases/tag/rust-v0.115.0
- Upstream includes multiple core/tooling updates and app-server/protocol refreshes.

### Termux Patches
- Rebased the latest Termux line onto `rust-v0.115.0`.
- Revalidated Termux patches (#1, #2, #4, #5, #6, #9, #10, #11, #12) via `verify-patches.sh`.
- Kept Android no-voice policy for Android consumers of `codex-tui`.
- Kept launcher hardening (`codex.bin` / `codex-exec.bin` plus sanitized `LD_LIBRARY_PATH`).
- Sanitized Android cross-compile config to remove local machine paths from tracked files.

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.115.0-termux`
- Upstream base: `rust-v0.115.0`

### Verification
- Merge completed on top of `rust-v0.115.0`.
- `bash verify-patches.sh` passes after merge.
- Android cross-build + npm tarball assembly prepared for on-device Termux validation before release/publish.

---

## [0.114.0-termux] - 2026-03-11

### Upstream
- OpenAI Codex `rust-v0.114.0` release: https://github.com/openai/codex/releases/tag/rust-v0.114.0
- Upstream added the experimental code mode and hooks engine, plus multiple TUI and app-server improvements.

### Termux Patches
- Rebased the latest Termux line onto `rust-v0.114.0`.
- Kept Android cross-compile settings in `codex-rs/.cargo/config.toml`.
- Kept Android no-voice policy only for Android consumers of `codex-tui` to avoid `libOpenSLES.so` and `oboe` linkage issues on Termux.
- Kept launcher hardening (`codex.bin` / `codex-exec.bin` plus sanitized `LD_LIBRARY_PATH`).

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.114.0-termux`
- Upstream base: `rust-v0.114.0`

### Verification
- Merge completed on top of `rust-v0.114.0`.
- Android cross-build, npm tarball smoke checks, GitHub release, and npm publish are tracked in this release cycle.

---

## [0.113.0-termux] - 2026-03-10

### Upstream
- OpenAI Codex rust-v0.113.0 release: https://github.com/openai/codex/releases/tag/rust-v0.113.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1, #2, #4, #5, #6, #9, #10, #11, #12) revalidated after merge.
- Maintained Android no-voice policy (`default-features = false` for `codex-tui` consumers on Android).
- Maintained launcher hardening (`codex.bin` / `codex-exec.bin` + sanitized `LD_LIBRARY_PATH`).

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.113.0-termux`
- Upstream base: `rust-v0.113.0`

### Verification
- `bash verify-patches.sh` passes.
- Wrapper routing guard still passes (`codex.js fork --help` and `codex.js debug --help` route correctly).
- Termux on-device TUI regression validation remains required before closing the incident gate.

---

## [0.112.0-termux] - 2026-03-09

### Upstream
- OpenAI Codex rust-v0.112.0 release: https://github.com/openai/codex/releases/tag/rust-v0.112.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1, #2, #4, #5, #6, #9, #10, #11) revalidated after merge.
- Added patch #12: npm launcher now discovers root subcommands dynamically from `codex --help`.
- Maintained Android no-voice policy (`default-features = false` for `codex-tui` consumers on Android).
- Maintained launcher hardening (`codex.bin` / `codex-exec.bin` + sanitized `LD_LIBRARY_PATH`).

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.112.0-termux`
- Upstream base: `rust-v0.112.0`

### Verification
- `bash verify-patches.sh` passes.
- Wrapper routing guard passes (`codex.js fork --help` and `codex.js debug --help` route correctly).
- Build, tarball smoke checks, release tag, and npm publish are pending on the remote build host.

---

## [0.111.0-termux] - 2026-03-08

### Upstream
- OpenAI Codex rust-v0.111.0 release: https://github.com/openai/codex/releases/tag/rust-v0.111.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1, #2, #4, #5, #6, #9, #10) revalidated after merge.
- Termux release now excludes `voice-input` on Android consumers to avoid runtime linkage on `libOpenSLES.so`.
- Launcher hardening retained (`codex.bin` / `codex-exec.bin` + safe `LD_LIBRARY_PATH`).

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.111.0-termux`
- Upstream base: `rust-v0.111.0`

### Verification
- Android consumer manifests disable `codex-tui` default features on Termux builds.
- Runtime validation and published test report are pending on-device Termux verification.

---

## [0.110.0-termux] - 2026-03-05

### Upstream
- OpenAI Codex rust-v0.110.0 release: https://github.com/openai/codex/releases/tag/rust-v0.110.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1, #2, #4, #5, #6, #9, #10) revalidated after merge.
- Android audio/linker guard retained (`cpal` with `oboe-shared-stdcxx`).
- Launcher hardening retained (`codex.bin` / `codex-exec.bin` + safe `LD_LIBRARY_PATH`).

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.110.0-termux`
- Upstream base: `rust-v0.110.0`

---

## [0.108.0-termux] - 2026-03-04

### Upstream
- OpenAI Codex rust-v0.108.0 release: https://github.com/openai/codex/releases/tag/rust-v0.108.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1, #2, #4, #5, #6, #9, #10) revalidated via `verify-patches.sh`.
- Launcher hardening from 0.106.2 (`codex.bin` / `codex-exec.bin` + safe `LD_LIBRARY_PATH`) retained.

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.108.0-termux`
- Upstream base: `rust-v0.108.0`

### Verification
- `env -u LD_LIBRARY_PATH ./bin/codex --version` -> PASS
- `env -u LD_LIBRARY_PATH ./bin/codex-exec --version` -> PASS
- `node ./bin/codex.js --version` -> PASS
- `node ./bin/codex-exec.js --version` -> PASS

---

## [0.106.2-termux] - 2026-03-02

### Packaging Fix
- Fixed direct binary invocation when `LD_LIBRARY_PATH` is missing.
- `npm-package/bin/codex` and `npm-package/bin/codex-exec` are now launcher scripts that export a safe `LD_LIBRARY_PATH` and invoke:
  - `bin/codex.bin`
  - `bin/codex-exec.bin`
- This resolves failures such as:
  - `CANNOT LINK EXECUTABLE .../bin/codex: library "libc++_shared.so" not found`
  when tools invoke package binaries directly (without Node launcher env setup).

### Version
- npm package bumped: `@mmmbuto/codex-cli-termux@0.106.2-termux`
- Upstream base remains: `rust-v0.106.0`

### Verification
- `env -u LD_LIBRARY_PATH ./bin/codex --version` -> PASS
- `env -u LD_LIBRARY_PATH ./bin/codex-exec --version` -> PASS
- `node ./bin/codex.js --version` -> PASS
- `node ./bin/codex-exec.js --version` -> PASS

---

## [0.88.0-termux] - 2026-01-22

### Upstream
- OpenAI Codex rust-v0.88.0 release: https://github.com/openai/codex/releases/tag/rust-v0.88.0
- Upstream release notes and details are maintained in link above.
- **New features:**
  * Collaboration modes and presets to streamline multi-agent workflows
  * Device-code auth as a standalone fallback in headless environments
  * Request-user-input tool for explicit agent prompts
  * Remote models and auto-enable WebSockets transport
  * Thread/fork endpoints (conversation branching)

### Termux Patches
- Termux patches (#1–#6, #9) revalidated via verify-patches.sh.
- **Patch #8 (bash execution)**: Not required (resolved upstream v0.80.0+)
- **Patch #2 (compilation)**: Updated with rustls fix (native-tls → rustls)

### Testing
- CODEX_TEST_REPORT_v0.88.0.md on SamsungWork (2026-01-22): ALL PASS
- Binary sizes: codex 65MB, codex-exec 38MB
- All Termux patches verified (#1–#6, #9)

### Documentation
- Updated patches/README.md for v0.88.0
- Updated STATUS.md with release status
- Created tag: v0.88.0-termux
- Ready for npm publish

---

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.80.0-termux] - 2026-01-10

### Upstream
- OpenAI Codex rust-v0.80.0 release: https://github.com/openai/codex/releases/tag/rust-v0.80.0
- Upstream release notes and details are maintained in the link above.
- **Important**: Process hardening removed from Codex CLI (upstream PR #8951)
- New features:
  * Thread/fork endpoints (conversation branching)
  * Requirements/list API
  * Elevated sandbox onboarding NUX
  * Skills explicit invocation via V2 API
  * Metrics capabilities (otel/metrics module)

### Termux Patches
- Termux patches (#1–#6, #9) revalidated via `verify-patches.sh`.
- **Patch #8 (bash execution)**: No longer required - resolved by upstream PR #8951 (process hardening removal)
- This improves bash execution in Agent mode without custom patches.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-10): 49 tests, 49 passed / 0 failed / 0 skipped.
- Package & Binary 8/8 passed; Termux-specific 10/10 passed.
- All binaries verified: codex (60M), codex-tui (42M), codex-exec (35M), codex-app-server (38M).

### Documentation
- Updated patches/README.md for v0.80.0 with Patch #8 resolution notes
- Updated README.md with v0.80.0 version references
- Removed outdated CODEX_TEST_REPORT_v0.79.0.md

---
## [0.79.0-termux] - 2026-01-08

### Upstream
- OpenAI Codex rust-v0.79.0 release: https://github.com/openai/codex/releases/tag/rust-v0.79.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1–#6, #8, #9) revalidated via `verify-patches.sh`.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-08): 49 tests, 47 passed / 0 failed / 2 skipped (WebSearch unavailable; git info skipped outside repo). Package & Binary 8/8 passed; Termux-specific 10/10 passed.

### Documentation
- Added GLM-4.7 quickstart guide (later removed when docs were generalized)
- Updated README.md with GLM-4.7 setup instructions in Quickstart section

---

## [0.78.0-termux] - 2026-01-06

### Upstream
- OpenAI Codex rust-v0.78.0 release: https://github.com/openai/codex/releases/tag/rust-v0.78.0
- Upstream release notes and details are maintained in the link above.

### Termux Patches
- Termux patches (#1–#6, #8, #9) revalidated via `verify-patches.sh`.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-06): 49 tests, 47 passed / 0 failed / 2 skipped (WebSearch unavailable; git info skipped outside repo). Package & Binary 8/8 passed; Termux-specific 10/10 passed.

---

## [0.77.1-termux] - 2026-01-04

### Upstream
- Base release: rust-v0.77.0 — https://github.com/openai/codex/releases/tag/rust-v0.77.0
- Termux build synced after rust-v0.77.0 (upstream commit range not listed here; see upstream history for details).

### Termux Patches
- Single entrypoint confirmed: `codex` for TUI; `codex exec` for automation; `codex-exec` kept as JS wrapper (no symlink).
- Termux patches (#1–#6, #8, #9) revalidated via `verify-patches.sh`.

### Testing
- CODEX_TEST_SUITE v1.2 on Termux (2026-01-04): 49 tests, 47 passed / 0 failed / 2 skipped (WebSearch disabled; git info skipped outside repo). Package & Binary 8/8 passed; Termux-specific 10/10 passed.
