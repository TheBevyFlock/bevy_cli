//! A test that ensures a plugin whose name is "spoofed" with `use T as F` does not sneak past the
//! lint.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::plugin_not_ending_in_plugin)]

use bevy::prelude::*;

mod bar {
    pub mod baz {
        pub struct Foo;
        //~^ HELP: rename the plugin
    }
}

// We try to be sneaky, but it doesn't work.
use self::bar::baz::Foo as FooPlugin;

//~v ERROR: implemented `Plugin` for a structure whose name does not end in "Plugin"
impl Plugin for FooPlugin {
    fn build(&self, _app: &mut App) {}
}

fn main() {
    App::new().add_plugins(FooPlugin);
}
