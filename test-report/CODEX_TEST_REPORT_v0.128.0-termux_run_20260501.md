# CODEX TEST REPORT v0.128.0-termux

- Date: 2026-05-01
- Device: Termux Android device
- Repo: `/data/data/com.termux/files/home/Dev/00_termux-stack/codex-termux`
- Commit under test: `570377973f` (`develop`)
- Global package under test: `codex-cli 0.128.0-termux`
- Suite type: runtime-only validation of the installed Termux package
- Suite reference: `test-reports/latest/CLI_RUNTIME_REPORT.md`

## Runtime Checks

- `PASS` package install check
- `PASS` `codex --version`
- `PASS` `codex-exec --version`
- `PASS` `codex --help`
- `PASS` `codex exec --help`
- `PASS` `codex review --help`
- `PASS` `codex login --help`
- `PASS` `codex logout --help`
- `PASS` `codex resume --help`
- `PASS` `codex fork --help`
- `PASS` `codex mcp --help`
- `PASS` `codex sandbox --help`
- `PASS` `codex app-server --help`
- `PASS` `codex completion bash`
- `PASS` `codex login status`
- `PASS` `codex mcp list`
- `PASS` `codex features list`
- `PASS` `codex debug prompt-input --help`
- `PASS` `codex app-server generate-json-schema --help`
- `PASS` `codex app-server generate-json-schema --out <dir>`
- `PASS` node wrapper `fork --help`
- `PASS` node wrapper `debug --help`
- `PASS` node wrapper `review --help`
- `PASS` node wrapper `exec --help`
- `PASS` node wrapper `login --help`
- `PASS` node wrapper `logout --help`
- `PASS` node wrapper `resume --help`
- `PASS` `codex exec --ephemeral "Reply with exactly: OK"`
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "print current directory and list files"`
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "create hello.txt with content 'hello' and then read it back"`
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "run one network check with curl -I https://www.google.com and report the first HTTP status line only"`
- `PASS` runtime wrapper files
- `PASS` `codex.bin/codex-exec.bin` runpath
- `PASS` `codex.bin/codex-exec.bin` needed libs
- `PASS` Termux runtime
- `FAIL` `verify-patches.sh`

## Failure Detail

`verify-patches.sh` stops at:

- `Patch #4/#5 (Fork Update Channel + Version Parser): ❌ MISSING!`

## Result

`FAIL`

The installed `codex-cli 0.128.0-termux` package passes the runtime command surface, wrapper routing, smoke execution, and binary linkage checks, but the Termux-specific patch inventory is incomplete because `verify-patches.sh` fails on patch #4/#5.
