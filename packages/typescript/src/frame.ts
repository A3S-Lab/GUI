import type {
  ProtocolCompiledNodeV1,
  ProtocolCompiledOrientationV1,
  ProtocolCompiledPropsV1,
  ProtocolCompiledStyleValueV1,
  ProtocolUiActionV1,
  ProtocolUiFrameV1,
  ProtocolWindowOptionsV1,
} from "./generated/protocol.ts";

import {
  A3sJsxError,
  type A3sJsxChild,
  type A3sJsxProps,
  type A3sSourceLocation,
} from "./element.ts";
import {
  requireDraftKey,
  resolveFrameRoot,
  type DraftElement,
  type DraftText,
  type DraftWindow,
} from "./frame-resolver.ts";
import {
  ARRAY_INDEX_NAME,
  EVENT_ALIASES,
  NULLABLE_BOOLEAN_PROPS,
  NUMBER_PROPS,
  PORTABLE_ATTRIBUTE_NAME,
  REQUIRED_BOOLEAN_PROPS,
  RESERVED_WIRE_PROPS,
  STRING_PROPS,
  UNSAFE_PROPERTY_NAMES,
} from "./frame-schema.ts";
import {
  isA3sAction,
  type A3sAction,
  type A3sEventHandler,
} from "./action.ts";
import type { ComponentRenderRuntime } from "./component-runtime.ts";

export interface CompileFrameOptions {
  readonly maximumDepth?: number;
  readonly maximumNodes?: number;
}

export interface CompiledA3sFrameV1 {
  readonly frame: Readonly<ProtocolUiFrameV1>;
  readonly callbacks: ReadonlyMap<string, A3sEventHandler>;
}

interface RegisteredAction {
  readonly wire: ProtocolUiActionV1;
  readonly handler: A3sEventHandler | null;
}

interface CompileState {
  readonly actions: Map<string, RegisteredAction>;
  readonly callbacks: Map<string, A3sEventHandler>;
}

class CallbackSnapshot implements ReadonlyMap<string, A3sEventHandler> {
  readonly #callbacks: ReadonlyMap<string, A3sEventHandler>;

  constructor(callbacks: ReadonlyMap<string, A3sEventHandler>) {
    this.#callbacks = new Map(callbacks);
    Object.freeze(this);
  }

  get size(): number {
    return this.#callbacks.size;
  }

  get(id: string): A3sEventHandler | undefined {
    return this.#callbacks.get(id);
  }

  has(id: string): boolean {
    return this.#callbacks.has(id);
  }

  forEach(
    callback: (
      value: A3sEventHandler,
      key: string,
      map: ReadonlyMap<string, A3sEventHandler>,
    ) => void,
    thisArg?: unknown,
  ): void {
    for (const [id, handler] of this.#callbacks) {
      callback.call(thisArg, handler, id, this);
    }
  }

  entries(): MapIterator<[string, A3sEventHandler]> {
    return this.#callbacks.entries();
  }

  keys(): MapIterator<string> {
    return this.#callbacks.keys();
  }

  values(): MapIterator<A3sEventHandler> {
    return this.#callbacks.values();
  }

  [Symbol.iterator](): MapIterator<[string, A3sEventHandler]> {
    return this.entries();
  }
}

const textEncoder = new TextEncoder();

export function compileFrameV1(
  frameId: string,
  root: A3sJsxChild,
  options: CompileFrameOptions = {},
): CompiledA3sFrameV1 {
  return compileFrameWithRuntimeV1(frameId, root, null, options);
}

