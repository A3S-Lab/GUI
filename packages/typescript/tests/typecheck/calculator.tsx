import {
  Button,
  Text,
  Toolbar,
  Window,
  compileFrameV1,
  defineAction,
  type A3sJsxProps,
} from "@a3s/gui";

const pressDigit = defineAction("pressDigit");
const pressOperator = defineAction("pressOperator");
const pressEquals = defineAction("pressEquals");

interface CalculatorKey {
  readonly key: string;
  readonly label: string;
  readonly action: typeof pressDigit;
  readonly value: string;
}

const digitKeys: readonly CalculatorKey[] = [
  { key: "seven", label: "7", action: pressDigit, value: "7" },
  { key: "eight", label: "8", action: pressDigit, value: "8" },
  { key: "nine", label: "9", action: pressDigit, value: "9" },
  { key: "four", label: "4", action: pressDigit, value: "4" },
  { key: "five", label: "5", action: pressDigit, value: "5" },
  { key: "six", label: "6", action: pressDigit, value: "6" },
  { key: "one", label: "1", action: pressDigit, value: "1" },
  { key: "two", label: "2", action: pressDigit, value: "2" },
  { key: "three", label: "3", action: pressDigit, value: "3" },
  { key: "zero", label: "0", action: pressDigit, value: "0" },
];

function Calculator(_props: A3sJsxProps) {
  return (
    <Window
      title="A3S Calculator"
      width={410}
      height={620}
      minWidth={360}
      minHeight={560}
    >
      <Toolbar
        key="calculator-root"
        label="Calculator"
        orientation="vertical"
        className="h-[620px] w-[396px] bg-[#f3f3f3]"
      >
        <Text key="display" label="0" />
        <Toolbar key="keypad" label="Calculator keypad" orientation="vertical">
          {digitKeys.map((item) => (
            <Button
              key={item.key}
              label={item.label}
              actionValue={item.value}
              onPress={item.action}
            />
          ))}
          <Button
            key="add"
            label="+"
            actionValue="+"
            onPress={pressOperator}
          />
          <Button
            key="equals"
            label="="
            actionValue="="
            onPress={pressEquals}
          />
        </Toolbar>
      </Toolbar>
    </Window>
  );
}

const compilation = compileFrameV1("calculator-typecheck", <Calculator />);
if (compilation.frame.root.kind === "element") {
  compilation.frame.root.tag satisfies string;
}
compilation.frame.actions satisfies readonly import("@a3s/gui/protocol").ProtocolUiActionV1[];
