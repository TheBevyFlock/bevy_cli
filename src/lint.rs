use anyhow::{anyhow, ensure, Context};
use std::{env, path::PathBuf};

use crate::external_cli::{cargo::install::AutoInstall, CommandExt};

/// Runs `bevy_lint`, if it is installed, with the given arguments.
///
/// Calling `lint(vec!["--workspace"])` is equivalent to calling `bevy_lint --workspace` in the
/// terminal. This will run [`find_bevy_lint()`] to locate `bevy_lint`.
pub fn lint(args: Vec<String>) -> anyhow::Result<()> {
    if let Ok(bevy_lint_path) = find_bevy_lint() {
        let status = CommandExt::new(bevy_lint_path)
            .args(args)
            .ensure_status(AutoInstall::Never)?;

        ensure!(
            status.success(),
            "`bevy_lint` exited with a non-zero exit code."
        );
        return Ok(());
    }

    #[cfg(feature = "rustup")]
    install_linter()?;
    Ok(())
}

/// Tries the find the path to `bevy_lint`, if it is installed.
///
/// The current strategy will find a file named `bevy_lint(.exe)` within the same directory as the
/// current executable, which is usually `~/.cargo/bin` or `target/debug`. It will **not** search
/// the `PATH`.
fn find_bevy_lint() -> anyhow::Result<PathBuf> {
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

#[cfg(feature = "rustup")]
fn install_linter() -> anyhow::Result<()> {
    use toml_edit::DocumentMut;
    const RUST_TOOLCHAIN: &str = include_str!("../rust-toolchain.toml");
    const BEVY_LINT_TAG: &str = "lint-v0.3.0";

    // TODO: pass AutoInstall args
    let auto_install = AutoInstall::Always;

    let rust_toolchain = RUST_TOOLCHAIN
        .parse::<DocumentMut>()
        .expect("Failed to parse `rust-toolchain.toml`.");

    let channel = rust_toolchain["toolchain"]["channel"]
        .as_str()
        .expect("Could not find `toolchain.channel` key in `rust-toolchain.toml`.");

    CommandExt::new("rustup")
        .arg("toolchain")
        .arg("install")
        .arg(channel)
        .args([
            "--component",
            "rustc-dev",
            "--component",
            "llvm-tools-preview",
        ])
        .ensure_status(auto_install)
        .context(format!("failed to install toolchain `{channel}`"))?;

    CommandExt::new("rustup")
        .arg("run")
        .arg(channel)
        .arg("cargo")
        .arg("install")
        .args([
            "--git",
            "https://github.com/TheBevyFlock/bevy_cli.git",
            "--tag",
            BEVY_LINT_TAG,
            "--locked",
            "bevy_lint",
        ])
        .ensure_status(auto_install)
        .context(format!("failed to install bevy_lint `{BEVY_LINT_TAG}`"))?;

    Ok(())
}
