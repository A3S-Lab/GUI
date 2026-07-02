const COMPONENT_NAME = Symbol.for('a3s.gui.component');

export const Fragment = Symbol.for('a3s.gui.fragment');

export function createComponent(name) {
  function A3sComponent() {
    throw new Error(`${name} is a compile-time a3s-gui component marker`);
  }
  Object.defineProperty(A3sComponent, 'name', {value: name});
  A3sComponent[COMPONENT_NAME] = name;
  return A3sComponent;
}

export const Button = createComponent('Button');
export const Label = createComponent('Label');
export const Text = createComponent('Text');
export const TextField = createComponent('TextField');
export const Input = createComponent('Input');
export const Checkbox = createComponent('Checkbox');
export const Switch = createComponent('Switch');
export const RadioGroup = createComponent('RadioGroup');
export const Radio = createComponent('Radio');
export const Select = createComponent('Select');
export const SelectValue = createComponent('SelectValue');
export const ListBox = createComponent('ListBox');
export const ListBoxItem = createComponent('ListBoxItem');
export const Dialog = createComponent('Dialog');
export const Popover = createComponent('Popover');
export const Tabs = createComponent('Tabs');
export const TabList = createComponent('TabList');
export const Tab = createComponent('Tab');
export const TabPanel = createComponent('TabPanel');
export const Group = createComponent('Group');
export const Form = createComponent('Form');
export const Menu = createComponent('Menu');
export const MenuItem = createComponent('MenuItem');
export const Separator = createComponent('Separator');
export const Slider = createComponent('Slider');
export const ProgressBar = createComponent('ProgressBar');
export const Toolbar = createComponent('Toolbar');
export const Link = createComponent('Link');

export function jsx(type, props, key) {
  return createNode(type, props ?? {}, key);
}

export const jsxs = jsx;

function createNode(type, props, key) {
  if (type === Fragment) {
    return normalizeChildren(props.children);
  }

  const tag = tagName(type);
  const normalizedKey = key ?? props.key ?? stableKey(tag, props);
  const {children, ...rest} = props;
  return {
    kind: 'element',
    key: String(normalizedKey),
    tag,
    props: normalizeProps(rest, tag),
    children: normalizeChildren(children),
  };
}

function tagName(type) {
  if (typeof type === 'string') {
    return type;
  }
  if (typeof type === 'function') {
    return type[COMPONENT_NAME] ?? type.displayName ?? type.name;
  }
  throw new Error(`unsupported JSX type ${String(type)}`);
}

function stableKey(tag, props) {
  return props.id ?? props.name ?? props['data-testid'] ?? tag;
}

function normalizeChildren(children) {
  if (children == null || children === false || children === true) {
    return [];
  }
  const values = Array.isArray(children) ? children.flat(Infinity) : [children];
  return values
    .filter((child) => child != null && child !== false && child !== true)
    .flatMap((child, index) => {
      if (Array.isArray(child)) return normalizeChildren(child);
      if (typeof child === 'string' || typeof child === 'number') {
        return [{kind: 'text', key: `text-${index}`, value: String(child)}];
      }
      return [child];
    });
}

