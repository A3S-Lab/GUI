pub(super) fn non_empty(value: Option<impl Into<String>>) -> Option<String> {
    value.map(Into::into).filter(|value| !value.is_empty())
}
