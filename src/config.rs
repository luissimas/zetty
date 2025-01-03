use inquire::{formatter::StringFormatter, validator::Validation, Text};
use serde::{Deserialize, Serialize};
use std::{io::ErrorKind, path::PathBuf};

use anyhow::{Context, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub zettelkasten_directory: PathBuf,
    pub templates_directory: Option<String>,
    pub inbox_folder: String,
}
const APP_NAME: &str = "zetty";

impl Config {
    /// Loads or creates a new `Config` from the system data.
    ///
    /// If no config file exists at $XDG_HOME/zetty/config.toml, the user
    /// is prompted for the config values to create the config file.
    pub fn from_system() -> Result<Self> {
        let config_path = confy::get_configuration_file_path(APP_NAME, "config")?;
        let config_exists = config_path
            .try_exists()
            .context("Could not check user config")?;
        if !config_exists {
            let path_validator = |s: &str| match PathBuf::from(s).canonicalize() {
                Ok(path) => {
                    if !path.is_dir() {
                        Ok(Validation::Invalid("Path is not a directory".into()))
                    } else {
                        Ok(Validation::Valid)
                    }
                }
                Err(error) if error.kind() == ErrorKind::NotFound => {
                    Ok(Validation::Invalid("Path does not exist".into()))
                }
                Err(_err) => Ok(Validation::Invalid("Invalid path".into())),
            };
            let path_formatter: StringFormatter = &|s| {
                PathBuf::from(s)
                    .canonicalize()
                    .unwrap_or(PathBuf::from(s))
                    .to_str()
                    .unwrap()
                    .into()
            };
            let zettelkasten_directory = Text::new("Zettelkasten path:")
                .with_help_message(
                    "The absolute or relative path to the Zettelkasten's root directory",
                )
                .with_validator(path_validator)
                .with_formatter(path_formatter)
                .prompt()
                .context("Could not get user input")?;
            let templates_directory = inquire::Text::new("Templates directory (optional):")
                .with_validator(move |s: &str| {
                    if s.is_empty() {
                        Ok(Validation::Valid)
                    } else {
                        path_validator(s)
                    }
                })
                .with_formatter(path_formatter)
                .prompt()
                .context("Could not get user input")?;
            let zettelkasten_directory = PathBuf::from(zettelkasten_directory)
                .canonicalize()
                .context("Could not get absolute path")?;
            let templates_directory = if templates_directory.is_empty() {
                None
            } else {
                Some(templates_directory)
            };
            let config = Config {
                zettelkasten_directory,
                templates_directory,
                inbox_folder: "Inbox".into(),
            };
            confy::store(APP_NAME, "config", config).context("Could not store user config")?;
        };

        tracing::debug!(APP_NAME, "Fetching config");
        let config: Config =
            confy::load(APP_NAME, "config").context("Could not load user config")?;
        tracing::debug!(config = ?config, "Got config from system");
        Ok(config)
    }
}
