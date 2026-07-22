use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use gtk::gio;
use gtk::prelude::*;
use gtk4_crate as gtk;

use crate::event::{NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::platform::NativeWidgetConfig;

#[derive(Debug, Clone)]
pub struct Gtk4Menu {
    pub(crate) model: gio::Menu,
    pub(crate) bar: gtk::PopoverMenuBar,
}

impl Gtk4Menu {
    pub(crate) fn new() -> Self {
        let model = gio::Menu::new();
        let bar = gtk::PopoverMenuBar::from_model(Some(&model));
        Self { model, bar }
    }
}

#[derive(Debug, Clone)]
pub struct Gtk4MenuItem {
    pub(crate) item: gio::MenuItem,
    pub(crate) action_name: String,
    pub(crate) label: String,
    pub(crate) value: String,
    pub(crate) selected: bool,
}

impl Gtk4MenuItem {
    pub(crate) fn from_config(
        id: HostNodeId,
        config: &NativeWidgetConfig,
        application: &gtk::Application,
        events: Rc<RefCell<Vec<NativeEvent>>>,
        events_suppressed: Rc<RefCell<bool>>,
    ) -> Self {
        let label = config
            .label
            .clone()
            .or_else(|| config.value.clone())
            .unwrap_or_default();
        let value = config.value.clone().unwrap_or_else(|| label.clone());
        let action_name = format!("a3s_menu_{}", id.get());
        let detailed_action = format!("app.{action_name}");
        let item = gio::MenuItem::new(Some(&label), Some(&detailed_action));
        let action = gio::SimpleAction::new(&action_name, None);

        action.connect_activate(move |_, _| {
            if !*events_suppressed.borrow() {
                events
                    .borrow_mut()
                    .push(NativeEvent::new(id, NativeEventKind::Press));
            }
        });
        application.add_action(&action);

        Self {
            item,
            action_name,
            label,
            value,
            selected: config.selected,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Gtk4MenuRegistry {
    menus: BTreeMap<HostNodeId, Gtk4Menu>,
    items: BTreeMap<HostNodeId, Gtk4MenuItem>,
    children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    item_parents: BTreeMap<HostNodeId, HostNodeId>,
}

impl Gtk4MenuRegistry {
    pub(crate) fn register_menu(&mut self, id: HostNodeId, menu: Gtk4Menu) {
        self.menus.insert(id, menu);
        self.children.entry(id).or_default();
    }

    pub(crate) fn register_item(&mut self, id: HostNodeId, item: Gtk4MenuItem) {
        self.items.insert(id, item);
    }

    pub(crate) fn insert_item(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        fallback: &Gtk4MenuItem,
        index: usize,
    ) {
        self.items.entry(child).or_insert_with(|| fallback.clone());
        self.detach_item(child);

        let children = self.children.entry(parent).or_default();
        let index = index.min(children.len());
        children.insert(index, child);
        self.item_parents.insert(child, parent);
        self.rebuild_menu(parent);
    }

    pub(crate) fn update_item_label(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4MenuItem,
        label: String,
    ) {
        let item = self.items.entry(id).or_insert_with(|| fallback.clone());
        if item.value == item.label {
            item.value = label.clone();
        }
        item.label = label.clone();
        item.item.set_label(Some(&label));
        self.rebuild_for_item(id);
    }

    pub(crate) fn update_item_value(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4MenuItem,
        value: String,
    ) {
        self.items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .value = value;
    }

    pub(crate) fn update_item_selected(
        &mut self,
        id: HostNodeId,
        fallback: &Gtk4MenuItem,
        selected: bool,
    ) {
        self.items
            .entry(id)
            .or_insert_with(|| fallback.clone())
            .selected = selected;
    }

    pub(crate) fn remove_menu(&mut self, id: HostNodeId) {
        self.menus.remove(&id);
        if let Some(children) = self.children.remove(&id) {
            for child in children {
                self.item_parents.remove(&child);
            }
        }
    }

    pub(crate) fn remove_item(&mut self, id: HostNodeId, application: &gtk::Application) {
        self.detach_item(id);
        if let Some(item) = self.items.remove(&id) {
            application.remove_action(&item.action_name);
        }
    }

    fn rebuild_for_item(&mut self, id: HostNodeId) {
        if let Some(parent) = self.item_parents.get(&id).copied() {
            self.rebuild_menu(parent);
        }
    }

    fn rebuild_menu(&self, id: HostNodeId) {
        let Some(menu) = self.menus.get(&id) else {
            return;
        };
        menu.model.remove_all();
        if let Some(children) = self.children.get(&id) {
            for child in children {
                if let Some(item) = self.items.get(child) {
                    menu.model.append_item(&item.item);
                }
            }
        }
    }

    fn detach_item(&mut self, child: HostNodeId) {
        let Some(parent) = self.item_parents.remove(&child) else {
            return;
        };
        if let Some(children) = self.children.get_mut(&parent) {
            children.retain(|existing| *existing != child);
        }
        self.rebuild_menu(parent);
    }
}
