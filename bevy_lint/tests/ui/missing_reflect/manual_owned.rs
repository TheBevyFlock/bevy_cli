//! A version of the normal UI test that replaces `#[derive(...)]` attributes with manual
//! implementations.

// We want to require annotations for all errors, but don't need them for notes.
//@require-annotations-for-level: ERROR

#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
#![deny(bevy::missing_reflect)]

use bevy::{ecs::component::StorageType, prelude::*};

struct MyComponent;
//~^ ERROR: defined a component without a `Reflect` implementation
//~| HELP: `Reflect` can be automatically derived

//~v NOTE: `Component` implemented here
impl Component for MyComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

struct MyResource;
//~^ ERROR: defined a resource without a `Reflect` implementation
//~| HELP: `Reflect` can be automatically derived

//~v NOTE: `Resource` implemented here
impl Resource for MyResource {}

struct MyEvent;
//~^ ERROR: defined an event without a `Reflect` implementation
//~| HELP: `Reflect` can be automatically derived

impl Component for MyEvent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

//~v NOTE: `Event` implemented here
impl Event for MyEvent {}
