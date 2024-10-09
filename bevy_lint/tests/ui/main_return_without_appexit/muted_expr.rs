//! A version of `main.rs` where the lint is muted on the expression. Since this lint is run on
//! functions, it will still error.
//!
//! This behavior is misleading, so this test ensures it is not accidentally fixed without updating
//! the documentation.

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
