import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';

import * as Gui from '../src/index.js';
import {
  Button,
  Dialog,
  Group,
  Input,
  Label,
  Link,
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
  createHandledNativeEvent,
  createHostEvent,
  createUiFrame,
  defineAction,
  jsx,
  jsxs,
} from '../src/index.js';

const SUPPORTED_COMPONENT_MARKERS = [
  ['Button', 'Button'],
  ['Label', 'Label'],
  ['Document', 'Document'],
  ['DocumentHead', 'DocumentHead'],
  ['DocumentBody', 'DocumentBody'],
  ['DocumentTitle', 'DocumentTitle'],
  ['Metadata', 'Metadata'],
  ['ResourceLink', 'ResourceLink'],
  ['StyleSheet', 'StyleSheet'],
  ['Script', 'Script'],
  ['Template', 'Template'],
  ['Slot', 'Slot'],
  ['Text', 'Text'],
  ['Abbreviation', 'Abbreviation'],
  ['Citation', 'Citation'],
  ['Definition', 'Definition'],
  ['DataValue', 'DataValue'],
  ['InsertedText', 'InsertedText'],
  ['DeletedText', 'DeletedText'],
  ['MarkedText', 'MarkedText'],
  ['Time', 'Time'],
  ['Emphasis', 'Emphasis'],
  ['StrongText', 'StrongText'],
  ['Code', 'Code'],
  ['KeyboardInput', 'KeyboardInput'],
  ['SampleOutput', 'SampleOutput'],
  ['Variable', 'Variable'],
  ['InlineQuote', 'InlineQuote'],
  ['Subscript', 'Subscript'],
  ['Superscript', 'Superscript'],
  ['SmallText', 'SmallText'],
  ['BoldText', 'BoldText'],
  ['ItalicText', 'ItalicText'],
  ['StruckText', 'StruckText'],
  ['UnderlinedText', 'UnderlinedText'],
  ['BidirectionalIsolate', 'BidirectionalIsolate'],
  ['BidirectionalOverride', 'BidirectionalOverride'],
  ['Paragraph', 'Paragraph'],
  ['PreformattedText', 'PreformattedText'],
  ['BlockQuote', 'BlockQuote'],
  ['ContactAddress', 'ContactAddress'],
  ['LineBreak', 'LineBreak'],
  ['WordBreakOpportunity', 'WordBreakOpportunity'],
  ['NoBreakText', 'NoBreakText'],
  ['CenteredText', 'CenteredText'],
  ['FontText', 'FontText'],
  ['BigText', 'BigText'],
  ['TeletypeText', 'TeletypeText'],
  ['Applet', 'Applet'],
  ['BackgroundSound', 'BackgroundSound'],
  ['Frame', 'Frame'],
  ['FrameSet', 'FrameSet'],
  ['NoEmbedFallback', 'NoEmbedFallback'],
  ['NoFramesFallback', 'NoFramesFallback'],
  ['Marquee', 'Marquee'],
  ['Math', 'Math'],
  ['NextId', 'NextId'],
  ['SelectedContent', 'SelectedContent'],
  ['Heading', 'Heading'],
  ['HeadingGroup', 'HeadingGroup'],
  ['Ruby', 'Ruby'],
  ['RubyBase', 'RubyBase'],
  ['RubyText', 'RubyText'],
  ['RubyParenthesis', 'RubyParenthesis'],
  ['RubyTextContainer', 'RubyTextContainer'],
  ['Main', 'Main'],
  ['Navigation', 'Navigation'],
  ['Header', 'Header'],
  ['Footer', 'Footer'],
  ['Article', 'Article'],
  ['Section', 'Section'],
  ['Aside', 'Aside'],
  ['Search', 'Search'],
  ['Disclosure', 'Disclosure'],
  ['DisclosureSummary', 'DisclosureSummary'],
  ['Figure', 'Figure'],
  ['FigureCaption', 'FigureCaption'],
  ['DescriptionList', 'DescriptionList'],
  ['DescriptionTerm', 'DescriptionTerm'],
  ['DescriptionDetails', 'DescriptionDetails'],
  ['Image', 'Image'],
  ['Media', 'Media'],
  ['Canvas', 'Canvas'],
  ['EmbeddedContent', 'EmbeddedContent'],
  ['Link', 'Link'],
  ['ImageMap', 'ImageMap'],
  ['ImageMapArea', 'ImageMapArea'],
  ['TextField', 'TextField'],
  ['Input', 'Input'],
  ['Checkbox', 'Checkbox'],
  ['Switch', 'Switch'],
  ['RadioGroup', 'RadioGroup'],
  ['Radio', 'Radio'],
  ['FieldSet', 'FieldSet'],
  ['Legend', 'Legend'],
  ['OptionGroup', 'OptionGroup'],
  ['Output', 'Output'],
  ['Meter', 'Meter'],
  ['Select', 'Select'],
  ['SelectValue', 'SelectValue'],
  ['ListBox', 'ListBox'],
  ['ListBoxItem', 'ListBoxItem'],
  ['Dialog', 'Dialog'],
  ['Popover', 'Popover'],
  ['Tabs', 'Tabs'],
  ['TabList', 'TabList'],
  ['Tab', 'Tab'],
  ['TabPanel', 'TabPanel'],
  ['Group', 'Group'],
  ['Form', 'Form'],
  ['Menu', 'Menu'],
  ['MenuItem', 'MenuItem'],
  ['Separator', 'Separator'],
  ['Slider', 'Slider'],
  ['ProgressBar', 'ProgressBar'],
  ['Toolbar', 'Toolbar'],
  ['Table', 'Table'],
  ['TableSection', 'TableSection'],
  ['TableRow', 'TableRow'],
  ['TableCell', 'TableCell'],
  ['TableColumn', 'TableColumn'],
  ['TableCaption', 'TableCaption'],
];
const JSX_RUNTIME_TYPES = readFileSync(
  new URL('../src/jsx-runtime.d.ts', import.meta.url),
  'utf8',
);

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

