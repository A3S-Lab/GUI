# Structured Web Input

`a3s-gui` accepts structured element records generated from JSX. The example
below uses supported React Aria component names and DOM-style props that the
TypeScript SDK serializes into a `UiFrame`:

```tsx
/** @jsxImportSource @a3s-lab/gui */
import {
  Button,
  TextField,
  Label,
  Input,
} from 'react-aria-components';
import {
  createAction,
  createUiFrame,
  defineAction,
} from '@a3s-lab/gui';

const setEmail = createAction('setEmail', 'Set email');
const saveProfile = createAction('saveProfile', 'Save profile');

const root = (
  <form className="profile-form" data-screen="profile">
    <TextField name="email" isRequired>
      <Label>Email</Label>
      <Input
        placeholder="you@example.com"
        style={{minWidth: 280}}
        onChange={setEmail}
      />
    </TextField>

    <Button className="primary" aria-label="Save profile" onPress={saveProfile}>
      Save
    </Button>
  </form>
);

export const frame = createUiFrame('profile', root, {
  window: {title: 'Profile', width: 640, height: 480, minWidth: 480},
  actions: [defineAction(setEmail), defineAction(saveProfile)],
});
```

The resulting frame contains semantic component names, DOM-style props, action
ids, and optional window options. Native widget selection happens after the
frame reaches the Rust renderer.

The zero-dependency SDK also exports component markers with the same names for
tests and compiler fixtures. The bridge reads serialized component identity and
props; DOM nodes are not part of the protocol.

## Compiler Bridge Shape

The compiler bridge lowers JSX into a serializable tree with the same
information carried by JSX props:

- supported component identity
- stable element key
- text children
- `className`
- inline `style`
- `aria-*`, `data-*`, and portable HTML attributes
- `onClick`, `onChange`, `onPress`, `onKeyDown`, `onKeyUp`, and other event
  prop names with named function callbacks
- React Aria state props such as `isDisabled`, `isSelected`, `isRequired`, and
  `isInvalid`
- HTML and ARIA state aliases such as `disabled`, `required`,
  `aria-disabled`, `aria-expanded`, `aria-valuemin`, `aria-valuemax`, and
  `aria-valuenow`

The Rust core maps that tree into `NativeElement` and `NativeProps` through
`ReactCompilerBridge`, then renders it with `GuiRuntime` and the keyed
`Renderer`.

## Native Lowering Rules

