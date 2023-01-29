mod render;
mod state;

use self::render::Render;
use egui_extras::RetainedImage;
use mirrorx_core::{
    api::endpoint::{
        client::EndPointClient,
        id::EndPointID,
        message::{EndPointInput, EndPointMessage, InputEvent, KeyboardEvent, MouseEvent},
    },
    component::input::key::MouseKey,
    DesktopDecodeFrame,
};
use state::State;
use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};
use tauri_egui::{
    eframe::{
        egui_glow::CallbackFn,
        glow::{self, Context},
    },
    egui::{
        epaint::Shadow, style::Margin, Align, CentralPanel, Color32, FontId, Frame, Layout, Pos2,
        Rect, RichText, Rounding, Sense, Stroke, Ui, Vec2,
    },
};

static ICON_MAXIMIZE_BYTES:&[u8]=br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--! Font Awesome Pro 6.2.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. --><path style="fill:rgb(255,255,255)" d="M168 32H24C10.7 32 0 42.7 0 56V200c0 9.7 5.8 18.5 14.8 22.2s19.3 1.7 26.2-5.2l40-40 79 79L81 335 41 295c-6.9-6.9-17.2-8.9-26.2-5.2S0 302.3 0 312V456c0 13.3 10.7 24 24 24H168c9.7 0 18.5-5.8 22.2-14.8s1.7-19.3-5.2-26.2l-40-40 79-79 79 79-40 40c-6.9 6.9-8.9 17.2-5.2 26.2s12.5 14.8 22.2 14.8H424c13.3 0 24-10.7 24-24V312c0-9.7-5.8-18.5-14.8-22.2s-19.3-1.7-26.2 5.2l-40 40-79-79 79-79 40 40c6.9 6.9 17.2 8.9 26.2 5.2s14.8-12.5 14.8-22.2V56c0-13.3-10.7-24-24-24H280c-9.7 0-18.5 5.8-22.2 14.8s-1.7 19.3 5.2 26.2l40 40-79 79-79-79 40-40c6.9-6.9 8.9-17.2 5.2-26.2S177.7 32 168 32z"/></svg>"#;
static ICON_SCALE_BYTES:&[u8]=br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><!--! Font Awesome Pro 6.2.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license (Commercial License) Copyright 2022 Fonticons, Inc. --><path style="fill:rgb(255,255,255)" d="M32 64c17.7 0 32 14.3 32 32l0 320c0 17.7-14.3 32-32 32s-32-14.3-32-32V96C0 78.3 14.3 64 32 64zm214.6 73.4c12.5 12.5 12.5 32.8 0 45.3L205.3 224l229.5 0-41.4-41.4c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l96 96c12.5 12.5 12.5 32.8 0 45.3l-96 96c-12.5 12.5-32.8 12.5-45.3 0s-12.5-32.8 0-45.3L434.7 288l-229.5 0 41.4 41.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0l-96-96c-12.5-12.5-12.5-32.8 0-45.3l96-96c12.5-12.5 32.8-12.5 45.3 0zM640 96V416c0 17.7-14.3 32-32 32s-32-14.3-32-32V96c0-17.7 14.3-32 32-32s32 14.3 32 32z"/></svg>"#;

pub struct DesktopWindow {
    state: State,
    icon_maximize: RetainedImage,
    icon_scale: RetainedImage,
    render: Arc<RwLock<Render>>,
    render_call_back: Arc<CallbackFn>,
    last_show_cursor: bool,
    current_show_cursor: bool,
}

