# [0.134.0-termux] - 2026-05-26

## Changed
- Synced the Termux fork to upstream OpenAI Codex `rust-v0.134.0` (stable).
- Preserved the Android/Termux runtime delta: browser login via
  `termux-open-url`, fork-owned update channels, npm wrapper hardening,
  ELF `RUNPATH=$ORIGIN`, Android no-voice policy, Termux-compatible
  release profile, and the `flock` ENOTSUP/EOPNOTSUPP tolerance for
  `codex remote-control` on Termux storage backends.
- Kept public install, source-build, update, and release-staging surfaces on
  `DioNanos/codex-termux` and `@mmmbuto/codex-cli-termux`.
- Synchronised README upstream-base references (root, npm package,
  `docs/install.md`, `BUILDING.md`) to `rust-v0.134.0`.
- Aligned Cargo workspace and lockfile package versions with the upstream
  `0.134.0` release.
- `rusty_v8` Android prebuilt manifest is unchanged: V8 stays at 147.4.0.

## Documented
- Surfaced 8 previously-undocumented Android/Termux compatibility patches
  in `patches/README.md` and added matching checks to `verify-patches.sh`
  (Patches #6b, #17, #18, #19, #20, #21, #22, #23). No new code; the
  inventory now reflects the existing fork delta accurately.

## Upstream
- Search across local conversation history (case-insensitive content matches
  with result previews).
- `--profile` becomes the primary profile selector across CLI, TUI
  permissions, and sandbox flows; legacy profile configs are rejected with
  migration guidance.
- MCP setup gained per-server environment targeting and OAuth options for
  streamable HTTP servers.
- Connector tool schemas more reliable: preserves local `$ref`/`$defs`
  structures and compacts oversized schemas before exposure.
- Read-only MCP tools can run concurrently when they advertise `readOnlyHint`.
- Richer extension and hook context: conversation history for extension
  tools, subagent identity in hook inputs.
- Goal accounting restored after thread resume; memory state moved to a
  dedicated SQLite DB.

# [0.133.1-termux] - 2026-05-23

## Fixed
- `codex remote-control` no longer aborts daemon startup with
  `lock() not supported` on Android Termux storage backends that
  reject `flock(2)` with `ENOTSUP` / `EOPNOTSUPP`. The two
  `try_lock_file` helpers in `app-server-daemon` now match the
  permissive degradation pattern already used by
  `core::installation_id::is_unsupported_file_lock_error` and return
  "lock acquired" when the OS reports the primitive is unsupported, so
  the daemon start path proceeds and the app-server can bind its
  socket. The race the locks were guarding against is best-effort on
  these filesystems; refusing to start the daemon was the real
  blocker.

## Upstream
- OpenAI Codex `rust-v0.133.0` (unchanged from `0.133.0`'s parent release).

# [0.133.0-termux] - 2026-05-22

## Changed
- Synced the Termux fork to upstream OpenAI Codex `rust-v0.133.0`.
- Preserved the Android/Termux runtime delta: browser login via
  `termux-open-url`, fork-owned update channels, npm wrapper hardening,
  ELF `RUNPATH=$ORIGIN`, Android no-voice policy, and Termux-compatible
  release profile.
- Kept public install, source-build, update, and release-staging surfaces on
  `DioNanos/codex-termux` and `@mmmbuto/codex-cli-termux`.
- Updated Android `rusty_v8` prebuilt artifacts to v147.4.0.
- Aligned Cargo workspace and lockfile package versions with the upstream
  `0.133.0` release.

## Upstream
- Goals are now enabled by default, backed by dedicated storage, and track
  progress across active turns.
- `codex remote-control` now runs like a foreground command, waits for
  readiness, reports machine status, and keeps explicit daemon-style
  `start`/`stop` commands.
- Permission profiles gained list APIs, inheritance, managed
  `requirements.toml` support, runtime refresh behavior, and stronger Windows
  sandbox integration.
- Plugin discovery is easier to inspect, with marketplace-aware list output,
  installed versions, visible marketplace roots, and remote collection support.
- Extensions can observe more lifecycle events, including subagent start/stop,
  tool execution, turn metadata, and async approval/turn processing.

# [0.132.0-termux] - 2026-05-20

## Changed
- Synced the Termux fork to upstream OpenAI Codex `rust-v0.132.0`.
- Preserved the Android/Termux runtime delta: browser login via
  `termux-open-url`, fork-owned update channels, npm wrapper hardening,
  ELF `RUNPATH=$ORIGIN`, Android no-voice policy, code-mode Android support,
  and Termux-compatible release profile.
- Kept public install, source-build, update, and release-staging surfaces on
  `DioNanos/codex-termux` and `@mmmbuto/codex-cli-termux`.
- Aligned Cargo workspace and lockfile package versions with the upstream
  `0.132.0` release.

## Upstream
- Python SDK authentication now includes API key login, ChatGPT browser and
  device-code flows, account inspection, and logout APIs.
- Python turn APIs accept plain string input for text-only workflows and return
  richer `TurnResult` metadata for handle-based runs.
- `codex exec resume` supports `--output-schema` for structured resumed
  automations.
- TUI startup probes are batched for faster first-frame rendering.
- Remote executor registration can use standard Codex auth.
- App-server turns preserve requested image fidelity, including
  original-resolution local images.

# [0.131.1-termux] - 2026-05-19

## Changed
- Completed fork-safety coverage for diagnostic update guidance: `codex doctor`
  now points npm and bun users to `@mmmbuto/codex-cli-termux`, and release
  checks read `DioNanos/codex-termux` tags.
- Kept TUI update notices on the fork release channel so failed update probes no
  longer send users to upstream OpenAI Codex releases.
- Reworked the Android no-voice policy to use target-OS cfg gates instead of a
  workspace crate feature, matching upstream manifest validation rules while
  keeping voice and realtime audio disabled for Termux builds.
- Updated patch verification so the fork-safety and no-voice invariants are
  checked together before release.

## Upstream
- Upstream base remains OpenAI Codex `rust-v0.131.0`; this is a Termux fork
  patch release with no upstream base change.

# [0.131.0-termux] - 2026-05-19

## Changed
- Synced the Termux fork to upstream OpenAI Codex `rust-v0.131.0`.
- Preserved the Android/Termux runtime delta: browser login via `termux-open-url`, fork-owned update channels, npm wrapper hardening, ELF `RUNPATH=$ORIGIN`, Android no-voice policy, code-mode Android stubs, and Termux-compatible release profile.
- Hardened fork update paths so standalone update actions and app-server daemon guidance stay on `@mmmbuto/codex-cli-termux@latest`.
- Disabled daemon automatic standalone updater fetches for the Termux fork and kept `autoUpdateEnabled` false by default.
- Aligned Cargo.lock with the upstream `rust-v0.131.0` dependency resolution while preserving workspace version `0.131.0`.

## Upstream
- OpenAI Codex `rust-v0.131.0` is the upstream base for this Termux package.
  The fork includes the upstream CLI/TUI improvements that are compatible with
  Android Termux, while preserving the Termux packaging and no-voice policy.
- Upstream TUI updates include richer session controls and display metadata:
  service-tier commands, blended token usage, permission/approval display,
  effective workspace roots, and responsive Markdown tables.
- Upstream `@` mentions now cover files, directories, plugins, and skills in a
  unified picker backed by app-server plugin metadata.
- Upstream plugin workflows add marketplace commands, version-aware sharing,
  clearer shared-workspace buckets, and default-enabled plugin hooks.
- Upstream remote workflows add daemon-managed `codex remote-control`, runtime
  enable/disable APIs, status reads, and configured remote environments.
- Upstream SDK and diagnostics updates include the `openai-codex` /
  `openai_codex` Python line, approval-mode coverage, and `codex doctor`.

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
