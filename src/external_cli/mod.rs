//! Wrappers and utilities to deal with external CLI applications, like `cargo`.

use std::process::{Command, ExitStatus, Output};

pub mod arg_builder;
pub(crate) mod cargo;
pub(crate) mod rustup;
pub(crate) mod wasm_bindgen;

pub trait CommandHelpers {
    fn ensure_status(&mut self) -> anyhow::Result<ExitStatus>;
    fn ensure_output(&mut self) -> anyhow::Result<Output>;
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

    /// Ensure that the command exits with a successful status code and return its output.
    fn ensure_output(&mut self) -> anyhow::Result<Output> {
        let output = self.output()?;
        let status = output.status;
        anyhow::ensure!(
            status.success(),
            "Command {} exited with status code {}",
            self.get_program().to_str().unwrap_or_default(),
            status
        );
        Ok(output)
    }
}
