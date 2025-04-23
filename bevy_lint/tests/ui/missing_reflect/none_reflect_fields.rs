//! Tests the `missing_reflect` lint when `Component`, `Resource`, and `Event` have fields that do
//! not implement `Reflect`

#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
//~v NOTE: the lint level is defined here
#![deny(bevy::missing_reflect)]

use bevy::prelude::*;

struct NonReflect(u64);

//~v NOTE: `Component` implemented here
#[derive(Component)]
//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a component without a `Reflect` implementation
struct MyComponent {
    reflect: u64,
    non_reflect: NonReflect,
}

//~v NOTE: `Resource` implemented here
#[derive(Resource)]
//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a resource without a `Reflect` implementation
struct MyResource {
    field_1: usize,
    field_2: NonReflect,
}

//~v NOTE: `Event` implemented here
#[derive(Event)]
//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined an event without a `Reflect` implementation
struct MyEvent(NonReflect);

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a resource without a `Reflect` implementation
struct MyImplResource {
    reflect: u64,
    non_reflect: NonReflect,
}

//~v NOTE: `Resource` implemented here
impl Resource for MyImplResource {}
