#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT_DIR"

echo "🔍 VERIFYING TERMUX PATCH SET..."
echo ""

pass() { echo "✅ PRESENT"; }
fail() { echo "❌ MISSING!"; exit 1; }

# Patch #1
printf "Patch #1 (Browser Login): "
if grep -q "target_os.*android" codex-rs/login/src/server.rs \
  && grep -q "termux-open-url" codex-rs/login/src/server.rs; then
  pass
else
  fail
fi

# Patch #2
printf "Patch #2 (RAM Optimization): "
if grep -q "lto = false" codex-rs/Cargo.toml; then
  pass
else
  fail
fi

# Patch #4
printf "Patch #4 (Auto-Update URL): "
if grep -q "DioNanos/codex-termux" codex-rs/tui/src/updates.rs; then
  pass
else
  fail
fi

# Patch #5
printf "Patch #5 (Version Parser): "
if grep -q "split('-')" codex-rs/tui/src/updates.rs; then
  pass
else
  fail
fi

# Patch #6
printf "Patch #6 (NPM Package): "
if grep -q "@mmmbuto/codex-cli-termux" codex-rs/tui/src/update_action.rs; then
  pass
else
  fail
fi

# Patch #9
printf "Patch #9 (Auto-Update Exec): "
if grep -q "update_action = exit_info.update_action" codex-rs/cli/src/main.rs \
  && grep -q "run_update_action" codex-rs/cli/src/main.rs; then
  pass
else
  fail
fi

# Patch #10
printf "Patch #10 (Launcher + libc++ fallback): "
if grep -q 'exec "\$SCRIPT_DIR/codex.bin"' npm-package/bin/codex \
  && grep -q 'exec "\$SCRIPT_DIR/codex-exec.bin"' npm-package/bin/codex-exec \
  && grep -q 'LD_LIBRARY_PATH' npm-package/bin/codex \
  && grep -q 'LD_LIBRARY_PATH' npm-package/bin/codex-exec \
  && grep -q '"bin/codex.bin"' npm-package/package.json \
  && grep -q '"bin/codex-exec.bin"' npm-package/package.json; then
  if [ -x npm-package/bin/codex.bin ] && [ -x npm-package/bin/codex-exec.bin ]; then
    pass
  else
    echo "✅ PRESENT (launchers + package wiring ok; binaries supplied at packaging/release time)"
  fi
else
  fail
fi

# Patch #11
printf "Patch #11 (Android no-voice policy): "
if grep -q "\[target.'cfg(target_os = \"android\")'.dependencies\]" codex-rs/cli/Cargo.toml \
  && grep -q 'codex-tui = { path = "../tui", default-features = false }' codex-rs/cli/Cargo.toml \
  && grep -q "\[target.'cfg(target_os = \"android\")'.dependencies\]" codex-rs/cloud-tasks/Cargo.toml \
  && grep -q 'codex-tui = { path = "../tui", default-features = false }' codex-rs/cloud-tasks/Cargo.toml; then
  pass
else
  fail
fi

# Patch #12
printf "Patch #12 (Dynamic subcommand routing): "
if grep -q "spawnSync(binaryPath" npm-package/bin/codex.js \
  && grep -q "detectSubcommands" npm-package/bin/codex.js \
  && grep -q "aliasesMatch" npm-package/bin/codex.js; then
  pass
else
  fail
fi

# Patch #13
printf "Patch #13 (Android network-proxy stub): "
if grep -q "pub use android_stub::\\*" codex-rs/network-proxy/src/lib.rs \
  && grep -q "NetworkDomainPermissions" codex-rs/network-proxy/src/android_stub.rs \
  && grep -q "NetworkUnixSocketPermissions" codex-rs/network-proxy/src/android_stub.rs \
  && grep -q "pub fn allowed_domains" codex-rs/network-proxy/src/android_stub.rs; then
  pass
else
  fail
fi

# Bazel/Toolchain patch set declared in MODULE.bazel
printf "Bazel declared patch files: "
DECLARED_PATCHES=$(grep -o "//patches:[^\" ]*\\.patch" MODULE.bazel | sed 's#//patches:##' | sort -u || true)
if [ -z "$DECLARED_PATCHES" ]; then
  fail
fi

MISSING_DECLARED=0
for patch in $DECLARED_PATCHES; do
  if [ ! -f "patches/$patch" ]; then
    echo ""
    echo "  ❌ Declared but missing: patches/$patch"
    MISSING_DECLARED=1
  fi
done
if [ "$MISSING_DECLARED" -ne 0 ]; then
  exit 1
fi
pass

# Informational: patch files not currently declared in MODULE.bazel
UNDECLARED=$(comm -23 \
  <(find patches -maxdepth 1 -type f -name "*.patch" -printf "%f\n" | sort) \
  <(printf "%s\n" "$DECLARED_PATCHES" | sort) || true)
if [ -n "$UNDECLARED" ]; then
  echo ""
  echo "ℹ️  Patch files present but not declared in MODULE.bazel:"
  printf "  - %s\n" $UNDECLARED
fi

echo ""
echo "🎉 ALL CRITICAL PATCHES VERIFIED!"
