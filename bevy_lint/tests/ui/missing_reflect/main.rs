// We want to require annotations for all errors, but don't need them for notes.
//@require-annotations-for-level: ERROR

#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
#![deny(bevy::missing_reflect)]

use bevy::prelude::*;

//~v NOTE: `Component` implemented here
#[derive(Component)]
struct MyComponent;
//~^ ERROR: defined a component without a `Reflect` implementation
//~| HELP: `Reflect` can be automatically derived

//~v NOTE: `Resource` implemented here
#[derive(Resource)]
struct MyResource;
//~^ ERROR: defined a resource without a `Reflect` implementation
//~| HELP: `Reflect` can be automatically derived

//~v NOTE: `Event` implemented here
#[derive(Event)]
struct MyEvent;
//~^ ERROR: defined an event without a `Reflect` implementation
//~| HELP: `Reflect` can be automatically derived
