use crate::accessibility::{AccessibilityNode, AccessibilityTreeHost};
use crate::capability::{CapabilityHost, NativeCapabilities};
use crate::error::GuiResult;
use crate::event::NativeEvent;
use crate::host::{HostNodeId, NativeHost, ProgrammaticFocusHost};
use crate::native::{NativeElement, NativeProps};
use crate::platform::{
    BlueprintHost, NativeWidgetBlueprint, PlatformAdapter, PlatformPlanningHost,
};

use super::traits::{NativeEventHost, NativeEventSource, PlatformCommandExecutor};

#[derive(Debug)]
pub struct CommandExecutingHost<A: PlatformAdapter, E: PlatformCommandExecutor> {
    planning: PlatformPlanningHost<A>,
    executor: E,
    executed_commands: usize,
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> CommandExecutingHost<A, E> {
    pub fn new(adapter: A, executor: E) -> Self {
        Self {
            planning: PlatformPlanningHost::new(adapter),
            executor,
            executed_commands: 0,
        }
    }

    pub fn planning(&self) -> &PlatformPlanningHost<A> {
        &self.planning
    }

    pub fn executor(&self) -> &E {
        &self.executor
    }

    pub fn executor_mut(&mut self) -> &mut E {
        &mut self.executor
    }

    pub fn into_parts(self) -> (PlatformPlanningHost<A>, E) {
        (self.planning, self.executor)
    }

    fn flush_commands(&mut self) -> GuiResult<()> {
        while self.executed_commands < self.planning.commands().len() {
            let command = self.planning.commands()[self.executed_commands].clone();
            self.executor.execute(&command)?;
            self.executed_commands += 1;
        }
        Ok(())
    }

    fn commit_planning<T>(
        &mut self,
        apply: impl FnOnce(&mut PlatformPlanningHost<A>) -> GuiResult<T>,
    ) -> GuiResult<T> {
        let checkpoint = self.planning.checkpoint();
        let value = match apply(&mut self.planning) {
            Ok(value) => value,
            Err(error) => {
                self.planning.restore(checkpoint);
                return Err(error);
            }
        };
        if let Err(error) = self.flush_commands() {
            self.planning.restore(checkpoint);
            return Err(error);
        }
        Ok(value)
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor + NativeEventSource> NativeEventHost
    for CommandExecutingHost<A, E>
{
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.executor.take_native_events()
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> AccessibilityTreeHost
    for CommandExecutingHost<A, E>
{
    fn accessibility_tree(&self) -> Option<AccessibilityNode> {
        self.planning.accessibility_tree()
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> CapabilityHost for CommandExecutingHost<A, E> {
    fn native_capabilities(&self) -> NativeCapabilities {
        self.planning.capabilities()
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> BlueprintHost for CommandExecutingHost<A, E> {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint> {
        self.planning.blueprint(id)
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> NativeHost for CommandExecutingHost<A, E> {
    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        self.commit_planning(|planning| planning.create(element))
    }

    fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
        self.commit_planning(|planning| planning.update(id, props))
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.commit_planning(|planning| planning.insert_child(parent, child, index))
    }

    fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.commit_planning(|planning| planning.remove(id))
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.commit_planning(|planning| planning.set_root(id))
    }

    fn programmatic_focus_host(&mut self) -> Option<&mut dyn ProgrammaticFocusHost> {
        Some(self)
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> ProgrammaticFocusHost
    for CommandExecutingHost<A, E>
{
    fn request_focus(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.commit_planning(|planning| planning.request_focus(id))
    }
}
