use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

mod component_variants;
mod components;
mod props;

use component_variants::merge_class_names;
pub use component_variants::ComponentClassVariants;
use components::component_from_rsx_tag;

use crate::error::{GuiError, GuiResult};
use crate::native::NativeElement;
use crate::selection::Selection;
use crate::semantic_ui::{
    use_autocomplete_value, use_breadcrumbs_value, use_button_value, use_calendar_cell_value,
    use_calendar_value, use_checkbox_group_value, use_checkbox_value, use_collection_item_value,
    use_collection_section_value, use_collection_value, use_color_area_value,
    use_color_field_value, use_color_picker_value, use_color_slider_value,
    use_color_swatch_picker_item_value, use_color_swatch_picker_value, use_color_swatch_value,
    use_color_thumb_value, use_color_wheel_value, use_combo_box_display_value, use_combo_box_value,
    use_date_field_value, use_date_input_value, use_date_picker_value, use_date_range_picker_value,
    use_date_segment_value, use_description_value, use_disclosure_group_value,
    use_disclosure_value, use_drag_value, use_drop_value, use_drop_zone_value,
    use_field_error_value, use_field_value, use_file_trigger_value, use_focus_ring_value,
    use_focus_scope_value, use_focus_within_value, use_focusable_value, use_form_value,
    use_grid_list_header_value, use_group_value, use_heading_value, use_i18n_value,
    use_keyboard_value, use_label_value, use_landmark_value, use_legend_value, use_link_value,
    use_list_box_header_value, use_load_more_item_value, use_menu_item_value, use_menu_value,
    use_overlay_value, use_press_value, use_radio_group_value, use_radio_value,
    use_range_calendar_value, use_range_value, use_select_display_value, use_select_value,
    use_selection_value, use_submenu_trigger_value, use_switch_value, use_tab_list_value,
    use_tab_panel_value, use_tab_value, use_table_caption_value, use_table_cell_value,
    use_table_column_value, use_table_row_value, use_table_section_value, use_table_value,
    use_text_field_value, use_text_value, use_time_field_value, use_toast_region_value,
    use_toast_value, use_toggle_button_group_value, use_toggle_button_value, use_toggle_value,
    use_tree_header_value, use_tree_item_value, use_tree_value, use_visually_hidden_value,
    CollectionSectionKind, SemanticElement, SemanticMapper, TableSectionKind, UseAutocompleteProps,
    UseBreadcrumbsProps, UseButtonProps, UseCalendarCellProps, UseCalendarProps,
    UseCheckboxGroupProps, UseCheckboxProps, UseCollectionItemProps, UseCollectionProps,
    UseCollectionSectionProps, UseColorAreaProps, UseColorFieldProps, UseColorPickerProps,
    UseColorRangeProps, UseColorSwatchPickerItemProps, UseColorSwatchPickerProps,
    UseColorSwatchProps, UseColorThumbProps, UseComboBoxDisplayProps, UseComboBoxProps,
    UseDateFieldProps, UseDateInputProps, UseDatePickerProps, UseDateRangePickerProps,
    UseDateSegmentProps, UseDisclosureGroupProps, UseDisclosureProps, UseDragProps, UseDropProps,
    UseDropZoneProps, UseFieldProps, UseFileTriggerProps, UseFocusRingProps, UseFocusScopeProps,
    UseFocusWithinProps, UseFocusableProps, UseFormProps, UseGroupProps, UseHeadingProps,
    UseI18nProps, UseLandmarkProps, UseLinkProps, UseLoadMoreItemProps, UseMenuItemProps,
    UseMenuProps, UseOverlayProps, UsePressProps, UseRadioGroupProps, UseRadioProps,
    UseRangeCalendarProps, UseRangeProps, UseSelectDisplayProps, UseSelectProps, UseSelectionProps,
    UseSubmenuTriggerProps, UseSwitchProps, UseTabListProps, UseTabPanelProps, UseTabProps,
    UseTableCaptionProps, UseTableCellProps, UseTableColumnProps, UseTableProps, UseTableRowProps,
    UseTableSectionProps, UseTextFieldProps, UseTextProps, UseTimeFieldProps, UseToastProps,
    UseToastRegionProps, UseToggleButtonGroupProps, UseToggleButtonProps, UseToggleProps,
    UseTreeItemProps, UseTreeProps,
};
use crate::semantic_ui::{
    use_clipboard_value, use_hover_value, use_keyboard_interaction_value, use_long_press_value,
    use_move_value, UseClipboardProps, UseHoverProps, UseKeyboardInteractionProps,
    UseLongPressProps, UseMoveProps,
};
use crate::semantic_ui::{
    use_drop_indicator_value, use_selection_indicator_value, use_separator_value,
    use_toolbar_value, UseDropIndicatorProps, UseSelectionIndicatorProps, UseSeparatorProps,
    UseToolbarProps,
};
use crate::semantic_ui::{use_number_field_value, UseNumberFieldProps};
use crate::semantic_ui::{
    use_slider_fill_value, use_slider_output_value, use_slider_track_value, UseSliderFillProps,
    UseSliderOutputProps, UseSliderTrackProps,
};
use crate::semantic_ui::{use_virtualizer_value, UseVirtualizerProps};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CompiledRsxNode {
    Element {
        key: String,
        tag: String,
        #[serde(default)]
        import_source: Option<String>,
        #[serde(default)]
        props: CompiledProps,
        #[serde(default)]
        children: Vec<CompiledRsxNode>,
    },
    Text {
        key: String,
        value: String,
    },
}

impl CompiledRsxNode {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            CompiledRsxNode::Element {
                key, tag, children, ..
            } => {
                if key.is_empty() {
                    return Err(GuiError::invalid_tree(
                        "a3s-gui compiled elements need non-empty keys",
                    ));
                }
                if tag.is_empty() {
                    return Err(GuiError::invalid_tree(
                        "a3s-gui compiled elements need non-empty tags",
                    ));
                }
                validate_compiled_children(children)
            }
            CompiledRsxNode::Text { key, .. } => {
                if key.is_empty() {
                    return Err(GuiError::invalid_tree(
                        "a3s-gui compiled text nodes need non-empty keys",
                    ));
                }
                Ok(())
            }
        }
    }

    fn key(&self) -> &str {
        match self {
            CompiledRsxNode::Element { key, .. } | CompiledRsxNode::Text { key, .. } => key,
        }
    }

    pub fn has_bindings(&self) -> bool {
        match self {
            CompiledRsxNode::Text { .. } => false,
            CompiledRsxNode::Element {
                props, children, ..
            } => {
                !props.bindings.is_empty()
                    || !props.spreads.is_empty()
                    || children.iter().any(CompiledRsxNode::has_bindings)
            }
        }
    }
}

fn validate_compiled_children(children: &[CompiledRsxNode]) -> GuiResult<()> {
    let mut sibling_keys = BTreeSet::new();
    for child in children {
        child.validate()?;
        let key = child.key();
        if !sibling_keys.insert(key) {
            return Err(GuiError::invalid_tree(format!(
                "a3s-gui compiled sibling nodes need unique keys; duplicate key {key:?}"
            )));
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledProps {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub text_value: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default, alias = "aria-label")]
    pub aria_label: Option<String>,
    #[serde(default, alias = "disabled", alias = "aria-disabled")]
    pub is_disabled: bool,
    #[serde(default, alias = "required", alias = "aria-required")]
    pub is_required: bool,
    #[serde(default, alias = "invalid", alias = "aria-invalid")]
    pub is_invalid: bool,
    #[serde(
        default,
        alias = "readOnly",
        alias = "readonly",
        alias = "aria-readonly"
    )]
    pub is_read_only: bool,
    #[serde(default, alias = "selected", alias = "aria-selected")]
    pub is_selected: bool,
    #[serde(default, alias = "checked", alias = "aria-checked")]
    pub is_checked: Option<bool>,
    #[serde(default, alias = "expanded", alias = "aria-expanded")]
    pub is_expanded: Option<bool>,
    #[serde(default, alias = "aria-orientation")]
    pub orientation: Option<CompiledOrientation>,
    #[serde(default, alias = "min", alias = "aria-valuemin")]
    pub min_value: Option<f64>,
    #[serde(default, alias = "max", alias = "aria-valuemax")]
    pub max_value: Option<f64>,
    #[serde(default, alias = "current", alias = "aria-valuenow")]
    pub value_number: Option<f64>,
    #[serde(default, alias = "step")]
    pub step_value: Option<f64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub form: Option<String>,
    #[serde(default)]
    pub input_type: Option<String>,
    #[serde(default)]
    pub accept: Option<String>,
    #[serde(default)]
    pub capture: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
    #[serde(default)]
    pub href: Option<String>,
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default, alias = "srcSet")]
    pub srcset: Option<String>,
    #[serde(default)]
    pub sizes: Option<String>,
    #[serde(default)]
    pub media: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub intrinsic_width: Option<u32>,
    #[serde(default)]
    pub intrinsic_height: Option<u32>,
    #[serde(default)]
    pub loading: Option<String>,
    #[serde(default)]
    pub decoding: Option<String>,
    #[serde(default)]
    pub fetch_priority: Option<String>,
    #[serde(default)]
    pub cross_origin: Option<String>,
    #[serde(default)]
    pub referrer_policy: Option<String>,
    #[serde(default)]
    pub poster: Option<String>,
    #[serde(default)]
    pub controls: Option<bool>,
    #[serde(default, alias = "autoPlay")]
    pub autoplay: Option<bool>,
    #[serde(default)]
    pub loop_playback: Option<bool>,
    #[serde(default)]
    pub muted: Option<bool>,
    #[serde(default)]
    pub plays_inline: Option<bool>,
    #[serde(default)]
    pub preload: Option<String>,
    #[serde(default)]
    pub track_kind: Option<String>,
    #[serde(default, alias = "srcLang")]
    pub srclang: Option<String>,
    #[serde(default)]
    pub track_label: Option<String>,
    #[serde(default)]
    pub default_track: Option<bool>,
    #[serde(default)]
    pub list: Option<String>,
    #[serde(default)]
    pub dirname: Option<String>,
    #[serde(default)]
    pub form_action: Option<String>,
    #[serde(default, alias = "formEncType")]
    pub form_enctype: Option<String>,
    #[serde(default)]
    pub form_method: Option<String>,
    #[serde(default)]
    pub form_target: Option<String>,
    #[serde(default)]
    pub form_no_validate: Option<bool>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub class_name: Option<String>,
    #[serde(default)]
    pub style: BTreeMap<String, CompiledStyleValue>,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
    #[serde(default)]
    pub events: BTreeMap<String, String>,
    #[serde(default)]
    pub action_labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub bindings: BTreeMap<String, CompiledBinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spreads: Vec<CompiledBinding>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub explicit_props: BTreeSet<String>,
}

