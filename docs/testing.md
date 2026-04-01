# Testing

## Test Reports

Comprehensive test validation for all platforms and release lines.

### Latest (Termux)

- **v0.118.0-termux**: current latest release built from upstream `rust-v0.118.0` with Android API 29 output on Termux ARM64; native Termux validation is pending post-release manual testing
- **v0.117.2-termux**: [test-reports/latest/0.117.2-termux/CODEX_TEST_REPORT_v0.117.2-termux.md](../test-reports/latest/0.117.2-termux/CODEX_TEST_REPORT_v0.117.2-termux.md)
- **Extended**: [test-reports/latest/0.117.2-termux/CODEX_TEST_REPORT_v0.117.2-termux_extended.md](../test-reports/latest/0.117.2-termux/CODEX_TEST_REPORT_v0.117.2-termux_extended.md)
- **v0.116.1-termux**: [test-reports/latest/0.116.1-termux/CODEX_TEST_REPORT_v0.116.1-termux.md](../test-reports/latest/0.116.1-termux/CODEX_TEST_REPORT_v0.116.1-termux.md)
- **Extended**: [test-reports/latest/0.116.1-termux/CODEX_TEST_REPORT_v0.116.1-termux_extended.md](../test-reports/latest/0.116.1-termux/CODEX_TEST_REPORT_v0.116.1-termux_extended.md)
- **v0.112.0-termux**: [test-reports/latest/0.112.0-termux/CODEX_TEST_REPORT_v0.112.0-termux.md](../test-reports/latest/0.112.0-termux/CODEX_TEST_REPORT_v0.112.0-termux.md)
- **Extended**: [test-reports/latest/0.112.0-termux/CODEX_TEST_REPORT_v0.112.0-termux_extended.md](../test-reports/latest/0.112.0-termux/CODEX_TEST_REPORT_v0.112.0-termux_extended.md)
- **v0.111.0-termux**: [test-reports/latest/0.111.0-termux/CODEX_TEST_REPORT_v0.111.0-termux.md](../test-reports/latest/0.111.0-termux/CODEX_TEST_REPORT_v0.111.0-termux.md)
- **v0.110.0-termux**: [test-reports/latest/0.110.0-termux/CODEX_TEST_REPORT_v0.110.0-termux.md](../test-reports/latest/0.110.0-termux/CODEX_TEST_REPORT_v0.110.0-termux.md)
- **v0.108.0-termux**: [test-reports/latest/0.108.0-termux/CODEX_TEST_REPORT_v0.108.0-termux.md](../test-reports/latest/0.108.0-termux/CODEX_TEST_REPORT_v0.108.0-termux.md)
- **v0.101.0-termux**: [test-reports/latest/0.101.0-termux/CODEX_TEST_REPORT_v0.101.0-termux.md](../test-reports/latest/0.101.0-termux/CODEX_TEST_REPORT_v0.101.0-termux.md)
- **Extended**: [test-reports/latest/0.101.0-termux/CODEX_TEST_REPORT_v0.101.0-termux_extended.md](../test-reports/latest/0.101.0-termux/CODEX_TEST_REPORT_v0.101.0-termux_extended.md)
- **v0.100.0-termux**: [test-reports/latest/0.100.0-termux/CODEX_TEST_REPORT_v0.100.0-termux.md](../test-reports/latest/0.100.0-termux/CODEX_TEST_REPORT_v0.100.0-termux.md)
- **Extended**: [test-reports/latest/0.100.0-termux/CODEX_TEST_REPORT_v0.100.0-termux_extended.md](../test-reports/latest/0.100.0-termux/CODEX_TEST_REPORT_v0.100.0-termux_extended.md)
- **v0.99.0-termux**: [test-reports/latest/0.99.0-termux/CODEX_TEST_REPORT_v0.99.0-termux.md](../test-reports/latest/0.99.0-termux/CODEX_TEST_REPORT_v0.99.0-termux.md)
- **v0.98.0-termux**: [test-reports/latest/0.98.0-termux/CODEX_TEST_REPORT_v0.98.0-termux.md](../test-reports/latest/0.98.0-termux/CODEX_TEST_REPORT_v0.98.0-termux.md)
- **Extended**: [test-reports/latest/0.98.0-termux/CODEX_TEST_REPORT_v0.98.0-termux_extended.md](../test-reports/latest/0.98.0-termux/CODEX_TEST_REPORT_v0.98.0-termux_extended.md)
- **Suite**: [test-reports/suites/latest/termux.md](../test-reports/suites/latest/termux.md)