impl DesktopWindow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        endpoint_id: EndPointID,
        gl_context: Arc<Context>,
        client: Arc<EndPointClient>,
        render_frame_rx: tokio::sync::mpsc::Receiver<DesktopDecodeFrame>,
    ) -> Self {
        let frame_slot = Arc::new(Mutex::new(DesktopDecodeFrame::default()));

        let state = State::new(endpoint_id, client, render_frame_rx, frame_slot.clone());

        let desktop_render = Arc::new(RwLock::new(
            Render::new(gl_context.as_ref()).expect("create desktop render failed"),
        ));

        let desktop_render_clone = desktop_render.clone();

        let cb = CallbackFn::new(move |_info, painter| {
            let mut render = desktop_render_clone.write().unwrap();
            let frame = frame_slot.lock().unwrap();

            if let Err(err) = render.paint(painter.gl(), &frame, painter.intermediate_fbo()) {
                tracing::error!(?err, "desktop render failed");
            }
        });

        Self {
            state,
            icon_maximize: RetainedImage::from_color_image(
                "fa_maximize",
                egui_extras::image::load_svg_bytes(ICON_MAXIMIZE_BYTES).unwrap(),
            ),
            icon_scale: RetainedImage::from_color_image(
                "fa_arrows-left-right-to-line",
                egui_extras::image::load_svg_bytes(ICON_SCALE_BYTES).unwrap(),
            ),
            render: desktop_render,
            render_call_back: Arc::new(cb),
            last_show_cursor: true,
            current_show_cursor: true,
        }
    }

    fn build_panel(&mut self, ui: &mut Ui) {
        // match self.state.visit_state() {
        //     state::VisitState::Connecting => {
        //         ui.centered_and_justified(|ui| {
        //             let (rect, response) = ui.allocate_exact_size(
        //                 Vec2::new(160.0, 80.0),
        //                 Sense::focusable_noninteractive(),
        //             );

        //             ui.allocate_ui_at_rect(rect, |ui| {
        //                 ui.spinner();
        //                 ui.label("connecting");
        //             });

        //             response
        //         });
        //     }
        //     state::VisitState::Negotiating => {
        //         ui.centered_and_justified(|ui| {
        //             let (rect, response) = ui.allocate_exact_size(
        //                 Vec2::new(160.0, 80.0),
        //                 Sense::focusable_noninteractive(),
        //             );

        //             ui.allocate_ui_at_rect(rect, |ui| {
        //                 ui.spinner();
        //                 ui.label("negotiating");
        //             });

        //             response
        //         });
        //     }
        //     state::VisitState::Serving => {
        self.build_desktop_texture(ui);
        self.build_toolbar(ui);
        //     }
        //     state::VisitState::ErrorOccurred => {
        //         ui.centered_and_justified(|ui| {
        //             ui.label(
        //                 self.state
        //                     .last_error()
        //                     .map(|err| err.to_string())
        //                     .unwrap_or_else(|| String::from("An unknown error occurred")),
        //             );
        //         });
        //     }
        // }
    }

    fn build_desktop_texture(&mut self, ui: &mut Ui) {
        let (frame_width, frame_height) = self.state.update_desktop_frame();

        if frame_width > 0 && frame_height > 0 {
            // when client area bigger than original desktop frame, disable scale button
            self.state.set_desktop_frame_scalable(
                ui.available_width() < frame_width as _
                    || ui.available_height() < frame_height as _,
            );

            if self.state.desktop_frame_scaled()
                && (ui.available_width() < frame_width as _
                    || ui.available_height() < frame_height as _)
            {
                let left = ((ui.available_width() - frame_width as f32) / 2.0).max(0.0);
                let top = ((ui.available_height() - frame_height as f32) / 2.0).max(0.0);

                let mut available_rect = ui.available_rect_before_wrap();
                available_rect.min = Pos2::new(left, top);

                ui.allocate_ui_at_rect(available_rect, |ui| {
                    tauri_egui::egui::ScrollArea::both()
                        .auto_shrink([false; 2])
                        .show_viewport(ui, |ui, view_port| {
                            ui.set_width(frame_width as f32);
                            ui.set_height(frame_height as f32);

                            let callback = tauri_egui::egui::PaintCallback {
                                rect: ui.available_rect_before_wrap(),
                                callback: self.render_call_back.clone(),
                            };

                            ui.painter().add(callback);

                            let input = ui.ctx().input();
                            let events = input.events.as_slice();
                            let left_top = view_port.left_top();

                            self.current_show_cursor = !input.pointer.has_pointer();

                            self.emit_input(events, move |pos| Some(pos + left_top.to_vec2()));
                        });
                });
            } else {
                let available_width = ui.available_width();
                let available_height = ui.available_height();
                let aspect_ratio = (frame_width as f32) / (frame_height as f32);

                let desktop_size = if (available_width / aspect_ratio) < available_height {
                    (available_width, available_width / aspect_ratio)
                } else {
                    (available_height * aspect_ratio, available_height)
                };

                let scale_ratio = desktop_size.0 / (frame_width as f32);

                let space_around_image = Vec2::new(
                    (available_width - desktop_size.0) / 2.0,
                    (available_height - desktop_size.1) / 2.0,
                );

                let callback = tauri_egui::egui::PaintCallback {
                    rect: Rect {
                        min: space_around_image.to_pos2(),
                        max: space_around_image.to_pos2() + desktop_size.into(),
                    },
                    callback: self.render_call_back.clone(),
                };

                ui.painter().add(callback);

                let input = ui.ctx().input();
                let events = input.events.as_slice();
                if let Some(pos) = input.pointer.hover_pos() {
                    if (space_around_image.x <= pos.x
                        && pos.x <= space_around_image.x + desktop_size.0)
                        && (space_around_image.y <= pos.y
                            && pos.y <= space_around_image.y + desktop_size.1)
                    {
                        self.current_show_cursor = false;
                    }
                }

                self.emit_input(events, move |pos| {
                    if (space_around_image.x <= pos.x
                        && pos.x <= space_around_image.x + desktop_size.0)
                        && (space_around_image.y <= pos.y
                            && pos.y <= space_around_image.y + desktop_size.1)
                    {
                        Some(Pos2::new(
                            (pos.x - space_around_image.x).max(0.0) / scale_ratio,
                            (pos.y - space_around_image.y).max(0.0) / scale_ratio,
                        ))
                    } else {
                        None
                    }
                });
            }
        } else {
            ui.centered_and_justified(|ui| {
                let (rect, _) = ui
                    .allocate_exact_size(Vec2::new(160.0, 80.0), Sense::focusable_noninteractive());

                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.spinner();
                    ui.label("preparing");
                });
            });
        }
    }

    fn build_toolbar(&mut self, ui: &mut Ui) {
        // put the toolbar at central top
        let (mut rect, _) = ui.allocate_at_least(Vec2::new(220.0, 35.0), Sense::click());
        rect.set_center(Pos2::new(ui.max_rect().width() / 2.0, 50.0));

        ui.allocate_ui_at_rect(rect, |ui| {
            Frame::default()
                .inner_margin(Margin::symmetric(6.0, 2.0))
                .rounding(Rounding::same(12.0))
                .fill(ui.style().visuals.window_fill())
                .shadow(Shadow::small_light())
                .stroke(Stroke::new(1.0, Color32::GRAY))
                .show(ui, |ui| {
                    ui.set_min_size(rect.size());
                    ui.style_mut().spacing.item_spacing = Vec2::new(6.0, 2.0);
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        // remote device id
                        ui.label(
                            RichText::new(self.state.format_remote_device_id())
                                .font(FontId::monospace(22.0)),
                        );

                        ui.separator();

                        self.build_toolbar_button_scale(ui);

                        ui.separator();

                        // FPS

                        ui.label(
                            RichText::new(self.render.read().unwrap().frame_rate().to_string())
                                .font(FontId::monospace(24.0)), // FontFamily::Name("LiquidCrystal".into()))),
                        );
                    })
                })
        });
    }

    fn build_toolbar_button_scale(&mut self, ui: &mut Ui) {
        // when use_original_resolution is true, the button should display 'fit size' icon
        ui.add_enabled_ui(self.state.desktop_frame_scalable(), |ui| {
            // ui.visuals_mut().widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
            let button = if self.state.desktop_frame_scaled() {
                tauri_egui::egui::ImageButton::new(
                    self.icon_scale.texture_id(ui.ctx()),
                    Vec2::new(18.0, 18.0),
                )
            } else {
                tauri_egui::egui::ImageButton::new(
                    self.icon_maximize.texture_id(ui.ctx()),
                    Vec2::new(18.0, 18.0),
                )
            }
            .tint(ui.visuals().noninteractive().fg_stroke.color);

            if ui.add(button).clicked() {
                self.state
                    .set_desktop_frame_scaled(!self.state.desktop_frame_scaled());
            }
        });
    }
}

