mod gpu;
mod pages;
mod widgets;

use fxhash::FxHashMap;
use mirrorx_core::api::signaling::KeyExchangeResponse;
use pages::{Page, PageOptions};
use std::fmt::Debug;
use winit::{
    dpi::LogicalSize,
    event::Event::{self, UserEvent},
    event_loop::EventLoopBuilder,
    platform::run_return::EventLoopExtRunReturn,
    window::WindowId,
};

use crate::gui::pages::desktop::DesktopView;

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
    // let native_options = eframe::NativeOptions {
    //     always_on_top: true,
    //     maximized: false,
    //     initial_window_size: Some(eframe::epaint::Vec2::new(380f32, 630f32)),
    //     resizable: false,
    //     follow_system_theme: false,
    //     default_theme: eframe::Theme::Light,
    //     // centered: true,
    //     // fullsize_content: true,
    //     ..Default::default()
    // };

    // eframe::run_native(
    //     "MirrorX",
    //     native_options,
    //     Box::new(|cc| Box::new(app::App::new(cc))),
    // );

    let mut event_loop = EventLoopBuilder::<CustomEvent>::with_user_event().build();
    let event_loop_proxy = event_loop.create_proxy();
    let mut pages = FxHashMap::default();

    let home_view = pages::home::HomeView::new(event_loop.create_proxy());
    let home_page = Page::new(
        "MirrorX",
        PageOptions {
            size: LogicalSize::new(380, 630),
            resizable: false,
            maximized: false,
            ..Default::default()
        },
        &event_loop,
        event_loop_proxy.clone(),
        Box::new(home_view),
    )?;

    pages.insert(home_page.window_id(), home_page);

    event_loop.run_return(move |event, window_target, control_flow| {
        // tracing::info!(?event, "event loop");
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, window_id } => {
                let mut removed = false;
                if let Some(page) = pages.get_mut(&window_id) {
                    page.handle_event(&event);

                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            println!("receive close request");
                            removed = true;
                        }
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

                if removed {
                    pages.remove(&window_id);
                }

                if pages.is_empty() {
                    control_flow.set_exit();
                }
            }
            Event::RedrawRequested(window_id) => {
                if let Some(page) = pages.get_mut(&window_id) {
                    if let Err(err) = page.render() {
                        tracing::error!(?err, "page render failed");
                        control_flow.set_exit();
                    }
                }
            }
            Event::RedrawEventsCleared => {
                // todo: consider about multiple windows
                if pages.len() == 1 {
                    for (_, page) in pages.values().enumerate() {
                        if let Some(next_repaint_instant) = page.next_repaint_instant() {
                            let now = std::time::Instant::now();
                            match next_repaint_instant
                                .checked_duration_since(now)
                                .map(|duration| now + duration)
                            {
                                Some(wait_instant) => control_flow.set_wait_until(wait_instant),
                                None => {
                                    page.request_redraw();
                                    control_flow.set_poll();
                                }
                            }
                        } else {
                            control_flow.set_wait();
                        }
                    }
                }
            }
            UserEvent(CustomEvent::Repaint(window_id)) => {
                if let Some(page) = pages.get_mut(&window_id) {
                    page.request_redraw();
                }
            }
            UserEvent(CustomEvent::NewDesktopPage {
                remote_device_id,
                key_exchange_resp,
            }) => {
                tracing::info!("receive build desktop visit page");
                let desktop_view = DesktopView::new(
                    key_exchange_resp.local_device_id,
                    remote_device_id,
                    key_exchange_resp.opening_key_bytes,
                    key_exchange_resp.opening_nonce_bytes,
                    key_exchange_resp.sealing_key_bytes,
                    key_exchange_resp.sealing_nonce_bytes,
                    key_exchange_resp.visit_credentials,
                );

                let desktop_page = Page::new(
                    format!("MirrorX Desktop {}", remote_device_id).as_str(),
                    PageOptions {
                        size: LogicalSize::new(960, 540),
                        resizable: true,
                        maximized: false,
                        ..Default::default()
                    },
                    window_target,
                    event_loop_proxy.clone(),
                    Box::new(desktop_view),
                )
                .unwrap();

                pages.insert(desktop_page.window_id(), desktop_page);
            }
            _ => (),
        }
    });

    Ok(())
}
