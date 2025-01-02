use std::{env, fs::File, process::Command};

use anyhow::{Context, Result};

pub fn create_note(name: &str) -> Result<()> {
    tracing::info!(name, "Opening file");
    File::create_new(name).context("Could not create note file")?;
    let editor = env::var("EDITOR").unwrap_or("vi".into());
    let mut command = Command::new(editor).arg(name).spawn()?;
    command.wait()?;
    Ok(())
}
