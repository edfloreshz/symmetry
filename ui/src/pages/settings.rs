use cosmic::iced::Settings;
use cosmic::iced_winit::Length;
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::settings;
use cosmic::widget::settings::{view_column, view_section, item};
use cosmic::{Element, Theme};
use cosmic::iced::widget::{row, radio};
use cosmic::theme::ThemeType;
use crate::app::{Message, Symmetry};

use super::Page;

pub fn view<'a>(app: &'a Symmetry) -> Element<'a, Message> {
    let theme = Some(evaluate_theme(&app.theme));
    let preferences = view_column(vec![
        app.page_title(Page::Settings),
        view_section("Appearance")
            .add(item("Color Scheme", row![
                horizontal_space(Length::Fill),
                radio("Light", ThemeType::Light, theme, Message::ThemeChanged),
                radio("Dark", ThemeType::Dark, theme, Message::ThemeChanged),
            ].spacing(10)))
            .into()
    ]);
    preferences.into()
}

fn evaluate_theme(theme: &Theme) -> ThemeType {
    theme.theme_type
}

pub fn get_settings() -> Settings<()> {
    settings::set_default_icon_theme("Adwaita");
    let mut settings = settings();
    settings.window.min_size = Some((600, 300));
    settings.window.decorations = false;
    settings
}