use dark_light::{Mode, detect};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ColorScheme {
    /// Dark mode
    Dark,
    /// Light mode
    Light,
    /// Unspecified
    Default,
}

impl From<Mode> for ColorScheme {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Dark => ColorScheme::Dark,
            Mode::Light => ColorScheme::Light,
            Mode::Default => ColorScheme::Default,
        }
    }
}

impl ColorScheme {
    /// Creates a new color scheme based on the currently selected desktop color scheme. 
    pub fn new() -> Self {
        detect().into()
    }

    /// Creates a new color scheme based on the currently saved desktop color scheme. 
    pub fn current() -> Self {
        todo!("Read from config file.")
    }
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme::Default
    }
}