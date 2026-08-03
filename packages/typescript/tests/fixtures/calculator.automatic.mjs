// This is the automatic-JSX shape emitted for the shared calculator scenario.
import {
  Button,
  Text,
  Toolbar,
  Window,
  compileFrameV1,
  defineAction,
} from "../../src/index.ts";
import { jsx as _jsx, jsxs as _jsxs } from "../../src/jsx-runtime.ts";

const actions = Object.freeze({
  backspace: defineAction("backspace"),
  clear: defineAction("clear"),
  clearEntry: defineAction("clearEntry"),
  percent: defineAction("percent"),
  pressDecimal: defineAction("pressDecimal"),
  pressDigit: defineAction("pressDigit"),
  pressEquals: defineAction("pressEquals"),
  pressOperator: defineAction("pressOperator"),
  reciprocal: defineAction("reciprocal"),
  square: defineAction("square"),
  squareRoot: defineAction("squareRoot"),
  toggleSign: defineAction("toggleSign"),
});

const buttonBase =
  "h-14 min-h-14 w-[94px] min-w-[94px] rounded-[5px] border border-[#e5e5e5] p-0 text-[#1b1b1b]";

function Space() {
  return " ";
}

function space(key) {
  return _jsx(Space, {}, key);
}

function text(key, label, className, attributes = {}) {
  return _jsx(Text, { label, className, ...attributes }, key);
}

function button(key, label, onPress, actionValue, className) {
  return _jsx(
    Button,
    {
      label,
      onPress,
      actionValue,
      className: `${buttonBase} ${className}`,
    },
    key,
  );
}

function toolbar(key, label, orientation, className, children) {
  return _jsxs(Toolbar, { label, orientation, className, children }, key);
}

function row(key, label, content, leadingContentSpace) {
  const children = [space(`${key.replace(/-root$/u, "")}-text-0`)];
  if (leadingContentSpace) {
    children.push(space(`${key.replace(/-root$/u, "")}-content-text-0`));
  }
  for (let index = 0; index < content.length; index += 1) {
    children.push(content[index]);
    children.push(space(`${key.replace(/-root$/u, "")}-content-text-${index + 1}`));
  }
  children.push(space(`${key.replace(/-root$/u, "")}-text-1`));
  return toolbar(
    key,
    label,
    "horizontal",
    "h-14 w-96 gap-[3px] bg-[#f3f3f3]",
    children,
  );
}

function titleBar() {
  return toolbar(
    "calculator-titlebar-root",
    "Calculator title",
    "horizontal",
    "h-[52px] w-[396px] gap-[6px] bg-[#f3f3f3] px-3 pb-[6px] pt-[10px]",
    [
      space("calculator-titlebar-text-0"),
      text(
        "calculator-titlebar-menu",
        "☰",
        "h-8 w-[34px] text-center text-[20px] font-normal text-[#1b1b1b]",
      ),
      text(
        "calculator-titlebar-mode",
        "Standard",
        "h-8 w-[212px] text-[20px] font-semibold text-[#1b1b1b]",
      ),
      space("calculator-titlebar-text-2"),
      text(
        "calculator-titlebar-history-button",
        "History",
        "h-8 w-24 text-right text-[13px] font-medium text-[#3b3b3b]",
      ),
    ],
  );
}

function display() {
  return toolbar(
    "calculator-display-root",
    "Display",
    "vertical",
    "h-[132px] w-[396px] gap-1 bg-[#f3f3f3] px-4 pb-[10px] pt-3",
    [
      space("calculator-display-text-0"),
      text(
        "calculator-display-history",
        "",
        "h-[26px] w-[364px] text-right text-[13px] font-normal text-[#737373]",
      ),
      space("calculator-display-text-1"),
      text(
        "calculator-display-value",
        "0",
        "h-[74px] w-[364px] text-right text-[48px] font-semibold leading-none text-[#1b1b1b] data-[error=true]:text-[#eb8e90]",
        { "data-error": false },
      ),
      space("calculator-display-text-2"),
    ],
  );
}

function memoryBar() {
  const labels = ["MC", "MR", "M+", "M-", "MS", "M⌄"];
  const keys = ["clear", "recall", "add", "subtract", "store", "list"];
  const children = [];
  for (let index = 0; index < labels.length; index += 1) {
    children.push(space(`calculator-memory-text-${index}`));
    children.push(
      text(
        `calculator-memory-memory-${keys[index]}`,
        labels[index],
        `h-[34px] w-[62px] text-center text-xs font-semibold ${
          index < 2 ? "text-[#8a8a8a]" : "text-[#1b1b1b]"
        }`,
      ),
    );
  }
  children.push(space("calculator-memory-text-6"));
  return toolbar(
    "calculator-memory-root",
    "Memory controls",
    "horizontal",
    "h-11 w-[396px] gap-0 bg-[#f3f3f3] px-2 py-1",
    children,
  );
}

