use app::Symmetry;
use cosmic::{
    iced::{Application, Settings},
    settings,
};

mod app;
mod pages;
mod widgets;

fn main() -> cosmic::iced::Result {
    Symmetry::run(iced_settings())
}

pub fn iced_settings() -> Settings<()> {
    settings::set_default_icon_theme("Adwaita");
    let mut settings = settings();
    settings.window.min_size = Some((600, 300));
    settings.window.decorations = false;
    settings
}
