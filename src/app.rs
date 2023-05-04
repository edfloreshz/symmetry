use iced::{Sandbox, Alignment, Length, Theme, theme, Color};
use iced::widget::{column, text, container, row, radio};

pub struct Symmetry {
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(ThemeType)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ThemeType {
    Light,
    Dark,
    Custom,
}

impl Sandbox for Symmetry {
    type Message = Message;
    
    fn new() -> Self {
        Self { theme: Theme::Dark }
    }
    
    fn title(&self) -> String {
        String::from("Symmetry")
    }
    
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ThemeChanged(theme) => self.theme = match theme {
                ThemeType::Light => Theme::Light,
                ThemeType::Dark => Theme::Dark,
                ThemeType::Custom => Theme::custom(theme::Palette {
                    background: Color::from_rgb(1.0, 0.9, 1.0),
                    text: Color::BLACK,
                    primary: Color::from_rgb(0.5, 0.5, 0.0),
                    success: Color::from_rgb(0.0, 1.0, 0.0),
                    danger: Color::from_rgb(1.0, 0.0, 0.0),
                }),
            },
        }
    }
    
    fn view(&self) -> iced::Element<'_, Self::Message> {
        container(
            column![
                text("Symmetry").size(50),
                text("Symmetry is a service that ensures your settings remain consistent across all your devices. ").size(20),
                row![
                    radio("Light", ThemeType::Light, Some(match self.theme {
                        Theme::Light => ThemeType::Light,
                        Theme::Dark => ThemeType::Dark,
                        Theme::Custom { .. } => ThemeType::Custom
                    }), Message::ThemeChanged),
                    radio("Dark", ThemeType::Dark, Some(match self.theme {
                        Theme::Light => ThemeType::Light,
                            Theme::Dark => ThemeType::Dark,
                            Theme::Custom { .. } => ThemeType::Custom
                    }), Message::ThemeChanged),
                    radio("Custom", ThemeType::Custom, Some(match self.theme {
                        Theme::Light => ThemeType::Light,
                            Theme::Dark => ThemeType::Dark,
                            Theme::Custom { .. } => ThemeType::Custom
                    }), Message::ThemeChanged),
                ]
                .spacing(10)
            ]
            .spacing(10)
            .align_items(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .padding(20)
        .into()
    }
    
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}