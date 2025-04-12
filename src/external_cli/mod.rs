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
    /// The compilation target that's needed to run the command.
    target: Option<OsString>,
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
            target: None,
            inner: Command::new(program),
            log_level: Level::DEBUG,
        }
    }

    /// Define the package that allows installation of the program.
    ///
    /// If the command fails and the package is missing,
    /// it can be installed automatically via `cargo install`.
    pub fn require_package<S: AsRef<OsStr>>(&mut self, name: S, version: VersionReq) -> &mut Self {
        self.package = Some(Package {
            name: name.as_ref().to_owned(),
            version,
        });
        self
    }

    /// Define the compilation target that's required to run the command.
    ///
    /// If the command fails and the target is missing,
    /// it can be installed automatically via `rustup`.
    pub fn maybe_require_target<S: AsRef<OsStr>>(&mut self, target: Option<S>) -> &mut Self {
        if let Some(target) = target {
            self.target = Some(target.as_ref().to_owned());
        } else {
            self.target = None;
        }
        self
    }

    /// Check if the correct version of the program is installed and install if needed.
    ///
    /// The user will be prompted before the installation begins.
    ///
    /// Returns `true` if a new version was installed.
    fn install_package_if_needed(&self, auto_install: AutoInstall) -> anyhow::Result<bool> {
        if let Some(package) = &self.package {
            cargo::install::if_needed(
                self.inner.get_program(),
                &package.name,
                &package.version,
                auto_install,
            )
        } else {
            Ok(false)
        }
    }

    /// Check if the needed compile targets are installed and install if needed.
    ///
    /// This requires the `rustup` feature to be enabled, otherwise it's a noop.
    fn install_target_if_needed(
        &self,
        // Only needed with the `rustup` feature
        #[allow(unused_variables)] auto_install: AutoInstall,
    ) -> anyhow::Result<bool> {
        #[cfg(feature = "rustup")]
        if let Some(target) = &self.target {
            rustup::install_target_if_needed(target, auto_install)
        } else {
            Ok(false)
        }

        #[cfg(not(feature = "rustup"))]
        Ok(false)
    }

    /// Try to fix erroneous configuration before retrying the command.
    ///
    /// Returns `true` if a fix was applied and retrying might work.
    fn try_fix_before_retry(&self, auto_install: AutoInstall) -> anyhow::Result<bool> {
        let mut retry = false;

        if self.package.is_some() || self.target.is_some() {
            tracing::warn!(
                "Failed to run {}, trying to find automatic fix...",
                self.inner.get_program().to_string_lossy()
            )
        }

        if self.install_package_if_needed(auto_install)? {
            retry = true;
        }
        if self.install_target_if_needed(auto_install)? {
            retry = true;
        }

        Ok(retry)
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
    pub fn ensure_status(&mut self, auto_install: AutoInstall) -> anyhow::Result<ExitStatus> {
        let program = self.log_execution();
        let mut status = self.inner.status()?;

        if !status.success() && (self.try_fix_before_retry(auto_install)?) {
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
    pub fn output(&mut self, auto_install: AutoInstall) -> anyhow::Result<Output> {
        let program = self.log_execution();

        let mut output = self.inner.output()?;

        if !output.status.success() && (self.try_fix_before_retry(auto_install)?) {
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