export function compileFrameWithRuntimeV1(
  frameId: string,
  root: A3sJsxChild,
  componentRuntime: ComponentRenderRuntime | null,
  options: CompileFrameOptions = {},
): CompiledA3sFrameV1 {
  if (typeof frameId !== "string" || frameId.length === 0) {
    throw new A3sJsxError("A3S frames need a non-empty string frame id");
  }
  const maximumDepth = normalizePositiveLimit(options.maximumDepth, 128, "maximumDepth");
  const maximumNodes = normalizePositiveLimit(options.maximumNodes, 100_000, "maximumNodes");
  const roots = resolveFrameRoot(
    root,
    maximumDepth,
    maximumNodes,
    componentRuntime,
  );

  if (roots.length !== 1) {
    throw new A3sJsxError(
      roots.length === 0
        ? "an A3S frame must resolve to one content element"
        : "a root fragment must resolve to exactly one content element; wrap multiple roots in View",
    );
  }

  const rootDraft = roots[0];
  if (!rootDraft.explicitKey) {
    rootDraft.key = "root";
  }
  const compileState: CompileState = { actions: new Map(), callbacks: new Map() };

  let wireRoot: ProtocolCompiledNodeV1;
  let window: ProtocolWindowOptionsV1 | undefined;
  if (rootDraft.kind === "window") {
    if (!rootDraft.content.explicitKey) {
      rootDraft.content.key = "root";
    }
    wireRoot = compileNode(rootDraft.content, [], compileState);
    window = compileWindow(rootDraft, compileState);
  } else {
    if (rootDraft.kind !== "element") {
      throw new A3sJsxError("an A3S frame root must be a content element", rootDraft.source);
    }
    wireRoot = compileNode(rootDraft, [], compileState);
  }

  const actions = [...compileState.actions.values()].map((entry) => entry.wire);
  const frame: ProtocolUiFrameV1 = window === undefined
    ? { frameId, root: wireRoot, actions }
    : { frameId, root: wireRoot, actions, window };

  return Object.freeze({
    frame: deepFreeze(frame),
    callbacks: new CallbackSnapshot(compileState.callbacks),
  });
}

function compileNode(
  draft: DraftElement | DraftText,
  parentPath: readonly string[],
  state: CompileState,
): ProtocolCompiledNodeV1 {
  const key = requireDraftKey(draft);
  if (draft.kind === "text") {
    return { kind: "text", key, value: draft.value };
  }
  const path = [...parentPath, key];
  const props = compileProps(draft.props, path, draft.source, state);
  const children = draft.children.map((child) => {
    if (child.kind === "window") {
      throw new A3sJsxError("Window is session metadata and can only appear at the root", child.source);
    }
    return compileNode(child, path, state);
  });
  return { kind: "element", key, tag: draft.tag, props, children };
}

