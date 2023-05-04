use app::Symmetry;
use cosmic::iced::Application;
use pages::settings::get_settings;

mod app;
mod pages;

fn main() -> cosmic::iced::Result {
    Symmetry::run(get_settings())
}
