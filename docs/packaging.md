# Native Packaging

A3S GUI is a runtime crate, not a product packager. The crate still needs a
repeatable path from "native dogfood app compiles" to "a developer can hand a
native artifact to another person." This document defines that path for the
current AppKit, GTK4, and WinUI dogfood applications.

The recipes below stage unsigned dogfood artifacts under `target/release/bundle`.
They are release smoke artifacts, not final installers. Product repositories
should copy the same pattern and then add product identifiers, icons, signing,
notarization, installers, and update metadata.

## Release Gate

Run these commands from `crates/gui` before creating a native artifact:

```bash
just verify
just dogfood-regression
just check-native
just release-native
just bundle-native
just check-bundle-native
```

`just release-native` builds the dogfood example for the current operating
system:

| Host | Release artifact |
|------|------------------|
| macOS | `target/release/examples/appkit_dogfood` |
| Linux | `target/release/examples/gtk4_dogfood` |
| Windows | `target/x86_64-pc-windows-msvc/release/examples/winui_dogfood.exe` |

Use `just bundle-native` to stage the matching host bundle. Use
`just check-bundle-native` after staging, or `just bundle-gate-native` to build,
stage, and validate in one step. The validation recipes check the staged file
layout, executable payload, and platform metadata that this crate owns.

## macOS AppKit

Prerequisites:

- Xcode Command Line Tools.
- A signing identity only when distributing outside local development.

Build and stage the unsigned app bundle:

```bash
just bundle-appkit
just check-bundle-appkit
```

Output:

```text
target/release/bundle/A3SGuiDogfood.app
```

The staged bundle uses
[`packaging/macos/A3SGuiDogfood-Info.plist`](../packaging/macos/A3SGuiDogfood-Info.plist)
and copies the release example binary into `Contents/MacOS/A3SGuiDogfood`.
`just check-bundle-appkit` verifies the executable, `Info.plist`, `PkgInfo`,
and required AppKit bundle metadata.

For distribution, the application owner must sign and notarize the bundle with
their own identity and bundle identifier:

```bash
codesign --force --deep --options runtime --sign "Developer ID Application: Example" \
  target/release/bundle/A3SGuiDogfood.app
xcrun notarytool submit A3SGuiDogfood.zip --wait
xcrun stapler staple target/release/bundle/A3SGuiDogfood.app
```

The crate does not currently generate a `.dmg` or `.pkg`.

## Linux GTK4

Prerequisites:

- GTK4 development libraries.
- `pkg-config`.
- A Linux package toolchain only when producing distro packages.

Typical development packages:

```bash
# Debian / Ubuntu
sudo apt-get install libgtk-4-dev pkg-config

# Fedora
sudo dnf install gtk4-devel pkgconf-pkg-config
```

Build and stage a filesystem bundle:

```bash
just bundle-gtk4
just check-bundle-gtk4
```

Output:

```text
target/release/bundle/a3s-gui-dogfood-linux/
|-- usr/bin/a3s-gui-dogfood
`-- usr/share/applications/a3s-gui-dogfood.desktop
```

The desktop entry comes from
[`packaging/linux/a3s-gui-dogfood.desktop`](../packaging/linux/a3s-gui-dogfood.desktop).
The staged tree is suitable input for a later `.deb`, `.rpm`, AppImage, or
Flatpak pipeline. It is not itself an installer.
`just check-bundle-gtk4` verifies the executable payload and desktop entry
fields owned by this crate.

## Windows WinUI

Prerequisites:

- Windows with the MSVC Rust toolchain.
- Visual Studio Build Tools or Visual Studio with the Desktop C++ workload.
- Windows App SDK runtime compatible with the `winio-winui3` dependency.

Install the target explicitly when the host has multiple Rust Windows
toolchains:

```bash
rustup target add x86_64-pc-windows-msvc
```

Build and stage the WinUI dogfood binary:

```bash
just bundle-winui
just check-bundle-winui
```

Output:

```text
target/release/bundle/a3s-gui-dogfood-windows/
|-- A3SGuiDogfood.exe
`-- A3SGuiDogfood.exe.manifest
```

The manifest comes from
[`packaging/windows/a3s-gui-dogfood.manifest`](../packaging/windows/a3s-gui-dogfood.manifest).
It is staged as sidecar metadata for local release smoke testing. A real Windows
application should embed the manifest, add product resources, and publish
through MSIX, MSI, winget, or another installer path owned by the product.
`just check-bundle-winui` verifies the executable payload and sidecar manifest
fields owned by this crate.

Non-Windows hosts can still run API checks for WinUI when the Rust target is
installed:

```bash
just check-winui
```

That check does not replace a Windows release build.

## Product Responsibilities

Applications that embed A3S GUI still own:

- product names, bundle identifiers, icons, and version metadata
- code signing identities, entitlements, notarization, and installer signing
- installer formats such as `.dmg`, `.pkg`, `.deb`, `.rpm`, AppImage, Flatpak,
  MSIX, MSI, or winget manifests
- update channels and rollback policy
- native-platform QA on real target operating systems

The crate-level recipes are intentionally small. They prove that each native
backend can produce a release artifact and give product repositories a stable
starting point for their own packaging automation.
