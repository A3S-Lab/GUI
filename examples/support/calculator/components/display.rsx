use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorDisplayProps {
    pub display: String,
    pub history: String,
    pub has_error: bool,
}

#[allow(non_snake_case)]
pub fn calculator_display(cx: &mut ComponentCx<CalculatorDisplayProps>) -> RSX {
    let display = cx.use_prop("display", |props: &CalculatorDisplayProps| {
        props.display.clone()
    });
    let history = cx.use_prop("history", |props: &CalculatorDisplayProps| {
        props.history.clone()
    });
    let hasError = cx.use_prop("hasError", |props: &CalculatorDisplayProps| props.has_error);

    a3s_gui::rsx!(
        <Toolbar
            key="root"
            label="Display"
            orientation="vertical"
            className="h-[132px] w-[396px] gap-1 bg-[#f3f3f3] px-4 pb-[10px] pt-3"
        >
            <Text
                key="history"
                label={history}
                className="h-[26px] w-[364px] text-right text-[13px] font-normal text-[#737373]"
            />
            <Text
                key="value"
                label={display}
                data-error={hasError}
                className="h-[74px] w-[364px] text-right text-[48px] font-semibold leading-none text-[#1b1b1b] data-[error=true]:text-[#eb8e90]"
            />
        </Toolbar>
    )
}