impl Default for CompiledProps {
    fn default() -> Self {
        Self {
            label: None,
            text_value: None,
            value: None,
            placeholder: None,
            action: None,
            aria_label: None,
            is_disabled: false,
            is_required: false,
            is_invalid: false,
            is_read_only: false,
            is_selected: false,
            is_checked: None,
            is_expanded: None,
            orientation: None,
            min_value: None,
            max_value: None,
            value_number: None,
            step_value: None,
            name: None,
            form: None,
            input_type: None,
            accept: None,
            capture: None,
            alt: None,
            href: None,
            src: None,
            srcset: None,
            sizes: None,
            media: None,
            resource_type: None,
            intrinsic_width: None,
            intrinsic_height: None,
            loading: None,
            decoding: None,
            fetch_priority: None,
            cross_origin: None,
            referrer_policy: None,
            poster: None,
            controls: None,
            autoplay: None,
            loop_playback: None,
            muted: None,
            plays_inline: None,
            preload: None,
            track_kind: None,
            srclang: None,
            track_label: None,
            default_track: None,
            list: None,
            dirname: None,
            form_action: None,
            form_enctype: None,
            form_method: None,
            form_target: None,
            form_no_validate: None,
            id: None,
            class_name: None,
            style: BTreeMap::new(),
            attributes: BTreeMap::new(),
            events: BTreeMap::new(),
            action_labels: BTreeMap::new(),
            bindings: BTreeMap::new(),
            spreads: Vec::new(),
            explicit_props: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompiledOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompiledStyleValue {
    String(String),
    Number(f64),
    Bool(bool),
}

impl CompiledStyleValue {
    pub fn to_portable_value(&self) -> String {
        match self {
            CompiledStyleValue::String(value) => value.clone(),
            CompiledStyleValue::Number(value) => {
                if value.fract() == 0.0 {
                    format!("{value:.0}")
                } else {
                    value.to_string()
                }
            }
            CompiledStyleValue::Bool(value) => value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledBinding {
    pub source: CompiledBindingSource,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompiledBindingSource {
    State,
    Props,
    Derived,
    Context,
    Resource,
    Local,
}

impl CompiledBinding {
    pub fn state(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::State,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn props(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::Props,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn derived(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::Derived,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn context(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::Context,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn resource(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::Resource,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn display_path(&self) -> String {
        let root = match self.source {
            CompiledBindingSource::State => "state",
            CompiledBindingSource::Props => "props",
            CompiledBindingSource::Derived => "derived",
            CompiledBindingSource::Context => "context",
            CompiledBindingSource::Resource => "resource",
            CompiledBindingSource::Local => {
                return if self.path.is_empty() {
                    "local".to_string()
                } else {
                    self.path.join(".")
                };
            }
        };
        if self.path.is_empty() {
            root.to_string()
        } else {
            format!("{root}.{}", self.path.join("."))
        }
    }

    fn resolve<'a>(&self, scope: &'a JsonValue) -> GuiResult<&'a JsonValue> {
        let root = match self.source {
            CompiledBindingSource::State => "state",
            CompiledBindingSource::Props => "props",
            CompiledBindingSource::Derived => "derived",
            CompiledBindingSource::Context => "context",
            CompiledBindingSource::Resource => "resource",
            CompiledBindingSource::Local => {
                let Some(root) = self.path.first() else {
                    return Err(GuiError::invalid_tree(
                        "RSX local binding cannot resolve an empty local path",
                    ));
                };
                root
            }
        };
        let mut value = scope.get(root).ok_or_else(|| {
            GuiError::invalid_tree(format!(
                "RSX binding {} cannot resolve missing scope root {root:?}",
                self.display_path()
            ))
        })?;
        let path = match self.source {
            CompiledBindingSource::State
            | CompiledBindingSource::Props
            | CompiledBindingSource::Derived
            | CompiledBindingSource::Context
            | CompiledBindingSource::Resource => self.path.as_slice(),
            CompiledBindingSource::Local => &self.path[1..],
        };
        let display_path = self.display_path();
        for segment in path {
            value = json_path_get(value, segment).ok_or_else(|| {
                GuiError::invalid_tree(format!(
                    "RSX binding {display_path} cannot resolve missing path segment {segment:?}",
                ))
            })?;
        }
        Ok(value)
    }
}

fn json_path_get<'a>(value: &'a JsonValue, segment: &str) -> Option<&'a JsonValue> {
    match value {
        JsonValue::Object(object) => object.get(segment),
        JsonValue::Array(items) => segment
            .parse::<usize>()
            .ok()
            .and_then(|index| items.get(index)),
        JsonValue::Null | JsonValue::Bool(_) | JsonValue::Number(_) | JsonValue::String(_) => None,
    }
}

impl CompiledRsxNode {
    pub fn resolve_bindings(&self, scope: &JsonValue) -> GuiResult<Self> {
        self.resolve_bindings_with_components(scope, &BTreeMap::new())
    }

    pub fn resolve_bindings_with_components(
        &self,
        scope: &JsonValue,
        components: &BTreeMap<String, CompiledRsxNode>,
    ) -> GuiResult<Self> {
        self.resolve_bindings_with_component_defaults(scope, components, &BTreeMap::new())
    }

    pub fn resolve_bindings_with_component_defaults(
        &self,
        scope: &JsonValue,
        components: &BTreeMap<String, CompiledRsxNode>,
        component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    ) -> GuiResult<Self> {
        self.resolve_bindings_with_component_options(
            scope,
            components,
            component_defaults,
            &BTreeMap::new(),
        )
    }

    pub fn resolve_bindings_with_component_options(
        &self,
        scope: &JsonValue,
        components: &BTreeMap<String, CompiledRsxNode>,
        component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
        component_variants: &BTreeMap<String, ComponentClassVariants>,
    ) -> GuiResult<Self> {
        let mut resolved = resolve_node_bindings_with_components(
            self,
            scope,
            components,
            component_defaults,
            component_variants,
            &mut Vec::new(),
            None,
        )?;
        match resolved.len() {
            0 => Ok(CompiledRsxNode::Element {
                key: self.key().to_string(),
                tag: "Fragment".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: Vec::new(),
            }),
            1 => Ok(resolved.remove(0)),
            _ => Ok(CompiledRsxNode::Element {
                key: self.key().to_string(),
                tag: "Fragment".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: resolved,
            }),
        }
    }
}

fn resolve_node_bindings_with_components(
    node: &CompiledRsxNode,
    scope: &JsonValue,
    components: &BTreeMap<String, CompiledRsxNode>,
    component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    component_variants: &BTreeMap<String, ComponentClassVariants>,
    component_stack: &mut Vec<String>,
    slots: Option<&ResolvedSlots>,
) -> GuiResult<Vec<CompiledRsxNode>> {
    match node {
        CompiledRsxNode::Text { .. } => Ok(vec![node.clone()]),
        CompiledRsxNode::Element {
            key,
            tag,
            import_source,
            props,
            children,
        } => {
            if is_for_control(tag) {
                return resolve_for_control(
                    key,
                    props,
                    children,
                    scope,
                    components,
                    component_defaults,
                    component_variants,
                    component_stack,
                    slots,
                );
            }

            let mut props = props.clone();
            if let Some(defaults) = component_defaults.get(tag) {
                props.apply_default_props(defaults)?;
            }
            props.resolve_bindings(scope)?;
            if let Some(variants) = component_variants.get(tag) {
                variants.apply_to_props(tag, &mut props)?;
            }

            if is_slot_control(tag) {
                if let Some(slots) = slots {
                    return Ok(slots
                        .children(props.name.as_deref())
                        .iter()
                        .cloned()
                        .map(|child| prefix_node_keys(child, key))
                        .collect());
                }
            }

            if let Some(component) = components.get(tag) {
                if component_stack.iter().any(|name| name == tag) {
                    let mut cycle = component_stack.clone();
                    cycle.push(tag.clone());
                    return Err(GuiError::invalid_tree(format!(
                        "RSX component cycle detected: {}",
                        cycle.join(" -> ")
                    )));
                }

                let resolved_slot_children = resolve_children_bindings_with_components(
                    children,
                    scope,
                    components,
                    component_defaults,
                    component_variants,
                    component_stack,
                    slots,
                )?;
                let resolved_slots = ResolvedSlots::from_children(resolved_slot_children);
                let component_scope = extend_component_scope(scope, &props)?;
                component_stack.push(tag.clone());
                let resolved = resolve_node_bindings_with_components(
                    component,
                    &component_scope,
                    components,
                    component_defaults,
                    component_variants,
                    component_stack,
                    Some(&resolved_slots),
                )?
                .into_iter()
                .map(|child| prefix_node_keys(child, key))
                .collect::<Vec<_>>();
                component_stack.pop();
                return Ok(resolved);
            }

            if is_show_control(tag) {
                return if show_condition(tag, &props)? {
                    resolve_children_bindings_with_components(
                        children,
                        scope,
                        components,
                        component_defaults,
                        component_variants,
                        component_stack,
                        slots,
                    )
                } else {
                    Ok(Vec::new())
                };
            }

            let children = resolve_children_bindings_with_components(
                children,
                scope,
                components,
                component_defaults,
                component_variants,
                component_stack,
                slots,
            )?;

            Ok(vec![CompiledRsxNode::Element {
                key: key.clone(),
                tag: tag.clone(),
                import_source: import_source.clone(),
                props,
                children,
            }])
        }
    }
}

fn resolve_children_bindings_with_components(
    children: &[CompiledRsxNode],
    scope: &JsonValue,
    components: &BTreeMap<String, CompiledRsxNode>,
    component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    component_variants: &BTreeMap<String, ComponentClassVariants>,
    component_stack: &mut Vec<String>,
    slots: Option<&ResolvedSlots>,
) -> GuiResult<Vec<CompiledRsxNode>> {
    let children = children
        .iter()
        .map(|child| {
            resolve_node_bindings_with_components(
                child,
                scope,
                components,
                component_defaults,
                component_variants,
                component_stack,
                slots,
            )
        })
        .collect::<GuiResult<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    Ok(children)
}

fn is_show_control(tag: &str) -> bool {
    matches!(tag, "Show" | "When")
}

fn is_for_control(tag: &str) -> bool {
    matches!(tag, "For" | "Each")
}

fn is_slot_control(tag: &str) -> bool {
    matches!(tag, "Slot" | "slot")
}

#[derive(Debug, Default)]
struct ResolvedSlots {
    default: Vec<CompiledRsxNode>,
    named: BTreeMap<String, Vec<CompiledRsxNode>>,
}

impl ResolvedSlots {
    fn from_children(children: Vec<CompiledRsxNode>) -> Self {
        let mut slots = Self::default();
        for mut child in children {
            match take_structural_slot_name(&mut child) {
                Some(name) if !name.is_empty() => {
                    slots.named.entry(name).or_default().push(child);
                }
                _ => slots.default.push(child),
            }
        }
        slots
    }

    fn children(&self, name: Option<&str>) -> &[CompiledRsxNode] {
        match name {
            Some(name) if !name.is_empty() => {
                self.named.get(name).map(Vec::as_slice).unwrap_or(&[])
            }
            _ => self.default.as_slice(),
        }
    }
}

fn take_structural_slot_name(node: &mut CompiledRsxNode) -> Option<String> {
    let CompiledRsxNode::Element { props, .. } = node else {
        return None;
    };
    props.attributes.remove("slot")
}

fn resolve_for_control(
    key: &str,
    props: &CompiledProps,
    children: &[CompiledRsxNode],
    scope: &JsonValue,
    components: &BTreeMap<String, CompiledRsxNode>,
    component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    component_variants: &BTreeMap<String, ComponentClassVariants>,
    component_stack: &mut Vec<String>,
    slots: Option<&ResolvedSlots>,
) -> GuiResult<Vec<CompiledRsxNode>> {
    let item_name = control_identifier_attribute(props, "as")?.unwrap_or("item");
    let index_name = control_identifier_attribute(props, "indexAs")?;
    if index_name == Some(item_name) {
        return Err(GuiError::invalid_tree(
            "RSX <For> indexAs cannot reuse the item variable name",
        ));
    }
    let key_by = props.attributes.get("keyBy").map(String::as_str);
    let each = for_each_binding(props)?;
    let items = each.resolve(scope)?.as_array().ok_or_else(|| {
        GuiError::invalid_tree(format!(
            "RSX <For> each binding {} must resolve to an array",
            each.display_path()
        ))
    })?;

    let mut rendered = Vec::new();
    let mut item_keys = BTreeSet::new();
    for (index, item) in items.iter().enumerate() {
        let item_key = for_item_key(item, key_by, index)?;
        if !item_keys.insert(item_key.clone()) {
            return Err(GuiError::invalid_tree(format!(
                "RSX <For> produced duplicate item key {item_key:?}"
            )));
        }
        let item_scope = extend_local_scope(scope, item_name, item, index_name, index)?;
        let item_prefix = format!("{key}-{item_key}");
        rendered.extend(
            resolve_children_bindings_with_components(
                children,
                &item_scope,
                components,
                component_defaults,
                component_variants,
                component_stack,
                slots,
            )?
            .into_iter()
            .map(|child| prefix_node_keys(child, &item_prefix)),
        );
    }

    Ok(rendered)
}

fn extend_component_scope(scope: &JsonValue, props: &CompiledProps) -> GuiResult<JsonValue> {
    let JsonValue::Object(scope) = scope else {
        return Err(GuiError::invalid_tree(
            "RSX binding scope must be a JSON object",
        ));
    };
    let mut scope = scope.clone();
    scope.insert("props".to_string(), props_scope_value(props));
    Ok(JsonValue::Object(scope))
}

fn props_scope_value(props: &CompiledProps) -> JsonValue {
    let mut scope = JsonMap::new();

    if let Ok(press) = press_scope_value(props) {
        if let JsonValue::Object(press_scope) = &press {
            if let Some(press_props) = press_scope.get("pressProps") {
                scope.insert("pressProps".to_string(), press_props.clone());
            }
            if let Some(is_pressed) = press_scope.get("isPressed") {
                scope.insert("isPressed".to_string(), is_pressed.clone());
            }
        }
        scope.insert("press".to_string(), press);
    }

    if let Ok(button) = button_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "button",
            button,
            &["buttonProps", "pressProps"],
            &["isPressed"],
        );
    }

    if let Ok(link) = link_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "link",
            link,
            &["linkProps"],
            &["href", "isDisabled", "isPressed"],
        );
    }

    if let Ok(hover) = hover_scope_value(props) {
        insert_hook_scope(&mut scope, "hover", hover, &["hoverProps"], &["isHovered"]);
    }

    if let Ok(keyboard_interaction) = keyboard_interaction_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "keyboardInteraction",
            keyboard_interaction,
            &["keyboardInteractionProps"],
            &["isKeyboardActive"],
        );
    }

    if let Ok(clipboard) = clipboard_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "clipboard",
            clipboard,
            &["clipboardProps"],
            &["isClipboardDisabled"],
        );
    }

