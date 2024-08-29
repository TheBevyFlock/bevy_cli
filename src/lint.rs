use anyhow::{anyhow, ensure, Context};
use std::{env, process::Command};

pub fn lint() -> anyhow::Result<()> {
    // The `bevy` CLI lives in the same folder as `bevy_lint_driver`, so we can easily find it
    // using the path of the current executable.
    let mut driver_path = env::current_exe()
        .context("Failed to retrieve the path to the current executable.")?
        .parent()
        .ok_or(anyhow!("Path to file must have a parent."))?
        .join("bevy_lint_driver");

    #[cfg(target_os = "windows")]
    driver_path.set_extension("exe");

    ensure!(
        driver_path.exists(),
        "Could not find `bevy_lint_driver` at {driver_path:?}, please ensure it is installed!",
    );

    // Convert the local path to the absolute path. We don't want `rustc` getting
    // confused! `canonicalize()` requires for the path to exist, so we do it after the nice error
    // message.
    driver_path = driver_path.canonicalize()?;

    // Run `cargo check`.
    let status = Command::new("cargo")
        .arg("check")
        // This instructs `rustc` to call `bevy_lint_driver` instead of its default routine.
        // This lets us register custom lints.
        .env("RUSTC_WORKSPACE_WRAPPER", driver_path)
        .status()
        .context("Failed to spawn `cargo check`.")?;

    ensure!(status.success(), "Check failed with non-zero exit code.");

    Ok(())
}
