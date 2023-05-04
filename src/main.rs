use app::Symmetry;
use iced::{Application, Settings};

mod app;

fn main() -> iced::Result {
    Symmetry::run(Settings::default())
}
