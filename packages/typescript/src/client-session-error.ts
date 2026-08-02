export type A3sClientSessionErrorCodeV1 =
  | "frameTooLarge"
  | "invalidMessage"
  | "invalidMessageId"
  | "invalidRevision"
  | "invalidSession"
  | "invalidState"
  | "invalidWelcome"
  | "messageIdExhausted";

export class A3sClientSessionError extends Error {
  readonly code: A3sClientSessionErrorCodeV1;

  constructor(code: A3sClientSessionErrorCodeV1, message: string, cause?: unknown) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sClientSessionError";
    this.code = code;
  }
}

export function clientSessionError(
  code: A3sClientSessionErrorCodeV1,
  message: string,
  cause?: unknown,
): A3sClientSessionError {
  return new A3sClientSessionError(code, message, cause);
}
