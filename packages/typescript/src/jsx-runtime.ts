import {
  Fragment,
  createA3sElement,
  type A3sElement,
  type A3sElementType,
  type A3sJsxProps,
  type A3sKey,
} from "./element.ts";

export { Fragment };
export type {
  A3sElement,
  A3sElementType,
  A3sFunctionComponent,
  A3sJsxChild,
  A3sJsxProps,
  A3sKey,
  A3sSourceLocation,
} from "./element.ts";

export function jsx(
  type: A3sElementType,
  props: A3sJsxProps | null,
  key?: A3sKey | null,
): A3sElement {
  return createA3sElement(type, props, key, { staticChildren: false });
}

export function jsxs(
  type: A3sElementType,
  props: A3sJsxProps | null,
  key?: A3sKey | null,
): A3sElement {
  return createA3sElement(type, props, key, { staticChildren: true });
}