test('link marker creates stable compiled element', () => {
  const root = jsxs(Link, {
    href: '/docs',
    children: 'Docs',
  }, 'docs');

  assert.equal(root.kind, 'element');
  assert.equal(root.key, 'docs');
  assert.equal(root.tag, 'Link');
  assert.equal(root.props.attributes.href, '/docs');
  assert.equal(root.children[0].value, 'Docs');
});

test('SDK exports markers for the Rust-supported component surface', () => {
  for (const [exportName, tagName] of SUPPORTED_COMPONENT_MARKERS) {
    const marker = Gui[exportName];
    assert.equal(typeof marker, 'function', `${exportName} should be exported`);
    assert.equal(marker.name, tagName);

    const root = jsx(marker, {'data-testid': exportName}, exportName);
    assert.equal(root.kind, 'element');
    assert.equal(root.key, exportName);
    assert.equal(root.tag, tagName);
    assert.equal(root.props.attributes['data-testid'], exportName);
    assert.match(JSX_RUNTIME_TYPES, new RegExp(`export const ${exportName}: ComponentMarker;`));
  }
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

test('intrinsic SVG elements preserve presentation props and Tailwind class names', () => {
  const root = jsxs('svg', {
    className: 'size-4 fill-none stroke-current stroke-2',
    viewBox: '0 0 24 24',
    'aria-hidden': true,
    children: jsxs('path', {
      d: 'M4 12h16',
      fill: 'none',
      strokeLinecap: 'round',
      strokeLinejoin: 'round',
    }, 'line'),
  }, 'icon');

  assert.equal(root.tag, 'svg');
  assert.equal(root.props.className, 'size-4 fill-none stroke-current stroke-2');
  assert.equal(root.props.attributes.viewBox, '0 0 24 24');
  assert.equal(root.props.attributes['aria-hidden'], 'true');
  assert.equal(root.children[0].tag, 'path');
  assert.equal(root.children[0].props.attributes.d, 'M4 12h16');
  assert.equal(root.children[0].props.attributes.fill, 'none');
  assert.equal(root.children[0].props.attributes.strokeLinecap, 'round');
  assert.equal(root.children[0].props.attributes.strokeLinejoin, 'round');
});

test('intrinsic global HTML attributes preserve Web JSX names', () => {
  const root = jsx('section', {
    title: 'Profile summary',
    hidden: true,
    lang: 'en-US',
    dir: 'rtl',
    tabIndex: -1,
    role: 'region',
    'aria-labelledby': 'profile-title',
    'aria-describedby': 'profile-help',
    'aria-controls': 'profile-panel',
    'aria-description': 'Profile summary panel',
    'aria-roledescription': 'profile region',
    'aria-keyshortcuts': 'Alt+P',
    'aria-valuetext': 'Complete',
    'aria-level': 2,
    'aria-posinset': 3,
    'aria-setsize': 10,
    'aria-rowcount': 20,
    'aria-rowindex': 4,
    'aria-rowspan': 2,
    'aria-colcount': 6,
    'aria-colindex': 5,
    'aria-colspan': 3,
    'aria-rowindextext': 'Row four',
    'aria-colindextext': 'Column five',
    'aria-sort': 'ascending',
    'aria-hidden': false,
    'aria-autocomplete': 'list',
    'aria-multiline': true,
    'aria-current': 'page',
    'aria-haspopup': 'dialog',
    'aria-pressed': 'mixed',
    'aria-live': 'polite',
    'aria-atomic': true,
    'aria-busy': false,
    'aria-relevant': 'additions text',
    'aria-modal': true,
    accessKey: 'p',
    contentEditable: 'plaintext-only',
    draggable: true,
    spellCheck: false,
    translate: 'no',
    inert: true,
    popover: true,
    anchor: 'profile-card-anchor',
    is: 'profile-card',
    nonce: 'nonce-1',
    slot: 'summary',
    part: 'panel header',
    exportParts: 'header: panel-header',
    itemScope: true,
    itemProp: 'profile',
    itemType: 'https://schema.org/ProfilePage',
    itemID: 'https://example.test/profiles/1',
    itemRef: 'profile-name profile-email',
  }, 'profile');

  assert.equal(root.props.attributes.title, 'Profile summary');
  assert.equal(root.props.attributes.hidden, 'true');
  assert.equal(root.props.attributes.lang, 'en-US');
  assert.equal(root.props.attributes.dir, 'rtl');
  assert.equal(root.props.attributes.tabIndex, '-1');
  assert.equal(root.props.attributes.role, 'region');
  assert.equal(root.props.attributes['aria-labelledby'], 'profile-title');
  assert.equal(root.props.attributes['aria-describedby'], 'profile-help');
  assert.equal(root.props.attributes['aria-controls'], 'profile-panel');
  assert.equal(root.props.attributes['aria-description'], 'Profile summary panel');
  assert.equal(root.props.attributes['aria-roledescription'], 'profile region');
  assert.equal(root.props.attributes['aria-keyshortcuts'], 'Alt+P');
  assert.equal(root.props.attributes['aria-valuetext'], 'Complete');
  assert.equal(root.props.attributes['aria-level'], '2');
  assert.equal(root.props.attributes['aria-posinset'], '3');
  assert.equal(root.props.attributes['aria-setsize'], '10');
  assert.equal(root.props.attributes['aria-rowcount'], '20');
  assert.equal(root.props.attributes['aria-rowindex'], '4');
  assert.equal(root.props.attributes['aria-rowspan'], '2');
  assert.equal(root.props.attributes['aria-colcount'], '6');
  assert.equal(root.props.attributes['aria-colindex'], '5');
  assert.equal(root.props.attributes['aria-colspan'], '3');
  assert.equal(root.props.attributes['aria-rowindextext'], 'Row four');
  assert.equal(root.props.attributes['aria-colindextext'], 'Column five');
  assert.equal(root.props.attributes['aria-sort'], 'ascending');
  assert.equal(root.props.attributes['aria-hidden'], 'false');
  assert.equal(root.props.attributes['aria-autocomplete'], 'list');
  assert.equal(root.props.attributes['aria-multiline'], 'true');
  assert.equal(root.props.attributes['aria-current'], 'page');
  assert.equal(root.props.attributes['aria-haspopup'], 'dialog');
  assert.equal(root.props.attributes['aria-pressed'], 'mixed');
  assert.equal(root.props.attributes['aria-live'], 'polite');
  assert.equal(root.props.attributes['aria-atomic'], 'true');
  assert.equal(root.props.attributes['aria-busy'], 'false');
  assert.equal(root.props.attributes['aria-relevant'], 'additions text');
  assert.equal(root.props.attributes['aria-modal'], 'true');
  assert.equal(root.props.attributes.accessKey, 'p');
  assert.equal(root.props.attributes.contentEditable, 'plaintext-only');
  assert.equal(root.props.attributes.draggable, 'true');
  assert.equal(root.props.attributes.spellCheck, 'false');
  assert.equal(root.props.attributes.translate, 'no');
  assert.equal(root.props.attributes.inert, 'true');
  assert.equal(root.props.attributes.popover, 'true');
  assert.equal(root.props.attributes.anchor, 'profile-card-anchor');
  assert.equal(root.props.attributes.is, 'profile-card');
  assert.equal(root.props.attributes.nonce, 'nonce-1');
  assert.equal(root.props.attributes.slot, 'summary');
  assert.equal(root.props.attributes.part, 'panel header');
  assert.equal(root.props.attributes.exportParts, 'header: panel-header');
  assert.equal(root.props.attributes.itemScope, 'true');
  assert.equal(root.props.attributes.itemProp, 'profile');
  assert.equal(root.props.attributes.itemType, 'https://schema.org/ProfilePage');
  assert.equal(root.props.attributes.itemID, 'https://example.test/profiles/1');
  assert.equal(root.props.attributes.itemRef, 'profile-name profile-email');
});

test('intrinsic media and resource attributes preserve Web JSX names', () => {
  const image = jsx('img', {
    alt: 'Hero',
    src: '/hero.png',
    srcSet: '/hero.png 1x, /hero@2x.png 2x',
    sizes: '100vw',
    width: 640,
    height: 360,
    loading: 'lazy',
    decoding: 'async',
    fetchPriority: 'high',
    crossOrigin: 'anonymous',
    referrerPolicy: 'no-referrer',
  }, 'hero');
  const video = jsxs('video', {
    src: '/demo.mp4',
    poster: '/poster.png',
    controls: true,
    autoPlay: true,
    loop: true,
    muted: true,
    playsInline: true,
    preload: 'metadata',
    children: jsx('track', {
      src: '/captions.vtt',
      kind: 'captions',
      srcLang: 'en',
      label: 'English',
      default: true,
    }, 'captions'),
  }, 'video');
  const stylesheet = jsx('link', {
    href: '/app.css',
    media: 'screen',
    type: 'text/css',
    fetchpriority: 'low',
    crossorigin: '',
    referrerpolicy: 'origin',
  }, 'stylesheet');

  assert.equal(image.props.attributes.alt, 'Hero');
  assert.equal(image.props.attributes.src, '/hero.png');
  assert.equal(image.props.attributes.srcSet, '/hero.png 1x, /hero@2x.png 2x');
  assert.equal(image.props.attributes.sizes, '100vw');
  assert.equal(image.props.attributes.width, '640');
  assert.equal(image.props.attributes.height, '360');
  assert.equal(image.props.attributes.loading, 'lazy');
  assert.equal(image.props.attributes.decoding, 'async');
  assert.equal(image.props.attributes.fetchPriority, 'high');
  assert.equal(image.props.attributes.crossOrigin, 'anonymous');
  assert.equal(image.props.attributes.referrerPolicy, 'no-referrer');
  assert.equal(image.props.alt, 'Hero');
  assert.equal(image.props.src, '/hero.png');
  assert.equal(image.props.srcset, '/hero.png 1x, /hero@2x.png 2x');
  assert.equal(image.props.sizes, '100vw');
  assert.equal(image.props.intrinsicWidth, 640);
  assert.equal(image.props.intrinsicHeight, 360);
  assert.equal(image.props.loading, 'lazy');
  assert.equal(image.props.decoding, 'async');
  assert.equal(image.props.fetchPriority, 'high');
  assert.equal(image.props.crossOrigin, 'anonymous');
  assert.equal(image.props.referrerPolicy, 'no-referrer');
  assert.equal(video.props.attributes.controls, 'true');
  assert.equal(video.props.attributes.autoPlay, 'true');
  assert.equal(video.props.attributes.playsInline, 'true');
  assert.equal(video.props.src, '/demo.mp4');
  assert.equal(video.props.poster, '/poster.png');
  assert.equal(video.props.controls, true);
  assert.equal(video.props.autoplay, true);
  assert.equal(video.props.loopPlayback, true);
  assert.equal(video.props.muted, true);
  assert.equal(video.props.playsInline, true);
  assert.equal(video.props.preload, 'metadata');
  assert.equal(video.children[0].props.attributes.srcLang, 'en');
  assert.equal(video.children[0].props.attributes.default, 'true');
  assert.equal(video.children[0].props.src, '/captions.vtt');
  assert.equal(video.children[0].props.trackKind, 'captions');
  assert.equal(video.children[0].props.srclang, 'en');
  assert.equal(video.children[0].props.trackLabel, 'English');
  assert.equal(video.children[0].props.defaultTrack, true);
  assert.equal(stylesheet.props.attributes.href, '/app.css');
  assert.equal(stylesheet.props.attributes.fetchpriority, 'low');
  assert.equal(stylesheet.props.attributes.crossorigin, '');
  assert.equal(stylesheet.props.href, '/app.css');
  assert.equal(stylesheet.props.media, 'screen');
  assert.equal(stylesheet.props.resourceType, 'text/css');
  assert.equal(stylesheet.props.fetchPriority, 'low');
  assert.equal(stylesheet.props.crossOrigin, '');
  assert.equal(stylesheet.props.referrerPolicy, 'origin');
});

test('intrinsic resource policy attributes preserve Web JSX names', () => {
  const anchor = jsx('a', {
    href: '/docs',
    target: '_blank',
    download: 'guide.pdf',
    ping: '/analytics',
    rel: 'noopener',
    hrefLang: 'en',
    referrerPolicy: 'no-referrer',
    children: 'Docs',
  }, 'docs');
  const preload = jsx('link', {
    rel: 'preload',
    as: 'image',
    href: '/hero.avif',
    integrity: 'sha384-link',
    blocking: 'render',
    imageSrcSet: '/hero.avif 1x',
    imageSizes: '100vw',
    disabled: true,
  }, 'preload');
  const script = jsx('script', {
    src: '/app.js',
    integrity: 'sha384-script',
    nonce: 'nonce-1',
    async: true,
    defer: true,
    noModule: true,
  }, 'script');
  const frame = jsx('iframe', {
    src: 'https://example.test/embed',
    name: 'preview',
    allow: 'fullscreen',
    allowFullScreen: true,
    sandbox: 'allow-scripts',
    srcDoc: '<p>Preview</p>',
  }, 'frame');

  assert.equal(anchor.props.attributes.target, '_blank');
  assert.equal(anchor.props.attributes.download, 'guide.pdf');
  assert.equal(anchor.props.attributes.ping, '/analytics');
  assert.equal(anchor.props.attributes.rel, 'noopener');
  assert.equal(anchor.props.attributes.hrefLang, 'en');
  assert.equal(anchor.props.attributes.referrerPolicy, 'no-referrer');
  assert.equal(preload.props.attributes.as, 'image');
  assert.equal(preload.props.attributes.integrity, 'sha384-link');
  assert.equal(preload.props.attributes.blocking, 'render');
  assert.equal(preload.props.attributes.imageSrcSet, '/hero.avif 1x');
  assert.equal(preload.props.attributes.imageSizes, '100vw');
  assert.equal(preload.props.attributes.disabled, 'true');
  assert.equal(script.props.attributes.async, 'true');
  assert.equal(script.props.attributes.defer, 'true');
  assert.equal(script.props.attributes.noModule, 'true');
  assert.equal(frame.props.attributes.name, 'preview');
  assert.equal(frame.props.attributes.allow, 'fullscreen');
  assert.equal(frame.props.attributes.allowFullScreen, 'true');
  assert.equal(frame.props.attributes.sandbox, 'allow-scripts');
  assert.equal(frame.props.attributes.srcDoc, '<p>Preview</p>');
});

test('intrinsic table and list attributes preserve Web JSX names', () => {
  const table = jsxs('table', {
    children: [
      jsxs('colgroup', {
        span: 2,
        children: jsx('col', {span: 3}, 'metric-col'),
      }, 'metric-cols'),
      jsxs('tr', {
        children: [
          jsx('th', {
            colSpan: 2,
            rowSpan: 3,
            headers: 'quarter revenue',
            scope: 'colgroup',
            abbr: 'Rev',
          }, 'heading'),
          jsx('td', {
            colspan: 4,
            rowspan: 1,
            headers: 'heading',
          }, 'cell'),
        ],
      }, 'metric-row'),
    ],
  }, 'metrics');
  const list = jsxs('ol', {
    start: 5,
    reversed: true,
    type: 'A',
    children: jsx('li', {
      value: 7,
      type: 'i',
      children: 'Inspect',
    }, 'step'),
  }, 'steps');

  assert.equal(table.children[0].props.attributes.span, '2');
  assert.equal(table.children[0].children[0].props.attributes.span, '3');
  assert.equal(table.children[1].children[0].props.attributes.colSpan, '2');
  assert.equal(table.children[1].children[0].props.attributes.rowSpan, '3');
  assert.equal(table.children[1].children[0].props.attributes.headers, 'quarter revenue');
  assert.equal(table.children[1].children[0].props.attributes.scope, 'colgroup');
  assert.equal(table.children[1].children[0].props.attributes.abbr, 'Rev');
  assert.equal(table.children[1].children[1].props.attributes.colspan, '4');
  assert.equal(table.children[1].children[1].props.attributes.rowspan, '1');
  assert.equal(list.props.attributes.start, '5');
  assert.equal(list.props.attributes.reversed, 'true');
  assert.equal(list.props.attributes.type, 'A');
  assert.equal(list.children[0].props.attributes.value, '7');
  assert.equal(list.children[0].props.attributes.type, 'i');
});

test('intrinsic form association and meter attributes preserve Web JSX names', () => {
  const label = jsx('label', {
    htmlFor: 'email',
    children: 'Email',
  }, 'email-label');
  const output = jsx('output', {
    for: 'price quantity',
    children: '$24',
  }, 'price-output');
  const meter = jsx('meter', {
    value: 73,
    min: 0,
    max: 100,
    low: 25,
    high: 90,
    optimum: 75,
  }, 'quota');

  assert.equal(label.props.attributes.htmlFor, 'email');
  assert.equal(output.props.attributes.for, 'price quantity');
  assert.equal(meter.props.valueNumber, 73);
  assert.equal(meter.props.minValue, 0);
  assert.equal(meter.props.maxValue, 100);
  assert.equal(meter.props.attributes.low, '25');
  assert.equal(meter.props.attributes.high, '90');
  assert.equal(meter.props.attributes.optimum, '75');
});

test('intrinsic activation attributes preserve Web JSX names', () => {
  const button = jsx('button', {
    command: 'show-modal',
    commandFor: 'settings-dialog',
    popoverTarget: 'settings-popover',
    popoverTargetAction: 'show',
    children: 'Settings',
  }, 'settings');
  const input = jsx('input', {
    type: 'button',
    value: 'Help',
    popovertarget: 'help-popover',
    popovertargetaction: 'toggle',
  }, 'help');

  assert.equal(button.props.attributes.command, 'show-modal');
  assert.equal(button.props.attributes.commandFor, 'settings-dialog');
  assert.equal(button.props.attributes.popoverTarget, 'settings-popover');
  assert.equal(button.props.attributes.popoverTargetAction, 'show');
  assert.equal(input.props.attributes.popovertarget, 'help-popover');
  assert.equal(input.props.attributes.popovertargetaction, 'toggle');
});

test('intrinsic text annotation attributes preserve Web JSX names', () => {
  const quote = jsx('blockquote', {
    cite: 'https://example.test/quote',
    children: 'Quoted paragraph',
  }, 'quote');
  const inserted = jsx('ins', {
    cite: 'https://example.test/change',
    dateTime: '2026-07-02T09:00:00Z',
    children: 'added',
  }, 'inserted');
  const removed = jsx('del', {
    datetime: '2026-07-01T18:00:00Z',
    children: 'removed',
  }, 'removed');
  const time = jsx('time', {
    dateTime: '2026-07-02',
    children: 'Today',
  }, 'today');

  assert.equal(quote.props.attributes.cite, 'https://example.test/quote');
  assert.equal(inserted.props.attributes.cite, 'https://example.test/change');
  assert.equal(inserted.props.attributes.dateTime, '2026-07-02T09:00:00Z');
  assert.equal(removed.props.attributes.datetime, '2026-07-01T18:00:00Z');
  assert.equal(time.props.attributes.dateTime, '2026-07-02');
});

test('intrinsic dialog attributes preserve Web JSX names', () => {
  const dialog = jsx('dialog', {
    open: true,
    children: 'Settings',
  }, 'settings');

  assert.equal(dialog.props.attributes.open, 'true');
});

test('CSS text parser preserves delimiters inside functions and strings', () => {
  const root = jsxs('div', {
    style: `
      color: rgb(10 20 30 / 50%);
      border-color: color-mix(in srgb, red 40%, blue) !important;
      border-color: #fff;
      background-image: url("https://example.com/a:b;c.svg");
      content: "label: value; still text";
      --accent: color-mix(in srgb, rebeccapurple 40%, white);
      /* ignored comment: with delimiter; */
      padding-inline: 1rem 2rem;
    `,
    children: 'Styled',
  }, 'styled');

  assert.equal(root.props.style.color, 'rgb(10 20 30 / 50%)');
  assert.equal(root.props.style['border-color'], 'color-mix(in srgb, red 40%, blue)');
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

test('web and aria state attributes normalize to native control props', () => {
  const root = jsxs(Slider, {
    disabled: true,
    required: true,
    selected: true,
    isReadOnly: true,
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
  assert.equal(root.props.isReadOnly, true);
  assert.equal(root.props.isExpanded, true);
  assert.equal(root.props.valueNumber, 50);
  assert.equal(root.props.minValue, 0);
  assert.equal(root.props.maxValue, 100);
  assert.equal(root.props.orientation, 'horizontal');
  assert.equal(root.props.attributes['aria-label'], 'Volume');
  assert.equal(root.props.events.onChange, 'setVolume');
});

test('intrinsic range input normalizes numeric value props', () => {
  const root = jsx('input', {
    type: 'range',
    value: 42,
    min: 0,
    max: 100,
    step: 5,
    onChange: 'setVolume',
  }, 'volume');

  assert.equal(root.tag, 'input');
  assert.equal(root.props.valueNumber, 42);
  assert.equal(root.props.stepValue, 5);
  assert.equal(root.props.value, undefined);
  assert.equal(root.props.minValue, 0);
  assert.equal(root.props.maxValue, 100);
  assert.equal(root.props.attributes.type, 'range');
  assert.equal(root.props.events.onChange, 'setVolume');
});

test('intrinsic number input normalizes numeric value props', () => {
  const root = jsx('input', {
    type: 'number',
    defaultValue: '7',
    min: '1',
    max: '10',
    step: '0.5',
    onChange: 'setQuantity',
  }, 'quantity');

  assert.equal(root.tag, 'input');
  assert.equal(root.props.valueNumber, 7);
  assert.equal(root.props.value, undefined);
  assert.equal(root.props.minValue, 1);
  assert.equal(root.props.maxValue, 10);
  assert.equal(root.props.stepValue, 0.5);
  assert.equal(root.props.attributes.type, 'number');
  assert.equal(root.props.events.onChange, 'setQuantity');
});

test('intrinsic form control attributes preserve Web JSX names', () => {
  const root = jsx('input', {
    type: 'email',
    name: 'email',
    form: 'profile-form',
    readOnly: true,
    autoFocus: true,
    autoComplete: 'email',
    inputMode: 'email',
    enterKeyHint: 'send',
    autoCapitalize: 'sentences',
    autoCorrect: 'on',
    virtualKeyboardPolicy: 'manual',
    accept: 'image/*',
    capture: 'environment',
    alt: 'Profile photo',
    src: '/photo.png',
    list: 'email-options',
    dirname: 'email.dir',
    formAction: '/profiles',
    formEncType: 'multipart/form-data',
    formMethod: 'post',
    formTarget: '_blank',
    formNoValidate: true,
    pattern: '.+@example\\.com',
    minLength: 3,
    maxLength: 64,
    size: 32,
  }, 'email');

  assert.equal(root.props.isReadOnly, true);
  assert.equal(root.props.attributes.readOnly, 'true');
  assert.equal(root.props.attributes.name, 'email');
  assert.equal(root.props.attributes.form, 'profile-form');
  assert.equal(root.props.attributes.autoFocus, 'true');
  assert.equal(root.props.attributes.autoComplete, 'email');
  assert.equal(root.props.attributes.inputMode, 'email');
  assert.equal(root.props.attributes.enterKeyHint, 'send');
  assert.equal(root.props.attributes.autoCapitalize, 'sentences');
  assert.equal(root.props.attributes.autoCorrect, 'on');
  assert.equal(root.props.attributes.virtualKeyboardPolicy, 'manual');
  assert.equal(root.props.attributes.accept, 'image/*');
  assert.equal(root.props.attributes.capture, 'environment');
  assert.equal(root.props.attributes.alt, 'Profile photo');
  assert.equal(root.props.attributes.src, '/photo.png');
  assert.equal(root.props.attributes.list, 'email-options');
  assert.equal(root.props.attributes.dirname, 'email.dir');
  assert.equal(root.props.attributes.formAction, '/profiles');
  assert.equal(root.props.attributes.formEncType, 'multipart/form-data');
  assert.equal(root.props.attributes.formMethod, 'post');
  assert.equal(root.props.attributes.formTarget, '_blank');
  assert.equal(root.props.attributes.formNoValidate, 'true');
  assert.equal(root.props.attributes.pattern, '.+@example\\.com');
  assert.equal(root.props.attributes.minLength, '3');
  assert.equal(root.props.attributes.maxLength, '64');
  assert.equal(root.props.attributes.size, '32');
  assert.equal(root.props.inputType, 'email');
  assert.equal(root.props.name, 'email');
  assert.equal(root.props.form, 'profile-form');
  assert.equal(root.props.accept, 'image/*');
  assert.equal(root.props.capture, 'environment');
  assert.equal(root.props.alt, 'Profile photo');
  assert.equal(root.props.src, '/photo.png');
  assert.equal(root.props.list, 'email-options');
  assert.equal(root.props.dirname, 'email.dir');
  assert.equal(root.props.formAction, '/profiles');
  assert.equal(root.props.formEnctype, 'multipart/form-data');
  assert.equal(root.props.formMethod, 'post');
  assert.equal(root.props.formTarget, '_blank');
  assert.equal(root.props.formNoValidate, true);
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
  const frame = createUiFrame('profile', root, {
    window: {
      title: '',
      width: 640,
      height: 480,
      minWidth: 320,
      maxWidth: 1280,
      resizable: false,
    },
  });

  assert.deepEqual(frame.window, {
    title: '',
    width: 640,
    height: 480,
    minWidth: 320,
    maxWidth: 1280,
    resizable: false,
  });
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
      window: {title: 'Profile', resizable: 'false'},
    }),
    /resizable.*boolean/,
  );
});
