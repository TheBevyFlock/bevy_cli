#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_event_resource)]

use bevy::prelude::*;

#[derive(Event)]
struct Foo;

fn main() {
    App::new().init_resource::<Events<Foo>>();
    //~^ ERROR: App::init_resource::<Events<T>>()` instead of `App::add_event::<T>()`

    App::new().insert_resource::<Events<Foo>>(Default::default());
    //~^ ERROR: called `App::insert_resource::<Events<Foo>>(Default::default())` instead of
    // `App::add_event::<Default::default()>()`
    // Make sure the correct type is detected, even when not explicitly passed to
    // `insert_resource()`.
    let implied_event: Events<Foo> = Default::default();
    App::new().insert_resource(implied_event);
    //~^ ERROR: called `App::insert_resource(implied_event)` instead of
    // `App::add_event::<implied_event>()`

    // Ensure the lint can be muted by annotating the expression.
    #[allow(bevy::insert_event_resource)]
    App::new().init_resource::<Events<Foo>>();
}
