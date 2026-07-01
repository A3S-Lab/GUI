use std::collections::BTreeMap;

use objc2::rc::Retained;
use objc2_app_kit::{NSMenu, NSMenuItem};

use crate::error::{GuiError, GuiResult};
use crate::host::HostNodeId;

#[derive(Debug, Default)]
pub(crate) struct AppKitMenuRegistry {
    menus: BTreeMap<HostNodeId, Retained<NSMenu>>,
    children: BTreeMap<HostNodeId, Vec<HostNodeId>>,
    item_parents: BTreeMap<HostNodeId, HostNodeId>,
}

impl AppKitMenuRegistry {
    pub(crate) fn register_menu(&mut self, id: HostNodeId, menu: Retained<NSMenu>) {
        self.menus.insert(id, menu);
        self.children.entry(id).or_default();
    }

    pub(crate) fn insert_item(
        &mut self,
        parent: HostNodeId,
        menu: &NSMenu,
        child: HostNodeId,
        item: &NSMenuItem,
        index: usize,
    ) -> GuiResult<()> {
        self.detach_item(child)?;

        let children = self.children.entry(parent).or_default();
        let index = index.min(children.len());
        menu.insertItem_atIndex(
            item,
            index
                .try_into()
                .map_err(|_| GuiError::host("AppKit menu item insertion index overflow"))?,
        );
        children.insert(index, child);
        self.item_parents.insert(child, parent);
        Ok(())
    }

    pub(crate) fn remove_menu(&mut self, id: HostNodeId) {
        self.menus.remove(&id);
        if let Some(children) = self.children.remove(&id) {
            for child in children {
                self.item_parents.remove(&child);
            }
        }
    }

    pub(crate) fn remove_item(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.detach_item(id)
    }

    fn detach_item(&mut self, child: HostNodeId) -> GuiResult<()> {
        let Some(parent) = self.item_parents.remove(&child) else {
            return Ok(());
        };
        let Some(children) = self.children.get_mut(&parent) else {
            return Ok(());
        };
        let Some(index) = children.iter().position(|existing| *existing == child) else {
            return Ok(());
        };

        if let Some(menu) = self.menus.get(&parent) {
            menu.removeItemAtIndex(
                index
                    .try_into()
                    .map_err(|_| GuiError::host("AppKit menu child removal index overflow"))?,
            );
        }
        children.remove(index);
        Ok(())
    }
}
