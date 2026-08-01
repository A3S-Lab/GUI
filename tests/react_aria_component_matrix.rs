#![cfg(feature = "design-system")]

use std::collections::BTreeSet;

use a3s_gui::builtin_component_registry;
use serde::Deserialize;

const OFFICIAL_COMPONENT_FAMILIES: [&str; 51] = [
    "Autocomplete",
    "Breadcrumbs",
    "Button",
    "Calendar",
    "Checkbox",
    "CheckboxGroup",
    "ColorArea",
    "ColorField",
    "ColorPicker",
    "ColorSlider",
    "ColorSwatch",
    "ColorSwatchPicker",
    "ColorWheel",
    "ComboBox",
    "DateField",
    "DatePicker",
    "DateRangePicker",
    "Disclosure",
    "DisclosureGroup",
    "DropZone",
    "FileTrigger",
    "Form",
    "GridList",
    "Group",
    "Link",
    "ListBox",
    "Menu",
    "Meter",
    "Modal",
    "NumberField",
    "Popover",
    "ProgressBar",
    "RadioGroup",
    "RangeCalendar",
    "SearchField",
    "Select",
    "Separator",
    "Slider",
    "Switch",
    "Table",
    "Tabs",
    "TagGroup",
    "TextField",
    "TimeField",
    "Toast",
    "ToggleButton",
    "ToggleButtonGroup",
    "Toolbar",
    "Tooltip",
    "Tree",
    "Virtualizer",
];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ComponentMatrix {
    schema_version: u32,
    upstream: Upstream,
    acceptance_dimensions: Vec<String>,
    families: Vec<ComponentFamily>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Upstream {
    package: String,
    version: String,
    release_date: String,
    catalog_url: String,
    release_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ComponentFamily {
    upstream: String,
    category: String,
    milestone: String,
    a3s_components: Vec<String>,
    planned_parts: Vec<PlannedPart>,
    self_drawn_status: String,
    evidence: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlannedPart {
    upstream: String,
    a3s_target: String,
}

fn matrix() -> ComponentMatrix {
    serde_json::from_str(include_str!("../docs/react-aria-component-matrix.json"))
        .expect("React Aria component matrix must remain valid and schema-checked")
}

#[test]
fn matrix_pins_the_complete_official_1_19_component_catalog() {
    let matrix = matrix();

    assert_eq!(matrix.schema_version, 1);
    assert_eq!(matrix.upstream.package, "react-aria-components");
    assert_eq!(matrix.upstream.version, "1.19.0");
    assert_eq!(matrix.upstream.release_date, "2026-06-18");
    assert_eq!(matrix.upstream.catalog_url, "https://react-aria.adobe.com/");
    assert_eq!(
        matrix.upstream.release_url,
        "https://react-aria.adobe.com/releases/v1-19-0"
    );

    let names = matrix
        .families
        .iter()
        .map(|family| family.upstream.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, OFFICIAL_COMPONENT_FAMILIES);
    assert_eq!(names.iter().copied().collect::<BTreeSet<_>>().len(), 51);
}

#[test]
fn every_family_maps_to_registered_a3s_authoring_components() {
    let matrix = matrix();
    let registry = builtin_component_registry().expect("built-in component registry");
    let mut mapped_components = BTreeSet::new();

    for family in &matrix.families {
        assert!(
            matches!(
                family.category.as_str(),
                "foundation" | "form" | "collection" | "overlay" | "data" | "date-time" | "color"
            ),
            "{} has an unknown category {}",
            family.upstream,
            family.category
        );
        assert!(
            matches!(family.milestone.as_str(), "M6" | "M7" | "M8"),
            "{} has an unknown milestone {}",
            family.upstream,
            family.milestone
        );
        assert!(
            !family.a3s_components.is_empty(),
            "{} must map to at least one A3S component",
            family.upstream
        );

        for component in &family.a3s_components {
            assert!(
                registry.contains(component),
                "{} maps to missing built-in component {}",
                family.upstream,
                component
            );
            assert!(
                mapped_components.insert(component),
                "A3S component {component} is assigned to multiple families"
            );
        }
    }
}

#[test]
fn planned_parts_and_self_drawn_claims_cannot_be_silent() {
    let matrix = matrix();
    let registry = builtin_component_registry().expect("built-in component registry");
    let dimensions = matrix
        .acceptance_dimensions
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        dimensions,
        BTreeSet::from([
            "accessibility",
            "authoring",
            "behavior",
            "native-host",
            "scene",
            "software-pixels",
        ])
    );

    let mut planned_targets = BTreeSet::new();
    for family in &matrix.families {
        assert!(
            matches!(
                family.self_drawn_status.as_str(),
                "planned" | "scene-smoke" | "conformant"
            ),
            "{} has an unknown self-drawn status {}",
            family.upstream,
            family.self_drawn_status
        );
        if family.self_drawn_status == "planned" {
            assert!(
                family.evidence.is_empty(),
                "{} cannot attach completion evidence while still planned",
                family.upstream
            );
        } else {
            assert!(
                !family.evidence.is_empty(),
                "{} must cite executable self-drawn evidence",
                family.upstream
            );
        }
        if family.self_drawn_status == "conformant" {
            for required in ["software:", "macos:", "windows:", "linux:"] {
                assert!(
                    family
                        .evidence
                        .iter()
                        .any(|item| item.starts_with(required)),
                    "{} is conformant without {required} evidence",
                    family.upstream
                );
            }
        }

        for part in &family.planned_parts {
            assert!(!part.upstream.is_empty());
            assert!(
                planned_targets.insert(part.a3s_target.as_str()),
                "planned A3S part {} is duplicated",
                part.a3s_target
            );
            assert!(
                !registry.contains(&part.a3s_target),
                "{} is registered; move it from planned_parts into a3s_components",
                part.a3s_target
            );
        }
    }

    assert_eq!(
        planned_targets,
        BTreeSet::from([
            "UiCheckboxButton",
            "UiCheckboxField",
            "UiRadioButton",
            "UiRadioField",
            "UiSwitchButton",
            "UiSwitchField",
            "UiToastContent",
            "UiToastList",
        ])
    );
}
