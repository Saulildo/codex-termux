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
# Both launchers must exec the BUNDLED binary via the absolute "$SCRIPT_DIR"
# path, never a bare name resolved through PATH. The standalone codex-exec.bin
# was dropped, so the codex-exec wrapper now dispatches codex.bin with the exec
# subcommand; the hardening property is preserved, just re-pointed.
if grep -q 'exec "\$SCRIPT_DIR/codex.bin"' npm-package/bin/codex \
  && grep -q 'exec "\$SCRIPT_DIR/codex.bin" exec' npm-package/bin/codex-exec \
  && grep -q 'CODEX_SELF_EXE' npm-package/bin/codex.js \
  && grep -q '"bin/codex.bin"' npm-package/package.json; then
  pass
else
  fail
fi

printf "Patch #10b (Android ELF Runpath): "
if grep -q 'link-arg=-Wl,-rpath,$ORIGIN' codex-rs/.cargo/config.toml; then
  if [ -x npm-package/bin/codex.bin ] && [ -n "$READELF_BIN" ]; then
    if "$READELF_BIN" -d npm-package/bin/codex.bin | grep -Eq '(RUNPATH|RPATH).*\$ORIGIN'; then
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

# Patch #11 (Android Realtime Audio + oboe-shared-stdcxx) RETIRED at
# rust-v0.140.0-alpha.18: upstream removed the entire TUI realtime voice feature
# (openai/codex#27801), so the fork's cpal/oboe Android enablement toggle had
# nothing left to gate and was dropped with it. The feature was never usable from
# the Termux CLI anyway (needs an Android JavaVM/Activity). Termux-native audio is
# tracked on the Codex VL roadmap.

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
  && grep -q 'ErrorKind::Unsupported' codex-rs/core/src/installation_id.rs \
  && grep -q 'pub unsafe extern "C" fn openpty' codex-rs/utils/pty/src/pty.rs \
  && grep -q 'target_os = "android"' codex-rs/utils/pty/src/pty.rs; then
  pass
else
  fail
fi

printf "Patch #19 (Android UI cfg Gates): "
# The app_event.rs android cfg gate was attached to the realtime audio event
# retired with upstream openai/codex#27801 (see Patch #11 note); the clipboard
# paste android gate is the remaining fork UI cfg surface.
if grep -q 'cfg(not(target_os = "android"))' codex-rs/tui/src/clipboard_paste.rs; then
  pass
else
  fail
fi

printf "Patch #20 (Android Code-Mode real, upstream-aligned): "
# 0.136.0: the Android code-mode stub was reverted. code-mode now uses the real
# in-process V8 runtime on Android too (stub files removed, v8 enabled for all
# targets), relying on the fork-owned aarch64-linux-android rusty_v8 prebuild.
if [ ! -f codex-rs/code-mode/src/runtime_stub.rs ] \
  && [ ! -f codex-rs/code-mode/src/service_stub.rs ] \
  && ! grep -q 'mod runtime_stub' codex-rs/code-mode/src/lib.rs \
  && ! grep -q 'mod service_stub' codex-rs/code-mode/src/lib.rs \
  && ! grep -q 'cfg(not(target_os = "android"))' codex-rs/code-mode/Cargo.toml \
  && grep -q 'v8 = { workspace = true }' codex-rs/code-mode/Cargo.toml; then
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
if grep -q "github.repository == 'openai/codex'" .github/workflows/repo-checks.yml \
  && [ -f .github/workflows/termux-npm-build-publish.yml ] \
  && [ ! -f .forgejo/workflows/termux-next-smoke.yml ]; then
  pass
else
  fail
fi

printf "Patch #24 (Termux TLS Roots, no rustls-platform-verifier panic): "
# Since upstream rust-v0.143.0-alpha.x the OAuth and streamable-HTTP MCP paths no
# longer build their own reqwest-0.13 client in perform_oauth_login.rs/rmcp_client.rs;
# they route all HTTP through the injected reqwest-0.12 codex_exec_server::ReqwestHttpClient
# (OAuthHttpClientAdapter / StreamableHttpClientAdapter), which does not construct
# rustls-platform-verifier and therefore cannot hit the Android panic. The only remaining
# reqwest-0.13 ClientBuilder is auth_status.rs, which keeps apply_termux_tls. Protection is
# intact; we verify it where a 0.13 client is actually built (utils.rs + auth_status.rs).
if grep -q "apply_termux_tls" codex-rs/rmcp-client/src/utils.rs \
  && grep -q "tls_certs_only" codex-rs/rmcp-client/src/utils.rs \
  && grep -q "webpki-root-certs" codex-rs/rmcp-client/Cargo.toml \
  && grep -q "apply_termux_tls" codex-rs/rmcp-client/src/auth_status.rs; then
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
