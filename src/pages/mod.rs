pub mod welcome;
pub mod settings;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Page {
    Welcome,
    Settings,
}

impl Page {
    pub fn title(&self) -> &'static str {
        use Page::*;
        match self {
            Welcome => "Welcome",
            Settings => "Settings",
        }
    }

    pub fn icon_name(&self) -> &'static str {
        use Page::*;
        match self {
            Welcome => "system-users-symbolic",
            Settings => "document-properties-symbolic",
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::Welcome
    }
}