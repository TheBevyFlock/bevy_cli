//! This tests the `panicking_methods` lint, specifically when triggered on the `QueryState`
//! type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::panicking_methods)]

use bevy::prelude::*;

#[derive(Component)]
struct Foo;

fn main() {
    let mut world = World::new();

    let mut query_state = QueryState::<&mut Foo>::new(&mut world);

    let _ = query_state.single(&world);
    //~^ ERROR:  called a `QueryState` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query_state.get_single(&world)`

    let _ = QueryState::single(&mut query_state, &world);
    //~^ ERROR:  called a `QueryState` method that can panic when a non-panicking alternative exists
    //~| HELP: use `QueryState::get_single(&mut query_state, &world)`

    query_state.single_mut(&mut world);
    //~^ ERROR:  called a `QueryState` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query_state.get_single_mut(&mut world)`

    QueryState::single_mut(&mut query_state, &mut world);
    //~^ ERROR:  called a `QueryState` method that can panic when a non-panicking alternative exists
    //~| HELP: use `QueryState::get_single_mut(&mut query_state, &mut world)`
}
