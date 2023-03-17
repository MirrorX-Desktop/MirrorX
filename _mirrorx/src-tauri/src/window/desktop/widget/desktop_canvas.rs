use crate::window::desktop::render::canvas::Canvas;
use mirrorx_core::{core_error, error::CoreResult, DesktopDecodeFrame};
use std::sync::Arc;
use tauri_egui::{
    eframe::{egui_glow::CallbackFn, glow},
    egui::*,
};

pub struct DesktopCanvas {
    gl_context: Arc<glow::Context>,
    video_frame: Option<Arc<DesktopDecodeFrame>>,
    canvas: Option<Arc<Canvas>>,
    canvas_paint_callback: Option<Arc<CallbackFn>>,
    should_update_canvas: bool,
    should_update_canvas_paint_callback: bool,
    scale_available: bool,
    scale_enable: bool,
}

impl DesktopCanvas {
    pub fn new(gl_context: Arc<tauri_egui::eframe::glow::Context>) -> Self {
        DesktopCanvas {
            gl_context,
            video_frame: None,
            canvas: None,
            canvas_paint_callback: None,
            should_update_canvas: false,
            should_update_canvas_paint_callback: false,
            scale_available: false,
            scale_enable: false,
        }
    }

    pub fn draw(
        &mut self,
        frame: Option<Arc<DesktopDecodeFrame>>,
        ui: &mut tauri_egui::egui::Ui,
    ) -> CoreResult<()> {
        self.update_video_frame(frame);

        let Some(frame) = self.video_frame.clone() else {
            self.draw_initial_progress(ui);
            return Ok(());
        };

        self.update_canvas(&frame)?;

        let Some(canvas) = self.canvas.clone() else {
            return Err(core_error!("canvas is empty"));
        };

        self.update_canvas_paint_callback(canvas, frame.clone());

        let Some(canvas_paint_callback) = self.canvas_paint_callback.clone() else {
            return Err(core_error!("canvas paint callback is empty"));
        };

        self.scale_available =
            ui.available_width() < frame.width as _ || ui.available_height() < frame.height as _;

        if self.scale_enable
            && (ui.available_width() < frame.width as _
                || ui.available_height() < frame.height as _)
        {
            self.draw_scaled_desktop_view(ui, canvas_paint_callback, frame);
        } else {
            self.draw_original_desktop_view(ui, canvas_paint_callback, frame);
        }

        Ok(())
    }

    fn update_video_frame(&mut self, frame: Option<Arc<DesktopDecodeFrame>>) {
        // if frame is None, don't update canvas and callback.
        // if frame is Some,
        //    if frame's format, width and height equal to old one, update callback.
        //    else update canvas and callback

        if let Some(new_frame) = frame {
            if let Some(ref old_frame) = self.video_frame {
                self.should_update_canvas = old_frame.width != new_frame.width
                    || old_frame.height != new_frame.height
                    || old_frame.format != new_frame.format;
            } else {
                self.should_update_canvas = true;
            }

            self.should_update_canvas_paint_callback = true;
            self.video_frame.replace(new_frame);
        } else {
            self.should_update_canvas = false;
            self.should_update_canvas_paint_callback = false;
        }
    }

    fn update_canvas(&mut self, frame: &DesktopDecodeFrame) -> CoreResult<()> {
        if self.should_update_canvas {
            match Canvas::new(
                self.gl_context.clone(),
                frame.width,
                frame.height,
                frame.format.clone(),
            ) {
                Ok(new_canvas) => self.canvas = Some(Arc::new(new_canvas)),
                Err(err) => {
                    return Err(core_error!(
                        "update_canvas: initialize canvas failed ({})",
                        err
                    ))
                }
            }
        }

        Ok(())
    }

    fn update_canvas_paint_callback(
        &mut self,
        canvas: Arc<Canvas>,
        frame: Arc<DesktopDecodeFrame>,
    ) {
        if self.should_update_canvas_paint_callback {
            self.canvas_paint_callback = Some(Arc::new(CallbackFn::new(move |_info, painter| {
                if let Err(err) =
                    canvas.paint(painter.gl(), frame.clone(), painter.intermediate_fbo())
                {
                    tracing::error!(?err, "desktop render failed");
                }
            })))
        }
    }

    fn draw_initial_progress(&self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            let (rect, _) =
                ui.allocate_exact_size(Vec2::new(160.0, 80.0), Sense::focusable_noninteractive());

            ui.allocate_ui_at_rect(rect, |ui| {
                ui.spinner();
                ui.label("preparing");
            });
        });
    }

    fn draw_scaled_desktop_view(
        &mut self,
        ui: &mut Ui,
        callback: Arc<CallbackFn>,
        frame: Arc<DesktopDecodeFrame>,
    ) {
        let left = ((ui.available_width() - frame.width as f32) / 2.0).max(0.0);
        let top = ((ui.available_height() - frame.height as f32) / 2.0).max(0.0);

        let mut available_rect = ui.available_rect_before_wrap();
        available_rect.min = Pos2::new(left, top);

        ui.allocate_ui_at_rect(available_rect, |ui| {
            tauri_egui::egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show_viewport(ui, |ui, view_port| {
                    ui.set_width(frame.width as f32);
                    ui.set_height(frame.height as f32);

                    let callback = tauri_egui::egui::PaintCallback {
                        rect: ui.available_rect_before_wrap(),
                        callback,
                    };

                    ui.painter().add(callback);

                    let input = ui.ctx().input();
                    let events = input.events.as_slice();
                    let left_top = view_port.left_top();

                    // self.current_show_cursor = !input.pointer.has_pointer();

                    // self.emit_input(events, move |pos| Some(pos + left_top.to_vec2()));
                });
        });
    }

    fn draw_original_desktop_view(
        &mut self,
        ui: &mut Ui,
        callback: Arc<CallbackFn>,
        frame: Arc<DesktopDecodeFrame>,
    ) {
        let available_width = ui.available_width();
        let available_height = ui.available_height();
        let aspect_ratio = (frame.width as f32) / (frame.height as f32);

        let desktop_size = if (available_width / aspect_ratio) < available_height {
            (available_width, available_width / aspect_ratio)
        } else {
            (available_height * aspect_ratio, available_height)
        };

        let scale_ratio = desktop_size.0 / (frame.width as f32);

        let space_around_image = Vec2::new(
            (available_width - desktop_size.0) / 2.0,
            (available_height - desktop_size.1) / 2.0,
        );

        let callback = tauri_egui::egui::PaintCallback {
            rect: Rect {
                min: space_around_image.to_pos2(),
                max: space_around_image.to_pos2() + desktop_size.into(),
            },
            callback,
        };

        ui.painter().add(callback);

        let input = ui.ctx().input();
        let events = input.events.as_slice();
        if let Some(pos) = input.pointer.hover_pos() {
            if (space_around_image.x <= pos.x && pos.x <= space_around_image.x + desktop_size.0)
                && (space_around_image.y <= pos.y && pos.y <= space_around_image.y + desktop_size.1)
            {
                // self.current_show_cursor = false;
            }
        }

        // self.emit_input(events, move |pos| {
        //     if (space_around_image.x <= pos.x && pos.x <= space_around_image.x + desktop_size.0)
        //         && (space_around_image.y <= pos.y && pos.y <= space_around_image.y + desktop_size.1)
        //     {
        //         Some(Pos2::new(
        //             (pos.x - space_around_image.x).max(0.0) / scale_ratio,
        //             (pos.y - space_around_image.y).max(0.0) / scale_ratio,
        //         ))
        //     } else {
        //         None
        //     }
        // });
    }
}