function compileProps(
  input: Readonly<A3sJsxProps>,
  hostPath: readonly string[],
  source: A3sSourceLocation | null,
  state: CompileState,
): ProtocolCompiledPropsV1 {
  const props = emptyCompiledProps();
  const attributes = new Map<string, string>();
  const styles = new Map<string, ProtocolCompiledStyleValueV1>();
  const events = new Map<string, unknown>();
  const actionLabels = new Map<string, string>();
  const explicitProps = new Set<string>();

  for (const [name, value] of Object.entries(input)) {
    if (name === "children" || name === "key" || name === "ref" || value === undefined || value === null) {
      continue;
    }
    if (RESERVED_WIRE_PROPS.has(name)) {
      throw new A3sJsxError(
        `JSX prop ${JSON.stringify(name)} is a reserved wire field and cannot bypass normalization`,
        source,
      );
    }
    if (isEventProp(name)) {
      const eventName = normalizeEventName(name);
      assertEventValue(value, eventName, source);
      events.set(eventName, value);
      explicitProps.add(eventName);
      continue;
    }
    if (name === "style") {
      compileStyle(value, styles, source);
      explicitProps.add("style");
      continue;
    }
    if (name === "orientation") {
      if (value !== "horizontal" && value !== "vertical") {
        throw new A3sJsxError(
          "orientation must be \"horizontal\" or \"vertical\"",
          source,
        );
      }
      props.orientation = value as ProtocolCompiledOrientationV1;
      explicitProps.add("orientation");
      continue;
    }

    const stringTarget = STRING_PROPS.get(name);
    if (stringTarget !== undefined) {
      props[stringTarget.field] = requireString(value, name, source);
      explicitProps.add(stringTarget.canonical);
      continue;
    }
    const requiredBooleanTarget = REQUIRED_BOOLEAN_PROPS.get(name);
    if (requiredBooleanTarget !== undefined) {
      const normalized = requireBoolean(value, name, source);
      props[requiredBooleanTarget.field] = normalized;
      explicitProps.add(requiredBooleanTarget.canonical);
      if (requiredBooleanTarget.retainAriaAttribute) {
        attributes.set(name, String(normalized));
      }
      continue;
    }
    const nullableBooleanTarget = NULLABLE_BOOLEAN_PROPS.get(name);
    if (nullableBooleanTarget !== undefined) {
      const normalized = requireBoolean(value, name, source);
      props[nullableBooleanTarget.field] = normalized;
      explicitProps.add(nullableBooleanTarget.canonical);
      if (nullableBooleanTarget.retainAriaAttribute) {
        attributes.set(name, String(normalized));
      }
      continue;
    }
    const numberTarget = NUMBER_PROPS.get(name);
    if (numberTarget !== undefined) {
      const normalized = requireFiniteNumber(value, name, source);
      if (
        (numberTarget.field === "intrinsicWidth" || numberTarget.field === "intrinsicHeight") &&
        (!Number.isInteger(normalized) || normalized < 0 || normalized > 0xffff_ffff)
      ) {
        throw new A3sJsxError(`${name} must be an unsigned 32-bit integer`, source);
      }
      props[numberTarget.field] = normalized;
      explicitProps.add(numberTarget.canonical);
      if (numberTarget.retainAriaAttribute) {
        attributes.set(name, scalarString(normalized));
      }
      continue;
    }

    if (
      !PORTABLE_ATTRIBUTE_NAME.test(name) ||
      ARRAY_INDEX_NAME.test(name) ||
      UNSAFE_PROPERTY_NAMES.has(name)
    ) {
      throw new A3sJsxError(
        `JSX prop ${JSON.stringify(name)} is not a portable attribute name`,
        source,
      );
    }
    attributes.set(name, requirePortableScalar(value, name, source));
    explicitProps.add(name);
  }

  for (const [eventName, value] of [...events.entries()].sort(([left], [right]) => utf8Compare(left, right))) {
    const registered = registerEventAction(
      value,
      eventName,
      hostPath,
      props.isDisabled,
      source,
      state,
    );
    props.events[eventName] = registered.wire.id;
    if (registered.wire.label !== undefined && registered.wire.label !== null) {
      actionLabels.set(registered.wire.id, registered.wire.label);
    }
  }

  props.style = sortedRecord(styles);
  props.attributes = sortedRecord(attributes);
  props.events = sortedRecord(new Map(Object.entries(props.events)));
  props.actionLabels = sortedRecord(actionLabels);
  props.explicitProps = [...explicitProps].sort(utf8Compare);
  return props;
}

