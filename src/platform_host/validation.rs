use crate::error::{GuiError, GuiResult};
use crate::geometry::{Rect, Size};

pub(super) fn validate_non_zero(name: &str, value: u64) -> GuiResult<()> {
    if value == 0 {
        Err(GuiError::host(format!(
            "platform host {name} must be non-zero"
        )))
    } else {
        Ok(())
    }
}

pub(super) fn validate_text(
    name: &str,
    value: &str,
    max_bytes: usize,
    allow_empty: bool,
) -> GuiResult<()> {
    if !allow_empty && value.is_empty() {
        return Err(GuiError::host(format!(
            "platform host {name} must not be empty"
        )));
    }
    if value.len() > max_bytes {
        return Err(GuiError::host(format!(
            "platform host {name} exceeds the {max_bytes}-byte limit"
        )));
    }
    Ok(())
}

pub(super) fn validate_positive_size(name: &str, size: Size) -> GuiResult<()> {
    if !size.width.is_finite()
        || !size.height.is_finite()
        || size.width <= 0.0
        || size.height <= 0.0
    {
        return Err(GuiError::host(format!(
            "platform host {name} must be finite and greater than zero"
        )));
    }
    Ok(())
}

pub(super) fn validate_non_negative_rect(name: &str, rect: Rect) -> GuiResult<()> {
    if !rect.x.is_finite()
        || !rect.y.is_finite()
        || !rect.width.is_finite()
        || !rect.height.is_finite()
        || rect.width < 0.0
        || rect.height < 0.0
    {
        return Err(GuiError::host(format!(
            "platform host {name} must have finite coordinates and non-negative dimensions"
        )));
    }
    Ok(())
}

pub(super) fn validate_finite_pair(name: &str, x: f64, y: f64) -> GuiResult<()> {
    if !x.is_finite() || !y.is_finite() {
        return Err(GuiError::host(format!(
            "platform host {name} must contain finite coordinates"
        )));
    }
    Ok(())
}
