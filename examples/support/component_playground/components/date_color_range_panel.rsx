use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DateColorRangePanelProps {
    pub record: String,
    pub set_value: String,
}

#[allow(non_snake_case)]
pub fn date_color_range_panel(cx: &mut ComponentCx<DateColorRangePanelProps>) -> RSX {
    let record = cx.use_prop("record", |props: &DateColorRangePanelProps| {
        props.record.clone()
    });
    let setValue = cx.use_prop("setValue", |props: &DateColorRangePanelProps| {
        props.set_value.clone()
    });

    a3s_gui::rsx!(
        <PlaygroundSection
            key="root"
            title="Date, Color, And Range"
            description="Calendars, color controls, sliders, progress, and scalar output components."
        >
            <UiToolbar key="root-stack" orientation="vertical" className="w-full gap-5 rounded-none border-none bg-transparent p-0">
                <UiToolbar key="date-grid" orientation="horizontal" className="grid w-full grid-cols-3 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiGroup key="date-pickers" label="Date pickers" className="gap-3 rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiDatePicker key="date-picker" label="Date picker" value="2026-07-07" placeholder="Date" onChange={setValue} onOpenChange={record} isOpen={false} className="w-full" />
                        <UiDateRangePicker key="date-range-picker" label="Range picker" startValue="2026-07-07" endValue="2026-07-14" placeholder="Range" onStartChange={setValue} onEndChange={setValue} onOpenChange={record} isOpen={false} className="w-full" />
                        <UiCalendarMonthPicker key="calendar-month-picker" label="Month" value="July" onSelectionChange={setValue} className="h-10 w-full rounded-md border border-hairline-strong bg-canvas px-3 py-2">July</UiCalendarMonthPicker>
                        <UiCalendarYearPicker key="calendar-year-picker" label="Year" value="2026" onSelectionChange={setValue} className="h-10 w-full rounded-md border border-hairline-strong bg-canvas px-3 py-2">2026</UiCalendarYearPicker>
                    </UiGroup>
                    <UiCalendar key="calendar" label="July 2026" value="2026-07-07" onChange={setValue} className="rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiCalendarHeading key="calendar-heading" label="July 2026" level={3} />
                        <UiCalendarGrid key="calendar-grid" label="July 2026">
                            <UiCalendarGridHeader key="calendar-grid-header">
                                <UiCalendarHeaderCell key="calendar-header-mon" textValue="Mon">Mon</UiCalendarHeaderCell>
                                <UiCalendarHeaderCell key="calendar-header-tue" textValue="Tue">Tue</UiCalendarHeaderCell>
                                <UiCalendarHeaderCell key="calendar-header-wed" textValue="Wed">Wed</UiCalendarHeaderCell>
                            </UiCalendarGridHeader>
                            <UiCalendarGridBody key="calendar-grid-body">
                                <UiCalendarCell key="calendar-cell-7" value="2026-07-07" textValue="7" isSelected={true} isToday={true} onPress={record} actionValue="2026-07-07">7</UiCalendarCell>
                                <UiCalendarCell key="calendar-cell-8" value="2026-07-08" textValue="8" onPress={record} actionValue="2026-07-08">8</UiCalendarCell>
                            </UiCalendarGridBody>
                        </UiCalendarGrid>
                    </UiCalendar>
                    <UiRangeCalendar key="range-calendar" label="Sprint range" startValue="2026-07-07" endValue="2026-07-14" onChange={setValue} className="rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiCalendarHeading key="range-calendar-heading" label="Sprint range" level={3} />
                        <UiCalendarGrid key="range-calendar-grid" label="Sprint range">
                            <UiCalendarGridBody key="range-calendar-body">
                                <UiCalendarCell key="range-calendar-start" value="2026-07-07" textValue="7" isSelected={true} onPress={record} actionValue="2026-07-07">7</UiCalendarCell>
                                <UiCalendarCell key="range-calendar-end" value="2026-07-14" textValue="14" isSelected={true} onPress={record} actionValue="2026-07-14">14</UiCalendarCell>
                            </UiCalendarGridBody>
                        </UiCalendarGrid>
                    </UiRangeCalendar>
                </UiToolbar>
                <UiToolbar key="color-range-grid" orientation="horizontal" className="grid w-full grid-cols-2 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiGroup key="color-group" label="Color controls" className="gap-3 rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiColorPicker key="color-picker" label="Accent" value="#000000" onChange={setValue} className="w-full">
                            <UiColorSwatch key="picker-swatch" label="Black" value="#000000" />
                        </UiColorPicker>
                        <UiColorArea key="color-area" label="Color area" value="#000000" xValue={50} yValue={60} onChange={setValue} className="h-32 w-full">
                            <UiColorThumb key="color-thumb" value="#000000" xValue={50} yValue={60} onPress={record} isDragging={true} actionValue="#000000" />
                        </UiColorArea>
                        <UiColorField key="color-field" label="Hex" value="#000000" placeholder="#000000" onChange={setValue} className="w-full" />
                        <UiColorSlider key="color-slider" label="Hue" channel="hue" valueNumber={210} minValue={0} maxValue={360} stepValue={1} onChange={setValue} className="w-full" />
                        <UiColorWheel key="color-wheel" label="Hue wheel" valueNumber={210} onChange={setValue} className="h-20 w-20">
                            <UiColorWheelTrack key="color-wheel-track" label="Hue track">
                                <UiColorThumb key="wheel-thumb" value="#000000" xValue={50} yValue={50} onPress={record} actionValue="#000000" />
                            </UiColorWheelTrack>
                        </UiColorWheel>
                        <UiColorSwatchPicker key="color-swatch-picker" label="Swatches" value="#000000" onSelectionChange={setValue}>
                            <UiColorSwatchPickerItem key="black-item" value="#000000" textValue="Black" isSelected={true}>
                                <UiColorSwatch key="black-swatch" label="Black" value="#000000" />
                            </UiColorSwatchPickerItem>
                            <UiColorSwatchPickerItem key="blue-item" value="#0d74ce" textValue="Link blue">
                                <UiColorSwatch key="blue-swatch" label="Link blue" value="#0d74ce" />
                            </UiColorSwatchPickerItem>
                        </UiColorSwatchPicker>
                    </UiGroup>
                    <UiGroup key="range-group" label="Range controls" className="gap-4 rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiSlider key="slider" label="Volume" valueNumber={64} minValue={0} maxValue={100} stepValue={1} onChange={setValue} className="w-full">
                            <UiSliderTrack key="slider-track" orientation="horizontal">
                                <UiSliderFill key="slider-fill" orientation="horizontal" valueNumber={64} />
                                <UiSliderThumb key="slider-thumb" valueNumber={64} onPress={record} isDragging={true} actionValue="64" />
                            </UiSliderTrack>
                            <UiSliderOutput key="slider-output" label="Volume output" value="64%" valueNumber={64}>64%</UiSliderOutput>
                        </UiSlider>
                        <UiProgressBar key="progress-bar" label="Progress" valueNumber={64} minValue={0} maxValue={100} className="w-full" />
                        <UiMeter key="meter" label="Quota" valueNumber={72} minValue={0} maxValue={100} className="w-full" />
                    </UiGroup>
                </UiToolbar>
            </UiToolbar>
        </PlaygroundSection>
    )
}
