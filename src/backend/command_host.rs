use crate::error::GuiResult;
use crate::event::NativeEvent;
use crate::host::{HostNodeId, NativeHost};
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
        for command in &self.planning.commands()[self.executed_commands..] {
            self.executor.execute(command)?;
        }
        self.executed_commands = self.planning.commands().len();
        Ok(())
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor + NativeEventSource> NativeEventHost
    for CommandExecutingHost<A, E>
{
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.executor.take_native_events()
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> BlueprintHost for CommandExecutingHost<A, E> {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint> {
        self.planning.blueprint(id)
    }
}

impl<A: PlatformAdapter, E: PlatformCommandExecutor> NativeHost for CommandExecutingHost<A, E> {
    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        let id = self.planning.create(element)?;
        self.flush_commands()?;
        Ok(id)
    }

    fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
        self.planning.update(id, props)?;
        self.flush_commands()
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.planning.insert_child(parent, child, index)?;
        self.flush_commands()
    }

    fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.planning.remove(id)?;
        self.flush_commands()
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.planning.set_root(id)?;
        self.flush_commands()
    }
}
