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
            "w-[884px] gap-5 p-6".to_string()
        } else {
            format!("w-[884px] gap-5 p-6 {}", props.class_name)
        }
    });

    a3s_gui::rsx!(
        <UiCard key="root" className={className}>
            <UiCardHeader key="header" className="gap-1 border-b border-hairline pb-4">
                <UiCardTitle key="title">{title}</UiCardTitle>
                <UiCardDescription key="description">{description}</UiCardDescription>
            </UiCardHeader>
            <UiCardContent key="content" className="pt-5">
                <Slot key="content" />
            </UiCardContent>
        </UiCard>
    )
}
