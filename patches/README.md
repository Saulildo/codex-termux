# Termux Patch Inventory

This fork tracks upstream OpenAI Codex and keeps only the compatibility delta
required to publish a working Android Termux package.

- Fork repo: `DioNanos/codex-termux`
- Upstream base for this release: `rust-v0.121.0`
- Current fork release target: `v0.121.0-termux`

## Runtime patches

| Patch | File(s) | Motivo |
| --- | --- | --- |
| #1 Browser login on Android | `codex-rs/login/src/server.rs` | Usa `termux-open-url` su Android invece del path browser desktop. |
| #2 Release profile for constrained devices | `codex-rs/Cargo.toml` | Disabilita `lto` e alza `codegen-units` per build mobile più affidabili. |
| #4 Update source points to fork releases | `codex-rs/tui/src/updates.rs` | Punta gli update alle release del fork `DioNanos/codex-termux`. |
| #5 Version parser accepts `-termux` | `codex-rs/tui/src/updates.rs` | Rimuove il suffisso `-termux` per il confronto semantico delle versioni. |
| #6 Correct package name for self-update | `codex-rs/tui/src/update_action.rs` | Usa `@mmmbuto/codex-cli-termux@latest` per self-update e help test. |
| #10 Launcher hardening | `npm-package/bin/codex`, `npm-package/bin/codex-exec`, `npm-package/bin/*.js` | Mantiene `LD_LIBRARY_PATH` e `CODEX_SELF_EXE` per non perdere `libc++_shared.so` nei re-exec. |
| #10b Android ELF runpath hardening | `codex-rs/.cargo/config.toml` | Aggiunge `RUNPATH=$ORIGIN` per risolvere `libc++_shared.so` anche senza wrapper env. |
| #11 Android no-voice policy | `codex-rs/tui/Cargo.toml`, `codex-rs/tui/src/*`, `codex-rs/cli/Cargo.toml`, `codex-rs/cloud-tasks/Cargo.toml` | Disabilita voice e realtime audio nei build Termux pubblicati. |
| #12 Dynamic npm wrapper routing | `npm-package/bin/codex.js` | Riconosce i root subcommands e non devia comandi validi su `codex exec`. |
| #13 Android network-proxy stub | retired | Lo stub non serve più: upstream `0.119.0` compila `network-proxy` direttamente su Android nel path Unix-family. |

## Verification

Run from repo root:

```bash
bash verify-patches.sh
```
