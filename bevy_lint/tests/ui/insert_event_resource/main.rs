//@aux-build:../auxiliary/proc_macros.rs
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_event_resource)]

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;

#[derive(Event)]
struct Foo;

fn main() {
    let mut app = App::new();
    App::new().init_resource::<Events<Foo>>();
    //~^ ERROR: called `App::init_resource::<Events<Foo>>()` instead of `App::add_event::<Foo>()`

    App::init_resource::<Events<Foo>>(&mut app);
    //~^ ERROR: called `App::init_resource::<Events<Foo>>(&mut app)` instead of
    // `App::add_event::<Foo>(&mut app)`

    App::new().insert_resource::<Events<Foo>>(Default::default());
    //~^ ERROR: called `App::insert_resource::<Events<Foo>>(Default::default())` instead of
    // `App::add_event::<Foo>()`

    App::insert_resource::<Events<Foo>>(&mut app, Default::default());
    //~^ ERROR: called `App::insert_resource::<Events<Foo>>(&mut app, Default::default())` instead
    // of `App::add_event::<Foo>(&mut app)`

    // Make sure the correct type is detected, even when not explicitly passed to
    // `insert_resource()`.
    let implied_event: Events<Foo> = Default::default();
    App::new().insert_resource(implied_event);
    //~^ ERROR: called `App::insert_resource(implied_event)` instead of `App::add_event::<Foo>()`

    // Ensure the lint can be muted by annotating the expression.
    #[allow(bevy::insert_event_resource)]
    App::new().init_resource::<Events<Foo>>();

    external!({
        let mut app = App::new();
        App::new().init_resource::<Events<Foo>>();
    });
}
