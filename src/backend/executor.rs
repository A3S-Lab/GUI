use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::platform::{NativeWidgetBlueprint, PlatformCommand};

use super::traits::{NativeEventSource, NativeWidgetDriver, PlatformCommandExecutor};

#[derive(Debug)]
pub struct DriverCommandExecutor<D: NativeWidgetDriver> {
    driver: D,
    commands: Vec<PlatformCommand>,
}

impl<D: NativeWidgetDriver> DriverCommandExecutor<D> {
    pub fn new(driver: D) -> Self {
        Self {
            driver,
            commands: Vec::new(),
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
        self.commands.push(command.clone());
        Ok(())
    }
}

impl<D: NativeWidgetDriver + NativeEventSource> NativeEventSource for DriverCommandExecutor<D> {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        self.driver.take_native_events()
    }
}
