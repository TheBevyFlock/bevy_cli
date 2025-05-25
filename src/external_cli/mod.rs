//! Wrappers and utilities to deal with external CLI applications, like `cargo`.

use std::{
    ffi::{OsStr, OsString},
    fmt::Display,
    process::{Command, ExitStatus, Output},
};

use cargo::install::AutoInstall;
use semver::VersionReq;
use tracing::{Level, debug, error, info, trace, warn};

pub(crate) mod arg_builder;
pub(crate) mod cargo;
#[cfg(feature = "rustup")]
pub(crate) mod rustup;
#[cfg(feature = "web")]
pub(crate) mod wasm_bindgen;
#[cfg(feature = "web")]
pub(crate) mod wasm_opt;

#[derive(Debug, Default)]
pub(crate) struct Package {
    /// The name of the package.
    pub(crate) name: OsString,
    /// A specific [`VersionReq`]
    pub(crate) version: Option<VersionReq>,
    /// The toolchain that should be used to install this package
    pub(crate) required_toolchain: Option<String>,
    /// Git URL to install the specified package from
    pub(crate) git: Option<String>,
    /// Branch to use when installing from git
    pub(crate) branch: Option<String>,
    /// Tag to use when installing from git
    pub(crate) tag: Option<String>,
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
    #[cfg(any(feature = "rustup", feature = "web"))]
    pub(crate) fn require_package(&mut self, package: Package) -> &mut Self {
        self.package = Some(package);
        self
    }

    /// Define the compilation target that's required to run the command.
    ///
    /// If the command fails and the target is missing,
    /// it can be installed automatically via `rustup`.
    #[cfg(feature = "web")]
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
            // If the package that needs to be installed if needed required a specific toolchain,
            // check first that the toolchain is installed / install it before checking if the
            // package needs to be installed.
            #[cfg(feature = "rustup")]
            if let Some(toolchain) = &package.required_toolchain {
                rustup::install_toolchain_if_needed(toolchain, auto_install)?;
            }
            cargo::install::if_needed(self.inner.get_program(), package, auto_install)
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
                "failed to run {}, trying to find automatic fix...",
                self.inner.get_program().to_string_lossy()
            );
        }

        if self.install_package_if_needed(auto_install)?
            || self.install_target_if_needed(auto_install)?
        {
            retry = true;
        }

        Ok(retry)
    }

    /// Ensure that the status is successful.
    /// If not, try to fix the issue automatically.
    fn success_or_try_fix<Err>(
        &self,
        status: &Result<ExitStatus, Err>,
        auto_install: AutoInstall,
    ) -> anyhow::Result<bool> {
        match status {
            Ok(status) if status.success() => Ok(false),
            _ => self.try_fix_before_retry(auto_install),
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

    /// Inserts or updates an explicit environment variable mapping if some value is provided.
    /// Wrapper method around [`Command::env`] that allows to pass an `Option<value>` instead.
    pub fn env<K, V>(&mut self, key: K, val: Option<V>) -> &mut CommandExt
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        if let Some(val) = val {
            self.inner.env(key, val);
        }
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
    fn log_execution(&self) {
        let envs = self
            .inner
            .get_envs()
            .filter_map(|(key, val)| {
                let key = key.to_string_lossy();
                val.map(|v| v.to_string_lossy())
                    .map(|value| format!("{key}={value}"))
            })
            .collect::<Vec<_>>()
            .join(",");

        self.log(format!("running: `{self}`").as_str());

        if !envs.is_empty() {
            self.log(&format!("with env: {envs}"));
        }
    }

    /// Wrapper method around [`Command::status`].
    ///
    /// Executes a command as a child process, waiting for it to finish.
    /// If the command did not terminate successfully, an error containing the [`ExitStatus`] is
    /// returned.
    pub fn ensure_status(&mut self, auto_install: AutoInstall) -> anyhow::Result<ExitStatus> {
        self.log_execution();
        let mut status = self.inner.status();

        if self.success_or_try_fix(&status, auto_install)? {
            // Retry command
            status = self.inner.status();
        }

        let status = status?;

        anyhow::ensure!(
            status.success(),
            "command `{self}` exited with status code {}",
            status
        );

        Ok(status)
    }

    /// Wrapper method around [`Command::output()`].
    ///
    /// Executes the command as a child process, waiting for it to finish and collecting all of its
    /// output.
    pub fn output(&mut self, auto_install: AutoInstall) -> anyhow::Result<Output> {
        self.log_execution();

        let mut output = self.inner.output();

        if self.success_or_try_fix(&output.as_ref().map(|output| output.status), auto_install)? {
            // Retry command
            output = self.inner.output();
        }

        let output = output?;

        anyhow::ensure!(
            output.status.success(),
            "command `{self}` exited with status code {}",
            output.status
        );

        Ok(output)
    }
}

impl Display for CommandExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let program = self.inner.get_program().to_string_lossy();

        let args = self
            .inner
            .get_args()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");

        if args.is_empty() {
            write!(f, "{program}")
        } else {
            write!(f, "{program} {args}")
        }
    }
}
