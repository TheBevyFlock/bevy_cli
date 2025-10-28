// A convenience feature used in `find_bevy_rlib()` that lets you chain multiple `if let`
// statements together with `&&`. This feature flag is needed in all integration tests that use the
// test_utils module, since each integration test is compiled independently.

use test_utils::base_config;
use ui_test::run_tests;

mod test_utils;

fn main() {
    let mut config = base_config("ui").unwrap();
    let defaults = config.comment_defaults.base();
    // This allows for any status code to be considered a success.
    defaults.exit_status = None.into();
    run_tests(config).unwrap();
}
