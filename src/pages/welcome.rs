use cosmic::Element;
use cosmic::iced::widget::{column, text};
use cosmic::iced_winit::Alignment;

use crate::app::Message;

pub fn view<'a>() -> Element<'a, Message> {
    column![
        text("Symmetry").size(50),
        text("Symmetry is a service that ensures your settings remain consistent across all your devices. ").size(20),
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}