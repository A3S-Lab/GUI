import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';

import * as Gui from '../src/index.js';
import {
  Button,
  Link,
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