| Input field | Native IR output |
| --- | --- |
| `className` | preserved in `WebProps.class_name` for style resolution |
| `style={{...}}` | preserved in `WebProps.style` and parsed into `PortableStyle`; non-rendered styles such as `display: none`, `visibility: hidden` / `collapse`, and `content-visibility: hidden` make the native widget config invisible, while `interactivity: inert` makes the native widget config inert |
| `aria-label` | used as the explicit native accessibility label before descendant text fallback |
| `aria-*` | preserved as accessibility metadata; supported state attributes also feed native control state |
| `aria-labelledby` / `aria-describedby` / `aria-details` / `aria-controls` / `aria-owns` / `aria-flowto` / `aria-errormessage` / `aria-activedescendant` | normalized to native accessibility relationship hints and preserved as metadata |
| `aria-description` / `aria-roledescription` / `aria-keyshortcuts` / `aria-valuetext` | normalized to native accessibility description/value hints and preserved as metadata |
| `aria-level` / `aria-posinset` / `aria-setsize` / `aria-rowcount` / `aria-rowindex` / `aria-rowspan` / `aria-colcount` / `aria-colindex` / `aria-colspan` / `aria-rowindextext` / `aria-colindextext` / `aria-sort` | normalized to native accessibility structure hints and preserved as metadata |
| `aria-hidden` / `aria-autocomplete` / `aria-multiline` / `aria-current` / `aria-haspopup` / `aria-pressed` / `aria-live` / `aria-atomic` / `aria-busy` / `aria-relevant` / `aria-modal` | normalized to native accessibility state hints and preserved as metadata; `aria-hidden` does not change visual widget visibility, and `aria-hidden="true"` omits the subtree from rendered accessibility trees |
| `data-*` | preserved as metadata for testing, analytics, and automation |
| `disabled` / `required` / `checked` / `selected` | normalized to React Aria-style native control state |
| `min` / `max` / `step` / `aria-valuenow` | normalized to native ranged control state |
| `readOnly` / `multiple` / `autoFocus` | normalized to native control state; `autoFocus` seeds initial runtime focus for rendered accessibility trees and lets real native surfaces request platform focus where their bindings expose it, `readOnly` suppresses value-changing events, and `readOnly` plus `multiple` are exposed in rendered accessibility trees |
| `autoComplete` / `inputMode` / `enterKeyHint` / `autoCapitalize` / `autoCorrect` / `virtualKeyboardPolicy` / `pattern` | normalized to native text-entry hints and preserved as metadata |
| `minLength` / `maxLength` / `rows` / `cols` / `size` | normalized to native numeric control hints and preserved as metadata |
| `input type="hidden"` | preserves form `name`, `form`, `type`, and `value` state but marks the native widget config invisible and suppresses rendered accessibility and native event projection |
| dialog `open` | normalized to native dialog visibility state when applicable; intrinsic dialogs without `open` are invisible |
| `title` / `hidden` / `lang` / `dir` / `tabIndex` / `role` / `accessKey` / `contentEditable` / `draggable` / `spellCheck` / `translate` / `inert` / `popover` / `anchor` / `is` / global `nonce` | normalized to native global HTML hints and preserved as metadata; native surfaces map `title` to platform tooltip/title hints where available, `hidden` also makes the native widget config invisible, and `hidden` or `inert` suppress native events and accessibility projection for the subtree |
| `slot` / `part` / `exportParts` | normalized to native shadow distribution and style-part hints when applicable |
| `itemScope` / `itemProp` / `itemType` / `itemID` / `itemRef` | normalized to native microdata metadata hints when applicable |
| `name` / `form` / `type` / `accept` / `capture` / `alt` / `src` / `list` / `dirname` | normalized to native HTML form and media control hints when applicable |
| `action` / `method` / `encType` / `target` / `noValidate` / `formAction` / `formEncType` / `formMethod` / `formTarget` / `formNoValidate` | normalized to native form submission hints when applicable |
| `href` / `srcSet` / `sizes` / `media` / `width` / `height` / `loading` / `decoding` / `fetchPriority` / `crossOrigin` / `referrerPolicy` | normalized to native media and resource loading hints when applicable |
| `poster` / `controls` / `autoPlay` / `loop` / `muted` / `playsInline` / `preload` / `kind` / `srcLang` / `label` / `default` | normalized to native media playback and track hints when applicable |
| `download` / `ping` / `rel` / `hrefLang` / link `as` / `integrity` / `blocking` / `nonce` / `imageSrcSet` / `imageSizes` / link `disabled` / script `async` / `defer` / `noModule` / iframe `allow` / `allowFullScreen` / `sandbox` / `srcDoc` | normalized to native resource policy hints when applicable |
| button `command` / `commandFor` / `popoverTarget` / `popoverTargetAction` / input `popoverTarget` / `popoverTargetAction` | normalized to native activation and popover command hints when applicable |
| quote/change `cite` / change `dateTime` / time `dateTime` | normalized to native text annotation citation and temporal hints when applicable |
| label `htmlFor` / label `for` / output `for` / meter `low` / `high` / `optimum` | normalized to native form association and meter range metadata when applicable |
| `colSpan` / `rowSpan` / `headers` / `scope` / `abbr` / `span` / `start` / `reversed` / list `type` / `li value` | normalized to native table and list structure hints when applicable |
| `onClick` / `onPress` | normalized to the primary native action |
| `onChange` / `onInput` | normalized to the primary action for change, selection, and value-toggle controls; `onChange` wins when both are present |
| `onFocus` / `onBlur` / `onFocusChange` | routed from native focus and blur events; `onFocusChange` receives boolean string payloads |
| `onToggle` / `onExpandedChange` | routed from native toggle events; expanded controls receive boolean string payloads |
| `onKeyDown` / `onKeyUp` | routed from native keyboard events; `NativeEvent.value` carries the key or shortcut token when the host supplies one |
| `TextField` + `Label` + `Input` | folded into one native text field; `Input` value, placeholder, style, metadata, and events are inherited |
| `textarea` direct text children | projected as native text-field value when no explicit value is supplied |
| `Select` + `ListBoxItem` | folded into a native select with native options |

## Native Adapter Contract

The current platform planning adapters map the same native IR to target widget
families:

