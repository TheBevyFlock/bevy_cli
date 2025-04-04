//! This tests the `panicking_methods` lint, specifically when triggered on the `Query` type.
#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::panicking_methods)]

use bevy::prelude::*;

#[derive(Component)]
struct Foo;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut query: Query<&mut Foo>) {
    let entities = [Entity::PLACEHOLDER; 3];
    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    let [_, _, _] = query.many(entities);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_many(entities)`

    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    let [_, _, _] = Query::many(&query, entities);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_many(&query, entities)`

    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    query.many_mut([]);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_many_mut([])`

    #[expect(
        deprecated,
        reason = "While this method is deprecated, we should still check for it while it exists."
    )]
    Query::many_mut(&mut query, []);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_many_mut(&mut query, [])`
}
