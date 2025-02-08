//@aux-build:../auxiliary/proc_macros.rs
//! This tests the `panicking_query_methods` lint, specifically when triggered on the `World` type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::panicking_world_methods)]

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;

#[derive(Component)]
struct Bob;

#[derive(Resource)]
struct Jeffrey;

// A non-send resource.
struct Patrick;

macro_rules! local_macro {
    () => {
        let mut world = World::new();

        let bob = world.spawn(Bob).id();

        world.entity(bob);
        //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
        //~| HELP: use `world.get_entity(bob)`

        World::entity(&world, bob);
        //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
        //~| HELP: use `World::get_entity(&world, bob)`
    };
}

fn main() {
    let mut world = World::new();

    let bob = world.spawn(Bob).id();

    world.entity(bob);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_entity(bob)`

    World::entity(&world, bob);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_entity(&world, bob)`

    world.entity_mut(bob);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_entity_mut(bob)`

    World::entity_mut(&mut world, bob);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_entity_mut(&mut world, bob)`

    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    world.many_entities([bob]);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_many_entities([bob])`

    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    World::many_entities(&mut world, [bob]);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_many_entities(&mut world, [bob])`

    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    world.many_entities_mut([bob]);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_many_entities_mut([bob])`

    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    World::many_entities_mut(&mut world, [bob]);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_many_entities_mut(&mut world, [bob])`

    world.resource::<Jeffrey>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_resource::<Jeffrey>()`

    World::resource::<Jeffrey>(&world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_resource::<Jeffrey>(&world)`

    world.resource_mut::<Jeffrey>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_resource_mut::<Jeffrey>()`

    World::resource_mut::<Jeffrey>(&mut world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_resource_mut::<Jeffrey>(&mut world)`

    world.resource_ref::<Jeffrey>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_resource_ref::<Jeffrey>()`

    World::resource_ref::<Jeffrey>(&world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_resource_ref::<Jeffrey>(&world)`

    world.non_send_resource::<Patrick>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_non_send_resource::<Patrick>()`

    World::non_send_resource::<Patrick>(&world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_non_send_resource::<Patrick>(&world)`

    world.non_send_resource_mut::<Patrick>();
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.get_non_send_resource_mut::<Patrick>()`

    World::non_send_resource_mut::<Patrick>(&mut world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_non_send_resource_mut::<Patrick>(&mut world)`

    world.run_schedule(Update);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.try_run_schedule(Update)`

    World::run_schedule(&mut world, Update);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::try_run_schedule(&mut world, Update)`

    world.schedule_scope(Update, |_world, _schedule| {});
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `world.try_schedule_scope(Update, |_world, _schedule| {})`

    World::schedule_scope(&mut world, Update, |_world, _schedule| {});
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::try_schedule_scope(&mut world, Update, |_world, _schedule| {})`
    external!({
        let mut world = World::new();
        let bob = world.spawn(Bob).id();
        world.entity(bob);
    });
    local_macro!();
}
