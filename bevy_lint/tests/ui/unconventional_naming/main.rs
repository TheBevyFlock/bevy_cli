#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unconventional_naming)]
//~^ NOTE: the lint level is defined here
#![allow(dead_code)]
use bevy::prelude::*;

//~v NOTE: `SystemSet` implemented here
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MyAudio;
//~^ ERROR: unconventional type name for a `Plugin` or `SystemSet`
//~| HELP: structure that implements SystemSet should end in Set, rename MyAudio to MyAudioSet

// This should raise an error, since it does not end in "Plugin".
struct Foo;
//~^ ERROR: unconventional type name for a `Plugin` or `SystemSet`
//~| HELP: structure that implements Plugin should end in Plugin, rename Foo to FooPlugin

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
#[allow(bevy::unconventional_naming)]
struct Baz;

impl Plugin for Baz {
    fn build(&self, _app: &mut App) {}
}

fn main() {
    App::new().add_plugins((Foo, BarPlugin, Baz));
}
