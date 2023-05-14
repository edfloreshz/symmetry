use cosmic::{
    iced::widget::{button, Image},
    iced_winit::{
        row,
        widget::{horizontal_space, text_input},
        Length,
    },
    theme,
    widget::{
        icon,
        settings::{item, view_section},
    },
    Element,
};
use once_cell::sync::Lazy;

use crate::pages::desktop::Message;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub(crate) fn wallpaper_section<'a>(wallpaper: String) -> Element<'a, Message> {
    let wallpaper_entry: Element<Message> = text_input(
        "Paste the wallpaper path or URL here.",
        &wallpaper,
        Message::WallpaperChanged,
    )
    .padding(10)
    .size(16)
    .width(Length::FillPortion(20))
    .id(INPUT_ID.clone())
    .into();

    view_section("Wallpaper")
        .add(item(
            "Source",
            row![
                horizontal_space(Length::Fill),
                wallpaper_entry,
                button(icon("computer-symbolic", 16).style(theme::Svg::SymbolicPrimary))
                    .padding(10)
                    .on_press(Message::SetCurrentWallpaper),
                button(icon("document-open-symbolic", 16).style(theme::Svg::SymbolicPrimary))
                    .padding(10)
                    .on_press(Message::OpenFilePicker)
            ]
            .spacing(10),
        ))
        .add(item(
            "Preview",
            Image::new(&wallpaper).width(Length::FillPortion(20)),
        ))
        .into()
}
