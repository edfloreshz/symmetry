use std::sync::atomic::{AtomicU32, Ordering};

use crate::components::header_bar::header;
use crate::pages::{desktop, settings, welcome, Page};
use cosmic::iced::Application;
use cosmic::iced_winit::widget::horizontal_space;
use cosmic::iced_winit::window::{self, close, drag, minimize, toggle_maximize};
use cosmic::iced_winit::{column, row, subscription, Command};
use cosmic::theme::ThemeType;
use cosmic::widget::segmented_button::{self, Entity, SingleSelectModel};
use cosmic::widget::{nav_bar, text, IconSource};
use cosmic::{iced, Element, Theme};
use iced::Length;
use symmetry_core::configuration::repository_type::RepositoryType;
use symmetry_core::configuration::Configuration;
use symmetry_core::sync;
use symmetry_core::sync::providers::git::GitSync;
use symmetry_core::traits::synchronization::Synchronization;

static WINDOW_WIDTH: AtomicU32 = AtomicU32::new(1000);
const BREAK_POINT: u32 = 700;

type SyncProvider =
    Box<dyn Synchronization<Status = sync::status::Status, Message = sync::message::Message>>;

pub struct Symmetry {
    theme: Theme,
    nav_bar: SingleSelectModel,
    nav_id_to_page: segmented_button::SecondaryMap<Page>,
    page: Page,
    error: String,
    show_warning: bool,
    welcome: crate::pages::welcome::State,
    desktop: crate::pages::desktop::State,
    settings: crate::pages::settings::State,
    sync: Option<SyncProvider>,
}

impl Default for Symmetry {
    fn default() -> Self {
        let sync = refresh_sync_provider();

        Self {
            theme: Default::default(),
            nav_bar: Default::default(),
            nav_id_to_page: Default::default(),
            page: Default::default(),
            error: Default::default(),
            show_warning: Default::default(),
            welcome: Default::default(),
            desktop: Default::default(),
            settings: Default::default(),
            sync,
        }
    }
}

pub fn refresh_sync_provider() -> Option<SyncProvider> {
    let sync: Option<SyncProvider> = if let Some(configuration) = Configuration::current() {
        match configuration.repo {
            RepositoryType::Git => Some(Box::new(GitSync::new())),
        }
    } else {
        None
    };
    sync
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
        row!(text(page.title()).size(30), horizontal_space(Length::Fill)).into()
    }

    /// Toggles the warning.
    fn toggle_warning(&mut self) {
        self.show_warning = !self.show_warning
    }

    fn is_condensed(&self) -> bool {
        WINDOW_WIDTH.load(Ordering::Relaxed) < BREAK_POINT
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    CondensedViewToggle,
    Welcome(welcome::Message),
    Desktop(desktop::Message),
    Settings(settings::Message),
    HandlePickedFile(Vec<String>),
    NavBar(Entity),
    Error(String),
    SwitchColorScheme,
    ToggleWarning,
    Maximize,
    Minimize,
    Close,
    Drag,
    Sync,
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

    fn subscription(&self) -> cosmic::iced_winit::Subscription<Self::Message> {
        let window_break = subscription::events_with(|event, _| match event {
            cosmic::iced::Event::Window(
                _window_id,
                window::Event::Resized { width, height: _ },
            ) => {
                let old_width = WINDOW_WIDTH.load(Ordering::Relaxed);
                if old_width == 0
                    || old_width < BREAK_POINT && width > BREAK_POINT
                    || old_width > BREAK_POINT && width < BREAK_POINT
                {
                    WINDOW_WIDTH.store(width, Ordering::Relaxed);
                    Some(())
                } else {
                    None
                }
            }
            _ => None,
        });

        cosmic::iced_winit::Subscription::batch(vec![
            window_break.map(|_| Message::CondensedViewToggle)
        ])
    }

    fn view(&self) -> Element<Message> {
        let header = header(self.title());
        let nav_bar: Element<_> = nav_bar(&self.nav_bar, Message::NavBar)
            .max_width(200)
            .into();
        let page: Element<_> = match self.page {
            Page::Welcome => self.welcome.view().map(Message::Welcome),
            Page::Desktop => self.desktop.view(&self).map(Message::Desktop),
            Page::Settings => self.settings.view(&self).map(Message::Settings),
        };

        let mut widgets: Vec<Element<_>> = vec![
            horizontal_space(Length::Fill).into(),
            page,
            horizontal_space(Length::Fill).into(),
        ];

        if !self.is_condensed() {
            widgets.insert(0, nav_bar);
        }

        let content = cosmic::iced::widget::row(widgets)
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

    fn update(&mut self, message: Self::Message) -> cosmic::iced::Command<Self::Message> {
        match message {
            Message::Drag => return drag(window::Id::new(0)),
            Message::Close => return close(window::Id::new(0)),
            Message::Minimize => return minimize(window::Id::new(0), true),
            Message::Maximize => return toggle_maximize(window::Id::new(0)),
            Message::ToggleWarning => self.toggle_warning(),
            Message::NavBar(key) => {
                if let Some(page) = self.nav_id_to_page.get(key).copied() {
                    self.nav_bar.activate(key);
                    self.page(page);
                }
            }
            Message::Welcome(message) => match self.welcome.update(message) {
                Some(welcome::Output::Message(message)) => {
                    self.update(Message::Error(message));
                }
                Some(welcome::Output::Error(error)) => {
                    self.update(Message::Error(error));
                }
                None => (),
            },
            Message::Desktop(message) => match self.desktop.update(message) {
                Some(desktop::Output::OpenFilePicker(request)) => {
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
                Some(desktop::Output::Error(msg)) => {
                    self.update(Message::Error(msg));
                }
                Some(desktop::Output::Message(msg)) => {
                    self.update(Message::Error(msg));
                }
                None => (),
            },
            Message::Settings(message) => match self.settings.update(message) {
                Some(settings::Output::ChangeTheme(theme)) => self.theme = theme,
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
            Message::Error(error) => {
                self.error = error;
                if !self.show_warning {
                    self.toggle_warning()
                }
            }
            Message::CondensedViewToggle => {}
            Message::Sync => {
                if let Some(sync) = self.sync.as_ref() {
                    match sync.sync() {
                        Ok(status) => match status {
                            sync::status::Status::UpToDate => {
                                self.update(Message::Error("Already up to date".into()));
                            }
                            sync::status::Status::ChangesUploaded => {
                                self.update(Message::Error("Successfully synchronized".into()));
                            }
                            sync::status::Status::NewChangesDetected => {
                                // todo!("Check if there are conflicts, if not, pull changes");
                                match sync.handle(sync::message::Message::Update) {
                                    Ok(_) => {
                                        self.update(Message::Error(
                                            "Latest changes downloaded".into(),
                                        ));
                                    }
                                    Err(err) => {
                                        self.update(Message::Error(
                                            format!("An error ocurred while trying to get the latest changes: {}.", err)
                                        ));
                                    }
                                }
                            }
                            sync::status::Status::RepoNotConfigured => {
                                self.update(Message::Error(
                                    "The repository has not been configured".into(),
                                ));
                            }
                        },
                        Err(err) => {
                            self.update(Message::Error(err.to_string()));
                        }
                    }
                }
            }
        }
        Command::none()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
