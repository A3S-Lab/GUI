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
const HOST_EVENT_KINDS = new Set([
  'press',
  'change',
  'selectionChange',
  'toggle',
  'focus',
  'blur',
  'keyDown',
  'keyUp',
]);
const COMPILED_STRING_PROPS = [
  'label',
  'textValue',
  'value',
  'placeholder',
  'action',
  'ariaLabel',
  'aria-label',
  'name',
  'form',
  'inputType',
  'accept',
  'capture',
  'alt',
  'href',
  'src',
  'srcset',
  'sizes',
  'media',
  'resourceType',
  'loading',
  'decoding',
  'fetchPriority',
  'crossOrigin',
  'referrerPolicy',
  'poster',
  'preload',
  'trackKind',
  'srclang',
  'trackLabel',
  'list',
  'dirname',
  'formAction',
  'formEnctype',
  'formMethod',
  'formTarget',
  'id',
  'className',
];
const COMPILED_BOOLEAN_PROPS = [
  'isDisabled',
  'isRequired',
  'isInvalid',
  'isReadOnly',
  'isSelected',
];
const COMPILED_NULLABLE_BOOLEAN_PROPS = [
  'isChecked',
  'isExpanded',
  'controls',
  'autoplay',
  'loopPlayback',
  'muted',
  'playsInline',
  'defaultTrack',
  'formNoValidate',
];
const COMPILED_NUMBER_PROPS = [
  'minValue',
  'maxValue',
  'valueNumber',
  'stepValue',
];
const COMPILED_U32_PROPS = [
  'intrinsicWidth',
  'intrinsicHeight',
];

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
  validateCompiledNode(root);
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
  const seen = new Set();
  return actions.map((action) => {
    if (action == null || typeof action.id !== 'string' || action.id.length === 0) {
      throw new Error('a3s-gui frame actions need non-empty string ids');
    }
    if (seen.has(action.id)) {
      throw new Error(`a3s-gui frame actions need unique ids; duplicate action ${action.id}`);
    }
    seen.add(action.id);
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

function validateCompiledNode(node) {
  if (node == null || typeof node !== 'object' || Array.isArray(node)) {
    throw new Error('a3s-gui compiled children need element or text nodes');
  }
  if (node.kind === 'element') {
    if (typeof node.key !== 'string' || node.key.length === 0) {
      throw new Error('a3s-gui compiled elements need non-empty string keys');
    }
    if (typeof node.tag !== 'string' || node.tag.length === 0) {
      throw new Error('a3s-gui compiled elements need non-empty string tags');
    }
    validateCompiledProps(node.props);
    validateCompiledChildren(node.children === undefined ? [] : node.children);
    return;
  }
  if (node.kind === 'text') {
    if (typeof node.key !== 'string' || node.key.length === 0) {
      throw new Error('a3s-gui compiled text nodes need non-empty string keys');
    }
    if (typeof node.value !== 'string') {
      throw new Error('a3s-gui compiled text nodes need string values');
    }
    return;
  }
  throw new Error('a3s-gui compiled children need element or text nodes');
}

function validateCompiledProps(props) {
  if (props === undefined) {
    return;
  }
  if (props == null || typeof props !== 'object' || Array.isArray(props)) {
    throw new Error('a3s-gui compiled props need an object');
  }
  validateCompiledStringProps(props);
  validateCompiledBooleanProps(props);
  validateCompiledNullableBooleanProps(props);
  validateCompiledNumberProps(props);
  validateCompiledU32Props(props);
  validateCompiledOrientation(props.orientation);
  validateCompiledStringMap(props.attributes, 'attributes');
  validateCompiledStringMap(props.events, 'events');
  validateCompiledStringMap(props.actionLabels, 'actionLabels');
  validateCompiledStyleMap(props.style);
}

function validateCompiledStringProps(props) {
  for (const name of COMPILED_STRING_PROPS) {
    const value = props[name];
    if (value == null) {
      continue;
    }
    if (typeof value !== 'string') {
      throw new Error(`a3s-gui compiled props.${name} values need strings`);
    }
  }
}

function validateCompiledBooleanProps(props) {
  for (const name of COMPILED_BOOLEAN_PROPS) {
    const value = props[name];
    if (value === undefined) {
      continue;
    }
    if (typeof value !== 'boolean') {
      throw new Error(`a3s-gui compiled props.${name} values need booleans`);
    }
  }
}

function validateCompiledNullableBooleanProps(props) {
  for (const name of COMPILED_NULLABLE_BOOLEAN_PROPS) {
    const value = props[name];
    if (value == null) {
      continue;
    }
    if (typeof value !== 'boolean') {
      throw new Error(`a3s-gui compiled props.${name} values need booleans`);
    }
  }
}

function validateCompiledNumberProps(props) {
  for (const name of COMPILED_NUMBER_PROPS) {
    const value = props[name];
    if (value == null) {
      continue;
    }
    if (typeof value !== 'number' || !Number.isFinite(value)) {
      throw new Error(`a3s-gui compiled props.${name} values need finite numbers`);
    }
  }
}

function validateCompiledU32Props(props) {
  for (const name of COMPILED_U32_PROPS) {
    const value = props[name];
    if (value == null) {
      continue;
    }
    if (
      typeof value !== 'number'
      || !Number.isInteger(value)
      || value < 0
      || value > 0xffffffff
    ) {
      throw new Error(
        `a3s-gui compiled props.${name} values need unsigned integer numbers`,
      );
    }
  }
}

function validateCompiledOrientation(orientation) {
  if (orientation == null) {
    return;
  }
  if (orientation !== 'horizontal' && orientation !== 'vertical') {
    throw new Error(
      'a3s-gui compiled props.orientation values need horizontal or vertical',
    );
  }
}

function validateCompiledStringMap(values, name) {
  if (values === undefined) {
    return;
  }
  if (values == null || typeof values !== 'object' || Array.isArray(values)) {
    throw new Error(`a3s-gui compiled props.${name} need an object`);
  }
  for (const value of Object.values(values)) {
    if (typeof value !== 'string') {
      throw new Error(`a3s-gui compiled props.${name} values need strings`);
    }
  }
}

function validateCompiledStyleMap(style) {
  if (style === undefined) {
    return;
  }
  if (style == null || typeof style !== 'object' || Array.isArray(style)) {
    throw new Error('a3s-gui compiled props.style need an object');
  }
  for (const value of Object.values(style)) {
    const valueType = typeof value;
    if (valueType === 'number' && !Number.isFinite(value)) {
      throw new Error('a3s-gui compiled props.style values need finite numbers');
    }
    if (
      valueType !== 'string'
      && valueType !== 'number'
      && valueType !== 'boolean'
    ) {
      throw new Error(
        'a3s-gui compiled props.style values need strings, numbers, or booleans',
      );
    }
  }
}

function validateCompiledChildren(children) {
  if (!Array.isArray(children)) {
    throw new Error('a3s-gui compiled elements need array children');
  }
  const seen = new Set();
  for (const child of children) {
    validateCompiledNode(child);
    if (seen.has(child.key)) {
      throw new Error(`a3s-gui compiled sibling nodes need unique keys; duplicate key ${child.key}`);
    }
    seen.add(child.key);
  }
}

export function createHostEvent(frameId, node, kind, value) {
  if (typeof frameId !== 'string' || frameId.length === 0) {
    throw new Error('a3s-gui host events need a non-empty frame id');
  }
  if (!Number.isSafeInteger(node) || node <= 0) {
    throw new Error('a3s-gui host events need a positive integer node id');
  }
  if (!HOST_EVENT_KINDS.has(kind)) {
    throw new Error('a3s-gui host events need a supported native event kind');
  }
  if (value != null && typeof value !== 'string') {
    throw new Error('a3s-gui host event values need to be strings');
  }
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
