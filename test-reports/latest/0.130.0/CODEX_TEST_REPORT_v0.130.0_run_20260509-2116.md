# CODEX TEST REPORT

- Date: 2026-05-09 21:16 CEST
- Device: Pixel9Pro Termux
- Repo: <termux-checkout>
- Source commit under test: `ed00bc8c8c` (`release: prepare codex termux 0.130.0`)
- Global package under test: @mmmbuto/codex-cli-termux 0.130.0
- Suite type: runtime-only validation of the installed Termux package
- Suite reference: test-reports/latest/CLI_RUNTIME_REPORT.md

## Runtime Checks

- `PASS` Package installed check
- `PASS` codex --version
- `PASS` codex-exec --version
- `PASS` codex --help
- `PASS` codex exec --help
- `PASS` codex review --help
- `PASS` codex login --help
- `PASS` codex logout --help
- `PASS` codex resume --help
- `PASS` codex fork --help
- `PASS` codex mcp --help
- `PASS` codex sandbox --help
- `PASS` codex app-server --help
- `PASS` codex login status
- `PASS` codex mcp list
- `PASS` codex features list
- `PASS` codex completion bash
- `PASS` codex debug prompt-input --help
- `PASS` codex app-server generate-json-schema --help
- `PASS` codex app-server generate-json-schema --out
- `PASS` node wrapper fork --help
- `PASS` node wrapper debug --help
- `PASS` node wrapper review --help
- `PASS` node wrapper exec --help
- `PASS` node wrapper login --help
- `PASS` node wrapper logout --help
- `PASS` node wrapper resume --help
- `PASS` codex-exec workspace list files
- `PASS` codex-exec create/read hello.txt
- `PASS` codex-exec network smoke
- `PASS` command path info
- `PASS` runtime wrapper files
- `PASS` codex.bin/codex-exec.bin runpath
- `PASS` codex.bin/codex-exec.bin needed libs
- `PASS` Termux runtime
- `PASS` node version
- `PASS` npm version
- `PASS` termux-open-url presence
- `PASS` verify-patches
- `PASS` temporary artifact cleanup

## Notes

- No recompilation was performed.
- This validation targets the installed package, not source-tree build parity.
- Installed package path resolved under `<termux-prefix>/lib/node_modules/@mmmbuto/codex-cli-termux/`.
- The runtime package version is `0.130.0` without a `-termux` suffix.

## Result

PASS
