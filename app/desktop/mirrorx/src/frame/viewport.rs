use super::{component::nav::NavBar, page::*};
use crate::frame::component::titlebar::TitleBar;
use eframe::egui::*;

#[derive(Debug, Hash, PartialEq, Eq, Copy)]
pub enum PageType {
    Home,
    Lan,
    History,
    Settings,
}

impl Clone for PageType {
    fn clone(&self) -> Self {
        match self {
            Self::Home => Self::Home,
            Self::Lan => Self::Lan,
            Self::History => Self::History,
            Self::Settings => Self::Settings,
        }
    }
}

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
                (PageType::Home, Box::new(HomePage::new())),
                (PageType::Lan, Box::new(LanPage::new())),
                (PageType::History, Box::new(HistoryPage::new())),
                (PageType::Settings, Box::new(SettingsPage::new())),
            ],
        }
    }

    pub fn draw(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
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

                        self.nav_bar.draw(ui);
                        ui.add(Separator::default().spacing(0.0));

                        ui.with_layout(
                            Layout::top_down(Align::Center).with_cross_justify(true),
                            |ui| {
                                self.title_bar.draw(ui, frame);
                                ui.add(Separator::default().spacing(0.0));

                                ui.allocate_ui(ui.available_rect_before_wrap().size(), |ui| {
                                    let current_page_type = self.nav_bar.current_page_type();
                                    if let Some((_, page)) = self
                                        .pages
                                        .iter_mut()
                                        .find(move |(page_type, _)| current_page_type.eq(page_type))
                                    {
                                        page.draw(ui);
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
