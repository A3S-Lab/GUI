# @a3s-lab/gui

JSX protocol bridge for `a3s-gui`.

This package provides the protocol-side JSX runtime used to emit the
serializable `UiFrame` format consumed by the Rust runtime. The package has no
runtime dependencies while the compiler integration stabilizes.
React Aria-compatible component names, semantic UI component names, intrinsic
HTML element names, and intrinsic SVG element names are accepted as JSX tags.
`style` can be an object or CSS text string, and `className` is preserved for
the Rust-side Tailwind utility resolver, including variant-prefixed utilities.
CSS text parsing preserves delimiters inside strings, functions, and URLs.

## JSX Runtime

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {Button, createAction, createUiFrame, defineAction} from '@a3s-lab/gui';

const saveProfile = createAction('saveProfile', 'Save profile');

const root = (
  <Button className="primary" onPress={saveProfile}>
    Save
  </Button>
);

export const frame = createUiFrame('profile', root, {
  window: {title: 'Profile', width: 640, height: 480, minWidth: 480},
  actions: [defineAction(saveProfile)],
});
```

When labels are not needed, `createUiFrame` can infer actions from JSX event
props. Use `defineAction` when the host needs action metadata beyond the stable
id. Inferred actions preserve labels from `createAction(id, label)` handlers.
Focus and toggle aliases such as `onFocusChange`, `onToggle`, and
`onExpandedChange` are preserved in the emitted protocol alongside press,
change, selection, and keyboard events.
Rust hosts route explicit `onKeyDown` handlers on the target or its ancestors
first; otherwise Enter and Space key-down events can activate press actions on
buttons, links, and menu items.
They also normalize keyboard activation for stateful controls into toggle or
selection events so action payloads carry checked, expanded, or selected values.

The runtime accepts React Aria-style state props such as `isDisabled` and HTML
or ARIA aliases such as `disabled`, `required`, `aria-expanded`,
`aria-selected`, `min`, `max`, `step`, and `aria-valuenow`; these normalize to
the same native control-state fields consumed by the Rust renderer. ARIA
relationship props such as `aria-labelledby`, `aria-describedby`, and
`aria-controls` are preserved and projected into native accessibility
relationship hints. ARIA description and value props such as `aria-description`,
`aria-roledescription`, `aria-keyshortcuts`, and `aria-valuetext` are projected
into native accessibility description hints. ARIA structure props such as
`aria-level`, `aria-posinset`, `aria-setsize`, `aria-rowindex`, and
`aria-colindex`, plus `aria-sort`, are projected into native accessibility
structure hints. ARIA state and live-region props such as `aria-hidden`,
`aria-autocomplete`, `aria-multiline`, `aria-current`, `aria-pressed`,
`aria-haspopup`, `aria-live`, and `aria-busy` are preserved and projected into
native accessibility state hints.
Intrinsic global and form-control props such as `title`, `hidden`, `lang`, `dir`,
`tabIndex`, `role`, `accessKey`, `contentEditable`, `draggable`, `spellCheck`,
`translate`, `inert`, `popover`, `anchor`, `is`, `nonce`, `readOnly`, `multiple`, `autoFocus`,
`slot`, `part`, `exportParts`, `itemScope`, `itemProp`, `itemType`, `itemID`,
`itemRef`, `autoComplete`, `inputMode`, `enterKeyHint`, `autoCapitalize`,
`autoCorrect`, `virtualKeyboardPolicy`, `pattern`, `minLength`, `maxLength`,
`rows`, `cols`, `size`, dialog `open`, `formAction`, `formEncType`,
`formMethod`, `formTarget`,
`formNoValidate`, `accept`, `capture`, `alt`, `href`, `src`, `srcSet`, `sizes`,
`loading`, `decoding`, `fetchPriority`, `crossOrigin`, `referrerPolicy`,
`poster`, `controls`, `autoPlay`, `playsInline`, `preload`, `srcLang`, `list`,
`dirname`, `colSpan`, `rowSpan`, `headers`, `scope`, `abbr`, `span`, `start`,
`reversed`, list `type`, `li value`, `download`, `ping`, `rel`, `hrefLang`,
link `as`, `integrity`, `blocking`, `nonce`, `imageSrcSet`, `imageSizes`,
script `async`, `defer`, `noModule`, iframe `allow`, `allowFullScreen`,
`sandbox`, `srcDoc`, button `command`, `commandFor`, `popoverTarget`,
`popoverTargetAction`, quote/change `cite`, change `dateTime`, time `dateTime`,
label `htmlFor`, output `for`, and meter `low`, `high`, and `optimum` are
preserved with their Web JSX names and projected by the Rust bridge into native
control, activation, text annotation, form association, and resource policy
hints.
Marker
exports include form and selection components such as `RadioGroup`, `Radio`,
`Select`, `ListBoxItem`, `Dialog`, `Popover`, `Tabs`, `TabList`, `Tab`,
`TabPanel`, `Menu`, `MenuItem`, `Link`, `Slider`, and `ProgressBar`, plus
structural markers such as `Separator` and `Toolbar`. The emitted frame
contains semantic JSX element names.
For intrinsic `input` tags, `type="range"` and `type="number"` normalize
numeric `value` and `defaultValue` props to `valueNumber`.
For intrinsic `textarea` tags, direct text children remain in the compiled
children list and are projected by the Rust bridge as the native text-field
value when no explicit value is supplied.

The emitted frame is plain JSON:

```json
{
  "frameId": "profile",
  "window": {"title": "Profile", "width": 640, "height": 480, "minWidth": 480},
  "actions": [{"id": "saveProfile", "label": "Save profile"}],
  "root": {
    "kind": "element",
    "key": "Button",
    "tag": "Button",
    "props": {"className": "primary", "events": {"onPress": "saveProfile"}},
    "children": [{"kind": "text", "key": "text-0", "value": "Save"}]
  }
}
```

The package also exports protocol types for native render responses, host event
responses, handled native event results, and rendered accessibility trees with
host node ids. `createHandledNativeEvent` mirrors the Rust serde shape for mock
hosts and process-boundary tests.

## Test

```bash
npm test
```
