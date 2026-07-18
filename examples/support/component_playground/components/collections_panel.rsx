use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CollectionsPanelProps {
    pub record: String,
    pub set_value: String,
    pub selected_value: String,
}

#[allow(non_snake_case)]
pub fn collections_panel(cx: &mut ComponentCx<CollectionsPanelProps>) -> RSX {
    let record = cx.use_prop("record", |props: &CollectionsPanelProps| {
        props.record.clone()
    });
    let setValue = cx.use_prop("setValue", |props: &CollectionsPanelProps| {
        props.set_value.clone()
    });
    let selectedValue = cx.use_prop("selectedValue", |props: &CollectionsPanelProps| {
        props.selected_value.clone()
    });

    a3s_gui::rsx!(
        <PlaygroundSection
            key="root"
            title="Collections"
            description="Selection inputs, lists, grid lists, tags, trees, and menus rendered as composed collection contexts."
        >
            <UiToolbar key="root-stack" orientation="vertical" className="w-full gap-5 rounded-none border-none bg-transparent p-0">
                <UiToolbar key="pickers" orientation="horizontal" className="grid w-full grid-cols-3 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiSelect key="select" label="Theme" value={selectedValue} placeholder="Theme" isOpen={false} onSelectionChange={setValue} className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                        <UiSelectValue key="select-value" value="Compact" placeholder="Theme" />
                        <UiListBoxItem key="select-compact" value="compact" textValue="Compact" isSelected={true}>Compact</UiListBoxItem>
                        <UiListBoxItem key="select-comfortable" value="comfortable" textValue="Comfortable">Comfortable</UiListBoxItem>
                    </UiSelect>
                    <UiComboBox key="combo-box" label="Assignee" value="ada" inputValue="Ada" placeholder="Assignee" isOpen={false} onChange={setValue} onSelectionChange={setValue} className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                        <UiComboBoxValue key="combo-box-value" value="Ada Lovelace" placeholder="Assignee" />
                        <UiListBoxItem key="combo-ada" value="ada" textValue="Ada Lovelace" isSelected={true}>Ada Lovelace</UiListBoxItem>
                        <UiListBoxItem key="combo-grace" value="grace" textValue="Grace Hopper">Grace Hopper</UiListBoxItem>
                    </UiComboBox>
                    <UiAutocomplete key="autocomplete" label="Search people" value="ada" inputValue="Ada" placeholder="Search" onChange={setValue} onSelectionChange={setValue} className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                        <UiListBoxItem key="autocomplete-ada" value="ada" textValue="Ada Lovelace">Ada Lovelace</UiListBoxItem>
                        <UiListBoxItem key="autocomplete-grace" value="grace" textValue="Grace Hopper">Grace Hopper</UiListBoxItem>
                    </UiAutocomplete>
                </UiToolbar>
                <UiToolbar key="lists" orientation="horizontal" className="grid w-full grid-cols-3 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiListBox key="list-box" label="People" value="ada" selectionMode="single" onSelectionChange={setValue} className="gap-2 rounded-md border border-hairline bg-canvas p-3">
                        <UiListBoxSection key="list-section" label="People" className="w-full">
                            <UiListBoxHeader key="list-header" label="People" textValue="People">People</UiListBoxHeader>
                        </UiListBoxSection>
                        <UiListBoxItem key="list-item-ada" value="ada" textValue="Ada Lovelace" isSelected={true}>Ada Lovelace</UiListBoxItem>
                        <UiListBoxItem key="list-item-grace" value="grace" textValue="Grace Hopper">Grace Hopper</UiListBoxItem>
                        <UiListBoxLoadMoreItem key="list-load-more" label="Load more people" onPress={record} isLoading={false}>Load more</UiListBoxLoadMoreItem>
                    </UiListBox>
                    <UiGridList key="grid-list" label="Files" value="app" selectionMode="single" onSelectionChange={setValue} className="gap-2 rounded-md border border-hairline bg-canvas p-3">
                        <UiGridListSection key="grid-section" label="Source files" className="w-full">
                            <UiGridListHeader key="grid-header" label="Source files" textValue="Source files">Source files</UiGridListHeader>
                        </UiGridListSection>
                        <UiGridListItem key="grid-item-app" value="app" textValue="app.rsx" isSelected={true}>app.rsx</UiGridListItem>
                        <UiGridListItem key="grid-item-shell" value="shell" textValue="shell.rsx">shell.rsx</UiGridListItem>
                        <UiGridListLoadMoreItem key="grid-load-more" label="Load more files" onPress={record} isLoading={false}>Load more</UiGridListLoadMoreItem>
                    </UiGridList>
                    <UiCollection key="collection" label="Recent items" itemCount={3} className="gap-2 rounded-md border border-hairline bg-canvas p-3">
                        <UiText key="collection-title" label="Recent items" className="font-medium text-ink" />
                        <UiText key="collection-row-one" label="Design tokens" className="text-sm text-body" />
                        <UiText key="collection-row-two" label="Native events" className="text-sm text-body" />
                        <UiText key="collection-row-three" label="RSX hooks" className="text-sm text-body" />
                    </UiCollection>
                </UiToolbar>
                <UiToolbar key="taxonomies" orientation="horizontal" className="grid w-full grid-cols-3 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiTagGroup key="tag-group" label="Labels" value="preview" selectionMode="multiple" onSelectionChange={setValue} className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                        <UiTagList key="tag-list" label="Active labels" value="preview" selectionMode="multiple" className="w-full gap-2">
                            <UiTag key="tag-preview" value="preview" textValue="Preview" isSelected={true} onRemove={record}>Preview</UiTag>
                            <UiTag key="tag-native" value="native" textValue="Native" onRemove={record}>Native</UiTag>
                            <UiTag key="tag-rsx" value="rsx" textValue="RSX" onRemove={record}>RSX</UiTag>
                        </UiTagList>
                    </UiTagGroup>
                    <UiTree key="tree" label="Workspace files" value="src" selectionMode="single" onSelectionChange={setValue} className="gap-2 rounded-md border border-hairline bg-canvas p-3">
                        <UiTreeSection key="tree-section" label="Workspace" className="w-full">
                            <UiTreeHeader key="tree-header" label="Workspace" textValue="Workspace">Workspace</UiTreeHeader>
                        </UiTreeSection>
                        <UiTreeItem key="tree-item-src" value="src" textValue="src" isExpanded={true} isSelected={true}>
                            <UiTreeItemContent key="tree-item-content">
                                <UiText key="tree-name" label="src/rsx_ui" className="text-sm text-ink" />
                            </UiTreeItemContent>
                        </UiTreeItem>
                        <UiTreeItem key="tree-item-readme" value="readme" textValue="README.md">README.md</UiTreeItem>
                        <UiTreeLoadMoreItem key="tree-load-more" label="Load more files" onPress={record} isLoading={false}>Load more</UiTreeLoadMoreItem>
                    </UiTree>
                    <UiGroup key="menu-card" label="Menu actions" className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                        <UiMenuTrigger key="menu-trigger" isOpen={false} onPress={record}>Open menu</UiMenuTrigger>
                        <UiMenu key="menu" label="File actions" className="w-full">
                            <UiMenuItem key="menu-save" textValue="Save" onAction={record}>Save</UiMenuItem>
                            <UiMenuItem key="menu-duplicate" textValue="Duplicate" onAction={record}>Duplicate</UiMenuItem>
                            <UiSubmenuTrigger key="submenu-trigger" onPress={record} isOpen={false}>Export</UiSubmenuTrigger>
                        </UiMenu>
                        <UiMenuSection key="menu-section" label="File" className="w-full rounded-md border border-hairline bg-canvas-soft px-2 py-1">
                            <UiText key="menu-section-label" label="File section" className="text-sm text-body" />
                        </UiMenuSection>
                    </UiGroup>
                </UiToolbar>
            </UiToolbar>
        </PlaygroundSection>
    )
}
