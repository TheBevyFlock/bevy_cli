//! A test that verifies annotating the structure definition of a plugin does not silence the lint.
//!
//! While this may eventually be desired behavior, this test ensures the behavior does not change
//! without a proper warning.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::plugin_not_ending_in_plugin)]

use bevy::prelude::*;

// This `#[allow(...)]` does nothing.
#[allow(bevy::plugin_not_ending_in_plugin)]
struct Foo;
//~^ HELP: rename the plugin

//~v ERROR: implemented `Plugin` for a structure whose name does not end in "Plugin"
impl Plugin for Foo {
    fn build(&self, _app: &mut App) {}
}

fn main() {
    App::new().add_plugins(Foo);
}
