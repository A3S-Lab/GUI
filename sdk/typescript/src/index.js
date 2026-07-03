export {
  Fragment,
  jsx,
  jsxs,
  createComponent,
  Button,
  Label,
  Document,
  DocumentHead,
  DocumentBody,
  DocumentTitle,
  Metadata,
  ResourceLink,
  StyleSheet,
  Script,
  Template,
  Slot,
  Text,
  Abbreviation,
  Citation,
  Definition,
  DataValue,
  InsertedText,
  DeletedText,
  MarkedText,
  Time,
  Emphasis,
  StrongText,
  Code,
  KeyboardInput,
  SampleOutput,
  Variable,
  InlineQuote,
  Subscript,
  Superscript,
  SmallText,
  BoldText,
  ItalicText,
  StruckText,
  UnderlinedText,
  BidirectionalIsolate,
  BidirectionalOverride,
  Paragraph,
  PreformattedText,
  BlockQuote,
  ContactAddress,
  LineBreak,
  WordBreakOpportunity,
  NoBreakText,
  CenteredText,
  FontText,
  BigText,
  TeletypeText,
  Applet,
  BackgroundSound,
  Frame,
  FrameSet,
  NoEmbedFallback,
  NoFramesFallback,
  Marquee,
  Math,
  NextId,
  SelectedContent,
  Heading,
  HeadingGroup,
  Ruby,
  RubyBase,
  RubyText,
  RubyParenthesis,
  RubyTextContainer,
  Main,
  Navigation,
  Header,
  Footer,
  Article,
  Section,
  Aside,
  Search,
  Disclosure,
  DisclosureSummary,
  Figure,
  FigureCaption,
  DescriptionList,
  DescriptionTerm,
  DescriptionDetails,
  Image,
  Media,
  Canvas,
  EmbeddedContent,
  Link,
  ImageMap,
  ImageMapArea,
  TextField,
  Input,
  Checkbox,
  Switch,
  RadioGroup,
  Radio,
  FieldSet,
  Legend,
  OptionGroup,
  Output,
  Meter,
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
  Table,
  TableSection,
  TableRow,
  TableCell,
  TableColumn,
  TableCaption,
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
  'close',
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
const ACCESSIBILITY_BOOLEAN_FIELDS = [
  'disabled',
  'required',
  'invalid',
  'readOnly',
  'multiple',
  'focused',
  'selected',
];
const ACCESSIBILITY_NULLABLE_BOOLEAN_FIELDS = [
  'checked',
  'expanded',
];
const ACCESSIBILITY_RELATIONSHIP_STRING_FIELDS = [
  'labelledBy',
  'describedBy',
  'details',
  'controls',
  'owns',
  'flowTo',
  'errorMessage',
  'activeDescendant',
];
const ACCESSIBILITY_DESCRIPTION_STRING_FIELDS = [
  'description',
  'roleDescription',
  'keyShortcuts',
  'valueText',
];
const ACCESSIBILITY_STRUCTURE_I32_FIELDS = [
  'positionInSet',
  'setSize',
  'rowCount',
  'rowIndex',
  'columnCount',
  'columnIndex',
];
const ACCESSIBILITY_STRUCTURE_U32_FIELDS = [
  'level',
  'rowSpan',
  'columnSpan',
];
const ACCESSIBILITY_STRUCTURE_STRING_FIELDS = [
  'rowIndexText',
  'columnIndexText',
  'sort',
];
const ACCESSIBILITY_STATE_BOOLEAN_FIELDS = [
  'hidden',
  'multiline',
  'atomic',
  'busy',
  'modal',
];
const ACCESSIBILITY_STATE_STRING_FIELDS = [
  'autocomplete',
  'current',
  'hasPopup',
  'pressed',
  'live',
  'relevant',
];
const PLATFORM_COMMAND_TYPES = new Set([
  'create',
  'update',
  'insertChild',
  'remove',
  'setRoot',
]);

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
  const frameOptions = normalizeFrameOptions(options);
  const window = frameOptions.window === undefined
    ? undefined
    : normalizeWindowOptions(frameOptions.window);
  const actions = normalizeFrameActions(
    frameOptions.actions === undefined
      ? collectActions(root, frameOptions.window)
      : frameOptions.actions,
  );
  return {
    frameId,
    root,
    actions,
    ...(window ? {window} : {}),
  };
}

