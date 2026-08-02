import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process";

import { snapshotA3sProtocolJsonV1 } from "./protocol-json.ts";
import {
  type A3sByteTransportV1,
  A3sTransportError,
  transportError,
} from "./transport.ts";

const DEFAULT_MAXIMUM_STDERR_BYTES = 64 * 1_024;
const HARD_MAXIMUM_STDERR_BYTES = 1_024 * 1_024;
const DEFAULT_SHUTDOWN_TIMEOUT_MS = 5_000;
const MAXIMUM_SHUTDOWN_TIMEOUT_MS = 60_000;
const MAXIMUM_PROCESS_OPTION_TEXT_BYTES = 1_024 * 1_024;
const MAXIMUM_PROCESS_OPTION_ITEMS = 4_096;

export interface SpawnA3sNodeProcessOptionsV1 {
  readonly command: string;
  readonly args?: readonly string[];
  readonly cwd?: string;
  readonly env?: Readonly<Record<string, string>>;
  readonly maximumStderrBytes?: number;
  readonly shutdownTimeoutMs?: number;
}

export type A3sNodeProcessStatusV1 =
  | "running"
  | "closing"
  | "exited"
  | "failed"
  | "closed";

export interface A3sNodeProcessStateV1 {
  readonly status: A3sNodeProcessStatusV1;
  readonly pid: number | null;
  readonly exitCode: number | null;
  readonly signal: NodeJS.Signals | null;
  readonly stderr: string;
  readonly stderrTruncated: boolean;
}

interface ProcessTerminationV1 {
  readonly exitCode: number | null;
  readonly signal: NodeJS.Signals | null;
  readonly spawnError: Error | null;
}

/** Explicit no-shell child process used as a TSX byte transport. */
export class A3sNodeProcessTransportV1 implements A3sByteTransportV1 {
  readonly #child: ChildProcessWithoutNullStreams;
  readonly #maximumStderrBytes: number;
  readonly #shutdownTimeoutMs: number;
  readonly #termination: Promise<ProcessTerminationV1>;
  readonly #stderrTask: Promise<void>;
  readonly #stderrBuffer: Uint8Array;
  #stderrBytes = 0;
  #stderrTruncated = false;
  #status: A3sNodeProcessStatusV1 = "running";
  #terminationState: ProcessTerminationV1 | null = null;
  #closePromise: Promise<void> | null = null;

  readonly incoming: AsyncIterable<Uint8Array>;

