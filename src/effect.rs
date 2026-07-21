use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError};
use std::sync::Arc;
use std::thread;

use crate::error::{GuiError, GuiResult};

pub const DEFAULT_EFFECT_QUEUE_CAPACITY: usize = 256;
pub const DEFAULT_EFFECT_IN_FLIGHT_LIMIT: usize = 32;

/// Stable identity for one background effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EffectId(u64);

impl EffectId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Cooperative cancellation token passed to an effect task.
#[derive(Debug, Clone, Default)]
pub struct EffectCancellation {
    cancelled: Arc<AtomicBool>,
}

impl EffectCancellation {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

type EffectTask<M> = Box<dyn FnOnce(EffectCancellation) -> GuiResult<M> + Send + 'static>;

/// Thread-safe callback used to wake the owning UI event loop.
#[derive(Clone)]
pub struct EffectWaker(Arc<dyn Fn() + Send + Sync + 'static>);

impl Debug for EffectWaker {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EffectWaker")
            .finish_non_exhaustive()
    }
}

impl EffectWaker {
    pub fn new(wake: impl Fn() + Send + Sync + 'static) -> Self {
        Self(Arc::new(wake))
    }

    pub fn wake(&self) {
        (self.0)();
    }
}

/// A single unit of background work that eventually produces an application message.
pub struct Effect<M> {
    name: String,
    task: EffectTask<M>,
}

impl<M> Debug for Effect<M> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Effect")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl<M> Effect<M> {
    pub fn new(
        name: impl Into<String>,
        task: impl FnOnce(EffectCancellation) -> GuiResult<M> + Send + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            task: Box::new(task),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Result delivered back to the UI thread after an effect finishes.
#[derive(Debug)]
pub struct EffectCompletion<M> {
    pub id: EffectId,
    pub name: String,
    pub result: GuiResult<M>,
}

/// Executor-owned worker lifetime returned to the effect supervisor.
///
/// Joinable workers keep product I/O scoped to the runtime. Custom async
/// executors may return [`EffectWorker::detached`] only when another owned task
/// scope is responsible for shutdown and still sends the terminal completion.
pub struct EffectWorker {
    join: Option<Box<dyn FnOnce() -> GuiResult<()> + Send + 'static>>,
}

impl Debug for EffectWorker {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EffectWorker")
            .field("joinable", &self.join.is_some())
            .finish()
    }
}

impl EffectWorker {
    pub fn joinable(join: impl FnOnce() -> GuiResult<()> + Send + 'static) -> Self {
        Self {
            join: Some(Box::new(join)),
        }
    }

    pub fn detached() -> Self {
        Self { join: None }
    }

    fn join(mut self) -> GuiResult<()> {
        match self.join.take() {
            Some(join) => join(),
            None => Ok(()),
        }
    }
}

/// Executor seam used by the GUI core without depending on a particular async runtime.
pub trait EffectExecutor<M>: Send + Sync + 'static {
    /// Starts one worker and sends exactly one terminal completion after a
    /// successful spawn, even when cancellation was requested. The runtime
    /// suppresses cancelled results but uses that terminal message to release
    /// the bounded in-flight slot.
    fn spawn(
        &self,
        id: EffectId,
        effect: Effect<M>,
        cancellation: EffectCancellation,
        sender: SyncSender<EffectCompletion<M>>,
        wake: EffectWaker,
    ) -> GuiResult<EffectWorker>;
}

/// Simple executor for blocking work. Applications using Tokio can provide their own executor.
#[derive(Debug, Default, Clone, Copy)]
pub struct ThreadEffectExecutor;

