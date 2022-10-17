use crate::gui::state::{State, StateUpdater};
use eframe::{
    egui::{style::Margin, Frame, TextEdit, TextFormat},
    epaint::{text::LayoutSection, Color32, FontId, Rounding, Stroke, Vec2},
};
use egui_extras::{Size, StripBuilder};

pub struct DeviceIDInputField<'a> {
    text: DeviceIDInputText<'a>,
}

impl<'a> DeviceIDInputField<'a> {
    pub fn new(app_state: &'a State, app_state_updater: &'a mut StateUpdater) -> Self {
        Self {
            text: DeviceIDInputText {
                app_state,
                app_state_updater,
                snapshot_str: app_state.connect_page_visit_device_id().to_string(),
            },
        }
    }
}

impl<'a> eframe::egui::Widget for &mut DeviceIDInputField<'a> {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let mut layouter = |ui: &eframe::egui::Ui, text: &str, wrap_width: f32| {
            let sections = if text.len() <= 2 {
                vec![LayoutSection {
                    leading_space: 0.0,
                    byte_range: 0..text.len(),
                    format: TextFormat::simple(FontId::monospace(53.0), Color32::BLACK),
                }]
            } else if text.len() > 2 && text.len() <= 6 {
                vec![
                    LayoutSection {
                        leading_space: 0.0,
                        byte_range: 0..2,
                        format: TextFormat::simple(FontId::monospace(53.0), Color32::BLACK),
                    },
                    LayoutSection {
                        leading_space: 18.0,
                        byte_range: 2..text.len(),
                        format: TextFormat::simple(FontId::monospace(53.0), Color32::BLACK),
                    },
                ]
            } else {
                vec![
                    LayoutSection {
                        leading_space: 0.0,
                        byte_range: 0..2,
                        format: TextFormat::simple(FontId::monospace(53.0), Color32::BLACK),
                    },
                    LayoutSection {
                        leading_space: 18.0,
                        byte_range: 2..6,
                        format: TextFormat::simple(FontId::monospace(53.0), Color32::BLACK),
                    },
                    LayoutSection {
                        leading_space: 18.0,
                        byte_range: 6..text.len(),
                        format: TextFormat::simple(FontId::monospace(53.0), Color32::BLACK),
                    },
                ]
            };

            let mut layout_job = eframe::egui::text::LayoutJob {
                sections,
                text: text.to_string(),
                break_on_newline: false,
                ..Default::default()
            };

            layout_job.wrap.max_width = wrap_width;

            ui.fonts().layout_job(layout_job)
        };

        StripBuilder::new(ui)
            .size(Size::relative(0.11))
            .size(Size::relative(0.78))
            .size(Size::relative(0.11))
            .horizontal(|mut strip| {
                strip.empty();
                strip.cell(|ui| {
                    Frame::default()
                        .rounding(Rounding::same(2.0))
                        .stroke(Stroke::new(1.0, Color32::GRAY))
                        .outer_margin(Margin::same(2.0))
                        .show(ui, |ui| {
                            let output = TextEdit::singleline(&mut self.text)
                                .layouter(&mut layouter)
                                .margin(Vec2::new(4.0, 4.0))
                                .frame(false)
                                .show(ui);

                            // first "-"
                            ui.painter().line_segment(
                                [
                                    output.response.rect.left_center() + Vec2::new(57.0, 2.0),
                                    output.response.rect.left_center() + Vec2::new(64.0, 2.0),
                                ],
                                Stroke::new(2.6, Color32::BLACK),
                            );

                            // second "-"
                            ui.painter().line_segment(
                                [
                                    output.response.rect.left_center() + Vec2::new(168.0, 2.0),
                                    output.response.rect.left_center() + Vec2::new(175.0, 2.0),
                                ],
                                Stroke::new(2.6, Color32::BLACK),
                            );
                        });
                });
                strip.empty();
            })
    }
}

struct DeviceIDInputText<'a> {
    app_state: &'a State,
    app_state_updater: &'a mut StateUpdater,
    snapshot_str: String,
}

impl<'a> eframe::egui::TextBuffer for DeviceIDInputText<'a> {
    fn is_mutable(&self) -> bool {
        false
    }

    fn as_str(&self) -> &str {
        &self.snapshot_str
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        if self.app_state.connect_page_visit_device_id().len() == 10 {
            return 0;
        }

        if !text.chars().all(|c| c.is_numeric()) {
            return 0;
        }

        let byte_idx = self.byte_index_from_char_index(char_index);

        let mut device_id = self.app_state.connect_page_visit_device_id().to_string();
        device_id.insert_str(byte_idx, text);

        self.snapshot_str = device_id.to_owned();
        self.app_state_updater
            .update_connect_page_visit_device_id(&device_id);

        text.chars().count()
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        let byte_start = self.byte_index_from_char_index(char_range.start);
        let byte_end = self.byte_index_from_char_index(char_range.end);

        let mut device_id = self.app_state.connect_page_visit_device_id().to_string();
        device_id.drain(byte_start..byte_end);

        self.snapshot_str = device_id.to_owned();
        self.app_state_updater
            .update_connect_page_visit_device_id(&device_id);
    }
}
