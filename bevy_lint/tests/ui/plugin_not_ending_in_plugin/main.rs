//@aux-build:../auxiliary/proc_macros.rs
//@no-rustfix
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::plugin_not_ending_in_plugin)]
//~^ NOTE: the lint level is defined here

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;

// This should raise an error, since it does not end in "Plugin".
struct Foo;
//~^ ERROR: implemented `Plugin` for a structure whose name does not end in "Plugin"
//~| HELP: rename the plugin

//~v NOTE: `Plugin` implemented here
impl Plugin for Foo {
    fn build(&self, _app: &mut App) {}
}

// This should _not_ raise an error, since it ends in "Plugin".
struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, _app: &mut App) {}
}

// Though this does not end in "Plugin", the lint is silenced, so no error is raised.
#[allow(bevy::plugin_not_ending_in_plugin)]
struct Baz;

impl Plugin for Baz {
    fn build(&self, _app: &mut App) {}
}

external! {
    // This should _not_ raise an error, since it is defined in an external crate.
    struct ExternalBaz;
}

impl Plugin for ExternalBaz {
    fn build(&self, _app: &mut App) {}
}

macro_rules! local_macro {
    () => {
        struct InternalBaz;
        //~^ ERROR: implemented `Plugin` for a structure whose name does not end in "Plugin"
        //~| HELP: rename the plugin
    };
}

local_macro!();

//~v NOTE: `Plugin` implemented here
impl Plugin for InternalBaz {
    fn build(&self, _app: &mut App) {}
}

fn main() {
    App::new().add_plugins((Foo, BarPlugin, Baz, InternalBaz, ExternalBaz));
}
