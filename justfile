# A3S GUI - Justfile

default:
    @just --list

# ============================================================================
# Build
# ============================================================================

# Build the default self-drawn crate
build:
    cargo build --locked

# Build release artifacts for the default self-drawn crate
release:
    cargo build --locked --release

# ============================================================================
# Check and test
# ============================================================================

# Run the default Rust test suite
test:
    cargo test --locked

# Compile and test the maintained examples
test-examples:
    cargo test --locked --examples

# Pin every official React Aria component family to an explicit A3S status
test-react-aria-catalog:
    cargo test --locked --test react_aria_component_matrix

# Regenerate the TypeScript wire declarations from versioned Rust DTOs
generate-tsx-protocol:
    cargo run --locked --quiet --no-default-features --features typescript-schema --bin a3s-gui-generate-tsx-protocol -- --write

# Reject checked-in TypeScript declarations that drift from Rust DTOs
check-tsx-protocol:
    cargo run --locked --quiet --no-default-features --features typescript-schema --bin a3s-gui-generate-tsx-protocol -- --check
    cargo test --locked --no-default-features --features typescript-schema --lib tsx_protocol::typescript::

# Type-check automatic TSX and run dependency-free runtime/fixture tests
test-typescript:
    npm --prefix packages/typescript test

# Drive protocol fixtures and real Node createApp through the Rust TSX host
test-tsx-host:
    cargo test --locked --no-default-features --features platform-runtime,software-reference --test tsx_host_process

# Exercise both GUI-to-Graphics renderer boundaries
test-graphics:
    cargo test --locked --no-default-features --features software-reference,gpu --lib drawing::

# Prove the semantic runtime builds without authoring or renderer dependencies
check-core:
    cargo check --locked --no-default-features --lib
    cargo check --locked --no-default-features --features authoring --lib
    cargo check --locked --no-default-features --features graphics --lib
    cargo check --locked --no-default-features --features software-reference --lib
    cargo check --locked --no-default-features --features gpu --lib
    cargo check --locked --no-default-features --features platform-host --lib

# Compile every zero-widget host marker without a content toolkit
check-platform-host:
    cargo check --locked --no-default-features --features platform-host,host-macos,host-windows,host-linux --lib

# Compile the shared self-drawn frame runtime over the zero-widget host edge
check-platform-runtime:
    cargo check --locked --no-default-features --features platform-runtime --lib

# Prove semantic-only builds do not acquire Graphics or an embedded JS runtime
check-core-graph:
    #!/usr/bin/env bash
    set -euo pipefail

    core_graph="$(cargo tree --locked --no-default-features --prefix none)"
    if grep -Eq '^(a3s-graphics|wgpu|napi|napi-derive|neon|neon-build|node-bindgen|node-bindgen-macro|deno_core|rusty_v8) ' <<<"$core_graph"; then
        echo "graphics or embedded JavaScript/Node dependencies entered the semantic-only graph" >&2
        exit 1
    fi

# Prove the H0 contract and raw Windows host graphs contain no renderer or OS content toolkit
check-platform-host-graph:
    #!/usr/bin/env bash
    set -euo pipefail

    host_graph="$(cargo tree --locked --no-default-features --features platform-host --prefix none)"
    if grep -Eq '^(a3s-graphics|wgpu|gtk4|gdk4|gsk4|winio-winui3|windows-collections|objc2-app-kit) ' <<<"$host_graph"; then
        echo "renderer or content-toolkit dependencies entered the H0 platform-host graph" >&2
        exit 1
    fi

    windows_host_graph="$(cargo tree --locked --no-default-features --features host-windows --target x86_64-pc-windows-msvc --prefix none)"
    if grep -Eq '^(a3s-graphics|wgpu|gtk4|gdk4|gsk4|winio-winui3|windows-collections|objc2-app-kit) ' <<<"$windows_host_graph"; then
        echo "renderer or content-toolkit dependencies entered the H2 Windows-host graph" >&2
        exit 1
    fi

