# [0.120.0-termux] - 2026-04-11

### Upstream
- OpenAI Codex `rust-v0.120.0` release: https://github.com/openai/codex/releases/tag/rust-v0.120.0
- Fork line rebuilt cleanly from upstream stable instead of extending the old public fork history.

### Termux Patches
- Kept Android browser login via `termux-open-url`.
- Kept the fork update channel and `-termux` version parsing for self-update UX.
- Kept Termux npm package/update commands targeting `@mmmbuto/codex-cli-termux`.
- Kept launcher hardening via wrapped entrypoints and `CODEX_SELF_EXE`.
- Included the Android ELF `RUNPATH=$ORIGIN` hardening from fork PR #1 so direct native invocation still resolves bundled `libc++_shared.so`.
- Kept the Android no-voice policy for the published Termux package.
- Kept the Android `openpty` shim for PTY compatibility on Bionic.
- Dropped the old Android `network-proxy` stub because upstream now compiles directly on Android through the Unix-family path.
- Fixed Android session bootstrap after the upstream `installation_id` locking change by tolerating unsupported file locks on Termux.

### Version
- npm package target: `@mmmbuto/codex-cli-termux@0.120.0-termux`
- Upstream base: `rust-v0.120.0`

### Verification
- `bash verify-patches.sh`
- Release packaging smoke for `codex`, `codex-exec`, and wrapper routing
- Native Termux validation remains a post-release manual gate
