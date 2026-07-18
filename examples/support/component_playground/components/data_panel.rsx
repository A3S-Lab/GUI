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
                <UiToolbar key="details-tabs" orientation="horizontal" className="grid w-full grid-cols-2 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiTable key="details-table" label="Release detail table" className="rounded-md border border-hairline bg-canvas p-3">
                        <UiTableHeader key="details-header">
                            <UiTableRow key="details-header-row">
                                <UiTableColumn key="details-key-column" label="Metric" textValue="Metric">Metric</UiTableColumn>
                                <UiTableColumn key="details-value-column" label="Value" textValue="Value">Value</UiTableColumn>
                            </UiTableRow>
                        </UiTableHeader>
                        <UiTableBody key="details-body">
                            <UiTableRow key="latency-row" isSelected={true}>
                                <UiTableCell key="latency-key-cell" textValue="Render latency">Render latency</UiTableCell>
                                <UiTableCell key="latency-value-cell" textValue="Stable">Stable</UiTableCell>
                            </UiTableRow>
                            <UiTableRow key="coverage-row">
                                <UiTableCell key="coverage-key-cell" textValue="Coverage">Coverage</UiTableCell>
                                <UiTableCell key="coverage-value-cell" textValue="Native components">Native components</UiTableCell>
                            </UiTableRow>
                        </UiTableBody>
                    </UiTable>
                    <UiToolbar key="tabs-stack" orientation="vertical" className="gap-4 rounded-none border-none bg-transparent p-0">
                        <UiTabs key="tabs" value={selectedValue} onSelectionChange={setValue} className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                            <UiTabsList key="tabs-list" label="Settings tabs">
                                <UiTabsTrigger key="profile-trigger" value="compact">Profile</UiTabsTrigger>
                                <UiTabsTrigger key="billing-trigger" value="billing">Billing</UiTabsTrigger>
                            </UiTabsList>
                            <UiTabsContent key="tabs-content" value="compact" className="rounded-md border border-hairline bg-canvas-soft p-3">
                                <UiText key="tabs-copy" label="Profile settings" className="text-sm text-ink" />
                            </UiTabsContent>
                        </UiTabs>
                        <UiTabs key="activity-tabs" value="overview" onSelectionChange={setValue} className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                            <UiTabsList key="activity-tabs-list" label="Activity tabs">
                                <UiTabsTrigger key="overview-tab" value="overview">Overview</UiTabsTrigger>
                                <UiTabsTrigger key="events-tab" value="events">Events</UiTabsTrigger>
                            </UiTabsList>
                            <UiTabPanels key="tab-panels">
                                <UiTabsContent key="activity-tab-panel" value="overview" className="rounded-md border border-hairline bg-canvas-soft p-3">
                                    <UiText key="activity-tabs-copy" label="Activity tab panel" className="text-sm text-ink" />
                                </UiTabsContent>
                            </UiTabPanels>
                        </UiTabs>
                    </UiToolbar>
                </UiToolbar>
            </UiToolbar>
        </PlaygroundSection>
    )
}
