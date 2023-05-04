use cosmic::iced_winit::widget::horizontal_space;
use cosmic::widget::segmented_button::{Entity, SingleSelectModel, self};
use cosmic::{iced, Element};
use cosmic::iced_winit::Command;
use cosmic::theme::ThemeType;
use cosmic::widget::{nav_bar, IconSource};
use cosmic::theme::Theme;
use iced::{Application, Length};
use iced::widget::{container, row};
use crate::pages::Page;

#[derive(Default)]
pub struct Symmetry {
    pub theme: Theme,
    nav_bar: SingleSelectModel,
    nav_id_to_page: segmented_button::SecondaryMap<Page>,
    page: Page,
}

impl Symmetry {
    /// Adds a page to the model we use for the navigation bar.
    fn insert_page(&mut self, page: Page) -> segmented_button::SingleSelectEntityMut {
        self.nav_bar
            .insert()
            .text(page.title())
            .icon(IconSource::from(page.icon_name()))
            .secondary(&mut self.nav_id_to_page, page)
    }

    fn page(&mut self, page: Page) {
        self.page = page;
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(ThemeType),
    NavBar(Entity)
}

impl Application for Symmetry {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;
    
    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut model = Self::default();
        model.theme = Theme::light();

        model.insert_page(Page::Welcome).activate();
        model.insert_page(Page::Settings);

        (model, Command::none())
    }
    
    fn title(&self) -> String {
        String::from("Symmetry")
    }
    
    fn update(&mut self, message: Self::Message) -> cosmic::iced::Command<Self::Message> {
        match message {
            Message::ThemeChanged(theme) => self.theme = match theme {
                ThemeType::Light => Theme::light(),
                ThemeType::Dark => Theme::dark(),
                ThemeType::HighContrastDark => Theme::dark_hc(),
                ThemeType::HighContrastLight => Theme::light_hc(),
            },
            Message::NavBar(key) => {
                if let Some(page) = self.nav_id_to_page.get(key).copied() {
                    self.nav_bar.activate(key);
                    self.page(page);
                }
            },
        }
        Command::none()
    }
    
    fn view(&self) -> Element<Message> {
        let nav_bar: Element<_> = nav_bar(&self.nav_bar, Message::NavBar).max_width(300).into();
        let content: Element<_> = match self.page {
            Page::Welcome => crate::pages::welcome::view(),
            Page::Settings => crate::pages::settings::view(&self),
        };

        container(
            row![
                nav_bar,
                horizontal_space(Length::Fill),
                content,
                horizontal_space(Length::Fill),
            ]
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
