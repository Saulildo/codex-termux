# [0.130.0-termux] - 2026-05-09

## Changed
- Synced the Termux fork to upstream OpenAI Codex `rust-v0.130.0`.
- Preserved all 9 Termux patches: browser login, release profile, update channel, npm scope, launcher, ELF runpath, no-voice, dynamic subcommand routing, Bazel inventory.
- Upstream highlights: plugin details/show bundled hooks, plugin share discoverability, `codex remote-control` headless entrypoint, thread pagination APIs, Bedrock AWS console-login auth, `view_image` multi-environment support, built-in MCPs as first-class runtime servers.

# [0.128.0-termux] - 2026-04-30

## Changed
- Synced the Termux fork to upstream OpenAI Codex `rust-v0.128.0`.
- Preserved Termux packaging, update URLs, Android runtime patches, and fork npm scope `@mmmbuto/codex-cli-termux`.

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
