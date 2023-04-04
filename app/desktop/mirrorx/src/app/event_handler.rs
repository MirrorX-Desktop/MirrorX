use crate::{App, Event};
use slint::ComponentHandle;

pub fn register_event_handler(app: &App) {
    on_init_app(app);
    on_close_button_clicked(app);
    on_min_button_clicked(app);
    on_drag_window(app);
}

#[tracing::instrument(skip(app))]
fn on_init_app(app: &App) {
    app.on_init_app(move || {
        //
    });
}

#[tracing::instrument(skip(app))]
fn on_close_button_clicked(app: &App) {
    let app_weak = app.as_weak();
    app.global::<Event>().on_close_button_clicked(move || {
        if let Err(err) = app_weak.upgrade_in_event_loop(move |handle| {
            if let Err(err) = handle.hide() {
                tracing::error!(?err, "window hide failed");
            }
        }) {
            tracing::error!(?err, "upgrade in event loop failed");
        }
    });
}

#[tracing::instrument(skip(app))]
fn on_min_button_clicked(app: &App) {
    let app_weak = app.as_weak();
    app.global::<Event>().on_min_button_clicked(move || {
        if let Err(err) = app_weak.upgrade_in_event_loop(move |_| {
            super::native::windows::set_window_minimize();
        }) {
            tracing::error!(?err, "upgrade in event loop failed");
        }
    });
}

#[tracing::instrument(skip(app))]
fn on_drag_window(app: &App) {
    let app_weak = app.as_weak();
    app.global::<Event>().on_drag_window(move |x, y| {
        if let Err(err) = app_weak.upgrade_in_event_loop(move |handle| {
            let scale_factor = handle.window().scale_factor();
            let mut pos = handle.window().position();
            pos.x += (x * scale_factor) as i32;
            pos.y += (y * scale_factor) as i32;
            handle.window().set_position(pos);
        }) {
            tracing::error!(?err, "upgrade in event loop failed");
        }
    });
}
