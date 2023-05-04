use cosmic::iced::widget::text;
use cosmic::iced::window;
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::iced_winit::window::{drag, close, minimize, toggle_maximize};
use cosmic::widget::segmented_button::{Entity, SingleSelectModel, self};
use cosmic::{iced, Element};
use cosmic::iced_winit::Command;
use cosmic::theme::ThemeType;
use cosmic::widget::{nav_bar, IconSource, header_bar};
use cosmic::theme::Theme;
use iced::{Application, Length};
use iced::widget::{row, column};
use symmetry_utils::configuration::Configuration;
use crate::pages::Page;

#[derive(Default)]
pub struct Symmetry {
    pub theme: Theme,
    nav_bar: SingleSelectModel,
    nav_id_to_page: segmented_button::SecondaryMap<Page>,
    page: Page,
    error: String,
    show_warning: bool
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

    /// Sets the current visited page.
    fn page(&mut self, page: Page) {
        self.page = page;
    }

    /// Return the title of the selected page.
    pub fn page_title<Message: 'static>(&self, page: Page) -> Element<Message> {
        row!(text(page.title()).size(30), horizontal_space(Length::Fill),).into()
    }

    /// Toggles the warning.
    fn toggle_warning(&mut self) {
        self.show_warning = !self.show_warning
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(ThemeType),
    NavBar(Entity),
    ToggleWarning,
    Initialize,
    Maximize,
    Minimize,
    Close,
    Drag
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
            Message::Initialize => {
                let config = Configuration::new();
                match config.init() {
                    Ok(_) => self.error = "Configuration created".to_string(),
                    Err(err) => self.error = err.to_string(),
                }
                self.toggle_warning()
            },
            Message::Drag => return drag(window::Id::new(0)),
            Message::Close => return close(window::Id::new(0)),
            Message::Minimize => return minimize(window::Id::new(0), true),
            Message::Maximize => return toggle_maximize(window::Id::new(0)),
            Message::ToggleWarning => self.toggle_warning(),
        }
        Command::none()
    }
    
    fn view(&self) -> Element<Message> {
        let header = header_bar()
            .title(self.title())
            .on_close(Message::Close)
            .on_drag(Message::Drag)
            .on_maximize(Message::Maximize)
            .on_minimize(Message::Minimize);

        let nav_bar: Element<_> = nav_bar(&self.nav_bar, Message::NavBar).max_width(200).into();
        let pages: Element<_> = match self.page {
            Page::Welcome => crate::pages::welcome::view(),
            Page::Settings => crate::pages::settings::view(&self),
        };
        let content = row![nav_bar, horizontal_space(Length::Fill), pages, horizontal_space(Length::Fill)]
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20);

        if self.show_warning {
            let warning = cosmic::widget::warning(&self.error).on_close(Message::ToggleWarning);
            column![header, warning, content].into()
        } else {
            column![header, content].into()
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
