use super::{state::SharedState, view::*, widget::StatefulWidget};
use crate::frame::component::titlebar::TitleBar;
use eframe::egui::*;
use egui_extras::{Size, StripBuilder};

pub struct Viewport {
    title_bar: TitleBar,
    current_page_type: ViewId,
    pages: Vec<(ViewId, Box<dyn StatefulWidget>)>,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            title_bar: TitleBar::new(),
            current_page_type: ViewId::Device,
            pages: vec![
                (ViewId::Device, Box::<HomePage>::default()),
                (ViewId::Lan, Box::new(LanView::new())),
                (ViewId::History, Box::new(HistoryView::new())),
                (ViewId::Settings, Box::new(SettingsView::new())),
            ],
        }
    }

    pub fn draw(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
        ui_state: &SharedState,
    ) {
        CentralPanel::default()
            .frame(Frame {
                fill: ui_state.theme_color().background_body,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.style_mut().spacing.item_spacing = Vec2::ZERO;
                StripBuilder::new(ui)
                    .size(Size::exact(48.0))
                    .size(Size::remainder())
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            // self.title_bar.draw(ui, frame, ui_state);
                        });
                        strip.cell(|ui| {
                            // ui.add(Separator::default().spacing(0.0));
                            let area_rect = ui.available_rect_before_wrap();

                            ui.painter().rect_filled(
                                area_rect,
                                Rounding::none(),
                                ui_state.theme_color().background_level1,
                            );

                            // if let Some((_, page)) = self
                            //     .pages
                            //     .iter_mut()
                            //     .find(move |(page_type, _)| page_type.eq(&self.current_page_type))
                            // {
                            //     page.draw(ui, ui_state);
                            // } else {
                            //     ui.label("page not found");
                            // }
                        });
                    });
            });
    }
}
