//! This is a regression test for [#656](https://github.com/TheBevyFlock/bevy_cli/issues/656) that
//! ensures `unit_in_bundle` is able to handle projection (`T::AssociatedType: Trait`) trait
//! bounds.

#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unit_in_bundle)]
#![allow(dead_code)]

use bevy::ecs::bundle::{Bundle, NoBundleEffect};

// The original reproducible example. This used to cause an ICE, but shouldn't anymore.
fn spawn_batch<I>(_batch: I)
where
    I: IntoIterator + Send + Sync + 'static,
    I::Item: Bundle<Effect: NoBundleEffect>,
{
}

trait WithBundle {
    type Bundle;
}

// A function that takes a bundle as an argument, but uses a projection alias rather than a generic
// parameter.
fn take_with_bundle<B>(_with: B, _bundle: B::Bundle)
where
    B: WithBundle,
    B::Bundle: Bundle,
{
}

struct FooWithBundle;

impl WithBundle for FooWithBundle {
    type Bundle = ();
}

fn main() {
    let _ = spawn_batch(std::iter::once(()));

    //~v ERROR: created a `Bundle` containing a unit `()`
    take_with_bundle(FooWithBundle, ());
}
