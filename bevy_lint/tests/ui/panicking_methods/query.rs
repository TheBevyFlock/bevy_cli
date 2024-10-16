//! This tests the `panicking_query_methods` lint, specifically when triggered on the `Query` type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::panicking_query_methods)]

use bevy::prelude::*;

#[derive(Component)]
struct Foo;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut query: Query<&mut Foo>) {
    query.single();
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_single()`

    query.single_mut();
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_single_mut()`

    let entities = [Entity::PLACEHOLDER; 3];

    let [_, _, _] = query.many(entities);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_many(entities)`

    query.many_mut([]);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_many_mut([])`
}
