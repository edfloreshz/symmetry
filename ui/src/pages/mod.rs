pub mod desktop;
pub mod settings;
pub mod welcome;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Page {
    #[default]
    Welcome,
    Desktop,
    Settings,
}

impl Page {
    pub fn title(&self) -> &'static str {
        use Page::*;
        match self {
            Welcome => "Welcome",
            Desktop => "Desktop",
            Settings => "Settings",
        }
    }

    pub fn icon_name(&self) -> &'static str {
        use Page::*;
        match self {
            Welcome => "face-smile-big-symbolic",
            Desktop => "computer-symbolic",
            Settings => "preferences-system-symbolic",
        }
    }
}
