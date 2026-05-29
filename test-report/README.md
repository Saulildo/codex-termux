# Codex Termux Test Reports

This directory keeps sanitized release validation evidence for the Termux fork.

## Current Flow

- `AI_GUIDED_TEST_SUITE.md` is the checklist for post-install Termux testing.
- Build logs and local machine details do not belong in committed reports.
- Device reports must be sanitized before GitHub mirroring.

## Report Rules

Reports must not include:

- absolute local paths
- private hosts, IPs, usernames, or account identifiers
- tokens, raw environment dumps, or MCP secrets
- unrelated process lists

Use placeholders such as `<repo>`, `<tmp-workspace>`, and `<termux-device>`.
