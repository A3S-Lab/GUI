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
  isA3sContextProvider,
  type A3sContextProvider,
} from "./context.ts";
import { isA3sErrorBoundary } from "./error-boundary.ts";
import type { ComponentRenderRuntime } from "./component-runtime.ts";
import {
  createComponentIdentityV1,
  encodeIdentitySegmentsV1,
} from "./identity.ts";

export interface DraftBase {
  key: string | null;
  explicitKey: boolean;
  readonly source: A3sSourceLocation | null;
}

export interface DraftText extends DraftBase {
  readonly kind: "text";
  readonly value: string;
}

export interface DraftElement extends DraftBase {
  readonly kind: "element";
  readonly tag: string;
  readonly props: Readonly<A3sJsxProps>;
  readonly children: DraftNode[];
}

export interface DraftWindow extends DraftBase {
  readonly kind: "window";
  readonly props: Readonly<A3sJsxProps>;
  readonly content: DraftElement;
}

export type DraftNode = DraftText | DraftElement | DraftWindow;

interface ResolveState {
  readonly maximumDepth: number;
  readonly maximumNodes: number;
  readonly componentRuntime: ComponentRenderRuntime | null;
  depth: number;
  nodes: number;
}

export function resolveFrameRoot(
  root: A3sJsxChild,
  maximumDepth: number,
  maximumNodes: number,
  componentRuntime: ComponentRenderRuntime | null = null,
): DraftNode[] {
  const state: ResolveState = {
    maximumDepth,
    maximumNodes,
    componentRuntime,
    depth: 0,
    nodes: 0,
  };
  return resolveValue(
    root,
    state,
    false,
    null,
    childAddress(["root"], root, 0),
  );
}

