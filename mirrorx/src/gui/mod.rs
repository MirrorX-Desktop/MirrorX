mod assets;
mod gpu;
mod pages;
mod themes;
mod widgets;

use crate::gui::pages::desktop::DesktopView;
use fxhash::FxHashMap;
use mirrorx_core::api::signaling::KeyExchangeResponse;
use pages::{create_page, home::HomeView, PageOptions};
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};
use winit::{
    dpi::LogicalSize,
    event::{
        Event::{self, UserEvent},
        WindowEvent,
    },
    event_loop::EventLoopBuilder,
    window::WindowId,
};

const EVENT_SEND_TIMEOUT: Duration = Duration::from_millis(1000 / 60);

pub enum CustomEvent {
    Repaint(WindowId),
    NewDesktopPage {
        remote_device_id: i64,
        key_exchange_resp: KeyExchangeResponse,
    },
}

impl Debug for CustomEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Repaint(window_id) => f.debug_tuple("Repaint").field(window_id).finish(),
            Self::NewDesktopPage {
                remote_device_id, ..
            } => f
                .debug_struct("NewDesktopVisitPage")
                .field("remote_device_id", remote_device_id)
                .finish(),
        }
    }
}

pub fn run_app() -> anyhow::Result<()> {
    let event_loop = EventLoopBuilder::<CustomEvent>::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();
    let mut pages = FxHashMap::default();

    let home_view = HomeView::new(event_loop_proxy.clone());
    let page = create_page(
        "MirrorX",
        &PageOptions {
            size: LogicalSize::new(380, 630),
            resizable: false,
            maximized: false,
            ..Default::default()
        },
        &event_loop,
        Box::new(home_view),
        event_loop_proxy.clone(),
    )
    .unwrap();

    pages.insert(page.window_id(), page);

    let mut next_repaint_time = Instant::now();

    event_loop.run(move |event, window_target, control_flow| {
        match !pages.is_empty() {
            true => control_flow.set_wait(),
            false => control_flow.set_exit(),
        };

        match event {
            Event::WindowEvent {
                event: window_event,
                window_id,
            } => {
                if matches!(
                    window_event,
                    WindowEvent::CloseRequested | WindowEvent::Destroyed
                ) {
                    pages.remove(&window_id);
                }

                // if let WindowEvent::KeyboardInput {
                //     device_id,
                //     input,
                //     is_synthetic,
                // } = window_event
                // {
                //     tracing::info!(?input, "winit input");
                // }

                if let Some(page) = pages.get_mut(&window_id) {
                    page.handle_event(&window_event);

                    match window_event {
                        winit::event::WindowEvent::Resized(size) => {
                            tracing::info!("resize");
                            if size.width > 0 && size.height > 0 {
                                page.resize(size);
                            }
                        }
                        winit::event::WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        } => {
                            tracing::info!("scale factor");
                            page.scale_factor(scale_factor);
                            page.resize(*new_inner_size);
                        }

                        _ => (),
                    }

                    page.request_redraw();
                }
            }
            Event::RedrawRequested(window_id) => {
                if let Some(page) = pages.get_mut(&window_id) {
                    if let Err(err) = page.render() {
                        tracing::error!(?err, ?window_id, "window render failed");
                        control_flow.set_exit();
                    }
                }
            }
            Event::RedrawEventsCleared => {
                let mut min_repaint_after = Duration::MAX;

                pages.iter().for_each(|(_, page)| {
                    min_repaint_after = min_repaint_after.min(page.repaint_after())
                });

                if min_repaint_after.is_zero() {
                    pages.iter().for_each(|(_, page)| page.request_redraw());
                    control_flow.set_poll();
                } else if let Some(wait_until) = Instant::now().checked_add(min_repaint_after) {
                    for (_, page) in pages.iter_mut() {
                        if let Err(err) = page.render() {
                            tracing::error!(?err, "window render failed");
                            control_flow.set_exit();
                            return;
                        }
                    }

                    // tracing::info!("wait next frame");
                    control_flow.set_wait_until(wait_until);
                }
            }
            UserEvent(CustomEvent::Repaint(window_id)) => {
                if let Some(page) = pages.get(&window_id) {
                    page.request_redraw();
                }
            }
            UserEvent(CustomEvent::NewDesktopPage {
                remote_device_id,
                key_exchange_resp,
            }) => {
                tracing::info!("receive build desktop visit page");

                let desktop_view = DesktopView::new(
                    // window.id(),
                    key_exchange_resp.local_device_id,
                    remote_device_id,
                    key_exchange_resp.opening_key_bytes,
                    key_exchange_resp.opening_nonce_bytes,
                    key_exchange_resp.sealing_key_bytes,
                    key_exchange_resp.sealing_nonce_bytes,
                    key_exchange_resp.visit_credentials,
                );

                let page = create_page(
                    &format!("MirrorX Desktop {}", remote_device_id),
                    &PageOptions {
                        size: LogicalSize::new(960, 540),
                        resizable: true,
                        maximized: false,
                        ..Default::default()
                    },
                    window_target,
                    Box::new(desktop_view),
                    event_loop_proxy.clone(),
                )
                .unwrap();

                pages.insert(page.window_id(), page);
            }
            _ => (),
        }
    });
}