    if let Ok(long_press) = long_press_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "longPress",
            long_press,
            &["longPressProps"],
            &["isPressed", "isLongPressed"],
        );
    }

    if let Ok(move_interaction) = move_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "moveInteraction",
            move_interaction,
            &["moveProps"],
            &["isMoving", "xDelta", "yDelta"],
        );
    }

    if let Ok(file_trigger) = file_trigger_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "fileTrigger",
            file_trigger,
            &["fileTriggerProps"],
            &[
                "acceptedFileTypes",
                "allowsMultiple",
                "isDisabled",
                "isPressed",
            ],
        );
    }

    if let Ok(drop_zone) = drop_zone_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "dropZone",
            drop_zone,
            &["dropZoneProps"],
            &["label", "isDisabled"],
        );
    }

    if let Ok(drag) = drag_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "drag",
            drag,
            &["dragProps", "dragButtonProps"],
            &["isDragging"],
        );
    }

    if let Ok(drop_target) = drop_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "drop",
            drop_target,
            &["dropProps", "dropButtonProps"],
            &["label", "isDisabled", "isDropTarget"],
        );
    }

    if let Ok(group) = group_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "group",
            group,
            &["groupProps"],
            &[
                "label",
                "isDisabled",
                "isInvalid",
                "isReadOnly",
                "isHovered",
                "isFocused",
                "isFocusVisible",
                "isFocusWithin",
            ],
        );
    }

    if let Ok(virtualizer) = virtualizer_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "virtualizer",
            virtualizer,
            &["virtualizerProps"],
            &[
                "label",
                "layout",
                "orientation",
                "itemCount",
                "estimatedItemSize",
                "visibleStart",
                "visibleEnd",
                "overscan",
                "gap",
                "padding",
                "isScrolling",
                "isDisabled",
            ],
        );
    }

    if let Ok(form) = form_scope_value(props) {
        if let JsonValue::Object(form_scope) = &form {
            if let Some(form_props) = form_scope.get("formProps") {
                scope.insert("formProps".to_string(), form_props.clone());
            }
            for key in [
                "validationBehavior",
                "isDisabled",
                "isInvalid",
                "noValidate",
            ] {
                if let Some(value) = form_scope.get(key) {
                    scope
                        .entry(key.to_string())
                        .or_insert_with(|| value.clone());
                }
            }
        }
    }

    if let Ok(breadcrumbs) = breadcrumbs_scope_value(props) {
        if let JsonValue::Object(breadcrumbs_scope) = &breadcrumbs {
            if let Some(breadcrumbs_props) = breadcrumbs_scope.get("breadcrumbsProps") {
                scope.insert("breadcrumbsProps".to_string(), breadcrumbs_props.clone());
            }
        }
        scope.insert("breadcrumbs".to_string(), breadcrumbs);
    }

    if let Ok(landmark) = landmark_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "landmark",
            landmark,
            &["landmarkProps"],
            &["label", "landmarkKind"],
        );
    }

    if let Ok(focusable) = focusable_scope_value(props) {
        if let JsonValue::Object(focusable_scope) = &focusable {
            if let Some(focus_props) = focusable_scope.get("focusProps") {
                scope.insert("focusProps".to_string(), focus_props.clone());
            }
            if let Some(is_focused) = focusable_scope.get("isFocused") {
                scope.insert("isFocused".to_string(), is_focused.clone());
            }
        }
        scope.insert("focusable".to_string(), focusable);
    }

    if let Ok(focus_ring) = focus_ring_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "focusRing",
            focus_ring,
            &["focusRingProps"],
            &["isFocused", "isFocusVisible", "isFocusWithin"],
        );
    }

    if let Ok(focus_within) = focus_within_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "focusWithin",
            focus_within,
            &["focusWithinProps"],
            &["isFocusWithin"],
        );
    }

    if let Ok(focus_scope) = focus_scope_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "focusScope",
            focus_scope,
            &["focusScopeProps"],
            &["contain", "restoreFocus", "autoFocus", "isDisabled"],
        );
    }

    if let Ok(field) = field_scope_value(props) {
        if let JsonValue::Object(field_scope) = &field {
            if let Some(field_props) = field_scope.get("fieldProps") {
                scope.insert("fieldProps".to_string(), field_props.clone());
            }
            if let Some(label) = field_scope.get("label") {
                scope.insert("label".to_string(), label.clone());
            }
        }
        scope.insert("field".to_string(), field);
    }

    if let Ok(checkbox) = checkbox_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "checkbox",
            checkbox,
            &["checkboxProps"],
            &[
                "value",
                "isChecked",
                "isSelected",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(checkbox_group) = checkbox_group_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "checkboxGroup",
            checkbox_group,
            &["checkboxGroupProps"],
            &[
                "label",
                "selectedValue",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(separator) = separator_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "separator",
            separator,
            &["separatorProps"],
            &["orientation"],
        );
    }

    if let Ok(toolbar) = toolbar_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "toolbar",
            toolbar,
            &["toolbarProps"],
            &["label", "orientation", "isDisabled"],
        );
    }

    if let Ok(drop_indicator) = drop_indicator_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "dropIndicator",
            drop_indicator,
            &["dropIndicatorProps"],
            &["orientation", "isTarget"],
        );
    }

    if let Ok(selection_indicator) = selection_indicator_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "selectionIndicator",
            selection_indicator,
            &["selectionIndicatorProps"],
            &["label", "isSelected"],
        );
    }

    if let Ok(i18n) = i18n_scope_value(props) {
        if let JsonValue::Object(i18n_scope) = &i18n {
            if let Some(i18n_props) = i18n_scope.get("i18nProps") {
                scope.insert("i18nProps".to_string(), i18n_props.clone());
            }
            if let Some(locale) = i18n_scope.get("locale") {
                scope.insert("locale".to_string(), locale.clone());
            }
            if let Some(direction) = i18n_scope.get("direction") {
                scope.insert("direction".to_string(), direction.clone());
            }
            if let Some(is_rtl) = i18n_scope.get("isRtl") {
                scope.insert("isRtl".to_string(), is_rtl.clone());
            }
        }
        scope.insert("i18n".to_string(), i18n);
    }

    if let Ok(overlay) = overlay_scope_value(props) {
        if let JsonValue::Object(overlay_scope) = &overlay {
            if let Some(overlay_props) = overlay_scope.get("overlayProps") {
                scope.insert("overlayProps".to_string(), overlay_props.clone());
            }
            if let Some(overlay_trigger_props) = overlay_scope.get("overlayTriggerProps") {
                scope.insert(
                    "overlayTriggerProps".to_string(),
                    overlay_trigger_props.clone(),
                );
            }
            if let Some(is_open) = overlay_scope.get("isOpen") {
                scope.insert("isOpen".to_string(), is_open.clone());
            }
        }
        scope.insert("overlay".to_string(), overlay);
    }

    if let Ok(menu) = menu_scope_value(props) {
        if let JsonValue::Object(menu_scope) = &menu {
            if let Some(menu_props) = menu_scope.get("menuProps") {
                scope.insert("menuProps".to_string(), menu_props.clone());
            }
            if let Some(label) = menu_scope.get("label") {
                scope.entry("label".to_string()).or_insert(label.clone());
            }
        }
        scope.insert("menu".to_string(), menu);
    }

    if let Ok(menu_item) = menu_item_scope_value(props) {
        if let JsonValue::Object(menu_item_scope) = &menu_item {
            if let Some(menu_item_props) = menu_item_scope.get("menuItemProps") {
                scope.insert("menuItemProps".to_string(), menu_item_props.clone());
            }
            if let Some(is_disabled) = menu_item_scope.get("isDisabled") {
                scope
                    .entry("isDisabled".to_string())
                    .or_insert(is_disabled.clone());
            }
            if let Some(is_selected) = menu_item_scope.get("isSelected") {
                scope
                    .entry("isSelected".to_string())
                    .or_insert(is_selected.clone());
            }
            if let Some(text_value) = menu_item_scope.get("textValue") {
                scope
                    .entry("textValue".to_string())
                    .or_insert(text_value.clone());
            }
        }
        scope.insert("menuItem".to_string(), menu_item);
    }

    if let Ok(submenu_trigger) = submenu_trigger_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "submenuTrigger",
            submenu_trigger,
            &["submenuTriggerProps"],
            &["isDisabled", "isPressed", "isOpen"],
        );
    }

    if let Ok(load_more_item) = load_more_item_scope_value(props) {
        if let JsonValue::Object(load_more_item_scope) = &load_more_item {
            if let Some(load_more_item_props) = load_more_item_scope.get("loadMoreItemProps") {
                scope.insert(
                    "loadMoreItemProps".to_string(),
                    load_more_item_props.clone(),
                );
            }
            if let Some(label) = load_more_item_scope.get("label") {
                scope.entry("label".to_string()).or_insert(label.clone());
            }
            if let Some(text_value) = load_more_item_scope.get("textValue") {
                scope
                    .entry("textValue".to_string())
                    .or_insert(text_value.clone());
            }
            if let Some(action_value) = load_more_item_scope.get("actionValue") {
                scope
                    .entry("actionValue".to_string())
                    .or_insert(action_value.clone());
            }
            if let Some(action_payload) = load_more_item_scope.get("actionPayload") {
                scope
                    .entry("actionPayload".to_string())
                    .or_insert(action_payload.clone());
            }
            if let Some(is_loading) = load_more_item_scope.get("isLoading") {
                scope.insert("isLoading".to_string(), is_loading.clone());
            }
            if let Some(is_disabled) = load_more_item_scope.get("isDisabled") {
                scope
                    .entry("isDisabled".to_string())
                    .or_insert(is_disabled.clone());
            }
        }
        scope.insert("loadMoreItem".to_string(), load_more_item);
    }

    if let Ok(collection) = collection_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "collection",
            collection,
            &["collectionProps"],
            &["label", "itemCount", "isEmpty", "isDisabled"],
        );
    }

    if let Ok(collection_section) = collection_section_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "collectionSection",
            collection_section,
            &["collectionSectionProps"],
            &["label", "collectionKind", "isDisabled"],
        );
    }

    if let Ok(collection_item) = collection_item_scope_value(props) {
        if let JsonValue::Object(collection_item_scope) = &collection_item {
            if let Some(collection_item_props) = collection_item_scope.get("collectionItemProps") {
                scope.insert(
                    "collectionItemProps".to_string(),
                    collection_item_props.clone(),
                );
            }
            if let Some(value) = collection_item_scope.get("value") {
                scope.entry("value".to_string()).or_insert(value.clone());
            }
            if let Some(text_value) = collection_item_scope.get("textValue") {
                scope
                    .entry("textValue".to_string())
                    .or_insert(text_value.clone());
            }
            if let Some(is_selected) = collection_item_scope.get("isSelected") {
                scope
                    .entry("isSelected".to_string())
                    .or_insert(is_selected.clone());
            }
            if let Some(is_disabled) = collection_item_scope.get("isDisabled") {
                scope
                    .entry("isDisabled".to_string())
                    .or_insert(is_disabled.clone());
            }
            if let Some(is_expanded) = collection_item_scope.get("isExpanded") {
                scope
                    .entry("isExpanded".to_string())
                    .or_insert(is_expanded.clone());
            }
        }
        scope.insert("collectionItem".to_string(), collection_item);
    }

    if let Ok(tree_item) = tree_item_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "treeItem",
            tree_item,
            &["treeItemProps"],
            &[
                "value",
                "textValue",
                "isSelected",
                "isDisabled",
                "isExpanded",
                "hasChildItems",
            ],
        );
    }

    if let Ok(radio_group) = radio_group_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "radioGroup",
            radio_group,
            &["radioGroupProps"],
            &[
                "label",
                "selectedValue",
                "selectionMode",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(radio) = radio_scope_value(props) {
        if let JsonValue::Object(radio_scope) = &radio {
            if let Some(radio_props) = radio_scope.get("radioProps") {
                scope.insert("radioProps".to_string(), radio_props.clone());
            }
            if let Some(value) = radio_scope.get("value") {
                scope.entry("value".to_string()).or_insert(value.clone());
            }
            if let Some(text_value) = radio_scope.get("textValue") {
                scope
                    .entry("textValue".to_string())
                    .or_insert(text_value.clone());
            }
            if let Some(is_selected) = radio_scope.get("isSelected") {
                scope
                    .entry("isSelected".to_string())
                    .or_insert(is_selected.clone());
            }
            if let Some(is_checked) = radio_scope.get("isChecked") {
                scope
                    .entry("isChecked".to_string())
                    .or_insert(is_checked.clone());
            }
            if let Some(is_disabled) = radio_scope.get("isDisabled") {
                scope
                    .entry("isDisabled".to_string())
                    .or_insert(is_disabled.clone());
            }
        }
        scope.insert("radio".to_string(), radio);
    }

    if let Ok(tab_list) = tab_list_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "tabList",
            tab_list,
            &["tabListProps"],
            &["label", "orientation", "isDisabled"],
        );
    }

    if let Ok(tab) = tab_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "tab",
            tab,
            &["tabProps"],
            &["value", "textValue", "isSelected", "isDisabled"],
        );
    }

    if let Ok(tab_panel) = tab_panel_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "tabPanel",
            tab_panel,
            &["tabPanelProps"],
            &["value"],
        );
    }

    if let Ok(table) = table_scope_value(props) {
        if let JsonValue::Object(table_scope) = &table {
            if let Some(table_props) = table_scope.get("tableProps") {
                scope.insert("tableProps".to_string(), table_props.clone());
            }
            if let Some(label) = table_scope.get("label") {
                scope.entry("label".to_string()).or_insert(label.clone());
            }
        }
        scope.insert("table".to_string(), table);
    }

    if let Ok(table_section) = table_section_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "tableSection",
            table_section,
            &["tableSectionProps"],
            &["sectionKind", "label"],
        );
    }

    if let Ok(table_row) = table_row_scope_value(props) {
        if let JsonValue::Object(table_row_scope) = &table_row {
            if let Some(table_row_props) = table_row_scope.get("tableRowProps") {
                scope.insert("tableRowProps".to_string(), table_row_props.clone());
            }
            if let Some(is_selected) = table_row_scope.get("isSelected") {
                scope
                    .entry("isSelected".to_string())
                    .or_insert(is_selected.clone());
            }
            if let Some(is_disabled) = table_row_scope.get("isDisabled") {
                scope
                    .entry("isDisabled".to_string())
                    .or_insert(is_disabled.clone());
            }
        }
        scope.insert("tableRow".to_string(), table_row);
    }

    if let Ok(table_cell) = table_cell_scope_value(props) {
        if let JsonValue::Object(table_cell_scope) = &table_cell {
            if let Some(table_cell_props) = table_cell_scope.get("tableCellProps") {
                scope.insert("tableCellProps".to_string(), table_cell_props.clone());
            }
            if let Some(label) = table_cell_scope.get("label") {
                scope.entry("label".to_string()).or_insert(label.clone());
            }
            if let Some(text_value) = table_cell_scope.get("textValue") {
                scope
                    .entry("textValue".to_string())
                    .or_insert(text_value.clone());
            }
        }
        scope.insert("tableCell".to_string(), table_cell);
    }

    if let Ok(table_column) = table_column_scope_value(props) {
        if let JsonValue::Object(table_column_scope) = &table_column {
            if let Some(table_column_props) = table_column_scope.get("tableColumnProps") {
                scope.insert("tableColumnProps".to_string(), table_column_props.clone());
            }
            if let Some(label) = table_column_scope.get("label") {
                scope.entry("label".to_string()).or_insert(label.clone());
            }
            if let Some(text_value) = table_column_scope.get("textValue") {
                scope
                    .entry("textValue".to_string())
                    .or_insert(text_value.clone());
            }
        }
        scope.insert("tableColumn".to_string(), table_column);
    }

    if let Ok(table_caption) = table_caption_scope_value(props) {
        if let JsonValue::Object(table_caption_scope) = &table_caption {
            if let Some(table_caption_props) = table_caption_scope.get("tableCaptionProps") {
                scope.insert("tableCaptionProps".to_string(), table_caption_props.clone());
            }
            if let Some(label) = table_caption_scope.get("label") {
                scope.entry("label".to_string()).or_insert(label.clone());
            }
            if let Some(text_value) = table_caption_scope.get("textValue") {
                scope
                    .entry("textValue".to_string())
                    .or_insert(text_value.clone());
            }
        }
        scope.insert("tableCaption".to_string(), table_caption);
    }

    if let Ok(text) = text_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "text",
            text,
            &["textProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(label) = label_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "labelText",
            label,
            &["labelProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(description) = description_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "descriptionText",
            description,
            &["descriptionProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(field_error) = field_error_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "fieldError",
            field_error,
            &["fieldErrorProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(legend) = legend_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "legend",
            legend,
            &["legendProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(visually_hidden) = visually_hidden_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "visuallyHidden",
            visually_hidden,
            &["visuallyHiddenProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(keyboard) = keyboard_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "keyboard",
            keyboard,
            &["keyboardProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(list_box_header) = list_box_header_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "listBoxHeader",
            list_box_header,
            &["listBoxHeaderProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(grid_list_header) = grid_list_header_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "gridListHeader",
            grid_list_header,
            &["gridListHeaderProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(tree_header) = tree_header_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "treeHeader",
            tree_header,
            &["treeHeaderProps"],
            &["label", "textValue"],
        );
    }

    if let Ok(heading) = heading_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "heading",
            heading,
            &["headingProps"],
            &["label", "textValue", "level"],
        );
    }

    if let Ok(date_field) = date_field_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "dateField",
            date_field,
            &["dateFieldProps", "dateFieldInputProps"],
            &[
                "label",
                "value",
                "placeholder",
                "granularity",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(time_field) = time_field_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "timeField",
            time_field,
            &["timeFieldProps", "timeFieldInputProps"],
            &[
                "label",
                "value",
                "placeholder",
                "granularity",
                "hourCycle",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(date_input) = date_input_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "dateInput",
            date_input,
            &["dateInputProps"],
            &["label", "value", "isDisabled", "isInvalid", "isReadOnly"],
        );
    }

    if let Ok(date_segment) = date_segment_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "dateSegment",
            date_segment,
            &["dateSegmentProps"],
            &[
                "segmentType",
                "value",
                "textValue",
                "placeholder",
                "isPlaceholder",
                "isDisabled",
                "isInvalid",
            ],
        );
    }

    if let Ok(calendar) = calendar_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "calendar",
            calendar,
            &["calendarProps"],
            &["label", "value", "isDisabled", "isInvalid", "isReadOnly"],
        );
    }

    if let Ok(range_calendar) = range_calendar_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "rangeCalendar",
            range_calendar,
            &["rangeCalendarProps"],
            &[
                "label",
                "startValue",
                "endValue",
                "isDisabled",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(calendar_cell) = calendar_cell_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "calendarCell",
            calendar_cell,
            &["calendarCellProps"],
            &[
                "value",
                "textValue",
                "isSelected",
                "isDisabled",
                "isUnavailable",
                "isOutsideMonth",
                "isToday",
                "isPressed",
            ],
        );
    }

    if let Ok(date_picker) = date_picker_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "datePicker",
            date_picker,
            &[
                "datePickerProps",
                "datePickerInputProps",
                "datePickerTriggerProps",
            ],
            &[
                "label",
                "value",
                "placeholder",
                "isOpen",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(date_range_picker) = date_range_picker_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "dateRangePicker",
            date_range_picker,
            &[
                "dateRangePickerProps",
                "dateRangePickerStartInputProps",
                "dateRangePickerEndInputProps",
                "dateRangePickerTriggerProps",
            ],
            &[
                "label",
                "startValue",
                "endValue",
                "placeholder",
                "isOpen",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(color_field) = color_field_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorField",
            color_field,
            &["colorFieldProps", "colorFieldInputProps"],
            &[
                "label",
                "value",
                "placeholder",
                "colorSpace",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(color_picker) = color_picker_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorPicker",
            color_picker,
            &["colorPickerProps"],
            &["label", "value", "isDisabled", "isReadOnly"],
        );
    }

    if let Ok(color_area) = color_area_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorArea",
            color_area,
            &["colorAreaProps"],
            &[
                "label",
                "value",
                "xChannel",
                "yChannel",
                "xValue",
                "yValue",
                "isDisabled",
                "isReadOnly",
            ],
        );
    }

    if let Ok(color_slider) = color_slider_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorSlider",
            color_slider,
            &["colorSliderProps"],
            &[
                "label",
                "channel",
                "valueNumber",
                "minValue",
                "maxValue",
                "stepValue",
                "valuePercent",
                "isDisabled",
                "isReadOnly",
            ],
        );
    }

    if let Ok(color_wheel) = color_wheel_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorWheel",
            color_wheel,
            &["colorWheelProps"],
            &[
                "label",
                "channel",
                "valueNumber",
                "minValue",
                "maxValue",
                "stepValue",
                "valuePercent",
                "isDisabled",
                "isReadOnly",
            ],
        );
    }

    if let Ok(color_swatch_picker) = color_swatch_picker_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorSwatchPicker",
            color_swatch_picker,
            &["colorSwatchPickerProps"],
            &[
                "label",
                "selectedValue",
                "selectionMode",
                "isDisabled",
                "isReadOnly",
            ],
        );
    }

    if let Ok(color_swatch_picker_item) = color_swatch_picker_item_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorSwatchPickerItem",
            color_swatch_picker_item,
            &["colorSwatchPickerItemProps"],
            &["value", "textValue", "isSelected", "isDisabled"],
        );
    }

    if let Ok(color_swatch) = color_swatch_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorSwatch",
            color_swatch,
            &["colorSwatchProps"],
            &["label", "value", "isDisabled"],
        );
    }

    if let Ok(color_thumb) = color_thumb_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "colorThumb",
            color_thumb,
            &["colorThumbProps"],
            &[
                "value",
                "xValue",
                "yValue",
                "isPressed",
                "isDragging",
                "isDisabled",
            ],
        );
    }

    if let Ok(combo_box) = combo_box_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "comboBox",
            combo_box,
            &[
                "comboBoxProps",
                "comboBoxInputProps",
                "comboBoxTriggerProps",
            ],
            &[
                "label",
                "selectedValue",
                "inputValue",
                "placeholder",
                "selectionMode",
                "isOpen",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(autocomplete) = autocomplete_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "autocomplete",
            autocomplete,
            &["autocompleteProps", "autocompleteInputProps"],
            &[
                "label",
                "selectedValue",
                "inputValue",
                "placeholder",
                "selectionMode",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(select) = select_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "select",
            select,
            &["selectProps", "selectTriggerProps"],
            &[
                "label",
                "selectedValue",
                "placeholder",
                "selectionMode",
                "isOpen",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(select_value) = select_display_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "selectDisplay",
            select_value,
            &["selectValueProps"],
            &["value", "displayValue", "isPlaceholder"],
        );
    }

    if let Ok(combo_box_value) = combo_box_display_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "comboBoxDisplay",
            combo_box_value,
            &["comboBoxValueProps"],
            &["value", "displayValue", "isPlaceholder"],
        );
    }

    if let Ok(selection) = selection_scope_value(props) {
        if let JsonValue::Object(selection_scope) = &selection {
            if let Some(selection_props) = selection_scope.get("selectionProps") {
                scope.insert("selectionProps".to_string(), selection_props.clone());
            }
            if let Some(selected_value) = selection_scope.get("selectedValue") {
                scope.insert("selectedValue".to_string(), selected_value.clone());
            }
            if let Some(selection_mode) = selection_scope.get("selectionMode") {
                scope.insert("selectionMode".to_string(), selection_mode.clone());
            }
        }
        scope.insert("selection".to_string(), selection);
    }

    if let Ok(tree) = tree_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "tree",
            tree,
            &["treeProps"],
            &[
                "label",
                "selectedValue",
                "expandedKeys",
                "selectionMode",
                "isDisabled",
                "isReadOnly",
            ],
        );
    }

    if let Ok(disclosure_group) = disclosure_group_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "disclosureGroup",
            disclosure_group,
            &["disclosureGroupProps"],
            &[
                "label",
                "expandedKeys",
                "allowsMultipleExpanded",
                "isDisabled",
            ],
        );
    }

    if let Ok(disclosure) = disclosure_scope_value(props) {
        if let JsonValue::Object(disclosure_scope) = &disclosure {
            if let Some(disclosure_props) = disclosure_scope.get("disclosureProps") {
                scope.insert("disclosureProps".to_string(), disclosure_props.clone());
            }
            if let Some(disclosure_trigger_props) = disclosure_scope.get("disclosureTriggerProps") {
                scope.insert(
                    "disclosureTriggerProps".to_string(),
                    disclosure_trigger_props.clone(),
                );
            }
            if let Some(disclosure_panel_props) = disclosure_scope.get("disclosurePanelProps") {
                scope.insert(
                    "disclosurePanelProps".to_string(),
                    disclosure_panel_props.clone(),
                );
            }
            if let Some(is_expanded) = disclosure_scope.get("isExpanded") {
                scope.insert("isExpanded".to_string(), is_expanded.clone());
            }
        }
        scope.insert("disclosure".to_string(), disclosure);
    }

    if let Ok(text_field) = text_field_scope_value(props) {
        if let JsonValue::Object(text_field_scope) = &text_field {
            if let Some(input_props) = text_field_scope.get("inputProps") {
                scope.insert("inputProps".to_string(), input_props.clone());
            }
            if let Some(field_props) = text_field_scope.get("fieldProps") {
                scope.insert("fieldProps".to_string(), field_props.clone());
            }
        }
        scope.insert("textField".to_string(), text_field);
    }

    if let Ok(toast) = toast_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "toast",
            toast,
            &["toastProps"],
            &["title", "description"],
        );
    }

    if let Ok(toast_region) = toast_region_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "toastRegion",
            toast_region,
            &["toastRegionProps"],
            &["label"],
        );
    }

    insert_optional_string(&mut scope, "label", props.label.as_ref());
    insert_optional_string(&mut scope, "textValue", props.text_value.as_ref());
    insert_optional_string(&mut scope, "value", props.value.as_ref());
    insert_optional_string(&mut scope, "placeholder", props.placeholder.as_ref());
    insert_optional_string(&mut scope, "action", props.action.as_ref());
    insert_optional_string(&mut scope, "aria-label", props.aria_label.as_ref());
    insert_optional_string(&mut scope, "ariaLabel", props.aria_label.as_ref());
    insert_optional_string(&mut scope, "id", props.id.as_ref());
    insert_optional_string(&mut scope, "className", props.class_name.as_ref());
    insert_optional_string(&mut scope, "name", props.name.as_ref());
    insert_optional_string(&mut scope, "form", props.form.as_ref());
    insert_optional_string(&mut scope, "type", props.input_type.as_ref());
    insert_optional_string(&mut scope, "inputType", props.input_type.as_ref());
    insert_optional_string(&mut scope, "href", props.href.as_ref());
    insert_optional_string(&mut scope, "src", props.src.as_ref());
    insert_optional_string(&mut scope, "alt", props.alt.as_ref());

    scope.insert("isDisabled".to_string(), JsonValue::Bool(props.is_disabled));
    scope.insert("disabled".to_string(), JsonValue::Bool(props.is_disabled));
    scope.insert("isRequired".to_string(), JsonValue::Bool(props.is_required));
    scope.insert("required".to_string(), JsonValue::Bool(props.is_required));
    scope.insert("isInvalid".to_string(), JsonValue::Bool(props.is_invalid));
    scope.insert("invalid".to_string(), JsonValue::Bool(props.is_invalid));
    scope.insert(
        "isReadOnly".to_string(),
        JsonValue::Bool(props.is_read_only),
    );
    scope.insert("readOnly".to_string(), JsonValue::Bool(props.is_read_only));
    scope.insert("isSelected".to_string(), JsonValue::Bool(props.is_selected));
    scope.insert("selected".to_string(), JsonValue::Bool(props.is_selected));

    insert_optional_bool(&mut scope, "isChecked", props.is_checked);
    insert_optional_bool(&mut scope, "checked", props.is_checked);
    insert_optional_bool(&mut scope, "isExpanded", props.is_expanded);
    insert_optional_bool(&mut scope, "expanded", props.is_expanded);

    if let Ok(switch) = switch_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "switch",
            switch,
            &["switchProps"],
            &[
                "isChecked",
                "isSelected",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(toggle) = toggle_scope_value(props) {
        if let JsonValue::Object(toggle_scope) = &toggle {
            if let Some(toggle_props) = toggle_scope.get("toggleProps") {
                scope.insert("toggleProps".to_string(), toggle_props.clone());
            }
            if let Some(is_selected) = toggle_scope.get("isSelected") {
                scope.insert("isSelected".to_string(), is_selected.clone());
            }
            if let Some(is_checked) = toggle_scope.get("isChecked") {
                scope.insert("isChecked".to_string(), is_checked.clone());
                scope.insert("checked".to_string(), is_checked.clone());
            }
        }
        scope.insert("toggle".to_string(), toggle);
    }

    if let Ok(toggle_button) = toggle_button_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "toggleButton",
            toggle_button,
            &["toggleButtonProps"],
            &["isSelected", "isPressed", "isDisabled"],
        );
    }

    if let Ok(toggle_button_group) = toggle_button_group_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "toggleButtonGroup",
            toggle_button_group,
            &["toggleButtonGroupProps"],
            &[
                "label",
                "selectedValue",
                "orientation",
                "selectionMode",
                "isDisabled",
                "isReadOnly",
            ],
        );
    }

    insert_optional_number(&mut scope, "min", props.min_value);
    insert_optional_number(&mut scope, "minValue", props.min_value);
    insert_optional_number(&mut scope, "max", props.max_value);
    insert_optional_number(&mut scope, "maxValue", props.max_value);
    insert_optional_number(&mut scope, "step", props.step_value);
    insert_optional_number(&mut scope, "stepValue", props.step_value);
    insert_optional_number(&mut scope, "valueNumber", props.value_number);

    if let Ok(range) = range_scope_value(props) {
        if let JsonValue::Object(range_scope) = &range {
            if let Some(range_props) = range_scope.get("rangeProps") {
                scope.insert("rangeProps".to_string(), range_props.clone());
            }
            if let Some(range_input_props) = range_scope.get("rangeInputProps") {
                scope.insert("rangeInputProps".to_string(), range_input_props.clone());
            }
        }
        scope.insert("range".to_string(), range);
    }

    if let Ok(number_field) = number_field_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "numberField",
            number_field,
            &["numberFieldProps", "numberFieldInputProps"],
            &[
                "label",
                "valueNumber",
                "placeholder",
                "minValue",
                "maxValue",
                "stepValue",
                "valuePercent",
                "isDisabled",
                "isRequired",
                "isInvalid",
                "isReadOnly",
            ],
        );
    }

    if let Ok(slider_track) = slider_track_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "sliderTrack",
            slider_track,
            &["sliderTrackProps"],
            &["orientation", "isDisabled"],
        );
    }

    if let Ok(slider_fill) = slider_fill_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "sliderFill",
            slider_fill,
            &["sliderFillProps"],
            &["orientation", "valueNumber", "isDisabled"],
        );
    }

    if let Ok(slider_output) = slider_output_scope_value(props) {
        insert_hook_scope(
            &mut scope,
            "sliderOutput",
            slider_output,
            &["sliderOutputProps"],
            &["label", "value", "valueNumber"],
        );
    }

    if let Some(orientation) = props.orientation {
        let value = match orientation {
            CompiledOrientation::Horizontal => "horizontal",
            CompiledOrientation::Vertical => "vertical",
        };
        scope.insert(
            "orientation".to_string(),
            JsonValue::String(value.to_string()),
        );
    }

    for (name, value) in &props.attributes {
        scope
            .entry(name.clone())
            .or_insert_with(|| JsonValue::String(value.clone()));
    }
    for (name, value) in &props.events {
        scope.insert(name.clone(), JsonValue::String(value.clone()));
    }

    JsonValue::Object(scope)
}