function normalizeFrameOptions(options) {
  if (options == null || typeof options !== 'object' || Array.isArray(options)) {
    throw new Error('a3s-gui frame options need an object');
  }
  return options;
}

function normalizeWindowOptions(window) {
  if (window == null || typeof window !== 'object' || Array.isArray(window)) {
    throw new Error('a3s-gui window options need an object');
  }
  if (typeof window.title !== 'string') {
    throw new Error('a3s-gui window options need a string title');
  }

  const normalized = {title: window.title};
  if (window.onClose != null) {
    normalized.onClose = actionId(window.onClose);
  }
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
  validateNativeEvent({node, kind, ...(value == null ? {} : {value})}, 'host events');
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
  validateNativeEvent(event, 'native events');
  validateHandledNativeEventOptions(options);
  return {
    event,
    invocation: options.invocation ?? null,
    interactionChanges: options.interactionChanges ?? [],
  };
}

export function createNativeRenderResponse(frameId, root, commands, options = {}) {
  validateResponseFrameId(frameId, 'native render responses');
  validateRootNodeId(root, 'native render responses');
  validatePlatformCommands(commands);
  validateResponseOptions(options, 'native render responses');
  const accessibilityTree = validateOptionalAccessibilityTree(
    options.accessibilityTree,
    'native render responses',
  );
  return {
    frameId,
    root,
    commands,
    ...(options.accessibilityTree === undefined ? {} : {accessibilityTree}),
  };
}

export function createHostEventResponse(frameId, invocation, options = {}) {
  validateResponseFrameId(frameId, 'host event responses');
  validateResponseOptions(options, 'host event responses');
  validateActionInvocation(invocation);
  const interactionChanges = validateOptionalInteractionChanges(
    options.interactionChanges,
    'host event responses',
  );
  return {
    frameId,
    invocation,
    ...(interactionChanges.length === 0 ? {} : {interactionChanges}),
  };
}

export function createNativeHostEventResponse(frameId, options = {}) {
  validateResponseFrameId(frameId, 'native host event responses');
  validateResponseOptions(options, 'native host event responses');
  if (options.invocation != null) {
    validateActionInvocation(options.invocation);
  }
  const accessibilityTree = validateOptionalAccessibilityTree(
    options.accessibilityTree,
    'native host event responses',
  );
  const interactionChanges = validateOptionalInteractionChanges(
    options.interactionChanges,
    'native host event responses',
  );
  return {
    frameId,
    ...(options.invocation === undefined ? {} : {invocation: options.invocation}),
    ...(options.accessibilityTree === undefined ? {} : {accessibilityTree}),
    ...(interactionChanges.length === 0 ? {} : {interactionChanges}),
  };
}

function validateResponseFrameId(frameId, context) {
  if (typeof frameId !== 'string' || frameId.length === 0) {
    throw new Error(`a3s-gui ${context} need a non-empty frame id`);
  }
}

function validateRootNodeId(root, context) {
  if (!Number.isSafeInteger(root) || root <= 0) {
    throw new Error(`a3s-gui ${context} need a positive integer root node id`);
  }
}

