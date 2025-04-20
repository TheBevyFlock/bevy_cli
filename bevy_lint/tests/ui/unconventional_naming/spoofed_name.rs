//! A test that ensures a plugin whose name is "spoofed" with `use T as F` does not sneak past the
//! lint.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unconventional_naming)]
//~^ NOTE: the lint level is defined here

use bevy::prelude::*;

mod bar {
    pub mod baz {
        pub struct Foo;
        //~^ ERROR: unconventional type name for a `Plugin` or `SystemSet`
        //~| HELP: structure that implements Plugin should end in Plugin, rename Foo to FooPlugin
    }
}

// We try to be sneaky, but it doesn't work.
use self::bar::baz::Foo as FooPlugin;

//~v NOTE: `Plugin` implemented here
impl Plugin for FooPlugin {
    fn build(&self, _app: &mut App) {}
}

fn main() {
    App::new().add_plugins(FooPlugin);
}
