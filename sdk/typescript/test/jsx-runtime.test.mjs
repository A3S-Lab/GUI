import assert from 'node:assert/strict';
import test from 'node:test';

import {
  Button,
  Dialog,
  Fragment,
  Group,
  Input,
  Menu,
  MenuItem,
  Popover,
  Radio,
  RadioGroup,
  Tab,
  TabList,
  TabPanel,
  Tabs,
  createAction,
  createUiFrame,
  defineAction,
  jsx,
  jsxs,
} from '../src/index.js';

test('jsx runtime assigns unique fallback keys to unkeyed siblings', () => {
  const root = jsxs(Group, {
    children: [
      jsx(Button, {children: 'Save'}),
      jsx(Button, {children: 'Cancel'}),
      jsx(Fragment, {
        children: [
          jsx('span', {children: 'Status'}),
          jsx('span', {children: 'Ready'}),
        ],
      }),
    ],
  }, 'toolbar');

  assert.deepEqual(
    root.children.map((child) => child.key),
    ['Button', 'Button-1', 'span', 'span-1'],
  );
  assert.doesNotThrow(() => createUiFrame('toolbar', root));
});

test('jsx runtime preserves explicit duplicate keys for frame validation', () => {
  const root = jsxs(Group, {
    children: [
      jsx(Button, {children: 'Save'}, 'action'),
      jsx(Button, {children: 'Cancel'}, 'action'),
    ],
  }, 'toolbar');

  assert.deepEqual(root.children.map((child) => child.key), ['action', 'action']);
  assert.throws(
    () => createUiFrame('toolbar', root),
    /sibling nodes need unique keys/,
  );
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
  assert.equal(root.props.actionLabels.saveProfile, 'Save profile');
  assert.deepEqual(frame.actions, [{id: 'saveProfile', label: 'Save profile'}]);
  assert.deepEqual(defineAction(saveProfile), {
    id: 'saveProfile',
    label: 'Save profile',
  });
});

test('inferred frame actions merge labels by stable action id', () => {
  const saveProfile = createAction('saveProfile', 'Save profile');
  const root = jsxs(Group, {
    children: [
      jsxs(Button, {onPress: 'saveProfile', children: 'Save'}, 'save-text'),
      jsxs(Button, {onPress: saveProfile, children: 'Save labeled'}, 'save-labeled'),
    ],
  }, 'root');
  const frame = createUiFrame('profile', root);

  assert.deepEqual(frame.actions, [{id: 'saveProfile', label: 'Save profile'}]);
});

test('inferred frame actions ignore empty event action ids', () => {
  const root = jsxs(Button, {
    onPress: '',
    onClick: 'saveProfile',
    children: 'Save',
  }, 'save');
  const frame = createUiFrame('profile', root);

  assert.equal(root.props.events.onPress, '');
  assert.deepEqual(frame.actions, [{id: 'saveProfile'}]);
});

test('focus and expanded event aliases compile to stable action ids', () => {
  const setFocus = createAction('setFocus');
  const setOpen = createAction('setOpen');
  const root = jsxs(Dialog, {
    onFocusChange: setFocus,
    onExpandedChange: setOpen,
    children: 'Details',
  }, 'details');
  const frame = createUiFrame('details', root);

  assert.equal(root.props.events.onFocusChange, 'setFocus');
  assert.equal(root.props.events.onExpandedChange, 'setOpen');
  assert.deepEqual(frame.actions, [{id: 'setFocus'}, {id: 'setOpen'}]);
});

test('keyboard event props compile to stable action ids', () => {
  const handleKeyDown = createAction('handleKeyDown', 'Handle key down');
  const handleKeyUp = createAction('handleKeyUp');
  const root = jsxs(Input, {
    onKeyDown: handleKeyDown,
    onKeyUp: handleKeyUp,
  }, 'query');
  const frame = createUiFrame('search', root);

  assert.equal(root.props.events.onKeyDown, 'handleKeyDown');
  assert.equal(root.props.events.onKeyUp, 'handleKeyUp');
  assert.equal(root.props.actionLabels.handleKeyDown, 'Handle key down');
  assert.deepEqual(frame.actions, [
    {id: 'handleKeyDown', label: 'Handle key down'},
    {id: 'handleKeyUp'},
  ]);
});

test('radio group markers lower to structured compiled nodes', () => {
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
  assert.deepEqual(frame.actions, [{id: 'setTheme', label: 'Set theme'}]);
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
  assert.deepEqual(frame.actions, [{id: 'setTab', label: 'Set tab'}]);
});

test('popover marker preserves structured overlay nodes', () => {
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
  assert.deepEqual(frame.actions, [{id: 'archiveItem', label: 'Archive item'}]);
});

test('menu markers preserve structured native menu nodes', () => {
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
  assert.deepEqual(frame.actions, [{id: 'openFile', label: 'Open file'}]);
});

test('anonymous event functions fail with a protocol hint', () => {
  const handler = () => {};
  Object.defineProperty(handler, 'name', {value: ''});

  assert.throws(
    () => jsxs(Button, {onPress: handler, children: 'Save'}, 'save'),
    /stable id/,
  );
});
