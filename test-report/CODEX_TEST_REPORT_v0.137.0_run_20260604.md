# Codex Termux 0.137.0 — Device Validation Report

- Date: 2026-06-04
- Package under test: `@mmmbuto/codex-cli-termux@0.137.0` (npm `latest`)
- Platform: Android/Termux, ARM64 device
- Validation mode: manual AI-guided checks on device, maintainer-confirmed

## Scope

Post-release validation of the 0.137.0 package (upstream base `rust-v0.137.0`)
on a physical Termux device, with focus on the regression class fixed in
0.136.1 (startup TLS panic, issue #11).

## Results

- PASS: install from npm `latest` and version check (`codex 0.137.0`)
- PASS: TUI startup — no `rustls-platform-verifier` panic (issue #11 class)
- PASS: first TLS handshake and model loading on device network
- PASS: interactive surface exercised without fatal UI breaks
- PASS: runtime smoke in a disposable workspace (shell read/write)

## Final Decision

PASS — 0.137.0 confirmed on device by the maintainer.
