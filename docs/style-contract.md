# Native Style Contract

`WebProps` keeps the original Web style map. `PortableStyle` keeps normalized
CSS declarations and projects the subset that native adapters can apply
deterministically:

- `all`
- `display`, including inline, block, flow-root, contents, list-item, flex,
  grid, table, ruby, and representable multi-keyword display modes
- `boxSizing`, `boxDecorationBreak`, `isolation`, `mixBlendMode`
- `float`, `clear`, `verticalAlign`
- `tableLayout`, `borderCollapse`, `borderSpacing`, `captionSide`
- `position`, `anchorName`, `anchorScope`, `positionAnchor`,
  `positionArea`, `positionTry*`, `positionVisibility`, `inset`,
  `insetBlock*`, `insetInline*`, `start`, `end`, `top`, `right`, `bottom`,
  `left`, `zIndex`
- `visibility`
- `flexDirection`, `flexWrap`, `flex`, `flexBasis`, `flexGrow`,
  `flexShrink`, `order`, `readingFlow`, `readingOrder`
- `alignItems`, `alignContent`, `alignSelf`, `justifyContent`,
  `justifyItems`, `justifySelf`, `placeContent`, `placeItems`, `placeSelf`
- `grid`, `gridTemplate*`, `gridAutoColumns`, `gridAutoRows`, `gridAutoFlow`,
  `gridColumn*`, `gridRow*`, `gridArea`
- `contain`, `container*`, `content`, `counterReset`, `counterIncrement`,
  `counterSet`, `quotes`, `stringSet`, `contentVisibility`, and
  `containIntrinsic*`
- `width`, `height`, `inlineSize`, `blockSize`, `interpolateSize`, and
  physical/logical min/max sizes
- `gap`, `rowGap`, `columnGap`, physical and logical `padding*`, `margin*`,
  `marginTrim`, and Tailwind `space*` child-spacing metadata
- `border`, physical and logical `borderWidth`, `borderStyle`,
  `borderColor`, uniform, physical-corner, and logical-corner `borderRadius`
- `borderImage`, `borderImageSource`, `borderImageSlice`,
  `borderImageWidth`, `borderImageOutset`, and `borderImageRepeat`
- `color`, `background`, `backgroundColor`, `backgroundImage`, `backgroundPosition`,
  `backgroundSize`, `backgroundRepeat`, `backgroundAttachment`,
  `backgroundOrigin`, `backgroundClip`, `backgroundBlendMode`
- `clip`, `clipPath`, `mask*`, and `maskBorder*`
- `imageRendering`, `imageOrientation`, `imageResolution`, `objectFit`,
  `objectPosition`, `shapeOutside`, `shapeInside`, `shapeMargin`,
  `shapePadding`, `shapeImageThreshold`,
  `listStyleType`, `listStylePosition`, `listStyleImage`, `markerSide`,
  `columns`, `columnCount`, `columnWidth`, `columnRule*`, `columnSpan`,
  `columnFill`, `size`, `page`, `pageOrientation`, `bleed`, `marks`,
  `orphans`, `widows`, `bookmark*`, `footnote*`, `breakBefore`, `breakAfter`,
  `breakInside`
- `font`, `fontFamily`, `fontStyle`, `fontSize`, `fontSizeAdjust`,
  `fontWeight`, `fontStretch`, `fontWidth`, `fontPalette`, `fontLanguageOverride`,
  `fontKerning`, `fontOpticalSizing`, `WebkitFontSmoothing`,
  `MozOsxFontSmoothing`, `fontFeatureSettings`, `fontVariationSettings`,
  `fontVariant*`, `fontSynthesis*`, `lineHeight`, `lineHeightStep`,
  `blockStep*`, `lineGrid`, `lineSnap`, `boxSnap`, `mathDepth`, `mathShift`,
  `mathStyle`, `dominantBaseline`,
  `baselineSource`, `alignmentBaseline`, `baselineShift`, `lineFitEdge`,
  `inlineSizing`, `initialLetter*`,
  `letterSpacing`, `wordSpacing`, `tabSize`, `textAlign`, `textAlignAll`,
  `textAlignLast`, `textGroupAlign`, `textJustify`, `wordSpaceTransform`,
  `textSizeAdjust`, `WebkitTextSizeAdjust`,
  `MozTextSizeAdjust`, `MsTextSizeAdjust`, `direction`, `unicodeBidi`,
  `writingMode`, `textOrientation`,
  `textCombineUpright`, `textTransform`, `textIndent`, `textWrap`,
  `textWrapMode`, `textWrapStyle`, `wrapBefore`, `wrapAfter`, `wrapInside`,
  `linePadding`, `textSpacing`, `textSpacingTrim`, `textAutospace`,
  `textBox*`, `hangingPunctuation`, `lineClamp`, `blockEllipsis`,
  `continue`, `maxLines`, `boxOrient`
