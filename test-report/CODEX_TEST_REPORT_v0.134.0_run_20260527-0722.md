# CODEX TEST REPORT v0.134.0 (Termux)

- Candidate under test: `@mmmbuto/codex-cli-termux@0.134.0`
- Date: `2026-05-27 07:22` (Europe/Rome)
- Suite source: `test-report/AI_GUIDED_TEST_SUITE.md`

## Sanitized Environment Summary

- Device class: Android/Termux
- Validation mode: manual AI-guided execution in required order
- Workspace for smoke: temporary directory under user home
- No secrets, private hosts, or raw environment dumps included

## 1. Package And Repo State

Commands run:
- `codex --version`
- `codex-exec --version`
- `npm ls -g --depth=0 @mmmbuto/codex-cli-termux`
- `npm view @mmmbuto/codex-cli-termux dist-tags --json`

Results:
- `codex --version`: `codex-cli 0.134.0`
- `codex-exec --version`: `codex-exec 0.134.0`
- Global install: `@mmmbuto/codex-cli-termux@0.134.0`
- Dist-tags: `{"latest":"0.133.1","next":"0.134.0"}`

Assessment:
- Version coherence for candidate `0.134.0`: PASS
- `latest` tag moved to 0.134.0: NOT YET (informational if release target is `next`)

## 2. Command And Wrapper Surface

Commands run:
- `codex --help`
- `codex exec --help`
- `codex login --help`
- `codex logout --help`
- `codex resume --help`
- `codex mcp --help`
- `codex sandbox --help`
- `codex completion bash`

Results:
- All help commands returned successfully via installed wrapper
- Completion output generated successfully

Assessment: PASS

## 3. Runtime Smoke

Setup:
- Created temp workspace
- Seed file: `seed.txt` containing `seed`

Commands run:
- `codex exec --skip-git-repo-check --ephemeral 'Reply with exactly: OK'`
- `codex-exec --sandbox workspace-write --skip-git-repo-check --json 'Print current directory and list files. Do not modify files.'`
- `codex-exec --sandbox workspace-write --skip-git-repo-check --json 'Create hello.txt with content hello-codex-termux, then read seed.txt and hello.txt back.'`
- `codex-exec --sandbox workspace-write --skip-git-repo-check --json 'Run one network check with curl -I https://www.google.com and report the first HTTP status line only.'`

Results:
- Exact ephemeral reply: `OK`
- Read/list: PASS (`seed.txt` visible)
- Write/read: PASS (`hello.txt` created, `seed` and `hello-codex-termux` read back)
- Network check: PASS (`HTTP/2 200`)

Assessment: PASS

## 4. Termux-Specific Checks

Commands run:
- `command -v termux-open-url`
- `readelf -d <codex.bin> | grep 'RUNPATH.*$ORIGIN'`
- `readelf -d <codex-exec.bin> | grep 'RUNPATH.*$ORIGIN'`

Results:
- `termux-open-url`: present
- `codex.bin`: `RUNPATH [$ORIGIN]`
- `codex-exec.bin`: `RUNPATH [$ORIGIN]`

Assessment: PASS

## Failures / Blockers

- No runtime blockers observed.
- Minor suite drift: `AI_GUIDED_TEST_SUITE.md` still states PASS target `0.131.0`; current candidate is `0.134.0`.
- Publish-state note: npm `latest` still points to `0.133.1` while `0.134.0` is on `next`.

## Final Verdict

- Candidate `0.134.0` functional validation on Termux: **PASS**.
- Promotion to `latest` depends on release decision, not on runtime test failures.
