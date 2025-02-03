use anyhow::{anyhow, ensure, Context};
use std::{
    env,
    process::{Command, ExitCode},
};

// This is set by `build.rs`. It is the version specified in `rust-toolchain.toml`.
const RUST_TOOLCHAIN_CHANNEL: &str = env!("RUST_TOOLCHAIN_CHANNEL");

fn main() -> anyhow::Result<ExitCode> {
    // If any of the arguments contains `--version`, print the version and exit.
    if std::env::args().skip(1).any(|arg| arg == "--version") {
        show_version();
        return Ok(ExitCode::SUCCESS);
    }

    // The `bevy_lint` lives in the same folder as `bevy_lint_driver`, so we can easily find it
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
        .arg(format!("+{RUST_TOOLCHAIN_CHANNEL}"))
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
            env::var("RUSTFLAGS").map_or("--cfg bevy_lint".to_string(), |mut flags| {
                flags.push_str(" --cfg bevy_lint");
                flags
            }),
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
