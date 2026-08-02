use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};
use crate::platform_host::PlatformWindowId;

use super::native::NativeWindow;
use super::plan::PreparedWindowsTransaction;
use super::WindowsPlatformHost;

impl WindowsPlatformHost {
    pub(super) fn stage_opened(&mut self, prepared: &PreparedWindowsTransaction) -> GuiResult<()> {
        let mut staged = BTreeMap::<PlatformWindowId, NativeWindow>::new();
        for window in &prepared.opened {
            let Some(state) = prepared.desired.get(window) else {
                return Err(GuiError::host(
                    "Windows host prepared an open window without desired state",
                ));
            };
            match NativeWindow::create(self.hinstance, &state.spec, self.events.clone()) {
                Ok(native) => {
                    staged.insert(*window, native);
                }
                Err(primary) => {
                    let mut failures = Vec::new();
                    for (created_window, native) in &mut staged {
                        if let Err(error) = native.destroy() {
                            failures.push(format!("window {}: {error}", created_window.get()));
                        }
                    }
                    return if failures.is_empty() {
                        Err(primary)
                    } else {
                        Err(GuiError::host(format!(
                            "{primary}; Windows host staging cleanup also failed: {}",
                            failures.join("; ")
                        )))
                    };
                }
            }
        }
        self.staged = staged;
        Ok(())
    }

    pub(super) fn destroy_staged(&mut self) -> GuiResult<()> {
        let windows = self.staged.keys().copied().collect::<Vec<_>>();
        let mut failures = Vec::new();
        for window in windows {
            let result = self
                .staged
                .get_mut(&window)
                .map_or(Ok(()), NativeWindow::destroy);
            match result {
                Ok(()) => {
                    self.staged.remove(&window);
                }
                Err(error) => failures.push(format!("window {}: {error}", window.get())),
            }
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(GuiError::host(format!(
                "Windows host could not discard staged windows: {}",
                failures.join("; ")
            )))
        }
    }
}
