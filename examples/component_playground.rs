#![recursion_limit = "4096"]

#[path = "support/component_playground/mod.rs"]
mod component_playground;

use a3s_gui::CompiledRsxNode;
use component_playground::{
    component_playground_frame, shared_component_playground_component, ComponentPlaygroundState,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let component =
        shared_component_playground_component("component-playground", "A3S Component Playground")?;
    let frame = component_playground_frame(&component, &ComponentPlaygroundState::default())?;

    println!(
        "rendered {} with {} registered actions and {} root children",
        frame.frame_id,
        frame.actions.len(),
        root_child_count(&frame.root)
    );
    Ok(())
}

fn root_child_count(root: &CompiledRsxNode) -> usize {
    match root {
        CompiledRsxNode::Element { children, .. } => children.len(),
        CompiledRsxNode::Text { .. } => 0,
    }
}
