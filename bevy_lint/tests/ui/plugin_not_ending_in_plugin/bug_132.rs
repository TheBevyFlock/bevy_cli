//! A test that checks that annotating the structure definition of a plugin does not silence the
//! lint.
//!
//! This test tracks the bug reported in [#132]. When this starts failing, the bug has been fixed.
//!
//! [#132]: https://github.com/TheBevyFlock/bevy_cli/issues/132

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
