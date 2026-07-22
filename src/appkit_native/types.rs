use super::*;

#[derive(Debug, Clone)]
pub struct AppKitOsHandle {
    pub id: HostNodeId,
    pub kind: AppKitWidgetKind,
    pub selected: bool,
    pub widget: AppKitOsWidget,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppKitComboBoxItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
    pub visible: bool,
    pub style: PortableStyle,
}

#[derive(Debug, Clone)]
pub(super) struct AppKitListViewState {
    pub(super) stack_view: Retained<NSStackView>,
    pub(super) rows: Rc<RefCell<Vec<AppKitListRow>>>,
    pub(super) style: PortableStyle,
}

#[derive(Debug, Clone)]
pub(super) struct AppKitListRow {
    pub(super) node: HostNodeId,
    pub(super) button: Retained<NSButton>,
    _label: Retained<NSTextField>,
    _target: Retained<AppKitActionTarget>,
    _constraints: Vec<Retained<NSLayoutConstraint>>,
    pub(super) value: String,
}

impl AppKitListRow {
    pub(super) fn new(
        node: HostNodeId,
        parent: HostNodeId,
        item: AppKitComboBoxItem,
        selected: bool,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        activation_contexts: interaction::AppKitActivationContexts,
        focused_node: Rc<Cell<Option<HostNodeId>>>,
        mtm: MainThreadMarker,
    ) -> Self {
        let label_text = if item.label.trim().is_empty() {
            item.value.as_str()
        } else {
            item.label.as_str()
        };
        let title = ns_string(label_text);
        let target = AppKitActionTarget::new_selection(
            parent,
            events,
            activation_contexts,
            focused_node,
            item.value.clone(),
            mtm,
        );
        let empty_title = ns_string("");
        let button = unsafe {
            NSButton::buttonWithTitle_target_action(
                &empty_title,
                Some(target.as_any_object()),
                Some(sel!(a3sGuiPress:)),
                mtm,
            )
        };
        let row_width = row_width_from_style(&item.style);
        let row_height = row_height_from_style(&item.style);
        button.setBordered(false);
        button.setRefusesFirstResponder(false);
        button.setContentTintColor(Some(&NSColor::labelColor()));
        button
            .as_super()
            .setFont(Some(&NSFont::systemFontOfSize(13.0)));
        button
            .as_super()
            .as_super()
            .setFrameSize(NSSize::new(row_width, row_height));
        button
            .as_super()
            .as_super()
            .setTranslatesAutoresizingMaskIntoConstraints(false);
        button.setState(appkit_state(selected));
        let label = NSTextField::labelWithString(&title, mtm);
        label
            .as_super()
            .setFont(Some(&NSFont::systemFontOfSize(13.0)));
        label.setTextColor(Some(&NSColor::labelColor()));
        let left = edge_length_points(item.style.padding.left.as_ref()).unwrap_or(8.0);
        let right = edge_length_points(item.style.padding.right.as_ref()).unwrap_or(left);
        let label_height = 17.0;
        let label_y = ((row_height - label_height) / 2.0).max(0.0);
        label.as_super().as_super().setFrame(NSRect::new(
            NSPoint::new(left, label_y),
            NSSize::new((row_width - left - right).max(24.0), label_height),
        ));
        button
            .as_super()
            .as_super()
            .addSubview(label.as_super().as_super());
        let height_constraint = size_constraint(
            button.as_super().as_super(),
            NSLayoutAttribute::Height,
            NSLayoutRelation::Equal,
            row_height,
        );
        let width_constraint = size_constraint(
            button.as_super().as_super(),
            NSLayoutAttribute::Width,
            NSLayoutRelation::Equal,
            row_width,
        );
        height_constraint.setActive(true);
        width_constraint.setActive(true);
        Self {
            node,
            button,
            _label: label,
            _target: target,
            _constraints: vec![height_constraint, width_constraint],
            value: item.value,
        }
    }

    pub(super) fn button_view(&self) -> &NSView {
        self.button.as_super().as_super()
    }
}

#[derive(Debug, Clone)]
pub struct AppKitPopoverState {
    pub(super) popover: Retained<NSPopover>,
    pub(super) content_view_controller: Retained<NSViewController>,
    pub(super) content_view: Retained<NSView>,
}

#[derive(Debug, Clone)]
pub struct AppKitScrollViewState {
    pub(super) scroll_view: Retained<NSScrollView>,
    pub(super) stack_view: Retained<NSStackView>,
}

#[derive(Debug, Clone)]
pub(super) struct AppKitSizeConstraints {
    pub(super) active_constraints: Vec<Retained<NSLayoutConstraint>>,
}

#[derive(Debug, Clone)]
pub enum AppKitOsWidget {
    Window(Retained<NSWindow>),
    Panel(Retained<NSPanel>),
    Popover(AppKitPopoverState),
    Menu(Retained<NSMenu>),
    MenuItem(Retained<NSMenuItem>),
    View(Retained<NSView>),
    StackView(Retained<NSStackView>),
    Button(Retained<NSButton>),
    Switch(Retained<NSSwitch>),
    ComboBox(Retained<NSComboBox>),
    ComboBoxItem(AppKitComboBoxItem),
    ListView(Retained<NSScrollView>),
    ScrollView(AppKitScrollViewState),
    Slider(Retained<NSSlider>),
    ProgressIndicator(Retained<NSProgressIndicator>),
    TabView(Retained<NSTabView>),
    TabViewItem(Retained<NSTabViewItem>),
    Box(Retained<NSBox>),
    TextField(Retained<NSTextField>),
    SearchField(Retained<NSSearchField>),
    SecureTextField(Retained<NSSecureTextField>),
}

