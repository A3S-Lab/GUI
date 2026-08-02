type NullableStringField =
  | "label"
  | "textValue"
  | "value"
  | "placeholder"
  | "action"
  | "ariaLabel"
  | "name"
  | "form"
  | "inputType"
  | "accept"
  | "capture"
  | "alt"
  | "href"
  | "src"
  | "srcset"
  | "sizes"
  | "media"
  | "resourceType"
  | "loading"
  | "decoding"
  | "fetchPriority"
  | "crossOrigin"
  | "referrerPolicy"
  | "poster"
  | "preload"
  | "trackKind"
  | "srclang"
  | "trackLabel"
  | "list"
  | "dirname"
  | "formAction"
  | "formEnctype"
  | "formMethod"
  | "formTarget"
  | "id"
  | "className";

type RequiredBooleanField =
  | "isDisabled"
  | "isRequired"
  | "isInvalid"
  | "isReadOnly"
  | "isSelected";

type NullableBooleanField =
  | "isChecked"
  | "isExpanded"
  | "controls"
  | "autoplay"
  | "loopPlayback"
  | "muted"
  | "playsInline"
  | "defaultTrack"
  | "formNoValidate";

type NullableNumberField =
  | "minValue"
  | "maxValue"
  | "valueNumber"
  | "stepValue"
  | "intrinsicWidth"
  | "intrinsicHeight";

interface PropTarget<Field extends string> {
  readonly field: Field;
  readonly canonical: string;
  readonly retainAriaAttribute?: boolean;
}

export const STRING_PROPS = new Map<string, PropTarget<NullableStringField>>([
  ["label", { field: "label", canonical: "label" }],
  ["textValue", { field: "textValue", canonical: "textValue" }],
  ["value", { field: "value", canonical: "value" }],
  ["placeholder", { field: "placeholder", canonical: "placeholder" }],
  ["action", { field: "action", canonical: "action" }],
  ["aria-label", { field: "ariaLabel", canonical: "aria-label" }],
  ["ariaLabel", { field: "ariaLabel", canonical: "aria-label" }],
  ["name", { field: "name", canonical: "name" }],
  ["form", { field: "form", canonical: "form" }],
  ["type", { field: "inputType", canonical: "inputType" }],
  ["inputType", { field: "inputType", canonical: "inputType" }],
  ["accept", { field: "accept", canonical: "accept" }],
  ["capture", { field: "capture", canonical: "capture" }],
  ["alt", { field: "alt", canonical: "alt" }],
  ["href", { field: "href", canonical: "href" }],
  ["src", { field: "src", canonical: "src" }],
  ["srcset", { field: "srcset", canonical: "srcset" }],
  ["srcSet", { field: "srcset", canonical: "srcset" }],
  ["sizes", { field: "sizes", canonical: "sizes" }],
  ["media", { field: "media", canonical: "media" }],
  ["resourceType", { field: "resourceType", canonical: "resourceType" }],
  ["loading", { field: "loading", canonical: "loading" }],
  ["decoding", { field: "decoding", canonical: "decoding" }],
  ["fetchPriority", { field: "fetchPriority", canonical: "fetchPriority" }],
  ["crossOrigin", { field: "crossOrigin", canonical: "crossOrigin" }],
  ["referrerPolicy", { field: "referrerPolicy", canonical: "referrerPolicy" }],
  ["poster", { field: "poster", canonical: "poster" }],
  ["preload", { field: "preload", canonical: "preload" }],
  ["trackKind", { field: "trackKind", canonical: "trackKind" }],
  ["srclang", { field: "srclang", canonical: "srclang" }],
  ["srcLang", { field: "srclang", canonical: "srclang" }],
  ["trackLabel", { field: "trackLabel", canonical: "trackLabel" }],
  ["list", { field: "list", canonical: "list" }],
  ["dirname", { field: "dirname", canonical: "dirname" }],
  ["formAction", { field: "formAction", canonical: "formAction" }],
  ["formEnctype", { field: "formEnctype", canonical: "formEnctype" }],
  ["formEncType", { field: "formEnctype", canonical: "formEnctype" }],
  ["formMethod", { field: "formMethod", canonical: "formMethod" }],
  ["formTarget", { field: "formTarget", canonical: "formTarget" }],
  ["id", { field: "id", canonical: "id" }],
  ["class", { field: "className", canonical: "className" }],
  ["className", { field: "className", canonical: "className" }],
]);

export const REQUIRED_BOOLEAN_PROPS = new Map<string, PropTarget<RequiredBooleanField>>([
  ["isDisabled", { field: "isDisabled", canonical: "isDisabled" }],
  ["disabled", { field: "isDisabled", canonical: "isDisabled" }],
  [
    "aria-disabled",
    { field: "isDisabled", canonical: "isDisabled", retainAriaAttribute: true },
  ],
  ["isRequired", { field: "isRequired", canonical: "isRequired" }],
  ["required", { field: "isRequired", canonical: "isRequired" }],
  [
    "aria-required",
    { field: "isRequired", canonical: "isRequired", retainAriaAttribute: true },
  ],
  ["isInvalid", { field: "isInvalid", canonical: "isInvalid" }],
  ["invalid", { field: "isInvalid", canonical: "isInvalid" }],
  [
    "aria-invalid",
    { field: "isInvalid", canonical: "isInvalid", retainAriaAttribute: true },
  ],
  ["isReadOnly", { field: "isReadOnly", canonical: "isReadOnly" }],
  ["readOnly", { field: "isReadOnly", canonical: "isReadOnly" }],
  ["readonly", { field: "isReadOnly", canonical: "isReadOnly" }],
  [
    "aria-readonly",
    { field: "isReadOnly", canonical: "isReadOnly", retainAriaAttribute: true },
  ],
  ["isSelected", { field: "isSelected", canonical: "isSelected" }],
  ["selected", { field: "isSelected", canonical: "isSelected" }],
  [
    "aria-selected",
    { field: "isSelected", canonical: "isSelected", retainAriaAttribute: true },
  ],
]);

