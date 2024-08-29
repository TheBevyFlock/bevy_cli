//! This is built into the `bevy_lint_driver` executable. This file does not actually contain any
//! logic, it simply calls [`bevy_lint::driver::main()`] and exits.

// `bevy_lint` uses the `rustc_private` feature, so in order for us to call `bevy_lint` we must also
// opt-in to `rustc_private`.
#![feature(rustc_private)]

fn main() -> Result<(), ()> {
    bevy_lint::driver::main()
}
