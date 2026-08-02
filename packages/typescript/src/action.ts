import {
  assertExplicitActionIdV1,
  isExplicitActionIdV1,
} from "./identity.ts";

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
  assertExplicitActionIdV1(id);
  if (handler !== null && typeof handler !== "function") {
    throw new TypeError("A3S action handlers must be functions or null");
  }
  if (!isPlainRecord(options)) {
    throw new TypeError("A3S action options must be a plain object");
  }
  const descriptors = Object.getOwnPropertyDescriptors(options);
  for (const key of Reflect.ownKeys(descriptors)) {
    if (key !== "disabled" && key !== "label") {
      throw new TypeError(`A3S action options contain unknown field ${String(key)}`);
    }
    const descriptor = descriptors[key];
    if (descriptor === undefined || !("value" in descriptor)) {
      throw new TypeError(`A3S action option ${String(key)} cannot be an accessor`);
    }
  }
  const disabled = descriptors.disabled?.value;
  const label = descriptors.label?.value;
  if (disabled !== undefined && typeof disabled !== "boolean") {
    throw new TypeError("A3S action disabled must be a boolean");
  }
  if (label !== undefined && typeof label !== "string") {
    throw new TypeError("A3S action label must be a string");
  }

  return Object.freeze({
    $$typeof: ACTION_MARKER,
    id,
    handler,
    disabled: disabled ?? null,
    label: label ?? null,
  });
}

export function isA3sAction(value: unknown): value is A3sAction {
  if (typeof value !== "object" || value === null || !Object.isFrozen(value)) {
    return false;
  }
  const descriptors = Object.getOwnPropertyDescriptors(value);
  const keys = Reflect.ownKeys(descriptors);
  if (
    keys.length !== 5 ||
    !keys.every((key) =>
      key === "$$typeof" ||
      key === "id" ||
      key === "handler" ||
      key === "disabled" ||
      key === "label"
    ) ||
    keys.some((key) => !("value" in descriptors[key]!))
  ) {
    return false;
  }
  const marker = descriptors.$$typeof!.value;
  const id = descriptors.id!.value;
  const handler = descriptors.handler!.value;
  const disabled = descriptors.disabled!.value;
  const label = descriptors.label!.value;
  return (
    marker === ACTION_MARKER &&
    isExplicitActionIdV1(id) &&
    (handler === null || typeof handler === "function") &&
    (disabled === null || typeof disabled === "boolean") &&
    (label === null || typeof label === "string")
  );
}

function isPlainRecord(value: unknown): value is Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}
