//! Tests when `main()` returns a type other than the unit `()`. When this is done no lint is
//! emitted, since we assume the user knows what they're doing.

//@check-pass
//@no-rustfix
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;

fn main() -> Result<(), ()> {
    App::new().run();

    Ok(())
}
