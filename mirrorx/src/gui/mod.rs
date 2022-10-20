mod gpu;
mod pages;
mod state;
mod widgets;

use fxhash::FxHashMap;
use pages::{Page, PageOptions};
use winit::{
    dpi::LogicalSize,
    event::Event::{self, UserEvent},
    event_loop::EventLoopBuilder,
    platform::run_return::EventLoopExtRunReturn,
    window::WindowId,
};

#[derive(Debug)]
pub enum CustomEvent {
    Repaint(WindowId),
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
    let mut pages = FxHashMap::default();

    let home_view = pages::home::HomeView::new();
    let home_page = Page::new(
        "MirrorX",
        PageOptions {
            size: LogicalSize::new(380, 630),
            resizable: false,
            maximized: false,
            ..Default::default()
        },
        &event_loop,
        Box::new(home_view),
    )?;

    pages.insert(home_page.window_id(), home_page);

    event_loop.run_return(move |event, _, control_flow| {
        tracing::info!(?event, "event loop");
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
                            if size.width > 0 && size.height > 0 {
                                page.resize(size);
                            }
                        }
                        winit::event::WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        } => {
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
                    tracing::info!("redraw");
                    if let Err(err) = page.render() {
                        tracing::error!(?err, "page render failed");
                        control_flow.set_exit();
                    }
                }
            }
            Event::RedrawEventsCleared => {
                // todo: consider about multiple windows
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
            UserEvent(CustomEvent::Repaint(window_id)) => {
                tracing::info!("receive repaint");
                if let Some(page) = pages.get_mut(&window_id) {
                    page.request_redraw();
                }
            }
            _ => (),
        }
    });

    Ok(())
}
