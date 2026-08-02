import { createHash } from "node:crypto";
import { createReadStream } from "node:fs";
import { readFile, stat } from "node:fs/promises";
import { createRequire } from "node:module";
import { basename, dirname, isAbsolute, join } from "node:path";

import { TSX_PROTOCOL_VERSION_V1 } from "./generated/protocol.ts";

const DEVELOPMENT_HOST_ENVIRONMENT = "A3S_GUI_TSX_HOST";
const ALLOW_UNVERIFIED_HOST_ENVIRONMENT = "A3S_GUI_ALLOW_UNVERIFIED_HOST";
const MAXIMUM_MANIFEST_BYTES = 64 * 1024;
const MAXIMUM_EXECUTABLE_BYTES = 512 * 1024 * 1024;
const SHA256_PATTERN = /^[0-9a-f]{64}$/u;
const require = createRequire(import.meta.url);

export type A3sHostArtifactErrorCodeV1 =
  | "artifactMissing"
  | "checksumMismatch"
  | "invalidManifest"
  | "invalidOverride"
  | "packageMissing"
  | "unsupportedTarget";

export class A3sHostArtifactError extends Error {
  readonly code: A3sHostArtifactErrorCodeV1;

  constructor(code: A3sHostArtifactErrorCodeV1, message: string, cause?: unknown) {
    super(message, cause === undefined ? undefined : { cause });
    this.name = "A3sHostArtifactError";
    this.code = code;
  }
}

export interface A3sResolvedHostArtifactV1 {
  readonly command: string;
  readonly source: "environment" | "package";
  readonly packageName: string | null;
}

export interface ResolveA3sHostArtifactOptionsV1 {
  readonly environment?: Readonly<Record<string, string | undefined>>;
  readonly platform?: string;
  readonly arch?: string;
  readonly linuxAbi?: "gnu" | "musl";
  readonly resolvePackageManifest?: (packageName: string) => string;
}

interface HostTargetV1 {
  readonly platform: "darwin" | "linux" | "win32";
  readonly arch: "arm64" | "x64";
  readonly abi: "gnu" | "msvc" | "musl" | "none";
  readonly packageName: string;
}

interface HostManifestV1 {
  readonly packageName: string;
  readonly executable: string;
  readonly executableBytes: number;
  readonly sha256: string;
}

/** Resolves and validates the exact native host artifact for this Node target. */
export async function resolveA3sHostArtifactV1(
  options: ResolveA3sHostArtifactOptionsV1 = {},
): Promise<Readonly<A3sResolvedHostArtifactV1>> {
  const environment = options.environment ?? process.env;
  const override = environment[DEVELOPMENT_HOST_ENVIRONMENT];
  if (override !== undefined) {
    if (environment[ALLOW_UNVERIFIED_HOST_ENVIRONMENT] !== "1") {
      throw artifactError(
        "invalidOverride",
        `${DEVELOPMENT_HOST_ENVIRONMENT} requires ${ALLOW_UNVERIFIED_HOST_ENVIRONMENT}=1`,
      );
    }
    return resolveDevelopmentOverride(override);
  }

  const target = resolveHostTarget(
    options.platform ?? process.platform,
    options.arch ?? process.arch,
    options.linuxAbi,
  );
  const resolveManifest = options.resolvePackageManifest ?? defaultResolvePackageManifest;
  let manifestPath: string;
  try {
    manifestPath = resolveManifest(target.packageName);
  } catch (cause) {
    throw artifactError(
      "packageMissing",
      `native TSX host package ${JSON.stringify(target.packageName)} is not installed`,
      cause,
    );
  }
  if (typeof manifestPath !== "string" || !isAbsolute(manifestPath)) {
    throw artifactError(
      "invalidManifest",
      `native TSX host package ${JSON.stringify(target.packageName)} resolved a non-absolute manifest path`,
    );
  }

  const manifest = await readHostManifest(manifestPath, target);
  const command = join(dirname(manifestPath), manifest.executable);
  const executable = await fileMetadata(command, "artifactMissing", "native TSX host executable");
  if (executable.size !== manifest.executableBytes) {
    throw artifactError(
      "invalidManifest",
      `native TSX host executable has ${executable.size} bytes; manifest requires ${manifest.executableBytes}`,
    );
  }
  const checksum = await sha256File(command);
  if (checksum !== manifest.sha256) {
    throw artifactError(
      "checksumMismatch",
      `native TSX host executable checksum ${checksum} does not match its manifest`,
    );
  }

  return Object.freeze({
    command,
    source: "package" as const,
    packageName: manifest.packageName,
  });
}