function normalizeProps(props, tag) {
  const out = {};
  const attributes = {};
  const events = {};
  const actionLabels = {};
  const style = {};

  for (const [name, value] of Object.entries(props)) {
    if (value == null || name === 'key') continue;
    if (name === 'className') {
      out.className = String(value);
    } else if (name === 'style') {
      if (typeof value === 'object') {
        for (const [property, styleValue] of Object.entries(value)) {
          if (styleValue != null) style[property] = styleValue;
        }
      } else if (typeof value === 'string') {
        Object.assign(style, parseStyleText(value));
      }
    } else if (name.startsWith('on') && typeof value === 'string') {
      events[name] = value;
    } else if (name.startsWith('on') && typeof value === 'function') {
      const actionId = actionIdForEvent(value);
      events[name] = actionId;
      const actionLabel = actionLabelForEvent(value);
      if (actionLabel != null) actionLabels[actionId] = actionLabel;
    } else if (name.startsWith('aria-') || name.startsWith('data-')) {
      attributes[name] = String(value);
      applySemanticAttribute(out, name, value);
    } else if (name === 'id') {
      out.id = String(value);
    } else if (name === 'disabled' && isAttributeDisabledTag(tag)) {
      attributes[name] = String(value);
    } else if (name === 'disabled') {
      out.isDisabled = Boolean(value);
    } else if (name === 'required') {
      out.isRequired = Boolean(value);
    } else if (name === 'invalid') {
      out.isInvalid = Boolean(value);
    } else if (name === 'selected') {
      out.isSelected = Boolean(value);
    } else if (name === 'checked') {
      out.isChecked = Boolean(value);
    } else if (name === 'defaultChecked') {
      out.isChecked = Boolean(value);
    } else if (name === 'expanded') {
      out.isExpanded = Boolean(value);
    } else if (name === 'isDisabled') {
      out.isDisabled = Boolean(value);
    } else if (name === 'isRequired') {
      out.isRequired = Boolean(value);
    } else if (name === 'isInvalid') {
      out.isInvalid = Boolean(value);
    } else if (name === 'isSelected') {
      out.isSelected = Boolean(value);
    } else if (name === 'isChecked') {
      out.isChecked = Boolean(value);
    } else if (name === 'isExpanded') {
      out.isExpanded = Boolean(value);
    } else if (name === 'textValue') {
      out.textValue = String(value);
    } else if (name === 'value' && isAttributeValueTag(tag)) {
      attributes[name] = String(value);
    } else if (name === 'value' || name === 'defaultValue') {
      const numericValue = isNumericValueTag(tag, props) ? numberValue(value) : undefined;
      if (numericValue !== undefined) {
        out.valueNumber = numericValue;
      } else {
        out.value = String(value);
      }
    } else if (name === 'min') {
      setNumber(out, 'minValue', value);
    } else if (name === 'max') {
      setNumber(out, 'maxValue', value);
    } else if (name === 'step') {
      setNumber(out, 'stepValue', value);
    } else if (name === 'placeholder') {
      out.placeholder = String(value);
    } else if (name === 'orientation') {
      out.orientation = value;
    } else if (name === 'minValue') {
      setNumber(out, 'minValue', value);
    } else if (name === 'maxValue') {
      setNumber(out, 'maxValue', value);
    } else if (name === 'valueNumber') {
      setNumber(out, 'valueNumber', value);
    } else if (name === 'stepValue') {
      setNumber(out, 'stepValue', value);
    } else {
      attributes[name] = String(value);
    }
  }

  if (Object.keys(style).length > 0) out.style = style;
  if (Object.keys(attributes).length > 0) out.attributes = attributes;
  if (Object.keys(events).length > 0) out.events = events;
  if (Object.keys(actionLabels).length > 0) out.actionLabels = actionLabels;
  return out;
}

function parseStyleText(value) {
  const style = {};
  const normal = [];
  const important = [];
  for (const declaration of splitCssDeclarations(value)) {
    const separator = findCssDeclarationSeparator(declaration);
    if (separator <= 0) continue;
    const property = declaration.slice(0, separator).trim();
    const parsed = stripImportantPriority(declaration.slice(separator + 1).trim());
    const styleValue = parsed.value;
    if (property.length > 0 && styleValue.length > 0) {
      (parsed.important ? important : normal).push([property, styleValue]);
    }
  }
  for (const [property, styleValue] of normal) style[property] = styleValue;
  for (const [property, styleValue] of important) style[property] = styleValue;
  return style;
}

function splitCssDeclarations(value) {
  const declarations = [];
  let current = '';
  let quote = null;
  let escaped = false;
  let parenDepth = 0;
  let bracketDepth = 0;

  for (let index = 0; index < value.length; index += 1) {
    const char = value[index];
    const next = value[index + 1];

    if (quote != null) {
      current += char;
      if (escaped) {
        escaped = false;
      } else if (char === '\\') {
        escaped = true;
      } else if (char === quote) {
        quote = null;
      }
      continue;
    }

    if (char === '/' && next === '*') {
      const commentEnd = value.indexOf('*/', index + 2);
      if (commentEnd === -1) break;
      index = commentEnd + 1;
      continue;
    }

    if (char === '"' || char === "'") {
      quote = char;
      current += char;
    } else if (char === '(') {
      parenDepth += 1;
      current += char;
    } else if (char === ')') {
      parenDepth = Math.max(0, parenDepth - 1);
      current += char;
    } else if (char === '[') {
      bracketDepth += 1;
      current += char;
    } else if (char === ']') {
      bracketDepth = Math.max(0, bracketDepth - 1);
      current += char;
    } else if (char === ';' && parenDepth === 0 && bracketDepth === 0) {
      declarations.push(current);
      current = '';
    } else {
      current += char;
    }
  }

  if (current.trim().length > 0) declarations.push(current);
  return declarations;
}

