//! Tests the `missing_reflect` lint when `Component`, `Resource`, and `Event` are manually
//! implemented for references.
//!
//! This test ensures that the lint calls `peel_refs()` on the type in order to correctly locate
//! the source definition.
//!
//! Note that all 3 traits require `'static` bounds, so generic `'a` lifetimes are not tested.

#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
//~v NOTE: the lint level is defined here
#![deny(bevy::missing_reflect)]

use bevy::{
    ecs::{
        component::{Mutable, StorageType},
        event::GlobalTrigger,
    },
    prelude::*,
};

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a component without a `Reflect` implementation
struct MyComponent;

//~v NOTE: `Component` implemented here
impl Component for &'static MyComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a resource without a `Reflect` implementation
struct MyResource {
    field_1: usize,
    field_2: bool,
}

//~v NOTE: `Resource` implemented here
impl Resource for &'static &'static MyResource {}

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined an event without a `Reflect` implementation
struct MyEvent(String);

impl Component for &'static &'static &'static MyEvent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

//~v NOTE: `Event` implemented here
impl Event for &'static &'static &'static MyEvent {
    type Trigger<'a> = GlobalTrigger;
}

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a message without a `Reflect` implementation
struct MyMessage(String);

impl Component for MyMessage {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

//~v NOTE: `Message` implemented here
impl Message for &'static &'static &'static MyMessage {}
