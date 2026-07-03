import assert from 'node:assert/strict';
import test from 'node:test';

import {
  Button,
  Group,
  Input,
  Label,
  TextField,
  createHandledNativeEvent,
  createHostEvent,
  createHostEventResponse,
  createNativeHostEventResponse,
  createNativeRenderResponse,
  createAction,
  createUiFrame,
  defineAction,
  jsx,
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
    window: {
      title: 'Profile',
      width: 640,
      height: 480,
      minWidth: 480,
      minHeight: 320,
      maxWidth: 1280,
      maxHeight: 960,
      resizable: false,
    },
    actions: [defineAction('setEmail', 'Set email')],
  });

  assert.equal(frame.frameId, 'profile');
  assert.equal(frame.window.title, 'Profile');
  assert.equal(frame.window.minWidth, 480);
  assert.equal(frame.window.maxHeight, 960);
  assert.equal(frame.window.resizable, false);
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
  assert.deepEqual(createHostEvent('profile', 7, 'keyDown', 'Enter'), {
    frameId: 'profile',
    event: {node: 7, kind: 'keyDown', value: 'Enter'},
  });
  assert.deepEqual(createHostEvent('profile', 7, 'change', null), {
    frameId: 'profile',
    event: {node: 7, kind: 'change'},
  });
  assert.deepEqual(createHostEvent('profile', 7, 'close'), {
    frameId: 'profile',
    event: {node: 7, kind: 'close'},
  });
});

test('event helper rejects invalid HostEvent contracts', () => {
  assert.throws(
    () => createHostEvent('', 7, 'press'),
    /non-empty frame id/,
  );
  assert.throws(
    () => createHostEvent('profile', 0, 'press'),
    /positive integer node id/,
  );
  assert.throws(
    () => createHostEvent('profile', 1.5, 'press'),
    /positive integer node id/,
  );
  assert.throws(
    () => createHostEvent('profile', 7, 'submit'),
    /supported native event kind/,
  );
  assert.throws(
    () => createHostEvent('profile', 7, 'change', 42),
    /values need to be strings/,
  );
});

test('handled native event helper mirrors Rust protocol shape', () => {
  const event = createHandledNativeEvent({node: 7, kind: 'focus'});

  assert.deepEqual(event, {
    event: {node: 7, kind: 'focus'},
    invocation: null,
    interactionChanges: [],
  });

  const changed = createHandledNativeEvent(
    {node: 7, kind: 'change', value: 'saved'},
    {
      invocation: {
        node: 7,
        action: 'saveProfile',
        event: 'change',
        value: 'saved',
      },
      interactionChanges: [
        {
          node: 7,
          before: {
            focused: false,
            selected: false,
            checked: null,
            expanded: null,
          },
          after: {
            focused: false,
            value: 'saved',
            selected: false,
            checked: null,
            expanded: null,
          },
        },
      ],
    },
  );

  assert.equal(changed.invocation.action, 'saveProfile');
  assert.equal(changed.interactionChanges[0].after.value, 'saved');
});

test('handled native event helper rejects invalid protocol shapes', () => {
  assert.throws(
    () => createHandledNativeEvent(null),
    /event object/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 0, kind: 'focus'}),
    /positive integer node id/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'submit'}),
    /supported native event kind/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'change', value: 42}),
    /native event values need strings/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'focus'}, null),
    /options need an object/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'focus'}, {invocation: {}}),
    /invocations need positive integer node ids/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'focus'}, {
      invocation: {
        node: 7,
        action: '',
        event: 'focus',
      },
    }),
    /invocations need non-empty string action ids/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'focus'}, {
      invocation: {
        node: 7,
        action: 'saveProfile',
        event: 'submit',
      },
    }),
    /invocations need supported native event kinds/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'focus'}, {interactionChanges: {}}),
    /interaction changes need an array/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'focus'}, {
      interactionChanges: [{node: 7, before: {}, after: {}}],
    }),
    /interaction state\.focused values need booleans/,
  );
  assert.throws(
    () => createHandledNativeEvent({node: 7, kind: 'focus'}, {
      interactionChanges: [
        {
          node: 7,
          before: {focused: false, selected: false},
          after: {focused: true, selected: false, checked: 'true'},
        },
      ],
    }),
    /interaction state\.checked values need booleans or null/,
  );
});

