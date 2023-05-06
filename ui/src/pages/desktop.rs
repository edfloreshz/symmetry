use super::Page;
use crate::app::Symmetry;
use crate::widgets::appearance::appearance_section;
use crate::widgets::wallpaper::wallpaper_section;
use cosmic::iced::widget::text;
use cosmic::widget::scrollable;
use cosmic::widget::settings::view_column;
use cosmic::Element;
use symmetry_utils::color_scheme::ColorScheme;

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
        let wallpaper = wallpaper_section(self.wallpaper.clone());
        let appearance = appearance_section(self.selected_color_scheme);
        scrollable(view_column(vec![
            app.page_title(Page::Desktop),
            text("The desktop preferences section allows you to customize and personalize your desktop environment to suit your unique preferences and workflow.")
                .size(16)
                .into(),
            wallpaper,
            appearance
        ])).into()
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
