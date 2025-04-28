use anyhow::{Context, ensure};
use std::{
    env,
    ffi::OsString,
    iter,
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
    let driver_path = rustc_workspace_wrapper();

    let mut command = Command::new("cargo");

    command
        .arg("check")
        // Forward all arguments to `cargo check` except for the first, which is the path to the
        // current executable.
        .args(std::env::args().skip(1))
        // This instructs `rustc` to call `bevy_lint_driver` instead of its default routine.
        // This lets us register custom lints.
        .env("RUSTC_WORKSPACE_WRAPPER", driver_path);

    append_rustc_libdir(&mut command)?;

    let status = command
        .status()
        .context("failed to spawn `cargo check`, is `cargo` installed?")?;

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

/// This returns the value that should be set for the `RUSTC_WORKSPACE_WRAPPER` environmental
/// variable when running the linter.
///
/// `RUSTC_WORKSPACE_WRAPPER` instructs Cargo to call `$RUSTC_WORKSPACE_WRAPPER rustc main.rs`
/// instead of just `rustc main.rs`. The `rustc` wrapper for `bevy_lint` is `bevy_lint_driver`,
/// which acts very similar to normal `rustc` with the exception that it registers custom lints.
///
/// This function does its best to return the path to `bevy_lint_driver`. If the linter was
/// installed with `cargo install`, `bevy_lint` and `bevy_lint_driver` will be within the same
/// folder (usually `~/.cargo/bin`). When this is the case, this function will return the absolute
/// path to `bevy_lint_driver` in the form of an [`OsString`].
///
/// When `bevy_lint_driver` is not within the same folder as `bevy_lint`, this function will simply
/// return the string `"bevy_lint_driver"`[^0] and hope that it is available in the `PATH`.
///
/// [^0]: On Windows this will be `"bevy_lint_driver.exe"`, but will have the same effect.
fn rustc_workspace_wrapper() -> OsString {
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
            return driver_path.into_os_string();
        }
    }

    driver_file_name.into_os_string()
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

/// Prints the path to the Rust target library folder.
///
/// This folder, the "libdir", contains `librustc_driver.so`, which is needed for
/// `bevy_lint_driver` to run. This function finds the libdir by running
/// `rustc --print=target-libdir`.
fn rustc_libdir() -> anyhow::Result<PathBuf> {
    let rustc_path = rustc_path()?;

    // TODO: --target parameter
    let output = Command::new(rustc_path)
        .arg("--print=target-libdir")
        .stderr(Stdio::inherit())
        .output()
        .context("failed to spawn `rustc`")?;

    ensure!(
        output.status.success(),
        "could not print `rustc` library path",
    );

    let libdir = Path::new(
        // Rustup should only emit UTF-8, as it's a Rust program, so it should be safe to error
        // here when invalid UTF-8 is found.
        str::from_utf8(&output.stdout)
            .context("the output of `rustc --print=target-libdir` is not valid UTF-8")?
            .trim_end(),
    );

    ensure!(
        libdir.is_dir(),
        "the path returned by `rustc --print=target-libdir`, {}, does not exist",
        libdir.display(),
    );

    debug_assert!(
        libdir.is_absolute(),
        "`rustc --print=target-libdir` should canonicalize the path, {}, but it is not absolute",
        libdir.display(),
    );

    Ok(libdir.to_path_buf())
}

/// Configures a `bevy_lint_driver` [`Command`] with the correct environmental variables so that it
/// can link to `librustc_driver.so`.
fn append_rustc_libdir(command: &mut Command) -> anyhow::Result<()> {
    let libdir = rustc_libdir()?;

    let path_name = if cfg!(target_os = "windows") {
        "PATH"
    } else if cfg!(target_os = "macos") {
        "DYLD_LIBRARY_PATH"
    } else {
        // We're assuming that platforms that are not Windows or MacOS are *probably* Unix-based.
        "LD_LIBRARY_PATH"
    };

    let original_path = env::var_os(path_name)
        .with_context(|| format!("could not fetch the `{path_name}` environmental variable"))?;

    // Prepend `libdir` to the beginning of `original_path`.
    let prepended_path =
        env::join_paths(iter::once(libdir).chain(env::split_paths(&original_path)))
            .context("error constructing new path environmental variable")?;

    command.env(path_name, prepended_path);

    Ok(())
}
