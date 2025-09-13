#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::insert_message_resource)]

use bevy::prelude::*;

#[derive(Message)]
struct Foo;

fn main() {
    let mut app = App::new();
    App::new().init_resource::<Events<Foo>>();
    //~^ ERROR: called `App::init_resource::<Events<Foo>>()` instead of
    // `App::add_message::<Foo>()`

    App::init_resource::<Events<Foo>>(&mut app);
    //~^ ERROR: called `App::init_resource::<Events<Foo>>(&mut app)` instead of
    // `App::add_message::<Foo>(&mut app)`

    App::new().insert_resource::<Events<Foo>>(Default::default());
    //~^ ERROR: called `App::insert_resource::<Events<Foo>>(Default::default())` instead of
    // `App::add_message::<Foo>()`

    App::insert_resource::<Events<Foo>>(&mut app, Default::default());
    //~^ ERROR: called `App::insert_resource::<Events<Foo>>(&mut app, Default::default())` instead
    // of `App::add_message::<Foo>(&mut app)`

    // Make sure the correct type is detected, even when not explicitly passed to
    // `insert_resource()`.
    let implied_event: Events<Foo> = Default::default();
    App::new().insert_resource(implied_event);
    //~^ ERROR: called `App::insert_resource(implied_event)` instead of `App::add_message::<Foo>()`

    // Ensure the lint can be muted by annotating the expression.
    #[allow(bevy::insert_message_resource)]
    App::new().init_resource::<Events<Foo>>();
}
