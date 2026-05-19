# CODEX TEST REPORT v0.131.0

- Date: 2026-05-19
- Device: Termux Android device
- Repo: Termux checkout of `DioNanos/codex-termux`
- Commit under test: `d75d1a74e` (`develop`)
- Global package under test: `@mmmbuto/codex-cli-termux 0.131.0`
- Suite type: runtime-only validation of the installed Termux package
- Suite reference: `test-report/AI_GUIDED_TEST_SUITE.md`

## Package And Repo State

- `PASS` `codex --version` returned `codex-cli 0.131.0`
- `PASS` `codex-exec --version` returned `codex-exec 0.131.0`
- `PASS` global npm package check found `@mmmbuto/codex-cli-termux@0.131.0`
- `PASS` npm dist-tags reported `latest: 0.131.0`
- `PASS` repo checkout was on `develop` at `d75d1a74e`

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

Runtime smoke tests ran from `<tmp-workspace>`, not from the repo.

- `PASS` `codex exec --skip-git-repo-check --ephemeral "Reply with exactly: OK"` returned `OK`
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "Print current directory and list files. Do not modify files."`
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "Create hello.txt with content hello-codex-termux, then read seed.txt and hello.txt back."`
- `PASS` `codex-exec --sandbox workspace-write --skip-git-repo-check --json "Run one network check with curl -I https://www.google.com and report the first HTTP status line only."`
- Network result: `HTTP/2 200`
- `PASS` temporary workspace cleanup

## Termux-Specific Checks

- `PASS` `termux-open-url` is available
- `PASS` installed `codex.bin` has `RUNPATH=$ORIGIN`
- `PASS` installed `codex-exec.bin` has `RUNPATH=$ORIGIN`
- `PASS` installed Android ELFs expose needed dynamic libraries
- `PASS` Termux runtime guard
- `PASS` `verify-patches.sh`

## Environment Summary

- Node.js: `v25.8.2`
- npm: `11.14.1`
- Android release: `16`
- Android ABI: `arm64-v8a`
- Kernel family: Android Linux `6.1.145`

## Notes

- No recompilation was performed.
- This validation targets the installed package, not source-tree build parity.
- The runtime package version is `0.131.0` without a `-termux` suffix.
- Local paths, usernames, package install paths, and raw environment dumps were intentionally omitted from this report.

## Final Verdict

`PASS`
