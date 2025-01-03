pub mod config;

use std::{env, fs::File, process::Command};

use anyhow::{Context, Result};
use config::Config;

pub fn create_note(config: &Config, name: &str) -> Result<()> {
    let path = config
        .zettelkasten_directory
        .join(&config.inbox_folder)
        .join(format!("{}.md", name));
    tracing::info!(path = path.to_str(), "Creating file");
    File::create_new(&path).context("Could not create note file")?;
    // TODO: put this in a config option, fallback to env and then to nano
    let editor = env::var("EDITOR").unwrap_or("nano".into());
    let mut command = Command::new(editor).arg(&path).spawn()?;
    command.wait()?;
    Ok(())
}
