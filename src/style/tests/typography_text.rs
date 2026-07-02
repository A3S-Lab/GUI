use super::support::*;

#[test]
fn parses_css_ruby_layout_properties() {
    let web = WebProps::new()
        .style("display", "ruby")
        .style("rubyAlign", "space-around")
        .style("rubyPosition", "under")
        .style("rubyMerge", "collapse")
        .style("rubyOverhang", "auto");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.display, Some(DisplayMode::Ruby));
    assert_eq!(style.ruby_align.as_deref(), Some("space-around"));
    assert_eq!(style.ruby_position.as_deref(), Some("under"));
    assert_eq!(style.ruby_merge.as_deref(), Some("collapse"));
    assert_eq!(style.ruby_overhang.as_deref(), Some("auto"));
    assert!(!style.unsupported.contains_key("ruby-align"));
    assert!(!style.unsupported.contains_key("ruby-position"));
    assert!(!style.unsupported.contains_key("ruby-merge"));
    assert!(!style.unsupported.contains_key("ruby-overhang"));
}

#[test]
fn parses_css_typography_text_properties_into_portable_tokens() {
    let web = WebProps::new()
        .style("font", "italic small-caps 16px/1.5 ui-serif")
        .style("fontFamily", "ui-monospace, monospace")
        .style("fontStyle", "italic")
        .style("fontStretch", "semi-condensed")
        .style("fontWidth", "condensed")
        .style("fontPalette", "dark")
        .style("fontLanguageOverride", "\"TRK\"")
        .style("fontKerning", "normal")
        .style("fontOpticalSizing", "auto")
        .style("WebkitFontSmoothing", "antialiased")
        .style("MozOsxFontSmoothing", "grayscale")
        .style("fontFeatureSettings", "\"kern\" 1, \"liga\" 0")
        .style("fontVariationSettings", "\"wght\" 650")
        .style("fontSizeAdjust", "0.5")
        .style("fontVariant", "small-caps tabular-nums")
        .style("fontVariantAlternates", "historical-forms")
        .style("fontVariantCaps", "small-caps")
        .style("fontVariantEastAsian", "jis78")
        .style("fontVariantEmoji", "emoji")
        .style("fontVariantLigatures", "common-ligatures")
        .style("fontVariantNumeric", "tabular-nums slashed-zero")
        .style("fontVariantPosition", "sub")
        .style("fontSynthesis", "weight style")
        .style("fontSynthesisWeight", "none")
        .style("fontSynthesisStyle", "auto")
        .style("fontSynthesisSmallCaps", "none")
        .style("fontSynthesisPosition", "auto")
        .style("lineHeightStep", "4px")
        .style("blockStep", "1lh center up")
        .style("blockStepSize", "1lh")
        .style("blockStepInsert", "margin-box")
        .style("blockStepAlign", "center")
        .style("blockStepRound", "up")
        .style("lineGrid", "create")
        .style("lineSnap", "baseline")
        .style("boxSnap", "block-start")
        .style("mathDepth", "add(1)")
        .style("mathShift", "compact")
        .style("mathStyle", "compact")
        .style("dominantBaseline", "central")
        .style("baselineSource", "first")
        .style("alignmentBaseline", "text-before-edge")
        .style("baselineShift", "super")
        .style("lineFitEdge", "leading")
        .style("inlineSizing", "stretch")
        .style("initialLetter", "3 2")
        .style("initialLetterAlign", "border-box")
        .style("initialLetterWrap", "first")
        .style("letterSpacing", "0.025em")
        .style("wordSpacing", "0.125em")
        .style("tabSize", "4")
        .style("textSizeAdjust", "100%")
        .style("WebkitTextSizeAdjust", "none")
        .style("MozTextSizeAdjust", "auto")
        .style("msTextSizeAdjust", "80%")
        .style("textAlignAll", "justify")
        .style("textAlignLast", "center")
        .style("textGroupAlign", "end")
        .style("textJustify", "inter-character")
        .style("wordSpaceTransform", "space")
        .style("direction", "rtl")
        .style("unicodeBidi", "isolate-override")
        .style("-webkitWritingMode", "vertical-lr")
        .style("textOrientation", "upright")
        .style("textCombineUpright", "digits 2")
        .style("textTransform", "uppercase")
        .style("textIndent", "2rem")
        .style("textWrap", "balance")
        .style("textWrapMode", "nowrap")
        .style("textWrapStyle", "pretty")
        .style("wrapBefore", "avoid")
        .style("wrapAfter", "avoid")
        .style("wrapInside", "avoid")
        .style("linePadding", "0.5em")
        .style("textSpacing", "trim-start allow-end")
        .style("textSpacingTrim", "trim-start")
        .style("textAutospace", "ideograph-alpha ideograph-numeric")
        .style("textBox", "trim-both cap alphabetic")
        .style("textBoxTrim", "trim-both")
        .style("textBoxEdge", "cap alphabetic")
        .style("hangingPunctuation", "first allow-end")
        .style("lineClamp", "3")
        .style("blockEllipsis", "\"...\"")
        .style("continue", "discard")
        .style("maxLines", "4")
        .style("display", "-webkit-box")
        .style("-webkitBoxOrient", "vertical")
        .style("textDecorationLine", "underline")
        .style("textDecorationColor", "#663399")
        .style("textDecorationStyle", "wavy")
        .style("textDecorationThickness", "from-font")
        .style("textDecorationSkip", "objects spaces")
        .style("textDecorationSkipBox", "all")
        .style("textDecorationSkipInk", "none")
        .style("textDecorationSkipInset", "0.15em")
        .style("textDecorationSkipSelf", "skip-all")
        .style("textDecorationSkipSpaces", "start end")
        .style("textUnderlineOffset", "4px")
        .style("textUnderlinePosition", "under left")
        .style("textEmphasis", "filled sesame #663399")
        .style("textEmphasisSkip", "spaces")
        .style("WebkitTextEmphasisPosition", "under right")
        .style("textShadow", "0 1px 2px rgb(0 0 0 / 0.3)")
        .style("textOverflow", "ellipsis")
        .style("lineBreak", "strict")
        .style("whiteSpace", "nowrap")
        .style("whiteSpaceCollapse", "preserve-breaks")
        .style("whiteSpaceTrim", "discard-before")
        .style("wordBreak", "keep-all")
        .style("overflowWrap", "anywhere")
        .style("hyphens", "auto")
        .style("hyphenateCharacter", "\"=\"")
        .style("hyphenateLimitZone", "8%")
        .style("hyphenateLimitChars", "6 3 2")
        .style("hyphenateLimitLines", "2")
        .style("hyphenateLimitLast", "always");

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.font.as_deref(),
        Some("italic small-caps 16px/1.5 ui-serif")
    );
    assert_eq!(
        style.font_family.as_deref(),
        Some("ui-monospace, monospace")
    );
    assert_eq!(style.font_style, Some(FontStyle::Italic));
    assert_eq!(style.font_stretch.as_deref(), Some("semi-condensed"));
    assert_eq!(style.font_width.as_deref(), Some("condensed"));
    assert_eq!(style.font_palette.as_deref(), Some("dark"));
    assert_eq!(style.font_language_override.as_deref(), Some("\"TRK\""));
    assert_eq!(style.font_kerning.as_deref(), Some("normal"));
    assert_eq!(style.font_optical_sizing.as_deref(), Some("auto"));
    assert_eq!(style.webkit_font_smoothing.as_deref(), Some("antialiased"));
    assert_eq!(style.moz_osx_font_smoothing.as_deref(), Some("grayscale"));
    assert_eq!(
        style.font_feature_settings.as_deref(),
        Some("\"kern\" 1, \"liga\" 0")
    );
    assert_eq!(
        style.font_variation_settings.as_deref(),
        Some("\"wght\" 650")
    );
    assert_eq!(style.font_size_adjust.as_deref(), Some("0.5"));
    assert_eq!(
        style.font_variant.as_deref(),
        Some("small-caps tabular-nums")
    );
    assert_eq!(
        style.font_variant_alternates.as_deref(),
        Some("historical-forms")
    );
    assert_eq!(style.font_variant_caps.as_deref(), Some("small-caps"));
    assert_eq!(style.font_variant_east_asian.as_deref(), Some("jis78"));
    assert_eq!(style.font_variant_emoji.as_deref(), Some("emoji"));
    assert_eq!(
        style.font_variant_ligatures.as_deref(),
        Some("common-ligatures")
    );
    assert_eq!(
        style.font_variant_numeric.as_deref(),
        Some("tabular-nums slashed-zero")
    );
    assert_eq!(style.font_variant_position.as_deref(), Some("sub"));
    assert_eq!(style.font_synthesis.as_deref(), Some("weight style"));
    assert_eq!(style.font_synthesis_weight.as_deref(), Some("none"));
    assert_eq!(style.font_synthesis_style.as_deref(), Some("auto"));
    assert_eq!(style.font_synthesis_small_caps.as_deref(), Some("none"));
    assert_eq!(style.font_synthesis_position.as_deref(), Some("auto"));
    assert_eq!(style.line_height_step.as_deref(), Some("4px"));
    assert_eq!(style.block_step.as_deref(), Some("1lh center up"));
    assert_eq!(style.block_step_size.as_deref(), Some("1lh"));
    assert_eq!(style.block_step_insert.as_deref(), Some("margin-box"));
    assert_eq!(style.block_step_align.as_deref(), Some("center"));
    assert_eq!(style.block_step_round.as_deref(), Some("up"));
    assert_eq!(style.line_grid.as_deref(), Some("create"));
    assert_eq!(style.line_snap.as_deref(), Some("baseline"));
    assert_eq!(style.box_snap.as_deref(), Some("block-start"));
    assert_eq!(style.math_depth.as_deref(), Some("add(1)"));
    assert_eq!(style.math_shift.as_deref(), Some("compact"));
    assert_eq!(style.math_style.as_deref(), Some("compact"));
    assert_eq!(style.dominant_baseline.as_deref(), Some("central"));
    assert_eq!(style.baseline_source.as_deref(), Some("first"));
    assert_eq!(
        style.alignment_baseline.as_deref(),
        Some("text-before-edge")
    );
    assert_eq!(style.baseline_shift.as_deref(), Some("super"));
    assert_eq!(style.line_fit_edge.as_deref(), Some("leading"));
    assert_eq!(style.inline_sizing.as_deref(), Some("stretch"));
    assert_eq!(style.initial_letter.as_deref(), Some("3 2"));
    assert_eq!(style.initial_letter_align.as_deref(), Some("border-box"));
    assert_eq!(style.initial_letter_wrap.as_deref(), Some("first"));
    assert_eq!(style.letter_spacing, Some(StyleLength::Points(0.4)));
    assert_eq!(style.word_spacing, Some(StyleLength::Points(2.0)));
    assert_eq!(style.tab_size.as_deref(), Some("4"));
    assert_eq!(style.text_size_adjust.as_deref(), Some("100%"));
    assert_eq!(style.webkit_text_size_adjust.as_deref(), Some("none"));
    assert_eq!(style.moz_text_size_adjust.as_deref(), Some("auto"));
    assert_eq!(style.ms_text_size_adjust.as_deref(), Some("80%"));
    assert_eq!(style.text_align_all.as_deref(), Some("justify"));
    assert_eq!(style.text_align_last.as_deref(), Some("center"));
    assert_eq!(style.text_group_align.as_deref(), Some("end"));
    assert_eq!(style.text_justify.as_deref(), Some("inter-character"));
    assert_eq!(style.word_space_transform.as_deref(), Some("space"));
    assert_eq!(style.direction, Some(TextDirection::Rtl));
    assert_eq!(style.unicode_bidi, Some(UnicodeBidi::IsolateOverride));
    assert_eq!(style.writing_mode, Some(WritingMode::VerticalLr));
    assert_eq!(style.text_orientation, Some(TextOrientation::Upright));
    assert_eq!(style.text_combine_upright.as_deref(), Some("digits 2"));
    assert_eq!(style.text_transform, Some(TextTransform::Uppercase));
    assert_eq!(style.text_indent, Some(StyleLength::Points(32.0)));
    assert_eq!(style.text_wrap, Some(TextWrapMode::NoWrap));
    assert_eq!(style.text_wrap_mode.as_deref(), Some("nowrap"));
    assert_eq!(style.text_wrap_style.as_deref(), Some("pretty"));
    assert_eq!(style.wrap_before.as_deref(), Some("avoid"));
    assert_eq!(style.wrap_after.as_deref(), Some("avoid"));
    assert_eq!(style.wrap_inside.as_deref(), Some("avoid"));
    assert_eq!(style.line_padding.as_deref(), Some("0.5em"));
    assert_eq!(style.text_spacing.as_deref(), Some("trim-start allow-end"));
    assert_eq!(style.text_spacing_trim.as_deref(), Some("trim-start"));
    assert_eq!(
        style.text_autospace.as_deref(),
        Some("ideograph-alpha ideograph-numeric")
    );
    assert_eq!(style.text_box.as_deref(), Some("trim-both cap alphabetic"));
    assert_eq!(style.text_box_trim.as_deref(), Some("trim-both"));
    assert_eq!(style.text_box_edge.as_deref(), Some("cap alphabetic"));
    assert_eq!(
        style.hanging_punctuation.as_deref(),
        Some("first allow-end")
    );
    assert_eq!(style.line_clamp.as_deref(), Some("3"));
    assert_eq!(style.block_ellipsis.as_deref(), Some("\"...\""));
    assert_eq!(style.continue_mode.as_deref(), Some("discard"));
    assert_eq!(style.max_lines.as_deref(), Some("4"));
    assert_eq!(style.display, Some(DisplayMode::WebkitBox));
    assert_eq!(style.box_orient.as_deref(), Some("vertical"));
    assert_eq!(style.text_decoration_line.as_deref(), Some("underline"));
    assert_eq!(
        style.text_decoration_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 255,
        })
    );
    assert_eq!(style.text_decoration_style, Some(TextDecorationStyle::Wavy));
    assert_eq!(
        style.text_decoration_thickness,
        Some(StyleLength::Css("from-font".to_string()))
    );
    assert_eq!(
        style.text_decoration_skip.as_deref(),
        Some("objects spaces")
    );
    assert_eq!(style.text_decoration_skip_box.as_deref(), Some("all"));
    assert_eq!(style.text_decoration_skip_ink.as_deref(), Some("none"));
    assert_eq!(style.text_decoration_skip_inset.as_deref(), Some("0.15em"));
    assert_eq!(style.text_decoration_skip_self.as_deref(), Some("skip-all"));
    assert_eq!(
        style.text_decoration_skip_spaces.as_deref(),
        Some("start end")
    );
    assert_eq!(style.text_underline_offset, Some(StyleLength::Points(4.0)));
    assert_eq!(style.text_underline_position.as_deref(), Some("under left"));
    assert_eq!(style.text_emphasis_style.as_deref(), Some("filled sesame"));
    assert_eq!(
        style.text_emphasis_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 255,
        })
    );
    assert_eq!(style.text_emphasis_position.as_deref(), Some("under right"));
    assert_eq!(style.text_emphasis_skip.as_deref(), Some("spaces"));
    assert_eq!(
        style.text_shadow.as_deref(),
        Some("0 1px 2px rgb(0 0 0 / 0.3)")
    );
    assert_eq!(style.text_overflow, Some(TextOverflow::Ellipsis));
    assert_eq!(style.line_break.as_deref(), Some("strict"));
    assert_eq!(style.white_space, Some(WhiteSpaceMode::NoWrap));
    assert_eq!(
        style.white_space_collapse.as_deref(),
        Some("preserve-breaks")
    );
    assert_eq!(style.white_space_trim.as_deref(), Some("discard-before"));
    assert_eq!(style.word_break, Some(WordBreakMode::KeepAll));
    assert_eq!(style.overflow_wrap, Some(OverflowWrapMode::Anywhere));
    assert_eq!(style.hyphens, Some(HyphensMode::Auto));
    assert_eq!(style.hyphenate_character.as_deref(), Some("\"=\""));
    assert_eq!(style.hyphenate_limit_zone.as_deref(), Some("8%"));
    assert_eq!(style.hyphenate_limit_chars.as_deref(), Some("6 3 2"));
    assert_eq!(style.hyphenate_limit_lines.as_deref(), Some("2"));
    assert_eq!(style.hyphenate_limit_last.as_deref(), Some("always"));
    assert!(!style.unsupported.contains_key("text-decoration-line"));
    assert!(!style.unsupported.contains_key("text-decoration-skip"));
    assert!(!style.unsupported.contains_key("text-decoration-skip-box"));
    assert!(!style.unsupported.contains_key("text-decoration-skip-ink"));
    assert!(!style.unsupported.contains_key("text-decoration-skip-inset"));
    assert!(!style.unsupported.contains_key("text-decoration-skip-self"));
    assert!(!style
        .unsupported
        .contains_key("text-decoration-skip-spaces"));
    assert!(!style.unsupported.contains_key("text-underline-position"));
    assert!(!style.unsupported.contains_key("text-emphasis"));
    assert!(!style.unsupported.contains_key("text-emphasis-skip"));
    assert!(!style
        .unsupported
        .contains_key("webkit-text-emphasis-position"));
    assert!(!style.unsupported.contains_key("font"));
    assert!(!style.unsupported.contains_key("font-feature-settings"));
    assert!(!style.unsupported.contains_key("font-size-adjust"));
    assert!(!style.unsupported.contains_key("font-width"));
    assert!(!style.unsupported.contains_key("font-palette"));
    assert!(!style.unsupported.contains_key("font-language-override"));
    assert!(!style.unsupported.contains_key("font-variant-numeric"));
    assert!(!style.unsupported.contains_key("line-height-step"));
    assert!(!style.unsupported.contains_key("block-step"));
    assert!(!style.unsupported.contains_key("block-step-size"));
    assert!(!style.unsupported.contains_key("block-step-insert"));
    assert!(!style.unsupported.contains_key("block-step-align"));
    assert!(!style.unsupported.contains_key("block-step-round"));
    assert!(!style.unsupported.contains_key("line-grid"));
    assert!(!style.unsupported.contains_key("line-snap"));
    assert!(!style.unsupported.contains_key("box-snap"));
    assert!(!style.unsupported.contains_key("math-depth"));
    assert!(!style.unsupported.contains_key("math-shift"));
    assert!(!style.unsupported.contains_key("math-style"));
    assert!(!style.unsupported.contains_key("text-size-adjust"));
    assert!(!style.unsupported.contains_key("dominant-baseline"));
    assert!(!style.unsupported.contains_key("baseline-source"));
    assert!(!style.unsupported.contains_key("alignment-baseline"));
    assert!(!style.unsupported.contains_key("baseline-shift"));
    assert!(!style.unsupported.contains_key("line-fit-edge"));
    assert!(!style.unsupported.contains_key("inline-sizing"));
    assert!(!style.unsupported.contains_key("initial-letter"));
    assert!(!style.unsupported.contains_key("initial-letter-align"));
    assert!(!style.unsupported.contains_key("initial-letter-wrap"));
    assert!(!style.unsupported.contains_key("block-ellipsis"));
    assert!(!style.unsupported.contains_key("continue"));
    assert!(!style.unsupported.contains_key("max-lines"));
    assert!(!style.unsupported.contains_key("webkit-text-size-adjust"));
    assert!(!style.unsupported.contains_key("moz-text-size-adjust"));
    assert!(!style.unsupported.contains_key("ms-text-size-adjust"));
    assert!(!style.unsupported.contains_key("text-align-all"));
    assert!(!style.unsupported.contains_key("text-align-last"));
    assert!(!style.unsupported.contains_key("text-group-align"));
    assert!(!style.unsupported.contains_key("text-justify"));
    assert!(!style.unsupported.contains_key("word-space-transform"));
    assert!(!style.unsupported.contains_key("white-space"));
    assert!(!style.unsupported.contains_key("text-shadow"));
    assert!(!style.unsupported.contains_key("webkit-font-smoothing"));
    assert!(!style.unsupported.contains_key("moz-osx-font-smoothing"));
    assert!(!style.unsupported.contains_key("-webkit-writing-mode"));
    assert!(!style.unsupported.contains_key("text-combine-upright"));
    assert!(!style.unsupported.contains_key("hanging-punctuation"));
    assert!(!style.unsupported.contains_key("text-wrap"));
    assert!(!style.unsupported.contains_key("text-wrap-mode"));
    assert!(!style.unsupported.contains_key("text-wrap-style"));
    assert!(!style.unsupported.contains_key("wrap-before"));
    assert!(!style.unsupported.contains_key("wrap-after"));
    assert!(!style.unsupported.contains_key("wrap-inside"));
    assert!(!style.unsupported.contains_key("line-padding"));
    assert!(!style.unsupported.contains_key("text-spacing"));
    assert!(!style.unsupported.contains_key("text-spacing-trim"));
    assert!(!style.unsupported.contains_key("text-autospace"));
    assert!(!style.unsupported.contains_key("text-box"));
    assert!(!style.unsupported.contains_key("text-box-trim"));
    assert!(!style.unsupported.contains_key("text-box-edge"));
    assert!(!style.unsupported.contains_key("white-space-collapse"));
    assert!(!style.unsupported.contains_key("white-space-trim"));
    assert!(!style.unsupported.contains_key("hyphenate-character"));
    assert!(!style.unsupported.contains_key("hyphenate-limit-zone"));
    assert!(!style.unsupported.contains_key("hyphenate-limit-chars"));
    assert!(!style.unsupported.contains_key("hyphenate-limit-lines"));
    assert!(!style.unsupported.contains_key("hyphenate-limit-last"));
    assert!(!style.unsupported.contains_key("-webkit-line-clamp"));
}