# Prove H1 adds Graphics without acquiring a content toolkit
check-platform-runtime-graph:
    #!/usr/bin/env bash
    set -euo pipefail

    runtime_graph="$(cargo tree --locked --no-default-features --features platform-runtime --prefix none)"
    if grep -Eq '^(gtk4|gdk4|gsk4|winio-winui3|objc2-app-kit) ' <<<"$runtime_graph"; then
        echo "content-toolkit dependencies entered the H1 platform-runtime graph" >&2
        exit 1
    fi

    native_tsx_graph="$(cargo tree --locked --no-default-features --features host-windows,platform-runtime,gpu --target x86_64-pc-windows-msvc --prefix none)"
    if grep -Eq '^(gtk4|gdk4|gsk4|winio-winui3|objc2-app-kit) ' <<<"$native_tsx_graph"; then
        echo "content-toolkit dependencies entered the Windows TSX product graph" >&2
        exit 1
    fi

# Run the zero-widget platform-host contract and firewall suites
test-platform-host:
    cargo test --locked --no-default-features --features platform-host --lib platform_host::
    cargo test --locked --no-default-features --features platform-host --test platform_host_firewall

# Exercise atomic H1 frames, lifecycle recovery, firewalls, and shared pixels
test-platform-runtime:
    cargo test --locked --no-default-features --features platform-runtime --lib platform_runtime::
    cargo test --locked --no-default-features --features platform-runtime --test platform_runtime_firewall
    cargo test --locked --no-default-features --features authoring,platform-runtime,software-reference --example self_drawn_calculator

# Exercise real Win32 lifecycle/input plus H1-to-H2 and DX12 presentation on Windows
test-windows-host:
    cargo test --locked --no-default-features --features host-windows --lib platform_host::windows::input::pointer::tests -- --test-threads=1
    cargo test --locked --no-default-features --features host-windows --test windows_pointer_input -- --test-threads=1
    cargo test --locked --no-default-features --features host-windows --test windows_pen_input -- --test-threads=1
    cargo test --locked --no-default-features --features host-windows --test windows_platform_host -- --test-threads=1
    cargo test --locked --no-default-features --features host-windows,platform-runtime --test windows_platform_host shared_self_drawn_runtime_commits_into_the_real_hidden_win32_host -- --exact --test-threads=1
    cargo test --locked --no-default-features --features host-windows,platform-runtime,gpu --test windows_platform_host graphics_presenter_draws_and_presents_the_first_real_win32_frame -- --exact --test-threads=1
    cargo test --locked --no-default-features --features host-windows,platform-runtime,gpu-fault-injection --test windows_lifecycle -- --test-threads=1
    cargo test --locked --no-default-features --features host-windows,platform-runtime,gpu --test windows_tsx_host -- --test-threads=1
    cargo test --locked --no-default-features --features host-windows,platform-runtime,gpu,software-reference --test windows_calculator_capture -- --test-threads=1
    cargo test --locked --no-default-features --features host-windows --test platform_host_firewall -- --test-threads=1

# Lint every maintained target and deny high-confidence warnings
clippy:
    cargo clippy --locked --all-targets --features gpu,platform-host,platform-runtime,typescript-schema -- -A clippy::all -D clippy::correctness -D clippy::suspicious -A clippy::unnecessary_get_then_check -D unused

# Build crate documentation and fail on rustdoc warnings
doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps --document-private-items
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-default-features --features platform-host --no-deps --document-private-items
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-default-features --features platform-runtime --no-deps --document-private-items
    RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-default-features --features typescript-schema --no-deps --document-private-items

# Run the full local verification suite
verify: fmt-check check-core check-core-graph check-platform-host check-platform-host-graph check-platform-runtime check-platform-runtime-graph check-tsx-protocol clippy doc-check test test-examples test-react-aria-catalog test-typescript test-tsx-host test-platform-host test-platform-runtime test-graphics diff-check

# Run dogfood reducer and protocol-boundary regression tests
dogfood-regression:
    cargo test --locked --example dogfood_session -- --nocapture

# Run one Rust test filter with output
test-one TEST:
    cargo test --locked {{ TEST }} -- --nocapture

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

# Run the headless semantic dogfood session
dogfood:
    cargo run --locked --example dogfood_session

# Run the semantic component catalog without a platform content toolkit
playground:
    cargo run --locked --features authoring --example component_playground

# Run the shared self-drawn calculator pipeline
self-drawn-calculator:
    cargo run --locked --no-default-features --features authoring,platform-runtime,software-reference --example self_drawn_calculator
