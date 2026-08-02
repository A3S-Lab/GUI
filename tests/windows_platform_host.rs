#![cfg(all(target_os = "windows", feature = "host-windows"))]

use std::num::NonZeroIsize;

use a3s_gui::geometry::{Rect, Size};
use a3s_gui::platform_host::{
    PlatformAccessibilitySnapshot, PlatformHost, PlatformHostCommand, PlatformHostEvent,
    PlatformHostRevision, PlatformHostTransaction, PlatformPresentationRequest,
    PlatformPresentationStatus, PlatformWindowCommand, PlatformWindowEvent, PlatformWindowId,
    PlatformWindowSpec, WindowsPlatformHost,
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetWindowTextW, IsWindow, IsWindowVisible, PostMessageW, WM_CLOSE,
};

fn window_id() -> PlatformWindowId {
    PlatformWindowId::new(41)
}

fn revision(value: u64) -> PlatformHostRevision {
    PlatformHostRevision::new(value)
}

fn window_spec(title: &str, size: Size) -> PlatformWindowSpec {
    PlatformWindowSpec {
        id: window_id(),
        title: title.to_string(),
        logical_size: size,
        min_size: Some(Size::new(160.0, 120.0)),
        max_size: Some(Size::new(1200.0, 900.0)),
        resizable: true,
        visible: false,
    }
}

fn open_transaction(value: u64) -> PlatformHostTransaction {
    let spec = window_spec("A3S hidden smoke", Size::new(320.0, 240.0));
    PlatformHostTransaction {
        revision: revision(value),
        commands: vec![
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::Open { spec: spec.clone() },
            },
            PlatformHostCommand::Accessibility {
                snapshot: Box::new(PlatformAccessibilitySnapshot {
                    window: spec.id,
                    root: None,
                }),
            },
            PlatformHostCommand::Present {
                request: PlatformPresentationRequest {
                    window: spec.id,
                    logical_size: spec.logical_size,
                    scale_factor: 1.0,
                    scene_fingerprint: 7,
                    damage: vec![Rect::new(0.0, 0.0, 320.0, 240.0)],
                },
            },
        ],
    }
}

fn native_hwnd(host: &WindowsPlatformHost) -> NonZeroIsize {
    let surface = host.surface(window_id()).unwrap();
    assert!(matches!(
        surface.display_handle().unwrap().as_raw(),
        RawDisplayHandle::Windows(_)
    ));
    match surface.window_handle().unwrap().as_raw() {
        RawWindowHandle::Win32(handle) => handle.hwnd,
        other => panic!("expected Win32 surface handle, got {other:?}"),
    }
}

fn native_title(hwnd: NonZeroIsize) -> String {
    let mut buffer = vec![0_u16; 256];
    let length = unsafe { GetWindowTextW(hwnd.get() as _, buffer.as_mut_ptr(), 256) };
    assert!(length >= 0);
    String::from_utf16(&buffer[..length as usize]).unwrap()
}

fn drain_events(host: &mut WindowsPlatformHost) {
    while host.poll_event().unwrap().is_some() {}
}

#[test]
fn real_win32_host_commits_hidden_window_surface_and_presentation() {
    let mut host = WindowsPlatformHost::new().unwrap();

    host.prepare(open_transaction(1)).unwrap();
    assert_eq!(host.window_count(), 0);
    assert_eq!(host.staged_window_count(), 1);
    let staged = host.presentation_target(window_id()).unwrap();
    let staged_hwnd = match staged.window_handle().unwrap().as_raw() {
        RawWindowHandle::Win32(handle) => handle.hwnd,
        other => panic!("expected staged Win32 surface handle, got {other:?}"),
    };
    assert_ne!(unsafe { IsWindow(staged_hwnd.get() as _) }, 0);
    assert_eq!(unsafe { IsWindowVisible(staged_hwnd.get() as _) }, 0);
    drop(staged);
    let ack = host.commit().unwrap();

    assert_eq!(ack.revision, revision(1));
    assert_eq!(ack.applied_commands, 3);
    assert_eq!(ack.presentations.len(), 1);
    assert_eq!(
        ack.presentations[0].status,
        PlatformPresentationStatus::Queued
    );
    assert_eq!(host.window_count(), 1);
    assert_eq!(host.staged_window_count(), 0);
    assert_eq!(host.last_committed_revision(), Some(revision(1)));
    assert_eq!(
        host.accessibility_snapshot(window_id()).unwrap().window,
        window_id()
    );
    let hwnd = native_hwnd(&host);
    assert_ne!(unsafe { IsWindow(hwnd.get() as _) }, 0);
    assert_eq!(native_title(hwnd), "A3S hidden smoke");
    let surface = host.surface(window_id()).unwrap();
    assert_eq!(surface.window(), window_id());
    assert!(surface.scale_factor() > 0.0);
    assert!(surface.physical_size().0 > 0);
    assert!(surface.physical_size().1 > 0);
    drop(surface);

    host.shutdown().unwrap();
    assert_eq!(unsafe { IsWindow(hwnd.get() as _) }, 0);
}