function findCssDeclarationSeparator(declaration) {
  let quote = null;
  let escaped = false;
  let parenDepth = 0;
  let bracketDepth = 0;

  for (let index = 0; index < declaration.length; index += 1) {
    const char = declaration[index];

    if (quote != null) {
      if (escaped) {
        escaped = false;
      } else if (char === '\\') {
        escaped = true;
      } else if (char === quote) {
        quote = null;
      }
      continue;
    }

    if (char === '"' || char === "'") {
      quote = char;
    } else if (char === '(') {
      parenDepth += 1;
    } else if (char === ')') {
      parenDepth = Math.max(0, parenDepth - 1);
    } else if (char === '[') {
      bracketDepth += 1;
    } else if (char === ']') {
      bracketDepth = Math.max(0, bracketDepth - 1);
    } else if (char === ':' && parenDepth === 0 && bracketDepth === 0) {
      return index;
    }
  }

  return -1;
}

function stripImportantPriority(value) {
  let quote = null;
  let escaped = false;
  let parenDepth = 0;
  let bracketDepth = 0;
  let importantStart = -1;

  for (let index = 0; index < value.length; index += 1) {
    const char = value[index];

    if (quote != null) {
      if (escaped) {
        escaped = false;
      } else if (char === '\\') {
        escaped = true;
      } else if (char === quote) {
        quote = null;
      }
      continue;
    }

    if (char === '"' || char === "'") {
      quote = char;
    } else if (char === '(') {
      parenDepth += 1;
    } else if (char === ')') {
      parenDepth = Math.max(0, parenDepth - 1);
    } else if (char === '[') {
      bracketDepth += 1;
    } else if (char === ']') {
      bracketDepth = Math.max(0, bracketDepth - 1);
    } else if (char === '!' && parenDepth === 0 && bracketDepth === 0) {
      importantStart = index;
    }
  }

  if (
    importantStart >= 0 &&
    value.slice(importantStart + 1).trim().toLowerCase() === 'important'
  ) {
    return {value: value.slice(0, importantStart).trimEnd(), important: true};
  }
  return {value, important: false};
}

function applySemanticAttribute(out, name, value) {
  if (name === 'aria-disabled') {
    setBoolean(out, 'isDisabled', value);
  } else if (name === 'aria-required') {
    setBoolean(out, 'isRequired', value);
  } else if (name === 'aria-invalid') {
    setInvalid(out, value);
  } else if (name === 'aria-selected') {
    setBoolean(out, 'isSelected', value);
  } else if (name === 'aria-checked') {
    setBoolean(out, 'isChecked', value);
  } else if (name === 'aria-expanded') {
    setBoolean(out, 'isExpanded', value);
  } else if (name === 'aria-orientation') {
    out.orientation = String(value);
  } else if (name === 'aria-valuemin') {
    setNumber(out, 'minValue', value);
  } else if (name === 'aria-valuemax') {
    setNumber(out, 'maxValue', value);
  } else if (name === 'aria-valuenow') {
    setNumber(out, 'valueNumber', value);
  }
}

function setBoolean(out, field, value) {
  const parsed = booleanValue(value);
  if (parsed !== undefined) {
    out[field] = parsed;
  }
}

function setInvalid(out, value) {
  const parsed = invalidValue(value);
  if (parsed !== undefined) {
    out.isInvalid = parsed;
  }
}

function booleanValue(value) {
  if (typeof value === 'boolean') return value;
  if (typeof value === 'string') {
    const normalized = value.trim().toLowerCase();
    if (normalized === '' || normalized === 'true') return true;
    if (normalized === 'false') return false;
  }
  return undefined;
}

function invalidValue(value) {
  if (typeof value === 'string') {
    const normalized = value.trim().toLowerCase();
    if (normalized === 'grammar' || normalized === 'spelling') return true;
  }
  return booleanValue(value);
}

function setNumber(out, field, value) {
  const parsed = numberValue(value);
  if (parsed !== undefined) {
    out[field] = parsed;
  }
}

function numberValue(value) {
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : undefined;
}

function isNumericValueTag(tag, props = {}) {
  const inputType = String(props.type ?? '').trim().toLowerCase();
  return (
    tag === 'Slider' ||
    tag === 'ProgressBar' ||
    tag === 'progress' ||
    tag === 'meter' ||
    (tag === 'input' && (inputType === 'range' || inputType === 'number'))
  );
}

function isAttributeValueTag(tag) {
  return tag === 'li';
}

function isAttributeDisabledTag(tag) {
  return tag === 'link';
}

function actionIdForEvent(handler) {
  const id = handler.a3sAction ?? handler.name;
  if (typeof id === 'string' && id.length > 0) {
    return id;
  }
  throw new Error(
    'a3s-gui cannot serialize anonymous event handlers without a stable id; use createAction("save") or a named function',
  );
}

function actionLabelForEvent(handler) {
  return handler.a3sLabel == null ? undefined : String(handler.a3sLabel);
}
