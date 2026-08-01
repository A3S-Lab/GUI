use serde_json::Value;

use crate::render_contract::{portable_style_field_milestone, RenderFieldMilestone};
use crate::style::PortableStyle;

use super::super::{
    LayoutDiagnostic, LayoutDiagnosticCode, LayoutDiagnosticSeverity, LayoutElementId,
};

pub(in crate::layout) fn diagnose_style_inventory(
    style: &PortableStyle,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> crate::GuiResult<()> {
    let value = serde_json::to_value(style).map_err(|error| {
        crate::GuiError::invalid_tree(format!(
            "failed to inspect renderer fields for layout node {id}: {error}"
        ))
    })?;
    let Some(object) = value.as_object() else {
        return Err(crate::GuiError::invalid_tree(format!(
            "portable style for layout node {id} must serialize as an object"
        )));
    };

    for (field, value) in object {
        if !is_effective_value(value) {
            continue;
        }
        match portable_style_field_milestone(field) {
            Some(RenderFieldMilestone::M3LayoutScene) if !supported_m3_field(field) => {
                push_error(
                    diagnostics,
                    id,
                    LayoutDiagnosticCode::UnsupportedM3StyleField,
                    Some(field),
                    format!("M3 style field {field:?} is not implemented by this layout slice"),
                );
            }
            Some(milestone) if milestone > RenderFieldMilestone::M3LayoutScene => {
                push_warning(
                    diagnostics,
                    id,
                    LayoutDiagnosticCode::DeferredStyleField,
                    Some(field),
                    format!("style field {field:?} is deferred to {milestone:?}"),
                );
            }
            _ => {}
        }
    }

    for property in style.unsupported.keys() {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnparsedStyle,
            Some(property),
            format!("style property {property:?} was not parsed into the portable style IR"),
        );
    }
    if !style.variant_declarations.is_empty() {
        push_warning(
            diagnostics,
            id,
            LayoutDiagnosticCode::DeferredStyleField,
            Some("variantDeclarations"),
            "interaction style variants are resolved by the M4 state projection",
        );
    }
    Ok(())
}

fn supported_m3_field(field: &str) -> bool {
    matches!(
        field,
        "display"
            | "boxSizing"
            | "position"
            | "flexDirection"
            | "flexWrap"
            | "order"
            | "alignItems"
            | "alignSelf"
            | "justifyContent"
            | "width"
            | "height"
            | "minWidth"
            | "minHeight"
            | "maxWidth"
            | "maxHeight"
            | "inlineSize"
            | "blockSize"
            | "minInlineSize"
            | "minBlockSize"
            | "maxInlineSize"
            | "maxBlockSize"
            | "contentVisibility"
            | "inset"
            | "logicalInset"
            | "padding"
            | "logicalPadding"
            | "margin"
            | "logicalMargin"
            | "gap"
            | "rowGap"
            | "columnGap"
            | "spaceX"
            | "spaceY"
            | "borderWidth"
            | "logicalBorderWidth"
            | "borderColor"
            | "borderColors"
            | "logicalBorderColors"
            | "borderStyle"
            | "borderStyles"
            | "logicalBorderStyles"
            | "background"
            | "backgroundColor"
            | "borderRadius"
            | "borderRadii"
            | "logicalBorderRadii"
            | "overflowX"
            | "overflowY"
            | "overflowBlock"
            | "overflowInline"
            | "visibility"
            | "zIndex"
            | "pointerEvents"
            | "opacity"
    )
}

fn is_effective_value(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::String(value) => !value.is_empty(),
        Value::Array(values) => values.iter().any(is_effective_value),
        Value::Object(values) => values.values().any(is_effective_value),
        Value::Number(_) => true,
    }
}

pub(in crate::layout) fn push_warning(
    diagnostics: &mut Vec<LayoutDiagnostic>,
    id: &LayoutElementId,
    code: LayoutDiagnosticCode,
    field: Option<&str>,
    message: impl Into<String>,
) {
    diagnostics.push(LayoutDiagnostic {
        severity: LayoutDiagnosticSeverity::Warning,
        code,
        element: id.clone(),
        field: field.map(str::to_string),
        message: message.into(),
    });
}

pub(super) fn push_error(
    diagnostics: &mut Vec<LayoutDiagnostic>,
    id: &LayoutElementId,
    code: LayoutDiagnosticCode,
    field: Option<&str>,
    message: impl Into<String>,
) {
    diagnostics.push(LayoutDiagnostic {
        severity: LayoutDiagnosticSeverity::Error,
        code,
        element: id.clone(),
        field: field.map(str::to_string),
        message: message.into(),
    });
}
