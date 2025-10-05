//! Tests the `missing_reflect` lint when `Component`, `Resource`, and `Event` are manually
//! implemented.

#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
//~v NOTE: the lint level is defined here
#![deny(bevy::missing_reflect)]
//! Tests the `missing_reflect` lint when `Component`, `Resource`, and `Event` are manually
//! implemented.

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
impl Component for MyComponent {
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
impl Resource for MyResource {}

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined an event without a `Reflect` implementation
struct MyEvent(String);

//~v NOTE: `Event` implemented here
impl Event for MyEvent {
    type Trigger<'a> = GlobalTrigger;
}

//~| HELP: `Reflect` can be automatically derived
//~v ERROR: defined a message without a `Reflect` implementation
struct MyMessage(String);

//~v NOTE: `Message` implemented here
impl Message for MyMessage {}