| Native role | macOS AppKit | Windows WinUI | Linux GTK4 |
| --- | --- | --- | --- |
| `Button` | `NSButton` | `Microsoft.UI.Xaml.Controls.Button` | `gtk::Button` |
| `Document` | `NSView(document)` | `Microsoft.UI.Xaml.Controls.StackPanel(document)` | `gtk::Box(document)` |
| `DocumentHead` | `NSView(document-head)` | `Microsoft.UI.Xaml.Controls.StackPanel(document-head)` | `gtk::Box(document-head)` |
| `DocumentBody` | `NSView(document-body)` | `Microsoft.UI.Xaml.Controls.StackPanel(document-body)` | `gtk::Box(document-body)` |
| `DocumentTitle` | `NSTextField(document-title)` | `Microsoft.UI.Xaml.Controls.TextBlock(document-title)` | `gtk::Label(document-title)` |
| `Metadata` | `NSView(metadata)` | `Microsoft.UI.Xaml.Controls.StackPanel(metadata)` | `gtk::Box(metadata)` |
| `ResourceLink` | `NSView(resource-link)` | `Microsoft.UI.Xaml.Controls.StackPanel(resource-link)` | `gtk::Box(resource-link)` |
| `StyleSheet` | `NSView(style-sheet)` | `Microsoft.UI.Xaml.Controls.StackPanel(style-sheet)` | `gtk::Box(style-sheet)` |
| `Script` | `NSView(script)` | `Microsoft.UI.Xaml.Controls.StackPanel(script)` | `gtk::Box(script)` |
| `Template` | `NSView(template)` | `Microsoft.UI.Xaml.Controls.StackPanel(template)` | `gtk::Box(template)` |
| `Slot` | `NSView(slot)` | `Microsoft.UI.Xaml.Controls.StackPanel(slot)` | `gtk::Box(slot)` |
| `Heading` | `NSTextField(heading)` | `Microsoft.UI.Xaml.Controls.TextBlock(heading)` | `gtk::Label(heading)` |
| `Abbreviation` | `NSTextField(abbreviation)` | `Microsoft.UI.Xaml.Controls.TextBlock(abbreviation)` | `gtk::Label(abbreviation)` |
| `Citation` | `NSTextField(citation)` | `Microsoft.UI.Xaml.Controls.TextBlock(citation)` | `gtk::Label(citation)` |
| `Definition` | `NSTextField(definition)` | `Microsoft.UI.Xaml.Controls.TextBlock(definition)` | `gtk::Label(definition)` |
| `DataValue` | `NSTextField(data-value)` | `Microsoft.UI.Xaml.Controls.TextBlock(data-value)` | `gtk::Label(data-value)` |
| `InsertedText` | `NSTextField(inserted-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(inserted-text)` | `gtk::Label(inserted-text)` |
| `DeletedText` | `NSTextField(deleted-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(deleted-text)` | `gtk::Label(deleted-text)` |
| `MarkedText` | `NSTextField(marked-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(marked-text)` | `gtk::Label(marked-text)` |
| `Time` | `NSTextField(time)` | `Microsoft.UI.Xaml.Controls.TextBlock(time)` | `gtk::Label(time)` |
| `Emphasis` | `NSTextField(emphasis)` | `Microsoft.UI.Xaml.Controls.TextBlock(emphasis)` | `gtk::Label(emphasis)` |
| `StrongText` | `NSTextField(strong-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(strong-text)` | `gtk::Label(strong-text)` |
| `Code` | `NSTextField(code)` | `Microsoft.UI.Xaml.Controls.TextBlock(code)` | `gtk::Label(code)` |
| `KeyboardInput` | `NSTextField(keyboard-input)` | `Microsoft.UI.Xaml.Controls.TextBlock(keyboard-input)` | `gtk::Label(keyboard-input)` |
| `SampleOutput` | `NSTextField(sample-output)` | `Microsoft.UI.Xaml.Controls.TextBlock(sample-output)` | `gtk::Label(sample-output)` |
| `Variable` | `NSTextField(variable)` | `Microsoft.UI.Xaml.Controls.TextBlock(variable)` | `gtk::Label(variable)` |
| `InlineQuote` | `NSTextField(inline-quote)` | `Microsoft.UI.Xaml.Controls.TextBlock(inline-quote)` | `gtk::Label(inline-quote)` |
| `Subscript` | `NSTextField(subscript)` | `Microsoft.UI.Xaml.Controls.TextBlock(subscript)` | `gtk::Label(subscript)` |
| `Superscript` | `NSTextField(superscript)` | `Microsoft.UI.Xaml.Controls.TextBlock(superscript)` | `gtk::Label(superscript)` |
| `SmallText` | `NSTextField(small-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(small-text)` | `gtk::Label(small-text)` |
| `BoldText` | `NSTextField(bold-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(bold-text)` | `gtk::Label(bold-text)` |
| `ItalicText` | `NSTextField(italic-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(italic-text)` | `gtk::Label(italic-text)` |
| `StruckText` | `NSTextField(struck-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(struck-text)` | `gtk::Label(struck-text)` |
| `UnderlinedText` | `NSTextField(underlined-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(underlined-text)` | `gtk::Label(underlined-text)` |
| `BidirectionalIsolate` | `NSTextField(bidi-isolate)` | `Microsoft.UI.Xaml.Controls.TextBlock(bidi-isolate)` | `gtk::Label(bidi-isolate)` |
| `BidirectionalOverride` | `NSTextField(bidi-override)` | `Microsoft.UI.Xaml.Controls.TextBlock(bidi-override)` | `gtk::Label(bidi-override)` |
| `Paragraph` | `NSView(paragraph)` | `Microsoft.UI.Xaml.Controls.StackPanel(paragraph)` | `gtk::Box(paragraph)` |
| `PreformattedText` | `NSView(preformatted-text)` | `Microsoft.UI.Xaml.Controls.StackPanel(preformatted-text)` | `gtk::Box(preformatted-text)` |
| `BlockQuote` | `NSView(block-quote)` | `Microsoft.UI.Xaml.Controls.StackPanel(block-quote)` | `gtk::Box(block-quote)` |
| `ContactAddress` | `NSView(contact-address)` | `Microsoft.UI.Xaml.Controls.StackPanel(contact-address)` | `gtk::Box(contact-address)` |
| `LineBreak` | `NSTextField(line-break)` | `Microsoft.UI.Xaml.Controls.TextBlock(line-break)` | `gtk::Label(line-break)` |
| `WordBreakOpportunity` | `NSTextField(word-break-opportunity)` | `Microsoft.UI.Xaml.Controls.TextBlock(word-break-opportunity)` | `gtk::Label(word-break-opportunity)` |
| `NoBreakText` | `NSView(no-break-text)` | `Microsoft.UI.Xaml.Controls.StackPanel(no-break-text)` | `gtk::Box(no-break-text)` |
| `CenteredText` | `NSView(centered-text)` | `Microsoft.UI.Xaml.Controls.StackPanel(centered-text)` | `gtk::Box(centered-text)` |
| `FontText` | `NSView(font-text)` | `Microsoft.UI.Xaml.Controls.StackPanel(font-text)` | `gtk::Box(font-text)` |
| `BigText` | `NSView(big-text)` | `Microsoft.UI.Xaml.Controls.StackPanel(big-text)` | `gtk::Box(big-text)` |
| `TeletypeText` | `NSView(teletype-text)` | `Microsoft.UI.Xaml.Controls.StackPanel(teletype-text)` | `gtk::Box(teletype-text)` |
| `Applet` | `NSView(applet)` | `Microsoft.UI.Xaml.Controls.StackPanel(applet)` | `gtk::Box(applet)` |
| `BackgroundSound` | `NSView(background-sound)` | `Microsoft.UI.Xaml.Controls.StackPanel(background-sound)` | `gtk::Box(background-sound)` |
| `Frame` | `NSView(frame)` | `Microsoft.UI.Xaml.Controls.StackPanel(frame)` | `gtk::Box(frame)` |
| `FrameSet` | `NSView(frameset)` | `Microsoft.UI.Xaml.Controls.StackPanel(frameset)` | `gtk::Box(frameset)` |
| `NoEmbedFallback` | `NSView(noembed-fallback)` | `Microsoft.UI.Xaml.Controls.StackPanel(noembed-fallback)` | `gtk::Box(noembed-fallback)` |
| `NoFramesFallback` | `NSView(noframes-fallback)` | `Microsoft.UI.Xaml.Controls.StackPanel(noframes-fallback)` | `gtk::Box(noframes-fallback)` |
| `Marquee` | `NSView(marquee)` | `Microsoft.UI.Xaml.Controls.StackPanel(marquee)` | `gtk::Box(marquee)` |
| `Math` | `NSView(math)` | `Microsoft.UI.Xaml.Controls.StackPanel(math)` | `gtk::Box(math)` |
| `NextId` | `NSView(nextid)` | `Microsoft.UI.Xaml.Controls.StackPanel(nextid)` | `gtk::Box(nextid)` |
| `SelectedContent` | `NSView(selected-content)` | `Microsoft.UI.Xaml.Controls.StackPanel(selected-content)` | `gtk::Box(selected-content)` |
| `HeadingGroup` | `NSView(heading-group)` | `Microsoft.UI.Xaml.Controls.StackPanel(heading-group)` | `gtk::Box(heading-group)` |
| `Ruby` | `NSView(ruby)` | `Microsoft.UI.Xaml.Controls.StackPanel(ruby)` | `gtk::Box(ruby)` |
| `RubyBase` | `NSTextField(ruby-base)` | `Microsoft.UI.Xaml.Controls.TextBlock(ruby-base)` | `gtk::Label(ruby-base)` |
| `RubyText` | `NSTextField(ruby-text)` | `Microsoft.UI.Xaml.Controls.TextBlock(ruby-text)` | `gtk::Label(ruby-text)` |
| `RubyParenthesis` | `NSTextField(ruby-parenthesis)` | `Microsoft.UI.Xaml.Controls.TextBlock(ruby-parenthesis)` | `gtk::Label(ruby-parenthesis)` |
| `RubyTextContainer` | `NSView(ruby-text-container)` | `Microsoft.UI.Xaml.Controls.StackPanel(ruby-text-container)` | `gtk::Box(ruby-text-container)` |
| `Main` | `NSView(main)` | `Microsoft.UI.Xaml.Controls.StackPanel(main)` | `gtk::Box(main)` |
| `Navigation` | `NSView(navigation)` | `Microsoft.UI.Xaml.Controls.StackPanel(navigation)` | `gtk::Box(navigation)` |
| `Header` | `NSView(header)` | `Microsoft.UI.Xaml.Controls.StackPanel(header)` | `gtk::Box(header)` |
| `Footer` | `NSView(footer)` | `Microsoft.UI.Xaml.Controls.StackPanel(footer)` | `gtk::Box(footer)` |
| `Article` | `NSView(article)` | `Microsoft.UI.Xaml.Controls.StackPanel(article)` | `gtk::Box(article)` |
| `Section` | `NSView(section)` | `Microsoft.UI.Xaml.Controls.StackPanel(section)` | `gtk::Box(section)` |
| `Aside` | `NSView(aside)` | `Microsoft.UI.Xaml.Controls.StackPanel(aside)` | `gtk::Box(aside)` |
| `Search` | `NSView(search)` | `Microsoft.UI.Xaml.Controls.StackPanel(search)` | `gtk::Box(search)` |
| `Disclosure` | `NSView(disclosure)` | `Microsoft.UI.Xaml.Controls.StackPanel(disclosure)` | `gtk::Box(disclosure)` |
| `DisclosureSummary` | `NSButton(disclosure-summary)` | `Microsoft.UI.Xaml.Controls.Button(disclosure-summary)` | `gtk::Button(disclosure-summary)` |
| `Figure` | `NSView(figure)` | `Microsoft.UI.Xaml.Controls.StackPanel(figure)` | `gtk::Box(figure)` |
| `FigureCaption` | `NSTextField(figure-caption)` | `Microsoft.UI.Xaml.Controls.TextBlock(figure-caption)` | `gtk::Label(figure-caption)` |
| `DescriptionList` | `NSView(description-list)` | `Microsoft.UI.Xaml.Controls.StackPanel(description-list)` | `gtk::Box(description-list)` |
| `DescriptionTerm` | `NSTextField(description-term)` | `Microsoft.UI.Xaml.Controls.TextBlock(description-term)` | `gtk::Label(description-term)` |
| `DescriptionDetails` | `NSView(description-details)` | `Microsoft.UI.Xaml.Controls.StackPanel(description-details)` | `gtk::Box(description-details)` |
| `TextField` / `input` | `NSTextField(input)` | `Microsoft.UI.Xaml.Controls.TextBox` | `gtk::Entry` |
| `TextField` from `input type="search"` | `NSSearchField` | `Microsoft.UI.Xaml.Controls.TextBox(search)` | `gtk::SearchEntry` |
| `TextField` from `input type="password"` | `NSSecureTextField` | `Microsoft.UI.Xaml.Controls.PasswordBox` | `gtk::PasswordEntry` |
| `TextField` from `textarea` | `NSTextField(textarea)` | `Microsoft.UI.Xaml.Controls.TextBox(textarea)` | `gtk::TextView` |
| `Image` | `NSImageView` | `Microsoft.UI.Xaml.Controls.Image` | `gtk::Picture` |
| `Media` | `AVPlayerView` | `Microsoft.UI.Xaml.Controls.MediaPlayerElement` | `gtk::Video` |
| `Canvas` | `NSView(canvas)` | `Microsoft.UI.Xaml.Controls.Canvas` | `gtk::DrawingArea` |
| `EmbeddedContent` | `NSView(embedded-content)` | `Microsoft.UI.Xaml.Controls.ContentControl(embedded-content)` | `gtk::Box(embedded-content)` |
| `Link` | `NSButton(link)` | `Microsoft.UI.Xaml.Controls.HyperlinkButton` | `gtk::LinkButton` |
| `ImageMap` | `NSView(image-map)` | `Microsoft.UI.Xaml.Controls.Canvas(image-map)` | `gtk::DrawingArea(image-map)` |
| `ImageMapArea` | `NSButton(image-map-area)` | `Microsoft.UI.Xaml.Controls.HyperlinkButton(image-map-area)` | `gtk::LinkButton(image-map-area)` |
| `Checkbox` | `NSButton(checkbox)` | `Microsoft.UI.Xaml.Controls.CheckBox` | `gtk::CheckButton` |
| `Switch` | `NSSwitch` | `Microsoft.UI.Xaml.Controls.ToggleSwitch` | `gtk::Switch` |
| `RadioGroup` | `NSStackView(radio-group)` | `Microsoft.UI.Xaml.Controls.RadioButtons` | `gtk::Box(radio-group)` |
| `Radio` | `NSButton(radio)` | `Microsoft.UI.Xaml.Controls.RadioButton` | `gtk::CheckButton(radio)` |
| `Form` | `NSView(form)` | `Microsoft.UI.Xaml.Controls.StackPanel(form)` | `gtk::Box(form)` |
| `FieldSet` | `NSView(fieldset)` | `Microsoft.UI.Xaml.Controls.StackPanel(fieldset)` | `gtk::Box(fieldset)` |
| `Legend` | `NSTextField(legend)` | `Microsoft.UI.Xaml.Controls.TextBlock(legend)` | `gtk::Label(legend)` |
| `OptionGroup` | `NSView(option-group)` | `Microsoft.UI.Xaml.Controls.StackPanel(option-group)` | `gtk::Box(option-group)` |
| `Output` | `NSTextField(output)` | `Microsoft.UI.Xaml.Controls.TextBlock(output)` | `gtk::Label(output)` |
| `Meter` | `NSProgressIndicator(meter)` | `Microsoft.UI.Xaml.Controls.ProgressBar(meter)` | `gtk::ProgressBar(meter)` |
| `Select` | `NSComboBox` | `Microsoft.UI.Xaml.Controls.ComboBox` | `gtk::DropDown` |
| `ListBox` | `NSScrollView+NSStackView` | `Microsoft.UI.Xaml.Controls.ListView` | `gtk::ListBox` |
| `ListBoxItem` | `NSButton(list-row)` | `Microsoft.UI.Xaml.Controls.ListViewItem` | `gtk::ListBoxRow` |
| `Dialog` | `NSPanel` | `Microsoft.UI.Xaml.Controls.ContentDialog` | `gtk::Dialog` |
| `Popover` | `NSPopover` | `Microsoft.UI.Xaml.Controls.ToolTip` | `gtk::Popover` |
| `Tabs` | `NSTabView` | `Microsoft.UI.Xaml.Controls.TabView` | `gtk::Notebook` |
| `Menu` | `NSMenu` | `Microsoft.UI.Xaml.Controls.StackPanel(menu)` | `gio::Menu` |
| `MenuItem` | `NSMenuItem` | `Microsoft.UI.Xaml.Controls.Button(menu-item)` | `gio::MenuItem` |
| `Separator` | `NSBox(separator)` | `Microsoft.UI.Xaml.Controls.Border(separator)` | `gtk::Separator` |
| `Slider` | `NSSlider` | `Microsoft.UI.Xaml.Controls.Slider` | `gtk::Scale` |
| `ProgressBar` | `NSProgressIndicator` | `Microsoft.UI.Xaml.Controls.ProgressBar` | `gtk::ProgressBar` |
| `Toolbar` | `NSStackView(toolbar)` | `Microsoft.UI.Xaml.Controls.StackPanel(toolbar)` | `gtk::Box(toolbar)` |
| `Table` | `NSTableView` | `Microsoft.UI.Xaml.Controls.Grid(table)` | `gtk::Grid(table)` |
| `TableSection` | `NSView(table-section)` | `Microsoft.UI.Xaml.Controls.StackPanel(table-section)` | `gtk::Box(table-section)` |
| `TableRow` | `NSTableRowView` | `Microsoft.UI.Xaml.Controls.Grid(row)` | `gtk::Grid(row)` |
| `TableCell` | `NSTableCellView` | `Microsoft.UI.Xaml.Controls.Grid(cell)` | `gtk::Grid(cell)` |
| `TableColumn` | `NSTableColumn` | `Microsoft.UI.Xaml.Controls.Grid(column)` | `gtk::ColumnViewColumn` |
| `TableCaption` | `NSTextField(table-caption)` | `Microsoft.UI.Xaml.Controls.TextBlock(table-caption)` | `gtk::Label(table-caption)` |