- `speak`, `speakAs`, `pause*`, `rest*`, `cue*`, and `voice*`
- SVG presentation properties such as `fill`, `fillOpacity`, `fillRule`,
  `clipRule`, `stroke`, `strokeWidth`, `strokeLinecap`, `strokeLinejoin`,
  `strokeMiterlimit`, `strokeDasharray`, `strokeDashoffset`, `strokeOpacity`,
  `vectorEffect`, `paintOrder`, `shapeRendering`, `textRendering`,
  `colorRendering`, `colorInterpolation`, `colorInterpolationFilters`,
  `marker*`, `stopColor`, `stopOpacity`, `floodColor`, `floodOpacity`, and
  `lightingColor`
- `textDecorationLine`, `textDecorationColor`, `textDecorationStyle`,
  `textDecorationThickness`, `textDecorationSkip*`, `textUnderlineOffset`,
  `textUnderlinePosition`, `textEmphasisStyle`, `textEmphasisColor`,
  `textEmphasisPosition`, `textEmphasisSkip`, `rubyAlign`, `rubyPosition`, `rubyMerge`,
  `rubyOverhang`, `textShadow`, `textOverflow`, `lineBreak`, `whiteSpace`,
  `whiteSpaceCollapse`, `whiteSpaceTrim`, `wordBreak`, `overflowWrap`,
  `hyphens`, `hyphenateCharacter`, and `hyphenateLimit*`
- `overflow`, `overflowX`, `overflowY`, `overflowBlock`, `overflowInline`,
  `overflowClipMargin`, `overflowAnchor`
- `emptyCells`
- `aspectRatio`, `boxShadow`, Tailwind `ring*` and `divide*` metadata,
  `outline*`, `transform`, `filter`,
  `backdropFilter`
- `translate`, `rotate`, `scale`, `transformOrigin`, `transformStyle`,
  `transformBox`, `offset*`, `backfaceVisibility`, `perspective`,
  `perspectiveOrigin`
- filter function components such as blur, brightness, contrast, drop shadow,
  grayscale, hue rotate, invert, saturate, and sepia
- backdrop-filter function components such as backdrop blur, brightness,
  contrast, grayscale, hue rotate, invert, opacity, saturate, and sepia
- `transition*`, `animation*`, `animationTimeline`, `animationRange*`,
  `viewTransition*`, `willChange`
- `colorScheme`, `forcedColorAdjust`, `printColorAdjust`, `colorAdjust`,
  `fieldSizing`, `appearance`, `accentColor`, `caretColor`, `caret`,
  `caretAnimation`, `caretShape`, `resize`
- `scrollBehavior`, `scrollTimeline*`, `viewTimeline*`, `timelineScope`,
  physical and logical `scrollMargin*`,
  `scrollPadding*`, `scrollSnap*`, `scrollbarGutter`, `scrollbarWidth`,
  `scrollbarColor`, `scrollInitialTarget`, `scrollTargetGroup`,
  `scrollMarkerGroup`, `overscrollBehavior*`, `overscrollBehaviorBlock`,
  `overscrollBehaviorInline`, `touchAction`, `nav*`, `spatialNavigation*`, and
  `interactivity`
- `cursor`, `pointerEvents`, `userSelect`
- `opacity`

