use anyhow::{anyhow, ensure, Context};
use std::{
    env,
    path::PathBuf,
    process::{Command, ExitCode},
};

/// The Rustup toolchain channel specified by `rust-toolchain.toml`. This is set by `build.rs`.
const RUST_TOOLCHAIN_CHANNEL: &str = env!("RUST_TOOLCHAIN_CHANNEL");

fn main() -> anyhow::Result<ExitCode> {
    // If any of the arguments contains `--version`, print the version and exit.
    if std::env::args()
        .skip(1)
        .any(|arg| arg == "--version" || arg == "-V")
    {
        show_version();
        return Ok(ExitCode::SUCCESS);
    }

    // Find the path to `bevy_lint_driver`.
    let driver_path = driver_path()?;

    // Run `rustup run nightly-YYYY-MM-DD cargo check`.
    let status = Command::new("rustup")
        .arg("run")
        .arg(RUST_TOOLCHAIN_CHANNEL)
        .arg("cargo")
        .arg("check")
        // Forward all arguments to `cargo check` except for the first, which is the path to the
        // current executable.
        .args(std::env::args().skip(1))
        // This instructs `rustc` to call `bevy_lint_driver` instead of its default routine.
        // This lets us register custom lints.
        .env("RUSTC_WORKSPACE_WRAPPER", driver_path)
        // Pass `--cfg bevy_lint` so that programs can conditionally configure lints. If
        // `RUSTFLAGS` is already set, we append `--cfg bevy_lint` to the end.
        .env(
            "RUSTFLAGS",
            env::var("RUSTFLAGS").map_or_else(
                |_| "--cfg bevy_lint".to_string(),
                |mut flags| {
                    flags.push_str(" --cfg bevy_lint");
                    flags
                },
            ),
        )
        .status()
        .context("Failed to spawn `cargo check`.")?;

    let code = if status.success() {
        // Exit status of 0, success!
        0
    } else {
        // Print out `cargo`'s exit code on failure.
        eprintln!("Check failed: {status}.");

        // Extract the exit code. `ExitCode` only supports being created from a `u8`, so we truncate
        // the bits. Additionally, `ExitStatus::code()` can return `None` on Unix if it was
        // terminated by a signal. In those cases, we just default to 1.
        status.code().unwrap_or(1) as u8
    };

    // Return `cargo`'s exit code.
    Ok(ExitCode::from(code))
}

/// Prints `bevy_lint`'s name and version (as specified in `Cargo.toml`) to stdout.
fn show_version() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("{NAME} {VERSION}");
}

/// Returns the path to `bevy_lint_driver`.
///
/// This function assumes that `bevy_lint` and `bevy_lint_driver` are installed into the same
/// folder, and will error if this is not the case. This function does not search the `PATH`.
///
/// # Errors
///
/// This may error if the current executable cannot be found or `bevy_lint_driver` does not exist.
fn driver_path() -> anyhow::Result<PathBuf> {
    // The `bevy_lint` lives in the same folder as `bevy_lint_driver`, so we can easily find it
    // using the path of the current executable.
    #[cfg_attr(not(target_os = "windows"), expect(unused_mut))]
    let mut driver_path = env::current_exe()
        .context("Failed to retrieve the path to the current executable.")?
        .parent()
        .ok_or(anyhow!("Path to file must have a parent."))?
        .join("bevy_lint_driver");

    #[cfg(target_os = "windows")]
    driver_path.set_extension("exe");

    ensure!(
        driver_path.exists(),
        "Could not find `bevy_lint_driver` at {}, please ensure it is installed!",
        driver_path.display(),
    );

    // Convert the local path to the absolute path. We don't want `rustc` getting
    // confused! `canonicalize()` requires for the path to exist, so we do it after the nice error
    // message.
    driver_path.canonicalize().map_err(anyhow::Error::from)
}
