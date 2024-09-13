use std::process::Command;

use super::PROGRAM;

/// Create a command to run `cargo build`.
pub(crate) fn command() -> Command {
    let mut command = Command::new(PROGRAM);
    command.arg("build");
    command
}
