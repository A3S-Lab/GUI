import {
  Button,
  Text,
  View,
  Window,
  compileFrameV1,
  defineAction,
  type A3sJsxProps,
} from "@a3s/gui";

interface CounterProps extends A3sJsxProps {
  readonly count: number;
  readonly onIncrement: () => void;
}

function Counter({ count, onIncrement }: CounterProps) {
  return (
    <Window title="Counter" width={360} height={220}>
      <View className="flex-col gap-4 p-6">
        <Text>Count: {count}</Text>
        <Button key="increment" onPress={onIncrement}>
          Increment
        </Button>
      </View>
    </Window>
  );
}

const increment = defineAction("increment", () => undefined);

compileFrameV1(
  "counter-typecheck",
  <Counter count={0} onIncrement={() => increment.handler?.({})} />,
);
