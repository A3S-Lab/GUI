A3S GUI Dogfood Bundle
======================

This is an unsigned smoke artifact for the A3S GUI runtime dogfood app.
It is intended for platform QA and developer handoff, not production
distribution.

Run the bundle:

- macOS: open A3SGuiDogfood.app.
- Linux: run usr/bin/a3s-gui-dogfood from this bundle tree.
- Windows: run A3SGuiDogfood.exe.

Check the bundle:

- macOS: inspect Contents/Resources/MANIFEST.txt.
- Linux and Windows: inspect MANIFEST.txt at the bundle root.

The manifest records the source commit plus SHA-256 and byte counts for the
staged bundle files.

The app exercises the native shell, menus, dialogs, keyboard routing, text
inputs, number inputs, toggles, sliders, selects, tabs, scroll containers,
close actions, reducer-driven rerendering, and app loop exit.

Product applications that embed A3S GUI still own product identifiers, icons,
signing, notarization, installers, update metadata, and target-platform QA.
