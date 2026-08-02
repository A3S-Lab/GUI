import {
  TSX_PROTOCOL_NAME,
  TSX_PROTOCOL_VERSION_V1,
  TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
  TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
  TSX_PROTOCOL_V1_MAX_DIAGNOSTICS,
  TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
  TSX_PROTOCOL_V1_MAX_EVENT_ITEMS,
  TSX_PROTOCOL_V1_MAX_SAFE_INTEGER,
  TSX_PROTOCOL_V1_MAX_SESSION_ID_BYTES,
  TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
  type TsxHostMessageV1,
} from "./generated/protocol.ts";
import { snapshotA3sProtocolJsonV1 } from "./protocol-json.ts";
import {
  clientSessionError,
  type A3sClientSessionErrorCodeV1,
} from "./client-session-error.ts";

type TsxWelcomeMessageV1 = Extract<TsxHostMessageV1, { readonly type: "welcome" }>;

const textEncoder = new TextEncoder();

export function validateWelcome(message: TsxWelcomeMessageV1): void {
  const record = requireRecord(message, "welcome message", "invalidWelcome");
  assertExactKeys(
    record,
    [
      "type",
      "protocol",
      "protocolVersion",
      "sessionId",
      "messageId",
      "renderRevision",
      "payload",
    ],
    [],
    "welcome message",
    "invalidWelcome",
  );
  if (
    record.type !== "welcome" ||
    record.protocol !== TSX_PROTOCOL_NAME ||
    record.protocolVersion !== TSX_PROTOCOL_VERSION_V1 ||
    record.messageId !== 1 ||
    record.renderRevision !== 0
  ) {
    throw clientSessionError(
      "invalidWelcome",
      `expected the first ${TSX_PROTOCOL_NAME} v${TSX_PROTOCOL_VERSION_V1} host message to be welcome`,
    );
  }
  requireBoundedText(
    record.sessionId,
    "welcome session id",
    TSX_PROTOCOL_V1_MAX_SESSION_ID_BYTES,
    "invalidWelcome",
  );

  const payload = requireRecord(record.payload, "welcome payload", "invalidWelcome");
  assertExactKeys(
    payload,
    [
      "selectedProtocolVersion",
      "hostVersion",
      "hostBuildId",
      "platform",
      "renderer",
      "limits",
    ],
    ["capabilities", "debugCapabilities"],
    "welcome payload",
    "invalidWelcome",
  );
  if (payload.selectedProtocolVersion !== TSX_PROTOCOL_VERSION_V1) {
    throw clientSessionError("invalidWelcome", "welcome selected an unsupported protocol version");
  }
  requireBoundedText(
    payload.hostVersion,
    "host version",
    TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
    "invalidWelcome",
  );
  requireBoundedText(
    payload.hostBuildId,
    "host build id",
    TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
    "invalidWelcome",
  );
  requireEnum(payload.platform, ["headless", "macos", "windows", "linux"], "host platform");
  requireEnum(payload.renderer, ["software", "gpu"], "host renderer");

  const limits = requireRecord(payload.limits, "welcome limits", "invalidWelcome");
  assertExactKeys(
    limits,
    ["maximumFrameBytes", "maximumInFlightRenders"],
    [],
    "welcome limits",
    "invalidWelcome",
  );
  requireSafeInteger(
    limits.maximumFrameBytes,
    "maximum frame bytes",
    1,
    "invalidWelcome",
    TSX_PROTOCOL_V1_HARD_MAX_FRAME_BYTES,
  );
  if (limits.maximumInFlightRenders !== 1) {
    throw clientSessionError("invalidWelcome", "protocol v1 requires one in-flight render");
  }
  validateUniqueEnumArray(
    payload.capabilities,
    [
      "headlessRendering",
      "selfDrawnRendering",
      "dropPolicyQueries",
      "structuredDiagnostics",
    ],
    "host capabilities",
  );
  validateUniqueEnumArray(
    payload.debugCapabilities,
    ["protocolTrace", "structuredDiagnostics", "inspector"],
    "debug capabilities",
  );
}

export function validateDiagnostics(value: unknown): void {
  if (value === undefined) {
    return;
  }
  if (!Array.isArray(value) || value.length > TSX_PROTOCOL_V1_MAX_DIAGNOSTICS) {
    throw clientSessionError(
      "invalidMessage",
      `committed diagnostics must contain at most ${TSX_PROTOCOL_V1_MAX_DIAGNOSTICS} items`,
    );
  }
  for (let index = 0; index < value.length; index += 1) {
    const diagnostic = requireRecord(
      value[index],
      `committed diagnostic ${index}`,
      "invalidMessage",
    );
    assertExactKeys(
      diagnostic,
      ["severity", "code", "message"],
      ["elementId"],
      `committed diagnostic ${index}`,
      "invalidMessage",
    );
    requireEnum(
      diagnostic.severity,
      ["information", "warning", "error"],
      `committed diagnostic ${index} severity`,
      "invalidMessage",
    );
    if (diagnostic.severity === "error") {
      throw clientSessionError(
        "invalidMessage",
        "committed messages cannot contain error diagnostics",
      );
    }
    requireBoundedText(
      diagnostic.code,
      `committed diagnostic ${index} code`,
      TSX_PROTOCOL_V1_MAX_VERSION_BYTES,
      "invalidMessage",
    );
    requireBoundedText(
      diagnostic.message,
      `committed diagnostic ${index} message`,
      TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
      "invalidMessage",
    );
    validateOptionalElementId(
      diagnostic.elementId,
      `committed diagnostic ${index} element id`,
    );
  }
}

