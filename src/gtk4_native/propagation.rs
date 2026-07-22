use super::*;

impl<S, F, R> NativeRuntimeApp<Gtk4RuntimeHost, S, F, R>
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &crate::event::ActionInvocation) -> GuiResult<ActionPropagation>,
{
    pub fn gtk4_with_propagation(state: S, frame_builder: F, action_reducer: R) -> GuiResult<Self> {
        Self::gtk4_with_application_id_and_propagation(
            "lab.a3s.gui",
            state,
            frame_builder,
            action_reducer,
        )
    }

    pub fn gtk4_with_application_id_and_propagation(
        application_id: &str,
        state: S,
        frame_builder: F,
        action_reducer: R,
    ) -> GuiResult<Self> {
        Ok(Self::new_with_propagation(
            Gtk4NativeSurface::with_application_id(application_id)?.into_host(),
            state,
            frame_builder,
            action_reducer,
        ))
    }

    pub fn pump_gtk4_event_with_propagation(
        &mut self,
        wait: Gtk4EventWait,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_gtk4_event_batch_with_propagation(wait)
            .map(|batch| batch.responses)
    }

    pub fn pump_gtk4_event_with_propagation_while(
        &mut self,
        wait: Gtk4EventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<Vec<NativeRuntimeEventResponse>> {
        self.pump_gtk4_event_batch_with_propagation_while(wait, &mut should_continue)
            .map(|batch| batch.responses)
    }

    pub fn pump_gtk4_event_batch_with_propagation(
        &mut self,
        wait: Gtk4EventWait,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        self.pump_gtk4_event_batch_with_propagation_while(wait, |_| true)
    }

    pub fn pump_gtk4_event_batch_with_propagation_while(
        &mut self,
        wait: Gtk4EventWait,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<NativeRuntimeEventBatch> {
        let mut batch =
            self.handle_pending_native_event_batch_with_propagation_while(&mut should_continue)?;
        if !should_continue(self.state()) {
            return Ok(batch);
        }
        if pump_gtk4_os_event(wait) {
            batch.extend(
                self.handle_pending_native_event_batch_with_propagation_while(
                    &mut should_continue,
                )?,
            );
        }
        Ok(batch)
    }

    pub fn run_gtk4_with_propagation(&mut self) -> GuiResult<()> {
        self.run_gtk4_with_propagation_while(|_| true)
    }

    pub fn run_gtk4_with_propagation_while(
        &mut self,
        mut should_continue: impl FnMut(&S) -> bool,
    ) -> GuiResult<()> {
        if self.root().is_none() {
            self.render()?;
        }
        while self.gtk4_root_window_open() && should_continue(self.state()) {
            self.pump_gtk4_event_with_propagation_while(Gtk4EventWait::Wait, &mut should_continue)?;
        }
        Ok(())
    }
}
