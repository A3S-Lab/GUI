export type A3sProtocolJsonErrorFactory = (message: string) => Error;

const textEncoder = new TextEncoder();

export function snapshotA3sProtocolJsonV1(
  value: unknown,
  path: string,
  error: A3sProtocolJsonErrorFactory,
  active = new Set<object>(),
): unknown {
  if (value === null || typeof value === "string" || typeof value === "boolean") {
    return value;
  }
  if (typeof value === "number") {
    if (!Number.isFinite(value)) {
      throw error(`${path} contains a non-finite number`);
    }
    return value;
  }
  if (typeof value !== "object") {
    throw error(`${path} contains a non-JSON value`);
  }
  if (active.has(value)) {
    throw error(`${path} contains a cycle`);
  }

  active.add(value);
  try {
    if (Array.isArray(value)) {
      const clone: unknown[] = [];
      for (let index = 0; index < value.length; index += 1) {
        const descriptor = Object.getOwnPropertyDescriptor(value, index);
        if (descriptor === undefined) {
          throw error(`${path} contains a sparse array`);
        }
        if (!("value" in descriptor)) {
          throw error(`${path}[${index}] cannot be an accessor`);
        }
        clone.push(
          snapshotA3sProtocolJsonV1(
            descriptor.value,
            `${path}[${index}]`,
            error,
            active,
          ),
        );
      }
      return Object.freeze(clone);
    }

    const prototype = Object.getPrototypeOf(value);
    if (prototype !== Object.prototype && prototype !== null) {
      throw error(`${path} must contain only plain objects`);
    }
    const descriptors = Object.getOwnPropertyDescriptors(value);
    const enumerableSymbols = Object.getOwnPropertySymbols(value).filter(
      (symbol) => Object.getOwnPropertyDescriptor(value, symbol)?.enumerable,
    );
    if (enumerableSymbols.length > 0) {
      throw error(`${path} cannot contain symbol fields`);
    }

    const clone: Record<string, unknown> = {};
    for (const [name, descriptor] of Object.entries(descriptors)) {
      if (!descriptor.enumerable) {
        continue;
      }
      if (!("value" in descriptor)) {
        throw error(`${path}.${name} cannot be an accessor`);
      }
      Object.defineProperty(clone, name, {
        configurable: false,
        enumerable: true,
        value: snapshotA3sProtocolJsonV1(
          descriptor.value,
          `${path}.${name}`,
          error,
          active,
        ),
        writable: false,
      });
    }
    return Object.freeze(clone);
  } finally {
    active.delete(value);
  }
}

export function encodeA3sProtocolJsonPayloadV1(
  value: unknown,
  path: string,
  error: A3sProtocolJsonErrorFactory,
): Uint8Array {
  const snapshot = snapshotA3sProtocolJsonV1(value, path, error);
  let json: string;
  try {
    json = JSON.stringify(snapshot);
  } catch (cause) {
    throw error(`${path} could not be serialized as JSON: ${String(cause)}`);
  }
  return textEncoder.encode(json);
}
