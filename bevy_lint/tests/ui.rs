use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use ui_test::{CommandBuilder, Config, dependencies::DependencyBuilder, run_tests};

// This is set by Cargo to the absolute path of `bevy_lint_driver`.
const DRIVER_PATH: &str = env!("CARGO_BIN_EXE_bevy_lint_driver");

fn main() {
    let driver_path = Path::new(DRIVER_PATH);

    assert!(
        driver_path.is_file(),
        "`bevy_lint_driver` could not be found at {}, make sure to build it with `cargo build -p bevy_lint --bin bevy_lint_driver`",
        driver_path.display(),
    );

    let mut config = Config {
        // We need to specify the host tuple manually, because if we don't then `ui_test` will try
        // running `bevy_lint_driver -vV` to discover the host and promptly error because it
        // doesn't realize `bevy_lint_driver` expects its first argument to be the path to `rustc`.
        // If `ui_test` ran `bevy_lint_driver rustc -vV` everything would work, but it's not smart
        // enough to do that.
        host: Some(host_tuple()),
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

    // Give UI tests access to all crate dependencies in the `dependencies` folder. This lets UI
    // tests import `bevy`.
    let revisioned = config.comment_defaults.base();
    revisioned.set_custom(
        "dependencies",
        DependencyBuilder {
            crate_manifest_path: PathBuf::from("tests/dependencies/Cargo.toml"),
            ..Default::default()
        },
    );

    run_tests(config).unwrap();
}

/// Queries the host tuple from `rustc` and returns it as a string.
fn host_tuple() -> String {
    let output = Command::new("rustc")
        .arg("--print=host-tuple")
        // Show errors directly to the user, rather than capturing them.
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run `rustc --print=host-tuple`");

    // `rustc` only works with UTF-8, so it's safe to error if invalid UTF-8 is found.
    str::from_utf8(&output.stdout)
        .expect("`rustc --print=host-tuple` did not emit valid UTF-8")
        // Remove the trailing `\n`.
        .trim_end()
        .to_string()
}
