//! Helper functions to run `bevy_lint`.

use anyhow::{Context, anyhow, ensure};
use std::{env, path::PathBuf};

use crate::external_cli::{CommandExt, cargo::install::AutoInstall};

/// Runs `bevy_lint`, if it is installed, with the given arguments.
///
/// Calling `lint(vec!["--workspace"])` is equivalent to calling `bevy_lint --workspace` in the
/// terminal. This will run [`find_bevy_lint()`] to locate `bevy_lint`.
pub fn lint(args: Vec<String>) -> anyhow::Result<()> {
    let bevy_lint_path = find_bevy_lint()?;

    let status = CommandExt::new(bevy_lint_path)
        .args(args)
        .ensure_status(AutoInstall::Never)?;

    ensure!(
        status.success(),
        "`bevy_lint` exited with a non-zero exit code."
    );

    Ok(())
}

/// Tries the find the path to `bevy_lint`, if it is installed.
///
/// The current strategy will find a file named `bevy_lint(.exe)` within the same directory as the
/// current executable, which is usually `~/.cargo/bin` or `target/debug`. It will **not** search
/// the `PATH`.
pub fn find_bevy_lint() -> anyhow::Result<PathBuf> {
    let mut bevy_lint_path = env::current_exe()
        .context("Failed to retrieve the path to the current executable.")?
        .parent()
        .ok_or(anyhow!("Path to file must have a parent."))?
        .join("bevy_lint");

    #[cfg(target_os = "windows")]
    bevy_lint_path.set_extension("exe");

    if cfg!(debug_assertions) {
        ensure!(
            bevy_lint_path.exists(),
            "`bevy_lint` could not be found at {}. Please run `cargo build -p bevy_lint` first!",
            bevy_lint_path.display(),
        );
    } else {
        ensure!(
            bevy_lint_path.exists(),
            "`bevy_lint` could not be found at {}. Please follow the instructions at <https://thebevyflock.github.io/bevy_cli/bevy_lint/#installation> to install it.",
            bevy_lint_path.display(),
        );
    }

    bevy_lint_path = bevy_lint_path.canonicalize()?;

    Ok(bevy_lint_path)
}
