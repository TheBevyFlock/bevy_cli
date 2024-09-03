//! Runs UI tests: tests that ensure errors and warnings are correctly emitted for given lints.
//! 
//! To see available options, run `cargo test -p bevy_lint --test ui -- --help`. For more
//! information, see <https://github.com/oli-obk/ui_test>.


use ui_test::{run_tests, Config};

fn main() -> ui_test::color_eyre::Result<()> {
    // Test everything in the `ui` folder.
    let config = Config::rustc("tests/ui");

    // TODO: Use `bevy_lint_driver` instead of pure `rustc`.

    run_tests(config)
}
