#![feature(register_tool)]
#![register_tool(bevy)]
#![warn(bevy::pedantic)]

use bevy::prelude::*;

fn main() {
    App::new().run();
    //~^ WARN: an entrypoint that calls `App::run()` does not return `AppExit`
}
