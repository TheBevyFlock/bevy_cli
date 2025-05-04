use std::{ffi::OsStr, process::exit, str::FromStr as _};

use anyhow::Context;
use dialoguer::Confirm;
use semver::Version;

use crate::external_cli::{CommandExt, Package};

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
    pub fn confirm<S: Into<String>>(self, prompt: S) -> anyhow::Result<bool> {
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
pub(crate) fn if_needed<Pr: AsRef<OsStr>>(
    program: Pr,
    package: &Package,
    auto_install: AutoInstall,
) -> anyhow::Result<bool> {
    let mut prompt: Option<String> = None;
    let program = program.as_ref();

    if let Some(stdout) = is_installed(program) {
        match &package.version {
            None => {
                // If no `package_version` is specified and the program is installed,
                // there is nothing to do.
                return Ok(false);
            }
            Some(package_version) => {
                if let Some(version) = parse_version(&stdout) {
                    if package
                        .version
                        .as_ref()
                        .is_some_and(|v| v.matches(&version))
                    {
                        return Ok(false);
                    }

                    prompt = Some(format!(
                        "`{}@{version}` is installed, but \
                version `{package_version}` is required. Install and replace?",
                        program.to_string_lossy()
                    ));
                }
            }
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

    // If the program needs to be installed with a specific toolchain, switch to `rustup run
    // <toolchain> cargo install`
    if let Some(toolchain) = &package.required_toolchain {
        cmd = CommandExt::new("rustup");
        cmd.arg("run").arg(toolchain).arg(super::program());
    }

    match &package.git {
        // Install from Git
        Some(git_url) => {
            cmd.arg("install").arg("--git").arg(git_url);

            // Install either from tag or branch, if none are present install from main branch.
            // If both a tag and a branch is passed, return an error.
            match (&package.tag, &package.branch) {
                (None, None) => cmd.arg("--branch").arg("main"),
                (None, Some(branch)) => cmd.arg("--branch").arg(branch),
                (Some(tag), None) => cmd.arg("--tag").arg(tag),
                (Some(_), Some(_)) => {
                    anyhow::bail!("cannot install from branch and tag at the same time, choose one")
                }
            };

            cmd.arg("--locked").arg(&package.name);
        }
        // Install the package from crates.io
        None => {
            cmd.arg("install").arg(package.name.clone());

            if let Some(package_version) = &package.version {
                cmd.arg("--version").arg(package_version.to_string());
            }
        }
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
