use crate::app::Message;
use cosmic::iced::widget::{button, column, container, text};
use cosmic::iced_winit::svg::Handle;
use cosmic::iced_winit::widget::Svg;
use cosmic::iced_winit::Alignment;
use cosmic::Element;
use symmetry_utils::configuration::Configuration;
use symmetry_utils::resources::Resources;

pub fn view<'a>() -> Element<'a, Message> {
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
                text("Before you continue, click to button below to create a new configuration file.")
                .size(18),
            )
            .padding(10)
            .into()
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
        .align_items(Alignment::Center)
        .into()
}
