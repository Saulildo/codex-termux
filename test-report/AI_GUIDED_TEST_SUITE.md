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

PASS requires both commands to report `0.131.0` and npm `latest` to point to
`0.131.0`.

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
test-report/CODEX_TEST_REPORT_v0.131.0_run_YYYYMMDD-HHMM.md
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
