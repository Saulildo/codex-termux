# CODEX TEST REPORT v0.128.0-termux

- Date: 2026-05-01
- Device: Termux Android device
- Repo: Termux checkout of `DioNanos/codex-termux`
- Initial commit under test: `570377973f` (`develop`)
- Global package under test: `codex-cli 0.128.0-termux`
- Suite type: runtime-only validation of the installed Termux package
- Suite reference: runtime checks from the installed Termux package

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

## Initial Inventory Result

`FAIL`

The installed `codex-cli 0.128.0-termux` package passed the runtime command surface, wrapper routing, smoke execution, and binary linkage checks. The first patch inventory check was collected from an older checkout and reported patch #4/#5 as missing.

## Forge Follow-up

- Date: 2026-05-01
- Forge `develop` HEAD checked by maintainer: `87d827b83c`
- Command: `bash verify-patches.sh`
- Result: `PASS`
- Note: the failure above was collected on older commit `570377973f`; Forge `develop` contains the update-channel and version-parser patches.

## Final Release Decision

`PASS`

The runtime checks passed on device, and the patch inventory was re-run successfully on the Forge `develop` head used for release preparation.
