use super::*;

impl<S, F, R> NativeRuntimeApp<AppKitRuntimeHost, S, F, R>
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &crate::event::ActionInvocation) -> GuiResult<ActionPropagation>,
{
    pub fn appkit_with_propagation(
        state: S,
        frame_builder: F,
        action_reducer: R,
    ) -> GuiResult<Self> {
        Ok(Self::new_with_propagation(
            AppKitNativeSurface::new()?.into_host(),
            state,
            frame_builder,
            action_reducer,
        ))
    }

    pub fn pump_appkit_event_with_propagation(
        &mut self,
        wait: AppKitEventWait,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_appkit_event_batch_with_propagation(wait)
            .map(|batch| batch.responses)
    }

    pub fn pump_appkit_event_with_propagation_while(
        &mut self,
        wait: AppKitEventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_appkit_event_batch_with_propagation_while(wait, &mut should_continue)
            .map(|batch| batch.responses)
    }

    pub fn pump_appkit_event_batch_with_propagation(
        &mut self,
        wait: AppKitEventWait,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        self.pump_appkit_event_batch_with_propagation_while(wait, |_| true)
    }

    pub fn pump_appkit_event_batch_with_propagation_while(
        &mut self,
        wait: AppKitEventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        let mut batch =
            self.handle_pending_native_event_batch_with_propagation_while(&mut should_continue)?;
        if !should_continue(self.state()) {
            return Ok(batch);
        }
        if pump_appkit_os_event(self, wait)? {
            batch.extend(
                self.handle_pending_native_event_batch_with_propagation_while(
                    &mut should_continue,
                )?,
            );
        }

        Ok(batch)
    }

    pub fn run_appkit_with_propagation(&mut self) -> GuiResult<()> {
        self.run_appkit_with_propagation_while(|_| true)
    }

    pub fn run_appkit_with_propagation_while(
        &mut self,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<()> {
        if self.root().is_none() {
            self.render()?;
        }
        while self.appkit_root_window_open() && should_continue(self.state()) {
            self.pump_appkit_event_with_propagation_while(
                AppKitEventWait::Wait,
                &mut should_continue,
            )?;
        }
        Ok(())
    }
}
