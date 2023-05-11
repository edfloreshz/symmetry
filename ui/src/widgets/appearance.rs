use cosmic::{
    iced::widget::radio,
    iced_winit::{row, widget::horizontal_space, Length},
    widget::settings::{item, view_section},
    Element,
};
use symmetry_core::color_scheme::ColorScheme;

use crate::pages::desktop::Message;

pub(crate) fn appearance_section<'a>(
    selected_color_scheme: Option<ColorScheme>,
) -> Element<'a, Message> {
    view_section("Appearance")
        .add(item(
            "Color Scheme",
            row![
                horizontal_space(Length::Fill),
                radio(
                    "Light",
                    ColorScheme::Light,
                    selected_color_scheme,
                    Message::ColorSchemeChanged
                ),
                radio(
                    "Dark",
                    ColorScheme::Dark,
                    selected_color_scheme,
                    Message::ColorSchemeChanged
                ),
                radio(
                    "Default",
                    ColorScheme::Default,
                    selected_color_scheme,
                    Message::ColorSchemeChanged
                ),
            ]
            .spacing(10),
        ))
        .into()
}
