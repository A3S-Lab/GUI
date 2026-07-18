use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlaygroundShellProps {
    pub interaction_count: String,
    pub last_event: String,
    pub query: String,
    pub selected_value: String,
    pub active_section: String,
    pub overview_active: bool,
    pub foundation_active: bool,
    pub controls_active: bool,
    pub collections_active: bool,
    pub data_active: bool,
    pub date_color_range_active: bool,
    pub overlays_feedback_active: bool,
    pub overlay_open: bool,
    pub record: String,
    pub set_value: String,
    pub set_section: String,
    pub open_overlay: String,
    pub close_overlay: String,
}

#[allow(non_snake_case)]
pub fn playground_shell(cx: &mut ComponentCx<PlaygroundShellProps>) -> RSX {
    let interactionCount = cx.use_prop("interactionCount", |props: &PlaygroundShellProps| {
        props.interaction_count.clone()
    });
    let lastEvent = cx.use_prop("lastEvent", |props: &PlaygroundShellProps| {
        props.last_event.clone()
    });
    let query = cx.use_prop("query", |props: &PlaygroundShellProps| props.query.clone());
    let selectedValue = cx.use_prop("selectedValue", |props: &PlaygroundShellProps| {
        props.selected_value.clone()
    });
    let activeSection = cx.use_prop("activeSection", |props: &PlaygroundShellProps| {
        props.active_section.clone()
    });
    let overviewActive = cx.use_prop("overviewActive", |props: &PlaygroundShellProps| {
        props.overview_active
    });
    let foundationActive = cx.use_prop("foundationActive", |props: &PlaygroundShellProps| {
        props.foundation_active
    });
    let controlsActive = cx.use_prop("controlsActive", |props: &PlaygroundShellProps| {
        props.controls_active
    });
    let collectionsActive = cx.use_prop("collectionsActive", |props: &PlaygroundShellProps| {
        props.collections_active
    });
    let dataActive = cx.use_prop("dataActive", |props: &PlaygroundShellProps| {
        props.data_active
    });
    let dateColorRangeActive = cx
        .use_prop("dateColorRangeActive", |props: &PlaygroundShellProps| {
            props.date_color_range_active
        });
    let overlaysFeedbackActive = cx
        .use_prop("overlaysFeedbackActive", |props: &PlaygroundShellProps| {
            props.overlays_feedback_active
        });
    let overlayOpen = cx.use_prop("overlayOpen", |props: &PlaygroundShellProps| {
        props.overlay_open
    });
    let record = cx.use_prop("record", |props: &PlaygroundShellProps| {
        props.record.clone()
    });
    let setValue = cx.use_prop("setValue", |props: &PlaygroundShellProps| {
        props.set_value.clone()
    });
    let setSection = cx.use_prop("setSection", |props: &PlaygroundShellProps| {
        props.set_section.clone()
    });
    let openOverlay = cx.use_prop("openOverlay", |props: &PlaygroundShellProps| {
        props.open_overlay.clone()
    });
    let closeOverlay = cx.use_prop("closeOverlay", |props: &PlaygroundShellProps| {
        props.close_overlay.clone()
    });

    a3s_gui::rsx!(
        <UiToolbar
            key="root"
            label="A3S GUI component playground"
            orientation="vertical"
            className="h-[860px] w-[1180px] gap-0 overflow-hidden bg-canvas-soft text-ink"
        >
            <UiHeader key="header" label="Playground header" className="h-20 w-[1180px] border-b border-hairline-soft bg-canvas px-6 py-3">
                <UiToolbar key="header-row" label="Header actions" orientation="horizontal" className="w-[1132px] items-center gap-6 rounded-none border-none bg-transparent p-0">
                    <UiGroup key="brand-group" label="Playground identity" className="w-[680px] gap-1">
                        <UiHeading key="title" level={1} label="Component Playground" className="text-lg font-semibold leading-7 text-ink" />
                        <UiDescription key="subtitle" label="A3S GUI semantic components" className="text-sm leading-5 text-body" />
                    </UiGroup>
                    <UiToolbar key="header-tools" label="Playground tools" orientation="horizontal" className="w-[430px] justify-end gap-3 rounded-none border-none bg-transparent p-0">
                        <UiBadge key="selected" variant="outline">{selectedValue}</UiBadge>
                        <UiKeyboard key="command-key" label="Open command palette" textValue="Command K" className="rounded-md border border-hairline bg-canvas-soft px-3 py-1.5 text-sm">Command K</UiKeyboard>
                        <UiButton key="record" variant="default" onPress={record}>Record event</UiButton>
                    </UiToolbar>
                </UiToolbar>
            </UiHeader>
            <UiToolbar key="body" orientation="horizontal" className="h-[736px] w-[1180px] gap-0 rounded-none border-none bg-transparent p-0">
                <UiNavigation key="navigation" label="Playground navigation" className="h-[736px] w-[248px] border-r border-hairline-soft bg-canvas p-3">
                    <UiToolbar key="nav-stack" orientation="vertical" className="w-[224px] gap-2 rounded-none border-none bg-transparent p-0">
                        <UiText key="nav-label" label="Components" className="mb-1 text-xs font-semibold text-muted" />
                        <UiSearch key="search" label="Component search" className="mb-3 w-[224px]">
                            <UiSearchField
                                key="field"
                                label="Search components"
                                value={query}
                                placeholder="Search components"
                                onChange={setValue}
                                className="w-[224px]"
                            />
                        </UiSearch>
                        <UiNavigateButton key="nav-overview" to="overview" onNavigate={setSection} isActive={overviewActive} className="h-9 w-[224px] justify-start">
                            Overview
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-foundation" to="foundation" onNavigate={setSection} isActive={foundationActive} className="h-9 w-[224px] justify-start">
                            Foundation
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-controls" to="controls" onNavigate={setSection} isActive={controlsActive} className="h-9 w-[224px] justify-start">
                            Controls
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-collections" to="collections" onNavigate={setSection} isActive={collectionsActive} className="h-9 w-[224px] justify-start">
                            Collections
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-data" to="data" onNavigate={setSection} isActive={dataActive} className="h-9 w-[224px] justify-start">
                            Data
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-date-color-range" to="date-color-range" onNavigate={setSection} isActive={dateColorRangeActive} className="h-9 w-[224px] justify-start">
                            Date, color, range
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-overlays-feedback" to="overlays-feedback" onNavigate={setSection} isActive={overlaysFeedbackActive} className="h-9 w-[224px] justify-start">
                            Overlays and files
                        </UiNavigateButton>
                        <UiSeparator key="nav-separator" orientation="horizontal" className="my-3" />
                        <UiArticle key="status-article" label="Playground status" className="w-[224px] gap-1 rounded-md border border-transparent bg-surface-strong p-3">
                            <UiToolbar key="status-row" orientation="horizontal" className="w-[200px] items-center justify-between gap-3 rounded-none border-none bg-transparent p-0">
                                <UiText key="status-heading" label="Events" className="text-sm font-medium text-ink" />
                                <UiText key="count" label={interactionCount} className="text-sm font-semibold text-ink" />
                            </UiToolbar>
                            <UiDescription key="event" label={lastEvent} className="text-sm leading-5 text-body" />
                        </UiArticle>
                    </UiToolbar>
                </UiNavigation>
                <UiMain key="main" label="Component playground content" className="h-[736px] w-[932px] overflow-y-auto bg-canvas p-6">
                    <UiRouter key="page-router" currentPath={activeSection} className="w-[860px] gap-0">
                        <UiRoutes key="page-routes" label="Component playground routes" className="w-[860px] gap-0">
                            <UiRoute key="overview-route" path="overview" label="Overview" isActive={overviewActive}>
                                <OverviewPanel key="overview" setSection={setSection} />
                            </UiRoute>
                            <UiRoute key="foundation-route" path="foundation" label="Foundation" isActive={foundationActive}>
                                <FoundationPanel key="foundation" record={record} setValue={setValue} />
                            </UiRoute>
                            <UiRoute key="controls-route" path="controls" label="Controls" isActive={controlsActive}>
                                <ControlsPanel key="controls" record={record} setValue={setValue} selectedValue={selectedValue} />
                            </UiRoute>
                            <UiRoute key="collections-route" path="collections" label="Collections" isActive={collectionsActive}>
                                <CollectionsPanel key="collections" record={record} setValue={setValue} selectedValue={selectedValue} />
                            </UiRoute>
                            <UiRoute key="data-route" path="data" label="Data" isActive={dataActive}>
                                <DataPanel key="data" record={record} setValue={setValue} selectedValue={selectedValue} />
                            </UiRoute>
                            <UiRoute key="date-color-range-route" path="date-color-range" label="Date, color, and range" isActive={dateColorRangeActive}>
                                <DateColorRangePanel key="date-color-range" record={record} setValue={setValue} />
                            </UiRoute>
                            <UiRoute key="overlays-feedback-route" path="overlays-feedback" label="Overlays and files" isActive={overlaysFeedbackActive}>
                                <OverlaysFeedbackPanel key="overlays-feedback" record={record} setValue={setValue} openOverlay={openOverlay} closeOverlay={closeOverlay} overlayOpen={overlayOpen} />
                            </UiRoute>
                        </UiRoutes>
                    </UiRouter>
                </UiMain>
            </UiToolbar>
            <UiFooter key="footer" label="Playground footer" className="h-11 w-[1180px] border-t border-hairline-soft bg-canvas-soft px-6 py-3">
                <UiText key="footer-copy" label={activeSection} className="text-sm text-body" />
            </UiFooter>
        </UiToolbar>
    )
}
