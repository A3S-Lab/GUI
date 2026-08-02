const ELEMENT_MARKER = Symbol.for("@a3s/gui.element.v1");
const FRAGMENT_MARKER = Symbol.for("@a3s/gui.fragment.v1");

export type A3sKey = string | number;

export interface A3sSourceLocation {
  readonly fileName: string;
  readonly lineNumber: number;
  readonly columnNumber: number;
}

export interface A3sJsxProps {
  readonly children?: A3sJsxChild;
  readonly key?: A3sKey | null;
  readonly ref?: never;
  readonly [name: string]: unknown;
}

export type A3sFunctionComponent<Props extends A3sJsxProps = A3sJsxProps> = (
  props: Readonly<Props>,
) => A3sJsxChild;

export type A3sElementType =
  | string
  | A3sFunctionComponent
  | typeof FRAGMENT_MARKER;

export interface A3sElement {
  readonly $$typeof: typeof ELEMENT_MARKER;
  readonly type: A3sElementType;
  readonly key: string | null;
  readonly props: Readonly<A3sJsxProps>;
  readonly source: A3sSourceLocation | null;
  readonly staticChildren: boolean;
}

export type A3sJsxChild =
  | A3sElement
  | string
  | number
  | boolean
  | null
  | undefined
  | readonly A3sJsxChild[];

export class A3sJsxError extends Error {
  readonly source: A3sSourceLocation | null;

  constructor(message: string, source: A3sSourceLocation | null = null, cause?: unknown) {
    super(formatSourceMessage(message, source), cause === undefined ? undefined : { cause });
    this.name = "A3sJsxError";
    this.source = source;
  }
}

export const Fragment = FRAGMENT_MARKER;

interface CreateElementOptions {
  readonly staticChildren: boolean;
  readonly source?: unknown;
}

export function createA3sElement(
  type: unknown,
  inputProps: unknown,
  explicitKey: unknown,
  options: CreateElementOptions,
): A3sElement {
  const source = normalizeSource(options.source);
  const normalizedType = normalizeElementType(type, source);
  const props = snapshotProps(inputProps, source);
  const keyInput = explicitKey === undefined ? props.key : explicitKey;
  const key = normalizeKey(keyInput, source);

  if (props.ref !== undefined && props.ref !== null) {
    throw new A3sJsxError(
      "refs are not part of protocol 1; use a typed focus or platform command instead",
      source,
    );
  }

  const elementProps: Record<string, unknown> = {};
  for (const [name, value] of Object.entries(props)) {
    if (name === "key" || name === "ref") {
      continue;
    }
    defineDataProperty(
      elementProps,
      name,
      name === "children" ? snapshotChild(value, source, new Set()) : value,
    );
  }

  const element: A3sElement = {
    $$typeof: ELEMENT_MARKER,
    type: normalizedType,
    key,
    props: Object.freeze(elementProps),
    source,
    staticChildren: options.staticChildren,
  };
  return Object.freeze(element);
}

export function isA3sElement(value: unknown): value is A3sElement {
  return (
    isObject(value) &&
    value.$$typeof === ELEMENT_MARKER &&
    Object.isFrozen(value) &&
    isObject(value.props) &&
    Object.isFrozen(value.props)
  );
}

export function describeElementType(type: A3sElementType): string {
  if (type === Fragment) {
    return "Fragment";
  }
  if (typeof type === "string") {
    return type;
  }
  const component = type as A3sFunctionComponent & {
    readonly displayName?: string;
    readonly name?: string;
  };
  return component.displayName ?? component.name ?? "AnonymousComponent";
}

function normalizeElementType(type: unknown, source: A3sSourceLocation | null): A3sElementType {
  if (type === Fragment || typeof type === "function") {
    return type as A3sElementType;
  }
  if (typeof type === "string" && type.length > 0) {
    return type;
  }
  throw new A3sJsxError(
    "JSX element types must be a non-empty intrinsic name, Fragment, or synchronous function component",
    source,
  );
}

