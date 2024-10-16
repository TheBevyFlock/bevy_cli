//! This test tracks the bug reported in [#94]. When this starts failing, the bug has been fixed.
//!
//! [#94]: https://github.com/TheBevyFlock/bevy_cli/issues/94

//@check-pass

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_event_resource)]

use bevy::prelude::*;

#[derive(Event)]
struct Foo;

fn main() {
    let mut app = App::new();

    // These both should error, but currently do not.
    App::init_resource::<Events<Foo>>(&mut app);
    App::insert_resource::<Events<Foo>>(&mut app, Default::default());
}