fn press_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_press_value(
        UsePressProps::new()
            .on_press(non_empty_prop_action(
                props.events.get("onPress").or(props.action.as_ref()),
            ))
            .on_press_start(non_empty_prop_action(props.events.get("onPressStart")))
            .on_press_end(non_empty_prop_action(props.events.get("onPressEnd")))
            .on_press_up(non_empty_prop_action(props.events.get("onPressUp")))
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .action_payload(action_payload_value(props))
            .disabled(props.is_disabled)
            .pressed(bool_attribute_value(props, &["isPressed", "pressed"]).unwrap_or(false)),
    )
}

fn button_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_button_value(
        UseButtonProps::new()
            .on_press(non_empty_prop_action(
                props.events.get("onPress").or(props.action.as_ref()),
            ))
            .on_press_start(non_empty_prop_action(props.events.get("onPressStart")))
            .on_press_end(non_empty_prop_action(props.events.get("onPressEnd")))
            .on_press_up(non_empty_prop_action(props.events.get("onPressUp")))
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .action_payload(action_payload_value(props))
            .disabled(props.is_disabled)
            .pressed(
                bool_attribute_value(props, &["isPressed", "pressed", "data-pressed"])
                    .unwrap_or(false),
            ),
    )
}

fn link_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_link_value(
        UseLinkProps::new()
            .href(
                props
                    .href
                    .clone()
                    .or_else(|| non_empty_attribute(props, &["href"])),
            )
            .on_press(non_empty_prop_action(
                props.events.get("onPress").or(props.action.as_ref()),
            ))
            .on_press_start(non_empty_prop_action(props.events.get("onPressStart")))
            .on_press_end(non_empty_prop_action(props.events.get("onPressEnd")))
            .on_press_up(non_empty_prop_action(props.events.get("onPressUp")))
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .action_payload(action_payload_value(props))
            .disabled(props.is_disabled)
            .pressed(
                bool_attribute_value(props, &["isPressed", "pressed", "data-pressed"])
                    .unwrap_or(false),
            ),
    )
}

fn hover_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_hover_value(
        UseHoverProps::new()
            .on_hover_start(non_empty_prop_action(props.events.get("onHoverStart")))
            .on_hover_end(non_empty_prop_action(props.events.get("onHoverEnd")))
            .on_hover_change(non_empty_prop_action(props.events.get("onHoverChange")))
            .disabled(props.is_disabled)
            .hovered(
                bool_attribute_value(props, &["isHovered", "hovered", "data-hovered"])
                    .unwrap_or(false),
            ),
    )
}

fn keyboard_interaction_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_keyboard_interaction_value(
        UseKeyboardInteractionProps::new()
            .on_key_down(non_empty_prop_action(props.events.get("onKeyDown")))
            .on_key_up(non_empty_prop_action(props.events.get("onKeyUp")))
            .disabled(props.is_disabled)
            .keyboard_active(
                bool_attribute_value(
                    props,
                    &["isKeyboardActive", "keyboardActive", "data-keyboard-active"],
                )
                .unwrap_or(false),
            )
            .tab_index(i32_attribute_value(props, &["tabIndex", "tabindex"]).unwrap_or(0)),
    )
}

fn clipboard_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_clipboard_value(
        UseClipboardProps::new()
            .on_copy(non_empty_prop_action(props.events.get("onCopy")))
            .on_cut(non_empty_prop_action(props.events.get("onCut")))
            .on_paste(non_empty_prop_action(props.events.get("onPaste")))
            .copy_value(non_empty_attribute(
                props,
                &["copyValue", "data-copy-value"],
            ))
            .copy_mime_type(non_empty_attribute(
                props,
                &["copyMimeType", "data-copy-mime-type"],
            ))
            .accepted_mime_types(non_empty_attribute(
                props,
                &["acceptedMimeTypes", "data-accepted-mime-types"],
            ))
            .disabled(props.is_disabled),
    )
}

fn long_press_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_long_press_value(
        UseLongPressProps::new()
            .on_long_press_start(non_empty_prop_action(props.events.get("onLongPressStart")))
            .on_long_press_end(non_empty_prop_action(props.events.get("onLongPressEnd")))
            .on_long_press(non_empty_prop_action(
                props.events.get("onLongPress").or(props.action.as_ref()),
            ))
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .action_payload(action_payload_value(props))
            .accessibility_description(non_empty_attribute(
                props,
                &["accessibilityDescription", "aria-description"],
            ))
            .threshold(
                usize_attribute_value(props, &["threshold", "data-long-press-threshold"])
                    .unwrap_or(500)
                    .min(60_000) as u64,
            )
            .disabled(props.is_disabled)
            .pressed(bool_attribute_value(props, &["isPressed", "pressed"]).unwrap_or(false))
            .long_pressed(
                bool_attribute_value(
                    props,
                    &["isLongPressed", "longPressed", "data-long-pressed"],
                )
                .unwrap_or(false),
            ),
    )
}

fn move_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_move_value(
        UseMoveProps::new()
            .on_move_start(non_empty_prop_action(props.events.get("onMoveStart")))
            .on_move(non_empty_prop_action(props.events.get("onMove")))
            .on_move_end(non_empty_prop_action(props.events.get("onMoveEnd")))
            .disabled(props.is_disabled)
            .moving(
                bool_attribute_value(props, &["isMoving", "moving", "data-moving"])
                    .unwrap_or(false),
            )
            .x_delta(number_attribute_value(props, &["xDelta", "data-x-delta"]).unwrap_or(0.0))
            .y_delta(number_attribute_value(props, &["yDelta", "data-y-delta"]).unwrap_or(0.0)),
    )
}

fn file_trigger_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_file_trigger_value(
        UseFileTriggerProps::new()
            .on_press(non_empty_prop_action(
                props.events.get("onPress").or(props.action.as_ref()),
            ))
            .on_select(non_empty_prop_action(props.events.get("onSelect")))
            .accepted_file_types(non_empty_attribute(
                props,
                &["acceptedFileTypes", "accept", "data-accepted-file-types"],
            ))
            .allows_multiple(
                bool_attribute_value(props, &["allowsMultiple", "multiple"]).unwrap_or(false),
            )
            .disabled(props.is_disabled)
            .pressed(bool_attribute_value(props, &["isPressed", "pressed"]).unwrap_or(false)),
    )
}

fn drop_zone_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_drop_zone_value(
        UseDropZoneProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .on_drop(non_empty_prop_action(props.events.get("onDrop")))
            .on_drag_enter(non_empty_prop_action(props.events.get("onDragEnter")))
            .on_drag_leave(non_empty_prop_action(props.events.get("onDragLeave")))
            .disabled(props.is_disabled),
    )
}

fn drag_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_drag_value(
        UseDragProps::new()
            .on_drag_start(non_empty_prop_action(props.events.get("onDragStart")))
            .on_drag_move(non_empty_prop_action(props.events.get("onDragMove")))
            .on_drag_end(non_empty_prop_action(props.events.get("onDragEnd")))
            .drag_type(non_empty_attribute(props, &["dragType", "data-drag-type"]))
            .drag_value(non_empty_attribute(
                props,
                &["dragValue", "data-drag-value"],
            ))
            .disabled(props.is_disabled)
            .dragging(
                bool_attribute_value(props, &["isDragging", "dragging", "data-dragging"])
                    .unwrap_or(false),
            ),
    )
}

fn drop_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_drop_value(
        UseDropProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .on_drop(non_empty_prop_action(props.events.get("onDrop")))
            .on_drop_enter(non_empty_prop_action(props.events.get("onDropEnter")))
            .on_drop_exit(non_empty_prop_action(props.events.get("onDropExit")))
            .on_drop_move(non_empty_prop_action(props.events.get("onDropMove")))
            .accepted_drag_types(non_empty_attribute(
                props,
                &["acceptedDragTypes", "data-accepted-drag-types"],
            ))
            .drop_operation(non_empty_attribute(
                props,
                &["dropOperation", "dropEffect", "data-drop-operation"],
            ))
            .disabled(props.is_disabled)
            .drop_target(
                bool_attribute_value(props, &["isDropTarget", "dropTarget", "data-drop-target"])
                    .unwrap_or(false),
            ),
    )
}

