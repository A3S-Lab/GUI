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
  Fragment,
  A3sJsxError,
  describeElementType,
  isA3sElement,
  type A3sElement,
  type A3sJsxChild,
  type A3sJsxProps,
  type A3sSourceLocation,
} from "./element.ts";
import {
  isA3sAction,
  type A3sAction,
  type A3sEventHandler,
} from "./action.ts";

export interface CompileFrameOptions {
  readonly maximumDepth?: number;
  readonly maximumNodes?: number;
}

export interface CompiledA3sFrameV1 {
  readonly frame: Readonly<ProtocolUiFrameV1>;
  readonly callbacks: ReadonlyMap<string, A3sEventHandler>;
}

type NullableStringField =
  | "label"
  | "textValue"
  | "value"
  | "placeholder"
  | "action"
  | "ariaLabel"
  | "name"
  | "form"
  | "inputType"
  | "accept"
  | "capture"
  | "alt"
  | "href"
  | "src"
  | "srcset"
  | "sizes"
  | "media"
  | "resourceType"
  | "loading"
  | "decoding"
  | "fetchPriority"
  | "crossOrigin"
  | "referrerPolicy"
  | "poster"
  | "preload"
  | "trackKind"
  | "srclang"
  | "trackLabel"
  | "list"
  | "dirname"
  | "formAction"
  | "formEnctype"
  | "formMethod"
  | "formTarget"
  | "id"
  | "className";

type RequiredBooleanField =
  | "isDisabled"
  | "isRequired"
  | "isInvalid"
  | "isReadOnly"
  | "isSelected";

type NullableBooleanField =
  | "isChecked"
  | "isExpanded"
  | "controls"
  | "autoplay"
  | "loopPlayback"
  | "muted"
  | "playsInline"
  | "defaultTrack"
  | "formNoValidate";

type NullableNumberField =
  | "minValue"
  | "maxValue"
  | "valueNumber"
  | "stepValue"
  | "intrinsicWidth"
  | "intrinsicHeight";

interface PropTarget<Field extends string> {
  readonly field: Field;
  readonly canonical: string;
  readonly retainAriaAttribute?: boolean;
}

interface DraftBase {
  key: string | null;
  explicitKey: boolean;
  readonly source: A3sSourceLocation | null;
}

interface DraftText extends DraftBase {
  readonly kind: "text";
  readonly value: string;
}

interface DraftElement extends DraftBase {
  readonly kind: "element";
  readonly tag: string;
  readonly props: Readonly<A3sJsxProps>;
  readonly children: DraftNode[];
}

interface DraftWindow extends DraftBase {
  readonly kind: "window";
  readonly props: Readonly<A3sJsxProps>;
  readonly content: DraftElement;
}

type DraftNode = DraftText | DraftElement | DraftWindow;

