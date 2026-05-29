# CODEX TEST REPORT v0.133.0

- Date: 2026-05-23
- Device: Termux Android device
- Repo: Termux checkout of `DioNanos/codex-termux`
- Commit under test: `170837e75` (`develop`)
- Global package under test: `@mmmbuto/codex-cli-termux 0.133.0`
- Suite type: runtime-only validation of the installed Termux package
- Suite reference: `test-report/AI_GUIDED_TEST_SUITE.md`

## Package And Repo State

- `PASS` `codex --version` returned `codex-cli 0.133.0`
- `PASS` `codex-exec --version` returned `codex-exec 0.133.0`
- `PASS` global npm package check found `@mmmbuto/codex-cli-termux@0.133.0`
- `PASS` npm dist-tags reported `latest: 0.132.0`, `next: 0.133.0`
- `PASS` repo checkout was on `develop` at `170837e75`

## Command And Wrapper Surface

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
- `PASS` `codex remote-control --help`
- `PASS` `codex login status`
- `PASS` `codex mcp list`
- `PASS` `codex features list`
- `PASS` `codex completion bash`
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

## Runtime Smoke

Runtime smoke tests were run from a temporary workspace, not from the repo.

- `PASS` `codex exec --skip-git-repo-check --ephemeral "Reply with exactly: OK"` — returned `OK`
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "Print current directory and list files"` — listed workspace correctly
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "Create hello.txt with content hello-codex-termux, then read seed.txt and hello.txt back."` — write and read both files confirmed
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "Run one network check with curl -I https://www.google.com and report the first HTTP status line only."` — returned `HTTP/2 200`

## Termux-Specific Checks

- `PASS` `termux-open-url` is available
- `PASS` installed `codex.bin` has `RUNPATH=$ORIGIN`
- `PASS` installed `codex-exec.bin` has `RUNPATH=$ORIGIN`
- `PASS` installed Android ELFs expose only system dynamic libraries (`libz`, `libdl`, `libm`, `libc`), `libc++_shared.so` resolved via `RUNPATH=$ORIGIN`
- `PASS` Termux runtime guard
- `PASS` `verify-patches.sh` — all 12 patches PRESENT:
  - Patch #1 (Browser Login)
  - Patch #2 (Release Profile)
  - Patch #4/#5 (Fork Update Channel + Version Parser)
  - Patch #6 (Termux npm Package Name)
  - Patch #10 (Launcher Hardening)
  - Patch #10b (Android ELF Runpath)
  - Patch #11 (Android No-Voice Policy)
  - Patch #12 (Dynamic Subcommand Routing)
  - Patch #13 (Fork-safe Managed Updates)
  - Patch #14 (Fork-owned Public Install Surfaces)
  - Patch #15 (Fork-owned Feedback Surfaces)
  - Patch #16 (Android Remote-Control Daemon)

## Environment Summary

- Node.js: `v25.8.2`
- npm: `11.14.1`
- Android release: `16`
- Android ABI: `arm64-v8a`

## Notes

- No recompilation was performed.
- This validation targets the installed package, not source-tree build parity.
- The runtime package version is `0.133.0` without a `-termux` suffix.
- The `next` dist-tag on npm points to `0.133.0`; `latest` still points to `0.132.0`.
- New in v0.133.0: `codex remote-control` subcommand (Patch #16) routes correctly; daemon lifecycle (start/stop) validated via `--help` surface only.
- Local paths, usernames, package install paths, and raw environment dumps were intentionally omitted from this report.

## Final Verdict

`PASS`
