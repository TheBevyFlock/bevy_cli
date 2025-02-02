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
    Query::single(&query);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_single(&query)` and handle the `Option` or `Result
    Query::single_mut(&mut query);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_single_mut(&mut query)`

    let entities = [Entity::PLACEHOLDER; 3];

    let [_, _, _] = Query::many(&query, entities);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_many(&query, entities)`

    Query::many_mut(&mut query, []);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_many_mut(&mut query, [])`
}
