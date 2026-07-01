import assert from 'node:assert/strict';
import test from 'node:test';

import {
  Button,
  Input,
  Label,
  Menu,
  MenuItem,
  Popover,
  Radio,
  RadioGroup,
  Slider,
  Tab,
  TabList,
  TabPanel,
  Tabs,
  TextField,
  createAction,
  createHostEvent,
  createUiFrame,
  defineAction,
  jsxs,
} from '../src/index.js';

test('jsx runtime lowers React Aria-like tree to UiFrame protocol', () => {
  const root = jsxs(TextField, {
    name: 'email',
    isRequired: true,
    children: [
      jsxs(Label, {children: 'Email'}, 'label'),
      jsxs(Input, {
        placeholder: 'you@example.com',
        style: {minWidth: 280},
        'data-testid': 'email-input',
        onChange: 'setEmail',
      }, 'input'),
    ],
  }, 'email-field');

  const frame = createUiFrame('profile', root, {
    window: {title: 'Profile', width: 640, height: 480},
    actions: [defineAction('setEmail', 'Set email')],
  });

  assert.equal(frame.frameId, 'profile');
  assert.equal(frame.window.title, 'Profile');
  assert.equal(frame.root.tag, 'TextField');
  assert.equal(frame.root.props.isRequired, true);
  assert.equal(frame.root.children[1].tag, 'Input');
  assert.equal(frame.root.children[1].props.events.onChange, 'setEmail');
  assert.equal(frame.root.children[1].props.style.minWidth, 280);
  assert.equal(frame.root.children[1].props.attributes['data-testid'], 'email-input');
});

test('event helper creates HostEvent protocol shape', () => {
  assert.deepEqual(createHostEvent('profile', 7, 'press'), {
    frameId: 'profile',
    event: {node: 7, kind: 'press'},
  });
});

test('button marker creates stable compiled element', () => {
  const root = jsxs(Button, {
    className: 'primary',
    'aria-label': 'Save profile',
    onPress: 'saveProfile',
    children: 'Save',
  }, 'save');

  assert.equal(root.kind, 'element');
  assert.equal(root.key, 'save');
  assert.equal(root.tag, 'Button');
  assert.equal(root.props.className, 'primary');
  assert.equal(root.props.events.onPress, 'saveProfile');
  assert.equal(root.props.attributes['aria-label'], 'Save profile');
  assert.equal(root.children[0].value, 'Save');
});

test('intrinsic HTML elements preserve CSS text and Tailwind class names', () => {
  const root = jsxs('article', {
    className: 'flex flex-col gap-4 p-2 bg-[#663399]',
    style: 'min-width: 280px; color: white;',
    'data-testid': 'article',
    children: jsxs('button', {
      type: 'submit',
      onClick: 'saveArticle',
      children: 'Save',
    }, 'save'),
  }, 'article');

  assert.equal(root.tag, 'article');
  assert.equal(root.props.className, 'flex flex-col gap-4 p-2 bg-[#663399]');
  assert.equal(root.props.style['min-width'], '280px');
  assert.equal(root.props.style.color, 'white');
  assert.equal(root.props.attributes['data-testid'], 'article');
  assert.equal(root.children[0].tag, 'button');
  assert.equal(root.children[0].props.attributes.type, 'submit');
  assert.equal(root.children[0].props.events.onClick, 'saveArticle');
});

test('CSS text parser preserves delimiters inside functions and strings', () => {
  const root = jsxs('div', {
    style: `
      color: rgb(10 20 30 / 50%);
      background-image: url("https://example.com/a:b;c.svg");
      content: "label: value; still text";
      --accent: color-mix(in srgb, rebeccapurple 40%, white);
      /* ignored comment: with delimiter; */
      padding-inline: 1rem 2rem;
    `,
    children: 'Styled',
  }, 'styled');

  assert.equal(root.props.style.color, 'rgb(10 20 30 / 50%)');
  assert.equal(root.props.style['background-image'], 'url("https://example.com/a:b;c.svg")');
  assert.equal(root.props.style.content, '"label: value; still text"');
  assert.equal(root.props.style['--accent'], 'color-mix(in srgb, rebeccapurple 40%, white)');
  assert.equal(root.props.style['padding-inline'], '1rem 2rem');
  assert.equal(root.props.style['ignored comment'], undefined);
});

test('function event props compile to stable action ids', () => {
  const saveProfile = createAction('saveProfile', 'Save profile');
  const root = jsxs(Button, {
    className: 'primary',
    onPress: saveProfile,
    children: 'Save',
  }, 'save');
  const frame = createUiFrame('profile', root);

  assert.equal(root.props.events.onPress, 'saveProfile');
  assert.deepEqual(frame.actions, [{id: 'saveProfile'}]);
  assert.deepEqual(defineAction(saveProfile), {
    id: 'saveProfile',
    label: 'Save profile',
  });
});