Top-level Web attributes such as `aria-label` are accepted by the compiler
bridge and preserved in the native accessibility metadata while also feeding the
native label. The renderer emits typed native commands for these widgets,
including updates and keyed reorders, so state changes do not remount stable
native controls.
Those commands are serializable and can be delivered to an OS-bound AppKit,
WinUI, or GTK host.

The backend execution path is independent of the source syntax:

```text
Structured TSX input
        |
        v
NativeElement
        |
        v
PlatformCommand::Create { widget_class: "NSButton" }
        |
        v
PlatformCommandExecutor
        |
        v
NativeWidgetDriver
        |
        v
HandleWidgetDriver / NativeHandleAdapter / NativeWidgetSurface
        |
        v
NSButton / WinUI Button / gtk::Button
```

The macOS `appkit-native` feature already exercises the rightmost side of this
pipeline in-process: `AppKitNativeSurface` maps create commands and
`NativeWidgetSetter` values to real AppKit objects for windows, views, buttons,
labels, text fields, checkboxes, switches, radio groups, radio buttons, combo
boxes, sliders, and progress indicators. Buttons enqueue native press events
through target/action callbacks, while editable text fields enqueue native
focus, change, and blur events through an `NSTextFieldDelegate`; change events
carry the current AppKit control value after max-length hints are applied.
Checkboxes, switches, and radio buttons enqueue native toggle events with the
current AppKit checked state. Radio groups use native `NSStackView` containers
with `NSButton(radio)` children. Select
children are inserted into `NSComboBox` as native object values and emit native
selection-change events. React Aria `Tabs` trees fold `TabList` and ordered
`TabPanel` children into native `NSTabViewItem` objects with panel views as
content, and tab selection changes emit native selection-change events. Sliders
apply native orientation and step hints, emit ranged change events with the
current double value, and progress indicators consume the same min/max/current
setter state.
The `appkit_controls`, `winui_controls`, and `gtk4_controls` examples share one
controls smoke frame so text input, toggles, sliders, selects, tabs, actions,
rerenders, and root-window close behavior are exercised through the same
protocol shape on each native surface. Native close requests route through
`window.onClose` action ids. The `appkit_dogfood`, `winui_dogfood`, and
`gtk4_dogfood` examples share a task editor frame that adds realistic app
state, menu commands, a review dialog, checklist gates, keyboard shortcuts,
window close lifecycle actions, a Close window command that stops the native app
loop from reducer state, logical sizing, and repeated reducer-driven rerenders
on top of the same native event path.

