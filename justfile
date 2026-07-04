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

# Build, stage, and validate the native dogfood bundle for this operating system
bundle-gate-native: bundle-native check-bundle-native

# Validate the staged native dogfood bundle for this operating system
check-bundle-native:
    #!/usr/bin/env bash
    set -euo pipefail

    case "$(uname -s)" in
        Darwin)
            just check-bundle-appkit
            ;;
        Linux)
            just check-bundle-gtk4
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            just check-bundle-winui
            ;;
        *)
            echo "unsupported operating system for native GUI bundle validation: $(uname -s)" >&2
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
    cp packaging/a3s-gui-dogfood-README.txt "$bundle_dir/Contents/Resources/README.txt"
    printf 'APPL????' > "$bundle_dir/Contents/PkgInfo"
    chmod +x "$bundle_dir/Contents/MacOS/A3SGuiDogfood"

    echo "staged $bundle_dir"

# Validate the staged macOS AppKit dogfood app bundle
check-bundle-appkit:
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ "$(uname -s)" != "Darwin" ]]; then
        echo "check-bundle-appkit must run on macOS" >&2
        exit 1
    fi

    bundle_dir="target/release/bundle/A3SGuiDogfood.app"
    binary="$bundle_dir/Contents/MacOS/A3SGuiDogfood"
    plist="$bundle_dir/Contents/Info.plist"
    pkginfo="$bundle_dir/Contents/PkgInfo"
    readme="$bundle_dir/Contents/Resources/README.txt"

    [[ -d "$bundle_dir/Contents/MacOS" ]] || { echo "missing app MacOS directory: $bundle_dir/Contents/MacOS" >&2; exit 1; }
    [[ -d "$bundle_dir/Contents/Resources" ]] || { echo "missing app Resources directory: $bundle_dir/Contents/Resources" >&2; exit 1; }
    [[ -x "$binary" ]] || { echo "missing executable app binary: $binary" >&2; exit 1; }
    [[ -f "$plist" ]] || { echo "missing app Info.plist: $plist" >&2; exit 1; }
    [[ -f "$pkginfo" ]] || { echo "missing app PkgInfo: $pkginfo" >&2; exit 1; }
    [[ -f "$readme" ]] || { echo "missing app handoff README: $readme" >&2; exit 1; }
    [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$plist")" == "A3SGuiDogfood" ]] || { echo "Info.plist CFBundleExecutable does not match A3SGuiDogfood" >&2; exit 1; }
    [[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundlePackageType' "$plist")" == "APPL" ]] || { echo "Info.plist CFBundlePackageType does not match APPL" >&2; exit 1; }
    [[ "$(cat "$pkginfo")" == "APPL????" ]] || { echo "PkgInfo does not match APPL????" >&2; exit 1; }
    grep -q 'unsigned smoke artifact' "$readme" || { echo "handoff README does not identify an unsigned smoke artifact" >&2; exit 1; }

    echo "validated $bundle_dir"

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
    cp packaging/a3s-gui-dogfood-README.txt "$bundle_dir/README.txt"
    chmod +x "$bundle_dir/usr/bin/a3s-gui-dogfood"

    echo "staged $bundle_dir"

# Validate the staged Linux GTK4 dogfood filesystem bundle
check-bundle-gtk4:
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ "$(uname -s)" != "Linux" ]]; then
        echo "check-bundle-gtk4 must run on Linux" >&2
        exit 1
    fi

    bundle_dir="target/release/bundle/a3s-gui-dogfood-linux"
    binary="$bundle_dir/usr/bin/a3s-gui-dogfood"
    desktop="$bundle_dir/usr/share/applications/a3s-gui-dogfood.desktop"
    readme="$bundle_dir/README.txt"

    [[ -x "$binary" ]] || { echo "missing executable GTK4 dogfood binary: $binary" >&2; exit 1; }
    [[ -f "$desktop" ]] || { echo "missing desktop entry: $desktop" >&2; exit 1; }
    [[ -f "$readme" ]] || { echo "missing GTK4 handoff README: $readme" >&2; exit 1; }
    grep -qx 'Type=Application' "$desktop" || { echo "desktop entry Type is not Application" >&2; exit 1; }
    grep -qx 'Name=A3S GUI Dogfood' "$desktop" || { echo "desktop entry Name does not match A3S GUI Dogfood" >&2; exit 1; }
    grep -qx 'Exec=a3s-gui-dogfood' "$desktop" || { echo "desktop entry Exec does not match a3s-gui-dogfood" >&2; exit 1; }
    grep -q 'unsigned smoke artifact' "$readme" || { echo "handoff README does not identify an unsigned smoke artifact" >&2; exit 1; }

    echo "validated $bundle_dir"

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
    cp packaging/a3s-gui-dogfood-README.txt "$bundle_dir/README.txt"

    echo "staged $bundle_dir"

# Validate the staged Windows WinUI dogfood bundle
check-bundle-winui:
    #!/usr/bin/env bash
    set -euo pipefail

    case "$(uname -s)" in
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            ;;
        *)
            echo "check-bundle-winui must run on Windows with the MSVC toolchain" >&2
            exit 1
            ;;
    esac

    bundle_dir="target/release/bundle/a3s-gui-dogfood-windows"
    binary="$bundle_dir/A3SGuiDogfood.exe"
    manifest="$bundle_dir/A3SGuiDogfood.exe.manifest"
    readme="$bundle_dir/README.txt"

    [[ -s "$binary" ]] || { echo "missing WinUI dogfood executable: $binary" >&2; exit 1; }
    [[ -f "$manifest" ]] || { echo "missing WinUI dogfood manifest: $manifest" >&2; exit 1; }
    [[ -f "$readme" ]] || { echo "missing WinUI handoff README: $readme" >&2; exit 1; }
    grep -q 'name="A3S.GUI.Dogfood"' "$manifest" || { echo "manifest assembly identity does not match A3S.GUI.Dogfood" >&2; exit 1; }
    grep -q '<dpiAwareness' "$manifest" || { echo "manifest is missing dpiAwareness" >&2; exit 1; }
    grep -q 'unsigned smoke artifact' "$readme" || { echo "handoff README does not identify an unsigned smoke artifact" >&2; exit 1; }

    echo "validated $bundle_dir"

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
