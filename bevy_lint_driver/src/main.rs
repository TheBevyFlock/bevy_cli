// Enables linking to `rustc` crates.
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;

mod callback;

fn main() {
    let args: Vec<String> = dbg!(std::env::args().skip(1).collect());

    // Call the compiler with our custom callback.
    rustc_driver::RunCompiler::new(&args, &mut callback::BevyLintCallback)
        .run()
        .unwrap()
}
