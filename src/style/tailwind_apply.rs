use super::*;

impl PortableStyle {
    pub(super) fn apply_tailwind_utility(&mut self, class: &str) {
        let Some(class) = tailwind::parse_class(class) else {
            return;
        };
        let variants = class.variants;
        let class = class.utility;
        if class.is_empty() {
            return;
        }
        let declarations = tailwind_utility_declarations(class);
        if !variants.is_empty() {
            let variant_key = tailwind::variant_key(&variants);
            for (property, value) in declarations {
                self.record_variant_declaration(variant_key.as_str(), property, value);
            }
            return;
        }
        for (property, value) in declarations {
            self.apply(&property, &value);
        }
        if let Some(arbitrary) = class
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))
        {
            if let Some((property, value)) = arbitrary.split_once(':') {
                self.apply(property.trim(), &tailwind_arbitrary_value(value.trim()));
            }
            return;
        }
        match class {
            "flex" => self.display = Some(DisplayMode::Flex),
            "inline-flex" => self.display = Some(DisplayMode::InlineFlex),
            "block" => self.display = Some(DisplayMode::Block),
            "inline-block" => self.display = Some(DisplayMode::InlineBlock),
            "inline" => self.display = Some(DisplayMode::Inline),
            "grid" => self.display = Some(DisplayMode::Grid),
            "inline-grid" => self.display = Some(DisplayMode::InlineGrid),
            "flow-root" => self.display = Some(DisplayMode::FlowRoot),
            "contents" => self.display = Some(DisplayMode::Contents),
            "list-item" => self.display = Some(DisplayMode::ListItem),
            "table" => self.display = Some(DisplayMode::Table),
            "inline-table" => self.display = Some(DisplayMode::InlineTable),
            "table-caption" => self.display = Some(DisplayMode::TableCaption),
            "table-cell" => self.display = Some(DisplayMode::TableCell),
            "table-column" => self.display = Some(DisplayMode::TableColumn),
            "table-column-group" => self.display = Some(DisplayMode::TableColumnGroup),
            "table-footer-group" => self.display = Some(DisplayMode::TableFooterGroup),
            "table-header-group" => self.display = Some(DisplayMode::TableHeaderGroup),
            "table-row-group" => self.display = Some(DisplayMode::TableRowGroup),
            "table-row" => self.display = Some(DisplayMode::TableRow),
            "line-clamp-1" | "line-clamp-2" | "line-clamp-3" | "line-clamp-4" | "line-clamp-5"
            | "line-clamp-6" => self.display = Some(DisplayMode::WebkitBox),
            "line-clamp-none" => self.display = Some(DisplayMode::Block),
            "hidden" => self.display = Some(DisplayMode::None),
            "static" => self.position = Some(PositionMode::Static),
            "fixed" => self.position = Some(PositionMode::Fixed),
            "absolute" => self.position = Some(PositionMode::Absolute),
            "relative" => self.position = Some(PositionMode::Relative),
            "sticky" => self.position = Some(PositionMode::Sticky),
            "flex-row" | "flex-row-reverse" => self.flex_direction = Some(Orientation::Horizontal),
            "flex-col" | "flex-col-reverse" => self.flex_direction = Some(Orientation::Vertical),
            "flex-wrap" => self.flex_wrap = Some(FlexWrap::Wrap),
            "flex-nowrap" => self.flex_wrap = Some(FlexWrap::NoWrap),
            "flex-wrap-reverse" => self.flex_wrap = Some(FlexWrap::WrapReverse),
            "items-start" => self.align_items = Some(AlignItems::Start),
            "items-center" => self.align_items = Some(AlignItems::Center),
            "items-end" => self.align_items = Some(AlignItems::End),
            "items-stretch" => self.align_items = Some(AlignItems::Stretch),
            "items-baseline" => self.align_items = Some(AlignItems::Baseline),
            "justify-start" => self.justify_content = Some(JustifyContent::Start),
            "justify-center" => self.justify_content = Some(JustifyContent::Center),
            "justify-end" => self.justify_content = Some(JustifyContent::End),
            "justify-between" => self.justify_content = Some(JustifyContent::SpaceBetween),
            "justify-around" => self.justify_content = Some(JustifyContent::SpaceAround),
            "justify-evenly" => self.justify_content = Some(JustifyContent::SpaceEvenly),
            "rounded" => self.border_radius = Some(StyleLength::Points(4.0)),
            "rounded-none" => self.border_radius = Some(StyleLength::Points(0.0)),
            "rounded-xs" => self.border_radius = Some(StyleLength::Points(2.0)),
            "rounded-sm" => self.border_radius = Some(StyleLength::Points(6.0)),
            "rounded-md" => self.border_radius = Some(StyleLength::Points(8.0)),
            "rounded-lg" => self.border_radius = Some(StyleLength::Points(12.0)),
            "rounded-xl" => self.border_radius = Some(StyleLength::Points(16.0)),
            "rounded-2xl" => self.border_radius = Some(StyleLength::Points(16.0)),
            "rounded-3xl" => self.border_radius = Some(StyleLength::Points(24.0)),
            "rounded-4xl" => self.border_radius = Some(StyleLength::Points(32.0)),
            "rounded-full" => {
                self.border_radius = Some(StyleLength::Css("calc(infinity * 1px)".to_string()));
            }
            _ => self.apply_tailwind_prefixed_utility(class),
        }
    }

    pub(super) fn apply_tailwind_prefixed_utility(&mut self, class: &str) {
        if let Some(value) = class.strip_prefix("w-").and_then(tailwind_length) {
            self.width = Some(value);
        } else if let Some(value) = class.strip_prefix("h-").and_then(tailwind_length) {
            self.height = Some(value);
        } else if let Some(value) = class.strip_prefix("size-").and_then(tailwind_length) {
            self.width = Some(value.clone());
            self.height = Some(value);
        } else if let Some(value) = class.strip_prefix("min-w-").and_then(tailwind_length) {
            self.min_width = Some(value);
        } else if let Some(value) = class.strip_prefix("min-h-").and_then(tailwind_length) {
            self.min_height = Some(value);
        } else if let Some(value) = class
            .strip_prefix("max-w-")
            .and_then(tailwind_max_width_value)
        {
            self.max_width = Some(value);
        } else if let Some(value) = class.strip_prefix("max-h-").and_then(tailwind_length) {
            self.max_height = Some(value);
        } else if let Some(value) = class.strip_prefix("gap-").and_then(tailwind_length) {
            self.gap = Some(value);
        } else if let Some(value) = class.strip_prefix("gap-x-").and_then(tailwind_length) {
            self.column_gap = Some(value);
        } else if let Some(value) = class.strip_prefix("gap-y-").and_then(tailwind_length) {
            self.row_gap = Some(value);
        } else if let Some(opacity) = class.strip_prefix("opacity-").and_then(tailwind_opacity) {
            self.opacity = Some(opacity);
        } else if let Some(color) = class.strip_prefix("bg-").and_then(tailwind_color) {
            self.background_color = Some(color);
        } else if let Some(color) = class.strip_prefix("text-").and_then(tailwind_color) {
            self.color = Some(color);
        } else if let Some((edges, value)) = tailwind_edge_utility(class, "p") {
            self.padding.apply_edges(edges, value);
        } else if let Some((edges, value)) = tailwind_edge_utility(class, "m") {
            self.margin.apply_edges(edges, value);
        }
    }
}