interface ResolveState {
  readonly maximumDepth: number;
  readonly maximumNodes: number;
  depth: number;
  nodes: number;
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

const STRING_PROPS = new Map<string, PropTarget<NullableStringField>>([
  ["label", { field: "label", canonical: "label" }],
  ["textValue", { field: "textValue", canonical: "textValue" }],
  ["value", { field: "value", canonical: "value" }],
  ["placeholder", { field: "placeholder", canonical: "placeholder" }],
  ["action", { field: "action", canonical: "action" }],
  ["aria-label", { field: "ariaLabel", canonical: "aria-label" }],
  ["ariaLabel", { field: "ariaLabel", canonical: "aria-label" }],
  ["name", { field: "name", canonical: "name" }],
  ["form", { field: "form", canonical: "form" }],
  ["type", { field: "inputType", canonical: "inputType" }],
  ["inputType", { field: "inputType", canonical: "inputType" }],
  ["accept", { field: "accept", canonical: "accept" }],
  ["capture", { field: "capture", canonical: "capture" }],
  ["alt", { field: "alt", canonical: "alt" }],
  ["href", { field: "href", canonical: "href" }],
  ["src", { field: "src", canonical: "src" }],
  ["srcset", { field: "srcset", canonical: "srcset" }],
  ["srcSet", { field: "srcset", canonical: "srcset" }],
  ["sizes", { field: "sizes", canonical: "sizes" }],
  ["media", { field: "media", canonical: "media" }],
  ["resourceType", { field: "resourceType", canonical: "resourceType" }],
  ["loading", { field: "loading", canonical: "loading" }],
  ["decoding", { field: "decoding", canonical: "decoding" }],
  ["fetchPriority", { field: "fetchPriority", canonical: "fetchPriority" }],
  ["crossOrigin", { field: "crossOrigin", canonical: "crossOrigin" }],
  ["referrerPolicy", { field: "referrerPolicy", canonical: "referrerPolicy" }],
  ["poster", { field: "poster", canonical: "poster" }],
  ["preload", { field: "preload", canonical: "preload" }],
  ["trackKind", { field: "trackKind", canonical: "trackKind" }],
  ["srclang", { field: "srclang", canonical: "srclang" }],
  ["srcLang", { field: "srclang", canonical: "srclang" }],
  ["trackLabel", { field: "trackLabel", canonical: "trackLabel" }],
  ["list", { field: "list", canonical: "list" }],
  ["dirname", { field: "dirname", canonical: "dirname" }],
  ["formAction", { field: "formAction", canonical: "formAction" }],
  ["formEnctype", { field: "formEnctype", canonical: "formEnctype" }],
  ["formEncType", { field: "formEnctype", canonical: "formEnctype" }],
  ["formMethod", { field: "formMethod", canonical: "formMethod" }],
  ["formTarget", { field: "formTarget", canonical: "formTarget" }],
  ["id", { field: "id", canonical: "id" }],
  ["class", { field: "className", canonical: "className" }],
  ["className", { field: "className", canonical: "className" }],
]);

const REQUIRED_BOOLEAN_PROPS = new Map<string, PropTarget<RequiredBooleanField>>([
  ["isDisabled", { field: "isDisabled", canonical: "isDisabled" }],
  ["disabled", { field: "isDisabled", canonical: "isDisabled" }],
  [
    "aria-disabled",
    { field: "isDisabled", canonical: "isDisabled", retainAriaAttribute: true },
  ],
  ["isRequired", { field: "isRequired", canonical: "isRequired" }],
  ["required", { field: "isRequired", canonical: "isRequired" }],
  [
    "aria-required",
    { field: "isRequired", canonical: "isRequired", retainAriaAttribute: true },
  ],
  ["isInvalid", { field: "isInvalid", canonical: "isInvalid" }],
  ["invalid", { field: "isInvalid", canonical: "isInvalid" }],
  [
    "aria-invalid",
    { field: "isInvalid", canonical: "isInvalid", retainAriaAttribute: true },
  ],
  ["isReadOnly", { field: "isReadOnly", canonical: "isReadOnly" }],
  ["readOnly", { field: "isReadOnly", canonical: "isReadOnly" }],
  ["readonly", { field: "isReadOnly", canonical: "isReadOnly" }],
  [
    "aria-readonly",
    { field: "isReadOnly", canonical: "isReadOnly", retainAriaAttribute: true },
  ],
  ["isSelected", { field: "isSelected", canonical: "isSelected" }],
  ["selected", { field: "isSelected", canonical: "isSelected" }],
  [
    "aria-selected",
    { field: "isSelected", canonical: "isSelected", retainAriaAttribute: true },
  ],
]);

const NULLABLE_BOOLEAN_PROPS = new Map<string, PropTarget<NullableBooleanField>>([
  ["isChecked", { field: "isChecked", canonical: "isChecked" }],
  ["checked", { field: "isChecked", canonical: "isChecked" }],
  [
    "aria-checked",
    { field: "isChecked", canonical: "isChecked", retainAriaAttribute: true },
  ],
  ["isExpanded", { field: "isExpanded", canonical: "isExpanded" }],
  ["expanded", { field: "isExpanded", canonical: "isExpanded" }],
  [
    "aria-expanded",
    { field: "isExpanded", canonical: "isExpanded", retainAriaAttribute: true },
  ],
  ["controls", { field: "controls", canonical: "controls" }],
  ["autoplay", { field: "autoplay", canonical: "autoplay" }],
  ["autoPlay", { field: "autoplay", canonical: "autoplay" }],
  ["loop", { field: "loopPlayback", canonical: "loopPlayback" }],
  ["loopPlayback", { field: "loopPlayback", canonical: "loopPlayback" }],
  ["muted", { field: "muted", canonical: "muted" }],
  ["playsInline", { field: "playsInline", canonical: "playsInline" }],
  ["defaultTrack", { field: "defaultTrack", canonical: "defaultTrack" }],
  ["formNoValidate", { field: "formNoValidate", canonical: "formNoValidate" }],
]);

const NUMBER_PROPS = new Map<string, PropTarget<NullableNumberField>>([
  ["min", { field: "minValue", canonical: "minValue" }],
  ["minValue", { field: "minValue", canonical: "minValue" }],
  [
    "aria-valuemin",
    { field: "minValue", canonical: "minValue", retainAriaAttribute: true },
  ],
  ["max", { field: "maxValue", canonical: "maxValue" }],
  ["maxValue", { field: "maxValue", canonical: "maxValue" }],
  [
    "aria-valuemax",
    { field: "maxValue", canonical: "maxValue", retainAriaAttribute: true },
  ],
  ["current", { field: "valueNumber", canonical: "valueNumber" }],
  ["valueNumber", { field: "valueNumber", canonical: "valueNumber" }],
  [
    "aria-valuenow",
    { field: "valueNumber", canonical: "valueNumber", retainAriaAttribute: true },
  ],
  ["step", { field: "stepValue", canonical: "stepValue" }],
  ["stepValue", { field: "stepValue", canonical: "stepValue" }],
  ["intrinsicWidth", { field: "intrinsicWidth", canonical: "intrinsicWidth" }],
  ["intrinsicHeight", { field: "intrinsicHeight", canonical: "intrinsicHeight" }],
]);

const EVENT_ALIASES = new Map<string, string>([
  ["onclick", "onClick"],
  ["onpress", "onPress"],
  ["onpressstart", "onPressStart"],
  ["onpressend", "onPressEnd"],
  ["onpressup", "onPressUp"],
  ["onpresschange", "onPressChange"],
  ["onchange", "onChange"],
  ["oninput", "onInput"],
  ["onselectionchange", "onSelectionChange"],
  ["onfocus", "onFocus"],
  ["onblur", "onBlur"],
  ["onfocuschange", "onFocusChange"],
  ["onfocuswithin", "onFocusWithin"],
  ["onblurwithin", "onBlurWithin"],
  ["onfocuswithinchange", "onFocusWithinChange"],
  ["ontoggle", "onToggle"],
  ["onexpandedchange", "onExpandedChange"],
  ["onhoverstart", "onHoverStart"],
  ["onhoverend", "onHoverEnd"],
  ["onhoverchange", "onHoverChange"],
  ["onkeydown", "onKeyDown"],
  ["onkeyup", "onKeyUp"],
  ["onwheel", "onWheel"],
  ["oncopy", "onCopy"],
  ["oncut", "onCut"],
  ["onpaste", "onPaste"],
]);

const RESERVED_WIRE_PROPS = new Set([
  "events",
  "actionLabels",
  "explicitProps",
  "importSource",
  "dangerouslySetInnerHTML",
  "innerHTML",
]);
const UNSAFE_PROPERTY_NAMES = new Set(["__proto__", "constructor", "prototype"]);

const PORTABLE_ATTRIBUTE_NAME = /^[A-Za-z_][A-Za-z0-9_.:-]*$/u;
const ARRAY_INDEX_NAME = /^(?:0|[1-9][0-9]*)$/u;
const textEncoder = new TextEncoder();

export function compileFrameV1(
  frameId: string,
  root: A3sJsxChild,
  options: CompileFrameOptions = {},
): CompiledA3sFrameV1 {
  if (typeof frameId !== "string" || frameId.length === 0) {
    throw new A3sJsxError("A3S frames need a non-empty string frame id");
  }
  const maximumDepth = normalizePositiveLimit(options.maximumDepth, 128, "maximumDepth");
  const maximumNodes = normalizePositiveLimit(options.maximumNodes, 100_000, "maximumNodes");
  const resolveState: ResolveState = { maximumDepth, maximumNodes, depth: 0, nodes: 0 };
  const roots = resolveValue(root, resolveState, false, null);

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

function resolveValue(
  value: unknown,
  state: ResolveState,
  staticArray: boolean,
  inheritedSource: A3sSourceLocation | null,
): DraftNode[] {
  if (value === null || value === undefined || typeof value === "boolean") {
    return [];
  }
  if (typeof value === "string") {
    countNode(state, inheritedSource);
    return [{
      kind: "text",
      key: null,
      explicitKey: false,
      source: inheritedSource,
      value,
    }];
  }
  if (typeof value === "number") {
    if (!Number.isFinite(value)) {
      throw new A3sJsxError("numeric JSX children must be finite", inheritedSource);
    }
    countNode(state, inheritedSource);
    return [{
      kind: "text",
      key: null,
      explicitKey: false,
      source: inheritedSource,
      value: Object.is(value, -0) ? "0" : String(value),
    }];
  }
  if (Array.isArray(value)) {
    return resolveArray(value, state, staticArray, inheritedSource);
  }
  if (isA3sElement(value)) {
    return resolveElement(value, state);
  }
  if (isThenable(value)) {
    throw new A3sJsxError(
      "promise and thenable children are not supported by protocol 1",
      inheritedSource,
    );
  }
  const kind = typeof value;
  if (kind === "bigint" || kind === "symbol" || kind === "function") {
    throw new A3sJsxError(`${kind} values cannot be rendered as JSX children`, inheritedSource);
  }
  if (isPlainRecord(value)) {
    throw new A3sJsxError("plain objects cannot be rendered as JSX children", inheritedSource);
  }
  throw new A3sJsxError(
    `${value?.constructor?.name ?? "object"} instances cannot be rendered as JSX children`,
    inheritedSource,
  );
}

function resolveArray(
  items: readonly unknown[],
  state: ResolveState,
  staticArray: boolean,
  source: A3sSourceLocation | null,
): DraftNode[] {
  const drafts: DraftNode[] = [];
  for (const item of items) {
    const resolved = resolveValue(item, state, false, source);
    if (!staticArray && resolved.length > 0 && !hasExplicitListIdentity(item)) {
      const itemSource = isA3sElement(item) ? item.source : source;
      throw new A3sJsxError(
        "mutable JSX arrays require an explicit key on every rendered item",
        itemSource,
      );
    }
    drafts.push(...resolved);
  }
  assignSiblingKeys(drafts);
  return drafts;
}

function resolveElement(element: A3sElement, state: ResolveState): DraftNode[] {
  enterDepth(state, element.source);
  try {
    if (element.type === Fragment) {
      const drafts = resolveChildren(element, state);
      return element.key === null ? clearFallbackKeys(drafts) : scopeDrafts(drafts, element.key);
    }
    if (typeof element.type === "function") {
      let output: unknown;
      try {
        output = element.type(element.props);
      } catch (error) {
        if (error instanceof A3sJsxError) {
          throw error;
        }
        throw new A3sJsxError(
          `function component ${describeElementType(element.type)} threw while rendering`,
          element.source,
          error,
        );
      }
      if (isThenable(output)) {
        throw new A3sJsxError(
          `function component ${describeElementType(element.type)} returned a promise; protocol 1 components are synchronous`,
          element.source,
        );
      }
      const drafts = resolveValue(output, state, false, element.source);
      return element.key === null ? clearFallbackKeys(drafts) : scopeDrafts(drafts, element.key);
    }

    if (typeof element.type !== "string") {
      throw new A3sJsxError("unsupported JSX element type", element.source);
    }
    countNode(state, element.source);
    if (element.type === "Window") {
      const children = resolveChildren(element, state);
      if (children.length !== 1 || children[0].kind !== "element") {
        throw new A3sJsxError(
          "Window must contain exactly one content element; use View to group content",
          element.source,
        );
      }
      const content = children[0];
      if (!content.explicitKey) {
        content.key = "root";
      }
      return [{
        kind: "window",
        key: element.key,
        explicitKey: element.key !== null,
        source: element.source,
        props: element.props,
        content,
      }];
    }

    const children = resolveChildren(element, state);
    if (children.some((child) => child.kind === "window")) {
      throw new A3sJsxError("Window is session metadata and can only appear at the root", element.source);
    }
    return [{
      kind: "element",
      key: element.key,
      explicitKey: element.key !== null,
      source: element.source,
      tag: element.type,
      props: element.props,
      children,
    }];
  } finally {
    state.depth -= 1;
  }
}

function resolveChildren(element: A3sElement, state: ResolveState): DraftNode[] {
  const children = element.props.children;
  const drafts = Array.isArray(children)
    ? resolveArray(children, state, element.staticChildren, element.source)
    : resolveValue(children, state, false, element.source);
  assignSiblingKeys(drafts);
  return drafts;
}

function scopeDrafts(drafts: DraftNode[], scope: string): DraftNode[] {
  if (drafts.length === 0) {
    return drafts;
  }
  assignSiblingKeys(drafts);
  if (drafts.length === 1 && !drafts[0].explicitKey) {
    drafts[0].key = scope;
    drafts[0].explicitKey = true;
    return drafts;
  }
  for (const draft of drafts) {
    draft.key = scopedKey(scope, requireDraftKey(draft));
    draft.explicitKey = true;
  }
  return drafts;
}

function clearFallbackKeys(drafts: DraftNode[]): DraftNode[] {
  for (const draft of drafts) {
    if (!draft.explicitKey) {
      draft.key = null;
    }
  }
  return drafts;
}

function assignSiblingKeys(drafts: DraftNode[]): void {
  let textIndex = 0;
  let elementIndex = 0;
  const keys = new Set<string>();
  for (const draft of drafts) {
    if (draft.key === null) {
      draft.key = draft.kind === "text" ? `text-${textIndex}` : `child-${elementIndex}`;
    }
    if (draft.kind === "text") {
      textIndex += 1;
    } else {
      elementIndex += 1;
    }
    if (keys.has(draft.key)) {
      throw new A3sJsxError(
        `compiled sibling nodes need unique keys; duplicate key ${JSON.stringify(draft.key)}`,
        draft.source,
      );
    }
    keys.add(draft.key);
  }
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

function scopedKey(scope: string, child: string): string {
  return `s1:${encodeSegments([scope, child])}`;
}

function requireDraftKey(draft: DraftNode): string {
  if (draft.key === null || draft.key.length === 0) {
    throw new A3sJsxError("compiled JSX nodes need non-empty keys", draft.source);
  }
  return draft.key;
}

function hasExplicitListIdentity(value: unknown): boolean {
  return isA3sElement(value) && value.key !== null;
}

function countNode(state: ResolveState, source: A3sSourceLocation | null): void {
  state.nodes += 1;
  if (state.nodes > state.maximumNodes) {
    throw new A3sJsxError(
      `JSX output exceeds the configured maximum of ${state.maximumNodes} nodes`,
      source,
    );
  }
}

function enterDepth(state: ResolveState, source: A3sSourceLocation | null): void {
  state.depth += 1;
  if (state.depth > state.maximumDepth) {
    state.depth -= 1;
    throw new A3sJsxError(
      `JSX output exceeds the configured maximum depth of ${state.maximumDepth}`,
      source,
    );
  }
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

function isThenable(value: unknown): value is PromiseLike<unknown> {
  return (
    (typeof value === "object" && value !== null) || typeof value === "function"
  ) && typeof (value as { then?: unknown }).then === "function";
}
