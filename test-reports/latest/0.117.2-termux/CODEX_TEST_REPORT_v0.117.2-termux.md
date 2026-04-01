# CODEX TEST REPORT v0.117.2-termux (Latest Suite)

- Date: 2026-03-28 12:50:37 CET
- Device: pixel9pro (Termux)
- Repo: `~/Dev/codex-termux`
- Global package under test: `@mmmbuto/codex-cli-termux`
- Suite reference: `test-reports/suites/latest/termux.md`
- Raw logs:
  - `test-reports/latest/0.117.2-termux/TEST_noninteractive_1.jsonl`
  - `test-reports/latest/0.117.2-termux/TEST_noninteractive_2.jsonl`
  - `test-reports/latest/0.117.2-termux/TEST_network_smoke.jsonl`
  - `test-reports/latest/0.117.2-termux/TEST_termux_tts_skill.jsonl`

## Version Check Snapshot
- Global installed: `@mmmbuto/codex-cli-termux@0.117.2-termux`
- CLI version: `codex-cli 0.117.2-termux`
- Exec version: `codex-exec 0.117.2-termux`

## Results

## TEST-100 - Install Guard: package installed
Result: PASS

## TEST-101 - Install Guard: global command paths
Result: PASS
- `codex` -> `/data/data/com.termux/files/usr/bin/codex`
- `codex-exec` -> `/data/data/com.termux/files/usr/bin/codex-exec`
- Both commands resolve to the global npm wrapper symlinks

## TEST-200 - Version Guard
Result: PASS

## TEST-300 - Workspace setup
Result: PASS

## TEST-301 - Help: `codex --help`
Result: PASS

## TEST-302 - Help: `codex exec --help`
Result: PASS

## TEST-303 - Help: `codex-exec --help`
Result: PASS

## TEST-400 - Non-interactive sanity: list files
Command:
```bash
cd ~/codex-test-workspace
codex-exec --sandbox workspace-write --skip-git-repo-check --json "print current directory and list files"
```
Evidence: `test-reports/latest/0.117.2-termux/TEST_noninteractive_1.jsonl`
Result: PASS

## TEST-401 - Non-interactive sanity: create/read hello.txt
Command:
```bash
cd ~/codex-test-workspace
codex-exec --sandbox workspace-write --skip-git-repo-check --json "create hello.txt with content 'hello' and then read it"
```
Evidence: `test-reports/latest/0.117.2-termux/TEST_noninteractive_2.jsonl`
Result: PASS
Notes:
- The agent first hit `patch rejected: writing outside of the project; rejected by user approval settings`
- It recovered by creating `hello.txt` with a shell command and then reading it back successfully

## TEST-500 - Binary architecture guard
Result: PASS
- `codex` and `codex-exec` are launcher scripts
- `codex.bin` and `codex-exec.bin` are Android ARM64 ELF binaries
- Both binaries target Android API 29

## TEST-600 - Network-path smoke (no panic)
Command:
```bash
cd ~/codex-test-workspace
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  "run one network check with curl -I https://www.google.com and report the first HTTP status line only"
```
Evidence: `test-reports/latest/0.117.2-termux/TEST_network_smoke.jsonl`
Result: PASS
- Returned `HTTP/2 200`
- No panic and no missing network policy symbol errors observed

## TEST-650 - Termux TTS skill guard
Command:
```bash
command -v termux-tts-speak || true
cd ~/Dev/codex-termux
codex-exec --sandbox workspace-write --json \
  "Use \$termux-tts to speak exactly: Codex Termux TTS test 2026-03-28. Then report whether the command succeeded and quote the exact shell command you ran."
```
Evidence: `test-reports/latest/0.117.2-termux/TEST_termux_tts_skill.jsonl`
Result: PASS
- `termux-tts-speak` found at `/data/data/com.termux/files/usr/bin/termux-tts-speak`
- The repo-local `termux-tts` skill loaded successfully after frontmatter fix
- Codex executed `termux-tts-speak "Codex Termux TTS test 2026-03-28."`
- Command exited with code `0`

## TEST-700 - Installed binary linkage guard
Result: PASS
- No `libOpenSLES` linkage found
- No `liboboe` linkage found
- No `libc++_static` linkage found
- Dynamic dependencies observed: `libz.so`, `libdl.so`, `liblog.so`, `libm.so`, `libc.so`

## TEST-800 - Wrapper routing guard: fork/debug
Result: PASS
- `fork --help` routes to `Usage: codex fork`
- `debug --help` routes to `Usage: codex debug`

## TEST-900 - Termux environment checks
Result: PASS
- Kernel: `Linux localhost 6.1.145-android14-11-gfa1d6308d1fe-ab14691759 #1 SMP PREEMPT Fri Jan 9 16:33:46 UTC 2026 aarch64 Android`
- Prefix: `/data/data/com.termux/files/usr`
- Node: `v25.3.0`
- npm: `11.12.0`
- `termux-open-url` present

## Summary
- PASS: 15
- FAIL: 0
- SKIP: 0
- Verdict: PASS

## Residual Risk
- The create/write sanity test succeeded, but the first write attempt via patch tool was rejected as outside project scope before the agent fell back to a shell write. This is not a suite failure, but it is worth re-checking in a focused helper-tool regression if we want confidence that `apply_patch` behaves correctly in ad hoc workspaces.