impl DesktopWindow {
    fn emit_input(
        &mut self,
        events: &[tauri_egui::egui::Event],
        pos_calc_fn: impl Fn(Pos2) -> Option<Pos2>,
    ) {
        let mut input_commands = Vec::new();
        for event in events.iter() {
            match event {
                tauri_egui::egui::Event::PointerMoved(pos) => {
                    if let Some(mouse_pos) = pos_calc_fn(*pos) {
                        // if mouse_pos != self.last_mouse_pos {
                        input_commands.push(InputEvent::Mouse(MouseEvent::Move(
                            MouseKey::None,
                            mouse_pos.x,
                            mouse_pos.y,
                        )));
                    }
                }
                tauri_egui::egui::Event::PointerButton {
                    pos,
                    button,
                    pressed,
                    ..
                } => {
                    let Some(mouse_pos) = pos_calc_fn(*pos) else {
                        continue;
                    };

                    let mouse_key = match button {
                        tauri_egui::egui::PointerButton::Primary => MouseKey::Left,
                        tauri_egui::egui::PointerButton::Secondary => MouseKey::Right,
                        tauri_egui::egui::PointerButton::Middle => MouseKey::Wheel,
                        tauri_egui::egui::PointerButton::Extra1 => MouseKey::SideBack,
                        tauri_egui::egui::PointerButton::Extra2 => MouseKey::SideForward,
                    };

                    let mouse_event = if *pressed {
                        MouseEvent::Down(mouse_key, mouse_pos.x, mouse_pos.y)
                    } else {
                        MouseEvent::Up(mouse_key, mouse_pos.x, mouse_pos.y)
                    };

                    input_commands.push(InputEvent::Mouse(mouse_event));
                }
                tauri_egui::egui::Event::Scroll(scroll_vector) => {
                    input_commands
                        .push(InputEvent::Mouse(MouseEvent::ScrollWheel(scroll_vector.y)));
                }
                tauri_egui::egui::Event::RawKeyInput { key, pressed } => {
                    tracing::info!(?key, "raw key");

                    let keyboard_event = if *pressed {
                        KeyboardEvent::KeyDown(*key)
                    } else {
                        KeyboardEvent::KeyUp(*key)
                    };

                    input_commands.push(InputEvent::Keyboard(keyboard_event))
                }
                _ => {}
            }
        }

        if input_commands.is_empty() {
            return;
        }

        if let Err(err) = self
            .state
            .endpoint_client()
            .try_send(&EndPointMessage::InputCommand(EndPointInput {
                events: input_commands,
            }))
        {
            tracing::error!(?err, "send input event failed");
        }
    }
}

impl tauri_egui::eframe::App for DesktopWindow {
    fn update(&mut self, ctx: &tauri_egui::egui::Context, _: &mut tauri_egui::eframe::Frame) {
        let update_instant = std::time::Instant::now();

        self.current_show_cursor = true;

        CentralPanel::default()
            .frame(tauri_egui::egui::Frame::none())
            .show(ctx, |ui| {
                self.build_panel(ui);
            });

        if self.current_show_cursor != self.last_show_cursor {
            mirrorx_core::api::system::set_show_cursor(self.current_show_cursor);
            self.last_show_cursor = self.current_show_cursor;
        }

        let cost = update_instant.elapsed();

        if let Some(wait) = cost.checked_sub(Duration::from_millis(16)) {
            ctx.request_repaint_after(wait);
        } else {
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.render.write().unwrap().destroy(gl);
        }
    }
}
