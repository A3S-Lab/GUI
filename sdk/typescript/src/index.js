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
  const actions = normalizeFrameActions(options.actions ?? collectActions(root));
  const window = options.window == null ? undefined : normalizeWindowOptions(options.window);
  return {
    frameId,
    root,
    actions,
    ...(window ? {window} : {}),
  };
}

function normalizeWindowOptions(window) {
  if (window == null || typeof window !== 'object' || Array.isArray(window)) {
    throw new Error('a3s-gui window options need an object');
  }
  if (typeof window.title !== 'string') {
    throw new Error('a3s-gui window options need a string title');
  }

  const normalized = {title: window.title};
  for (const property of ['width', 'height', 'minWidth', 'minHeight', 'maxWidth', 'maxHeight']) {
    if (window[property] == null) continue;
    const value = window[property];
    if (typeof value !== 'number' || !Number.isFinite(value) || value <= 0) {
      throw new Error(`a3s-gui window.${property} must be a positive finite number`);
    }
    normalized[property] = value;
  }
  if (window.resizable != null) {
    if (typeof window.resizable !== 'boolean') {
      throw new Error('a3s-gui window.resizable must be a boolean');
    }
    normalized.resizable = window.resizable;
  }

  validateWindowDimensionBounds(normalized, 'width', 'minWidth', 'maxWidth');
  validateWindowDimensionBounds(normalized, 'height', 'minHeight', 'maxHeight');
  return normalized;
}

function validateWindowDimensionBounds(window, valueName, minName, maxName) {
  if (window[minName] != null && window[maxName] != null && window[minName] > window[maxName]) {
    throw new Error(`a3s-gui window.${minName} cannot be greater than window.${maxName}`);
  }
  if (window[valueName] != null && window[minName] != null && window[valueName] < window[minName]) {
    throw new Error(`a3s-gui window.${valueName} cannot be smaller than window.${minName}`);
  }
  if (window[valueName] != null && window[maxName] != null && window[valueName] > window[maxName]) {
    throw new Error(`a3s-gui window.${valueName} cannot be greater than window.${maxName}`);
  }
}

function normalizeFrameActions(actions) {
  if (!Array.isArray(actions)) {
    throw new Error('a3s-gui frame actions need an array');
  }
  return actions.map((action) => {
    if (action == null || typeof action.id !== 'string' || action.id.length === 0) {
      throw new Error('a3s-gui frame actions need non-empty string ids');
    }
    return action.label == null
      ? {id: action.id}
      : {id: action.id, label: String(action.label)};
  });
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
