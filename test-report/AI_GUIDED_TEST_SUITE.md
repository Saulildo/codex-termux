# Codex Termux AI-Guided Test Suite

This suite validates the installed `@mmmbuto/codex-cli-termux` package on a
Termux device. The validating AI or operator must inspect each result and write
a sanitized report; do not treat a bulk script exit code as the release verdict.

## Required Order

1. Confirm package and repo state.
2. Check command and wrapper surface.
3. Run runtime smoke tests in a temporary workspace.
4. Check Termux-specific integration points.
5. Write a sanitized report.

## 1. Package And Repo State

```sh
codex --version
codex-exec --version
npm ls -g --depth=0 @mmmbuto/codex-cli-termux
npm view @mmmbuto/codex-cli-termux dist-tags --json
```

PASS requires both commands to report the candidate version (currently
`0.136.0`) and npm `next` to point to it. `latest` stays on `0.135.0` until the
candidate is promoted.

## 2. Command And Wrapper Surface

```sh
codex --help
codex exec --help
codex login --help
codex logout --help
codex resume --help
codex mcp --help
codex sandbox --help
codex completion bash >/tmp/codex-termux-completion.bash
```

PASS requires help routing to work through the installed npm wrapper.

## 3. Runtime Smoke

Run from a temporary workspace, not from the repo:

```sh
tmp="$(mktemp -d)"
printf 'seed\n' > "$tmp/seed.txt"
cd "$tmp"

codex exec --skip-git-repo-check --ephemeral 'Reply with exactly: OK'
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Print current directory and list files. Do not modify files.'
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Create hello.txt with content hello-codex-termux, then read seed.txt and hello.txt back.'
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Run one network check with curl -I https://www.google.com and report the first HTTP status line only.'
```

PASS requires exact `OK`, read/list success, write/read success, and either an
HTTP status line or a clearly classified environmental network failure.

## 3.1 Code-mode `exec` / `wait` (Android V8 restore)

On `0.136.0` the Android build runs real code-mode via the in-process V8
runtime (previously a no-op stub). This case exercises the `exec` (start) and
`wait` (await a still-running command) path, which only works when V8 is
actually linked and functional on the device.

Run from the same temporary workspace:

```sh
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Start a shell command that sleeps about 5 seconds and then prints DONE-WAIT-OK (for example: sleep 5 && echo DONE-WAIT-OK). Do not block the turn while it runs: launch it, then wait for it to finish, and report only its final stdout.'
```

PASS requires the final output to contain `DONE-WAIT-OK`, demonstrating that the
command was launched and awaited to completion (the `exec`+`wait` code-mode
path). A no-op/stubbed V8 build would either fail to run the command or return
no real output.

> Note (audio): realtime voice (`/realtime`, `/settings`) is out of scope for
> this suite on Termux — the audio backend cannot initialize in a Termux CLI
> process. Do not enable `realtime_conversation` on Termux. See README.

## 4. Termux-Specific Checks

```sh
command -v termux-open-url
readelf -d "$(dirname "$(readlink -f "$(command -v codex)")")/codex.bin" | grep 'RUNPATH.*$ORIGIN'
readelf -d "$(dirname "$(readlink -f "$(command -v codex-exec)")")/codex-exec.bin" | grep 'RUNPATH.*$ORIGIN'
```

PASS requires `termux-open-url` to be available for browser login and both
Android ELFs to resolve sibling libraries with `RUNPATH=$ORIGIN`.

## 5. Report Format

Create one report per candidate:

```text
test-report/CODEX_TEST_REPORT_v0.136.0_run_YYYYMMDD-HHMM.md
```

Include:

- version and package under test
- sanitized environment summary
- command surface results
- runtime smoke results
- Termux-specific checks
- failures, blockers, and final verdict

Do not include absolute local paths, private hosts, usernames, tokens, raw
environment dumps, or unrelated process lists.
