# Codex Termux

> Latest Termux line built from upstream OpenAI Codex `rust-v0.119.0`. The separate LTS line remains available as `@mmmbuto/codex-cli-lts`.

[![npm termux](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
[![npm lts](https://img.shields.io/npm/v/@mmmbuto/codex-cli-lts?style=flat-square&logo=npm)](https://www.npmjs.com/package/@mmmbuto/codex-cli-lts)
[![latest release](https://img.shields.io/github/v/release/DioNanos/codex-termux?style=flat-square)](https://github.com/DioNanos/codex-termux/releases/latest)

<p align="center">
  <img src="https://github.com/openai/codex/blob/main/.github/codex-cli-splash.png" alt="Codex CLI splash" width="80%" />
</p>

## About

This fork keeps two public release lines:

- `main`: latest Termux-focused line published as `@mmmbuto/codex-cli-termux`
- `lts`: long-term support line published as `@mmmbuto/codex-cli-lts`

The latest line tracks upstream OpenAI Codex closely and carries only the
compatibility patches required to keep Codex usable on Android Termux.

## Release Lines

- Latest: `@mmmbuto/codex-cli-termux@0.119.0-termux`
- Upstream base: `rust-v0.119.0`
- LTS: separate compatibility-focused line published as `@mmmbuto/codex-cli-lts`

Supported platforms:

- Android Termux ARM64: latest + LTS
- Linux x64/arm64: LTS
- macOS x64/arm64: LTS

## Project Scope

### Latest (Termux-only)

- Native ARM64 Android build for Termux
- Tracks upstream OpenAI Codex closely
- Minimal compatibility delta only
- Fork update checks and release links point to `DioNanos/codex-termux`

### LTS (Multi-platform)

- Conservative support line for compatibility-focused use
- Separate npm package: `@mmmbuto/codex-cli-lts`
- Linux and macOS remain supported there

### What This Fork Does

- Uses the official OpenAI Codex source as upstream
- Builds native Android ARM64 binaries for Termux
- Applies only compatibility patches that upstream does not carry
- Publishes release artifacts on GitHub and npm for Termux users

### What This Fork Does Not Do

- Maintain a broad feature fork
- Replace upstream Codex
- Carry unrelated product behavior changes

## Termux Compatibility Notes

Current Termux-specific carry patches include:

- browser login via `termux-open-url`
- launcher hardening for helper re-exec
- `RUNPATH=$ORIGIN` on Android ELF binaries
- fork-specific update channel and release links

Current published limitation:

- voice and realtime audio remain disabled in the Termux latest package

This keeps the packaged binaries free of Android audio linker dependencies while
preserving the rest of the upstream CLI flow.

## Installation

### Termux (Android ARM64)

```bash
pkg update && pkg upgrade -y
pkg install nodejs-lts -y

# Latest Termux line
npm install -g @mmmbuto/codex-cli-termux@latest

# Verify
codex --version
codex login
```

Requirements:

- Android 7+ / API 24+
- ARM64 device
- Node.js >= 18

### Linux and macOS

For non-Termux systems, use the LTS line instead:

```bash
npm install -g @mmmbuto/codex-cli-lts
codex --version
codex login
```

## Releases and Updates

- Latest GitHub release: [v0.119.0-termux](https://github.com/DioNanos/codex-termux/releases/tag/v0.119.0-termux)
- Upstream release base: [rust-v0.119.0](https://github.com/openai/codex/releases/tag/rust-v0.119.0)
- npm latest: [`@mmmbuto/codex-cli-termux`](https://www.npmjs.com/package/@mmmbuto/codex-cli-termux)
- npm LTS: [`@mmmbuto/codex-cli-lts`](https://www.npmjs.com/package/@mmmbuto/codex-cli-lts)

## Documentation

- [Changelog](./CHANGELOG.md)
- [Patch inventory](./patches/README.md)
- [Building from source](./BUILDING.md)
- [Install and build docs](./docs/install.md)
- [Authentication](./docs/authentication.md)
- [Configuration](./docs/config.md)

## License

This project remains under the Apache 2.0 license inherited from OpenAI Codex.

- Original work: OpenAI
- Termux port: minimal Android compatibility patches

See [LICENSE](./LICENSE).
