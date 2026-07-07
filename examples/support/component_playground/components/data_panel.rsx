use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DataPanelProps {
    pub record: String,
    pub set_value: String,
    pub selected_value: String,
}

#[allow(non_snake_case)]
pub fn data_panel(cx: &mut ComponentCx<DataPanelProps>) -> RSX {
    let record = cx.use_prop("record", |props: &DataPanelProps| props.record.clone());
    let setValue = cx.use_prop("setValue", |props: &DataPanelProps| props.set_value.clone());
    let selectedValue = cx.use_prop("selectedValue", |props: &DataPanelProps| {
        props.selected_value.clone()
    });

    a3s_gui::rsx!(
        <PlaygroundSection
            key="root"
            title="Data"
            description="Tables, resizable columns, captions, load-more rows, and tabbed panels."
        >
            <UiToolbar key="root-stack" orientation="vertical" className="w-full gap-5 rounded-none border-none bg-transparent p-0">
                <UiResizableTableContainer key="resizable-table-container" label="Members" className="w-full bg-canvas">
                    <UiTable key="table" label="Members" className="w-full">
                        <UiTableCaption key="table-caption" label="Team members" textValue="Team members">Team members</UiTableCaption>
                        <UiTableHeader key="table-header">
                            <UiTableRow key="header-row">
                                <UiTableColumn key="name-column" label="Name" textValue="Name">Name<UiColumnResizer key="column-resizer" onPress={record} isResizing={false} valueNumber={150} minValue={80} maxValue={260} /></UiTableColumn>
                                <UiTableColumn key="role-column" label="Role" textValue="Role">Role</UiTableColumn>
                                <UiTableColumn key="status-column" label="Status" textValue="Status">Status</UiTableColumn>
                            </UiTableRow>
                        </UiTableHeader>
                        <UiTableBody key="table-body">
                            <UiTableRow key="ada-row" isSelected={true}>
                                <UiTableCell key="ada-name" textValue="Ada Lovelace">Ada Lovelace</UiTableCell>
                                <UiTableCell key="ada-role" textValue="Compiler">Compiler</UiTableCell>
                                <UiTableCell key="ada-status" textValue="Active">Active</UiTableCell>
                            </UiTableRow>
                            <UiTableRow key="grace-row">
                                <UiTableCell key="grace-name" textValue="Grace Hopper">Grace Hopper</UiTableCell>
                                <UiTableCell key="grace-role" textValue="Runtime">Runtime</UiTableCell>
                                <UiTableCell key="grace-status" textValue="Ready">Ready</UiTableCell>
                            </UiTableRow>
                            <UiTableLoadMoreItem key="table-load-more" label="Load more members" onPress={record} isLoading={false}>Load more members</UiTableLoadMoreItem>
                        </UiTableBody>
                        <UiTableFooter key="table-footer">
                            <UiTableRow key="footer-row">
                                <UiTableCell key="footer-cell" textValue="2 members">2 members</UiTableCell>
                            </UiTableRow>
                        </UiTableFooter>
                    </UiTable>
                </UiResizableTableContainer>
                <UiToolbar key="aliases-tabs" orientation="horizontal" className="grid w-full grid-cols-2 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiTable key="alias-table" label="Alias table" className="rounded-lg border border-hairline-strong bg-canvas p-3">
                        <UiTableHeader key="alias-header">
                            <UiRow key="alias-header-row">
                                <UiColumn key="alias-key-column" label="Alias" textValue="Alias">Alias</UiColumn>
                                <UiColumn key="alias-target-column" label="Target" textValue="Target">Target</UiColumn>
                            </UiRow>
                        </UiTableHeader>
                        <UiTableBody key="alias-body">
                            <UiRow key="row-alias-row" isSelected={true}>
                                <UiCell key="row-alias-cell" textValue="UiRow">UiRow</UiCell>
                                <UiCell key="row-target-cell" textValue="UiTableRow">UiTableRow</UiCell>
                            </UiRow>
                            <UiRow key="cell-alias-row">
                                <UiCell key="cell-alias-cell" textValue="UiCell">UiCell</UiCell>
                                <UiCell key="cell-target-cell" textValue="UiTableCell">UiTableCell</UiCell>
                            </UiRow>
                        </UiTableBody>
                    </UiTable>
                    <UiToolbar key="tabs-stack" orientation="vertical" className="gap-4 rounded-none border-none bg-transparent p-0">
                        <UiTabs key="tabs" value={selectedValue} onSelectionChange={setValue} className="gap-3 rounded-lg border border-hairline-strong bg-canvas p-4">
                            <UiTabsList key="tabs-list" label="Settings tabs">
                                <UiTabsTrigger key="profile-trigger" value="compact">Profile</UiTabsTrigger>
                                <UiTabsTrigger key="billing-trigger" value="billing">Billing</UiTabsTrigger>
                            </UiTabsList>
                            <UiTabsContent key="tabs-content" value="compact" className="rounded-md border border-hairline bg-canvas-soft p-3">
                                <UiText key="tabs-copy" label="Profile settings" className="text-sm text-ink" />
                            </UiTabsContent>
                        </UiTabs>
                        <UiTabs key="alias-tabs" value="overview" onSelectionChange={setValue} className="gap-3 rounded-lg border border-hairline-strong bg-canvas p-4">
                            <UiTabList key="tab-list" label="Alias tabs">
                                <UiTab key="overview-tab" value="overview">Overview</UiTab>
                                <UiTab key="events-tab" value="events">Events</UiTab>
                            </UiTabList>
                            <UiTabPanels key="tab-panels">
                                <UiTabPanel key="tab-panel" value="overview" className="rounded-md border border-hairline bg-canvas-soft p-3">
                                    <UiText key="alias-tabs-copy" label="Alias tab panel" className="text-sm text-ink" />
                                </UiTabPanel>
                            </UiTabPanels>
                        </UiTabs>
                    </UiToolbar>
                </UiToolbar>
            </UiToolbar>
        </PlaygroundSection>
    )
}
