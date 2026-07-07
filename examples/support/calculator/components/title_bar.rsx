use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorTitleBarProps;

pub fn calculator_title_bar(_cx: &mut ComponentCx<CalculatorTitleBarProps>) -> RSX {
    a3s_gui::rsx!(
        <Toolbar
            key="root"
            label="Calculator title"
            orientation="horizontal"
            className="h-[52px] w-[396px] gap-[6px] bg-[#f3f3f3] px-3 pb-[6px] pt-[10px]"
        >
            <Text
                key="menu"
                label="☰"
                className="h-8 w-[34px] text-center text-[20px] font-normal text-[#1b1b1b]"
            />
            <Text
                key="mode"
                label="Standard"
                className="h-8 w-[212px] text-[20px] font-semibold text-[#1b1b1b]"
            />
            <Text
                key="history-button"
                label="History"
                className="h-8 w-24 text-right text-[13px] font-medium text-[#3b3b3b]"
            />
        </Toolbar>
    )
}