export function validateBoundedArray(value: unknown, name: string): void {
  if (value === undefined) {
    return;
  }
  if (!Array.isArray(value) || value.length > TSX_PROTOCOL_V1_MAX_EVENT_ITEMS) {
    throw clientSessionError(
      "invalidMessage",
      `${name} must contain at most ${TSX_PROTOCOL_V1_MAX_EVENT_ITEMS} items`,
    );
  }
}

export function validateOptionalElementId(value: unknown, name: string): void {
  if (value !== undefined && value !== null) {
    requireBoundedText(
      value,
      name,
      TSX_PROTOCOL_V1_MAX_ELEMENT_ID_BYTES,
      "invalidMessage",
    );
  }
}

export function validateOptionalCloseMessage(value: unknown): string | null {
  if (value === undefined || value === null) {
    return null;
  }
  return requireBoundedText(
    value,
    "close message",
    TSX_PROTOCOL_V1_MAX_DIAGNOSTIC_BYTES,
    "invalidMessage",
  );
}

function validateUniqueEnumArray(
  value: unknown,
  allowed: readonly string[],
  name: string,
): void {
  if (value === undefined) {
    return;
  }
  if (!Array.isArray(value)) {
    throw clientSessionError("invalidWelcome", `${name} must be an array`);
  }
  const seen = new Set<string>();
  for (const item of value) {
    const entry = requireEnum(item, allowed, name);
    if (seen.has(entry)) {
      throw clientSessionError("invalidWelcome", `${name} must not contain duplicates`);
    }
    seen.add(entry);
  }
}

export function requireEnum(
  value: unknown,
  allowed: readonly string[],
  name: string,
  code: A3sClientSessionErrorCodeV1 = "invalidWelcome",
): string {
  if (typeof value !== "string" || !allowed.includes(value)) {
    throw clientSessionError(code, `${name} is invalid`);
  }
  return value;
}

export function requireFingerprint(value: unknown, name: string): void {
  if (typeof value !== "string" || !/^[0-9a-f]{16}$/u.test(value)) {
    throw clientSessionError(
      "invalidMessage",
      `${name} must be sixteen lowercase hexadecimal digits`,
    );
  }
}

export function requireBoundedText(
  value: unknown,
  name: string,
  maximumBytes: number,
  code: A3sClientSessionErrorCodeV1,
): string {
  if (
    typeof value !== "string" ||
    value.trim().length === 0 ||
    textEncoder.encode(value).byteLength > maximumBytes ||
    /[\u0000-\u001f\u007f-\u009f]/u.test(value)
  ) {
    throw clientSessionError(code, `${name} must be non-empty bounded text`);
  }
  return value;
}

export function requireSafeInteger(
  value: unknown,
  name: string,
  minimum: number,
  code: A3sClientSessionErrorCodeV1,
  maximum: number = TSX_PROTOCOL_V1_MAX_SAFE_INTEGER,
): number {
  if (
    typeof value !== "number" ||
    !Number.isSafeInteger(value) ||
    value < minimum ||
    value > maximum
  ) {
    throw clientSessionError(
      code,
      `${name} must be an integer from ${minimum} through ${maximum}`,
    );
  }
  return value;
}

export function nextSafeInteger(value: number, name: string): number {
  if (value >= TSX_PROTOCOL_V1_MAX_SAFE_INTEGER) {
    throw clientSessionError(
      "messageIdExhausted",
      `${name} exhausted the protocol-safe range`,
    );
  }
  return value + 1;
}

export function assertEncodedSize(
  message: unknown,
  maximumBytes: number,
  code: "frameTooLarge" | "invalidWelcome",
): void {
  const bytes = textEncoder.encode(JSON.stringify(message)).byteLength;
  if (bytes === 0 || bytes > maximumBytes) {
    throw clientSessionError(
      code,
      `protocol message contains ${bytes} bytes, exceeding the negotiated ${maximumBytes}-byte limit`,
    );
  }
}

export function assertExactKeys(
  record: Readonly<Record<string, unknown>>,
  required: readonly string[],
  optional: readonly string[],
  name: string,
  code: A3sClientSessionErrorCodeV1,
): void {
  const allowed = new Set([...required, ...optional]);
  for (const key of required) {
    if (!Object.hasOwn(record, key)) {
      throw clientSessionError(code, `${name} is missing field ${JSON.stringify(key)}`);
    }
  }
  for (const key of Object.keys(record)) {
    if (!allowed.has(key)) {
      throw clientSessionError(code, `${name} contains unknown field ${JSON.stringify(key)}`);
    }
  }
}

export function requireRecord(
  value: unknown,
  name: string,
  code: A3sClientSessionErrorCodeV1,
): Readonly<Record<string, unknown>> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw clientSessionError(code, `${name} must be an object`);
  }
  return value as Readonly<Record<string, unknown>>;
}

export function snapshotProtocolValue(
  value: unknown,
  path: string,
  code: A3sClientSessionErrorCodeV1,
): unknown {
  return snapshotA3sProtocolJsonV1(
    value,
    path,
    (message) => clientSessionError(code, message),
  );
}
