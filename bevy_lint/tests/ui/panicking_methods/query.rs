//@aux-build:../auxiliary/proc_macros.rs
//! This tests the `panicking_query_methods` lint, specifically when triggered on the `Query` type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::panicking_query_methods)]

use bevy::prelude::*;
extern crate proc_macros;
use proc_macros::external;

macro_rules! bad_macro {
    ($q:expr) => {
        $q.single()
        //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
        //~| HELP: use `query.get_single()`
    };
}

#[derive(Component)]
struct Foo;

fn main() {
    App::new().add_systems(Startup, my_system);
}

fn my_system(mut query: Query<&mut Foo>) {
    query.single();
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_single()`

    Query::single(&query);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_single(&query)` and handle the `Option` or `Result

    query.single_mut();
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_single_mut()`

    Query::single_mut(&mut query);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_single_mut(&mut query)`

    let entities = [Entity::PLACEHOLDER; 3];

    let [_, _, _] = query.many(entities);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_many(entities)`

    let [_, _, _] = Query::many(&query, entities);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_many(&query, entities)`

    query.many_mut([]);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `query.get_many_mut([])`

    Query::many_mut(&mut query, []);
    //~^ ERROR:  called a `Query` method that can panic when a non-panicking alternative exists
    //~| HELP: use `Query::get_many_mut(&mut query, [])`

    // Test if external macros get skipped
    external!({
        fn my_system(mut query: Query<&mut Foo>) {
            query.single();
        }
    });
    bad_macro!(query);
}
