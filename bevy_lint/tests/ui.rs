use test_utils::base_config;
use ui_test::run_tests;

mod test_utils;

fn main() {
    let config = base_config("ui").unwrap();
    run_tests(config).unwrap();
}
