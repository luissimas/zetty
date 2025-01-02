use assert_cmd::Command;

#[test]
fn help_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.arg("help").assert().success();

    Ok(())
}
