import type {ActionLike, CompiledJsxNode} from './index.js';

export const Fragment: unique symbol;

export type StyleValue = string | number | boolean | null | undefined;

export interface WebCompatibleProps {
  key?: string | number;
  id?: string;
  title?: string;
  hidden?: boolean | string;
  lang?: string;
  dir?: 'ltr' | 'rtl' | 'auto' | string;
  tabIndex?: number | string;
  tabindex?: number | string;
  role?: string;
  accessKey?: string;
  accesskey?: string;
  contentEditable?: boolean | 'true' | 'false' | 'plaintext-only' | string;
  contenteditable?: boolean | 'true' | 'false' | 'plaintext-only' | string;
  draggable?: boolean | 'true' | 'false' | 'auto' | string;
  spellCheck?: boolean | 'true' | 'false' | string;
  spellcheck?: boolean | 'true' | 'false' | string;
  translate?: boolean | 'yes' | 'no' | string;
  inert?: boolean | string;
  popover?: boolean | 'auto' | 'manual' | string;
  name?: string;
  href?: string;
  src?: string;
  srcSet?: string;
  srcset?: string;
  sizes?: string;
  media?: string;
  type?: string;
  download?: boolean | string;
  ping?: string;
  rel?: string;
  hrefLang?: string;
  hreflang?: string;
  as?: string;
  integrity?: string;
  blocking?: string;
  nonce?: string;
  imageSrcSet?: string;
  imagesrcset?: string;
  imageSizes?: string;
  imagesizes?: string;
  async?: boolean;
  defer?: boolean;
  noModule?: boolean;
  nomodule?: boolean;
  allow?: string;
  allowFullScreen?: boolean;
  allowfullscreen?: boolean;
  sandbox?: boolean | string;
  srcDoc?: string;
  srcdoc?: string;
  form?: string;
  width?: number | string;
  height?: number | string;
  className?: string;
  style?: Record<string, StyleValue> | string;
  children?: unknown;
  disabled?: boolean;
  required?: boolean;
  readOnly?: boolean;
  multiple?: boolean;
  autoFocus?: boolean;
  invalid?: boolean;
  selected?: boolean;
  checked?: boolean;
  defaultChecked?: boolean;
  expanded?: boolean;
  onClick?: ActionLike;
  onPress?: ActionLike;
  onChange?: ActionLike;
  onSelectionChange?: ActionLike;
  onFocus?: ActionLike;
  onBlur?: ActionLike;
  isDisabled?: boolean;
  isRequired?: boolean;
  isInvalid?: boolean;
  isSelected?: boolean;
  isChecked?: boolean;
  isExpanded?: boolean;
  textValue?: string;
  value?: string | number | boolean;
  defaultValue?: string | number | boolean;
  placeholder?: string;
  autocomplete?: string;
  autoComplete?: string;
  inputMode?: string;
  accept?: string;
  capture?: boolean | string;
  alt?: string;
  loading?: string;
  decoding?: string;
  fetchPriority?: string;
  fetchpriority?: string;
  crossOrigin?: string;
  crossorigin?: string;
  referrerPolicy?: string;
  referrerpolicy?: string;
  poster?: string;
  controls?: boolean;
  autoPlay?: boolean;
  autoplay?: boolean;
  loop?: boolean;
  muted?: boolean;
  playsInline?: boolean;
  playsinline?: boolean;
  preload?: string;
  kind?: string;
  srcLang?: string;
  srclang?: string;
  label?: string;
  default?: boolean;
  list?: string;
  dirname?: string;
  action?: string;
  method?: string;
  encType?: string;
  enctype?: string;
  target?: string;
  noValidate?: boolean;
  formAction?: string;
  formEncType?: string;
  formMethod?: string;
  formTarget?: string;
  formNoValidate?: boolean;
  pattern?: string;
  minLength?: number | string;
  maxLength?: number | string;
  rows?: number | string;
  cols?: number | string;
  size?: number | string;
  colSpan?: number | string;
  colspan?: number | string;
  rowSpan?: number | string;
  rowspan?: number | string;
  headers?: string;
  scope?: string;
  abbr?: string;
  span?: number | string;
  start?: number | string;
  reversed?: boolean;
  orientation?: 'horizontal' | 'vertical';
  min?: number | string;
  max?: number | string;
  step?: number | string;
  minValue?: number;
  maxValue?: number;
  valueNumber?: number;
  stepValue?: number;
  [attribute: `aria-${string}`]: StyleValue;
  [attribute: `data-${string}`]: StyleValue;
  [attribute: string]: unknown;
}

export type ComponentMarker<P extends WebCompatibleProps = WebCompatibleProps> = {
  readonly name: string;
  (props: P): never;
};

export function createComponent<P extends WebCompatibleProps = WebCompatibleProps>(
  name: string,
): ComponentMarker<P>;

export const Button: ComponentMarker;
export const Label: ComponentMarker;
export const Text: ComponentMarker;
export const TextField: ComponentMarker;
export const Input: ComponentMarker;
export const Checkbox: ComponentMarker;
export const Switch: ComponentMarker;
export const RadioGroup: ComponentMarker;
export const Radio: ComponentMarker;
export const Select: ComponentMarker;
export const SelectValue: ComponentMarker;
export const ListBox: ComponentMarker;
export const ListBoxItem: ComponentMarker;
export const Dialog: ComponentMarker;
export const Popover: ComponentMarker;
export const Tabs: ComponentMarker;
export const TabList: ComponentMarker;
export const Tab: ComponentMarker;
export const TabPanel: ComponentMarker;
export const Group: ComponentMarker;
export const Form: ComponentMarker;
export const Menu: ComponentMarker;
export const MenuItem: ComponentMarker;
export const Separator: ComponentMarker;
export const Slider: ComponentMarker;
export const ProgressBar: ComponentMarker;
export const Toolbar: ComponentMarker;
export const Link: ComponentMarker;

export function jsx(
  type: string | ComponentMarker | typeof Fragment,
  props: WebCompatibleProps,
  key?: unknown,
): CompiledJsxNode | CompiledJsxNode[];
export const jsxs: typeof jsx;

export namespace JSX {
  type Element = CompiledJsxNode | CompiledJsxNode[];
  interface ElementChildrenAttribute {
    children: {};
  }
  interface IntrinsicElements {
    [elementName: string]: WebCompatibleProps;
  }
}
