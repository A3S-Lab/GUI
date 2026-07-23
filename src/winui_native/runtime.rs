use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;

use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{KillTimer, SetTimer};
use windows_core::{implement, IInspectable_Vtbl, Interface, Ref, HRESULT};
use winui3::bootstrap::PackageDependency;
use winui3::Microsoft::UI::Dispatching::{
    DispatcherQueue, DispatcherQueueHandler, DispatcherQueueTimer,
};
use winui3::Microsoft::UI::Xaml as xaml;
use winui3::{
    ChildClass, ChildClassImpl, Compose, CreateInstanceFn,
    Microsoft::UI::Xaml::{
        ApplicationInitializationCallback, IApplicationFactory, IApplicationFactory_Vtbl,
        IApplicationOverrides, IApplicationOverrides_Impl, LaunchActivatedEventArgs,
    },
};

use super::{map_winui, GuiError, GuiResult};

const E_FAIL: HRESULT = HRESULT(0x8000_4005_u32 as i32);
type WinUiLaunchTask = Box<dyn FnOnce(xaml::Application) + Send + 'static>;
type WinUiUiTask = Box<dyn FnOnce() + 'static>;
type WinUiFuture<T> = Pin<Box<dyn Future<Output = GuiResult<T>> + 'static>>;

static NEXT_WINUI_FUTURE_TASK_ID: AtomicUsize = AtomicUsize::new(1);

thread_local! {
    static WINUI_UI_TASK: RefCell<Option<WinUiUiTask>> = RefCell::new(None);
    static WINUI_FUTURE_TASKS: RefCell<HashMap<usize, Box<dyn WinUiLocalFutureTask>>> =
        RefCell::new(HashMap::new());
}

#[implement(IApplicationOverrides)]
struct WinUiApplication {
    task: Mutex<Option<WinUiLaunchTask>>,
}

impl WinUiApplication {
    fn compose(task: WinUiLaunchTask) -> windows_core::Result<xaml::Application> {
        Compose::compose(Self {
            task: Mutex::new(Some(task)),
        })
    }
}

impl ChildClassImpl for WinUiApplication_Impl {}

impl IApplicationOverrides_Impl for WinUiApplication_Impl {
    fn OnLaunched(&self, _args: Ref<LaunchActivatedEventArgs>) -> windows_core::Result<()> {
        let task = self
            .task
            .lock()
            .map_err(|_| windows_core::Error::from_hresult(E_FAIL))?
            .take()
            .ok_or_else(|| windows_core::Error::from_hresult(E_FAIL))?;
        let application = self.base()?.cast::<xaml::Application>()?;
        task(application);
        Ok(())
    }
}

impl ChildClass for WinUiApplication {
    type BaseType = xaml::Application;
    type FactoryInterface = IApplicationFactory;

    fn create_interface_fn(vtable: &IApplicationFactory_Vtbl) -> CreateInstanceFn {
        vtable.CreateInstance
    }

    fn identity_vtable(vtable: &mut Self::Outer) -> &mut &'static IInspectable_Vtbl {
        &mut vtable.identity
    }

    fn ref_count(vtable: &Self::Outer) -> &windows_core::imp::WeakRefCount {
        &vtable.count
    }

    fn into_outer(self) -> Self::Outer {
        Self::into_outer(self)
    }
}

fn schedule_winui_ui_task(task: WinUiUiTask) -> windows_core::Result<()> {
    let stored = WINUI_UI_TASK.with(|slot| {
        let Ok(mut slot) = slot.try_borrow_mut() else {
            return false;
        };
        if slot.is_some() {
            false
        } else {
            *slot = Some(task);
            true
        }
    });
    if !stored {
        return Err(windows_core::Error::from_hresult(E_FAIL));
    }
    let timer = unsafe { SetTimer(None, 0, 1, Some(run_winui_ui_task)) };
    if timer == 0 {
        WINUI_UI_TASK.with(|slot| {
            if let Ok(mut slot) = slot.try_borrow_mut() {
                slot.take();
            }
        });
        return Err(windows_core::Error::from_hresult(E_FAIL));
    }
    Ok(())
}

unsafe extern "system" fn run_winui_ui_task(
    _window: HWND,
    _message: u32,
    timer: usize,
    _time: u32,
) {
    let _ = KillTimer(None, timer);
    WINUI_UI_TASK.with(|slot| {
        let task = slot.try_borrow_mut().ok().and_then(|mut slot| slot.take());
        if let Some(task) = task {
            task();
        }
    });
}