fn group_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_group_value(
        UseGroupProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .on_hover_start(non_empty_prop_action(props.events.get("onHoverStart")))
            .on_hover_end(non_empty_prop_action(props.events.get("onHoverEnd")))
            .on_hover_change(non_empty_prop_action(props.events.get("onHoverChange")))
            .on_focus(non_empty_prop_action(props.events.get("onFocus")))
            .on_blur(non_empty_prop_action(props.events.get("onBlur")))
            .on_focus_change(non_empty_prop_action(props.events.get("onFocusChange")))
            .on_focus_within(non_empty_prop_action(props.events.get("onFocusWithin")))
            .on_blur_within(non_empty_prop_action(props.events.get("onBlurWithin")))
            .on_focus_within_change(non_empty_prop_action(
                props.events.get("onFocusWithinChange"),
            ))
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .hovered(
                bool_attribute_value(props, &["isHovered", "hovered", "data-hovered"])
                    .unwrap_or(false),
            )
            .focused(
                bool_attribute_value(props, &["isFocused", "focused", "data-focused"])
                    .unwrap_or(false),
            )
            .focus_visible(
                bool_attribute_value(props, &["isFocusVisible", "data-focus-visible"])
                    .unwrap_or(false),
            )
            .focus_within(
                bool_attribute_value(props, &["isFocusWithin", "data-focus-within"])
                    .unwrap_or(false),
            )
            .auto_focus(bool_attribute_value(props, &["autoFocus", "autofocus"]).unwrap_or(false))
            .tab_index(i32_attribute_value(props, &["tabIndex", "tabindex"]).unwrap_or(0)),
    )
}

fn virtualizer_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_virtualizer_value(
        UseVirtualizerProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .layout(non_empty_attribute(
                props,
                &["layout", "virtualizerLayout", "data-layout"],
            ))
            .orientation(
                non_empty_attribute(props, &["orientation", "data-orientation"])
                    .or_else(|| orientation_attribute_value(props).map(ToOwned::to_owned)),
            )
            .item_count(
                usize_attribute_value(props, &["itemCount", "item-count", "data-item-count"])
                    .unwrap_or(0),
            )
            .estimated_item_size(
                u32_attribute_value(
                    props,
                    &[
                        "estimatedItemSize",
                        "estimated-item-size",
                        "data-estimated-item-size",
                    ],
                )
                .unwrap_or(40),
            )
            .visible_start(
                usize_attribute_value(
                    props,
                    &["visibleStart", "visible-start", "data-visible-start"],
                )
                .unwrap_or(0),
            )
            .visible_end(
                usize_attribute_value(props, &["visibleEnd", "visible-end", "data-visible-end"])
                    .unwrap_or(0),
            )
            .overscan(usize_attribute_value(props, &["overscan", "data-overscan"]).unwrap_or(2))
            .gap(u32_attribute_value(props, &["gap", "data-gap"]).unwrap_or(0))
            .padding(u32_attribute_value(props, &["padding", "data-padding"]).unwrap_or(0))
            .scrolling(
                bool_attribute_value(props, &["isScrolling", "scrolling", "data-scrolling"])
                    .unwrap_or(false),
            )
            .disabled(props.is_disabled)
            .tab_index(i32_attribute_value(props, &["tabIndex", "tabindex"]).unwrap_or(0)),
    )
}

fn form_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_form_value(
        UseFormProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .on_submit(non_empty_prop_action(props.events.get("onSubmit")))
            .on_reset(non_empty_prop_action(props.events.get("onReset")))
            .on_invalid(non_empty_prop_action(props.events.get("onInvalid")))
            .validation_behavior(non_empty_attribute(
                props,
                &["validationBehavior", "data-validation-behavior"],
            ))
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .no_validate(
                bool_attribute_value(props, &["noValidate", "novalidate"]).unwrap_or(false),
            ),
    )
}

fn breadcrumbs_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_breadcrumbs_value(
        UseBreadcrumbsProps::new().label(props.label.clone().or_else(|| props.aria_label.clone())),
    )
}

fn landmark_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_landmark_value(
        UseLandmarkProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .kind(non_empty_attribute(
                props,
                &["landmarkKind", "data-landmark-kind", "role"],
            )),
    )
}

fn focusable_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_focusable_value(
        UseFocusableProps::new()
            .on_focus(non_empty_prop_action(props.events.get("onFocus")))
            .on_blur(non_empty_prop_action(props.events.get("onBlur")))
            .on_focus_change(non_empty_prop_action(props.events.get("onFocusChange")))
            .disabled(props.is_disabled)
            .focused(
                bool_attribute_value(props, &["isFocused", "focused", "data-focused"])
                    .unwrap_or(false),
            )
            .auto_focus(bool_attribute_value(props, &["autoFocus", "autofocus"]).unwrap_or(false))
            .tab_index(i32_attribute_value(props, &["tabIndex", "tabindex"]).unwrap_or(0)),
    )
}

fn focus_within_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_focus_within_value(
        UseFocusWithinProps::new()
            .on_focus_within(non_empty_prop_action(props.events.get("onFocusWithin")))
            .on_blur_within(non_empty_prop_action(props.events.get("onBlurWithin")))
            .on_focus_within_change(non_empty_prop_action(
                props.events.get("onFocusWithinChange"),
            ))
            .disabled(props.is_disabled)
            .focus_within(
                bool_attribute_value(props, &["isFocusWithin", "data-focus-within"])
                    .unwrap_or(false),
            ),
    )
}

fn focus_ring_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_focus_ring_value(
        UseFocusRingProps::new()
            .on_focus(non_empty_prop_action(props.events.get("onFocus")))
            .on_blur(non_empty_prop_action(props.events.get("onBlur")))
            .on_focus_change(non_empty_prop_action(props.events.get("onFocusChange")))
            .disabled(props.is_disabled)
            .focused(
                bool_attribute_value(props, &["isFocused", "focused", "data-focused"])
                    .unwrap_or(false),
            )
            .focus_visible(
                bool_attribute_value(props, &["isFocusVisible", "data-focus-visible"])
                    .unwrap_or(false),
            )
            .within(
                bool_attribute_value(props, &["within", "data-focus-ring-within"]).unwrap_or(false),
            )
            .focus_within(
                bool_attribute_value(props, &["isFocusWithin", "data-focus-within"])
                    .unwrap_or(false),
            )
            .auto_focus(bool_attribute_value(props, &["autoFocus", "autofocus"]).unwrap_or(false))
            .tab_index(i32_attribute_value(props, &["tabIndex", "tabindex"]).unwrap_or(0)),
    )
}

fn focus_scope_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_focus_scope_value(
        UseFocusScopeProps::new()
            .contain(bool_attribute_value(props, &["contain", "data-contain"]).unwrap_or(false))
            .restore_focus(
                bool_attribute_value(props, &["restoreFocus", "data-restore-focus"])
                    .unwrap_or(false),
            )
            .auto_focus(bool_attribute_value(props, &["autoFocus", "autofocus"]).unwrap_or(false))
            .disabled(props.is_disabled)
            .tab_index(i32_attribute_value(props, &["tabIndex", "tabindex"]).unwrap_or(-1)),
    )
}

fn field_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_field_value(
        UseFieldProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn checkbox_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_checkbox_value(
        UseCheckboxProps::new()
            .value(props.value.clone())
            .on_change(non_empty_prop_action(props.events.get("onChange")))
            .checked(
                props
                    .is_checked
                    .or_else(|| bool_attribute_value(props, &["checked", "data-checked"]))
                    .unwrap_or(false),
            )
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn checkbox_group_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_checkbox_group_value(
        UseCheckboxGroupProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .on_change(non_empty_prop_action(props.events.get("onChange")))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn separator_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_separator_value(UseSeparatorProps::new().orientation(orientation_attribute_value(props)))
}

fn toolbar_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_toolbar_value(
        UseToolbarProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .orientation(orientation_attribute_value(props))
            .disabled(props.is_disabled),
    )
}

fn drop_indicator_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_drop_indicator_value(
        UseDropIndicatorProps::new()
            .orientation(orientation_attribute_value(props))
            .target(
                bool_attribute_value(props, &["isTarget", "target", "data-target"])
                    .unwrap_or(false),
            ),
    )
}

fn selection_indicator_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_selection_indicator_value(
        UseSelectionIndicatorProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .selected(props.is_selected),
    )
}

fn i18n_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_i18n_value(
        UseI18nProps::new()
            .locale(non_empty_attribute(props, &["locale", "lang"]))
            .direction(non_empty_attribute(props, &["direction", "dir"])),
    )
}

fn overlay_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_overlay_value(
        UseOverlayProps::new()
            .open(bool_attribute_value(props, &["isOpen", "open", "data-open"]).unwrap_or(false))
            .on_open_change(
                non_empty_prop_action(props.events.get("onOpenChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onPress"))),
            )
            .on_close(non_empty_prop_action(props.events.get("onClose")))
            .disabled(props.is_disabled)
            .trigger_kind(non_empty_attribute(
                props,
                &["overlayTriggerKind", "triggerKind", "aria-haspopup"],
            )),
    )
}

fn menu_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    let mut menu_props = UseMenuProps::new()
        .label(props.label.clone().or_else(|| props.aria_label.clone()))
        .value(props.value.clone())
        .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
        .disabled(props.is_disabled)
        .read_only(props.is_read_only)
        .selection_mode(non_empty_attribute(
            props,
            &["selectionMode", "data-selection-mode"],
        ))
        .selection_behavior(non_empty_attribute(
            props,
            &["selectionBehavior", "data-selection-behavior"],
        ))
        .disabled_behavior(non_empty_attribute(
            props,
            &["disabledBehavior", "data-disabled-behavior"],
        ))
        .disallow_empty_selection(selection_disallows_empty(props))
        .should_focus_wrap(selection_should_focus_wrap(props));
    if let Some(selected_keys) = selection_attribute(props, &["selectedKeys", "data-selected-keys"])
    {
        menu_props = menu_props.selected_keys(selected_keys);
    }
    if let Some(default_selected_keys) = selection_attribute(
        props,
        &["defaultSelectedKeys", "data-default-selected-keys"],
    ) {
        menu_props = menu_props.default_selected_keys(default_selected_keys);
    }
    if let Some(disabled_keys) = selection_disabled_keys_attribute(props) {
        menu_props = menu_props.disabled_keys(disabled_keys);
    }
    use_menu_value(menu_props)
}

fn menu_item_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_menu_item_value(
        UseMenuItemProps::new()
            .text_value(
                props
                    .text_value
                    .clone()
                    .or_else(|| props.label.clone())
                    .or_else(|| props.value.clone()),
            )
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .on_action(
                non_empty_prop_action(props.events.get("onAction"))
                    .or_else(|| non_empty_prop_action(props.events.get("onPress")))
                    .or_else(|| non_empty_prop_action(props.action.as_ref())),
            )
            .disabled(props.is_disabled)
            .selected(props.is_selected),
    )
}

fn submenu_trigger_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_submenu_trigger_value(
        UseSubmenuTriggerProps::new()
            .on_press(non_empty_prop_action(
                props.events.get("onPress").or(props.action.as_ref()),
            ))
            .on_press_start(non_empty_prop_action(props.events.get("onPressStart")))
            .on_press_end(non_empty_prop_action(props.events.get("onPressEnd")))
            .on_press_up(non_empty_prop_action(props.events.get("onPressUp")))
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .action_payload(action_payload_value(props))
            .disabled(props.is_disabled)
            .pressed(
                bool_attribute_value(props, &["isPressed", "pressed", "data-pressed"])
                    .unwrap_or(false),
            )
            .open(
                props
                    .is_expanded
                    .or_else(|| bool_attribute_value(props, &["isOpen", "open", "data-open"]))
                    .unwrap_or(false),
            ),
    )
}

fn load_more_item_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_load_more_item_value(
        UseLoadMoreItemProps::new()
            .label(props.label.clone().or_else(|| props.text_value.clone()))
            .text_value(props.text_value.clone().or_else(|| props.label.clone()))
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .action_payload(action_payload_value(props))
            .on_press(
                non_empty_prop_action(props.events.get("onPress"))
                    .or_else(|| non_empty_prop_action(props.action.as_ref())),
            )
            .loading(
                bool_attribute_value(props, &["isLoading", "loading", "data-loading"])
                    .unwrap_or(false),
            )
            .disabled(props.is_disabled),
    )
}

fn collection_item_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_collection_item_value(
        UseCollectionItemProps::new()
            .value(props.value.clone())
            .text_value(
                props
                    .text_value
                    .clone()
                    .or_else(|| props.label.clone())
                    .or_else(|| props.value.clone()),
            )
            .selected(props.is_selected)
            .disabled(props.is_disabled)
            .expanded(props.is_expanded),
    )
}

fn tree_item_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    let has_child_items =
        bool_attribute_value(props, &["hasChildItems", "data-has-child-items"]).unwrap_or(false);
    use_tree_item_value(
        UseTreeItemProps::new()
            .value(props.value.clone())
            .text_value(
                props
                    .text_value
                    .clone()
                    .or_else(|| props.label.clone())
                    .or_else(|| props.value.clone()),
            )
            .selected(props.is_selected)
            .disabled(props.is_disabled)
            .expanded(props.is_expanded)
            .has_child_items(has_child_items),
    )
}

fn collection_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_collection_value(
        UseCollectionProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .item_count(
                usize_attribute_value(props, &["itemCount", "item-count", "data-item-count"])
                    .unwrap_or(0),
            )
            .empty(
                bool_attribute_value(props, &["isEmpty", "empty", "data-empty"]).unwrap_or(false),
            )
            .disabled(props.is_disabled),
    )
}

fn collection_section_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_collection_section_value(
        UseCollectionSectionProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .collection_kind(collection_section_kind(props))
            .disabled(props.is_disabled),
    )
}

fn collection_section_kind(props: &CompiledProps) -> CollectionSectionKind {
    let kind = props
        .attributes
        .get("data-collection-kind")
        .or_else(|| props.attributes.get("data-slot"))
        .map(String::as_str)
        .unwrap_or_default();

    match kind {
        "list-box" | "list-box-section" => CollectionSectionKind::ListBox,
        "grid-list" | "grid-list-section" => CollectionSectionKind::GridList,
        "menu" | "menu-section" => CollectionSectionKind::Menu,
        "tree" | "tree-section" => CollectionSectionKind::Tree,
        _ => CollectionSectionKind::Generic,
    }
}

fn radio_group_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_radio_group_value(
        UseRadioGroupProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .default_value(non_empty_attribute(
                props,
                &["defaultValue", "data-default-value"],
            ))
            .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only)
            .selection_mode(non_empty_attribute(
                props,
                &["selectionMode", "data-selection-mode"],
            )),
    )
}

fn radio_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_radio_value(
        UseRadioProps::new()
            .value(props.value.clone())
            .text_value(
                props
                    .text_value
                    .clone()
                    .or_else(|| props.label.clone())
                    .or_else(|| props.value.clone()),
            )
            .selected(props.is_selected || props.is_checked.unwrap_or(false))
            .disabled(props.is_disabled),
    )
}

fn tab_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_tab_value(
        UseTabProps::new()
            .value(props.value.clone())
            .text_value(
                props
                    .text_value
                    .clone()
                    .or_else(|| props.label.clone())
                    .or_else(|| props.value.clone()),
            )
            .selected(props.is_selected)
            .disabled(props.is_disabled),
    )
}

fn tab_list_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_tab_list_value(
        UseTabListProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .orientation(orientation_attribute_value(props))
            .disabled(props.is_disabled),
    )
}

fn tab_panel_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_tab_panel_value(UseTabPanelProps::new().value(props.value.clone()))
}

fn table_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_table_value(
        UseTableProps::new().label(props.label.clone().or_else(|| props.aria_label.clone())),
    )
}

fn table_section_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_table_section_value(
        UseTableSectionProps::new()
            .kind(table_section_kind(props))
            .label(props.label.clone().or_else(|| props.aria_label.clone())),
    )
}

fn table_section_kind(props: &CompiledProps) -> TableSectionKind {
    let kind = props
        .attributes
        .get("data-table-section")
        .or_else(|| props.attributes.get("data-slot"))
        .map(String::as_str)
        .unwrap_or_default();

    match kind {
        "header" | "table-header" => TableSectionKind::Header,
        "footer" | "table-footer" => TableSectionKind::Footer,
        _ => TableSectionKind::Body,
    }
}

fn table_row_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_table_row_value(
        UseTableRowProps::new()
            .selected(props.is_selected)
            .disabled(props.is_disabled),
    )
}

fn table_cell_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_table_cell_value(
        UseTableCellProps::new()
            .label(props.label.clone().or_else(|| props.text_value.clone()))
            .text_value(props.text_value.clone().or_else(|| props.label.clone())),
    )
}

fn table_column_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_table_column_value(
        UseTableColumnProps::new()
            .label(props.label.clone().or_else(|| props.text_value.clone()))
            .text_value(props.text_value.clone().or_else(|| props.label.clone())),
    )
}

