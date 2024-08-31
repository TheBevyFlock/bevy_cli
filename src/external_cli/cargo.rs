use std::process::{exit, Command};

use dialoguer::Confirm;

const PROGRAM: &str = "cargo";

/// Create a new command to execute `cargo`.
fn command() -> Command {
    Command::new(PROGRAM)
}

/// Create a command to run `cargo build`.
pub(crate) fn build() -> Command {
    let mut command = command();
    command.arg("build");
    command
}

/// Create a command to run `cargo run`.
pub(crate) fn run() -> Command {
    let mut command = command();
    command.arg("run");
    command
}

/// Check if the given program is installed on the system.
///
/// This assumes that the program offers a `--version` flag.
fn is_installed(program: &str) -> bool {
    let output = Command::new(program).arg("--version").output();

    if let Ok(output) = output {
        output.status.success()
    } else {
        false
    }
}

/// Checks if the program is installed and installs it if it isn't.
///
/// Returns `true` if the program needed to be installed.
pub(crate) fn install_if_needed(
    program: &str,
    package: &str,
    ask_user: bool,
    hidden: bool,
) -> anyhow::Result<bool> {
    if is_installed(program) {
        return Ok(false);
    }

    // Abort if the user doesn't want to install it
    if ask_user
        && !Confirm::new()
            .with_prompt(format!(
                "`{program}` is missing, should I install it for you?"
            ))
            .interact()?
    {
        exit(1);
    }

    let mut cmd = command();
    cmd.arg("install").arg(package);

    let status = if hidden {
        cmd.output()?.status
    } else {
        cmd.status()?
    };

    if !status.success() {
        Err(anyhow::anyhow!("Failed to install `{program}`."))
    } else {
        Ok(true)
    }
}
