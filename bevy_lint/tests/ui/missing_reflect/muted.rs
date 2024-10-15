//@check-pass

#![feature(register_tool)]
#![register_tool(bevy)]
#![allow(dead_code)]
#![deny(bevy::missing_reflect)]

use bevy::prelude::*;

// Test when the trait is derived.
#[allow(bevy::missing_reflect)]
#[derive(Component)]
struct MyComponent;

// Test when the trait is manually implemented.
#[allow(bevy::missing_reflect)]
struct MyResource;

impl Resource for MyResource {}