function compileWindow(draft: DraftWindow, state: CompileState): ProtocolWindowOptionsV1 {
  const allowed = new Set([
    "children",
    "key",
    "ref",
    "title",
    "onClose",
    "width",
    "height",
    "minWidth",
    "minHeight",
    "maxWidth",
    "maxHeight",
    "resizable",
  ]);
  for (const [name, value] of Object.entries(draft.props)) {
    if (!allowed.has(name) && value !== undefined && value !== null) {
      throw new A3sJsxError(
        `Window prop ${JSON.stringify(name)} is not protocol-1 session metadata`,
        draft.source,
      );
    }
  }

  const title = requireString(draft.props.title, "Window.title", draft.source);
  const dimensions = new Map<string, number>();
  for (const name of [
    "width",
    "height",
    "minWidth",
    "minHeight",
    "maxWidth",
    "maxHeight",
  ] as const) {
    const value = draft.props[name];
    if (value === undefined || value === null) {
      continue;
    }
    const number = requireFiniteNumber(value, `Window.${name}`, draft.source);
    if (number <= 0) {
      throw new A3sJsxError(`Window.${name} must be a positive finite number`, draft.source);
    }
    dimensions.set(name, number);
  }
  validateDimensionBounds(dimensions, "width", "minWidth", "maxWidth", draft.source);
  validateDimensionBounds(dimensions, "height", "minHeight", "maxHeight", draft.source);

  let onClose: string | undefined;
  const onCloseValue = draft.props.onClose;
  if (onCloseValue !== undefined && onCloseValue !== null) {
    assertEventValue(onCloseValue, "onClose", draft.source);
    onClose = registerEventAction(
      onCloseValue,
      "onClose",
      ["$window"],
      false,
      draft.source,
      state,
    ).wire.id;
  }
  const resizableValue = draft.props.resizable;
  const resizable = resizableValue === undefined || resizableValue === null
    ? true
    : requireBoolean(resizableValue, "Window.resizable", draft.source);

  const window: ProtocolWindowOptionsV1 = { title, resizable };
  if (onClose !== undefined) window.onClose = onClose;
  if (dimensions.has("width")) window.width = dimensions.get("width")!;
  if (dimensions.has("height")) window.height = dimensions.get("height")!;
  if (dimensions.has("minWidth")) window.minWidth = dimensions.get("minWidth")!;
  if (dimensions.has("minHeight")) window.minHeight = dimensions.get("minHeight")!;
  if (dimensions.has("maxWidth")) window.maxWidth = dimensions.get("maxWidth")!;
  if (dimensions.has("maxHeight")) window.maxHeight = dimensions.get("maxHeight")!;
  return reorderWindow(window);
}

function registerEventAction(
  value: unknown,
  eventName: string,
  hostPath: readonly string[],
  elementDisabled: boolean,
  source: A3sSourceLocation | null,
  state: CompileState,
): RegisteredAction {
  const explicitAction = typeof value === "function" ? null : value as A3sAction;
  const id = explicitAction?.id ?? automaticActionId(hostPath, eventName);
  if (typeof id !== "string" || id.length === 0 || isArrayIndexName(id)) {
    throw new A3sJsxError(
      `${eventName} action ids must be non-empty, non-array-index strings`,
      source,
    );
  }
  const handler = explicitAction === null ? value as A3sEventHandler : explicitAction.handler;
  const disabled = explicitAction?.disabled ?? elementDisabled;
  const label = explicitAction?.label ?? null;
  const wire: ProtocolUiActionV1 = label === null
    ? { id, disabled }
    : { id, disabled, label };
  const existing = state.actions.get(id);
  if (existing !== undefined) {
    if (
      existing.wire.disabled !== wire.disabled ||
      (existing.wire.label ?? null) !== (wire.label ?? null) ||
      (existing.handler !== null && handler !== null && existing.handler !== handler)
    ) {
      throw new A3sJsxError(
        `action id ${JSON.stringify(id)} is registered with conflicting callback metadata`,
        source,
      );
    }
    if (existing.handler === null && handler !== null) {
      const replacement = { wire: existing.wire, handler };
      state.actions.set(id, replacement);
      state.callbacks.set(id, handler);
      return replacement;
    }
    return existing;
  }
  const registered = { wire, handler };
  state.actions.set(id, registered);
  if (handler !== null) {
    state.callbacks.set(id, handler);
  }
  return registered;
}

