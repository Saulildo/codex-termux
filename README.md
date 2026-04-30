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
- `exec`/code-mode remains disabled in the Android Termux package

## Releases and Updates

- Latest GitHub release: [releases/latest](https://github.com/DioNanos/codex-termux/releases/latest)
- Upstream base: OpenAI Codex main, packaged as `0.126.0-termux`
- npm package: [`@mmmbuto/codex-cli-termux`](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)

Maintainer publish flow:

- land validated changes on `develop`
- run `.github/workflows/termux-npm-build-publish.yml` once with `source_ref=develop` and `create_release=true`
- validate the release assets
- run it again with `source_ref=develop`, `publish_npm=true`, and the desired dist-tag
- after Termux validation passes, promote the tested commit to `main` and publish the GitHub release

## Documentation

- [Changelog](./CHANGELOG.md)
- [Patch inventory](./patches/README.md)
- [Building from source](./BUILDING.md)
- [Runtime validation report](./test-reports/latest/CLI_RUNTIME_REPORT.md)
- [Install docs](./docs/install.md)
- [Authentication](./docs/authentication.md)
- [Configuration](./docs/config.md)

## License

This project remains under the Apache 2.0 license inherited from OpenAI Codex.

- Original work: OpenAI
- Termux port: minimal Android compatibility patches

See [LICENSE](./LICENSE).
