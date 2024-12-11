//! Tests the `missing_reflect` lint when `Component`, `Resource`, and `Event` are manually
//! implemented.

#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
//~v NOTE: the lint level is defined here
#![deny(bevy::missing_reflect)]

use bevy::{ecs::component::StorageType, prelude::*};

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a component without a `Reflect` implementation
struct MyComponent;

//~v NOTE: `Component` implemented here
impl Component for MyComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a resource without a `Reflect` implementation
struct MyResource {
    field_1: usize,
    field_2: bool,
}

//~v NOTE: `Resource` implemented here
impl Resource for MyResource {}

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined an event without a `Reflect` implementation
struct MyEvent(String);

impl Component for MyEvent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

//~v NOTE: `Event` implemented here
impl Event for MyEvent {
    type Traversal = ();
}
