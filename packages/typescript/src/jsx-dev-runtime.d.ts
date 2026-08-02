export {
  Fragment,
  jsxDEV,
  type A3sElement,
  type A3sElementType,
  type A3sFunctionComponent,
  type A3sJsxChild,
  type A3sJsxProps,
  type A3sKey,
  type A3sSourceLocation,
} from "./jsx-dev-runtime.ts";

import type {
  A3sElement,
  A3sJsxChild,
  A3sJsxProps,
  A3sKey,
} from "./element.ts";

export namespace JSX {
  type Element = A3sElement;
  type ElementType = string | ((props: any) => A3sJsxChild);

  interface ElementChildrenAttribute {
    children: unknown;
  }

  interface IntrinsicAttributes {
    key?: A3sKey | null;
  }

  interface IntrinsicElements {
    [name: string]: A3sJsxProps;
  }
}
