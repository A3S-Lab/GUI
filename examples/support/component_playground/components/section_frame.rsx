use a3s_gui::{ComponentCx, RSX};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlaygroundSectionProps {
    pub title: String,
    pub description: String,
    pub class_name: String,
}

#[allow(non_snake_case)]
pub fn playground_section(cx: &mut ComponentCx<PlaygroundSectionProps>) -> RSX {
    let title = cx.use_prop("title", |props: &PlaygroundSectionProps| {
        props.title.clone()
    });
    let description = cx.use_prop("description", |props: &PlaygroundSectionProps| {
        props.description.clone()
    });
    let className = cx.use_prop("className", |props: &PlaygroundSectionProps| {
        if props.class_name.is_empty() {
            "w-[860px] gap-4".to_string()
        } else {
            format!("w-[860px] gap-4 {}", props.class_name)
        }
    });

    a3s_gui::rsx!(
        <UiSection key="root" label={title} className={className}>
            <UiHeader key="header" label={title} className="grid gap-1 border-b border-hairline pb-3">
                <UiHeading key="title" level={2} label={title} className="text-lg font-semibold leading-7 text-ink" />
                <UiDescription key="description" label={description} className="text-sm leading-6 text-body" />
            </UiHeader>
            <UiGroup key="content" label="Section content" className="gap-4 pt-1">
                <Slot key="content" />
            </UiGroup>
        </UiSection>
    )
}
