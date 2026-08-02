use std::io::Write;
use std::time::{Duration, Instant};

use a3s_gui::tsx_protocol::{
    write_tsx_json_frame_v1, TsxFrameLimitsV1, TsxHostApplicationSessionV1,
};
use a3s_gui::{GuiResult, SelfDrawnHostEventOutcome, SelfDrawnInputDispatch};

use super::runtime_backend::HostRuntime;

pub(super) const HOST_EVENT_POLL_INTERVAL: Duration = Duration::from_millis(8);
const MAXIMUM_HOST_EVENTS_PER_TICK: usize = 256;
const MAXIMUM_INTERACTION_TICKS_PER_TURN: usize = 64;

#[derive(Debug, Default)]
struct InteractionClock {
    anchor: Option<(Instant, u64)>,
}

impl InteractionClock {
    fn observe(&mut self, timestamp_micros: Option<u64>) {
        if let Some(timestamp_micros) = timestamp_micros {
            self.anchor = Some((Instant::now(), timestamp_micros));
        }
    }

    fn now_micros(&self) -> Option<u64> {
        self.anchor.map(|(instant, timestamp)| {
            timestamp.saturating_add(instant.elapsed().as_micros().min(u128::from(u64::MAX)) as u64)
        })
    }
}

#[derive(Debug, Default)]
pub(super) struct HostEventPump {
    interaction_clock: InteractionClock,
}

impl HostEventPump {
    pub(super) fn drain<W>(
        &mut self,
        output: &mut W,
        limits: TsxFrameLimitsV1,
        session: &mut TsxHostApplicationSessionV1,
        runtime: &mut Option<HostRuntime>,
    ) -> GuiResult<()>
    where
        W: Write,
    {
        self.drain_host_events(output, limits, session, runtime)?;
        self.drain_interaction_ticks(output, limits, session, runtime)
    }

    fn drain_host_events<W>(
        &mut self,
        output: &mut W,
        limits: TsxFrameLimitsV1,
        session: &mut TsxHostApplicationSessionV1,
        runtime: &mut Option<HostRuntime>,
    ) -> GuiResult<()>
    where
        W: Write,
    {
        let Some(runtime) = runtime.as_mut() else {
            return Ok(());
        };
        for _ in 0..MAXIMUM_HOST_EVENTS_PER_TICK {
            let Some(outcome) = runtime.poll_event()? else {
                break;
            };
            if let SelfDrawnHostEventOutcome::Input(dispatch) = outcome {
                self.interaction_clock
                    .observe(runtime.last_input_timestamp_micros());
                emit_runtime_dispatch(output, limits, session, &dispatch)?;
            }
        }
        Ok(())
    }

    fn drain_interaction_ticks<W>(
        &self,
        output: &mut W,
        limits: TsxFrameLimitsV1,
        session: &mut TsxHostApplicationSessionV1,
        runtime: &mut Option<HostRuntime>,
    ) -> GuiResult<()>
    where
        W: Write,
    {
        let Some(runtime) = runtime.as_mut() else {
            return Ok(());
        };
        for _ in 0..MAXIMUM_INTERACTION_TICKS_PER_TURN {
            let (Some(now), Some(deadline)) = (
                self.interaction_clock.now_micros(),
                runtime.next_interaction_deadline_micros(),
            ) else {
                break;
            };
            if now < deadline {
                break;
            }
            let Some(dispatch) = runtime.advance_interaction_time(deadline)? else {
                break;
            };
            emit_runtime_dispatch(output, limits, session, &dispatch)?;
        }
        Ok(())
    }
}

fn emit_runtime_dispatch<W>(
    output: &mut W,
    limits: TsxFrameLimitsV1,
    session: &mut TsxHostApplicationSessionV1,
    dispatch: &SelfDrawnInputDispatch,
) -> GuiResult<()>
where
    W: Write,
{
    let event = session.emit_self_drawn_event(dispatch)?;
    write_tsx_json_frame_v1(output, &event, limits)
}