function emptyCompiledProps(): ProtocolCompiledPropsV1 {
  return {
    label: null,
    textValue: null,
    value: null,
    placeholder: null,
    action: null,
    ariaLabel: null,
    isDisabled: false,
    isRequired: false,
    isInvalid: false,
    isReadOnly: false,
    isSelected: false,
    isChecked: null,
    isExpanded: null,
    minValue: null,
    maxValue: null,
    valueNumber: null,
    stepValue: null,
    name: null,
    form: null,
    inputType: null,
    accept: null,
    capture: null,
    alt: null,
    href: null,
    src: null,
    srcset: null,
    sizes: null,
    media: null,
    resourceType: null,
    intrinsicWidth: null,
    intrinsicHeight: null,
    loading: null,
    decoding: null,
    fetchPriority: null,
    crossOrigin: null,
    referrerPolicy: null,
    poster: null,
    controls: null,
    autoplay: null,
    loopPlayback: null,
    muted: null,
    playsInline: null,
    preload: null,
    trackKind: null,
    srclang: null,
    trackLabel: null,
    defaultTrack: null,
    list: null,
    dirname: null,
    formAction: null,
    formEnctype: null,
    formMethod: null,
    formTarget: null,
    formNoValidate: null,
    id: null,
    className: null,
    orientation: null,
    style: {},
    attributes: {},
    events: {},
    actionLabels: {},
    explicitProps: [],
  };
}

function compileStyle(
  value: unknown,
  output: Map<string, ProtocolCompiledStyleValueV1>,
  source: A3sSourceLocation | null,
): void {
  if (!isPlainRecord(value)) {
    throw new A3sJsxError("style must be a plain object of portable scalar values", source);
  }
  for (const [name, item] of Object.entries(value)) {
    if (
      name.length === 0 ||
      ARRAY_INDEX_NAME.test(name) ||
      UNSAFE_PROPERTY_NAMES.has(name)
    ) {
      throw new A3sJsxError(`style key ${JSON.stringify(name)} is not portable`, source);
    }
    if (item === null || item === undefined) {
      continue;
    }
    if (typeof item === "number" && !Number.isFinite(item)) {
      throw new A3sJsxError(`style.${name} must be finite`, source);
    }
    if (typeof item !== "string" && typeof item !== "number" && typeof item !== "boolean") {
      throw new A3sJsxError(`style.${name} must be a string, number, or boolean`, source);
    }
    output.set(name, item);
  }
}

function assertEventValue(
  value: unknown,
  eventName: string,
  source: A3sSourceLocation | null,
): asserts value is A3sEventHandler | A3sAction {
  if (typeof value === "function" || isA3sAction(value)) {
    return;
  }
  throw new A3sJsxError(
    `${eventName} must be a function or an action created with defineAction`,
    source,
  );
}

function isEventProp(name: string): boolean {
  return EVENT_ALIASES.has(name) || /^on[A-Z][A-Za-z0-9]*$/u.test(name);
}

function normalizeEventName(name: string): string {
  return EVENT_ALIASES.get(name) ?? name;
}

function automaticActionId(hostPath: readonly string[], eventName: string): string {
  return `a3s:a1:${encodeSegments([...hostPath, eventName])}`;
}

function encodeSegments(segments: readonly string[]): string {
  return segments.map((segment) => `${textEncoder.encode(segment).byteLength}:${segment}`).join("");
}

function normalizePositiveLimit(value: number | undefined, fallback: number, name: string): number {
  if (value === undefined) {
    return fallback;
  }
  if (!Number.isSafeInteger(value) || value < 1) {
    throw new TypeError(`${name} must be a positive safe integer`);
  }
  return value;
}

function requireString(
  value: unknown,
  name: string,
  source: A3sSourceLocation | null,
): string {
  if (typeof value !== "string") {
    throw new A3sJsxError(`${name} must be a string`, source);
  }
  return value;
}

function requireBoolean(
  value: unknown,
  name: string,
  source: A3sSourceLocation | null,
): boolean {
  if (typeof value === "boolean") {
    return value;
  }
  if (value === "true") return true;
  if (value === "false") return false;
  throw new A3sJsxError(`${name} must be a boolean`, source);
}

function requireFiniteNumber(
  value: unknown,
  name: string,
  source: A3sSourceLocation | null,
): number {
  const number = typeof value === "number"
    ? value
    : typeof value === "string" && value.trim().length > 0
      ? Number(value)
      : Number.NaN;
  if (!Number.isFinite(number)) {
    throw new A3sJsxError(`${name} must be a finite number`, source);
  }
  return Object.is(number, -0) ? 0 : number;
}

