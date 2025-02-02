//! This tests the `panicking_query_methods` lint, specifically when triggered on the `QueryState`
//! type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::panicking_query_methods)]

use bevy::prelude::*;

#[derive(Component)]
struct Foo;

fn main() {
    let mut world = World::new();

    let mut query_state = QueryState::<&mut Foo>::new(&mut world);

    let _ = QueryState::single(&mut query_state, &world);
    //~^ ERROR:  called a `QueryState` method that can panic when a non-panicking alternative exists
    //~| HELP: use `QueryState::get_single(&mut query_state, &world)`

    QueryState::single_mut(&mut query_state, &mut world);
    //~^ ERROR:  called a `QueryState` method that can panic when a non-panicking alternative exists
    //~| HELP: use `QueryState::get_single_mut(&mut query_state, &mut world)`
}
