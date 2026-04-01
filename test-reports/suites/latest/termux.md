# Latest Test Suite (Termux / Android ARM64)

Purpose: validate the Latest Termux-only build (`@mmmbuto/codex-cli-termux`) using
the global `codex` and `codex-exec` commands.

WARNING: This release may ship with incomplete re-validation. Run this suite
before relying on it in production.

Manual execution policy:
- Execute tests manually, command-by-command.
- Do not run this suite via local automation scripts/custom runners.
- Compile reports only from manually executed test evidence.

## Install Guard (Required)

Confirm you are testing the Termux package (not upstream):

```bash
npm ls -g --depth=0 @mmmbuto/codex-cli-termux || true
```

Expected: installed version ends with `-termux` (example `0.116.0-termux`).

Confirm the commands you are running are the global ones:

```bash
command -v codex
command -v codex-exec
ls -la "$(command -v codex)" "$(command -v codex-exec)"
```

## Version Guard (Required)

The CLI should report the expected upstream semver line. Depending on upstream,
the `--version` output may be plain semver even when the npm/tag version uses
`-termux`.

```bash
codex --version
codex-exec --version
```

## Core Tests

Workspace:

```bash
rm -rf ~/codex-test-workspace
mkdir -p ~/codex-test-workspace
cd ~/codex-test-workspace
```

Help:

```bash
codex --help
codex exec --help
codex-exec --help
```

Non-interactive sanity:

```bash
# NOTE: Recent upstream builds can refuse to run outside a trusted directory.
# If you see: "Not inside a trusted directory and --skip-git-repo-check was not specified."
# rerun with --skip-git-repo-check (as below), or run inside a trusted git repo.
# NOTE: Default sandbox can be read-only; use workspace-write so file creation checks work.
codex-exec --sandbox workspace-write --skip-git-repo-check --json "print current directory and list files"
codex-exec --sandbox workspace-write --skip-git-repo-check --json "create hello.txt with content 'hello' and then read it"
```

## v0.117.2 Termux TTS Skill Guard

Run this from the source repo so Codex can load the repo-local `.codex/skills/termux-tts/`.

```bash
command -v termux-tts-speak || true
cd ~/Dev/codex-termux
codex-exec --sandbox workspace-write --json \
  "Use \$termux-tts to speak exactly: Codex Termux TTS test 2026-03-28. Then report whether the command succeeded and quote the exact shell command you ran."
```

Expected:
- `termux-tts-speak` is present in `PATH`
- the skill loads without `SKILL.md` parsing errors
- Codex reports that it used `termux-tts`
- the executed command is `termux-tts-speak "Codex Termux TTS test 2026-03-28."`
- the command exits with code `0`

## v0.104.0 Regression Guard (Android network policy stub)

Binary architecture guard:

```bash
PKG_BIN_DIR="$(npm root -g)/@mmmbuto/codex-cli-termux/bin"
file "$PKG_BIN_DIR/codex"
file "$PKG_BIN_DIR/codex-exec"
file "$PKG_BIN_DIR/codex.bin"
file "$PKG_BIN_DIR/codex-exec.bin"
```

Expected:
- `codex` and `codex-exec` are launcher scripts
- `codex.bin` and `codex-exec.bin` are Linux/Android ELF
- ELF architecture is `aarch64`/`ARM64`

Network-path smoke (must not panic):

```bash
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  "run one network check with curl -I https://www.google.com and report the first HTTP status line only"
```

Expected:
- no crash/panic
- no errors referencing missing network policy symbols (for example
  `NetworkDecision::ask`, `NetworkDecision::deny`, or `BlockedRequest.decision`)
- command may succeed or be blocked by policy, but failure must be graceful

## v0.111.0 Dependency Crash Guard (Android audio/linkage)

Source dependency feature guard (maintainer-only, from source repo):

```bash
cd ~/Dev/codex-termux/codex-rs
cargo tree -p codex-cli -e features --target aarch64-linux-android | rg -e 'voice-input|cpal|oboe|oboe-sys' || true
cargo tree -p codex-cloud-tasks -e features --target aarch64-linux-android | rg -e 'voice-input|cpal|oboe|oboe-sys' || true
```

Expected:
- output is empty for both commands
- Android consumers do not pull `voice-input`, `cpal`, `oboe`, or `oboe-sys`

Installed binary linkage guard:

```bash
PKG_BIN_DIR="$(npm root -g)/@mmmbuto/codex-cli-termux/bin"
READELF_BIN="$(command -v readelf || command -v llvm-readelf || true)"
if [ -n "$READELF_BIN" ]; then
  "$READELF_BIN" -d "$PKG_BIN_DIR/codex.bin" | rg "NEEDED|libc\\+\\+|OpenSLES|oboe"
  "$READELF_BIN" -d "$PKG_BIN_DIR/codex-exec.bin" | rg "NEEDED|libc\\+\\+|OpenSLES|oboe"
else
  echo "SKIP: readelf/llvm-readelf not available"
fi
```

Expected:
- if a C++ runtime is listed, it must be shared (`libc++_shared.so`)
- no reference to `libc++_static`
- no reference to `libOpenSLES.so`
- no reference to `liboboe.so`
- no missing-library runtime errors when invoking `codex`/`codex-exec`

## v0.113.0+ Wrapper Routing Guard (npm launcher)

The Node launcher must not misroute valid root subcommands to `codex exec`.

```bash
PKG_BIN_DIR="$(npm root -g)/@mmmbuto/codex-cli-termux/bin"
node "$PKG_BIN_DIR/codex.js" fork --help | sed -n '1,6p'
node "$PKG_BIN_DIR/codex.js" debug --help | sed -n '1,6p'
```

Expected:
- first command includes `Usage: codex fork`
- second command includes `Usage: codex debug`
- output does not start with `Usage: codex exec`

Maintainer-only compile guard (optional, from source repo):

```bash
cd ~/Dev/codex-termux/codex-rs
cargo check -p codex-network-proxy --target aarch64-linux-android
cargo check -p codex-core --target aarch64-linux-android
cargo check -p codex-cli --target aarch64-linux-android
```

Expected:
- all checks complete without compile errors

Termux checks:

```bash
uname -a
echo "$PREFIX"
node --version
npm --version
command -v termux-open-url || true
```
