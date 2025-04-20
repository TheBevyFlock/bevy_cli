use std::{ffi::OsStr, process::exit, str::FromStr as _};

use anyhow::Context;
use dialoguer::Confirm;
use semver::{Version, VersionReq};

use crate::external_cli::CommandExt;

/// Whether to automatically install packages.
#[derive(Debug, Clone, Copy)]
pub enum AutoInstall {
    /// Show a prompt to the user and ask them first before installing.
    AskUser,
    /// Always perform installation and don't show a prompt to the user.
    Always,
    /// Never perform installation and don't show a prompt to the user.
    Never,
}

impl AutoInstall {
    /// Confirm the installation with the auto install preferences.
    ///
    /// The given prompt is used when the user should be asked before installing.
    ///
    /// Returns `true` if the installation should be performed and `false` if not.
    /// An error is returned when an interactive prompt cannot be shown,
    /// e.g. when used in a non-interactive shell.
    pub fn confirm<S: Into<String>>(&self, prompt: S) -> anyhow::Result<bool> {
        match self {
            AutoInstall::AskUser => Confirm::new().with_prompt(prompt).interact().context(
                "failed to show interactive prompt, try using `--yes` to confirm automatically",
            ),
            AutoInstall::Always => Ok(true),
            AutoInstall::Never => Ok(false),
        }
    }
}

/// Check if the given program is installed on the system.
///
/// This assumes that the program offers a `--version` flag.
pub fn is_installed<P: AsRef<OsStr>>(program: P) -> Option<Vec<u8>> {
    CommandExt::new(program)
        .arg("--version")
        .output(AutoInstall::Never)
        .map(|output| output.stdout)
        .ok()
}

/// Checks if the program is installed and installs it if it isn't.
///
/// Returns `true` if the program needed to be installed.
pub(crate) fn if_needed<Pr: AsRef<OsStr>, Pa: AsRef<OsStr>>(
    program: Pr,
    package: Pa,
    package_version: &VersionReq,
    auto_install: AutoInstall,
) -> anyhow::Result<bool> {
    let mut prompt: Option<String> = None;
    let program = program.as_ref();

    if let Some(stdout) = is_installed(program) {
        if *package_version == VersionReq::STAR {
            // If no `package_version` is specified and the program is installed,
            // there is nothing to do.
            return Ok(false);
        };

        if let Some(version) = parse_version(&stdout) {
            if package_version.matches(&version) {
                return Ok(false);
            }

            prompt = Some(format!(
                "`{}@{version}` is installed, but \
                version `{package_version}` is required. Install and replace?",
                program.to_string_lossy()
            ));
        }
    }

    // Abort if the user doesn't want to install it
    if !auto_install.confirm(prompt.unwrap_or_else(|| {
        format!(
            "`{}` is missing, should I install it for you?",
            program.to_string_lossy()
        )
    }))? {
        exit(1);
    }

    let mut cmd = CommandExt::new(super::program());
    cmd.arg("install").arg(package);

    if *package_version != VersionReq::STAR {
        cmd.arg("--version").arg(package_version.to_string());
    }

    cmd.ensure_status(auto_install)?;

    Ok(true)
}

/// Try to determine the package version from the output of a `--version` command.
fn parse_version(stdout: &[u8]) -> Option<Version> {
    String::from_utf8_lossy(stdout)
        .split_whitespace()
        .find_map(|word| Version::from_str(word).ok())
}
