//! Utilities for the `rustup` CLI tool.

use std::{env, ffi::OsString, path::Path};

use anyhow::Context;
use dialoguer::Confirm;
use tracing::info;

use super::CommandExt;

/// The rustup command can be customized via the `BEVY_CLI_RUSTUP` env
fn program() -> OsString {
    env::var_os("BEVY_CLI_RUSTUP").unwrap_or("rustup".into())
}

/// The rustc command can be customized via the `BEVY_CLI_RUSTC` env
fn rustc_program() -> OsString {
    env::var_os("BEVY_CLI_RUSTC").unwrap_or("rustc".into())
}

/// Given a target triple, determine if it is already installed.
fn is_target_installed(target: &str) -> bool {
    let output = CommandExt::new(rustc_program())
        .arg("--print")
        .arg("sysroot")
        .output();

    let Ok(output) = output else { return false };

    let sysroot = String::from_utf8_lossy(&output.stdout);

    Path::new(&format!(
        "{}/lib/rustlib/{target}",
        sysroot.strip_suffix("\n").unwrap_or(&sysroot)
    ))
    .is_dir()
}

/// Install a compilation target, if it is not already installed.
pub(crate) fn install_target_if_needed(target: &str, silent: bool) -> anyhow::Result<()> {
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

    let mut cmd = CommandExt::new(program());
    cmd.arg("target").arg("add").arg(target).ensure_status()?;
    Ok(())
}
