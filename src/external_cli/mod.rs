//! Wrappers and utilities to deal with external CLI applications, like `cargo`.

use std::{
    borrow::Cow,
    ffi::OsStr,
    process::{Command as StdCommand, ExitStatus, Output},
};

use tracing::{debug, error, info, trace, warn, Level};

pub mod arg_builder;
pub(crate) mod cargo;
pub(crate) mod rustup;
pub(crate) mod wasm_bindgen;

pub struct Command {
    inner: StdCommand,
    log_level: Level,
}

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self {
            inner: StdCommand::new(program),
            log_level: Level::INFO,
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.inner.arg(arg.as_ref());
        self
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.inner.args(args);
        self
    }

    pub fn log_level(&mut self, level: Level) -> &mut Command {
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

    pub fn run(&mut self) -> anyhow::Result<ExitStatus> {
        let program = self.inner.get_program().to_str().unwrap_or_default();
        let args = self
            .inner
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<Cow<_>>>()
            .join(" ");

        self.log(format!("Running: `{program} {args}`").as_str());

        let status = self.inner.status()?;

        //TODO: streamline error handling
        anyhow::ensure!(
            status.success(),
            "Command {} exited with status code {}",
            self.inner.get_program().to_str().unwrap_or_default(),
            status
        );
        Ok(status)
    }

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