test('native response helpers mirror Rust protocol envelopes', () => {
  const accessibilityTree = {
    node: 1,
    role: 'Button',
    label: 'Save',
    relationships: {},
    description: {},
    structure: {},
    state: {},
    disabled: false,
    required: false,
    invalid: false,
    readOnly: false,
    multiple: false,
    focused: false,
    selected: false,
    checked: null,
    expanded: null,
    children: [],
  };
  const invocation = {
    node: 1,
    action: 'saveProfile',
    event: 'press',
    value: 'Save',
  };
  const blueprint = {
    backend: 'gtk4',
    widgetClass: 'gtk::Button',
    role: 'Button',
    accessibilityRole: 'Button',
    controlState: {},
    style: {},
    portableStyle: {},
    events: {},
    metadata: {},
  };
  const commands = [
    {type: 'create', id: 1, blueprint},
    {type: 'update', id: 1, blueprint},
    {type: 'insertChild', parent: 1, child: 2, index: 0},
    {type: 'remove', id: 2},
    {type: 'setRoot', id: 1},
  ];
  const interactionChanges = [{
    node: 1,
    before: {
      focused: false,
      selected: false,
      checked: null,
      expanded: null,
    },
    after: {
      focused: true,
      selected: false,
      checked: null,
      expanded: null,
    },
  }];

  assert.deepEqual(
    createNativeRenderResponse(
      'profile',
      1,
      commands,
      {accessibilityTree},
    ),
    {
      frameId: 'profile',
      root: 1,
      commands,
      accessibilityTree,
    },
  );
  assert.deepEqual(createHostEventResponse('profile', invocation), {
    frameId: 'profile',
    invocation,
  });
  assert.deepEqual(createHostEventResponse('profile', invocation, {interactionChanges}), {
    frameId: 'profile',
    invocation,
    interactionChanges,
  });
  assert.deepEqual(
    createNativeHostEventResponse('profile', {
      invocation: null,
      accessibilityTree: null,
    }),
    {
      frameId: 'profile',
      invocation: null,
      accessibilityTree: null,
    },
  );
  assert.deepEqual(
    createNativeHostEventResponse('profile', {
      invocation,
      accessibilityTree,
      interactionChanges,
    }),
    {
      frameId: 'profile',
      invocation,
      accessibilityTree,
      interactionChanges,
    },
  );
});

test('native response helpers reject invalid protocol envelopes', () => {
  const invocation = {node: 1, action: 'saveProfile', event: 'press'};
  const accessibilityTree = {
    node: 1,
    role: 'Button',
    relationships: {},
    description: {},
    structure: {},
    state: {},
    disabled: false,
    required: false,
    invalid: false,
    readOnly: false,
    multiple: false,
    focused: false,
    selected: false,
    children: [],
  };

  assert.throws(
    () => createNativeRenderResponse('', 1, []),
    /non-empty frame id/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 0, []),
    /positive integer root node id/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, {}),
    /commands need an array/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [{id: 1}]),
    /commands need object commands with non-empty string types/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [{type: 'openWindow', id: 1}]),
    /commands need supported native command types/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [{type: 'setRoot'}]),
    /commands\.setRoot\.id need positive integer node ids/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [{
      type: 'insertChild',
      parent: 1,
      child: 2,
      index: -1,
    }]),
    /commands\.insertChild\.index values need non-negative integer numbers/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [{
      type: 'create',
      id: 1,
      blueprint: {backend: 'gtk4', role: 'Button'},
    }]),
    /commands\.create\.blueprint\.widgetClass values need non-empty strings/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], null),
    /options need an object/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], {
      accessibilityTree: {role: 'Button'},
    }),
    /accessibilityTree children need an array/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], {
      accessibilityTree: {node: 0, role: 'Button', children: []},
    }),
    /accessibilityTree node ids need positive integers/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], {
      accessibilityTree: {...accessibilityTree, label: 42},
    }),
    /accessibilityTree label values need strings or null/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], {
      accessibilityTree: {...accessibilityTree, relationships: null},
    }),
    /accessibilityTree relationships need an object/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], {
      accessibilityTree: {
        ...accessibilityTree,
        relationships: {labelledBy: 42},
      },
    }),
    /accessibilityTree relationships\.labelledBy values need strings or null/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], {
      accessibilityTree: {
        ...accessibilityTree,
        structure: {level: -1},
      },
    }),
    /accessibilityTree structure\.level values need unsigned integer numbers or null/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], {
      accessibilityTree: {
        ...accessibilityTree,
        state: {hidden: 'true'},
      },
    }),
    /accessibilityTree state\.hidden values need booleans or null/,
  );
  assert.throws(
    () => createNativeRenderResponse('profile', 1, [], {
      accessibilityTree: {...accessibilityTree, focused: 'false'},
    }),
    /accessibilityTree focused values need booleans/,
  );
  assert.throws(
    () => createHostEventResponse('profile', {node: 1, action: 'saveProfile', event: 'submit'}),
    /supported native event kinds/,
  );
  assert.throws(
    () => createHostEventResponse('profile', invocation, {interactionChanges: {}}),
    /interaction changes need an array/,
  );
  assert.throws(
    () => createNativeHostEventResponse('profile', null),
    /options need an object/,
  );
  assert.throws(
    () => createNativeHostEventResponse('profile', {invocation: {}}),
    /invocations need positive integer node ids/,
  );
});

test('frame ids must be stable non-empty strings', () => {
  const root = jsx(Group, {children: 'Profile'}, 'profile');

  assert.throws(
    () => createUiFrame('', root),
    /non-empty string frame id/,
  );
  assert.throws(
    () => createUiFrame(null, root),
    /non-empty string frame id/,
  );
});

