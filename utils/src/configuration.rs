use std::{path::PathBuf, io::Write};

use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

use crate::color_scheme::ColorScheme;

pub const APP_NAME: &str = "symmetry";
pub const CONFIG_PATH: &str = "symmetry/configuration.toml";

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Configuration {
    pub color_scheme: ColorScheme
}

impl Configuration {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Creates a new instance from a path.
    pub fn from(path: PathBuf) -> Self {
        let data = std::fs::read_to_string(path).unwrap();
        let config: Configuration = toml::from_str(data.as_str()).unwrap();
        config
    }
    
    /// Creates a new file called `configuration.toml` and saves the current configuration to it.
    ///
    /// Example:
    /// ```rust
    /// use anyhow::Result;
    /// use symmetry_utils::color_scheme::ColorScheme;
    /// use symmetry_utils::configuration::Configuration;
    ///
    /// fn main() -> Result<()> {
    ///     let mut config = Configuration::new();
    ///     config.init()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn init(&self) -> Result<()> {
        let data_dir = dirs::data_dir().context("Data directory not available.")?;
        std::fs::create_dir_all(&data_dir.join(APP_NAME))?;
        std::fs::File::create(data_dir.join(CONFIG_PATH))?;
        Self::write(self)?;
        Ok(())
    }
    
    /// Writes the current configuration to `configuration.toml`.
    /// Example:
    /// ```rust
    /// use anyhow::Result;
    /// use symmetry_utils::color_scheme::ColorScheme;
    /// use symmetry_utils::configuration::Configuration;
    ///
    /// fn main() -> Result<()> {
    ///     let mut config = Configuration::new();
    ///     config.init()?;
    ///     config.color_scheme = ColorScheme::new();
    ///     config.write()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn write(&self) -> Result<()> {
        let config_dir = dirs::data_dir().unwrap().join(CONFIG_PATH);
        let config = toml::to_string(self)?;
        let mut file = std::fs::File::create(&config_dir)?;
        file.write_all(config.as_bytes())?;
        Ok(())
    }
}