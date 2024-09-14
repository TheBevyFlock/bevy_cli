//! Utilities for the `rustup` CLI tool.

#![expect(dead_code, reason = "Will be used for build/run commands")]

use std::process::{exit, Command};

use dialoguer::Confirm;

const PROGRAM: &str = "rustup";

/// Given a target triple, determine if it is already installed.
fn is_target_installed(target: &str) -> bool {
    let output = Command::new(PROGRAM).arg("target").arg("list").output();

    // Check if the target list has an entry like this:
    // <target_triple> (installed)
    let Ok(output) = output else { return false };
    let Ok(list) = String::from_utf8(output.stdout) else {
        return false;
    };
    list.lines()
        .any(|line| line.contains(target) && line.contains("(installed)"))
}

/// Install a compilation target, if it is not already installed.
pub(crate) fn install_target_if_needed(target: &str) -> anyhow::Result<()> {
    if is_target_installed(target) {
        return Ok(());
    }

    // Abort if the user doesn't want to install it
    if !Confirm::new()
        .with_prompt(format!(
            "Compilation target `{target}` is missing, should I install it for you?",
        ))
        .interact()?
    {
        exit(1);
    }

    let mut cmd = Command::new(PROGRAM);
    cmd.arg("target").arg("add").arg(target);

    anyhow::ensure!(
        cmd.output()?.status.success(),
        "Failed to install target `{target}`."
    );
    Ok(())
}
