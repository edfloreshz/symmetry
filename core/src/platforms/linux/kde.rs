use crate::{
    configuration::Configuration,
    traits::desktop::{color_scheme::ColorSchemeManager, wallpaper::WallpaperManager},
};

use anyhow::{Context, Result};
use async_trait::async_trait;
use ini::ini;
use zbus::{dbus_proxy, Connection};

#[dbus_proxy(
    interface = "org.kde.PlasmaShell",
    default_service = "org.kde.plasmashell",
    default_path = "/PlasmaShell"
)]
trait PlasmaShell {
    #[dbus_proxy(name = "evaluateScript")]
    fn evaluate_script(&self, script: &str) -> Result<String>;
}

pub struct KdePlasma;

#[async_trait]
impl WallpaperManager for KdePlasma {
    async fn get_wallpaper() -> anyhow::Result<String> {
        let config_dir = dirs::config_dir().unwrap();

        let desktop_config = config_dir.join("plasma-org.kde.plasma.desktop-appletsrc");

        let config = ini!(desktop_config.to_string_lossy().to_string().as_str());

        if let Some(containments) = config.get("containments][1][wallpaper][org.kde.image][general")
        {
            if let Some(wallpaper) = containments.get("image") {
                if let Some(wallpaper) = wallpaper {
                    return Ok(wallpaper.clone());
                }
            }
        }

        Ok(String::new())
    }

    async fn set_wallpaper() -> anyhow::Result<()> {
        let config = Configuration::current().context("context")?;
        let connection = Connection::session().await?;
        let proxy = PlasmaShellProxy::new(&connection).await?;

        let script = format!(
            r#"var allDesktops = desktops();for (i=0;i<allDesktops.length;i++) {{ d = allDesktops[i]; d.wallpaperPlugin = "org.kde.image"; d.currentConfigGroup = Array("Wallpaper", "org.kde.image", "General"); d.writeConfig("Image", "{}"); }}"#,
            config.wallpaper
        );

        proxy.evaluate_script(script.as_str()).await?;

        println!("Wallpaper set successfully!");

        Ok(())
    }
}

impl ColorSchemeManager for KdePlasma {}