function requirePortableScalar(
  value: unknown,
  name: string,
  source: A3sSourceLocation | null,
): string {
  if (typeof value === "string" || typeof value === "boolean") {
    return String(value);
  }
  if (typeof value === "number" && Number.isFinite(value)) {
    return scalarString(value);
  }
  throw new A3sJsxError(
    `${name} must be a portable string, finite number, or boolean`,
    source,
  );
}

function scalarString(value: number): string {
  return Object.is(value, -0) ? "0" : String(value);
}

function validateDimensionBounds(
  dimensions: ReadonlyMap<string, number>,
  valueName: string,
  minName: string,
  maxName: string,
  source: A3sSourceLocation | null,
): void {
  const value = dimensions.get(valueName);
  const min = dimensions.get(minName);
  const max = dimensions.get(maxName);
  if (min !== undefined && max !== undefined && min > max) {
    throw new A3sJsxError(`Window.${minName} cannot be greater than Window.${maxName}`, source);
  }
  if (value !== undefined && min !== undefined && value < min) {
    throw new A3sJsxError(`Window.${valueName} cannot be smaller than Window.${minName}`, source);
  }
  if (value !== undefined && max !== undefined && value > max) {
    throw new A3sJsxError(`Window.${valueName} cannot be greater than Window.${maxName}`, source);
  }
}

function reorderWindow(window: ProtocolWindowOptionsV1): ProtocolWindowOptionsV1 {
  const ordered: Partial<ProtocolWindowOptionsV1> & Pick<ProtocolWindowOptionsV1, "title"> = {
    title: window.title,
  };
  if (window.onClose !== undefined && window.onClose !== null) ordered.onClose = window.onClose;
  if (window.width !== undefined && window.width !== null) ordered.width = window.width;
  if (window.height !== undefined && window.height !== null) ordered.height = window.height;
  if (window.minWidth !== undefined && window.minWidth !== null) ordered.minWidth = window.minWidth;
  if (window.minHeight !== undefined && window.minHeight !== null) ordered.minHeight = window.minHeight;
  if (window.maxWidth !== undefined && window.maxWidth !== null) ordered.maxWidth = window.maxWidth;
  if (window.maxHeight !== undefined && window.maxHeight !== null) ordered.maxHeight = window.maxHeight;
  ordered.resizable = window.resizable;
  return ordered as ProtocolWindowOptionsV1;
}

function sortedRecord<Value>(values: ReadonlyMap<string, Value>): Record<string, Value> {
  const record: Record<string, Value> = {};
  for (const [name, value] of [...values.entries()].sort(([left], [right]) => utf8Compare(left, right))) {
    Object.defineProperty(record, name, {
      configurable: true,
      enumerable: true,
      value,
      writable: true,
    });
  }
  return record;
}

function isArrayIndexName(value: string): boolean {
  if (!ARRAY_INDEX_NAME.test(value)) {
    return false;
  }
  const index = Number(value);
  return Number.isInteger(index) && index >= 0 && index < 0xffff_ffff;
}

function utf8Compare(left: string, right: string): number {
  const leftBytes = textEncoder.encode(left);
  const rightBytes = textEncoder.encode(right);
  const count = Math.min(leftBytes.length, rightBytes.length);
  for (let index = 0; index < count; index += 1) {
    if (leftBytes[index] !== rightBytes[index]) {
      return leftBytes[index] - rightBytes[index];
    }
  }
  return leftBytes.length - rightBytes.length;
}

function deepFreeze<Value>(value: Value): Value {
  if (Array.isArray(value)) {
    value.forEach((item) => deepFreeze(item));
    return Object.freeze(value) as Value;
  }
  if (isPlainRecord(value)) {
    Object.values(value).forEach((item) => deepFreeze(item));
    return Object.freeze(value) as Value;
  }
  return value;
}

function isPlainRecord(value: unknown): value is Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}
