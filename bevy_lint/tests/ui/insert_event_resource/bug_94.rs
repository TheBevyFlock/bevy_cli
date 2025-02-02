//! [#94]: https://github.com/TheBevyFlock/bevy_cli/issues/94

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
    //~^ ERROR: called `App::init_resource::<&mut app, Events<Foo>>()` instead of
    //`App::add_event::<Foo>(&mut app)
    App::insert_resource::<Events<Foo>>(&mut app, Default::default());
    //~^ ERROR: called `App::insert_resource(&mut app, Events<Foo>)` instead of
    // `App::add_event::<Foo>(&mut app)`
}