#[test]
fn staged_surface_lease_blocks_rollback_until_the_presenter_releases_it() {
    let mut host = WindowsPlatformHost::new().unwrap();
    host.prepare(open_transaction(1)).unwrap();
    let surface = host.presentation_target(window_id()).unwrap();
    let hwnd = match surface.window_handle().unwrap().as_raw() {
        RawWindowHandle::Win32(handle) => handle.hwnd,
        other => panic!("expected Win32 surface handle, got {other:?}"),
    };

    let error = host.rollback().unwrap_err();
    assert!(error.to_string().contains("active Graphics surface lease"));
    assert_ne!(unsafe { IsWindow(hwnd.get() as _) }, 0);
    assert_eq!(host.staged_window_count(), 1);

    drop(surface);
    host.rollback().unwrap();
    assert_eq!(unsafe { IsWindow(hwnd.get() as _) }, 0);
    assert_eq!(host.staged_window_count(), 0);
    host.shutdown().unwrap();
}

#[test]
fn updates_rollback_without_native_mutation_and_close_is_explicit() {
    let mut host = WindowsPlatformHost::new().unwrap();
    host.prepare(open_transaction(1)).unwrap();
    host.commit().unwrap();
    drain_events(&mut host);
    let hwnd = native_hwnd(&host);

    host.prepare(PlatformHostTransaction {
        revision: revision(2),
        commands: vec![
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::SetTitle {
                    window: window_id(),
                    title: "not committed".to_string(),
                },
            },
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::Resize {
                    window: window_id(),
                    logical_size: Size::new(480.0, 320.0),
                },
            },
        ],
    })
    .unwrap();
    assert_eq!(native_title(hwnd), "A3S hidden smoke");
    host.rollback().unwrap();
    assert_eq!(native_title(hwnd), "A3S hidden smoke");

    host.prepare(PlatformHostTransaction {
        revision: revision(2),
        commands: vec![
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::SetTitle {
                    window: window_id(),
                    title: "committed title".to_string(),
                },
            },
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::Resize {
                    window: window_id(),
                    logical_size: Size::new(480.0, 320.0),
                },
            },
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::RequestRedraw {
                    window: window_id(),
                },
            },
            PlatformHostCommand::Window {
                command: PlatformWindowCommand::SetVisible {
                    window: window_id(),
                    visible: true,
                },
            },
        ],
    })
    .unwrap();
    host.commit().unwrap();
    assert_eq!(native_title(hwnd), "committed title");
    assert_ne!(unsafe { IsWindowVisible(hwnd.get() as _) }, 0);
    assert_eq!(
        host.window_spec(window_id()).unwrap().logical_size,
        Size::new(480.0, 320.0)
    );
    let surface = host.surface(window_id()).unwrap();
    let logical_width = f64::from(surface.physical_size().0) / surface.scale_factor();
    let logical_height = f64::from(surface.physical_size().1) / surface.scale_factor();
    assert!((logical_width - 480.0).abs() <= 1.0);
    assert!((logical_height - 320.0).abs() <= 1.0);
    drop(surface);

    drain_events(&mut host);
    assert_ne!(unsafe { PostMessageW(hwnd.get() as _, WM_CLOSE, 0, 0) }, 0);
    let close_requested = (0..16).find_map(|_| match host.poll_event().unwrap() {
        Some(PlatformHostEvent::Window {
            event: PlatformWindowEvent::CloseRequested { window },
        }) => Some(window),
        _ => None,
    });
    assert_eq!(close_requested, Some(window_id()));
    assert_ne!(unsafe { IsWindow(hwnd.get() as _) }, 0);

    host.prepare(PlatformHostTransaction {
        revision: revision(3),
        commands: vec![PlatformHostCommand::Window {
            command: PlatformWindowCommand::Close {
                window: window_id(),
            },
        }],
    })
    .unwrap();
    host.commit().unwrap();
    assert_eq!(host.window_count(), 0);
    assert_eq!(unsafe { IsWindow(hwnd.get() as _) }, 0);
    host.shutdown().unwrap();
}

#[test]
fn native_event_queue_is_bounded_and_zero_capacity_is_rejected() {
    assert!(WindowsPlatformHost::with_event_queue_limit(0).is_err());
    let mut host = WindowsPlatformHost::with_event_queue_limit(1).unwrap();
    host.prepare(open_transaction(1)).unwrap();
    host.commit().unwrap();
    drain_events(&mut host);
    let hwnd = native_hwnd(&host);

    assert_ne!(unsafe { PostMessageW(hwnd.get() as _, WM_CLOSE, 0, 0) }, 0);
    assert_ne!(unsafe { PostMessageW(hwnd.get() as _, WM_CLOSE, 0, 0) }, 0);
    let error = host.poll_event().unwrap_err();

    assert!(error.to_string().contains("1-event limit"));
    host.shutdown().unwrap();
}

