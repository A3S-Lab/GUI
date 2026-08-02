const ACTION_MARKER = Symbol.for("@a3s/gui.action.v1");

export type A3sEventHandler<Event = unknown> = (event: Event) => unknown;

export interface A3sActionOptions {
  readonly disabled?: boolean;
  readonly label?: string;
}

export interface A3sAction<Event = unknown> {
  readonly $$typeof: typeof ACTION_MARKER;
  readonly id: string;
  readonly handler: A3sEventHandler<Event> | null;
  readonly disabled: boolean | null;
  readonly label: string | null;
}

export function defineAction<Event = unknown>(
  id: string,
  handler: A3sEventHandler<Event> | null = null,
  options: A3sActionOptions = {},
): A3sAction<Event> {
  if (typeof id !== "string" || id.length === 0) {
    throw new TypeError("A3S action ids must be non-empty strings");
  }
  if (isArrayIndexName(id)) {
    throw new TypeError("A3S action ids cannot be canonical JavaScript array-index names");
  }
  if (handler !== null && typeof handler !== "function") {
    throw new TypeError("A3S action handlers must be functions or null");
  }
  if (!isPlainRecord(options)) {
    throw new TypeError("A3S action options must be a plain object");
  }
  if (options.disabled !== undefined && typeof options.disabled !== "boolean") {
    throw new TypeError("A3S action disabled must be a boolean");
  }
  if (options.label !== undefined && typeof options.label !== "string") {
    throw new TypeError("A3S action label must be a string");
  }

  return Object.freeze({
    $$typeof: ACTION_MARKER,
    id,
    handler,
    disabled: options.disabled ?? null,
    label: options.label ?? null,
  });
}

export function isA3sAction(value: unknown): value is A3sAction {
  if (typeof value !== "object" || value === null || !Object.isFrozen(value)) {
    return false;
  }
  const candidate = value as Partial<A3sAction>;
  return (
    candidate.$$typeof === ACTION_MARKER &&
    typeof candidate.id === "string" &&
    candidate.id.length > 0 &&
    !isArrayIndexName(candidate.id) &&
    (candidate.handler === null || typeof candidate.handler === "function") &&
    (candidate.disabled === null || typeof candidate.disabled === "boolean") &&
    (candidate.label === null || typeof candidate.label === "string")
  );
}

function isArrayIndexName(value: string): boolean {
  if (!/^(?:0|[1-9][0-9]*)$/u.test(value)) {
    return false;
  }
  const index = Number(value);
  return Number.isInteger(index) && index >= 0 && index < 0xffff_ffff;
}

function isPlainRecord(value: unknown): value is Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}
