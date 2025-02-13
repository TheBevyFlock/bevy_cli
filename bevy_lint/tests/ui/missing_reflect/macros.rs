//@aux-build:../auxiliary/proc_macros.rs
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::missing_reflect)]
#![allow(dead_code)]

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;

external! {
    #[derive(Component)]
    struct MyComponentExternal;
}

macro_rules! local_macro {
    () => {
        //~ NOTE: `Component` implemented here
        #[derive(Component)]
        //~| HELP: `Reflect` can be automatically derived
        //~v ERROR: defined a component without a `Reflect` implementation
        struct MyComponent;
    };
}

local_macro!();
