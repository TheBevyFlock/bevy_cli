//! Runs UI tests: tests that ensure errors and warnings are correctly emitted for given lints.
//!
//! To see available options, run `cargo test -p bevy_lint --test ui -- --help`. For more
//! information, see <https://github.com/oli-obk/ui_test>.

use core::str;
use std::{mem, path::PathBuf};
use ui_test::{
    color_eyre::{self, eyre::ensure},
    run_tests, CommandBuilder, Config,
};

const TOOLCHAIN: &str = "nightly-2024-08-21";

fn main() -> color_eyre::Result<()> {
    let config = config()?;
    run_tests(config)
}

fn config() -> color_eyre::Result<Config> {
    Ok(Config {
        program: program()?,
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

fn program() -> color_eyre::Result<CommandBuilder> {
    let mut program = CommandBuilder::rustc();

    let mut driver_path = PathBuf::from("../target/debug/bevy_lint_driver");

    if cfg!(target_os = "windows") {
        driver_path.set_extension("exe");
    }

    ensure!(
        driver_path.exists(),
        "`bevy_lint_driver` does not exist, please build it with `cargo build -p bevy_lint --bin bevy_lint_driver`.",
    );

    // Swap `rustc` for `rustup`.
    let rustc = mem::replace(&mut program.program, PathBuf::from("rustup"));

    // Add `rustup` args to `run TOOLCHAIN driver_path rustc`.
    let args = vec![
        "run".into(),
        TOOLCHAIN.into(),
        driver_path.into(),
        rustc.into(),
    ];

    // Replace existing args with new ones.
    let mut additional_args = mem::replace(&mut program.args, args);

    // Add the existing args to the end of the new ones.
    program.args.append(&mut additional_args);

    Ok(program)
}
