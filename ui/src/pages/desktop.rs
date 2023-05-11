use super::Page;
use crate::app::Symmetry;
use crate::widgets::appearance::appearance_section;
use crate::widgets::wallpaper::wallpaper_section;
use ashpd::desktop::file_chooser::OpenFileRequest;
use ashpd::WindowIdentifier;
use cosmic::iced::widget::text;
use cosmic::widget::scrollable;
use cosmic::widget::settings::view_column;
use cosmic::Element;
use symmetry_core::color_scheme::ColorScheme;
use symmetry_core::configuration::Configuration;

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
    OpenFilePicker(OpenFileRequest),
    Message(String),
    Error(String),
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
        let desktop: Element<'a, Message> = view_column(vec![
            app.page_title(Page::Desktop),
            text("The desktop preferences section allows you to customize and personalize your desktop environment to suit your unique preferences and workflow.")
                .size(16)
                .into(),
            wallpaper,
            appearance
        ]).into();
        scrollable(desktop).into()
    }

    pub fn update(&mut self, message: Message) -> Option<Output> {
        match message {
            Message::WallpaperChanged(path) => {
                self.wallpaper = path.clone();
                let config = Configuration::current();
                if let Some(mut config) = config {
                    config.wallpaper = path;
                    return match config.write() {
                        Ok(_) => Some(Output::Message("Wallpaper path updated".to_string())),
                        Err(err) => Some(Output::Error(err.to_string())),
                    };
                }
                None
            }
            Message::ColorSchemeChanged(theme) => {
                self.selected_color_scheme = Some(theme);
                let config = Configuration::current();
                if let Some(mut config) = config {
                    config.color_scheme = theme;
                    return match config.write() {
                        Ok(_) => Some(Output::Message("Color scheme updated".into())),
                        Err(err) => Some(Output::Error(err.to_string())),
                    };
                }
                None
            }
            Message::OpenFilePicker => {
                let request = OpenFileRequest::default()
                    .directory(false)
                    .identifier(Some(WindowIdentifier::None))
                    .modal(true)
                    .title("Select your wallpaper")
                    .multiple(false)
                    .accept_label("Pick wallpaper");
                Some(Output::OpenFilePicker(request))
            }
        }
    }
}
