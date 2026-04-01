# 🔨 Building Codex CLI (Termux fork)

This repository packages the official OpenAI Codex CLI for Android Termux (ARM64) with a small set of compatibility patches. Most users should install the precompiled npm package:

```bash
npm install -g @mmmbuto/codex-cli-termux@latest
```

If you want or need to build the binary yourself, follow the steps below.

---

## 1. Prerequisites (Termux)

On a Termux environment with ARM64:

```bash
pkg update && pkg upgrade -y
pkg install git clang lld rust pkg-config openssl openssl-tool -y
```

You will also need Node.js if you plan to build and test the npm package:

```bash
pkg install nodejs-lts -y
```

---

## 2. Clone this fork

```bash
git clone https://github.com/DioNanos/codex-termux.git
cd codex-termux
```

The Rust workspace lives in `codex-rs/` and the npm wrapper in `npm-package/`.

---

## 3. Build the Rust binary

From the workspace root on Termux:

```bash
cd codex-rs
cargo build --release -p codex-cli -p codex-exec
```

From a Linux maintainer host with Android NDK installed:

```bash
export TOOLCHAIN_BIN="$HOME/.rustup/toolchains/1.93.0-x86_64-unknown-linux-gnu/bin"
export ANDROID_NDK_HOME="$HOME/android-sdk/android-ndk-r26d"
export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
export PATH="$TOOLCHAIN_BIN:$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
export CARGO="$TOOLCHAIN_BIN/cargo"
export RUSTC="$TOOLCHAIN_BIN/rustc"

cd codex-rs
cargo build --target aarch64-linux-android --release -p codex-cli -p codex-exec
```

If the pinned `v8` crate needs fork-owned Android artifacts, fetch them first:

```bash
python3 scripts/fetch_rusty_v8_android.py
eval "$(python3 scripts/fetch_rusty_v8_android.py | rg '^export ')"
```

This resolves the exact `v8` crate version from `codex-rs/Cargo.lock`, downloads
the matching Android `archive + binding` pair from the fork release tag
`rusty-v8-v<crate_version>`, and exports `RUSTY_V8_ARCHIVE` plus
`RUSTY_V8_SRC_BINDING_PATH` for the Cargo build.

If the Android pair does not exist yet and you need to produce it from source,
do not build from the crates.io archive alone. The `v8` crate source tree is not
enough for Android. Start from a full `denoland/rusty_v8` checkout at the exact
tag and initialize the Chromium-facing submodules needed by the build, including
at least `build`, `v8`, `tools/clang`, and `buildtools`. Use a recent `gn`
binary; older system `gn` builds can fail on Chromium files with errors such as
`Unknown function: path_exists`.

To bootstrap that checkout reproducibly:

```bash
python3 scripts/prepare_rusty_v8_android_source.py
```

That helper now initializes the required submodules recursively and bootstraps
the Chromium-facing symlink `third_party/android_toolchain/ndk ->
../android_ndk`, which the Android GN files expect once the NDK payload has
been downloaded into `third_party/android_ndk`.

Before the first Android source-build in that checkout, install the host sysroot:

```bash
cd .artifacts/rusty_v8-src/v146.4.0
python3 build/linux/sysroot_scripts/install-sysroot.py --arch=amd64
```

For the published Termux package, Android consumers build `codex-tui` with
default features disabled. This intentionally excludes voice/realtime audio
from the npm release so the packaged binaries do not depend on Android audio
linker paths such as `libOpenSLES.so`.

Termux-specific optimizations are already baked into `codex-rs/Cargo.toml`:

- `lto = false`
- `codegen-units = 16`

These settings are tuned so that the build can complete on RAM‑constrained devices while keeping good runtime performance.

The resulting binaries will be:

```bash
codex-rs/target/release/codex
codex-rs/target/release/codex-exec
```

Or, for the Linux maintainer-host Android cross-build:

```bash
codex-rs/target/aarch64-linux-android/release/codex
codex-rs/target/aarch64-linux-android/release/codex-exec
```

You can run it directly:

```bash
./target/release/codex --version
./target/release/codex-exec --help
```

---

## 4. Use the binary with the npm wrapper (optional)

If you want to test the same layout used by the published npm package:

```bash
cd ../npm-package
cp ../codex-rs/target/release/codex bin/codex.bin
cp ../codex-rs/target/release/codex-exec bin/codex-exec.bin
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" bin/libc++_shared.so
chmod +x bin/codex bin/codex-exec bin/codex.bin bin/codex-exec.bin bin/libc++_shared.so
```

For the maintainer-host Android cross-build, copy from the Android target directory instead:

```bash
cd ../npm-package
cp ../codex-rs/target/aarch64-linux-android/release/codex bin/codex.bin
cp ../codex-rs/target/aarch64-linux-android/release/codex-exec bin/codex-exec.bin
cp "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so" bin/libc++_shared.so
chmod +x bin/codex bin/codex-exec bin/codex.bin bin/codex-exec.bin bin/libc++_shared.so
```

After this, from inside `npm-package/` you can run:

```bash
node bin/codex.js --version
```

This uses the Node.js launchers (`bin/codex.js` / `bin/codex-exec.js`) together with your locally built binaries.

For the npm layout, keep the wrapped launchers in front of the raw ELF binaries.
The launchers export `CODEX_SELF_EXE` so helper tools such as `apply_patch`
re-exec through `bin/codex` instead of `bin/codex.bin`, preserving access to the
bundled `libc++_shared.so` on Termux.

---

## 5. Maintainer notes (release workflow)

For maintainers who publish `@mmmbuto/codex-cli-termux`:

1. **Sync with upstream** in your local clone (fetch and merge the relevant `rust-v*` tag from `openai/codex` into this fork).
2. **Update versions**:
   - `codex-rs/Cargo.toml` → `[workspace.package] version`
   - `npm-package/package.json` → `"version": "<same>-termux"`
3. **Build the Termux binary** as in section 3 and keep the Android `no-voice`
   dependency policy in place for published npm artifacts. If the pinned `v8`
   crate is not covered by upstream Android assets, fetch the fork-owned pair
   first with `python3 scripts/fetch_rusty_v8_android.py`.
4. **Copy the binaries and bundled libc++ runtime into the npm wrapper** as in section 4.
   For the `0.118.0-termux` line and later, the preferred fix is to build
   with the Android API 29 linker so the emitted `PT_TLS` layout is already
   valid for ARM64 Bionic.
5. **Run release gates** from repo root:

   ```bash
   bash verify-patches.sh
   cd npm-package
   npm pack
   ```

   Smoke-test the generated tarball before publish:

   ```bash
   env -u LD_LIBRARY_PATH ./bin/codex --version
   env -u LD_LIBRARY_PATH ./bin/codex-exec --version
   node ./bin/codex.js fork --help
   node ./bin/codex.js debug --help
   ```

   Include one helper regression smoke check before publish:
   open an interactive Codex session from the packaged launcher and confirm
   `apply_patch` no longer fails with `libc++_shared.so not found` when invoked
   through the normal tool flow on Termux.

   The native Termux suite remains a post-release manual validation step. Do not
   mark the release as fully native-validated until that on-device pass completes.

6. **Publish** from `npm-package/` (for authorized maintainers only):

   ```bash
   npm publish
   ```

This matches the automated pipeline used in the private build scripts, while keeping all steps reproducible from this public repository.