#[test]
fn parses_prefixed_text_combine_aliases() {
    let webkit = PortableStyle::from_web(&WebProps::new().style("WebkitTextCombine", "digits 2"));
    assert_eq!(webkit.text_combine_upright.as_deref(), Some("digits 2"));
    assert!(!webkit.unsupported.contains_key("webkit-text-combine"));

    let ms = PortableStyle::from_web(&WebProps::new().style("-msTextCombineHorizontal", "all"));
    assert_eq!(ms.text_combine_upright.as_deref(), Some("all"));
    assert!(!ms.unsupported.contains_key("-ms-text-combine-horizontal"));
}

#[test]
fn parses_prefixed_font_language_override_alias() {
    let style =
        PortableStyle::from_web(&WebProps::new().style("MozFontLanguageOverride", "\"SRB\""));

    assert_eq!(style.font_language_override.as_deref(), Some("\"SRB\""));
    assert!(!style.unsupported.contains_key("moz-font-language-override"));
}

#[test]
fn parses_prefixed_text_autospace_alias() {
    let style =
        PortableStyle::from_web(&WebProps::new().style("MsTextAutospace", "ideograph-alpha"));

    assert_eq!(style.text_autospace.as_deref(), Some("ideograph-alpha"));
    assert!(!style.unsupported.contains_key("ms-text-autospace"));
}
