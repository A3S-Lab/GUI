# A3S GUI Roadmap

Updated: 2026-08-01

## Product Direction

A3S GUI is a Rust-native, cross-platform implementation of the interaction,
accessibility, layout, and component model represented by
[React Aria](https://react-aria.adobe.com/getting-started). It does not embed a
browser and does not expose a DOM or CSSOM at runtime.

Application content is moving from three operating-system widget renderers to
one A3S-owned drawing pipeline. The shared
[`a3s-graphics`](https://github.com/A3S-Lab/Graphics) crate owns the versioned
graphics scene, retained frame damage, render preparation, deterministic
software reference renderer, and GPU rendering. Its production GPU backend is
built directly on `wgpu` for Metal, Direct3D 12, and Vulkan. No UI framework
owns the scene, layout, or content renderer.

A3S GUI continues to own the UI-specific layers: RSX, semantic components,
portable style, layout, text editing, hit testing, interaction, focus,
selection, IME coordination, and accessibility. Platform code becomes a thin
host for windows, input, IME, accessibility bridges, menus, dialogs, clipboard,
and presentation. It does not choose component geometry or draw content.

## Planning Rules

- Milestones are dependency ordered. A later prototype cannot compensate for a
  failed earlier contract or parity gate.
- Status is based on executable evidence, not the presence of a type or stub.
- Changes land as small, reviewable commits and are pushed as soon as their
  tests pass. There is no milestone-sized integration branch.
- New work follows the self-drawn path. Legacy control backends are frozen
  except for fixes needed to preserve the migration baseline.
- Dead code is removed with its last consumer. A compatibility module is not
  deleted before an equivalent tested path exists, and it does not remain after
  the cutover gate passes.
- Every visual fixture must lower the shared `NativeElement` tree. A bespoke
  calculator or component-only scene does not count as renderer progress.
- Unsupported style, text, graphics, input, or accessibility fields are
  explicit diagnostics. No layer silently discards a built-in requirement.
- Files over 1,000 lines are split when their area is changed; new files target
  one concern and remain well below that threshold.

| Priority | Scope |
| --- | --- |
| P0 | Architecture cleanup, Graphics GPU foundation, generic layout/scene path, calculator slice, input/IME/accessibility, and legacy backend removal |
| P1 | Full design-system projection, overlays, collections, date/color controls, tables, virtualization, themes, assets, and localization |
| P2 | Developer tooling, animation, advanced content surfaces, performance work, and shared Graphics capabilities needed by future game runtimes |

## Non-Negotiable Architecture

```text
Rust ComponentCx function + optional RSX template
                         |
                         v
                 CompiledRsxNode
                         |
                         v
              semantic NativeElement tree
                 /         |          \
                /          |           \
               v           v            v
       UI layout tree   semantic tree   interaction tree
               |       + accessibility  + hit regions
               v
       a3s-graphics Scene
               |
               v
          FramePlanner
               |
               v
       render preparation
          /           \
         v             v
software reference   wgpu renderer
                         |
            +------------+------------+
            v            v            v
          Metal         DX12        Vulkan
            |            |            |
            +------------+------------+
                         |
                         v
             thin platform window host
                         |
                         v
        normalized input / IME / accessibility
                         |
                         v
              InteractionState -> actions
```

The UI layout tree, paint scene, semantic tree, and interaction tree share
stable element identity but remain separate data products. Paint commands do
not become the accessibility tree, and the graphics engine never infers an
action from a colored rectangle.

### Ownership Boundaries

| Layer | Owns | Must not own |
| --- | --- | --- |
| Authoring and design system | Rust components, RSX, typed props, contracts, variants, tokens, and Stories | GPU resources, native handles, product workflows |
| GUI semantic runtime | `NativeElement`, reducer flow, actions, interaction, focus, selection, overlays, i18n, capabilities, and accessibility | Graphics device, window toolkit state, product I/O |
| GUI layout and scene adapter | portable style resolution, intrinsic measurement requests, layout boxes, paint extraction, hit regions, scene diagnostics | product state, backend-specific GPU calls, OS widget geometry |
| A3S Graphics | scene schema, geometry, draw identity, damage, preparation, resources, shaders, software reference, and GPU rendering | RSX, CSS, widgets, accessibility, IME, game world/ECS, windows |
| Platform host | windows, event loop, raw input, IME, accessibility bridge, clipboard, menus, dialogs, surface attachment, and frame presentation | component layout, style interpretation, application-content drawing |
| Product application | model/messages, effects, data sources, storage, capability broker, ACL loading, theme and asset provisioning | renderer handles, scene mutation after submission, platform branching in components |

Graphics types may flow into the GUI scene adapter and platform presentation
edge. `wgpu` types remain inside `a3s-graphics` and its surface integration.
Thread-affine platform and GPU handles never enter protocol, semantic, or
authoring APIs.

## Cross-Platform Consistency Contract

One scene and one renderer eliminate toolkit layout divergence, but GPU drivers
and host text services can still differ. Consistency is therefore proven at
several layers rather than inferred from one screenshot hash.

### Deterministic Reference Environment

Reference fixtures fix:

- logical viewport, scale, sRGB target, and transparent-background policy
- light/dark theme, density, locale, direction, time zone, and reduced motion
- animation time, focus visibility, input modality, clock, and random seed
- embedded font bytes, fallback order, icons, and raster assets
- Graphics scene schema, shader revision, and snapshot schema

System fonts are forbidden in reference fixtures. Every checked-in asset must
carry its source, license, and checksum.

### Required Evidence

| Layer | Gate |
| --- | --- |
| Semantic | Canonical Native IR and accessibility fingerprints match for the same input |
| Layout | Quantized boxes, baselines, clipping, scroll extents, and z-order match |
| Scene | Ordered primitives, transforms, clips, resources, opacity, and hit identities have the same fingerprint |
| Software image | Repeated clean and incremental renders are byte-identical |
| GPU image | Metal, DX12, and Vulkan output stays within reviewed non-text and text thresholds against the software reference |
| Interaction | The same scenario produces the same actions, focus path, model state, and final scene fingerprint |
| Accessibility | The same supported semantic nodes and actions reach each OS bridge; platform smoke evidence verifies exposure |

Layout drift, different line breaks, missing glyphs, missing assets, or changed
component geometry always fail even when an aggregate image metric passes.

The first required visual fixture remains the calculator at 410 by 620 logical
pixels. Component Stories add fixed viewports after that slice is stable.

## Current Baseline

The repository already provides:

- Rust `ComponentCx` functions, `.rsx` templates, reducers, hooks, routing,
  contracts, and a broad semantic component registry
- compiled RSX, shared Native IR, stable element keys, protocol frames, ordered
  transactions, rollback, ACK validation, and recovery replay
- portable style parsing with Tailwind-compatible utilities and interaction
  variants
- press, hover, focus, selection, collection navigation, overlays, i18n,
  NumberField behavior, and live-region semantics
- accessibility names, descriptions, relationships, states, structure,
  conformance checks, and capability reports
- a shared calculator state model, reducer, RSX component tree, and three
  platform entrypoints
- AppKit, GTK4, and WinUI control backends used as migration baselines

The independent Graphics repository has a versioned scene, stable draw IDs,
canonical fingerprints, retained damage, affine transforms, clipping, opacity,
solid and rounded rectangles, borders, and a deterministic software renderer as
of commit `2cad948`. Its GPU backend is not yet complete and must not be claimed
as production-ready before M2 passes.

## Cleanup Inventory

The existing control renderer remains compatibility code during the cutover.
Its removal gates are explicit so “temporary” code cannot become permanent.

| Current area | Migration use | Removal gate |
| --- | --- | --- |
| `renderer.rs`, `host.rs` | Stable-tree and rollback baseline | New layout/scene renderer preserves keyed state, transaction behavior, and runtime queries |
| `platform/`, `backend/` | Portable command and recovery baseline | Scene frames, resource commits, presentation ACKs, and recovery have equivalent tests |
| `appkit.rs`, `gtk4.rs`, `winui.rs` | Headless widget-planning evidence | Generic scene and capability audits replace class/setter assertions |
| `appkit_native/`, `gtk4_native/`, `winui_native/` | Current real input, IME, accessibility, menu, dialog, and window evidence | Thin hosts cover those services and all three self-drawn calculator lanes pass |
| platform-specific examples | Migration comparison and OS smoke | One shared self-drawn example plus platform-host smoke runners covers the same scenarios |
| legacy Cargo features and dependencies | Build compatibility | Their final source and CI consumer is deleted in the same commit |
| native-input conformance artifacts | Behavioral evidence | Generalized host manifests preserve or strengthen every claimed scenario |

Immediate cleanup includes obsolete renderer plans, unused dependencies,
unreferenced exports, duplicate wrappers, completed TODOs, and docs for paths
that no longer exist. Large active modules are split by concern as their
milestones touch them; tests move beside the extracted concern rather than into
another catch-all file.

## P0 Renderer Program

### M0 - Graphics boundary and deterministic core

Status: Graphics foundation pushed; GUI integration pending.

Deliverables:

- standalone `a3s-graphics` repository with no GUI or window dependency
- accepted Graphics architecture and roadmap
- versioned scene, validation, stable IDs, fingerprints, retained damage, and
  deterministic reference rasterization
- GUI decision record selecting Graphics and rejecting framework-owned drawing

Acceptance gates:

- Graphics default and no-default-feature checks pass on Rust 1.95
- software incremental output equals a clean full repaint
- scene validation rejects duplicate identity, non-finite geometry, invalid
  transforms, invalid scale, and invalid opacity before mutation
- GUI contains no obsolete framework-renderer dependency, module, feature, or
  active architecture plan

### M1 - GUI architecture cleanup and dependency integration

Status: current.

Deliverables:

- replace obsolete architecture, README, and roadmap claims with this boundary
- pin GUI to one reviewed Graphics commit; no floating branch dependency
- define feature boundaries for semantic-only, software-reference, GPU, and
  platform-host builds
- add a compile-time dependency-direction gate
- inventory every `PortableStyle`, `NativeRole`, input, focus, overlay, text,
  and accessibility field needed by the cutover
- remove only code proven to have no current consumer or superseded contract
- split touched 1,000-line modules before adding new responsibilities

Acceptance gates:

- source and dependency searches find no obsolete framework-renderer residue
- `cargo check --no-default-features --lib` remains green
- default tests remain green before and after cleanup
- dependency tooling reports no unused direct dependencies
- deletion commits name the replacement evidence for every removed path

### M2 - Graphics GPU backend

Status: planned after M1.

Deliverables:

- owned `wgpu` device selection and capability report
- surface-independent sRGB render target and asynchronous readback
- WGSL pipelines for fills, rounded rectangles, borders, affine transforms,
  clipping, opacity, and ordered source-over blending
- bounded instance/staging buffers, pipeline cache, and frame diagnostics
- typed adapter absence, device loss, out-of-memory, and surface errors

Acceptance gates:

- shader and pipeline validation passes for Metal, DX12, and Vulkan CI targets
- GPU output matches the software fixtures within reviewed edge-AA thresholds
- transparent overlap preserves command order
- GPU-disabled Graphics builds contain no `wgpu` dependency
- device recreation can replay all resources required by the rectangle fixture

### M3 - Generic layout and scene vertical slice

Status: planned after M2.

Deliverables:

- versioned GUI layout records with stable element identity
- deterministic block, row/column flex, padding, margin, gap, explicit/min/max
  size, alignment, absolute positioning, overflow clipping, and z-order for the
  calculator subset
- `NativeElement -> layout -> Graphics Scene` lowering with diagnostics
- separate hit regions and semantic references keyed to the same elements
- retained layout/scene diff and redraw scheduling
- software and GPU scene snapshots for the existing shared calculator tree

Acceptance gates:

- no calculator-specific visual tree or platform-specific component layout
- the same Native IR produces the same layout and scene fingerprints on all
  three desktop systems
- unsupported required style fails the fixture instead of being omitted
- incremental layout and scene output equals a clean rebuild
- the rectangle-only path presents inside real macOS, Linux, and Windows windows

### M4 - Text, input, IME, accessibility, and overlays

Status: planned after M3.

Deliverables:

- embedded reference fonts, shaping, fallback, line layout, glyph atlas, and
  text selection geometry through Graphics
- platform input normalization and GUI-owned pointer capture, hover, focus,
  keyboard navigation, scrolling, drag, and gesture policy
- text editing with composition ranges, candidate positioning, clipboard,
  caret, selection, password handling, and platform IME bridges
- parallel accessibility tree bridges for macOS, Linux, and Windows
- overlay layout, clipping, stacking, focus containment, dismissal, and system
  menu/dialog exceptions

Acceptance gates:

- the 410x620 calculator passes reference/GPU image, keyboard, pointer, focus,
  and action scenarios on all desktop hosts
- composition tests cover CJK, dead keys, emoji, RTL, selection replacement,
  and cancellation
- screen-reader/UI Automation smoke verifies names, roles, values, states,
  actions, focus, and live regions
- password values never enter scene snapshots, diagnostics, or automation logs

### M5 - Default cutover and legacy deletion

Status: planned after M4.

Deliverables:

- self-drawn renderer becomes the sole application-content path
- one shared calculator, controls, playground, and dogfood entrypoint
- AppKit/GTK4/WinUI widget creation, styling, layout, and update code removed
- thin platform hosts retain only windows, input, IME, accessibility, menus,
  dialogs, clipboard, surface presentation, and automation hooks
- obsolete features, dependencies, examples, packaging entries, CI lanes,
  command schemas, exports, tests, and documentation removed with their last
  consumers

Acceptance gates:

- source search finds no platform widget creation for application content
- default macOS, Linux, and Windows builds open the self-drawn calculator
- the full semantic, interaction, accessibility, reference-image, GPU-image,
  recovery, packaging, and platform smoke matrix passes
- fresh builds prove removed toolkit dependencies are absent from Cargo.lock
  and distribution artifacts
- repository file-size and dead-code audits pass

## P1 Component Projection

P1 starts only after the default cutover. Every component lands through its
semantic behavior, layout, scene, hit testing, accessibility, and visual Story
together.

### M6 - Foundations and forms

- Box/View, text, heading, separators, icons, images, buttons, links, fields,
  text areas, checkboxes, radios, switches, sliders, progress, and meters
- tokens, themes, density, disabled/read-only/invalid states, focus rings, and
  reduced motion
- intrinsic sizing and baseline alignment

### M7 - Overlays, selection, and collections

- dialogs, popovers, tooltips, menus, combo boxes, select, tabs, disclosures,
  toasts, drag/drop, list boxes, grid lists, and trees
- portals/layers, anchored placement, scroll containers, virtualization,
  typeahead, range selection, and collection mutation

### M8 - Date, color, tables, and advanced data

- calendars, date/time/range fields, color controls, tables, data grids,
  column resizing, sorting, large data sets, and localized formatting
- virtualization budgets and stable accessibility semantics for recycled views

## P2 Runtime, Tooling, and Shared Graphics

- typed application message/effect profile and deterministic session replay
- hot reload with transactional scene/resource replacement
- component Stories, inspector, layout/paint diagnostics, and accessibility audit
- animation timelines resolved before scene submission
- image, path, gradient, filter, shadow, and custom canvas primitives
- offscreen targets and custom GPU surfaces for visualization and future games
- Graphics cameras, sprites, meshes, materials, frame graph, particles, and
  compute work remain in the Graphics roadmap and never pull game-world state
  into GUI

## Continuous Integration Matrix

| Lane | Required evidence |
| --- | --- |
| Semantic-only | no-default-feature check, protocol/IR snapshots, reducers, interaction, focus, selection, i18n, and accessibility conformance |
| Graphics software | scene validation, fingerprints, full/incremental parity, reference images, serialization, and fuzz/property checks |
| Graphics GPU | shader validation, headless rendering, readback comparison, cache/resource recovery, and adapter report |
| Linux host | Vulkan window, Wayland/X11 input and IME, accessibility bridge, packaging, and controlled screenshot artifacts |
| macOS host | Metal window, input/IME/accessibility smoke, reference Stories, bundle validation, and lifecycle recovery |
| Windows host | DX12 window, real input injection, IME/UI Automation bridge, reference Stories, packaging, and lifecycle recovery |
| Dependency | pinned Graphics revision, licenses, advisories, unused dependency audit, and semantic-only boundary proof |

## Performance and Reliability Budgets

Budgets are recorded only after representative fixtures have stable
measurements. Required fixtures include the calculator, component playground,
large form, overlay stack, 1,000-row collection, 100,000-row virtualized list,
mixed-script text document, and image-heavy grid.

Metrics include:

- event, reducer, semantic projection, layout, scene extraction, preparation,
  GPU submission, presentation, and accessibility timings
- visited, relaid-out, repainted, and redrawn node counts
- draw commands, batches, vertices, glyphs, uploads, evictions, and dirty area
- CPU allocations, retained memory, GPU memory, queue depth, and high-water marks
- dropped frames, missed frame deadlines, device/surface recovery, and replay
- cold build, incremental build, binary size, and package size

## Risk Controls

| Risk | Control |
| --- | --- |
| Graphics API churn | Exact commit pin in GUI, dedicated upgrade commits, schema compatibility tests |
| GPU driver variance | Software oracle, backend image thresholds, adapter metadata, tri-platform fixtures |
| Framework creep | Window hosts cannot own scene, layout, or application state |
| Game requirements distort GUI | Graphics owns general rendering only; GUI and future game runtime keep separate extraction/state layers |
| Legacy code survives indefinitely | Named removal gate and last-consumer deletion for every compatibility area |
| Premature deletion breaks dogfood | Preserve legacy path until equivalent real-host evidence passes |
| Silent style loss | Required/deferred/unsupported inventory plus fixture-failing diagnostics |
| Text divergence | Embedded fonts, deterministic shaping records, fixed fallback, baseline and line-break gates |
| Accessibility regression | Parallel semantic truth, conformance tests, and real OS bridge smoke |
| Large-module growth | Split-on-touch rule, one concern per module, file-size CI audit |

## Definition of Done

A component or subsystem is complete only when:

- it uses the shared Native IR, layout, scene, Graphics, and platform-host path
- behavior and state transitions are deterministic and tested
- supported style and accessibility fields are projected explicitly
- software reference and required GPU/OS evidence pass
- failure, cancellation, resize, recovery, and teardown are covered
- diagnostics redact sensitive values and explain unsupported capabilities
- documentation and examples match the implemented path
- superseded code, exports, dependencies, tests, fixtures, and docs are deleted
- formatting, clippy, default tests, semantic-only checks, and relevant native
  lanes pass

## Explicit Non-Goals

- No browser, DOM, CSSOM, or WebView application-content renderer.
- No framework-owned content renderer or second renderer state model.
- No platform widget tree for application content after cutover.
- No calculator-specific scene that bypasses shared Native IR.
- No second public reactive store in the renderer.
- No silent fallback from GPU or required styles to a different visual result.
- No game world, ECS, physics, audio, or scripting inside `a3s-graphics`.
- No deletion justified only by a name such as “legacy”; removal requires
  replacement evidence or proof that no consumer remains.

## Immediate Commit Sequence

1. Push the standalone deterministic Graphics core and accepted architecture.
2. Replace the obsolete renderer documentation, pin Graphics, and audit dead residue.
3. Land the Graphics GPU rectangle pipeline with software parity tests.
4. Land generic GUI layout records and Native IR-to-scene rectangle lowering.
5. Present the generic rectangle slice in thin macOS, Linux, and Windows hosts.
6. Add text shaping/rasterization, hit testing, input, IME, and accessibility.
7. Pass the shared calculator cutover matrix.
8. Delete the three legacy application-content widget backends and all final
   consumers in reviewable, platform-scoped commits.
