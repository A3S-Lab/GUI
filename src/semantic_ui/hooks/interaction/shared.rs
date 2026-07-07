pub(super) fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}

pub(super) fn finite_or(value: f64, fallback: f64) -> f64 {
    if value.is_finite() {
        value
    } else {
        fallback
    }
}
