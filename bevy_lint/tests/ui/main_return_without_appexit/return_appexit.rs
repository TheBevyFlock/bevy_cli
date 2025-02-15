//! Tests when `main()` returns `AppExit`, meaning the user has fixed the lint. No diagnostics
//! should be emitted in this case.

//@check-pass
//@no-rustfix

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;

fn main() -> AppExit {
    App::new().run()
}
