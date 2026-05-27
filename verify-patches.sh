#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT_DIR"

pass() { echo "✅ PRESENT"; }
fail() { echo "❌ MISSING!"; exit 1; }

READELF_BIN="${READELF_BIN:-$(command -v llvm-readelf || command -v readelf || true)}"

printf "Patch #1 (Browser Login): "
if grep -q "termux-open-url" codex-rs/login/src/server.rs; then
  pass
else
  fail
fi

printf "Patch #2 (Release Profile): "
if grep -q 'lto = "thin"' codex-rs/Cargo.toml && grep -q "codegen-units = 16" codex-rs/Cargo.toml; then
  pass
else
  fail
fi

printf "Patch #4/#5 (Fork Update Channel + Termux Tag Parser): "
if grep -q "DioNanos/codex-termux" codex-rs/tui/src/updates.rs \
  && grep -q "split('-')" codex-rs/tui/src/update_versions.rs \
  && grep -q "strip_prefix(\"rust-v\")" codex-rs/tui/src/update_versions.rs \
  && grep -q "strip_prefix('v')" codex-rs/tui/src/update_versions.rs; then
  pass
else
  fail
fi

printf "Patch #6 (Termux npm Package Name): "
if grep -q "@mmmbuto/codex-cli-termux@latest" codex-rs/tui/src/update_action.rs; then
  pass
else
  fail
fi

printf "Patch #10 (Launcher Hardening): "
if grep -q 'exec "\$SCRIPT_DIR/codex.bin"' npm-package/bin/codex \
  && grep -q 'exec "\$SCRIPT_DIR/codex-exec.bin"' npm-package/bin/codex-exec \
  && grep -q 'CODEX_SELF_EXE' npm-package/bin/codex.js \
  && grep -q '"bin/codex.bin"' npm-package/package.json; then
  pass
else
  fail
fi

printf "Patch #10b (Android ELF Runpath): "
if grep -q 'link-arg=-Wl,-rpath,$ORIGIN' codex-rs/.cargo/config.toml; then
  if [ -x npm-package/bin/codex.bin ] && [ -x npm-package/bin/codex-exec.bin ] && [ -n "$READELF_BIN" ]; then
    if "$READELF_BIN" -d npm-package/bin/codex.bin | grep -Eq '(RUNPATH|RPATH).*\$ORIGIN' \
      && "$READELF_BIN" -d npm-package/bin/codex-exec.bin | grep -Eq '(RUNPATH|RPATH).*\$ORIGIN'; then
      pass
    else
      fail
    fi
  else
    echo "✅ PRESENT (source configured; binary validation deferred until packaged Android ELFs are staged)"
  fi
else
  fail
fi

printf "Patch #11 (Android No-Voice Policy): "
if grep -q 'target_os = "android"' codex-rs/tui/src/lib.rs \
  && grep -q 'voice input is unavailable in this build' codex-rs/tui/src/lib.rs \
  && grep -Fq '#[cfg(not(any(target_os = "linux", target_os = "android")))]' codex-rs/tui/src/lib.rs \
  && ! grep -Fq "[features]" codex-rs/tui/Cargo.toml \
  && ! grep -Fq 'voice-input = []' codex-rs/tui/Cargo.toml \
  && grep -Fq "[target.'cfg(all(not(target_os = \"linux\"), not(target_os = \"android\")))'.dependencies]" codex-rs/tui/Cargo.toml \
  && grep -Fq 'cpal = "0.15"' codex-rs/tui/Cargo.toml; then
  pass
else
  fail
fi

printf "Patch #12 (Dynamic Subcommand Routing): "
if grep -q 'detectSubcommands' npm-package/bin/codex.js \
  && grep -q 'spawnSync(binaryPath' npm-package/bin/codex.js; then
  pass
else
  fail
fi

