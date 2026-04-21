# CODEX TEST REPORT v0.122.2-termux

- Date: 2026-04-21 02:57 CEST
- Device: Pixel9Pro Termux
- Repo: `/data/data/com.termux/files/home/Dev/05_termux/codex-termux`
- Commit under test: `d9e60f3150` (`v0.122.2-termux`)
- Global package under test: `@mmmbuto/codex-cli-termux@0.122.2-termux`
- Suite type: runtime-only validation of the installed Termux package
- Suite reference: `test-reports/latest/CLI_RUNTIME_REPORT.md`

## Version Snapshot

- `codex --version` -> `codex-cli 0.122.2-termux`
- `codex-exec --version` -> `codex-exec 0.122.2-termux`
- Global npm package -> `@mmmbuto/codex-cli-termux@0.122.2-termux`
- Global command path -> `/data/data/com.termux/files/usr/bin/codex`
- Global wrapper target -> `/data/data/com.termux/files/usr/lib/node_modules/@mmmbuto/codex-cli-termux/bin/codex.js`

## Runtime Checks

- `PASS` package installed globally at the expected version
- `PASS` global command paths resolve to the installed Termux package
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
- `PASS` `codex login status`
- `PASS` `codex mcp list`
- `PASS` `codex features list`
- `PASS` `codex completion bash`
- `PASS` `codex debug prompt-input --help`
- `PASS` `codex app-server generate-json-schema --help`
- `PASS` `codex app-server generate-json-schema --out <local-dir>`
- `PASS` wrapper routing through `node .../bin/codex.js` for `fork`, `debug`, `review`, `exec`, `login`, `logout`, and `resume`
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json` list-files smoke
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json` create/read `hello.txt` smoke
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json` network smoke returned HTTP 200
- `PASS` installed `codex.bin` and `codex-exec.bin` have `RUNPATH=$ORIGIN`
- `PASS` installed binary shared library needs are the expected Android runtime libraries
- `PASS` Termux environment confirmed via `uname`, `node`, `npm`, and `termux-open-url`
- `PASS` `verify-patches.sh`
- `PASS` temporary runtime artifact cleanup

## Patch Inventory Check

- `PASS` `verify-patches.sh`

The local repository had stale, untracked staged ELF artifacts under `npm-package/bin`. They were not part of the installed package under test and were removed from the checkout before rerunning the patch inventory check.

The installed package under test reports the expected runtime hardening:

```text
RUNPATH: $ORIGIN
```

With no stale staged ELFs present, `verify-patches.sh` validates the source configuration and defers packaged ELF validation until fresh Android ELFs are staged.

## Result

`PASS`

The installed `@mmmbuto/codex-cli-termux@0.122.2-termux` package passed the runtime suite on Pixel9Pro Termux, including real `codex-exec` workspace and network smoke checks.
