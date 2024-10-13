//! A version of `main.rs` where the lint is muted on the expression. Since this lint is run on
//! functions, it will still error.
//!
//! This test tracks the bug reported in [#132]. When this starts failing, the bug has been fixed.
//!
//! [#132]: https://github.com/TheBevyFlock/bevy_cli/issues/132

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;

fn main() {
    //~^ HELP: try

    #[allow(bevy::main_return_without_appexit)]
    App::new().run();
    //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`
}
