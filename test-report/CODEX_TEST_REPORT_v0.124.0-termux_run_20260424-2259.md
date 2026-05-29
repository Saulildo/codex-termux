# CODEX TEST REPORT v0.124.0-termux

- Date: 2026-04-24 22:59 CEST
- Device: Android (Termux)
- Repo: `~/codex-termux`
- Commit under test: `3844b1a49c` (`docs: restore README-linked assets`)
- Global package under test: `@mmmbuto/codex-cli-termux@0.124.0-termux`
- Suite type: runtime validation of the installed Termux package
- Output location requested: `test-report/`

## Version Snapshot

- `codex --version` -> `codex-cli 0.124.0-termux`
- `codex-exec --version` -> `codex-exec 0.124.0-termux`
- Global npm package -> `@mmmbuto/codex-cli-termux@0.124.0-termux`
- Global command path -> `/data/data/com.termux/files/usr/bin/codex`
- Global wrapper target -> `/data/data/com.termux/files/usr/lib/node_modules/@mmmbuto/codex-cli-termux/bin/codex.js`

## Runtime Checks

- `PASS` package installed globally at the expected version
- `PASS` global command path resolves to the installed Termux package
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
- `PASS` `codex app-server generate-json-schema --out <local-dir>`
- `PASS` wrapper routing through `node .../bin/codex.js` for `fork`, `debug`, `review`, `exec`, `login`, `logout`, and `resume`
- `FAIL` `codex exec --ephemeral "Reply with exactly: OK"` did not complete within `90s`
- `FAIL` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "print current directory and list files"` did not complete within `90s`
- `FAIL` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "create hello.txt with content 'hello' and then read it back"` did not complete within `90s`
- `FAIL` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "run one network check with curl -I https://www.google.com and report the first HTTP status line only"` did not complete within `90s`
- `PASS` installed `codex.bin` and `codex-exec.bin` have `RUNPATH=$ORIGIN`
- `PASS` installed binary shared library needs are the expected Android runtime libraries
- `PASS` Termux environment confirmed via `uname`, `node`, `npm`, and `termux-open-url`
- `PASS` `verify-patches.sh` when invoked with `bash`

## Failure Detail

The regression is on real non-interactive execution, not on wrapper/help routing.

Observed behavior:

- `codex exec --ephemeral` prints `Reading additional input from stdin...`, starts a session, and never returns within the timeout window.
- `codex-exec --json` smoke commands print `Reading additional input from stdin...`, emit `thread.started` and `turn.started`, and then stall without a terminal result.

Minimal captured outputs:

```text
Reading additional input from stdin...
OpenAI Codex v0.124.0-termux (research preview)
...
user
Reply with exactly: OK
```

```text
Reading additional input from stdin...
{"type":"thread.started","thread_id":"019dc146-0468-7662-95dd-d7d34af46f4a"}
{"type":"turn.started"}
```

```text
Reading additional input from stdin...
{"type":"thread.started","thread_id":"019dc147-681a-7b82-98de-5ae5ad270815"}
{"type":"turn.started"}
```

```text
Reading additional input from stdin...
{"type":"thread.started","thread_id":"019dc148-e832-7811-bc99-97b8614d3d8a"}
{"type":"turn.started"}
```

## Patch Inventory Check

- `PASS` `bash ./verify-patches.sh`

Patch inventory result:

```text
Patch #1 (Browser Login): ✅ PRESENT
Patch #2 (Release Profile): ✅ PRESENT
Patch #4/#5 (Fork Update Channel + Version Parser): ✅ PRESENT
Patch #6 (Termux npm Package Name): ✅ PRESENT
Patch #10 (Launcher Hardening): ✅ PRESENT
Patch #10b (Android ELF Runpath): ✅ PRESENT
Patch #11 (Android No-Voice Policy): ✅ PRESENT
Patch #12 (Dynamic Subcommand Routing): ✅ PRESENT
Bazel patch inventory present: ✅ PRESENT
```

## Binary Linkage Check

Both installed binaries report:

```text
RUNPATH: $ORIGIN
NEEDED: libz.so, libdl.so, libm.so, libc.so
```

## Result

`FAIL`

The installed `@mmmbuto/codex-cli-termux@0.124.0-termux` package is healthy on version reporting, help routing, MCP visibility, schema generation, patch inventory, and Android binary linkage, but it fails the practical runtime gate because real non-interactive execution stalls and does not produce a final result within `90s`.
