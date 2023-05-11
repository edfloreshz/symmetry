pub mod repository_type;

use std::{io::Write, path::PathBuf};

use anyhow::{Context, Result};
use git2::Repository;
use git2_credentials::CredentialHandler;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::color_scheme::ColorScheme;

use self::repository_type::RepositoryType;

pub const APP_NAME: &str = "symmetry";
pub const CONFIG_PATH: &str = "symmetry/configuration.ron";

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Configuration {
    pub color_scheme: ColorScheme,
    pub wallpaper: String,
    pub repo: RepositoryType,
}

impl Configuration {
    /// Creates a new instance of this struct.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the current configuration.
    pub fn current() -> Option<Self> {
        let data_dir = dirs::data_dir()
            .context("Data directory not available.")
            .ok();
        if data_dir.is_none() {
            return None;
        }
        if let Ok(data) = std::fs::read_to_string(data_dir.unwrap().join(CONFIG_PATH)) {
            return match ron::from_str(data.as_str()) {
                Ok(config) => Some(config),
                Err(err) => {
                    eprintln!("{err}");
                    None
                }
            };
        }
        None
    }

    pub fn path() -> Result<PathBuf> {
        let path = dirs::data_dir()
            .context("Data directory not available.")?
            .join(CONFIG_PATH);
        Ok(path)
    }

    pub fn local_path() -> Result<PathBuf> {
        let path = dirs::data_dir()
            .context("Data directory not available.")?
            .join(APP_NAME);
        Ok(path)
    }

    /// Creates a new instance from a path.
    pub fn from(path: PathBuf) -> Self {
        let data = std::fs::read_to_string(path).unwrap();
        let config: Configuration = ron::from_str(data.as_str()).unwrap();
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
    pub fn init(&self, repository: String) -> Result<()> {
        let data_dir = dirs::data_dir().context("Data directory not available.")?;
        let app_config_dir = data_dir.join(APP_NAME);
        std::fs::create_dir_all(&app_config_dir)?;
        let repo = Repository::init(&app_config_dir)?;
        let mut callbacks = git2::RemoteCallbacks::new();
        let git_config = git2::Config::open_default()?;
        let mut credential_handler = CredentialHandler::new(git_config);
        callbacks.credentials(move |url, username, allowed| {
            credential_handler.try_next_credential(url, username, allowed)
        });
        let mut remote = repo.remote("origin", repository.as_str())?;
        remote.connect_auth(git2::Direction::Push, Some(callbacks), None)?;
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
        let config_file_dir = dirs::data_dir().unwrap().join(CONFIG_PATH);
        let config = ron::ser::to_string_pretty(self, PrettyConfig::new().struct_names(true))?;
        let mut file = std::fs::File::create(&config_file_dir)?;
        file.write_all(config.as_bytes())?;
        Ok(())
    }
}
