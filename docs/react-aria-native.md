# React Aria Self-Drawn Direction

## Scope

A3S GUI targets semantic parity with every top-level family in the official
React Aria Components catalog.

The versioned source of truth is
[`react-aria-component-matrix.json`](react-aria-component-matrix.json). It
currently pins `react-aria-components` 1.19.0, records all 51 families,
assigns each family to an A3S milestone, maps it to concrete A3S authoring
components, and records self-drawn evidence.

The matrix is executable policy, not a marketing checklist. CI rejects schema
errors, missing families, duplicate mappings, unknown statuses, invalid
evidence, and untracked upstream contract deltas.

## Renderer decision

All components render through the same self-drawn pipeline:

```text
component props + state
          |
          v
  semantic NativeElement tree
     /       |        \
    v        v         v
behavior  accessibility  portable style
    \        |         /
     +--------+--------+
              v
       layout + hit regions
              |
              v
        Graphics scene
              |
              v
        self-drawn pixels
```

No platform content widget may count as component evidence. The deleted
AppKit, GTK4, and WinUI backends are not comparison baselines and are not part
of the conformance plan.

## Status vocabulary

| Status | Meaning |
| --- | --- |
| `planned` | The family has an explicit A3S mapping and milestone but lacks a self-drawn scene gate |
| `scene-smoke` | At least one representative story reaches deterministic layout/scene/software output |
| `conformant` | Every required acceptance dimension passes for the supported contract |

Only `Button` currently has `scene-smoke` evidence through the shared
self-drawn calculator. No family is `conformant`.

## Required invariants

Every family must preserve:

- stable keyed identity across rerenders;
- typed semantic roles, states, values, actions, and relationships;
- controlled and uncontrolled state semantics;
- keyboard, pointer, touch, and assistive activation where applicable;
- focus-visible, focus-within, focus scopes, restoration, and `autoFocus`;
- locale, direction, formatting, and typeahead behavior;
- deterministic layout and hit testing;
- self-drawn interaction states and visual variants;
- a semantic accessibility tree with matching geometry;
- bounded histories, queues, payloads, and diagnostics;
- stale-frame and stale-event rejection;
- identical semantic behavior across macOS, Windows, and Linux hosts.

A component cannot hide an unsupported behavior behind a platform-specific
control.

## Implemented semantic foundation

The current runtime already provides reusable behavior for component families:

### Actions and interaction

- ordered press lifecycle, long press, move, hover, focus, keyboard, clipboard,
  wheel, selection, toggle, and close routing;
- input modality and related-target context;
- disabled, read-only, hidden, inert, required, invalid, selected, checked,
  expanded, and pressed state normalization;
- interaction style variants resolved from mounted state.

### Focus and overlays

- programmatic focus, tab order, focus-visible, focus-within, contained scopes,
  restoration, and post-mount autofocus;
- overlay stack ordering, topmost Escape/outside-press dismissal, modal
  background suppression, close-on-blur, portaled descendants, and restoration;
- typed anchored-overlay placement requests.

### Selection and collections

- stable collection keys, single/multiple selection, `all`, disabled behavior,
  disallow-empty policy, range selection, replacement/toggle modes;
- ListBox, Tree, Tabs, GridList, and Table navigation foundations;
- variable-size page navigation and locale-aware typeahead;
- controlled/uncontrolled selection synchronization.

### Forms and values

- text length enforcement and sensitive-value redaction;
- number parsing/formatting, range clamping, step grids, keyboard/page/bound
  stepping, wheel policy, and localized announcements;
- normalized checkbox/radio/switch/toggle values;
- field labels, descriptions, errors, relationships, and structure metadata.

### Drag and drop

- pointer and keyboard drag sessions;
- multi-item and multi-format payloads;
- MIME wildcard negotiation;
- collection root and keyed before/on/after targets;
- insert, move, reorder, and ordinary drop routing;
- drop activation timing;
- revision-scoped dynamic acceptance/operation policy that fails closed.

OS and cross-application transfer, drag previews, accessibility exposure, and
real host input evidence remain unfinished.

