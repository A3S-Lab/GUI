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

# Build the native dogfood release artifact for this operating system
release-native:
    #!/usr/bin/env bash
    set -euo pipefail

    case "$(uname -s)" in
        Darwin)
            cargo build --release --features appkit-native --example appkit_dogfood
            ;;
        Linux)
            cargo build --release --features gtk4-native --example gtk4_dogfood
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            cargo build --release --target x86_64-pc-windows-msvc --features winui-native --example winui_dogfood
            ;;
        *)
            echo "unsupported operating system for native GUI release builds: $(uname -s)" >&2
            exit 1
            ;;
    esac

# Stage a native dogfood bundle for this operating system
bundle-native:
    #!/usr/bin/env bash
    set -euo pipefail

    case "$(uname -s)" in
        Darwin)
            just bundle-appkit
            ;;
        Linux)
            just bundle-gtk4
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            just bundle-winui
            ;;
        *)
            echo "unsupported operating system for native GUI bundles: $(uname -s)" >&2
            exit 1
            ;;
    esac

# Stage the macOS AppKit dogfood app bundle
bundle-appkit:
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ "$(uname -s)" != "Darwin" ]]; then
        echo "bundle-appkit must run on macOS" >&2
        exit 1
    fi

    cargo build --release --features appkit-native --example appkit_dogfood

    bundle_dir="target/release/bundle/A3SGuiDogfood.app"
    rm -rf "$bundle_dir"
    mkdir -p "$bundle_dir/Contents/MacOS" "$bundle_dir/Contents/Resources"
    cp target/release/examples/appkit_dogfood "$bundle_dir/Contents/MacOS/A3SGuiDogfood"
    cp packaging/macos/A3SGuiDogfood-Info.plist "$bundle_dir/Contents/Info.plist"
    printf 'APPL????' > "$bundle_dir/Contents/PkgInfo"
    chmod +x "$bundle_dir/Contents/MacOS/A3SGuiDogfood"

    echo "staged $bundle_dir"

# Stage the Linux GTK4 dogfood filesystem bundle
bundle-gtk4:
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ "$(uname -s)" != "Linux" ]]; then
        echo "bundle-gtk4 must run on Linux" >&2
        exit 1
    fi

    cargo build --release --features gtk4-native --example gtk4_dogfood

    bundle_dir="target/release/bundle/a3s-gui-dogfood-linux"
    rm -rf "$bundle_dir"
    mkdir -p "$bundle_dir/usr/bin" "$bundle_dir/usr/share/applications"
    cp target/release/examples/gtk4_dogfood "$bundle_dir/usr/bin/a3s-gui-dogfood"
    cp packaging/linux/a3s-gui-dogfood.desktop "$bundle_dir/usr/share/applications/a3s-gui-dogfood.desktop"
    chmod +x "$bundle_dir/usr/bin/a3s-gui-dogfood"

    echo "staged $bundle_dir"

# Stage the Windows WinUI dogfood bundle
bundle-winui:
    #!/usr/bin/env bash
    set -euo pipefail

    case "$(uname -s)" in
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            ;;
        *)
            echo "bundle-winui must run on Windows with the MSVC toolchain" >&2
            exit 1
            ;;
    esac

    cargo build --release --target x86_64-pc-windows-msvc --features winui-native --example winui_dogfood

    bundle_dir="target/release/bundle/a3s-gui-dogfood-windows"
    rm -rf "$bundle_dir"
    mkdir -p "$bundle_dir"
    cp target/x86_64-pc-windows-msvc/release/examples/winui_dogfood.exe "$bundle_dir/A3SGuiDogfood.exe"
    cp packaging/windows/a3s-gui-dogfood.manifest "$bundle_dir/A3SGuiDogfood.exe.manifest"

    echo "staged $bundle_dir"

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

# Run dogfood reducer and protocol-boundary regression tests
dogfood-regression:
    cargo test --example dogfood_session -- --nocapture

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
