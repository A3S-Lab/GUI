use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FoundationPanelProps {
    pub record: String,
    pub set_value: String,
}

#[allow(non_snake_case)]
pub fn foundation_panel(cx: &mut ComponentCx<FoundationPanelProps>) -> RSX {
    let record = cx.use_prop("record", |props: &FoundationPanelProps| {
        props.record.clone()
    });
    let setValue = cx.use_prop("setValue", |props: &FoundationPanelProps| {
        props.set_value.clone()
    });

    a3s_gui::rsx!(
        <PlaygroundSection
            key="root"
            title="Foundation"
            description="Typography, links, cards, focus scopes, and low-level interaction primitives."
        >
            <UiToolbar key="grid" orientation="vertical" className="w-[836px] gap-5 rounded-none border-none bg-transparent p-0">
                <UiCard key="typography-card" className="w-[836px] p-4">
                    <UiCardHeader key="card-header" className="gap-1 p-0">
                        <UiCardTitle key="card-title">Project summary</UiCardTitle>
                        <UiCardDescription key="card-description">A composed card using the foundation primitives.</UiCardDescription>
                    </UiCardHeader>
                    <UiCardContent key="card-content" className="grid gap-3 p-0">
                        <UiLabel key="label" label="Release channel" className="text-sm font-medium text-ink" />
                        <UiHeading key="heading" level={3} label="Native component system" className="text-xl font-semibold leading-7 text-ink" />
                        <UiText key="text" label="The playground uses real rsx_ui components." className="text-sm leading-6 text-ink" />
                        <UiDescription key="description" label="Muted helper copy stays in the same semantic layer." className="text-sm leading-6 text-body" />
                        <UiBreadcrumbs key="breadcrumbs" label="Location" className="gap-2">
                            <UiBreadcrumb key="home" href="/" onPress={record}>Home</UiBreadcrumb>
                            <UiBreadcrumb key="gui" href="/gui" onPress={record}>GUI</UiBreadcrumb>
                            <UiBreadcrumb key="components" href="/gui/components" onPress={record}>Components</UiBreadcrumb>
                        </UiBreadcrumbs>
                        <UiToolbar key="links" orientation="horizontal" className="gap-3 rounded-none border-none bg-transparent p-0">
                            <UiLink key="link" href="https://github.com/A3S-Lab/GUI" onPress={record}>Open repository</UiLink>
                            <UiBadge key="badge" variant="outline">Preview</UiBadge>
                        </UiToolbar>
                        <UiAside key="landmark-aside" label="Related component note" className="rounded-md border border-hairline bg-canvas-soft p-3">
                            <UiText key="aside-copy" label="Aside remains a semantic component sample inside the content page." className="text-sm text-body" />
                        </UiAside>
                    </UiCardContent>
                    <UiCardFooter key="card-footer" className="p-0">
                        <UiButton key="footer-action" variant="secondary" onPress={record}>Inspect card</UiButton>
                    </UiCardFooter>
                </UiCard>
                <UiToolbar key="interaction-grid" orientation="horizontal" className="h-[232px] w-[836px] gap-4 rounded-none border-none bg-transparent p-0">
                    <UiGroup key="interaction-group" label="Interaction primitives" className="h-[232px] w-[410px] gap-3 rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiPressable key="pressable" onPress={record} isPressed={true} className="h-10 w-[378px] rounded-md border border-hairline-strong px-3 py-2">Pressable action</UiPressable>
                        <UiHoverable key="hoverable" isHovered={true} className="h-10 w-[378px] rounded-md border border-hairline-strong px-3 py-2">Hoverable row</UiHoverable>
                        <UiKeyboardTarget key="keyboard-target" onKeyDown={record} isKeyboardActive={true} tabIndex={0} className="h-10 w-[378px] rounded-md border border-hairline-strong px-3 py-2">Keyboard target</UiKeyboardTarget>
                        <UiClipboardTarget key="clipboard" label="Copy token" onCopy={record} copyValue="a3s-token" className="h-10 w-[378px] rounded-md border border-hairline-strong px-3 py-2">Clipboard target</UiClipboardTarget>
                    </UiGroup>
                    <UiGroup key="motion-group" label="Focus and motion primitives" className="h-[232px] w-[410px] gap-3 rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiLongPressable key="long-pressable" onLongPress={record} isLongPressed={true} className="h-10 w-[378px] rounded-md border border-hairline-strong px-3 py-2">Long pressable</UiLongPressable>
                        <UiMovable key="movable" onMove={record} isMoving={true} xDelta={8} yDelta={4} className="h-10 w-[378px] rounded-md border border-hairline-strong px-3 py-2">Movable surface</UiMovable>
                        <UiFocusable key="focusable" onFocus={record} isFocused={true} tabIndex={0} className="h-10 w-[378px] rounded-md border border-hairline-strong px-3 py-2">Focusable region</UiFocusable>
                        <UiFocusRing key="focus-ring" isFocused={true} isFocusVisible={true} tabIndex={0} className="h-10 w-[378px] rounded-md border border-hairline-strong px-3 py-2">Focus ring</UiFocusRing>
                    </UiGroup>
                </UiToolbar>
                <UiToolbar key="support-grid" orientation="horizontal" className="h-[260px] w-[836px] gap-4 rounded-none border-none bg-transparent p-0">
                    <UiFocusScope key="focus-scope" contain={true} restoreFocus={true} className="h-[260px] w-[410px] gap-4 rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiI18nProvider key="i18n" locale="en-US" direction="ltr" className="w-[378px] rounded-md border border-hairline bg-canvas-soft p-3">
                            <UiText key="i18n-text" label="Locale aware content" className="text-sm text-ink" />
                        </UiI18nProvider>
                        <UiVisuallyHidden key="hidden" label="Screen reader status" textValue="Hidden accessible status" />
                        <UiSharedElementTransition key="transition" id="hero-card" isTransitioning={true}>
                            <UiSharedElement key="shared" id="hero-card">
                                <UiCard key="shared-card" className="w-[378px] p-4">
                                    <UiText key="shared-text" label="Shared element transition" className="text-sm text-ink" />
                                </UiCard>
                            </UiSharedElement>
                        </UiSharedElementTransition>
                    </UiFocusScope>
                    <UiGroup key="textareas" label="Long-form input" className="h-[260px] w-[410px] gap-4 rounded-lg border border-hairline-strong bg-canvas p-4">
                        <UiTextarea key="textarea" value="Release notes" placeholder="Notes" onChange={setValue} className="h-24 w-[378px]" />
                        <UiTextArea key="text-area" value="Follow-up checklist" placeholder="Checklist" onChange={setValue} className="h-24 w-[378px]" />
                    </UiGroup>
                </UiToolbar>
                <UiRouter key="router-card" currentPath="/components" className="rounded-lg border border-hairline-strong bg-canvas p-4">
                    <UiNavigation key="router-nav" label="Router navigation" className="gap-2">
                        <UiNavLink key="router-home-link" to="/" onNavigate={record} isActive={false}>Home</UiNavLink>
                        <UiNavigateButton key="router-components-button" to="/components" onNavigate={record} isActive={true}>Components</UiNavigateButton>
                    </UiNavigation>
                    <UiRoutes key="router-routes" label="Router routes" className="rounded-md border border-hairline bg-canvas-soft p-3">
                        <UiRoute key="router-home-route" path="/" label="Home" isActive={false}>
                            <UiText key="router-home-text" label="Home route" />
                        </UiRoute>
                        <UiRoute key="router-components-route" path="/components" label="Components" isActive={true}>
                            <UiText key="router-components-text" label="Components route rendered from UiRoute" />
                        </UiRoute>
                    </UiRoutes>
                </UiRouter>
            </UiToolbar>
        </PlaygroundSection>
    )
}
