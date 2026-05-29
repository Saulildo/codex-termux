# CODEX TEST REPORT - v0.135.0 (Termux)

- Package under test: `@mmmbuto/codex-cli-termux@0.135.0`
- Date: `2026-05-29 17:39`
- Test mode: manual AI-guided suite (`AI_GUIDED_TEST_SUITE.md`)
- Environment: Android + Termux (sanitized)

## 1. Package And Repo State

Commands executed:

```sh
codex --version
codex-exec --version
npm ls -g --depth=0 @mmmbuto/codex-cli-termux
npm view @mmmbuto/codex-cli-termux dist-tags --json
```

Results:

- `codex-cli 0.135.0`
- `codex-exec 0.135.0`
- Global npm package installed at `0.135.0`
- Dist-tags:
  - `latest: 0.134.1`
  - `next: 0.135.0`

Assessment:

- Version consistency between wrapper and binaries: PASS
- Channel status: `0.135.0` is currently tagged as `next` (not `latest`)

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
codex completion bash > completion-file
```

Results:

- All help commands completed successfully.
- Completion generation completed successfully.
- Installed npm wrapper correctly routes command surface.

Assessment: PASS

## 3. Runtime Smoke (Temporary Workspace)

Workspace created in a temporary local directory outside repo. Seed file prepared.

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

Note: one intermediate attempt read `hello.txt` before creation; tool automatically retried in sequence and completed successfully.

Assessment: PASS

### Smoke 4

```sh
codex-exec --sandbox workspace-write --skip-git-repo-check --json \
  'Run one network check with curl -I https://www.google.com and report the first HTTP status line only.'
```

Result: `HTTP/2 200`

Assessment: PASS

## 4. Termux-Specific Checks

Commands executed:

```sh
command -v termux-open-url
readelf -d <codex.bin> | grep 'RUNPATH.*$ORIGIN'
readelf -d <codex-exec.bin> | grep 'RUNPATH.*$ORIGIN'
```

Results:

- `termux-open-url` available
- `codex.bin` RUNPATH contains `$ORIGIN`
- `codex-exec.bin` RUNPATH contains `$ORIGIN`

Assessment: PASS

## 5. Failures / Blockers

- No blocking failures found in runtime or integration checks.
- Non-blocking release note: npm `latest` still points to `0.134.1`; `0.135.0` is available on `next`.

## Final Verdict

**PASS for current-version validation (`0.135.0`) on Termux.**

The package/wrapper surface, runtime smoke tests, and Termux-specific integration points are all functioning for the tested current version.
