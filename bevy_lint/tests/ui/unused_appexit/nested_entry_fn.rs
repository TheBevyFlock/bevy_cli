#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unused_appexit)]

use bevy::prelude::*;

fn main() {
    let _closure = || {
        App::new().run();
        //~^ ERROR: called `App::run()` without handling the returned `AppExit`
    };

    fn _nested_function() {
        App::new().run();
        //~^ ERROR: called `App::run()` without handling the returned `AppExit`
    }

    let _async_block = async {
        App::new().run();
        //~^ ERROR: called `App::run()` without handling the returned `AppExit`
    };
}
