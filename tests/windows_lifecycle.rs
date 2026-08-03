#![cfg(all(target_os = "windows", feature = "host-windows"))]

use std::num::NonZeroIsize;

use a3s_gui::geometry::{Rect, Size};
use a3s_gui::platform_host::{
    PlatformAccessibilitySnapshot, PlatformHost, PlatformHostCommand, PlatformHostEvent,
    PlatformHostRevision, PlatformHostTransaction, PlatformPresentationRequest,
    PlatformWindowCommand, PlatformWindowEvent, PlatformWindowId, PlatformWindowSpec,
    WindowsPlatformHost,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    IsIconic, SetWindowPos, ShowWindow, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOZORDER, SW_MINIMIZE,
    SW_RESTORE,
};

const WINDOW: PlatformWindowId = PlatformWindowId::new(73);

fn window_spec(title: &str) -> PlatformWindowSpec {
    PlatformWindowSpec {
        id: WINDOW,
        title: title.to_string(),
        logical_size: Size::new(320.0, 240.0),
        min_size: Some(Size::new(160.0, 120.0)),
        max_size: Some(Size::new(1200.0, 900.0)),
        resizable: true,
        visible: true,
    }
}

fn open_transaction() -> PlatformHostTransaction {
    let spec = window_spec("A3S Win32 lifecycle recovery");
    PlatformHostTransaction {
        revision: PlatformHostRevision::new(1),
        commands: vec![
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::Open { spec: spec.clone() },
            },
            PlatformHostCommand::Accessibility {
                snapshot: Box::new(PlatformAccessibilitySnapshot {
                    window: WINDOW,
                    root: None,
                }),
            },
            PlatformHostCommand::Present {
                request: PlatformPresentationRequest {
                    window: WINDOW,
                    logical_size: spec.logical_size,
                    scale_factor: 1.0,
                    scene_fingerprint: 17,
                    damage: vec![Rect::new(0.0, 0.0, 320.0, 240.0)],
                },
            },
        ],
    }
}

fn native_hwnd(host: &WindowsPlatformHost) -> NonZeroIsize {
    let surface = host.surface(WINDOW).unwrap();
    match surface.window_handle().unwrap().as_raw() {
        RawWindowHandle::Win32(handle) => handle.hwnd,
        other => panic!("expected Win32 surface handle, got {other:?}"),
    }
}

fn drain_native_events(host: &mut WindowsPlatformHost) {
    while host.poll_event().unwrap().is_some() {}
}

#[test]
fn real_win32_restore_reports_geometry_before_exposure() {
    let mut host = WindowsPlatformHost::new().unwrap();
    host.prepare(open_transaction()).unwrap();
    host.commit().unwrap();
    let hwnd = native_hwnd(&host);
    drain_native_events(&mut host);

    unsafe {
        ShowWindow(hwnd.get() as _, SW_MINIMIZE);
    }
    let mut minimized = Vec::new();
    for _ in 0..64 {
        let Some(event) = host.poll_event().unwrap() else {
            continue;
        };
        let done = matches!(
            event,
            PlatformHostEvent::Window {
                event: PlatformWindowEvent::OcclusionChanged {
                    window: WINDOW,
                    occluded: true,
                }
            }
        );
        minimized.push(event);
        if done {
            break;
        }
    }
    assert_ne!(unsafe { IsIconic(hwnd.get() as _) }, 0);
    assert!(minimized.iter().any(|event| matches!(
        event,
        PlatformHostEvent::Window {
            event: PlatformWindowEvent::OcclusionChanged {
                window: WINDOW,
                occluded: true,
            }
        }
    )));
    assert!(!minimized.iter().any(|event| matches!(
        event,
        PlatformHostEvent::Window {
            event: PlatformWindowEvent::Resized { .. }
        }
    )));

    unsafe {
        ShowWindow(hwnd.get() as _, SW_RESTORE);
    }
    let mut restored = Vec::new();
    for _ in 0..64 {
        if let Some(event) = host.poll_event().unwrap() {
            restored.push(event);
        }
        let resized = restored.iter().any(|event| {
            matches!(
                event,
                PlatformHostEvent::Window {
                    event: PlatformWindowEvent::Resized { window: WINDOW, .. }
                }
            )
        });
        let exposed = restored.iter().any(|event| {
            matches!(
                event,
                PlatformHostEvent::Window {
                    event: PlatformWindowEvent::OcclusionChanged {
                        window: WINDOW,
                        occluded: false,
                    }
                }
            )
        });
        if resized && exposed {
            break;
        }
    }
    let resized = restored
        .iter()
        .position(|event| {
            matches!(
                event,
                PlatformHostEvent::Window {
                    event: PlatformWindowEvent::Resized {
                        window: WINDOW,
                        logical_size,
                    }
                } if logical_size.width > 0.0 && logical_size.height > 0.0
            )
        })
        .expect("restore should report a nonzero client size");
    let exposed = restored
        .iter()
        .position(|event| {
            matches!(
                event,
                PlatformHostEvent::Window {
                    event: PlatformWindowEvent::OcclusionChanged {
                        window: WINDOW,
                        occluded: false,
                    }
                }
            )
        })
        .expect("restore should expose the window");
    assert!(resized < exposed, "restore events were {restored:?}");
    assert_eq!(unsafe { IsIconic(hwnd.get() as _) }, 0);

    host.shutdown().unwrap();
}

