# CODEX TEST REPORT - v0.136.0 (Termux)

- Package under test: `@mmmbuto/codex-cli-termux@0.136.0`
- Date: `2026-06-02 19:03`
- Test mode: manual AI-guided suite (`AI_GUIDED_TEST_SUITE.md`)
- Environment: Android + Termux (sanitized)
- Repo branch: `develop`
- Repo HEAD: `a47b8fd98`
- Remote tracking: `origin/develop` at `a47b8fd98`

## 1. Package And Repo State

Commands executed:

```sh
codex --version
codex-exec --version
npm ls -g --depth=0 @mmmbuto/codex-cli-termux
npm view @mmmbuto/codex-cli-termux dist-tags --json
```

Results:

- `codex-cli 0.136.0`
- `codex-exec 0.136.0`
- Global npm package installed at `0.136.0`
- Dist-tags:
  - `latest: 0.135.0`
  - `next: 0.136.0`
- Local repo `develop` matches `origin/develop`

Assessment:

- Version consistency between wrapper and binaries: PASS
- Branch sync with forge/origin: PASS
- Release channel status: PASS for candidate validation, with `next` on `0.136.0`

## 2. Command And Wrapper Surface

Commands executed:

```sh
codex --help
codex exec --help
codex login --help
codex logout --help
codex resume --help
codex mcp --help
codex sandbox --help
codex completion bash
```

Results:

- All help commands completed successfully.
- `codex completion bash` succeeded when written to a writable temp workspace file.
- A first attempt to redirect completion output to `/tmp/codex-termux-completion.bash` failed with a filesystem permission error in this Termux environment; the feature itself passed when rerun in a writable temp workspace.

Assessment: PASS

## 3. Runtime Smoke

Workspace created in a temporary local directory outside the repo. Seed file prepared.

### Smoke 1

```sh
codex exec --skip-git-repo-check --ephemeral 'Reply with exactly: OK'
```

Result: exact response `OK`.

Assessment: PASS

### Smoke 2

```sh
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Print current directory and list files. Do not modify files.'
```

Result: command execution succeeded; current directory and file list returned.

Assessment: PASS

### Smoke 3

```sh
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Create hello.txt with content hello-codex-termux, then read seed.txt and hello.txt back.'
```

Result: final step confirmed content:

- `seed.txt` -> `seed`
- `hello.txt` -> `hello-codex-termux`

Note: initial write attempts were rejected by workspace rules; the command succeeded when the file was written through the shell inside the temp workspace.

Assessment: PASS

### Smoke 4

```sh
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Run one network check with curl -I https://www.google.com and report the first HTTP status line only.'
```

Result: `HTTP/2 200`

Assessment: PASS

### Smoke 5: Code-mode `exec` / `wait`

```sh
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Start a shell command that sleeps about 5 seconds and then prints DONE-WAIT-OK (for example: sleep 5 && echo DONE-WAIT-OK). Do not block the turn while it runs: launch it, then wait for it to finish, and report only its final stdout.'
```

Result: `DONE-WAIT-OK`

Assessment: PASS

## 4. Termux-Specific Checks

Commands executed:

```sh
command -v termux-open-url
readelf -d "$(dirname "$(readlink -f "$(command -v codex)")")/codex.bin" | grep 'RUNPATH.*$ORIGIN'
readelf -d "$(dirname "$(readlink -f "$(command -v codex-exec)")")/codex-exec.bin" | grep 'RUNPATH.*$ORIGIN'
```

Results:

- `termux-open-url` available
- `codex.bin` RUNPATH contains `$ORIGIN`
- `codex-exec.bin` RUNPATH contains `$ORIGIN`

Assessment: PASS

## 5. Failures / Blockers

- No blocking failures found in runtime or integration checks.
- One non-blocking environment issue: redirecting shell output for `codex completion bash` to `/tmp` failed in this Termux shell, but the same command succeeded when redirected to a writable temp workspace path.
- Non-blocking release note remains true: npm `latest` still points to `0.135.0`; `0.136.0` is available on `next`.

## Final Verdict

**PASS for current-version validation (`0.136.0`) on Termux.**

The package/wrapper surface, runtime smoke tests, `exec`/`wait` code-mode path, and Termux-specific integration points are functioning for the tested current version, and the local repo is synchronized on `develop` with `origin/develop`.
