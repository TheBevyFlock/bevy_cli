//! This tests the `borrowed_reborrowable` lint, specifically when triggered on the
//! `FilteredEntityMut` type.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::borrowed_reborrowable)]

use bevy::{
    ecs::world::{FilteredEntityMut, FilteredEntityRef},
    prelude::*,
};

// OK: Lint does not apply to immutable references
fn immutable_reference(_param: &FilteredEntityMut) {
    // ...
}

//~| HELP: use `FilteredEntityMut` instead
//~v ERROR: parameter takes `&mut FilteredEntityMut` instead of a re-borrowed `FilteredEntityMut`
fn mutable_reference(_param: &mut FilteredEntityMut) {
    // ...
}

//~| HELP: use `FilteredEntityMut` instead
//~v ERROR: parameter takes `&mut FilteredEntityMut` instead of a re-borrowed `FilteredEntityMut`
fn mutable_reference_return(_param: &'_ mut FilteredEntityMut) -> usize {
    123
}

// OK: Lint does not apply when return type relies on reference lifetime
fn mutable_reference_bounded_return<'w, 's>(
    param: &'w mut FilteredEntityMut<'w, 's>,
) -> FilteredEntityRef<'w, 's> {
    param.as_readonly()
}

// OK: Lint does not apply when return type relies on reference lifetime
fn mutable_reference_bounded_return_complex<'w, 's>(
    param: &'w mut FilteredEntityMut<'w, 's>,
) -> Vec<(usize, FilteredEntityRef<'w, 's>)> {
    vec![(1, param.as_readonly())]
}

fn main() {
    fn some_system(world: &mut World) {
        let mut param: FilteredEntityMut = world.spawn_empty().into();
        immutable_reference(&param);
        mutable_reference(&mut param);
        let _ = mutable_reference_return(&mut param);
        let _ = mutable_reference_bounded_return(&mut param);

        let mut param: FilteredEntityMut = world.spawn_empty().into();
        let _ = mutable_reference_bounded_return_complex(&mut param);
    }

    App::new().add_systems(Update, some_system).run();
}