impl<M: Send + 'static> EffectExecutor<M> for ThreadEffectExecutor {
    fn spawn(
        &self,
        id: EffectId,
        effect: Effect<M>,
        cancellation: EffectCancellation,
        sender: SyncSender<EffectCompletion<M>>,
        wake: EffectWaker,
    ) -> GuiResult<EffectWorker> {
        let Effect { name, task } = effect;
        let thread_name = format!("a3s-gui-effect-{}-{name}", id.get());
        let worker = thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                let result = if cancellation.is_cancelled() {
                    Err(GuiError::host(format!(
                        "background effect {name:?} was cancelled before it started"
                    )))
                } else {
                    catch_unwind(AssertUnwindSafe(|| task(cancellation.clone()))).unwrap_or_else(
                        |_| {
                            Err(GuiError::host(format!(
                                "background effect {name:?} panicked"
                            )))
                        },
                    )
                };
                // Always report worker termination, including after cancellation. The runtime
                // suppresses cancelled results but must retain the in-flight slot until the
                // underlying task has actually stopped.
                if sender.send(EffectCompletion { id, name, result }).is_ok() {
                    wake.wake();
                }
            })
            .map_err(|error| {
                GuiError::host(format!("failed to spawn background effect: {error}"))
            })?;
        Ok(EffectWorker::joinable(move || {
            worker
                .join()
                .map_err(|_| GuiError::host("background effect worker panicked outside its task"))
        }))
    }
}

/// UI-owned effect supervisor with a bounded completion queue.
pub struct EffectRuntime<M, E = ThreadEffectExecutor> {
    executor: E,
    sender: SyncSender<EffectCompletion<M>>,
    receiver: Receiver<EffectCompletion<M>>,
    wake: EffectWaker,
    active: BTreeMap<EffectId, ActiveEffect>,
    max_in_flight: usize,
    next_id: u64,
}

#[derive(Debug)]
struct ActiveEffect {
    cancellation: EffectCancellation,
    cancelled: bool,
    worker: EffectWorker,
}

impl<M, E: Debug> Debug for EffectRuntime<M, E> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("EffectRuntime")
            .field("executor", &self.executor)
            .field("active_effects", &self.active.len())
            .field("max_in_flight", &self.max_in_flight)
            .field("next_id", &self.next_id)
            .finish_non_exhaustive()
    }
}

impl<M: Send + 'static> EffectRuntime<M, ThreadEffectExecutor> {
    /// Runs each accepted effect on a dedicated background thread.
    pub fn threaded() -> Self {
        Self::new(ThreadEffectExecutor)
    }
}

impl<M: Send + 'static, E: EffectExecutor<M>> EffectRuntime<M, E> {
    pub fn new(executor: E) -> Self {
        Self::with_limits_and_waker(
            executor,
            DEFAULT_EFFECT_IN_FLIGHT_LIMIT,
            DEFAULT_EFFECT_QUEUE_CAPACITY,
            || {},
        )
    }

    pub fn with_capacity(executor: E, capacity: usize) -> Self {
        Self::with_capacity_and_waker(executor, capacity, || {})
    }

    pub fn with_capacity_and_waker(
        executor: E,
        capacity: usize,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let capacity = capacity.max(1);
        Self::with_limits_and_waker(executor, capacity, capacity, wake)
    }

    pub fn with_limits(executor: E, max_in_flight: usize, completion_capacity: usize) -> Self {
        Self::with_limits_and_waker(executor, max_in_flight, completion_capacity, || {})
    }

    pub fn with_limits_and_waker(
        executor: E,
        max_in_flight: usize,
        completion_capacity: usize,
        wake: impl Fn() + Send + Sync + 'static,
    ) -> Self {
        let (sender, receiver) = sync_channel(completion_capacity.max(1));
        Self {
            executor,
            sender,
            receiver,
            wake: EffectWaker::new(wake),
            active: BTreeMap::new(),
            max_in_flight: max_in_flight.max(1),
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, effect: Effect<M>) -> GuiResult<EffectId> {
        if self.active.len() >= self.max_in_flight {
            return Err(GuiError::host(format!(
                "background effect capacity {} is exhausted",
                self.max_in_flight
            )));
        }
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or_else(|| GuiError::host("background effect id space exhausted"))?;
        let id = EffectId::new(self.next_id);
        let cancellation = EffectCancellation::default();
        let worker = self.executor.spawn(
            id,
            effect,
            cancellation.clone(),
            self.sender.clone(),
            self.wake.clone(),
        )?;
        self.active.insert(
            id,
            ActiveEffect {
                cancellation,
                cancelled: false,
                worker,
            },
        );
        Ok(id)
    }

    pub fn cancel(&mut self, id: EffectId) -> bool {
        let Some(active) = self.active.get_mut(&id) else {
            return false;
        };
        if active.cancelled {
            return false;
        }
        active.cancelled = true;
        active.cancellation.cancel();
        true
    }

    pub fn cancel_all(&mut self) {
        for active in self.active.values_mut() {
            active.cancelled = true;
            active.cancellation.cancel();
        }
    }

    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    pub fn is_idle(&self) -> bool {
        self.active.is_empty()
    }

    pub fn drain(&mut self) -> Vec<EffectCompletion<M>> {
        let mut completions = Vec::new();
        loop {
            match self.receiver.try_recv() {
                Ok(mut completion) => {
                    let Some(active) = self.active.remove(&completion.id) else {
                        continue;
                    };
                    let cancelled = active.cancelled;
                    if let Err(error) = active.worker.join() {
                        completion.result = Err(error);
                    }
                    if !cancelled {
                        completions.push(completion);
                    }
                }
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
            }
        }
        completions
    }
}

