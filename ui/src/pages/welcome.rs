use cosmic::iced::widget::{button, column, container, text, text_input};
use cosmic::iced_winit::svg::Handle;
use cosmic::iced_winit::widget::Svg;
use cosmic::iced_winit::Alignment;
use cosmic::Element;
use symmetry_utils::configuration::Configuration;
use symmetry_utils::resources::Resources;

use crate::app::refresh_sync_provider;

#[derive(Debug, Default)]
pub struct State {
    repository: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    Initialize,
    RepositoryChanged(String),
}

pub enum Output {
    Message(String),
    Error(String),
}

impl State {
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let config = Configuration::current();
        let icon = Resources::get("icon/dev.edfloreshz.Symmetry.svg")
            .unwrap()
            .data;
        let mut widgets: Vec<Element<_>> = vec![
            Svg::new(Handle::from_memory(icon)).into(),
            text("Symmetry").size(50).into(),
            text("Symmetry is a service that ensures your settings remain consistent across all your devices.")
                .size(20)
                .into()
        ];

        if config.is_none() {
            widgets.push(
                container(
                    text("Before you continue, create a git repository, paste the link and click the button below to create a new configuration file.")
                        .size(18),
                )
                .padding(10)
                .into()
            );
            widgets.push(
                text_input(
                    "Paste the repository URL here.",
                    &self.repository,
                    Message::RepositoryChanged,
                )
                .into(),
            );
            widgets.push(
                button("Initialize")
                    .padding(10)
                    .on_press(Message::Initialize)
                    .into(),
            );
        }

        column(widgets)
            .spacing(10)
            .padding([0, 20, 0, 20])
            .align_items(Alignment::Center)
            .into()
    }

    pub fn update(&mut self, message: Message) -> Option<Output> {
        match message {
            Message::Initialize => {
                if !self.repository.is_empty() {
                    let config = Configuration::new();
                    match config.init(self.repository.clone()) {
                        Ok(_) => {
                            refresh_sync_provider();
                            Some(Output::Message("Configuration created".into()));
                        }
                        Err(err) => {
                            Some(Output::Error(err.to_string()));
                        }
                    }
                }
                None
            }
            Message::RepositoryChanged(repo) => {
                self.repository = repo;
                None
            }
        }
    }
}
