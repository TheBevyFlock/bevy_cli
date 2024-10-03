#![feature(register_tool)]
#![register_tool(bevy)]
#![warn(bevy::main_return_without_appexit)]

use bevy::prelude::*;

fn main() {
    App::new().run();
    //~^ WARN: an entrypoint that calls `App::run()` does not return `AppExit`
}
