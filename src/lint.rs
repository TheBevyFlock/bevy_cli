use anyhow::{anyhow, ensure, Context};
use std::{env, path::PathBuf, process::Command};

/// Runs `bevy_lint` if it is installed with the given arguments.
pub fn lint(args: Vec<String>) -> anyhow::Result<()> {
    let bevy_lint_path = find_bevy_lint()?;

    let status = Command::new(bevy_lint_path).args(args).status()?;

    ensure!(
        status.success(),
        "`bevy_lint` exited with a non-zero exit code."
    );

    Ok(())
}

/// Tries the find the path to `bevy_lint`, if it is installed.
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
            "`bevy_lint` could not be found at {bevy_lint_path:?}. Please run `cargo build -p bevy_lint` first!",
        );
    } else {
        ensure!(bevy_lint_path.exists(), "`bevy_lint` could not be found at {bevy_lint_path:?}. Please follow the instructions in the Bevy CLI `README.md` to install it.");
    }

    bevy_lint_path = bevy_lint_path.canonicalize()?;

    Ok(bevy_lint_path)
}