function validatePlatformCommands(commands) {
  if (!Array.isArray(commands)) {
    throw new Error('a3s-gui native render response commands need an array');
  }
  for (const command of commands) {
    if (
      command == null ||
      typeof command !== 'object' ||
      Array.isArray(command) ||
      typeof command.type !== 'string' ||
      command.type.length === 0
    ) {
      throw new Error(
        'a3s-gui native render response commands need object commands with non-empty string types',
      );
    }
    if (!PLATFORM_COMMAND_TYPES.has(command.type)) {
      throw new Error(
        'a3s-gui native render response commands need supported native command types',
      );
    }
    validatePlatformCommand(command);
  }
}

function validatePlatformCommand(command) {
  switch (command.type) {
    case 'create':
    case 'update':
      validateCommandNodeId(command.id, `commands.${command.type}.id`);
      validateNativeWidgetBlueprint(command.blueprint, `commands.${command.type}.blueprint`);
      return;
    case 'insertChild':
      validateCommandNodeId(command.parent, 'commands.insertChild.parent');
      validateCommandNodeId(command.child, 'commands.insertChild.child');
      validateCommandIndex(command.index, 'commands.insertChild.index');
      return;
    case 'remove':
    case 'setRoot':
      validateCommandNodeId(command.id, `commands.${command.type}.id`);
      return;
    default:
      return;
  }
}

function validateCommandNodeId(value, context) {
  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new Error(
      `a3s-gui native render response ${context} need positive integer node ids`,
    );
  }
}

function validateCommandIndex(value, context) {
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new Error(
      `a3s-gui native render response ${context} values need non-negative integer numbers`,
    );
  }
}

function validateNativeWidgetBlueprint(blueprint, context) {
  validatePlainObject(blueprint, context);
  for (const field of ['backend', 'widgetClass', 'role', 'accessibilityRole']) {
    validateNonEmptyString(blueprint[field], `${context}.${field}`);
  }
  validatePlainObject(blueprint.controlState, `${context}.controlState`);
  validateStringMap(blueprint.style, `${context}.style`);
  validatePlainObject(blueprint.portableStyle, `${context}.portableStyle`);
  validateStringMap(blueprint.events, `${context}.events`);
  validateStringMap(blueprint.metadata, `${context}.metadata`);
}

function validateStringMap(value, context) {
  validatePlainObject(value, context);
  for (const item of Object.values(value)) {
    if (typeof item !== 'string') {
      throw new Error(`a3s-gui native render response ${context} values need strings`);
    }
  }
}

function validateResponseOptions(options, context) {
  if (options == null || typeof options !== 'object' || Array.isArray(options)) {
    throw new Error(`a3s-gui ${context} options need an object`);
  }
}

function validateOptionalAccessibilityTree(tree, context) {
  if (tree === undefined || tree === null) {
    return tree;
  }
  validateAccessibilityNode(tree, context);
  return tree;
}

function validateAccessibilityNode(node, context) {
  if (node == null || typeof node !== 'object' || Array.isArray(node)) {
    throw new Error(`a3s-gui ${context} accessibilityTree needs an object or null`);
  }
  if (node.node != null && (!Number.isSafeInteger(node.node) || node.node <= 0)) {
    throw new Error(
      `a3s-gui ${context} accessibilityTree node ids need positive integers`,
    );
  }
  if (typeof node.role !== 'string' || node.role.length === 0) {
    throw new Error(
      `a3s-gui ${context} accessibilityTree roles need non-empty strings`,
    );
  }
  validateNullableString(node.label, `${context} accessibilityTree label`);
  validateNullableString(node.value, `${context} accessibilityTree value`);
  if (!Array.isArray(node.children)) {
    throw new Error(`a3s-gui ${context} accessibilityTree children need an array`);
  }
  validateAccessibilityStringRecord(
    node.relationships,
    ACCESSIBILITY_RELATIONSHIP_STRING_FIELDS,
    `${context} accessibilityTree relationships`,
  );
  validateAccessibilityStringRecord(
    node.description,
    ACCESSIBILITY_DESCRIPTION_STRING_FIELDS,
    `${context} accessibilityTree description`,
  );
  validateAccessibilityStructure(node.structure, context);
  validateAccessibilityState(node.state, context);
  for (const field of ACCESSIBILITY_BOOLEAN_FIELDS) {
    validateRequiredBoolean(node[field], `${context} accessibilityTree ${field}`);
  }
  for (const field of ACCESSIBILITY_NULLABLE_BOOLEAN_FIELDS) {
    validateNullableBoolean(node[field], `${context} accessibilityTree ${field}`);
  }
  for (const child of node.children) {
    validateAccessibilityNode(child, context);
  }
}

