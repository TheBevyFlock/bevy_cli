use anyhow::{Context, ensure};
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
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

    todo!();

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
        // Rustup on Windows does not modify the `PATH` variable by default so a toolchain-specific
        // version of `cargo` or `rustc` is not accidentally run instead of Rustup's proxy version.
        // This isn't desired for us, however, because we need the `PATH` modified to discover and
        // link to `rustc_driver.dll`. Setting `RUSTUP_WINDOWS_PATH_ADD_BIN=1` forces Rustup to
        // modify the path. For more info, please see <https://github.com/rust-lang/rustup/pull/3703>.
        .env("RUSTUP_WINDOWS_PATH_ADD_BIN", "1")
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

    println!("{NAME} v{VERSION}");
}

/// Returns the path to `bevy_lint_driver`.
///
/// This function will first search the folder of the current executable for `bevy_lint_driver`. If
/// found, that path will be returned. If `bevy_lint_driver` is not within the same folder as
/// `bevy_lint`, it will be searched for in the `PATH` instead.
///
/// # Errors
///
/// This will error if `bevy_lint_driver` could not be found in either the current executable's
/// folder or the `PATH`, or if the `PATH` environmental variable could not be accessed.
fn driver_path() -> anyhow::Result<PathBuf> {
    // The file name of `bevy_lint_driver` with the correct executable extension.
    let driver_file_name = Path::new("bevy_lint_driver").with_extension(env::consts::EXE_EXTENSION);

    // The folder the current executable is within. This resolves all symbolic links, meaning the
    // folder of the actual executable and not the link will be found. If the executable folder
    // could not be found, this will be `None`.
    let executable_folder = env::current_exe()
        .and_then(|path| path.canonicalize())
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf));

    if let Some(executable_folder) = executable_folder {
        let driver_path = executable_folder.join(&driver_file_name);

        // If `bevy_lint_driver` exists within the executable folder, we return that path.
        if driver_path.is_file() {
            // The driver path should be absolute so `rustc` does not get confused when we give it
            // the path.
            debug_assert!(
                driver_path.is_absolute(),
                "the executable folder was previously canonicalized, but the driver path {} is not absolute",
                driver_path.display(),
            );

            // There is a `bevy_lint_driver` within the same folder as the executable, return it
            // and do not search the `PATH`.
            return Ok(driver_path);
        }
    }

    let path = env::var_os("PATH").context("could not fetch the `PATH` environmental variable")?;

    // Search the `PATH` for `bevy_lint_driver`. This is adopted from
    // <https://stackoverflow.com/a/37499032>, thank you!
    let driver_path = env::split_paths(&path)
        // Filter the `PATH` for paths to `bevy_lint_driver`.
        .filter_map(|folder| {
            let driver_path = folder.join(&driver_file_name);

            // If `bevy_lint_driver` exists in this `PATH` folder, return it.
            driver_path.is_file().then_some(driver_path)
        })
        // Get the first occurrence of `bevy_lint_driver` in `PATH`.
        .next()
        .context("could not find `bevy_lint_driver` in the `PATH`")?;

    // Get the absolute path the `bevy_lint_driver` and return it.
    driver_path.canonicalize().with_context(|| {
        format!(
            "could not get the absolute path of {}",
            driver_path.display(),
        )
    })
}

/// Locates the path to the Rust compiler.
///
/// If the `BEVY_LINT_RUSTC` environmental variable is specified, its value will be returned. If
/// the variable does not exist, `rustup which rustc` will be called to locate `rustc`.
fn rustc_path() -> anyhow::Result<PathBuf> {
    if let Some(rustc_path) = env::var_os("BEVY_LINT_RUSTC") {
        let rustc_path = PathBuf::from(rustc_path);

        ensure!(
            rustc_path.is_file(),
            "the path in `BEVY_LINT_RUSTC`, {}, does not exist",
            rustc_path.display(),
        );

        return rustc_path.canonicalize().with_context(|| {
            format!(
                "could not get the absolute path of {}",
                rustc_path.display(),
            )
        });
    }

    let output = Command::new("rustup")
        .arg("which")
        .arg("rustc")
        .arg(format!("--toolchain={RUST_TOOLCHAIN_CHANNEL}"))
        .stderr(Stdio::inherit())
        .output()
        .context("failed to spawn `rustup` to locate a `rustc`, is it installed?")?;

    ensure!(
        output.status.success(),
        "could not locate `rustc` using `rustup`, is the toolchain {RUST_TOOLCHAIN_CHANNEL} installed?",
    );

    let rustc_path = Path::new(
        // Rustup should only emit UTF-8, as it's a Rust program, so it should be safe to error
        // here when invalid UTF-8 is found.
        str::from_utf8(&output.stdout)
            .context("the output of `rustup which` is not valid UTF-8")?
            .trim_end(),
    );

    ensure!(
        rustc_path.is_file(),
        "the path returned by `rustup which rustc`, {}, does not exist",
        rustc_path.display(),
    );

    debug_assert!(
        rustc_path.is_absolute(),
        "`rustup which` should canonicalize the path to `rustc`, {}, but it is not absolute",
        rustc_path.display(),
    );

    Ok(rustc_path.to_path_buf())
}