The Linux `gtk4-native` feature exercises the same path with `gtk4-rs`.
`Gtk4NativeSurface` maps the native command stream to real GTK4 widgets for
windows, boxes, labels, buttons, entries, check buttons, switches, drop-downs,
list boxes, rows, notebook tabs, separators, scales, and progress bars. React
Aria `Tabs` trees become native `gtk::Notebook` pages with `TabPanel` content
attached as native GTK widgets. GTK signals enqueue native press, change, focus,
blur, toggle, selection-change, key-down, and key-up events; programmatic setter
updates are suppressed so render diffs do not trigger serialized actions. The
feature is Linux-only and requires GTK4 development libraries plus `pkg-config`.
`Gtk4RuntimeApp` can own the GTK surface directly, pump the GLib main context,
rerender after reducer updates, emit `window.onClose` actions from native close
callbacks, and exit when the root GTK window closes.

The Windows `winui-native` feature follows the same contract with WinUI 3 and
the Windows App SDK. `WinUiNativeSurface` creates real XAML windows, panels,
text blocks, buttons, text boxes, checkboxes, radio buttons, combo boxes, list
boxes, tab views, tab view items, sliders, and progress bars through XAML
controls. React Aria `Tabs` trees become native `TabView` / `TabViewItem`
controls, with `TabPanel` content attached as native XAML content. WinUI events
are queued as native events and routed back to serialized action ids. The React
Aria `Switch` semantic remains in the IR, while `winio-winui3` 0.4.2 is bridged
through a native CheckBox-backed toggle until the generated WinUI bindings
expose `ToggleSwitch`. `WinUiRuntimeApp` can own the WinUI surface directly,
pump the Windows message queue, emit `window.onClose` actions from HWND
`WM_CLOSE` messages, rerender after reducer updates, and exit when the root
WinUI window closes.

