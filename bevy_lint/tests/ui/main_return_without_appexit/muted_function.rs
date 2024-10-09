//! A version of `main.rs` where the lint is muted on the function. This should pass without any
//! errors.

//@check-pass

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::main_return_without_appexit)]

use bevy::prelude::*;

#[allow(bevy::main_return_without_appexit)]
fn main() {
    App::new().run();
}
