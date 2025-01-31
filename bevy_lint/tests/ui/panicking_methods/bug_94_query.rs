#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::panicking_query_methods)]

use bevy::prelude::*;

#[derive(Component)]
struct Foo;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(query: Query<&Foo>) {
    Query::single(&query);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_single()`
}
