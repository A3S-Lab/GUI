export interface CompiledJsxElement {
  kind: 'element';
  key: string;
  tag: string;
  importSource?: string;
  props?: CompiledProps;
  children?: CompiledJsxNode[];
}

export interface CompiledJsxText {
  kind: 'text';
  key: string;
  value: string;
}

export type CompiledJsxNode = CompiledJsxElement | CompiledJsxText;

export interface CompiledProps {
  label?: string;
  textValue?: string;
  value?: string;
  placeholder?: string;
  action?: string;
  ariaLabel?: string;
  'aria-label'?: string;
  isDisabled?: boolean;
  isRequired?: boolean;
  isInvalid?: boolean;
  isReadOnly?: boolean;
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
  actionLabels?: Record<string, string>;
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
  minWidth?: number;
  minHeight?: number;
  maxWidth?: number;
  maxHeight?: number;
  resizable?: boolean;
}

export interface UiFrame {
  frameId: string;
  root: CompiledJsxElement;
  actions?: UiAction[];
  window?: WindowOptions;
}

export interface NativeEvent {
  node: number;
  kind:
    | 'press'
    | 'change'
    | 'selectionChange'
    | 'toggle'
    | 'focus'
    | 'blur'
    | 'keyDown'
    | 'keyUp';
  value?: string | null;
}

export interface HostEvent {
  frameId: string;
  event: NativeEvent;
}

export interface AccessibilityRelationshipProps {
  labelledBy?: string | null;
  describedBy?: string | null;
  details?: string | null;
  controls?: string | null;
  owns?: string | null;
  flowTo?: string | null;
  errorMessage?: string | null;
  activeDescendant?: string | null;
}

export interface AccessibilityDescriptionProps {
  description?: string | null;
  roleDescription?: string | null;
  keyShortcuts?: string | null;
  valueText?: string | null;
}

export interface AccessibilityStructureProps {
  level?: number | null;
  positionInSet?: number | null;
  setSize?: number | null;
  rowCount?: number | null;
  rowIndex?: number | null;
  rowSpan?: number | null;
  columnCount?: number | null;
  columnIndex?: number | null;
  columnSpan?: number | null;
  rowIndexText?: string | null;
  columnIndexText?: string | null;
  sort?: string | null;
}

export interface AccessibilityStateProps {
  hidden?: boolean | null;
  autocomplete?: string | null;
  multiline?: boolean | null;
  current?: string | null;
  hasPopup?: string | null;
  pressed?: string | null;
  live?: string | null;
  atomic?: boolean | null;
  busy?: boolean | null;
  relevant?: string | null;
  modal?: boolean | null;
}

export interface AccessibilityNode {
  node?: number | null;
  role: string;
  label?: string | null;
  value?: string | null;
  relationships: AccessibilityRelationshipProps;
  description: AccessibilityDescriptionProps;
  structure: AccessibilityStructureProps;
  state: AccessibilityStateProps;
  disabled: boolean;
  required: boolean;
  invalid: boolean;
  readOnly: boolean;
  multiple: boolean;
  focused: boolean;
  selected: boolean;
  checked?: boolean | null;
  expanded?: boolean | null;
  children: AccessibilityNode[];
}

export interface NativePlatformCommand {
  type: string;
  [key: string]: unknown;
}

export interface NativeRenderResponse {
  frameId: string;
  root: number;
  commands: NativePlatformCommand[];
  accessibilityTree?: AccessibilityNode | null;
}

export interface ActionInvocation {
  node: number;
  action: string;
  event: HostEvent['event']['kind'];
  value?: string | null;
}

export interface InteractionNodeState {
  focused: boolean;
  value?: string | null;
  selected: boolean;
  checked?: boolean | null;
  expanded?: boolean | null;
}

export interface InteractionChange {
  node: number;
  before: InteractionNodeState;
  after: InteractionNodeState;
}

export interface HostEventResponse {
  frameId: string;
  invocation: ActionInvocation;
  interactionChanges?: InteractionChange[];
}

export interface NativeHostEventResponse {
  frameId: string;
  invocation?: ActionInvocation | null;
  accessibilityTree?: AccessibilityNode | null;
  interactionChanges?: InteractionChange[];
}

export interface HandledNativeEvent {
  event: NativeEvent;
  invocation: ActionInvocation | null;
  interactionChanges: InteractionChange[];
}

export function createAction(id: string, label?: string): ActionHandler;
export const action: typeof createAction;
export function defineAction(id: string, label?: string): UiAction;
export function defineAction(action: ActionHandler, label?: string): UiAction;
export function createUiFrame(
  frameId: string,
  root: CompiledJsxElement,
  options?: {actions?: UiAction[]; window?: WindowOptions},
): UiFrame;
export function createHostEvent(
  frameId: string,
  node: number,
  kind: HostEvent['event']['kind'],
  value?: string,
): HostEvent;
export function createHandledNativeEvent(
  event: NativeEvent,
  options?: {
    invocation?: ActionInvocation | null;
    interactionChanges?: InteractionChange[];
  },
): HandledNativeEvent;

export * from './jsx-runtime.js';