function validateAccessibilityStringRecord(record, fields, context) {
  validatePlainObject(record, context);
  for (const field of fields) {
    validateNullableString(record[field], `${context}.${field}`);
  }
}

function validateAccessibilityStructure(structure, context) {
  const structureContext = `${context} accessibilityTree structure`;
  validatePlainObject(structure, structureContext);
  for (const field of ACCESSIBILITY_STRUCTURE_I32_FIELDS) {
    validateNullableI32(structure[field], `${structureContext}.${field}`);
  }
  for (const field of ACCESSIBILITY_STRUCTURE_U32_FIELDS) {
    validateNullableU32(structure[field], `${structureContext}.${field}`);
  }
  for (const field of ACCESSIBILITY_STRUCTURE_STRING_FIELDS) {
    validateNullableString(structure[field], `${structureContext}.${field}`);
  }
}

function validateAccessibilityState(state, context) {
  const stateContext = `${context} accessibilityTree state`;
  validatePlainObject(state, stateContext);
  for (const field of ACCESSIBILITY_STATE_BOOLEAN_FIELDS) {
    validateNullableBoolean(state[field], `${stateContext}.${field}`);
  }
  for (const field of ACCESSIBILITY_STATE_STRING_FIELDS) {
    validateNullableString(state[field], `${stateContext}.${field}`);
  }
}

function validatePlainObject(value, context) {
  if (value == null || typeof value !== 'object' || Array.isArray(value)) {
    throw new Error(`a3s-gui ${context} need an object`);
  }
}

function validateRequiredBoolean(value, context) {
  if (typeof value !== 'boolean') {
    throw new Error(`a3s-gui ${context} values need booleans`);
  }
}

function validateNonEmptyString(value, context) {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`a3s-gui native render response ${context} values need non-empty strings`);
  }
}

function validateNullableBoolean(value, context) {
  if (value != null && typeof value !== 'boolean') {
    throw new Error(`a3s-gui ${context} values need booleans or null`);
  }
}

function validateNullableString(value, context) {
  if (value != null && typeof value !== 'string') {
    throw new Error(`a3s-gui ${context} values need strings or null`);
  }
}

function validateNullableI32(value, context) {
  if (value == null) {
    return;
  }
  if (
    !Number.isSafeInteger(value) ||
    value < -0x80000000 ||
    value > 0x7fffffff
  ) {
    throw new Error(`a3s-gui ${context} values need integer numbers or null`);
  }
}

function validateNullableU32(value, context) {
  if (value == null) {
    return;
  }
  if (!Number.isSafeInteger(value) || value < 0 || value > 0xffffffff) {
    throw new Error(
      `a3s-gui ${context} values need unsigned integer numbers or null`,
    );
  }
}

function validateOptionalInteractionChanges(changes, context) {
  if (changes === undefined) {
    return [];
  }
  if (!Array.isArray(changes)) {
    throw new Error(`a3s-gui ${context} interaction changes need an array`);
  }
  for (const change of changes) {
    validateInteractionChange(change);
  }
  return changes;
}

