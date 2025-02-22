// Enables linking to `rustc` crates.
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_session;
extern crate rustc_span;

use std::process::ExitCode;

use bevy_lint::BevyLintCallback;
use rustc_driver::{catch_with_exit_code, init_rustc_env_logger, install_ice_hook, run_compiler};
use rustc_session::{EarlyDiagCtxt, config::ErrorOutputType};

const BUG_REPORT_URL: &str = "https://github.com/TheBevyFlock/bevy_cli/issues";

fn main() -> ExitCode {
    // Setup a diagnostic context that can be used for error messages.
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());

    // Setup `rustc`'s builtin `tracing` logger.
    init_rustc_env_logger(&early_dcx);

    // "ICE" stands for Internal Compiler Error. An ICE hook is a special type of panic handler
    // that dumps an absurd amount of data when the driver panics. We take advantage of this, but
    // override the default bug report URL so that users bother us, not Rust compiler devs. :)
    install_ice_hook(BUG_REPORT_URL, |dcx| {
        dcx.handle()
            .note("This is likely a bug with `bevy_lint`, not `rustc` or `cargo`.");
    });

    // Run the passed closure, but catch any panics and return the respective exit code.
    let exit_code = catch_with_exit_code(move || {
        // Get the arguments passed through the CLI. This is equivalent to `std::env::args()`, but
        // it returns a `Result` instead of panicking.
        let mut args = rustc_driver::args::raw_args(&early_dcx);

        // The arguments are formatted as `[DRIVER_PATH, RUSTC_PATH, ARGS...]`. We skip the driver
        // path so that `run_compiler()` just sees `rustc`'s path.
        args.remove(0);

        // Call the compiler with our custom callback.
        run_compiler(&args, &mut BevyLintCallback);
    });

    // We truncate the `i32` to a `u8`. `catch_with_exit_code()` currently only returns 1 or 0, so
    // this should does not discard any data. We prefer returning an `ExitCode` instead of calling
    // `std::process::exit()` because this calls `Drop` implementations, in case we need them in
    // the future.
    ExitCode::from(exit_code as u8)
}
