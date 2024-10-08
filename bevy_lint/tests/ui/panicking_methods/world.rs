//! The tests ths `panicking_query_methods` lint, specifically when triggered on the `World` type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::panicking_world_methods)]

use bevy::prelude::*;

#[derive(Component)]
struct Bob;

#[derive(Resource)]
struct Jeffrey;

// A non-send resource.
struct Patrick;

fn main() {
    let mut world = World::new();

    let bob = world.spawn(Bob).id();

    world.entity(bob);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.entity_mut(bob);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.many_entities([bob]);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.many_entities_mut([bob]);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.resource::<Jeffrey>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.resource_mut::<Jeffrey>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.resource_ref::<Jeffrey>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.non_send_resource::<Patrick>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.non_send_resource_mut::<Patrick>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists

    world.schedule_scope(Update, |_world, _schedule| {});
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
}
