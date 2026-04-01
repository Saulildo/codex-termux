# Codex CLI for Termux

> Latest Android Termux package built from upstream OpenAI Codex `rust-v0.118.0`.

[![npm termux](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-termux)
[![npm lts](https://img.shields.io/npm/v/@mmmbuto/codex-cli-lts?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-lts)

---

![Codex Termux Header](https://github.com/DioNanos/codex-termux/blob/main/docs/assets/codex-termux-header.jpg?raw=1)

## About

This npm package is the latest Termux-focused line: `@mmmbuto/codex-cli-termux`.

If you want the separate multi-platform LTS line, use `@mmmbuto/codex-cli-lts`.

**Supported platform:** Android Termux (ARM64)
**Current package line:** `0.118.0-termux`

---

## Scope

- Built from official OpenAI Codex source: https://github.com/openai/codex
- Tracks upstream closely
- Applies only the compatibility patches required for Android Termux
- Ships a native Android ARM64 binary plus launchers

---

## Patches & Updates

### Latest Patches
We apply patches for issues that:
- **Prevent Codex from working on Termux**
- **Are not addressed by upstream** (Termux is not officially supported)
- **Are minimal and well-documented**

**Current patches**: See [patches/README.md](https://github.com/DioNanos/codex-termux/blob/main/patches/README.md) for full documentation.

**Termux build note:** the published latest Termux package disables voice/realtime audio.
This avoids Android linker failures such as missing `libOpenSLES.so` while keeping the
rest of the upstream CLI behavior intact.

The packaged launchers also preserve a wrapped self-exe path for helper tools.
This keeps relaunches such as `apply_patch` on the launcher path so bundled
libraries like `libc++_shared.so` remain visible on Termux.

### LTS Updates
- Based on rust-v0.80.0 (minimal features + security only)
- Maintains /chat wire API compatibility
- Stability-focused for production use

Need help debugging upgrade alerts? See
[docs/termux-upgrade-checks.md](https://github.com/DioNanos/codex-termux/blob/main/docs/termux-upgrade-checks.md)
for known causes and fix strategies.

**Found an issue?** Well-documented bug reports with reproduction steps are welcome! Open an [issue](https://github.com/DioNanos/codex-termux/issues).

---

## 📦 Installation

### Termux (Android ARM64)

```bash
# Update Termux packages and install Node.js
pkg update && pkg upgrade -y
pkg install nodejs-lts -y

# Install latest Termux line
npm install -g @mmmbuto/codex-cli-termux@latest

# Verify
codex --version
codex login
```

**Requirements:** Android 7+, ARM64, Node.js >=18 (recommended v22+), ~50MB storage

**Current latest limitation:** voice/realtime audio is intentionally disabled in the
published Termux builds to keep packaged binaries free of Android-only audio linker
dependencies.

**Release validation note:** the package is built and published first, then
validated on native Termux as a manual post-release gate.

---

For the LTS line on Linux/macOS/Termux, see the main repository README and `@mmmbuto/codex-cli-lts`.

## Documentation

- [Installation Details](https://github.com/DioNanos/codex-termux/blob/main/docs/installation.md)
- [Testing](https://github.com/DioNanos/codex-termux/blob/main/docs/testing.md)
- [Building from Source](https://github.com/DioNanos/codex-termux/blob/main/BUILDING.md)
- [Test Reports](https://github.com/DioNanos/codex-termux/tree/main/test-reports)
- [Main Repository README](https://github.com/DioNanos/codex-termux/blob/main/README.md)

---

## License

Original work by OpenAI: https://github.com/openai/codex
Termux packaging and compatibility patches by this fork.

See [LICENSE](https://github.com/DioNanos/codex-termux/blob/main/LICENSE) for details.
