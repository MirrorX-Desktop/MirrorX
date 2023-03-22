use super::{
    component::nav::NavBar,
    page::*,
    state::{PageType, UIState},
};
use crate::frame::component::titlebar::TitleBar;
use eframe::egui::*;

pub struct Viewport {
    nav_bar: NavBar,
    title_bar: TitleBar,
    pages: Vec<(PageType, Box<dyn Page>)>,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            nav_bar: NavBar::new(),
            title_bar: TitleBar::new(),
            pages: vec![
                (PageType::Home, Box::<HomePage>::default()),
                (PageType::Lan, Box::new(LanPage::new())),
                (PageType::History, Box::new(HistoryPage::new())),
                (PageType::Settings, Box::new(SettingsPage::new())),
            ],
        }
    }

    pub fn draw(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
        ui_state: &mut UIState,
    ) {
        CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgb(35, 36, 39),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.with_layout(
                    Layout::left_to_right(Align::Center).with_cross_justify(true),
                    |ui| {
                        ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);

                        self.nav_bar.draw(ui, ui_state);
                        ui.add(Separator::default().spacing(0.0));

                        ui.with_layout(
                            Layout::top_down(Align::Center).with_cross_justify(true),
                            |ui| {
                                self.title_bar.draw(ui, frame, ui_state);
                                ui.add(Separator::default().spacing(0.0));

                                ui.allocate_ui(ui.available_rect_before_wrap().size(), |ui| {
                                    let current_page_type = ui_state.current_page_type.clone();
                                    if let Some((_, page)) = self
                                        .pages
                                        .iter_mut()
                                        .find(move |(page_type, _)| current_page_type.eq(page_type))
                                    {
                                        page.draw(ui, ui_state);
                                    } else {
                                        ui.label("page not found");
                                    }
                                })
                            },
                        );
                    },
                );
            });
    }
}
