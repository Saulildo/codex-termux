# Codex Termux

> Latest Termux line built from upstream OpenAI Codex `rust-v0.118.0`. The separate LTS line remains available for Termux, Linux, and macOS as `@mmmbuto/codex-cli-lts`.

[![npm termux](https://img.shields.io/npm/v/@mmmbuto/codex-cli-termux?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-termux)
[![npm lts](https://img.shields.io/npm/v/@mmmbuto/codex-cli-lts?style=flat-square&logo=npm)](https://www.npmjs.org/package/@mmmbuto/codex-cli-lts)

---

![Codex Termux Header](./docs/assets/codex-termux-header.jpg)

## About

This fork keeps two public release lines:
- `main`: latest Termux-focused line published as `@mmmbuto/codex-cli-termux`
- `lts`: long-term support line published as `@mmmbuto/codex-cli-lts`

The latest line tracks upstream OpenAI Codex closely and applies only the compatibility patches required to keep Android Termux usable.

### Release Lines

- **Latest**: `@mmmbuto/codex-cli-termux@0.118.0-termux`
- **LTS**: Long-term support based on upstream rust-v0.80.0, minimal features + security patches to maintain /chat compatibility

**Supported Platforms:**
- Android Termux (ARM64): Latest + LTS
- Linux x64/arm64: LTS
- macOS x64/arm64: LTS

---

## Project Scope

### Skills 
- Termux-specific skills are available in `.codex/skills/` directory. 
- Includes `termux-tts` skill for text-to-speech functionality using `termux-tts-speak` command. 
- Skills enhance Termux experience without modifying core functionality. 


### Latest (Termux-only)
- Compiles for ARM64 native on Android Termux
- Tracks upstream OpenAI closely
- Applies minimal patches for Termux-specific compatibility

### LTS (Multi-platform)
- Based on upstream rust-v0.80.0
- Supports /chat and /responses wire APIs
- Minimal features + security patches only
- Stable for compatibility-focused use cases

### What We Do
- Use official OpenAI Codex source: https://github.com/openai/codex
- Compile native Android ARM64 binaries for Termux
- Apply minimal compatibility patches only where upstream does not support Termux
- Publish npm packages for the latest Termux line and the separate LTS line

### What We Do Not Do
- Maintain a divergent feature fork
- Replace upstream Codex
- Carry broad behavior changes unrelated to compatibility

---

## Patches & Updates

### Latest Patches
We apply patches for issues that:
- **Prevent Codex from working on Termux**
- **Are not addressed by upstream** (Termux is not officially supported)
- **Are minimal and well-documented**

**Current patches**: See [patches/README.md](./patches/README.md) for full documentation.

**Termux build note:** the published latest Termux package disables voice/realtime audio.
This avoids Android linker failures such as missing `libOpenSLES.so` while keeping the
rest of the upstream CLI behavior intact.

The npm launchers also preserve a wrapped self-exe path for helper tools.
This keeps helper re-exec flows such as `apply_patch` on the launcher path so
bundled Termux libraries like `libc++_shared.so` remain available.

### LTS Updates
- Based on rust-v0.80.0 (minimal features + security only)
- Maintains /chat wire API compatibility
- Stability-focused for production use

Need help debugging upgrade alerts? See
[docs/termux-upgrade-checks.md](./docs/termux-upgrade-checks.md) for known causes
and fix strategies.

**Found an issue?** Well-documented bug reports with reproduction steps are welcome! Open an [issue](https://github.com/DioNanos/codex-termux/issues).

---

## 📦 Installation

### Termux (Android ARM64)

```bash
# Update Termux packages and install Node.js
pkg update && pkg upgrade -y
pkg install nodejs-lts -y

# Latest Termux line
npm install -g @mmmbuto/codex-cli-termux@latest

# LTS line
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** Android 7+, ARM64, Node.js >=18 (recommended v22+), ~50MB storage

**Current latest:** `0.118.0-termux`

**Current latest limitation:** voice/realtime audio is intentionally disabled in the
published Termux builds to keep packaged binaries free of Android-only audio linker
dependencies.

**Release validation note:** this release is built, packaged, and published from the
maintainer host, then validated on native Termux as a post-release manual gate.
The full on-device suite is not claimed as complete until that Termux run finishes.

---

### Linux

```bash
# Install Node.js (example for Debian/Ubuntu)
sudo apt-get update
sudo apt-get install -y nodejs npm

# LTS only
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** Linux x64/arm64, Node.js >=18 (recommended v22+), ~80MB storage

---

### macOS

```bash
# LTS only
npm install -g @mmmbuto/codex-cli-lts

# Verify
codex --version
codex login
```

**Requirements:** macOS x64/arm64, Node.js >=18 (recommended v22+), ~100MB storage

---

## 📚 Documentation

- [Installation Details](./docs/installation.md)
- [Testing](./docs/testing.md)
- [Building from Source](./BUILDING.md)
- [Test Reports](./test-reports/)
- [Latest Changelog](./CHANGELOG.md)
- [LTS Changelog](https://github.com/DioNanos/codex-termux/blob/lts/CHANGELOG_LTS.md)
- [Full Documentation](./docs/)

---

## Maintenance

Community-maintained port enabling AI-powered coding on Android Termux. Activities include ARM64 compilation, upstream synchronization, Termux compatibility patches, and documentation.

---

## License

This project maintains full compliance with Apache 2.0 license from OpenAI Codex.

**Original work**: Copyright OpenAI (https://github.com/openai/codex)
**Termux port**: Minimal patches for Android compatibility

See [LICENSE](./LICENSE) file for details.

---