export const NULLABLE_BOOLEAN_PROPS = new Map<string, PropTarget<NullableBooleanField>>([
  ["isChecked", { field: "isChecked", canonical: "isChecked" }],
  ["checked", { field: "isChecked", canonical: "isChecked" }],
  [
    "aria-checked",
    { field: "isChecked", canonical: "isChecked", retainAriaAttribute: true },
  ],
  ["isExpanded", { field: "isExpanded", canonical: "isExpanded" }],
  ["expanded", { field: "isExpanded", canonical: "isExpanded" }],
  [
    "aria-expanded",
    { field: "isExpanded", canonical: "isExpanded", retainAriaAttribute: true },
  ],
  ["controls", { field: "controls", canonical: "controls" }],
  ["autoplay", { field: "autoplay", canonical: "autoplay" }],
  ["autoPlay", { field: "autoplay", canonical: "autoplay" }],
  ["loop", { field: "loopPlayback", canonical: "loopPlayback" }],
  ["loopPlayback", { field: "loopPlayback", canonical: "loopPlayback" }],
  ["muted", { field: "muted", canonical: "muted" }],
  ["playsInline", { field: "playsInline", canonical: "playsInline" }],
  ["defaultTrack", { field: "defaultTrack", canonical: "defaultTrack" }],
  ["formNoValidate", { field: "formNoValidate", canonical: "formNoValidate" }],
]);

export const NUMBER_PROPS = new Map<string, PropTarget<NullableNumberField>>([
  ["min", { field: "minValue", canonical: "minValue" }],
  ["minValue", { field: "minValue", canonical: "minValue" }],
  [
    "aria-valuemin",
    { field: "minValue", canonical: "minValue", retainAriaAttribute: true },
  ],
  ["max", { field: "maxValue", canonical: "maxValue" }],
  ["maxValue", { field: "maxValue", canonical: "maxValue" }],
  [
    "aria-valuemax",
    { field: "maxValue", canonical: "maxValue", retainAriaAttribute: true },
  ],
  ["current", { field: "valueNumber", canonical: "valueNumber" }],
  ["valueNumber", { field: "valueNumber", canonical: "valueNumber" }],
  [
    "aria-valuenow",
    { field: "valueNumber", canonical: "valueNumber", retainAriaAttribute: true },
  ],
  ["step", { field: "stepValue", canonical: "stepValue" }],
  ["stepValue", { field: "stepValue", canonical: "stepValue" }],
  ["intrinsicWidth", { field: "intrinsicWidth", canonical: "intrinsicWidth" }],
  ["intrinsicHeight", { field: "intrinsicHeight", canonical: "intrinsicHeight" }],
]);

export const EVENT_ALIASES = new Map<string, string>([
  ["onclick", "onClick"],
  ["onpress", "onPress"],
  ["onpressstart", "onPressStart"],
  ["onpressend", "onPressEnd"],
  ["onpressup", "onPressUp"],
  ["onpresschange", "onPressChange"],
  ["onchange", "onChange"],
  ["oninput", "onInput"],
  ["onselectionchange", "onSelectionChange"],
  ["onfocus", "onFocus"],
  ["onblur", "onBlur"],
  ["onfocuschange", "onFocusChange"],
  ["onfocuswithin", "onFocusWithin"],
  ["onblurwithin", "onBlurWithin"],
  ["onfocuswithinchange", "onFocusWithinChange"],
  ["ontoggle", "onToggle"],
  ["onexpandedchange", "onExpandedChange"],
  ["onhoverstart", "onHoverStart"],
  ["onhoverend", "onHoverEnd"],
  ["onhoverchange", "onHoverChange"],
  ["onkeydown", "onKeyDown"],
  ["onkeyup", "onKeyUp"],
  ["onwheel", "onWheel"],
  ["oncopy", "onCopy"],
  ["oncut", "onCut"],
  ["onpaste", "onPaste"],
]);

export const RESERVED_WIRE_PROPS = new Set([
  "events",
  "actionLabels",
  "explicitProps",
  "importSource",
  "dangerouslySetInnerHTML",
  "innerHTML",
]);
export const UNSAFE_PROPERTY_NAMES = new Set(["__proto__", "constructor", "prototype"]);

export const PORTABLE_ATTRIBUTE_NAME = /^[A-Za-z_][A-Za-z0-9_.:-]*$/u;
export const ARRAY_INDEX_NAME = /^(?:0|[1-9][0-9]*)$/u;
