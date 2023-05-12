use crate::traits::desktop::{color_scheme::ColorSchemeManager, wallpaper::WallpaperManager};

pub struct Desktop;

impl WallpaperManager for Desktop {
    fn get_wallpaper() -> anyhow::Result<String> {
        todo!()
    }

    fn set_wallpaper() -> anyhow::Result<()> {
        todo!()
    }
}

impl ColorSchemeManager for Desktop {}
