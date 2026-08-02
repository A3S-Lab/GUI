# Packaging Gate

## Status

Native packaging is intentionally disabled.

The old unsigned bundles were built around deleted platform content-control
examples, so their manifests, scripts, desktop files, CI jobs, and archive
validators were removed. Keeping them would imply that A3S currently ships a
native application when it does not.

Packaging returns only after a real zero-widget host produces a self-drawn
artifact.

## Re-entry requirements

Before adding any platform bundle, the corresponding host must pass:

- concrete window and Graphics-surface lifecycle;
- real self-drawn presentation;
- resize, scale, close, and surface-loss recovery;
- pointer, keyboard, wheel, text/IME, and accessibility smoke;
- deterministic reference-story parity;
- dependency firewall with no content-widget toolkit;
- release-mode build on the target operating system.

A packaging PR must not add a temporary toolkit renderer.

## Required artifacts

### macOS

The eventual artifact is a signed application bundle containing:

- one self-drawn executable;
- bundle metadata, icon, entitlements, and privacy declarations;
- required dynamic libraries or embedded resources;
- an explicit minimum OS version;
- notarization evidence.

Application content must be presented by the A3S Graphics surface.

### Windows

The eventual artifact is a signed self-drawn executable or installer
containing:

- application identity, icon, version metadata, and DPI declarations;
- required runtime libraries;
- installer/uninstaller metadata if an installer is produced;
- code-signing evidence.

It must not require WinUI/XAML or the Windows App SDK as a content renderer.

### Linux

The eventual artifacts may include an archive and distribution packages with:

- a Wayland-first executable and explicitly gated X11 fallback;
- desktop entry, icons, MIME metadata, and declared runtime dependencies;
- reproducible file manifest and checksums;
- package-manager metadata appropriate to each target distribution.

They must not depend on GTK4, GDK, or GSK.

## Bundle validation

Every produced artifact must be checked for:

- exact expected files and no undeclared extras;
- executable identity and version consistency;
- deterministic manifest and checksums;
- license/notice content;
- forbidden dynamic dependencies;
- launch/close smoke on a clean target image;
- renderer story capture and event/accessibility smoke;
- signing/notarization status appropriate to the channel.

## CI shape

When concrete hosts exist, packaging jobs should run after the portable
`just verify` gate and matching host tests. Artifacts are uploaded only from
successful target-native jobs.

The current CI deliberately has one portable verification job and publishes no
native bundle.