function editRow() {
  return row(
    "calculator-keypad-row-edit-root-root",
    "Edit controls",
    [
      button(
        "calculator-keypad-row-edit-root-content-percent-root",
        "%",
        actions.percent,
        "",
        "bg-[#f9f9f9] text-[20px] font-normal",
      ),
      button(
        "calculator-keypad-row-edit-root-content-clear-entry-root",
        "CE",
        actions.clearEntry,
        "",
        "bg-[#f9f9f9] text-[15px] font-normal",
      ),
      button(
        "calculator-keypad-row-edit-root-content-clear-root",
        "C",
        actions.clear,
        "",
        "bg-[#f9f9f9] text-[15px] font-normal",
      ),
      button(
        "calculator-keypad-row-edit-root-content-backspace-root",
        "⌫",
        actions.backspace,
        "",
        "bg-[#f9f9f9] text-[18px] font-normal",
      ),
    ],
    true,
  );
}

function functionRow() {
  return row(
    "calculator-keypad-row-functions-root-root",
    "Function controls",
    [
      button(
        "calculator-keypad-row-functions-root-content-reciprocal-root",
        "1/x",
        actions.reciprocal,
        "",
        "bg-[#f9f9f9] text-[15px] font-normal",
      ),
      button(
        "calculator-keypad-row-functions-root-content-square-root",
        "x²",
        actions.square,
        "",
        "bg-[#f9f9f9] text-[15px] font-normal",
      ),
      button(
        "calculator-keypad-row-functions-root-content-square-root-root",
        "√x",
        actions.squareRoot,
        "",
        "bg-[#f9f9f9] text-[15px] font-normal",
      ),
      button(
        "calculator-keypad-row-functions-root-content-divide-root",
        "÷",
        actions.pressOperator,
        "/",
        "bg-[#f9f9f9] text-[20px] font-normal",
      ),
    ],
    true,
  );
}

function digitOperatorRow(prefix, label, digits, operator, operatorLabel, operatorValue) {
  return row(
    `${prefix}-root-root`,
    label,
    [
      ...digits.map(([key, value]) =>
        button(
          `${prefix}-root-content-${key}-root`,
          value,
          actions.pressDigit,
          value,
          "bg-white text-[20px] font-semibold",
        )),
      button(
        `${prefix}-root-content-${operator}-root`,
        operatorLabel,
        actions.pressOperator,
        operatorValue,
        "bg-[#f9f9f9] text-[20px] font-normal",
      ),
    ],
    prefix.endsWith("row-one"),
  );
}

function zeroRow() {
  const prefix = "calculator-keypad-row-zero";
  return row(
    `${prefix}-root-root`,
    "Sign zero decimal equals",
    [
      button(
        `${prefix}-root-content-toggle-sign-root`,
        "±",
        actions.toggleSign,
        "",
        "bg-white text-[20px] font-normal",
      ),
      button(
        `${prefix}-root-content-zero-root`,
        "0",
        actions.pressDigit,
        "0",
        "bg-white text-[20px] font-semibold",
      ),
      button(
        `${prefix}-root-content-decimal-root`,
        ".",
        actions.pressDecimal,
        "",
        "bg-white text-[20px] font-normal",
      ),
      button(
        `${prefix}-root-content-equals-root`,
        "=",
        actions.pressEquals,
        "=",
        "border-[#0067c0] bg-[#0067c0] text-[22px] font-semibold text-white",
      ),
    ],
    false,
  );
}

function keypad() {
  return toolbar(
    "calculator-keypad-root",
    "Calculator keypad",
    "vertical",
    "h-[390px] w-[396px] gap-[3px] bg-[#f3f3f3] px-[6px] pb-2 pt-1",
    [
      editRow(),
      space("calculator-keypad-text-1"),
      functionRow(),
      space("calculator-keypad-text-2"),
      digitOperatorRow(
        "calculator-keypad-row-seven",
        "Seven eight nine multiply",
        [["seven", "7"], ["eight", "8"], ["nine", "9"]],
        "multiply",
        "×",
        "*",
      ),
      digitOperatorRow(
        "calculator-keypad-row-four",
        "Four five six subtract",
        [["four", "4"], ["five", "5"], ["six", "6"]],
        "subtract",
        "−",
        "-",
      ),
      space("calculator-keypad-text-4"),
      digitOperatorRow(
        "calculator-keypad-row-one",
        "One two three add",
        [["one", "1"], ["two", "2"], ["three", "3"]],
        "add",
        "+",
        "+",
      ),
      space("calculator-keypad-text-5"),
      zeroRow(),
      space("calculator-keypad-text-6"),
    ],
  );
}

function calculator() {
  return toolbar(
    "calculator-root",
    "Calculator",
    "vertical",
    "h-[620px] w-[396px] gap-0 bg-[#f3f3f3] font-[Segoe_UI,Inter,-apple-system,system-ui,sans-serif] text-[#1b1b1b]",
    [
      space("calculator-text-0"),
      titleBar(),
      space("calculator-text-1"),
      display(),
      memoryBar(),
      space("calculator-text-3"),
      keypad(),
      space("calculator-text-4"),
    ],
  );
}

export const calculatorCompilation = compileFrameV1(
  "calculator",
  _jsx(Window, {
    title: "A3S Calculator",
    width: 410,
    height: 620,
    minWidth: 360,
    minHeight: 560,
    resizable: true,
    children: calculator(),
  }),
);
