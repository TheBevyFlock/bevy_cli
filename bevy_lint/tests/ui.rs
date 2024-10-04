use std::{ffi::OsString, path::PathBuf};
use ui_test::{color_eyre, run_tests, CommandBuilder, Config};

fn main() -> color_eyre::Result<()> {
    let config = config();
    run_tests(config)
}

fn config() -> Config {
    Config {
        // Make this an empty string, because `bevy_lint_driver` does not currently support the
        // `--version` flag, which is required to auto-discover the host.
        host: Some(String::new()),
        program: {
            let mut p = CommandBuilder::rustc();

            // Switch from raw `rustc` calls to `rustup run TOOLCHAIN bevy_lint_driver`.
            p.program = PathBuf::from("rustup");

            p.args = [
                "run",
                "nightly-2024-08-21",
                "../target/debug/bevy_lint_driver",
                "rustc",
                "--error-format=json",
                // This allows examples to `use bevy;`.
                "-L",
                "all=../target/debug/deps",
                "--extern=bevy",
            ]
            .into_iter()
            .map(OsString::from)
            .collect();

            p
        },
        out_dir: PathBuf::from("../target/ui"),
        ..Config::rustc("tests/ui")
    }
}