  constructor(options: SpawnA3sNodeProcessOptionsV1) {
    const validated = validateSpawnOptions(options);
    this.#maximumStderrBytes = validated.maximumStderrBytes;
    this.#shutdownTimeoutMs = validated.shutdownTimeoutMs;
    this.#stderrBuffer = new Uint8Array(this.#maximumStderrBytes);
    try {
      this.#child = spawn(validated.command, validated.args, {
        cwd: validated.cwd,
        env: validated.env,
        shell: false,
        stdio: ["pipe", "pipe", "pipe"],
        windowsHide: true,
      });
    } catch (cause) {
      throw transportError(
        "processSpawnFailed",
        `could not spawn TSX host command ${JSON.stringify(validated.command)}`,
        cause,
      );
    }

    this.#termination = new Promise((resolve) => {
      let settled = false;
      const settle = (termination: ProcessTerminationV1) => {
        if (settled) {
          return;
        }
        settled = true;
        this.#terminationState = termination;
        if (termination.spawnError !== null || termination.exitCode !== 0) {
          this.#status = "failed";
        } else if (this.#status !== "closing") {
          this.#status = "exited";
        }
        resolve(termination);
      };
      this.#child.once("error", (error) => {
        settle({ exitCode: null, signal: null, spawnError: error });
      });
      this.#child.once("exit", (exitCode, signal) => {
        settle({ exitCode, signal, spawnError: null });
      });
    });
    this.#child.stdin.on("error", () => {
      // Individual writes surface their callback error; keep late EPIPE events handled.
    });
    this.#stderrTask = this.#captureStderr();
    this.incoming = this.#readStdout();
  }

  get state(): Readonly<A3sNodeProcessStateV1> {
    return Object.freeze({
      status: this.#status,
      pid: this.#child.pid ?? null,
      exitCode: this.#terminationState?.exitCode ?? null,
      signal: this.#terminationState?.signal ?? null,
      stderr: new TextDecoder().decode(this.#stderrBuffer.subarray(0, this.#stderrBytes)),
      stderrTruncated: this.#stderrTruncated,
    });
  }

  async write(chunk: Uint8Array): Promise<void> {
    if (!(chunk instanceof Uint8Array)) {
      throw transportError("writeFailed", "process transport writes must be Uint8Array values");
    }
    if (this.#status !== "running") {
      throw this.#exitError("cannot write to a TSX host process that is not running");
    }
    const bytes = Uint8Array.from(chunk);
    try {
      await new Promise<void>((resolve, reject) => {
        this.#child.stdin.write(bytes, (error) => {
          if (error === null || error === undefined) {
            resolve();
          } else {
            reject(error);
          }
        });
      });
    } catch (cause) {
      if (cause instanceof A3sTransportError) {
        throw cause;
      }
      throw transportError("writeFailed", "could not write TSX host stdin", cause);
    }
  }

  close(): Promise<void> {
    if (this.#closePromise !== null) {
      return this.#closePromise;
    }
    this.#closePromise = this.#close();
    return this.#closePromise;
  }

  async *#readStdout(): AsyncGenerator<Uint8Array> {
    try {
      for await (const chunk of this.#child.stdout) {
        if (!(chunk instanceof Uint8Array)) {
          throw transportError("streamFailed", "TSX host stdout yielded a non-byte chunk");
        }
        yield Uint8Array.from(chunk);
      }
    } catch (cause) {
      if (cause instanceof A3sTransportError) {
        throw cause;
      }
      throw transportError("streamFailed", "could not read TSX host stdout", cause);
    }

    const termination = await this.#termination;
    await this.#stderrTask;
    if (termination.spawnError !== null) {
      throw transportError(
        "processSpawnFailed",
        `TSX host process failed to start: ${termination.spawnError.message}`,
        termination.spawnError,
      );
    }
    if (termination.exitCode !== 0) {
      throw this.#exitError("TSX host process exited abnormally");
    }
  }

  async #captureStderr(): Promise<void> {
    try {
      for await (const chunk of this.#child.stderr) {
        if (!(chunk instanceof Uint8Array)) {
          continue;
        }
        const remaining = this.#maximumStderrBytes - this.#stderrBytes;
        if (remaining > 0) {
          const retainedBytes = Math.min(chunk.byteLength, remaining);
          this.#stderrBuffer.set(chunk.subarray(0, retainedBytes), this.#stderrBytes);
          this.#stderrBytes += retainedBytes;
        }
        if (chunk.byteLength > remaining) {
          this.#stderrTruncated = true;
        }
      }
    } catch {
      this.#stderrTruncated = true;
    }
  }

  async #close(): Promise<void> {
    if (this.#status === "closed") {
      return;
    }
    this.#status = "closing";
    if (!this.#child.stdin.destroyed) {
      this.#child.stdin.end();
    }

    let termination = await waitForTermination(
      this.#termination,
      this.#shutdownTimeoutMs,
    );
    if (termination === null) {
      this.#child.kill();
      termination = await waitForTermination(this.#termination, 1_000);
    }
    if (termination === null) {
      this.#status = "failed";
      throw transportError(
        "shutdownFailed",
        `TSX host process ${this.#child.pid ?? "unknown"} did not exit after termination`,
      );
    }
    await this.#stderrTask;
    this.#status = "closed";
  }

  #exitError(message: string): A3sTransportError {
    const termination = this.#terminationState;
    const detail = termination?.spawnError !== null && termination?.spawnError !== undefined
      ? `: ${termination.spawnError.message}`
      : termination === null
        ? ""
        : termination.signal !== null
          ? ` with signal ${termination.signal}`
          : ` with code ${termination.exitCode}`;
    const stderr = this.state.stderr;
    const stderrDetail = stderr.length === 0 ? "" : `; stderr: ${JSON.stringify(stderr)}`;
    return transportError("processExited", `${message}${detail}${stderrDetail}`);
  }
}

export function spawnA3sNodeProcessTransportV1(
  options: SpawnA3sNodeProcessOptionsV1,
): A3sNodeProcessTransportV1 {
  return new A3sNodeProcessTransportV1(options);
}

