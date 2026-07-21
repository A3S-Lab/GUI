mod command_host;
mod driver;
mod executor;
mod recording;
mod surface;
#[cfg(test)]
mod tests;
mod traits;

pub use command_host::{CommandExecutingHost, DegradedNativeState};
pub use driver::HandleWidgetDriver;
pub use executor::{DriverCommandExecutor, DEFAULT_DRIVER_COMMAND_HISTORY_LIMIT};
pub use recording::{
    RecordedNativeObject, RecordingBackend, DEFAULT_RECORDING_COMMAND_HISTORY_LIMIT,
};
pub use surface::SurfaceHandleAdapter;
pub use traits::{
    NativeEventHost, NativeEventSource, NativeHandleAdapter, NativeWidgetDriver,
    NativeWidgetSurface, PlatformBatchAck, PlatformBatchFailure, PlatformCommandBatch,
    PlatformCommandExecutor,
};
