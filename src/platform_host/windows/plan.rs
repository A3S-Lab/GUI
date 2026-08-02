use std::collections::{BTreeMap, BTreeSet};

use crate::error::{GuiError, GuiResult};

use super::super::{
    PlatformAccessibilitySnapshot, PlatformHostCommand, PlatformHostTransaction,
    PlatformWindowCommand, PlatformWindowId, PlatformWindowSpec,
};

#[derive(Debug, Clone)]
pub(super) struct WindowsWindowState {
    pub spec: PlatformWindowSpec,
    pub accessibility: Option<PlatformAccessibilitySnapshot>,
    pub scene_fingerprint: Option<u64>,
}

impl WindowsWindowState {
    fn opened(spec: PlatformWindowSpec) -> Self {
        Self {
            spec,
            accessibility: None,
            scene_fingerprint: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct PreparedWindowsTransaction {
    pub transaction: PlatformHostTransaction,
    pub desired: BTreeMap<PlatformWindowId, WindowsWindowState>,
    pub opened: BTreeSet<PlatformWindowId>,
    pub closed: BTreeSet<PlatformWindowId>,
    pub redraw: BTreeSet<PlatformWindowId>,
}

pub(super) fn plan_transaction(
    transaction: PlatformHostTransaction,
    current: BTreeMap<PlatformWindowId, WindowsWindowState>,
) -> GuiResult<PreparedWindowsTransaction> {
    let mut desired = current;
    let mut opened = BTreeSet::new();
    let mut closed = BTreeSet::new();
    let mut redraw = BTreeSet::new();

    for command in &transaction.commands {
        match command {
            PlatformHostCommand::Window { command } => match command {
                PlatformWindowCommand::Open { spec } => {
                    validate_title(&spec.title)?;
                    if desired.contains_key(&spec.id) || !opened.insert(spec.id) {
                        return Err(GuiError::host(format!(
                            "Windows host window {} is already open",
                            spec.id.get()
                        )));
                    }
                    desired.insert(spec.id, WindowsWindowState::opened(spec.clone()));
                }
                PlatformWindowCommand::SetTitle { window, title } => {
                    validate_title(title)?;
                    window_state(&mut desired, *window)?
                        .spec
                        .title
                        .clone_from(title);
                }
                PlatformWindowCommand::Resize {
                    window,
                    logical_size,
                } => {
                    window_state(&mut desired, *window)?.spec.logical_size = *logical_size;
                }
                PlatformWindowCommand::SetConstraints {
                    window,
                    min_size,
                    max_size,
                } => {
                    let state = window_state(&mut desired, *window)?;
                    state.spec.min_size = *min_size;
                    state.spec.max_size = *max_size;
                }
                PlatformWindowCommand::SetResizable { window, resizable } => {
                    window_state(&mut desired, *window)?.spec.resizable = *resizable;
                }
                PlatformWindowCommand::SetVisible { window, visible } => {
                    window_state(&mut desired, *window)?.spec.visible = *visible;
                }
                PlatformWindowCommand::RequestRedraw { window } => {
                    window_state(&mut desired, *window)?;
                    redraw.insert(*window);
                }
                PlatformWindowCommand::Close { window } => {
                    if opened.contains(window) {
                        return Err(GuiError::host(
                            "Windows host cannot open and close one window in the same transaction",
                        ));
                    }
                    if desired.remove(window).is_none() {
                        return Err(missing_window(*window));
                    }
                    if !closed.insert(*window) || closed.len() > 1 {
                        return Err(GuiError::host(
                            "Windows host supports one destructive close per transaction",
                        ));
                    }
                    redraw.remove(window);
                }
            },
            PlatformHostCommand::Present { request } => {
                let state = window_state(&mut desired, request.window)?;
                if state.spec.logical_size != request.logical_size {
                    return Err(GuiError::host(format!(
                        "Windows host presentation size does not match window {}",
                        request.window.get()
                    )));
                }
                state.scene_fingerprint = Some(request.scene_fingerprint);
                redraw.insert(request.window);
            }
            PlatformHostCommand::Accessibility { snapshot } => {
                let state = window_state(&mut desired, snapshot.window)?;
                state.accessibility = Some((**snapshot).clone());
            }
            PlatformHostCommand::TextInput { .. } => {
                return Err(GuiError::host(
                    "Windows H2 host has not implemented the TSF text-input bridge",
                ));
            }
            PlatformHostCommand::System { .. } => {
                return Err(GuiError::host(
                    "Windows H2 host has not implemented Windows system services",
                ));
            }
        }
    }

    for state in desired.values() {
        state.spec.validate()?;
        validate_title(&state.spec.title)?;
    }

    Ok(PreparedWindowsTransaction {
        transaction,
        desired,
        opened,
        closed,
        redraw,
    })
}

fn window_state(
    windows: &mut BTreeMap<PlatformWindowId, WindowsWindowState>,
    window: PlatformWindowId,
) -> GuiResult<&mut WindowsWindowState> {
    windows
        .get_mut(&window)
        .ok_or_else(|| missing_window(window))
}

fn missing_window(window: PlatformWindowId) -> GuiError {
    GuiError::host(format!("Windows host window {} is not open", window.get()))
}

fn validate_title(title: &str) -> GuiResult<()> {
    if title.contains('\0') {
        return Err(GuiError::host(
            "Windows host window titles cannot contain NUL characters",
        ));
    }
    Ok(())
}
