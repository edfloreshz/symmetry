use crate::app::Symmetry;
use cosmic::iced::widget::{radio, row, text};
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::iced_winit::Length;
use cosmic::theme::ThemeType;
use cosmic::widget::settings::{item, view_column, view_section};
use cosmic::{Element, Theme};

use super::Page;

#[derive(Default)]
pub struct State {
    pub theme: ThemeType,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeTheme(ThemeType),
}

pub enum Output {
    ChangeTheme(Theme),
}

impl State {
    pub fn view<'a>(&'a self, app: &'a Symmetry) -> Element<'a, Message> {
        let theme = Some(self.theme);
        let preferences = view_column(vec![
            app.page_title(Page::Settings),
            text("The settings page allows you to tailor your application experience to your preferences.")
                .size(16)
                .into(),
            view_section("Appearance")
                .add(item(
                    "Color Scheme",
                    row![
                        horizontal_space(Length::Fill),
                        radio("Light", ThemeType::Light, theme, Message::ChangeTheme),
                        radio("Dark", ThemeType::Dark, theme, Message::ChangeTheme),
                    ]
                    .spacing(10),
                ))
                .into(),
        ]);
        preferences.into()
    }

    pub fn update(&mut self, message: Message) -> Option<Output> {
        match message {
            Message::ChangeTheme(theme) => {
                let theme = match theme {
                    ThemeType::Dark => Theme::dark(),
                    ThemeType::Light => Theme::light(),
                    ThemeType::HighContrastDark => Theme::dark_hc(),
                    ThemeType::HighContrastLight => Theme::light_hc(),
                };
                Some(Output::ChangeTheme(theme))
            }
        }
    }
}
