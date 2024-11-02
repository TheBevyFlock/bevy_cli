//! This tests the `zst_query` lint, specifically when triggered on the `Query` type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::zst_query)]

use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component)]
struct Foo;

#[derive(Component)]
#[allow(dead_code)]
struct Bar(u32);

#[derive(Component)]
#[allow(dead_code)]
struct Baz<T: Sized + Send + Sync + 'static>(T);

#[derive(Component)]
#[allow(dead_code)]
struct Phantom<T>(PhantomData<T>);

fn main() {
    App::new()
        .add_systems(
            Startup,
            (
                unit_query,
                immutable_zst,
                mutable_zst,
                immutable_zst_tuple,
                mutable_zst_tuple,
                immutable_query,
                mutable_query,
                generic_immutable_query::<u32>,
                generic_mutable_query::<u32>,
                immutable_query_tuple,
                mutable_query_tuple,
                phantom_data_query,
            ),
        )
        .run();
}

fn unit_query(_query: Query<()>) {}

//~| HELP: consider using a filter instead: `With<Foo>`
//~v ERROR: query for a zero-sized type
fn immutable_zst(_query: Query<&Foo>) {}

//~| HELP: consider using a filter instead: `With<Foo>`
//~v ERROR: query for a zero-sized type
fn mutable_zst(_query: Query<&mut Foo>) {}

//~| HELP: consider using a filter instead: `With<Foo>`
//~v ERROR: query for a zero-sized type
fn immutable_zst_tuple(_query: Query<(Entity, &Foo)>) {}

//~| HELP: consider using a filter instead: `With<Foo>`
//~v ERROR: query for a zero-sized type
fn mutable_zst_tuple(_query: Query<(Entity, &mut Foo)>) {}

fn immutable_query(_query: Query<&Bar>) {}

fn mutable_query(_query: Query<&mut Bar>) {}

fn generic_immutable_query<T: Sized + Send + Sync + 'static>(_query: Query<&Baz<T>>) {}

fn generic_mutable_query<T: Sized + Send + Sync + 'static>(_query: Query<&mut Baz<T>>) {}

fn immutable_query_tuple(_query: Query<(Entity, &Bar)>) {}

fn mutable_query_tuple(_query: Query<(Entity, &mut Bar)>) {}

//~| HELP: consider using a filter instead: `With<Phantom<Bar>>`
//~v ERROR: query for a zero-sized type
fn phantom_data_query(_query: Query<&Phantom<Bar>>) {}