#[test]
fn unsupported_service_commands_fail_during_prepare_without_partial_state() {
    use a3s_gui::platform_host::{PlatformTextInputSessionId, PlatformTextInputUpdate};

    let mut host = WindowsPlatformHost::new().unwrap();
    let error = host
        .prepare(PlatformHostTransaction {
            revision: revision(1),
            commands: vec![PlatformHostCommand::TextInput {
                update: PlatformTextInputUpdate::Deactivate {
                    session: PlatformTextInputSessionId::new(1),
                },
            }],
        })
        .unwrap_err();

    assert!(error.to_string().contains("TSF"));
    assert_eq!(host.window_count(), 0);
    assert!(host.rollback().is_ok());
    host.shutdown().unwrap();
}

#[cfg(feature = "platform-runtime")]
#[test]
fn shared_self_drawn_runtime_commits_into_the_real_hidden_win32_host() {
    use a3s_gui::native::{NativeElement, NativeProps, NativeRole};
    use a3s_gui::platform_runtime::{
        RecordingScenePresenter, SelfDrawnFrameCommitStatus, SelfDrawnWindowRuntime,
    };
    use a3s_gui::web::WebProps;

    let host = WindowsPlatformHost::new().unwrap();
    let presenter = RecordingScenePresenter::new();
    let spec = window_spec("A3S H1 Win32 integration", Size::new(320.0, 240.0));
    let mut runtime = SelfDrawnWindowRuntime::new(host, presenter, spec, 1.0).unwrap();
    let root = NativeElement::new("root", NativeRole::View).with_props(
        NativeProps::new().web(WebProps::new().class_name("relative h-[240px] w-[320px] bg-black")),
    );

    let commit = runtime.render(root).unwrap();

    assert_eq!(commit.status, SelfDrawnFrameCommitStatus::Committed);
    assert!(commit.presentation_requested);
    assert_eq!(runtime.host().window_count(), 1);
    let hwnd = native_hwnd(runtime.host());
    assert_ne!(unsafe { IsWindow(hwnd.get() as _) }, 0);
    runtime.shutdown().unwrap();
    assert_eq!(unsafe { IsWindow(hwnd.get() as _) }, 0);
}

#[cfg(all(feature = "platform-runtime", feature = "gpu"))]
#[test]
fn graphics_presenter_draws_and_presents_the_first_real_win32_frame() {
    use a3s_gui::drawing::{GpuBackend, GpuPowerPreference, GpuRendererOptions};
    use a3s_gui::native::{NativeElement, NativeProps, NativeRole};
    use a3s_gui::platform_runtime::{
        GpuScenePresenter, SelfDrawnFrameCommitStatus, SelfDrawnWindowRuntime,
    };
    use a3s_gui::web::WebProps;

    let host = WindowsPlatformHost::new().unwrap();
    let scale_factor = host.initial_scale_factor().unwrap();
    let presenter = GpuScenePresenter::with_options(GpuRendererOptions {
        power_preference: GpuPowerPreference::None,
        allow_software_adapter: true,
        ..GpuRendererOptions::default()
    });
    let mut spec = window_spec("A3S Graphics DX12 presentation", Size::new(320.0, 240.0));
    spec.visible = true;
    let mut runtime = SelfDrawnWindowRuntime::new(host, presenter, spec, scale_factor).unwrap();
    let root = NativeElement::new("root", NativeRole::View).with_props(
        NativeProps::new().web(WebProps::new().class_name("h-[240px] w-[320px] bg-black")),
    );

    let commit = runtime.render(root).unwrap();

    assert_eq!(commit.status, SelfDrawnFrameCommitStatus::Committed);
    assert_eq!(
        commit.presentation_status,
        Some(PlatformPresentationStatus::Presented)
    );
    let capabilities = runtime.presenter().capabilities().unwrap();
    assert_eq!(capabilities.backend, GpuBackend::Direct3d12);
    let presented = runtime.presenter().committed().unwrap();
    assert!(presented.gpu().presented);
    assert_eq!(
        presented.gpu().fingerprint,
        presented.frame().scene_fingerprint
    );
    let hwnd = native_hwnd(runtime.host());
    assert_ne!(unsafe { IsWindow(hwnd.get() as _) }, 0);
    assert_ne!(unsafe { IsWindowVisible(hwnd.get() as _) }, 0);

    runtime.shutdown().unwrap();
    assert_eq!(unsafe { IsWindow(hwnd.get() as _) }, 0);
}
