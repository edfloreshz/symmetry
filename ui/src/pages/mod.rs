pub mod desktop;
pub mod settings;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Page {
    Desktop,
    Settings,
}

impl Page {
    pub fn title(&self) -> &'static str {
        use Page::*;
        match self {
            Desktop => "Desktop",
            Settings => "Settings",
        }
    }

    pub fn icon_name(&self) -> &'static str {
        use Page::*;
        match self {
            Desktop => "system-users-symbolic",
            Settings => "document-properties-symbolic",
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::Desktop
    }
}
