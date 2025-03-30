//! Utilities for the `rustup` CLI tool.

use std::{env, ffi::OsString};

use anyhow::Context;
use dialoguer::Confirm;
use tracing::info;

use crate::external_cli::cargo::install::is_installed;

use super::CommandExt;

/// The rustup command can be customized via the `BEVY_CLI_RUSTUP` env
fn program() -> OsString {
    env::var_os("BEVY_CLI_RUSTUP").unwrap_or("rustup".into())
}

/// Given a target triple, determine if it is already installed.
fn is_target_installed(target: &str) -> bool {
    let output = CommandExt::new(program())
        .arg("target")
        .arg("list")
        .output();

    // Check if the target list has an entry like this:
    // <target_triple> (installed)
    let Ok(output) = output else { return false };
    let list = String::from_utf8_lossy(&output.stdout);
    list.lines()
        .any(|line| line.contains(target) && line.contains("(installed)"))
}

/// Install a compilation target, if it is not already installed.
pub(crate) fn install_target_if_needed(target: &str, silent: bool) -> anyhow::Result<()> {
    if is_installed(program()).is_none() {
        // `rustup` is not installed on the system
        // Don't perform the check and hope for the best!
        return Ok(());
    }

    if is_target_installed(target) {
        return Ok(());
    }

    if !silent {
        // Abort if the user doesn't want to install it
        if !Confirm::new()
            .with_prompt(format!(
                "Compilation target `{target}` is missing, should I install it for you?",
            ))
            .interact()
            .context(
                "failed to show interactive prompt, try using `--yes` to confirm automatically",
            )?
        {
            anyhow::bail!("User does not want to install target `{target}`.");
        }
    }

    info!("Installing missing target: `{target}`");

    CommandExt::new(program())
        .arg("target")
        .arg("add")
        .arg(target)
        .ensure_status()
        .context(format!("failed to install target `{target}`"))?;

    Ok(())
}