function snapshotProps(input: unknown, source: A3sSourceLocation | null): Record<string, unknown> {
  if (input === null || input === undefined) {
    return {};
  }
  if (!isPlainRecord(input)) {
    throw new A3sJsxError("JSX props must be a plain object", source);
  }

  const descriptors = Object.getOwnPropertyDescriptors(input);
  const enumerableSymbols = Object.getOwnPropertySymbols(input).filter(
    (symbol) => descriptors[symbol]?.enumerable,
  );
  if (enumerableSymbols.length > 0) {
    throw new A3sJsxError("JSX props cannot contain enumerable symbol keys", source);
  }

  const props: Record<string, unknown> = {};
  for (const [name, descriptor] of Object.entries(descriptors)) {
    if (!descriptor.enumerable) {
      continue;
    }
    if (!("value" in descriptor)) {
      throw new A3sJsxError(`JSX prop ${JSON.stringify(name)} cannot be an accessor`, source);
    }
    defineDataProperty(props, name, descriptor.value);
  }
  return props;
}

function snapshotChild(
  child: unknown,
  source: A3sSourceLocation | null,
  activeArrays: Set<readonly unknown[]>,
): A3sJsxChild {
  if (
    child === null ||
    child === undefined ||
    typeof child === "string" ||
    typeof child === "boolean"
  ) {
    return child;
  }
  if (typeof child === "number") {
    if (!Number.isFinite(child)) {
      throw new A3sJsxError("numeric JSX children must be finite", source);
    }
    return child;
  }
  if (Array.isArray(child)) {
    if (activeArrays.has(child)) {
      throw new A3sJsxError("JSX child arrays cannot be cyclic", source);
    }
    activeArrays.add(child);
    const snapshot = child.map((item) => snapshotChild(item, source, activeArrays));
    activeArrays.delete(child);
    return Object.freeze(snapshot);
  }
  if (isA3sElement(child)) {
    return child;
  }
  if (isThenable(child)) {
    throw new A3sJsxError(
      "promise and thenable children are not supported by protocol 1",
      source,
    );
  }

  const kind = typeof child;
  if (kind === "bigint" || kind === "symbol" || kind === "function") {
    throw new A3sJsxError(`${kind} values cannot be rendered as JSX children`, source);
  }
  if (isPlainRecord(child)) {
    throw new A3sJsxError("plain objects cannot be rendered as JSX children", source);
  }
  throw new A3sJsxError(
    `${child?.constructor?.name ?? "object"} instances cannot be rendered as JSX children`,
    source,
  );
}

function normalizeKey(value: unknown, source: A3sSourceLocation | null): string | null {
  if (value === null || value === undefined) {
    return null;
  }
  if (typeof value === "string") {
    if (value.length === 0) {
      throw new A3sJsxError("explicit JSX keys must not be empty", source);
    }
    return value;
  }
  if (typeof value === "number" && Number.isFinite(value)) {
    return Object.is(value, -0) ? "0" : String(value);
  }
  throw new A3sJsxError("JSX keys must be finite numbers or non-empty strings", source);
}

function normalizeSource(value: unknown): A3sSourceLocation | null {
  if (!isObject(value)) {
    return null;
  }
  const fileName = value.fileName;
  const lineNumber = value.lineNumber;
  const columnNumber = value.columnNumber;
  if (
    typeof fileName !== "string" ||
    fileName.length === 0 ||
    typeof lineNumber !== "number" ||
    !Number.isSafeInteger(lineNumber) ||
    lineNumber < 1 ||
    typeof columnNumber !== "number" ||
    !Number.isSafeInteger(columnNumber) ||
    columnNumber < 1
  ) {
    return null;
  }
  return Object.freeze({ fileName, lineNumber, columnNumber });
}

function formatSourceMessage(message: string, source: A3sSourceLocation | null): string {
  if (source === null) {
    return message;
  }
  return `${source.fileName}:${source.lineNumber}:${source.columnNumber}: ${message}`;
}

function isPlainRecord(value: unknown): value is Record<PropertyKey, unknown> {
  if (!isObject(value) || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

function isThenable(value: unknown): value is PromiseLike<unknown> {
  return isObject(value) && typeof value.then === "function";
}

function isObject(value: unknown): value is Record<PropertyKey, unknown> {
  return (typeof value === "object" && value !== null) || typeof value === "function";
}

function defineDataProperty(target: Record<string, unknown>, name: string, value: unknown): void {
  Object.defineProperty(target, name, {
    configurable: true,
    enumerable: true,
    value,
    writable: true,
  });
}
