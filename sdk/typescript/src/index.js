export {
  Fragment,
  jsx,
  jsxs,
  createComponent,
  Button,
  Label,
  Text,
  TextField,
  Input,
  Checkbox,
  Switch,
  RadioGroup,
  Radio,
  Select,
  SelectValue,
  ListBox,
  ListBoxItem,
  Dialog,
  Popover,
  Tabs,
  TabList,
  Tab,
  TabPanel,
  Group,
  Form,
  Menu,
  MenuItem,
  Separator,
  Slider,
  ProgressBar,
  Toolbar,
  Link,
} from './jsx-runtime.js';

export function createAction(id, label) {
  if (typeof id !== 'string' || id.length === 0) {
    throw new Error('a3s-gui actions need a non-empty string id');
  }

  function a3sActionHandler() {}
  Object.defineProperty(a3sActionHandler, 'name', {value: id});
  a3sActionHandler.a3sAction = id;
  if (label != null) {
    a3sActionHandler.a3sLabel = String(label);
  }
  return a3sActionHandler;
}

export const action = createAction;

export function defineAction(actionOrId, label) {
  const id = actionId(actionOrId);
  const resolvedLabel = label ?? actionOrId?.a3sLabel;
  return resolvedLabel == null ? {id} : {id, label: String(resolvedLabel)};
}

export function createUiFrame(frameId, root, options = {}) {
  if (Array.isArray(root)) {
    throw new Error(
      'a3s-gui frames need one root element; wrap fragment children in a Group or form',
    );
  }
  const actions = options.actions ?? collectActions(root);
  return {
    frameId,
    root,
    actions,
    ...(options.window ? {window: options.window} : {}),
  };
}

export function createHostEvent(frameId, node, kind, value) {
  return {
    frameId,
    event: {
      node,
      kind,
      ...(value == null ? {} : {value}),
    },
  };
}

export function createHandledNativeEvent(event, options = {}) {
  return {
    event,
    invocation: options.invocation ?? null,
    interactionChanges: options.interactionChanges ?? [],
  };
}

function actionId(actionOrId) {
  if (typeof actionOrId === 'string' && actionOrId.length > 0) {
    return actionOrId;
  }
  if (typeof actionOrId === 'function') {
    const id = actionOrId.a3sAction ?? actionOrId.name;
    if (typeof id === 'string' && id.length > 0) {
      return id;
    }
  }
  throw new Error(
    'a3s-gui actions need a stable id; use createAction("save") or a named function',
  );
}

function collectActions(root) {
  const actions = new Map();
  for (const id of walkActionIds(root)) {
    if (!actions.has(id)) {
      actions.set(id, {id});
    }
  }
  return [...actions.values()];
}

function* walkActionIds(node) {
  if (Array.isArray(node)) {
    for (const child of node) {
      yield* walkActionIds(child);
    }
    return;
  }
  if (node == null || node.kind !== 'element') {
    return;
  }
  for (const id of Object.values(node.props?.events ?? {})) {
    if (typeof id === 'string' && id.length > 0) {
      yield id;
    }
  }
  for (const child of node.children ?? []) {
    yield* walkActionIds(child);
  }
}
