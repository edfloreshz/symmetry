use cosmic::iced::Settings;
use cosmic::settings;
use cosmic::{Element, Theme};
use cosmic::iced::widget::{column, text, row, radio};
use cosmic::iced_winit::Alignment;
use cosmic::theme::ThemeType;

use crate::app::{Message, Symmetry};

pub fn view<'a>(app: &Symmetry) -> Element<'a, Message> {
    let theme = Some(evaluate_theme(&app.theme));
    column![
        text("Settings").size(50),
        row![
            radio("Light", ThemeType::Light, theme, Message::ThemeChanged),
            radio("Dark", ThemeType::Dark, theme, Message::ThemeChanged),
        ]
        .spacing(10)
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}

fn evaluate_theme(theme: &Theme) -> ThemeType {
    theme.theme_type
}

pub fn get_settings() -> Settings<()> {
    settings::set_default_icon_theme("Adwaita");
    let mut settings = settings();
    settings.window.min_size = Some((600, 300));
    settings
}