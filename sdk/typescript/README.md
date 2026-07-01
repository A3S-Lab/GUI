# @a3s-lab/gui

Web-compatible authoring bridge for `a3s-gui`.

This package provides the protocol-side JSX runtime used to lower Web-compatible
TSX into the serializable `UiFrame` format consumed by the Rust runtime. It is
intentionally small and dependency-free while the compiler integration
stabilizes. React Aria-compatible component names and props are supported, but
the package is a general semantic UI bridge rather than a React Aria-only SDK.

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
  window: {title: 'Profile', width: 640, height: 480},
  actions: [defineAction(saveProfile)],
});
```

When labels are not needed, `createUiFrame` can infer actions from JSX event
props. Use `defineAction` when the host needs action metadata beyond the stable
id.

The runtime accepts React Aria-style state props such as `isDisabled` and
Web-compatible aliases such as `disabled`, `required`, `aria-expanded`,
`aria-selected`, `min`, `max`, and `aria-valuenow`; these normalize to the same
native control-state fields consumed by the Rust renderer. Marker exports
include form and selection components such as `RadioGroup`, `Radio`, `Select`,
`ListBoxItem`, `Dialog`, `Popover`, `Tabs`, `TabList`, `Tab`, `TabPanel`,
`Menu`, `MenuItem`, `Slider`, and `ProgressBar`, plus structural markers such as
`Separator` and `Toolbar`; authors still use Web-compatible semantic JSX instead
of platform widget names.

The emitted frame is plain JSON:

```json
{
  "frameId": "profile",
  "window": {"title": "Profile", "width": 640, "height": 480},
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

## Test

```bash
npm test
```
