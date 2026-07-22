use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::platform::{NativeWidgetBlueprint, PlatformCommand};

use super::traits::{
    NativeEventSource, NativeWidgetDriver, PlatformCommandBatch, PlatformCommandExecutor,
};

/// Maximum number of successfully executed commands retained for diagnostics by default.
pub const DEFAULT_DRIVER_COMMAND_HISTORY_LIMIT: usize = 256;

#[derive(Debug)]
pub struct DriverCommandExecutor<D: NativeWidgetDriver> {
    driver: D,
    commands: Vec<PlatformCommand>,
    command_history_limit: usize,
}

impl<D: NativeWidgetDriver> DriverCommandExecutor<D> {
    pub fn new(driver: D) -> Self {
        Self {
            driver,
            commands: Vec::new(),
            command_history_limit: DEFAULT_DRIVER_COMMAND_HISTORY_LIMIT,
        }
    }

    /// Creates an executor with a bounded diagnostic command history.
    ///
    /// A limit of zero disables command history without affecting command execution.
    pub fn with_command_history_limit(driver: D, command_history_limit: usize) -> Self {
        Self {
            driver,
            commands: Vec::new(),
            command_history_limit,
        }
    }

    pub fn driver(&self) -> &D {
        &self.driver
    }

    pub fn driver_mut(&mut self) -> &mut D {
        &mut self.driver
    }

    pub fn commands(&self) -> &[PlatformCommand] {
        &self.commands
    }

    pub fn command_history_limit(&self) -> usize {
        self.command_history_limit
    }

    /// Takes the retained diagnostic commands without affecting the native driver.
    pub fn take_commands(&mut self) -> Vec<PlatformCommand> {
        std::mem::take(&mut self.commands)
    }

    pub fn into_driver(self) -> D {
        self.driver
    }

    fn ensure_backend(&self, blueprint: &NativeWidgetBlueprint) -> GuiResult<()> {
        if blueprint.backend == self.driver.backend() {
            Ok(())
        } else {
            Err(GuiError::host(format!(
                "{:?} driver received {:?} blueprint",
                self.driver.backend(),
                blueprint.backend
            )))
        }
    }
}

impl<D: NativeWidgetDriver + Default> Default for DriverCommandExecutor<D> {
    fn default() -> Self {
        Self::new(D::default())
    }
}

impl<D: NativeWidgetDriver> PlatformCommandExecutor for DriverCommandExecutor<D> {
    fn prepare_batch(&mut self, batch: &PlatformCommandBatch) -> GuiResult<()> {
        for command in &batch.commands {
            match command {
                PlatformCommand::Create { blueprint, .. }
                | PlatformCommand::Update { blueprint, .. } => self.ensure_backend(blueprint)?,
                PlatformCommand::InsertChild { .. }
                | PlatformCommand::Remove { .. }
                | PlatformCommand::SetRoot { .. }
                | PlatformCommand::RequestFocus { .. } => {}
            }
        }
        Ok(())
    }

    fn execute(&mut self, command: &PlatformCommand) -> GuiResult<()> {
        match command {
            PlatformCommand::Create { id, blueprint } => {
                self.ensure_backend(blueprint)?;
                self.driver.create_widget(*id, blueprint)?;
            }
            PlatformCommand::Update { id, blueprint } => {
                self.ensure_backend(blueprint)?;
                self.driver.update_widget(*id, blueprint)?;
            }
            PlatformCommand::InsertChild {
                parent,
                child,
                index,
            } => {
                self.driver.insert_child(*parent, *child, *index)?;
            }
            PlatformCommand::Remove { id } => {
                self.driver.remove_widget(*id)?;
            }
            PlatformCommand::SetRoot { id } => {
                self.driver.set_root_widget(*id)?;
            }
            PlatformCommand::RequestFocus { id } => {
                self.driver.request_focus(*id)?;
            }
        }
        push_bounded(
            &mut self.commands,
            command.redacted_for_diagnostics(),
            self.command_history_limit,
        );
        Ok(())
    }
}

fn push_bounded<T>(items: &mut Vec<T>, item: T, limit: usize) {
    if limit == 0 {
        return;
    }
    if items.len() == limit {
        items.remove(0);
    }
    items.push(item);
}

impl<D: NativeWidgetDriver + NativeEventSource> NativeEventSource for DriverCommandExecutor<D> {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.driver.take_native_events()
    }
}