At runtime the compiled tree crosses the host boundary as a `UiFrame`:

```json
{
  "frameId": "frame-1",
  "window": {"title": "Profile", "width": 640, "height": 480, "minWidth": 480},
  "actions": [{"id": "saveProfile"}],
  "root": {
    "kind": "element",
    "key": "save",
    "tag": "Button",
    "props": {"events": {"onPress": "saveProfile"}},
    "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
  }
}
```

Native input comes back as `HostEvent`, and `a3s-gui` resolves it to a validated
`ActionInvocation`.
Host events use the active non-empty `frameId`, a non-zero rendered host node
id, a supported native event kind, and an optional string `value`.

`frameId` must be a non-empty string, `root` must be a single compiled element,
and explicit action ids must be non-empty and unique. Text nodes and fragment
arrays are valid children, but not valid frame roots. Every compiled node needs
a non-empty key, element tags must be non-empty, and sibling keys must be unique.

If labels are not needed, the TypeScript SDK can infer `UiFrame.actions` from
the compiled event props. Explicit `defineAction(...)` calls are still useful
when the host wants labels for menus, logs, or command palettes.

The window wrapper is part of the host protocol, not the source component
tree. The source tree stays independent of host surface selection; hosts decide
which frames become windows, panels, or embedded surfaces. `window.width`,
`window.height`, and optional min/max dimensions are projected into portable
native style. Window dimensions must be positive finite numbers; min/max
constraints must not conflict with each other or with the explicit width/height.
`window.resizable` defaults to `true` and is projected into typed native
config/setter state while remaining available as protocol metadata for
compatibility. AppKit applies min/max constraints directly to the window frame;
WinUI enforces the same bounds through the native `WM_GETMINMAXINFO` resize
message.

