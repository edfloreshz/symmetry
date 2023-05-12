use crate::traits::desktop::{color_scheme::ColorSchemeManager, wallpaper::WallpaperManager};

use anyhow::Context;
use async_trait::async_trait;
use configparser::ini::Ini;

pub struct KdePlasma;

#[async_trait]
impl WallpaperManager for KdePlasma {
    async fn get_wallpaper() -> anyhow::Result<String> {
        let home_dir = dirs::home_dir().unwrap();

        let config_path = home_dir.join(".config/plasma-org.kde.plasma.desktop-appletsrc");

        let mut config = Ini::new();
        config
            .read(config_path.to_string_lossy().to_string())
            .unwrap();

        // Retrieve the wallpaper image path
        let wallpaper_config = config
            .get("Containments", "1")
            .context("Failed to get the key")?;

        let image_line = wallpaper_config
            .lines()
            .find(|line| line.starts_with("Image="))
            .unwrap_or("");

        let image_path = image_line.trim_start_matches("Image=").to_owned();

        Ok(image_path)
    }

    async fn set_wallpaper() -> anyhow::Result<()> {
        todo!()
    }
}

impl ColorSchemeManager for KdePlasma {}