function validateNativeEvent(event, context) {
  if (event == null || typeof event !== 'object' || Array.isArray(event)) {
    throw new Error(`a3s-gui ${context} need an event object`);
  }
  if (!Number.isSafeInteger(event.node) || event.node <= 0) {
    throw new Error(`a3s-gui ${context} need a positive integer node id`);
  }
  if (!HOST_EVENT_KINDS.has(event.kind)) {
    throw new Error(`a3s-gui ${context} need a supported native event kind`);
  }
  if (event.value != null && typeof event.value !== 'string') {
    if (context === 'host events') {
      throw new Error('a3s-gui host event values need to be strings');
    }
    throw new Error(`a3s-gui ${context} native event values need strings`);
  }
}

function validateHandledNativeEventOptions(options) {
  if (options == null || typeof options !== 'object' || Array.isArray(options)) {
    throw new Error('a3s-gui handled native event options need an object');
  }
  if (options.invocation != null) {
    validateActionInvocation(options.invocation);
  }
  const interactionChanges = options.interactionChanges ?? [];
  if (!Array.isArray(interactionChanges)) {
    throw new Error(
      'a3s-gui handled native event interaction changes need an array',
    );
  }
  for (const change of interactionChanges) {
    validateInteractionChange(change);
  }
}

function validateActionInvocation(invocation) {
  if (
    invocation == null
    || typeof invocation !== 'object'
    || Array.isArray(invocation)
  ) {
    throw new Error('a3s-gui handled native event invocations need an object');
  }
  if (!Number.isSafeInteger(invocation.node) || invocation.node <= 0) {
    throw new Error(
      'a3s-gui handled native event invocations need positive integer node ids',
    );
  }
  if (typeof invocation.action !== 'string' || invocation.action.length === 0) {
    throw new Error(
      'a3s-gui handled native event invocations need non-empty string action ids',
    );
  }
  if (!HOST_EVENT_KINDS.has(invocation.event)) {
    throw new Error(
      'a3s-gui handled native event invocations need supported native event kinds',
    );
  }
  if (invocation.value != null && typeof invocation.value !== 'string') {
    throw new Error(
      'a3s-gui handled native event invocation values need strings',
    );
  }
}

function validateInteractionChange(change) {
  if (change == null || typeof change !== 'object' || Array.isArray(change)) {
    throw new Error(
      'a3s-gui handled native event interaction changes need objects',
    );
  }
  if (!Number.isSafeInteger(change.node) || change.node <= 0) {
    throw new Error(
      'a3s-gui handled native event interaction changes need positive integer node ids',
    );
  }
  validateInteractionState(change.before, 'before');
  validateInteractionState(change.after, 'after');
}

function validateInteractionState(state, name) {
  if (state == null || typeof state !== 'object' || Array.isArray(state)) {
    throw new Error(
      `a3s-gui handled native event interaction change ${name} state needs an object`,
    );
  }
  if (typeof state.focused !== 'boolean') {
    throw new Error(
      'a3s-gui handled native event interaction state.focused values need booleans',
    );
  }
  if (state.value != null && typeof state.value !== 'string') {
    throw new Error(
      'a3s-gui handled native event interaction state.value values need strings or null',
    );
  }
  if (typeof state.selected !== 'boolean') {
    throw new Error(
      'a3s-gui handled native event interaction state.selected values need booleans',
    );
  }
  if (state.checked != null && typeof state.checked !== 'boolean') {
    throw new Error(
      'a3s-gui handled native event interaction state.checked values need booleans or null',
    );
  }
  if (state.expanded != null && typeof state.expanded !== 'boolean') {
    throw new Error(
      'a3s-gui handled native event interaction state.expanded values need booleans or null',
    );
  }
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

function collectActions(root, window) {
  const actions = new Map();
  for (const action of walkActions(root)) {
    collectAction(actions, action);
  }
  if (window?.onClose != null) {
    collectAction(actions, defineAction(window.onClose));
  }
  return [...actions.values()];
}

function collectAction(actions, action) {
  const existing = actions.get(action.id);
  if (existing == null || (existing.label == null && action.label != null)) {
    actions.set(action.id, action);
  }
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
