use crate::app::Symmetry;
use cosmic::iced::widget::{radio, row, text};
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::iced_winit::Length;
use cosmic::theme::ThemeType;
use cosmic::widget::settings::{item, view_column, view_section};
use cosmic::widget::toggler;
use cosmic::{Element, Theme};
use symmetry_core::configuration::repository_type::RepositoryType;
use symmetry_core::sync::providers::config::crdt::CrdtConfig;
use symmetry_core::sync::providers::config::git::GitConfig;

use super::Page;

#[derive(Default)]
pub struct State {
    pub theme: ThemeType,
    services: Services,
}

#[derive(Debug, Default)]
struct Services {
    git: GitConfig,
    crdt: CrdtConfig,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeTheme(ThemeType),
    ToggleService(RepositoryType, bool),
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
            view_section("Sync Services")
                .add(item("Git", row![horizontal_space(Length::Fill), toggler(None, self.services.git.enabled, |state| Message::ToggleService(RepositoryType::Git, state))]))
                .add(item("CRDT", row![horizontal_space(Length::Fill), toggler(None, self.services.crdt.enabled, |state| Message::ToggleService(RepositoryType::Git, state))])).into()
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
            Message::ToggleService(service, state) => match service {
                RepositoryType::Git => {
                    self.services.git.enabled = state;
                    todo!("Configure Git repository.")
                }
                RepositoryType::Crdt => {
                    self.services.crdt.enabled = state;
                    todo!("Configure CRDT repository.")
                }
            },
        }
    }
}
