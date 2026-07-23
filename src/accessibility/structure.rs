use super::AccessibilityStructureProps;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessibilitySortValue {
    None,
    Ascending,
    Descending,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AccessibilityStructureViolation {
    pub(crate) attribute: &'static str,
    pub(crate) requirement: &'static str,
}

pub(crate) fn accessibility_structure_violations(
    structure: &AccessibilityStructureProps,
) -> Vec<AccessibilityStructureViolation> {
    let mut violations = Vec::new();
    let level = normalize_positive_u32(structure.level);
    push_invalid(
        &mut violations,
        "aria-level",
        structure.level.is_some() && level.is_none(),
        "must be an integer between 1 and 2147483647",
    );

    let set_size = normalize_set_size(structure.set_size);
    push_invalid(
        &mut violations,
        "aria-setsize",
        structure.set_size.is_some() && set_size.is_none(),
        "must be -1 for an unknown size or an integer greater than or equal to 1",
    );
    let position_in_set = normalize_position_in_set(structure.position_in_set, set_size);
    push_invalid(
        &mut violations,
        "aria-posinset",
        structure.position_in_set.is_some() && position_in_set.is_none(),
        "must be at least 1 and no greater than aria-setsize when the set size is known",
    );

    let row_count = normalize_grid_count(structure.row_count);
    push_invalid(
        &mut violations,
        "aria-rowcount",
        structure.row_count.is_some() && row_count.is_none(),
        "must be -1 for an unknown count or an integer greater than or equal to 0",
    );
    let row_index = normalize_grid_index(structure.row_index, row_count);
    push_invalid(
        &mut violations,
        "aria-rowindex",
        structure.row_index.is_some() && row_index.is_none(),
        "must be at least 1 and no greater than aria-rowcount when the row count is known",
    );
    let row_span = normalize_grid_span(structure.row_span, row_index, row_count);
    push_invalid(
        &mut violations,
        "aria-rowspan",
        structure.row_span.is_some() && row_span.is_none(),
        "must be at least 1, fit a native integer, and remain within a known row count",
    );

    let column_count = normalize_grid_count(structure.column_count);
    push_invalid(
        &mut violations,
        "aria-colcount",
        structure.column_count.is_some() && column_count.is_none(),
        "must be -1 for an unknown count or an integer greater than or equal to 0",
    );
    let column_index = normalize_grid_index(structure.column_index, column_count);
    push_invalid(
        &mut violations,
        "aria-colindex",
        structure.column_index.is_some() && column_index.is_none(),
        "must be at least 1 and no greater than aria-colcount when the column count is known",
    );
    let column_span = normalize_grid_span(structure.column_span, column_index, column_count);
    push_invalid(
        &mut violations,
        "aria-colspan",
        structure.column_span.is_some() && column_span.is_none(),
        "must be at least 1, fit a native integer, and remain within a known column count",
    );

    push_invalid(
        &mut violations,
        "aria-sort",
        structure
            .sort
            .as_deref()
            .is_some_and(|value| normalize_sort(value).is_none()),
        "must be none, ascending, descending, or other",
    );
    violations
}

pub(crate) fn normalize_positive_u32(value: Option<u32>) -> Option<i32> {
    value
        .filter(|value| *value >= 1)
        .and_then(|value| i32::try_from(value).ok())
}

pub(crate) fn normalize_set_size(value: Option<i32>) -> Option<i32> {
    value.filter(|value| *value == -1 || *value >= 1)
}

pub(crate) fn normalize_position_in_set(value: Option<i32>, set_size: Option<i32>) -> Option<i32> {
    value.filter(|value| *value >= 1 && set_size.is_none_or(|size| size == -1 || *value <= size))
}

pub(crate) fn normalize_grid_count(value: Option<i32>) -> Option<i32> {
    value.filter(|value| *value == -1 || *value >= 0)
}

pub(crate) fn normalize_grid_index(value: Option<i32>, count: Option<i32>) -> Option<i32> {
    value.filter(|value| *value >= 1 && count.is_none_or(|count| count == -1 || *value <= count))
}

pub(crate) fn normalize_grid_span(
    value: Option<u32>,
    index: Option<i32>,
    count: Option<i32>,
) -> Option<i32> {
    let span = normalize_positive_u32(value)?;
    let remains_within_count = match (index, count) {
        (Some(index), Some(count)) if count >= 0 => index
            .checked_add(span - 1)
            .is_some_and(|last| last <= count),
        _ => true,
    };
    remains_within_count.then_some(span)
}

pub(crate) fn normalize_sort(value: &str) -> Option<AccessibilitySortValue> {
    match value.trim().to_ascii_lowercase().as_str() {
        "none" => Some(AccessibilitySortValue::None),
        "ascending" => Some(AccessibilitySortValue::Ascending),
        "descending" => Some(AccessibilitySortValue::Descending),
        "other" => Some(AccessibilitySortValue::Other),
        _ => None,
    }
}

fn push_invalid(
    violations: &mut Vec<AccessibilityStructureViolation>,
    attribute: &'static str,
    invalid: bool,
    requirement: &'static str,
) {
    if invalid {
        violations.push(AccessibilityStructureViolation {
            attribute,
            requirement,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structure_validation_accepts_aria_boundaries() {
        let structure = AccessibilityStructureProps::default()
            .level(Some(1))
            .position_in_set(Some(1))
            .set_size(Some(-1))
            .row_count(Some(0))
            .row_span(Some(1))
            .column_count(Some(-1))
            .column_span(Some(1))
            .sort("other");

        assert!(accessibility_structure_violations(&structure).is_empty());
    }

    #[test]
    fn structure_validation_rejects_cross_field_overflow() {
        let structure = AccessibilityStructureProps::default()
            .position_in_set(Some(4))
            .set_size(Some(3))
            .row_count(Some(5))
            .row_index(Some(5))
            .row_span(Some(2))
            .column_count(Some(2))
            .column_index(Some(3));

        assert_eq!(
            accessibility_structure_violations(&structure)
                .iter()
                .map(|violation| violation.attribute)
                .collect::<Vec<_>>(),
            ["aria-posinset", "aria-rowspan", "aria-colindex"]
        );
    }
}
