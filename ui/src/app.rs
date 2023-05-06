use crate::pages::{desktop, Page};
use crate::widgets::header_bar::header;
use ashpd::desktop::file_chooser::OpenFileRequest;
use ashpd::WindowIdentifier;
use cosmic::iced::{window, Application};
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::iced_winit::window::{close, drag, minimize, toggle_maximize};
use cosmic::iced_winit::{column, row, Command};
use cosmic::theme::ThemeType;
use cosmic::widget::segmented_button::{self, Entity, SingleSelectModel};
use cosmic::widget::{nav_bar, text, IconSource};
use cosmic::{iced, Element, Theme};
use iced::Length;
use symmetry_utils::configuration::Configuration;

#[derive(Default)]
pub struct Symmetry {
    pub theme: Theme,
    nav_bar: SingleSelectModel,
    nav_id_to_page: segmented_button::SecondaryMap<Page>,
    page: Page,
    error: String,
    show_warning: bool,
    desktop: crate::pages::desktop::State,
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
    Desktop(desktop::Message),
    SwitchColorScheme,
    HandlePickedFile(Vec<String>),
    NavBar(Entity),
    ToggleWarning,
    Error(String),
    Initialize,
    Maximize,
    Minimize,
    Close,
    Drag,
}

impl Application for Symmetry {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let mut model = Self::default();
        let config = Configuration::current();
        model.theme = Theme::light();

        if let Some(config) = config {
            model.desktop = desktop::State::new(config.wallpaper, Some(config.color_scheme));
        }

        model.insert_page(Page::Welcome).activate();
        model.insert_page(Page::Desktop);
        model.insert_page(Page::Settings);

        (model, Command::none())
    }

    fn title(&self) -> String {
        String::from("Symmetry")
    }

    fn view(&self) -> Element<Message> {
        let header = header(self.title());

        let nav_bar: Element<_> = nav_bar(&self.nav_bar, Message::NavBar)
            .max_width(200)
            .into();
        let page: Element<_> = match self.page {
            Page::Welcome => crate::pages::welcome::view(),
            Page::Desktop => self.desktop.view(&self).map(Message::Desktop),
            Page::Settings => crate::pages::settings::view(&self),
        };
        let content = row![
            nav_bar,
            horizontal_space(Length::Fill),
            page,
            horizontal_space(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10);

        if self.show_warning {
            let warning = cosmic::widget::warning(&self.error).on_close(Message::ToggleWarning);
            column![header, warning, content].into()
        } else {
            column![header, content].into()
        }
    }

    fn update(&mut self, message: Self::Message) -> cosmic::iced::Command<Self::Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = match theme {
                    ThemeType::Dark => Theme::dark(),
                    ThemeType::Light => Theme::light(),
                    ThemeType::HighContrastDark => Theme::dark_hc(),
                    ThemeType::HighContrastLight => Theme::light_hc(),
                }
            }
            Message::NavBar(key) => {
                if let Some(page) = self.nav_id_to_page.get(key).copied() {
                    self.nav_bar.activate(key);
                    self.page(page);
                }
            }
            Message::Drag => return drag(window::Id::new(0)),
            Message::Close => return close(window::Id::new(0)),
            Message::Minimize => return minimize(window::Id::new(0), true),
            Message::Maximize => return toggle_maximize(window::Id::new(0)),
            Message::ToggleWarning => self.toggle_warning(),
            Message::Desktop(message) => match self.desktop.update(message) {
                Some(desktop::Output::WallpaperInputChanged(path)) => {
                    println!("REACHED");
                    let config = Configuration::current();
                    if let Some(mut config) = config {
                        config.wallpaper = path;
                        match config.write() {
                            Ok(_) => self.error = "Wallpaper path updated".to_string(),
                            Err(err) => self.error = err.to_string(),
                        }
                    }
                }
                Some(desktop::Output::ColorSchemeChanged(color_scheme)) => {
                    let config = Configuration::current();
                    if let Some(mut config) = config {
                        config.color_scheme = color_scheme;
                        match config.write() {
                            Ok(_) => self.error = "Color scheme updated".to_string(),
                            Err(err) => self.error = err.to_string(),
                        }
                    }
                }
                Some(desktop::Output::OpenFilePicker) => {
                    let request = OpenFileRequest::default()
                        .directory(false)
                        .identifier(Some(WindowIdentifier::None))
                        .modal(true)
                        .title("Select your wallpaper")
                        .multiple(false)
                        .accept_label("Pick wallpaper");

                    return Command::perform(request.send(), |response| match response {
                        Ok(request) => match request.response() {
                            Ok(files) => {
                                let files: Vec<String> =
                                    files.uris().iter().map(|s| s.to_string()).collect();
                                return Message::HandlePickedFile(files);
                            }
                            Err(err) => Message::Error(err.to_string()),
                        },
                        Err(err) => Message::Error(err.to_string()),
                    });
                }
                None => (),
            },
            Message::SwitchColorScheme => {
                self.theme = match self.theme.theme_type {
                    ThemeType::Dark => Theme::light(),
                    ThemeType::Light => Theme::dark(),
                    ThemeType::HighContrastDark => Theme::light_hc(),
                    ThemeType::HighContrastLight => Theme::dark_hc(),
                }
            }
            Message::HandlePickedFile(files) => {
                let file = files.first().unwrap().replace("file://", "");
                self.desktop
                    .update(desktop::Message::WallpaperChanged(file.clone()));
                self.update(Message::Desktop(desktop::Message::WallpaperChanged(file)));
            }
            Message::Error(error) => eprintln!("{error}"),
            Message::Initialize => {
                let config = Configuration::new();
                match config.init() {
                    Ok(_) => self.error = "Configuration created".to_string(),
                    Err(err) => self.error = err.to_string(),
                }
                self.toggle_warning()
            }
        }
        Command::none()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
