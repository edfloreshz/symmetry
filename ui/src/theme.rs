use cosmic::{iced::application, iced_style::text, iced_winit::color};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub color_scheme: ColorScheme,
}

impl Theme {
    pub const Light: Self = Self {
        color_scheme: ColorScheme::Light,
    };
    pub const Dark: Self = Self {
        color_scheme: ColorScheme::Dark,
    };
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    #[default]
    Light,
    Dark,
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> cosmic::iced_winit::application::Appearance {
        application::Appearance {
            background_color: color!(0x28, 0x28, 0x28),
            text_color: color!(0x28, 0x28, 0x28),
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    
    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: color!(0xeb, 0xeb, 0xeb).into(),
        }
    }
}
