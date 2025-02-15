//! This test tracks the bug reported in [#87]. When this starts failing, the bug has been fixed.
//!
//! [#87]: https://github.com/TheBevyFlock/bevy_cli/issues/87

//@no-rustfix
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;

fn main() {
    // This should not raise an error, since `AppExit` is not ignored.
    let app_exit = App::new().run();
    //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`

    println!("{app_exit:?}");
}
