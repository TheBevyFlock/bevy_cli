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
        program: CommandBuilder {
            program: "rustup".into(),
            args: vec![
                "run".into(),
                "nightly-2024-08-21".into(),
                "../target/debug/bevy_lint_driver".into(),
                "rustc".into(),
                "--error-format=json".into(),
                // This allows examples to `use bevy;`.
                "-L".into(),
                "all=../target/debug/deps".into(),
                "--extern=bevy".into(),
            ],
            out_dir_flag: Some("--out-dir".into()),
            input_file_flag: None,
            envs: Vec::new(),
            cfg_flag: Some("--print=cfg".into()),
        },
        out_dir: PathBuf::from("../target/ui"),
        ..Config::rustc("tests/ui")
    }
}
