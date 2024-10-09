#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;

fn main() -> () {
    //~^ HELP: try
    App::new().run();
    //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`
}
