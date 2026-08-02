use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;

use crate::error::{GuiError, GuiResult};

use super::super::PlatformHostEvent;

#[derive(Clone)]
pub(super) struct WindowsEventQueue {
    inner: Rc<WindowsEventQueueInner>,
}

struct WindowsEventQueueInner {
    state: RefCell<EventQueueState>,
    reentered: Cell<bool>,
}

struct EventQueueState {
    events: VecDeque<PlatformHostEvent>,
    limit: usize,
    suppressed: bool,
    failure: Option<String>,
}

impl WindowsEventQueue {
    pub(super) fn new(limit: usize) -> Self {
        Self {
            inner: Rc::new(WindowsEventQueueInner {
                state: RefCell::new(EventQueueState {
                    events: VecDeque::new(),
                    limit,
                    suppressed: false,
                    failure: None,
                }),
                reentered: Cell::new(false),
            }),
        }
    }

    pub(super) fn limit(&self) -> usize {
        self.inner.state.borrow().limit
    }

    pub(super) fn set_suppressed(&self, suppressed: bool) {
        self.inner.state.borrow_mut().suppressed = suppressed;
    }

    pub(super) fn push(&self, event: PlatformHostEvent) {
        let Ok(mut state) = self.inner.state.try_borrow_mut() else {
            self.inner.reentered.set(true);
            return;
        };
        if state.suppressed {
            return;
        }
        if let Err(error) = event.validate() {
            state.failure = Some(format!("Windows host produced an invalid event: {error}"));
            return;
        }
        if state.events.len() >= state.limit {
            state.failure = Some(format!(
                "Windows host event queue reached its {}-event limit",
                state.limit
            ));
            return;
        }
        state.events.push_back(event);
    }

    pub(super) fn fail(&self, message: impl Into<String>) {
        let Ok(mut state) = self.inner.state.try_borrow_mut() else {
            self.inner.reentered.set(true);
            return;
        };
        if !state.suppressed {
            state.failure = Some(message.into());
        }
    }

    pub(super) fn pop(&self) -> GuiResult<Option<PlatformHostEvent>> {
        if self.inner.reentered.replace(false) {
            return Err(GuiError::host("Windows host event queue was re-entered"));
        }
        let mut state = self
            .inner
            .state
            .try_borrow_mut()
            .map_err(|_| GuiError::host("Windows host event queue is re-entered"))?;
        if let Some(failure) = state.failure.take() {
            return Err(GuiError::host(failure));
        }
        Ok(state.events.pop_front())
    }

    pub(super) fn clear(&self) {
        let mut state = self.inner.state.borrow_mut();
        state.events.clear();
        state.failure = None;
        self.inner.reentered.set(false);
    }
}
