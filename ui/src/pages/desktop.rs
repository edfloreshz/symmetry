use cosmic::iced::widget::{button, column, container, text, text_input, radio};
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::iced_winit::{row, Alignment, Length};
use cosmic::widget::settings::{item, view_column, view_section};
use cosmic::Element;
use once_cell::sync::Lazy;
use symmetry_utils::color_scheme::ColorScheme;
use symmetry_utils::configuration::Configuration;

use crate::app::Symmetry;

use super::Page;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug, Default)]
pub struct State {
    wallpaper: String,
    selected_color_scheme: Option<ColorScheme>
}

#[derive(Clone, Debug)]
pub enum Message {
    Initialize,
    WallpaperInputChanged(String),
    ColorSchemeChanged(ColorScheme)
}

pub enum Output {
    Initialize,
    WallpaperInputChanged(String),
    ColorSchemeChanged(ColorScheme)
}

impl State {
    pub fn new(wallpaper: String, selected_color_scheme: Option<ColorScheme>) -> Self {
        Self { wallpaper, selected_color_scheme }
    }

    pub fn view<'a>(&'a self, app: &'a Symmetry) -> Element<'a, Message> {
        let config = Configuration::current();
        let wallpaper_entry: Element<Message> = text_input(
            "Paste the wallpaper path or URL here.",
            &self.wallpaper,
            Message::WallpaperInputChanged,
        )
        .padding(8)
        .size(20)
        .width(Length::FillPortion(20))
        .id(INPUT_ID.clone())
        .into();

        let wallpaper = view_section("Wallpaper")
            .add(item(
                "Source",
                row![horizontal_space(Length::Fill), wallpaper_entry].spacing(10),
            ))
            .into();
        let appearance = view_section("Appearance")
            .add(item(
                "Color Scheme",
                row![
                    horizontal_space(Length::Fill),
                    radio("Light", ColorScheme::Light, self.selected_color_scheme, Message::ColorSchemeChanged),
                    radio("Dark", ColorScheme::Dark, self.selected_color_scheme, Message::ColorSchemeChanged),
                    radio("Default", ColorScheme::Default, self.selected_color_scheme, Message::ColorSchemeChanged),
                ]
                .spacing(10),
            ))
            .into();
        let settings = vec![
            app.page_title(Page::Desktop),
            wallpaper,
            appearance
        ];
        let mut widgets: Vec<Element<Message>> = vec![];
        if config.is_none() {
            widgets.push(text("Symmetry").size(50).into());
            widgets.push(text("Symmetry is a service that ensures your settings remain consistent across all your devices.").size(20).into());
            widgets.push(container(text("Before you continue, click to button below to create a new configuration file.").size(18)).padding(10).into());
            widgets.push(
                button("Initialize")
                    .padding(10)
                    .on_press(Message::Initialize)
                    .into(),
            )
        } else {
            widgets.push(view_column(settings).into())
        }
        column(widgets)
            .spacing(10)
            .align_items(Alignment::Center)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Output> {
        match message {
            Message::WallpaperInputChanged(path) => {
                self.wallpaper = path.clone();
                Some(Output::WallpaperInputChanged(path))
            },
            Message::Initialize => Some(Output::Initialize),
            Message::ColorSchemeChanged(theme) => {
                self.selected_color_scheme = Some(theme);
                Some(Output::ColorSchemeChanged(theme))
            },
        }
    }
}
