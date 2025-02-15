//@aux-build:../auxiliary/proc_macros.rs
//! Tests the most basic version: where `main()` returns nothing and `AppExit` is not handled.

//@no-rustfix
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;

macro_rules! local_macro {
    () => {
        let mut app = App::new();
        App::new().run();
        //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`

        App::run(&mut app);
        //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`
    };
}

fn main() {
    let mut app = App::new();
    App::new().run();
    //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`

    App::run(&mut app);
    //~^ ERROR: an entrypoint that calls `App::run()` does not return `AppExit`

    external!({
        let mut app = App::new();
        App::new().run();
        App::run(&mut app);
    });
    local_macro!();
}
