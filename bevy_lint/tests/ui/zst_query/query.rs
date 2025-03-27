//! This tests the `zst_query` lint, specifically when triggered on the `Query` type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::zst_query)]

use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component)]
struct ZST;

#[derive(Component)]
#[allow(dead_code)]
struct NonZST(u32);

#[derive(Component)]
#[allow(dead_code)]
struct Generic<T: Sized + Send + Sync + 'static>(T);

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
                generic_immutable_zst,
                generic_mutable_zst,
                generic_mutable_query::<u32>,
                immutable_query_tuple,
                mutable_query_tuple,
                phantom_data_query,
            ),
        )
        .run();
}

fn unit_query(_query: Query<()>) {}

//~| HELP: consider using a filter instead: `With<ZST>`
//~v ERROR: queried a zero-sized type
fn immutable_zst(_query: Query<&ZST>) {}

//~| HELP: consider using a filter instead: `With<ZST>`
//~v ERROR: queried a zero-sized type
fn mutable_zst(_query: Query<&mut ZST>) {}

//~| HELP: consider using a filter instead: `With<ZST>`
//~v ERROR: queried a zero-sized type
fn immutable_zst_tuple(_query: Query<(Entity, &ZST)>) {}

//~| HELP: consider using a filter instead: `With<ZST>`
//~v ERROR: queried a zero-sized type
fn mutable_zst_tuple(_query: Query<(Entity, &mut ZST)>) {}

fn immutable_query(_query: Query<&NonZST>) {}

fn mutable_query(_query: Query<&mut NonZST>) {}

fn generic_immutable_query<T: Sized + Send + Sync + 'static>(_query: Query<&Generic<T>>) {}

fn generic_mutable_query<T: Sized + Send + Sync + 'static>(_query: Query<&mut Generic<T>>) {}

//~| HELP: consider using a filter instead: `With<Generic<ZST>>`
//~v ERROR: queried a zero-sized type
fn generic_immutable_zst(_query: Query<&Generic<ZST>>) {}

//~| HELP: consider using a filter instead: `With<Generic<ZST>>`
//~v ERROR: queried a zero-sized type
fn generic_mutable_zst(_query: Query<&mut Generic<ZST>>) {}

fn immutable_query_tuple(_query: Query<(Entity, &NonZST)>) {}

fn mutable_query_tuple(_query: Query<(Entity, &mut NonZST)>) {}

//~| HELP: consider using a filter instead: `With<Phantom<NonZST>>`
//~v ERROR: queried a zero-sized type
fn phantom_data_query(_query: Query<&Phantom<NonZST>>) {}
