// A convenience feature used in `find_bevy_rlib()` that lets you chain multiple `if let`
// statements together with `&&`. This feature flag is needed in all integration tests that use the
// test_utils module, since each integration test is compiled independently.
#![feature(let_chains)]

use std::env;

use test_utils::base_config;
use ui_test::{CommandBuilder, status_emitter};

mod test_utils;
/// This [`Config`] will run the `bevy_lint` command for all paths that end in `Cargo.toml`
/// # Example:
/// ```bash
/// bevy_lint" "--quiet" "--target-dir"
/// "../target/ui/0/tests/ui-cargo/duplicate_bevy_dependencies/fail" "--manifest-path"
/// "tests/ui-cargo/duplicate_bevy_dependencies/fail/Cargo.toml"```
fn main() {
    let mut config = base_config("ui-cargo").unwrap();

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
        status_emitter::Text::from(ui_test::Format::Pretty),
    )
    .unwrap();
}
