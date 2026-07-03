# A3S GUI - Justfile

default:
    @just --list

# ============================================================================
# Build
# ============================================================================

# Build the default headless crate
build:
    cargo build

# Build release artifacts for the default headless crate
release:
    cargo build --release

# Check all planning adapters without native OS bindings
check-platforms:
    cargo check --features appkit,winui,gtk4

# Check the native backend that matches this operating system
check-native:
    #!/usr/bin/env bash
    set -euo pipefail

    case "$(uname -s)" in
        Darwin)
            cargo check --features appkit-native --example appkit_dogfood
            ;;
        Linux)
            cargo check --features gtk4-native --example gtk4_dogfood
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            cargo check --features winui-native --example winui_dogfood
            ;;
        *)
            echo "unsupported operating system for native GUI checks: $(uname -s)" >&2
            exit 1
            ;;
    esac

# Check the Windows dogfood target from any host with the target installed
check-winui:
    cargo check --target x86_64-pc-windows-msvc --features winui-native --example winui_dogfood

# ============================================================================
# Test
# ============================================================================

# Run the default Rust test suite
test:
    cargo test

# Run protocol and native runtime examples as tests
test-examples:
    cargo test --examples

# Run adapter planning tests without native OS bindings
test-platforms:
    cargo test --features appkit,winui,gtk4

# Run the TypeScript protocol SDK tests
test-sdk:
    npm test --prefix sdk/typescript

# Run the full local verification suite
verify: fmt-check test test-examples test-platforms test-sdk diff-check

# Run one Rust test filter with output
test-one TEST:
    cargo test {{TEST}} -- --nocapture

# ============================================================================
# Formatting
# ============================================================================

# Format Rust code
fmt:
    cargo fmt --all

# Check Rust formatting
fmt-check:
    cargo fmt --all --check

# Check whitespace in the current git diff
diff-check:
    git diff --check

# ============================================================================
# Examples
# ============================================================================

# Run the headless dogfood session
dogfood:
    cargo run --example dogfood_session

# Run the native dogfood app for this operating system
dogfood-native:
    #!/usr/bin/env bash
    set -euo pipefail

    case "$(uname -s)" in
        Darwin)
            cargo run --features appkit-native --example appkit_dogfood
            ;;
        Linux)
            cargo run --features gtk4-native --example gtk4_dogfood
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            cargo run --features winui-native --example winui_dogfood
            ;;
        *)
            echo "unsupported operating system for native GUI dogfood: $(uname -s)" >&2
            exit 1
            ;;
    esac

# Run the native controls smoke app for this operating system
controls-native:
    #!/usr/bin/env bash
    set -euo pipefail

    case "$(uname -s)" in
        Darwin)
            cargo run --features appkit-native --example appkit_controls
            ;;
        Linux)
            cargo run --features gtk4-native --example gtk4_controls
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            cargo run --features winui-native --example winui_controls
            ;;
        *)
            echo "unsupported operating system for native GUI controls: $(uname -s)" >&2
            exit 1
            ;;
    esac
