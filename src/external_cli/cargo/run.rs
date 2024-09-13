use std::process::Command;

use super::PROGRAM;

/// Create a command to run `cargo run`.
pub(crate) fn command() -> Command {
    let mut command = Command::new(PROGRAM);
    command.arg("run");
    command
}
