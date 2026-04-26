# Building Codex CLI (Termux fork)

This repository packages upstream OpenAI Codex for Android Termux (ARM64) with
only the compatibility delta required for packaging and runtime.

Most users should install the published package:

```bash
npm install -g @mmmbuto/codex-cli-termux@latest
```

## Native Termux build

On an ARM64 Termux device:

```bash
pkg update && pkg upgrade -y
pkg install git clang lld rust pkg-config openssl openssl-tool nodejs-lts -y

git clone https://github.com/DioNanos/codex-termux.git
cd codex-termux/codex-rs
cargo build --release -p codex-cli -p codex-exec
```

Expected outputs:

```bash
codex-rs/target/release/codex
codex-rs/target/release/codex-exec
```

## Linux-host Android cross-build

On a Linux maintainer host with Android NDK installed:

```bash
export ANDROID_NDK_HOME="$HOME/android-ndk/android-ndk-r27c"
export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
export LIBLZMA_NO_PKG_CONFIG=1
export PATH="$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"

python3 scripts/fetch_rusty_v8_android.py
eval "$(python3 scripts/fetch_rusty_v8_android.py | grep '^export ' | sed 's/^export //')"

cd codex-rs
rustup run 1.93.0 cargo build --target aarch64-linux-android --release -p codex-cli -p codex-exec
```

Expected outputs:

```bash
codex-rs/target/aarch64-linux-android/release/codex
codex-rs/target/aarch64-linux-android/release/codex-exec
```

Notes:

- The canonical maintainer toolchain is Rust `1.93.0`.
- The canonical CI/release NDK is `r27c`.
- A local host can use a newer NDK for exploratory builds, but release parity is checked against the GitHub Actions workflow.

## Package layout smoke test

To test the npm wrapper layout locally after a successful cross-build:

```bash
cd npm-package
cp ../codex-rs/target/aarch64-linux-android/release/codex bin/codex.bin
cp ../codex-rs/target/aarch64-linux-android/release/codex-exec bin/codex-exec.bin
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" bin/libc++_shared.so
chmod +x bin/codex bin/codex-exec bin/codex.bin bin/codex-exec.bin bin/libc++_shared.so
node ./bin/codex.js --version
node ./bin/codex.js --help >/dev/null
```

## Maintainer release notes

- Workspace version lives in `codex-rs/Cargo.toml`.
- Published npm version lives in `npm-package/package.json`.
- Termux patch verification lives in `verify-patches.sh`.
- The maintainer GitHub Actions workflow is `.github/workflows/termux-npm-build-publish.yml`.
- Fork-owned Android `rusty_v8` assets are described in `third_party/v8/android-artifacts.toml`.
- Recommended publish order: push the integration commit to `develop`, run once with `source_ref=develop` and `create_release=true`, validate the draft release artifact, then run again with `source_ref=develop`, `publish_npm=true`, and the desired npm dist-tag.

If the Android `rusty_v8` pair for the resolved crate version does not exist
yet, bootstrap a source checkout with:

```bash
python3 scripts/prepare_rusty_v8_android_source.py
```