fn table_caption_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_table_caption_value(
        UseTableCaptionProps::new()
            .label(props.label.clone().or_else(|| props.text_value.clone()))
            .text_value(props.text_value.clone().or_else(|| props.label.clone())),
    )
}

fn text_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_text_value(text_like_props(props))
}

fn label_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_label_value(text_like_props(props))
}

fn description_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_description_value(text_like_props(props))
}

fn field_error_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_field_error_value(text_like_props(props))
}

fn legend_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_legend_value(text_like_props(props))
}

fn visually_hidden_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_visually_hidden_value(text_like_props(props))
}

fn keyboard_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_keyboard_value(text_like_props(props))
}

fn list_box_header_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_list_box_header_value(text_like_props(props))
}

fn grid_list_header_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_grid_list_header_value(text_like_props(props))
}

fn tree_header_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_tree_header_value(text_like_props(props))
}

fn heading_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_heading_value(
        UseHeadingProps::new()
            .label(
                props
                    .label
                    .clone()
                    .or_else(|| props.aria_label.clone())
                    .or_else(|| props.text_value.clone()),
            )
            .text_value(props.text_value.clone().or_else(|| props.label.clone()))
            .level(heading_level_value(props)),
    )
}

fn date_field_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_date_field_value(
        UseDateFieldProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .placeholder(props.placeholder.clone())
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .granularity(non_empty_attribute(
                props,
                &["granularity", "data-granularity"],
            ))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn time_field_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_time_field_value(
        UseTimeFieldProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .placeholder(props.placeholder.clone())
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .granularity(non_empty_attribute(
                props,
                &["granularity", "data-granularity"],
            ))
            .hour_cycle(non_empty_attribute(
                props,
                &["hourCycle", "hour-cycle", "data-hour-cycle"],
            ))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn date_input_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_date_input_value(
        UseDateInputProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn date_segment_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_date_segment_value(
        UseDateSegmentProps::new()
            .segment_type(non_empty_attribute(
                props,
                &["segmentType", "segment-type", "data-type"],
            ))
            .value(props.value.clone())
            .text_value(props.text_value.clone().or_else(|| props.label.clone()))
            .placeholder(props.placeholder.clone())
            .placeholder_segment(
                bool_attribute_value(props, &["isPlaceholder", "data-placeholder"])
                    .unwrap_or(false),
            )
            .disabled(props.is_disabled)
            .invalid(props.is_invalid),
    )
}

fn calendar_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_calendar_value(
        UseCalendarProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .on_change(non_empty_prop_action(props.events.get("onChange")))
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn range_calendar_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_range_calendar_value(
        UseRangeCalendarProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .start_value(non_empty_attribute(
                props,
                &["startValue", "start-value", "data-start-value"],
            ))
            .end_value(non_empty_attribute(
                props,
                &["endValue", "end-value", "data-end-value"],
            ))
            .on_change(non_empty_prop_action(props.events.get("onChange")))
            .disabled(props.is_disabled)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn calendar_cell_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_calendar_cell_value(
        UseCalendarCellProps::new()
            .value(props.value.clone())
            .text_value(
                props
                    .text_value
                    .clone()
                    .or_else(|| props.label.clone())
                    .or_else(|| props.value.clone()),
            )
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .on_press(
                non_empty_prop_action(props.events.get("onPress"))
                    .or_else(|| non_empty_prop_action(props.action.as_ref())),
            )
            .selected(props.is_selected)
            .disabled(props.is_disabled)
            .unavailable(
                bool_attribute_value(props, &["isUnavailable", "data-unavailable"])
                    .unwrap_or(false),
            )
            .outside_month(
                bool_attribute_value(props, &["isOutsideMonth", "data-outside-month"])
                    .unwrap_or(false),
            )
            .today(bool_attribute_value(props, &["isToday", "data-today"]).unwrap_or(false))
            .pressed(bool_attribute_value(props, &["isPressed", "data-pressed"]).unwrap_or(false)),
    )
}

fn date_picker_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_date_picker_value(
        UseDatePickerProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .placeholder(props.placeholder.clone())
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .on_open_change(
                non_empty_prop_action(props.events.get("onOpenChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onPress"))),
            )
            .open(bool_attribute_value(props, &["isOpen", "open", "data-open"]).unwrap_or(false))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn date_range_picker_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_date_range_picker_value(
        UseDateRangePickerProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .start_value(non_empty_attribute(
                props,
                &["startValue", "start-value", "data-start-value"],
            ))
            .end_value(non_empty_attribute(
                props,
                &["endValue", "end-value", "data-end-value"],
            ))
            .placeholder(props.placeholder.clone())
            .on_start_change(non_empty_prop_action(props.events.get("onStartChange")))
            .on_end_change(non_empty_prop_action(props.events.get("onEndChange")))
            .on_open_change(
                non_empty_prop_action(props.events.get("onOpenChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onPress"))),
            )
            .open(bool_attribute_value(props, &["isOpen", "open", "data-open"]).unwrap_or(false))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn color_field_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_color_field_value(
        UseColorFieldProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .placeholder(props.placeholder.clone())
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .color_space(non_empty_attribute(
                props,
                &["colorSpace", "color-space", "data-color-space"],
            ))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn color_picker_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_color_picker_value(
        UseColorPickerProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .on_change(non_empty_prop_action(props.events.get("onChange")))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only),
    )
}

fn color_area_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_color_area_value(
        UseColorAreaProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .x_channel(non_empty_attribute(
                props,
                &["xChannel", "x-channel", "data-x-channel"],
            ))
            .y_channel(non_empty_attribute(
                props,
                &["yChannel", "y-channel", "data-y-channel"],
            ))
            .x_value(number_attribute_value(props, &["xValue", "data-x-value"]).unwrap_or(0.0))
            .y_value(number_attribute_value(props, &["yValue", "data-y-value"]).unwrap_or(0.0))
            .on_change(non_empty_prop_action(props.events.get("onChange")))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only),
    )
}

fn color_slider_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_color_slider_value(
        UseColorRangeProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .channel(non_empty_attribute(
                props,
                &["channel", "data-channel", "colorChannel"],
            ))
            .value_number(
                props
                    .value_number
                    .or_else(|| number_attribute_value(props, &["valueNumber", "aria-valuenow"]))
                    .unwrap_or(0.0),
            )
            .min_value(
                props
                    .min_value
                    .or_else(|| {
                        number_attribute_value(props, &["minValue", "min", "aria-valuemin"])
                    })
                    .unwrap_or(0.0),
            )
            .max_value(
                props
                    .max_value
                    .or_else(|| {
                        number_attribute_value(props, &["maxValue", "max", "aria-valuemax"])
                    })
                    .unwrap_or(360.0),
            )
            .step_value(
                props
                    .step_value
                    .or_else(|| number_attribute_value(props, &["stepValue", "step"]))
                    .unwrap_or(1.0),
            )
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .disabled(props.is_disabled)
            .read_only(props.is_read_only),
    )
}

fn color_wheel_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_color_wheel_value(
        UseColorRangeProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .channel(non_empty_attribute(
                props,
                &["channel", "data-channel", "colorChannel"],
            ))
            .value_number(
                props
                    .value_number
                    .or_else(|| number_attribute_value(props, &["valueNumber", "aria-valuenow"]))
                    .unwrap_or(0.0),
            )
            .min_value(
                props
                    .min_value
                    .or_else(|| {
                        number_attribute_value(props, &["minValue", "min", "aria-valuemin"])
                    })
                    .unwrap_or(0.0),
            )
            .max_value(
                props
                    .max_value
                    .or_else(|| {
                        number_attribute_value(props, &["maxValue", "max", "aria-valuemax"])
                    })
                    .unwrap_or(360.0),
            )
            .step_value(
                props
                    .step_value
                    .or_else(|| number_attribute_value(props, &["stepValue", "step"]))
                    .unwrap_or(1.0),
            )
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .disabled(props.is_disabled)
            .read_only(props.is_read_only),
    )
}

fn color_swatch_picker_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    let mut picker_props = UseColorSwatchPickerProps::new()
        .label(props.label.clone().or_else(|| props.aria_label.clone()))
        .value(props.value.clone())
        .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
        .disabled(props.is_disabled)
        .read_only(props.is_read_only)
        .selection_mode(non_empty_attribute(
            props,
            &["selectionMode", "data-selection-mode"],
        ))
        .selection_behavior(non_empty_attribute(
            props,
            &["selectionBehavior", "data-selection-behavior"],
        ))
        .disabled_behavior(non_empty_attribute(
            props,
            &["disabledBehavior", "data-disabled-behavior"],
        ))
        .disallow_empty_selection(selection_disallows_empty(props));
    if let Some(selected_keys) = selection_attribute(props, &["selectedKeys", "data-selected-keys"])
    {
        picker_props = picker_props.selected_keys(selected_keys);
    }
    if let Some(default_selected_keys) = selection_attribute(
        props,
        &["defaultSelectedKeys", "data-default-selected-keys"],
    ) {
        picker_props = picker_props.default_selected_keys(default_selected_keys);
    }
    if let Some(disabled_keys) = selection_disabled_keys_attribute(props) {
        picker_props = picker_props.disabled_keys(disabled_keys);
    }
    use_color_swatch_picker_value(picker_props)
}

fn color_swatch_picker_item_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_color_swatch_picker_item_value(
        UseColorSwatchPickerItemProps::new()
            .value(props.value.clone())
            .text_value(
                props
                    .text_value
                    .clone()
                    .or_else(|| props.label.clone())
                    .or_else(|| props.value.clone()),
            )
            .selected(props.is_selected)
            .disabled(props.is_disabled),
    )
}

fn color_swatch_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_color_swatch_value(
        UseColorSwatchProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .disabled(props.is_disabled),
    )
}

fn color_thumb_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_color_thumb_value(
        UseColorThumbProps::new()
            .value(props.value.clone())
            .x_value(number_attribute_value(props, &["xValue", "data-x-value"]).unwrap_or(0.0))
            .y_value(number_attribute_value(props, &["yValue", "data-y-value"]).unwrap_or(0.0))
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .action_payload(action_payload_value(props))
            .on_press(
                non_empty_prop_action(props.events.get("onPress"))
                    .or_else(|| non_empty_prop_action(props.action.as_ref())),
            )
            .on_press_start(non_empty_prop_action(props.events.get("onPressStart")))
            .on_press_end(non_empty_prop_action(props.events.get("onPressEnd")))
            .on_press_up(non_empty_prop_action(props.events.get("onPressUp")))
            .disabled(props.is_disabled)
            .pressed(bool_attribute_value(props, &["isPressed", "data-pressed"]).unwrap_or(false))
            .dragging(
                bool_attribute_value(props, &["isDragging", "dragging", "data-dragging"])
                    .unwrap_or(false),
            ),
    )
}

fn combo_box_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    let mut combo_box_props = UseComboBoxProps::new()
        .label(props.label.clone().or_else(|| props.aria_label.clone()))
        .value(props.value.clone())
        .input_value(input_value_attribute(props).or_else(|| props.value.clone()))
        .placeholder(props.placeholder.clone())
        .on_change(
            non_empty_prop_action(props.events.get("onChange"))
                .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
        )
        .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
        .on_open_change(
            non_empty_prop_action(props.events.get("onOpenChange"))
                .or_else(|| non_empty_prop_action(props.events.get("onPress"))),
        )
        .open(open_attribute_value(props))
        .disabled(props.is_disabled)
        .required(props.is_required)
        .invalid(props.is_invalid)
        .read_only(props.is_read_only)
        .selection_mode(non_empty_attribute(
            props,
            &["selectionMode", "data-selection-mode"],
        ))
        .selection_behavior(non_empty_attribute(
            props,
            &["selectionBehavior", "data-selection-behavior"],
        ))
        .disabled_behavior(non_empty_attribute(
            props,
            &["disabledBehavior", "data-disabled-behavior"],
        ))
        .disallow_empty_selection(selection_disallows_empty(props));
    if let Some(selected_keys) = selection_attribute(props, &["selectedKeys", "data-selected-keys"])
    {
        combo_box_props = combo_box_props.selected_keys(selected_keys);
    }
    if let Some(default_selected_keys) = selection_attribute(
        props,
        &["defaultSelectedKeys", "data-default-selected-keys"],
    ) {
        combo_box_props = combo_box_props.default_selected_keys(default_selected_keys);
    }
    if let Some(disabled_keys) = selection_disabled_keys_attribute(props) {
        combo_box_props = combo_box_props.disabled_keys(disabled_keys);
    }
    use_combo_box_value(combo_box_props)
}

fn autocomplete_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    let mut autocomplete_props = UseAutocompleteProps::new()
        .label(props.label.clone().or_else(|| props.aria_label.clone()))
        .value(props.value.clone())
        .input_value(input_value_attribute(props).or_else(|| props.value.clone()))
        .placeholder(props.placeholder.clone())
        .on_change(
            non_empty_prop_action(props.events.get("onChange"))
                .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
        )
        .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
        .disabled(props.is_disabled)
        .required(props.is_required)
        .invalid(props.is_invalid)
        .read_only(props.is_read_only)
        .selection_mode(non_empty_attribute(
            props,
            &["selectionMode", "data-selection-mode"],
        ))
        .selection_behavior(non_empty_attribute(
            props,
            &["selectionBehavior", "data-selection-behavior"],
        ))
        .disabled_behavior(non_empty_attribute(
            props,
            &["disabledBehavior", "data-disabled-behavior"],
        ))
        .disallow_empty_selection(selection_disallows_empty(props));
    if let Some(selected_keys) = selection_attribute(props, &["selectedKeys", "data-selected-keys"])
    {
        autocomplete_props = autocomplete_props.selected_keys(selected_keys);
    }
    if let Some(default_selected_keys) = selection_attribute(
        props,
        &["defaultSelectedKeys", "data-default-selected-keys"],
    ) {
        autocomplete_props = autocomplete_props.default_selected_keys(default_selected_keys);
    }
    if let Some(disabled_keys) = selection_disabled_keys_attribute(props) {
        autocomplete_props = autocomplete_props.disabled_keys(disabled_keys);
    }
    use_autocomplete_value(autocomplete_props)
}

fn select_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    let mut select_props = UseSelectProps::new()
        .label(props.label.clone().or_else(|| props.aria_label.clone()))
        .value(props.value.clone())
        .placeholder(props.placeholder.clone())
        .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
        .on_open_change(
            non_empty_prop_action(props.events.get("onOpenChange"))
                .or_else(|| non_empty_prop_action(props.events.get("onPress"))),
        )
        .open(open_attribute_value(props))
        .disabled(props.is_disabled)
        .required(props.is_required)
        .invalid(props.is_invalid)
        .read_only(props.is_read_only)
        .selection_mode(non_empty_attribute(
            props,
            &["selectionMode", "data-selection-mode"],
        ))
        .selection_behavior(non_empty_attribute(
            props,
            &["selectionBehavior", "data-selection-behavior"],
        ))
        .disabled_behavior(non_empty_attribute(
            props,
            &["disabledBehavior", "data-disabled-behavior"],
        ))
        .disallow_empty_selection(selection_disallows_empty(props));
    if let Some(selected_keys) = selection_attribute(props, &["selectedKeys", "data-selected-keys"])
    {
        select_props = select_props.selected_keys(selected_keys);
    }
    if let Some(default_selected_keys) = selection_attribute(
        props,
        &["defaultSelectedKeys", "data-default-selected-keys"],
    ) {
        select_props = select_props.default_selected_keys(default_selected_keys);
    }
    if let Some(disabled_keys) = selection_disabled_keys_attribute(props) {
        select_props = select_props.disabled_keys(disabled_keys);
    }
    use_select_value(select_props)
}

fn select_display_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_select_display_value(
        UseSelectDisplayProps::new()
            .value(props.value.clone())
            .placeholder(props.placeholder.clone()),
    )
}

fn combo_box_display_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_combo_box_display_value(
        UseComboBoxDisplayProps::new()
            .value(props.value.clone())
            .placeholder(props.placeholder.clone()),
    )
}

