pub mod desktop;
pub mod services;
pub mod settings;
pub mod welcome;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Page {
    #[default]
    Welcome,
    Desktop,
    Services,
    Settings,
}

impl Page {
    pub fn title(&self) -> &'static str {
        use Page::*;
        match self {
            Welcome => "Welcome",
            Desktop => "Desktop",
            Services => "Services",
            Settings => "Settings",
        }
    }

    pub fn icon_name(&self) -> &'static str {
        use Page::*;
        match self {
            Welcome => "face-smile-big-symbolic",
            Desktop => "computer-symbolic",
            Services => "network-server-symbolic",
            Settings => "preferences-system-symbolic",
        }
    }
}
