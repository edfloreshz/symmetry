use cosmic::Element;
use cosmic::iced::widget::{column, text, button, container};
use cosmic::iced_winit::Alignment;

use crate::app::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let widgets: Vec<Element<Message>> = vec![
        text("Symmetry").size(50).into(),
        text("Symmetry is a service that ensures your settings remain consistent across all your devices.").size(20).into(),
        container(text("Before you continue, click to button below to create a new configuration file.").size(18)).padding(10).into(),
        button("Initialize").padding(10).on_press(Message::Initialize).into()
    ];
    column(widgets)
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}