impl AppKitOsWidget {
    pub(super) fn as_responder(&self) -> Option<&NSResponder> {
        match self {
            AppKitOsWidget::Window(window) => Some(window.as_super()),
            AppKitOsWidget::Panel(panel) => Some(panel.as_super().as_super()),
            _ => self.as_view().map(NSView::as_super),
        }
    }

    pub(super) fn as_view(&self) -> Option<&NSView> {
        match self {
            AppKitOsWidget::Window(_)
            | AppKitOsWidget::Panel(_)
            | AppKitOsWidget::Popover(_)
            | AppKitOsWidget::Menu(_)
            | AppKitOsWidget::MenuItem(_) => None,
            AppKitOsWidget::View(view) => Some(view),
            AppKitOsWidget::StackView(stack_view) => Some(stack_view.as_super()),
            AppKitOsWidget::Button(button) => Some(button.as_super().as_super()),
            AppKitOsWidget::Switch(switch) => Some(switch.as_super().as_super()),
            AppKitOsWidget::ComboBox(combo_box) => Some(combo_box.as_super().as_super().as_super()),
            AppKitOsWidget::ListView(scroll_view) => Some(scroll_view.as_super()),
            AppKitOsWidget::ScrollView(state) => Some(state.scroll_view.as_super()),
            AppKitOsWidget::Slider(slider) => Some(slider.as_super().as_super()),
            AppKitOsWidget::ProgressIndicator(progress) => Some(progress.as_super()),
            AppKitOsWidget::TabView(tab_view) => Some(tab_view.as_super()),
            AppKitOsWidget::Box(box_) => Some(box_.as_super()),
            AppKitOsWidget::TextField(text_field) => Some(text_field.as_super().as_super()),
            AppKitOsWidget::SearchField(text_field) => {
                Some(text_field.as_super().as_super().as_super())
            }
            AppKitOsWidget::SecureTextField(text_field) => {
                Some(text_field.as_super().as_super().as_super())
            }
            AppKitOsWidget::ComboBoxItem(_) | AppKitOsWidget::TabViewItem(_) => None,
        }
    }

    pub(super) fn as_control(&self) -> Option<&objc2_app_kit::NSControl> {
        match self {
            AppKitOsWidget::Button(button) => Some(button.as_super()),
            AppKitOsWidget::Switch(switch) => Some(switch.as_super()),
            AppKitOsWidget::ComboBox(combo_box) => Some(combo_box.as_super().as_super()),
            AppKitOsWidget::Slider(slider) => Some(slider.as_super()),
            AppKitOsWidget::TextField(text_field) => Some(text_field.as_super()),
            AppKitOsWidget::SearchField(text_field) => Some(text_field.as_super().as_super()),
            AppKitOsWidget::SecureTextField(text_field) => Some(text_field.as_super().as_super()),
            AppKitOsWidget::Window(_)
            | AppKitOsWidget::Panel(_)
            | AppKitOsWidget::Popover(_)
            | AppKitOsWidget::Menu(_)
            | AppKitOsWidget::MenuItem(_)
            | AppKitOsWidget::View(_)
            | AppKitOsWidget::StackView(_)
            | AppKitOsWidget::ComboBoxItem(_)
            | AppKitOsWidget::ListView(_)
            | AppKitOsWidget::ScrollView(_)
            | AppKitOsWidget::TabView(_)
            | AppKitOsWidget::TabViewItem(_)
            | AppKitOsWidget::Box(_)
            | AppKitOsWidget::ProgressIndicator(_) => None,
        }
    }
}

pub(super) fn focus_appkit_widget(widget: &AppKitOsWidget) -> bool {
    let Some(responder) = widget.as_responder() else {
        return false;
    };

    if let Some(view) = widget.as_view() {
        return view
            .window()
            .is_some_and(|window| window.makeFirstResponder(Some(responder)));
    }

    match widget {
        AppKitOsWidget::Window(window) => window.makeFirstResponder(Some(responder)),
        AppKitOsWidget::Panel(panel) => panel.as_super().makeFirstResponder(Some(responder)),
        _ => false,
    }
}

pub(super) fn focus_appkit_view(view: &NSView) -> bool {
    view.window()
        .is_some_and(|window| window.makeFirstResponder(Some(view.as_super())))
}

pub(super) fn install_window_content_view(window: &NSWindow, child: &NSView) {
    let size = window.contentLayoutRect().size;
    child.setFrame(NSRect::new(NSPoint::new(0.0, 0.0), size));
    child.setAutoresizingMask(flexible_view_mask());
    window.setContentView(Some(child));
    interaction::install_pointer_tracking_area(window, child);
    child.setNeedsLayout(true);
    child.layoutSubtreeIfNeeded();
    window.displayIfNeeded();
}

pub(super) fn flexible_view_mask() -> NSAutoresizingMaskOptions {
    let mut mask = NSAutoresizingMaskOptions::ViewWidthSizable;
    mask.insert(NSAutoresizingMaskOptions::ViewHeightSizable);
    mask
}
