//! This tests the `panicking_query_methods` lint, specifically when triggered on the `World` type.

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
    World::entity(&world, bob);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_entity(&world, bob)`

    World::entity_mut(&mut world, bob);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_entity_mut(&mut world, bob)`

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
    World::many_entities_mut(&mut world, [bob]);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_many_entities_mut(&mut world, [bob])`

    World::resource::<Jeffrey>(&world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_resource::<Jeffrey>(&world)`

    World::resource_mut::<Jeffrey>(&mut world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_resource_mut::<Jeffrey>(&mut world)`

    World::resource_ref::<Jeffrey>(&world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_resource_ref::<Jeffrey>(&world)`

    World::non_send_resource::<Patrick>(&world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_non_send_resource::<Patrick>(&world)`

    World::non_send_resource_mut::<Patrick>(&mut world);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::get_non_send_resource_mut::<Patrick>(&mut world)`

    World::run_schedule(&mut world, Update);
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::try_run_schedule(&mut world, Update)`

    World::schedule_scope(&mut world, Update, |_world, _schedule| {});
    //~^ ERROR: called a `World` method that can panic when a non-panicking alternative exists
    //~| HELP: use `World::try_schedule_scope(&mut world, Update, |_world, _schedule| {})`
}