test('frame roots must be a single compiled element', () => {
  assert.throws(
    () => createUiFrame('profile', [
      jsxs(Button, {children: 'One'}, 'one'),
      jsxs(Button, {children: 'Two'}, 'two'),
    ]),
    /one root element/,
  );
  assert.throws(
    () => createUiFrame('profile', null),
    /one root element/,
  );
  assert.throws(
    () => createUiFrame('profile', 'Profile'),
    /one root element/,
  );
  assert.throws(
    () => createUiFrame('profile', {kind: 'text', key: 'text-0', value: 'Profile'}),
    /one root element/,
  );
  assert.throws(
    () => createUiFrame('profile', {kind: 'element', key: 'profile'}),
    /one root element/,
  );
});

test('compiled frame nodes must have stable identities', () => {
  assert.throws(
    () => createUiFrame('profile', {kind: 'element', key: '', tag: 'Group'}),
    /elements need non-empty string keys/,
  );
  assert.throws(
    () => createUiFrame('profile', {kind: 'element', key: 'profile', tag: ''}),
    /elements need non-empty string tags/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      children: [{kind: 'text', key: '', value: 'Profile'}],
    }),
    /text nodes need non-empty string keys/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      children: [{kind: 'text', key: 'label', value: 42}],
    }),
    /text nodes need string values/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      children: {kind: 'text', key: 'label', value: 'Profile'},
    }),
    /array children/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      children: null,
    }),
    /array children/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      props: null,
    }),
    /props need an object/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      props: {events: {onPress: 42}},
    }),
    /props\.events values need strings/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      props: {style: {color: {token: 'red'}}},
    }),
    /props\.style values need strings, numbers, or booleans/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      props: {isDisabled: 'true'},
    }),
    /props\.isDisabled values need booleans/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      props: {valueNumber: '42'},
    }),
    /props\.valueNumber values need finite numbers/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      props: {orientation: 'diagonal'},
    }),
    /props\.orientation values need horizontal or vertical/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      props: {intrinsicWidth: -1},
    }),
    /props\.intrinsicWidth values need unsigned integer numbers/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      props: {style: {opacity: NaN}},
    }),
    /props\.style values need finite numbers/,
  );
  assert.throws(
    () => createUiFrame('profile', {
      kind: 'element',
      key: 'profile',
      tag: 'Group',
      children: [
        {kind: 'element', key: 'save', tag: 'Button'},
        {kind: 'text', key: 'save', value: 'Save'},
      ],
    }),
    /sibling nodes need unique keys/,
  );
});

test('frame options must be explicit protocol objects', () => {
  const root = jsx(Group, {children: 'Profile'}, 'profile');

  assert.throws(
    () => createUiFrame('profile', root, null),
    /frame options need an object/,
  );
  assert.throws(
    () => createUiFrame('profile', root, 'saveProfile'),
    /frame options need an object/,
  );
  assert.throws(
    () => createUiFrame('profile', root, []),
    /frame options need an object/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {actions: null}),
    /actions need an array/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {window: null}),
    /window options need an object/,
  );
});

test('frame actions must have stable ids', () => {
  const root = jsx(Group, {children: 'Profile'}, 'profile');

  assert.deepEqual(
    createUiFrame('profile', root, {actions: [{id: 'saveProfile', label: 42}]}).actions,
    [{id: 'saveProfile', label: '42'}],
  );
  assert.throws(
    () => createUiFrame('profile', root, {actions: [{id: ''}]}),
    /non-empty string ids/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {
      actions: [{id: 'saveProfile'}, {id: 'saveProfile', label: 'Save'}],
    }),
    /unique ids/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {actions: 'saveProfile'}),
    /actions need an array/,
  );
});

test('window options must be valid native dimensions', () => {
  const root = jsx(Group, {children: 'Profile'}, 'profile');
  const closeProfile = createAction('closeProfile', 'Close profile');
  const frame = createUiFrame('profile', root, {
    window: {
      title: '',
      onClose: closeProfile,
      width: 640,
      height: 480,
      minWidth: 320,
      maxWidth: 1280,
      resizable: false,
    },
  });

  assert.deepEqual(frame.window, {
    title: '',
    onClose: 'closeProfile',
    width: 640,
    height: 480,
    minWidth: 320,
    maxWidth: 1280,
    resizable: false,
  });
  assert.deepEqual(frame.actions, [{id: 'closeProfile', label: 'Close profile'}]);
  assert.throws(
    () => createUiFrame('profile', root, {window: {width: 640}}),
    /string title/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {window: {title: 'Profile', width: 0}}),
    /positive finite number/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {
      window: {title: 'Profile', minWidth: 800, maxWidth: 640},
    }),
    /minWidth.*greater than.*maxWidth/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {
      window: {title: 'Profile', width: 320, minWidth: 640},
    }),
    /width.*smaller than.*minWidth/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {
      window: {title: 'Profile', onClose: ''},
    }),
    /stable id/,
  );
  assert.throws(
    () => createUiFrame('profile', root, {
      window: {title: 'Profile', resizable: 'false'},
    }),
    /resizable.*boolean/,
  );
});
