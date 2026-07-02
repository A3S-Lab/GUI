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

const FRAME_ROOT_ERROR =
  'a3s-gui frames need one root element; wrap fragment children in a Group or form';

export function defineAction(actionOrId, label) {
  const id = actionId(actionOrId);
  const resolvedLabel = label ?? actionOrId?.a3sLabel;
  return resolvedLabel == null ? {id} : {id, label: String(resolvedLabel)};
}

export function createUiFrame(frameId, root, options = {}) {
  if (typeof frameId !== 'string' || frameId.length === 0) {
    throw new Error('a3s-gui frames need a non-empty string frame id');
  }
  if (!isCompiledElement(root)) {
    throw new Error(FRAME_ROOT_ERROR);
  }
  const actions = options.actions ?? collectActions(root);
  return {
    frameId,
    root,
    actions,
    ...(options.window ? {window: options.window} : {}),
  };
}

function isCompiledElement(node) {
  return node != null
    && typeof node === 'object'
    && !Array.isArray(node)
    && node.kind === 'element'
    && typeof node.key === 'string'
    && typeof node.tag === 'string';
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
  for (const action of walkActions(root)) {
    const existing = actions.get(action.id);
    if (existing == null || (existing.label == null && action.label != null)) {
      actions.set(action.id, action);
    }
  }
  return [...actions.values()];
}

function* walkActions(node) {
  if (Array.isArray(node)) {
    for (const child of node) {
      yield* walkActions(child);
    }
    return;
  }
  if (node == null || node.kind !== 'element') {
    return;
  }
  const labels = node.props?.actionLabels ?? {};
  for (const id of Object.values(node.props?.events ?? {})) {
    if (typeof id === 'string' && id.length > 0) {
      const label = typeof labels[id] === 'string' && labels[id].length > 0
        ? labels[id]
        : undefined;
      yield label == null ? {id} : {id, label};
    }
  }
  for (const child of node.children ?? []) {
    yield* walkActions(child);
  }
}
