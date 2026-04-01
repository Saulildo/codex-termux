# CODEX TEST REPORT v0.117.2-termux (Extended)

- Date: 2026-03-28 12:50:37 CET
- Device: pixel9pro (Termux)
- Repo: `~/Dev/codex-termux`
- Scope: extended validation on top of `test-reports/suites/latest/termux.md`
- Raw log: not captured beyond referenced command outputs

## Extended Checks

## EXT-100 - Patch set verification
Command: `bash verify-patches.sh`
Result: PASS
Notes:
- Critical patches #1, #2, #4, #5, #6, #9, #10, #11, #12 present
- Informational warning on extra undeclared patch files remains unchanged from baseline

## EXT-110 - Android feature guard (`codex-cli`)
Command:
```bash
cd codex-rs
cargo tree -p codex-cli -e features --target aarch64-linux-android | rg -e 'voice-input|cpal|oboe|oboe-sys' || true
```
Result: PASS (empty output)

## EXT-111 - Android feature guard (`codex-cloud-tasks`)
Command:
```bash
cd codex-rs
cargo tree -p codex-cloud-tasks -e features --target aarch64-linux-android | rg -e 'voice-input|cpal|oboe|oboe-sys' || true
```
Result: PASS (empty output)
Notes:
- A first parallel attempt hit a Cargo registry cache unpack race on `cexpr v0.6.0`
- Re-running the command in isolation completed cleanly

## EXT-130 - Trusted-directory guard behavior
Command:
```bash
cd ~/codex-test-workspace
codex-exec --sandbox workspace-write --json "print current directory"
```
Result: PASS
Expected refusal observed:
```text
Not inside a trusted directory and --skip-git-repo-check was not specified.
```

## EXT-140 - Repo-local `termux-tts` skill validity and execution
Command:
```bash
cd ~/Dev/codex-termux
codex-exec --sandbox workspace-write --json \
  "Use \$termux-tts to speak exactly: Codex Termux TTS test 2026-03-28. Then report whether the command succeeded and quote the exact shell command you ran."
```
Result: PASS
Notes:
- Initial validation exposed an invalid `SKILL.md` format: missing YAML frontmatter
- After fixing the skill file, Codex loaded it correctly and executed `termux-tts-speak` with exit code `0`

## Summary
- PASS: 5
- FAIL: 0
- SKIP: 0
- Verdict: PASS