printf "Patch #13 (Fork-safe Managed Updates): "
if grep -q "@mmmbuto/codex-cli-termux@latest" codex-rs/tui/src/update_action.rs \
  && grep -q "@mmmbuto/codex-cli-termux@latest" codex-rs/app-server-daemon/src/lib.rs \
  && grep -q "@mmmbuto/codex-cli-termux@latest" codex-rs/app-server-daemon/README.md \
  && grep -q "auto_update_enabled: false" codex-rs/app-server-daemon/src/lib.rs \
  && ! grep -R -q "chatgpt.com/codex/install" codex-rs/tui/src/update_action.rs codex-rs/app-server-daemon; then
  pass
else
  fail
fi

printf "Patch #14 (Fork-owned Public Install Surfaces): "
if grep -q "DioNanos/codex-termux" scripts/install/install.sh \
  && grep -q "DioNanos/codex-termux" scripts/install/install.ps1 \
  && grep -q "DioNanos/codex-termux" scripts/stage_npm_packages.py \
  && grep -q "@mmmbuto/codex-cli-termux" codex-rs/README.md \
  && ! grep -R -q "github.com/openai/codex/releases\\|api.github.com/repos/openai/codex\\|@openai/codex" scripts/install scripts/stage_npm_packages.py codex-rs/README.md; then
  pass
else
  fail
fi

printf "Patch #15 (Fork-owned Feedback Surfaces): "
if grep -q "DioNanos/codex-termux/issues/new" codex-rs/tui/src/bottom_pane/feedback_view.rs \
  && grep -q "DioNanos/codex-termux/main/announcement_tip.toml" codex-rs/tui/src/tooltips.rs \
  && grep -q "@mmmbuto/codex-cli-termux" .github/ISSUE_TEMPLATE/3-cli.yml \
  && grep -q "DioNanos/codex-termux/discussions" .github/ISSUE_TEMPLATE/4-bug-report.yml \
  && grep -q "DioNanos/codex-termux/blob/main/docs/contributing.md" .github/ISSUE_TEMPLATE/5-feature-request.yml \
  && grep -q "DioNanos/codex-termux/blob/main/docs/contributing.md" .github/pull_request_template.md \
  && ! grep -R -q "github.com/openai/codex/issues\\|github.com/openai/codex/discussions\\|npmjs.com/package/@openai/codex\\|raw.githubusercontent.com/openai/codex/main/announcement_tip.toml" .github/ISSUE_TEMPLATE .github/pull_request_template.md codex-rs/tui/src/bottom_pane/feedback_view.rs codex-rs/tui/src/tooltips.rs; then
  pass
else
  fail
fi

printf "Patch #16 (Android Remote-Control Daemon): "
if grep -q 'CODEX_SELF_EXE' codex-rs/app-server-daemon/src/managed_install.rs \
  && grep -q 'target_os = "android"' codex-rs/app-server-daemon/src/managed_install.rs \
  && grep -q '/proc/' codex-rs/app-server-daemon/src/backend/pid.rs \
  && grep -q 'target_os = "android"' codex-rs/app-server-daemon/src/backend/pid.rs \
  && grep -q 'temp_dir()' codex-rs/cli/src/remote_control_cmd.rs \
  && ! grep -q 'tempdir_in("/tmp")' codex-rs/cli/src/remote_control_cmd.rs; then
  pass
else
  fail
fi

printf "Patch #6b (Fork Identity Across UI/Doctor/NPM Surfaces): "
if grep -q "DioNanos/codex-termux/releases/latest" codex-rs/cli/src/doctor/updates.rs \
  && grep -q "@mmmbuto/codex-cli-termux" codex-rs/cli/src/doctor/updates.rs \
  && grep -q "@mmmbuto/codex-cli-termux" codex-rs/cli/src/doctor.rs \
  && grep -q "@mmmbuto%2fcodex-cli-termux" codex-rs/tui/src/npm_registry.rs \
  && grep -q "DioNanos/codex-termux" codex-rs/tui/src/update_prompt.rs \
  && grep -q "DioNanos/codex-termux" codex-rs/tui/src/history_cell/notices.rs \
  && ! grep -R -q "api.github.com/repos/openai/codex\|@openai%2fcodex\|@openai/codex" \
      codex-rs/cli/src/doctor/updates.rs \
      codex-rs/cli/src/doctor.rs \
      codex-rs/tui/src/npm_registry.rs \
      codex-rs/tui/src/update_prompt.rs \
      codex-rs/tui/src/history_cell/notices.rs; then
  pass
