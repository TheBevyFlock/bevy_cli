#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::system_set_not_ending_in_set)]
//~^ NOTE: the lint level is defined here
#![allow(dead_code)]
use bevy::prelude::*;

//~v NOTE: `SystemSet` implemented here
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudio;
//~^ ERROR: implemented `SystemSet` for a struct whose name does not end in "Set"
//~| HELP: rename the SystemSet