fn selection_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    let mut selection_props = UseSelectionProps::new()
        .value(props.value.clone())
        .on_action(non_empty_prop_action(props.events.get("onAction")))
        .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
        .disabled(props.is_disabled)
        .read_only(props.is_read_only)
        .selection_mode(non_empty_attribute(
            props,
            &["selectionMode", "data-selection-mode"],
        ))
        .selection_behavior(non_empty_attribute(
            props,
            &["selectionBehavior", "data-selection-behavior"],
        ))
        .disabled_behavior(non_empty_attribute(
            props,
            &["disabledBehavior", "data-disabled-behavior"],
        ))
        .disallow_empty_selection(selection_disallows_empty(props))
        .should_focus_wrap(selection_should_focus_wrap(props))
        .escape_key_behavior(selection_escape_key_behavior(props));

    if let Some(selected_keys) = selection_attribute(props, &["selectedKeys", "data-selected-keys"])
    {
        selection_props = selection_props.selected_keys(selected_keys);
    }
    if let Some(default_selected_keys) = selection_attribute(
        props,
        &["defaultSelectedKeys", "data-default-selected-keys"],
    ) {
        selection_props = selection_props.default_selected_keys(default_selected_keys);
    }
    if let Some(disabled_keys) = selection_disabled_keys_attribute(props) {
        selection_props = selection_props.disabled_keys(disabled_keys);
    }

    use_selection_value(selection_props)
}

fn tree_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    let mut tree_props = UseTreeProps::new()
        .label(props.label.clone().or_else(|| props.aria_label.clone()))
        .value(props.value.clone())
        .on_action(non_empty_prop_action(props.events.get("onAction")))
        .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
        .disabled(props.is_disabled)
        .read_only(props.is_read_only)
        .selection_mode(non_empty_attribute(
            props,
            &["selectionMode", "data-selection-mode"],
        ))
        .selection_behavior(non_empty_attribute(
            props,
            &["selectionBehavior", "data-selection-behavior"],
        ))
        .disabled_behavior(non_empty_attribute(
            props,
            &["disabledBehavior", "data-disabled-behavior"],
        ))
        .disallow_empty_selection(selection_disallows_empty(props))
        .should_focus_wrap(selection_should_focus_wrap(props))
        .escape_key_behavior(selection_escape_key_behavior(props))
        .on_expanded_change(non_empty_prop_action(props.events.get("onExpandedChange")));

    if let Some(selected_keys) = selection_attribute(props, &["selectedKeys", "data-selected-keys"])
    {
        tree_props = tree_props.selected_keys(selected_keys);
    }
    if let Some(default_selected_keys) = selection_attribute(
        props,
        &["defaultSelectedKeys", "data-default-selected-keys"],
    ) {
        tree_props = tree_props.default_selected_keys(default_selected_keys);
    }
    if let Some(disabled_keys) = selection_disabled_keys_attribute(props) {
        tree_props = tree_props.disabled_keys(disabled_keys);
    }
    if let Some(expanded_keys) = collection_key_set_attribute(
        props,
        &["expandedKeys", "expanded-keys", "data-expanded-keys"],
    ) {
        tree_props = tree_props.expanded_keys(expanded_keys);
    }
    if let Some(default_expanded_keys) = collection_key_set_attribute(
        props,
        &[
            "defaultExpandedKeys",
            "default-expanded-keys",
            "data-default-expanded-keys",
        ],
    ) {
        tree_props = tree_props.default_expanded_keys(default_expanded_keys);
    }

    use_tree_value(tree_props)
}

fn disclosure_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_disclosure_value(
        UseDisclosureProps::new()
            .on_expanded_change(
                non_empty_prop_action(props.events.get("onExpandedChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onPress"))),
            )
            .expanded(
                props
                    .is_expanded
                    .or_else(|| bool_attribute_value(props, &["expanded", "aria-expanded"]))
                    .unwrap_or(false),
            )
            .disabled(props.is_disabled),
    )
}

fn disclosure_group_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_disclosure_group_value(
        UseDisclosureGroupProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .expanded_keys(non_empty_attribute(
                props,
                &["expandedKeys", "expanded-keys", "data-expanded-keys"],
            ))
            .on_expanded_change(non_empty_prop_action(props.events.get("onExpandedChange")))
            .allows_multiple_expanded(
                bool_attribute_value(
                    props,
                    &[
                        "allowsMultipleExpanded",
                        "allows-multiple-expanded",
                        "data-allows-multiple-expanded",
                    ],
                )
                .unwrap_or(false),
            )
            .disabled(props.is_disabled),
    )
}

fn range_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_range_value(
        UseRangeProps::new()
            .value_number(
                props
                    .value_number
                    .or_else(|| number_attribute_value(props, &["valueNumber", "aria-valuenow"]))
                    .unwrap_or(0.0),
            )
            .min_value(
                props
                    .min_value
                    .or_else(|| {
                        number_attribute_value(props, &["minValue", "min", "aria-valuemin"])
                    })
                    .unwrap_or(0.0),
            )
            .max_value(
                props
                    .max_value
                    .or_else(|| {
                        number_attribute_value(props, &["maxValue", "max", "aria-valuemax"])
                    })
                    .unwrap_or(100.0),
            )
            .step_value(
                props
                    .step_value
                    .or_else(|| number_attribute_value(props, &["stepValue", "step"]))
                    .unwrap_or(1.0),
            )
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn number_field_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_number_field_value(
        UseNumberFieldProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value_number(
                props
                    .value_number
                    .or_else(|| {
                        number_attribute_value(
                            props,
                            &["valueNumber", "data-value-number", "aria-valuenow"],
                        )
                    })
                    .unwrap_or(0.0),
            )
            .placeholder(
                props
                    .placeholder
                    .clone()
                    .or_else(|| non_empty_attribute(props, &["placeholder", "aria-placeholder"])),
            )
            .min_value(
                props
                    .min_value
                    .or_else(|| {
                        number_attribute_value(props, &["minValue", "min", "aria-valuemin"])
                    })
                    .unwrap_or(0.0),
            )
            .max_value(
                props
                    .max_value
                    .or_else(|| {
                        number_attribute_value(props, &["maxValue", "max", "aria-valuemax"])
                    })
                    .unwrap_or(100.0),
            )
            .step_value(
                props
                    .step_value
                    .or_else(|| number_attribute_value(props, &["stepValue", "step"]))
                    .unwrap_or(1.0),
            )
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn slider_track_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_slider_track_value(
        UseSliderTrackProps::new()
            .orientation(orientation_attribute_value(props))
            .disabled(props.is_disabled),
    )
}

fn slider_fill_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_slider_fill_value(
        UseSliderFillProps::new()
            .orientation(orientation_attribute_value(props))
            .value_number(
                props
                    .value_number
                    .or_else(|| {
                        number_attribute_value(props, &["valueNumber", "data-value-number"])
                    })
                    .unwrap_or(0.0),
            )
            .disabled(props.is_disabled),
    )
}

fn slider_output_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_slider_output_value(
        UseSliderOutputProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .value_number(
                props
                    .value_number
                    .or_else(|| {
                        number_attribute_value(props, &["valueNumber", "data-value-number"])
                    })
                    .unwrap_or(0.0),
            ),
    )
}

fn toggle_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_toggle_value(
        UseToggleProps::new()
            .on_change(non_empty_prop_action(props.events.get("onChange")))
            .checked(props.is_checked.unwrap_or(props.is_selected))
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn switch_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_switch_value(
        UseSwitchProps::new()
            .on_change(non_empty_prop_action(props.events.get("onChange")))
            .checked(
                props
                    .is_checked
                    .or_else(|| bool_attribute_value(props, &["checked", "data-checked"]))
                    .unwrap_or(false),
            )
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn toggle_button_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_toggle_button_value(
        UseToggleButtonProps::new()
            .on_press(non_empty_prop_action(
                props.events.get("onPress").or(props.action.as_ref()),
            ))
            .on_press_start(non_empty_prop_action(props.events.get("onPressStart")))
            .on_press_end(non_empty_prop_action(props.events.get("onPressEnd")))
            .on_press_up(non_empty_prop_action(props.events.get("onPressUp")))
            .action_value(non_empty_attribute(props, &["actionValue"]))
            .action_payload(action_payload_value(props))
            .selected(props.is_selected)
            .disabled(props.is_disabled)
            .pressed(bool_attribute_value(props, &["isPressed", "pressed"]).unwrap_or(false)),
    )
}

fn toggle_button_group_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_toggle_button_group_value(
        UseToggleButtonGroupProps::new()
            .label(props.label.clone().or_else(|| props.aria_label.clone()))
            .value(props.value.clone())
            .orientation(orientation_attribute_value(props))
            .on_selection_change(non_empty_prop_action(props.events.get("onSelectionChange")))
            .disabled(props.is_disabled)
            .read_only(props.is_read_only)
            .selection_mode(non_empty_attribute(
                props,
                &["selectionMode", "data-selection-mode"],
            )),
    )
}

fn text_field_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_text_field_value(
        UseTextFieldProps::new()
            .value(props.value.clone())
            .placeholder(props.placeholder.clone())
            .input_type(props.input_type.clone())
            .on_change(
                non_empty_prop_action(props.events.get("onChange"))
                    .or_else(|| non_empty_prop_action(props.events.get("onInput"))),
            )
            .disabled(props.is_disabled)
            .required(props.is_required)
            .invalid(props.is_invalid)
            .read_only(props.is_read_only),
    )
}

fn toast_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_toast_value(
        UseToastProps::new()
            .title(
                non_empty_attribute(props, &["title", "data-title"])
                    .or_else(|| props.label.clone()),
            )
            .description(non_empty_attribute(
                props,
                &["description", "data-description"],
            ))
            .on_close(non_empty_prop_action(props.events.get("onClose"))),
    )
}

fn toast_region_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_toast_region_value(
        UseToastRegionProps::new().label(props.label.clone().or_else(|| props.aria_label.clone())),
    )
}

fn insert_hook_scope(
    scope: &mut JsonMap<String, JsonValue>,
    hook_name: &str,
    hook_value: JsonValue,
    prop_keys: &[&str],
    state_keys: &[&str],
) {
    if let JsonValue::Object(hook_scope) = &hook_value {
        for key in prop_keys {
            if let Some(value) = hook_scope.get(*key) {
                scope.insert((*key).to_string(), value.clone());
            }
        }
        for key in state_keys {
            if let Some(value) = hook_scope.get(*key) {
                scope.entry((*key).to_string()).or_insert(value.clone());
            }
        }
    }
    scope.insert(hook_name.to_string(), hook_value);
}

fn non_empty_prop_action(action: Option<&String>) -> Option<String> {
    action.filter(|action| !action.is_empty()).cloned()
}

fn non_empty_attribute(props: &CompiledProps, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        props
            .attributes
            .get(*name)
            .filter(|value| !value.is_empty())
            .cloned()
    })
}

fn selection_attribute(props: &CompiledProps, names: &[&str]) -> Option<Selection> {
    names.iter().find_map(|name| {
        props.attributes.get(*name).and_then(|value| {
            let value = value.trim();
            if value.is_empty() || value.eq_ignore_ascii_case("null") {
                return None;
            }
            serde_json::from_str(value)
                .ok()
                .or_else(|| serde_json::from_value(JsonValue::String(value.to_string())).ok())
        })
    })
}

fn selection_disabled_keys_attribute(
    props: &CompiledProps,
) -> Option<BTreeSet<crate::selection::CollectionKey>> {
    selection_attribute(props, &["disabledKeys", "data-disabled-keys"])
        .and_then(|selection| selection.explicit_keys().cloned())
}

fn collection_key_set_attribute(
    props: &CompiledProps,
    names: &[&str],
) -> Option<BTreeSet<crate::selection::CollectionKey>> {
    selection_attribute(props, names).and_then(|selection| selection.explicit_keys().cloned())
}

fn selection_disallows_empty(props: &CompiledProps) -> bool {
    bool_attribute_value(
        props,
        &["disallowEmptySelection", "data-disallow-empty-selection"],
    )
    .unwrap_or(false)
}

fn selection_should_focus_wrap(props: &CompiledProps) -> bool {
    bool_attribute_value(props, &["shouldFocusWrap", "data-should-focus-wrap"]).unwrap_or(false)
}

fn selection_escape_key_behavior(props: &CompiledProps) -> Option<String> {
    non_empty_attribute(props, &["escapeKeyBehavior", "data-escape-key-behavior"])
}

fn input_value_attribute(props: &CompiledProps) -> Option<String> {
    non_empty_attribute(props, &["inputValue", "input-value", "data-input-value"])
}

fn text_like_props(props: &CompiledProps) -> UseTextProps {
    UseTextProps::new()
        .label(
            props
                .label
                .clone()
                .or_else(|| props.aria_label.clone())
                .or_else(|| props.text_value.clone()),
        )
        .text_value(props.text_value.clone().or_else(|| props.label.clone()))
}

fn heading_level_value(props: &CompiledProps) -> u32 {
    number_attribute_value(props, &["level", "aria-level", "data-level"])
        .map(|level| level.round().clamp(1.0, 6.0) as u32)
        .unwrap_or(2)
}

fn open_attribute_value(props: &CompiledProps) -> bool {
    bool_attribute_value(props, &["isOpen", "open", "data-open"]).unwrap_or(false)
}

fn orientation_attribute_value(props: &CompiledProps) -> Option<&'static str> {
    props.orientation.map(|orientation| match orientation {
        CompiledOrientation::Horizontal => "horizontal",
        CompiledOrientation::Vertical => "vertical",
    })
}

fn action_payload_value(props: &CompiledProps) -> JsonValue {
    props
        .attributes
        .get("actionPayload")
        .or_else(|| props.attributes.get("action-payload"))
        .or_else(|| props.attributes.get("data-action-payload"))
        .or_else(|| props.attributes.get("data-a3s-action-payload"))
        .cloned()
        .map(JsonValue::String)
        .unwrap_or(JsonValue::Null)
}

fn bool_attribute_value(props: &CompiledProps, names: &[&str]) -> Option<bool> {
    names.iter().find_map(|name| {
        props.attributes.get(*name).and_then(|value| {
            match value.trim().to_ascii_lowercase().as_str() {
                "" | "true" => Some(true),
                "false" => Some(false),
                _ => None,
            }
        })
    })
}

fn i32_attribute_value(props: &CompiledProps, names: &[&str]) -> Option<i32> {
    names.iter().find_map(|name| {
        props
            .attributes
            .get(*name)
            .and_then(|value| value.trim().parse::<i32>().ok())
    })
}

fn number_attribute_value(props: &CompiledProps, names: &[&str]) -> Option<f64> {
    names.iter().find_map(|name| {
        props.attributes.get(*name).and_then(|value| {
            value
                .trim()
                .parse::<f64>()
                .ok()
                .filter(|value| value.is_finite())
        })
    })
}

fn usize_attribute_value(props: &CompiledProps, names: &[&str]) -> Option<usize> {
    names.iter().find_map(|name| {
        props.attributes.get(*name).and_then(|value| {
            let value = value.trim();
            value.parse::<usize>().ok().or_else(|| {
                value
                    .parse::<f64>()
                    .ok()
                    .filter(|value| value.is_finite() && *value >= 0.0)
                    .map(|value| value.round().min(usize::MAX as f64) as usize)
            })
        })
    })
}

fn u32_attribute_value(props: &CompiledProps, names: &[&str]) -> Option<u32> {
    usize_attribute_value(props, names).map(|value| value.min(u32::MAX as usize) as u32)
}

fn insert_optional_string(
    scope: &mut JsonMap<String, JsonValue>,
    name: &str,
    value: Option<&String>,
) {
    if let Some(value) = value {
        scope.insert(name.to_string(), JsonValue::String(value.clone()));
    }
}

fn insert_optional_bool(scope: &mut JsonMap<String, JsonValue>, name: &str, value: Option<bool>) {
    if let Some(value) = value {
        scope.insert(name.to_string(), JsonValue::Bool(value));
    }
}

fn insert_optional_number(scope: &mut JsonMap<String, JsonValue>, name: &str, value: Option<f64>) {
    if let Some(value) = value {
        if let Some(number) = serde_json::Number::from_f64(value) {
            scope.insert(name.to_string(), JsonValue::Number(number));
        }
    }
}

fn for_each_binding(props: &CompiledProps) -> GuiResult<&CompiledBinding> {
    let each = props.bindings.get("each");
    let of = props.bindings.get("of");
    match (each, of) {
        (Some(_), Some(_)) => Err(GuiError::invalid_tree(
            "RSX <For> cannot use both each={...} and of={...}",
        )),
        (Some(binding), None) | (None, Some(binding)) => {
            for property in props.bindings.keys() {
                if property != "each" && property != "of" {
                    return Err(GuiError::invalid_tree(format!(
                        "RSX <For> only supports dynamic each/of bindings; property {property:?} must be static"
                    )));
                }
            }
            Ok(binding)
        }
        (None, None) => Err(GuiError::invalid_tree(
            "RSX <For> needs an each={state.items} binding",
        )),
    }
}

