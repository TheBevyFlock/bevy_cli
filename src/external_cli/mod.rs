//! Wrappers and utilities to deal with external CLI applications, like `cargo`.

use std::process::{Command, ExitStatus};

pub mod arg_builder;
pub(crate) mod cargo;
#[cfg(feature = "web")]
pub(crate) mod rustup;
#[cfg(feature = "web")]
pub(crate) mod wasm_bindgen;

pub trait CommandHelpers {
    fn ensure_status(&mut self) -> anyhow::Result<ExitStatus>;
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
}
