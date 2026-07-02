use super::support::*;

#[test]
fn parses_tailwind_arbitrary_ruby_layout_properties() {
    let web = WebProps::new().class_name(
        "[display:ruby] [ruby-align:space-around] [ruby-position:over] \
             [ruby-merge:collapse] [ruby-overhang:auto] \
             motion-safe:[display:ruby-text] hover:[ruby-position:under] \
             focus:[ruby-align:center] active:[ruby-merge:separate] \
             rtl:[ruby-overhang:none]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.display, Some(DisplayMode::Ruby));
    assert_eq!(style.ruby_align.as_deref(), Some("space-around"));
    assert_eq!(style.ruby_position.as_deref(), Some("over"));
    assert_eq!(style.ruby_merge.as_deref(), Some("collapse"));
    assert_eq!(style.ruby_overhang.as_deref(), Some("auto"));
    assert_eq!(
        style
            .variant_declarations
            .get("motion-safe")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("ruby-text")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("ruby-position"))
            .map(String::as_str),
        Some("under")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("ruby-align"))
            .map(String::as_str),
        Some("center")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("ruby-merge"))
            .map(String::as_str),
        Some("separate")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("rtl")
            .and_then(|styles| styles.get("ruby-overhang"))
            .map(String::as_str),
        Some("none")
    );
}