Native platform hosts can use `NativeProtocolSession` as the frame boundary:

```rust
use a3s_gui::{Gtk4Adapter, HostEvent, NativeEvent, NativeEventKind, NativeProtocolSession};

let mut session = NativeProtocolSession::new(Gtk4Adapter);
let rendered = session.render_frame(&frame)?;
for command in &rendered.commands {
    // Apply Create/Update/InsertChild/Remove/SetRoot on the native UI thread.
    // For create/update commands, blueprint.config().create_setters() and
    // config.diff(&next).setters() return native setter operations.
}

let response = session.dispatch_host_event(&HostEvent {
    frame_id: rendered.frame_id,
    event: NativeEvent::new(rendered.root, NativeEventKind::Press),
})?;
```

The `state_loop` example shows the next host layer: render a frame, dispatch
native events, apply the returned action invocations to application state, and
render the next frame from that state.
Hosts can use `NativeProtocolApp` for that loop directly by supplying an
initial state value, a state-to-`UiFrame` builder, and an action reducer.
Rust-owned native hosts can use `NativeRuntimeApp` for the embedded form of the
same loop: drain pending native events, reduce action invocations, and rerender
the next frame into the existing `GuiRuntime`. Platform runtime app aliases add
the OS event pump for real native surfaces: `AppKitRuntimeApp` on macOS,
`WinUiRuntimeApp` on Windows, and `Gtk4RuntimeApp` on Linux.
Use `handle_pending_native_events_while` or the platform `run_*_while` methods
when reducer state can request app shutdown. The drain stops before the next
queued native event once the predicate returns false, so a Close window command
cannot be followed by stale input from the same event batch.

