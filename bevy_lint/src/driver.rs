//! Contains code related to writing `bevy_lint_driver`.

use crate::callback::BevyLintCallback;
use std::process::ExitCode;

/// This is the main entrypoint into the driver, exported so that `bevy_cli` may call it.
pub fn main() -> ExitCode {
    let args: Vec<String> = dbg!(std::env::args().skip(1).collect());

    // Call the compiler with our custom callback.
    let result = rustc_driver::RunCompiler::new(&args, &mut BevyLintCallback).run();

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
