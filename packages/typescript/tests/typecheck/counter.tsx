import {
  A3sJsxError,
  Button,
  ErrorBoundary,
  Text,
  View,
  Window,
  RevisionActionRegistryV1,
  compileFrameV1,
  createApp,
  createContext,
  defineAction,
  useContext,
  useEffect,
  useMemo,
  useReducer,
  useRef,
  useState,
  type A3sJsxProps,
} from "@a3s/gui";

const Theme = createContext<"light" | "dark">("light");

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

const compiled = compileFrameV1(
  "counter-typecheck",
  <Counter count={0} onIncrement={() => increment.handler?.({})} />,
);

const callbacks = new RevisionActionRegistryV1();
callbacks.stage(1, compiled);
callbacks.state.pending?.renderRevision satisfies number | undefined;

function StatefulCounter() {
  const theme = useContext(Theme);
  const [count, setCount] = useState(0);
  const [offset, dispatch] = useReducer(
    (value: number, delta: number) => value + delta,
    0,
  );
  const renders = useRef(0);
  renders.current += 1;
  const value = useMemo(() => count + offset, [count, offset]);
  useEffect(() => () => undefined, [value]);

  return (
    <Window title="Stateful Counter" width={360} height={220}>
      <View>
        <Text>Value: {value}; renders: {renders.current}; theme: {theme}</Text>
        <Button
          onPress={() => {
            setCount((current) => current + 1);
            dispatch(1);
          }}
        >
          Increment
        </Button>
      </View>
    </Window>
  );
}

const contextAndBoundary = (
  <Theme.Provider value="dark">
    <ErrorBoundary
      fallback={(error) => {
        error satisfies A3sJsxError;
        return <Text>Failed</Text>;
      }}
    >
      <StatefulCounter />
    </ErrorBoundary>
  </Theme.Provider>
);
contextAndBoundary satisfies import("@a3s/gui").A3sElement;

// @ts-expect-error context providers retain their value type
const invalidContextValue = <Theme.Provider value="sepia" />;
invalidContextValue satisfies import("@a3s/gui").A3sElement;

// @ts-expect-error error boundaries require an explicit fallback
const missingBoundaryFallback = <ErrorBoundary><StatefulCounter /></ErrorBoundary>;
missingBoundaryFallback satisfies import("@a3s/gui").A3sElement;

const typeOnlyHost = {
  async submitRender(candidate: {
    readonly renderRevision: number;
    readonly frame: { readonly frameId: string };
  }) {
    candidate.renderRevision satisfies number;
    candidate.frame.frameId satisfies string;
    throw new Error("type-only host");
  },
};

const app = createApp(StatefulCounter, {
  frameId: "stateful-counter-typecheck",
  host: typeOnlyHost,
});
app.state.status satisfies "created" | "running" | "closing" | "closed";

createApp(Counter, {
  host: typeOnlyHost,
  props: { count: 0, onIncrement: () => undefined },
});
// @ts-expect-error required root props cannot be omitted
createApp(Counter, { host: typeOnlyHost });
