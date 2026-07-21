use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OverviewPanelProps {
    pub set_section: String,
}

#[allow(non_snake_case)]
pub fn overview_panel(cx: &mut ComponentCx<OverviewPanelProps>) -> RSX {
    let setSection = cx.use_prop("setSection", |props: &OverviewPanelProps| {
        props.set_section.clone()
    });

    a3s_gui::rsx!(
        <PlaygroundSection
            key="root"
            title="Overview"
            description="Component families grouped for quick visual review."
        >
            <Group key="atlas" data-slot="playground-overview" class="grid w-[820px] gap-5">
                <UiGroup key="summary" label="Component coverage" className="w-[820px] gap-4 rounded-md border border-transparent bg-surface-dark p-5 text-on-dark">
                    <UiToolbar key="summary-row" orientation="horizontal" className="w-[780px] items-center justify-between gap-6 rounded-none border-none bg-transparent p-0">
                        <UiGroup key="summary-copy" label="Summary" className="w-[480px] gap-1">
                            <UiHeading key="summary-heading" level={3} label="162 components, one visual baseline" className="text-lg font-semibold leading-7 text-on-dark" />
                            <UiDescription key="summary-description" label="Use the sections to inspect controls in their native RSX context." className="text-sm leading-5 text-on-dark-soft" />
                        </UiGroup>
                        <UiToolbar key="summary-metrics" orientation="horizontal" className="gap-2 rounded-none border-none bg-transparent p-0">
                            <UiBadge key="sections-badge" variant="secondary">7 sections</UiBadge>
                            <UiBadge key="design-badge" variant="secondary">native</UiBadge>
                        </UiToolbar>
                    </UiToolbar>
                    <UiProgressBar key="progress-bar" label="Coverage" valueNumber={100} minValue={0} maxValue={100} className="w-[780px]" />
                </UiGroup>

                <UiToolbar key="badges" orientation="horizontal" className="w-[820px] items-center gap-3 rounded-none border-none bg-transparent p-0">
                    <UiBadge key="coverage-badge" variant="secondary">Registered</UiBadge>
                    <UiBadge key="selection-badge" variant="outline">
                        <UiSelectionIndicator key="section-indicator" label="Current section" isSelected={true}>Selected</UiSelectionIndicator>
                    </UiBadge>
                </UiToolbar>

                <UiGroup key="section-links" label="Sections" className="grid w-[820px] gap-2">
                    <UiToolbar key="row-one" orientation="horizontal" className="w-[820px] gap-2 rounded-none border-none bg-transparent p-0">
                        <UiNavigateButton key="foundation-open" to="foundation" onNavigate={setSection} className="w-[130px] justify-start">Foundation</UiNavigateButton>
                        <UiNavigateButton key="controls-open" to="controls" onNavigate={setSection} className="w-[130px] justify-start">Controls</UiNavigateButton>
                        <UiNavigateButton key="collections-open" to="collections" onNavigate={setSection} className="w-[130px] justify-start">Collections</UiNavigateButton>
                    </UiToolbar>
                    <UiToolbar key="row-two" orientation="horizontal" className="w-[820px] gap-2 rounded-none border-none bg-transparent p-0">
                        <UiNavigateButton key="data-open" to="data" onNavigate={setSection} className="w-[130px] justify-start">Data</UiNavigateButton>
                        <UiNavigateButton key="range-open" to="date-color-range" onNavigate={setSection} className="w-[180px] justify-start">Date, color, range</UiNavigateButton>
                        <UiNavigateButton key="overlays-open" to="overlays-feedback" onNavigate={setSection} className="w-[160px] justify-start">Overlays</UiNavigateButton>
                    </UiToolbar>
                </UiGroup>

                <UiGroup key="sample-controls" label="Representative controls" className="grid w-[820px] gap-3 rounded-md border border-hairline bg-surface-card p-3">
                    <UiToolbar key="controls-preview" orientation="horizontal" className="items-center gap-3 rounded-none border-none bg-transparent p-0">
                        <UiButton key="button" size="sm">Button</UiButton>
                        <UiCheckbox key="checkbox" value="preview" isChecked={true}>Check</UiCheckbox>
                        <UiProgressBar key="progress" label="Progress" valueNumber={64} minValue={0} maxValue={100} className="w-44" />
                    </UiToolbar>
                    <UiToastRegion key="toast-region" label="Preview notification">
                        <UiToast key="toast" title="Preview" description="Toast surface" />
                    </UiToastRegion>
                </UiGroup>
            </Group>
        </PlaygroundSection>
    )
}
