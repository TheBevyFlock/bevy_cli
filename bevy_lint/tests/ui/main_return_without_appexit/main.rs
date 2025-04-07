//! Tests the most basic version: where `main()` returns nothing and `AppExit` is not handled.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;

fn main() {
    // This should not raise an error, since `AppExit` is not ignored.
    #[allow(unused_variables)]
    let app_exit = App::new().run();

    let mut app = App::new();
    App::new().run();
    //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`

    App::run(&mut app);
    //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`
}