#[test]
fn parses_tailwind_typography_text_utilities() {
    let web = WebProps::new().class_name(
        "font-mono italic antialiased tracking-wide uppercase underline decoration-wavy \
             decoration-[#663399]/50 decoration-2 underline-offset-4 truncate \
             whitespace-pre-wrap break-all wrap-anywhere hyphens-auto -indent-[2px] text-balance \
             ordinal slashed-zero tabular-nums diagonal-fractions \
             font-stretch-condensed font-features-[\"kern\"_1] tab-4 text-shadow-sm \
             [font:italic_1rem/1.5_serif] [font-size-adjust:0.5] \
             [font-width:condensed] [font-palette:dark] [font-language-override:\"TRK\"] \
             [dominant-baseline:central] [baseline-source:first] \
             [alignment-baseline:text-before-edge] [baseline-shift:super] \
             [line-fit-edge:leading] [inline-sizing:stretch] \
             [line-height-step:4px] [block-step:1lh_center_up] \
             [block-step-size:1lh] [block-step-insert:margin-box] \
             [block-step-align:center] [block-step-round:up] \
             [line-grid:create] [line-snap:baseline] [box-snap:block-start] \
             [math-depth:add(1)] [math-shift:compact] [math-style:compact] \
             [initial-letter:3_2] [initial-letter-align:border-box] \
             [initial-letter-wrap:first] \
             [text-size-adjust:100%] \
             [-webkit-text-size-adjust:none] [-moz-text-size-adjust:auto] \
             [-ms-text-size-adjust:80%] \
             [text-align-all:justify] [text-align-last:center] \
             [text-group-align:end] [text-justify:inter-word] \
             [word-space-transform:space] \
             [direction:rtl] [unicode-bidi:isolate] [writing-mode:vertical-rl] \
             [text-orientation:upright] [text-combine-upright:all] \
             [text-decoration-skip:objects_spaces] [text-decoration-skip-box:all] \
             [text-decoration-skip-ink:none] \
             [text-decoration-skip-inset:0.15em] [text-decoration-skip-self:skip-all] \
             [text-decoration-skip-spaces:start_end] \
             [text-underline-position:under_left] [text-emphasis-style:open_dot] \
             [text-emphasis-color:#663399] [text-emphasis-position:under_right] \
             [text-emphasis-skip:spaces] \
             line-clamp-3 md:tracking-[0.2em] hover:decoration-[3px] \
             focus:text-pretty lg:line-clamp-none ltr:[direction:ltr] \
             rtl:[unicode-bidi:plaintext] md:[writing-mode:horizontal-tb] \
             hover:[text-orientation:sideways] md:font-stretch-[87.5%] \
             hover:[font-width:expanded] focus:[math-depth:0] \
             active:[math-style:normal] before:[math-shift:normal] \
             hover:[font-size-adjust:0.6] focus:[text-size-adjust:none] \
             hover:[font-palette:light] focus:[font-language-override:normal] \
             hover:[dominant-baseline:hanging] focus:[baseline-shift:sub] \
             active:[line-height-step:8px] hover:[block-step:none] \
             focus:[block-step-size:2lh] before:[line-grid:match-parent] \
             after:[line-snap:contain] visited:[box-snap:none] \
             active:[initial-letter:2] before:[initial-letter-wrap:all] \
             after:[block-ellipsis:auto] visited:[max-lines:none] \
             md:[text-align-all:center] hover:[text-group-align:start] \
             focus:[word-space-transform:ideographs] \
             hover:[text-align-last:right] focus:[text-justify:inter-character] \
             visited:[hanging-punctuation:last_force-end] \
             active:[text-combine-upright:digits_2] \
             [white-space-collapse:preserve-breaks] [text-wrap-mode:nowrap] \
             [text-wrap-style:stable] [wrap-before:avoid] [wrap-after:avoid] \
             [wrap-inside:avoid] [line-padding:0.5em] \
             [text-spacing:trim-start_allow-end] [text-spacing-trim:space-all] \
             [text-autospace:ideograph-alpha] [text-box:trim-both_cap_alphabetic] \
             [text-box-trim:trim-start] [text-box-edge:cap_alphabetic] \
             [block-ellipsis:\"...\"] [continue:discard] [max-lines:4] \
             [white-space-trim:discard-before] [hyphenate-character:\"=\"] \
             [hyphenate-limit-zone:8%] [hyphenate-limit-chars:6_3_2] \
             [hyphenate-limit-lines:2] [hyphenate-limit-last:always] \
             hover:[text-decoration-skip:objects] focus:[text-decoration-skip-box:none] \
             active:[text-decoration-skip-inset:auto] visited:[text-decoration-skip-self:skip] \
             before:[text-decoration-skip-spaces:none] after:[text-emphasis-skip:punctuation] \
             hover:[text-decoration-skip-ink:all] focus:[text-underline-position:left] \
             hover:[text-emphasis-style:filled_sesame] focus:[text-emphasis-color:#663399] \
             active:[text-emphasis-position:over_left] \
             hover:font-features-(--font-features) focus:text-shadow-[0_1px_2px_rgb(0_0_0_/_0.3)] \
             lg:normal-nums content-none before:content-['Hello_World'] \
             after:content-(--suffix-content) hover:content-['Hello\\_World'] \
             hover:[white-space-collapse:preserve-spaces] focus:[text-wrap-style:pretty] \
             active:[text-spacing-trim:trim-auto] before:[wrap-before:auto] \
             after:[hyphenate-limit-lines:no-limit] before:[text-box-trim:none] \
             after:[text-box-edge:text] visited:[text-autospace:no-autospace] \
             [hanging-punctuation:first_allow-end] \
             hover:wrap-break-word focus:wrap-[normal] active:wrap-(--overflow-wrap) \
             md:subpixel-antialiased",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.font_family.as_deref(),
        Some("ui-monospace, monospace")
    );
    assert_eq!(style.font_style, Some(FontStyle::Italic));
    assert_eq!(style.letter_spacing, Some(StyleLength::Points(0.4)));
    assert_eq!(
        style.declarations.get("letter-spacing").map(String::as_str),
        Some("0.025em")
    );
    assert_eq!(style.font_stretch.as_deref(), Some("condensed"));
    assert_eq!(style.webkit_font_smoothing.as_deref(), Some("antialiased"));
    assert_eq!(style.moz_osx_font_smoothing.as_deref(), Some("grayscale"));
    assert_eq!(style.font.as_deref(), Some("italic 1rem/1.5 serif"));
    assert_eq!(style.font_feature_settings.as_deref(), Some("\"kern\" 1"));
    assert_eq!(style.font_size_adjust.as_deref(), Some("0.5"));
    assert_eq!(style.font_width.as_deref(), Some("condensed"));
    assert_eq!(style.font_palette.as_deref(), Some("dark"));
    assert_eq!(style.font_language_override.as_deref(), Some("\"TRK\""));
    assert_eq!(style.dominant_baseline.as_deref(), Some("central"));
    assert_eq!(style.baseline_source.as_deref(), Some("first"));
    assert_eq!(
        style.alignment_baseline.as_deref(),
        Some("text-before-edge")
    );
    assert_eq!(style.baseline_shift.as_deref(), Some("super"));
    assert_eq!(style.line_fit_edge.as_deref(), Some("leading"));
    assert_eq!(style.inline_sizing.as_deref(), Some("stretch"));
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
    assert_eq!(style.initial_letter.as_deref(), Some("3 2"));
    assert_eq!(style.initial_letter_align.as_deref(), Some("border-box"));
    assert_eq!(style.initial_letter_wrap.as_deref(), Some("first"));
    assert_eq!(
        style.font_variant_numeric.as_deref(),
        Some("ordinal slashed-zero tabular-nums diagonal-fractions")
    );
    assert_eq!(style.tab_size.as_deref(), Some("4"));
    assert_eq!(style.text_size_adjust.as_deref(), Some("100%"));
    assert_eq!(style.webkit_text_size_adjust.as_deref(), Some("none"));
    assert_eq!(style.moz_text_size_adjust.as_deref(), Some("auto"));
    assert_eq!(style.ms_text_size_adjust.as_deref(), Some("80%"));
    assert_eq!(style.text_align_all.as_deref(), Some("justify"));
    assert_eq!(style.text_align_last.as_deref(), Some("center"));
    assert_eq!(style.text_group_align.as_deref(), Some("end"));
    assert_eq!(style.text_justify.as_deref(), Some("inter-word"));
    assert_eq!(style.word_space_transform.as_deref(), Some("space"));
    assert_eq!(style.text_shadow.as_deref(), Some("var(--text-shadow-sm)"));
    assert_eq!(style.text_transform, Some(TextTransform::Uppercase));
    assert_eq!(style.direction, Some(TextDirection::Rtl));
    assert_eq!(style.unicode_bidi, Some(UnicodeBidi::Isolate));
    assert_eq!(style.writing_mode, Some(WritingMode::VerticalRl));
    assert_eq!(style.text_orientation, Some(TextOrientation::Upright));
    assert_eq!(style.text_combine_upright.as_deref(), Some("all"));
    assert_eq!(style.text_indent, Some(StyleLength::Points(-2.0)));
    assert_eq!(style.text_wrap, Some(TextWrapMode::NoWrap));
    assert_eq!(style.text_wrap_mode.as_deref(), Some("nowrap"));
    assert_eq!(style.text_wrap_style.as_deref(), Some("stable"));
    assert_eq!(style.wrap_before.as_deref(), Some("avoid"));
    assert_eq!(style.wrap_after.as_deref(), Some("avoid"));
    assert_eq!(style.wrap_inside.as_deref(), Some("avoid"));
    assert_eq!(style.line_padding.as_deref(), Some("0.5em"));
    assert_eq!(style.text_spacing.as_deref(), Some("trim-start allow-end"));
    assert_eq!(style.text_spacing_trim.as_deref(), Some("space-all"));
    assert_eq!(style.text_autospace.as_deref(), Some("ideograph-alpha"));
    assert_eq!(style.text_box.as_deref(), Some("trim-both cap alphabetic"));
    assert_eq!(style.text_box_trim.as_deref(), Some("trim-start"));
    assert_eq!(style.text_box_edge.as_deref(), Some("cap alphabetic"));
    assert_eq!(
        style.hanging_punctuation.as_deref(),
        Some("first allow-end")
    );
    assert_eq!(style.line_clamp.as_deref(), Some("3"));
    assert_eq!(style.block_ellipsis.as_deref(), Some("\"...\""));
    assert_eq!(style.continue_mode.as_deref(), Some("discard"));
    assert_eq!(style.max_lines.as_deref(), Some("4"));
    assert_eq!(style.box_orient.as_deref(), Some("vertical"));
    assert_eq!(style.display, Some(DisplayMode::WebkitBox));
    assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
    assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
    assert_eq!(style.text_decoration_line.as_deref(), Some("underline"));
    assert_eq!(style.text_decoration_style, Some(TextDecorationStyle::Wavy));
    assert_eq!(
        style.text_decoration_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 128,
        })
    );
    assert_eq!(
        style.text_decoration_thickness,
        Some(StyleLength::Points(2.0))
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
    assert_eq!(style.text_emphasis_style.as_deref(), Some("open dot"));
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
    assert_eq!(style.text_overflow, Some(TextOverflow::Ellipsis));
    assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
    assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
    assert_eq!(style.white_space, Some(WhiteSpaceMode::PreWrap));
    assert_eq!(
        style.white_space_collapse.as_deref(),
        Some("preserve-breaks")
    );
    assert_eq!(style.white_space_trim.as_deref(), Some("discard-before"));
    assert_eq!(style.word_break, Some(WordBreakMode::BreakAll));
    assert_eq!(style.overflow_wrap, Some(OverflowWrapMode::Anywhere));
    assert_eq!(style.hyphens, Some(HyphensMode::Auto));
    assert_eq!(style.hyphenate_character.as_deref(), Some("\"=\""));
    assert_eq!(style.hyphenate_limit_zone.as_deref(), Some("8%"));
    assert_eq!(style.hyphenate_limit_chars.as_deref(), Some("6 3 2"));
    assert_eq!(style.hyphenate_limit_lines.as_deref(), Some("2"));
    assert_eq!(style.hyphenate_limit_last.as_deref(), Some("always"));
    assert_eq!(style.content.as_deref(), Some("none"));
    assert_eq!(
        style
            .variant_declarations
            .get("ltr")
            .and_then(|styles| styles.get("direction"))
            .map(String::as_str),
        Some("ltr")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("rtl")
            .and_then(|styles| styles.get("unicode-bidi"))
            .map(String::as_str),
        Some("plaintext")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("-webkit-font-smoothing"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("-moz-osx-font-smoothing"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("letter-spacing"))
            .map(String::as_str),
        Some("0.2em")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("font-stretch"))
            .map(String::as_str),
        Some("87.5%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("font-width"))
            .map(String::as_str),
        Some("expanded")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("math-depth"))
            .map(String::as_str),
        Some("0")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("math-style"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("math-shift"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("font-size-adjust"))
            .map(String::as_str),
        Some("0.6")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("font-palette"))
            .map(String::as_str),
        Some("light")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("font-language-override"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("dominant-baseline"))
            .map(String::as_str),
        Some("hanging")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("baseline-shift"))
            .map(String::as_str),
        Some("sub")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("line-height-step"))
            .map(String::as_str),
        Some("8px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("block-step"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("block-step-size"))
            .map(String::as_str),
        Some("2lh")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("line-grid"))
            .map(String::as_str),
        Some("match-parent")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("line-snap"))
            .map(String::as_str),
        Some("contain")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("box-snap"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("initial-letter"))
            .map(String::as_str),
        Some("2")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("initial-letter-wrap"))
            .map(String::as_str),
        Some("all")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("block-ellipsis"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("max-lines"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("text-align-all"))
            .map(String::as_str),
        Some("center")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-group-align"))
            .map(String::as_str),
        Some("start")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("word-space-transform"))
            .map(String::as_str),
        Some("ideographs")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("wrap-before"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("hyphenate-limit-lines"))
            .map(String::as_str),
        Some("no-limit")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-size-adjust"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-align-last"))
            .map(String::as_str),
        Some("right")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-justify"))
            .map(String::as_str),
        Some("inter-character")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("white-space-collapse"))
            .map(String::as_str),
        Some("preserve-spaces")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-wrap-style"))
            .map(String::as_str),
        Some("pretty")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("text-spacing-trim"))
            .map(String::as_str),
        Some("trim-auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("text-box-trim"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("text-box-edge"))
            .map(String::as_str),
        Some("text")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("text-autospace"))
            .map(String::as_str),
        Some("no-autospace")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("hanging-punctuation"))
            .map(String::as_str),
        Some("last force-end")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("writing-mode"))
            .map(String::as_str),
        Some("horizontal-tb")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("text-combine-upright"))
            .map(String::as_str),
        Some("digits 2")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-decoration-thickness"))
            .map(String::as_str),
        Some("3px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("overflow-wrap"))
            .map(String::as_str),
        Some("break-word")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("overflow-wrap"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("overflow-wrap"))
            .map(String::as_str),
        Some("var(--overflow-wrap)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-decoration-skip"))
            .map(String::as_str),
        Some("objects")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-decoration-skip-box"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("text-decoration-skip-inset"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("text-decoration-skip-self"))
            .map(String::as_str),
        Some("skip")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("text-decoration-skip-spaces"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("text-emphasis-skip"))
            .map(String::as_str),
        Some("punctuation")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-decoration-skip-ink"))
            .map(String::as_str),
        Some("all")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-underline-position"))
            .map(String::as_str),
        Some("left")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-emphasis-style"))
            .map(String::as_str),
        Some("filled sesame")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-emphasis-color"))
            .map(String::as_str),
        Some("#663399")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("text-emphasis-position"))
            .map(String::as_str),
        Some("over left")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-orientation"))
            .map(String::as_str),
        Some("sideways")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("font-feature-settings"))
            .map(String::as_str),
        Some("var(--font-features)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("content"))
            .map(String::as_str),
        Some("'Hello World'")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("content"))
            .map(String::as_str),
        Some("var(--suffix-content)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("content"))
            .map(String::as_str),
        Some("'Hello_World'")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-wrap"))
            .map(String::as_str),
        Some("pretty")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-shadow"))
            .map(String::as_str),
        Some("0 1px 2px rgb(0 0 0 / 0.3)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("lg")
            .and_then(|styles| styles.get("-webkit-line-clamp"))
            .map(String::as_str),
        Some("unset")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("lg")
            .and_then(|styles| styles.get("font-variant-numeric"))
            .map(String::as_str),
        Some("normal")
    );
}