## Event Flow

React callbacks are compiled to stable action identifiers. Native adapters emit
typed events; `GuiRuntime` first updates portable focus/value/selection state in
`InteractionState`, then `EventRouter` maps the event back to the serialized
action id and `ActionRegistry` validates that the action exists. Empty event
action ids are ignored rather than dispatched. Rendering a new `UiFrame`
replaces the registered action set with that frame's declared actions only after
the native render succeeds, so failed renders keep the previous action scope.
Raw protocol frames may omit `actions`; in that case Rust infers the set from
compiled event props and `actionLabels`. Explicit `actions`, including an empty
list, override inference.

```text
onPress={saveProfile}
        |
        v
events: {"onPress": "saveProfile"}
        |
        v
NativeWidgetDriver callback
        |
        v
NativeEventSource queue
        |
        v
NativeEventKind::Press
        |
        v
ActionInvocation { action: "saveProfile" }
```

React Aria's `onPress` is preferred over `onClick` when both are present, so the
compiled frame keeps native press semantics.

Keyboard callbacks follow the same action-routing path. `onKeyDown` and
`onKeyUp` are routed from `NativeEventKind::KeyDown` and
`NativeEventKind::KeyUp`; hosts can put the key or shortcut token in
`NativeEvent.value`. If no explicit `onKeyDown` handler is present on the
target or its route ancestors, Enter and Space key-down events fall back to the
primary press action for activatable controls such as buttons, links, and menu
items. Keyboard activation is also normalized into `Toggle` or
`SelectionChange` events for checkboxes, switches, expanded controls, radios,
listbox items, and tabs, so interaction state and action payloads stay
semantic.
`createUiFrame` also accepts `window.onClose` as an action-like value. Rust
wraps the root in a native window with an `onClose` event binding, infers that
action when frame actions are omitted, and dispatches it from
`NativeEventKind::Close`. AppKit and GTK native surfaces emit close events from
their real window, panel, and dialog callbacks. WinUI emits the same event from
the HWND `WM_CLOSE` message path.
Invisible and inert controls suppress native events and rendered accessibility
projection for their subtree. Invisibility includes HTML `hidden`, CSS
`display: none`, `visibility: hidden` / `collapse`,
`content-visibility: hidden`, and closed intrinsic dialogs. Inert controls
include the HTML `inert` attribute and CSS `interactivity: inert`. Disabled
controls suppress user activation and input events. Read-only controls still
allow focus, blur, press, and explicit keyboard routing, but suppress value,
selection, and toggle events before state changes or action dispatch.
On the first render without prior focus history, the first renderable
`autoFocus` control seeds runtime focus for accessibility projection. AppKit
and GTK native surfaces also defer the request until the target is mounted and
then ask the platform to focus it; WinUI records the pending target but waits
for native focus callbacks because the current `winio-winui3` binding does not
wrap programmatic focus. Later native focus and blur events take ownership of
focus state.

For folded controls, event ownership follows the source element structure. For
example, `TextField` receives the visible label, while `Input` can own
`onChange`, `placeholder`, inline style, and `data-*` metadata; the native
renderer folds those into a single native text field.

## Compatibility Boundary

The portable input contract covers the supported JSX, DOM-style prop, ARIA,
HTML attribute, CSS, and Tailwind subset documented above. Browser DOM, CSSOM,
and `HTMLElement` instances are outside the runtime protocol and can be reported
by the compiler bridge before runtime.
