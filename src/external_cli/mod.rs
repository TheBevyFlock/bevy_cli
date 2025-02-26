//! Wrappers and utilities to deal with external CLI applications, like `cargo`.

use std::{
    borrow::Cow,
    process::{Command, ExitStatus},
};

use tracing::debug;

pub mod arg_builder;
pub(crate) mod cargo;
pub(crate) mod rustup;
pub(crate) mod wasm_bindgen;

pub trait CommandHelpers {
    fn ensure_status(&mut self) -> anyhow::Result<ExitStatus>;

    fn log_command(&mut self) -> &mut Command;
}

impl CommandHelpers for Command {
    /// Ensure that the command exits with a successful status code.
    fn ensure_status(&mut self) -> anyhow::Result<ExitStatus> {
        let status = self.status()?;
        anyhow::ensure!(
            status.success(),
            "Command {} exited with status code {}",
            self.get_program().to_str().unwrap_or_default(),
            status
        );
        Ok(status)
    }

    fn log_command(&mut self) -> &mut Command {
        let program = self.get_program().to_str().unwrap_or_default();
        let args = self
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<Cow<_>>>()
            .join(" ");

        debug!("{} {}", program, args);
        self
    }
}
