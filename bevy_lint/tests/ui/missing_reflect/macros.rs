//@aux-build:../auxiliary/proc_macros.rs
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::missing_reflect)]
#![allow(dead_code)]

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;

macro_rules! local_macro {
    () => {
        #[derive(Component)]
        //~| HELP: `Reflect` can be automatically derived
        //~v ERROR: defined a component without a `Reflect` implementation
        struct MyComponent;
    };
}

external! {
    #[derive(Component)]
    struct MyComponentExternal;
}

local_macro!();
