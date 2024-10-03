use std::{ffi::OsString, path::PathBuf};
use ui_test::{color_eyre, run_tests, CommandBuilder, Config};

fn main() -> color_eyre::Result<()> {
    let config = config();
    run_tests(config)
}

fn config() -> Config {
    Config {
        host: Some(String::new()),
        program: {
            let mut p = CommandBuilder::rustc();

            p.program = PathBuf::from("rustup");

            p.args = [
                "run",
                "nightly-2024-08-21",
                "../target/debug/bevy_lint_driver",
                "--error-format=json",
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
