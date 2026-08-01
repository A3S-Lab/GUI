//! Executable ownership inventory for the self-drawn renderer migration.
//!
//! The inventory is deliberately separate from style parsing. Parsing a CSS
//! field means that the semantic IR can preserve it; it does not mean that the
//! current layout, paint, text, or host path implements that field.

use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::event::NativeEventKind;
use crate::native::NativeRole;
use crate::style::PortableStyle;

pub const RENDER_FIELD_INVENTORY_VERSION: u16 = 1;

/// First roadmap milestone that must completely project a renderer field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RenderFieldMilestone {
    /// Authoring or semantic bookkeeping that never becomes visual state.
    Semantic,
    /// Generic box layout, rectangle paint, clipping, z-order, and hit bounds.
    M3LayoutScene,
    /// Text, input, IME, focus, overlays, and accessibility bridges.
    M4InteractionText,
    /// Full component, collection, asset, and advanced layout projection.
    P1Components,
    /// Animation and advanced Graphics primitives.
    P2AdvancedGraphics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderFieldInventoryEntry {
    pub field: String,
    pub milestone: RenderFieldMilestone,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableStyleFieldInventory {
    pub schema_version: u16,
    pub fields: Vec<RenderFieldInventoryEntry>,
}

/// Enumerates every top-level `PortableStyle` field and its delivery owner.
///
/// This is generated from the serialized type shape so documentation and
/// diagnostics cannot quietly omit a field that already exists in the IR.
pub fn portable_style_field_inventory() -> GuiResult<PortableStyleFieldInventory> {
    let mut style = PortableStyle::default();
    style
        .variant_declaration_order
        .push(("inventory".to_string(), "inventory".to_string()));
    let value = serde_json::to_value(style).map_err(|error| {
        GuiError::invalid_tree(format!(
            "failed to inspect the portable style renderer contract: {error}"
        ))
    })?;
    let object = value.as_object().ok_or_else(|| {
        GuiError::invalid_tree("portable style renderer inventory must serialize as an object")
    })?;
    let mut fields = Vec::with_capacity(object.len());
    for field in object.keys() {
        let milestone = portable_style_field_milestone(field).ok_or_else(|| {
            GuiError::invalid_tree(format!(
                "PortableStyle field {field:?} has no renderer milestone assignment"
            ))
        })?;
        fields.push(RenderFieldInventoryEntry {
            field: field.clone(),
            milestone,
        });
    }
    Ok(PortableStyleFieldInventory {
        schema_version: RENDER_FIELD_INVENTORY_VERSION,
        fields,
    })
}

/// Classifies either a serialized `PortableStyle` field name or a CSS
/// declaration name. Punctuation and casing do not affect the result.
pub fn portable_style_field_milestone(field: &str) -> Option<RenderFieldMilestone> {
    let field = canonical_field_name(field);
    if field.is_empty() {
        return None;
    }

    if matches!(
        field.as_str(),
        "declarations"
            | "customproperties"
            | "variantdeclarations"
            | "variantdeclarationorder"
            | "all"
            | "unsupported"
    ) {
        return Some(RenderFieldMilestone::Semantic);
    }

    if has_prefix(
        &field,
        &[
            "mask",
            "filter",
            "backdropfilter",
            "transform",
            "translate",
            "rotate",
            "scale",
            "offset",
            "perspective",
            "backface",
            "animation",
            "transition",
            "viewtransition",
            "scrolltimeline",
            "viewtimeline",
            "timelinescope",
            "willchange",
            "mixblend",
            "clip",
            "shape",
            "vector",
            "paintorder",
            "shaperendering",
            "colorrendering",
            "colorinterpolation",
            "stopcolor",
            "stopopacity",
            "flood",
            "lighting",
        ],
    ) || field == "isolation"
    {
        return Some(RenderFieldMilestone::P2AdvancedGraphics);
    }

    if field == "contentvisibility"
        || (field.starts_with("border")
            && !has_prefix(&field, &["borderimage", "bordercollapse", "borderspacing"]))
        || matches!(
            field.as_str(),
            "display"
                | "boxsizing"
                | "position"
                | "order"
                | "width"
                | "height"
                | "gap"
                | "rowgap"
                | "columngap"
                | "inset"
                | "padding"
                | "margin"
                | "background"
                | "backgroundcolor"
                | "visibility"
                | "zindex"
                | "pointerevents"
                | "opacity"
        )
        || has_prefix(
            &field,
            &[
                "flex",
                "align",
                "justify",
                "place",
                "minwidth",
                "minheight",
                "maxwidth",
                "maxheight",
                "inlinesize",
                "blocksize",
                "mininline",
                "minblock",
                "maxinline",
                "maxblock",
                "logicalinset",
                "logicalpadding",
                "logicalmargin",
                "spacex",
                "spacey",
                "borderwidth",
                "logicalborderwidth",
                "bordercolor",
                "bordercolors",
                "logicalbordercolor",
                "borderstyle",
                "borderstyles",
                "logicalborderstyle",
                "borderradius",
                "borderradii",
                "logicalborderradii",
                "overflowx",
                "overflowy",
                "overflowblock",
                "overflowinline",
            ],
        )
    {
        return Some(RenderFieldMilestone::M3LayoutScene);
    }

    if has_prefix(
        &field,
        &[
            "font",
            "webkitfont",
            "moz",
            "webkittext",
            "mstext",
            "text",
            "line",
            "word",
            "letter",
            "tabsize",
            "direction",
            "unicode",
            "writing",
            "whitespace",
            "overflowwrap",
            "hyphen",
            "speak",
            "pause",
            "rest",
            "cue",
            "voice",
            "math",
            "dominantbaseline",
            "baselinesource",
            "alignmentbaseline",
            "baselineshift",
            "initialletter",
            "inlinesizing",
            "blockstep",
            "boxsnap",
            "blockellipsis",
            "continue",
            "continuemode",
            "maxlines",
            "boxorient",
            "hangingpunctuation",
            "wrap",
            "outline",
            "ring",
            "insetring",
            "tailwindshadow",
            "tailwindinsetshadow",
            "textdecoration",
            "textunderline",
            "textemphasis",
            "ruby",
            "caret",
            "touchaction",
            "nav",
            "spatialnavigation",
        ],
    ) || matches!(
        field.as_str(),
        "color"
            | "accentcolor"
            | "boxshadow"
            | "fieldsizing"
            | "appearance"
            | "resize"
            | "interactivity"
            | "cursor"
            | "userselect"
            | "overlay"
    ) {
        return Some(RenderFieldMilestone::M4InteractionText);
    }

    if has_prefix(
        &field,
        &[
            "anchor",
            "positionanchor",
            "positionarea",
            "positiontry",
            "positionvisibility",
            "reading",
            "interpolatesize",
            "grid",
            "contain",
            "container",
            "counter",
            "quotes",
            "stringset",
            "scroll",
            "logicalscroll",
            "overscroll",
            "margintrim",
            "borderimage",
            "divide",
            "backgroundimage",
            "backgroundposition",
            "backgroundsize",
            "backgroundrepeat",
            "backgroundattachment",
            "backgroundorigin",
            "backgroundclip",
            "backgroundblend",
            "image",
            "object",
            "list",
            "marker",
            "column",
            "page",
            "bleed",
            "marks",
            "orphan",
            "widow",
            "bookmark",
            "footnote",
            "break",
            "fill",
            "stroke",
            "table",
            "bordercollapse",
            "borderspacing",
            "caption",
            "emptycells",
            "overflowclipmargin",
            "overflowanchor",
            "aspectratio",
            "colorscheme",
            "forcedcolor",
            "printcolor",
            "coloradjust",
            "scrollbar",
        ],
    ) || matches!(
        field.as_str(),
        "boxdecorationbreak" | "content" | "float" | "clear" | "verticalalign"
    ) {
        return Some(RenderFieldMilestone::P1Components);
    }

    None
}

/// Full visual/behavioral projection milestone for a semantic native role.
/// Generic M3 box layout may still carry roles assigned to later milestones.
pub fn native_role_render_milestone(role: NativeRole) -> RenderFieldMilestone {
    use NativeRole::*;
    match role {
        DocumentHead | Metadata | ResourceLink | StyleSheet | Script | Template | Slot
        | NoEmbedFallback | NoFramesFallback | NextId => RenderFieldMilestone::Semantic,

        Window | View | Document | DocumentBody | Main | Navigation | Header | Footer | Article
        | Section | Aside | Search | Disclosure | Figure | FigureCaption | DescriptionList
        | DescriptionTerm | DescriptionDetails | Form | FieldSet | Legend | OptionGroup
        | Toolbar => RenderFieldMilestone::M3LayoutScene,

        DocumentTitle
        | Text
        | Abbreviation
        | Citation
        | Definition
        | DataValue
        | InsertedText
        | DeletedText
        | MarkedText
        | Time
        | Emphasis
        | StrongText
        | Code
        | KeyboardInput
        | SampleOutput
        | Variable
        | InlineQuote
        | Subscript
        | Superscript
        | SmallText
        | BoldText
        | ItalicText
        | StruckText
        | UnderlinedText
        | BidirectionalIsolate
        | BidirectionalOverride
        | Paragraph
        | PreformattedText
        | BlockQuote
        | ContactAddress
        | LineBreak
        | WordBreakOpportunity
        | NoBreakText
        | CenteredText
        | FontText
        | BigText
        | TeletypeText
        | BackgroundSound
        | Marquee
        | Math
        | SelectedContent
        | Heading
        | HeadingGroup
        | Ruby
        | RubyBase
        | RubyText
        | RubyParenthesis
        | RubyTextContainer
        | DisclosureSummary
        | Button
        | Link
        | ImageMapArea
        | TextField
        | Checkbox
        | Switch
        | RadioGroup
        | Radio
        | Output
        | Meter
        | Separator
        | Slider
        | ProgressBar => RenderFieldMilestone::M4InteractionText,

        Applet | Frame | FrameSet | Image | Media | EmbeddedContent | ImageMap | Select
        | ComboBox | ListBox | ListBoxItem | Tree | TreeItem | Dialog | Popover | Tabs
        | TabList | Tab | TabPanel | Menu | MenuItem | Table | TableSection | TableRow
        | TableCell | TableColumn | TableCaption => RenderFieldMilestone::P1Components,

        Canvas => RenderFieldMilestone::P2AdvancedGraphics,
    }
}

/// Every currently normalized input event belongs to the M4 interaction gate.
/// The exhaustive match forces additions to update this contract.
pub fn native_event_render_milestone(kind: NativeEventKind) -> RenderFieldMilestone {
    use NativeEventKind::*;
    match kind {
        PressStart | PressEnd | PressUp | PressCancel | Press | LongPressStart | LongPressEnd
        | LongPress | MoveStart | Move | MoveEnd | DragStart | DragMove | DragEnd | DropEnter
        | DropMove | DropExit | Drop | Action | HoverStart | HoverEnd | Change
        | SelectionChange | Toggle | Focus | Blur | KeyDown | KeyUp | Wheel | Copy | Cut
        | Paste | Close => RenderFieldMilestone::M4InteractionText,
    }
}

fn canonical_field_name(field: &str) -> String {
    field
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn has_prefix(field: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| field.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_covers_every_current_portable_style_field() {
        let inventory = portable_style_field_inventory().unwrap();
        assert_eq!(inventory.schema_version, RENDER_FIELD_INVENTORY_VERSION);
        assert_eq!(inventory.fields.len(), 504);
        assert!(inventory
            .fields
            .windows(2)
            .all(|fields| fields[0].field < fields[1].field));
    }

    #[test]
    fn css_and_serialized_names_share_one_assignment() {
        assert_eq!(
            portable_style_field_milestone("background-color"),
            portable_style_field_milestone("backgroundColor")
        );
        assert_eq!(
            portable_style_field_milestone("border-inline-start-width"),
            Some(RenderFieldMilestone::M3LayoutScene)
        );
        assert_eq!(
            portable_style_field_milestone("animation-timeline"),
            Some(RenderFieldMilestone::P2AdvancedGraphics)
        );
        assert_eq!(portable_style_field_milestone(""), None);
    }

    #[test]
    fn role_inventory_separates_box_text_asset_and_canvas_completion() {
        assert_eq!(
            native_role_render_milestone(NativeRole::View),
            RenderFieldMilestone::M3LayoutScene
        );
        assert_eq!(
            native_role_render_milestone(NativeRole::Text),
            RenderFieldMilestone::M4InteractionText
        );
        assert_eq!(
            native_role_render_milestone(NativeRole::Image),
            RenderFieldMilestone::P1Components
        );
        assert_eq!(
            native_role_render_milestone(NativeRole::Canvas),
            RenderFieldMilestone::P2AdvancedGraphics
        );
    }
}
