use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalculatorMemoryBarProps;

pub fn calculator_memory_bar(_cx: &mut ComponentCx<CalculatorMemoryBarProps>) -> RSX {
    a3s_gui::rsx!(
        <Toolbar
            key="root"
            label="Memory controls"
            orientation="horizontal"
            className="h-11 w-[396px] gap-0 bg-[#f3f3f3] px-2 py-1"
        >
            <Text key="memory-clear" label="MC" className="h-[34px] w-[62px] text-center text-xs font-semibold text-[#8a8a8a]" />
            <Text key="memory-recall" label="MR" className="h-[34px] w-[62px] text-center text-xs font-semibold text-[#8a8a8a]" />
            <Text key="memory-add" label="M+" className="h-[34px] w-[62px] text-center text-xs font-semibold text-[#1b1b1b]" />
            <Text key="memory-subtract" label="M-" className="h-[34px] w-[62px] text-center text-xs font-semibold text-[#1b1b1b]" />
            <Text key="memory-store" label="MS" className="h-[34px] w-[62px] text-center text-xs font-semibold text-[#1b1b1b]" />
            <Text key="memory-list" label="M⌄" className="h-[34px] w-[62px] text-center text-xs font-semibold text-[#1b1b1b]" />
        </Toolbar>
    )
}
