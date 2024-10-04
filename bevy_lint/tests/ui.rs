use std::path::{Path, PathBuf};
use ui_test::{
    color_eyre::{self, eyre::ensure},
    run_tests, CommandBuilder, Config,
};

// This is set by `build.rs`. It is the version specified in `rust-toolchain.toml`.
const RUST_TOOLCHAIN_CHANNEL: &str = env!("RUST_TOOLCHAIN_CHANNEL");

fn main() -> color_eyre::Result<()> {
    let config = config()?;
    run_tests(config)
}

/// Generates a custom [`Config`] for `bevy_lint`'s UI tests.
fn config() -> color_eyre::Result<Config> {
    const DRIVER_PATH: &str = "../target/debug/bevy_lint_driver";

    ensure!(Path::new(DRIVER_PATH).is_file());

    let config = Config {
        // When `host` is `None`, `ui_test` will attempt to auto-discover the host by calling
        // `program -vV`. Unfortunately, `bevy_lint_driver` does not yet support the version flag,
        // so we manually specify the host as an empty string. This means that, for now, host-
        // specific configuration in UI tests will not work.
        host: Some(String::new()),
        program: CommandBuilder {
            // We call `rustup run` to setup the proper environmental variables, so that
            // `bevy_lint_driver` can link to `librustc_driver.so`.
            program: "rustup".into(),
            args: vec![
                "run".into(),
                RUST_TOOLCHAIN_CHANNEL.into(),
                DRIVER_PATH.into(),
                // `bevy_lint_driver` expects the first argument to be the path to `rustc`.
                "rustc".into(),
                // This is required so that `ui_test` can parse warnings and errors.
                "--error-format=json".into(),
                // These two lines tell `rustc` to search in `target/debug/deps` for dependencies.
                // This is required for UI tests to import `bevy`.
                "-L".into(),
                "all=../target/debug/deps".into(),
                // This lets UI tests write `use bevy::*;` without `extern bevy;` first.
                "--extern=bevy".into(),
            ],

            out_dir_flag: Some("--out-dir".into()),
            input_file_flag: None,
            envs: Vec::new(),
            cfg_flag: Some("--print=cfg".into()),
        },
        out_dir: PathBuf::from("../target/ui"),
        ..Config::rustc("tests/ui")
    };

    Ok(config)
}
