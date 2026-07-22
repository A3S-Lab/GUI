use crate::compiler::CompiledRsxNode;
use crate::rsx_app::{RsxComponent, RsxTemplate};

#[derive(Debug, Default)]
struct InteractionState;

#[test]
fn button_compiles_the_complete_press_lifecycle() {
    let template = RsxTemplate::parse(
        r#"
        <UiButton
          key="save"
          onPress={press}
          onPressStart={pressStart}
          onPressEnd={pressEnd}
          onPressUp={pressUp}
        >
          Save
        </UiButton>
        "#,
    )
    .unwrap();
    let component = RsxComponent::<InteractionState>::from_template("button", template)
        .unwrap()
        .use_reducer("press", |_state, _invocation| Ok(()))
        .use_reducer("pressStart", |_state, _invocation| Ok(()))
        .use_reducer("pressEnd", |_state, _invocation| Ok(()))
        .use_reducer("pressUp", |_state, _invocation| Ok(()));

    let frame = component.render(&InteractionState).unwrap();
    let props = find_props_by_attribute(&frame.root, "data-slot", "button").unwrap();

    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("press")
    );
    assert_eq!(
        props.events.get("onPressStart").map(String::as_str),
        Some("pressStart")
    );
    assert_eq!(
        props.events.get("onPressEnd").map(String::as_str),
        Some("pressEnd")
    );
    assert_eq!(
        props.events.get("onPressUp").map(String::as_str),
        Some("pressUp")
    );
}

fn find_props_by_attribute<'a>(
    node: &'a CompiledRsxNode,
    name: &str,
    value: &str,
) -> Option<&'a crate::compiler::CompiledProps> {
    match node {
        CompiledRsxNode::Text { .. } => None,
        CompiledRsxNode::Element {
            props, children, ..
        } => {
            if props.attributes.get(name).map(String::as_str) == Some(value) {
                return Some(props);
            }
            children
                .iter()
                .find_map(|child| find_props_by_attribute(child, name, value))
        }
    }
}
