export type CompiledJsxNode =
  | {
      kind: 'element';
      key: string;
      tag: string;
      importSource?: string;
      props?: CompiledProps;
      children?: CompiledJsxNode[];
    }
  | {kind: 'text'; key: string; value: string};

export interface CompiledProps {
  label?: string;
  textValue?: string;
  value?: string;
  placeholder?: string;
  action?: string;
  isDisabled?: boolean;
  isRequired?: boolean;
  isInvalid?: boolean;
  isSelected?: boolean;
  isChecked?: boolean;
  isExpanded?: boolean;
  orientation?: 'horizontal' | 'vertical';
  minValue?: number;
  maxValue?: number;
  valueNumber?: number;
  stepValue?: number;
  name?: string;
  form?: string;
  inputType?: string;
  accept?: string;
  capture?: string;
  alt?: string;
  href?: string;
  src?: string;
  srcset?: string;
  sizes?: string;
  media?: string;
  resourceType?: string;
  intrinsicWidth?: number;
  intrinsicHeight?: number;
  loading?: string;
  decoding?: string;
  fetchPriority?: string;
  crossOrigin?: string;
  referrerPolicy?: string;
  poster?: string;
  controls?: boolean;
  autoplay?: boolean;
  loopPlayback?: boolean;
  muted?: boolean;
  playsInline?: boolean;
  preload?: string;
  trackKind?: string;
  srclang?: string;
  trackLabel?: string;
  defaultTrack?: boolean;
  list?: string;
  dirname?: string;
  formAction?: string;
  formEnctype?: string;
  formMethod?: string;
  formTarget?: string;
  formNoValidate?: boolean;
  id?: string;
  className?: string;
  style?: Record<string, string | number | boolean>;
  attributes?: Record<string, string>;
  events?: Record<string, string>;
}

export interface UiAction {
  id: string;
  label?: string;
}

export interface ActionHandler {
  readonly a3sAction?: string;
  readonly a3sLabel?: string;
  (...args: unknown[]): void;
}

export type ActionLike = string | ActionHandler | ((...args: unknown[]) => void);

export interface WindowOptions {
  title: string;
  width?: number;
  height?: number;
  resizable?: boolean;
}

export interface UiFrame {
  frameId: string;
  root: CompiledJsxNode;
  actions: UiAction[];
  window?: WindowOptions;
}

export interface HostEvent {
  frameId: string;
  event: {
    node: number;
    kind: 'press' | 'change' | 'selectionChange' | 'toggle' | 'focus' | 'blur';
    value?: string;
  };
}

export function createAction(id: string, label?: string): ActionHandler;
export const action: typeof createAction;
export function defineAction(id: string, label?: string): UiAction;
export function defineAction(action: ActionHandler, label?: string): UiAction;
export function createUiFrame(
  frameId: string,
  root: CompiledJsxNode,
  options?: {actions?: UiAction[]; window?: WindowOptions},
): UiFrame;
export function createHostEvent(
  frameId: string,
  node: number,
  kind: HostEvent['event']['kind'],
  value?: string,
): HostEvent;

export * from './jsx-runtime.js';
