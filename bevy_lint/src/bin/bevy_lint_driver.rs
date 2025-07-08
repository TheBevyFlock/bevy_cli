//! A `rustc` wrapper that does the actual linting, called by the main `bevy_lint` executable.
//!
//! While this mostly mimics `rustc`'s CLI interface, it is not intended to be called by users.

// Enables linking to `rustc` crates.
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_session;
extern crate rustc_span;

use std::{ffi::OsStr, path::Path, process::ExitCode};

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
        // it prints a pretty error message instead of panicking when encountering non-UTF-8 args.
        let mut args = rustc_driver::args::raw_args(&early_dcx);

        // There are two scenarios we want to catch:
        // 1. When called by Cargo: `[DRIVER_PATH, RUSTC_PATH, ...ARGS]`
        // 2. When called by user: `[DRIVER_PATH, ...ARGS]`
        //
        // This handles both cases and converts the args to `[RUSTC_PATH, ...ARGS]`, since that is
        // what `run_compiler()` expects.
        let args =
            if args.get(1).map(Path::new).and_then(Path::file_stem) == Some(OsStr::new("rustc")) {
                // When called by Cargo, remove the driver path.
                &args[1..]
            } else {
                // When called by user, replace the driver path with the `rustc` path.
                args[0] = "rustc".to_string();
                &args
            };

        // Call the compiler with our custom callback.
        run_compiler(&args, &mut BevyLintCallback);
    });

    // We truncate the `i32` to a `u8`. `catch_with_exit_code()` currently only returns 1 or 0, so
    // this should does not discard any data. We prefer returning an `ExitCode` instead of calling
    // `std::process::exit()` because this calls `Drop` implementations, in case we need them in
    // the future.
    ExitCode::from(exit_code as u8)
}
