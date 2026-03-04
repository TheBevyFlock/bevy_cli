//! Tests that `missing_reflect` can be applied to individual items without a warning.

#![feature(register_tool)]
#![register_tool(bevy)]

#![deny(bevy::missing_reflect)]
#![deny(unused_attributes)]

use bevy::prelude::*;

//~v unused_attributes
#[allow(bevy::missing_reflect)]
#[derive(Component)]
pub struct MyComponent(NonReflect);

struct NonReflect;