### Accessibility and i18n

- independent name, description, role-description, shortcut, value-text,
  relationship, structure, and state fields;
- live-region diffing with polite/assertive, atomic, relevant, and busy policy;
- semantic focus and selection projection;
- locale/direction inheritance, collation, number/date formatting, and localized
  NumberField messages.

These are portable semantics. Native assistive-technology conformance is not
claimed until concrete zero-widget hosts expose and test them.

## Upstream 1.19 contract deltas

The matrix tracks these explicit deltas:

- GridList and Tree embedded-control keyboard navigation;
- Menu `onAction` delivering both stable item key and action value;
- Popover positioning against an arbitrary target/character rectangle;
- drag/drop matching multiple MIME types and wildcard patterns.

A future upstream version bump must update the matrix and add or revise
executable gates in the same change.

## Known authoring gaps

Eight upstream parts have explicit planned A3S targets:

- `CheckboxField` and `CheckboxButton`;
- `RadioField` and `RadioButton`;
- `SwitchField` and `SwitchButton`;
- `ToastList` and `ToastContent`.

They are tracked in the matrix and cannot disappear into prose-only backlog.

## Milestone groups

### M6 — foundations and forms

Includes foundational content, buttons/toggles, fields, forms, sliders,
meters/progress, toolbars, links, groups, separators, search, file trigger, and
related form semantics.

M6 must establish production text, editing, focus visuals, validation/error
presentation, and common control scene primitives before individual families
can become conformant.

### M7 — overlays and collections

Includes disclosure, modal/dialog/popover/tooltip/toast, menus, select,
combobox/autocomplete, list/grid/tree/table collections, tags, tabs,
virtualization, and drag/drop.

M7 depends on robust overlay geometry, scrolling/clipping, collection layout,
virtualization, drag previews/transfer, and accessibility ownership/action
bridges.

### M8 — date, time, color, and advanced data

Includes calendars, date/time fields and pickers, color controls, and advanced
data interactions. M8 depends on locale-complete segment editing, complex
geometry, high-precision pointer/keyboard control, and mature host text/input
services.

## Conformance dimensions

A family becomes `conformant` only when all dimensions below have versioned
evidence.

| Dimension | Required evidence |
| --- | --- |
| Authoring | Typed Rust component/RSX and target TSX API; defaults and prop validation |
| Behavior | Controlled/uncontrolled state, complete input/focus/action traces, rerender identity |
| Layout/hit | Deterministic geometry, clipping/scrolling, hit regions, direction/scale coverage |
| Scene | Stable draw identity, interaction states, style/theme variants |
| Software pixels | Deterministic golden stories within reviewed tolerances |
| Accessibility | Semantic tree, name/state/value/relationships, geometry, focus/actions, announcements |
| Native host | Same story and traces on real macOS, Windows, and Linux zero-widget hosts |
| Reliability | Bounds, redaction, stale-event rejection, failure/recovery behavior |

Passing headless semantic tests alone is necessary but insufficient.

## Story corpus

Each family should have a shared story set covering:

- default, disabled, read-only, required, invalid, loading, and empty states as
  applicable;
- mouse, touch, pen where available, keyboard, and assistive activation;
- focus-visible and focus-within transitions;
- controlled and uncontrolled rerenders;
- LTR, RTL, locale, scale, constrained size, scroll, and overlay cases;
- theme and interaction visual states;
- accessibility snapshot and action traces;
- deterministic software output and reviewed target GPU capture;
- surface loss, host recovery, and stale-event cases where relevant.

Stories are authored once and consumed by software reference tests and all real
hosts.

## Immediate priorities

1. finish generic text shaping, paragraph layout, text editing, and IME;
2. implement the first concrete zero-widget host and accessibility bridge;
3. close the eight explicit authoring-part gaps;
4. move M6 primitives from semantic-only coverage to scene/pixel evidence;
5. extend the same corpus through M7 and M8;
6. promote status only through matrix-backed evidence.

The complete schedule is in [Roadmap](roadmap.md).
