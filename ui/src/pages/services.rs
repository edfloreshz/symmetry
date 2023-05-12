use crate::app::Symmetry;
use cosmic::iced::widget::{button, row, text, text_input};
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::iced_winit::Length;
use cosmic::widget::settings::{item, view_column, view_section};
use cosmic::widget::{icon, toggler};
use cosmic::{theme, Element};
use once_cell::sync::Lazy;
use symmetry_core::configuration::repository_type::Service;
use symmetry_core::configuration::Configuration;
use symmetry_core::sync::providers::config::Services;

use super::Page;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub struct State {
    pub active_service: Service,
    service_config: Services,
}

impl Default for State {
    fn default() -> Self {
        if let Some(config) = Configuration::current() {
            Self {
                service_config: config.service_config,
                active_service: config.active_service,
            }
        } else {
            Self {
                service_config: Services::default(),
                active_service: Default::default(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    GitUrlChanged(String),
    InitializeGitRepo,
    ToggleService(Service, bool),
}

pub enum Output {
    Error(String),
    Sync,
}

impl State {
    pub fn view<'a>(&'a self, app: &'a Symmetry) -> Element<'a, Message> {
        let mut git_section = view_section("Git").add(item(
            "Status",
            row![
                horizontal_space(Length::Fill),
                toggler(
                    Some("Free and open source distributed version control system".into()),
                    self.service_config.git.enabled,
                    |state| Message::ToggleService(Service::Git, state)
                )
            ],
        ));
        if self.service_config.git.enabled {
            git_section = git_section.add(item(
                "Url",
                row![
                    text_input(
                        "Paste the Git repo URL here.",
                        &self.service_config.git.url,
                        Message::GitUrlChanged,
                    )
                    .padding(10)
                    .size(16)
                    .width(Length::FillPortion(20))
                    .id(INPUT_ID.clone()),
                    button(icon("object-select-symbolic", 16).style(theme::Svg::SymbolicPrimary))
                        .padding(10)
                        .on_press(Message::InitializeGitRepo)
                ]
                .spacing(10),
            ))
        }
        let preferences = view_column(vec![
            app.page_title(Page::Services),
            text("The settings page allows you manage your sync services.")
                .size(16)
                .into(),
            git_section.into(),
            view_section("CRDT")
                .add(item(
                    "Status",
                    row![
                        horizontal_space(Length::Fill),
                        toggler(
                            Some("Allows multiple devices to collaborate without conflicts".into()),
                            self.service_config.crdt.enabled,
                            |state| Message::ToggleService(Service::Crdt, state)
                        )
                    ],
                ))
                .into(),
        ]);
        preferences.into()
    }

    pub fn update(&mut self, message: Message) -> Option<Output> {
        match message {
            Message::GitUrlChanged(url) => {
                self.service_config.git.url = url;
                None
            }
            Message::InitializeGitRepo => {
                self.service_config = self.service_config.clone();
                self.active_service = Service::Git;
                match self.write_to_config() {
                    Some(output) => Some(output),
                    None => Some(Output::Sync),
                }
            }
            Message::ToggleService(service, state) => match service {
                Service::Git => {
                    self.service_config.git.enabled = state;
                    self.write_to_config()
                }
                Service::Crdt => {
                    self.service_config.crdt.enabled = state;
                    self.write_to_config()
                }
            },
        }
    }

    fn write_to_config(&mut self) -> Option<Output> {
        if let Some(mut config) = Configuration::current() {
            config.service_config = self.service_config.clone();
            config.active_service = self.active_service.clone();
            return match config.write() {
                Ok(_) => None,
                Err(err) => Some(Output::Error(err.to_string())),
            };
        }
        None
    }
}
