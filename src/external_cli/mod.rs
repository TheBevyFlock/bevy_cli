//! Wrappers and utilities to deal with external CLI applications, like `cargo`.

use std::{
    borrow::Cow,
    ffi::OsStr,
    process::{Command, ExitStatus, Output},
};

use tracing::{debug, error, info, trace, warn, Level};

pub mod arg_builder;
pub(crate) mod cargo;
#[cfg(feature = "web")]
pub(crate) mod rustup;
#[cfg(feature = "web")]
pub(crate) mod wasm_bindgen;

pub struct CommandExt {
    inner: Command,
    log_level: Level,
}

impl CommandExt {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self {
            inner: Command::new(program),
            log_level: Level::DEBUG,
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut CommandExt {
        self.inner.arg(arg.as_ref());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut CommandExt
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.inner.args(args);
        self
    }

    pub fn log_level(&mut self, level: Level) -> &mut CommandExt {
        self.log_level = level;
        self
    }

    fn log(&self, message: &str) {
        match self.log_level {
            Level::ERROR => error!("{}", message),
            Level::WARN => warn!("{}", message),
            Level::INFO => info!("{}", message),
            Level::DEBUG => debug!("{}", message),
            Level::TRACE => trace!("{}", message),
        }
    }

    /// Wrapper method around [`Command::status`].
    ///
    /// Executes a command as a child process, waiting for it to finish.
    /// If the command did not terminate successfully, an error containing the [`ExitStatus`] is
    /// returned.
    pub fn ensure_status(&mut self) -> anyhow::Result<ExitStatus> {
        let program = self
            .inner
            .get_program()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let args = self
            .inner
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<Cow<_>>>()
            .join(" ");

        self.log(format!("Running: `{program} {args}`").as_str());

        let status = self.inner.status()?;

        anyhow::ensure!(
            status.success(),
            "Command {} exited with status code {}",
            program,
            status
        );

        Ok(status)
    }

    /// Wrapper method around [`Command::output()`].
    ///
    /// Executes the command as a child process, waiting for it to finish and collecting all of its
    /// output.
    pub fn output(&mut self) -> anyhow::Result<Output> {
        let program = self.inner.get_program().to_str().unwrap_or_default();
        let args = self
            .inner
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<Cow<_>>>()
            .join(" ");

        self.log(format!("Running: `{program} {args}`").as_str());

        Ok(self.inner.output()?)
    }
}
