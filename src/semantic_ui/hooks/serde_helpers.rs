pub(super) fn is_false(value: &bool) -> bool {
    !*value
}

pub(super) fn is_none_or_false(value: &Option<bool>) -> bool {
    !value.unwrap_or(false)
}
