# Codex Termux

Latest Android Termux line built from upstream OpenAI Codex `rust-v0.119.0`.

## Packages

- Latest Termux line: `@mmmbuto/codex-cli-termux@0.119.0-termux`
- Separate LTS line: `@mmmbuto/codex-cli-lts`

The latest line tracks upstream closely and carries only the compatibility delta
required to keep Codex usable on Android Termux.

## Install on Termux

```bash
pkg update && pkg upgrade -y
pkg install nodejs-lts -y
npm install -g @mmmbuto/codex-cli-termux@latest
codex --version
codex login
```

## Fork-specific notes

- Android Termux ARM64 only
- Built from upstream `rust-v0.119.0`
- Voice and realtime audio remain disabled in the published Termux package
- Launchers preserve bundled `libc++_shared.so` visibility for helper re-exec
- Android ELFs are hardened with `RUNPATH=$ORIGIN`
- Update checks and release links point to `DioNanos/codex-termux`

## References

- Upstream release: https://github.com/openai/codex/releases/tag/rust-v0.119.0
- Fork releases: https://github.com/DioNanos/codex-termux/releases
- Patch inventory: [patches/README.md](./patches/README.md)
- Changelog: [CHANGELOG.md](./CHANGELOG.md)
- Build notes: [BUILDING.md](./BUILDING.md)
