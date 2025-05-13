//! Utilities for the `rustup` CLI tool.

use std::{
    env,
    ffi::{OsStr, OsString},
};

use anyhow::Context;
use tracing::info;

use crate::external_cli::cargo::install::is_installed;

use super::{CommandExt, cargo::install::AutoInstall};

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

/// Install a rust toolchain, if it is not already installed.
///
/// Returns `true` if the toolchain was missing and got installed.
pub(crate) fn install_toolchain_if_needed(
    toolchain: &str,
    auto_install: AutoInstall,
) -> anyhow::Result<()> {
    let rustup_list_toolchain = CommandExt::new(program())
        .arg("toolchain")
        .arg("list")
        .output(auto_install)
        .context("failed to list installed toolchains with rustup list toolchain")?
        .stdout;

    // using a `let` binding to create a longer lived value
    let installed_toolchains = String::from_utf8_lossy(&rustup_list_toolchain);

    // For more information on the standard toolchain names see: https://rust-lang.github.io/rustup/concepts/toolchains.html#toolchain-specification
    // in practice, this looks like this
    // ❯ rustup toolchain list
    // stable-aarch64-apple-darwin (active, default)
    // nightly-2024-11-14-aarch64-apple-darwin
    // nightly-2024-11-28-aarch64-apple-darwin
    let installed_toolchains = installed_toolchains
        .lines()
        .map(str::trim)
        .collect::<Vec<&str>>();

    // check if the desired toolchain is installed
    if installed_toolchains
        .iter()
        // ignore <host> part of the toolchain name
        .any(|installed_toolchain| installed_toolchain.starts_with(toolchain))
    {
        return Ok(());
    }

    if !auto_install.confirm(format!(
        "rust toolchain `{toolchain}` is missing, should I install it for you?",
    ))? {
        anyhow::bail!("User does not want to install rust toolchain `{toolchain}`.");
    }

    info!("Installing missing rust toolchain: `{toolchain}`");

    CommandExt::new("rustup")
        .arg("toolchain")
        .arg("install")
        .arg(toolchain)
        .args([
            "--component",
            "rustc-dev",
            "--component",
            "llvm-tools-preview",
        ])
        .ensure_status(auto_install)
        .context(format!("failed to install toolchain `{toolchain}`"))?;

    Ok(())
}
