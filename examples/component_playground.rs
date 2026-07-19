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
    let state = ComponentPlaygroundState::default();
    let frame = component_playground_frame(&component, &state)?;

    println!(
        "rendered {} in `{}` mode with {} registered actions and {} root children",
        frame.frame_id,
        state.active_section,
        frame.actions.len(),
        root_child_count(&frame.root)
    );
    println!(
        "open the native playground with: {}",
        native_playground_command()
    );
    Ok(())
}

fn native_playground_command() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "cargo run --features appkit-native --example appkit_component_playground"
    }
    #[cfg(target_os = "linux")]
    {
        "cargo run --features gtk4-native --example gtk4_component_playground"
    }
    #[cfg(target_os = "windows")]
    {
        "cargo run --features winui-native --example winui_component_playground"
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        "native component playground examples are available on macOS, Linux, and Windows"
    }
}

fn root_child_count(root: &CompiledRsxNode) -> usize {
    match root {
        CompiledRsxNode::Element { children, .. } => children.len(),
        CompiledRsxNode::Text { .. } => 0,
    }
}
