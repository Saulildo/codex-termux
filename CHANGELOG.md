# [0.126.0-termux] - 2026-04-30

- Merged current upstream OpenAI Codex `main` into the Termux fork.
- Kept Android/Termux packaging, rusty_v8 Android artifact fetching, voice/realtime stubs, and npm self-update targeting `@mmmbuto/codex-cli-termux`.
- Prepared GitHub Actions Android build/publish pipeline for staged `next` testing before any `latest` or public release promotion.

# [0.125.0-termux] - 2026-04-26

### Upstream
- OpenAI Codex `rust-v0.125.0` release: https://github.com/openai/codex/releases/tag/rust-v0.125.0
- Fork line rebuilt cleanly from upstream `rust-v0.125.0`.

### Termux Patches
- Kept Android browser login via `termux-open-url`.
- Kept the fork update channel and `-termux` version parsing for self-update UX.
- Kept Termux npm package/update commands targeting `@mmmbuto/codex-cli-termux`.
- Kept launcher hardening via wrapped entrypoints and `CODEX_SELF_EXE`.
- Kept Android ELF `RUNPATH=$ORIGIN` hardening so direct native invocation still resolves bundled `libc++_shared.so`.
- Kept the Android no-voice policy for the published Termux package.
- Kept Android `exec`/code-mode disabled in the published Termux package.
- Kept the Android `openpty` shim for PTY compatibility on Bionic.
- Kept Android tolerance for unsupported file locks used by `installation_id` and arg0 helper setup.
- Aligned code-mode Android stubs with upstream 0.125.0 runtime restructuring (new `runtime/` dir, `WaitOutcome`, `CodeModeNestedToolCall`).

### Build and Release
- Kept the maintainer-oriented `BUILDING.md` for native Termux and Linux-host Android builds.
- Kept fork-owned `rusty_v8` Android artifact manifest and fetch helpers.
- Kept the dispatchable GitHub Actions workflow to cross-build, package, release, and publish the Termux npm package.

### Version
- npm package target: `@mmmbuto/codex-cli-termux@0.125.0-termux`
- Upstream base: `rust-v0.125.0`

### Verification
- `bash verify-patches.sh`
- local Android cross-build of `codex` and `codex-exec`
- npm wrapper smoke for `--version` and `--help`
- GitHub Actions release build before npm publish

# [0.124.0-termux] - 2026-04-24

### Upstream
- OpenAI Codex `rust-v0.124.0` release: https://github.com/openai/codex/releases/tag/rust-v0.124.0
- Fork line rebuilt cleanly from upstream `rust-v0.124.0`.

### Termux Patches
- Kept Android browser login via `termux-open-url`.
- Kept the fork update channel and `-termux` version parsing for self-update UX.
- Kept Termux npm package/update commands targeting `@mmmbuto/codex-cli-termux`.
- Kept launcher hardening via wrapped entrypoints and `CODEX_SELF_EXE`.
- Kept Android ELF `RUNPATH=$ORIGIN` hardening so direct native invocation still resolves bundled `libc++_shared.so`.
- Kept the Android no-voice policy for the published Termux package.
- Kept Android `exec`/code-mode disabled in the published Termux package.
- Kept the Android `openpty` shim for PTY compatibility on Bionic.
- Kept Android tolerance for unsupported file locks used by `installation_id` and arg0 helper setup.
- Removed the legacy `/vivling` fork feature from the public Termux line.

### Build and Release
- Kept the maintainer-oriented `BUILDING.md` for native Termux and Linux-host Android builds.
- Kept fork-owned `rusty_v8` Android artifact manifest and fetch helpers.
- Kept the dispatchable GitHub Actions workflow to cross-build, package, release, and publish the Termux npm package.

### Version
- npm package target: `@mmmbuto/codex-cli-termux@0.124.0-termux`
- Upstream base: `rust-v0.124.0`

### Verification
- `bash verify-patches.sh`
- local Android cross-build of `codex` and `codex-exec`
- npm wrapper smoke for `--version` and `--help`
- GitHub Actions release build before npm publish