async function resolveDevelopmentOverride(
  command: string,
): Promise<Readonly<A3sResolvedHostArtifactV1>> {
  if (
    typeof command !== "string" ||
    command.length === 0 ||
    command.length > 32_768 ||
    command.includes("\0") ||
    !isAbsolute(command)
  ) {
    throw artifactError(
      "invalidOverride",
      `${DEVELOPMENT_HOST_ENVIRONMENT} must be an absolute bounded executable path`,
    );
  }
  await fileMetadata(command, "invalidOverride", DEVELOPMENT_HOST_ENVIRONMENT);
  return Object.freeze({
    command,
    source: "environment" as const,
    packageName: null,
  });
}

function resolveHostTarget(
  platform: string,
  arch: string,
  linuxAbi: "gnu" | "musl" | undefined,
): HostTargetV1 {
  if (arch !== "arm64" && arch !== "x64") {
    throw artifactError(
      "unsupportedTarget",
      `A3S GUI does not publish a native TSX host for architecture ${JSON.stringify(arch)}`,
    );
  }
  if (platform === "darwin") {
    return {
      platform,
      arch,
      abi: "none",
      packageName: `@a3s/gui-host-darwin-${arch}`,
    };
  }
  if (platform === "win32") {
    return {
      platform,
      arch,
      abi: "msvc",
      packageName: `@a3s/gui-host-win32-${arch}-msvc`,
    };
  }
  if (platform === "linux") {
    const abi = linuxAbi ?? detectLinuxAbi();
    return {
      platform,
      arch,
      abi,
      packageName: `@a3s/gui-host-linux-${arch}-${abi}`,
    };
  }
  throw artifactError(
    "unsupportedTarget",
    `A3S GUI does not publish a native TSX host for platform ${JSON.stringify(platform)}`,
  );
}

function detectLinuxAbi(): "gnu" | "musl" {
  const report = process.report?.getReport();
  if (typeof report === "object" && report !== null) {
    const header = (report as { readonly header?: unknown }).header;
    if (
      typeof header === "object" &&
      header !== null &&
      typeof (header as Record<string, unknown>).glibcVersionRuntime === "string"
    ) {
      return "gnu";
    }
  }
  return "musl";
}

function defaultResolvePackageManifest(packageName: string): string {
  return require.resolve(`${packageName}/host-manifest.json`);
}

async function readHostManifest(
  manifestPath: string,
  target: HostTargetV1,
): Promise<HostManifestV1> {
  const metadata = await fileMetadata(
    manifestPath,
    "invalidManifest",
    `native TSX host manifest for ${target.packageName}`,
  );
  if (metadata.size > MAXIMUM_MANIFEST_BYTES) {
    throw artifactError(
      "invalidManifest",
      `native TSX host manifest exceeds ${MAXIMUM_MANIFEST_BYTES} bytes`,
    );
  }
  let value: unknown;
  try {
    value = JSON.parse(await readFile(manifestPath, "utf8"));
  } catch (cause) {
    throw artifactError("invalidManifest", "native TSX host manifest is not valid JSON", cause);
  }
  const record = requireExactManifestRecord(value);
  requireManifestInteger(record.schemaVersion, "schemaVersion", 1, 1);
  const packageName = requireManifestText(record.packageName, "packageName");
  if (packageName !== target.packageName) {
    throw artifactError(
      "invalidManifest",
      `native TSX host manifest names ${JSON.stringify(packageName)}; expected ${JSON.stringify(target.packageName)}`,
    );
  }
  requireManifestText(record.hostVersion, "hostVersion");
  requireManifestTarget(record.platform, "platform", target.platform);
  requireManifestTarget(record.arch, "arch", target.arch);
  requireManifestTarget(record.abi, "abi", target.abi);
  const minimum = requireManifestInteger(
    record.minimumProtocolVersion,
    "minimumProtocolVersion",
    1,
    TSX_PROTOCOL_VERSION_V1,
  );
  const maximum = requireManifestInteger(
    record.maximumProtocolVersion,
    "maximumProtocolVersion",
    TSX_PROTOCOL_VERSION_V1,
    Number.MAX_SAFE_INTEGER,
  );
  if (minimum > TSX_PROTOCOL_VERSION_V1 || maximum < TSX_PROTOCOL_VERSION_V1) {
    throw artifactError("invalidManifest", "native TSX host does not support protocol v1");
  }
  const executable = requireManifestText(record.executable, "executable");
  if (
    basename(executable) !== executable ||
    executable === "." ||
    executable === ".." ||
    executable.includes("/") ||
    executable.includes("\\")
  ) {
    throw artifactError("invalidManifest", "native TSX host executable must be one file name");
  }
  const executableBytes = requireManifestInteger(
    record.executableBytes,
    "executableBytes",
    1,
    MAXIMUM_EXECUTABLE_BYTES,
  );
  const sha256 = requireManifestText(record.sha256, "sha256");
  if (!SHA256_PATTERN.test(sha256)) {
    throw artifactError("invalidManifest", "native TSX host sha256 must be lowercase hexadecimal");
  }
  return { packageName, executable, executableBytes, sha256 };
}

