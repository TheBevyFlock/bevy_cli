//! Wrappers and utilities to deal with external CLI applications, like `cargo`.

use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    process::{Command, ExitStatus, Output},
};

use cargo::install::AutoInstall;
use semver::VersionReq;
use tracing::{Level, debug, error, info, trace, warn};

pub mod arg_builder;
pub(crate) mod cargo;
#[cfg(feature = "rustup")]
pub(crate) mod rustup;
#[cfg(feature = "web")]
pub(crate) mod wasm_bindgen;
#[cfg(feature = "wasm-opt")]
pub(crate) mod wasm_opt;

struct Package {
    /// The name of the package.
    name: OsString,
    /// The version the package needs to match.
    version: VersionReq,
}

pub struct CommandExt {
    /// The package that the program can be installed with.
    package: Option<Package>,
    /// The command that is configured.
    inner: Command,
    /// The level to use for logging the command.
    log_level: Level,
}

impl CommandExt {
    /// Create a new command for the given program.
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self {
            package: None,
            inner: Command::new(program),
            log_level: Level::DEBUG,
        }
    }

    /// Define the package that allows installation of the program.
    pub fn require_package<S: AsRef<OsStr>>(&mut self, name: S, version: VersionReq) -> &mut Self {
        self.package = Some(Package {
            name: name.as_ref().to_owned(),
            version,
        });
        self
    }

    /// Check if the correct version of the program is installed and install if needed.
    ///
    /// The user will be prompted before the installation begins.
    ///
    /// Returns `true` if a new version was installed.
    fn install_if_needed(&self) -> anyhow::Result<bool> {
        if let Some(package) = &self.package {
            cargo::install::if_needed(
                self.inner.get_program(),
                &package.name,
                &package.version,
                // FIXME: Configure
                AutoInstall::Never,
            )
        } else {
            Ok(false)
        }
    }

    /// Add an argument to the program.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.inner.arg(arg.as_ref());
        self
    }

    /// Add multiple arguments to the program.
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.inner.args(args);
        self
    }

    /// Define at which level the execution of the program should be logged.
    pub fn log_level(&mut self, level: Level) -> &mut Self {
        self.log_level = level;
        self
    }

    /// Log the given message at the configured log level.
    fn log(&self, message: &str) {
        match self.log_level {
            Level::ERROR => error!("{}", message),
            Level::WARN => warn!("{}", message),
            Level::INFO => info!("{}", message),
            Level::DEBUG => debug!("{}", message),
            Level::TRACE => trace!("{}", message),
        }
    }

    /// Log the execution of the program.
    ///
    /// Returns the name of the program as String.
    fn log_execution(&self) -> String {
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

        program
    }

    /// Wrapper method around [`Command::status`].
    ///
    /// Executes a command as a child process, waiting for it to finish.
    /// If the command did not terminate successfully, an error containing the [`ExitStatus`] is
    /// returned.
    pub fn ensure_status(&mut self) -> anyhow::Result<ExitStatus> {
        let program = self.log_execution();
        let mut status = self.inner.status()?;

        if !status.success() && (self.install_if_needed()?) {
            // Retry command
            status = self.inner.status()?;
        }

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
        let program = self.log_execution();

        let mut output = self.inner.output()?;

        if !output.status.success() && (self.install_if_needed()?) {
            // Retry command
            output = self.inner.output()?;
        }

        let status = output.status;

        anyhow::ensure!(
            status.success(),
            "Command {} exited with status code {}",
            program,
            status
        );

        Ok(output)
    }
}
