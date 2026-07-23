use super::*;

impl<S, F, R> NativeRuntimeApp<WinUiRuntimeHost, S, F, R>
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &crate::event::ActionInvocation) -> GuiResult<ActionPropagation>,
{
    pub fn winui_with_propagation(
        state: S,
        frame_builder: F,
        action_reducer: R,
    ) -> GuiResult<Self> {
        Ok(Self::new_with_propagation(
            WinUiNativeSurface::new()?.into_host(),
            state,
            frame_builder,
            action_reducer,
        ))
    }

    pub fn pump_winui_event_with_propagation(
        &mut self,
        wait: WinUiEventWait,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_winui_event_batch_with_propagation(wait)
            .map(|batch| batch.responses)
    }

    pub fn pump_winui_event_with_propagation_while(
        &mut self,
        wait: WinUiEventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_winui_event_batch_with_propagation_while(wait, &mut should_continue)
            .map(|batch| batch.responses)
    }

    pub fn pump_winui_event_batch_with_propagation(
        &mut self,
        wait: WinUiEventWait,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        self.pump_winui_event_batch_with_propagation_while(wait, |_| true)
    }

    pub fn pump_winui_event_batch_with_propagation_while(
        &mut self,
        wait: WinUiEventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        let mut batch =
            self.handle_pending_native_event_batch_with_propagation_while(&mut should_continue)?;
        if !should_continue(self.state()) {
            return Ok(batch);
        }
        if pump_winui_os_event(self, wait)? {
            batch.extend(
                self.handle_pending_native_event_batch_with_propagation_while(
                    &mut should_continue,
                )?,
            );
        }
        Ok(batch)
    }

    pub fn run_winui_with_propagation(&mut self) -> GuiResult<()> {
        self.run_winui_with_propagation_while(|_| true)
    }

    pub fn run_winui_with_propagation_while(
        &mut self,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<()> {
        if self.root().is_none() {
            self.render()?;
        }
        while self.winui_root_window_open() && should_continue(self.state()) {
            self.pump_winui_event_with_propagation_while(
                WinUiEventWait::Wait,
                &mut should_continue,
            )?;
        }
        Ok(())
    }

    pub async fn run_winui_with_propagation_async(&mut self) -> GuiResult<()> {
        self.run_winui_with_propagation_while_async(|_| true).await
    }

    pub async fn run_winui_with_propagation_while_async(
        &mut self,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<()> {
        if self.root().is_none() {
            self.render()?;
        }
        while self.winui_root_window_open() && should_continue(self.state()) {
            self.pump_winui_event_with_propagation_while(
                WinUiEventWait::Poll,
                &mut should_continue,
            )?;
            wait_winui_dispatcher(std::time::Duration::from_millis(8)).await?;
        }
        Ok(())
    }
}
