use std::path::{Path, PathBuf};

use ui_test::{
    CommandBuilder, Config, color_eyre::{self, eyre::ensure}, dependencies::DependencyBuilder, run_tests
};

// This is set by Cargo to the absolute path of `bevy_lint_driver`.
const DRIVER_PATH: &str = env!("CARGO_BIN_EXE_bevy_lint_driver");

fn main() {
    let config = base_config().unwrap();
    run_tests(config).unwrap();
}

fn base_config() -> color_eyre::Result<Config> {
    let driver_path = Path::new(DRIVER_PATH);

    ensure!(
        driver_path.is_file(),
        "`bevy_lint_driver` could not be found at {}, make sure to build it with `cargo build -p bevy_lint --bin bevy_lint_driver`",
        driver_path.display(),
    );

    let mut config = Config {
        // When `host` is `None`, `ui_test` will attempt to auto-discover the host by calling
        // `program -vV`. Unfortunately, `bevy_lint_driver` does not yet support the version flag,
        // so we manually specify the host as an empty string. This means that, for now, host-
        // specific configuration in UI tests will not work.
        host: Some(String::from("aarch64-apple-darwin")), //TODO: switch to host-tuple
        program: CommandBuilder {
            // We don't need `rustup run` here because we're already using the correct toolchain
            // due to `rust-toolchain.toml`.
            program: driver_path.into(),
            args: vec![
                // `bevy_lint_driver` expects the first argument to be the path to `rustc`.
                "rustc".into(),
                // This is required so that `ui_test` can parse warnings and errors.
                "--error-format=json".into(),
            ],
            out_dir_flag: Some("--out-dir".into()),
            input_file_flag: None,
            envs: Vec::new(),
            cfg_flag: Some("--print=cfg".into()),
        },
        out_dir: PathBuf::from("../target/ui"),
        ..Config::rustc(Path::new("tests/ui"))
    };

    let revisioned = config.comment_defaults.base();
    revisioned.set_custom("dependencies", DependencyBuilder {
        crate_manifest_path: PathBuf::from("tests/dependencies/Cargo.toml"),
        ..Default::default()
    });

    Ok(config)
}
