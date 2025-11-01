// A convenience feature used in `find_bevy_rlib()` that lets you chain multiple `if let`
// statements together with `&&`. This feature flag is needed in all integration tests that use the
// test_utils module, since each integration test is compiled independently.

use std::{env, path::{Path, PathBuf}};

use ui_test::{CommandBuilder, Config, status_emitter};

// This is set by Cargo to the absolute paths of `bevy_lint` and `bevy_lint_driver`.
const LINTER_PATH: &str = env!("CARGO_BIN_EXE_bevy_lint");
const DRIVER_PATH: &str = env!("CARGO_BIN_EXE_bevy_lint_driver");

/// This [`Config`] will run the `bevy_lint` command for all paths that end in `Cargo.toml`
/// # Example:
/// ```sh
/// bevy_lint" "--quiet" "--target-dir"
/// "../target/ui/0/tests/ui-cargo/duplicate_bevy_dependencies/fail" "--manifest-path"
/// "tests/ui-cargo/duplicate_bevy_dependencies/fail/Cargo.toml"```
fn main() {
    let linter_path = Path::new(LINTER_PATH);
    let driver_path = Path::new(DRIVER_PATH);

    assert!(
        linter_path.is_file(),
        "`bevy_lint` could not be found at {}, make sure to build it with `cargo build -p bevy_lint --bin bevy_lint`",
        linter_path.display(),
    );
    assert!(
        driver_path.is_file(),
        "`bevy_lint_driver` could not be found at {}, make sure to build it with `cargo build -p bevy_lint --bin bevy_lint_driver`",
        driver_path.display(),
    );

    let mut config = Config {
        // When `host` is `None`, `ui_test` will attempt to auto-discover the host by calling
        // `program -vV`. Unfortunately, `bevy_lint_driver` does not yet support the version flag,
        // so we manually specify the host as an empty string. This means that, for now, host-
        // specific configuration in UI tests will not work.
        host: Some(String::new()),
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
        ..Config::rustc(Path::new("tests/ui-cargo"))
    };

    let defaults = config.comment_defaults.base();
    // The driver returns a '101' on error.
    // This allows for any status code to be considered a success.
    defaults.exit_status = None.into();

    defaults.require_annotations = None.into();

    // This sets the '--manifest-path' flag
    config.program.input_file_flag = CommandBuilder::cargo().input_file_flag;
    config.program.out_dir_flag = CommandBuilder::cargo().out_dir_flag;
    // Do not print cargo log messages
    config.program.args = vec!["--quiet".into(), "--color".into(), "never".into()];

    let current_exe_path = env::current_exe().unwrap();
    let deps_path = current_exe_path.parent().unwrap();
    let profile_path = deps_path.parent().unwrap();

    // Specify the binary to use when executing tests with this `Config`
    config.program.program = profile_path.join(if cfg!(windows) {
        "bevy_lint_driver.exe"
    } else {
        "bevy_lint_driver"
    });

    config.program.program.set_file_name(if cfg!(windows) {
        "bevy_lint.exe"
    } else {
        "bevy_lint"
    });

    // this clears the default `--edition` flag
    config.comment_defaults.base().custom.clear();

    // Run this `Config` for all paths that end with `Cargo.toml` resulting
    // only in the `Cargo` lints.
    ui_test::run_tests_generic(
        vec![config],
        |path, config| {
            path.ends_with("Cargo.toml")
                .then(|| ui_test::default_any_file_filter(path, config))
        },
        |_config, _file_contents| {},
        status_emitter::Text::verbose(),
    )
    .unwrap();
}
