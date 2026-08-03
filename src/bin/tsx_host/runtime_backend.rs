use a3s_gui::tsx_protocol::{TsxHostCapabilityV1, TsxHostPlatformV1, TsxRendererV1};
#[cfg(not(any(
    all(target_os = "windows", feature = "host-windows", feature = "gpu"),
    feature = "software-reference"
)))]
use a3s_gui::GuiError;
#[cfg(any(
    all(target_os = "windows", feature = "host-windows", feature = "gpu"),
    feature = "software-reference"
))]
use a3s_gui::SelfDrawnWindowRuntime;
use a3s_gui::{
    GuiResult, NativeElement, PlatformWindowSpec, SelfDrawnFrameCommit, SelfDrawnFrameSnapshot,
    SelfDrawnHostEventOutcome, SelfDrawnInputDispatch,
};

#[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
use a3s_gui::drawing::{GpuPowerPreference, GpuRendererOptions};
#[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
use a3s_gui::platform_host::WindowsPlatformHost;
#[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
use a3s_gui::GpuScenePresenter;
#[cfg(all(
    feature = "software-reference",
    not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
))]
use a3s_gui::{RecordingPlatformHost, ReferenceScenePresenter};

#[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
type NativeRuntime = SelfDrawnWindowRuntime<WindowsPlatformHost, GpuScenePresenter>;

#[cfg(all(
    feature = "software-reference",
    not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
))]
type ReferenceRuntime = SelfDrawnWindowRuntime<RecordingPlatformHost, ReferenceScenePresenter>;

#[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
const NATIVE_CAPABILITIES: &[TsxHostCapabilityV1] = &[TsxHostCapabilityV1::SelfDrawnRendering];
#[cfg(all(
    feature = "software-reference",
    not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
))]
const REFERENCE_CAPABILITIES: &[TsxHostCapabilityV1] = &[
    TsxHostCapabilityV1::HeadlessRendering,
    TsxHostCapabilityV1::SelfDrawnRendering,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct HostRuntimeProfile {
    pub(super) platform: TsxHostPlatformV1,
    pub(super) renderer: TsxRendererV1,
    pub(super) capabilities: &'static [TsxHostCapabilityV1],
    pub(super) window_visible: bool,
}

pub(super) fn selected_profile() -> GuiResult<HostRuntimeProfile> {
    #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
    {
        return Ok(HostRuntimeProfile {
            platform: TsxHostPlatformV1::Windows,
            renderer: TsxRendererV1::Gpu,
            capabilities: NATIVE_CAPABILITIES,
            window_visible: true,
        });
    }

    #[cfg(all(
        feature = "software-reference",
        not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
    ))]
    {
        return Ok(HostRuntimeProfile {
            platform: TsxHostPlatformV1::Headless,
            renderer: TsxRendererV1::Software,
            capabilities: REFERENCE_CAPABILITIES,
            window_visible: false,
        });
    }

    #[cfg(not(any(
        all(target_os = "windows", feature = "host-windows", feature = "gpu"),
        feature = "software-reference"
    )))]
    {
        Err(GuiError::host(
            "TSX host has no renderer backend; enable host-windows,gpu on Windows",
        ))
    }
}

#[cfg(any(
    all(target_os = "windows", feature = "host-windows", feature = "gpu"),
    feature = "software-reference"
))]
pub(super) enum HostRuntime {
    #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
    Native(NativeRuntime),
    #[cfg(all(
        feature = "software-reference",
        not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
    ))]
    Reference(ReferenceRuntime),
}

#[cfg(not(any(
    all(target_os = "windows", feature = "host-windows", feature = "gpu"),
    feature = "software-reference"
)))]
pub(super) struct HostRuntime;

impl HostRuntime {
    pub(super) fn new(window_spec: PlatformWindowSpec) -> GuiResult<Self> {
        #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
        {
            let host = WindowsPlatformHost::new()?;
            let scale_factor = host.initial_scale_factor()?;
            let presenter = GpuScenePresenter::with_options(GpuRendererOptions {
                power_preference: GpuPowerPreference::None,
                allow_software_adapter: true,
                ..GpuRendererOptions::default()
            });
            return NativeRuntime::new(host, presenter, window_spec, scale_factor)
                .map(Self::Native);
        }

        #[cfg(all(
            feature = "software-reference",
            not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
        ))]
        {
            return ReferenceRuntime::new(
                RecordingPlatformHost::new(),
                ReferenceScenePresenter::new(),
                window_spec,
                1.0,
            )
            .map(Self::Reference);
        }

        #[cfg(not(any(
            all(target_os = "windows", feature = "host-windows", feature = "gpu"),
            feature = "software-reference"
        )))]
        {
            let _ = window_spec;
            Err(GuiError::host(
                "TSX host has no renderer backend; enable host-windows,gpu on Windows",
            ))
        }
    }

    pub(super) fn window_spec(&self) -> &PlatformWindowSpec {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.window_spec(),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.window_spec(),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }

    pub(super) fn render(&mut self, _root: NativeElement) -> GuiResult<SelfDrawnFrameCommit> {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.render(_root),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.render(_root),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }

    pub(super) fn snapshot(&self) -> Option<&SelfDrawnFrameSnapshot> {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.snapshot(),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.snapshot(),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }

    pub(super) fn poll_event(&mut self) -> GuiResult<Option<SelfDrawnHostEventOutcome>> {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.poll_event(),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.poll_event(),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }

    pub(super) fn retry_pending_redraw(&mut self) -> GuiResult<Option<SelfDrawnFrameCommit>> {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.retry_pending_redraw(),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.retry_pending_redraw(),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }

    pub(super) fn next_interaction_deadline_micros(&self) -> Option<u64> {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.next_interaction_deadline_micros(),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.next_interaction_deadline_micros(),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }

    pub(super) fn last_input_timestamp_micros(&self) -> Option<u64> {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.last_input_timestamp_micros(),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.last_input_timestamp_micros(),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }

    pub(super) fn advance_interaction_time(
        &mut self,
        _timestamp_micros: u64,
    ) -> GuiResult<Option<SelfDrawnInputDispatch>> {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.advance_interaction_time(_timestamp_micros),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.advance_interaction_time(_timestamp_micros),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }

    pub(super) fn shutdown(&mut self) -> GuiResult<()> {
        match self {
            #[cfg(all(target_os = "windows", feature = "host-windows", feature = "gpu"))]
            Self::Native(runtime) => runtime.shutdown(),
            #[cfg(all(
                feature = "software-reference",
                not(all(target_os = "windows", feature = "host-windows", feature = "gpu"))
            ))]
            Self::Reference(runtime) => runtime.shutdown(),
            #[cfg(not(any(
                all(target_os = "windows", feature = "host-windows", feature = "gpu"),
                feature = "software-reference"
            )))]
            _ => unreachable!(),
        }
    }
}
