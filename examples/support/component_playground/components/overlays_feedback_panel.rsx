use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct OverlaysFeedbackPanelProps {
    pub record: String,
    pub set_value: String,
    pub open_overlay: String,
    pub close_overlay: String,
    pub overlay_open: bool,
}

#[allow(non_snake_case)]
pub fn overlays_feedback_panel(cx: &mut ComponentCx<OverlaysFeedbackPanelProps>) -> RSX {
    let record = cx.use_prop("record", |props: &OverlaysFeedbackPanelProps| {
        props.record.clone()
    });
    let setValue = cx.use_prop("setValue", |props: &OverlaysFeedbackPanelProps| {
        props.set_value.clone()
    });
    let openOverlay = cx.use_prop("openOverlay", |props: &OverlaysFeedbackPanelProps| {
        props.open_overlay.clone()
    });
    let closeOverlay = cx.use_prop("closeOverlay", |props: &OverlaysFeedbackPanelProps| {
        props.close_overlay.clone()
    });
    let overlayOpen = cx.use_prop("overlayOpen", |props: &OverlaysFeedbackPanelProps| {
        props.overlay_open
    });

    a3s_gui::rsx!(
        <PlaygroundSection
            key="root"
            title="Overlays, Feedback, Drag, Drop, And Files"
            description="Dialogs, popovers, disclosure, toasts, virtual regions, file triggers, and drag/drop surfaces."
        >
            <UiToolbar key="root-stack" orientation="vertical" className="w-full gap-5 rounded-none border-none bg-transparent p-0">
                <UiToolbar key="overlay-grid" orientation="horizontal" className="grid w-full grid-cols-2 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiGroup key="dialog-card" label="Dialog primitives" className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                        <UiDialogTrigger key="dialog-trigger" isOpen={overlayOpen} onPress={openOverlay} actionValue="dialog">Open dialog</UiDialogTrigger>
                        <UiDialog key="dialog" label="Review dialog" isOpen={overlayOpen} onClose={closeOverlay} className="rounded-md border border-hairline bg-canvas p-3">
                            <UiHeading key="dialog-heading" level={3} label="Review changes" />
                            <UiText key="dialog-copy" label="Dialog content rendered in the playground." />
                        </UiDialog>
                        <UiModalOverlay key="modal-overlay" label="Modal overlay" isOpen={false} className="rounded-md border border-hairline bg-canvas-soft p-3">
                            <UiText key="overlay-copy" label="Modal overlay surface" />
                        </UiModalOverlay>
                        <UiModal key="modal" label="Modal" isOpen={false} onClose={closeOverlay} className="rounded-md border border-hairline bg-canvas p-3">
                            <UiCard key="modal-card" className="p-3">
                                <UiText key="modal-copy" label="Modal content" />
                            </UiCard>
                        </UiModal>
                    </UiGroup>
                    <UiGroup key="disclosure-card" label="Disclosure and popover primitives" className="gap-3 rounded-md border border-hairline bg-canvas p-3">
                        <UiDisclosureGroup key="disclosure-group" label="Details" expandedKeys="details" allowsMultipleExpanded={true} className="w-full">
                            <UiDisclosure key="disclosure" label="Details" isExpanded={true} onExpandedChange={setValue}>
                                <UiDisclosureSummary key="disclosure-summary" isExpanded={true} onPress={record}>Implementation details</UiDisclosureSummary>
                                <UiDisclosurePanel key="disclosure-panel" label="Panel" isExpanded={true}>Disclosure panel content</UiDisclosurePanel>
                            </UiDisclosure>
                        </UiDisclosureGroup>
                        <UiPopover key="popover" isOpen={false} className="p-3">
                            <UiOverlayArrow key="overlay-arrow" placement="top" />
                            <UiText key="popover-copy" label="Popover content" />
                        </UiPopover>
                        <UiTooltipTrigger key="tooltip-trigger" isOpen={false} onPress={record} actionValue="tooltip">
                            Tooltip target
                            <UiTooltip key="tooltip" label="Tooltip" isOpen={false}>Tooltip content</UiTooltip>
                        </UiTooltipTrigger>
                    </UiGroup>
                </UiToolbar>
                <UiToolbar key="feedback-files-grid" orientation="horizontal" className="grid w-full grid-cols-2 gap-4 rounded-none border-none bg-transparent p-0">
                    <UiGroup key="feedback-card" label="Feedback" className="gap-4 rounded-md border border-hairline bg-canvas p-3">
                        <UiToastRegion key="toast-region" label="Notifications" className="gap-2">
                            <UiToast key="toast" title="Saved" description="Changes synced to native state." onClose={closeOverlay} />
                        </UiToastRegion>
                        <UiVirtualizer key="virtualizer" label="Virtual list" layout="list" orientation="vertical" itemCount={120} estimatedItemSize={40} visibleStart={0} visibleEnd={4} overscan={2} gap={4} padding={8} className="h-40 w-full rounded-md border border-hairline bg-canvas-soft p-3">
                            <UiText key="virtual-row-one" label="Virtual row 1" className="text-sm text-ink" />
                            <UiText key="virtual-row-two" label="Virtual row 2" className="text-sm text-ink" />
                        </UiVirtualizer>
                    </UiGroup>
                    <UiGroup key="files-card" label="Files and drag/drop" className="gap-4 rounded-md border border-hairline bg-canvas p-3">
                        <UiFileTrigger key="file-trigger" onPress={record} onSelect={setValue} acceptedFileTypes=".rsx,.rs" allowsMultiple={true} className="h-10 w-fit">
                            Choose RSX files
                        </UiFileTrigger>
                        <UiDropZone key="drop-zone" label="Drop files" onDrop={setValue} onDragEnter={record} onDragLeave={record} className="h-24 w-full">
                            Drop files here
                        </UiDropZone>
                        <UiDraggable key="draggable" onDragStart={record} onDragMove={record} onDragEnd={record} dragType="text/plain" dragValue="component-card" isDragging={true} className="h-10 w-fit rounded-md border border-hairline bg-canvas-soft px-3 py-1.5">
                            Draggable component card
                        </UiDraggable>
                        <UiDroppable key="droppable" label="Drop target" onDrop={setValue} onDropEnter={record} onDropExit={record} onDropMove={record} acceptedDragTypes="text/plain" dropOperation="move" isDropTarget={true} className="h-24 w-full rounded-md border border-dashed border-hairline bg-canvas-soft p-3">
                            Drop target
                            <UiDropIndicator key="drop-indicator" orientation="horizontal" isTarget={true} className="w-full" />
                        </UiDroppable>
                    </UiGroup>
                </UiToolbar>
            </UiToolbar>
        </PlaygroundSection>
    )
}
