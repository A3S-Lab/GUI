mod components;
mod model;
mod view;

use std::sync::Arc;

use a3s_gui::{ActionInvocation, GuiResult, RsxComponent, UiFrame};

pub use model::ComponentPlaygroundState;

pub type ComponentPlaygroundComponent = Arc<RsxComponent<ComponentPlaygroundState>>;

pub fn shared_component_playground_component(
    frame_id: &str,
    title: &str,
) -> GuiResult<ComponentPlaygroundComponent> {
    view::component_playground_component(frame_id, title).map(Arc::new)
}

pub fn component_playground_frame(
    component: &ComponentPlaygroundComponent,
    state: &ComponentPlaygroundState,
) -> GuiResult<UiFrame> {
    component.render(state)
}

#[allow(dead_code)]
pub fn component_playground_reduce(
    component: &ComponentPlaygroundComponent,
    state: &mut ComponentPlaygroundState,
    invocation: &ActionInvocation,
) -> GuiResult<()> {
    component.reduce(state, invocation)
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, fs, path::Path};

    use a3s_gui::{CompiledRsxNode, HostNodeId, NativeEventKind};

    use super::*;

    #[test]
    fn component_playground_renders_a_stateful_frame() {
        let component = shared_component_playground_component(
            "test-component-playground",
            "Component Playground",
        )
        .unwrap();
        let frame = component_playground_frame(&component, &ComponentPlaygroundState::default())
            .expect("playground frame");

        assert_eq!(frame.frame_id, "test-component-playground");
        assert!(frame.actions.iter().any(|action| action.id == "record"));
        assert!(frame.actions.iter().any(|action| action.id == "setValue"));
        assert!(frame.actions.iter().any(|action| action.id == "setSection"));
        assert!(root_child_count(&frame.root) >= 3);
    }

    #[test]
    fn component_playground_uses_docs_style_section_navigation() {
        let component = shared_component_playground_component(
            "test-component-playground",
            "Component Playground",
        )
        .unwrap();
        let mut state = ComponentPlaygroundState::default();

        let foundation = component_playground_frame(&component, &state).expect("foundation frame");
        assert!(find_element_by_attribute(&foundation.root, "data-slot", "navigation").is_some());
        assert!(find_element_by_attribute(&foundation.root, "data-slot", "main").is_some());
        let foundation_nav = find_element_by_attributes(
            &foundation.root,
            &[
                ("data-slot", "navigate-button"),
                ("data-route-to", "foundation"),
            ],
        )
        .expect("foundation nav item");
        assert_eq!(attribute_value(foundation_nav, "data-active"), Some("true"));
        assert!(find_element_by_attribute(&foundation.root, "data-slot", "router").is_some());
        assert!(find_element_by_attribute(&foundation.root, "data-slot", "form").is_none());

        component
            .reduce(
                &mut state,
                &ActionInvocation {
                    node: HostNodeId::new(1),
                    action: "setSection".to_string(),
                    event: NativeEventKind::Press,
                    value: Some("controls".to_string()),
                },
            )
            .expect("set controls section");

        let controls = component_playground_frame(&component, &state).expect("controls frame");
        let controls_nav = find_element_by_attributes(
            &controls.root,
            &[
                ("data-slot", "navigate-button"),
                ("data-route-to", "controls"),
            ],
        )
        .expect("controls nav item");
        assert_eq!(attribute_value(controls_nav, "data-active"), Some("true"));
        assert!(find_element_by_attribute(&controls.root, "data-slot", "form").is_some());
        assert!(find_element_by_attribute(&controls.root, "data-slot", "router").is_none());
    }

    #[test]
    fn component_playground_rsx_covers_every_registered_semantic_component() {
        let registered = registered_ui_components();
        let source = components::PLAYGROUND_RSX_SOURCES.join("\n");

        let missing = registered
            .iter()
            .filter(|component| !source.contains(&format!("<{component}")))
            .cloned()
            .collect::<Vec<_>>();

        assert!(
            missing.is_empty(),
            "component playground is missing registered components: {missing:?}"
        );
        assert_eq!(registered.len(), 168);
    }

    fn registered_ui_components() -> BTreeSet<String> {
        let registry_dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/rsx_ui/components/registry");
        let mut components = BTreeSet::new();

        for entry in fs::read_dir(registry_dir).expect("registry directory") {
            let path = entry.expect("registry entry").path();
            if path.extension().and_then(|value| value.to_str()) != Some("rs") {
                continue;
            }
            let text = fs::read_to_string(path).expect("registry source");
            collect_registry_components(&text, &mut components);
        }

        components
    }

    fn collect_registry_components(text: &str, components: &mut BTreeSet<String>) {
        let mut remaining = text;
        while let Some(index) = remaining.find("with_builtin_template") {
            remaining = &remaining[index + "with_builtin_template".len()..];
            let Some(component_arg) = remaining.find("component,") else {
                continue;
            };
            remaining = &remaining[component_arg + "component,".len()..];
            let Some(quote_start) = remaining.find('"') else {
                continue;
            };
            let after_quote = &remaining[quote_start + 1..];
            let Some(quote_end) = after_quote.find('"') else {
                continue;
            };
            components.insert(after_quote[..quote_end].to_string());
            remaining = &after_quote[quote_end + 1..];
        }
    }

    fn root_child_count(root: &CompiledRsxNode) -> usize {
        match root {
            CompiledRsxNode::Element { children, .. } => children.len(),
            CompiledRsxNode::Text { .. } => 0,
        }
    }

    fn attribute_value<'a>(node: &'a CompiledRsxNode, name: &str) -> Option<&'a str> {
        let CompiledRsxNode::Element { props, .. } = node else {
            panic!("element node")
        };
        props.attributes.get(name).map(String::as_str)
    }

    fn find_element_by_attribute<'a>(
        node: &'a CompiledRsxNode,
        name: &str,
        value: &str,
    ) -> Option<&'a CompiledRsxNode> {
        find_element_by_attributes(node, &[(name, value)])
    }

    fn find_element_by_attributes<'a>(
        node: &'a CompiledRsxNode,
        attributes: &[(&str, &str)],
    ) -> Option<&'a CompiledRsxNode> {
        match node {
            CompiledRsxNode::Text { .. } => None,
            CompiledRsxNode::Element {
                props, children, ..
            } => {
                if attributes.iter().all(|(name, value)| {
                    props.attributes.get(*name).map(String::as_str) == Some(*value)
                }) {
                    return Some(node);
                }
                children
                    .iter()
                    .find_map(|child| find_element_by_attributes(child, attributes))
            }
        }
    }
}
