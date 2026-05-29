# Bug Note: `codex-cli 0.124.0-termux`

Date: `2026-04-24`

Runtime issue found on Android (Termux):

- `codex exec --ephemeral "Reply with exactly: OK"` starts a session but does not complete within the runtime timeout window.
- `codex-exec --json` smoke commands emit `thread.started` and `turn.started` and then stall without a final result.

What still worked:

- installed package/version resolution
- help and wrapper routing
- `codex login status`
- `codex mcp list`
- `codex features list`
- app-server schema generation
- patch inventory
- Android ELF linkage checks

Full report:

- `test-report/CODEX_TEST_REPORT_v0.124.0-termux_run_20260424-2259.md`