function validateSpawnOptions(options: SpawnA3sNodeProcessOptionsV1): {
  command: string;
  args: string[];
  cwd: string | undefined;
  env: Record<string, string> | undefined;
  maximumStderrBytes: number;
  shutdownTimeoutMs: number;
} {
  const snapshot = snapshotA3sProtocolJsonV1(
    options,
    "process transport options",
    (message) => transportError("invalidOptions", message),
  ) as Record<string, unknown>;
  const allowed = new Set([
    "command",
    "args",
    "cwd",
    "env",
    "maximumStderrBytes",
    "shutdownTimeoutMs",
  ]);
  for (const key of Object.keys(snapshot)) {
    if (!allowed.has(key)) {
      throw transportError(
        "invalidOptions",
        `process transport options contain unknown field ${JSON.stringify(key)}`,
      );
    }
  }
  const command = requireProcessText(snapshot.command, "process command");
  const args = snapshot.args === undefined
    ? []
    : requireStringArray(snapshot.args, "process arguments");
  const cwd = snapshot.cwd === undefined
    ? undefined
    : requireProcessText(snapshot.cwd, "process working directory");
  const env = snapshot.env === undefined ? undefined : requireEnvironment(snapshot.env);
  return {
    command,
    args,
    cwd,
    env,
    maximumStderrBytes: requireBoundedInteger(
      snapshot.maximumStderrBytes ?? DEFAULT_MAXIMUM_STDERR_BYTES,
      "maximum stderr bytes",
      1,
      HARD_MAXIMUM_STDERR_BYTES,
    ),
    shutdownTimeoutMs: requireBoundedInteger(
      snapshot.shutdownTimeoutMs ?? DEFAULT_SHUTDOWN_TIMEOUT_MS,
      "shutdown timeout",
      1,
      MAXIMUM_SHUTDOWN_TIMEOUT_MS,
    ),
  };
}

function requireEnvironment(value: unknown): Record<string, string> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw transportError("invalidOptions", "process environment must be an object");
  }
  const entries = Object.entries(value);
  if (entries.length > MAXIMUM_PROCESS_OPTION_ITEMS) {
    throw transportError(
      "invalidOptions",
      `process environment must contain at most ${MAXIMUM_PROCESS_OPTION_ITEMS} entries`,
    );
  }
  const result: Record<string, string> = {};
  let totalBytes = 0;
  for (const [key, entry] of entries) {
    const name = requireProcessText(key, "environment name");
    if (name.includes("=")) {
      throw transportError("invalidOptions", "environment names cannot contain equals signs");
    }
    const text = requireProcessValue(entry, `environment value ${JSON.stringify(name)}`);
    totalBytes += Buffer.byteLength(name) + Buffer.byteLength(text);
    if (totalBytes > MAXIMUM_PROCESS_OPTION_TEXT_BYTES) {
      throw transportError(
        "invalidOptions",
        `process environment exceeds ${MAXIMUM_PROCESS_OPTION_TEXT_BYTES} text bytes`,
      );
    }
    result[name] = text;
  }
  return result;
}

function requireStringArray(value: unknown, name: string): string[] {
  if (!Array.isArray(value) || value.length > MAXIMUM_PROCESS_OPTION_ITEMS) {
    throw transportError(
      "invalidOptions",
      `${name} must contain at most ${MAXIMUM_PROCESS_OPTION_ITEMS} strings`,
    );
  }
  const result: string[] = [];
  let totalBytes = 0;
  for (let index = 0; index < value.length; index += 1) {
    const text = requireProcessValue(value[index], `${name}[${index}]`);
    totalBytes += Buffer.byteLength(text);
    if (totalBytes > MAXIMUM_PROCESS_OPTION_TEXT_BYTES) {
      throw transportError(
        "invalidOptions",
        `${name} exceed ${MAXIMUM_PROCESS_OPTION_TEXT_BYTES} text bytes`,
      );
    }
    result.push(text);
  }
  return result;
}

function requireProcessText(value: unknown, name: string): string {
  if (
    typeof value !== "string" ||
    value.length === 0 ||
    value.length > 32_768
  ) {
    throw transportError("invalidOptions", `${name} must be non-empty bounded text without NUL`);
  }
  return requireProcessValue(value, name);
}

function requireProcessValue(value: unknown, name: string): string {
  if (
    typeof value !== "string" ||
    value.length > 32_768 ||
    value.includes("\0")
  ) {
    throw transportError("invalidOptions", `${name} must be bounded text without NUL`);
  }
  return value;
}

function requireBoundedInteger(
  value: unknown,
  name: string,
  minimum: number,
  maximum: number,
): number {
  if (
    typeof value !== "number" ||
    !Number.isSafeInteger(value) ||
    value < minimum ||
    value > maximum
  ) {
    throw transportError(
      "invalidOptions",
      `${name} must be an integer from ${minimum} through ${maximum}`,
    );
  }
  return value;
}

async function waitForTermination(
  termination: Promise<ProcessTerminationV1>,
  timeoutMs: number,
): Promise<ProcessTerminationV1 | null> {
  let timer: ReturnType<typeof setTimeout> | undefined;
  try {
    return await Promise.race([
      termination,
      new Promise<null>((resolve) => {
        timer = setTimeout(() => resolve(null), timeoutMs);
      }),
    ]);
  } finally {
    if (timer !== undefined) {
      clearTimeout(timer);
    }
  }
}
