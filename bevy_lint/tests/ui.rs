//! Runs UI tests: tests that ensure errors and warnings are correctly emitted for given lints.
//!
//! To see available options, run `cargo test -p bevy_lint --test ui -- --help`. For more
//! information, see <https://github.com/oli-obk/ui_test>.

use core::str;
use std::{
    mem,
    path::{Path, PathBuf},
    process::Command,
};
use ui_test::{
    color_eyre::{
        self,
        eyre::{ensure, eyre},
    },
    run_tests, CommandBuilder, Config,
};

fn main() -> color_eyre::Result<()> {
    let config = config()?;

    // TODO: Use `bevy_lint_driver` instead of pure `rustc`.

    run_tests(config)
}

fn config() -> color_eyre::Result<Config> {
    let mut program = CommandBuilder::rustc();

    let driver_path = PathBuf::from("../target/debug/bevy_lint_driver");

    // TODO: Set .exe suffix.

    ensure!(
        driver_path.exists(),
        "`bevy_lint_driver` does not exist, please build it with `cargo build -p bevy_lint --bin bevy_lint_driver`.",
    );

    let rustc = mem::replace(&mut program.program, driver_path);

    program.args.insert(0, rustc.into());

    program.envs.push((
        "LD_LIBRARY_PATH".into(),
        Some(locate_toolchain_libraries()?.into()),
    ));

    Ok(Config {
        program,
        // We set this to an empty string because else it will try auto-detecting the host by
        // calling `bevy_lint_driver -vV`, which is currently not supported and will error. This
        // does break host-specific configuration, but we currently don't use it.
        host: Some(String::new()),
        // Point to the workspace `target`, since we're within a subdirectory.
        out_dir: PathBuf::from("../target/ui"),
        // Test everything in the `ui` folder.
        ..Config::rustc("tests/ui")
    })
}

fn locate_toolchain_libraries() -> color_eyre::Result<PathBuf> {
    let rustup_output = Command::new("rustup").arg("which").arg("rustc").output()?;

    ensure!(
        rustup_output.status.success(),
        "`rustup which rustc` failed with non-zero exit code."
    );

    // We're assuming the path to `rustc` is valid UTF-8. If this gives you an error, please report
    // an issue!
    let rustc_path = Path::new(str::from_utf8(&rustup_output.stdout)?);

    // From `~/.rustup/toolchains/*/bin/rustc`, find `~/.rustup/toolchains/*/lib`.
    let lib_path = rustc_path
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| {
            eyre!(
                "Failed to find toolchain path from `rustc` at {}.",
                rustc_path.display()
            )
        })?
        .join("lib");

    ensure!(
        lib_path.exists(),
        "Toolchain library path does not exist at {}.",
        lib_path.display()
    );

    Ok(lib_path)
}