else
  fail
fi

printf "Patch #17 (flock ENOTSUP/EOPNOTSUPP Tolerance): "
if grep -q 'raw_os_error().*libc::ENOTSUP' codex-rs/app-server-daemon/src/backend/pid.rs \
  && grep -q 'raw_os_error().*libc::EOPNOTSUPP' codex-rs/app-server-daemon/src/backend/pid.rs \
  && grep -q 'raw_os_error().*libc::ENOTSUP' codex-rs/app-server-daemon/src/lib.rs \
  && grep -q 'raw_os_error().*libc::EOPNOTSUPP' codex-rs/app-server-daemon/src/lib.rs; then
  pass
else
  fail
fi

printf "Patch #18 (Android Runtime Compat Shims): "
if grep -q 'CODEX_SELF_EXE' codex-rs/arg0/src/lib.rs \
  && grep -q 'resolve_codex_self_exe' codex-rs/arg0/src/lib.rs \
  && grep -q 'is_unsupported_file_lock_error' codex-rs/core/src/installation_id.rs \
  && grep -q 'pub unsafe extern "C" fn openpty' codex-rs/utils/pty/src/pty.rs \
  && grep -q 'target_os = "android"' codex-rs/utils/pty/src/pty.rs; then
  pass
else
  fail
fi

printf "Patch #19 (Android UI cfg Gates): "
if grep -q 'cfg(not(target_os = "android"))' codex-rs/tui/src/clipboard_paste.rs \
  && grep -q 'target_os = "android"' codex-rs/tui/src/app_event.rs; then
  pass
else
  fail
fi

printf "Patch #20 (Android Code-Mode Stubs): "
if [ -f codex-rs/code-mode/src/runtime_stub.rs ] \
  && [ -f codex-rs/code-mode/src/service_stub.rs ] \
  && grep -q 'cfg(target_os = "android")' codex-rs/code-mode/src/lib.rs \
  && grep -q 'mod runtime_stub' codex-rs/code-mode/src/lib.rs \
  && grep -q 'mod service_stub' codex-rs/code-mode/src/lib.rs \
  && grep -q 'cfg(not(target_os = "android"))' codex-rs/code-mode/Cargo.toml; then
  pass
else
  fail
fi

printf "Patch #21 (Android Vendored OpenSSL): "
if grep -q 'aarch64-linux-android' codex-rs/core/Cargo.toml \
  && grep -q '"vendored"' codex-rs/core/Cargo.toml; then
  pass
else
  fail
fi

printf "Patch #22 (V8 Android Prebuilt Infrastructure): "
if [ -f scripts/fetch_rusty_v8_android.py ] \
  && [ -f scripts/prepare_rusty_v8_android_source.py ] \
  && [ -f third_party/v8/android-artifacts.toml ] \
  && grep -q 'aarch64-linux-android' third_party/v8/android-artifacts.toml \
  && grep -q 'RUSTY_V8_ARCHIVE' scripts/fetch_rusty_v8_android.py; then
  pass
else
  fail
fi

printf "Patch #23 (Fork-Owned Workflows + CI Guards): "
if grep -q "github.repository == 'openai/codex'" .github/workflows/ci.yml \
  && [ -f .github/workflows/termux-npm-build-publish.yml ] \
  && [ -f .forgejo/workflows/termux-next-smoke.yml ]; then
  pass
else
  fail
fi

printf "Bazel patch inventory present: "
if [ -f patches/windows-link.patch ] && [ -f patches/aws-lc-sys_memcmp_check.patch ]; then
  pass
else
  fail
fi
