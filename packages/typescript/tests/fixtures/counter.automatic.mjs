// This is the standard automatic-JSX shape emitted for the static counter.
import { Button, compileFrameV1, defineAction } from "../../src/index.ts";
import { jsx as _jsx } from "../../src/jsx-runtime.ts";

export const counterCompilation = compileFrameV1(
  "counter",
  _jsx(
    Button,
    {
      onPress: defineAction("increment"),
      children: "Count 0",
    },
    "increment",
  ),
);
