use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct GitConfig {
    pub url: String,
    pub username: String,
    pub enabled: bool,
}
