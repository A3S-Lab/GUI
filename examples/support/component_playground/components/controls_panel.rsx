use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ControlsPanelProps {
    pub record: String,
    pub set_value: String,
    pub selected_value: String,
}

#[allow(non_snake_case)]
pub fn controls_panel(cx: &mut ComponentCx<ControlsPanelProps>) -> RSX {
    let record = cx.use_prop("record", |props: &ControlsPanelProps| props.record.clone());
    let setValue = cx.use_prop("setValue", |props: &ControlsPanelProps| {
        props.set_value.clone()
    });
    let selectedValue = cx.use_prop("selectedValue", |props: &ControlsPanelProps| {
        props.selected_value.clone()
    });

    a3s_gui::rsx!(
        <PlaygroundSection
            key="root"
            title="Controls"
            description="Buttons, fields, validation messages, selection controls, and form structure."
        >
            <UiForm key="form" label="Profile form" onSubmit={record} className="w-full gap-5 rounded-lg border border-hairline-strong bg-canvas p-5">
                <UiToolbar key="button-row" orientation="horizontal" className="flex-wrap gap-3 rounded-none border-none bg-transparent p-0">
                    <UiButton key="primary" variant="default" onPress={record}>Save changes</UiButton>
                    <UiButton key="secondary" variant="secondary" onPress={record}>Secondary</UiButton>
                    <UiButton key="outline" variant="outline" onPress={record}>Outline</UiButton>
                    <UiButton key="ghost" variant="ghost" onPress={record}>Ghost</UiButton>
                    <UiButton key="link" variant="link" onPress={record}>Link</UiButton>
                </UiToolbar>
                <UiToolbar key="field-grid" orientation="horizontal" className="grid w-full grid-cols-2 gap-5 rounded-none border-none bg-transparent p-0">
                    <UiFieldSet key="identity" label="Identity" className="gap-3 rounded-lg border border-hairline-strong bg-canvas-soft p-4">
                        <UiLegend key="legend" label="Identity" />
                        <UiLabel key="input-label" label="Plain input" className="text-sm font-medium text-ink" />
                        <UiInput key="input" value="Ada Lovelace" placeholder="Name" onChange={setValue} className="w-full" />
                        <UiTextField key="text-field" label="Email" value="ada@a3s.dev" placeholder="Email" onChange={setValue} className="w-full" />
                        <UiSearchField key="search-field" label="Search field" value={selectedValue} placeholder="Search" onChange={setValue} className="w-full" />
                        <UiFieldError key="field-error" label="Example validation message" textValue="Example validation message" className="text-sm text-semantic-error" />
                    </UiFieldSet>
                    <UiFieldSet key="numbers" label="Date and number" className="gap-3 rounded-lg border border-hairline-strong bg-canvas-soft p-4">
                        <UiNumberField key="number-field" label="Seats" valueNumber={42} minValue={0} maxValue={100} stepValue={1} onChange={setValue} className="w-full" />
                        <UiDateField key="date-field" label="Release date" value="2026-07-07" placeholder="Date" onChange={setValue} className="w-full" />
                        <UiDateInput key="date-input" label="Date input" value="2026-07-07" className="w-full">
                            <UiDateSegment key="year" segmentType="year" value="2026" textValue="2026" />
                            <UiDateSegment key="month" segmentType="month" value="07" textValue="07" />
                            <UiDateSegment key="day" segmentType="day" value="07" textValue="07" />
                        </UiDateInput>
                        <UiTimeField key="time-field" label="Deploy time" value="10:30" placeholder="Time" onChange={setValue} className="w-full" />
                    </UiFieldSet>
                </UiToolbar>
                <UiToolbar key="selection-grid" orientation="horizontal" className="grid w-full grid-cols-3 gap-5 rounded-none border-none bg-transparent p-0">
                    <UiCheckboxGroup key="checkbox-group" label="Notifications" value="email" onChange={setValue} className="gap-3 rounded-lg border border-hairline-strong bg-canvas-soft p-4">
                        <UiCheckbox key="checkbox-email" value="email" isChecked={true} onChange={setValue}>Email updates</UiCheckbox>
                        <UiCheckbox key="checkbox-product" value="product" onChange={setValue}>Product news</UiCheckbox>
                    </UiCheckboxGroup>
                    <UiRadioGroup key="radio-group" label="Density" value="compact" onSelectionChange={setValue} className="gap-3 rounded-lg border border-hairline-strong bg-canvas-soft p-4">
                        <UiRadio key="radio-compact" value="compact" textValue="Compact" isSelected={true}>Compact</UiRadio>
                        <UiRadio key="radio-comfortable" value="comfortable" textValue="Comfortable">Comfortable</UiRadio>
                    </UiRadioGroup>
                    <UiGroup key="switch-group" label="Switch row" className="gap-3 rounded-lg border border-hairline-strong bg-canvas-soft p-4">
                        <UiSwitch key="switch" isChecked={true} onChange={setValue}>Native rendering</UiSwitch>
                        <UiText key="switch-copy" label="Switches share form semantics." className="text-sm leading-6 text-body" />
                    </UiGroup>
                </UiToolbar>
            </UiForm>
        </PlaygroundSection>
    )
}