test('web and aria state attributes normalize to native control props', () => {
  const root = jsxs(Slider, {
    disabled: true,
    required: true,
    selected: true,
    value: 50,
    min: 0,
    max: 100,
    orientation: 'horizontal',
    'aria-label': 'Volume',
    'aria-invalid': 'grammar',
    'aria-expanded': true,
    onChange: 'setVolume',
  }, 'volume');

  assert.equal(root.props.isDisabled, true);
  assert.equal(root.props.isRequired, true);
  assert.equal(root.props.isSelected, true);
  assert.equal(root.props.isInvalid, true);
  assert.equal(root.props.isExpanded, true);
  assert.equal(root.props.valueNumber, 50);
  assert.equal(root.props.minValue, 0);
  assert.equal(root.props.maxValue, 100);
  assert.equal(root.props.orientation, 'horizontal');
  assert.equal(root.props.attributes['aria-label'], 'Volume');
  assert.equal(root.props.events.onChange, 'setVolume');
});

test('radio group markers lower to Web-compatible compiled nodes', () => {
  const setTheme = createAction('setTheme', 'Set theme');
  const root = jsxs(RadioGroup, {
    'aria-label': 'Theme',
    value: 'dark',
    onChange: setTheme,
    children: [
      jsxs(Radio, {value: 'light', children: 'Light'}, 'light'),
      jsxs(Radio, {value: 'dark', isSelected: true, children: 'Dark'}, 'dark'),
    ],
  }, 'theme');
  const frame = createUiFrame('settings', root);

  assert.equal(root.tag, 'RadioGroup');
  assert.equal(root.props.value, 'dark');
  assert.equal(root.props.events.onChange, 'setTheme');
  assert.equal(root.props.attributes['aria-label'], 'Theme');
  assert.equal(root.children[0].tag, 'Radio');
  assert.equal(root.children[1].props.isSelected, true);
  assert.deepEqual(frame.actions, [{id: 'setTheme'}]);
});

test('tabs markers preserve React Aria tablist and panel structure', () => {
  const setTab = createAction('setTab', 'Set tab');
  const root = jsxs(Tabs, {
    onSelectionChange: setTab,
    children: [
      jsxs(TabList, {
        children: [
          jsxs(Tab, {id: 'profile', isSelected: true, children: 'Profile'}, 'profile-tab'),
          jsxs(Tab, {id: 'billing', children: 'Billing'}, 'billing-tab'),
        ],
      }, 'settings-tabs'),
      jsxs(TabPanel, {id: 'profile', children: 'Profile settings'}, 'profile-panel'),
      jsxs(TabPanel, {id: 'billing', children: 'Billing settings'}, 'billing-panel'),
    ],
  }, 'settings');
  const frame = createUiFrame('settings', root);

  assert.equal(root.tag, 'Tabs');
  assert.equal(root.props.events.onSelectionChange, 'setTab');
  assert.equal(root.children[0].tag, 'TabList');
  assert.equal(root.children[0].children[0].tag, 'Tab');
  assert.equal(root.children[0].children[0].props.isSelected, true);
  assert.equal(root.children[1].tag, 'TabPanel');
  assert.deepEqual(frame.actions, [{id: 'setTab'}]);
});

test('popover marker preserves Web-compatible overlay structure', () => {
  const archiveItem = createAction('archiveItem', 'Archive item');
  const root = jsxs(Popover, {
    'aria-label': 'Actions',
    children: [
      jsxs(Button, {onPress: archiveItem, children: 'Archive'}, 'archive'),
    ],
  }, 'actions-popover');
  const frame = createUiFrame('item-actions', root);

  assert.equal(root.tag, 'Popover');
  assert.equal(root.props.attributes['aria-label'], 'Actions');
  assert.equal(root.children[0].tag, 'Button');
  assert.equal(root.children[0].props.events.onPress, 'archiveItem');
  assert.deepEqual(frame.actions, [{id: 'archiveItem'}]);
});

test('menu markers preserve Web-compatible native menu structure', () => {
  const openFile = createAction('openFile', 'Open file');
  const root = jsxs(Menu, {
    'aria-label': 'File',
    children: [
      jsxs(MenuItem, {value: 'open', onPress: openFile, children: 'Open'}, 'open'),
    ],
  }, 'file-menu');
  const frame = createUiFrame('document', root);

  assert.equal(root.tag, 'Menu');
  assert.equal(root.props.attributes['aria-label'], 'File');
  assert.equal(root.children[0].tag, 'MenuItem');
  assert.equal(root.children[0].props.value, 'open');
  assert.equal(root.children[0].props.events.onPress, 'openFile');
  assert.deepEqual(frame.actions, [{id: 'openFile'}]);
});

test('anonymous event functions fail with a protocol hint', () => {
  const handler = () => {};
  Object.defineProperty(handler, 'name', {value: ''});

  assert.throws(
    () => jsxs(Button, {onPress: handler, children: 'Save'}, 'save'),
    /stable id/,
  );
});

test('frame roots must be a single element', () => {
  assert.throws(
    () => createUiFrame('profile', [
      jsxs(Button, {children: 'One'}, 'one'),
      jsxs(Button, {children: 'Two'}, 'two'),
    ]),
    /one root element/,
  );
});
