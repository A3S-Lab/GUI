use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlaygroundShellProps {
    pub interaction_count: String,
    pub last_event: String,
    pub query: String,
    pub selected_value: String,
    pub active_section: String,
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
            className="h-[860px] w-[1180px] gap-0 overflow-hidden bg-canvas text-ink"
        >
            <UiHeader key="header" label="Playground header" className="h-24 w-[1180px] border-b border-hairline-strong bg-canvas px-8 py-4">
                <UiToolbar key="header-row" label="Header actions" orientation="horizontal" className="h-16 w-[1116px] items-center justify-between gap-6 rounded-none border-none bg-transparent p-0">
                    <UiGroup key="brand-group" label="Playground identity" className="h-16 w-[720px] gap-1">
                        <UiBadge key="brand-kicker" variant="secondary">A3S GUI</UiBadge>
                        <UiHeading key="title" level={1} label="Component Playground" className="text-2xl font-semibold leading-tight text-ink" />
                        <UiDescription key="subtitle" label="A unified DESIGN.md pass across every semantic RSX component." className="text-sm leading-6 text-body" />
                    </UiGroup>
                    <UiToolbar key="header-tools" label="Playground tools" orientation="horizontal" className="h-10 w-[360px] justify-end gap-3 rounded-none border-none bg-transparent p-0">
                        <UiBadge key="selected" variant="outline">{selectedValue}</UiBadge>
                        <UiKeyboard key="command-key" label="Open command palette" textValue="Command K" className="rounded-md border border-hairline-strong bg-canvas-soft px-3 py-2 text-sm">Command K</UiKeyboard>
                        <UiButton key="record" variant="default" onPress={record}>Record event</UiButton>
                    </UiToolbar>
                </UiToolbar>
            </UiHeader>
            <UiToolbar key="body" orientation="horizontal" className="h-[728px] w-[1180px] gap-0 rounded-none border-none bg-transparent p-0">
                <UiNavigation key="navigation" label="Playground navigation" className="h-[728px] w-60 border-r border-hairline bg-canvas-soft p-5">
                    <UiToolbar key="nav-stack" orientation="vertical" className="h-[688px] w-[200px] gap-2 rounded-none border-none bg-transparent p-0">
                        <UiText key="nav-label" label="Sections" className="mb-2 text-xs font-semibold uppercase tracking-[0.08em] text-muted" />
                        <UiSearch key="search" label="Component search" className="mb-3 h-11 w-[200px]">
                            <UiSearchField
                                key="field"
                                label="Search components"
                                value={query}
                                placeholder="Search components"
                                onChange={setValue}
                                className="w-[200px]"
                            />
                        </UiSearch>
                        <UiToolbar key="current-section-row" orientation="horizontal" className="h-8 w-[200px] gap-2 rounded-none border-none bg-transparent p-0">
                            <UiSelectionIndicator key="section-indicator" label="Current section" isSelected={true}>*</UiSelectionIndicator>
                            <UiText key="current-section-label" label={activeSection} className="text-sm text-body" />
                        </UiToolbar>
                        <UiToggleButtonGroup key="view-toggle" label="View mode" value="all" onSelectionChange={setValue} className="mb-2 h-10 w-[200px]">
                            <UiToggleButton key="all-toggle" isSelected={true} onPress={record} actionValue="all">All</UiToggleButton>
                            <UiToggleButton key="active-toggle" onPress={record} actionValue="active">Live</UiToggleButton>
                        </UiToggleButtonGroup>
                        <UiNavigateButton key="nav-foundation" to="foundation" onNavigate={setSection} isActive={foundationActive} className="h-10 w-[200px] justify-start">
                            Foundation
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-controls" to="controls" onNavigate={setSection} isActive={controlsActive} className="h-10 w-[200px] justify-start">
                            Controls
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-collections" to="collections" onNavigate={setSection} isActive={collectionsActive} className="h-10 w-[200px] justify-start">
                            Collections
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-data" to="data" onNavigate={setSection} isActive={dataActive} className="h-10 w-[200px] justify-start">
                            Data
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-date-color-range" to="date-color-range" onNavigate={setSection} isActive={dateColorRangeActive} className="h-10 w-[200px] justify-start">
                            Date, color, range
                        </UiNavigateButton>
                        <UiNavigateButton key="nav-overlays-feedback" to="overlays-feedback" onNavigate={setSection} isActive={overlaysFeedbackActive} className="h-10 w-[200px] justify-start">
                            Overlays and files
                        </UiNavigateButton>
                        <UiSeparator key="nav-separator" orientation="horizontal" className="my-3" />
                        <UiArticle key="status-article" label="Playground status" className="w-[200px] rounded-lg border border-hairline-strong bg-canvas p-4">
                            <UiHeading key="status-heading" level={3} label="Session" className="text-base font-semibold text-ink" />
                            <UiText key="count-label" label="Interactions" className="mt-2 text-xs font-semibold uppercase tracking-[0.08em] text-muted" />
                            <UiText key="count" label={interactionCount} className="text-2xl font-semibold text-ink" />
                            <UiText key="event-label" label="Last event" className="mt-3 text-xs font-semibold uppercase tracking-[0.08em] text-muted" />
                            <UiDescription key="event" label={lastEvent} className="text-sm leading-6 text-body" />
                        </UiArticle>
                    </UiToolbar>
                </UiNavigation>
                <UiMain key="main" label="Component playground content" className="h-[728px] w-[940px] overflow-y-auto bg-canvas p-7">
                    <UiSection key="hero-section" label="Playground overview" className="mb-6 h-[116px] w-[884px] rounded-lg border border-hairline-strong bg-canvas p-5">
                        <UiToolbar key="hero-row" orientation="horizontal" className="h-[76px] w-[844px] items-center justify-between gap-6 rounded-none border-none bg-transparent p-0">
                            <UiGroup key="hero-copy" label="Playground summary" className="h-[76px] w-[560px] gap-2">
                                <UiHeading key="hero-heading" level={2} label="Semantic components with one default design language" className="text-xl font-semibold leading-7 text-ink" />
                                <UiDescription key="hero-description" label="Controls, collections, overlays, and data surfaces now share DESIGN.md colors, radii, spacing, and state treatment." className="text-sm leading-6 text-body" />
                            </UiGroup>
                            <UiToolbar key="hero-metrics" orientation="horizontal" className="gap-3 rounded-none border-none bg-transparent p-0">
                                <UiBadge key="metric-components" variant="secondary">168 components</UiBadge>
                                <UiBadge key="metric-actions" variant="outline">{interactionCount} events</UiBadge>
                                <UiBadge key="metric-section" variant="outline">{activeSection}</UiBadge>
                            </UiToolbar>
                        </UiToolbar>
                    </UiSection>
                    <UiSection key="content-section" label="Current semantic component section" className="w-[884px]">
                        <UiToolbar key="content-stack" orientation="vertical" className="w-[884px] gap-5 rounded-none border-none bg-transparent p-0">
                            <Show key="foundation-section" when={foundationActive}><FoundationPanel key="foundation" record={record} setValue={setValue} /></Show>
                            <Show key="controls-section" when={controlsActive}><ControlsPanel key="controls" record={record} setValue={setValue} selectedValue={selectedValue} /></Show>
                            <Show key="collections-section" when={collectionsActive}><CollectionsPanel key="collections" record={record} setValue={setValue} selectedValue={selectedValue} /></Show>
                            <Show key="data-section" when={dataActive}><DataPanel key="data" record={record} setValue={setValue} selectedValue={selectedValue} /></Show>
                            <Show key="date-color-range-section" when={dateColorRangeActive}><DateColorRangePanel key="date-color-range" record={record} setValue={setValue} /></Show>
                            <Show key="overlays-feedback-section" when={overlaysFeedbackActive}><OverlaysFeedbackPanel key="overlays-feedback" record={record} setValue={setValue} openOverlay={openOverlay} closeOverlay={closeOverlay} overlayOpen={overlayOpen} /></Show>
                        </UiToolbar>
                    </UiSection>
                </UiMain>
            </UiToolbar>
            <UiFooter key="footer" label="Playground footer" className="h-9 w-[1180px] border-t border-hairline bg-canvas px-8 py-2">
                <UiText key="footer-copy" label="A3S GUI semantic components rendered from split RSX files." className="text-sm text-body" />
            </UiFooter>
        </UiToolbar>
    )
}
