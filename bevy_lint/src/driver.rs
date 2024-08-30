//! Contains code related to writing `bevy_lint_driver`.

use crate::callback::BevyLintCallback;

/// This is the main entrypoint into the driver, exported so that `bevy_cli` may call it.
#[allow(clippy::result_unit_err)]
pub fn main() -> Result<(), ()> {
    let args: Vec<String> = dbg!(std::env::args().skip(1).collect());

    // Call the compiler with our custom callback.
    rustc_driver::RunCompiler::new(&args, &mut BevyLintCallback)
        .run()
        .map_err(|_| ())
}
