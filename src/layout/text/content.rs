use std::borrow::Cow;

use crate::error::GuiResult;
use crate::geometry::Size;
use crate::native::{effective_input_type, NativeProps, NativeRole, ValueSensitivity};
use crate::style::{PortableStyle, TextDirection, WritingMode};

use super::{validate_source_text, ShapedText, TextContentSource, TextShapeRequest, TextShaper};

#[derive(Debug, Clone)]
pub(in crate::layout) struct MeasuredText {
    pub source: TextContentSource,
    pub sensitivity: ValueSensitivity,
    pub shaped_text_bytes: u32,
    pub shape: ShapedText,
}

pub(in crate::layout) fn shape_node_text(
    shaper: &dyn TextShaper,
    role: NativeRole,
    props: &NativeProps,
    style: &PortableStyle,
    available: Size,
) -> GuiResult<Option<MeasuredText>> {
    let Some(mut content) = primary_text_content(role, props) else {
        return Ok(None);
    };
    validate_source_text(&content.text)?;
    if content.sensitivity.is_sensitive() {
        content.text = Cow::Owned(std::iter::repeat_n('•', content.text.chars().count()).collect());
    }
    let direction = style.direction.unwrap_or_else(|| {
        if props
            .dir
            .as_deref()
            .is_some_and(|value| value.trim().eq_ignore_ascii_case("rtl"))
        {
            TextDirection::Rtl
        } else {
            TextDirection::Ltr
        }
    });
    let request = TextShapeRequest {
        text: &content.text,
        source: content.source,
        sensitivity: content.sensitivity,
        role,
        language: props.lang.as_deref(),
        direction,
        writing_mode: style.writing_mode.unwrap_or(WritingMode::HorizontalTb),
        available,
        style,
    };
    request.validate()?;
    let shape = shaper.shape(&request)?.quantized();
    shape.validate(request.text)?;
    Ok(Some(MeasuredText {
        source: content.source,
        sensitivity: content.sensitivity,
        shaped_text_bytes: request.text.len() as u32,
        shape,
    }))
}

struct TextContent<'a> {
    text: Cow<'a, str>,
    source: TextContentSource,
    sensitivity: ValueSensitivity,
}

fn primary_text_content<'a>(role: NativeRole, props: &'a NativeProps) -> Option<TextContent<'a>> {
    if role == NativeRole::TextField {
        if let Some(value) = props.value.as_deref().filter(|value| !value.is_empty()) {
            let sensitivity = ValueSensitivity::from_input_type(effective_input_type(props));
            return Some(TextContent {
                text: Cow::Borrowed(value),
                source: TextContentSource::Value,
                sensitivity,
            });
        }
        if let Some(placeholder) = props.placeholder.as_deref() {
            return Some(TextContent {
                text: Cow::Borrowed(placeholder),
                source: TextContentSource::Placeholder,
                sensitivity: ValueSensitivity::Public,
            });
        }
        if props.value.is_some() {
            return Some(TextContent {
                text: Cow::Borrowed(""),
                source: TextContentSource::Value,
                sensitivity: ValueSensitivity::from_input_type(effective_input_type(props)),
            });
        }
        return None;
    }

    if !supports_primary_label(role) {
        return None;
    }
    props
        .label
        .as_deref()
        .map(|label| TextContent {
            text: Cow::Borrowed(label),
            source: TextContentSource::Label,
            sensitivity: ValueSensitivity::Public,
        })
        .or_else(|| {
            props.value.as_deref().map(|value| TextContent {
                text: Cow::Borrowed(value),
                source: TextContentSource::Value,
                sensitivity: ValueSensitivity::Public,
            })
        })
}

fn supports_primary_label(role: NativeRole) -> bool {
    use NativeRole::*;
    matches!(
        role,
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
            | NoBreakText
            | CenteredText
            | FontText
            | BigText
            | TeletypeText
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
            | Checkbox
            | Switch
            | RadioGroup
            | Radio
            | Output
            | Meter
            | Slider
            | ProgressBar
    )
}