fn extend_local_scope(
    scope: &JsonValue,
    item_name: &str,
    item: &JsonValue,
    index_name: Option<&str>,
    index: usize,
) -> GuiResult<JsonValue> {
    let JsonValue::Object(scope) = scope else {
        return Err(GuiError::invalid_tree(
            "RSX binding scope must be a JSON object",
        ));
    };
    let mut scope = scope.clone();
    scope.insert(item_name.to_string(), item.clone());
    if let Some(index_name) = index_name {
        scope.insert(
            index_name.to_string(),
            JsonValue::Number(serde_json::Number::from(index as u64)),
        );
    }
    Ok(JsonValue::Object(scope))
}

fn for_item_key(item: &JsonValue, key_by: Option<&str>, index: usize) -> GuiResult<String> {
    let Some(key_by) = key_by else {
        return Ok(index.to_string());
    };
    let key = if key_by == "." {
        item
    } else {
        let mut value = item;
        for segment in key_by.split('.') {
            if segment.trim().is_empty() {
                return Err(GuiError::invalid_tree(
                    "RSX <For> keyBy cannot contain empty path segments",
                ));
            }
            value = json_path_get(value, segment).ok_or_else(|| {
                GuiError::invalid_tree(format!(
                    "RSX <For> keyBy path segment {segment:?} is missing"
                ))
            })?;
        }
        value
    };
    let key = binding_string("keyBy", key)?;
    if key.is_empty() {
        Err(GuiError::invalid_tree(
            "RSX <For> keyBy resolved to an empty item key",
        ))
    } else {
        Ok(key)
    }
}

fn prefix_node_keys(node: CompiledRsxNode, prefix: &str) -> CompiledRsxNode {
    match node {
        CompiledRsxNode::Text { key, value } => CompiledRsxNode::Text {
            key: format!("{prefix}-{key}"),
            value,
        },
        CompiledRsxNode::Element {
            key,
            tag,
            import_source,
            props,
            children,
        } => CompiledRsxNode::Element {
            key: format!("{prefix}-{key}"),
            tag,
            import_source,
            props,
            children: children
                .into_iter()
                .map(|child| prefix_node_keys(child, prefix))
                .collect(),
        },
    }
}

fn control_identifier_attribute<'a>(
    props: &'a CompiledProps,
    name: &str,
) -> GuiResult<Option<&'a str>> {
    if props.bindings.contains_key(name) {
        return Err(GuiError::invalid_tree(format!(
            "RSX control attribute {name:?} must be a static identifier"
        )));
    }
    let Some(value) = props.attributes.get(name) else {
        return Ok(None);
    };
    if is_valid_local_identifier(value) {
        Ok(Some(value.as_str()))
    } else {
        Err(GuiError::invalid_tree(format!(
            "RSX control attribute {name:?} must be a valid identifier"
        )))
    }
}

fn is_valid_local_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if matches!(
        value,
        "state" | "props" | "derived" | "context" | "resource"
    ) {
        return false;
    }
    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn show_condition(tag: &str, props: &CompiledProps) -> GuiResult<bool> {
    let when = control_bool_attribute(props, "when")?;
    let unless = control_bool_attribute(props, "unless")?;

    match (when, unless) {
        (Some(when), Some(unless)) => Ok(when && !unless),
        (Some(when), None) => Ok(when),
        (None, Some(unless)) => Ok(!unless),
        (None, None) => Err(GuiError::invalid_tree(format!(
            "RSX <{tag}> needs a boolean when={{...}} or unless={{...}} binding"
        ))),
    }
}

fn control_bool_attribute(props: &CompiledProps, name: &str) -> GuiResult<Option<bool>> {
    let Some(value) = props.attributes.get(name) else {
        return Ok(None);
    };
    match value.as_str() {
        "true" => Ok(Some(true)),
        "false" => Ok(Some(false)),
        _ => Err(GuiError::invalid_tree(format!(
            "RSX control attribute {name:?} must resolve to a boolean"
        ))),
    }
}

impl CompiledProps {
    pub(crate) fn apply_default_props(
        &mut self,
        defaults: &BTreeMap<String, JsonValue>,
    ) -> GuiResult<()> {
        let explicit_props = self.explicit_props.clone();
        for (property, value) in defaults {
            if explicit_props.contains(&canonical_prop_name(property)) {
                continue;
            }
            self.apply_resolved_binding(property, value)?;
        }
        Ok(())
    }

    pub fn resolve_bindings(&mut self, scope: &JsonValue) -> GuiResult<()> {
        let explicit_props = self.explicit_props.clone();
        for binding in self.spreads.clone() {
            let value = binding.resolve(scope)?;
            self.apply_resolved_spread(&binding, value, &explicit_props)?;
        }
        for (property, binding) in self.bindings.clone() {
            let value = binding.resolve(scope)?;
            self.apply_resolved_binding(&property, value)?;
        }
        self.bindings.clear();
        self.spreads.clear();
        self.explicit_props.clear();
        Ok(())
    }

    fn apply_resolved_spread(
        &mut self,
        binding: &CompiledBinding,
        value: &JsonValue,
        explicit_props: &BTreeSet<String>,
    ) -> GuiResult<()> {
        let JsonValue::Object(object) = value else {
            return Err(GuiError::invalid_tree(format!(
                "RSX spread {} must resolve to an object",
                binding.display_path()
            )));
        };

        for (property, value) in object {
            if property == "key" {
                return Err(GuiError::invalid_tree(
                    "RSX spread props cannot provide key; keyed identity must be explicit",
                ));
            }
            if explicit_props.contains(&canonical_prop_name(property)) {
                continue;
            }
            self.apply_resolved_binding(property, value)?;
        }
        Ok(())
    }

    fn apply_resolved_binding(&mut self, property: &str, value: &JsonValue) -> GuiResult<()> {
        match property {
            "label" => self.label = Some(binding_string(property, value)?),
            "textValue" => self.text_value = Some(binding_string(property, value)?),
            "value" => self.value = Some(binding_string(property, value)?),
            "placeholder" => self.placeholder = Some(binding_string(property, value)?),
            "action" => self.action = Some(binding_string(property, value)?),
            "aria-label" | "ariaLabel" => self.aria_label = Some(binding_string(property, value)?),
            "id" => self.id = Some(binding_string(property, value)?),
            "name" => self.name = Some(binding_string(property, value)?),
            "form" => self.form = Some(binding_string(property, value)?),
            "type" | "inputType" => self.input_type = Some(binding_string(property, value)?),
            "href" => self.href = Some(binding_string(property, value)?),
            "class" | "className" => {
                self.class_name =
                    merge_class_names(self.class_name.take(), binding_string(property, value)?)
            }
            "style" => self
                .style
                .extend(parse_style_text(&binding_string(property, value)?)),
            "orientation" => {
                self.orientation = match binding_string(property, value)?.as_str() {
                    "horizontal" => Some(CompiledOrientation::Horizontal),
                    "vertical" => Some(CompiledOrientation::Vertical),
                    other => {
                        return Err(GuiError::invalid_tree(format!(
                            "RSX binding for property {property:?} resolved to unsupported orientation {other:?}"
                        )))
                    }
                };
            }
            "isDisabled" | "disabled" => self.is_disabled = binding_bool(property, value)?,
            "aria-disabled" => {
                self.is_disabled = binding_bool(property, value)?;
                self.attributes
                    .insert(property.to_string(), binding_string(property, value)?);
            }
            "isRequired" | "required" => self.is_required = binding_bool(property, value)?,
            "aria-required" => {
                self.is_required = binding_bool(property, value)?;
                self.attributes
                    .insert(property.to_string(), binding_string(property, value)?);
            }
            "isInvalid" | "invalid" => self.is_invalid = binding_bool(property, value)?,
            "aria-invalid" => {
                self.is_invalid = binding_bool(property, value)?;
                self.attributes
                    .insert(property.to_string(), binding_string(property, value)?);
            }
            "isReadOnly" | "readOnly" | "readonly" => {
                self.is_read_only = binding_bool(property, value)?
            }
            "aria-readonly" => {
                self.is_read_only = binding_bool(property, value)?;
                self.attributes
                    .insert(property.to_string(), binding_string(property, value)?);
            }
            "isSelected" | "selected" => self.is_selected = binding_bool(property, value)?,
            "aria-selected" => {
                self.is_selected = binding_bool(property, value)?;
                self.attributes
                    .insert(property.to_string(), binding_string(property, value)?);
            }
            "isChecked" | "checked" => self.is_checked = Some(binding_bool(property, value)?),
            "aria-checked" => {
                self.is_checked = Some(binding_bool(property, value)?);
                self.attributes
                    .insert(property.to_string(), binding_string(property, value)?);
            }
            "isExpanded" | "expanded" => self.is_expanded = Some(binding_bool(property, value)?),
            "aria-expanded" => {
                self.is_expanded = Some(binding_bool(property, value)?);
                self.attributes
                    .insert(property.to_string(), binding_string(property, value)?);
            }
            "min" | "minValue" => self.min_value = Some(binding_number(property, value)?),
            "max" | "maxValue" => self.max_value = Some(binding_number(property, value)?),
            "step" | "stepValue" => self.step_value = Some(binding_number(property, value)?),
            "valueNumber" => self.value_number = Some(binding_number(property, value)?),
            "selectedKeys"
            | "defaultSelectedKeys"
            | "disabledKeys"
            | "expandedKeys"
            | "defaultExpandedKeys" => {
                self.attributes.insert(
                    property.to_string(),
                    binding_payload_string(property, value)?,
                );
            }
            other if other.starts_with("on") => {
                self.events.insert(
                    normalize_event_name(other),
                    binding_string(property, value)?,
                );
            }
            other if is_action_payload_property(other) => {
                self.attributes
                    .insert(other.to_string(), binding_payload_string(property, value)?);
            }
            other if other.starts_with("aria-") || other.starts_with("data-") => {
                self.attributes
                    .insert(other.to_string(), binding_string(property, value)?);
            }
            other => {
                self.attributes
                    .insert(other.to_string(), binding_string(property, value)?);
            }
        }
        Ok(())
    }
}

fn canonical_prop_name(name: &str) -> String {
    match name {
        "class" | "className" => "className".to_string(),
        "aria-label" | "ariaLabel" => "aria-label".to_string(),
        "disabled" | "aria-disabled" | "isDisabled" => "isDisabled".to_string(),
        "required" | "aria-required" | "isRequired" => "isRequired".to_string(),
        "invalid" | "aria-invalid" | "isInvalid" => "isInvalid".to_string(),
        "readOnly" | "readonly" | "aria-readonly" | "isReadOnly" => "isReadOnly".to_string(),
        "selected" | "aria-selected" | "isSelected" => "isSelected".to_string(),
        "checked" | "aria-checked" | "isChecked" => "isChecked".to_string(),
        "expanded" | "aria-expanded" | "isExpanded" => "isExpanded".to_string(),
        "min" | "minValue" => "minValue".to_string(),
        "max" | "maxValue" => "maxValue".to_string(),
        "step" | "stepValue" => "stepValue".to_string(),
        "type" | "inputType" => "inputType".to_string(),
        other if other.starts_with("on") => normalize_event_name(other),
        other => other.to_string(),
    }
}

fn normalize_event_name(name: &str) -> String {
    match name {
        "onclick" => "onClick",
        "onpress" => "onPress",
        "onpressstart" => "onPressStart",
        "onpressend" => "onPressEnd",
        "onpressup" => "onPressUp",
        "onpresschange" => "onPressChange",
        "onchange" => "onChange",
        "oninput" => "onInput",
        "onselectionchange" => "onSelectionChange",
        "onfocus" => "onFocus",
        "onblur" => "onBlur",
        "onfocuschange" => "onFocusChange",
        "onfocuswithin" => "onFocusWithin",
        "onblurwithin" => "onBlurWithin",
        "onfocuswithinchange" => "onFocusWithinChange",
        "ontoggle" => "onToggle",
        "onexpandedchange" => "onExpandedChange",
        "onhoverstart" => "onHoverStart",
        "onhoverend" => "onHoverEnd",
        "onhoverchange" => "onHoverChange",
        "onkeydown" => "onKeyDown",
        "onkeyup" => "onKeyUp",
        "oncopy" => "onCopy",
        "oncut" => "onCut",
        "onpaste" => "onPaste",
        _ => name,
    }
    .to_string()
}

fn parse_style_text(style: &str) -> BTreeMap<String, CompiledStyleValue> {
    style
        .split(';')
        .filter_map(|declaration| {
            let (property, value) = declaration.split_once(':')?;
            let property = property.trim();
            let value = value.trim();
            if property.is_empty() || value.is_empty() {
                return None;
            }
            Some((
                property.to_string(),
                value
                    .parse::<f64>()
                    .map(CompiledStyleValue::Number)
                    .unwrap_or_else(|_| CompiledStyleValue::String(value.to_string())),
            ))
        })
        .collect()
}

fn is_action_payload_property(property: &str) -> bool {
    matches!(
        property,
        "actionPayload" | "action-payload" | "data-action-payload" | "data-a3s-action-payload"
    )
}

fn binding_payload_string(property: &str, value: &JsonValue) -> GuiResult<String> {
    match value {
        JsonValue::String(value) => Ok(value.clone()),
        JsonValue::Number(value) => Ok(value.to_string()),
        JsonValue::Bool(value) => Ok(value.to_string()),
        JsonValue::Null => Ok(String::new()),
        JsonValue::Array(_) | JsonValue::Object(_) => {
            serde_json::to_string(value).map_err(|error| {
                GuiError::invalid_tree(format!(
                    "RSX binding for property {property:?} could not serialize action payload: {error}"
                ))
            })
        }
    }
}

fn binding_string(property: &str, value: &JsonValue) -> GuiResult<String> {
    match value {
        JsonValue::String(value) => Ok(value.clone()),
        JsonValue::Number(value) => Ok(value.to_string()),
        JsonValue::Bool(value) => Ok(value.to_string()),
        JsonValue::Null => Ok(String::new()),
        JsonValue::Array(_) | JsonValue::Object(_) => Err(GuiError::invalid_tree(format!(
            "RSX binding for property {property:?} must resolve to a scalar value"
        ))),
    }
}

fn binding_bool(property: &str, value: &JsonValue) -> GuiResult<bool> {
    match value {
        JsonValue::Bool(value) => Ok(*value),
        JsonValue::String(value) if value == "true" => Ok(true),
        JsonValue::String(value) if value == "false" => Ok(false),
        _ => Err(GuiError::invalid_tree(format!(
            "RSX binding for property {property:?} must resolve to a boolean"
        ))),
    }
}

fn binding_number(property: &str, value: &JsonValue) -> GuiResult<f64> {
    match value {
        JsonValue::Number(value) => {
            value
                .as_f64()
                .filter(|value| value.is_finite())
                .ok_or_else(|| {
                    GuiError::invalid_tree(format!(
                        "RSX binding for property {property:?} must resolve to a finite number"
                    ))
                })
        }
        JsonValue::String(value) => value
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .ok_or_else(|| {
                GuiError::invalid_tree(format!(
                    "RSX binding for property {property:?} must resolve to a finite number"
                ))
            }),
        _ => Err(GuiError::invalid_tree(format!(
            "RSX binding for property {property:?} must resolve to a number"
        ))),
    }
}

#[derive(Debug, Clone, Default)]
pub struct RsxCompilerBridge {
    mapper: SemanticMapper,
}

impl RsxCompilerBridge {
    pub fn new() -> Self {
        Self {
            mapper: SemanticMapper::new(),
        }
    }

    pub fn lower_to_semantic(&self, node: &CompiledRsxNode) -> GuiResult<SemanticElement> {
        node.validate()?;
        lower_node(node)
    }

    pub fn lower_to_native(&self, node: &CompiledRsxNode) -> GuiResult<NativeElement> {
        let semantic = self.lower_to_semantic(node)?;
        self.mapper.map(&semantic)
    }
}

fn lower_node(node: &CompiledRsxNode) -> GuiResult<SemanticElement> {
    match node {
        CompiledRsxNode::Text { key, value } => {
            Ok(SemanticElement::text(key.clone(), value.clone()))
        }
        CompiledRsxNode::Element {
            key,
            tag,
            props,
            children,
            ..
        } => {
            let component = component_from_rsx_tag(tag, props)?;
            let mut element = SemanticElement::new(key.clone(), component)
                .with_props(props.clone().into_semantic_props_for_tag(tag, children));
            element.children = children
                .iter()
                .map(lower_node)
                .collect::<GuiResult<Vec<_>>>()?;
            Ok(element)
        }
    }
}

#[cfg(test)]
mod tests;
