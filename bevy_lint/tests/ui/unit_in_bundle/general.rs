#![feature(register_tool)]
#![register_tool(bevy)]
#![deny(bevy::unit_in_bundle)]
#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component, Clone)]
struct ComponentA;

#[derive(Component, Clone)]
struct ComponentB;

fn generic_param<B: Bundle>(_bundle: B) {}
fn impl_trait(_bundle: impl Bundle) {}

fn many_bounds<A: Clone, B: Bundle, C: Bundle + Clone>(_a: A, _b: B, _c: C) {}

struct Receiver(u8);

impl Receiver {
    fn associated_fn<B: Bundle>(_bundle: B) {}
    fn method<B: Bundle>(&mut self, _bundle: B) {}
}

struct BundleWrapper<B: Bundle>(B);

impl<B: Bundle> BundleWrapper<B> {
    fn associated_fn(_bundle: B) {}
    fn method(&self, _bundle: B) {}
}

pub fn test_generic_params() {
    let unit = ();
    let contains_unit = (ComponentA, ());

    generic_param(
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );

    generic_param(
        //~v ERROR: created a `Bundle` containing a unit `()`
        unit,
    );

    generic_param((
        ComponentA,
        (
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
            ComponentB,
        ),
    ));

    generic_param((
        //~v ERROR: created a `Bundle` containing a unit `()`
        contains_unit,
        ComponentB,
    ));

    fn returns_unit() {}

    generic_param(
        //~v ERROR: created a `Bundle` containing a unit `()`
        returns_unit(),
    );
}

pub fn test_impl_trait() {
    impl_trait(
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );

    impl_trait((
        ComponentA,
        (
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
            ComponentB,
        ),
    ));
}

pub fn test_many_bounds() {
    many_bounds(
        (),
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );

    many_bounds(
        (),
        (
            ComponentA,
            (
                //~v ERROR: created a `Bundle` containing a unit `()`
                (),
            ),
        ),
        (ComponentA, ComponentB),
    )
}

pub fn test_receiver() {
    Receiver::associated_fn(
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );

    Receiver::associated_fn((
        ComponentA,
        (
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
            ComponentB,
        ),
    ));

    let mut receiver = Receiver(0);

    receiver.method(
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );

    receiver.method((
        ComponentA,
        (
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
            ComponentB,
        ),
    ));

    Receiver::method(
        &mut receiver,
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );
}

pub fn test_bundle_wrapper() {
    BundleWrapper::associated_fn(
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );

    let unit_wrapper = BundleWrapper(
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );

    unit_wrapper.method(
        //~v ERROR: created a `Bundle` containing a unit `()`
        (),
    );

    let bundle_wrapper = BundleWrapper((
        ComponentA,
        (
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
            ComponentB,
        ),
    ));

    bundle_wrapper.method((
        ComponentA,
        (
            //~v ERROR: created a `Bundle` containing a unit `()`
            (),
            ComponentB,
        ),
    ));
}
