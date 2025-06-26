use std::{
    env, iter,
    path::PathBuf,
    process::{Command, ExitCode},
};

use anyhow::{Context, ensure};

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

    // Find the path to the custom sysroot, if specified.
    let custom_sysroot = custom_sysroot()?;

    let mut cargo = match custom_sysroot {
        // When there's a custom sysroot, run `$SYSROOT/bin/cargo`.
        Some(sysroot) => {
            let cargo = sysroot.join("bin/cargo");

            ensure!(
                cargo.exists(),
                "path to sysroot cargo executable, {}, does not exist",
                cargo.display(),
            );

            let mut c = Command::new(cargo);

            let path_name = match env::consts::OS {
                "windows" => "PATH",
                "macos" => "DYLD_LIBRARY_PATH",
                // Fallback to assuming the platform is Unix-based and uses `LD_LIBRARY_PATH`.
                _ => "LD_LIBRARY_PATH",
            };

            let library_path = match env::consts::OS {
                // `librustc_driver.dll` is in the `bin` folder on Windows.
                "windows" => sysroot.join("bin"),
                _ => sysroot.join("lib"),
            };

            let original_paths = env::var_os(path_name).unwrap_or_default();
            let original_paths =
                env::split_paths(&original_paths).filter(|path| !path.as_os_str().is_empty());

            let appended_paths = original_paths.chain(iter::once(library_path));
            let appended_paths = env::join_paths(appended_paths).with_context(|| {
                format!("error constructing new {path_name} environmental variable")
            })?;

            // Make `librustc_driver.so` discoverable by appending its folder to the correct path
            // environmental variable. Rustup (mostly) does this by default, but since we're using
            // a custom sysroot we have to do it ourselves.
            c.env(path_name, appended_paths);

            c
        }
        // When using Rustup, run `rustup run $TOOLCHAIN cargo`.
        None => {
            let mut c = Command::new("rustup");

            c.arg("run")
                .arg(RUST_TOOLCHAIN_CHANNEL)
                .arg("cargo")
                // Between 1.27.1 and 1.28.2, Rustup by default wouldn't modify the `PATH` variable
                // on Windows in order so a toolchain-specific version of `cargo` or `rustc` is not
                // accidentally run instead of Rustup's proxy version. This isn't desired for us,
                // however, because we need the `PATH` modified to discover and link to
                // `rustc_driver.dll`. Setting `RUSTUP_WINDOWS_PATH_ADD_BIN=1` forces Rustup
                // prepend the sysroot `bin` folder to the `PATH`.
                //
                // From 1.28.2 onwards, Rustup will append the `bin` folder to the `PATH` by
                // default (which is also what we do when there's a custom sysroot). Once 1.28.2
                // gets enough adoption (late 2025), we can remove this line and say the minimum
                // supported Rustup version is 1.28.2.
                //
                // For more info, please see <https://github.com/rust-lang/rustup/pull/3703> and
                // <https://github.com/rust-lang/rustup/pull/4249>.
                .env("RUSTUP_WINDOWS_PATH_ADD_BIN", "1");

            c
        }
    };

    let status = cargo
        .arg("check")
        // Forward all arguments to `cargo check` except for the first, which is the path to the
        // current executable.
        .args(std::env::args().skip(1))
        // This instructs Cargo to call `bevy_lint_driver` instead of `rustc`, which lets us use
        // custom lints.
        .env("RUSTC_WORKSPACE_WRAPPER", driver_path)
        .status()
        .context("failed to spawn `cargo check`")?;

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
/// This function assumes that `bevy_lint` and `bevy_lint_driver` are installed into the same
/// folder, and will error if this is not the case. This function does not search the `PATH`.
///
/// # Errors
///
/// This may error if the current executable cannot be found or `bevy_lint_driver` does not exist.
fn driver_path() -> anyhow::Result<PathBuf> {
    // The `bevy_lint` lives in the same folder as `bevy_lint_driver`, so we can easily find it
    // using the path of the current executable.
    let driver_path = env::current_exe()
        .context("failed to retrieve the path to the current executable")?
        .parent()
        .expect("path to file must have a parent")
        .join("bevy_lint_driver")
        .with_extension(env::consts::EXE_EXTENSION);

    ensure!(
        driver_path.is_file(),
        "could not find `bevy_lint_driver` at {}, please ensure it is installed alongside `bevy_lint`",
        driver_path.display(),
    );

    // Convert the local path to the absolute path. We don't want `rustc` getting confused!
    driver_path.canonicalize().map_err(anyhow::Error::from)
}

/// Returns the path to the custom sysroot used by `bevy_lint_driver`, if specified by the user.
///
/// If the result is [`Some`], the path is guaranteed to exist.
fn custom_sysroot() -> anyhow::Result<Option<PathBuf>> {
    let Some(sysroot) = env::var_os("BEVY_LINT_SYSROOT").map(PathBuf::from) else {
        return Ok(None);
    };

    ensure!(
        sysroot.exists(),
        "the path specified by `BEVY_LINT_SYSROOT`, {}, does not exist",
        sysroot.display(),
    );

    match sysroot.canonicalize() {
        Ok(sysroot) => Ok(Some(sysroot)),
        Err(error) => Err(error.into()),
    }
}