/// Runs a synchronous task inside the WinUI-owned application lifecycle.
pub fn run_winui_application<T, F>(task: F) -> GuiResult<T>
where
    T: Send + 'static,
    F: FnOnce() -> GuiResult<T> + Send + 'static,
{
    let result = Arc::new(Mutex::new(None));
    let launch_result = Arc::clone(&result);
    let launch_task: WinUiLaunchTask = Box::new(move |application| {
        store_task_result(&launch_result, catch_gui_task(task));
        let _ = application.Exit();
    });
    run_winui_lifecycle(launch_task, result)
}

/// Runs a WinUI task in two UI-thread turns so the initial XAML tree can load.
///
/// The first stage should construct and render the native app. WinUI regains
/// control before the second stage receives that app. Keep the second stage
/// short; event loops and automation that wait for native input should use
/// [`run_winui_application_staged_async`].
pub fn run_winui_application_staged<A, T, I, F>(initial: I, finish: F) -> GuiResult<T>
where
    A: 'static,
    T: Send + 'static,
    I: FnOnce() -> GuiResult<A> + Send + 'static,
    F: FnOnce(A) -> GuiResult<T> + Send + 'static,
{
    let result = Arc::new(Mutex::new(None));
    let launch_result = Arc::clone(&result);
    let launch_task: WinUiLaunchTask = Box::new(move |application| match catch_gui_task(initial) {
        Ok(stage) => {
            let finish_result = Arc::clone(&launch_result);
            let exit_application = application.clone();
            let task = Box::new(move || {
                store_task_result(&finish_result, catch_gui_task(|| finish(stage)));
                let _ = exit_application.Exit();
            });
            if let Err(error) = schedule_winui_ui_task(task) {
                store_task_result(
                    &launch_result,
                    Err(GuiError::host(format!(
                        "failed to schedule the second WinUI application stage: {error}"
                    ))),
                );
                let _ = application.Exit();
            }
        }
        Err(error) => {
            store_task_result(&launch_result, Err(error));
            let _ = application.Exit();
        }
    });
    run_winui_lifecycle(launch_task, result)
}

/// Runs a WinUI task as a future after its initial XAML tree has been mounted.
///
/// The initial stage runs from `Application::OnLaunched` and must create and
/// activate the first window. The asynchronous stage is polled through the
/// WinUI dispatcher after `OnLaunched` returns, so native input and layout can
/// continue while the future is pending.
pub fn run_winui_application_staged_async<A, T, I, F, Fut>(initial: I, finish: F) -> GuiResult<T>
where
    A: 'static,
    T: Send + 'static,
    I: FnOnce() -> GuiResult<A> + Send + 'static,
    F: FnOnce(A) -> Fut + Send + 'static,
    Fut: Future<Output = GuiResult<T>> + 'static,
{
    let result = Arc::new(Mutex::new(None));
    let launch_result = Arc::clone(&result);
    let launch_task: WinUiLaunchTask = Box::new(move |application| match catch_gui_task(initial) {
        Ok(stage) => {
            let queue = match DispatcherQueue::GetForCurrentThread() {
                Ok(queue) => queue,
                Err(error) => {
                    store_task_result(
                        &launch_result,
                        Err(GuiError::host(format!(
                            "failed to access the WinUI dispatcher queue: {error}"
                        ))),
                    );
                    let _ = application.Exit();
                    return;
                }
            };
            let task_id = NEXT_WINUI_FUTURE_TASK_ID.fetch_add(1, Ordering::Relaxed);
            let inserted = WINUI_FUTURE_TASKS.with(|tasks| {
                let Ok(mut tasks) = tasks.try_borrow_mut() else {
                    return false;
                };
                tasks.insert(
                    task_id,
                    Box::new(WinUiLocalFutureTaskState {
                        future: Box::pin(finish(stage)),
                        result: Arc::clone(&launch_result),
                        application: application.clone(),
                    }),
                );
                true
            });
            if !inserted {
                store_task_result(
                    &launch_result,
                    Err(GuiError::host(
                        "failed to store the WinUI future task on its UI thread",
                    )),
                );
                let _ = application.Exit();
                return;
            }
            let task = Arc::new(WinUiFutureWaker {
                task_id,
                result: Arc::clone(&launch_result),
                application,
                queue,
                queued: AtomicBool::new(false),
            });
            if let Err(error) = task.schedule() {
                remove_winui_future_task(task_id);
                task.finish(Err(error));
            }
        }
        Err(error) => {
            store_task_result(&launch_result, Err(error));
            let _ = application.Exit();
        }
    });
    run_winui_lifecycle(launch_task, result)
}

/// Waits without blocking the WinUI application dispatcher.
pub async fn wait_winui_dispatcher(duration: Duration) -> GuiResult<()> {
    WinUiDispatcherDelay::new(duration)?.await
}

