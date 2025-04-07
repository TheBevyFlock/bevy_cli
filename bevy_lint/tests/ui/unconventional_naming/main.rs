#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unconventional_naming)]
//~^ NOTE: the lint level is defined here
#![allow(dead_code)]
use bevy::prelude::*;

//~v NOTE: `SystemSet` implemented here
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudio;
//~^ ERROR: Unconventional struct name for this trait impl
//~| HELP: rename the SystemSet
