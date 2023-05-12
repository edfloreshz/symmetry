use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait WallpaperManager {
    async fn get_wallpaper() -> Result<String>;
    async fn set_wallpaper() -> Result<()>;
}
