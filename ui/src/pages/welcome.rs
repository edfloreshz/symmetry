use cosmic::iced::widget::{column, text};
use cosmic::iced_winit::svg::Handle;
use cosmic::iced_winit::widget::Svg;
use cosmic::iced_winit::Alignment;
use cosmic::Element;
use symmetry_core::resources::Resources;

use crate::app::Message;

#[derive(Debug, Default)]
pub struct State;

impl State {
    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let icon = Resources::get("icon/dev.edfloreshz.Symmetry.svg")
            .unwrap()
            .data;

        column![
            Svg::new(Handle::from_memory(icon)),
            text("Symmetry").size(50),
            text("Symmetry is a service that ensures your settings remain consistent across all your devices.").size(20)
        ]
        .spacing(10)
        .padding([0, 20, 0, 20])
        .align_items(Alignment::Center)
        .into()
    }
}
