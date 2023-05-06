use super::Page;
use crate::app::Symmetry;
use cosmic::iced::widget::{button, radio, text, text_input, Image};
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::iced_winit::{row, Length};
use cosmic::theme;
use cosmic::widget::settings::{item, view_column, view_section};
use cosmic::widget::{icon, scrollable, IconSource};
use cosmic::Element;
use once_cell::sync::Lazy;
use symmetry_utils::color_scheme::ColorScheme;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug, Default)]
pub struct State {
    wallpaper: String,
    selected_color_scheme: Option<ColorScheme>,
}

#[derive(Clone, Debug)]
pub enum Message {
    WallpaperChanged(String),
    ColorSchemeChanged(ColorScheme),
    OpenFilePicker,
}

pub enum Output {
    WallpaperInputChanged(String),
    ColorSchemeChanged(ColorScheme),
    OpenFilePicker,
}

impl State {
    pub fn new(wallpaper: String, selected_color_scheme: Option<ColorScheme>) -> Self {
        Self {
            wallpaper,
            selected_color_scheme,
        }
    }

    pub fn view<'a>(&'a self, app: &'a Symmetry) -> Element<'a, Message> {
        let wallpaper_entry: Element<Message> = text_input(
            "Paste the wallpaper path or URL here.",
            &self.wallpaper,
            Message::WallpaperChanged,
        )
        .padding(10)
        .size(16)
        .width(Length::FillPortion(20))
        .id(INPUT_ID.clone())
        .into();

        let wallpaper = view_section("Wallpaper")
            .add(item(
                "Source",
                row![
                    horizontal_space(Length::Fill),
                    wallpaper_entry,
                    button(
                        icon(IconSource::from("document-open-symbolic"), 16)
                            .style(theme::Svg::SymbolicPrimary)
                    )
                    .padding(10)
                    .on_press(Message::OpenFilePicker)
                ]
                .spacing(10),
            ))
            .add(item(
                "Preview",
                Image::new(&self.wallpaper).width(Length::FillPortion(20)),
            ))
            .into();
        let appearance = view_section("Appearance")
            .add(item(
                "Color Scheme",
                row![
                    horizontal_space(Length::Fill),
                    radio(
                        "Light",
                        ColorScheme::Light,
                        self.selected_color_scheme,
                        Message::ColorSchemeChanged
                    ),
                    radio(
                        "Dark",
                        ColorScheme::Dark,
                        self.selected_color_scheme,
                        Message::ColorSchemeChanged
                    ),
                    radio(
                        "Default",
                        ColorScheme::Default,
                        self.selected_color_scheme,
                        Message::ColorSchemeChanged
                    ),
                ]
                .spacing(10),
            ))
            .into();
        let settings = vec![
            app.page_title(Page::Desktop),
            text("The desktop preferences section allows you to customize and personalize your desktop environment to suit your unique preferences and workflow.")
                .size(16)
                .into(),
            wallpaper,
            appearance
        ];
        scrollable(view_column(settings)).into()
    }

    pub fn update(&mut self, message: Message) -> Option<Output> {
        match message {
            Message::WallpaperChanged(path) => {
                self.wallpaper = path.clone();
                Some(Output::WallpaperInputChanged(path))
            }
            Message::ColorSchemeChanged(theme) => {
                self.selected_color_scheme = Some(theme);
                Some(Output::ColorSchemeChanged(theme))
            }
            Message::OpenFilePicker => Some(Output::OpenFilePicker),
        }
    }
}
