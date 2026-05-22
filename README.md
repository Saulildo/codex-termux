# Codex Termux

> Native Codex CLI for **Termux / Android ARM64**.
> This fork tracks upstream OpenAI Codex main and carries only the Android/Termux compatibility delta needed to package and run it.

[![npm termux](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![latest release](https://img.shields.io/github/v/release/DioNanos/codex-termux?style=flat-square)](https://github.com/DioNanos/codex-termux/releases/latest)

<p align="center">
  <img src="./.github/termux-robot.png" alt="Termux robot" width="80%" />
</p>

## Install

### Termux (Android ARM64)

```bash
pkg update && pkg upgrade -y
pkg install nodejs-lts -y
npm install -g @mmmbuto/codex-cli-termux@latest
codex --version
codex login
```

Requirements:

- Android 7+ / API 24+
- ARM64 device
- Node.js >= 18

## Scope

What this fork does:

- tracks upstream OpenAI Codex closely
- builds native Android ARM64 binaries for Termux
- applies only the compatibility patches upstream does not ship
- publishes GitHub release assets and an npm package for Termux users

What this fork does not do:

- maintain a broad feature fork
- replace upstream Codex
- carry fork-only product features unrelated to Termux compatibility

## Current Termux Delta

- browser login uses `termux-open-url`
- self-update points to `DioNanos/codex-termux` and `@mmmbuto/codex-cli-termux`
- packaged wrappers preserve `CODEX_SELF_EXE`, sanitize `LD_LIBRARY_PATH`, and bundle `libc++_shared.so`
- Android binaries are linked with `RUNPATH=$ORIGIN`
- voice and realtime audio remain disabled in the published Termux package
- Android PTY and lock-handling compatibility patches remain enabled where upstream behavior still breaks on Bionic/Termux
- `exec`/code-mode is built for Android Termux and routed through the packaged `codex-exec` binary

## Releases and Updates

- Latest GitHub release: [releases/latest](https://github.com/DioNanos/codex-termux/releases/latest)
- Upstream base: OpenAI Codex `rust-v0.133.0`, packaged as `0.133.0` for npm `latest`
- npm package: [`@mmmbuto/codex-cli-termux`](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)

Maintainer publish flow:

- land validated changes on `develop`
- publish the tested npm package to `latest`
- promote the tested commit to clean GitHub `main`
- publish the GitHub release from `main`
- add post-release Termux validation reports after device testing

## Documentation

- [Changelog](./CHANGELOG.md)
- [Patch inventory](./patches/README.md)
- [Building from source](./BUILDING.md)
- Latest runtime validation report: [v0.131.0 Termux device smoke](./test-report/CODEX_TEST_REPORT_v0.131.0_run_20260519-1601.md)
- [Install docs](./docs/install.md)
- [Authentication](./docs/authentication.md)
- [Configuration](./docs/config.md)

## License

This project remains under the Apache 2.0 license inherited from OpenAI Codex.

- Original work: OpenAI
- Termux port: minimal Android compatibility patches

See [LICENSE](./LICENSE).
