#![feature(register_tool)]
#![register_tool(bevy)]
//~v NOTE: the lint level is defined here
#![deny(bevy::unused_appexit)]

use bevy::prelude::*;

pub fn foo() {
    App::new().run();
    //~^ ERROR: called `App::run()` without handling the returned `AppExit`
    //~| NOTE: `App::run()` returns `AppExit`, which is used to determine whether the app exited successfully or not
    //~| HELP: consider logging a warning if the returned `AppExit` is an error
    //~| HELP: handle the returned `AppExit`
}
