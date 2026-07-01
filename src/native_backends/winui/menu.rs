use crate::geometry::Orientation;
use crate::winui::WinUiWidgetKind;

pub(crate) fn stack_panel_orientation(
    kind: WinUiWidgetKind,
    configured: Option<Orientation>,
) -> Option<Orientation> {
    configured.or_else(|| {
        matches!(
            kind,
            WinUiWidgetKind::CommandBar | WinUiWidgetKind::MenuPanel
        )
        .then_some(Orientation::Horizontal)
    })
}
