## Installing Codex CLI for Termux

This package is for Android Termux on ARM64 devices.

### Requirements

| Requirement | Details |
| --- | --- |
| Android | Android 7+ / API 24+ |
| CPU | ARM64 |
| Shell | Termux |
| Node.js | 18+ |

### Install from npm

```bash
pkg update && pkg upgrade -y
pkg install nodejs-lts -y
npm install -g @mmmbuto/codex-cli-termux@latest
codex --version
codex login
```

The npm package includes native Android ARM64 `codex` and `codex-exec`
binaries, wrapper scripts, and the bundled `libc++_shared.so` runtime library.

### Install from GitHub release

Download the `mmmbuto-codex-cli-termux-<version>.tgz` asset from the matching
GitHub release, then install it with npm:

```bash
npm install -g ./mmmbuto-codex-cli-termux-0.134.0.tgz
codex --version
```

Each release also publishes a `.sha256` checksum file for the npm tarball.

### Build from source

For source builds and maintainer cross-build notes, see [BUILDING.md](../BUILDING.md).

## Logging

Codex honors the `RUST_LOG` environment variable. The TUI writes logs under the
Codex log directory by default, and `codex exec` prints error-level messages
inline for non-interactive runs.
