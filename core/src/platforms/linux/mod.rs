use async_trait::async_trait;
use detect_desktop_environment::DesktopEnvironment;

use crate::traits::desktop::{color_scheme::ColorSchemeManager, wallpaper::WallpaperManager};

use self::kde::KdePlasma;

pub mod kde;

pub struct Desktop;

impl Desktop {
    pub async fn get_wallpaper() -> anyhow::Result<String> {
        match DesktopEnvironment::detect() {
            Some(desktop) => match desktop {
                DesktopEnvironment::Cinnamon => todo!(),
                DesktopEnvironment::Cosmic => todo!(),
                DesktopEnvironment::Enlightenment => todo!(),
                DesktopEnvironment::Gnome => todo!(),
                DesktopEnvironment::Kde => KdePlasma::get_wallpaper().await,
                DesktopEnvironment::Lxde => todo!(),
                DesktopEnvironment::Lxqt => todo!(),
                DesktopEnvironment::MacOs => todo!(),
                DesktopEnvironment::Mate => todo!(),
                DesktopEnvironment::Unity => todo!(),
                DesktopEnvironment::Windows => todo!(),
                DesktopEnvironment::Xfce => todo!(),
                _ => todo!(),
            },
            None => todo!(),
        }
    }

    pub async fn set_wallpaper() -> anyhow::Result<()> {
        match DesktopEnvironment::detect() {
            Some(desktop) => match desktop {
                DesktopEnvironment::Cinnamon => todo!(),
                DesktopEnvironment::Cosmic => todo!(),
                DesktopEnvironment::Enlightenment => todo!(),
                DesktopEnvironment::Gnome => todo!(),
                DesktopEnvironment::Kde => KdePlasma::set_wallpaper().await,
                DesktopEnvironment::Lxde => todo!(),
                DesktopEnvironment::Lxqt => todo!(),
                DesktopEnvironment::MacOs => todo!(),
                DesktopEnvironment::Mate => todo!(),
                DesktopEnvironment::Unity => todo!(),
                DesktopEnvironment::Windows => todo!(),
                DesktopEnvironment::Xfce => todo!(),
                _ => todo!(),
            },
            None => todo!(),
        }
    }
}

// #[async_trait]
// impl WallpaperManager for Desktop {
//     async fn get_wallpaper() -> anyhow::Result<String> {
//         match DesktopEnvironment::detect() {
//             Some(desktop) => match desktop {
//                 DesktopEnvironment::Cinnamon => todo!(),
//                 DesktopEnvironment::Cosmic => todo!(),
//                 DesktopEnvironment::Enlightenment => todo!(),
//                 DesktopEnvironment::Gnome => todo!(),
//                 DesktopEnvironment::Kde => KdePlasma::get_wallpaper().await,
//                 DesktopEnvironment::Lxde => todo!(),
//                 DesktopEnvironment::Lxqt => todo!(),
//                 DesktopEnvironment::MacOs => todo!(),
//                 DesktopEnvironment::Mate => todo!(),
//                 DesktopEnvironment::Unity => todo!(),
//                 DesktopEnvironment::Windows => todo!(),
//                 DesktopEnvironment::Xfce => todo!(),
//                 _ => todo!(),
//             },
//             None => todo!(),
//         }
//     }

//     async fn set_wallpaper() -> anyhow::Result<()> {
//         match DesktopEnvironment::detect() {
//             Some(desktop) => match desktop {
//                 DesktopEnvironment::Cinnamon => todo!(),
//                 DesktopEnvironment::Cosmic => todo!(),
//                 DesktopEnvironment::Enlightenment => todo!(),
//                 DesktopEnvironment::Gnome => todo!(),
//                 DesktopEnvironment::Kde => KdePlasma::set_wallpaper().await,
//                 DesktopEnvironment::Lxde => todo!(),
//                 DesktopEnvironment::Lxqt => todo!(),
//                 DesktopEnvironment::MacOs => todo!(),
//                 DesktopEnvironment::Mate => todo!(),
//                 DesktopEnvironment::Unity => todo!(),
//                 DesktopEnvironment::Windows => todo!(),
//                 DesktopEnvironment::Xfce => todo!(),
//                 _ => todo!(),
//             },
//             None => todo!(),
//         }
//     }
// }

impl ColorSchemeManager for Desktop {}
