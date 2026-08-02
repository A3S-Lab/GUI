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
  depth: number;
  nodes: number;
}

const textEncoder = new TextEncoder();

export function resolveFrameRoot(
  root: A3sJsxChild,
  maximumDepth: number,
  maximumNodes: number,
): DraftNode[] {
  const state: ResolveState = {
    maximumDepth,
    maximumNodes,
    depth: 0,
    nodes: 0,
  };
  return resolveValue(root, state, false, null);
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

function scopedKey(scope: string, child: string): string {
  return `s1:${encodeSegments([scope, child])}`;
}

function encodeSegments(segments: readonly string[]): string {
  return segments
    .map((segment) => `${textEncoder.encode(segment).byteLength}:${segment}`)
    .join("");
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
