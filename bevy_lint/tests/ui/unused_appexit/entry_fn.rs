#![feature(register_tool)]
#![register_tool(bevy)]
//~v NOTE: the lint level is defined here
#![deny(bevy::unused_appexit)]

use bevy::prelude::*;

//~v HELP: set the return type of `fn main()`
fn main() {
    App::new().run();
    //~^ ERROR: called `App::run()` without handling the returned `AppExit`
    //~| NOTE: `App::run()` returns `AppExit`, which is used to determine whether the app exited successfully or not
    //~| HELP: `AppExit` implements `Termination`, so it can be returned directly from `fn main()`
    //~| HELP: return the result of `App::run()`
}
