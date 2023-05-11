use cosmic::{
    iced::widget::button,
    theme::{Button, Svg},
    widget::{header_bar, icon, IconSource},
    Element,
};

use crate::app::Message;

pub(crate) fn header<'a>(title: String) -> Element<'a, Message> {
    header_bar()
        .title(title)
        .start(
            button(
                icon(IconSource::from("display-brightness-symbolic"), 16)
                    .style(Svg::SymbolicActive),
            )
            .padding([8, 16, 8, 16])
            .style(Button::Text)
            .on_press(Message::SwitchColorScheme)
            .into(),
        )
        .end(
            button(
                icon(IconSource::from("emblem-synchronizing-symbolic"), 16)
                    .style(Svg::SymbolicActive),
            )
            .padding([8, 16, 8, 16])
            .style(Button::Text)
            .on_press(Message::Sync)
            .into(),
        )
        .on_close(Message::Close)
        .on_drag(Message::Drag)
        .on_maximize(Message::Maximize)
        .on_minimize(Message::Minimize)
        .into()
}