#[cfg(feature = "platform-runtime")]
fn runtime_root(color: &str) -> a3s_gui::native::NativeElement {
    use a3s_gui::native::{NativeElement, NativeProps, NativeRole};
    use a3s_gui::web::WebProps;

    NativeElement::new("root", NativeRole::View).with_props(
        NativeProps::new().web(WebProps::new().class_name(format!("h-[240px] w-[320px] {color}"))),
    )
}

#[cfg(feature = "platform-runtime")]
#[test]
fn shared_runtime_rebuilds_hidden_state_before_restore_redraw() {
    use a3s_gui::platform_runtime::{
        RecordingScenePresenter, SelfDrawnHostEventOutcome, SelfDrawnWindowRuntime,
    };

    let host = WindowsPlatformHost::new().unwrap();
    let scale_factor = host.initial_scale_factor().unwrap();
    let mut runtime = SelfDrawnWindowRuntime::new(
        host,
        RecordingScenePresenter::new(),
        window_spec("A3S runtime restore ordering"),
        scale_factor,
    )
    .unwrap();
    runtime.render(runtime_root("bg-white")).unwrap();
    let hwnd = native_hwnd(runtime.host());
    drain_native_events(runtime.host_mut());

    unsafe {
        ShowWindow(hwnd.get() as _, SW_MINIMIZE);
    }
    for _ in 0..64 {
        let _ = runtime.poll_event().unwrap();
        if runtime.is_occluded() {
            break;
        }
    }
    assert!(runtime.is_occluded());
    let publishes = runtime.presenter().publish_count();
    let hidden = runtime.render(runtime_root("bg-black")).unwrap();
    assert!(!hidden.presentation_requested);
    assert!(runtime.pending_redraw());
    assert_eq!(runtime.presenter().publish_count(), publishes);

    unsafe {
        ShowWindow(hwnd.get() as _, SW_RESTORE);
    }
    let mut restore_frames = Vec::new();
    for _ in 0..64 {
        if let Some(SelfDrawnHostEventOutcome::Frame(frame)) = runtime.poll_event().unwrap() {
            restore_frames.push(frame);
            if restore_frames.len() == 2 {
                break;
            }
        }
    }
    assert_eq!(restore_frames.len(), 2);
    assert!(!restore_frames[0].presentation_requested);
    assert!(restore_frames[1].presentation_requested);
    assert!(!runtime.is_occluded());
    assert!(!runtime.pending_redraw());
    assert_eq!(runtime.presenter().publish_count(), publishes + 1);

    runtime.shutdown().unwrap();
}

