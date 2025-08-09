//! Utilities for the `rustup` CLI tool.

use std::{
    env,
    ffi::{OsStr, OsString},
};

use anyhow::Context;
use tracing::info;

use super::{CommandExt, cargo::install::AutoInstall};
use crate::external_cli::cargo::install::is_installed;

/// The rustup command can be customized via the `BEVY_CLI_RUSTUP` env
fn program() -> OsString {
    env::var_os("BEVY_CLI_RUSTUP").unwrap_or("rustup".into())
}

/// Given a target triple, determine if it is already installed.
fn is_target_installed(target: &str) -> bool {
    let output = CommandExt::new(program())
        .arg("target")
        .arg("list")
        .output(AutoInstall::Never);

    // Check if the target list has an entry like this:
    // <target_triple> (installed)
    let Ok(output) = output else { return false };
    let list = String::from_utf8_lossy(&output.stdout);
    list.lines()
        .any(|line| line.contains(target) && line.contains("(installed)"))
}

/// Install a compilation target, if it is not already installed.
///
/// Returns `true` if the target was missing and got installed.
pub(crate) fn install_target_if_needed<T: AsRef<OsStr>>(
    target: T,
    auto_install: AutoInstall,
) -> anyhow::Result<bool> {
    let target = target.as_ref();
    let target_str = &target.to_string_lossy();

    if is_installed(program()).is_none() {
        // `rustup` is not installed on the system
        // Don't perform the check and hope for the best!
        return Ok(false);
    }

    if is_target_installed(target_str) {
        return Ok(false);
    }

    if !auto_install.confirm(format!(
        "Compilation target `{target_str}` is missing, should I install it for you?",
    ))? {
        anyhow::bail!("user does not want to install target `{target_str}`.");
    }

    info!("installing missing target: `{target_str}`");

    CommandExt::new(program())
        .arg("target")
        .arg("add")
        .arg(target)
        .ensure_status(auto_install)
        .context(format!("failed to install target `{target_str}`"))?;

    Ok(true)
}