function requireExactManifestRecord(value: unknown): Readonly<Record<string, unknown>> {
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw artifactError("invalidManifest", "native TSX host manifest must be an object");
  }
  const record = value as Readonly<Record<string, unknown>>;
  const expected = [
    "schemaVersion",
    "packageName",
    "hostVersion",
    "platform",
    "arch",
    "abi",
    "minimumProtocolVersion",
    "maximumProtocolVersion",
    "executable",
    "executableBytes",
    "sha256",
  ];
  const allowed = new Set(expected);
  for (const field of expected) {
    if (!Object.hasOwn(record, field)) {
      throw artifactError("invalidManifest", `native TSX host manifest is missing ${field}`);
    }
  }
  for (const field of Object.keys(record)) {
    if (!allowed.has(field)) {
      throw artifactError(
        "invalidManifest",
        `native TSX host manifest contains unknown field ${JSON.stringify(field)}`,
      );
    }
  }
  return record;
}

function requireManifestText(value: unknown, field: string): string {
  if (
    typeof value !== "string" ||
    value.trim().length === 0 ||
    value.length > 1_024 ||
    /[\u0000-\u001f\u007f-\u009f]/u.test(value)
  ) {
    throw artifactError("invalidManifest", `native TSX host ${field} is invalid`);
  }
  return value;
}

function requireManifestInteger(
  value: unknown,
  field: string,
  minimum: number,
  maximum: number,
): number {
  if (
    typeof value !== "number" ||
    !Number.isSafeInteger(value) ||
    value < minimum ||
    value > maximum
  ) {
    throw artifactError("invalidManifest", `native TSX host ${field} is invalid`);
  }
  return value;
}

function requireManifestTarget(value: unknown, field: string, expected: string): void {
  if (value !== expected) {
    throw artifactError(
      "invalidManifest",
      `native TSX host ${field} ${JSON.stringify(value)} does not match ${JSON.stringify(expected)}`,
    );
  }
}

async function fileMetadata(
  path: string,
  code: A3sHostArtifactErrorCodeV1,
  name: string,
): Promise<{ readonly size: number }> {
  try {
    const metadata = await stat(path);
    if (!metadata.isFile()) {
      throw artifactError(code, `${name} is not a file`);
    }
    return { size: metadata.size };
  } catch (cause) {
    if (cause instanceof A3sHostArtifactError) {
      throw cause;
    }
    throw artifactError(code, `${name} could not be read`, cause);
  }
}

async function sha256File(path: string): Promise<string> {
  const hash = createHash("sha256");
  try {
    for await (const chunk of createReadStream(path)) {
      hash.update(chunk);
    }
    return hash.digest("hex");
  } catch (cause) {
    throw artifactError("artifactMissing", "native TSX host executable could not be hashed", cause);
  }
}

function artifactError(
  code: A3sHostArtifactErrorCodeV1,
  message: string,
  cause?: unknown,
): A3sHostArtifactError {
  return new A3sHostArtifactError(code, message, cause);
}