#[cfg(feature = "platform-runtime")]
#[test]
fn shared_runtime_accepts_native_resize_before_presenting_new_geometry() {
    use std::ptr::null_mut;

    use a3s_gui::platform_runtime::{
        RecordingScenePresenter, SelfDrawnHostEventOutcome, SelfDrawnWindowRuntime,
    };

    let host = WindowsPlatformHost::new().unwrap();
    let scale_factor = host.initial_scale_factor().unwrap();
    let mut runtime = SelfDrawnWindowRuntime::new(
        host,
        RecordingScenePresenter::new(),
        window_spec("A3S native resize reconciliation"),
        scale_factor,
    )
    .unwrap();
    runtime.render(runtime_root("bg-black")).unwrap();
    let hwnd = native_hwnd(runtime.host());
    drain_native_events(runtime.host_mut());
    let publishes = runtime.presenter().publish_count();

    assert_ne!(
        unsafe {
            SetWindowPos(
                hwnd.get() as _,
                null_mut(),
                0,
                0,
                520,
                400,
                SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
            )
        },
        0
    );
    let mut resized = None;
    for _ in 0..64 {
        if let Some(SelfDrawnHostEventOutcome::Frame(frame)) = runtime.poll_event().unwrap() {
            resized = Some(frame);
            break;
        }
    }
    let resized = resized.expect("native resize should commit a replacement frame");
    assert!(resized.presentation_requested);
    assert_ne!(runtime.window_spec().logical_size, Size::new(320.0, 240.0));
    assert_eq!(
        runtime.host().window_spec(WINDOW).unwrap().logical_size,
        runtime.window_spec().logical_size
    );
    assert_eq!(runtime.presenter().publish_count(), publishes + 1);

    runtime.shutdown().unwrap();
}

#[cfg(all(feature = "platform-runtime", feature = "gpu-fault-injection"))]
#[test]
fn dx12_device_loss_defers_then_recreates_the_presenter() {
    use a3s_gui::drawing::{GpuBackend, GpuPowerPreference, GpuRendererOptions};
    use a3s_gui::platform_host::PlatformPresentationStatus;
    use a3s_gui::platform_runtime::{
        GpuScenePresenter, SelfDrawnFrameCommitStatus, SelfDrawnWindowRuntime,
    };

    let host = WindowsPlatformHost::new().unwrap();
    let scale_factor = host.initial_scale_factor().unwrap();
    let presenter = GpuScenePresenter::with_options(GpuRendererOptions {
        power_preference: GpuPowerPreference::None,
        allow_software_adapter: true,
        ..GpuRendererOptions::default()
    });
    let mut runtime = SelfDrawnWindowRuntime::new(
        host,
        presenter,
        window_spec("A3S DX12 device-loss recovery"),
        scale_factor,
    )
    .unwrap();
    let first = runtime.render(runtime_root("bg-black")).unwrap();
    assert_eq!(
        runtime.presenter().capabilities().unwrap().backend,
        GpuBackend::Direct3d12
    );

    runtime.presenter_mut().inject_device_loss().unwrap();
    let lost = runtime.redraw().unwrap();
    assert_eq!(lost.status, SelfDrawnFrameCommitStatus::Deferred);
    assert_eq!(lost.revision, first.revision);
    assert_eq!(
        lost.presentation_status,
        Some(PlatformPresentationStatus::SurfaceLost)
    );
    assert!(runtime.pending_redraw());
    assert_eq!(runtime.presenter().surface_loss_count(), 1);
    assert_eq!(runtime.presenter().deferred_count(), 1);
    assert!(runtime
        .presenter()
        .last_failure()
        .is_some_and(|message| message.contains("GPU device lost")));
    assert!(runtime.presenter().capabilities().is_none());
    assert_eq!(runtime.stats().surface_recoveries, 1);

    let recovered = runtime.retry_pending_redraw().unwrap().unwrap();
    assert_eq!(recovered.status, SelfDrawnFrameCommitStatus::Committed);
    assert_eq!(
        recovered.presentation_status,
        Some(PlatformPresentationStatus::Presented)
    );
    assert_eq!(recovered.revision.get(), first.revision.get() + 1);
    assert!(!runtime.pending_redraw());
    assert!(runtime.presenter().last_failure().is_none());
    assert_eq!(
        runtime.presenter().capabilities().unwrap().backend,
        GpuBackend::Direct3d12
    );

    runtime.shutdown().unwrap();
}
