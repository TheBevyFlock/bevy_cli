use std::{ffi::OsStr, process::exit, str::FromStr as _};

use anyhow::Context;
use dialoguer::Confirm;
use semver::{Version, VersionReq};

use crate::external_cli::CommandExt;

/// Check if the given program is installed on the system.
///
/// This assumes that the program offers a `--version` flag.
pub fn is_installed<P: AsRef<OsStr>>(program: P) -> Option<Vec<u8>> {
    CommandExt::new(program)
        .arg("--version")
        .output()
        .map(|output| output.stdout)
        .ok()
}

/// Checks if the program is installed and installs it if it isn't.
///
/// Returns `true` if the program needed to be installed.
pub(crate) fn if_needed(
    program: &str,
    package: &str,
    package_version: VersionReq,
    skip_prompts: bool,
) -> anyhow::Result<bool> {
    let mut prompt: Option<String> = None;

    if let Some(stdout) = is_installed(program) {
        if package_version == VersionReq::STAR {
            // If no `package_version` is specified and the program is installed,
            // there is nothing to do.
            return Ok(false);
        };

        if let Some(version) = parse_version(&stdout) {
            if package_version.matches(&version) {
                return Ok(false);
            }

            prompt = Some(format!(
                "`{program}@{version}` is installed, but \
                version `{package_version}` is required. Install and replace?"
            ));
        }
    }

    // Abort if the user doesn't want to install it
    if !skip_prompts
        && !Confirm::new()
            .with_prompt(
                prompt.unwrap_or_else(|| {
                    format!("`{program}` is missing, should I install it for you?")
                }),
            )
            .interact()
            .context(
                "failed to show interactive prompt, try using `--yes` to confirm automatically",
            )?
    {
        exit(1);
    }

    let mut cmd = CommandExt::new(super::program());
    cmd.arg("install").arg(package);

    if package_version != VersionReq::STAR {
        cmd.arg("--version").arg(package_version.to_string());
    }

    cmd.ensure_status()?;

    Ok(true)
}

/// Try to determine the package version from the output of a `--version` command.
fn parse_version(stdout: &[u8]) -> Option<Version> {
    String::from_utf8_lossy(stdout)
        .split_whitespace()
        .find_map(|word| Version::from_str(word).ok())
}
