// A convenience feature used in `find_bevy_rlib()` that lets you chain multiple `if let`
// statements together with `&&`. This feature flag is needed in all integration tests that use the
// test_utils module, since each integration test is compiled independently.

use test_utils::base_config;
use ui_test::run_tests;

mod test_utils;

fn main() {
    let config = base_config("ui").unwrap();

    run_tests(config).unwrap();
}