impl<M, E> Drop for EffectRuntime<M, E> {
    fn drop(&mut self) {
        let active = std::mem::take(&mut self.active);
        for effect in active.values() {
            effect.cancellation.cancel();
        }

        // A worker can be blocked by the bounded completion channel. Disconnect
        // that channel before joining so scoped shutdown cannot deadlock.
        let (replacement_sender, replacement_receiver) = sync_channel(1);
        let receiver = std::mem::replace(&mut self.receiver, replacement_receiver);
        drop(receiver);
        self.sender = replacement_sender;

        for active in active.into_values() {
            let _ = active.worker.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    fn wait_for_completions<M: Send + 'static>(
        runtime: &mut EffectRuntime<M>,
        expected: usize,
    ) -> Vec<EffectCompletion<M>> {
        let deadline = Instant::now() + Duration::from_secs(2);
        let mut completions = Vec::new();
        while completions.len() < expected && Instant::now() < deadline {
            completions.extend(runtime.drain());
            if completions.len() < expected {
                thread::sleep(Duration::from_millis(1));
            }
        }
        completions
    }

    #[test]
    fn effect_runtime_delivers_messages_and_releases_active_entries() {
        let mut runtime = EffectRuntime::threaded();
        let first = runtime
            .spawn(Effect::new("first", |_| Ok::<_, GuiError>(1_u32)))
            .unwrap();
        let second = runtime
            .spawn(Effect::new("second", |_| Ok::<_, GuiError>(2_u32)))
            .unwrap();

        let mut completions = wait_for_completions(&mut runtime, 2);
        completions.sort_by_key(|completion| completion.id);

        assert_eq!(completions.len(), 2);
        assert_eq!(completions[0].id, first);
        assert_eq!(completions[0].result.as_ref().unwrap(), &1);
        assert_eq!(completions[1].id, second);
        assert_eq!(completions[1].result.as_ref().unwrap(), &2);
        assert!(runtime.is_idle());
    }

    #[test]
    fn effect_runtime_turns_panics_into_contextual_errors() {
        let mut runtime = EffectRuntime::<()>::threaded();
        runtime
            .spawn(Effect::new("panic", |_| -> GuiResult<()> {
                panic!("boom")
            }))
            .unwrap();

        let completions = wait_for_completions(&mut runtime, 1);

        assert_eq!(completions.len(), 1);
        assert!(completions[0]
            .result
            .as_ref()
            .unwrap_err()
            .to_string()
            .contains("background effect \"panic\" panicked"));
    }

    #[test]
    fn cancelled_effect_completion_is_not_delivered() {
        let mut runtime = EffectRuntime::threaded();
        let id = runtime
            .spawn(Effect::new("cancelled", |cancellation| {
                while !cancellation.is_cancelled() {
                    thread::yield_now();
                }
                Ok::<_, GuiError>(())
            }))
            .unwrap();

        assert!(runtime.cancel(id));
        let deadline = Instant::now() + Duration::from_millis(50);
        while Instant::now() < deadline {
            assert!(runtime.drain().is_empty());
            thread::sleep(Duration::from_millis(1));
        }
        assert!(runtime.is_idle());
    }

    #[test]
    fn effect_runtime_applies_backpressure_at_the_in_flight_limit() {
        let mut runtime = EffectRuntime::with_limits(ThreadEffectExecutor, 1, 1);
        let gate = Arc::new(AtomicBool::new(false));
        let task_gate = Arc::clone(&gate);
        runtime
            .spawn(Effect::new("held", move |_| {
                while !task_gate.load(Ordering::Acquire) {
                    thread::yield_now();
                }
                Ok::<_, GuiError>(())
            }))
            .unwrap();

        let error = runtime
            .spawn(Effect::new("rejected", |_| Ok::<_, GuiError>(())))
            .unwrap_err();
        assert!(error.to_string().contains("effect capacity 1 is exhausted"));

        gate.store(true, Ordering::Release);
        assert_eq!(wait_for_completions(&mut runtime, 1).len(), 1);
        runtime
            .spawn(Effect::new("accepted", |_| Ok::<_, GuiError>(())))
            .unwrap();
        assert_eq!(wait_for_completions(&mut runtime, 1).len(), 1);
    }

    #[test]
    fn cancellation_does_not_release_capacity_before_the_worker_stops() {
        let mut runtime = EffectRuntime::with_limits(ThreadEffectExecutor, 1, 1);
        let gate = Arc::new(AtomicBool::new(false));
        let started = Arc::new(AtomicBool::new(false));
        let task_gate = Arc::clone(&gate);
        let task_started = Arc::clone(&started);
        let id = runtime
            .spawn(Effect::new("non-cooperative", move |_| {
                task_started.store(true, Ordering::Release);
                while !task_gate.load(Ordering::Acquire) {
                    thread::yield_now();
                }
                Ok::<_, GuiError>(())
            }))
            .unwrap();

        let start_deadline = Instant::now() + Duration::from_secs(2);
        while !started.load(Ordering::Acquire) && Instant::now() < start_deadline {
            thread::yield_now();
        }
        assert!(started.load(Ordering::Acquire));

        assert!(runtime.cancel(id));
        assert_eq!(runtime.active_count(), 1);
        assert!(runtime
            .spawn(Effect::new("still-bounded", |_| Ok::<_, GuiError>(())))
            .unwrap_err()
            .to_string()
            .contains("effect capacity 1 is exhausted"));

        gate.store(true, Ordering::Release);
        let deadline = Instant::now() + Duration::from_secs(2);
        while !runtime.is_idle() && Instant::now() < deadline {
            assert!(runtime.drain().is_empty());
            thread::sleep(Duration::from_millis(1));
        }
        assert!(runtime.is_idle());
    }

    #[test]
    fn dropping_runtime_joins_a_non_cooperative_worker() {
        let started = Arc::new(AtomicBool::new(false));
        let release = Arc::new(AtomicBool::new(false));
        let finished = Arc::new(AtomicBool::new(false));
        let task_started = Arc::clone(&started);
        let task_release = Arc::clone(&release);
        let task_finished = Arc::clone(&finished);
        let mut runtime = EffectRuntime::with_limits(ThreadEffectExecutor, 1, 1);
        runtime
            .spawn(Effect::new("scoped", move |_| {
                task_started.store(true, Ordering::Release);
                while !task_release.load(Ordering::Acquire) {
                    thread::yield_now();
                }
                task_finished.store(true, Ordering::Release);
                Ok::<_, GuiError>(())
            }))
            .unwrap();

        let start_deadline = Instant::now() + Duration::from_secs(2);
        while !started.load(Ordering::Acquire) && Instant::now() < start_deadline {
            thread::yield_now();
        }
        assert!(started.load(Ordering::Acquire));

        let runtime_dropped = Arc::new(AtomicBool::new(false));
        let dropped = Arc::clone(&runtime_dropped);
        let drop_thread = thread::spawn(move || {
            drop(runtime);
            dropped.store(true, Ordering::Release);
        });
        thread::sleep(Duration::from_millis(20));
        assert!(!runtime_dropped.load(Ordering::Acquire));

        release.store(true, Ordering::Release);
        drop_thread.join().unwrap();
        assert!(finished.load(Ordering::Acquire));
        assert!(runtime_dropped.load(Ordering::Acquire));
    }
}
