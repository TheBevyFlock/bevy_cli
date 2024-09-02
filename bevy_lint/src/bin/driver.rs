// Enables linking to `rustc` crates.
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_span;

use bevy_lint::BevyLintCallback;
use rustc_span::ErrorGuaranteed;

fn main() -> Result<(), ErrorGuaranteed> {
    // The arguments are formatted as `[DRIVER_PATH, RUSTC_PATH, ARGS...]`. We skip the driver path
    // so that `RunCompiler` just sees `rustc`'s path.
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Call the compiler with our custom callback.
    rustc_driver::RunCompiler::new(&args, &mut BevyLintCallback).run()
}