### LTS

#### Linux
- **v0.80.4-lts**: [test-reports/lts/0.80.4-lts/CODEX_TEST_REPORT_v0.80.4-lts_linux.md](../test-reports/lts/0.80.4-lts/CODEX_TEST_REPORT_v0.80.4-lts_linux.md)
- **Suite**: [test-reports/suites/lts/linux.md](../test-reports/suites/lts/linux.md)

#### Termux
- **v0.80.4-lts**: [test-reports/lts/0.80.4-lts/CODEX_TEST_REPORT_v0.80.4-lts_termux.md](../test-reports/lts/0.80.4-lts/CODEX_TEST_REPORT_v0.80.4-lts_termux.md)
- **Suite**: [test-reports/suites/lts/termux.md](../test-reports/suites/lts/termux.md)

#### macOS
- **v0.80.4-lts**: [test-reports/lts/0.80.4-lts/CODEX_TEST_REPORT_v0.80.4-lts_mac.md](../test-reports/lts/0.80.4-lts/CODEX_TEST_REPORT_v0.80.4-lts_mac.md)
- **Suite**: [test-reports/suites/lts/macos.md](../test-reports/suites/lts/macos.md)

---

## Test Suite Overview

### Categories

1. System Information (3 tests)
2. File Operations (8 tests)
3. Search & Discovery (3 tests)
4. Shell Execution (4 tests)
5. Text Processing (2 tests)
6. Web & Network (2 tests - optional)
7. Git Operations (2 tests - optional)
8. AI Capabilities (3 tests)
9. Error Handling (3 tests)
10. **Termux-Specific (14 tests)** ⭐ - Validates Android patches, dependency crash guards, npm wrapper routing, and the repo-local Termux TTS skill
11. Cleanup (1 test)
12. **Package & Binary (8 tests)** ⭐ - Validates npm installation and binaries

### How to Run Tests

Manual execution policy for Latest Termux:
- Run tests manually, command-by-command.
- Do not use local automation scripts or custom runners.
- Write reports only from manually executed results.

```bash
# Start Codex
codex

# Feed the test suite
> Read and execute all tests in https://github.com/DioNanos/codex-termux/blob/main/test-reports/suites/latest/termux.md
```

For each test, record:
1. PASS/FAIL/SKIP
2. Command(s) run
3. Short evidence/output note

### Success Criteria

- All System, Files, Shell, and Termux tests must pass
- At least 80% overall pass rate
- No critical crashes

---

## Platform-Specific Notes

### Termux

Tests validate:
- Environment paths (`$PREFIX`, `$HOME`, `$LD_LIBRARY_PATH`)
- Shell detection (bash/zsh on Android)
- Package manager (`pkg` commands)
- Storage access (`/sdcard`, `~/storage`)
- Android permissions and sandbox isolation
- Library path preservation
- Helper alias relaunch through wrapped launcher (`apply_patch` must not fail on missing `libc++_shared.so`)
- Browser opener availability
- Architecture detection (aarch64/ARM64)
- Absence of Android-only voice/audio linker dependencies in the published latest build
- Repo-local `termux-tts` skill loading and `termux-tts-speak` invocation

### Linux

Tests validate:
- Standard Linux filesystem structure
- npm installation and binary verification
- Shell execution in typical Linux environment
- Network access and package management

### macOS

Tests validate:
- macOS-specific filesystem paths
- Homebrew integration
- ARM64 compatibility
- Standard Unix tools availability

---

## All Test Reports

See [test-reports/](../test-reports/) directory for all test reports and suites.
