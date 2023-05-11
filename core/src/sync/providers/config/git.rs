#[derive(Debug, Default)]
pub struct GitConfig {
    url: String,
    username: String,
    pub enabled: bool,
}
