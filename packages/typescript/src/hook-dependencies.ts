import { A3sHookError } from "./hooks.ts";

export function snapshotDependencies(
  dependencies: readonly unknown[],
  hook: string,
): readonly unknown[] {
  if (!Array.isArray(dependencies)) {
    throw new A3sHookError(
      "invalidDependencies",
      `${hook} dependencies must be an array`,
    );
  }
  return Object.freeze([...dependencies]);
}

export function dependenciesEqual(
  previous: readonly unknown[],
  next: readonly unknown[],
): boolean {
  return previous.length === next.length &&
    previous.every((value, index) => Object.is(value, next[index]));
}