trait WinUiLocalFutureTask {
    fn poll(&mut self, context: &mut Context<'_>) -> Poll<()>;
}

struct WinUiLocalFutureTaskState<T> {
    future: WinUiFuture<T>,
    result: Arc<Mutex<Option<GuiResult<T>>>>,
    application: xaml::Application,
}

impl<T> WinUiLocalFutureTask for WinUiLocalFutureTaskState<T>
where
    T: Send + 'static,
{
    fn poll(&mut self, context: &mut Context<'_>) -> Poll<()> {
        match catch_unwind(AssertUnwindSafe(|| self.future.as_mut().poll(context))) {
            Ok(Poll::Ready(result)) => {
                store_task_result(&self.result, result);
                let _ = self.application.Exit();
                Poll::Ready(())
            }
            Ok(Poll::Pending) => Poll::Pending,
            Err(panic) => {
                store_task_result(
                    &self.result,
                    Err(GuiError::host(format!(
                        "WinUI future task panicked: {}",
                        panic_message(panic)
                    ))),
                );
                let _ = self.application.Exit();
                Poll::Ready(())
            }
        }
    }
}

struct WinUiFutureWaker<T> {
    task_id: usize,
    result: Arc<Mutex<Option<GuiResult<T>>>>,
    application: xaml::Application,
    queue: DispatcherQueue,
    queued: AtomicBool,
}

impl<T> WinUiFutureWaker<T>
where
    T: Send + 'static,
{
    fn schedule(self: &Arc<Self>) -> GuiResult<()> {
        if self.queued.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        let task = Arc::clone(self);
        let handler = DispatcherQueueHandler::new(move || {
            task.queued.store(false, Ordering::Release);
            poll_winui_future_task(task.task_id, Arc::clone(&task));
            Ok(())
        });
        let accepted = self.queue.TryEnqueue(&handler).map_err(|error| {
            self.queued.store(false, Ordering::Release);
            GuiError::host(format!("failed to enqueue the WinUI future task: {error}"))
        })?;
        if !accepted {
            self.queued.store(false, Ordering::Release);
            return Err(GuiError::host("WinUI dispatcher rejected the future task"));
        }
        Ok(())
    }

    fn finish(&self, result: GuiResult<T>) {
        store_task_result(&self.result, result);
        let _ = self.application.Exit();
    }
}

impl<T> Wake for WinUiFutureWaker<T>
where
    T: Send + 'static,
{
    fn wake(self: Arc<Self>) {
        if let Err(error) = self.schedule() {
            self.finish(Err(error));
        }
    }

    fn wake_by_ref(self: &Arc<Self>) {
        if let Err(error) = self.schedule() {
            self.finish(Err(error));
        }
    }
}

fn poll_winui_future_task<T>(task_id: usize, task_waker: Arc<WinUiFutureWaker<T>>)
where
    T: Send + 'static,
{
    let task = WINUI_FUTURE_TASKS.with(|tasks| {
        tasks
            .try_borrow_mut()
            .ok()
            .and_then(|mut tasks| tasks.remove(&task_id))
    });
    let Some(mut task) = task else {
        return;
    };
    let waker = Waker::from(Arc::clone(&task_waker));
    let mut context = Context::from_waker(&waker);
    if task.poll(&mut context).is_pending() {
        let restored = WINUI_FUTURE_TASKS.with(|tasks| {
            let Ok(mut tasks) = tasks.try_borrow_mut() else {
                return false;
            };
            tasks.insert(task_id, task);
            true
        });
        if !restored {
            task_waker.finish(Err(GuiError::host(
                "failed to restore the pending WinUI future task on its UI thread",
            )));
        }
    }
}

fn remove_winui_future_task(task_id: usize) {
    WINUI_FUTURE_TASKS.with(|tasks| {
        if let Ok(mut tasks) = tasks.try_borrow_mut() {
            tasks.remove(&task_id);
        }
    });
}

struct WinUiDispatcherDelay {
    timer: DispatcherQueueTimer,
    token: i64,
    state: Arc<Mutex<WinUiDispatcherDelayState>>,
}

#[derive(Default)]
struct WinUiDispatcherDelayState {
    fired: bool,
    waker: Option<Waker>,
}

impl WinUiDispatcherDelay {
    fn new(duration: Duration) -> GuiResult<Self> {
        let queue = map_winui(
            "failed to access the WinUI dispatcher queue",
            DispatcherQueue::GetForCurrentThread(),
        )?;
        let timer = map_winui(
            "failed to create a WinUI dispatcher timer",
            queue.CreateTimer(),
        )?;
        let ticks = duration.as_nanos().div_ceil(100).clamp(1, i64::MAX as u128) as i64;
        map_winui(
            "failed to configure the WinUI dispatcher timer interval",
            timer.SetInterval(windows::Foundation::TimeSpan { Duration: ticks }),
        )?;
        map_winui(
            "failed to configure the WinUI dispatcher timer",
            timer.SetIsRepeating(false),
        )?;
        let state = Arc::new(Mutex::new(WinUiDispatcherDelayState::default()));
        let handler_state = Arc::clone(&state);
        let handler = windows::Foundation::TypedEventHandler::<
            DispatcherQueueTimer,
            windows_core::IInspectable,
        >::new(move |_, _| {
            let waker = handler_state.lock().ok().and_then(|mut state| {
                state.fired = true;
                state.waker.take()
            });
            if let Some(waker) = waker {
                waker.wake();
            }
            Ok(())
        });
        let token = map_winui(
            "failed to register the WinUI dispatcher timer",
            timer.Tick(&handler),
        )?;
        if let Err(error) = timer.Start() {
            let _ = timer.RemoveTick(token);
            return Err(GuiError::host(format!(
                "failed to start the WinUI dispatcher timer: {error}"
            )));
        }
        Ok(Self {
            timer,
            token,
            state,
        })
    }
}

impl Future for WinUiDispatcherDelay {
    type Output = GuiResult<()>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                return Poll::Ready(Err(GuiError::host(
                    "WinUI dispatcher timer lock was poisoned",
                )))
            }
        };
        if state.fired {
            Poll::Ready(Ok(()))
        } else {
            state.waker = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

impl Drop for WinUiDispatcherDelay {
    fn drop(&mut self) {
        let _ = self.timer.Stop();
        let _ = self.timer.RemoveTick(self.token);
    }
}

fn run_winui_lifecycle<T>(
    task: WinUiLaunchTask,
    result: Arc<Mutex<Option<GuiResult<T>>>>,
) -> GuiResult<T>
where
    T: Send + 'static,
{
    map_winui(
        "failed to initialize the WinRT single-threaded apartment",
        winui3::init_apartment(winui3::ApartmentType::SingleThreaded),
    )?;
    let _package_dependency = map_winui(
        "failed to initialize Windows App SDK package dependency",
        PackageDependency::initialize(),
    )?;

    let callback_task = Arc::new(Mutex::new(Some(task)));
    let application_callback = ApplicationInitializationCallback::new(move |_| {
        let task = callback_task
            .lock()
            .map_err(|_| windows_core::Error::from_hresult(E_FAIL))?
            .take()
            .ok_or_else(|| windows_core::Error::from_hresult(E_FAIL))?;
        let _application = WinUiApplication::compose(task)?;
        Ok(())
    });
    map_winui(
        "failed to run the WinUI XAML application",
        xaml::Application::Start(&application_callback),
    )?;

    let task_result = result
        .lock()
        .map_err(|_| GuiError::host("WinUI application result lock was poisoned"))?
        .take()
        .ok_or_else(|| GuiError::host("WinUI application exited before running its task"))?;
    task_result
}

fn catch_gui_task<T>(task: impl FnOnce() -> GuiResult<T>) -> GuiResult<T> {
    catch_unwind(AssertUnwindSafe(task)).unwrap_or_else(|panic| {
        Err(GuiError::host(format!(
            "WinUI application task panicked: {}",
            panic_message(panic)
        )))
    })
}

fn store_task_result<T>(result: &Arc<Mutex<Option<GuiResult<T>>>>, task_result: GuiResult<T>) {
    if let Ok(mut result) = result.lock() {
        *result = Some(task_result);
    }
}

fn panic_message(panic: Box<dyn Any + Send>) -> String {
    panic
        .downcast_ref::<&str>()
        .map(|message| (*message).to_owned())
        .or_else(|| panic.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "unknown panic payload".to_owned())
}

pub(super) struct WinUiThreadRuntime {
    _application: xaml::Application,
    _package_dependency: Option<PackageDependency>,
}

impl WinUiThreadRuntime {
    pub(super) fn current(package_dependency: Option<PackageDependency>) -> GuiResult<Self> {
        map_winui(
            "WinUI surface must be created inside run_winui_application",
            DispatcherQueue::GetForCurrentThread(),
        )?;
        let application = map_winui(
            "WinUI surface must be created inside run_winui_application",
            xaml::Application::Current(),
        )?;
        Ok(Self {
            _application: application,
            _package_dependency: package_dependency,
        })
    }
}
