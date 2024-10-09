//! This test tracks the bug reported in [#94]. When this starts failing, the bug has been fixed.
//!
//! [#94]: https://github.com/TheBevyFlock/bevy_cli/issues/94

//@check-pass

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // This should error because the `AppExit` is not handled, but it does not.
    App::run(&mut app);
}