function resolveValue(
  value: unknown,
  state: ResolveState,
  staticArray: boolean,
  inheritedSource: A3sSourceLocation | null,
  address: readonly string[],
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
    return resolveArray(value, state, staticArray, inheritedSource, address);
  }
  if (isA3sElement(value)) {
    return resolveElement(value, state, address);
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
  address: readonly string[],
): DraftNode[] {
  const drafts: DraftNode[] = [];
  for (let index = 0; index < items.length; index += 1) {
    const item = items[index];
    const resolved = resolveValue(
      item,
      state,
      false,
      source,
      childAddress(address, item, index),
    );
    if (
      !staticArray &&
      (resolved.length > 0 || isA3sElement(item)) &&
      !hasExplicitListIdentity(item)
    ) {
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

function resolveElement(
  element: A3sElement,
  state: ResolveState,
  address: readonly string[],
): DraftNode[] {
  enterDepth(state, element.source);
  try {
    if (element.type === Fragment) {
      const drafts = resolveChildren(element, state, [...address, "fragment"]);
      return element.key === null ? clearFallbackKeys(drafts) : scopeDrafts(drafts, element.key);
    }
    if (isA3sContextProvider(element.type)) {
      return resolveContextProvider(element, element.type, state, address);
    }
    if (isA3sErrorBoundary(element.type)) {
      return resolveErrorBoundary(element, state, address);
    }
    if (typeof element.type === "function") {
      const component = element.type;
      let output: unknown;
      try {
        const invoke = () => component(element.props);
        output = state.componentRuntime === null
          ? invoke()
          : state.componentRuntime.renderComponent(
            {
              identity: createComponentIdentityV1(address),
              component,
              props: element.props,
              source: element.source,
            },
            invoke,
          );
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
      const drafts = resolveValue(
        output,
        state,
        false,
        element.source,
        childAddress([...address, "output"], output, 0),
      );
      return element.key === null ? clearFallbackKeys(drafts) : scopeDrafts(drafts, element.key);
    }

    if (typeof element.type !== "string") {
      throw new A3sJsxError("unsupported JSX element type", element.source);
    }
    countNode(state, element.source);
    if (element.type === "Window") {
      const children = resolveChildren(element, state, [...address, "window"]);
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

    const children = resolveChildren(element, state, [...address, "host"]);
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

function resolveContextProvider(
  element: A3sElement,
  provider: A3sContextProvider<unknown>,
  state: ResolveState,
  address: readonly string[],
): DraftNode[] {
  assertTransparentProps(element, ["value", "children"]);
  if (!Object.hasOwn(element.props, "value")) {
    throw new A3sJsxError("context providers require a value prop", element.source);
  }
  const resolve = () => resolveChildren(element, state, [...address, "context"]);
  const drafts = state.componentRuntime === null
    ? resolve()
    : state.componentRuntime.withContextValue(
      provider.context,
      element.props.value,
      resolve,
    );
  return scopeTransparentDrafts(element, drafts);
}

function resolveErrorBoundary(
  element: A3sElement,
  state: ResolveState,
  address: readonly string[],
): DraftNode[] {
  assertTransparentProps(element, ["fallback", "children"]);
  if (!Object.hasOwn(element.props, "fallback")) {
    throw new A3sJsxError("error boundaries require a fallback prop", element.source);
  }

  const nodeCheckpoint = state.nodes;
  const renderCheckpoint = state.componentRuntime?.createCheckpoint() ?? null;
  let drafts: DraftNode[];
  try {
    drafts = resolveChildren(element, state, [...address, "boundary"]);
  } catch (cause) {
    state.nodes = nodeCheckpoint;
    if (renderCheckpoint !== null) {
      state.componentRuntime?.rollbackToCheckpoint(renderCheckpoint);
    }
    const error = normalizeBoundaryError(cause, element.source);
    let fallback = element.props.fallback;
    if (typeof fallback === "function") {
      try {
        fallback = fallback(error);
      } catch (fallbackCause) {
        throw new A3sJsxError(
          "error boundary fallback threw while rendering",
          element.source,
          fallbackCause,
        );
      }
    }
    drafts = resolveValue(
      fallback,
      state,
      false,
      element.source,
      childAddress([...address, "fallback"], fallback, 0),
    );
    assignSiblingKeys(drafts);
  }
  return scopeTransparentDrafts(element, drafts);
}

function scopeTransparentDrafts(element: A3sElement, drafts: DraftNode[]): DraftNode[] {
  return element.key === null ? clearFallbackKeys(drafts) : scopeDrafts(drafts, element.key);
}

function assertTransparentProps(
  element: A3sElement,
  allowed: readonly string[],
): void {
  const allowedNames = new Set(allowed);
  for (const name of Object.keys(element.props)) {
    if (!allowedNames.has(name)) {
      throw new A3sJsxError(
        `${describeElementType(element.type)} does not accept prop ${JSON.stringify(name)}`,
        element.source,
      );
    }
  }
}

function normalizeBoundaryError(
  cause: unknown,
  source: A3sSourceLocation | null,
): A3sJsxError {
  return cause instanceof A3sJsxError
    ? cause
    : new A3sJsxError("error boundary caught a descendant render failure", source, cause);
}

function resolveChildren(
  element: A3sElement,
  state: ResolveState,
  address: readonly string[],
): DraftNode[] {
  const children = element.props.children;
  const drafts = Array.isArray(children)
    ? resolveArray(children, state, element.staticChildren, element.source, address)
    : resolveValue(
      children,
      state,
      false,
      element.source,
      childAddress(address, children, 0),
    );
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

function scopedKey(scope: string, child: string): string {
  return `s1:${encodeIdentitySegmentsV1([scope, child])}`;
}

function childAddress(
  parent: readonly string[],
  value: unknown,
  index: number,
): readonly string[] {
  const segment = isA3sElement(value) && value.key !== null
    ? `key:${value.key}`
    : `index:${index}`;
  return [...parent, segment];
}

export function requireDraftKey(draft: DraftNode): string {
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