CSS custom properties are stored separately. Tailwind variant utilities are
stored under `variant_declarations` so state or responsive processing can apply
them without reparsing `className`. Tailwind important utilities using the `!`
modifier are evaluated after normal utilities within the same `className`, with
the original relative order preserved inside each priority group. Tailwind
arbitrary values decode `_` as a space, preserve escaped `\_` as an underscore,
keep underscores inside `url(...)` values, and apply the same bracketed-segment
decoding to arbitrary variant keys. Unsupported style declarations are preserved
so callers can report unmapped declarations without dropping source data.
CSS length values that cannot be converted to numeric points or percentages are
kept as `StyleLength::Css`, including `calc(...)`, `calc-size(...)`,
`var(...)`, `clamp(...)`, `anchor(...)`, `anchor-size(...)`,
CSS math functions such as `round(...)`, `hypot(...)`, and `abs(...)`,
viewport/container units, and sizing keywords such as `min-content`.
CSS time values that cannot be converted to milliseconds are kept as
`StyleTime::Css`, including custom properties and CSS math functions.
CSS colors parse hex, RGB/RGBA, HSL/HSLA, and slash alpha syntax into portable
RGBA tokens when possible. Native CSS color functions such as `hwb(...)`,
`lab(...)`, `lch(...)`, `oklab(...)`, `oklch(...)`, `color(...)`,
`color-mix(...)`, `light-dark(...)`, `contrast-color(...)`, and
`alpha(...)`, and `device-cmyk(...)` are preserved as function color tokens.
Tailwind color opacity modifiers are preserved for both base and variant
utilities, including arbitrary color functions.
Common Tailwind visual-effect and interaction utilities such as `shadow-*`,
`shadow-(...)`, `shadow-(color:...)`, `inset-shadow-*`, `ring-*`,
`inset-ring-*`, `outline-*`, `cursor-*`, `pointer-events-*`, `select-*`,
`aspect-*`, `mix-blend-*`, `bg-blend-*`, and `mask-*` project into the same
declaration model.
Tailwind container marker utilities such as `@container`, `@container-size`,
and named container forms project into container declarations. Container query
variants such as `@md:` are stored in `variant_declarations`.
Common Tailwind formatting and table utilities such as `box-*`,
`box-decoration-*`, `isolate`, `isolation-auto`, `float-*`, `clear-*`,
`align-*`, `border-collapse`, `border-separate`, `border-spacing-*`, and
`caption-*`, plus arbitrary `empty-cells` and `border-image*` properties,
project into the same declaration model. Tailwind display
utilities such as `inline-block`, `flow-root`, `contents`, `list-item`,
`table-*`, `inline-table`, `inline-flex`, and `inline-grid` project into
portable display tokens. Arbitrary `display` properties project into the same
display token when the display value has an equivalent portable mode. Tailwind
screen-reader utilities such as `sr-only` and `not-sr-only` project into their
generated declaration groups.
Common Tailwind SVG presentation utilities such as `fill-*`, `stroke-*`, and
`stroke-{width}`, plus arbitrary SVG marker, rendering, paint server, and
filter color properties, project into the same declaration model.
Common Tailwind transform utilities such as `translate-*`, `scale-*`,
`rotate-*`, `skew-*`, `origin-*`, `perspective-*`, `backface-*`, and
`transform-*`, plus arbitrary `transform-box` and CSS Motion Path properties,
project into individual transform properties or the transform function
pipeline.
Common Tailwind filter and backdrop-filter utilities such as `blur-*`,
`brightness-*`, `contrast-*`, `drop-shadow-*`, `grayscale`, `hue-rotate-*`,
`invert-*`, `saturate-*`, `sepia-*`, and `backdrop-*` project into composable
filter tokens.
Common Tailwind Grid utilities such as `grid-cols-*`, `grid-rows-*`,
`auto-cols-*`, `auto-rows-*`, `grid-flow-*`, `col-*`, and `row-*` project into
the same declaration model.
Common Tailwind Flexbox item and box-alignment utilities such as `flex-*`,
`basis-*`, `grow-*`, `shrink-*`, `order-*`, `content-*`, `self-*`,
`justify-items-*`, `justify-self-*`, `place-*`, and arbitrary `reading-*`
properties project into the same declaration model.
Common Tailwind sizing and child-spacing utilities such as `size-*`,
`space-x-*`, `space-y-*`, `space-x-reverse`, and `space-y-reverse` project
into the same declaration model.
Common Tailwind typography and text utilities such as `font-*`, `italic`,
`not-italic`, `antialiased`, `subpixel-antialiased`, `tracking-*`,
`font-stretch-*`, `font-features-*`, arbitrary `font`, `font-width`,
`font-size-adjust`, `font-palette`, and `font-language-override` properties,
font variant numeric utilities, arbitrary rhythmic sizing and line-grid
properties such as `line-height-step`, `block-step*`, `line-grid`,
`line-snap`, and `box-snap`, arbitrary MathML math properties such as
`math-depth`, `math-shift`, and `math-style`, `tab-*`, text transform utilities, text decoration
utilities, `underline-offset-*`, arbitrary `text-decoration-skip*` and
`text-underline-position` properties, arbitrary `text-emphasis-*` properties,
arbitrary `text-size-adjust`, `text-combine-upright`, `text-align-last`,
`text-align-all`, `text-group-align`, `text-justify`, baseline and
initial-letter properties, `text-wrap-*`, arbitrary `wrap-*`,
`line-padding`, `text-spacing`, `text-spacing-trim`, `text-autospace`,
`word-space-transform`,
`text-box*`, `white-space-collapse`, `white-space-trim`,
`hanging-punctuation`, hyphenation limit properties, and line-clamp
longhand properties,
arbitrary CSS Speech properties such as `speak`, `speak-as`, `pause`, `rest`,
`cue`, and `voice-*`,
arbitrary `ruby-*`
properties, `truncate`, `text-ellipsis`, `text-clip`, `indent-*`, `line-clamp-*`,
`text-shadow-*`, `text-wrap`, `text-nowrap`,
`text-balance`, `text-pretty`, `whitespace-*`, `wrap-*`, word-break utilities,
`hyphens-*`, generated-content utilities such as `content-[...]`,
`content-(...)`, and `content-none`, and arbitrary `counter-*`, `quotes`, and
`string-set` properties project into the same declaration model.
CSS writing-mode arbitrary property utilities, CSS Anchor Positioning arbitrary
properties such as `anchor-name` and `position-area`, and `ltr:`/`rtl:`
variants are stored in the same declaration model.
Arbitrary `all` properties project into cascade reset metadata.
Common Tailwind background, object, list, columns, and fragmentation utilities
such as `bg-*`, `object-*`, `list-*`, `list-image-*`, `columns-*`,
`break-before-*`, `break-after-*`, and `break-inside-*`, plus arbitrary CSS
background shorthand, image, and shape properties such as `background`,
`image-rendering`, `shape-outside`, `shape-inside`, and `shape-padding`,
and arbitrary paged media and list
properties such as `page`, `orphans`, `widows`, and `marker-side`, plus bookmark
and footnote properties, plus paged media `size`, `page-orientation`, `bleed`,
and `marks`, project into the same declaration model.
Tailwind border radius utilities such as `rounded-*`, `rounded-t-*`,
`rounded-r-*`, `rounded-b-*`, `rounded-l-*`, `rounded-tl-*`,
`rounded-tr-*`, `rounded-br-*`, `rounded-bl-*`, `rounded-s-*`,
`rounded-e-*`, `rounded-ss-*`, `rounded-se-*`, `rounded-ee-*`, and
`rounded-es-*` project into physical or logical corner radius tokens.
Tailwind border width, color, and divide utilities such as `border-*`, `border-x-*`,
`border-y-*`, `border-t-*`, `border-r-*`, `border-b-*`, `border-l-*`,
`border-s-*`, `border-e-*`, `border-bs-*`, `border-be-*`, `divide-x-*`,
`divide-y-*`, `divide-*-reverse`, `divide-{color}`, and `divide-{style}`
project into physical, logical, or native child-divider tokens.
Common Tailwind motion, interaction, and scroll utilities such as
`transition-*`, `duration-*`, `delay-*`, `ease-*`, `animate-*`,
arbitrary animation and scroll-driven animation properties such as
`animation-composition`, `animation-timeline`, `scroll-timeline`, and
`view-timeline`, top-layer `overlay` metadata, arbitrary CSS View Transitions
properties, `will-change-*`, `appearance-*`,
`accent-*`, `caret-*`, arbitrary `caret`, `caret-animation`, and
`caret-shape` properties, `resize-*`,
`scheme-*`, `forced-color-adjust-*`, arbitrary `print-color-adjust`,
`field-sizing-*`, `scroll-*`, `snap-*`,
`scrollbar-*`, `scrollbar-gutter-*`, `scrollbar-thumb-*`,
`scrollbar-track-*`, `overscroll-*`, arbitrary logical overflow, overflow clip
margin, scroll anchoring, scroll initial target, scroll target groups, scroll
marker groups, logical overscroll, CSS UI directional navigation, CSS Spatial
Navigation, CSS UI interactivity, and `touch-*` properties project into the
same declaration model.
Tailwind logical direction utilities such as `start-*`, `end-*`, `ms-*`,
`me-*`, `mbs-*`, `mbe-*`, `mis-*`, `mie-*`, `ps-*`, `pe-*`, `pbs-*`,
`pbe-*`, `pis-*`, `pie-*`, `scroll-ms-*`, `scroll-me-*`, `scroll-mbs-*`,
`scroll-mbe-*`, `scroll-ps-*`, `scroll-pe-*`, `scroll-pbs-*`, and
`scroll-pbe-*` project into logical portable style tokens.
