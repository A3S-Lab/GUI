import { randomUUID } from "node:crypto";
import { createRequire } from "node:module";

import {
  connectA3sNodeApplicationHostV1,
  type A3sHostEventHandlerV1,
} from "./application-host.ts";
import type {
  A3sApplicationHostV1,
  A3sApplicationV1,
} from "./application.ts";
import { resolveA3sHostArtifactV1 } from "./host-artifact.ts";
import type { A3sJsxProps } from "./element.ts";

const require = createRequire(import.meta.url);
const A3S_TYPESCRIPT_SDK_VERSION = packageVersion();

/** Supplies one negotiated native host without owning application state. */
export interface A3sApplicationRuntimeV1 {
  connect(onEvent: A3sHostEventHandlerV1): Promise<A3sApplicationHostV1>;
}

export interface A3sApplicationRunOptionsV1 {
  readonly runtime?: A3sApplicationRuntimeV1;
}

type ApplicationFactoryV1<Props extends A3sJsxProps> = (
  host: A3sApplicationHostV1,
) => A3sApplicationV1<Props>;

/** Default Node runtime that launches the validated platform host artifact. */
export class A3sNodeApplicationRuntimeV1 implements A3sApplicationRuntimeV1 {
  async connect(onEvent: A3sHostEventHandlerV1): Promise<A3sApplicationHostV1> {
    if (typeof onEvent !== "function") {
      throw new TypeError("A3S application runtime requires an event handler");
    }
    const artifact = await resolveA3sHostArtifactV1();
    return connectA3sNodeApplicationHostV1({
      process: { command: artifact.command },
      handshake: {
        sdkVersion: A3S_TYPESCRIPT_SDK_VERSION,
        sessionId: randomUUID(),
        requestedRenderer: "auto",
      },
      onEvent,
    });
  }
}

/** One-shot hostless application definition returned by `createApp`. */
export class A3sApplicationRunnerV1<Props extends A3sJsxProps = A3sJsxProps> {
  readonly #factory: ApplicationFactoryV1<Props>;
  #status: "created" | "starting" | "started" | "failed" = "created";

  constructor(factory: ApplicationFactoryV1<Props>) {
    if (typeof factory !== "function") {
      throw new TypeError("A3S application runner requires an application factory");
    }
    this.#factory = factory;
  }

  async run(
    options: A3sApplicationRunOptionsV1 = {},
  ): Promise<A3sApplicationV1<Props>> {
    if (this.#status !== "created") {
      throw new Error(`cannot run an A3S application runner in ${this.#status} state`);
    }
    const runtime = validateRunOptions(options);
    this.#status = "starting";

    let application: A3sApplicationV1<Props> | null = null;
    let host: A3sApplicationHostV1 | null = null;
    try {
      host = await runtime.connect((message) => {
        if (application === null) {
          throw new Error("native TSX host emitted an event before the application was bound");
        }
        return application.dispatch(message).then(() => undefined);
      });
      validateConnectedHost(host);
      application = this.#factory(host);
      await application.start();
      this.#status = "started";
      return application;
    } catch (cause) {
      this.#status = "failed";
      try {
        if (application !== null) {
          await application.shutdown();
        } else {
          await host?.close?.();
        }
      } catch (cleanupError) {
        const aggregate = new AggregateError(
          [cause, cleanupError],
          "A3S application startup failed and its native host could not close cleanly",
        );
        throw aggregate;
      }
      throw cause;
    }
  }
}

function packageVersion(): string {
  const metadata = require("../package.json") as unknown;
  if (typeof metadata !== "object" || metadata === null) {
    throw new TypeError("@a3s/gui package metadata must be an object");
  }
  const version = (metadata as Record<string, unknown>).version;
  if (
    typeof version !== "string" ||
    version.trim().length === 0 ||
    version.length > 1_024 ||
    /[\u0000-\u001f\u007f-\u009f]/u.test(version)
  ) {
    throw new TypeError("@a3s/gui package version is invalid");
  }
  return version;
}

function validateRunOptions(options: A3sApplicationRunOptionsV1): A3sApplicationRuntimeV1 {
  if (!isPlainRecord(options)) {
    throw new TypeError("A3S application run options must be a plain object");
  }
  const descriptors = Object.getOwnPropertyDescriptors(options);
  for (const key of Reflect.ownKeys(descriptors)) {
    if (key !== "runtime") {
      throw new TypeError(`A3S application run options contain unknown field ${String(key)}`);
    }
    const descriptor = descriptors[key];
    if (descriptor === undefined || !("value" in descriptor)) {
      throw new TypeError(`A3S application run option ${String(key)} cannot be an accessor`);
    }
  }
  const runtime = descriptors.runtime === undefined
    ? new A3sNodeApplicationRuntimeV1()
    : descriptors.runtime.value;
  if (
    typeof runtime !== "object" ||
    runtime === null ||
    typeof (runtime as A3sApplicationRuntimeV1).connect !== "function"
  ) {
    throw new TypeError("A3S application run runtime must implement connect");
  }
  return runtime as A3sApplicationRuntimeV1;
}

function validateConnectedHost(host: A3sApplicationHostV1): void {
  if (
    typeof host !== "object" ||
    host === null ||
    typeof host.submitRender !== "function"
  ) {
    throw new TypeError("A3S application runtime returned an invalid host");
  }
}

function isPlainRecord(value: unknown): value is Record<string, unknown> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}
