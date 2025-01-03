pub mod config;

use std::{env, process::Command};

use anyhow::Result;
use config::Config;

pub fn open_note(config: &Config, name: &str) -> Result<()> {
    let path = config
        .zettelkasten_directory
        .join(&config.inbox_folder)
        .join(format!("{}.md", name));
    // TODO: put this in a config option, fallback to env and then to nano
    let editor = env::var("EDITOR").unwrap_or("nano".into());
    tracing::info!(path = path.to_str(), editor, "Opening file in editor");
    let mut command = Command::new(editor).arg(&path).spawn()?;
    command.wait()?;
    Ok(())
}
