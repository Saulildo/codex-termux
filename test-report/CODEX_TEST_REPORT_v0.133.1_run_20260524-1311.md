# CODEX TEST REPORT — v0.133.1 (Termux)

## Candidate
- Package: `@mmmbuto/codex-cli-termux`
- Version under test: `0.133.1`
- Run date: `2026-05-24 13:11` (Europe/Rome)
- Suite: `test-report/AI_GUIDED_TEST_SUITE.md`

## Sanitized Environment Summary
- Platform: Termux (Android)
- Branch context: `develop`
- Local repo state: clean
- Note: `/tmp` is not writable in this runtime; temporary artifacts were written to a user temp directory.

## 1) Package And Repo State
- `codex --version` → `codex-cli 0.133.1` ✅
- `codex-exec --version` → `codex-exec 0.133.1` ✅
- `npm ls -g --depth=0 @mmmbuto/codex-cli-termux` → installed `0.133.1` ✅
- `npm view @mmmbuto/codex-cli-termux dist-tags --json` → `latest: 0.133.1` ✅
- npm produced non-blocking warnings about unknown project config keys. ⚠️ (informational)

## 2) Command And Wrapper Surface
- `codex --help` ✅
- `codex exec --help` ✅
- `codex login --help` ✅
- `codex logout --help` ✅
- `codex resume --help` ✅
- `codex mcp --help` ✅
- `codex sandbox --help` ✅
- `codex completion bash` ✅ (generated completion script)

## 3) Runtime Smoke (Temporary Workspace)
- `codex exec --skip-git-repo-check --ephemeral 'Reply with exactly: OK'` → `OK` ✅
- `codex-exec ... 'Print current directory and list files...'` → directory and listing returned ✅
- `codex-exec ... 'Create hello.txt ... read seed.txt and hello.txt back.'` → final readback succeeded ✅
  - Observation: one early parallel read attempted `cat hello.txt` before creation, then recovered and returned correct final output.
- `codex-exec ... 'curl -I https://www.google.com ...'` → `HTTP/2 200` ✅

## 4) Termux-Specific Checks
- `command -v termux-open-url` → present ✅
- `readelf .../codex.bin | grep 'RUNPATH.*$ORIGIN'` → matched ✅
- `readelf .../codex-exec.bin | grep 'RUNPATH.*$ORIGIN'` → matched ✅

## Failures / Blockers
- No blocking failures.
- Informational deviations:
  - `/tmp` not writable in this environment.
  - npm warnings about unknown project config keys.

## Final Verdict
- **PASS** for `0.133.1` on this Termux run.
