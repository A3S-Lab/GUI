import type { A3sContext } from "./context.ts";

export class ContextValueStack {
  readonly #values = new Map<A3sContext<unknown>, unknown[]>();

  withValue<Value, Result>(
    context: A3sContext<Value>,
    value: Value,
    callback: () => Result,
  ): Result {
    const key = context as A3sContext<unknown>;
    let stack = this.#values.get(key);
    if (stack === undefined) {
      stack = [];
      this.#values.set(key, stack);
    }
    stack.push(value);
    try {
      return callback();
    } finally {
      stack.pop();
      if (stack.length === 0) {
        this.#values.delete(key);
      }
    }
  }

  read<Value>(context: A3sContext<Value>): Value {
    const stack = this.#values.get(context as A3sContext<unknown>);
    return stack === undefined || stack.length === 0
      ? context.defaultValue
      : stack[stack.length - 1] as Value;
  }
}
